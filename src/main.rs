pub mod errors;
pub mod get_items;
pub mod login;
pub mod logout;
pub mod place_order;
pub mod register;
pub mod utils;

use std::io::Error;

use actix_cors::Cors;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, web, App, HttpServer};
use env_logger::Env;
use get_items::items;
use login::account_login;
use place_order::place;
use register::account_register;
use sqlx::PgPool;
use tracing::error;
use tracing_actix_web::TracingLogger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let pool = web::Data::new(
        PgPool::connect("postgres://postgres:abcd@localhost/postgres")
            .await
            .map_err(|err| {
                error!("{err}");
                Error::new(
                    std::io::ErrorKind::NotConnected,
                    "Cannot Connect To Database",
                )
            })?,
    );

    sqlx::migrate!("./migrations")
        .run(&**pool)
        .await
        .map_err(|err| {
            error!("{err}");
            Error::new(
                std::io::ErrorKind::NotConnected,
                "Cannot Connect To Database",
            )
        })?;

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            .wrap(TracingLogger::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[1; 64]))
                    .cookie_secure(false)
                    .build(),
            )
            .service(account_login)
            .service(account_register)
            .service(items)
            .service(place)
            .app_data(pool.clone())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
