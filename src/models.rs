use super::chrono;
use super::schema::{bets, outcomes, users, wagers, winners};

#[derive(Identifiable, Queryable, AsChangeset, Debug)]
#[primary_key(bet_id)]
pub struct Bet {
    pub bet_id: i32,
    pub bet_name: String,
    pub bet_description: Option<String>,
    pub bet_close: Option<chrono::NaiveDateTime>,
    pub created_on: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="bets"]
pub struct NewBet<'a> {
    pub bet_name: &'a str,
    pub bet_description: Option<&'a str>,
    pub bet_close: Option<&'a chrono::NaiveDateTime>,
}

#[derive(Identifiable, Queryable, AsChangeset, Debug)]
#[primary_key(outcome_id)]
pub struct Outcome {
    pub outcome_id: i32,
    pub outcome_name: String,
    pub outcome_text: Option<String>,
    pub bet_id: i32,
    pub created_on: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="outcomes"]
pub struct NewOutcome<'a> {
    pub outcome_name: &'a str,
    pub outcome_text: Option<&'a str>,
    pub bet_id: &'a i32,
}

#[derive(Identifiable, Queryable, AsChangeset, Debug)]
#[primary_key(user_id)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub user_description: Option<String>,
    pub is_banned: bool,
    pub created_on: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub user_description: Option<&'a str>,
}

#[derive(Identifiable, Queryable, AsChangeset, Debug)]
#[primary_key(wager_id)]
pub struct Wager {
    pub wager_id: i32,
    pub bet_id: i32,
    pub user_id: i32,
    pub outcome_id: i32,
    pub amount: i64,
    pub created_on: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="wagers"]
pub struct NewWager<'a> {
    pub bet_id: &'a i32,
    pub user_id: &'a i32,
    pub outcome_id: &'a i32,
    pub amount: &'a i64,
}

#[derive(Queryable, Identifiable, AsChangeset, Debug)]
#[primary_key(id)]
pub struct Winner {
    pub id: i32,
    pub bet_id: i32,
    pub outcome_id: i32,
    pub created_on: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="winners"]
pub struct NewWinner<'a> {
    pub bet_id: &'a i32,
    pub outcome_id: &'a i32,
}
