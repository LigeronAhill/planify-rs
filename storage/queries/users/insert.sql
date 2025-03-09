INSERT INTO users (id, first_name, last_name, username, is_bot)
VALUES ($1, $2, $3, $4, $5)
ON CONFLICT (id) DO UPDATE SET first_name = excluded.first_name,
                               last_name = excluded.last_name,
                               username = excluded.username,
                               is_bot = excluded.is_bot;