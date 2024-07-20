#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

export OUT_DIR="."

if command -v cargo &>/dev/null; then
  cargo run --quiet --release
else
  echo "install rust"
fi
