#!/usr/bin/env bash
set -x
set -eo pipefail

if ! [ -x "$(command -v docker)" ]; then
  echo 'Error: docker is not installed.' >&2
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo 'Error: sqlx is not installed.' >&2
  exit 1
fi

export POSTGRES_USER=${POSTGRES_USER:-postgres}
export POSTGRES_PASSWORD=${POSTGRES_PASSWORD:-postgres}
export POSTGRES_DB=${POSTGRES_DB:-newsletter}
export POSTGRES_PORT=${POSTGRES_PORT:-5432}
export POSTGRES_HOST=${POSTGRES_HOST:-localhost}

DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DB}

export DATABASE_URL

docker compose up -d
./scripts/wait-for-it.sh -s ${POSTGRES_HOST}:${POSTGRES_PORT} -- sqlx database create
./scripts/wait-for-it.sh -s ${POSTGRES_HOST}:${POSTGRES_PORT} -- sqlx migrate run
