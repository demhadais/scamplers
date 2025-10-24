#!/usr/bin/env bash

set -euo pipefail

cargo clippy --fix --allow-dirty --package default-vec --all-features
cargo +nightly fmt
