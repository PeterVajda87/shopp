-- Add migration script here
CREATE TABLE product (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    title TEXT NOT NULL
);

WITH ins AS (
INSERT INTO product (title) VALUES ('Makovy prd') RETURNING id
)
INSERT INTO slug (slug, page_type, item_id) VALUES ('Prd', 'product', (SELECT id FROM ins)); 