#!/usr/bin/env bash
# Testlauf für den Game Swap Jam: misst Setup- und Build-Zeiten auf einem frischen Klon.
# Aufruf: ./scripts/testlauf.sh [REPO-URL oder Pfad]   (Default: dieses Repo)
# Kalter Lauf: eigenes CARGO_HOME (leerer Crate-Cache) und eigenes target/ in einem Temp-Ordner.
# Ergebnis: Tabelle auf stdout, Exit != 0 wenn ein Schritt rot ist.
set -euo pipefail

src="${1:-$(cd "$(dirname "$0")/.." && pwd)}"
work="$(mktemp -d)"
export CARGO_HOME="$work/cargo-home"
mkdir -p "$CARGO_HOME"
# rustup-Proxies (cargo, rustc) liegen im echten ~/.cargo/bin; RUSTUP_HOME bleibt unverändert.
export PATH="${HOME}/.cargo/bin:${PATH}"

rows=()
step() {
  local name="$1"; shift
  local t0 t1 rc=0
  t0=$(date +%s.%N)
  "$@" >"$work/log-${#rows[@]}.txt" 2>&1 || rc=$?
  t1=$(date +%s.%N)
  local dt; dt=$(awk -v a="$t0" -v b="$t1" 'BEGIN{printf "%.1f", b-a}')
  local st="ok"; [ "$rc" -eq 0 ] || st="ROT (log: $work/log-${#rows[@]}.txt)"
  rows+=("| $name | ${dt} s | $st |")
  [ "$rc" -eq 0 ] || { printf '%s\n' "${rows[@]}"; exit 1; }
}

echo "Testlauf $(date '+%Y-%m-%d %H:%M %Z'), Quelle: $src, Arbeitsordner: $work"
echo "rustc: $(rustc --version), cargo: $(cargo --version)"
echo "CPU: $(nproc) Kerne, $(uname -sm)"

step "git clone"                            git clone -q "$src" "$work/repo"
cd "$work/repo"
step "rustup target add wasm32 (falls fehlt)" rustup target add wasm32-unknown-unknown
step "cargo fetch (kalter Crate-Cache)"     cargo fetch
step "cargo test (kalt)"                     cargo test
step "Web-Build (kalt, release wasm32)"      ./scripts/build-web.sh
step "gate-check.sh"                         ./scripts/gate-check.sh
if command -v google-chrome >/dev/null || command -v chromium >/dev/null || command -v chromium-browser >/dev/null; then
  step "headless-check.sh"                   ./scripts/headless-check.sh
else
  rows+=("| headless-check.sh | - | übersprungen (kein Chrome/Chromium) |")
fi
touch src/game.rs
step "Web-Build (warm, nach Änderung in game.rs)" ./scripts/build-web.sh
step "cargo test (warm)"                     cargo test

echo
echo "| Schritt | Zeit | Status |"
echo "|---|---|---|"
printf '%s\n' "${rows[@]}"
echo
echo "Aufräumen: rm -rf $work"
