table! {
    bets (bet_id) {
        bet_id -> Int4,
        bet_name -> Varchar,
        bet_description -> Nullable<Text>,
        bet_close -> Nullable<Timestamp>,
        created_on -> Timestamp,
    }
}
