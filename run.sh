#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

export OUT_DIR="."

if command -v cargo &>/dev/null; then
  cargo run --release
else
  echo "please install rust"
fi
