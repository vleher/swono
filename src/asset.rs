use std::{
    cmp::Ordering,
    fmt::{self, Formatter},
};

use log::{debug, info};
use serde::{Deserialize, Serialize};

use crate::{
    market::MarketConditions,
    user::User,
    utils::{calculate_compound, calculate_principal},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AccountType {
    FourOone,
    Ira,
    Bonds,
    Stocks,
    Cash,
    Other,
    Income,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Asset {
    name: String,
    asset_type: AccountType,
    real_return: f64,
    // Age at which it can be withdrawn
    start_age: f64,
    end_age: f64,
    tax_rate: f64,
}

impl fmt::Display for Asset {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Asset : {} type: {:?} real return: {} startage: {} endage: {} tax: {}",
            self.name,
            self.asset_type,
            self.real_return,
            self.start_age,
            self.end_age,
            self.tax_rate
        )
    }
}

impl PartialEq for Asset {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Asset {}

impl PartialOrd for Asset {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Asset {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start_age()
            .partial_cmp(&other.start_age())
            .unwrap_or(Ordering::Equal)
            .then_with(|| {
                self.real_return_yearly()
                    .partial_cmp(&other.real_return_yearly())
                    .unwrap_or(Ordering::Equal)
            })
    }
}

impl Asset {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn start_age(&self) -> f64 {
        self.start_age
    }

    pub fn real_return_yearly(&self) -> f64 {
        self.real_return
    }

    pub fn end_age(&self) -> f64 {
        self.end_age
    }

    pub fn is_accessable(&self, age: f64) -> bool {
        age >= self.start_age() && age <= self.end_age()
    }
}

#[derive(Debug, Clone)]
pub struct AssetWithValue<'a> {
    config: &'a Asset,
    value: f64,
}

impl<'a> AssetWithValue<'a> {
    pub fn new(config: &'a Asset, value: f64) -> AssetWithValue<'a> {
        Self { config, value }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn set_value(&mut self, new_value: f64) -> f64 {
        self.value = new_value;
        self.value
    }

    pub fn config(&self) -> &Asset {
        self.config
    }

    pub fn process_asset(
        &self,
        current_user: &User,
        market: &MarketConditions,
        age_in_retirement: f64,
        period_in_retirement: usize,
        revenue_needed: f64,
    ) -> (f64, f64, f64) {
        // Real return is the rate over the inflation
        let rate = (self.config().real_return_yearly() + market.inflation_yearly())
            / (current_user.periods_in_year() as f64);

        let mut updated_asset_value = self.value();
        let mut revenue_still_needed = revenue_needed;
        let mut frozen_asset = 0.0;

        if self.value() > 0.0
            && self.config().is_accessable(age_in_retirement)
            && revenue_needed > 0.0
        {
            let principal = calculate_principal(revenue_needed, rate, period_in_retirement as f64);

            updated_asset_value = f64::max(0.0, updated_asset_value - principal);
            frozen_asset += f64::min(self.value(), principal);
            let updated_withdrawal =
                calculate_compound(frozen_asset, rate, period_in_retirement as f64);
            revenue_still_needed -= f64::min(revenue_still_needed, updated_withdrawal);
            debug!(
                "{} : Withdraw {updated_withdrawal:.2}: Asset Value: {:.2} => {:.2}",
                self.config().name(),
                self.value(),
                updated_asset_value
            );
            if period_in_retirement > 0 {
                debug!(">>>> Freezing {frozen_asset:.2}");
            }
        }
        debug!(
        "{} : Revenue: {revenue_needed:.2} => {revenue_still_needed:.2} : Asset Value: {:.2} => {:.2}",
        self.config().name(),
        self.value(),
        updated_asset_value
    );

        (updated_asset_value, frozen_asset, revenue_still_needed)
    }
}

impl<'a> PartialEq for AssetWithValue<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<'a> Eq for AssetWithValue<'a> {}

impl<'a> PartialOrd for AssetWithValue<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for AssetWithValue<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.config()
            .partial_cmp(other.config())
            .unwrap_or(Ordering::Equal)
            .then_with(|| {
                self.value()
                    .partial_cmp(&other.value())
                    .unwrap_or(Ordering::Equal)
            })
    }
}
