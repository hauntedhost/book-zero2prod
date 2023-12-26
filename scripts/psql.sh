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

psql \
  --host="${DB_HOST}" \
  --username="${DB_USER}" \
  --port="${DB_PORT}" \
  --dbname="${DB_NAME}"
