use chrono::prelude::{DateTime, Utc};

use super::participant::{Participant, Reputation};

#[derive(Debug)]
pub enum Status {
    Fisnished,
    Online,
    Live,
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum MoneyExchange {
    USD(i32),
    ARS(i32),
    BTC(i32),
}

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

#[derive(Debug)]
pub struct Auctions {
    pub state: Status,
    base_price: MoneyExchange,
    start_at: DateTime<Utc>,
    end_at: DateTime<Utc>,
    discoverable: bool,
    pub bid: Vec<Bid>,
    best_bid: Option<Bid>,
    min_reputation: Option<Reputation>,
}

impl Auctions {
    pub fn new(
        state: Status,
        base_price: MoneyExchange,
        start_at: DateTime<Utc>,
        end_at: DateTime<Utc>,
        discoverable: bool,
        min_reputation: Option<Reputation>,
    ) -> Auctions {
        Self {
            state: Status::Online,
            base_price: base_price,
            start_at: start_at,
            end_at: end_at,
            discoverable: false,
            bid: Vec::new(),
            best_bid: None,
            min_reputation: min_reputation,
        }
    }

    pub fn set_bid(&mut self, bid: Bid) {
        // should be a map_or_else
        match &self.best_bid {
            Some(b) => {
                if b.amount < bid.amount
                    && bid.participant.reputation >= self.min_reputation.unwrap()
                {
                    self.best_bid = Some(bid);
                }
                self.bid.push(bid);
            }
            None => {
                self.best_bid = Some(bid);
                self.bid.push(bid);
            }
        }
    }

    pub fn get_best_bid(&mut self) -> &Option<Bid> {
        &self.best_bid
    }

    pub fn set_fisinsh_status(&mut self) {
        self.state = Status::Fisnished;
    }
}
