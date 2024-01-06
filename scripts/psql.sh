# Keep pinging Postgres until it's ready to accept commands
until psql \
  --username="${PGUSER}" \
  --host="${PGHOST}" \
  --port="${PGPORT}" \
  --dbname="postgres" \
  --command='\q';
do
  >&2 echo "Postgres is still unavailable - sleeping"
  sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT}!"

psql \
  --username="${PGUSER}" \
  --host="${PGHOST}" \
  --port="${PGPORT}" \
  --dbname="${PGNAME}"
