use actix_web::{post, web, HttpResponse};
use anyhow::anyhow;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::error;

use crate::{
    errors::Error,
    utils::{password::verify_password, session::TypedSession, user::check_exists},
};

#[derive(Deserialize)]
pub struct LoginData {
    pub email: String,
    pub password: String,
}

#[post("/login")]
#[tracing::instrument(
    name = "Logging in account",
    skip(data, pool, session),
    fields(email = % data.email)
)]
pub async fn account_login(
    pool: web::Data<PgPool>,
    data: web::Json<LoginData>,
    session: TypedSession,
) -> Result<HttpResponse, Error> {
    let exists = check_exists(&pool, &data.email).await;

    if !exists {
        return Err(anyhow!("User Does Not Exist").into());
    }

    // We already know that the user exists, hence unwrap
    let credentials = get_stored_credentials(&data, &pool).await?;
    match check_password(&data.into_inner(), credentials.1).await? {
        true => {
            session.renew();
            session.insert_user_id(credentials.0).map_err(|err| {
                error!("{err}");
                std::convert::Into::<Error>::into(anyhow!(err))
            })?;
            Ok(HttpResponse::Ok().finish())
        }
        false => Err(anyhow!("Invalid Password").into()),
    }
}

#[tracing::instrument(name = "Get stored credentials", skip(data, pool))]
async fn get_stored_credentials(
    data: &LoginData,
    pool: &PgPool,
) -> Result<(uuid::Uuid, String), Error> {
    match sqlx::query!(
        "SELECT customer_id, password FROM customer WHERE email = $1",
        data.email
    )
    .fetch_one(pool)
    .await
    {
        Ok(row) => Ok((row.customer_id, (row.password))),
        Err(_) => Err(anyhow!("User Not Found").into()),
    }
}

#[tracing::instrument(name = "Checking against PHC", skip(data))]
async fn check_password(data: &LoginData, password_hash: String) -> Result<bool, Error> {
    verify_password(data.password.as_bytes(), &password_hash).map_err(|err| {
        error!("{err}");
        std::convert::Into::<Error>::into(anyhow!("Password Error"))
    })
}
