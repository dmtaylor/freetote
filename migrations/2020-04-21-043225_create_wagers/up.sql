-- Create wagers

CREATE TABLE wagers (
    wager_id SERIAL PRIMARY KEY,
    bet_id INTEGER REFERENCES bets(bet_id),
    user_id INTEGER REFERENCES users(user_id),
    outcome_id INTEGER REFERENCES outcomes(outcome_id),
    amount BIGINT NOT NULL,
    created_on TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX wagers_bet_id_idx ON wagers(bet_id);
CREATE INDEX wagers_user_id_idx ON wagers(user_id);
CREATE INDEX wagers_outcome_id_idx ON wagers(outcome_id);
