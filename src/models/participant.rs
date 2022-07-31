pub type Reputation = i8;

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct Participant {
    pub reputation: Reputation,
}

impl Participant {
    pub fn new(r: Reputation) -> Self {
        Self { reputation: r }
    }
}
