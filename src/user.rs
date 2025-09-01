use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    name: String,
    birth_month: usize,
    birth_year: usize,
    max_age: f64,
    yearly_expenses: f64,
    buffer: f64,
    periods_in_year: usize,
    input_file: String,
}

impl User {
    pub fn new(name: String, birth_month: usize, birth_year: usize) -> Self {
        User {
            name,
            birth_month,
            birth_year,
            max_age: 100.0,
            yearly_expenses: 0.0,
            buffer: 1.0,
            periods_in_year: 12,
            input_file: String::from("Income_Assets.csv"),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn current_age(&self) -> f64 {
        let now = chrono::Utc::now().date_naive();
        let birth =
            chrono::NaiveDate::from_ymd_opt(self.birth_year as i32, self.birth_month as u32, 1);
        let duration = now - birth.unwrap();
        let days = duration.num_days();

        days as f64 / 365.0
    }

    pub fn max_age(&self) -> f64 {
        self.max_age
    }

    pub fn yearly_expenses(&self) -> f64 {
        self.yearly_expenses
    }

    pub(crate) fn periods_in_year(&self) -> usize {
        self.periods_in_year
    }

    pub(crate) fn buffer(&self) -> f64 {
        self.buffer
    }

    pub(crate) fn input_file(&self) -> &str {
        &self.input_file
    }

    pub(crate) fn length_of_retirement(&self) -> f64 {
        self.max_age() - self.current_age()
    }

    pub(crate) fn total_periods_of_retirement(&self) -> usize {
        self.length_of_retirement() as usize * self.periods_in_year()
    }
}

#[cfg(test)]
mod tests {
    use crate::user::User;

    #[test]
    fn test_current_age() {
        let test_user = User::new(String::from("Leher"), 12, 1975);
        assert!((test_user.current_age() - 49.10).abs() < 0.9);
        let test_user = User::new(String::from("D"), 2, 1963);
        assert!((test_user.current_age() - 62.0).abs() < 0.9);
    }
}
