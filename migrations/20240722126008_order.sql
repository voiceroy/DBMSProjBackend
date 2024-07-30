CREATE TYPE order_status AS ENUM ('cancelled', 'placed', 'dispatched', 'delivered');

CREATE TABLE customer_order
(
    order_id      UUID PRIMARY KEY,
    customer_id   UUID REFERENCES customer(customer_id),
    order_time    TIMESTAMPTZ  NOT NULL,
    delivery_time TIMESTAMPTZ  NOT NULL,
    status        order_status NOT NULL,
    payment_id    UUID REFERENCES payment(payment_id)
);
