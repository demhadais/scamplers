#!/usr/bin/env sh

set -euo pipefail

function cleanup_docker() {
    docker kill scamplers-dev >/dev/null
    docker rm scamplers-dev --volumes >/dev/null
}
trap cleanup_docker EXIT

docker run --name scamplers-dev --env POSTGRES_HOST_AUTH_METHOD=trust --publish 5432:5432 --detach postgres:18-alpine

# thanks ChatGPT
until docker exec --user postgres scamplers-dev pg_isready >/dev/null 2>&1; do
    sleep 0.1
done

cd scamplers-api && DATABASE_URL="postgres://postgres@localhost:5432/postgres" cargo run --seed-data-path ../seed_data.dev.json
