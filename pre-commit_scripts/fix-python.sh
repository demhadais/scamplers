#!/usr/bin/env bash

set -euo pipefail

cd python
ruff check
ruff format
