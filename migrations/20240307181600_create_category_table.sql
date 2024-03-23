CREATE TABLE category (
    category_id UUID NOT NULL DEFAULT gen_random_uuid(),
    title TEXT NOT NULL UNIQUE,
    parent_category_id UUID,
    PRIMARY KEY(category_id),
    CONSTRAINT fk_parent
        FOREIGN KEY(parent_category_id)
            REFERENCES category(category_id)
);

