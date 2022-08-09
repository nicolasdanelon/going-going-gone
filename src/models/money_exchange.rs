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

    pub fn is_equal(&self, other: &MoneyExchange) -> bool {
        self.get_amount() == other.get_amount()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bid_is_higher() {
        let money1 = MoneyExchange::BTC(20);
        let money2 = MoneyExchange::BTC(14);

        assert_eq!(money1.is_higher(&money2), true);
    }

    #[test]
    fn test_bid_is_equal() {
        let money1 = MoneyExchange::BTC(11);
        let money2 = MoneyExchange::BTC(11);

        assert_eq!(money1.is_equal(&money2), true);
    }
}
