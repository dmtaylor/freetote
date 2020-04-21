-- create outcomes

CREATE TABLE outcomes (
    outcome_id SERIAL PRIMARY KEY,
    outcome_name VARCHAR(127) NOT NULL,
    outcome_text TEXT,
    bet_id INTEGER REFERENCES bets(bet_id),
    created_on TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX outcome_name_idx ON outcomes(outcome_name);
