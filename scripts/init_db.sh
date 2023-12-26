#!/usr/bin/env bash
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: psql is not installed."
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed."
  exit 1
fi

docker ps --quiet --filter name=zero2prod | grep --quiet . \
  && (docker logs zero2prod; docker attach zero2prod) \
  || docker run \
    --env POSTGRES_USER=${DB_USER} \
    --env POSTGRES_PASSWORD=${DB_PASSWORD} \
    --env POSTGRES_DB=${DB_NAME} \
    --publish "${DB_PORT}":5432 \
    --name zero2prod \
    postgres --max_connections=1000

# Keep pinging Postgres until it's ready to accept commands
export PGPASSWORD="${DB_PASSWORD}"
until psql \
  --host="${DB_HOST}" \
  --username="${DB_USER}" \
  --port="${DB_PORT}" \
  --dbname="postgres" \
  --command='\q';
do
  >&2 echo "Postgres is still unavailable - sleeping"
  sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT}!"

sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"
