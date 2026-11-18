# Swap-Checkliste

Technischer Ablauf für Team-Repos aus diesem Template. Zeiten und Rotation stehen im Ablauf des Jams.

## Kickoff (Ursprungsteam)

- [ ] Repo aus dem Template in `draht-dev` erzeugt (Name laut Orga, z. B. `team-01`), public, Lizenz MIT bleibt
- [ ] Alle Teammitglieder als Collaborators eingetragen
- [ ] Gate-CI (Actions-Tab, Workflow `gate`) auf `main` einmal grün
- [ ] Namen und Links in `CREDITS.md`

## Handoff (Ursprungsteam)

- [ ] `HANDOFF.md` ausgefüllt und auf `main` gepusht
- [ ] Gate-CI-Stand notiert (grün oder rot, rot ist erlaubt, aber in `HANDOFF.md` erklärt)
- [ ] Tag setzen und pushen:
  ```sh
  git tag handoff
  git push origin handoff
  ```
- [ ] Danach keine Commits mehr vom Ursprungsteam

## Swap (empfangendes Team)

- [ ] Collaborator-Zugang erhalten (oder Fork angelegt)
- [ ] `HANDOFF.md`, `AGENTS.md`, `README.md` gelesen, `cargo test` lokal laufen lassen
- [ ] Auf Branch `swap` arbeiten, ab dem Tag `handoff`:
  ```sh
  git fetch --tags
  git switch -c swap handoff
  git push -u origin swap
  ```
- [ ] Eigene Namen und Links in `CREDITS.md` ergänzen

## Final (empfangendes Team)

- [ ] Letzter Push auf `swap`, Gate-CI auf `swap` abgewartet
- [ ] Tag setzen und pushen:
  ```sh
  git tag final
  git push origin final
  ```
- [ ] Leistung von Phase 2 ansehen: `git diff handoff...swap` oder auf GitHub `compare/handoff...swap`

## Pages (optional, im eigenen public Repo)

- [ ] Settings > Pages > Source: "GitHub Actions"
- [ ] Workflow `pages` manuell starten (Actions > pages > Run workflow), Branch `swap` oder `main` wählen
