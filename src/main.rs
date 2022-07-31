mod models;

use chrono::prelude::{DateTime, Utc};
use chrono::Duration;
use models::auction::{Auctions, Bid, MoneyExchange, Status};
use models::participant::Participant;

fn main() {
    let money1 = MoneyExchange::BTC(1);
    let money2 = MoneyExchange::ARS(2);
    let money3 = MoneyExchange::USD(3);
    let now: DateTime<Utc> = Utc::now();
    let end: DateTime<Utc> = now + Duration::days(6);
    let mut auction: Auctions = Auctions::new(Status::Online, money1, now, end, false, Some(80));
    let participant1: Participant = Participant::new(60);
    let participant2: Participant = Participant::new(70);
    let participant3: Participant = Participant::new(90);
    let bid1: Bid = Bid::new(money1, participant1);
    let bid2: Bid = Bid::new(money2, participant2);
    let bid3: Bid = Bid::new(money3, participant3);

    auction.set_bid(bid1);
    auction.set_bid(bid2);
    auction.set_bid(bid3);

    let g_bid = auction.get_best_bid();

    println!("{:#?}", g_bid);

    auction.set_fisinsh_status();

    if bid1 == bid2 {
        println!("Nico, los bid son iguales");
    } else {
        println!("Nico, los bid no son iguales");
    }

    assert_ne!(bid1, bid2);

    if auction.state != Status::Live {
        println!("{:#?}", auction.state);
        println!("{:#?}", auction.bid);
    }
}
