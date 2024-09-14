CREATE TABLE IF NOT EXISTS language (
    language_id uuid NOT NULL UNIQUE DEFAULT gen_random_uuid(),
    language_code VARCHAR(2) PRIMARY KEY,
    language_name VARCHAR(255) UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS "user" (
    login VARCHAR(255) PRIMARY KEY,
    password VARCHAR(255) NOT NULL,
    language_id uuid REFERENCES language(language_id)
);

CREATE TYPE entity_type AS ENUM ('product', 'category', 'sku', 'user');

CREATE TABLE IF NOT EXISTS category (
    category_id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    parent_category_id uuid REFERENCES category(category_id)
);

CREATE TABLE IF NOT EXISTS slug (
    target_id uuid NOT NULL,
    target_type VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL,
    language_code VARCHAR(2) REFERENCES language(language_code)
);

CREATE TABLE IF NOT EXISTS category_translation (
    category_id uuid NOT NULL REFERENCES category(category_id) ON DELETE CASCADE,
    language_code VARCHAR(2) NOT NULL REFERENCES language(language_code) DEFAULT 'en',
    category_name VARCHAR(255) NOT NULL,
    category_description VARCHAR(255),
    PRIMARY KEY (category_id, language_code),
    UNIQUE (category_id, category_name)
);

CREATE TABLE IF NOT EXISTS product (
    product_id uuid DEFAULT gen_random_uuid() PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS product_translation (
    product_id uuid NOT NULL REFERENCES product(product_id) ON DELETE CASCADE,
    language_code VARCHAR(2) NOT NULL REFERENCES language(language_code) DEFAULT 'en',
    product_name VARCHAR(255) NOT NULL,
    product_description VARCHAR(255),
    PRIMARY KEY (product_id, language_code),
    UNIQUE (product_id, product_name)
);

CREATE TABLE IF NOT EXISTS product_to_category (
    product_id uuid REFERENCES product(product_id) NOT NULL,
    category_id uuid REFERENCES category(category_id) NOT NULL,
    PRIMARY KEY (product_id, category_id)
);

CREATE TABLE IF NOT EXISTS sku (
    sku_id uuid DEFAULT gen_random_uuid() PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS sku_translation (
    sku_id uuid NOT NULL REFERENCES sku(sku_id) ON DELETE CASCADE,
    language_code VARCHAR(2) NOT NULL REFERENCES language(language_code) DEFAULT 'en',
    sku_name VARCHAR(255) NOT NULL,
    sku_description VARCHAR(255),
    PRIMARY KEY (sku_id, language_code),
    UNIQUE (sku_id, sku_name)
);

CREATE TABLE IF NOT EXISTS sku_to_product (
    sku_id uuid REFERENCES sku(sku_id) NOT NULL,
    product_id uuid REFERENCES product(product_id) NOT NULL,
    PRIMARY KEY (sku_id, product_id)
);
