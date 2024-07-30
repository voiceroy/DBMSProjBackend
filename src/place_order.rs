use actix_web::{post, web, HttpResponse, Responder};
use chrono::Duration;
use chrono::Utc;
use rand::random;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::info;
use uuid::Uuid;

use crate::utils::session::TypedSession;

#[derive(Debug, Serialize, Deserialize)]
struct PizzaOrderItem {
    pizza_id: Uuid,
    quantity: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct NonPizzaOrderItem {
    non_pizza_id: Uuid,
    quantity: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct PlaceOrderRequest {
    pizzas: Vec<PizzaOrderItem>,
    non_pizzas: Vec<NonPizzaOrderItem>,
    payment_amount: i32,
}

#[derive(Serialize)]
struct PlaceOrderResponse {
    order_id: Uuid,
    status: String,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "order_status", rename_all = "lowercase")]
pub enum OrderStatus {
    Cancelled,
    Placed,
    Dispatched,
    Delivered,
}

#[post("/place_order")]
pub async fn place(
    pool: web::Data<PgPool>,
    session: TypedSession,
    data: web::Json<PlaceOrderRequest>,
) -> impl Responder {
    let order_id = Uuid::new_v4();
    let payment_id = Uuid::new_v4();

    info!("{data:#?}");

    // Insert payment
    sqlx::query!(
        "INSERT INTO payment (payment_id, amount) VALUES ($1, $2)",
        payment_id,
        data.payment_amount
    )
    .execute(&**pool)
    .await
    .unwrap();

    // Insert order
    sqlx::query!(
        "INSERT INTO customer_order (order_id, customer_id, order_time, delivery_time, status, payment_id) VALUES ($1, $2, $3, $4, $5, $6)",
        order_id,
        session.get_user_id().unwrap(),
        Utc::now(),
        Utc::now() + Duration::minutes(random::<i64>().clamp(15, 45)),
        OrderStatus::Placed as _,
        payment_id
    )
    .execute(&**pool)
    .await.unwrap();

    // Insert pizza order items
    for item in &data.pizzas {
        sqlx::query!(
            "INSERT INTO order_pizza (order_id, pizza_id, quantity) VALUES ($1, $2, $3)",
            order_id,
            item.pizza_id,
            item.quantity
        )
        .execute(&**pool)
        .await
        .unwrap();
    }

    // Insert non-pizza order items
    for item in &data.non_pizzas {
        sqlx::query!(
            "INSERT INTO order_non_pizza (order_id, non_pizza_id, quantity) VALUES ($1, $2, $3)",
            order_id,
            item.non_pizza_id,
            item.quantity
        )
        .execute(&**pool)
        .await
        .unwrap();
    }

    HttpResponse::Ok().json(PlaceOrderResponse {
        order_id,
        status: String::from("Order placed successfully"),
    })
}
