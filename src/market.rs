use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MarketConditions {
    // Inflation
    inflation: f64,
    periods_in_year: usize,
}

impl MarketConditions {
    pub fn inflation_yearly(&self) -> f64 {
        self.inflation
    }

    pub fn periods_in_year(&self) -> usize {
        self.periods_in_year
    }
}
