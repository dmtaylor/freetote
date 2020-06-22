use crate::models::{Winner, NewWinner, Bet};
use crate::db::FTDB;
use crate::models::*;
use crate::bets::*;

use diesel::prelude::*;
use anyhow::{Context, Result};
use thiserror::Error;
use chrono::{NaiveDateTime, Utc};

#[derive(Error, Debug)]
pub enum WinnerError {
    #[error("bet closed on `{0}`, cannot add winner")]
    ClosedBet(chrono::NaiveDateTime),
    #[error("Bet close is empty when expected non-empty")]
    EmptyBetClose,
}

impl FTDB {
    pub fn new_winner<'a> (
        &self,
        bet_id: &'a i32,
        outcome_id: &'a i32
    ) -> Result<Winner> {
        use crate::schema::winners;
        use crate::schema::bets;

        let mut bet: Bet = crate::schema::bets::dsl::bets.find(bet_id).
            first(&self.conn)
            .with_context(|| format!("Failed to find bet_id {}", bet_id))?;

        if bet.is_closed() {
            match bet.bet_close {
                Some(close) => return Err(WinnerError::ClosedBet(close).into()),
                None => return Err(WinnerError::EmptyBetClose.into()),
            }
        }

        let new_winner = NewWinner {
            bet_id,
            outcome_id,
        };
        let result = diesel::insert_into(winners::table)
            .values(&new_winner)
            .get_result(&self.conn)?;
        
        // set bet close here
        diesel::update(&bet)
            .set(crate::schema::bets::dsl::bet_close.eq(Utc::now().naive_utc()))
            .execute(&self.conn)?;

        Ok(result)
    }
}
