use super::chrono;

#[derive(Queryable)]
pub struct Bet {
    pub bet_id: i32,
    pub bet_name: String,
    pub bet_description: Option<String>,
    pub bet_close: Option<chrono::NaiveDateTime>,
    pub created_on: chrono::NaiveDateTime,
}

#[derive(Queryable)]
pub struct Outcome {
    pub outcome_id: i32,
    pub outcome_name: String,
    pub outcome_text: Option<String>,
    pub bet_id: Option<i32>,
    pub created_on: chrono::NaiveDateTime,
}

#[derive(Queryable)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub user_description: Option<String>,
    pub is_banned: bool,
    pub created_on: chrono::NaiveDateTime,
}

#[derive(Queryable)]
pub struct Wager {
    pub wager_id: i32,
    pub bet_id: Option<i32>,
    pub user_id: Option<i32>,
    pub outcome_id: Option<i32>,
    pub amount: i64,
    pub created_on: chrono::NaiveDateTime,
}

