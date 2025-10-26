#!/usr/bin/env bash

set -euo pipefail

cargo run --package scamplers-jsonschema
cd scamplers-frontend
deno task check
deno fmt
