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
