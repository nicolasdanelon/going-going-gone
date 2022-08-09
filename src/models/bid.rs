use super::money_exchange::MoneyExchange;
use super::participant::Participant;

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct Bid {
    pub amount: MoneyExchange,
    pub participant: Participant,
}

impl Bid {
    pub fn new(a: MoneyExchange, p: Participant) -> Bid {
        Self {
            amount: a,
            participant: p,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::models::participant;

    use super::*;

    #[test]
    fn test_bid_is_higher() {
        let money1 = MoneyExchange::BTC(20);
        let money2 = MoneyExchange::BTC(15);
        let participant1: Participant = Participant::new(60);

        let bid1: Bid = Bid::new(money1, participant1);

        assert_eq!(bid1.amount.is_higher(&money2), true);
    }

    #[test]
    fn test_bid_is_equal() {
        let money1 = MoneyExchange::BTC(112);
        let money2 = MoneyExchange::BTC(112);
        let participant1: Participant = Participant::new(60);

        let bid1: Bid = Bid::new(money1, participant1);

        assert_eq!(bid1.amount.is_equal(&money2), true);
    }

    #[test]
    fn test_bid_has_participant() {
        let money1 = MoneyExchange::BTC(1);
        let participant1: Participant = Participant::new(60);

        let bid1: Bid = Bid::new(money1, participant1);

        assert_eq!(
            bid1.participant,
            participant::Participant { reputation: 60 }
        );
    }
}
