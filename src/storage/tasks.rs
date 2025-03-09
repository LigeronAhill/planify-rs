use crate::models::{Task, TaskStatus};
use crate::Storage;
use anyhow::Result;
use tracing::instrument;

impl Storage {
    #[instrument(skip(self))]
    pub async fn create_task(&self, task: &Task) -> Result<Task> {
        let status = task.status.to_string();
        let created = sqlx::query_file_as!(
            Task,
            "storage/queries/tasks/create.sql",
            task.user_id,
            task.title,
            status,
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(created)
    }
    #[instrument(skip(self))]
    pub async fn get_all_tasks(&self, user_id: i64) -> Result<Vec<Task>> {
        let result = sqlx::query_file_as!(Task, "storage/queries/tasks/get_all.sql", user_id)
            .fetch_all(&self.pool)
            .await?;
        Ok(result)
    }
    #[instrument(skip(self))]
    pub async fn update_task_status(&self, task_id: i64, status: TaskStatus, user_id: i64) -> Result<Task> {
        let status = status.to_string();
        let result = sqlx::query_file_as!(
            Task,
            "storage/queries/tasks/update_status.sql",
            status,
            task_id,
            user_id,
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(result)
    }
    #[instrument(skip(self))]
    pub async fn get_completed_tasks(&self, user_id: i64) -> Result<Vec<Task>> {
        let result =
            sqlx::query_file_as!(Task, "storage/queries/tasks/get_completed.sql", user_id)
                .fetch_all(&self.pool)
                .await?;
        Ok(result)
    }
}
