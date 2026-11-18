#!/usr/bin/env bash
# Gate-Check für den Game Swap Jam: prüft, ob dist/ eine lauffähige statische Seite ist.
# Aufruf nach scripts/build-web.sh. Exit != 0 = Gate rot.
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
dist="$root/dist"
fail=0

check() {
  if eval "$2"; then
    echo "ok    $1"
  else
    echo "FEHLER $1"
    fail=1
  fi
}

check "dist/index.html vorhanden"            "[ -s '$dist/index.html' ]"
check "dist/mq_js_bundle.js vorhanden"       "[ -s '$dist/mq_js_bundle.js' ]"
check "index.html lädt mq_js_bundle.js"      "grep -q 'mq_js_bundle.js' '$dist/index.html'"

wasm="$(grep -o 'load(\"[^\"]*\.wasm\")' "$dist/index.html" | sed -E 's/load\("(.*)"\)/\1/' || true)"
check "index.html ruft load(\"*.wasm\") auf"   "[ -n '$wasm' ]"
if [ -n "$wasm" ]; then
  check "referenzierte WASM-Datei $wasm existiert" "[ -s '$dist/$wasm' ]"
  check "$wasm ist ein WebAssembly-Modul"         "[ \"\$(head -c 4 '$dist/$wasm' | od -An -tx1 | tr -d ' \n')\" = '0061736d' ]"
fi

if [ "$fail" -ne 0 ]; then
  echo "Gate: ROT"
  exit 1
fi
echo "Gate: GRÜN"
