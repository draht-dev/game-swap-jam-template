# AGENTS.md

Hinweise für Coding-Agents (und Menschen), die in diesem Repo arbeiten.

## Aufbau

- `src/game.rs`: Spiellogik. Keine Abhängigkeit zu macroquad. Hier gehört neue Logik hin, mit Tests im `#[cfg(test)]`-Modul derselben Datei.
- `src/sfx.rs`: Soundeffekte als im Code erzeugte WAV-Bytes (testbar ohne macroquad).
- `src/main.rs`: Fenster, Eingabe (`read_input`), Zeichnen, Ton abspielen. Liest Tasten, ruft `State::update` auf und zeichnet den Zustand. Möglichst dünn halten.
- `web/index.html`, `web/mq_js_bundle.js`: Web-Hülle. `mq_js_bundle.js` ist vendort (Version und Quelle im README), nicht von Hand ändern.
- `scripts/build-web.sh`: baut WASM und füllt `dist/`.
- `scripts/gate-check.sh`: prüft `dist/` (Gate).
- `scripts/headless-check.sh`: lädt `dist/` in headless Chrome, rot bei ungefangenem JS-Fehler.
- `.github/workflows/ci.yml`: Gate-CI bei jedem Push und PR.
- `.github/workflows/pages.yml`: Pages-Deploy, im Template nur manuell.
- `HANDOFF.md`: Übergabe an das nächste Team. `SWAP.md`: Ablauf beim Swap.
- `CREDITS.md`: alle Mitwirkenden mit Name und Link.

## Befehle

```sh
cargo test                      # Spiellogik nativ testen
cargo run                       # nativ spielen
./scripts/build-web.sh          # WASM-Build nach dist/
./scripts/gate-check.sh         # Gate lokal prüfen (nach build-web.sh)
./scripts/headless-check.sh     # Headless-Gate (braucht Chrome/Chromium, python3)
python3 -m http.server --directory dist 8080
```

## Regeln

1. Vor jedem Push: `cargo test` und `./scripts/build-web.sh && ./scripts/gate-check.sh && ./scripts/headless-check.sh` grün.
2. Logik in `game.rs`, nicht in `main.rs`. Jede neue Regel bekommt mindestens einen Test.
3. Kein Zugriff auf Dateisystem, Netzwerk oder Threads im Spielcode: das Spiel läuft als statische Seite im Browser (`wasm32-unknown-unknown`).
4. Neue Crates nur, wenn sie für `wasm32-unknown-unknown` bauen. Danach Web-Build prüfen.
5. Assets (Bilder, Töne, Schriften) nur mit passender Lizenz, Quelle in `CREDITS.md`.
6. Keine Secrets oder API-Keys ins Repo. Das Repo ist public.
7. Nach dem Tag `handoff` arbeitet nur noch das empfangende Team, auf Branch `swap` (siehe `SWAP.md`).

## Wenn du das Repo übernimmst

Lies in dieser Reihenfolge: `HANDOFF.md`, dieses Dokument, `README.md`. Dann `cargo test` laufen lassen, bevor du etwas änderst.
