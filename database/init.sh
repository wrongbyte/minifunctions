#!/usr/bin/env bash

# Create PostgreSQL directories if they don't exist
mkdir -p "$PGHOST"
mkdir -p "$LOG_PATH"

# Initialize database if it doesn't exist
if [ ! -d "$PGDATA" ]; then
  echo "Initializing PostgreSQL database..."
  initdb -D "$PGDATA" --auth=trust
fi

# Start PostgreSQL if it's not running
if ! pg_ctl status -D "$PGDATA" > /dev/null 2>&1; then
  echo "Starting PostgreSQL server..."
  pg_ctl start -l "$LOG_PATH" -D "$PGDATA" -o "-c listen_addresses= -c unix_socket_directories=$PGHOST"
fi

# Create the database if it doesn't exist
if ! psql -lqt | cut -d \| -f 1 | grep -qw "$PGDATABASE" 2>/dev/null; then
  echo "Creating database: $PGDATABASE"
  createdb "$PGDATABASE"
fi

if [ -f ./database/init.sql ]; then
  echo "Running initialization SQL script..."
  psql -d "$PGDATABASE" -f ./database/init.sql
else
  echo "Note: ./database/init.sql not found, skipping initialization script."
fi

echo "PostgreSQL setup complete!"
