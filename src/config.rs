use config::{Config, ConfigError};
use serde::{Deserialize, Serialize};

use crate::asset::Asset;
use crate::income::Income;
use crate::market::MarketConditions;
use crate::user::User;

#[derive(Debug, Deserialize, Serialize)]
pub struct Configuration {
    title: String,
    user: User,
    market: MarketConditions,
    assets: Vec<Asset>,
    income: Vec<Income>,
}

impl Configuration {
    pub fn new_from_file(file_name: &str) -> Result<Self, ConfigError> {
        let configuration = Config::builder()
            .add_source(config::File::with_name(file_name))
            .build()?;

        configuration.try_deserialize()
    }

    pub(crate) fn user(&self) -> &User {
        &self.user
    }

    pub(crate) fn market(&self) -> &MarketConditions {
        &self.market
    }

    pub fn assets(&self) -> &Vec<Asset> {
        &self.assets
    }

    pub fn income(&self) -> &Vec<Income> {
        &self.income
    }
}
