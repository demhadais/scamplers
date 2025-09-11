#!/usr/bin/env bash

set -euo pipefail

cd rust/scamplers
wasm-pack build --release --out-dir ../../typescript/scamplers
cd ../../typescript/scamplers
rm -r .gitignore rust-src
cp -r ../../rust/scamplers/src rust-src
