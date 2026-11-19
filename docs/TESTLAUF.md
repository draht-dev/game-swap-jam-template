# Testlauf: Setup- und Build-Zeit

Ziel (ops#99): messen, wie lange ein Team vom frischen Klon bis zum grünen Gate braucht, und ob die Build-Zeit im Jam-Ablauf realistisch ist.

## Messen

```sh
./scripts/testlauf.sh                 # misst einen frischen Klon dieses Repos
./scripts/testlauf.sh <REPO-URL>      # misst ein aus dem Template erzeugtes Repo
```

Das Skript klont in einen Temp-Ordner, nutzt ein leeres `CARGO_HOME` (kalter Crate-Cache) und ein eigenes `target/`, und gibt eine Tabelle aus. Nicht gemessen: Installation von rustup/Rust selbst und der Linux-Pakete aus dem README (einmalig vor dem Jam).

## Lauf 1: Box (kein frischer Laptop), 2026-11-19 14:26 MEZ

Umgebung: Linux x86_64, 8 Kerne, rustc 1.100.0, Rust-Toolchain und wasm32-Target schon installiert. Branch `cp8-input-check` (Stand PR #7).

| Schritt | Zeit | Status |
|---|---|---|
| git clone | 0.1 s | ok |
| rustup target add wasm32 (falls fehlt) | 0.1 s | ok |
| cargo fetch (kalter Crate-Cache) | 1.8 s | ok |
| cargo test (kalt) | 4.9 s | ok |
| Web-Build (kalt, release wasm32) | 5.0 s | ok |
| gate-check.sh | 0.0 s | ok |
| headless-check.sh | 41.2 s | ok |
| Web-Build (warm, nach Änderung in game.rs) | 0.5 s | ok |
| cargo test (warm) | 0.3 s | ok |

Ergebnis: Vom Klon bis zum grünen Gate rund 53 s, davon 41 s Headless-Check (zwei Chrome-Läufe mit je bis zu 20 s Timeout). Ein Rebuild nach einer Änderung an der Spiellogik dauert unter 1 s. Die Build-Zeit ist für den Jam kein Engpass.

## Offen

- Lauf 2 auf einem frischen Laptop (ohne vorinstalliertes Rust), inklusive rustup-Installation und Linux-Paketen, Zeit hier eintragen.
- Lauf 3 mit einem aus dem Template erzeugten Repo, sobald das Repo als Template markiert ist.
