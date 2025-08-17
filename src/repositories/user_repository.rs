use async_trait::async_trait;
use sqlx::query_as;
use uuid::Uuid;

use crate::{db::DBClient, models::user::User};

#[async_trait]
pub trait UserRepository {
    async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>, sqlx::Error>;
    async fn find_all(&self, page: u32, limit: usize) -> Result<Vec<User>, sqlx::Error>;
    async fn save_user(&self, user: User) -> Result<User, sqlx::Error>;
    async fn update_user(
        &self,
        user_id: uuid::Uuid,
        new_user_info: User,
    ) -> Result<User, sqlx::Error>;

    async fn delete_user(&self, user_id: uuid::Uuid) -> Result<(), sqlx::Error>;
}

#[async_trait]
impl UserRepository for DBClient {
    async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
        let option_user: Option<User> =
            query_as!(User, r#"SELECT id, username, email, password, birth_date from "user" WHERE id = $1"#, user_id)
                .fetch_optional(&self.pool)
                .await?;
        return Ok(option_user);
    }

    async fn find_all(&self, page: u32, limit: usize) -> Result<Vec<User>, sqlx::Error> {
        let offset = (page - 1) * limit as u32;
        let users = sqlx::query_as!(
            User,
            r#"SELECT id, username, email, password, birth_date FROM "user" ORDER BY username DESC LIMIT $1 OFFSET $2"#,
            limit as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await?;

        return Ok(users);
    }

    async fn save_user(&self, user: User) -> Result<User, sqlx::Error> {
        let new_user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO "user" (username, email, password, birth_date) VALUES ($1, $2, $3, $4)
            RETURNING id, username, email, password, birth_date
            "#,
            user.username,
            user.email,
            user.password,
            user.birth_date
        )
        .fetch_one(&self.pool)
        .await?;

        return Ok(new_user);
    }

    async fn update_user(
        &self,
        user_id: uuid::Uuid,
        new_user_info: User,
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE "user" SET username = $1, email = $2, password = $3, birth_date = $4 WHERE id = $5
            RETURNING id, username, email, password, birth_date
            "#,
            new_user_info.username, new_user_info.email, new_user_info.password, new_user_info.birth_date, user_id 
        ).fetch_one(&self.pool).await?;

        return Ok(user);
    }

    async fn delete_user(&self, user_id: uuid::Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"DELETE FROM "user" WHERE id = $1"#,
            user_id
        ).fetch_one(&self.pool).await?;
        return Ok(());
    }
}
