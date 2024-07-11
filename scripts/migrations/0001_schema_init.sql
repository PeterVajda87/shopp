CREATE TYPE "media_type" AS ENUM (
  'image',
  'video',
  'audio',
  'document'
);

CREATE TYPE "media_role" AS ENUM (
  'gallery',
  'description',
  'instructions',
  'downloadable'
);

CREATE TYPE "entity_type" AS ENUM (
  'product',
  'sku',
  'category'
);

CREATE TABLE "entity" (
  "id" uuid PRIMARY KEY DEFAULT (gen_random_uuid()),
  "entity_id" uuid NOT NULL,
  "entity_type" entity_type NOT NULL
);

CREATE TABLE "slug" (
  "entity_id" uuid PRIMARY KEY,
  "text" varchar NOT NULL,
  "language_id" uuid NOT NULL
);

CREATE TABLE "product" (
  "id" uuid PRIMARY KEY DEFAULT (gen_random_uuid()),
  "category_id" uuid NOT NULL,
  "name" varchar NOT NULL,
  "created_at" timestamptz DEFAULT (now()) NOT NULL
);

CREATE TABLE "description" (
  "id" uuid DEFAULT (gen_random_uuid()),
  "entity_id" uuid,
  "language_id" uuid NOT NULL,
  "text" varchar,
  PRIMARY KEY ("id", "entity_id")
);

CREATE TABLE "sku" (
  "id" uuid PRIMARY KEY DEFAULT (gen_random_uuid()),
  "name" varchar NOT NULL,
  "product_id" uuid NOT NULL,
  "created_at" timestamp DEFAULT (now()),
  "entity_id" uuid
);

CREATE TABLE "category" (
  "id" uuid PRIMARY KEY DEFAULT (gen_random_uuid()),
  "name" varchar UNIQUE NOT NULL,
  "parent_category_id" uuid,
  "created_at" timestamp DEFAULT (now()),
  "entity_id" uuid
);

CREATE TABLE "media_set" (
  "id" uuid PRIMARY KEY DEFAULT (gen_random_uuid()),
  "created_at" timestamp DEFAULT (now())
);

CREATE TABLE "media_set_to_entity" (
  "media_set_id" uuid,
  "entity_id" uuid,
  PRIMARY KEY ("media_set_id", "entity_id")
);

CREATE TABLE "media_item" (
  "id" uuid PRIMARY KEY DEFAULT (gen_random_uuid()),
  "path" varchar UNIQUE NOT NULL,
  "type" media_type NOT NULL,
  "role" media_role NOT NULL,
  "media_set_id" uuid
);

CREATE TABLE "language" (
  "id" uuid PRIMARY KEY DEFAULT (gen_random_uuid()),
  "language_code" varchar NOT NULL,
  "language_name" varchar NOT NULL
);

CREATE TABLE "attribute" (
  "name" varchar PRIMARY KEY
);

CREATE TABLE "attribute_value" (
  "value" varchar PRIMARY KEY
);

CREATE TABLE "variant" (
  "id" uuid PRIMARY KEY DEFAULT (gen_random_uuid()),
  "attribute" varchar,
  "attribute_value" varchar,
  "sku" uuid
);

CREATE UNIQUE INDEX ON "product" ("name", "category_id");

CREATE UNIQUE INDEX ON "sku" ("name", "product_id");

ALTER TABLE "slug" ADD FOREIGN KEY ("entity_id") REFERENCES "entity" ("id");

ALTER TABLE "slug" ADD FOREIGN KEY ("language_id") REFERENCES "language" ("id");

ALTER TABLE "product" ADD FOREIGN KEY ("entity_id") REFERENCES "entity" ("id");

ALTER TABLE "description" ADD FOREIGN KEY ("entity_id") REFERENCES "entity" ("id");

ALTER TABLE "description" ADD FOREIGN KEY ("language_id") REFERENCES "language" ("id");

ALTER TABLE "sku" ADD FOREIGN KEY ("entity_id") REFERENCES "entity" ("id");

ALTER TABLE "category" ADD FOREIGN KEY ("parent_category_id") REFERENCES "category" ("id");

ALTER TABLE "category" ADD FOREIGN KEY ("entity_id") REFERENCES "entity" ("id");

ALTER TABLE "media_set_to_entity" ADD FOREIGN KEY ("media_set_id") REFERENCES "media_set" ("id");

ALTER TABLE "media_set_to_entity" ADD FOREIGN KEY ("entity_id") REFERENCES "entity" ("id");

ALTER TABLE "media_item" ADD FOREIGN KEY ("media_set_id") REFERENCES "media_set" ("id");

ALTER TABLE "variant" ADD FOREIGN KEY ("attribute") REFERENCES "attribute" ("name");

ALTER TABLE "variant" ADD FOREIGN KEY ("attribute_value") REFERENCES "attribute_value" ("value");

ALTER TABLE "variant" ADD FOREIGN KEY ("sku") REFERENCES "sku" ("id");

ALTER TABLE "product" ADD FOREIGN KEY ("category_id") REFERENCES "category" ("id");

ALTER TABLE "sku" ADD FOREIGN KEY ("product_id") REFERENCES "product" ("id");
