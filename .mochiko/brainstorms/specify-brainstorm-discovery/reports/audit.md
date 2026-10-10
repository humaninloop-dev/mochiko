---
report: review
round: 1
grader: g2-gate-grader
tier: opus
wave: 1
units: [1, 2, 3]
---

## Unit 1 — 0051 and its view diff

VALIDATE: schema content — `plugins/mochiko/migrations/0051-specify-story-stage.yaml` with its regenerated view diff (`.mochiko/schema-views/commands/specify.yaml`) and the one allowlist row its mints raised (`scripts/similar-rules-allowlist.yaml`).

Checklist run: `validation-primitive-edit.judgment-items-schema`, the AM-2 five: intent stated · anchor present where the exit requires one · ID lifecycle right · floor and fail survival · register. Each obligation was also held against the ruling it executes (`.mochiko/brainstorms/specify-brainstorm-discovery/record.md`, D1–D6 and the confirmed defaults batch).

Evidence read: the 0051 migration file, read whole · `git diff -- .mochiko/schema-views/commands/specify.yaml`, read whole · `git diff -- scripts/similar-rules-allowlist.yaml` · the full current specify view (vars block, roles section, fail-conditions section) · the HEAD specify view, read with `git show` · `plugins/mochiko/migrations/README.md` · the record's `## Decisions`, `## Defaults batch — confirmed`, cold-review and verify sections · `.mochiko/schema-views/commands/brainstorm.yaml` (the `brainstorm.own-rules-win` precedent) · the label registries · the `DECISIONS.md` 2026-10-10 row · the rendered specify preamble at the current state and at a 0050 prefix replay.

Pre-pass: these commands were run first-hand from the repo root at the current tree.

```
$ mochiko-cli migrate validate --report --plugin-root plugins/mochiko
pointer resolution: 88 checked against plugins/mochiko
=== similar-rule clusters (threshold 0.60) ===
none — no pair clears the threshold
rules scanned: 1196 · in-kind pairs scored: 200540 · clusters: 0 (none)
allowlist-suppressed edges: 185
mochiko-cli migrate validate · 0 rejecting · 113 advisory

$ mochiko-cli migrate status --plugin-root plugins/mochiko
log plugins/mochiko/migrations · grammar 2 · sequences 1..56 (54 migrations)
state sha256:fad81e3486c53fe2f8f41fbc62892300f46c2c620c781f101addb7cb8d54568c · 87 documents · 1196 rules

$ mochiko-cli views emit --plugin-root plugins/mochiko --out <scratch> && diff -r <scratch> .mochiko/schema-views
(no output: the committed views equal the replay)
```

Advisory delta against a 0050 prefix replay: `spec.fail.story-outside-slate` joins the specify `enforces-coverage` list, which is expected for a fail node. The `zero-member-label · command/specify · attempt-economy` finding clears because `spec.story-stage-stop-rule` carries that label. Under HEAD's allowlist, the replay raises exactly one cluster, `CROSS-PAIR · kind: constraint · best 0.87 · brainstorm.own-rules-win / spec.own-rules-win`. The new allowlist row suppresses that edge and no other, and its reason cites D5-specify-own-rules. The specify render grows from 20,118 to 27,999 characters, all of it from 0051. The payload is 32,387 characters: a body of 4,388 plus that render. Commands carry no ledger budget, so this figure is context only.

- intent stated — PASS. The header names the two moments, the `slate_source` condition, three id-keeping rewords, the stage and slate mints, the PM's slate-close vet and one `kind: fail`, and it states that no floor or fail is retired. Every op matches that list, except the unstated var swap held under register below.
- anchor present where the exit requires one — PASS. 0051 makes no supersession, tombstone or protection lowering. Its rewords keep their existing anchors, including the 2026-08-02 anchor on `spec.lockstep-prototyping`. The header anchor is well-formed and resolves to the `DECISIONS.md` 2026-10-10 row. Each decision segment on a mint matches its card: D1 on the stage, the reservation and the fail, D2 on the slate unit, D3 on lead-holds, D4 on the blind list, D5 on own-rules-win and D6 on the gate.
- ID lifecycle right — PASS. The three rewords keep their ids. The 19 mints are all fresh: four in roles, two in reserved, eight in tools, four in ways-of-working and one fail. That is the section growth from 9·5·17·9·9 to 13·7·25·13·10 at the 0050 replay, and mint-once is clean at 0 rejecting. Nothing is tombstoned. Each reword carries its prior obligations forward: skeleton-first and wet click-through, the rejection recorded with its why, and the streak flag.
- floor and fail survival — PASS. No existing floor or fail is touched. The new rule `spec.fail.story-outside-slate` is `class: floor · kind: fail`, and each id in its `enforces:` list resolves to a live local rule. The render pins move from `kind: fail · 9` and `class: floor · 17` to `10` and `18`. The fail text P1 disclosed rewording keeps the record's meaning, "a story authored with no slate ruled, or outside the ruled slate", and the replay now raises no cluster on it.
- register — FAIL. The schema declares `vars: pm_seat: product-manager`, and at HEAD every one of its six PM references binds `${pm_seat}`. Not one names the seat literally. The 0051 reword of `spec.filter-rejections-recorded` replaces `${pm_seat}` with the literal `product-manager`, a change its intent does not state. The mints `spec.pm-vets-slate-once` and `spec.slate-rulings-users` also name `product-manager` literally. The render is identical today. A later `set-var pm_seat`, the op the log provides for exactly this, would leave these three rules naming the old seat. The rest of the register holds: the second-person lead voice, the noun-phrase fail text, the labels and the `when:` guards (`slate_source` covers both its values, and no condition-coverage finding is raised).
- coverage of the ruling (context, not a sixth item): every specify-side obligation of D1–D6 and of the defaults batch is carried. That covers the moments and gate (D1, D6), the line, priority and seed unit (D2), the lead-held stage and pen hand-off (D3), the blind list's seat and alias (D4), and the precedence clause (D5). It also covers the brainstorm mechanisms, the provenance values, the stall clause, floor-obligated strikes with the waiver hold, skip as handed-in, slate-not-the-frame, the filter timing and UX amendments. Roster disclosure of the blind-list seat is single-homed in `patterns-model-tiering.seat-roster-disclosure`, so it is correctly not restated. The Goal-step clause belongs to `specify.md`, a later unit.

VERDICT: FAIL

Issues requiring fix:
1. register — 0051 drops the schema's own `${pm_seat}` binding from three rule texts. `spec.filter-rejections-recorded` lost it in a reword that removed an existing var use, and `spec.pm-vets-slate-once` and `spec.slate-rulings-users` were minted with it written literally. Fix: add one migration from the held range, `0057`, carrying three `reword-rule` ops. Each op restores `${pm_seat}` in place of `product-manager` and leaves the rest of its text verbatim. Re-stamp the file, re-run `migrate validate`, and re-emit the views. The resolved render stays byte-identical, so no budget figure moves.

## Unit 2 — 0052–0055 and their view diffs

VALIDATE: schema content — `0052-stories-from-slate.yaml` · `0053-review-slate-checks.yaml` · `0054-fr-cites-story.yaml` · `0055-map-slate-touchpoint.yaml`, each with its regenerated view diff (`.mochiko/schema-views/skills/authoring-user-stories.yaml` · `review-specifications.yaml` · `authoring-requirements.yaml` · `authoring-feature-map.yaml`).

Checklist run: `validation-primitive-edit.judgment-items-schema`, the AM-2 five, applied to each migration and its view diff. Each was also held against the D5-specify-own-rules touch-set and the defaults *Slate home and binding* (as repaired at N2 and N3) and *Filter*.

Evidence read: all four migration files, read whole · the four `git diff` view diffs, read whole · the current views of `authoring-user-stories` (scope, artifact and scenario-bound rules; no conditions block), `review-specifications` (the verdict section, its severity grammar and the `manifest-present` precedent) and `authoring-feature-map` (its filter mentions) · the skill-labels registry · `.mochiko/schema-views/homes/spec-stories.yaml`, whose story file is `US-<n>.md` · the record sections cited above.

Pre-pass: the same first-hand run as unit 1 (0 rejecting · 113 advisory · clusters 0 · 185 allowlist-suppressed edges · 1196 rules scanned; views equal the replay). Advisory delta: 0053 adds `condition-coverage · skill/review-specifications · slate-present · value "absent" is declared but named by no rule's when:`. Its sibling `manifest-present` raises the same finding, so it is not held as a defect. Stepped prefix replays reproduce the ledger's v0.118.0 render figures exactly at 0050, which validates the measurement method. The render deltas are attributable one migration each.

| skill | render delta | payload now (body + render) | ledger budget |
|---|---|---|---|
| authoring-user-stories | +717, from 0052 | 14,445 (4,531 + 9,914) | 13,444 |
| review-specifications | +747, from 0053 | 18,105 (3,441 + 14,664) | 16,174 |
| authoring-requirements | +74, from 0054 | 12,948 (3,535 + 9,413) | 12,373 |
| authoring-feature-map | +78, from 0055 | 22,783 (5,954 + 16,829) | 22,323 |

These are context here and carry into the skill-pair units.

- intent stated — PASS. Each header states its change and the ruling it executes: the slate binding (0052), the four checks guarded by the slate header (0053), the FR story cite as the carrier of check (3) (0054), and the slate-close vet with post-stories homing and dedup (0055). Every op matches its header.
- anchor present where the exit requires one — PASS. None of the four makes a supersession, tombstone or lowering. The rewords keep each rule's prior anchor, as the log's own precedent does: 0047's reword of `authoring-requirements.fr-format` kept the 2026-07-25 anchor. All four header anchors are well-formed.
- ID lifecycle right — PASS. Five rewords keep their ids: `story-structure`, `independent-test-required`, `complete-coverage`, `fr-format` and `four-touchpoints`. There are two fresh mints, `authoring-user-stories.slate-fidelity` and `review-specifications.slate-checks`, and one new condition, `slate-present`. Nothing is tombstoned. Each reword carries its prior obligations forward: the 2–5 bound for a no-slate caller, the story structure list verbatim, the QA-isolation text verbatim, the coverage trio, the RFC 2119 trio and the four touchpoints.
- floor and fail survival — PASS. No floor is touched. The `authoring-user-stories.pm-frame-boundary` floor stands, and `slate-fidelity` does not contradict it, since the user owns which stories and the craft owns how well. The skills carry no `kind: fail`, and `slate-checks` is `kind: duty`.
- register — PASS. "Slate Critical checks, each failure a blocking gap" matches its siblings "Feature-layer Critical checks" and "Screens & Flows Critical checks". `slate-present` mirrors `manifest-present`: surface-presence, `when: {…: present}`, and the same absent-value advisory. `authoring-user-stories` declares no conditions block, so its in-text "where a slate exists" guards follow that skill's standing form. FR cites use the joined `US-<n>-<slug>`, and story file names stay `US-<n>.md` per the home.
- coverage of the ruling (context): all of the following are carried. The seed expands without change of meaning, and a `test: open` row becomes an Open Questions entry plus a provisional test (D2 as changed). The 2–5 bound yields to the slate's count. "Landing in `spec.md`" now reads as the index row plus the story file. All four checks are present, with the waiver-pending row not yet due (N2) and the FR cite as check (3)'s carrier (N3). The filter's judgment half runs once at slate close.

VERDICT: PASS

Issues requiring fix: none.

## Unit 3 — 0056 and its view diff

VALIDATE: schema content — `plugins/mochiko/migrations/0056-spec-template-slate.yaml` with its regenerated view diff (`.mochiko/schema-views/templates/spec.yaml`), against the prior content in `0046-joined-id-templates.yaml` and the render `mochiko-cli template spec --plugin-root plugins/mochiko`.

Checklist run: `validation-primitive-edit.judgment-items-schema`, the AM-2 five. The template was also held against the defaults *Slate home and binding* and *Size and skip* and the D5 touch-set items for the template.

Evidence read: the 0056 header and op · `git diff -- .mochiko/schema-views/templates/spec.yaml`, read whole · the current template view's header, User Stories and FR sections and its `conformance` block · the full render of `mochiko-cli template spec` (240 lines), including its Conformance and Skeleton sections · the 0046 `replace-document` for the spec template · HEAD views compared with views emitted from a 0050 prefix replay. All six touched views at HEAD equal the 0050 replay, so the view diff is exactly 0051–0056's delta.

Pre-pass: the same first-hand run as unit 1 (0 rejecting · views equal the replay). 0056 adds 0 characters to every render measured; templates are unbudgeted. Conformance dry run: the filled skeleton was sent through `mochiko-cli check --path .mochiko/specs/grader-probe/spec.md --content -`. With the `---` separators removed, it returned `"permissionDecision":"allow"`. With them kept, it was denied: "`## Intent` is 11 lines against a budget of 10". The same deny occurs at the 0050 replay, so it predates this wave (see Notes of note).

- intent stated — PASS. The header names the header line, the Test column, the two-phase Disposition, the Struck candidates sub-table and the FR story cite. It states that conformance, headings and budgets are unchanged, and the view diff confirms it: the change is confined to the User Stories contract and check, the FR contract and check, and the skeleton's User Stories block.
- anchor present where the exit requires one — PASS. A template `replace-document` touches no protected rule, so no anchor is required. The header anchor is present and well-formed anyway.
- ID lifecycle right — PASS. A template carries no rule ids. The joined US, FR, FEAT and SC forms that 0046 wrote survive in the contract and skeleton.
- floor and fail survival — PASS. The template carries no floor or fail. `conformance: extra_headings: deny`, the required-heading order and every `max_lines` are unchanged.
- register — PASS. The contract and check prose keeps the template's own form, and every prior element is kept: index only, per-story files, the `rejected` story-native status, the FEAT-ID derivation and the columns. The header line `> Slate: {{slate_source}}` matches the `review-specifications` `slate-present` note, "`> Slate: …`". `### Struck candidates` is legal under the rendered rule "An undeclared `##` heading is denied. `###` and deeper are yours." The User Stories section carries no line budget, consistent with "the story count is the feature's, the section's length is not".
- coverage of the ruling (context): all of the following are carried. The columns are additive, with Test added and Feature kept. Disposition has two phases. `in — waiver pending` rows are unlinked in the index, and `out — <why>` rows are unlinked in the sub-table. The provenance mark and any narrowing cut sit in the Story cell. The skip is recorded in the header line, never in the Intent (N1). Each FR cites its story (N3).

VERDICT: PASS

Issues requiring fix: none.

## Failure narrative

Unit 1 failed on register alone. The specify schema single-homes the PM seat's name in `vars: pm_seat`, and HEAD used that var at every PM reference. 0051's reword of `spec.filter-rejections-recorded` swapped the var for its resolved value without stating it, and two new mints named the seat literally too. The render resolves either form to the same text, so `migrate validate` cannot see the difference. The grader found it by diffing the HEAD and current views and grepping both for `${pm_seat}` and `product-manager`. It is held blocking because the next `set-var pm_seat` would silently miss three rules. The fix is mechanical: one `0057` with three rewords. On resume, re-audit reads only `0057`, its view diff and the validate output.

## Notes of note

- Non-blocking, unit 1: `spec.slate-home` names `mochiko-cli template spec` literally although `${spec_schema}` exists. HEAD already mixes the two forms for that var, so the gap was not held. Folding it into `0057` would cost one more reword.
- Non-blocking: the condition value `handed-in` is a token, while the header and provenance prose say `handed in`. A run could write either form in the header line.
- Pre-existing, not this wave: the spec skeleton's `## Intent` block counts 11 lines against its budget of 10 once its `---` separator is counted. It is the same at the 0050 replay.
- For later units: the contract suite's frozen specify floor set must gain `spec.fail.story-outside-slate`, since the render pins are now fail 10 and floor 18. The four skills' payloads exceed their ledger budgets, as the table in unit 2 shows, so the argued-overage path is owed. `authoring-feature-map`'s description measures 598 characters with the canonical snippet, against a ledger row of 495 and a budget of 619.
- Seat disclosure: no `Explore` subagent was spawned. The locate reads were single targeted greps run on the seat tier, each feeding one judgment item. The stepped prefix replays used scratchpad copies of the log. Under `--log-dir` the cluster scan finds no allowlist unless a `scripts/` ancestor exists, so the allowlist-edge check ran in a scratch tree carrying HEAD's allowlist.

## Unit 1, round 2 — 0057 re-audit

VALIDATE: schema content — `plugins/mochiko/migrations/0057-specify-var-bindings.yaml` with its regenerated view diff (`.mochiko/schema-views/commands/specify.yaml`). This is the fix for the round-1 register FAIL. The same seat was resumed and read only what the fix touched and what it could have broken.

Checklist run: `validation-primitive-edit.judgment-items-schema`, the AM-2 five, applied to `0057`. The round-1 issue was re-checked against the view, and the specify render was compared against its pre-fix state.

Evidence read: the 0057 migration file, read whole · the specify view diffed between the round-1 emit and the current one · byte comparisons of `0001`–`0056` against the copies the grader took in round 1 · the full specify render (preamble plus six sections) at a 0056 prefix replay and at the current state.

Pre-pass: these commands were run first-hand from the repo root.

```
$ mochiko-cli migrate validate --report --plugin-root plugins/mochiko
pointer resolution: 88 checked against plugins/mochiko
=== similar-rule clusters (threshold 0.60) ===
none — no pair clears the threshold
rules scanned: 1196 · in-kind pairs scored: 200540 · clusters: 0 (none)
allowlist-suppressed edges: 185
mochiko-cli migrate validate · 0 rejecting · 113 advisory

$ mochiko-cli migrate status --plugin-root plugins/mochiko
log plugins/mochiko/migrations · grammar 2 · sequences 1..57 (55 migrations)
state sha256:603bb29408410b72f5b16e15b82e89248352db4d1efd185099a7cc1eb5294732 · 87 documents · 1196 rules

specify render, all seven blocks, 0056 replay vs current: identical (27,999 = 27,999 characters)
views emit to scratch, diff -r against .mochiko/schema-views: identical
0001–0056 against the round-1 copies: no file changed
```

- intent stated — PASS. The header names the gate-audit fix, the four rules and both var restorations. It states that each text is otherwise verbatim and that the render is unchanged, and the byte-identical render proves that claim.
- anchor present where the exit requires one — PASS. The file makes four rewords and no supersession, tombstone or lowering. Each rule keeps its existing anchor, since the view diff touches text lines only. The header anchor is well-formed.
- ID lifecycle right — PASS. Four rewords keep their ids, and the fix adds no mints and no tombstones. The rule count stays at 1196.
- floor and fail survival — PASS. No floor or fail is touched. Because the render is byte-identical, the pins stay at `kind: fail · 10` and `class: floor · 18`.
- register — PASS. The round-1 issue is closed. The view now binds `${pm_seat}` at all nine product-manager references, and no literal `product-manager` remains in any rule text. The lead's disclosed fold restores `${spec_schema}` in `spec.slate-home`, in the backticked form `spec.deliverable` uses. That clears the non-blocking round-1 note. The pre-wave literal in `spec.artifact-home` is outside this wave, and leaving it is correct.

VERDICT: PASS

Issues requiring fix: none.

## Unit 9 — crate fixtures, contract expectations, specify kit

VALIDATE: test and kit content, P3's work, round 1. There are 13 files, each diffed against HEAD:
- crate tests and fixtures: `crates/mochiko-cli/tests/fidelity.rs` · `validate.rs` · `matrix_similar.rs` · `tests/fixtures/template/spec.producer.txt` · `spec.check.txt`
- contract suite: `evals/contract/run.py` (`EXPECTED["specify"]`) · `evals/contract/README.md`
- plan kit: `evals/plan/README.md` · `evals/plan/specify/observable.yaml` · `evals.json` · `preregistration.md`
- review kit: `evals/review-specifications/rules.json` · `rekey.md`

Checklist run: `validation-primitive-edit.judgment-items-prose`, coherence and preserved responsibilities, applied as the unit-kind brief binds. The crate test edits also took the independent non-author code review that `.claude/rules/mochiko/rust-cli.md` requires, on four limbs:
- every figure equals the tool's output;
- every comment narrates the move truthfully;
- no crate source and no `Cargo.toml` is touched;
- fixtures are recaptured by command, never hand-edited.

The scope was held against the wave plan §3.9.

Evidence read:
- all 13 diffs, read whole; `evals.json` through a structural JSON diff;
- the fixture consumer in `crates/mochiko-cli/tests/render.rs`;
- `evals/plan/README.md` in full, through its kit invariants;
- the specify fixtures' `CLAUDE.md` governance lines, and s1's no-client-accounts decision;
- `evals/plan/brainstorm/observable.yaml`, the precedent for the precedence class;
- `scripts/similar-rules-allowlist.yaml`, row 394;
- the wave plan §3.9.

Pre-pass, all run first-hand from the repo root:

```
$ cargo test --all
20 test binaries · passed 765 · failed 0 · ignored 0
$ MOCHIKO_FULL_SIMILAR=1 cargo test -p mochiko-cli --test matrix_similar
test the_detector_reproduces_its_figures_over_the_command_family ... ok
test the_detector_reproduces_the_live_runs_figures_over_the_corpus ... ok
test result: ok. 48 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 220.37s
$ cargo fmt --all --check                      exit 0
$ cargo clippy --all-targets -- -D warnings    Finished `dev` profile, no warning
$ cargo audit --deny warnings                  Scanning Cargo.lock for vulnerabilities (31 crate dependencies) · exit 0
$ mochiko-cli migrate status --plugin-root plugins/mochiko
log plugins/mochiko/migrations · grammar 2 · sequences 1..57 (55 migrations)
state sha256:603bb29408410b72f5b16e15b82e89248352db4d1efd185099a7cc1eb5294732 · 87 documents · 1196 rules
$ mochiko-cli migrate validate --report --plugin-root plugins/mochiko
rules scanned: 1196 · in-kind pairs scored: 200540 · clusters: 0 (none)
allowlist-suppressed edges: 185
mochiko-cli migrate validate · 0 rejecting · 113 advisory
$ uv run evals/run.py command check-rubric specify
rubric OK: 56 observable, 15 out-of-instrument, 71 total
$ uv run evals/run.py command check-fixtures specify
fixture consistency: OK
$ uv run evals/run.py command partition specify --old-ref HEAD
"changed": [spec.filter-rejections-recorded, spec.intent-probe-discipline, spec.lockstep-prototyping] · "removed": [] · "added": the 19 ids 0051 mints
$ cargo build --release -p mochiko-cli && python3 -B evals/contract/run.py --case converted-shape
ok    the pre-registered floor set matches the specify render (18 ids)
contract suite: 1/1 cases passed, 1 ran, 1 measurement(s) recorded and not asserted
FILTERED — 1 of 97 declared cases. Not a gate run.
```

Independent checks beyond the listed commands:
- **Template fixtures:** `mochiko-cli template spec`, in both its producer and `--check` views, diffs clean against the two fixtures once the trailing `schemas:` line is dropped.
- **Command floors:** the six commands' rendered floor pins sum to 120, spread 23 · 8 · 14 · 37 · 20 · 18.
- **Pair arithmetic:** each per-kind in-kind pair delta in `matrix_similar.rs` equals the change in n(n−1)/2. The command family moves 2,276, from 15,713 to 17,989. The corpus moves 8,488, from 192,052 to 200,540.
- **Empty-allowlist sweep:** the sweep ran over scratch trees of the `1..50` and `1..57` logs. Both report 78 clusters, and their membership differs by exactly one pair each way. The pair `authoring-feature-map.complete-disposition` / `spec.filter-rejections-recorded` leaves, and `brainstorm.own-rules-win` / `spec.own-rules-win` arrives.
- **Partition:** parsing `observable.yaml` shows 56 observable and 15 out-of-instrument ids. No HEAD id was dropped from either list, and no id appears twice.
- **Review kit:** `rules.json` holds 32 ids against the view's 33, in view order. The one missing id is the disclosed `sf-direction-checks`. Both touched texts equal the view, there are 8 floors, and both `when` values use the file's own string form.

- **coherence — PASS.** The kit, the contract expectations and the census tests tell one story, and it matches the log.
  - The census tests move by 19 command mints and 2 skill mints, giving 362 → 381, 813 → 815 and 1175 → 1196. Command floors move 119 → 120 and fail nodes 36 → 37, while skill floors hold at 264.
  - `EXPECTED["specify"]` gains exactly the new floor. That makes 18 ids, equal to the render, and the contract README now reads "120 across six".
  - `observable.yaml` partitions the 19 mints 16 · 3. The precedence class follows the brainstorm kit's precedent. The conditional entry for `spec.handed-in-slate` follows from the goldens: none of them hands in story lines or names a brainstorm record.
  - The "eleven" in the unplanted-branches count is now correct. HEAD listed five entries but said "four", and six are added.
  - `preregistration.md` is updated before any grid has run, since its fill log reads `_none yet_`. Its figures follow from the new partition: 3 × 56 = 168 pairs, and 56 rules make four judge chunks of at most 15.
  - The `evals.json` changes are rationale only, since the kit's README says the runner reads nothing in that file. They describe the slate stage consistently with the rules.
  - In the review kit, `rules.json` and `rekey.md` agree with the view.
  - §3.9 named three file groups: crate census, contract and plan kit. The review-kit re-key and the `evals/plan/README.md` figure fall outside it. The re-key follows from 0053, and the README change is the lead-ruled widening. Both are coherent with the rest.
- **preserved responsibilities — PASS.**
  - No assertion leaves any crate test: the skill-floor assert and the full `sequences()` list are kept, the latter extended by 51–57.
  - No `EXPECTED` entry is removed, and `expected-skills.json` is untouched. That is correct, since no skill floor moved and the `review-specifications` floor set still matches at 8.
  - No partition entry is dropped. The `spec.filter-rejections-recorded` why is re-pointed to the rule's new post-authoring scope.
  - The one `evals.json` assertion that was reworded, on the PDF-cents bait, keeps its "never homed on the map" core. Every other assertion is kept, and one is added per golden.
  - In `rules.json`, every other entry is byte-identical, as the `git diff` shows.
- **figures equal the tool's output — PASS.** Every pinned figure passes against the live log in `cargo test` and the full similarity sweep. Every figure narrated in a comment was re-derived independently above, and each one matched.
- **comments narrate the move truthfully — PASS.**
  - The fidelity sequence list and its message name each of `0051`–`0057`, and say that `0056` replaces the `spec` template in place.
  - The census comments state the mint split correctly: 19 on `specify`, of which 18 are musts and one is the floor fail node, plus one on each of the two skills. They also give the twelve rewords as eight plus four.
  - The similarity comments' "one in, one out" claim is confirmed by the empty-allowlist sweep. The "cross-family, so no command-family hit leaves" claim agrees with the command pin moving 57 → 58.
- **no crate source or `Cargo.toml` touched — PASS.** `git diff --stat -- crates/mochiko-cli/src crates/mochiko-cli/Cargo.toml Cargo.toml Cargo.lock` is empty. The release build was already current, with no recompile needed.
- **fixtures recaptured by command — PASS.** Both template fixtures equal a fresh render byte for byte, in the comparison `every_template_*_view_is_byte_identical_to_its_captured_fixture` runs and in the grader's own diff.

VERDICT: PASS

Issues requiring fix: none.

Notes for unit 9, none blocking:
- The why on `spec.floor-obligated-rows` grounds "planted on s2 only" in s1's no-accounts ruling. That rules out an auth row but not a failure-recovery row, and s1 also ratifies a production floor at depth high. Because that why text reaches no judge, the partition is unaffected.
- Allowlist row 394, `spec.filter-rejections-recorded` / `authoring-feature-map.complete-disposition`, now suppresses no edge, as the comment says. It stays legal while both ids resolve.
- The disclosed pre-existing defects remain unrepaired and are not this unit's: the stray comma in the `spec.filter-disagreement-escalates` why, which still parses as an extra key; the absent `sf-direction-checks` in the review kit; and that kit's stale "30 rules".
- The converted-shape case ran FILTERED and is not gate 6.

## Shared pre-pass for units 4–8

These commands were run first-hand from the repo root. The tree had not moved since unit 9 except for P2's 14 paths.

```
$ mochiko-cli migrate validate --report --plugin-root plugins/mochiko
rules scanned: 1196 · in-kind pairs scored: 200540 · clusters: 0 (none)
allowlist-suppressed edges: 185
mochiko-cli migrate validate · 0 rejecting · 113 advisory
$ mochiko-cli migrate status --plugin-root plugins/mochiko
log plugins/mochiko/migrations · grammar 2 · sequences 1..57 (55 migrations)
state sha256:603bb29408410b72f5b16e15b82e89248352db4d1efd185099a7cc1eb5294732 · 87 documents · 1196 rules
$ git diff -U0 -- plugins/mochiko/skills/*/SKILL.md | grep -c '^[-+]description:'
0
```

The char-budget figures below come from the ledger's canonical snippet. The render counts all seven blocks, the preamble plus six sections. A stepped replay at the 0050 prefix reproduces every [v0.118.0] ledger figure exactly.

| primitive | body | render | payload | budget | description |
|---|---|---|---|---|---|
| specify (command, unbudgeted) | 4,388 → 4,667 | 20,118 → 27,999 | 32,666 | — | 138 |
| authoring-user-stories | 4,531 → 4,662 | 9,197 → 9,914 | 14,576 | 13,444 | 425 |
| review-specifications | 3,441 → 3,441 | 13,917 → 14,664 | 18,105 | 16,174 | 490 |
| authoring-requirements | 3,535 → 3,604 | 9,339 → 9,413 | 13,017 | 12,373 | 379 |
| authoring-feature-map | 5,954 → 6,027 | 16,751 → 16,829 | 22,856 | 22,323 | 598 |

Every figure equals the editor's named overage and the lead's measurement. Every description is byte-untouched and under the 1,536-character delivery cap.

## Unit 4 — the specify command pair

VALIDATE: command pair — `plugins/mochiko/commands/specify.md` with the `specify` render (preamble plus six sections). The strip entry is the [v0.119.0] entry in `.mochiko/strips/specify.md`.

Checklist run: `validation-primitive-edit.judgment-items-pair`, held against the canonical-scaffold criteria in `.claude/rules/mochiko/primitive-edits.md`, on the run-command branch.

Evidence read: the full `specify.md` and its word diff against HEAD · the [v0.119.0] strip entry · the rendered preamble (pins `kind: fail · 10 rules`, `class: floor · 18 rules`; six sections) · the reserved and tools sections in the current view, from the unit 1 reads · `.mochiko/strips/README.md`, for the entry grammar.

Pre-pass: the shared pre-pass above. Mechanical items are read from it: section set, fail segment, ID continuity, ontology grammar and pointer resolution, all at 0 rejecting. The FILTERED converted-shape case from unit 9 matched the `.md`'s seven `!` lines and its 18-id floor set against this render.

- scaffold headings and order — PASS. The frontmatter key set is intact. The headings, in order, are `# Specify — Feature Specification`, `## Identity & Mission`, `## Rules — delivered by mochiko-cli` with its seven `!` lines, and `## Adaptive Goal Protocol`, whose steps run Entry, then Goal, then Not done last. The word diff shows insertions only, so the re-wrap changed no wording.
- preserved responsibilities — PASS. The strip entry records the stories clause verbatim against `ea57167` as a supersession by ruling, citing D1, D5 as changed, D6 and the default *Slate home and binding*. The per-story contents list stays, as do every other done clause, Entry and Not-done. The Identity & Mission clause is a pure addition.
- floor survival — PASS. No floor or fail leaves. The Not-done line cites the printed `kind: fail` pin rather than a count, so the tenth fail needs no `.md` edit. Its halt-on-disagreement clause is intact.
- independence, no seat grading its own row — PASS. The edit touches no seat wiring. The stress-test seat and `spec.author-grader-default-fail` are unchanged, and the new blind-list seat is fresh by its own rule.
- reserved-to-user content in the reserved section — PASS. The two new user decisions sit in `spec.sec.reserved`: `spec.slate-rulings-users`, a reservation, and `spec.gate-slate-confirm`, a gate. The `.md` names the slate as "confirmed", never as the run's.
- matching done-condition branch — PASS. This is a run command, so the contract is fixed. The Goal now fixes the confirmed slate in the User Stories index (every row `in` or `out` with its disposition, and the `out` rows under Struck candidates) and story files "for `in` rows". Entry's empty-arguments route and the fixed, count-pinned Not-done are unchanged, and no negotiated goal was introduced.
- argued overage — PASS, nothing owed. Commands are unbudgeted. The render growth from 20,118 to 27,999 is unit 1's 19 mints, and the `.md` grows from 4,651 to 4,930 file characters.

VERDICT: PASS

Issues requiring fix: none.

## Unit 5 — the authoring-user-stories skill pair

VALIDATE: skill pair — `plugins/mochiko/skills/authoring-user-stories/SKILL.md` with its render. The strip entry is the [v0.119.0] entry in `.mochiko/strips/authoring-user-stories.md`.

Checklist run: `validation-primitive-edit.judgment-items-pair`, held against the skill-pair criteria, plus the two plan-grader advisories the brief cites.

Evidence read: the full `SKILL.md` and its diff · the strip entry and the [v0.107.0] entry it cites · `scripts/validate-user-stories.py` lines 110–140, the Independent Test parser · the `authoring-user-stories` view rules `story-structure`, `independent-test-required`, `slate-fidelity` and `scenario-count-bound`.

Pre-pass: the shared pre-pass above. The payload is 14,576 against a budget of 13,444, an overage of 1,132. Its standing part is 284, ruled HOLDS at [v0.118.0]. This wave adds 848: render +717 from 0052 and body +131.

- scaffold headings and order — PASS. The load-first section, its seven `!` lines and the floor read-back sentence are unchanged. The body sections, User Story Format, then Priorities and Scenarios, then Common Rationalizations, are unchanged in order.
- preserved responsibilities — PASS. Both superseded strings are recorded verbatim with their ruling. The density obligation and the other block fields stay. The body never had "2–5" prose, which I confirmed by grep. The [v0.107.0] entry's kept clause, "that the story lands in `spec.md`", is retired by this ruling, and its `spec.md` half survives as the index row.
- floor survival — PASS. No floor moves, and the `pm-frame-boundary` floor is untouched.
- independence — PASS. No grading role is touched.
- reserved-to-user content — PASS. "A story the slate lacks is proposed to the user" lives in `slate-fidelity`, and the body does not compete with it.
- matching done-condition branch — n/a. A skill carries none.
- argued overage — PASS: the argument holds.
  - The +717 is a genuine new obligation of the ruling, not restored playbook prose. It covers one story per `in` row, the seed expansion, no story outside the slate, and the provisional test (D2 and D5, 0052).
  - The +131 keeps the format block, which `story-structure` names as "the exact structure", true to the rule.
- Plan-grader advisory 1, a placeholder restating `slate-fidelity` — not a defect. The placeholder names the field's source, "the row's seed expanded", and its mark's form. It does not restate the meaning-fidelity obligation or the Open Questions half, which stay in the rules. The format block is the home those rules delegate field shape to.
- Plan-grader advisory 2, the "Provisional:" form being body-only — not a defect. It is the concrete form of the rule's "marked provisional" and contradicts nothing. The editor's mechanism is correct. `check_independent_test` matches `\*\*Independent Test\*\*:\s*(.+?)` with a 20-character floor, so a mark on the label would break the parse, and the mark inside the field text keeps it.

VERDICT: PASS

Issues requiring fix: none.

## Unit 6 — the review-specifications skill pair

VALIDATE: skill pair — `plugins/mochiko/skills/review-specifications/SKILL.md`, whose body is unchanged (`git diff --stat` is empty), with its render. The render grew by 0053, and the change is recorded in the budget ledger row. No strip entry is owed, since no body text left.

Checklist run: `validation-primitive-edit.judgment-items-pair`, held against the skill-pair criteria.

Evidence read: the full body, including its Procedure section · the `review-specifications` view's verdict section, read in unit 2 · the ledger row.

Pre-pass: the shared pre-pass above. The payload is 18,105 against a budget of 16,174, an overage of 1,931. Its standing part is 1,184, ruled HOLDS at [v0.118.0]. This wave adds 747, all render, from 0053.

- scaffold headings and order — PASS, unchanged.
- preserved responsibilities — PASS. Nothing left the body or the render. `complete-coverage` gained "every slate row" and kept its trio.
- floor survival — PASS. The 8 floors are unchanged, and the converted-shape case in unit 9 matched 8 ids.
- independence — PASS, unchanged.
- reserved-to-user content — PASS. The reserved section is unchanged.
- matching done-condition branch — n/a.
- argued overage — PASS: the argument holds. The four blocking slate checks and the slate rows in the coverage duty are a ruled new obligation (*Slate home and binding* as repaired, N2/N3). There is no body growth.
- No body line became false. The editor's finding stands, checked against the full Procedure text.

VERDICT: PASS

Issues requiring fix: none.

## Unit 7 — the authoring-requirements and authoring-feature-map skill pairs, plus validate-requirements.py

VALIDATE: two skill pairs and one script.
- `plugins/mochiko/skills/authoring-requirements/SKILL.md` with its render, and `scripts/validate-requirements.py`. Strips: two [v0.119.0] entries in `.mochiko/strips/authoring-requirements.md`.
- `plugins/mochiko/skills/authoring-feature-map/SKILL.md` with its render. Strip: the [v0.119.0] entry in `.mochiko/strips/authoring-feature-map.md`.

Checklist run: `validation-primitive-edit.judgment-items-pair` for each pair. The script is graded as code on three points the brief binds: it does what its strip entry says, the checks still run on the requirement's own words, and nothing else moved.

Evidence read: both `SKILL.md` diffs · the full body of `authoring-requirements` · the `authoring-feature-map` intro and rationalizations table · the script diff and its full structure (`find_requirements`, `check_rfc_keywords`, `check_tech_agnostic`, `check_outcome_focus`) · all three strip entries · the views of `fr-format` and `four-touchpoints`, read in unit 2.

Pre-pass: the shared pre-pass above, plus the script runs. The HEAD script came from `git show HEAD:…` into a scratch file, and both versions ran under `python3 -I`.

```
fixture 1 (brief, verbatim)  HEAD: exit 1 · tech_agnostic ['queue', 'rust', 'queue']   now: exit 0
fixture 2 (brief, verbatim)  HEAD: exit 0                                               now: exit 1 · rfc_keywords "Missing RFC 2119 keyword (MUST, SHOULD, MAY, etc.)"
identity: all 47 tracked evals/**/spec.md — stdout and exit code byte-identical HEAD vs now (0 diffs)
the 32 among them carrying FR lines: exit 0 × 19, exit 1 × 13 — the strip entry's figures
output JSON `requirements` entries carry id and line only (no text) · no file-mode change
```

The payloads are as follows:

| skill | payload vs budget | overage | standing part | this wave |
|---|---|---|---|---|
| authoring-requirements | 13,017 vs 12,373 | +644 | +501 | +143: render +74 from 0054, body +69 |
| authoring-feature-map | 22,856 vs 22,323 | +533 | +382 | +151: render +78 from 0055, body +73 |

- scaffold headings and order — PASS for both. The load-first sections are unchanged. The `authoring-requirements` format headings are unchanged, and the `authoring-feature-map` change sits inside the intro paragraph.
- preserved responsibilities — PASS.
  - The FR block keeps its joined IDs, RFC keywords and placeholders, and each line gains a trailing citation, one of them citing two stories. The SC block is untouched.
  - The `authoring-feature-map` parenthetical keeps "frame at intent", "stories authored inside the frame" and "selection after". It adds the slate-close vet and narrows the filter to "homing and dedup". The rationalizations table's "the rejection is recorded with its why" still holds.
  - Every superseded string is recorded verbatim, with its ruling, against `ea57167`.
- floor survival — PASS. No floor moves in either skill.
- independence and reserved — PASS, unchanged.
- matching done-condition branch — n/a.
- argued overage — PASS for both: each argument holds.
  - `authoring-requirements` +143: the FR story citation is the only carrier of slate check (3) (N3), a ruled new obligation. Its body examples mirror the reworded `fr-format`.
  - `authoring-feature-map` +151: the slate-close vet is the default *Filter*, ruled on its own turn. The body change keeps the intro's flow true, since the old "filter and selection after" would now misstate the filter's judgment half.
- script, does what the strip says — PASS. `STORY_CITATION` sits after `RFC_KEYWORDS`. `find_requirements` stores the text with only a trailing citation removed, in the bare, joined, multi-story and trailing-period forms, so all three text checks scan the requirement's own words. Fixture 1's false flags clear, and fixture 2's masked missing keyword now reports.
- script, nothing else moved — PASS. The diff is two hunks: the constant, and the one assignment with its comment. The capture pattern, `BANNED_TERMS`, `RFC_KEYWORDS`, the output shape and the file mode are unchanged. The strip entry's "above" is accurate for its placement in the file, as the editor disclosed.

VERDICT: PASS

Issues requiring fix: none.

## Unit 8 — strips, ripple lines, the budget ledger, the changelog

VALIDATE: prose — the five [v0.119.0] strip entries, the ripple lines, the four budget-ledger rows, and the `CHANGELOG.md` `## [0.119.0] — 2026-10-10` entry.
- strip entries: `specify`, `authoring-user-stories`, `authoring-feature-map`, and two on `authoring-requirements`
- ripple lines: `README.md` at lines 99 and 132 · `ARCHITECTURE.md`'s specify section and its user–lead edge · the router's `/mochiko:specify` row
- budget-ledger rows in `.mochiko/memory/primitive-cost-budgets.md`: `authoring-user-stories`, `review-specifications`, `authoring-requirements`, `authoring-feature-map`

Checklist run: `validation-primitive-edit.judgment-items-prose`, coherence and preserved responsibilities, against the grammar in `.mochiko/strips/README.md` and `.claude/rules/mochiko/primitive-edits.md`.

Evidence read: all four strip-file diffs · the README, ARCHITECTURE and router diffs · `.claude/rules/mochiko/operating-docs.md` and the architecture store's `spine.md` · the four ledger row diffs in full · the `CHANGELOG.md` diff, read whole · a grep of the router, README and ARCHITECTURE for story-flow claims the wave could have made stale.

Pre-pass: the shared pre-pass above. Each ledger figure was re-derived by my own stepped replays and body counts, and each matches. The figures cover every render delta, every body delta, every payload, every overage, the state hash `603bb294…`, and the "1..50 prefix replay under binary 0.4.0 reproduces it exactly" claim. The CHANGELOG's `specify.md` figures, 4,651 to 4,930, are whole-file characters and match my count.

- coherence — PASS.
  - **Strip entries:** all five use the supersession grammar (Disposition, Tier failed, Content, Kept deliberately, Consumers assessed), stamped [v0.119.0] and placed newest-first. Their consumer claims check out: the `validate-user-stories.py` line ranges, and the script's 32-spec oracle with its 19 × 0 and 13 × 1 exit codes.
  - **Ripple lines:** they describe the stage as the rules do: lead-held, line plus priority plus seed test, a blind list, a once-only PM vet, a slate confirmed on its own screen, and authoring for `in` rows only.
  - **ARCHITECTURE.md edit:** it is legal. `operating-docs.md` makes the file derived only once the store carries ruled content, and `.mochiko/product/architecture/spine.md` is a scaffold stub, so the file stays hand-maintained legacy.
  - **Router row:** the change is insertion only.
  - **Ledger rows:** each prepends the [v0.119.0] figure and keeps the prior text after "Plus the standing … below. Prior:", the form the earlier rows use.
  - **CHANGELOG:** its claims agree with the log, the pins, the validate output, the strips and the budgets. The three `[lead: …]` placeholders are the lead's to fill at close.
- preserved responsibilities — PASS.
  - No prior ledger text is lost. Every [v0.118.0] and earlier figure survives verbatim behind "Prior:".
  - No README, ARCHITECTURE or router claim was dropped. Each edit adds the stage and keeps the frame, filter, selection and acceptance text.
  - No earlier CHANGELOG entry is touched.
  - The word "argued" in the four ledger rows is the lead's to replace with the ruling word. This audit rules all four overages HOLDS, in units 5, 6 and 7.

VERDICT: PASS

Issues requiring fix: none.

Notes for units 4–8, none blocking:
- The `review-specifications` Procedure says "widen to the spec's other layers" and names the feature layer and Screens & Flows, but not the slate. The delivered `slate-checks` duty carries the slate, the body says nothing false, and the lead ruled no addition.
- The router's `product-manager` row and the ARCHITECTURE mermaid label "frame brief · derivation brief" do not name the slate-close vet. Both are incomplete rather than false.
- These are pre-existing, not this wave: the ledger's description table still lists `authoring-feature-map` at 495, against a measured 598 at HEAD and now, under its budget of 619. The ARCHITECTURE header still says v0.110.0.
- The ARCHITECTURE specify paragraph's re-wrap leaves one short line. This is cosmetic.
