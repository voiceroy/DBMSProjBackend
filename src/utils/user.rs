use sqlx::PgPool;

pub async fn check_exists(pool: &PgPool, email: &String) -> bool {
    sqlx::query("SELECT * FROM customer WHERE email = $1 LIMIT 1")
        .bind(email)
        .fetch_one(pool)
        .await
        .is_ok()
}
