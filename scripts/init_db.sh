#!/usr/bin/env bash

# Check if a custom user has been set, otherwise default to 'postgres'
DB_USER="${POSTGRES_USER:=postgres}"
# Check if a custom password has been set, otherwise default to 'password'
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
# Check if a custom database name has been set, otherwise default to 'newsletter'
DB_NAME="${POSTGRES_DB:=shopp}"
# Check if a custom port has been set, otherwise default to '5432
DB_PORT="${POSTGRES_PORT:=5432}"
# Check if a custom host has been set, otherwise default to 'localhost'
DB_HOST="${POSTGRES_HOST:=localhost}"

# Init DB
podman run \
--name postgres \
--network host \
--replace \
-e POSTGRES_USER=$DB_USER \
-e POSTGRES_PASSWORD=$DB_PASSWORD \
-e POSTGRES_DB=$DB_NAME \
-e DB_NAME=$DB_NAME \
-e DB_HOST=$DB_HOST \
-e DB_USER=$DB_USER \
-e DB_PORT=$DB_PORT \
-v /var/lib/data \
-d postgres

>&2 echo "Copying db_schema_init file from /home/peter/shopp/scripts"
podman cp /home/peter/shopp/scripts/schema.sql postgres:/etc/schema.sql

# Keep pinging Postgres until it's ready to accept commands
until podman exec -it postgres psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "${DB_NAME}" -c '\q'; do
>&2 echo "Postgres is still unavailable - sleeping"
sleep 1
done
>&2 echo "Postgres is up and running on port ${DB_PORT}!"

>&2 echo "Creating DB tables"
podman exec -it postgres psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "${DB_NAME}" -f /etc/schema.sql

DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}"
export DATABASE_URL
echo $DATABASE_URL
