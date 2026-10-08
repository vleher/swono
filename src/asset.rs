use std::{
    cmp::Ordering,
    fmt::{self, Display, Formatter},
};

use serde::{Deserialize, Serialize};

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

    pub fn is_accessable(&self, age: f64) -> bool {
        self.config.is_accessable(age)
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

impl<'a> Display for AssetWithValue<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}: ${:.2}", self.config().name(), self.value())
    }
}
