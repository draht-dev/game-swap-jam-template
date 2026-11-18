# game-swap-jam-template

Rust-Crate mit macroquad 0.4. Lizenz: MIT.

## Voraussetzungen

- Rust über rustup, Target `wasm32-unknown-unknown`
- Linux (nativer Build): `pkg-config`, `libx11-dev`, `libxi-dev`, `libgl1-mesa-dev`, `libasound2-dev`

## Build

```sh
cargo build
cargo build --target wasm32-unknown-unknown
```

Nativ starten: `cargo run`

Das WASM-Modul liegt danach unter `target/wasm32-unknown-unknown/debug/game_swap_jam_template.wasm`.

HTML-Hülle und Pages-Deploy sind nicht Teil dieses Stands.
