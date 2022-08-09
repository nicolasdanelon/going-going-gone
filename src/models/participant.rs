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

#[cfg(test)]
mod test {
    use crate::models::participant;

    use super::*;

    #[test]
    fn test_bid_has_participant() {
        let participant1: Participant = Participant::new(60);

        assert_eq!(participant1, participant::Participant { reputation: 60 });
    }
}
