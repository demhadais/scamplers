#! /usr/bin/env bash

set -euo pipefail

function cleanup_docker() {
    docker compose kill
    docker compose rm
    docker volume ls --format json | jq '.[].Name | select(startswith("scamplers"))' --slurp | xargs docker volume rm
}

trap cleanup_docker EXIT

docker compose --env-file .env.compose --file compose.yaml --file compose.dev.yaml up --build
