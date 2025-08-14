#!/usr/bin/env bash

set -euo pipefail

cd rust
cargo clippy --fix --workspace --allow-dirty --exclude scamplers-schema --all-features
cargo clippy --fix --allow-dirty --workspace --target wasm32-unknown-unknown
cargo +nightly fmt
