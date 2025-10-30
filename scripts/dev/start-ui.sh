#!/usr/bin/env sh

set -euo pipefail

bun run --bun --cwd=scamplers-ui --install=force --sql-preconnect --env-file=.env dev
