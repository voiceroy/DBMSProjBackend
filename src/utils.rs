use actix_session::SessionExt;
use actix_session::{Session, SessionGetError, SessionInsertError};
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use anyhow::anyhow;
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2,
};
use argon2::{PasswordHash, PasswordHasher, PasswordVerifier};
use sqlx::PgPool;
use std::future::{ready, Ready};
use uuid::Uuid;

use crate::errors::Error;

pub fn hash_password(password: &[u8]) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password, &salt).unwrap().to_string();

    password_hash
}

pub fn verify_password(password: &[u8], password_hash: &str) -> Result<bool, Error> {
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

pub async fn check_exists(pool: &PgPool, email: &String) -> bool {
    sqlx::query("SELECT * FROM customer WHERE email = $1 LIMIT 1")
        .bind(email)
        .fetch_one(pool)
        .await
        .is_ok()
}

pub struct TypedSession(Session);

impl TypedSession {
    const USER_ID_KEY: &'static str = "user_id";

    pub fn renew(&self) {
        self.0.renew();
    }

    pub fn insert_user_id(&self, user_id: Uuid) -> Result<(), SessionInsertError> {
        self.0.insert(Self::USER_ID_KEY, user_id)
    }

    pub fn get_user_id(&self) -> Result<Option<Uuid>, SessionGetError> {
        self.0.get(Self::USER_ID_KEY)
    }

    pub fn log_out(self) {
        self.0.purge()
    }
}

impl FromRequest for TypedSession {
    type Error = <Session as FromRequest>::Error;
    type Future = Ready<Result<TypedSession, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(Ok(TypedSession(req.get_session())))
    }
}
