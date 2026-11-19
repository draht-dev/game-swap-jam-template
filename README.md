# game-swap-jam-template

Rust-Crate mit macroquad 0.4 (gelockt 0.4.16). Lizenz: MIT.

Ein minimales Beispielspiel: ein Rechteck bewegt sich mit Pfeiltasten oder WASD und sammelt Punkte. Die Spiellogik liegt in `src/game.rs` und hat keine Abhängigkeit zu macroquad. Rendering und Eingabe stehen in `src/main.rs`.

## Voraussetzungen

- Rust über rustup, Target `wasm32-unknown-unknown`
- Linux (nativer Build): `pkg-config`, `libx11-dev`, `libxi-dev`, `libgl1-mesa-dev`, `libasound2-dev`

## Ton

macroquad läuft mit dem Feature `audio`. `src/sfx.rs` erzeugt kurze Soundeffekte als WAV-Bytes im Code (keine Asset-Dateien), `src/main.rs` spielt beim Einsammeln einen Ton. Im Browser ist Ton erst nach der ersten Eingabe hörbar (Autoplay-Regel der Browser).

`.cargo/config.toml` setzt für `wasm32-unknown-unknown` das Linker-Flag `--allow-undefined`, weil quad-snd 0.2.8 seine JS-Importe sonst ab Rust 1.96 nicht mehr linkt (Quelle: https://blog.rust-lang.org/2026/04/04/changes-to-webassembly-targets-and-handling-undefined-symbols/).

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
- Lokaler Patch (eine Stelle, quad-net-Teil am Dateiende): Upstream weist dort `register_plugin=function(e){...}` ohne Deklaration zu. Weil die Datei mit `"use strict"` beginnt, wirft der Browser beim Laden `Uncaught ReferenceError: register_plugin is not defined`, und das Plugin `quad_net` wird nicht registriert. Patch: `function i(){}var register_plugin=function(e){...};miniquad_add_plugin({register_plugin,...})` (`var` davor, `;` statt `,` danach). Prüfen: `node --check web/mq_js_bundle.js`, danach im Browser keine `Uncaught`-Zeile in der Konsole. Bei einem Update der Datei den Patch erneut anwenden.

## CI und Pages

- `.github/workflows/ci.yml` (Workflow `gate`): `cargo test`, WASM-Build nach `dist/`, `scripts/gate-check.sh`, `scripts/headless-check.sh` (lädt `dist/` in headless Chrome, rot bei `Uncaught`-Konsolenfehler). Läuft bei jedem Push und PR.
- `.github/workflows/pages.yml` (Workflow `pages`): baut wie das Gate und deployt `dist/` auf GitHub Pages. Im Template nur manuell (`workflow_dispatch`); Pages muss im eigenen public Repo unter Settings > Pages > Source "GitHub Actions" eingeschaltet sein.

## Dokumente

- `AGENTS.md`: Aufbau, Befehle und Regeln für Coding-Agents
- `HANDOFF.md`: Übergabe-Vorlage für den Swap
- `SWAP.md`: Swap-Checkliste (Tags `handoff` und `final`, Branch `swap`)
- `CREDITS.md`: Mitwirkende mit Name und Link
