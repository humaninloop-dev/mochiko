---
report: review
feature: hook-enforced-artifact-schema
round: 3
verdict: PASS
graded_artifact: plugins/mochiko/migrations/0005-artifact-homes.yaml (46 ops, 2,014 lines) and its renders, plus the uncommitted crate/eval pin re-keys
graded_against: wave3-plan.md (approved, R1–R11) · wave3-budget-table.md · record.md D2/D4 (as clarified at G8)/D5/D6/D11 wave 3 · wave1-template-readiness.md · wave3-census-raw.md
reviewer_role: standing non-author reviewer; authored none of it
producer_report: reports/wave3-cycle-report.md
fix_list_count: 8
blocking: []
nothing_edited: true

item_grades:
  - id: 1
    name: validate + replay + views
    grade: PASS
    commands:
      - "mochiko-cli --plugin-root plugins/mochiko migrate validate → `0 rejecting · 104 advisory`, exit 0"
      - "migrate status twice → identical `state sha256:cbb455a6…`, 73 documents, 1039 rules"
      - "views emit --out <scratch> then `diff -rq` against .mochiko/schema-views → no differences, 73 files each side"
    detail: >
      Replay is deterministic across invocations and the emitted views are byte-identical to the
      committed tree. The advisory count moved 105 → 104 against a log replayed without `0005`
      (I built a four-migration copy to check), so the migration removes one advisory and adds none.
      The 26 changed view paths are exactly the log's own change: the 17 primitives that gained an
      `artifact-home` rule (6 commands, 11 skills), the 5 replaced templates, the 3 new templates,
      and the new `homes/` shelf. Nothing else under .mochiko/schema-views moved.
  - id: 2
    name: census honesty
    grade: PASS
    detail: >
      I tallied the raw census myself rather than taking the plan's table. It carries exactly 346
      `###` pattern headings — 199 under `## kinako — 292 files` and 147 under `## mochiko — 274
      files`, so D5's two-sample requirement is met and both repos are in scope. My own
      classification reproduces the plan's §1 collapse row for row: report/review-shaped 174 ·
      brainstorm working files 44 · benchmarks 42 · root-level 14 · declared deliverables and the
      named stragglers 72. They sum to 346.
    dispositions_verified_through_the_binary: >
      Ten sampled patterns, each run as a real `check --hook-json -` payload against the shipped log:
      `.mochiko/features/B53/cycle-1-report.md` deny (`B53/` is not a declared sub-directory of
      `.mochiko/features/`; declared: `desk`) · `features/desk/<date-slug>/derivation.md` allow ·
      `epics/EPIC-001/reviews/card-review.md` deny · `features/FEAT-001/reports/evidence/…` deny ·
      `epics/EPIC-001/landing/close.md` deny · `brainstorms/<slug>/wave1-plan.md` allow ·
      `brainstorms/<slug>/wave1-reports/p1-core.md` deny · `.mochiko/benchmarks/…/README.md` allow
      (R5, out of scope) · root `DECISIONS.md` allow (R6, not a home) · a `reports/` file with no
      envelope deny, its reason naming the home and the file. Every one matches the ruling.
    counts: 20 home documents and 68 `- file:` entries, as stated; I counted both off the YAML.
  - id: 3
    name: rulings landed as ruled
    grade: PASS
    detail: >
      R1 `B53` forbidden — not by a forbid list but by the closed sub-directory set of
      `features-index`, which is the cleaner mechanism; verified denying. R2 one desk home at
      `.mochiko/features/desk/<date-slug>/`, with `desk` the single declared subdir of
      `features-index`. R3 `implement-log.md` carries `form: log` with `entry_max_lines: 60`
      (migration lines 235-237). R4 `reports/` only — `wave<n>-<slug>.md` is a declared deliverable
      and `wave<n>-reports/` is not a subdir, so the 47 files are wave-5 violator material as ruled.
      R5 benchmarks out of scope, allowed. R6 root docs absent from every home, allowed. R7 the
      `tasks` **skeleton** now emits `## Cycle Cards` (producer view line 129, inside the Skeleton
      section) and the conformance block requires the three headings in order. R8 the split landed:
      `architecture-store` renders with no conformance block, `architecture-spine` with one,
      `architecture-concerns` with none. R9a/R9b `features-index` and `governance-surfaces` carry no
      block. R11's five `max_lines`-absent rows are present and are the five the table names —
      `spec`'s User Stories, Edge Cases, Functional Requirements and Success Criteria, and `tasks`'s
      Cycle Cards.
  - id: 4
    name: deviations
    grade: PASS
    rulings:
      seventeen_mints_not_nineteen: >
        OK, and the claim is true. I checked independently: neither `patterns-entity-modeling` nor
        `patterns-api-contracts` has a document in the replayed state — no view file under
        .mochiko/schema-views/skills/ and no section a `rules` call can reach — so no `mint-rule` op
        can target them. Their rule is markdown content and belongs to a wave-4/5 primitive edit
        under the strip ceremony, which the report routes to the lead.
      three_skills_hosted_elsewhere: >
        OK. `sec.artifact` exists only in the authoring family; `patterns-vertical-tdd` took
        `sec.disclosure` and `executing-tdd-cycle` and `analysis-codebase` took `sec.output`. I
        confirmed all 17 sections off the ops. Each is the section that already owns the artifact
        seam in that family, so the rule sits where a reader of that skill would look.
      seven_homes_to_bounds_elsewhere: >
        OK, and each carries its `bounds_cite`: `memory`, `brainstorms-index` and `specs-index` cite
        `.mochiko/memory/knowledge-management.md`; the four `contracts` homes cite the contract's own
        interface per `mochiko:patterns-api-contracts`. This is a real widening — the plan's §2 had
        all seven at whole-file — and it is argued per home in the budget table, which is the
        artifact the user ratifies, and disclosed as deviation 4 in the cycle report. The right place
        for it.
      spec_prototype_deferred: >
        OK. `subdirs` match literally with no token support, so a `spec-prototype` home would have
        denied every nested prototype directory. `prototype/` is a declared subdir of `spec` that no
        home governs — the crate's documented Deferred case, allowed and unchecked. 21 homes → 20,
        disclosed.
  - id: 5
    name: budget table vs D6
    grade: PASS
    detail: >
      I extracted every `heading`/`max_lines` pair from the YAML and compared against the table
      rather than spot-checking ten. All 40 numeric rows agree row for row, and the five
      deliberately-absent rows are absent in the YAML. Every numeric row carries A4p, A4l or RE;
      the `observed` column is present and the table states in two places that it is the gap and not
      the source, which is D6's requirement. The three keys resolve to real rule text, though not
      all to the file the table names — see W1.
    findings: [W1, W2, W3]
  - id: 6
    name: report-envelope
    grade: PASS
    detail: >
      The producer view carries the six enum types verbatim — cycle, verification, final-validation,
      review, feasibility, disclosure — as the only required frontmatter field, plus three real
      sections at 15 lines each (`## Failure narrative`, `## Notes of note`, `## Null-exit
      reasoning`), all optional, plus two placeholder tokens. No undeclared-heading deny line, which
      matches the table's `allow` posture for an envelope serving six types. A `report: invented`
      payload denies against the shipped log and the reason lists all six allowed values, so the
      wave-4 rows have something real to bind to.
  - id: 7
    name: forward constraint
    grade: PASS
    detail: >
      All 46 ops are in the one file — 23 `import-document` (20 homes, 3 templates), 5
      `replace-document` (the five templates that gained conformance keys), 17 `mint-rule`, 1
      `reword-rule` — so the conformance keys and the `home` ops cannot be split across migrations
      and an older reader halts on the unknown kind before it can read a budget-less template.
      `mochiko-cli -V` reports `grammar 1..1` and `migrate status` reports `grammar 1`, unchanged.
  - id: 8
    name: crate and eval touches
    grade: PASS
    detail: >
      Wave 1 is committed (c395dab), so the working tree is wave 3 alone. Scope under `crates/` and
      `evals/` is tests, test fixtures and the contract suite only — **no `src/` file is touched**.
      Every change is a pin the log determines and each carries a comment naming `0005`: the doc
      count 50 → 73 and the rule counts 1022 → 1039 (327 command, 712 skill) and the floor counts
      110 → 116 and 228 → 239, all of which match my own `migrate status` and the validate census;
      the `home` view-kind arm in `shipped_documents`, needed because a home view carries no `kind:`
      discriminator; `KINDS_NOT_SHIPPED_YET` shrunk to empty, which is wave 1's self-deleting assert
      firing exactly as designed; the similarity pins 1022 → 1039 scanned with the cluster count
      holding at 0; the three new template names in `SHIPPED_TEMPLATES`; and the five producer-view
      fixtures at 74 insertions and 0 deletions, which I confirmed by `git diff --numstat` and which
      is the report's own additivity claim.
    gates_my_own_run: "cargo test --all 453 passed 0 failed · fmt exit 0 · clippy -D warnings exit 0 · audit exit 0, 31 dependencies"
    contract_suite: "`python3 evals/contract/run.py --host-only` → 4/4 cases passed, 4 measurements recorded; the deliverables case rendered 11 templates × 2 views plus 3 documents"
    findings: [W6, W7]
  - id: 9
    name: anchors
    grade: PASS
    detail: >
      All 17 mints carry `anchor: 2026-09-13 hook-enforced-artifact-schema D1`, each beside a
      `class: floor` and its own id; I listed them off the ops rather than sampling. The
      `reword-rule` on `impl.reports-envelope` needs none, confirmed both ways: the rule in the
      committed view carries `labels`, `class`, `kind` and `text` and no `anchor:` field either
      before or after the reword, while 35 other rules in the same document do carry one — so the
      field was available and deliberately not added to a rule that never had one.
  - id: 10
    name: the two items the seat raised
    grade: PASS — both real, both graded below
    detail: >
      Both bite, neither blocks. On the first I have a correction in the seat's favour: the five
      `Home*` codes are absent from `Code::REJECTING`, but `Code::severity()` returns `Reject` for
      everything outside the explicit `ADVISORY` list, so they do reject at runtime. I proved it
      rather than reading it — a scratch log with an unknown segment token, a dangling template
      binding and `bounds: elsewhere` without a cite reports `3 rejecting · 0 advisory` and exits 1.
      The gap is test coverage, not behaviour.
    findings: [W4, W5]

findings:
  - {id: W1, type: citation, sev: Minor,
     at: "wave3-budget-table.md, `## The three keys`, the A4p and A4l entries",
     gap: "Both are attributed to \"the ADR's rule 4\". The ADR carries no such rule; its own line 32 refers to `artifact-format.md` rule 4 (\"reported, never graded\") and its ruling section records that it amended that file's rule 4/8. The quoted text — \"Overview / context / rationale prose defaults to ≤ 3 lines\" and \"list entries … are one line each\" — is verbatim `plugins/mochiko/templates/artifact-format.md` rule 4 (\"Size guidance\"), lines 38-40. The RE key's first half is genuinely the ADR's line 82 and its second half is `report-format.md` rule 2. So 38 of the 40 numeric rows point a reader at a file that does not contain their basis.",
     fix: "Re-point A4p and A4l at `templates/artifact-format.md` rule 4 as amended by the 2026-08-22 ADR, and split the RE key's two sources. The numbers do not move."}
  - {id: W2, type: derivation, sev: Minor,
     at: "wave3-budget-table.md, `## Whole-file bounds — the template-less deliverables`",
     gap: "Every per-section row carries a key; the whole-file table carries none. 150 lines across 26 file names and 300 for `baseline-delta.md` are stated without a derivation, and `artifact-format.md` and the ADR carry no document-level line figure I could find. D6 requires each budget set at the compact target the ADR named for that class, so an unkeyed number is the one shape D6 excludes — even though it is plainly not derived from observed sizes, since the six named gaps run from 836 to 4,745 lines against it.",
     fix: "Name the basis for 150 and for 300, or state plainly that both are the seat's call offered for ratification with no ADR class behind them, so the user ratifies a number knowing which kind it is."}
  - {id: W3, type: disclosure, sev: Minor,
     at: "wave3-budget-table.md, `## What ratification costs today`",
     gap: "The source rule the budgets are keyed to says they are \"defaults, not caps on substance — a genuinely complex [case may exceed]\", and the ADR's own ruling made exceeding them *an advisory finding a reviewer names* when undisclosed. `0005` converts them into a mechanical deny, and the gate has no channel for a disclosed justification: the only escape is first-touch amnesty, which needs a file already on disk. A brand-new artifact with an honestly long section cannot disclose anything and is denied. D6 rules the deny, so this is not a build defect — but the user ratifying the table is ratifying a change in kind, from author-disclosable default to unappealable cap, and the table does not say so.",
     fix: "One line in `What ratification costs today`: past the budget, the justification that used to travel with the artifact now travels as a reviewer finding or a migration, and a new artifact has no amnesty."}
  - {id: W4, type: test-coverage, sev: Minor,
     at: "crates/mochiko-cli/src/validate.rs:205 `Code::REJECTING` (46 entries) against tests/validate.rs:1686",
     gap: "The five `Home*` codes reject correctly — I verified `3 rejecting`, exit 1, on a deliberately broken home — but they are absent from `Code::REJECTING`, and that constant is the manifest the coverage test compares the probe corpus against. Two consequences: the five home checks have no probe-coverage assertion, so a regression that stopped raising one would pass that test; and the same test asserts no raised code sits outside the constant, so the day a probe corpus does raise a home code it fails with \"these codes were raised but are not in Code::REJECTING\". The individual checks are covered in tests/home.rs, which is what keeps this Minor.",
     fix: "Add the five codes to `Code::REJECTING` and give the probe corpus one home document that raises each — a crate change, so it belongs to whichever wave next licenses one."}
  - {id: W5, type: documentation, sev: Minor,
     at: "plugins/mochiko/migrations/README.md lines 88-90",
     gap: "The kind list still reads `command`, `skill`, `command-common`, `skill-common`, `command-labels`, `skill-labels`, `template`, `shelf` — eight kinds, no `home`, in the document a future migration author reads to learn the grammar. The log now carries 20 home documents.",
     fix: "Add `home` to that list. It is the same file whose Change-ops section the wave-1 grammar decision leaned on, so it is worth keeping current."}
  - {id: W6, type: fixture-contract, sev: Minor,
     at: "evals/contract/expected-skills.json, the `provenance` string and the eleven `floor_pin` values",
     gap: "The fixture's own provenance says it was frozen before wave 5's conversion and \"Never edited after; a floor rule added or renamed later breaks the cross-check in `converted-shape` rather than regrading quietly.\" Wave 3 added eleven `artifact-home` floor ids and incremented eleven `floor_pin` values — which is the absorb-quietly path that sentence exists to forbid. The lead licensed the re-key, so the edit is authorised; what is left is a fixture whose text now contradicts its own history.",
     fix: "Record the wave-3 re-key inside that provenance string, the way the floor-ids row in the README already records both the 2026-09-05 and 2026-09-13 rulings. Otherwise the next reader trusts a freeze that no longer holds."}
  - {id: W7, type: scope, sev: Minor,
     at: "evals/contract/expected-skills.json, the `provenance` and `binary` strings",
     gap: "Both were re-encoded from `\\u00a7` and `\\u00b7` escapes to literal `§` and `·`. The decoded values are identical, so nothing the suite reads has changed — but it is a byte edit the log does not determine, inside the one file this wave was licensed to touch only for pins.",
     fix: "Restore the two escapes, or note the re-encode in the cycle report's crate-touches list so the diff has no unexplained line."}
  - {id: W8, type: accuracy, sev: Minor,
     at: "reports/wave3-cycle-report.md, the `Not a deviation, worth stating` paragraph on R9a",
     gap: "It says `features-index`'s \"only placeholders sit in a level-1 title\". They sit in table cells — `{{capability_name}}`, `{{status}}`, `{{work_row}}` and friends; the level-1 `# Features` carries none. The conclusion is unaffected, since a table cell is as far outside D4c's frontmatter-and-heading surface as a title is, and I agree a conformance block there would assert nothing.",
     fix: "Correct the clause to name table cells. The reasoning stands as written."}

strengths: the census tally reproduces at 346 across both repos and every sampled disposition behaves as ruled when run through the binary; the resolver does R1's forbid with a closed subdir set rather than a forbid list, so B53, reviews/, landing/ and reports/evidence/ all fall out of one mechanism; the five template replaces are provably additive at 74 insertions and 0 deletions; the 40 budget rows match the YAML row for row and the observed column is labelled as the gap twice; every one of the 17 mints carries the D1 anchor and names its own home rather than sharing a block, with the similarity detector's cluster count holding at 0 as the evidence; the seven bounds-elsewhere widenings are each argued in the artifact the user ratifies; the two mechanism findings and the report's own forbidden location are volunteered rather than found.
---

## Notes of note

**Three forward items, none of them findings.**

The first is a release-order dependency the cycle report names and I want to restate in one place: a
published binary that predates the `home` kind rejects `0005` loudly with an `op-malformed` finding,
which is GI-020 working as designed. It also means the plugin cannot ship this migration before the
crate publishes. Every `plugin.json` bump from here needs the crate tag ahead of it, and until the
tag exists every mochiko command in this repo needs the break-glass install. That belongs on the
release gate, not in a wave report.

The second: this review and the cycle report beside it both sit in `wave3-reports/`, which `0005`
forbids — `wave<n>-reports/` is not a declared sub-directory of the session home. The seat disclosed
the same about its own report. Written where the lead asked, both are violator-pass material with the
other 47 files. Worth saying out loud that the gate's first real bite lands on the build's own
paperwork, because that is the wave-5 pass's least sympathetic and most useful test case.

The third is for wave 4 rather than here. The shared fixture log's `report-envelope` still declares
`sections: []`, so the contract suite sees only the frontmatter half of the envelope binding, while
the shipped envelope now carries three budgeted sections. If the suite is meant to exercise the
heading, placeholder and budget legs on a report, its fixture envelope needs sections — the shipped
one cannot stand in, since the suite reads the fixture by design.

**What I could not check.** The 78 sandboxed contract cases were not run here either; the four host
cases are green on my own run, and the gate that needs the rest is wave 4's bump. I also did not
re-derive the `observed` column of the budget table from the corpus — it is stated as the gap rather
than a source, so a wrong observed figure would mislead a ratification conversation without moving a
single budget.

## Delta-check

Fresh non-author reviewer, 2026-09-14, taking over from the seat that graded the migration above.
Scope: the eight Minor fix items in this review's own fix list, re-read against the artifacts on
disk, plus an independent gate run. I authored none of the graded material and edited nothing
outside this section.

**Per-item verdicts.**

- **W1 — held.** `wave3-budget-table.md`'s `## The three keys, and where each one's text lives` now
  attributes A4p to `plugins/mochiko/templates/artifact-format.md` rule 4 ("Size guidance"), "as
  amended by the 2026-08-22 ADR, which carries no rule 4 of its own", and A4l to "the same rule". I
  checked both quotations against the source rather than the fix note: `artifact-format.md` rule 4
  carries "Overview / context / rationale prose defaults to ≤ 3 lines" and "list entries … are one
  line each" verbatim, and the ADR's own line refers outward to "`artifact-format.md` rule 8 … and
  rule 4 ('reported, never graded')", so the ADR indeed has no rule 4. The RE key is split into its
  two sources — the ADR's watch line "reviews should land at report-envelope scale (KBs, not tens of
  KBs)", which I confirmed in the ADR, and `report-format.md` rules 2–3, which are "Conditional
  prose — the failure exception" and "Omit empty". The split names rule 3 alongside rule 2, wider
  than the fix note asked and correct: rule 3 is what makes a clean report frontmatter-only. No
  budget number moved; the 40 numeric rows are unchanged.

- **W2 — held.** A fourth key **S** is now declared — "the seat's call, no class behind it; the
  whole-file bounds are the only rows keyed so" — and `## Whole-file bounds — the template-less
  deliverables` opens "All keyed **S**: no document-level line figure exists in artifact-format.md
  or the ADR, so 150 and 300 are the seat's call rather than a derivation, and not read off the
  corpus either." That is the second branch of the fix — stated plainly as the seat's call with no
  ADR class behind it — and it closes both halves of the gap, the missing key and the missing
  provenance. The widest-gaps line below it still names `baseline-delta.md` at 4,745 against 300, so
  the reader ratifying the number sees the size of the bet.

- **W3 — held.** `## What ratification costs today` now opens with a bolded **A change in kind,
  first.** paragraph: rule 4 calls the numbers "defaults, not caps on substance", the ADR made an
  undisclosed overage an advisory finding, and "Under the gate they become **caps**: a write past a
  budget is denied, and the hook has no channel for the disclosure that used to ride with the
  artifact — past the budget that justification travels as a reviewer finding or as a migration
  moving the number." It then names the escapes as structural rather than editorial — first-touch
  amnesty, `extra_headings: allow`, splitting into declared sections — and closes "A new artifact
  with an honestly long section has none of them." Every clause the fix asked for is present, and
  the change-in-kind is stated before the mitigations rather than after.

- **W4 — held.** `crates/mochiko-cli/src/validate.rs` carries `Code::REJECTING` at 51 entries, up
  from 46, with `HomeShape`, `HomePattern`, `HomeDuplicate`, `HomeBinding` and `HomeBounds` added
  under a comment recording that `severity()` already rejected them and what was missing was their
  place in the manifest the coverage guard reads. The probe corpus in `tests/validate.rs` gained six
  home probes covering the five codes: a non-list `path` (shape), a bogus segment token (pattern), a
  deliverable naming an absent template (binding), `bounds: elsewhere` with no `bounds_cite` and a
  template-less deliverable with no whole-file bound (the two probed `home-bounds` arms), and a
  two-document insertion for the duplicate-path check, which needs two homes rather than one. Each
  is a real YAML document inserted beside the corpus, which is the right shape for an opaque kind.
  `every_rejecting_code_is_raised_by_some_probe` is green in my own run, so the manifest and the
  corpus now agree in both directions.

- **W5 — held, and additive.** `plugins/mochiko/migrations/README.md` lists `home` as the ninth
  kind, and a following paragraph records that it arrived at the 2026-09-13 wave, that it is opaque
  like `template` and `shelf`, what it declares, and that only `import-document` and
  `replace-document` apply to it. The diff is six insertions against one deletion, the deletion
  being the reflowed list line. Nothing else in the file moved.

- **W6 — held.** The `provenance` string in `evals/contract/expected-skills.json` keeps its original
  freeze sentence and appends the re-key: "Re-keyed once since, and only for ids the log determines:
  the 2026-09-13 hook-enforced-artifact-schema wave added one `artifact-home` floor to each of
  eleven skills (`floor_ids` plus `floor_pin`), on the lead's licence, with the same move recorded
  in this directory's README floor-ids row. Every byte-size field is still the freeze's own, and no
  other field has been touched." I checked the last clause rather than taking it: the diff's only
  added lines are eleven `floor_pin` values, eleven `<skill>.artifact-home` floor ids, and the
  provenance string itself. No byte-size or other field is touched, so the string is true about its
  own diff.

- **W7 — held.** The file is pure ASCII on disk — a `grep` for any byte outside the printable ASCII
  range returns nothing — and both strings carry their escapes again: `"binary": "mochiko-cli 0.1.0
  · grammar 1..1"` is absent from the diff entirely, meaning it was restored to its committed
  bytes, and the `provenance` line's `§` escapes appear on both the removed and the added side.
  The unexplained re-encode is gone; what remains in the diff is the pins and the provenance text,
  which the log and the lead's licence determine.

- **W8 — held.** The `Not a deviation, worth stating` paragraph in `cycle-report.md` now reads that
  `features-index`'s "placeholders sit in table cells (`{{capability_name}}`, `{{status}}`,
  `{{work_row}}`), which is as far outside D4c's frontmatter-and-heading surface as a title would
  be". The claim matches the artifact and the reasoning is preserved rather than rewritten, which is
  what the fix asked.

**Gates, run by me on the working tree as it now stands.**

- `cargo test --all` — 453 passed, 0 failed across 17 test binaries, exit 0.
- `cargo fmt --all --check` — exit 0.
- `cargo clippy --all-targets -- -D warnings` — exit 0, zero warnings; I touched `src/validate.rs`
  and `tests/validate.rs` first to force a fresh lint rather than accept a warm cache.
- `mochiko-cli migrate validate --plugin-root plugins/mochiko` — `0 rejecting · 104 advisory`,
  exit 0. Binary `mochiko-cli 0.1.0 · grammar 1..1`.

**The migration is untouched by the fix round.** `migrate status` reports `state
sha256:cbb455a6c5407e54d6f8fba8346f911268d6942e3ede3c80603addbe4389083f · 73 documents · 1039
rules`, byte-identical to the `cbb455a6…` this review recorded at item 1 before the fixes — a state
hash that would move if any op in `0005` had changed. Three further checks agree: the migration's
own stamped `hash: sha256:c3eb3dd3…` verifies clean under `migrate validate`, which raises a body
hash mismatch as a rejecting finding and raised none; under `plugins/mochiko/migrations/` only
`README.md` is modified in `git status`, with `0005-artifact-homes.yaml` the sole new file; and the
file's mtime (19:58 on 2026-09-13) precedes this review (20:19), while every fixed artifact's mtime
follows it — README 20:22, budget table 20:25, cycle report 20:25.

**Not re-checked here**, and unchanged from the review's own limits: the 78 sandboxed contract
cases, and the `observed` column of the budget table, which is stated as the gap rather than as a
source.

**Status: CLEAN** — 8 of 8 Minor items held, no new findings, all five gates green.
