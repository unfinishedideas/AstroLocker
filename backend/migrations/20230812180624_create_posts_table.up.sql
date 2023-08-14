CREATE TABLE IF NOT EXISTS posts
(
    id          serial PRIMARY KEY,
    title       VARCHAR(255) UNIQUE NOT NULL,
    explanation TEXT NOT NULL,
    img_url     VARCHAR(255) NOT NULL,
    user_id     integer REFERENCES users on DELETE CASCADE,
    created_on  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);