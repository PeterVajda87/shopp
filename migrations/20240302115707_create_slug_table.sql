CREATE TYPE page_type AS ENUM ('product', 'category', 'cms', 'checkout', 'cart');

CREATE TABLE slug (
    slug TEXT PRIMARY KEY NOT NULL UNIQUE,
    page_type page_type NOT NULL,
    item_id UUID NOT NULL
);