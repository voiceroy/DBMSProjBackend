CREATE TABLE order_pizza
(
    order_id UUID REFERENCES customer_order(order_id),
    pizza_id UUID REFERENCES pizza(pizza_id),
    quantity INTEGER NOT NULL,
    PRIMARY KEY (order_id, pizza_id)
);
