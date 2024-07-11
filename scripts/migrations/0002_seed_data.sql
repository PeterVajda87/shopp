DO $$
DECLARE
    media_set_last_id media_set.id%TYPE;
    category_last_id category.id%TYPE;
    product_last_id product.id%TYPE;
    language_last_id language.id%TYPE;
    attribute_last_name attribute.name%TYPE;
    attribute_last_value attribute_value.value%TYPE;
    variant_last_id variant.id%TYPE;
    sku_last_id sku.id%TYPE;

BEGIN

--- 1 for category, 2 for product, 3 for sku
INSERT INTO entity (id, entity_id, entity_type) VALUES ('11111111-1111-4111-8111-111111111111', '11111111-1111-4111-8111-111111111111', 'category'),
('22222222-2222-4222-8222-222222222222', '22222222-2222-4222-8222-222222222222', 'product'),
('33333333-3333-4333-8333-333333333333', '33333333-3333-4333-8333-333333333333', 'sku');

--- dummy media set
INSERT INTO media_set (id) VALUES ('11111111-1111-4111-8111-111111111111');
INSERT INTO media_item (type, path, role, media_set_id) VALUES ('image', '/static/product/prd.webp', 'gallery', '11111111-1111-4111-8111-111111111111');
INSERT INTO media_set_to_entity (media_set_id, entity_id) VALUES ('11111111-1111-4111-8111-111111111111', '11111111-1111-4111-8111-111111111111');

INSERT INTO category (name, entity_id) VALUES ('cars', '11111111-1111-4111-8111-111111111111') RETURNING id INTO category_last_id;

INSERT INTO product (id, name, category_id) VALUES ('22222222-2222-4222-8222-222222222222', 'Skoda 105', category_last_id) RETURNING id INTO product_last_id;

INSERT INTO language (language_code, language_name) VALUES ('CZ', 'Čeština') RETURNING id INTO language_last_id;

INSERT INTO slug (text, entity_id, language_id) VALUES ('auta', '22222222-2222-4222-8222-222222222222', language_last_id);

INSERT INTO sku (name, product_id) VALUES ('modra skoda 105', product_last_id) RETURNING id INTO sku_last_id;

INSERT INTO description (entity_id, language_id, text) VALUES ('33333333-3333-4333-8333-333333333333', language_last_id, 'Popis SKU');
INSERT INTO description (entity_id, language_id, text) VALUES ('22222222-2222-4222-8222-222222222222', language_last_id, 'Popis produktu');
INSERT INTO description (entity_id, language_id, text) VALUES ('11111111-1111-4111-8111-111111111111', language_last_id, 'Popis kategorie');

INSERT INTO attribute (name) VALUES ('Size') RETURNING name INTO attribute_last_name;
INSERT INTO attribute_value (value) VALUES ('S') RETURNING value INTO attribute_last_value;
INSERT INTO variant (attribute, attribute_value, sku) VALUES (attribute_last_name, attribute_last_value, sku_last_id);

INSERT INTO attribute_value (value) VALUES ('M') RETURNING value INTO attribute_last_value;
INSERT INTO variant (attribute, attribute_value, sku) VALUES (attribute_last_name, attribute_last_value, sku_last_id);

INSERT INTO attribute (name) VALUES ('Color') RETURNING name INTO attribute_last_name;
INSERT INTO attribute_value (value) VALUES ('Blue') RETURNING value INTO attribute_last_value;
INSERT INTO variant (attribute, attribute_value, sku) VALUES (attribute_last_name, attribute_last_value, sku_last_id);

END $$
