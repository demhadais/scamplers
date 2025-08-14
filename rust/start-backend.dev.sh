#!/usr/bin/env sh

set -euo pipefail
cargo run --features app -- --dev --seed-data-path ../seed_data.dev.json
