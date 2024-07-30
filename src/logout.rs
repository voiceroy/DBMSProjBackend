use crate::errors::ProjError;
use crate::utils::session::TypedSession;
use actix_web::{post, HttpResponse};
use tracing::instrument;

#[post("/logout")]
#[instrument(name = "Logging out account", skip(session))]
pub async fn account_logout(session: TypedSession) -> Result<HttpResponse, ProjError> {
    session.log_out();
    Ok(HttpResponse::Ok().finish())
}
