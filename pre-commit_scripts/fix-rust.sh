#!/usr/bin/env bash

set -euo pipefail

cd rust
cargo clippy --fix --workspace --allow-dirty --all-targets --exclude scamplers-schema
cargo clippy --fix --allow-dirty --package scamplers-core --features backend
cargo clippy --fix --allow-dirty --package scamplers-core --features python
cargo clippy --fix --allow-dirty --package scamplers-core --target wasm32-unknown-unknown
cargo +nightly fmt
