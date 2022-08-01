#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum MoneyExchange {
    USD(i32),
    ARS(i32),
    BTC(i32),
}

impl MoneyExchange {
    pub fn get_amount(&self) -> i32 {
        match self {
            MoneyExchange::USD(value) => *value,
            MoneyExchange::ARS(value) => *value,
            MoneyExchange::BTC(value) => *value,
        }
    }

    pub fn is_higher(&self, other: &MoneyExchange) -> bool {
        self.get_amount() > other.get_amount()
    }
}
