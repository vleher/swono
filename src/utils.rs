use log::debug;

pub fn calculate_compound(initial: f64, rate: f64, period: f64) -> f64 {
    debug!("Calculating Compound: Initial: {initial} Rate: {rate} period: {period}");
    initial * (1.0 + (rate)).powf(period)
}

pub fn calculate_principal(required_value: f64, rate: f64, period: f64) -> f64 {
    debug!("Calculating Principal: Required Value: {required_value} Rate: {rate} period: {period}");
    required_value / (1.0 + (rate)).powf(period)
}
