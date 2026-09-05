# AM-2 — governance producer report (surface set v3.0.0)

**Seat:** gov-producer · **Skill:** `mochiko:authoring-constitution` · **Date:** 2026-09-04
**Input:** the ratified synthesis `.mochiko/memory/governance-intent.md` (ratified 2026-09-04 by
Deepesh), with `.mochiko/brainstorms/cli-schema-delivery/wave2-amendments.md` as the lead's
verbatim-where-marked proposal. Where the two differed, the synthesis won — three known
differences, all resolved to the synthesis: the log's ruled home
(`plugins/mochiko/migrations/` from wave 3), the conditional first-public-release discharge, and
the GI-012 additions (SKIPPED blocks, the crate release train, the ToS substrate mark).
**Status:** authored on a lead-approved plan; awaiting the independent
`validation-constitution` grade and the user's acceptance gate.

## 1 — What was written

| File | Regenerated | Preserved |
|------|-------------|-----------|
| `CLAUDE.md` | the region between `<!-- mochiko:governance:begin -->` / `<!-- mochiko:governance:end -->`; outside it, the `## Non-negotiable constraints` paragraph now titled "Plugin install stays clone-only" and the kernel-class paragraph's trace parenthetical | everything else byte-for-byte, incl. the `<!-- mochiko:output-style -->` block verbatim (carve-out) |
| `.claude/rules/mochiko/rust-cli.md` | full rewrite (stale on every bullet: renderer-only, additive install, D8 fallback, 8-template scope) | — |
| `.mochiko/memory/governance-ledger.md` | version line · amendment-policy triggers · waiver note · GI-004 · GI-005 · GI-006 · GI-012 · GI-019 · GI-020 · new GI-022 · amendment-log row + addendum | header floor/depth line · waiver row · semver policy · approvers · declined-durable line · exception registry · GI-003 · GI-009 · GI-010 · GI-017 · confrontation rulings · floor-status paragraph · AM-1 addendum |

Not touched: `operating-docs.md`, `primitive-edits.md`, `output-style.md`, every `plugins/mochiko/`
primitive, `plugin.json`. No commits. `cargo test --all`: 300 passed, 0 failed.
`git diff --stat` beyond this report: `CLAUDE.md`, `.claude/rules/mochiko/rust-cli.md`,
`.mochiko/memory/governance-ledger.md` — plus `.mochiko/memory/governance-intent.md` and the
untracked `.mochiko/product/` scaffold, neither of which this seat wrote.

Two lead corrections applied throughout. Every surface line about the log's home speaks in the
ratified end-state tense ("the migration log the plugin carries from wave 3 at
`plugins/mochiko/migrations/`", with today's repo-root reality stated), never "shipped in the
plugin" as present fact. The region stays pointer-only under GI-017: no line restates the
migration grammar, the ruling-anchor rule, or the hook mechanics — grepping `CLAUDE.md` for
`SessionStart`, `UserPromptExpansion`, `PreToolUse`, `ruling anchor`, `class: floor`, `kind: fail`
and `hash:` returns zero hits.

## 2 — Trace summary (Shape 4): GI element → homes

| GI-ID | Element | Source | Primary home | Companions |
|-------|---------|--------|--------------|------------|
| GI-001 | Fact profile (no modules) | session fact profile | region ratified stamp | ledger header ✓ |
| GI-002 | Identity · risk surface · team reality | session dimensions 1–5 | region Technology stack line | ledger amendment-policy trigger ✓ (no Three-Part entry — FP-3) |
| GI-003 | Secrets out of the repo | floor-asserted: FLOOR-SEC | region line (NON-NEGOTIABLE) | index ✓ · ledger ✓ |
| GI-004 | Primitive audit ratchet | floor-asserted: FLOOR-TEST | region line (NON-NEGOTIABLE) | index ✓ · ledger ✓ · `rust-cli.md` quality-gate bullet ✓ |
| GI-005 | Record-layer integrity | floor-asserted: FLOOR-ERR | region line (NON-NEGOTIABLE) | index ✓ · ledger ✓ · `rust-cli.md` log-is-truth bullet ✓ |
| GI-006 | Traceability as observability | floor-asserted: FLOOR-OBS | region line (NON-NEGOTIABLE) | index ✓ · ledger ✓ |
| GI-007 | Deliberate exclusions (narrowed to markdown primitives) | session dimension 10 | ledger GI-004/005/006 trace lines | synthesis ✓ |
| GI-008 | Waiver — FLOOR-TEST for the six skill-shipped helpers | waiver | ledger waiver table | AM-2 note ✓ (row unchanged) |
| GI-009 | Knowledge-management core | module: knowledge-management | region Governance-operations line | index ✓ · ledger ✓ · `operating-docs.md` ✓ |
| GI-010 | CHANGELOG elective | module: knowledge-management-elective | ledger GI-010 | region release-gates line (gate 4) ✓ |
| GI-011 | RUNBOOK elective — declined durable | module decline | synthesis only | see observation O-1 |
| GI-012 | Release gates | module: release-gates | ledger GI-012 | region gates lines ×2 ✓ · `rust-cli.md` gate + release bullets ✓ |
| GI-013 | layer-rules — declined durable | module decline | ledger amendment-policy line | ✓ |
| GI-014 | evolution-notes — declined durable | module decline | ledger amendment-policy line | ✓ |
| GI-015 | Live-token confrontation | brownfield confrontation | ledger confrontation table | folded into GI-003 ✓ |
| GI-016 | Marketplace-lag confrontation | brownfield confrontation | ledger confrontation table | folded into GI-012 gate 5 ✓ |
| GI-017 | Pointer-only region | minted | region line | index ✓ · ledger ✓ |
| GI-018 | ARCHITECTURE.md lag accepted | brownfield confrontation | ledger confrontation table | ✓ |
| GI-019 | Kernel-class tooling admission | minted at AM-1, widened AM-2 | `CLAUDE.md` `## Non-negotiable constraints` prose | region index line ✓ · ledger ✓ · `rust-cli.md` bright-line bullet ✓ |
| GI-020 | Clone-only install, required `mochiko-cli` | minted AM-1, superseded-by-ruling AM-2 | `CLAUDE.md` `## Non-negotiable constraints` prose | region index line ✓ · ledger ✓ · `rust-cli.md` dependency bullet ✓ |
| GI-021 | Depth level `high` | minted at AM-1 | region ratified stamp | ledger header ✓ (re-recorded unchanged) |
| GI-022 | No repo-level feature map | minted AM-2 | region index line | ledger ✓ |

Waivers: GI-008 (unchanged). Floor categories: all four accounted for — GI-003 · GI-004 · GI-005 ·
GI-006, each carrying its AM-2 re-expression, none dropped.

## 3 — Trace summary, reverse: changed surface line → GI element

| Surface | Changed line | GI |
|---------|--------------|-----|
| `CLAUDE.md` prose | kernel-class paragraph, trace parenthetical gains the widened admission | GI-019 |
| `CLAUDE.md` prose | "Plugin install stays additive" replaced by "Plugin install stays clone-only; delivery depends on `mochiko-cli`" | GI-020 |
| region | ratified stamp v2.0.1 → v3.0.0 · 2026-09-04 (AM-2) | GI-001, GI-021 |
| region | audit-ratchet line gains the crate-code review pointer | GI-004 |
| region | record-layer line gains the mechanical schema-rule limb pointer | GI-005 |
| region | reconstructibility line gains the migration log | GI-006 |
| region | kernel-class index line gains the widened-admission pointer | GI-019 |
| region | "Additive plugin install" index line replaced by the clone-only line | GI-020 |
| region | new index line: no repo-level feature map | GI-022 |
| region | Technology stack line: crate re-described as kernel-class delivery infrastructure, contract suite named | GI-002, GI-012, GI-020 |
| region | quality-gates line 1: GI-012 activation reference re-worded | GI-004, GI-005, GI-012 |
| region | release-gates line: gate 5 = view ≡ replay, gate 6 = `cargo test` + contract suite, SKIPPED blocks | GI-012 |
| region | new line: the `mochiko-cli-v*` release train | GI-012 |
| region | amend-triggers line: first-public-release fired with a conditional discharge; transition-clause expiry pre-authorized PATCH | GI-002, GI-020 |
| region | standing Read-injection line names `migrations/` and `evals/contract/` | GI-020 (delivery caveat) |
| `rust-cli.md` | whole file, `paths` widened to six globs | GI-019, GI-020, GI-012, GI-004 |
| ledger | version 2.0.1 → 3.0.0 | GI-001 |
| ledger | waiver-table AM-2 note (trigger observation) | GI-008, GI-019 |
| ledger | amendment policy: first-public-release conditional discharge · pre-authorized PATCH clauses | GI-002, GI-020, GI-004 |
| ledger | GI-004 schema-content audit unit, retired count self-check, code author≠grader | GI-004 |
| ledger | GI-005 two regimes named | GI-005 |
| ledger | GI-006 migration log as carrier | GI-006 |
| ledger | GI-012 gates 5/6, SKIPPED, crate train, gate note, ToS substrate mark | GI-012 |
| ledger | GI-019 admission, three-clause argument, advisory-checker placement | GI-019 |
| ledger | GI-020 rewritten superseded-by-ruling, two-tier Testability, revisit trigger, marks | GI-020 |
| ledger | new GI-022 entry | GI-022 |
| ledger | amendment-log row 3.0.0 MAJOR + AM-2 addendum | all of the above |

## 4 — Flagged proposals (ruled by the user at the acceptance gate)

All five were authored as recommended on the lead's instruction; each remains the user's to
overturn, and each is reversible by a single edit.

**FP-1 — `rust-cli.md` `paths` includes `.github/workflows/**`.** The ratified scope named three
data paths. The release-train gate this run adds to GI-012 is violated in `release.yml` and the
crate layers in `ci.yml`, so a globs-honest reading has to cover them. *Alternative:* keep the
three ratified globs; the release train then has no touch-time carrier and lives only in the
ledger.

**FP-2 — `rust-cli.md` `paths` includes `plugins/mochiko/hooks/**`.** GI-019's clause that hooks
block only on absence or grammar skew is violable exactly in the hook file wave 3 authors; the
path does not exist yet, so the glob costs nothing today and reaches that author at touch time.
*Alternative:* omit it and add it at the wave-3 landing.

**FP-3 — no GI-002 ledger entry minted.** GI-002 is a fact element, not a Three-Part principle,
and AM-1 set the precedent of annotating it without an entry. Its AM-2 obligations are routed to
enforceable homes instead: the two owed controls onto GI-012's release train, the conditional
discharge onto the amendment-policy trigger line. *Alternative:* mint GI-002 as a full
Three-Part entry, which would make the identity and risk statements gradeable in their own right.

**FP-4 — GI-022 gets a region index line, not ledger-only.** `setup.fail.no-feature-map` fires in
ordinary command runs that read only the always-on region; ledger-only placement would leave the
ruling invisible where it is consumed. *Alternative:* ledger-only, accepting that a future run
re-surfaces the question.

**FP-5 — the GI-004 and GI-005 region lines each gain one pointer clause.** Both stay pointers,
neither restates its home. The crate's code review and the mechanized schema-rule limb are new
obligations with no other always-on carrier. *Alternative:* leave both lines verbatim and carry
the clauses in the ledger alone.

## 5 — Observations for the validator (not authored, no change made)

**O-1 — GI-011 has no ledger home.** The RUNBOOK elective was declined durable at v1.0.0, but the
ledger's declined-durable line names only GI-013 and GI-014. The decline lives in the synthesis
alone. Pre-existing, outside AM-2's ratified scope, and left untouched. Recommend adding GI-011
to that line at the next PATCH.

**O-2 — GI-020's Testability is deliberately two-tier.** The end-state rows are dormant until the
wave-3 pilot, per review I1 and the AM-1 dormant-clause idiom. A validator reading them as
unassertable claims should read the tier heading above them: what is assertable today is stated
separately and was verified this run (`cargo test --all` 300/300; the log's hard set rejecting
nothing at validate).

**O-3 — the AM-1 GI-020 text is preserved inside the new entry**, as a superseded-by-ruling
note rather than deleted, so the withdrawal of the raw-Read degraded path is reconstructible
(GI-006). This is the record layer's own protected-content rule applied to the ledger.

## 6 — Advisory round (2026-09-04, after the validator's PASS)

The independent `validation-constitution` seat passed the set 60/61 with no blocking finding
(`wave2-reports/gov-validator.md`). Six of its nine advisories were taken in this round, wording
only — no principle, routing, or scope changed.

| ID | Change | Surface |
|----|--------|---------|
| A1 | The GI-002 discharge note no longer calls the first two controls release-train gates; it now says `cargo audit --deny warnings` is one of the four crate layers in GI-012's train. sha256-published assets are a control, not a gate. | ledger, amendment policy |
| A2 | The manual-approval control's "owed" reason is stated precisely: the `crates-io` environment is declared in `release.yml`, its approval rule is a GitHub setting not visible in the tree, and the job is `if: false` today. | ledger, amendment policy |
| A3 | The region's standing new-file read line adds `.github/workflows/`, which `rust-cli.md` now scopes. | region, Governance operations |
| A4 | The wave-6 re-key pre-authorization names its full scope — the ceremony re-keyed from schema strips to migrations, and `primitive-edits.md`'s `paths` gaining `plugins/mochiko/migrations/**` and `plugins/mochiko/hooks/**`. The glob half is recorded as a wave-3 obligation, landing when those directories ship. | ledger, amendment policy |
| A6 | GI-004's trace parenthetical names the three executable gates' homes: the crate's `cargo test` on GI-004, the log's hard set on GI-005, the plugin path's contract suite on GI-012. | ledger, GI-004 |
| A7 | The 3.0.0 addendum names the two `## Non-negotiable constraints` paragraph edits made outside the region under the ratified Card 1 ruling. | ledger, amendment log |

A5 (GI-011 has no ledger home) stays as observation O-1 above, recommended for the next PATCH and
deliberately not authored here — it is outside AM-2's ratified scope. A8 and A9 needed no action.

Re-verified after this round: `git diff --stat` shows `CLAUDE.md`,
`.claude/rules/mochiko/rust-cli.md`, and `.mochiko/memory/governance-ledger.md` plus this report,
and nothing else this seat wrote. `cargo test --all`: 300 passed, 0 failed. No commits.
