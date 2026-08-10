# Strip notes — `skills/grooming-operating-docs/`

Entry formats: `strips/README.md`.

**Wave context (v0.44.0 — the D7 leakage scrub).** `verbosity-caveman-ops-separation` D7 as
folded at review (S4): **full scrub** of ops leakage from the shipped tree, with no
changelog-worthy detail lost — every removed block is preserved verbatim below. Ruling:
`DECISIONS.md` 2026-08-01 "Output verbosity, caveman & ops separation ruled" row.

**The leak test this wave used, recorded so a future sweep inherits it: *whose artifact does the
pointer name?*** Mochiko's own ops records — `.mochiko/strips/`, `.mochiko/brainstorms/`,
`.mochiko/decisions/`, `.mochiko/archive/` — are leaks: they resolve to nothing in an installed
plugin. Adopter runtime paths (`.mochiko/specs/`, `.mochiko/memory/`) and the KM module's
document contracts are the **user's** artifacts and are untouchable. A prefix-based sweep on
`.mochiko/` would gut the KM module and the brainstorm command; 101 of this tree's 146
`.mochiko/` references were correctly left alone on that test.

## [v0.64.0] Slim description (guardrails-vs-detail Wave 2 editorial cut) — body no-op
- **Disposition:** superseded → Wave 2 editorial guardrails cut (D4 cut line). Description only;
  the body carries no D4-class content and is unchanged.
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail Wave 2, `DECISIONS.md`
  2026-08-11 build row Wave 2 residual + user rulings 2026-08-10/11; method warrant: benchmark
  verdict `.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md`).
- **Content (faithfully compressed):** description 1,068 → 490 chars (−54%). **Body 2,666 →
  2,666 (0%) — deliberate no-op** (the batch's grooming special case): the body has no "When to
  Use" section, no restatement of the description, and no worked example — the Overview, the
  8-step Procedure, and the Boundaries are all owned craft whose obligations survive nowhere
  else, so there is nothing to delete. Description cut: the trip-condition parentheticals
  (Now/Next/Later, per-open-item, the index-triple spelled out), the "attached to already-firing
  command boundaries … never dependent on the user remembering to groom" provenance sentence,
  several SHOULD trigger phrases, and the "the subtractive landing ritual itself is the commands'
  job" clause compressed; the MUST clause, the trip-condition list, core triggers, the
  `knowledge-management.md` source-of-truth + no-copy-nothing-to-groom rule, and "compresses and
  moves, never deletes" kept. Verbatim homes: git history of this file (pre-v0.64.0).
- **Old description (verbatim):**
  > This skill MUST be invoked when a knowledge-management invariant cap or bound trips at a command boundary — a ROADMAP.md horizon cap (Now/Next/Later), the BACKLOG.md per-open-item size bound or open-item-count watch, a dead pointer, a status disagreement across the brainstorms index / record / decisions index, or a `[x]` item found in BACKLOG.md — restoring the operating docs to their shape contracts under fix-on-sight. SHOULD also invoke on "groom the operating docs", "merge backlog sections", "compress superseded decisions", "re-rank Now/Next/Later", or an "are these items already delivered?" delivery sweep over open BACKLOG.md items. Attached to already-firing command boundaries (brainstorm open/close, setup/amend, specify/plan/implement landings) — never dependent on the user remembering to groom. Resolves every cap, bound, and format from the project-pinned copy at `.mochiko/memory/knowledge-management.md`; no copy → nothing to groom. Compresses and moves, never deletes; the subtractive landing ritual itself is the commands' job, not this skill's.
- **Kept deliberately:** the entire body — the Overview, the 8-step Procedure (read the pinned
  copy → fix tripped invariant → delivery sweep → BACKLOG → ROADMAP → DECISIONS → stamp →
  expansion-heavy-surface watch), and the Boundaries section. The description keeps the MUST
  trip-condition trigger, the source-of-truth pointer, and the compress-move-never-delete rule.
- **KEPT reconciliation:** the [v0.44.0] supersessions below record content deleted from the
  shipped file (preserved verbatim there); none is a body line this cut touches (body no-op). No
  prior KEPT or protected line is touched.
- **Consumers assessed:** mochiko router (the sole citing surface; indexes the skill by name,
  links no description clause or body section anchor). Contract intact.

## [v0.44.0] Delivery-sweep source list: the `.mochiko/strips/` entry
- **Disposition:** superseded → deleted from the shipped file; preserved verbatim here.
- **Tier failed:** n/a — supersession by ruling (`verbosity-caveman-ops-separation` D7 + S4; the `DECISIONS.md` 2026-08-01 row above)
- **Content (verbatim):**
```
trail, `.mochiko/strips/`, the decisions layer, git history, and the primitives on disk.
```
- **Kept deliberately:** the other four sweep sources — trail, decisions layer, git history, primitives on disk — all of which exist in an adopter project. The strips directory never did: it is mochiko-repo-only, so the line sent an adopter's groom run at a path that cannot exist.

## [v0.44.0] Report-writer re-open trigger's session-slug citation
- **Disposition:** superseded → deleted from the shipped file; preserved verbatim here.
- **Tier failed:** n/a — supersession by ruling (`verbosity-caveman-ops-separation` D7 + S4; the `DECISIONS.md` 2026-08-01 row above)
- **Content (verbatim):**
```
(`model-tiered-seats` D3)
```
- **Kept deliberately:** the whole trigger — an expansion-heavy-surface hit is logged as a BACKLOG item for the user and never acted on in the skill.
