#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"

cargo build --release --target wasm32-unknown-unknown

mkdir -p dist
cp target/wasm32-unknown-unknown/release/game_swap_jam_template.wasm dist/
cp web/index.html dist/
cp web/mq_js_bundle.js dist/

echo "Web-Build liegt in $root/dist"
echo "Lokal ausliefern: python3 -m http.server --directory dist 8080"
