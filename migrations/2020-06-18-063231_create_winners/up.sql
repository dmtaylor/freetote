-- create table tracking winners

CREATE TABLE winners (
    id SERIAL PRIMARY KEY,
    bet_id INTEGER UNIQUE NOT NULL REFERENCES bets(bet_id),
    outcome_id INTEGER UNIQUE NOT NULL REFERENCES outcomes(outcome_id),
    created_on TIMESTAMP NOT NULL DEFAULT NOW()
);
