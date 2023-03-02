#!/usr/bin/env bash

if ! [ -x "$(command -v psql)" ]; then
  echo "Error: psql not installed" >&2
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo "Error: sqlx not installed" >&2
  echo "Use:" >&2
  echo
  echo "   cargo install --version="~0.6" sqlx-cli --no-default-features --features rustls,postgres" >&2
  echo
  echo "to install it" >&2
  exit 1
fi

# all executed commands are printed to the terminal
set -x
set -eo pipefail

DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD=${DB_PASSWORD:=password}
DB_NAME=${POSTGRES_DB:=newsletter}
DB_PORT=${POSTGRES_PORT:=5440}

docker run \
  -e "POSTGRES_USER=${DB_USER}" \
  -e "POSTGRES_PASSWORD=${DB_PASSWORD}" \
  -e "POSTGRES_DB=${DB_NAME}" \
  -p "${DB_PORT}:5432" \
 postgres postgres -N 1000
