use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MarketConditions {
    // Inflation
    inflation: f64,
}

impl MarketConditions {
    pub fn inflation_yearly(&self) -> f64 {
        self.inflation
    }
}
