#!/usr/bin/env bash

set -euo pipefail

cd scamplers-frontend
deno task check
deno fmt
