use crate::{
    app::AppState, entity::Adventurer, errors::AppResult, modules::auth::dto::RegisterDto,
};
use async_trait::async_trait;
use mockall::automock;
use redis::AsyncCommands;
use tracing::debug;

#[automock]
#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn find_by_username(&self, username: &str) -> AppResult<Option<Adventurer>>;
    async fn find_by_email(&self, email: &str) -> AppResult<Option<Adventurer>>;
    async fn save_adventurer(&self, user: RegisterDto) -> AppResult<()>;
    async fn otp_redis(&self, email: &str, otp: String) -> AppResult<()>;
    async fn verify_otp(&self, email: &str, otp: &str) -> AppResult<bool>;
    async fn login(&self, identifier: &str) -> AppResult<Option<Adventurer>>;
}

pub struct AuthRepositoryImpl {
    pub state: AppState,
}

#[async_trait]
impl AuthRepository for AuthRepositoryImpl {
    async fn find_by_username(&self, username: &str) -> AppResult<Option<Adventurer>> {
        debug!("Repo : {}", username);
        Ok(
            sqlx::query_as::<_, Adventurer>("select * from adventurers where username = $1")
                .bind(username)
                .fetch_optional(self.state.db.as_ref())
                .await?,
        )
    }

    async fn find_by_email(&self, email: &str) -> AppResult<Option<Adventurer>> {
        debug!("Repo : {}", email);
        Ok(
            sqlx::query_as::<_, Adventurer>("select * from adventurers where email = $1")
                .bind(email)
                .fetch_optional(self.state.db.as_ref())
                .await?,
        )
    }

    async fn save_adventurer(&self, user: RegisterDto) -> AppResult<()> {
        sqlx::query("INSERT INTO adventurers (username,email,password) VALUES ($1,$2,$3)")
            .bind(user.username)
            .bind(user.email)
            .bind(user.password)
            .execute(self.state.db.as_ref())
            .await?;
        debug!("saved");
        Ok(())
    }

    async fn otp_redis(&self, email: &str, otp: String) -> AppResult<()> {
        let mut conn = (*self.state.redis).clone();

        let _: () = conn.set_ex(format!("otp:{}", email), otp, 60).await?;

        debug!("saved otp");
        Ok(())
    }

    async fn verify_otp(&self, email: &str, otp: &str) -> AppResult<bool> {
        let mut conn = (*self.state.redis).clone();

        let stored_otp: Option<String> = conn.get(format!("otp:{}", email)).await?;

        let is_valid = match stored_otp {
            Some(redis_otp) => redis_otp == otp,
            None => false,
        };

        if is_valid {
            let _: () = conn.del(format!("otp:{}", email)).await?;
            sqlx::query(
                "UPDATE adventurers
                 SET is_verified = true
                 WHERE email = $1",
            )
            .bind(email)
            .execute(self.state.db.as_ref())
            .await?;
        }

        Ok(is_valid)
    }

    async fn login(&self, identifier: &str) -> AppResult<Option<Adventurer>> {
        Ok(sqlx::query_as::<_, Adventurer>(
            "select * from adventurers where username = $1 or email = $1",
        )
        .bind(identifier)
        .fetch_optional(self.state.db.as_ref())
        .await?)
    }
}
