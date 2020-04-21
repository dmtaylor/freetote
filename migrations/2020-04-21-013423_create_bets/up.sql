-- Create Bets table

CREATE TABLE bets (
    bet_id SERIAL PRIMARY KEY,
    bet_name VARCHAR(127) NOT NULL,
    bet_description TEXT,
    bet_close TIMESTAMP,
    created_on TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX bet_name_idx ON bets (bet_name);
