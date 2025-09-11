#! /usr/bin/env bash

set -euo pipefail

function cleanup_docker() {
    docker kill scamplers-api_unit_test > /dev/null
    docker rm scamplers-api_unit_test --volumes > /dev/null
}
trap cleanup_docker EXIT

cargo test --workspace "$@" --all-features
