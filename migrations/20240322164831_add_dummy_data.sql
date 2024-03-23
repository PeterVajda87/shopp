DO $$
DECLARE
    v_category_id UUID;
    v_product_id UUID;
BEGIN
    -- Insert into category and get the category_id
    INSERT INTO category (title) VALUES ('Prdy') RETURNING category_id INTO v_category_id;

    -- Insert into product using the obtained category_id and get the product_id
    INSERT INTO product (title, category_id) VALUES ('Makovy prd', v_category_id) RETURNING product_id INTO v_product_id;

    -- Insert into slug for the product
    INSERT INTO slug (slug, page_type, item_id) VALUES ('makovy-prd', 'product', v_product_id);

    -- Insert into slug for the category
    INSERT INTO slug (slug, page_type, item_id) VALUES ('prdy', 'category', v_category_id);
END $$;