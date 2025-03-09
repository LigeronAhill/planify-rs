use super::Storage;
use crate::models::User;
use anyhow::Result;
use tracing::instrument;

impl Storage {
    #[instrument(skip(self))]
    pub async fn insert_user(&self, user: &User) -> Result<()> {
        sqlx::query_file!(
            "./storage/queries/users/insert.sql",
            user.id,
            user.first_name,
            user.last_name,
            user.username,
            user.is_bot
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
    #[instrument(skip(self))]
    pub async fn get_user(&self, id: i64) -> Result<Option<User>> {
        let result = sqlx::query_file_as!(User, "./storage/queries/users/get.sql", id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::User;
    use anyhow::Result;
    use fake::faker::internet::en::Username;
    use fake::faker::name::en::{FirstName, LastName};
    use fake::{Fake, Faker};

    fn fake_user() -> User {
        let id: i64 = Faker.fake();
        let first_name: String = FirstName().fake();
        let last_name: String = LastName().fake();
        let username: String = Username().fake();
        User {
            id,
            first_name,
            last_name: Some(last_name),
            username: Some(username.clone()),
            is_bot: false,
        }
    }
    #[tokio::test]
    async fn test_insert_user() -> Result<()> {
        let storage = Storage::new().await?;
        let user = fake_user();
        storage.insert_user(&user).await?;
        storage.close().await;
        Ok(())
    }
    #[tokio::test]
    async fn test_get_user() -> Result<()> {
        let storage = Storage::new().await?;
        let user = fake_user();
        storage.insert_user(&user).await?;
        let existing_user = storage.get_user(user.id).await?;
        assert!(existing_user.is_some());
        assert_eq!(user.username, existing_user.unwrap().username);
        storage.close().await;
        Ok(())
    }
}
