-- Add migration script here
CREATE TABLE product (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    title TEXT NOT NULL
);