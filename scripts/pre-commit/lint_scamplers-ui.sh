#!/usr/bin/env bash

set -euo pipefail

cargo run --package scamplers-jsonschema
deno task --cwd scamplers-ui check
deno fmt scamplers-ui
