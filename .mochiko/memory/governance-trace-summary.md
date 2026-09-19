# Trace summary — governance surface set v3.1.0 (2026-09-14, AM-3)

The producer's manifest over the ratified synthesis (`governance-intent.md`, GI-001–022). Grading
surface for `validation-constitution`. Superseded rounds: v2.0.0 and v3.0.0 carried their
manifests inside their producer reports
(`.mochiko/brainstorms/cli-schema-delivery/reports/wave2-gov-producer.md` for v3.0.0), and the
v1.0.0 manifest this file held — including its ruled FP-1 — is in git history.

## Trace summary — GI element to homes

| GI-ID | Principle / element | Source | Primary home | Companions present |
|-------|--------------------|--------|--------------|--------------------|
| GI-001 | Fact profile — no compliance modules, negatives incl. no-UI; unchanged at AM-3 | session fact profile | region ratified stamp | ledger header ✓ · **stamp re-versioned v3.1.0** |
| GI-002 | Identity · risk surface (hook vector widened at AM-3) · team reality | session dimensions 1–5 | region Technology stack line | ledger amendment-policy trigger ✓ — **C5 wave-4 precondition added**; no Three-Part entry (FP-3 at AM-2, accepted) |
| GI-003 | Secrets out of the repo | floor-asserted: FLOOR-SEC | region line (NON-NEGOTIABLE) | index = line ✓ · ledger ✓ — untouched at AM-3 |
| GI-004 | Primitive audit ratchet | floor-asserted: FLOOR-TEST | region line (NON-NEGOTIABLE) | index = line ✓ · ledger ✓ · `rust-cli.md` quality-gate bullet ✓ — untouched (Card 7) |
| GI-005 | Record-layer integrity | floor-asserted: FLOOR-ERR | region line (NON-NEGOTIABLE) | index = line ✓ · ledger ✓ · `rust-cli.md` log-is-truth bullet ✓ — untouched (Card 7) |
| GI-006 | Traceability as observability | floor-asserted: FLOOR-OBS | region line (NON-NEGOTIABLE) | index = line ✓ · ledger ✓ — untouched (Card 7); the rule this round's preserved prior-text paragraph serves |
| GI-007 | Deliberate exclusions, narrowed to markdown primitives | session dimension 10 | ledger GI-004/005/006 trace lines | synthesis ✓ |
| GI-008 | Waiver — FLOOR-TEST for the skill-shipped helpers | waiver | ledger waiver table | synthesis ✓ — row and both notes untouched |
| GI-009 | Knowledge-management core | module: knowledge-management-core | `.mochiko/memory/knowledge-management.md` | region Governance-operations line ✓ · ledger ✓ · `operating-docs.md` ✓ |
| GI-010 | CHANGELOG elective | module: km-elective-changelog | `CHANGELOG.md` | ledger ✓ · region release-gates line, gate 4 ✓ · `rust-cli.md` `paths` now reaches `CHANGELOG.md` (FP-5) ✓ |
| GI-011 | RUNBOOK elective — declined durable | module decline | ledger declined-durable line | synthesis ✓ |
| GI-012 | Release gates | module: release-gates | ledger GI-012 (detail) | region gates line 1 **amended (C5 clause)** ✓ · region release-tag line verbatim ✓ · ledger **wave-4 precondition (with its M2 pre-authorized-PATCH expiry at both homes) + C9 cost watch + Testability limbs + trace extended** ✓ · `rust-cli.md` gate and release bullets verbatim, `paths` now reaching the bump's own files (FP-5) ✓ |
| GI-013 | layer-rules — declined durable | module decline | ledger amendment-policy line | ✓ |
| GI-014 | evolution-notes — declined durable | module decline | ledger amendment-policy line | ✓ |
| GI-015 | Live-token confrontation | brownfield confrontation | ledger confrontation table | folded into GI-003 enforcement ✓ |
| GI-016 | Marketplace-lag confrontation | brownfield confrontation | ledger confrontation table | folded into GI-012 gate 5 ✓ |
| GI-017 | Pointer-only region | minted | region line | index = line ✓ · ledger ✓ — binds this round's routing: every region clause added is a pointer, no home restated |
| GI-018 | ARCHITECTURE.md lag accepted | brownfield confrontation | ledger confrontation table | ✓ |
| GI-019 | Kernel-class tooling admission (bright line) | minted AM-1 · widened AM-2 · **conformance-gate admission AM-3** | `CLAUDE.md` `## Non-negotiable constraints` prose — **trace parenthetical extended** | region index line **amended** ✓ · ledger **7 Enforcement bullets + preserved prior `rust-cli.md` text + trace extended** ✓ · ledger **Testability re-keyed into two tiers** — assertable at ratification (dependency-halt limbs + `check` exit 4) vs **dormant until the wave-4 hook ship** (the conformance limb, activated at that bump as a pre-authorized PATCH) ✓ · `rust-cli.md` header + bright-line bullet **rewritten**, `paths` **widened by three globs (FP-5)** ✓ |
| GI-020 | Clone-only install, required `mochiko-cli` | minted AM-1, superseded-by-ruling AM-2 | `CLAUDE.md` `## Non-negotiable constraints` prose | region index line ✓ · ledger ✓ · `rust-cli.md` dependency bullet ✓ — **untouched at AM-3 (Card 7)**; reconciled with D7e inside GI-019's scope-of-guarantee clause |
| GI-021 | Depth level `high` | minted AM-1 | region ratified stamp | ledger header ✓ — unchanged |
| GI-022 | No repo-level feature map for this repo | minted AM-2 | region index line | ledger ✓ — unchanged |

## Trace summary, reverse — changed surface line to GI element

| Surface | Changed line | GI |
|---------|--------------|-----|
| `CLAUDE.md` prose (outside the markers) | kernel-class paragraph, trace parenthetical gains the conformance-gate admission pointer | GI-019 |
| region | ratified stamp v3.0.3 → v3.1.0 · 2026-09-14 (AM-3) | GI-001, GI-021 |
| region | kernel-class index line gains the conformance-gate admission pointer | GI-019 |
| region | release-gates line gains the wave-4 hook-ship precondition clause | GI-012, GI-002 |
| region | amend-triggers line: the two owed controls named as wave-4 hook-ship preconditions | GI-002, GI-012 |
| region | path-scoped-rules read line names `.claude-plugin/` (both manifests) and `CHANGELOG.md` (residual R2) | GI-012, GI-010, GI-019 |
| `rust-cli.md` | header — two recorded rulings become three | GI-019 |
| `rust-cli.md` | bright-line bullet rewritten (mechanical conformance admitted; grading of meaning/quality forbidden; two grounds; explicit allow) | GI-019 |
| `rust-cli.md` | log-is-truth bullet's record shorthand qualified to `cli-schema-delivery` (FP-3) | GI-005, GI-006 |
| `rust-cli.md` | `paths` gains `plugins/mochiko/.claude-plugin/plugin.json` · `.claude-plugin/marketplace.json` · `CHANGELOG.md` (FP-5, validator B1) | GI-012, GI-010 |
| ledger | version 3.0.3 → 3.1.0 | GI-001 |
| ledger | amendment policy, semver MINOR limb gains "a principle significantly expanded" | amendment policy (no GI — disclosed lead addition) |
| ledger | amendment policy, first-public-release paragraph gains the wave-4 precondition | GI-002, GI-012 |
| ledger | GI-019 — admission ruling · clause (iv) with its C1 condition · amnesty · reach · what the gate reads · hook floor with the explicit-allow rule · scope of the guarantee | GI-019 |
| ledger | GI-019 — the prior `rust-cli.md` bullet preserved verbatim as a recorded supersession | GI-019, GI-006 |
| ledger | GI-019 — Testability re-keyed into two tiers, the conformance limb dormant until the wave-4 hook ship; trace line extended | GI-019 |
| ledger | GI-012 — wave-4 hook-ship precondition paragraph | GI-012, GI-002 |
| ledger | GI-012 — gate note gains the ≤ 60 s aggregate hook-cost watch (FP-1) | GI-012 |
| ledger | GI-012 — Testability Pass/Fail limbs (FP-2); trace line extended | GI-012 |
| ledger | amendment-log row 3.1.0 + the AM-3 addendum | all of the above |
| this file | regenerated whole at v3.1.0 from a stale v1.0.0 manifest; superseded-rounds pointer added (FP-4) | GI-006 |

## Floor coverage

All four Essential Floor categories accounted for, none dropped and none re-expressed at AM-3:
GI-003 (FLOOR-SEC) · GI-004 (FLOOR-TEST) · GI-005 (FLOOR-ERR) · GI-006 (FLOOR-OBS). Module set
unchanged: compliance none · knowledge-management (core + CHANGELOG) · release-gates.

**Flagged proposals** — FP-1 to FP-4 authored on the lead's ruling at the plan gate, FP-5 at the
lead-scoped fix round over the validator's blocking finding B1; each still the user's to overturn
at acceptance, each reversible by a single edit:

- **FP-1** — GI-012's gate note carries the ≤ 60 s aggregate per-run hook-cost cap as a D10 watch
  by ruling, never a bump gate. The synthesis rules C9 under GI-002's risk surface, and GI-002 has
  no Three-Part entry, so without this the ruling would live only in the synthesis. It is the
  second GI-012 touch beyond Card 7's C5 clause. *Alternative:* synthesis-only.
- **FP-2** — GI-012's Testability gains one Pass limb and one Fail limb for the wave-4
  precondition, because the three-part rule requires Testability to cover Enforcement and the
  synthesis names only the enforcement clause. *Alternative:* the clause without a testable form.
- **FP-3** — `rust-cli.md`'s log-is-truth bullet is qualified to `(record `cli-schema-delivery`
  D1/D2/D6)`. This run's header rewrite introduces a second driver record whose own D1/D7 sits
  three lines above, so the bare shorthand would point at the wrong record — an ambiguity this run
  creates. Outside Card 4's literal scope; lead-ruled at the plan gate. *Alternative:* leave the
  bullet byte-identical and carry the ambiguity.
- **FP-4** — this file's superseded-rounds pointer sentence, so regenerating Shape 4 whole does
  not take the v1.0.0 manifest and its ruled FP-1 dark (GI-006). *Alternative:* no pointer;
  recovery from git alone.
- **FP-5** — `rust-cli.md`'s `paths` gains `plugins/mochiko/.claude-plugin/plugin.json`,
  `.claude-plugin/marketplace.json` and `CHANGELOG.md` (all three verified on disk). Every GI-012
  gate fires at the `plugin.json` bump and AM-3 puts the set's strictest MUST NOT there, so a
  globs-honest reading has to cover the bump's own files; `CHANGELOG.md` is GI-010's home. Same
  reasoning AM-2 used to add `.github/workflows/**` (FP-1/FP-2 there). Authored at the validator's
  blocking finding B1. *Alternative:* keep the five ratified globs and leave the wave-4 precondition
  with no touch-time carrier at the files it gates.

**Waivers:** GI-008 (sole) — row and notes unchanged at AM-3.
**Narrowings:** GI-003 secret-scanning, recorded in its ledger entry and un-narrowed at v0.76.0.
**Owed at the next PATCH:** the ledger's semver MAJOR limb lacks the template's `depth-level flip
(low→high)` clause — pre-existing, outside AM-3's scope, not authored here.
