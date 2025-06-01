-- Add up migration script here
CREATE TABLE order_details
(
    id           VARCHAR PRIMARY KEY,
    order_id     VARCHAR                  NOT NULL,
    product_name VARCHAR                  NOT NULL,
    quantity     INT                      NOT NULL,
    created_at   TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at   TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_order_details_order FOREIGN KEY (order_id) REFERENCES orders (id)
);
