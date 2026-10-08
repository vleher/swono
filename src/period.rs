use std::fmt::{Debug, Display};

use log::debug;

use crate::{incomeasset::IncomeAsset, market::MarketConditions, utils};

pub struct Period<'a> {
    id: usize,
    age: f64,
    income_asset: IncomeAsset<'a>,
    market: &'a MarketConditions,
    spending: f64,
}

impl<'a> Period<'a> {
    pub fn new(
        id: usize,
        age: f64,
        income_asset: IncomeAsset<'a>,
        market: &'a MarketConditions,
    ) -> Self {
        let mut p = Period {
            id,
            age,
            income_asset,
            market,
            spending: 0.0,
        };
        p.initialize();
        p
    }

    fn initialize(&mut self) {
        self.income_asset
            .initialize_values(self.market, self.id as f64);
    }

    pub fn income_asset(&self) -> &IncomeAsset<'_> {
        &self.income_asset
    }

    pub fn set_spending(&mut self, new_spending: f64) -> f64 {
        self.spending = new_spending;
        self.spending
    }
}

impl<'a> Display for Period<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "----")?;
        writeln!(f, "{}  Age: {:.1}", self.id, self.age)?;

        write!(
            f,
            "Assets Unlocked: ${:.2}",
            self.income_asset.total_unlocked_assets_value(self.age),
        )?;
        for a in self.income_asset.assets_unlocked(self.age) {
            write!(f, " {}  ", a)?;
        }
        writeln!(f)?;
        write!(
            f,
            "Income Unlocked: ${:.2}",
            self.income_asset.total_unlocked_income_value(self.age)
        )?;
        for i in self.income_asset.income_unlocked(self.age) {
            write!(f, " {}  ", i)?;
        }
        writeln!(f)?;
        writeln!(
            f,
            "Total Unlocked: ${:.2}",
            self.income_asset.total_unlocked_assets_value(self.age)
                + self.income_asset.total_unlocked_income_value(self.age),
        )?;

        writeln!(f, "Spending: ${:.2}", self.spending)?;

        writeln!(f, "---")?;
        Ok(())
    }
}
