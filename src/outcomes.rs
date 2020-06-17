use crate::models::{Outcome, NewOutcome};
use crate::db::FTDB;
use crate::models::*;
use diesel::prelude::*;

impl FTDB {

    pub fn new_wager<'a> (
        &self,
        outcome_name: &'a str,
        outcome_text: Option<&'a str>,
        bet_id: &'a i32
    ) -> QueryResult<Outcome> {
        use crate::schema::outcomes;
        let new_outcome = NewOutcome {
            outcome_name,
            outcome_text,
            bet_id,
        };
        diesel::insert_into(outcomes::table)
            .values(&new_outcome)
            .get_result(&self.conn)
    }

    pub fn find_outcome_by_name(&self, name: &str) -> QueryResult<Outcome> {
        use crate::schema::outcomes::dsl::*;
        outcomes.filter(outcome_name.eq(name)).first(&self.conn)
    }
}
