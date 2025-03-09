INSERT INTO tasks (user_id, title, status)
VALUES ($1, $2, $3)
RETURNING *;