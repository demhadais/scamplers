#!/usr/bin/env bash

set -euo pipefail

cargo run --package scamplers-models --features schema
cd scamplers-frontend
deno task check
deno fmt
