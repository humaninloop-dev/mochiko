# AM-3 — independent validation of the governance surface set v3.1.0

**VALIDATION RESULT: FAIL**

**Seat:** gov-validator · **Skill:** `mochiko:validation-constitution` · **Date:** 2026-09-14
**Graded set:** the CLAUDE.md governance region (lines 106–141) + the user-ruled
`## Non-negotiable constraints` paragraphs at CLAUDE.md:72 and :74 · `.claude/rules/mochiko/`
(4 files: `rust-cli.md` amended; `primitive-edits.md`, `operating-docs.md`, `output-style.md`
untouched) · `.mochiko/memory/governance-ledger.md` v3.1.0. There is no `constitution.md`; none
on disk (confirmed).
**Contract:** `.mochiko/memory/governance-intent.md`, ratified 2026-09-14 by Deepesh.
**Manifest:** `.mochiko/memory/governance-trace-summary.md`, regenerated whole at v3.1.0 (Shape 4).
**Author:** the gov-producer seat. This seat authored none of it and read every input from the
files, never from the producer's report. Single writer of this file.

- **Checklist items:** 59/61 passed · **1 FAILED** · 1 n-a by recorded decline. Core 49 + 2
  selected module fragments (`release-gates` 4 · `knowledge-management` 8). The `layer-rules`
  and `evolution-notes` fragments were not run — declined durable (GI-013, GI-014); the
  brownfield "Evolution Notes present" item is the n-a.
- **Surface integrity:** region markers present at CLAUDE.md:106 / :141 · index → home → ledger
  closes both ways for GI-003, GI-004, GI-005, GI-006, GI-009, GI-010, GI-012, GI-017, GI-019,
  GI-020, GI-022 · all 4 rules files `paths`-scoped with valid globs · **scope coverage
  incomplete — the blocking finding below** · standing new-file read line present at
  CLAUDE.md:139 and re-pathed correctly · no universal principle relocated into a rules file ·
  content outside the markers limited to the two prose paragraphs, both edited under a recorded
  ruling and disclosed in the ledger's AM-3 addendum.
- **Trace closure:** forward — every AM-3 synthesis element reaches a surface line and, where
  principle-bearing, a Three-Part ledger entry. Reverse — every changed surface line reaches a
  synthesis element or a disclosed lead addition. No GI-ID claimed twice. Waivers one-for-one.
  Modules one-for-one.
- **Floor / module accounting:** production floor + depth `high` asserted in the region stamp
  and the ledger header, agreeing at v3.1.0 · compliance modules none, matching the GI-001 fact
  profile (unchanged at AM-3) · template modules match GI-009/GI-010/GI-012 one-for-one,
  declines recorded · all four Essential Floor categories carry a principle (GI-003 · GI-004 ·
  GI-005 · GI-006), none re-expressed this round, none waived · waiver GI-008 unchanged, naming
  no legal-mandate module obligation.
- **Anti-patterns found:** none blocking. Two watched: build-state detail inside a standing
  ledger Enforcement section (A3), and one unmeasured operative term inside a disclosed
  standing-open condition (A4).
- **Version bump:** **MINOR — 3.0.3 → 3.1.0. Agrees** with the synthesis, the ledger version
  line, the amendment-log row, and the region stamp. Reasoning and the dissenting MAJOR reading
  in §5.
- **Issues requiring fix:** 1 blocking, 2 minor. §1 and §6.
- **Advisory:** 9, non-blocking; none changes a ruling. §4.

---

## 1 — Blocking findings

### B1 — Rules-file scope does not reach the surface AM-3's new obligation is violated on

**Checklist item failed:** *Rules-File Scope & Delivery* — "Each rules file's `paths` globs cover
every path whose code can violate the concern — per-layer violation test against the kept
architecture card, not just the mechanism's home layer."

The four rules files' globs, verified from their frontmatter:

| File | `paths` |
|------|---------|
| `rust-cli.md` | `crates/mochiko-cli/**` · `plugins/mochiko/migrations/**` · `plugins/mochiko/hooks/**` · `evals/contract/**` · `.github/workflows/**` |
| `primitive-edits.md` | `plugins/mochiko/{commands,skills,agents,templates,migrations,hooks}/**` |
| `operating-docs.md` | `DECISIONS.md` · `ROADMAP.md` · `BACKLOG.md` · `ARCHITECTURE.md` · `GLOSSARY.md` · `.mochiko/brainstorms/index.md` · `.mochiko/decisions/**` · `.mochiko/archive/**` |
| `output-style.md` | `.mochiko/specs/**` |

Three paths are violable and unscoped by any of them:

- **`plugins/mochiko/.claude-plugin/plugin.json`** (present, `"version": "0.108.0"`). Every one
  of GI-012's gates fires at the bump of this file, and AM-3 adds the strictest obligation in the
  whole set there: "the wave-4 bump that ships the conformance hooks MUST NOT land before the
  first `mochiko-cli` publish carrying all four named supply-chain controls." A maintainer
  editing this file gets no injected rule. `rust-cli.md` itself names the moment — "the
  maintainer-side gate at every `plugin.json` bump" — while not being scoped to the file that
  moment touches.
- **`.claude-plugin/marketplace.json`** (present). Gate 5's sync obligation, folded from the
  GI-016 confrontation, is violated here and nowhere else.
- **`CHANGELOG.md`** (present). This is GI-010's declared **home** — an adopted module principle
  whose home file no rules file scopes. Gate 4 is violated here.

The remedy is precedented inside this same ledger: at AM-2 the producer added
`.github/workflows/**` to `rust-cli.md` as a flagged proposal on exactly this reasoning — "the
release-train gate (GI-012) is violable in the workflows" (ledger, AM-2 addendum). The identical
argument reaches the bump surfaces, and AM-3 sharpens it by adding a MUST NOT keyed to the bump.

**Fix:** extend `.claude/rules/mochiko/rust-cli.md`'s `paths` with
`plugins/mochiko/.claude-plugin/plugin.json`, `.claude-plugin/marketplace.json`, and
`CHANGELOG.md` — or home the release-gate reminder in a rules file scoped to them. One edit; no
ruling changes. If the lead prefers to hold this as a recorded PATCH obligation rather than fix
it in-run (the posture the producer took for the depth-flip gap, A9), that is the user's call to
rule, not this seat's to assume.

**Note on scope:** the gap is pre-existing — gates 1–6 have keyed on `plugin.json` since v1.0.0
and AM-2's validator passed this item. A prior PASS does not license a repeat. The set is graded
whole and now, and this round put a new MUST NOT on the unscoped surface.

---

## 2 — What was checked, and the result

### Trace closure — forward (synthesis element → surface + ledger)

| GI | Element | Surface reached | Three-Part entry | Result |
|----|---------|-----------------|------------------|--------|
| GI-001 | fact profile, unchanged at AM-3 | region stamp (v3.1.0) | ledger header line 3 | ✓ |
| GI-002 | identity · risk surface widened (hook vector) | region Technology-stack line | none by ruling (FP-3 at AM-2, accepted) | ✓ |
| GI-003 | FLOOR-SEC | region line, NON-NEGOTIABLE | ledger, untouched | ✓ |
| GI-004 | FLOOR-TEST | region line, NON-NEGOTIABLE | ledger, untouched | ✓ |
| GI-005 | FLOOR-ERR | region line, NON-NEGOTIABLE | ledger, untouched | ✓ |
| GI-006 | FLOOR-OBS | region line, NON-NEGOTIABLE | ledger, untouched; served by the preserved prior text | ✓ |
| GI-007 | exclusions, markdown-only | ledger GI-004/005/006 trace lines | n-a (exclusion) | ✓ |
| GI-008 | waiver | ledger waiver table, unchanged | n-a (waiver) | ✓ |
| GI-009 | KM core | region Governance-ops line · `operating-docs.md` | ledger + the project pin | ✓ |
| GI-010 | CHANGELOG elective | region gates line (gate 4) | ledger | ✓ (home unscoped — B1) |
| GI-011 | RUNBOOK declined durable | ledger declined-durable line | n-a | ✓ |
| GI-012 | release gates + **C5 precondition** | region gates lines 129/130 | ledger, extended | ✓ |
| GI-013 | layer-rules declined | ledger amendment policy | n-a | ✓ |
| GI-014 | evolution-notes declined | ledger amendment policy | n-a | ✓ |
| GI-015 | live-token confrontation | ledger confrontation table | folded into GI-003 | ✓ |
| GI-016 | marketplace lag | ledger confrontation table | folded into GI-012 gate 5 | ✓ |
| GI-017 | pointer-only region | region line | ledger | ✓ |
| GI-018 | ARCHITECTURE lag accepted | ledger confrontation table | n-a | ✓ |
| GI-019 | **conformance-gate admission** | CLAUDE.md:72 prose · region index line 118 · `rust-cli.md` | ledger, 7 new bullets + preserved prior text + Testability re-key + trace | ✓ |
| GI-020 | clone-only + required binary | CLAUDE.md:74 prose, **untouched** | ledger, untouched | ✓ |
| GI-021 | depth `high` | region stamp | ledger header | ✓ |
| GI-022 | no feature map here | region index line | ledger | ✓ |

Every Trace GI-ID in the set exists in `governance-intent.md`; every one points at a
principle-bearing element; no two principles claim the same ID.

### Trace closure — reverse (changed line → element)

Verified by `git diff HEAD` over the three surfaces. Every changed line reaches an element:

| Surface | Changed line | Element | Sanctioned by |
|---------|--------------|---------|---------------|
| CLAUDE.md:72 prose | trace parenthetical gains the conformance-gate admission pointer | GI-019 | Card 2 |
| region | stamp v3.0.3 → v3.1.0 · 2026-09-14 (AM-3) | GI-001, GI-021 | Card 2 |
| region:118 | GI-019 index line gains the admission pointer | GI-019 | Card 2 |
| region:129 | release-gates line gains the wave-4 precondition clause | GI-012, GI-002 | review C5 |
| region:135 | amend-trigger line names the two owed controls as preconditions | GI-002, GI-012 | review C5 |
| `rust-cli.md` header | two recorded rulings become three | GI-019 | Card 4 |
| `rust-cli.md` bright-line bullet | rewritten; mechanical conformance admitted, grading of meaning/quality forbidden, two grounds, explicit allow | GI-019 | Card 4 (D7d) |
| `rust-cli.md` log-is-truth bullet | `(record D1/D2/D6)` → `(record `cli-schema-delivery` D1/D2/D6)` | GI-005, GI-006 | FP-3, lead-ruled |
| ledger | version 3.0.3 → 3.1.0 | GI-001 | Card 2 |
| ledger | amendment policy, MINOR limb gains "a principle significantly expanded" | no GI — disclosed lead addition | verify R3 |
| ledger | first-public-release paragraph gains the wave-4 precondition | GI-002, GI-012 | review C5 |
| ledger | GI-012 wave-4 precondition paragraph | GI-012 | review C5 |
| ledger | GI-012 gate note gains the ≤ 60 s hook-cost watch | GI-012 | FP-1, review C9 |
| ledger | GI-012 Testability limbs; trace extended | GI-012 | FP-2, FC-4 |
| ledger | GI-019 — 7 Enforcement bullets, preserved prior bullet, Testability re-key, trace | GI-019 | Card 3, Card 4 |
| ledger | amendment-log row 3.1.0 + AM-3 addendum | all of the above | Cards 1–7 |
| trace summary | regenerated whole; superseded-rounds pointer | GI-006 | FP-4 |

Nothing changed that no element reaches. GI-020's paragraph and Three-Part entry are
byte-unchanged, as Card 7 ruled.

### Three-Part completeness — every changed ledger entry

| Entry | Enforcement | Testability | Rationale | Trace |
|-------|-------------|-------------|-----------|-------|
| GI-012 | ✓ gates 1–6 + release train + **wave-4 precondition** | ✓ Pass and Fail limbs both extended (FP-2) | ✓ | ✓ extended (FC-4) |
| GI-019 | ✓ + 7 new bullets | ✓ re-keyed — **see M1**, no dormancy tier | ✓ | ✓ extended |

Every unchanged entry retains all four sections; spot-verified on GI-003, GI-004, GI-005, GI-006,
GI-009, GI-010, GI-017, GI-020, GI-022.

### The named checks

- **Preserved prior text.** The `rust-cli.md` bright-line bullet superseded at Card 4 is
  preserved verbatim in the ledger GI-019 entry as a block quote, with the two superseded clauses
  named. Compared against `git show HEAD:.claude/rules/mochiko/rust-cli.md` — the preserved
  quote is faithful, word for word. GI-006 reconstructibility holds; the no-strip reasoning (a
  governance surface, not a plugin primitive; AM-2's full rewrite took none) is recorded.
- **Placeholders.** Scan across all six files for `[PLACEHOLDER]`, `[COMMAND]`, `[THRESHOLD]`,
  `GI-XXX`, `[TBD]`, `[TODO]`, and the module templates' bracketed examples: **zero hits**.
- **Pointers introduced or carried this round** — all resolve:
  `hook-enforced-artifact-schema/record.md` · `wave2-amendments.md` ·
  `wave2-reports/intent-review.md` · `wave3-budget-table.md` · `cli-schema-delivery/record.md` ·
  `schema-based-template-guidance/record.md` · `crates/mochiko-cli/src/hook.rs` ·
  `plugins/mochiko/migrations/0005-artifact-homes.yaml` · `.mochiko/memory/knowledge-management.md` ·
  `.mochiko/strips/README.md`.
- **Cited facts, against the tree.** `EXIT_CONFORMANCE: i32 = 4` at
  `crates/mochiko-cli/src/hook.rs:37` ✓ (and `Outcome::Deny => EXIT_CONFORMANCE` at :83).
  `if: false` at `.github/workflows/release.yml:100` ✓, with `environment: crates-io` at :102 ✓.
  `cargo audit --deny warnings` present in `ci.yml` ✓. A secret-scan step present in `ci.yml` ✓.
  The two owed controls are genuinely owed: the publish job is disabled and no tag signing exists.
- **The hook floor, against `plugins/mochiko/hooks/hooks.json`.** Three hooks ship today —
  `SessionStart`, `UserPromptExpansion` on `^mochiko:`, `PreToolUse` on `Skill` — each with
  `"timeout": 5`. The 5-second floor the ledger re-ratifies holds. **No conformance hook ships
  yet**, which is what M1 turns on.
- **Version-triple agreement.** Region stamp v3.1.0 ≡ ledger `**Version:** 3.1.0` ≡ amendment-log
  row 3.1.0 ≡ trace-summary header v3.1.0. Four surfaces, one number.

### Selected module fragments

**`release-gates` (4/4):** environments and cadence stated with the project's real terms —
"Environments: none — nothing deploys; distribution is the Claude Code marketplace · Cadence:
manual, at `plugin.json` bumps" ✓ · every gate carries a concrete verification, no placeholders
(gates 1–6 plus the release train, each naming a command or a recorded artifact) ✓ · rollback
documented, `git revert` of the bump commit + marketplace re-sync, with the time expectation
recorded as an explicit justified absence ("no time-bound SLO — no operated service") ✓ · gates
consistent with attached compliance modules — none attached ✓. Form note in A8.

**`knowledge-management` (8/8):** core artifacts named with read-job, writer moment, and carrier
in the project pin ✓ · all three enforcement surfaces present — the pin, the `paths`-scoped
`operating-docs.md`, the CLAUDE.md pointers ✓ · landing ritual stated as one three-part move
covering closing **and** supersession ✓ · invariants stated mechanically, with the
vacuous-at-zero note ✓ · decision-record schema and glossary term format both present ✓ ·
disambiguation present ✓ · never-overwrite floor and the "no collisions" ruling recorded in
GI-009 ✓ · **re-audit against the repo:** ROADMAP Now 5 / Next 7 / Later 10, all three at or
under cap ✓ · no `[x]` in BACKLOG ✓ · `hook-enforced-artifact-schema` has a brainstorms-index
entry and a `DECISIONS.md` row, statuses agreeing ✓ (one wording contradiction in that row —
A7) · dead-pointer scan over the new pointers clean ✓.

---

## 3 — Flagged proposals and the post-plan disclosure

All five judged **sanctioned as authored**. None unsanctioned. Each is the user's to overturn.

- **FP-1** (GI-012 gate note, ≤ 60 s hook-cost watch). The synthesis rules C9 under GI-002's risk
  surface, and GI-002 carries no Three-Part entry by the accepted AM-2 precedent, so the ruling
  would otherwise live only in the synthesis. The chosen home is exact: the gate note already
  carries the "reported, never gating" posture for the read-back metric, and this is the same
  species. Disclosed twice (trace summary, ledger addendum).
- **FP-2** (GI-012 Testability limbs). Not discretionary in substance — the three-part rule
  requires Testability to cover Enforcement, and the synthesis supplied only the enforcement
  clause. Without it the checklist's "every MUST statement has an enforcement mechanism" would
  have been met with no way to check the strictest MUST in the set.
- **FP-3** (`rust-cli.md` log-is-truth record qualifier). The ambiguity is one this run creates:
  the rewritten header three lines above now names a second driver record with its own D1. The
  bare shorthand would point at the wrong record. Minimal, disclosed, lead-ruled.
- **FP-4** (trace-summary superseded-rounds pointer). Regenerating Shape 4 whole would otherwise
  take the v1.0.0 manifest and its ruled FP-1 dark; GI-006 reconstructibility asks for the
  pointer. Correct.
- **FC-4** (GI-012 trace line extended, disclosed post-plan). Mandatory, not discretionary — the
  checklist requires a Trace stamp reflecting the change, and GI-012 changed.

---

## 4 — Advisory findings (non-blocking)

- **A1 — the region gives no hint that a deny-on-write gate now exists.** CLAUDE.md:72 still
  reads "never gates pipeline progress", and the region index line is a pointer. A reader must
  follow two hops to the ledger to learn that a hook will deny their write. This is GI-017
  working as designed and the clause-(iv) argument is recorded, so it is not a defect — but it is
  the sharpest gap between what the top-level surface says and what a consumer will experience.
- **A2 — the ledger's MINOR-limb widening rides in the bump it licenses.** Amending the
  amendment policy to license this amendment's own semver is circular on its face. Two things
  defuse it: the wording was imported from `validation-constitution`'s standing bump grammar
  rather than invented, and it is disclosed in the synthesis Scope bullet and the ledger addendum.
  Recorded, not hidden.
- **A3 — build-state detail inside a standing Enforcement section.** GI-019 now carries "an
  `Edit` to an existing undeclared file name rides amnesty with a bare allow … carried to the
  wave-4 build as a fix candidate" and "the `PowerShell` matcher arm ships on the doc quote,
  unverifiable on macOS". Both are build-log facts with a named non-governance home. They will go
  stale in the ledger the moment wave 4 lands. Consider pointing at the build log.
- **A4 — "a budget table that admits honest content" is an unmeasured operative term** governing
  whether clause (iv) holds. The C1 measurement is carried (43 of 66 records over the 150-line
  bound; largest 1,536) and the ruling venue named, so it is disclosed rather than vague. But the
  condition has no Testability limb, while its sibling obligation got one under FP-2 on the
  producer's own three-part reasoning. Uneven application of the same standard.
- **A5 — GI-019 and GI-020 index lines carry no `(NON-NEGOTIABLE)` marker** though their homes
  are the `## Non-negotiable constraints` paragraphs. Carried from AM-2 as a recorded advisory
  under the GI-017 pointer precedent; unchanged this round.
- **A6 — FLOOR-SEC's high row asks that secret scanning block merge.** GI-003 asserts a CI
  secret-scan step that runs on push/PR; no branch-protection requirement is recorded anywhere.
  For a solo maintainer the failing job is the practical equivalent. Pre-existing, unchanged at
  AM-3.
- **A7 — `DECISIONS.md` contradicts Card 4.** The 2026-09-13 row says the supersession lands "via
  a `/mochiko:setup` amend run + GI-019 carve + `rust-cli.md` strip". Card 4 ruled **no**
  `.mochiko/strips/` entry. The row predates the ruling; left as-is it records an obligation that
  will never be discharged. Outside the graded set — fix at the landing.
- **A8 — release-gates form.** The module fragment asks for a gate table; the ledger uses a
  numbered list. Content fully satisfies the fragment. Carried from AM-2.
- **A9 — the producer's own "Owed at the next PATCH" line is accurate.** Verified: the ledger's
  semver MAJOR limb reads "principle removal / incompatible redefinition / floor-level change /
  module attach or detach" and lacks the template's `depth-level flip (low→high)` clause.
  Pre-existing, correctly declared out of AM-3's scope.

---

## 5 — Version bump

**Determined independently: MINOR. 3.0.3 → 3.1.0. Agrees** with the synthesis's AM-3 Semver
bullet, the ledger row, the ledger version line, and the region stamp.

Walked against the bump grammar:

- **MAJOR — principle removed?** No. Nothing left the set.
- **MAJOR — principle incompatibly redefined?** No, on the operative text. The CLAUDE.md
  non-negotiable paragraph and its three "never" clauses are byte-unchanged; the diff touches
  only the trace parenthetical. The admission is widened *under* an unchanged principle.
- **MAJOR — floor-level change?** No. Four Essential Floor categories, none re-expressed; depth
  `high` unchanged; no low→high flip.
- **MAJOR — module attach or detach?** No. Compliance none, knowledge-management, release-gates —
  identical to v3.0.3.
- **MINOR — principle significantly expanded?** Yes. GI-019 gains clause (iv), seven Enforcement
  bullets, and a recorded supersession; GI-012 gains a blocking precondition.
- **MINOR — waiver added or removed?** No.

→ MINOR.

**The dissenting MAJOR reading, recorded honestly.** It is stronger than the ledger row makes it
sound. GI-019's prior Testability **Fail** limb read: "a shipped hook that blocks on anything
other than the binary's absence or grammar skew." That is precisely the behavior AM-3 admits. A
testable form whose Fail limb becomes a Pass limb is the signature of a redefinition, and AM-1
and AM-2 both ruled MAJOR on the softer ground that "a non-negotiable's meaning changes."

Two things carry MINOR anyway: the user was put this exact reading twice — at Card 1 and again at
review C6 — and ruled MINOR both times, recorded as a deliberate departure with its reason; and
the checklist's own semver item names only floor-level change and module attach/detach as MAJOR,
neither of which fired. The departure is recorded where a future reader will find it. Accepted.

---

## 6 — Issues requiring fix

**Blocking (1):**

1. **B1** — extend `.claude/rules/mochiko/rust-cli.md`'s `paths` to cover
   `plugins/mochiko/.claude-plugin/plugin.json`, `.claude-plugin/marketplace.json`, and
   `CHANGELOG.md`; or home the release-gate reminder in a rules file scoped to them. One edit.
   Alternatively the user may rule it a recorded PATCH obligation, as the producer did for A9 —
   that is a ruling, not a producer or validator call.

**Minor (2):**

2. **M1 — GI-019's re-keyed Testability carries no dormancy tier, and its conformance limb is
   unassertable today.** The Pass limb reads "every shipped hook blocks on exactly two grounds —
   the binary's absence or a log outside its grammar range, **and a conformance deny from
   `check`**". Verified against `plugins/mochiko/hooks/hooks.json`: the three shipped hooks are
   `SessionStart`, `UserPromptExpansion`, and `PreToolUse` on `Skill`, all dependency-halt; no
   `check` hook ships until wave 4. The row is vacuously true now and becomes assertable later.
   The house idiom exists and was reviewer-driven: GI-020's Testability carries two explicit
   tiers ("Assertable at ratification" / "Activated at v3.0.1 by the wave-3 pilot") after AM-2's
   review I1 raised exactly this. **Fix:** mark the conformance limb dormant until the wave-4
   ship, in GI-020's two-tier form.

3. **M2 — the wave-4 precondition is a one-shot clause in two standing homes with no expiry
   route.** Once the hooks ship, "the wave-4 bump … MUST NOT land before the first publish" is
   dead text in the region's release-gates line and in two ledger paragraphs, forever. The set
   already has the idiom for this: GI-020's transition clause shipped with "its expiry is a
   pre-authorized PATCH amendment", and that is exactly how it was struck at v3.0.3. **Fix:** add
   the same sentence — the clause is struck by pre-authorized PATCH when the first publish lands
   with all four controls.

---

## 7 — Method

Inputs read from file in full: `CLAUDE.md` · all four `.claude/rules/mochiko/` files ·
`governance-ledger.md` (635 lines, whole) · `governance-intent.md` (347 lines, whole) ·
`governance-trace-summary.md` · the project KM pin · the `release-gates` and
`knowledge-management` module templates and their embedded validator fragments · the
universal-floor card's `high` rows. The producer's report was not opened.

The delta was established from `git diff HEAD` over the three surfaces, not from the producer's
account of it. Cited facts were checked against the tree: the crate's exit code, the release
workflow's disabled publish job and `crates-io` environment, the CI audit and secret-scan steps,
the shipped hook set and its timeouts, the plugin version, and the existence of every pointer
introduced this round.

Per the brief's model-tiering rule, one native `Explore` subagent at `model: haiku` was spawned
for a locate-and-enumerate gap (hook declaration sites, the KM pin's headings, the presence of
`constitution.md` / `FEATURES.md`, and two workflow line references). Every returned fact that
bears on a finding was re-read from the file by this seat. Every judgment read and the verdict
are this seat's own.

Floor read-back stated before the first procedural step: 14 floor rules, ids as the preamble
lists them.

**Transport:** the set was declared frozen. This seat wrote only this file and made no git
mutation. `wave2-reports/` is not a declared sub-directory of the brainstorm home — known,
disclosed in the brief, and scheduled for the violator pass; written here as the producer did.

---

## Delta-confirm — 2026-09-14, round 2

**VALIDATION RESULT: PASS**

**Bounded to:** the three fixes (B1, M1, M2) plus everything they could have disturbed. Re-read
from the files, not from the producer's account of the round.

- **Checklist items:** 60/61 passed · **0 FAILED** · 1 n-a by recorded decline. The failed item —
  *Rules-File Scope & Delivery*, "each rules file's `paths` globs cover every path whose code can
  violate the concern" — now passes. Every other item re-confirmed unchanged or re-checked where
  the fixes touched it.
- **Surface integrity:** scope coverage **now closes**. `rust-cli.md` carries eight globs: the
  five ratified plus `plugins/mochiko/.claude-plugin/plugin.json`, `.claude-plugin/marketplace.json`,
  and `CHANGELOG.md`. All three targets verified present on disk. GI-012's gates and GI-010's home
  now have a touch-time carrier at the files they are violated on. No glob collides with
  `primitive-edits.md` (which scopes `plugins/mochiko/{commands,skills,agents,templates,migrations,hooks}/**`,
  not `.claude-plugin/`) or with `operating-docs.md` (which does not scope `CHANGELOG.md`).
- **Trace closure:** still closes both ways. The trace summary lists **FP-5** in the flagged-proposals
  block with its reasoning, its AM-2 FP-1/FP-2 precedent, and its alternative; the reverse table
  gains the `rust-cli.md` `paths` row keyed to GI-012 and GI-010; the forward table's GI-010 and
  GI-012 companion cells name the same change. The GI-019 reverse row now reads "Testability
  re-keyed into two tiers, the conformance limb dormant until the wave-4 hook ship", and the GI-012
  row names the M2 expiry at both homes. No row was orphaned and no new line lacks an element.
- **Contract untouched:** `governance-intent.md`'s AM-3 `**Accepted:**` line is still *(pending)*
  and the fix round left no trace in the ratified synthesis, which is correct — the producer fixed
  surfaces, not the contract.
- **Region untouched:** `git diff HEAD -- CLAUDE.md` is still 5 insertions / 5 deletions, identical
  to round 1. The M2 fix correctly left the region gates line alone; the ledger's pre-authorization
  covers the region-side strike, exactly as GI-020's transition clause was handled at v3.0.1/v3.0.3.
- **Version bump:** unchanged. MINOR 3.0.3 → 3.1.0. None of the three fixes moves it — a glob
  addition, a Testability re-shaping, and an expiry pre-authorization are all clarification-class
  under the grammar and ride the MINOR.
- **Anti-patterns found:** none.

### The three fixes, each verified against the file

**B1 — closed.** `paths` re-read from `rust-cli.md`'s frontmatter; eight globs, the three new ones
last. Recorded as FP-5 in the ledger's AM-3 addendum with the reasoning stated ("every GI-012 gate
fires at the `plugin.json` bump and AM-3 puts the set's strictest MUST NOT there, and `CHANGELOG.md`
is GI-010's home") and in the trace summary. The proposal is correctly marked the user's to
overturn at acceptance.

**M1 — closed, and better than the minimum.** GI-019's Testability is now two tiers in the GI-020
idiom, with the reason stated inline: "the conformance limb cannot be asserted until wave 4 ships
the hooks (`plugins/mochiko/hooks/hooks.json` carries only the three dependency-halt hooks today)."
The assertable tier keeps the pre-AM-3 hook clause and adds the one new fact that *is* checkable
now — `check` exists and returns exit 4. The dormant tier carries the two-grounds form and names
its activation: the wave-4 bump, as a pre-authorized PATCH, no fresh amend run. The two tiers do
not contradict: they are explicitly time-scoped, the same structure GI-020 uses.

**M2 — closed at both homes, identically worded.** The amendment-policy first-publish paragraph
and the GI-012 wave-4 paragraph each end: "Once the wave-4 bump has landed under this precondition,
striking the clause from the region gates line and these two paragraphs is a pre-authorized PATCH
amendment, recorded in the log row." The sentence names all three sites it authorizes, including
the region line, which is why leaving the region untouched is right.

### Residuals (non-blocking, 2)

- **R1 — the amendment-log row under-describes the fix round.** Row 3.1.0 still reads "GI-019
  Testability re-keyed" and names neither the two-tier shape nor its wave-4 PATCH activation, nor
  M2's expiry pre-authorization, nor FP-5. The AM-3 addendum names FP-5 only. Both
  pre-authorizations are stated in their own homes, which is where the GI-020 precedent put them,
  so nothing is unrecorded — but a reader of the amendment log alone will not find the two new
  pre-authorized PATCHes, and the log row is where the v2.0.1, v3.0.1, and v3.0.3 rows recorded
  exactly this class of thing. One clause in the addendum or the row's GI delta closes it.
- **R2 — the region's new-file read line and the rules globs have drifted.** CLAUDE.md:139 names
  `.mochiko/specs/`, `plugins/mochiko/`, `crates/mochiko-cli/`, `plugins/mochiko/migrations/`,
  `plugins/mochiko/hooks/`, `evals/contract/`, `.github/workflows/`, and "the operating docs". Two
  of FP-5's three new globs are outside that list: `.claude-plugin/marketplace.json` (the repo-root
  directory is not named) and `CHANGELOG.md` (not named, and "the operating docs" means the five
  files `operating-docs.md` scopes, which do not include it). Practical bite is near zero — the
  line governs *creating* a file under a scoped path and all three targets already exist — and the
  checklist item ("the region carries the standing new-file read line") still passes. Worth one
  edit at the next touch for honesty between the two surfaces.

Round-1 advisories A1–A9 stand unchanged; A9 (the ledger's semver MAJOR limb lacking the
depth-flip clause) remains correctly declared owed at the next PATCH. Nothing in the fix round
disturbed them.

**Not graded:** the producer's own report, including its "six ratified globs" self-correction —
that file is not a member of the graded set. Inputs were read from the surfaces throughout.

**Transport:** set declared frozen; this seat wrote only this file, made no edit to the set, and
performed no git mutation.

---

## Delta-confirm round 3 — 2026-09-14

**VALIDATION RESULT: PASS**

**Bounded to:** the R1 ledger log-row clause, the R2 region read-line edit, and the new reverse-trace
row, plus what those three could disturb. Re-read from the files.

- **Checklist items:** 60/61 passed · 0 FAILED · 1 n-a by recorded decline. No item's disposition
  moved; the two edits sit inside items that already passed and make both more honest.
- **Residuals: none.** R1 and R2 are both closed. One wording nit and one bookkeeping note below,
  neither worth an edit on its own.

### The three edits, verified

**R1 — closed.** The 3.1.0 amendment-log row now carries a terminal `**fix round (validator
B1/M1/M2):**` clause naming all three: the two-tier GI-019 Testability with its conformance limb
"dormant until the wave-4 hook ship" and activated at that `plugin.json` bump as a pre-authorized
PATCH, citing the v2.0.1 / v3.0.1 / v3.0.3 idiom by name; M2's post-landing strike as a
pre-authorized PATCH recorded at both ledger homes; and the three FP-5 globs. A reader of the
amendment log alone now finds both new pre-authorized PATCHes, which was the whole of R1.

**R2 — closed.** CLAUDE.md:139 now reads "… `.github/workflows/`, `.claude-plugin/` (both
manifests), `CHANGELOG.md`, or the operating docs …". Both FP-5 targets that were outside the list
are now named. The read line and `rust-cli.md`'s eight globs no longer disagree.

**Trace row — correct and keyed right.** `governance-trace-summary.md:45` adds "region | path-scoped-rules
read line names `.claude-plugin/` (both manifests) and `CHANGELOG.md` (residual R2) | GI-012,
GI-010, GI-019". All three keys hold: GI-010 because `CHANGELOG.md` is its home, GI-012 because the
bump is where its gates fire, GI-019 because `rust-cli.md` is that principle's rules carrier and the
read line is what routes a reader to it. The producer raised the gap itself, which is the right
posture. Reverse closure holds; the forward table needs nothing, since element → home is unchanged.

### Nothing disturbed

- Round-2 fixes intact: the two-tier Testability block stands at ledger:459–473; the M2 expiry
  sentence appears exactly twice, one per home; `rust-cli.md` still carries eight globs.
- Region: exactly **six** changed lines against HEAD and no seventh — kernel-class paragraph ·
  ratified stamp · GI-019 index line · release-gates line · amend-triggers line · the read line.
  Everything else between the markers is byte-unchanged.
- Contract untouched: `governance-intent.md`'s AM-3 `**Accepted:**` is still *(pending)*.
- Version bump unchanged: MINOR 3.0.3 → 3.1.0. A log-row clause and a read-line enumeration are
  clarification-class and ride it.
- Anti-patterns: none.

### Two notes, no edit required

- **Wording nit.** "`.claude-plugin/` (both manifests)" reads as one directory holding two files;
  they are in fact two directories — `plugins/mochiko/.claude-plugin/plugin.json` and the repo-root
  `.claude-plugin/marketplace.json`. The parenthetical does the disambiguating work, and a reader
  creating either file will match, so the line does its job. Worth tightening only if that line is
  touched again.
- **Bookkeeping.** The round was described as "six hunks"; `git diff` reports four `@@` hunks
  carrying six changed lines, because the Quality-gates and Governance-operations edits are
  contiguous. Six changed lines is the correct count and is what I verified. Noted so a later
  reader grepping for six hunk markers is not confused by finding four.

Round-1 advisories A1–A9 stand unchanged and undisturbed; A9 remains correctly declared owed at the
next PATCH.

**Transport:** set frozen; this seat wrote only this file, edited nothing in the set, and made no
git mutation.
