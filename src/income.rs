use std::fmt::{self, Display};

use log::{debug, info};
use serde::{Deserialize, Serialize};

use super::asset::AccountType;
use crate::user::User;
use crate::utils::calculate_compound;

#[derive(Serialize, Deserialize, Debug, Clone)]
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

    pub fn is_accessable(&self, age: f64) -> bool {
        age >= self.start_age() && age <= self.end_age()
    }
}

#[derive(Clone, Debug)]
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

    pub fn is_accessable(&self, age: f64) -> bool {
        self.config().is_accessable(age)
    }

    pub(crate) fn set_value(&mut self, new_value: f64) -> f64 {
        self.value = new_value;
        self.value
    }
}

impl<'a> Display for IncomeWithValue<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: ${:.2}", self.config().name(), self.value())
    }
}
