use crate::models::*;
use crate::db::FTDB;
use diesel::prelude::*;

impl FTDB {
    pub fn get_bet_by_id(&self, id: i32) -> QueryResult<Bet> {
        use crate::schema::bets::dsl::*;
        bets.find(id)
            .first(&self.conn)
    }

    pub fn get_bet_by_name(&self, name: &str) -> QueryResult<Bet> {
        use crate::schema::bets::dsl::*;
        bets.filter(bet_name.eq(name))
            .first(&self.conn)
    }

}

pub fn print_bet(bet: &Bet) {
    println!("Bet Name:\n{}\n", bet.bet_name);
    match &bet.bet_description {
        Some(desc) => println!("Bet Description:\n{}\n", desc),
        None => println!("No Bet Description"),
    };
    match &bet.bet_close {
        Some(close) => println!("Bet Close: {}", close),
        None => println!("No bet close date set"),
    };
    println!("-----------------------------");
}

pub fn is_bet_closed(bet: &Bet) -> bool {
    let now : = chrono::Utc::now().naive_utc();
    match &bet.bet_close {
        Some(close) => {
            if now < *close {
                true
            } else {
                false
            }
        },
        None => false,
    }
}
