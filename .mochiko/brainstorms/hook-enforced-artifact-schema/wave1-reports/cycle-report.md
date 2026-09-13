---
report: cycle
feature: hook-enforced-artifact-schema
cycle: 1
attempt: 1
wave: 1
seat: staff-engineer
plan: .mochiko/brainstorms/hook-enforced-artifact-schema/wave1-plan.md
verdict: complete
tasks_planned: 9
tasks_delivered: 9
tests_added: 102
tests_total: 453
tests_failed: 0
review_round_1: FAIL — 10 items (1 Critical · 3 Important · 6 Minor); fix list wave1-reports/code-review.md
corrections_from_review_1: 10 of 10 addressed; G8 was routed to the lead and is now settled in
  `record.md` — D4f reads "no size check where bounds live elsewhere; shape checks still run where a
  template binds; location + set always bind", the reading this build took
regrade_1: G1 withdrawn by the reviewer (the first-pass fmt failure raced this seat's `cargo fmt`);
  the step-6 `Edit` change rules OK; the shared fixture branches PASS. G2 and G4 were the blockers
  and are fixed; G6 and G7 were re-scoped by the lead to `wave1-plan.md` text and are now landed
  there (§5 file list, §6 matrix row)
explicit_allow_on_every_non_deny: confirmed
explicit_allow_asserted_by: tests/home.rs the_shared_fixture_log_never_answers_with_empty_stdout_on_a_non_deny
shared_fixture_log: crates/mochiko-cli/tests/fixtures/home-log/
shared_fixture_branches: 4 of 4 present, each able to allow and deny
gate_cargo_test: PASS — 453 passed, 0 failed, 17 suites
gate_cargo_fmt: PASS — re-verified after the fix round
gate_cargo_clippy: PASS — --all-targets -- -D warnings, no warnings
gate_cargo_audit: PASS — 31 crate dependencies, no advisory
gates_measured_at: after review round 1's fix round
dependencies_added: none
plugins_byte_identical: true
plugin_json_bumped: false
grammar_bumped: false
grammar_range: 1..1
exit_code_minted: 4
worker_dispatches: 2
committed: false
exit_condition_owed: the crate published as mochiko-cli-v* — the user's act, not performed
---

## Files

| file | state | lines | tests |
|---|---|---|---|
| `crates/mochiko-cli/src/home.rs` | new — model, segment tokens, `Homes`, resolution | 434 | — |
| `crates/mochiko-cli/src/conform.rs` | new — the six checks, faults, amnesty, edit leg, sniff | 637 | — |
| `crates/mochiko-cli/src/hook.rs` | new — payload, decision JSON, shell operator scan | 408 | — |
| `crates/mochiko-cli/src/schema.rs` | modified — `Section::{heading, max_lines}`, `Template::conformance`, producer-view block | +172 −1 | — |
| `crates/mochiko-cli/src/validate.rs` | modified — five `home-*` codes and their checks | +161 −0 | — |
| `crates/mochiko-cli/src/render.rs` | modified — `home_view` | +121 −0 | — |
| `crates/mochiko-cli/src/cli.rs` | modified — `check`, `home`, `dispatch_io`, exit 4 | +113 −6 | — |
| `crates/mochiko-cli/src/model.rs` | modified — `DocKind::Home`, `ALL` 8→9, `is_replaceable` | +19 −6 | — |
| `crates/mochiko-cli/src/lib.rs` | modified — three `pub mod` | +6 −0 | — |
| `crates/mochiko-cli/src/views.rs` | modified — `homes` shelf arm | +1 −0 | — |
| `crates/mochiko-cli/tests/home.rs` | new — kind, tokens, resolution, hard set, conformance block, authoring-rule golden | 1084 | 31 |
| `crates/mochiko-cli/tests/conform.rs` | new — the six checks, amnesty, edit leg, reports, sniff | 586 | 35 |
| `crates/mochiko-cli/tests/hook.rs` | new — payload, explicit allow, shell table, JSON escape | 387 | 17 |
| `crates/mochiko-cli/tests/cli.rs` | modified — exit-code matrix, `home` render goldens | +337 −0 | 11 |
| `crates/mochiko-cli/tests/validate.rs` | modified — `KINDS_NOT_SHIPPED_YET` | +22 −0 | 1 amended |
| `crates/mochiko-cli/tests/fixtures/home-log/0001-homes.yaml` | new — the shared fixture log, read by this crate and by the plugin contract suite | 172 | 4 |

## The shared fixture log

`crates/mochiko-cli/tests/fixtures/home-log/` is the single place either side declares an artifact
home: the plugin contract suite reads it rather than carrying a fixture of its own (wave-4 contract
plan). It replays clean — `migrate validate` reports 0 rejecting, 0 advisory — and carries five
homes and two templates.

| branch | declared as | allow case | deny case |
|---|---|---|---|
| 1 — templated deliverable with per-section `max_lines` | `feature/spec.md` → `template: demo-spec`, headings `Intent` 8 lines and `Requirements` 20, plus an optional `Open Questions`, required + enumerated frontmatter, three placeholder tokens | a conforming spec | one section over its budget |
| 2 — template-less deliverable, whole-file bound (D4f) | `feature/gates.md` → `max_lines: 40` | 3 lines | 60 lines |
| 3 — `reports/` dir with an envelope enum (D2) | `feature` → `reports.envelope: report-envelope`, six `report:` types | `report: cycle` under any file name | `report: invented` |
| 4 — bounds elsewhere, cited (D4f/V2) | `memory` → `bounds: elsewhere`, `bounds_cite: .mochiko/memory/knowledge-management.md` | a 5,000-line declared file, since size is not checked at all | an undeclared name, since the file set still binds |

Two further shapes ride along so the resolver's own rules have something to bind to: an append-only
log bounded per `##` entry (`feature/implement-log.md`, 30 lines), and a nested `stories` home plus a
literal `desk` home so the longest-literal-prefix rule has a race to win.

**A dropped branch fails the build rather than stranding a contract row.**
`the_shared_fixture_log_covers_every_branch_the_contract_suite_reads` asserts each of the four is
present by its distinguishing property, not by name, and a sibling test proves each can both allow
and deny. The contract suite is downstream and cannot defend itself, so the defence lives here.

## Decomposition

| task | red | green | refactor |
|---|---|---|---|
| 1 `DocKind::Home` | 4 tests, compile failure | model/views arms | — |
| 2 `home.rs` model, tokens, resolution | 12 tests | pattern matcher, `Homes` | leaked-static cache replaced by an owning `Homes` type |
| 3 hard set over homes | 9 tests | 5 codes, `validate_homes` | — |
| 4 conformance block, producer view | 5 tests | `schema.rs` keys, `conformance_block` | two `impl Template` blocks merged |
| 5 six checks | 35 tests | `conform.rs` faults | — |
| 6 amnesty, edit-in-memory | in the 35 above | `settle`, `Fault::not_worsened_from` | — |
| 7 `hook.rs` | 17 tests | payload, shell table, JSON escape | — |
| 8 CLI surface, exit codes | 11 tests | `check`, `home`, `dispatch_io` | stdin made injectable |
| 9 goldens, cost | 1 golden | authoring-rule render golden | — |

## Cost — restated after review round 1 (G3)

**The cost is replay, and it scales with log size.** My first reading — "the 36 ms is process start,
not replay" — was wrong in both limbs, and the conclusion drawn from it pointed the wrong way. The
reviewer's correction reproduces on my own re-measure. Release binary, 40 invocations per repetition,
median of 3 repetitions, same box:

| command | log | log size | ms/call |
|---|---|---|---|
| `/usr/bin/true` | — | — | 2 |
| `mochiko-cli --version` — process start and nothing else | none read | — | 3 |
| `check --hook-json -` | the shared fixture log | 152 lines | 3 |
| `check --hook-json -` | the real log | 12,063 lines · 640 KB | 37 |
| `rules --section` (the referent) | the real log | 12,063 lines · 640 KB | 38 |

CPU for one real-log check, `/usr/bin/time -p`: real 0.03, **user 0.03**. So roughly 34 of the 37 ms
is log-size-dependent replay, not the 1.2 ms of process start I first reported — that figure came
from timing a subshell loop, which did not attribute child CPU.

**What this means for wave 3, which is the part that matters.** `check` still costs what `rules`
costs, and the 100 ms trigger is still unmet, so **no cache is built this wave** — that conclusion
survives. But it survives for a different reason than I gave: the figure tracks the log, and wave 3's
census migration declares homes for a 288-file tree. The re-measure after that migration is expected
to move toward 100 ms, so the `${CLAUDE_PLUGIN_DATA}` cache seam becomes **likely rather than
hypothetical**. The seam is one `load_for_delivery` call site in `cli.rs`, keyed on the state's
existing content hash.

## Worker dispatches

| task | rung | brief | read-back |
|---|---|---|---|
| 3 — hard set over homes | sonnet | the 8 failing tests by name and code, the decided check list, `validate.rs` only | Read the diff. Code matches the brief. Its reported blocker was real and correct: the shared `HOMES` fixture declared bindings to templates it never imported, so `replay::load` refused the state once bindings were checked. Fixture fixed by the seat; 25 tests green. |
| 8 — exit-code matrix and `home` goldens | sonnet | the five-row table, the property test, the four render cases, `tests/cli.rs` only | Read the added tests, re-ran all four gates. 51 in `cli`, 445 total. Its flagged side effect verified: whole-crate `cargo fmt` reformatted one `format!` in `home_view`, whitespace only, end line intact. |

Seven of nine tasks stayed with the seat against a plan that had five going down. Reason: the
ladder's own test. Tasks 5 and 6 share the `Fault` type — the fault key *is* the amnesty
mechanism — so splitting them across a brief would have cost more to specify than to write.
Tasks 1, 4, 7 and 9 are each under an hour of design-bearing code. The two dispatches that did go
down were the two genuinely separable blocks.

## Corrections from review round 1

| id | sev | old | new |
|---|---|---|---|
| G1 | C | **withdrawn at re-grade** — `cargo fmt --all --check` red at `tests/home.rs:1154` and `:1189`; the report read `gate_cargo_fmt: PASS` from an earlier run | fmt clean on my own run. My post-revision `cargo fmt --all` had in fact landed before this fix round, so the tree was already formatted — but the report's gate lines were a stale snapshot, and they now carry the post-fix figures with the measurement point named |
| G2 | I | `placeholder_faults` built its haystack from every line whose trimmed start was `#` — no fence awareness, no level restriction — so a `#` comment inside a fenced example was heading text and **denied a conforming artifact** | one fence-aware `heading_scan` now classifies every line once and both callers consume it, so the fence rule cannot diverge again; scope restricted to `##` and `###` per plan §3.7, and `##text` without a space is not a heading. Three new rows: the reviewer's fenced `# FEAT-XXX` reproduction (allow) with an unfenced `###` control (deny), and a levels row proving `#`, `####` and `##text` are outside scope |
| G3 | I | "CPU is 1.2 ms per call, so the 36 ms is process start, not replay" | wrong in both limbs and corrected above from my own re-measure: process start is 3 ms, the 152-line fixture log is 3 ms, the 12,063-line real log is 37 ms with 30 ms user CPU. Cost is replay, scales with the log, and the cache seam becomes likely at wave 3 rather than hypothetical. Each table row now carries its log size |
| G4a | I | `a_home_whose_bounds_live_elsewhere_takes_no_size_check` graded a 3-line body in a `bounds: template` home against a 6-line bound — it would have passed with the exemption deleted | the conform fixture gains a `bounds: elsewhere` home binding the same template whose `## Intent` budget is 4; the row now grades a 25-line body, **six times over**, plus two control legs: the identical body denies at a `bounds: template` home, and an undeclared name under the exempt home still denies |
| G4b | I | `a_placeholder_token_in_heading_text_is_denied` mutated `## Notes` into an undeclared heading, so the heading fault fired first and the row passed on the extra-heading rule | the token goes into a `###` sub-heading — inside D4c's scope but not a `##` span — so no heading rule can fire, and the assert now requires the reason to name ``placeholder token `[entity]` `` rather than only that a deny happened |
| G5 | m | `>\|` was a one-character evasion: stripping `>` left `\|file`, which resolved to nothing and allowed the write | the redirect arm strips a leading `\|` in both the glued and spaced forms. Eight new shell rows: `>\|` spaced and glued, `dd of=`, `install`, `tee -a`, `2>`, `&>` |
| G6 | m | (re-scoped by the lead to the plan text, now edited there) an `Edit` whose `old_string` is absent returns an explicit allow, where the approved plan said exit 2 in both the algorithm and the matrix — an undeclared contract change | disclosed here as a **sixth deviation**, and the plan's §3 step 6 and its §6 matrix row are both folded to the built behaviour, so the wave-4 contract suite inherits exit 0 with an explicit allow. Reason: an unmatched `old_string` is not a usage error in D3's sense, the platform rejects the call itself, and a second denial from the gate would only confuse the seat |
| G7 | m | (re-scoped by the lead to the plan text, now edited there) the plan named `tests/matrix_home.rs` and extensions to `tests/render.rs` and `tests/views.rs`; none exists | disclosed: the home matrix landed in `tests/home.rs`, the `home` render goldens in `tests/cli.rs`, and `render.rs`/`views.rs` carry no test changes. Three files became two, and the §5 file table above is the accurate list |
| G8 | m | **settled by the lead in `record.md`, the build's reading upheld** — record D4f/V2 ("location + set only") and D6 ("no size bound here at all") disagree on what survives `bounds: elsewhere`; the build follows the D6 reading, so frontmatter, headings and placeholders still run | **routed to the lead, not fixed here**: `record.md` is the lead's pen. The build and the approved plan both take the D6 reading, and the wave-3 plan's `memory` home depends on it. Flagged because wave 3 binds the root operating docs to this branch, and the stricter reading would gate their shape |
| G9 | m | `let _ = home;` and `let _ = name;` — two parameters existing only to be discarded, both on the report path | both are used: every report fault's message is now prefixed with the home and the file name, so a denied seat is not left hunting for which report under which run tripped. One new row asserts the reason names both |
| G10 | m | a report was graded on the envelope's frontmatter only; the envelope's own declared headings, placeholder tokens and section budgets were never applied, and the fixture's `sections: []` meant no test could see it | the envelope is graded in full — frontmatter, headings, placeholders and section budgets — with a `by_type` template adding its own budgets on top, which is D6's reading. The fixture envelope gains a required `## Findings` at budget 4 and a placeholder token, and one new row proves each of the four now binds |

Two older report rows had to be repaired as a consequence of G10: their bodies lacked the envelope's
newly required heading, so they would have denied for a second reason. Both now carry a conforming
payload, so each tests the rule it names — the same discipline G4 asks for.

## Notes of note

**The grammar decision held, and wave 3 inherits a constraint.** No bump: the log README reserves
one for a change that alters an existing file's meaning, no `mochiko-cli-v*` tag exists, and
`migration.rs` already halts loudly on an unknown `kind:`. Consequence for wave 3, also carried at
the top of `wave1-template-readiness.md`: `schema::Section` ignores unknown keys by design, so a
post-publish migration adding a conformance key **alone** would degrade silently on an older
binary. Wave 3 is safe only because its conformance keys ride the same file as the `home`
`import-document` ops. Split them and that migration takes a bump.

**One test was weakened by ruling, and made self-deleting.**
`the_shipped_corpus_covers_every_document_kind_the_store_holds` broke the moment `Home` joined
`DocKind::ALL`, because wave 1 ships no home document. Rather than skip the kind, the test now
asserts the uncovered set is *exactly* `KINDS_NOT_SHIPPED_YET` — so the day wave 3 lands the first
home, the test fails and the constant must shrink. A plain skip would have rotted silently.

**A design element failed the plan-minimalism ladder mid-build and was replaced.** `home.rs` first
resolved paths through a process-lifetime `Box::leak` cache keyed on the state hash, so a
`Resolution` could borrow from `&State`. That is a deliberate leak plus a mutex for one decode per
invocation. Replaced with an owning `Homes` type the caller holds. Rung: minimum.

**Six design deviations beyond the approved plan, each disclosed.** The sixth arrived at review
round 1 (G6): an `Edit` whose `old_string` is absent returns an explicit allow where the plan said
exit 2. The plan is folded to match, because the built behaviour is the better of the two and the
contract suite reads the plan. The fifth is that the plan's three test files became two (G7): the
home matrix is in `tests/home.rs`, the render goldens in `tests/cli.rs`.

**The first four, as originally reported:**
1. A fifth finding code, `home-shape`, for a home document the decoder cannot read. Without it
   `Homes::load` would skip a malformed home silently, which is the record-layer corruption class
   GI-005 forbids.
2. `Resolution` carries six variants, not the three the plan sketched, because a deny reason has to
   say whether the problem is the file name, a sub-directory, or nothing at all.
3. `dispatch_io`, so the exit-code contract is driven in-process like every other CLI test rather
   than by spawning a binary.

**Two implementation traps found and closed by test.** A `##` inside a fenced code block is not a
heading — every template skeleton carries fenced examples, and reading those as headings would deny
conforming files. And a heading's span runs to the next `##` with nested `###` counted in, so
nesting is not an escape from the parent budget; both have their own cases plus an unfenced control
leg, because a fence test alone would pass on a checker that ignored headings entirely.

**Two of my own test bodies were wrong and were corrected, not bent.** The nested-heading and
fenced-block cases initially tripped the size budget rather than the rule under test, so they read
as implementation failures. Both bodies were tightened to stay inside the budget and the fence case
gained an unfenced control. No assertion was loosened to pass.

**The `serde_json` dependency was declined on measured evidence.** A throwaway probe confirmed
`serde_norway` parses real hook payloads: multi-line escaped content, quotes, backslashes,
`\uXXXX`, unknown fields, and `\/` — which is legal JSON that plain YAML rejects. JSON *output* is
hand-written, since the shape is three fixed keys; the escaper is exhaustive over the control range
and has its own case. No dependency added, `cargo audit` surface unchanged at 31 crates.

**The wave-0 folds all landed, and the explicit allow is confirmed as asked.** `check` prints an
explicit `permissionDecision: allow` on **every** non-deny outcome and never empty stdout: a
conforming write, an amnestied one (carrying `additionalContext`), a path under no home, a shell
miss, a sniff miss, a tool outside the matcher set, a write outside the working directory, and an
`Edit` whose `old_string` is absent. Asserted run-wide over the shared fixture by
`the_shared_fixture_log_never_answers_with_empty_stdout_on_a_non_deny`, mirroring the suite's own
run-wide assert at the source, and per-code by
`check_stdout_carries_json_only_when_the_binary_could_read_its_log`. Codes 1, 2 and 3 print nothing
and the wave-4 wrapper supplies their allow, because a binary that cannot read its log must never
deny a write. `Read`'s `tool_input.file_path` shares the payload struct and takes no branch. The
probe line `printf 'retry probe' > "<abs>/probe-home/c.md"` — which the wrapper's `grep`/`sed` reader
false-allowed — is a deny case in the shell matrix. The 100 ms cache trigger is unmet.

**The plan text lagged the build in three places and was corrected, not left to contradict it.**
Steps 3, 4 and 8 of the plan's algorithm said "empty stdout" or "0 silent", superseded by the wave-1
open; step 6 said an `Edit` with an absent `old_string` exits 2, where the build allows it with an
explicit allow, since the platform rejects that edit itself and a second denial would only confuse
the seat. The plan is the review seat's referent, so it now says what was built and marks the
supersession.

**Deliberate holes, unchanged from the record.** `NotebookEdit` and any MCP file writer stay
outside the matcher set. The shell parse is a scan, not a shell parser, and its deny reason says so.
A dangling template binding does not deny a seat's write — it is a log defect `migrate validate`
reports, and denying over it would punish the wrong party.

**One item is the lead's, not mine (G8).** Record D4f/V2 and D6 disagree on what survives
`bounds: elsewhere`: "location + set only" against "no size bound here at all". The build follows
D6 — size is skipped, frontmatter and headings and placeholders still run — because that is what the
approved plan chose. `record.md` is the lead's pen, so I have not edited it. It matters now rather
than later: the wave-3 plan binds the root operating docs and `.mochiko/memory` to this branch, and
the stricter reading would gate their shape. **Settled since:** the lead clarified D4f in the record —
no size check where bounds live elsewhere, shape checks still run where a template binds, location and
file set always bind. That is the reading this build carries, so no code changes.

**Not done, and owed.** The crate is not published; that tag is the user's act and wave 1's stated
exit condition. Nothing is committed. No file under `plugins/mochiko/` changed, verified by empty
`git diff` and `git status` over that path.
