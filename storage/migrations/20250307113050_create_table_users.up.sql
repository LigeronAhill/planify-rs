CREATE TABLE IF NOT EXISTS users
(
    id INTEGER PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT,
    username TEXT,
    is_bot BOOLEAN NOT NULL DEFAULT false
)