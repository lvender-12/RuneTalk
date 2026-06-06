use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct RegisterDto {
    #[validate(length(min = 4, message = "Username minimal harus 4 karakter"))]
    pub username: String,

    #[validate(email(message = "Format email tidak valid"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password minimal harus 8 karakter"))]
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct VerifyOtpDto {
    #[validate(email(message = "Format email tidak valid"))]
    pub email: String,

    #[validate(length(equal = 6, message = "OTP harus 6 digit"))]
    pub otp: String,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct LoginDto {
    #[validate(length(min = 4, message = "Username atau Email minimal harus 4 karakter"))]
    pub identifier: String,

    #[validate(length(min = 8, message = "Password minimal harus 8 karakter"))]
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct ResendOtpDto {
    #[validate(email(message = "Format email tidak valid"))]
    pub email: String,
}
