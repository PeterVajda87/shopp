CREATE TYPE "entity_type" AS ENUM (
  'product',
  'category',
  'sku'
);

CREATE TYPE "media_type" AS ENUM (
  'image',
  'video',
  'audio',
  'document'
);

CREATE TABLE "slug" (
  "entity_type" entity_type NOT NULL,
  "entity_id" uuid NOT NULL,
  "text" varchar NOT NULL,
  "language_code" varchar NOT NULL,
  PRIMARY KEY ("entity_type", "entity_id")
);

CREATE TABLE "product" (
  "id" uuid PRIMARY KEY DEFAULT (gen_random_uuid()),
  "category_id" uuid NOT NULL,
  "name" varchar NOT NULL,
  "gallery_id" uuid,
  "created_at" timestamp DEFAULT (now())
);

CREATE TABLE "sku" (
  "id" uuid PRIMARY KEY DEFAULT (gen_random_uuid()),
  "name" varchar NOT NULL,
  "product_id" uuid NOT NULL,
  "gallery_id" uuid,
  "created_at" timestamp DEFAULT (now())
);

CREATE TABLE "category" (
  "id" uuid PRIMARY KEY DEFAULT (gen_random_uuid()),
  "name" varchar UNIQUE NOT NULL,
  "gallery_id" uuid,
  "parent_category_id" uuid,
  "created_at" timestamp DEFAULT (now())
);

CREATE TABLE "gallery" (
  "id" uuid PRIMARY KEY DEFAULT (gen_random_uuid()),
  "created_at" timestamp DEFAULT (now())
);

CREATE TABLE "media" (
  "id" uuid PRIMARY KEY DEFAULT (gen_random_uuid()),
  "path" varchar UNIQUE NOT NULL,
  "type" media_type NOT NULL,
  "gallery_id" uuid
);

CREATE TABLE "language" (
  "code" varchar PRIMARY KEY,
  "name" varchar NOT NULL
);

CREATE UNIQUE INDEX ON "product" ("name", "category_id");

CREATE UNIQUE INDEX ON "sku" ("name", "product_id");

ALTER TABLE "slug" ADD FOREIGN KEY ("language_code") REFERENCES "language" ("code");

ALTER TABLE "category" ADD FOREIGN KEY ("parent_category_id") REFERENCES "category" ("id");

ALTER TABLE "product" ADD FOREIGN KEY ("category_id") REFERENCES "category" ("id");

ALTER TABLE "sku" ADD FOREIGN KEY ("product_id") REFERENCES "product" ("id");

ALTER TABLE "media" ADD FOREIGN KEY ("gallery_id") REFERENCES "gallery" ("id");

ALTER TABLE "product" ADD FOREIGN KEY ("gallery_id") REFERENCES "gallery" ("id");

ALTER TABLE "sku" ADD FOREIGN KEY ("gallery_id") REFERENCES "gallery" ("id");

ALTER TABLE "category" ADD FOREIGN KEY ("gallery_id") REFERENCES "gallery" ("id");
