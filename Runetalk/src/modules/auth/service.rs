use std::sync::Arc;

use async_trait::async_trait;
use tracing::debug;

use crate::{
    common::response::ApiResponse,
    errors::{AppResult, AuthError},
    model::config_model::ConfigModel,
    modules::auth::{
        dto::{LoginDto, RegisterDto, VerifyOtpDto},
        repository::AuthRepository,
    },
    utils::{
        hash::{hash_password, verif_password},
        jwt::generate_jwt,
        smtp::send_otp,
    },
};

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn register_service(&self, dto: RegisterDto) -> AppResult<ApiResponse>;
    async fn send_verification_otp(&self, email: &str) -> AppResult<ApiResponse>;
    async fn verification_otp(&self, dto: VerifyOtpDto) -> AppResult<ApiResponse>;
    async fn login_service(&self, dto: LoginDto) -> AppResult<String>;
}

pub struct AuthServiceImpl {
    repo: Arc<dyn AuthRepository>,
    config: Arc<ConfigModel>,
}

impl AuthServiceImpl {
    pub fn new(repo: Arc<dyn AuthRepository>, config: Arc<ConfigModel>) -> Self {
        Self { repo, config }
    }
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    async fn register_service(&self, dto: RegisterDto) -> AppResult<ApiResponse> {
        debug!("Service : {:?}", dto);

        let email_user = self.repo.find_by_email(&dto.email).await?;
        if let Some(user) = email_user {
            if user.is_verified {
                return Err(AuthError::EmailAlreadyExists(user.email).into());
            }

            self.send_verification_otp(&user.email).await?;
            return Ok(ApiResponse::success_msg("OTP resent"));
        }

        let username_user = self.repo.find_by_username(&dto.username).await?;
        if username_user.is_some() {
            return Err(AuthError::UsernameAlreadyExists.into());
        }

        let hash = hash_password(&dto.password)?;
        debug!(hash);

        let to_email = dto.email.clone();

        let user = RegisterDto {
            username: dto.username,
            email: dto.email,
            password: hash,
        };

        debug!("{:?}", user);

        self.repo.save_adventurer(user).await?;

        self.send_verification_otp(&to_email).await?;

        Ok(ApiResponse::success_msg("Berhasil mendaftar"))
    }

    async fn send_verification_otp(&self, email: &str) -> AppResult<ApiResponse> {
        let otp = rand::random_range(100_000..1_000_000);

        self.repo.otp_redis(email, otp.to_string()).await?;

        send_otp(
            &self.config.smtp.email,
            &self.config.smtp.password,
            email,
            otp,
        )?;

        Ok(ApiResponse::success_msg("Berhasil mengirim OTP"))
    }

    async fn verification_otp(&self, dto: VerifyOtpDto) -> AppResult<ApiResponse> {
        let is_valid = self.repo.verify_otp(&dto.email, &dto.otp).await?;

        if !is_valid {
            return Err(AuthError::InvalidOtp.into());
        }

        Ok(ApiResponse::success_msg("Berhasil memverifikasi OTP"))
    }

    async fn login_service(&self, dto: LoginDto) -> AppResult<String> {
        let user = self
            .repo
            .login(&dto.identifier)
            .await?
            .ok_or(AuthError::NotFound)?;
        debug!("{:?}", user);

        if !user.is_verified {
            return Err(AuthError::NotVerified(user.email).into());
        }

        if !verif_password(&dto.password, &user.password)? {
            return Err(AuthError::InvalidPassword.into());
        }

        let token = generate_jwt(
            user.id.clone().to_string(),
            user.email.clone(),
            &self.config,
        )?;

        debug!("token: {}", token);

        Ok(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Adventurer;
    use crate::model::config_model::{
        ApiConfig, AppConfig, ConfigModel, DbConfig, JwtConfig, RedisConfig, Smtp, Storage,
    };
    use crate::modules::auth::dto::RegisterDto;
    use crate::modules::auth::repository::MockAuthRepository;
    use chrono::Utc;
    use mockall::predicate::*;
    use uuid::Uuid;

    fn dummy_config() -> Arc<ConfigModel> {
        Arc::new(ConfigModel {
            app: AppConfig {
                name: "Test".to_string(),
                host: "localhost".to_string(),
                port: 8080,
            },
            db: DbConfig {
                host: "".to_string(),
                port: 0,
                username: "".to_string(),
                password: "".to_string(),
                name: "".to_string(),
                ssl_mode: "".to_string(),
            },
            redis: RedisConfig {
                host: "".to_string(),
                port: 0,
                username: "".to_string(),
                password: "".to_string(),
            },
            jwt: JwtConfig {
                secret: "test_secret_key_long_enough_for_jwt".to_string(),
                expiry: 3600,
            },
            api: ApiConfig {
                secret: "".to_string(),
            },
            smtp: Smtp {
                email: "smtp@example.com".to_string(),
                password: "password".to_string(),
            },
            storage: Storage {
                path: "./public/user".to_string(),
            },
            allowed_origins: vec![],
        })
    }

    fn dummy_adventurer(username: &str, email: &str, is_verified: bool) -> Adventurer {
        Adventurer {
            id: Uuid::new_v4(),
            username: username.to_string(),
            display_name: None,
            email: email.to_string(),
            password: crate::utils::hash::hash_password("password123").unwrap(),
            avatar_url: None,
            banner_url: None,
            bio: None,
            is_verified,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }

    #[tokio::test]
    async fn test_register_success() {
        let mut mock_repo = MockAuthRepository::new();

        mock_repo
            .expect_find_by_email()
            .with(eq("new@example.com"))
            .times(1)
            .returning(|_| Ok(None));

        mock_repo
            .expect_find_by_username()
            .with(eq("newuser"))
            .times(1)
            .returning(|_| Ok(None));

        mock_repo
            .expect_save_adventurer()
            .times(1)
            .returning(|_| Ok(()));

        mock_repo
            .expect_otp_redis()
            .with(eq("new@example.com"), always())
            .times(1)
            .returning(|_, _| Ok(()));

        let service = AuthServiceImpl::new(Arc::new(mock_repo), dummy_config());
        let dto = RegisterDto {
            username: "newuser".to_string(),
            email: "new@example.com".to_string(),
            password: "password123".to_string(),
        };

        let res = service.register_service(dto).await.unwrap();
        assert!(res.success);
        assert_eq!(res.message, "Berhasil mendaftar");
    }

    #[tokio::test]
    async fn test_register_email_already_exists_verified() {
        let mut mock_repo = MockAuthRepository::new();
        let existing = dummy_adventurer("existing", "existing@example.com", true);

        mock_repo
            .expect_find_by_email()
            .with(eq("existing@example.com"))
            .times(1)
            .returning(move |_| Ok(Some(existing.clone())));

        let service = AuthServiceImpl::new(Arc::new(mock_repo), dummy_config());
        let dto = RegisterDto {
            username: "newuser".to_string(),
            email: "existing@example.com".to_string(),
            password: "password123".to_string(),
        };

        let err = service.register_service(dto).await.unwrap_err();
        assert!(matches!(
            err,
            crate::errors::AppError::Auth(AuthError::EmailAlreadyExists(_))
        ));
    }

    #[tokio::test]
    async fn test_register_email_already_exists_unverified() {
        let mut mock_repo = MockAuthRepository::new();
        let existing = dummy_adventurer("existing", "existing@example.com", false);

        mock_repo
            .expect_find_by_email()
            .with(eq("existing@example.com"))
            .times(1)
            .returning(move |_| Ok(Some(existing.clone())));

        mock_repo
            .expect_otp_redis()
            .with(eq("existing@example.com"), always())
            .times(1)
            .returning(|_, _| Ok(()));

        let service = AuthServiceImpl::new(Arc::new(mock_repo), dummy_config());
        let dto = RegisterDto {
            username: "existing".to_string(),
            email: "existing@example.com".to_string(),
            password: "password123".to_string(),
        };

        let res = service.register_service(dto).await.unwrap();
        assert!(res.success);
        assert_eq!(res.message, "OTP resent");
    }

    #[tokio::test]
    async fn test_register_username_already_exists() {
        let mut mock_repo = MockAuthRepository::new();
        let existing = dummy_adventurer("existing_user", "existing@example.com", true);

        mock_repo
            .expect_find_by_email()
            .with(eq("new@example.com"))
            .times(1)
            .returning(|_| Ok(None));

        mock_repo
            .expect_find_by_username()
            .with(eq("existing_user"))
            .times(1)
            .returning(move |_| Ok(Some(existing.clone())));

        let service = AuthServiceImpl::new(Arc::new(mock_repo), dummy_config());
        let dto = RegisterDto {
            username: "existing_user".to_string(),
            email: "new@example.com".to_string(),
            password: "password123".to_string(),
        };

        let err = service.register_service(dto).await.unwrap_err();
        assert!(matches!(
            err,
            crate::errors::AppError::Auth(AuthError::UsernameAlreadyExists)
        ));
    }

    #[tokio::test]
    async fn test_verification_otp_success() {
        let mut mock_repo = MockAuthRepository::new();
        mock_repo
            .expect_verify_otp()
            .with(eq("user@example.com"), eq("123456"))
            .times(1)
            .returning(|_, _| Ok(true));

        let service = AuthServiceImpl::new(Arc::new(mock_repo), dummy_config());
        let dto = VerifyOtpDto {
            email: "user@example.com".to_string(),
            otp: "123456".to_string(),
        };

        let res = service.verification_otp(dto).await.unwrap();
        assert!(res.success);
        assert_eq!(res.message, "Berhasil memverifikasi OTP");
    }

    #[tokio::test]
    async fn test_verification_otp_invalid() {
        let mut mock_repo = MockAuthRepository::new();
        mock_repo
            .expect_verify_otp()
            .with(eq("user@example.com"), eq("123456"))
            .times(1)
            .returning(|_, _| Ok(false));

        let service = AuthServiceImpl::new(Arc::new(mock_repo), dummy_config());
        let dto = VerifyOtpDto {
            email: "user@example.com".to_string(),
            otp: "123456".to_string(),
        };

        let err = service.verification_otp(dto).await.unwrap_err();
        assert!(matches!(
            err,
            crate::errors::AppError::Auth(AuthError::InvalidOtp)
        ));
    }

    #[tokio::test]
    async fn test_login_success() {
        let mut mock_repo = MockAuthRepository::new();
        let user = dummy_adventurer("user1", "user1@example.com", true);

        mock_repo
            .expect_login()
            .with(eq("user1"))
            .times(1)
            .returning(move |_| Ok(Some(user.clone())));

        let service = AuthServiceImpl::new(Arc::new(mock_repo), dummy_config());
        let dto = LoginDto {
            identifier: "user1".to_string(),
            password: "password123".to_string(),
        };

        let token = service.login_service(dto).await.unwrap();
        assert!(!token.is_empty());
    }

    #[tokio::test]
    async fn test_login_not_found() {
        let mut mock_repo = MockAuthRepository::new();
        mock_repo
            .expect_login()
            .with(eq("nonexistent"))
            .times(1)
            .returning(|_| Ok(None));

        let service = AuthServiceImpl::new(Arc::new(mock_repo), dummy_config());
        let dto = LoginDto {
            identifier: "nonexistent".to_string(),
            password: "password123".to_string(),
        };

        let err = service.login_service(dto).await.unwrap_err();
        assert!(matches!(
            err,
            crate::errors::AppError::Auth(AuthError::NotFound)
        ));
    }

    #[tokio::test]
    async fn test_login_not_verified() {
        let mut mock_repo = MockAuthRepository::new();
        let user = dummy_adventurer("unverified", "unverified@example.com", false);

        mock_repo
            .expect_login()
            .with(eq("unverified"))
            .times(1)
            .returning(move |_| Ok(Some(user.clone())));

        let service = AuthServiceImpl::new(Arc::new(mock_repo), dummy_config());
        let dto = LoginDto {
            identifier: "unverified".to_string(),
            password: "password123".to_string(),
        };

        let err = service.login_service(dto).await.unwrap_err();
        assert!(matches!(
            err,
            crate::errors::AppError::Auth(AuthError::NotVerified(_))
        ));
    }

    #[tokio::test]
    async fn test_login_invalid_password() {
        let mut mock_repo = MockAuthRepository::new();
        let user = dummy_adventurer("user1", "user1@example.com", true);

        mock_repo
            .expect_login()
            .with(eq("user1"))
            .times(1)
            .returning(move |_| Ok(Some(user.clone())));

        let service = AuthServiceImpl::new(Arc::new(mock_repo), dummy_config());
        let dto = LoginDto {
            identifier: "user1".to_string(),
            password: "wrongpassword".to_string(),
        };

        let err = service.login_service(dto).await.unwrap_err();
        assert!(matches!(
            err,
            crate::errors::AppError::Auth(AuthError::InvalidPassword)
        ));
    }
}
