-- create users

CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(127) NOT NULL,
    user_description TEXT,
    is_banned BOOLEAN NOT NULL DEFAULT FALSE,
    created_on TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX username_idx ON users (username);
