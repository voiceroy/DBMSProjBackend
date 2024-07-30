use actix_web::{post, web, HttpResponse};
use anyhow::anyhow;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::error;
use uuid::Uuid;

use crate::{
    errors::ProjError,
    utils::{password::hash_password, user::check_exists},
};

#[derive(Deserialize)]
pub struct RegisterData {
    pub email: String,
    pub name: String,
    pub password: String,
    pub address: String,
}

#[post("/register")]
#[tracing::instrument(
    name = "Adding a new account",
    skip(data, pool),
    fields(account_email = %data.email, account_name = %data.name)
)]
pub async fn account_register(
    pool: web::Data<PgPool>,
    data: web::Json<RegisterData>,
) -> Result<HttpResponse, ProjError> {
    let exists = check_exists(&pool, &data.email).await;

    match exists {
        false => match insert_account(&pool, data.into_inner()).await {
            Ok(_) => Ok(HttpResponse::Created().body("Account Registered")),
            Err(err) => Err(anyhow!(err).into()),
        },
        true => Err(anyhow!("Email Already in Use").into()),
    }
}

async fn insert_account(pool: &PgPool, data: RegisterData) -> Result<(), ProjError> {
    let (name, email, password, address) = (data.name, data.email, data.password, data.address);
    let password_hash = hash_password(password.as_bytes());

    let _ = sqlx::query!(
        "INSERT INTO customer (customer_id, name, email, password, address) VALUES ($1, $2, $3, $4, $5)",
        Uuid::new_v4(),
        name,
        email,
        password_hash,
        address
    )
    .bind(&password_hash)
    .execute(pool)
    .await
    .map_err(|err| {
        error!("{err}");
        std::convert::Into::<ProjError>::into(anyhow!(err))
    })?;

    Ok(())
}
