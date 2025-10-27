#!/usr/bin/env bash

set -euo pipefail

cargo clippy --fix --allow-dirty --package scamplers-models --features schema
cargo +nightly fmt
