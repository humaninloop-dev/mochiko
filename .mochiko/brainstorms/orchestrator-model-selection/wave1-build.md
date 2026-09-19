# Wave 1 — Seat default key build (D7)

**Ruling:** `record.md` D1–D8 as review-amended · `DECISIONS.md` 2026-09-19. **Base:** worktree
`seat-default-key` off `main` (v0.108.0, migration log sequences 1..4). **Target version:**
0.110.0 — 0.109.0 is already claimed on the `primitive-evals-v2` branch; a gap avoids two different
plugins carrying one version. **Migration sequence:** 0007 — 0005 and 0006 are both claimed on that branch
(`0005-artifact-homes`, `0006-leads-pen-no-patched-copy`); gaps are legal, collisions are not.
*(Renumbered from 0006 mid-wave on the lead's re-check of the branch's log; rule content unchanged.)* **Floors:** sound-loop (seats produce on lead-approved plans,
a non-author seat grades, the user rules) · transport (single writer per surface, all messaging
through the lead, fan-in confirmed) · primitive-edits ceremony (strip → record → audit).

## Seats and ownership (strictly disjoint)

| Seat | Persona | Owns |
|---|---|---|
| P1 | `mochiko:staff-engineer` | `plugins/mochiko/migrations/0007-seat-default-key.yaml` · `.mochiko/schema-views/**` (regenerated) · `evals/contract/expected-skills.json` (`floor_ids`/`floor_pin` of `patterns-model-tiering` only) · `plugins/mochiko/skills/patterns-model-tiering/SKILL.md` (description · tagline · Overview) · `.mochiko/strips/patterns-model-tiering.md` · `plugins/mochiko/skills/mochiko/SKILL.md` (the tiering row) · `.mochiko/strips/mochiko.md` |
| P2 | `mochiko:staff-engineer` | `plugins/mochiko/agents/{requirements-analyst,technical-analyst,product-manager,product-engineer}.md` (frontmatter `model:` only) · their four `.mochiko/strips/<persona>.md` files · `ARCHITECTURE.md` line 60 (per the operating-docs rule's legacy/derived branch) · any other repo prose asserting "all personas are opus" (report, edit only README-class prose) |
| V1 | `mochiko:validator` | grades P1's pair — `SKILL.md` + the rendered rules — under the skill-pair criteria block of `.claude/rules/mochiko/primitive-edits.md`, plus `mochiko-cli migrate validate --report`, the char-budget pre-assert with the argued overage, the contract suite `--host-only` result, and the router row + strips |
| V2 | `mochiko:validator` | grades P2's edits — internal coherence, preserved responsibilities, strip entries, plugin.json agent list untouched |
| Lead | — | `CHANGELOG.md` · `plugins/mochiko/.claude-plugin/plugin.json` · `.claude-plugin/marketplace.json` · `.mochiko/memory/primitive-cost-budgets.md` (budget row after V1's overage ruling) · operating docs · this plan · the session record |

## P1 — content of migration 0007 (the plan P1 refines; the ruling it executes is fixed)

Header `anchor: 2026-09-19 orchestrator-model-selection D1`; per-rule anchors as noted.

1. `supersede-rule` `patterns-model-tiering.rostered-seats-never-retier` — disposition: superseded by the seat default key (D1–D3); anchor D1.
2. `mint-rule` in `sec.scope` — id `patterns-model-tiering.seat-default-key`, `class: floor`, `kind: binding`, anchor D3. Text carries: every rostered persona pins its ruled default tier as an alias in frontmatter; the criterion (does a structurally independent seat stand between this seat's output and the run's verdict); the ten-row assignment — `strong` (`opus`): validator · devils-advocate · tech-lead · qa-engineer · staff-engineer · principal-architect; `down` (`sonnet`): requirements-analyst · technical-analyst · product-manager · product-engineer; held rows argued in the record; a persona pins a tier alias never a full id (D8); `inherit` or an absent `model:` is out of floor (D2).
3. `mint-rule` in `sec.discipline` — id `patterns-model-tiering.seat-deviation-lane`, `class: must`, `kind: latitude`, anchor D2. The lead may deviate any seat per run by passing `model:` on the spawn only when deviating; reason in the seat's brief; named in the run report's roster line; nested persona spawns take the default and disclose in the spawning seat's report; deviation is the lead's lane only.
4. `mint-rule` in `sec.discipline` — id `patterns-model-tiering.seat-deviation-bounds`, `class: floor`, `kind: bound`, anchor D5. No seat below `sonnet`; `haiku` is never a seat; a grader or reviewer never runs below the tier its producer ran at in the same run (deviating the build pair down drops the worker rung for that run); one tier per seat per loop, a change is a disclosed respawn; a `fable` deviation names the `refusal` stop reason and the Opus-prompted persona as disclosed risks.
5. `mint-rule` in `sec.discipline` — id `patterns-model-tiering.seat-version-floor`, `class: must`, `kind: bound`. Frontmatter outranks `CLAUDE_CODE_SUBAGENT_MODEL` from Claude Code v2.1.251 (below it the env var silently retiers every seat); `CLAUDE_CODE_SUBAGENT_MODEL_FORCE=1` and an organization `availableModels` substitution override every model choice on both transports and are left as the consumer's own environment, undetected in-run (D2 as review-amended; G1/G12 user-ruled).
6. `mint-rule` in `sec.disclosure` — id `patterns-model-tiering.seat-roster-disclosure`, `class: must`, `kind: duty`. Every run report's seat roster line carries each seat's tier — its default, or the deviation and its reason (G13: single-source here; a command pair carrying its own roster grammar edits its migration section too — P1 checks the six command renders and reports, edits none unless one carries roster grammar).
7. `reword-rule` `patterns-model-tiering.override-is-the-pin` (floor; header anchor covers) — on the dispatch rungs the override is the pin (unchanged clause); on a seat spawn the persona file is the pin — the spawn carries `model:` only as a disclosed deviation; an undisclosed override, or a persona file carrying `inherit` or no `model:`, has failed this floor.
8. `reword-rule` `patterns-model-tiering.class-key-session-tier` (floor) — "Session tier" → "Seat tier"; the parenthetical `(model-tiered-seats D5; …)` → `(orchestrator-model-selection D1/D3; …)`; the id survives.
9. `reword-rule` `patterns-model-tiering.worker-seat-set-reserved` (floor) — the closing parenthetical `(model-tiered-seats D5)` → `(orchestrator-model-selection D3: both build seats are `strong`)`.
10. `reword-section` `sec.scope` (title "Scope — dispatch tier and the seat default", intent) · `reword-section` `sec.reserved` note (retiering reservation now rides `seat-default-key`: the table is ruled, never run-decided; deviation is the lead's disclosed lane).
11. `mochiko-cli migrate stamp` the file · `mochiko-cli migrate validate --report --plugin-root plugins/mochiko` clean · `mochiko-cli views emit --plugin-root plugins/mochiko --out .mochiko/schema-views` · `expected-skills.json`: `floor_ids` = the seven floors (drop `rostered-seats-never-retier`, add `seat-default-key`, `seat-deviation-bounds`), `floor_pin` 7, every byte column untouched · `python3 evals/contract/run.py --host-only` green.
12. `SKILL.md`: description (≤ 1,536 chars) — "Governs dispatch tier only — rostered seats never change model (model-tiered-seats D5)" → the seat default key sentence (rostered personas pin a ruled tier alias — six `opus`, four `sonnet` — the lead deviates only with disclosure; `inherit` never; orchestrator-model-selection D1–D5); tagline and Overview first sentence reworded from "Rostered mochiko personas run on the strong tier and stay there" to the key. The Rules block byte-identical. Strip entries (supersession-by-ruling) in `.mochiko/strips/patterns-model-tiering.md` stamped `[v0.110.0]`.
13. Router row (`skills/mochiko/SKILL.md`, the `patterns-model-tiering` row): "rostered seats never change tier (model-tiered-seats D5)" → the seat default key clause; strip entry in `.mochiko/strips/mochiko.md`.
14. Budget: the payload grows past the 14,696 standing figure; the overage is argued in P1's report for V1 (a genuine new obligation set — the ruling's own floors), never restored prose.

## P2 — personas

1. Four frontmatter flips `model: opus` → `model: sonnet`; nothing else in those files changes. Six strong personas untouched (no stale prose asserts a uniform tier — verified by grep at plan time).
2. Four strip entries, supersession-by-ruling, stamped `[v0.110.0]`, verbatim `model: opus`, `Kept deliberately:` the whole body incl. `## Delegating Cheap Reads`; `Consumers assessed:` `plugin.json` agents list (unchanged), router agent rows (report whether they name a model), the eval kits (the runner is on the eval branch; noted, not edited).
3. `ARCHITECTURE.md:60` "Personas (all `model: opus`)" → per the operating-docs rule: read the file header; if the store (`.mochiko/product/architecture/`) carries ruled content and the file is the derived index, do not hand-edit — report; else edit the phrase to "Personas carrying a ruled default tier — six `model: opus`, four `model: sonnet` (seat default key, 2026-09-19)".
4. grep the repo (outside `.mochiko/brainstorms/`, `.mochiko/strips/`, `.mochiko/archive/`, `CHANGELOG.md`) for prose asserting every persona is opus; report hits; edit README-class prose only.

## Gates before the bump (lead)

`mochiko-cli migrate validate --report` clean · `cargo test -p mochiko-cli` green (crate untouched; still run) · contract suite: `--host-only` green at minimum, full sandbox run attempted (a SKIPPED full run blocks the bump per GI-012 gate 6 — reported, never waved) · V1 + V2 PASS · CHANGELOG entry · `plugin.json` + `marketplace.json` → 0.110.0 · budgets row.

## Deferred

D7 item 4 (persona-kit runner `ARM_MODEL` → per-persona default) — the runner lives on `primitive-evals-v2`, not `main`; carried in BACKLOG's build item for the branch merge.

## Disclosure

`floor: tripped · seats: P1 + P2 (staff-engineer) produced / V1 + V2 (validator) reviewed`.
