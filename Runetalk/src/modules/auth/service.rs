use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use tracing::debug;

use crate::{
    app::AppState,
    common::response::ApiResponse,
    errors::AuthError,
    modules::auth::{
        dto::{LoginDto, RegisterDto, VerifyOtpDto},
        repository::AuthRepository,
    },
    utils::{hash::hash_password, smtp::send_otp},
};

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn register_service(&self, dto: RegisterDto) -> Result<ApiResponse>;
    async fn send_verification_otp(&self, email: &str) -> Result<ApiResponse>;
    async fn login_service(&self, dto: LoginDto) -> Result<ApiResponse>;
    async fn verification_otp(&self, dto: VerifyOtpDto) -> Result<ApiResponse>;
}

pub struct AuthServiceImpl {
    repo: Arc<dyn AuthRepository>,
    state: AppState,
}

impl AuthServiceImpl {
    pub fn new(repo: Arc<dyn AuthRepository>, state: AppState) -> Self {
        Self { repo, state }
    }
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    async fn register_service(&self, dto: RegisterDto) -> Result<ApiResponse> {
        debug!("Service : {:?}", dto);

        let email_user = self.repo.find_by_email(&dto.email).await?;
        if let Some(user) = email_user {
            if user.is_verified {
                return Err(AuthError::EmailAlreadyExists.into());
            }

            self.send_verification_otp(&user.email).await?;
            return Ok(ApiResponse::success_msg("OTP resent"));
        }

        let username_user = self.repo.find_by_username(&dto.username).await?;
        if let Some(user) = username_user {
            if user.is_verified {
                return Err(AuthError::UsernameAlreadyExists.into());
            }

            self.send_verification_otp(&user.email).await?;

            return Ok(ApiResponse::success_msg("OTP resent"));
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

    async fn send_verification_otp(&self, email: &str) -> Result<ApiResponse> {
        let otp = rand::random_range(100_000..1_000_000);

        self.repo.otp_redis(email, otp.to_string()).await?;

        send_otp(
            &self.state.config.smtp.email,
            &self.state.config.smtp.password,
            email,
            otp,
        )?;

        Ok(ApiResponse::success_msg("Berhasil mengirim OTP"))
    }

    async fn verification_otp(&self, dto: VerifyOtpDto) -> Result<ApiResponse> {
        let is_valid = self.repo.verify_otp(&dto.email, &dto.otp).await?;

        if !is_valid {
            return Err(AuthError::InvalidOtp.into());
        }

        Ok(ApiResponse::success_msg("Berhasil memverifikasi OTP"))
    }

    async fn login_service(&self, dto: LoginDto) -> Result<ApiResponse> {
        todo!()
    }
}
