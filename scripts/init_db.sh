#!/usr/bin/env bash
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "🦀 Error: psql is not installed"
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "🦀 Error: sqlx is not installed"
  exit 1
fi

if [[ -z "${SKIP_DOCKER}" ]]; then
  echo >&2 "🦀 Starting Postgres container ..."
  docker run \
    --env POSTGRES_USER=${DB_USER} \
    --env POSTGRES_PASSWORD=${DB_PASSWORD} \
    --env POSTGRES_DB=${DB_NAME} \
    --publish "${DB_PORT}":5432 \
    --detach \
    postgres \
    postgres --max_connections=1000
fi

echo >&2 "🦀 Container 'zero2prod' is up and running 😎"

# Keep pinging Postgres until it's ready to accept commands
export PGPASSWORD="${DB_PASSWORD}"
until psql \
  --host="${DB_HOST}" \
  --username="${DB_USER}" \
  --port="${DB_PORT}" \
  --dbname="postgres" \
  --command='\q' \
  > /dev/null 2>&1;
do
  echo >&2 "🦀 Waiting for Postgres ..."
  sleep 1
done

echo >&2 "🦀 Postgres is up and running on port ${DB_PORT} 😎"

sqlx database create
sqlx migrate run

echo >&2 "🦀 Postgres migrations complete 🎉"
