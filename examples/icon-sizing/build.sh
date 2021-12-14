#!/bin/sh

set -eux

cd "$(dirname "$0")"
wasm-pack build --dev --target web --out-dir static/pkg
