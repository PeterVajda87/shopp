#!/usr/bin/env bash 
container_name=postgres_db
postgres_user=postgres
postgres_password=5teveJo85


# Init DB 
podman run \
--name postgres \
-e POSTGRES_USER=$postgres_user \
-e POSTGRES_PASSWORD=$postgres_password \
-p 5432:5432 \
-v /var/lib/data \
-d postgres
