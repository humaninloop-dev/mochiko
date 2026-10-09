---
report: review
round: 2
seat: w3-gate
tier: opus
wave: human-readable-ids wave 3 (back-fill) — eight plugin units and the done check
units: 8 of 8
verdict: 8 PASS · 0 blocking — unit 2 (the mochiko router) FAIL at round 1, PASS at round 2 after its one fix
done-check: mochiko-cli ids --check · 0 bare · 0 drift — exit 0, amended Exclude list, release binary, this report included
---

## Failure narrative

- Round 1: unit 2 (`skills/mochiko/SKILL.md`) failed on one blocking item. The edit's two joins were right; what was left out was the problem.
- Line 71 read "the transport choice stays neutral (realignment D5)". "realignment" is a one-word shorthand for `command-architecture-realignment`, and `command-architecture-realignment` D5-transport-neutral-harness is the graded entry in `slug-map-a.tsv`.
- The strip's "Kept deliberately" line said the mention stays bare under `human-readable-ids` D11-cross-session-qualifier as narrowed and `human-readable-ids` D18-build-done-check as changed at build. Neither ruling says that:
  - `human-readable-ids` D11-cross-session-qualifier as narrowed is about a qualifier at the start of a line owning that line's other bare `D<n>`. It says "A mention's own qualifier is untouched".
  - `human-readable-ids` D18-build-done-check as changed at build says shorthand session cites "leave the check's view and are found by wave 3's slug map instead".
  - Wave plan item 5 says a shorthand qualifier is rewritten through a file-scoped `--alias`.
  - The wave's own graded hand rows qualified the same phrase as a "single-word shorthand": `hand-c.tsv` rows 99, 101 and 102, "realignment D5 row" in `teammate-message-races/record.md`.
- So the router mention was a slug-map miss, but the strip recorded it as kept by ruling. An auditor reading that strip would take an oversight for a ruling. That is the reverse of what the field is for (`.mochiko/strips/README.md`).
- What I tried: I scanned all 16 changed files for word-qualified `D<n>` cites. `realignment` is the only one-word session shorthand among them; every other residual is a plain word ("record", "per", "the") or carries no qualifier.
- Resolved at round 2: the one fix joined :71 and corrected the strip entry, and the re-audit passes (see Round 2).

## Notes of note

- Contract: I rendered it first-hand, and it is byte-identical to `S/post-apply/vpe-render-w3.txt` (sha256 `28ac240d1bc4f2e2` for both). Brief hash `ffeb1479e0a1899f`, apply log `1d5903ef1969adbb` and fix log `aa8bda68586e8aaa` (round 1) / `aed2c2bb87a9f5ac` (round 2) all match.
- The brief asks for a char-budget measurement only where the `SKILL.md` body changed. `validation-primitive-edit` changed only its `description:`, so I measured it as well: 730 → 754 against a 730 budget with no headroom. That is a new **+24** overage. I rule it HOLDS in unit 6.
- `patterns-model-tiering` payload is 18,347 → 18,369 (+22 body), which moves the standing overage from +7,495 to +7,517. It HOLDS (unit 3).
- `authoring-constitution` payload is 29,162 → 29,207, still inside 29,614. The router body (unbudgeted) is 47,303 → 47,345 at round 1, and 47,394 after the round-2 fix.
- The ledger rows in `.mochiko/memory/primitive-cost-budgets.md` still carry the pre-wave-3 figures. The landing's budget sweep owes them.
- The migration log is untouched (`git diff --stat 0b8982a -- plugins/mochiko/migrations .mochiko/schema-views plugins/mochiko/.claude-plugin` is empty), so no render moved.
- Routing: I dispatched no `Explore` subagent. Every read was either a known path, a completeness-driving enumeration, or a decision-driving absence, which `mochiko:patterns-model-tiering` keeps on the seat.
- `mochiko-cli rules mochiko` exits with "no command or skill named 'mochiko' in the log". The router has no render, so I graded it as an unconverted skill body (the wave-2 P15/P16 precedent).
- Round 2: at the lead's order I quote-masked or joined this report's own ID tokens. The round-2 rename's first preview had hit four of its lines.

## Floor read-back and seat

The preamble's `class: floor` pin prints **11** rules: `validation-primitive-edit.author-grader` · `plain-seat-explicit-tier` · `rendered-contract-only` · `from-file-floor` · `pre-pass-first-hand` · `binary-verdict` · `default-fail` · `tamper-proof-clause` · `gate-loop-bound` · `evidence-floor` · `second-fail-user` (each prefixed `validation-primitive-edit.`).

Seat `w3-gate`: plain, persona-less, `model: opus` (runtime `claude-opus-5-5`), and author of nothing in any unit. One seat took all eight units, and every file fit this context, so no split was needed (`one-seat-per-wave`).

Render command: `for s in preamble validation-primitive-edit.sec.{independence,scope,inputs,verdict,output,reserved}; do mochiko-cli rules validation-primitive-edit --section $s --plugin-root plugins/mochiko; done`, using the release binary `S/apply/bin-release/mochiko-cli` (sha256 `b82706463b0344b2`, verified), binary 0.3.0, grammar 2. `diff` against the lead's copy printed nothing.

Gate-loop bound, from the `setup.gate-loop-bound` render: "A gate verdict of FAIL allows one fix and one re-audit — by the same grader seat resumed, reading only what the fix touched and what it could have broken; a second FAIL halts the landing and goes to the user with both fix lists, fix again or drop".

## Pre-pass — validate, diff, log state

`mochiko-cli migrate validate --report --plugin-root plugins/mochiko`, run first-hand (exit 0):

```
rules scanned: 1175 · in-kind pairs scored: 192052 · clusters: 0 (none)
allowlist-suppressed edges: 185
mochiko-cli migrate validate · 0 rejecting · 113 advisory
```

`mochiko-cli migrate status`: `sequences 1..50 (48 migrations)` · `state sha256:c7e23ab026d07fb3… · 87 documents · 1175 rules`. That is the state wave 2's round 3 recorded, so the log has not moved.

`git diff --stat 0b8982a -- plugins/mochiko/` reported `16 files changed, 42 insertions(+), 42 deletions(-)` at round 1. Every file is in a unit below; there are no untracked files under `plugins/mochiko/` and no file outside the list.

Traceability: from `apply-log.tsv`, every plugin hunk comes from one of 21 `ids rename … --write` calls, all on the release binary and all carrying "`diff check: … · ID tokens only`". Each call is a graded map row, with `--alias PO-D` or `ER-D` from `alias-c.tsv` rows 2 and 5 where a glued form was normalized. No hand row and no literal touched a plugin file, and the round-1 fix log has 0 rows under `plugins/mochiko/`. All 16 files run through `mochiko-cli check --path <file> --content -` return `allow`. `sh -n` passes on both hook scripts.

## Pre-pass — strips check (9 entries, 9 files)

`mochiko-cli check --path .mochiko/strips/<f>.md --content - --plugin-root plugins/mochiko < <f>`, run on `authoring-constitution`, `mochiko`, `patterns-model-tiering`, `review-brainstorm`, `validation-constitution`, `validation-primitive-edit`, `release-gates-module`, `artifact-gate` and `dependency-halt`, gives `{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"allow"}}` × 9, exit 0 each. `mochiko-cli home .mochiko/strips` declares `<slug>.md · no template · no bound declared`.

My own script read back every entry's `Content (superseded)` fence against `git show 0b8982a:<path>`:

- authoring-constitution: 26/26 lines verbatim
- mochiko: 2/2 (3/3 after the round-2 fix)
- patterns-model-tiering: 1/1
- review-brainstorm: 7/7
- validation-constitution: 2/2
- validation-primitive-edit: 1/1
- release-gates-module: 1/1
- artifact-gate: 1/1
- dependency-halt: 1/1

In each, every old line in the diff is claimed and nothing extra is. Each wave-3 entry sits newest-first, above any wave-2 `[v0.118.0]` entry in the same file.

`ids --check` over the nine strip files reports only "`.mochiko/strips/validation-constitution.md:68:116 · bare · GI-007`". That is the wave-2 entry's nested quote, which the amended Exclude list covers. No wave-3 fence is misread.

## Pre-pass — char-budget measurement

Method: the canonical snippet from `.mochiko/memory/primitive-cost-budgets.md` "How to measure" for the body and `description:`. On top of that, the characters of each block the `SKILL.md` `!` lines deliver (`mochiko-cli rules <skill> --section <id>`, stdout+stderr), which is wave 2's method. Script: `S/gate-w3/measure.py`; output: `S/gate-w3/budget.tsv`. Pre is `0b8982a`; now is the round-1 tree. Both use one log.

| skill | budget | pre payload | now payload | body Δ | desc | standing |
|---|---|---|---|---|---|---|
| authoring-constitution | 29,614 | 29,162 (7,219 + 21,943) | 29,207 (7,264 + 21,943) | +45 | 461 = | 407 under |
| patterns-model-tiering | 10,852 | 18,347 (3,113 + 15,234) | 18,369 (3,135 + 15,234) | +22 | 1,209 = | +7,517 (was +7,495) |
| validation-primitive-edit | 14,968 | 14,947 | 14,947 | 0 | **730 → 754** (budget 730, no headroom) | desc **+24, new** |
| mochiko (router) | unbudgeted | 47,303 | 47,345 (47,394 at round 2) | +42 (+91 at round 2) | 206 = | — |

## Unit 1 — skill pair `authoring-constitution`

VALIDATE: skill pair `authoring-constitution`: `SKILL.md` with its seven-block render, plus the six edited references (`COMPLIANCE-MODULES.md`, `DOMAIN-DEPENDENCIES.md`, `INTERROGATION-AGENDA.md`, `catalog/README.md`, `catalog/backend-service.md`, `catalog/universal-floor.md`).
Checklist run: `judgment-items-pair` for `SKILL.md`; `judgment-items-prose` for the references; the strip's "Kept deliberately" reasons, as the brief asked.
Evidence read: `git diff --word-diff 0b8982a -- plugins/mochiko/skills/authoring-constitution` (26 lines) · every residual bare line the strip lists, read in place · `slug-map-c.tsv` rows `production-only-focus` 1–7 and 4.2, and `setup-product-agnostic` 1–4 · `alias-c.tsv:2` (`PO-D`) · `apply-log.tsv` writes c:48–56 and c:107–110 · `.mochiko/strips/authoring-constitution.md` `[v0.118.0]` Wave 3 · record D5, D11, D15, D18, D22.
Pre-pass: `0 rejecting · 113 advisory`; payload 29,207 ≤ 29,614; pin `class: floor · 13 rules`, unchanged.
- PASS — scaffold headings and order: the one `SKILL.md` change is a table cell (:89). No heading changed, and no `!` line.
- PASS — preserved responsibilities: each of the 26 lines changes only its ID token. Every joined slug equals the graded map's: `production-only-focus` D1-customer-product-target, D2-tier-axis-retired, D3-library-owned-standard, D4-waivers-reach-everything, D4.2-legal-mandate-exception, D5-production-depth-agenda and D7-immature-team-onramp; `setup-product-agnostic` D1-profile-leaves-setup, D2-rule-not-instance, D3-seven-dimension-agenda and D4-closed-event-set. Each owner fits its line's meaning.
- PASS — the dotted sub-decision takes its own slug (`human-readable-ids` D22-sub-id-forms), and two-item lists are expanded (`human-readable-ids` D5-compound-reference-forms).
- PASS — floor survival, independence and reserved section: the render is byte-identical (the log did not move), and the pin is unchanged.
- PASS — done-condition branch: n/a; skills carry none.
- PASS — coherence of the references: no anchor link targets a changed line. The cite forms read cleanly.
- PASS — "Kept deliberately" reasons. Every residual bare token I scanned appears in the list: COMPLIANCE-MODULES :3, :16, :35–36; DOMAIN-DEPENDENCIES :64; INTERROGATION-AGENDA :42, :132, :134, :141, :152, :154, :178; catalog/README :41–42, :45, :51–52, :66; universal-floor :9–10, :13–14, :37, :51, :65, :79. Each one is a bare `D<n>` with no session word and no record link. That is `human-readable-ids` D18-build-done-check as changed at build ("a per-artifact or session mention with no resolvable owner, is never reported and never rewritten"), together with `human-readable-ids` D11-cross-session-qualifier as narrowed.
- PASS — the two other survivors: `catalog/README.md`:41's "`D1–D8`" is a range (`human-readable-ids` D5-compound-reference-forms), and `COMPLIANCE-MODULES.md`:3 is a `>` blockquote (`human-readable-ids` D15-protected-line-rewrites's mask).
- Advisory (non-gating): the reason's gloss "an unindexed mention" misnames the ruled class. These mentions are indexed IDs with no resolvable owner. Fix: write "a session mention with no resolvable owner" in the entry, at the next touch.

VERDICT: PASS

Issues requiring fix: none.

## Unit 2 — `mochiko` router (unconverted skill body)

Round 1. The round-2 re-audit is in "Round 2" below.

VALIDATE: `plugins/mochiko/skills/mochiko/SKILL.md`. I graded it as a prose primitive (an unconverted skill body): it has no `!` lines and no log document (`mochiko-cli rules mochiko` gives "no command or skill named 'mochiko' in the log"). Its body is unbudgeted.
Checklist run: `judgment-items-prose` (coherence, preserved responsibilities), plus the strip's "Kept deliberately" reason.
Evidence read: `git diff 0b8982a -- plugins/mochiko/skills/mochiko/SKILL.md` (:39, :129) · `SKILL.md:71` in place · `slug-map-a.tsv` rows `author-grader-consolidation` 7, `command-architecture-realignment` 5 and GI 004 · `alias-c.tsv:98-99` (`transport-neutral D` aliased to `command-architecture-realignment` D5-transport-neutral-harness) · `hand-c.tsv:99`, `:101`, `:102` ("realignment D5 row", kind `qualify`, "single-word shorthand") · `apply-log.tsv` writes a:36 and a:225 · `.mochiko/strips/mochiko.md` `[v0.118.0]` · wave plan item 5 · diff review findings `S/diff-review/findings.md` (no router line).
Evidence read, rulings: `human-readable-ids` D11-cross-session-qualifier (narrowed at build) and `human-readable-ids` D18-build-done-check (changed at build).
Pre-pass: `0 rejecting · 113 advisory`; body 47,303 → 47,345 (unbudgeted); description 206, unchanged.
- PASS — coherence of the edit: :39 now reads "`` (`author-grader-consolidation` D7-fresh-gate-grader) ``" and :129 reads "`(GI-004-primitive-audit-ratchet)`". Both slugs are the graded map's, and both owners are right: the plain fresh gate grader, and mochiko's own primitive-audit gate.
- PASS — preserved responsibilities: both router rows keep their routing text word for word apart from the token.
- FAIL — the survivor at :71 and its recorded reason. "(realignment D5)" is a one-word session shorthand for `command-architecture-realignment` D5-transport-neutral-harness, and the wave's hand rows qualified the identical phrase elsewhere.
- FAIL, continued — the strip said the mention stays bare under `human-readable-ids` D11-cross-session-qualifier as narrowed and `human-readable-ids` D18-build-done-check as changed at build. The first does not cover a mention's own qualifier. The second hands shorthand cites to the slug map, so this was a map miss recorded as a ruled keep (see Failure narrative).

VERDICT: FAIL

Issues requiring fix:
1. Item: survivor at `SKILL.md`:71 and its "Kept deliberately" reason. Missing: the join that `human-readable-ids` D18-build-done-check as changed at build and wave plan item 5 rule for a shorthand session cite. Fix:
   - First, qualify the mention by hand to "`` (`command-architecture-realignment` D5) ``", dry-running it through `check`, as `hand-c.tsv` row 99 did.
   - Next, preview and then `--write` "`ids rename .mochiko/brainstorms/command-architecture-realignment/record.md D5 transport-neutral-harness`", logging its diff-check line. The line should then read "`` (`command-architecture-realignment` D5-transport-neutral-harness) ``".
   - Then rewrite the `[v0.118.0]` entry in `.mochiko/strips/mochiko.md`: put :71 into Disposition, put its `0b8982a` text verbatim into Content (superseded), and set Kept deliberately to "none".
   - Last, re-run the done check.

## Unit 3 — skill pair `patterns-model-tiering`

VALIDATE: skill pair `patterns-model-tiering`: `SKILL.md` (Overview :22) with its seven-block render.
Checklist run: `judgment-items-pair`.
Evidence read: `git diff 0b8982a -- plugins/mochiko/skills/patterns-model-tiering/SKILL.md` · `SKILL.md:3` (the description's range) · `slug-map-b.tsv` row `model-tiered-seats` 1 · `apply-log.tsv` write b:79 · `.mochiko/strips/patterns-model-tiering.md` `[v0.118.0]` · budget row `primitive-cost-budgets.md:88` · record `human-readable-ids` D2-joined-form-placement and D5-compound-reference-forms.
Pre-pass: `0 rejecting · 113 advisory`; payload 18,369 (body 3,135 + render 15,234); description 1,209, unchanged and hard-cap-only.
- PASS — scaffold headings and order: one Overview line changed. No heading changed, and no `!` line.
- PASS — preserved responsibilities: "(model-tiered-seats D1-usage-accounting-unit)" keeps the economics clause. The slug is the graded map's, and that decision is the unit of account the line cites.
- PASS — floor survival, independence and reserved section: the render is byte-identical; pin `class: floor · 8 rules`, unchanged.
- PASS — done-condition branch: n/a.
- PASS — "Kept deliberately": `SKILL.md`:3's "(orchestrator-model-selection D1–D5)" is a range and stays bare (`human-readable-ids` D5-compound-reference-forms).
- PASS — argued overage: +22 body moves the standing overage to +7,517. This is a ruled form change, not restored prose: `human-readable-ids` D2-joined-form-placement puts the joined form at every single-ID mention, and `human-readable-ids` D14-live-layer-backfill applies it to the live layer. The first ruling's accepted risk names the length cost. It HOLDS. The ledger row is the lead's to update.

VERDICT: PASS

Issues requiring fix: none.

## Unit 4 — prose primitive `review-brainstorm/references/EXTERNAL-CLAIMS.md`

VALIDATE: prose primitive `plugins/mochiko/skills/review-brainstorm/references/EXTERNAL-CLAIMS.md`: six headings (:10, :30, :36, :43, :70, :76) and :98.
Checklist run: `judgment-items-prose`.
Evidence read: `git diff 0b8982a` of the file · its `##` heading list · `CROSS-EXAM.md:45` (its one anchor, `#pair-source-conflict-resolution`, an explicit id on an unchanged heading) · `slug-map-a.tsv` rows `external-research-in-review` 1–6 · `alias-c.tsv:5` (`ER-D`) · `apply-log.tsv` writes a:193–197 · `.mochiko/strips/review-brainstorm.md` `[v0.118.0]` Wave 3 · record `human-readable-ids` D5-compound-reference-forms and D15-protected-line-rewrites.
Pre-pass: `0 rejecting · 113 advisory`; `references/` files are exempt from the budget.
- PASS — coherence: each heading now names its decision: `external-research-in-review` D2-load-bearing-trigger, D3-verify-at-review, D4-inline-reviewer-checks (three headings), D5-no-review-paths and D6-shared-reference-carrier. Each slug is the graded map's and matches its section.
- PASS — preserved responsibilities: section text is unchanged and no anchor or consumer breaks. No plugin consumer cites a changed heading's text, and the eval-variant copies are history (S4).
- PASS — "Kept deliberately": :8's "(ER-D1–D6, 2026-08-04)" is a range inside a `>` blockquote (`human-readable-ids` D5-compound-reference-forms and D15-protected-line-rewrites).

VERDICT: PASS

Issues requiring fix: none.

## Unit 5 — prose primitive `validation-constitution` references

VALIDATE: prose primitive `plugins/mochiko/skills/validation-constitution/references/ANTI-PATTERNS.md` (:16) and `references/QUALITY-CHECKLIST.md` (:25).
Checklist run: `judgment-items-prose`.
Evidence read: `git diff 0b8982a` of both files · `QUALITY-CHECKLIST.md:55-65` · `.mochiko/memory/governance-intent.md:63` (GI-017-pointer-only-region) · `migrations/0020-setup-agnostic-rule-not-instance.yaml:31` ("a GI-017 violation — governance surfaces point at constraint homes": the plugin cites mochiko's own GI-017-pointer-only-region) · `apply-log.tsv` write a:238 · `.mochiko/strips/validation-constitution.md` `[v0.118.0]` (both entries) · `ids --check` on the strip file.
Pre-pass: `0 rejecting · 113 advisory`; exempt references; `SKILL.md` unchanged.
- PASS — coherence: GI-017-pointer-only-region is mochiko's own principle, the ground this anti-pattern and this check have always cited. It is not a foreign or example GI, so the slug is the right one.
- PASS — preserved responsibilities: the table row and the checklist line are otherwise unchanged.
- PASS — "Kept deliberately": :61's "permanent (D4.1 pending)" sits inside `"…"`, a literal the waiver record carries (`human-readable-ids` D15-protected-line-rewrites's mask).
- PASS — strip file outside the done check: the amended Exclude list drops this strip file, so I checked it directly. Its one report is wave 2's `:68` nested quote; the wave-3 entry is clean.

VERDICT: PASS

Issues requiring fix: none.

## Unit 6 — skill pair `validation-primitive-edit`

VALIDATE: skill pair `validation-primitive-edit`: `SKILL.md` (the `description:` only) with its seven-block render.
Checklist run: `judgment-items-pair`, plus the description budget.
Evidence read: `git diff 0b8982a -- plugins/mochiko/skills/validation-primitive-edit/SKILL.md` · the render (this seat's own contract, identical) · `primitive-cost-budgets.md:304` (`| validation-primitive-edit | 730 | 730 (no headroom) |`) and :314-322 · `apply-log.tsv` write a:225 · `.mochiko/strips/validation-primitive-edit.md` (new file) · record `human-readable-ids` D2-joined-form-placement and D21-log-rule-citations.
Pre-pass: `0 rejecting · 113 advisory`; payload 14,947 unchanged; description **754** against budget 730.
- PASS — scaffold headings and order: the body is byte-identical (3,449). Only the frontmatter `description:` changed.
- PASS — preserved responsibilities: every routing clause survives. "`(GI-004)`" became "`(GI-004-primitive-audit-ratchet)`", which is mochiko's own gate principle and the right owner.
- PASS — the render's `gate-job` rule still cites "`(GI-004)`" bare. `human-readable-ids` D21-log-rule-citations keeps log text bare, so the mix is ruled.
- PASS — floor survival, independence and reserved section: the render is byte-identical; pin `class: floor · 11 rules`, unchanged.
- PASS — done-condition branch: n/a.
- PASS — argued overage: +24, from the joined slug alone. `human-readable-ids` D2-joined-form-placement rules the joined form at every single-ID mention, and its accepted risk names the description cap. 754 sits 782 under the 1,536 delivery cap. It is not restored prose, so it HOLDS. The ledger row and the :314 note owe the [v0.118.0] figure at the landing sweep.
- PASS — strip: the new file carries the small-file header, and the one entry has Disposition, Tier failed, verbatim Content and "Kept deliberately: none". That last field is true: no bare ID remains in the file.

VERDICT: PASS

Issues requiring fix: none.

## Unit 7 — prose primitive `templates/constitution-modules/release-gates.md`

VALIDATE: prose primitive `plugins/mochiko/templates/constitution-modules/release-gates.md` (:5).
Checklist run: `judgment-items-prose`.
Evidence read: `git diff 0b8982a` of the file · `slug-map-c.tsv` row `production-only-focus` 1 · `apply-log.tsv` write c:48 · `.mochiko/strips/release-gates-module.md` `[v0.118.0]`.
Pre-pass: `0 rejecting · 113 advisory`; no budgeted class.
- PASS — coherence: "(`production-only-focus` D1-customer-product-target)" is the target-boundary decision that makes a release process definitional. That matches the slug and its owner.
- PASS — preserved responsibilities: the "Attach when" guidance is otherwise unchanged. "Kept deliberately: none" is true.

VERDICT: PASS

Issues requiring fix: none.

## Unit 8 — prose primitive hook scripts

VALIDATE: prose primitive `plugins/mochiko/hooks/scripts/artifact-gate.sh` (:2) and `dependency-halt.sh` (:5), comment lines only.
Checklist run: `judgment-items-prose`. The code-versus-comment check is the crate review's.
Evidence read: `git diff -U0 0b8982a -- plugins/mochiko/hooks` (two `#` lines) · `sh -n` on both (pass) · `governance-intent.md:67` (GI-019-kernel-tooling-admission) · `hook-enforced-artifact-schema/record.md` card headings (D1–D11, no "`D7b`" or "`D7c`" card) · `slug-map-a.tsv:96`, `slug-map-b.tsv:23` · `apply-log.tsv` write a:240 · `.mochiko/strips/artifact-gate.md` and `dependency-halt.md` (new files) · wave plan S8.
Pre-pass: `0 rejecting · 113 advisory`; `check` gives `allow` on both.
- PASS — coherence: "ledger GI-019-kernel-tooling-admission clause iv" and "(GI-019-kernel-tooling-admission)" cite mochiko's own kernel-admission principle, the ruling these hooks were admitted under.
- PASS — preserved responsibilities: only the token changed. Both edits are comments, and the scripts still parse.
- PASS — "Kept deliberately": "record D1, D3, D7c" (:2), "(record D9)" (:7, :44), "(record D7c)" (:36) and "D7c floor" (:47) in `artifact-gate.sh`, and "(record D7b)" in `dependency-halt.sh`. Each is a plain word before a number, with no session named and no link. That is the class `human-readable-ids` D18-build-done-check as changed at build leaves unrewritten (its "`user-ruled D4`" case).
- PASS — "`D7b`" and "`D7c`" have no card of their own, so they are clause pointers (S8).

VERDICT: PASS

Issues requiring fix: none.

## Departures and flags — dispositions

- Hand rows written by script, and the tool's `--write` bypassing the write hook: no hand row touched a plugin file. All 16 plugin files and all 9 strips return `allow` on first-hand `check`, so nothing escaped a check.
- Release binary from row 026: the 21 writes that touched plugin files all ran on `bin-release` (a:36 onward).
- S3 superseded for `hand-b.tsv:40`, `:46` and `hand-a.tsv:200`: none of the three sites is in a plugin file. It does not apply here.
- Round-1 fix round (77 edits plus 9 strip entries): 0 rows under `plugins/mochiko/`. Every row reads `allow` and `written yes`.
- Exclude list amended with two strip files: used for the done check. Without them the check printed `5 bare · 0 drift` at round 1, exactly the five disclosed lines (`governance-surfaces-template.md:123`, `:193`, `:194`, `:195`; `validation-constitution.md:68`).
- Strip form (superseded lines in an indented fenced `text` block instead of `"…"`): accepted. `strips/README.md` asks for verbatim text, `human-readable-ids` D15-protected-line-rewrites masks fences, and `ids --check` reads every wave-3 fence as masked.
- "Kept deliberately" reasons: units 1, 3, 4, 5, 8 PASS; unit 2 FAIL at round 1, PASS at round 2.

## Done check (wave plan item 10)

Command: `S/apply/bin-release/mochiko-cli ids --check --plugin-root plugins/mochiko`, run from the repo root with all 24 `--exclude` prefixes of the amended Exclude list. Those are the defaults plus `evals/.work/`, `crates/mochiko-cli/tests/ids.rs`, `.mochiko/benchmarks/`, the five `inputs/` prefixes, `wave0-read-test.md`, `wave3-census-raw.md`, the six kits' `variants/` and `pass-report.md`, and the two strip files. Every path exists; the list is in `S/gate-w3/excludes.txt`. This report is not excluded.

Round 2 result, exit 0, after the U2 fix and this report's own token fix:

```
mochiko-cli ids --check · 0 bare · 0 drift
```

Round 1 printed the same line. The lead's run after the U2 fix, before this report's token fix, printed `8 bare · 0 drift`, all eight in this report.

`human-readable-ids` D15-protected-line-rewrites's diff-check line, verified against `apply-log.tsv` (sha256 `1d5903ef1969adbb`):
- Step 2: each of 639 `--write` calls has its own row whose diff-check column begins `diff check:`, and all read "ID tokens only". The 640 preview rows carry `-`; row 091 was previewed twice around its S3 stop.
- Step 3: each of 48 literal `--write` calls has its own `diff check:` row.
- Step 1: 118 per-file rows log `(dry run allow)`, covering all 628 hand rows, plus one summary row.
- Two lead-supersession rows carry no call.
- Every exit code is `0`.
- The round-2 U2 rename has its own line in `fix-log.tsv` row 92: "`diff check: 1 files · 1 spans · ID tokens only`", exit 0.

The read-test clause of `human-readable-ids` D18-build-done-check is ruled met by the user's override, and is not re-judged here.

## Round 2 — U2 re-audit

Same seat (`w3-gate`) and tier (`opus`), resumed, against the same render. The contract and the log are unchanged since round 1. Per the bound, this round reads only what the fix touched and what it could have broken.

Pre-pass, run first-hand:
- `mochiko-cli migrate validate --report --plugin-root plugins/mochiko` gives `0 rejecting · 113 advisory`.
- `migrate status` gives `state sha256:c7e23ab026d07fb3… · 87 documents · 1175 rules`, unchanged.
- Router body is 47,394 (unbudgeted); description 206, unchanged.
- `check` gives `allow` on `SKILL.md` and on `.mochiko/strips/mochiko.md`. `ids --check` on those two files gives `0 bare · 0 drift`.

VALIDATE: `plugins/mochiko/skills/mochiko/SKILL.md` (prose primitive, unconverted skill body) with its `[v0.118.0]` strip entry, re-audited after the round-1 FAIL's fix.
Checklist run: `judgment-items-prose` (coherence, preserved responsibilities), plus the round-1 issue.
Evidence read: `git diff -U0 0b8982a -- plugins/mochiko/skills/mochiko/SKILL.md` (three hunks: :39, :71, :129) · `S/apply/fix-log.tsv` rows 88–94 (sha256 `aed2c2bb87a9f5ac`) · `S/apply/out/gate/step1-check.txt`, `step2-write.txt`, `step4-done-check.txt` · `.mochiko/strips/mochiko.md` `[v0.118.0]` Wave 3, with its `git diff` · my read-back of its Content fence against `git show 0b8982a:`.
- PASS — round-1 issue closed: :71 now reads "`` (`command-architecture-realignment` D5-transport-neutral-harness) ``", the graded map's slug.
- PASS — route: a hand qualify (row 88, `check` `allow`), then a preview showing one hunk (row 91), then the write (row 92, exit 0, "`diff check: 1 files · 1 spans · ID tokens only`").
- PASS — coherence: the owner is right; that decision is the transport-neutral harness the line's "transport choice stays neutral" cites. All three changed lines in the file are joined, and no word-qualified bare `D<n>` or bare GI remains.
- PASS — preserved responsibilities: the write's old and new :71 differ only in the token. The row's routing text is unchanged.
- PASS — strip entry: the Content fence reads 3/3 verbatim against `0b8982a`, and every changed line is claimed. Disposition names :71, and "Kept deliberately: none" is now true. The lead's heading edit names the third cite and touches nothing else.
- PASS — what the fix could have broken: the write touched one file. Its first preview also hit four lines of this report, which the lead ruled out of that call. I quote-masked or joined those lines and the report's other tokens myself in this round. No other mention moved.

VERDICT: PASS

Issues requiring fix: none.

## Outcome lines

- `audit: U1 authoring-constitution pair · w3-gate · opus · 8 files · 1 rounds · 0 blocking`
- Round 1, superseded by the round-2 line below: `audit: U2 mochiko router SKILL.md · w3-gate · opus · 2 files · 1 rounds · 1 blocking`
- `audit: U3 patterns-model-tiering pair · w3-gate · opus · 2 files · 1 rounds · 0 blocking`
- `audit: U4 review-brainstorm/references/EXTERNAL-CLAIMS.md · w3-gate · opus · 2 files · 1 rounds · 0 blocking`
- `audit: U5 validation-constitution references · w3-gate · opus · 3 files · 1 rounds · 0 blocking`
- `audit: U6 validation-primitive-edit pair · w3-gate · opus · 2 files · 1 rounds · 0 blocking`
- `audit: U7 constitution-modules/release-gates.md · w3-gate · opus · 2 files · 1 rounds · 0 blocking`
- `audit: U8 hooks/scripts artifact-gate.sh and dependency-halt.sh · w3-gate · opus · 4 files · 1 rounds · 0 blocking`
- `audit: U2 mochiko router SKILL.md · w3-gate · opus · 2 files · 2 rounds · 0 blocking`
