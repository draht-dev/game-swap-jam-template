#!/usr/bin/env bash
# Headless-Gate: lädt dist/ in headless Chrome und schlägt fehl, wenn die Seite
# einen ungefangenen JS-Fehler ("Uncaught ...") in die Konsole schreibt.
# Aufruf nach scripts/build-web.sh. Exit != 0 = Gate rot.
# Braucht google-chrome oder chromium (auf ubuntu-latest vorinstalliert) und python3.
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
dist="$root/dist"
port="${HEADLESS_PORT:-8765}"
budget_ms="${HEADLESS_BUDGET_MS:-5000}"

chrome="${CHROME:-$(command -v google-chrome || command -v chromium || command -v chromium-browser || true)}"
if [ -z "$chrome" ]; then
  echo "FEHLER kein Chrome/Chromium gefunden (CHROME=... setzen)"
  exit 1
fi

python3 -m http.server "$port" --bind 127.0.0.1 --directory "$dist" >/dev/null 2>&1 &
srv=$!
profile="$(mktemp -d)"
log="$(mktemp)"
trap 'kill $srv 2>/dev/null || true; rm -rf "$profile" "$log"' EXIT
sleep 1

# --enable-logging=stderr schreibt Konsolenmeldungen der Seite als "CONSOLE(...)" nach stderr.
# SwiftShader liefert WebGL ohne GPU.
timeout "${HEADLESS_TIMEOUT_S:-20}" "$chrome" --headless=new --disable-extensions --no-sandbox --disable-gpu-sandbox \
  --use-angle=swiftshader --enable-unsafe-swiftshader \
  --user-data-dir="$profile" --enable-logging=stderr --v=0 \
  --virtual-time-budget="$budget_ms" --dump-dom \
  "http://127.0.0.1:$port/index.html" >/dev/null 2>"$log" || true

console="$(grep -E 'CONSOLE|Uncaught' "$log" || true)"
[ -n "$console" ] && echo "$console"

if grep -q 'Uncaught' "$log"; then
  echo "FEHLER ungefangener JS-Fehler beim Laden von dist/index.html"
  echo "Headless-Gate: ROT"
  exit 1
fi
echo "ok    keine ungefangenen JS-Fehler (headless Chrome, ${budget_ms} ms)"
echo "Headless-Gate: GRÜN"
