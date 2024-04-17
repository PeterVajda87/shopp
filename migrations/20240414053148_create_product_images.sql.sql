CREATE TABLE product_image (
    product_image_id UUID NOT NULL DEFAULT gen_random_uuid(),
    main_image BOOLEAN NOT NULL,
    image_order SMALLINT NOT NULL,
    path VARCHAR NOT NULL,
    product_id UUID NOT NULL,
    PRIMARY KEY(product_image_id),
    CONSTRAINT fk_product
        FOREIGN KEY(product_id)
            REFERENCES product(product_id)
);