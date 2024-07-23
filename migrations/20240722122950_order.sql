CREATE TYPE order_status AS ENUM ('cancelled', 'placed', 'dispatched', 'delivered');

CREATE TABLE customer_order
(
    order_id      UUID PRIMARY KEY,
    order_time    TIMESTAMPTZ  NOT NULL,
    delivery_time TIMESTAMPTZ  NOT NULL,
    order_status  order_status NOT NULL
);
