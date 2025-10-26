#!/usr/bin/env sh

set -euo pipefail

docker run --name scamplers-compilation --env POSTGRES_HOST_AUTH_METHOD=trust --publish 5432:5432 --detach postgres:18-alpine
