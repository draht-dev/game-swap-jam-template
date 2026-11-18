# game-swap-jam-template

Rust-Crate mit macroquad 0.4 (gelockt 0.4.16). Lizenz: MIT.

Ein minimales Beispielspiel: ein Rechteck bewegt sich mit Pfeiltasten oder WASD und sammelt Punkte. Die Spiellogik liegt in `src/game.rs` und hat keine Abhängigkeit zu macroquad. Rendering und Eingabe stehen in `src/main.rs`.

## Voraussetzungen

- Rust über rustup, Target `wasm32-unknown-unknown`
- Linux (nativer Build): `pkg-config`, `libx11-dev`, `libxi-dev`, `libgl1-mesa-dev`, `libasound2-dev`

## Build und Tests

```sh
cargo test
cargo build
cargo run
```

## Web-Build

```sh
./scripts/build-web.sh
python3 -m http.server --directory dist 8080
```

Danach im Browser `http://127.0.0.1:8080` öffnen. `dist/` entsteht beim Build und liegt in `.gitignore`.

Das Skript baut `cargo build --release --target wasm32-unknown-unknown` und kopiert WASM, `web/index.html` und `web/mq_js_bundle.js` nach `dist/`.

## mq_js_bundle.js

Die Datei `web/mq_js_bundle.js` ist vendort, passend zu macroquad 0.4.16 und der gelockten miniquad 0.4.11.

- Quelle: Crate `macroquad` 0.4.16, Pfad `js/mq_js_bundle.js` (Upstream: https://github.com/not-fl3/macroquad, Crate bzw. Tag 0.4.16)
- Herkunft laut `js/README.md` der Crate: Bundling aus miniquad `js/gl.js`, quad-snd `js/audio.js`, sapp-jsutils und quad-net, danach minifiziert
- Lizenz der Datei und der Crate: MIT OR Apache-2.0

## CI und Pages

- `.github/workflows/ci.yml` (Workflow `gate`): `cargo test`, WASM-Build nach `dist/`, `scripts/gate-check.sh`. Läuft bei jedem Push und PR.
- `.github/workflows/pages.yml` (Workflow `pages`): baut wie das Gate und deployt `dist/` auf GitHub Pages. Im Template nur manuell (`workflow_dispatch`); Pages muss im eigenen public Repo unter Settings > Pages > Source "GitHub Actions" eingeschaltet sein.

## Dokumente

- `AGENTS.md`: Aufbau, Befehle und Regeln für Coding-Agents
- `HANDOFF.md`: Übergabe-Vorlage für den Swap
- `SWAP.md`: Swap-Checkliste (Tags `handoff` und `final`, Branch `swap`)
- `CREDITS.md`: Mitwirkende mit Name und Link
