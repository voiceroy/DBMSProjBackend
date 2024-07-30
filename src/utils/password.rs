use crate::errors::ProjError;
use anyhow::anyhow;
use argon2::PasswordHasher;
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};

pub fn hash_password(password: &[u8]) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password, &salt).unwrap().to_string();

    password_hash
}

pub fn verify_password(password: &[u8], password_hash: &str) -> Result<bool, ProjError> {
    let parsed_hash = match PasswordHash::new(password_hash) {
        Ok(hash) => hash,
        Err(_) => {
            return Err(anyhow!("Failed To Parse Error").into());
        }
    };

    match Argon2::default().verify_password(password, &parsed_hash) {
        Ok(_) => Ok(true),
        Err(_) => Err(anyhow!("Invalid Password").into()),
    }
}
