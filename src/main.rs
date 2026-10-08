mod asset;
mod config;
mod income;
mod incomeasset;
mod market;
mod period;
mod user;
mod utils;

use asset::AssetWithValue;
use chrono::NaiveDate;
use config::Configuration;
use env_logger::Env;
use income::IncomeWithValue;
use incomeasset::IncomeAsset;
use simplecsv::csv::CSVFile;

use log::{debug, error, info};
use std::io::Write;

use crate::{market::MarketConditions, period::Period, user::User};

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .init();

    if let Err(e) = application() {
        error!("Application failed: {}", e);
        std::process::exit(1);
    }
}

/// Main application logic
fn application() -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting SWONO!");

    let configurations = load_configurations()?;
    let configuration = &configurations[0];

    let input_data = load_input_data(configuration)?;
    let complete_asset_list = load_income_assets(&input_data, configuration);
    let current_user = configuration.user();
    let market = configuration.market();
    // Process simulation logic with loaded configuration
    process_simulation_results(complete_asset_list, current_user, market)?;

    info!("\n{current_user:?}");
    Ok(())
}

/// Loads input data from CSV file
fn load_input_data(configuration: &Configuration) -> Result<CSVFile, Box<dyn std::error::Error>> {
    info!(
        "Loading Input values from {}",
        configuration.user().input_file()
    );
    let result =
        simplecsv::parse_from_file(configuration.user().input_file(), true).map_err(|error| {
            error!(
                "Error parsing input file : {} Error: {}",
                configuration.user().input_file(),
                error
            );
            format!("Failed to parse input file: {}", error)
        })?;
    Ok(result)
}

/// Loads all income and asset data from CSV
fn load_income_assets<'a>(
    input_data: &'a CSVFile,
    configuration: &'a Configuration,
) -> Vec<IncomeAsset<'a>> {
    let size = input_data.data().len();
    debug!("Size of the input data file : {size}");

    (0..size)
        .map(|i| load_single_income_assets(input_data, configuration, i))
        .collect()
}

/// Loads income and asset data for a single row
fn load_single_income_assets<'a>(
    input_data: &'a CSVFile,
    configuration: &'a Configuration,
    index: usize,
) -> IncomeAsset<'a> {
    let input_date_str = input_data.get_value_by_index(index, 0).unwrap_or_default();
    let input_date = NaiveDate::parse_from_str(&input_date_str, "%Y-%m-%d").unwrap_or_default();

    // Load assets
    let asset_list: Vec<AssetWithValue> = configuration
        .assets()
        .iter()
        .map(|asset| {
            let value = input_data
                .get_value_by_name(index, asset.name())
                .unwrap_or_default()
                .parse::<f64>()
                .unwrap_or(0.0);
            debug!(
                "Parsed Asset {} with {:.2} at {}",
                asset.name(),
                value,
                asset.start_age()
            );
            AssetWithValue::new(asset, value)
        })
        .collect();

    // Load income
    let income_list: Vec<IncomeWithValue> = configuration
        .income()
        .iter()
        .map(|income| {
            let value = input_data
                .get_value_by_name(index, income.name())
                .unwrap_or_default()
                .parse::<f64>()
                .unwrap_or(0.0);
            debug!(
                "Parsed Income {} with {:.2} at {}",
                income.name(),
                value,
                income.start_age()
            );
            IncomeWithValue::new(income, value)
        })
        .collect();

    IncomeAsset::new(input_date, income_list, asset_list)
}

/// Loads configuration file
fn load_configuration(config_file_name: &str) -> Result<Configuration, Box<dyn std::error::Error>> {
    debug!("Loading configuration file : {config_file_name}");
    let result = Configuration::new_from_file(config_file_name).map_err(|error| {
        error!("Cannot load config file: {config_file_name}. Error: {error}");
        format!("Failed to load config file: {}", error)
    })?;
    Ok(result)
}

/// Loads all configurations
fn load_configurations() -> Result<Vec<Configuration>, Box<dyn std::error::Error>> {
    let config_names = ["swono.config.toml"];
    let mut configs = Vec::new();

    for cn in config_names {
        configs.push(load_configuration(cn)?);
    }

    Ok(configs)
}

/// Process simulation results and generate output
fn process_simulation_results(
    complete_asset_list: Vec<IncomeAsset>,
    current_user: &User,
    market: &MarketConditions,
) -> Result<(), Box<dyn std::error::Error>> {
    for current_incomeasset in complete_asset_list {
        info!("\n-----------------------------------");
        let total_starting_asset: f64 = current_incomeasset.total_asset_value();
        info!(
            "{:?} Total Asset : ${total_starting_asset:.2}",
            current_incomeasset.input_date()
        );

        // Process the current line
        simulate(current_incomeasset, current_user, market);
    }
    Ok(())
}

fn simulate(income_asset: IncomeAsset, user: &User, market: &MarketConditions) {
    // For each period, calculate the locked asset and unlocked assets
    let total_periods = user.length_of_retirement() as usize * market.periods_in_year();

    // Final period
    let age_at_p = utils::calculate_age(user, total_periods, market.periods_in_year());
    let final_period = Period::new(total_periods, age_at_p, income_asset.clone(), market);

    // Current period
    let age_at_p = utils::calculate_age(user, 0, market.periods_in_year());
    let mut current_period = Period::new(0, age_at_p, income_asset.clone(), market);

    let current_income = current_period
        .income_asset()
        .total_unlocked_income_value(user.current_age())
        / market.periods_in_year() as f64;

    let min_asset_withdrawal =
        current_period.income_asset().total_asset_value() / total_periods as f64;
    let min_spending = min_asset_withdrawal + current_income;

    let factor = current_period
        .income_asset()
        .total_unlocked_assets_value(user.current_age())
        / current_period.income_asset().total_asset_value();

    current_period.set_spending(min_spending);

    let max_asset_withdrawal =
        final_period.income_asset().total_asset_value() * factor / total_periods as f64;
    let max_spending = max_asset_withdrawal + current_income;

    debug!("{}", final_period);
    info!("{}", current_period);
    info!(
        "Unlocked Factor: {:.2} Spending Min: ${:.2} ({min_asset_withdrawal:.2}) Max: ${:.2} ({max_asset_withdrawal:.2})",
        factor, min_spending, max_spending
    );
}
