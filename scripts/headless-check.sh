#!/usr/bin/env bash
# Headless-Gate: lädt dist/ in headless Chrome und schlägt fehl, wenn die Seite
# einen ungefangenen JS-Fehler ("Uncaught ...") in die Konsole schreibt oder
# nicht auf Eingabe reagiert (Eingabe-Check über index.html?gate-input).
# Aufruf nach scripts/build-web.sh. Exit != 0 = Gate rot.
# Braucht google-chrome oder chromium (auf ubuntu-latest vorinstalliert) und python3.
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
dist="$root/dist"
port="${HEADLESS_PORT:-8765}"
budget_ms="${HEADLESS_BUDGET_MS:-5000}"
# Eingabe-Check läuft in echter Zeit (ohne --virtual-time-budget) und endet, sobald
# "GATE-INPUT ok" kommt, spätestens nach input_timeout_s Sekunden (dann rot).
input_timeout_s="${HEADLESS_INPUT_TIMEOUT_S:-30}"

chrome="${CHROME:-$(command -v google-chrome || command -v chromium || command -v chromium-browser || true)}"
if [ -z "$chrome" ]; then
  echo "FEHLER kein Chrome/Chromium gefunden (CHROME=... setzen)"
  exit 1
fi

python3 -m http.server "$port" --bind 127.0.0.1 --directory "$dist" >/dev/null 2>&1 &
srv=$!
profile="$(mktemp -d)"
log="$(mktemp)"
cpid=""
trap 'kill $srv $cpid 2>/dev/null || true; sleep 1; rm -rf "$profile" "$log" 2>/dev/null || true' EXIT
sleep 1

# --enable-logging=stderr schreibt Konsolenmeldungen der Seite als "CONSOLE(...)" nach stderr.
# SwiftShader liefert WebGL ohne GPU.
chrome_flags=(--headless=new --disable-extensions --no-sandbox --disable-gpu-sandbox
  --use-angle=swiftshader --enable-unsafe-swiftshader
  --user-data-dir="$profile" --enable-logging=stderr --v=0)
run_chrome() { # $1 = URL-Suffix, $2 = Logdatei
  timeout "${HEADLESS_TIMEOUT_S:-20}" "$chrome" "${chrome_flags[@]}" \
    --virtual-time-budget="$budget_ms" --dump-dom \
    "http://127.0.0.1:$port/index.html$1" >/dev/null 2>"$2" || true
}

# 1) Laden ohne ungefangene JS-Fehler
run_chrome "" "$log"
console="$(grep -E 'CONSOLE|Uncaught' "$log" || true)"
[ -n "$console" ] && echo "$console"

if grep -q 'Uncaught' "$log"; then
  echo "FEHLER ungefangener JS-Fehler beim Laden von dist/index.html"
  echo "Headless-Gate: ROT"
  exit 1
fi
echo "ok    keine ungefangenen JS-Fehler (headless Chrome, ${budget_ms} ms)"

# 2) Eingabe-Check: index.html?gate-input hält "Pfeil rechts" gedrückt (an Frames
#    gekoppelt, siehe web/index.html), das Spiel meldet "GATE-INPUT ok", wenn die
#    Eingabe den Spielzustand verändert (siehe src/main.rs).
#    Bewusst ohne --virtual-time-budget: auf CI-Runnern läuft die virtuelle Uhr
#    dort viel schneller als Frames gerendert werden, das Budget war aufgebraucht,
#    bevor die Spielschleife einen Frame mit gedrückter Taste sah (flaky rot).
#    Stattdessen echte Zeit, Abbruch sobald die Zeile da ist.
rm -rf "$profile"; mkdir -p "$profile"
: >"$log"
"$chrome" "${chrome_flags[@]}" "http://127.0.0.1:$port/index.html?gate-input" >/dev/null 2>"$log" &
cpid=$!
waited=0
while [ "$waited" -lt $((input_timeout_s * 2)) ] && kill -0 "$cpid" 2>/dev/null; do
  grep -q -e 'GATE-INPUT ok' -e 'Uncaught' "$log" && break
  sleep 0.5
  waited=$((waited + 1))
done
kill "$cpid" 2>/dev/null || true
wait "$cpid" 2>/dev/null || true
if grep -q 'Uncaught' "$log"; then
  grep -E 'CONSOLE|Uncaught' "$log" || true
  echo "FEHLER ungefangener JS-Fehler im Eingabe-Check"
  echo "Headless-Gate: ROT"
  exit 1
fi
if ! grep -q 'GATE-INPUT ok' "$log"; then
  grep -E 'CONSOLE' "$log" || true
  echo "FEHLER Spiel reagiert nicht auf Eingabe (kein \"GATE-INPUT ok\" in der Konsole nach ${input_timeout_s} s)"
  echo "Headless-Gate: ROT"
  exit 1
fi
echo "ok    Spiel reagiert auf Eingabe (Pfeil rechts, GATE-INPUT ok nach ca. $((waited / 2)) s)"
echo "Headless-Gate: GRÜN"
