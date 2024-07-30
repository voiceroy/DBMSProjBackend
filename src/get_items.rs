use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize)]
struct Pizza {
    pizza_id: Uuid,
    name: String,
    price: i32,
}

#[derive(Serialize)]
struct NonPizza {
    non_pizza_id: Uuid,
    name: String,
    price: i32,
}

#[derive(Serialize)]
struct ItemsResponse {
    pizzas: Vec<Pizza>,
    non_pizzas: Vec<NonPizza>,
}

#[get("/items")]
async fn items(pool: web::Data<PgPool>) -> impl Responder {
    let pizzas: Vec<Pizza> = sqlx::query_as!(Pizza, "SELECT pizza_id, name, price FROM pizza")
        .fetch_all(pool.get_ref())
        .await
        .unwrap();

    let non_pizzas: Vec<NonPizza> =
        sqlx::query_as!(NonPizza, "SELECT non_pizza_id, name, price FROM non_pizza")
            .fetch_all(pool.get_ref())
            .await
            .unwrap();

    HttpResponse::Ok().json(ItemsResponse { pizzas, non_pizzas })
}
