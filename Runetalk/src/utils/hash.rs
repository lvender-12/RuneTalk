use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};

use crate::errors::AppResult;

pub fn hash_password(password: &str) -> AppResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon = Argon2::default();
    Ok(argon.hash_password(password.as_bytes(), &salt)?.to_string())
}

pub fn verif_password(password: &str, password_hash: &str) -> AppResult<bool> {
    let hash = PasswordHash::new(password_hash)?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &hash)
        .is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify_password() {
        let password = "my_secret_password";
        let hashed = hash_password(password).unwrap();
        assert_ne!(password, hashed);

        let is_valid = verif_password(password, &hashed).unwrap();
        assert!(is_valid);

        let is_invalid = verif_password("wrong_password", &hashed).unwrap();
        assert!(!is_invalid);
    }

    #[test]
    fn test_verify_invalid_hash_format() {
        let res = verif_password("password", "invalid-hash");
        assert!(res.is_err());
    }
}
