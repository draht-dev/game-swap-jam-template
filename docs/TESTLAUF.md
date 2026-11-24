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

## Lauf 1b: Box, Stand main nach #6 bis #8, 2026-11-19 18:25 MEZ

Umgebung wie Lauf 1 (Linux x86_64, 8 Kerne, rustc 1.100.0). Stand `main` (d41eb4e), Eingabe-Check läuft jetzt mit echter Zeit statt `--virtual-time-budget`.

| Schritt | Zeit | Status |
|---|---|---|
| git clone | 0.1 s | ok |
| rustup target add wasm32 (falls fehlt) | 0.2 s | ok |
| cargo fetch (kalter Crate-Cache) | 2.5 s | ok |
| cargo test (kalt) | 4.7 s | ok |
| Web-Build (kalt, release wasm32) | 4.4 s | ok |
| gate-check.sh | 0.0 s | ok |
| headless-check.sh | 23.1 s | ok |
| Web-Build (warm, nach Änderung in game.rs) | 0.4 s | ok |
| cargo test (warm) | 0.2 s | ok |

Ergebnis: Vom Klon bis zum grünen Gate rund 35 s (vorher 53 s), davon 23 s Headless-Check (vorher 41 s). Rebuild weiter unter 1 s.

## Lauf 3: Repo aus dem Template, Box, 2026-11-24 10:31 MEZ

Abnahme ops#99. Das Template ist öffentlich und als Template markiert. Test-Repo `draht-dev/gsj-abnahme-lauf3` (privat) per `gh repo create --template draht-dev/game-swap-jam-template` erzeugt (Stand main, d41eb4e), dann `./scripts/testlauf.sh https://github.com/draht-dev/gsj-abnahme-lauf3.git`. Umgebung: Linux x86_64, 8 Kerne, rustc 1.100.0.

Lauf 3a, ohne Default-Toolchain (rustup installiert, `rustup default` nicht gesetzt): rot bei `rustup target add`, Meldung "no default is configured". Workaround: `rustup default stable`. Mit `rust-toolchain.toml` aus PR #10 läuft derselbe Fall ohne Workaround grün (Lauf 3c).

Lauf 3b, nach `rustup default stable`:

| Schritt | Zeit | Status |
|---|---|---|
| git clone | 0.7 s | ok |
| rustup target add wasm32 (falls fehlt) | 0.1 s | ok |
| cargo fetch (kalter Crate-Cache) | 2.4 s | ok |
| cargo test (kalt, 12 Tests) | 4.6 s | ok |
| Web-Build (kalt, release wasm32) | 4.8 s | ok |
| gate-check.sh | 0.0 s | ok |
| headless-check.sh (inkl. Eingabe-Check) | 23.1 s | ok |
| Web-Build (warm, nach Änderung in game.rs) | 0.4 s | ok |
| cargo test (warm) | 0.2 s | ok |

Ergebnis: Vom Klon bis zum grünen Gate rund 36 s, Rebuild unter 1 s. Die Gate-CI im neuen Repo lief beim Initial Commit von selbst und war nach 74 s grün. Gate rot nachvollziehbar: ein absichtlich falscher Test gibt `cargo test` Exit 101, ein `Uncaught` in `dist/index.html` gibt "Headless-Gate: ROT".

Lauf 3c, Branch `cp11-rust-toolchain` (PR #10) ohne Default-Toolchain: alle Schritte ok, Klon bis grünes Gate rund 31 s.

## Offen

- Lauf 2 auf einem frischen Laptop (ohne vorinstalliertes Rust), inklusive rustup-Installation und Linux-Paketen. Läuft Oskar beim Technik-Check am 28.11. (kein Abnahmekriterium für ops#99 mehr).
