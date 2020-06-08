use super::chrono;
use super::schema::bets;

#[derive(Queryable, AsChangeset, Debug)]
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

#[derive(Queryable)]
pub struct Outcome {
    pub outcome_id: u32,
    pub outcome_name: String,
    pub outcome_text: Option<String>,
    pub bet_id: u32,
    pub created_on: chrono::NaiveDateTime,
}

#[derive(Queryable)]
pub struct User {
    pub user_id: u32,
    pub username: String,
    pub user_description: Option<String>,
    pub is_banned: bool,
    pub created_on: chrono::NaiveDateTime,
}

#[derive(Queryable)]
pub struct Wager {
    pub wager_id: u32,
    pub bet_id: u32,
    pub user_id: u32,
    pub outcome_id: u32,
    pub amount: u64,
    pub created_on: chrono::NaiveDateTime,
}

