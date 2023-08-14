CREATE TABLE IF NOT EXISTS posts
(
    id          serial PRIMARY KEY,
    created_on  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    title       VARCHAR(255) UNIQUE NOT NULL,
    explanation TEXT NOT NULL,
    img_url     VARCHAR(255) NOT NULL,
    user_id     INTEGER REFERENCES users ON DELETE CASCADE
);