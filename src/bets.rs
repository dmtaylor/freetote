use crate::models::{Bet, NewBet};
use crate::db::FTDB;
use crate::models::*;
use diesel::prelude::*;

impl FTDB {
    pub fn get_bet_by_id(&self, id: i32) -> QueryResult<Bet> {
        use crate::schema::bets::dsl::*;
        bets.find(id).first(&self.conn)
    }

    pub fn get_bet_by_name(&self, name: &str) -> QueryResult<Bet> {
        use crate::schema::bets::dsl::*;
        bets.filter(bet_name.eq(name)).first(&self.conn)
    }
    
    pub fn new_bet<'a>(&self,
        name: &'a str,
        desc: Option<&'a str>,
        close: Option<&'a chrono::NaiveDateTime>)
        -> QueryResult<Bet>
    {
        use crate::schema::bets;
        let new_bet = NewBet {
            bet_name: name,
            bet_description: desc,
            bet_close: close,
        };
        diesel::insert_into(bets::table)
            .values(&new_bet)
            .get_result(&self.conn)
    }

}

impl Bet {
    pub fn print_bet(&self) {
        println!("Bet Name:\n{}\n", &self.bet_name);
        match &self.bet_description {
            Some(desc) => println!("Bet Description:\n{}\n", desc),
            None => println!("No Bet Description"),
        };
        match &self.bet_close {
            Some(close) => println!("Bet Close: {}", close),
            None => println!("No bet close date set"),
        };
        println!("-----------------------------");
    }

    pub fn is_closed(&self) -> bool {
        let now = chrono::Utc::now().naive_utc();
        match &self.bet_close {
            Some(close) => {
                if now < *close {
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }
}
