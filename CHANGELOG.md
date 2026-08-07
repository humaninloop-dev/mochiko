# Changelog

All notable changes to the mochiko plugin. One entry per `plugin.json` version bump —
appending here is release gate 4 (`.mochiko/memory/governance-ledger.md`, GI-010/GI-012).
Entries before 0.53.0 predate this file; their history lives in `ROADMAP.md` stamp lines,
`DECISIONS.md`, and git log.

## [0.55.0] — 2026-08-07

- Two native output styles shipped in `output-styles/` (new plugin surface): **Caveman**
  (terse register, baked full level) and **Caveman BLUF** (answer-first BLUF structure +
  caveman diction, BLUF-wins conflict rule). Both `keep-coding-instructions: true`, no
  `force-for-plugin` — user-selectable via `/config` → Output style; main conversation only,
  pipeline reports/artifacts untouched (`templates/output-style.md` still governs those).
- Router skill gains an Output-styles discoverability section (pure addition).
- Design: `.mochiko/brainstorms/plugin-output-styles-delivery/record.md` (D1–D6,
  solo-cold-reviewed 9/9 dispositioned). Author≠grader audit PASS round 1, all three
  artifacts.

## [0.54.0] — 2026-08-06

- `specify.md` gains its missing KM landing Bindings line (governance v1.0.0 validator
  finding — the pin named specify landings but the command carried no reference; pure
  addition, author≠grader audit PASS round 1; pin deviation line struck).
- `marketplace.json` synced 0.10.0 → 0.54.0 — first execution of release gate 5 (GI-016).
- Governance surface set v1.0.0 ratified via first in-repo `/mochiko:setup` run (brownfield):
  CLAUDE.md governance region · governance ledger · KM pin ratified into ruled core ·
  release gates adopted · CHANGELOG elective adopted (this file) ·
  `.claude/settings.local.json` gitignored (GI-015 fix).

## [0.53.0] — 2026-08-05

- Code-minimalism ladder + review lens: `patterns-code-minimalism` + `review-code-minimalism`
  skills minted (26→28); staff/qa personas widened; implement lens wiring. (Pre-CHANGELOG
  entry, reconstructed from the ROADMAP stamp line.)
