CREATE TABLE order_non_pizza
(
    order_id UUID REFERENCES customer_order(order_id),
    non_pizza_id UUID REFERENCES non_pizza(non_pizza_id),
    quantity INTEGER NOT NULL,
    PRIMARY KEY (order_id, non_pizza_id)
);
