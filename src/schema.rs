table! {
    bets (bet_id) {
        bet_id -> Int4,
        bet_name -> Varchar,
        bet_description -> Nullable<Text>,
        bet_close -> Nullable<Timestamp>,
        created_on -> Timestamp,
    }
}

table! {
    outcomes (outcome_id) {
        outcome_id -> Int4,
        outcome_name -> Varchar,
        outcome_text -> Nullable<Text>,
        bet_id -> Int4,
        created_on -> Timestamp,
    }
}

table! {
    users (user_id) {
        user_id -> Int4,
        username -> Varchar,
        user_description -> Nullable<Text>,
        is_banned -> Bool,
        created_on -> Timestamp,
    }
}

table! {
    wagers (wager_id) {
        wager_id -> Int4,
        bet_id -> Int4,
        user_id -> Int4,
        outcome_id -> Int4,
        amount -> Int8,
        created_on -> Timestamp,
    }
}

joinable!(outcomes -> bets (bet_id));
joinable!(wagers -> bets (bet_id));
joinable!(wagers -> outcomes (outcome_id));
joinable!(wagers -> users (user_id));

allow_tables_to_appear_in_same_query!(
    bets,
    outcomes,
    users,
    wagers,
);
