use chrono::NaiveDate;

use crate::{asset::AssetWithValue, income::IncomeWithValue, utils};

#[derive(Clone)]
pub struct IncomeAsset<'a> {
    input_date: NaiveDate,
    income_list: Vec<IncomeWithValue<'a>>,
    asset_list: Vec<AssetWithValue<'a>>,
}

impl<'a> IncomeAsset<'a> {
    pub fn new(
        input_date: NaiveDate,
        income_list: Vec<IncomeWithValue<'a>>,
        asset_list: Vec<AssetWithValue<'a>>,
    ) -> Self {
        Self {
            input_date,
            income_list,
            asset_list,
        }
    }

    pub fn input_date(&self) -> NaiveDate {
        self.input_date
    }

    pub fn total_asset_value(&self) -> f64 {
        self.asset_list.iter().map(|asset| asset.value()).sum()
    }

    pub fn total_income_value(&self) -> f64 {
        self.income_list.iter().map(|i| i.value()).sum()
    }

    pub fn asset_list(&self) -> &Vec<AssetWithValue> {
        &self.asset_list
    }

    pub fn income_list(&self) -> &Vec<IncomeWithValue> {
        &self.income_list
    }

    pub fn assets_unlocked(&self, age: f64) -> Vec<AssetWithValue> {
        self.asset_list
            .iter()
            .filter(|a| a.is_accessable(age))
            .cloned()
            .collect()
    }

    pub fn income_unlocked(&self, age: f64) -> Vec<IncomeWithValue> {
        self.income_list
            .iter()
            .filter(|i| i.is_accessable(age))
            .cloned()
            .collect()
    }

    pub(crate) fn total_unlocked_assets_value(&self, age: f64) -> f64 {
        self.asset_list
            .iter()
            .filter(|asset| asset.is_accessable(age))
            .map(|asset| asset.value())
            .sum()
    }

    pub(crate) fn total_unlocked_income_value(&self, age: f64) -> f64 {
        self.income_list
            .iter()
            .filter(|i| i.is_accessable(age))
            .map(|i| i.value())
            .sum()
    }

    pub(crate) fn total_locked_assets_value(&self, age: f64) -> f64 {
        self.asset_list
            .iter()
            .filter(|asset| !asset.is_accessable(age))
            .map(|asset| asset.value())
            .sum()
    }

    pub(crate) fn total_locked_income_value(&self, age: f64) -> f64 {
        self.income_list
            .iter()
            .filter(|i| !i.is_accessable(age))
            .map(|i| i.value())
            .sum()
    }

    pub(crate) fn initialize_values(
        &mut self,
        market: &'a crate::market::MarketConditions,
        period_in_retirement: f64,
    ) {
        self.asset_list.iter_mut().for_each(|a| {
            let rate = (a.config().real_return_yearly() + market.inflation_yearly())
                / (market.periods_in_year() as f64);

            let new_value = utils::calculate_compound(a.value(), rate, period_in_retirement);
            a.set_value(new_value);
        });
        self.income_list.iter_mut().for_each(|i| {
            let rate = (i.config().real_return()) / (market.periods_in_year() as f64);
            let new_value = utils::calculate_compound(i.value(), rate, period_in_retirement);
            i.set_value(new_value);
        });
    }
}
