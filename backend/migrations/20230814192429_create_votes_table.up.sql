CREATE TABLE IF NOT EXISTS votes
(
    id          serial PRIMARY KEY,
    user_id     INTEGER REFERENCES users ON DELETE CASCADE NOT NULL,
    post_id     INTEGER REFERENCES posts ON DELETE CASCADE NOT NULL,
    created_on  TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT unique_combinations UNIQUE(user_id, post_id)
);