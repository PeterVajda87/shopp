CREATE TABLE product (
    product_id UUID NOT NULL DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,
    category_id UUID,
    PRIMARY KEY(product_id),
    CONSTRAINT fk_category
        FOREIGN KEY(category_id)
            REFERENCES category(category_id),
    CONSTRAINT uq_product_category UNIQUE (title, category_id)
);