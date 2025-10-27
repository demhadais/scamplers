#!/usr/bin/env bash

set -euo pipefail

cargo clippy --fix --allow-dirty --package non-empty-string --all-features
cargo +nightly fmt
