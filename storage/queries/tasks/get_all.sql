SELECT * FROM tasks WHERE user_id = $1 AND NOT status = 'Выполнена';