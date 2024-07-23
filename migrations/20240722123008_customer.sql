CREATE TABLE customer
(
    customer_id UUID PRIMARY KEY,
    email       VARCHAR(64)  NOT NULL,
    name        VARCHAR(64)  NOT NULL,
    password    VARCHAR(97)  NOT NULL,
    address     VARCHAR(128) NOT NULL
);
