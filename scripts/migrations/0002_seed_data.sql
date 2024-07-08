DO $$
DECLARE
    media_set_last_id media_set.id%TYPE;
    category_last_id category.id%TYPE;
    product_last_id product.id%TYPE;
    language_last_code language.code%TYPE;
    attribute_last_name attribute.name%TYPE;
    attribute_last_value attribute_value.value%TYPE;
    variant_last_id variant.id%TYPE;
    sku_last_id sku.id%TYPE;

BEGIN

INSERT INTO media_set DEFAULT VALUES RETURNING id INTO media_set_last_id;

INSERT INTO media_item (type, path, role, media_set_id) VALUES ('image', '/static/product/prd.webp', 'gallery', media_set_last_id);

INSERT INTO category (name) VALUES ('cars') RETURNING id INTO category_last_id;

INSERT INTO product (name, category_id, media_set_id) VALUES ('Skoda 105', category_last_id, media_set_last_id) RETURNING id INTO product_last_id;

INSERT INTO language (code, name) VALUES ('CZ', 'Čeština') RETURNING code INTO language_last_code;

INSERT INTO slug (text, entity_id, entity_type, language_code) VALUES ('auta', product_last_id, 'product', language_last_code);

INSERT INTO sku (name, product_id) VALUES ('modra skoda 105', product_last_id) RETURNING id INTO sku_last_id;

INSERT INTO description (entity_type, entity_id, language_code, text) VALUES ('sku', sku_last_id, language_last_code, 'Popis SKU');
INSERT INTO description (entity_type, entity_id, language_code, text) VALUES ('product', product_last_id, language_last_code, 'Popis produktu');

INSERT INTO attribute (name) VALUES ('Size') RETURNING name INTO attribute_last_name;
INSERT INTO attribute_value (value) VALUES ('S') RETURNING value INTO attribute_last_value;
INSERT INTO variant (attribute, attribute_value, sku) VALUES (attribute_last_name, attribute_last_value, sku_last_id);

INSERT INTO attribute_value (value) VALUES ('M') RETURNING value INTO attribute_last_value;
INSERT INTO variant (attribute, attribute_value, sku) VALUES (attribute_last_name, attribute_last_value, sku_last_id);

INSERT INTO attribute (name) VALUES ('Color') RETURNING name INTO attribute_last_name;
INSERT INTO attribute_value (value) VALUES ('Blue') RETURNING value INTO attribute_last_value;
INSERT INTO variant (attribute, attribute_value, sku) VALUES (attribute_last_name, attribute_last_value, sku_last_id);

END $$