#!/usr/bin/env sh

set -euo pipefail

docker rm scamplers-compilation --force --volumes
docker run --name scamplers-compilation --env POSTGRES_HOST_AUTH_METHOD=trust --env POSTGRES_DB=scamplers-compilation --publish 5432:5432 --detach postgres:18-alpine
