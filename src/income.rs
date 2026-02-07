use std::fmt;

use log::{debug, info};
use serde::{Deserialize, Serialize};

use super::asset::AccountType;
use crate::user::User;
use crate::utils::calculate_compound;

#[derive(Serialize, Deserialize, Debug)]
pub struct Income {
    name: String,
    asset_type: AccountType,
    real_return: f64,
    start_age: f64,
    end_age: f64,
    tax_rate: f64,
}

impl fmt::Display for Income {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Income : {} startage: {} endage: {}",
            self.name, self.start_age, self.end_age
        )
    }
}

impl Income {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn start_age(&self) -> f64 {
        self.start_age
    }
    pub fn end_age(&self) -> f64 {
        self.end_age
    }

    pub(crate) fn real_return(&self) -> f64 {
        self.real_return
    }
}

pub struct IncomeWithValue<'a> {
    config: &'a Income,
    value: f64,
}

impl<'a> IncomeWithValue<'a> {
    pub fn new(config: &'a Income, value: f64) -> IncomeWithValue<'a> {
        Self { config, value }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn config(&self) -> &Income {
        self.config
    }

    pub fn compute_income(
        &self,
        current_user: &User,
        age_in_retirement: f64,
        period_in_retirement: usize,
    ) -> f64 {
        let mut generated_income = 0.0;
        if self.value() > 0.0
            && self.config().start_age() <= age_in_retirement
            && self.config().end_age() > age_in_retirement
        {
            let rate = (self.config().real_return()) / (current_user.periods_in_year() as f64);
            generated_income = calculate_compound(
                self.value() / (current_user.periods_in_year() as f64),
                rate,
                period_in_retirement as f64,
            );

            debug!("{} : pays {:.2}", self.config().name(), generated_income);
        }
        generated_income
    } // application
}
