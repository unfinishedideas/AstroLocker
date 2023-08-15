CREATE TABLE IF NOT EXISTS posts
(
    id              serial PRIMARY KEY,
    created_on      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    title           VARCHAR(255) UNIQUE NOT NULL,
    explanation     TEXT NOT NULL,
    query_string    TEXT NOT NULL,
    img_url         TEXT NOT NULL,
    apod_date       TEXT NOT NULL
);