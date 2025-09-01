mod asset;
mod config;
mod income;
mod market;
mod user;
mod utils;

use asset::AssetWithValue;
use config::Configuration;
use income::IncomeWithValue;
use utils::calculate_compound;

use env_logger::Env;
use log::debug;
use log::error;
use log::info;
use market::MarketConditions;
use std::io::Write;

use user::User;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .init();
    application();
}

fn load_income_assets(
    configuration: &Configuration,
) -> (Vec<AssetWithValue>, Vec<IncomeWithValue>) {
    info!(
        "Loading Input values from {}",
        configuration.user().input_file()
    );
    let input_data = match simplecsv::parse_from_file(configuration.user().input_file(), true) {
        Ok(data) => data,
        Err(error) => {
            error!(
                "Error parsing input file : {} Error: {}",
                configuration.user().input_file(),
                error
            );
            panic!("Exit!!!. No input data found.");
        }
    };
    let size = input_data.data().len();
    debug!("Size of the input data file : {size}");

    let mut asset_list: Vec<AssetWithValue> = Vec::new();
    let mut income_list: Vec<IncomeWithValue> = Vec::new();

    for asset in configuration.assets() {
        debug!("Reading asset {} from input", asset.name());
        let value = input_data
            .get_value_by_name(size - 1, asset.name())
            .unwrap_or_default()
            .parse()
            .unwrap();
        info!("Asset {} with {:.2}", asset.name(), value);
        asset_list.push(AssetWithValue::new(asset, value));
    }

    for income in configuration.income() {
        debug!("Reading income {} from input", income.name());
        let value = input_data
            .get_value_by_name(size - 1, income.name())
            .unwrap_or_default()
            .parse()
            .unwrap();
        info!("Income {} with {:.2}", income.name(), value);
        income_list.push(IncomeWithValue::new(income, value));
    }

    (asset_list, income_list)
}

fn load_configuration() -> Configuration {
    let config_file_name = "swono.config.toml";
    debug!("Loading configuration file : {config_file_name}");
    let config = Configuration::new_from_file(config_file_name);

    let config = match config {
        Ok(cf) => cf,
        Err(error) => {
            error!("Cannot load config file: {config_file_name}. Error: {error}");
            panic!("Exit!!!. Create a valid configuration to proceed.")
        }
    };
    config
}

fn application() {
    info!("Starting SWONO!");

    // Configuration for the application
    let configuration = load_configuration();
    // Data for the asset and income accounts
    let (asset_list, income_list) = load_income_assets(&configuration);
    // User information
    let current_user = configuration.user();
    // Market Information
    let market = configuration.market();
    // start value for iteration
    let start_loop_value = 1000.0;
    let mut expense_loop_value = start_loop_value;

    let mut try_again = true;
    while try_again {
        debug!("Running with {expense_loop_value:.2}");
        let (revenue_shortfall, total_revenue, assets_left, output_row) = run_simulation(
            &income_list,
            &asset_list,
            current_user,
            market,
            expense_loop_value,
        );

        let mut asset_value_left = 0.0;
        for asset in &assets_left {
            if asset.config().is_accessable(current_user.current_age()) {
                asset_value_left += asset.value();
            }
        }
        let total_asset_revenue = total_revenue + asset_value_left;
        let per_period_withdrawal = expense_loop_value / (current_user.periods_in_year() as f64);
        info!("Running with {expense_loop_value:.2} ({per_period_withdrawal:.2}) caused a shortfall of {revenue_shortfall:.2} and a revenue of {total_revenue:.2} with accessable assets worth {asset_value_left:.2} : Total : {total_asset_revenue:.2}");

        try_again = match asset_value_left {
            0.0..100.0 => {
                if let 1000.0.. = expense_loop_value {
                    info!("");
                    let mut initial_asset = 0.0;
                    for asset in &asset_list {
                        initial_asset += asset.value();
                    }
                    let withdrawal_rate = expense_loop_value / initial_asset;
                    info!("Can have an yearly expense of {expense_loop_value:.2} ({withdrawal_rate:.4}) ({per_period_withdrawal:.2})");
                }
                let output_file = simplecsv::new_csv_builder()
                    .has_header(true)
                    .header(
                        "Period,Age,Expenses,Income,AssetWithdrawal,TotalRevenue,TotalAssets"
                            .to_string(),
                    )
                    .rows(output_row);
                let _ = output_file.build().save_to_file("outputfile.csv");

                if expense_loop_value < current_user.yearly_expenses() {
                    info!(
                        "[{}] expenses are HIGHER than revenue.Not Yet!",
                        current_user.name()
                    );
                }
                false
            }
            100.0.. => {
                expense_loop_value +=
                    asset_value_left / (current_user.total_periods_of_retirement() as f64);
                true
            }
            _ => {
                error!(
                    "ERROR {expense_loop_value:.2} {total_asset_revenue:.2} {revenue_shortfall:.2}"
                );
                false
            }
        };
    }
    info!("{current_user:?}");
}

fn run_simulation<'a>(
    income_list: &Vec<IncomeWithValue>,
    asset_list: &'a [AssetWithValue<'a>],
    current_user: &User,
    market: &MarketConditions,
    yearly_expenses: f64,
) -> (f64, f64, Vec<AssetWithValue<'a>>, Vec<String>) {
    let mut revenue_still_needed = 0.0;
    let mut initial_revenue = 0.0;
    let mut total_frozen_asset = 0.0;
    let mut current_asset_list = asset_list.to_owned();
    let mut output_row = Vec::new();
    for period_in_retirement in (0..current_user.total_periods_of_retirement()).rev() {
        let age_in_retirement = current_user.current_age()
            + (period_in_retirement / current_user.periods_in_year()) as f64
            + (period_in_retirement % current_user.periods_in_year()) as f64
                / current_user.periods_in_year() as f64;

        let initial_fixed_cost = yearly_expenses / (current_user.periods_in_year() as f64);
        let inflation = market.inflation_yearly() / (current_user.periods_in_year() as f64);
        let fixed_cost =
            calculate_compound(initial_fixed_cost, inflation, period_in_retirement as f64);
        info!("");
        info!("[{period_in_retirement}] : Age: {age_in_retirement:.2} FixedCost: {fixed_cost:.2}");

        current_asset_list.sort_by(|a, b| b.cmp(a));

        // Calculate Total Income for the period
        let mut total_income = 0.0;
        for income in income_list {
            let generated_income =
                income.compute_income(current_user, age_in_retirement, period_in_retirement);
            total_income += generated_income;
        }
        info!("Generated Total Income : {total_income:.2}");
        let rest_of_revenue = (fixed_cost - total_income) * current_user.buffer();

        debug!("Assets will need to provide for {rest_of_revenue:.2}");
        revenue_still_needed = rest_of_revenue;
        for asset in &mut current_asset_list {
            let (new_asset_value, frozen_asset, revenue_left) = asset.process_asset(
                current_user,
                market,
                age_in_retirement,
                period_in_retirement,
                revenue_still_needed,
            );
            revenue_still_needed = revenue_left;
            total_frozen_asset += frozen_asset;
            debug!("Still Needed : {revenue_still_needed:.2} Frozen : {frozen_asset:.2} Total Frozen: {total_frozen_asset:.2} New Asset Value: {new_asset_value:.2}");

            asset.set_value(new_asset_value);
        }

        initial_revenue = total_income + (rest_of_revenue - revenue_still_needed);

        output_row.push(format!("{period_in_retirement},{age_in_retirement:.2},{fixed_cost:.2},{total_income:.2},{:.2},{:.2},{total_frozen_asset:.2}",(rest_of_revenue-revenue_still_needed), (initial_revenue)));
        if revenue_still_needed > 0.0 {
            print_shortfall(fixed_cost, revenue_still_needed);
        }
    }
    (
        revenue_still_needed,
        initial_revenue,
        current_asset_list,
        output_row,
    )
}

fn print_shortfall(min_expenses: f64, shortfall: f64) {
    info!("---------Shortfall----------");
    info!("Min Needed:{min_expenses:.2} Shortfall:{shortfall:.2}");
    info!("----------------------------");
}
