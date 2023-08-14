CREATE TABLE IF NOT EXISTS admins
(
    id                 serial PRIMARY KEY,
    admin_user_id      integer REFERENCES users ON DELETE CASCADE
);