use chrono::prelude::{DateTime, Utc};

use super::bid::Bid;
use super::money_exchange::MoneyExchange;
use super::participant::Reputation;

#[derive(Debug, PartialEq)]
pub enum Status {
    Fisnished,
    Online,
    Live,
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
            state: state,
            base_price: base_price,
            start_at: start_at,
            end_at: end_at,
            discoverable: discoverable,
            bid: Vec::new(),
            best_bid: None,
            min_reputation: min_reputation,
        }
    }

    pub fn set_bid(&mut self, bid: Bid) {
        // should be a map_or_else
        match &self.best_bid {
            Some(b) => {
                if bid.amount.is_higher(&b.amount)
                    && self.min_reputation.unwrap() <= bid.participant.reputation
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
