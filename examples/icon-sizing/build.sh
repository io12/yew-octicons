#!/bin/sh

set -eux

cd "$(dirname "$0")"
CARGO_TARGET_DIR=../../target wasm-pack build --dev --target web --out-dir static/pkg
