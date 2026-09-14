# Build log — hook-enforced artifact schema (D11 waves 0–5)

Lead: session lead · opened 2026-09-13 on the user's word ("you manage the build, get staff
engineer and QA involved. use me when needed too"). Floor: `mochiko:patterns-sound-loop`
tripped for every wave (judgment-authored writes on governing surfaces); seats produce on
lead-approved plans, non-author seats review, the user rules the reserved calls (wave-0
abort · wave-2 amend run · wave-3 budget table · every `plugin.json` bump · the crate publish).
Transport: message lane fired (lead ↔ seats); topology lane — every seat writes its own files
only; no shared write surface unless disclosed here.

## Wave 0 — probe (D8)

- 2026-09-13 · `qa-wave0` (`mochiko:qa-engineer`) spawned, plan-first. Deliverables:
  `wave0-probe-plan.md` → approval → `wave0-probe-report.md`. Review leg: a non-author seat
  reads the report for evidence sufficiency before the abort/proceed call goes to the user.

## Wave 1 — crate (D3/D4/D6/D10)

- 2026-09-13 · `se-wave1` (`mochiko:staff-engineer`) spawned, plan-only until wave 0 clears.
  Deliverable: `wave1-plan.md` → approval → build on a second explicit open → independent
  non-author code review (rust-cli.md) → `cargo test/fmt/clippy/audit` green → exit condition:
  crate published (user's act).

## Disclosure lines (pinned grammar, one per wave at close)

- 2026-09-13 · wave-0 plan (`wave0-probe-plan.md`, 119 lines, 32 cells, two throwaway plugins,
  four transports, abort rule as D8 + I12) read by the lead and **approved**; lead added leg 8 —
  `SubagentStart` context injection (candidate lever for D1b's per-read repetition / OQ4; changes
  no ruling). Seat-found platform fact carried forward to wave 4: background subagents in
  non-interactive mode deny when no hook returns a decision → the wrapper must return an explicit
  `allow`. Unverifiable on macOS: the `PowerShell` leg's firing (registration validity only).
- 2026-09-13 · wave-1 plan (`wave1-plan.md`, 217 lines — bound of 150 waived by the lead for
  substance) read and **approved**: no grammar bump (range freezes at wave 1's publish; unknown
  `kind:` already halts loudly) · `home/<name>` opaque documents with a seven-token segment
  vocabulary (`regex`/`globset` declined under adopt-first) · `template` sections gain
  `heading:` + `max_lines:`, template-level `conformance:` (frontmatter required/enum ·
  placeholders · `extra_headings`) after reconnaissance showed 5 of 8 templates' `sections[].name`
  are not their heading grammar · `check --hook-json -` eight-step algorithm, exit **4 minted** ·
  D9 sniff pulled into wave 1 (publish is the freeze) · 9 TDD tasks, 5 to sonnet workers. Lead
  answers: readiness table → `wave1-template-readiness.md` for wave 3, with the boxed forward
  constraint (conformance keys ride the same migration as the `home` ops). **Build held** until
  wave 0's reading.
- 2026-09-13 · `wave1-template-readiness.md` (118 lines) delivered for wave 3: 1 of 8 templates
  ready as-is (`spec`), 2 after a heading-less meta-section ruling (`governance-intent`,
  `feature-entry`), `codebase-analysis` + 3 `heading:` overrides, 4 need a wave-3 authoring
  decision (`tasks` · `features-index` · `architecture-store` · `governance-surfaces`, the last
  two multi-file). Four findings for the census: (1) **the shipped `tasks` skeleton is itself a
  drift source** — it places cards under `## Cycle Format` with no `## Cycle Cards`/`## Header`
  heading, so kinako's `FEAT-002/tasks.md` (record F10) followed the skeleton faithfully;
  **F10 annotation candidate, put to the user at the wave-0 checkpoint**; (2) placeholder spellings
  vary (`{{x}}` ×4 · `[x]` ×3 · `<x>` ×2) and `tasks` uses `[P]` as real syntax — D4c's exact-token
  rule must be honored literally; (3) no shipped skeleton carries frontmatter — `conformance.
  frontmatter` binds the report envelope and census-chosen deliverables only; (4) multi-file
  templates cannot bind one file's shape. Boxed forward constraint on the page: conformance keys
  ride the same migration as the `home` ops or take a grammar bump.
- 2026-09-13 · **wave 0 DONE — PROCEED, abort not tripped** (`wave0-probe-report.md`, 280 lines;
  21 headless runs, 153 captured payloads, $2.78; 31/32 cells resolved, the `PowerShell` arm
  unverifiable on macOS). All four transports fire on all three channels; `Read` carries
  `tool_input.file_path`; deny held in `dontAsk`/`acceptEdits`/`bypassPermissions`/`auto` on lead
  and subagent; dead gate proceeds silently (exit 126/127, operator-only stream event). **Leg 8
  PASS:** `SubagentStart` `hookSpecificOutput.additionalContext` lands before the seat's first turn
  (both spawn kinds; proof: `attachment: hook_additional_context` in both sidechains) — candidate
  D1b lever, user's call. Findings for later waves: the shipped `field()` grep/sed parse produced
  one **false allow** (a `printf … > "<abs>/probe-home/c.md"` created the file — settles D3/I4 by
  experiment); Bash is 85–86 % of gated volume in the two measured kinako runs (Write+Edit 9/4
  vs Bash 484/443, 206/211 with a write operator); `if` must sit on the handler object (silently
  ignored on the matcher group); Bash `if` narrows by path substring only; the spawn tool's wire
  name is `Task`; the reminder line was read by one seat as a prompt injection and ignored;
  a denied seat escalates to delegation and the subagent inherits the gate; `NotebookEdit` is a
  live tool. Cost: 47.1 ms median per call (wrapper + binary), projected 10.6–26.6 s per run on
  two non-implement transcripts (no EPIC-002 implement transcript exists on this machine —
  disclosed); proposed budget ≤ 60 s aggregate and ≤ 2 % wall-clock, cache trigger > 100 ms
  median or > 60 s projection. Review leg: `reviewer` (`mochiko:tech-lead`) grading evidence
  sufficiency → `wave0-probe-review.md`. Abort/proceed ruling: the user's.
- 2026-09-13 · `wave0-probe-review.md` (`reviewer`, `mochiko:tech-lead`): **FAIL — 15 fixes**
  (G4 Critical · 5 Important · 9 Minor), **abort reading PASS, safe to rule**. G4: the second
  session's narrowed Bash count (287) does not reproduce — under the report's own predicate
  (`.mochiko` in the command text) it is 154, so the `if` narrowing removes ~64 % of gated calls,
  not 38 %, inverting the report's "not the lever it looked like" line and weakening the case for
  promoting `SubagentStart` over it. G5: write-operator counts (206/211) do not reproduce
  (169–180 under any stated operator set; the qualitative claim survives). G6: no wall-clock
  span in the report; the 60 s cap binds, the 2 % limb never fires on either session. G2: D8's
  Edit limb was not run per transport (one Edit deny in the whole probe) — disclosed hole. G8:
  the `Task` wire-name claim is half-measured (init array says `Task`, every spawn `tool_use`
  says `Agent`) — UNVERIFIED, one-cell wave-1 probe owed. Fix round opened on `qa-wave0`
  (report only); re-grade by `reviewer` after.
- 2026-09-13 · reviewer lead-side note, **binding on waves 1 and 4:** the hooks guide states a
  background subagent's call is DENIED when no hook returns a decision (the probe's plugin
  returned an explicit `allow` for this reason). Consequence: D3's "CLI exit 1/2/3 pass-through
  silent" must be implemented as the wrapper emitting an explicit `permissionDecision: allow`
  (empty stdout would fail closed for every producing seat when the binary is absent or predates
  `check`). Lead repair inside D7's fail-open intent; `check` prints an explicit allow JSON on 0
  with no verdict too. Also: the graded report carries two body sections outside its envelope
  (G-minor, in the fix round).
- 2026-09-13 · wave-0 fix round done (`qa-wave0`, report only; `corrections_from_round_1` block
  in the frontmatter). Root cause of G4/G5: line counts over jq-extracted commands instead of
  per-call counts. Corrected: narrowing predicate stated; Bash mentioning `.mochiko` 200/154;
  narrowed totals 225/188 of 564/520 gated (60 %/64 % reduction) — **the `if` narrowing is the
  primary cost lever**; projections 11.0/9.2 s at 49.0 ms per call; write-operator sets literal
  (core 169/165, extended 180/180); spans 21,218 s / 4,461 s; **budget re-proposed as one 60 s
  aggregate cap** (the 2 % limb dropped — unmeetable at the probe's 41 s median run and never
  binding on the long ones; share kept as a watch metric); leg 8 re-derived: a delivery-shape
  lever, not a cost lever (Read term ≈ 1.0–1.6 s per run). Spend $3.00 over 22 runs (3 runs over
  the plan's bound, named). Re-grade dispatched to `reviewer`, bounded to G1–G15.
- 2026-09-13 · re-grade **PASS, 0/15 not held** (`reviewer` re-verified every corrected figure
  to the call and reproduced the disclosed root cause). Lead waives the envelope note (the
  `## Abort / proceed reading` and `## Budget proposal` body sections are the deliverables the
  brief asked for; the envelope permits extra fields). Evidence-trail note: the reviewer's own
  timing check appended 20 `ENTRY=narrow-timingcheck` lines/files to the probe log and `fires/`
  after the report's counts were taken — left in place, recorded in the re-grade section. One
  explanatory clause in the report (the 165-vs-166 gap blamed on a `cp`/`mv` word boundary) is
  wrong — the reviewer's own `tee` pattern was the cause; the number 165 is right. Not re-opened.
  **Wave 0 closed pending the user's R4 (abort/proceed).**
  `floor: tripped · seats: qa-wave0 (mochiko:qa-engineer) / reviewer (mochiko:tech-lead)`
- 2026-09-13 · **user rulings:** R4 PROCEED · R5 option 1 (`SubagentStart` per-seat reminder
  replaces the `Read`-time form) · R6 yes (F10 annotated — the `tasks` template's own section
  list and skeleton disagree). Transcribed into the record (D1b/D1c/D3/D8/OQ3/OQ4/F10, status
  line amended), the index About, and the DECISIONS row. **Wave 1 OPEN** on `se-wave1` with four
  crate-touching results (explicit allow JSON on every non-deny path · `Read` key · the `printf
  >` false-allow as a matrix row · the 60 s cap). `qa-wave0` re-tasked: plan the wave-4 contract
  cases (`wave4-contract-plan.md`), nothing under `evals/` until wave 1 lands.
- 2026-09-13 · **wave-2 amendment proposal drafted** by the lead (`wave2-amendments.md`) as
  input to the user's `/mochiko:setup` amend run: semver proposed MINOR v3.1.0 (MAJOR reading
  flagged for the user); CLAUDE.md pointer-only touches; ledger GI-019 gains the AM-3 admission +
  clause (iv) + hook floor re-ratified + explicit-allow rule + hooks-disabled scope;
  `rust-cli.md` bright-line bullet rewritten (verbatim prior text recorded). Gate: wave 4 does
  not open before ratification. The run is the user's to invoke after wave 1 lands.
- 2026-09-13 · `wave3-census-raw.md` generated mechanically by the lead (floor-exempt: no
  judgment — path patterns normalised on ids/slugs/dates, per-file lines, frontmatter presence,
  `##` heading lists) over kinako's and mochiko's `.mochiko/` trees + root docs: 346 raw
  patterns, 1,136 lines. Input to the wave-3 census seat, which collapses patterns into homes
  and proposes the table.
- 2026-09-13 · `wave4-contract-plan.md` (`qa-wave0`, 99 lines) **approved**: 3 host cases / 32
  rows (`gate-input` 25 incl. one row per CLI exit code, the wave-0 `printf >` false-allow as a
  must-deny regression, a run-wide never-empty-stdout assert; `reminder-input` 5; `if-placement`
  2 static) + 2 sandbox cases / 3 sessions (`reminder-spawn` named + unnamed; `gate-live` one
  blocked write). Settled: wave 2 lands before any case is written (rust-cli.md currently forbids
  the assertion) · D10's reminder clause re-keyed to R5 (record amended) · wave-1 §3/§4
  contradiction already closed in the wave-1 open · `G-PWSH` as `report()`. Wave 1 owes the
  suite a fixture log with a `home` document covering four branches, a stub binary exiting 2,
  and the frozen reminder line as a golden.
- 2026-09-13 · `wave0-fixtures/` (4 verbatim wave-0 payloads + README) preserved in the session
  dir for the wave-4 rows; the contract plan repointed. Lead ruling on redaction: the narrow
  `transcript_path` home-prefix substitution stands; the username inside other path segments is
  not a secret (GI-003) and the redirect fixture's absolute path is the tested input.
- 2026-09-13 · **wave 1 BUILT** (`se-wave1`; `wave1-reports/cycle-report.md`): 94 tests added,
  445 total / 0 failed across 17 binaries; `cargo test/fmt/clippy/audit` green (31 deps, none
  added); `plugins/mochiko/` byte-identical (lead-verified: `git status --short plugins/mochiko`
  empty); `check` median 36 ms per call = `rules`' 36 ms (process start; CPU 1.2 ms) — cache
  trigger not tripped, seam at `load_for_delivery`. Deviations disclosed: self-deleting
  `KINDS_NOT_SHIPPED_YET` coverage assert (fails the day wave 3 ships a home) · fifth finding
  code `home-shape` (GI-005: an unreadable home must not skip silently) · six `Resolution`
  variants · `dispatch_io` in-process exit-code tests · 7/9 tasks kept on the seat (two sonnet
  workers, one reported a real fixture blocker) · `serde_norway` retained over `serde_json` on a
  measured `\/` probe, hand-written JSON emitter with exhaustive control-range escaping · fenced
  `##` not a heading, unfenced control legs. Not committed; crate not published (user's act).
  **Non-author code review dispatched** to `reviewer` → `wave1-reports/code-review.md`. Crate
  frozen until the verdict (quiesce-before-cold-grade).
- 2026-09-13 · wave-1 addendum (before the freeze took hold): shared fixture log
  `tests/fixtures/home-log/0001-homes.yaml` with the four contract-suite branches (+ log shape,
  + `stories`/`desk` resolution race), each with an allow and a deny case, presence asserted by
  property; 449 tests, gates green. Plan §3 corrected in four places to match the build (step 6:
  `Edit` with absent `old_string` → explicit allow, not exit 2 — a deviation for the reviewer to
  rule). Reviewer told the frozen state is the current tree.
- 2026-09-13 · `wave1-reports/code-review.md` (`reviewer`): **FAIL — 10 fixes** (G1 Critical:
  `cargo fmt --check` fails on the post-report fixture test · G2: `placeholder_faults` is not
  fence-aware — a token in a fenced code block false-denies · G4: two matrix rows pass for the
  wrong rule · G3: the 36 ms is replay not process start — binary start 3.3 ms, fixture log
  3.7 ms, real log ~30 ms CPU — so cost scales with log size and wave 3's migration moves it
  toward the 100 ms cache trigger · 6 Minor). Deviations all ruled OK incl. `serde_norway`;
  bright line intact (one filesystem read: the amnesty baseline); fixture branches PASS; 449
  tests / clippy / audit green on the reviewer's own run. Fix round opened on `se-wave1` (wave-3
  plan paused); re-grade after.
- 2026-09-13 · `wave3-plan.md` (`se-wave1`, 209 lines) read and **approved**: 346 raw patterns →
  21 homes (174 report/review patterns → one `reports/` rule per home; 44 brainstorm working
  files; 42 `.mochiko/benchmarks/**`; 14 root docs; 72 deliverables + odd cases); templates —
  `spec` ready, `governance-intent`/`feature-entry` heading-less meta section, `codebase-analysis`
  3 overrides, `tasks` skeleton FIXED (not ratified), `architecture-store` split into
  spine/concerns templates, `features-index` frontmatter+placeholders+whole-file, `governance-
  surfaces` no block; new `report-envelope` template (the six enum types); budget table keyed to
  the ADR's rule-4 numbers (prose 6 · list ceilings · report sections 15 · whole-file 150 ·
  `baseline-delta` 300 · log entry 60 · five `none` rows under OQ1); 19 authoring-time rule
  mints (`<prefix>.artifact-home`, floor); `impl.reports-envelope` plain `reword-rule`; migration
  `0005-artifact-homes.yaml` (~2,900 lines, one file). New disclosed hole: repo-root writes are
  uncaught (root docs not a home; prose-enforced). Lead answered R4: `reports/` only, the
  physical re-home is wave 5's violator pass on both repos. Build held on the wave-1 fix round +
  the user's R1–R11.
- 2026-09-13 · reviewer re-grade on the frozen tree: **FAIL, 9 open, G1 withdrawn** (gates green
  on the reviewer's own run — the first-pass fmt failure raced the seat's `cargo fmt`); step-6
  `Edit` deviation ruled OK; fixture branches PASS (5 homes, 2 templates, all four branches
  allow+deny). Blocking G2 (fence-unaware placeholder scan) and G4 (two rows prove the wrong
  rule); G6/G7 re-scoped to plan text (matrix line 175 `old_string` absent → exit 2 is stale;
  §5 names a non-existent `tests/matrix_home.rs`). Fix round on `se-wave1` amended accordingly.
- 2026-09-13 · wave-1 fix round done (`se-wave1`): 453 tests / 0 failed, four gates green on the
  seat's run; G2 fixed structurally (one fence-aware line pass feeds both `heading_spans` and
  `placeholder_faults`; scope `##`/`###` only; both directions reproduced through the release
  binary); G4 rows now fail when their rule is deleted (control legs); G3 re-measured — start
  3 ms, 152-line fixture 3 ms, 12,063-line real log 37 ms / 30 ms CPU: replay scales with log
  size, cache seam likely at wave 3; G5 eight more shell rows (`dd of=`, `install`, `tee -a`,
  `2>`, `&>`); G9/G10 report deny reasons name home+file, envelope shape checks bind; G6/G7 plan
  text fixed. **G8 routed to the lead:** D4f ("location + set only") vs D6 ("no size bound") on
  `bounds: elsewhere` — lead repaired D4f as a fix-on-sight integrity clarification: size
  skipped, shape checks run where a template binds; D6 already read so. Re-grade dispatched.
- 2026-09-13 · fix-round re-grade **PASS, 0 not held** (`reviewer`: gates green on own run;
  G2 verified through the binary; G4 mutation-tested in a scratch crate copy; G5 twelve write
  shapes deny; G10 envelope graded in full, rows re-payloaded not relaxed). Carried forward, not
  findings: plan §5 lists `render.rs`/`views.rs` test files that carry no change (cycle report
  says so); the shared fixture envelope has `sections: []`, so wave 4's suite sees only the
  frontmatter half of the envelope binding unless the fixture gains sections (QA's call at the
  wave-4 open). **WAVE 1 CLOSED (lead) — user acceptance + crate publish (exit condition) owed.**
  `floor: tripped · seats: se-wave1 (mochiko:staff-engineer; 2 sonnet workers read back) / reviewer (mochiko:tech-lead)`
- 2026-09-13 · **user: wave 1 ACCEPTED**; committed on the user's word — `89139a8` (session
  landing + artifacts) and `c395dab` (crate wave 1). **User ruled R1–R11 all as proposed.**
  Wave-2 amend run: the user runs `/mochiko:setup` next. Crate publish (wave-1 exit): not yet
  ruled — working assumption (b), break-glass install through waves 3–4, publish gating the
  `plugin.json` bump; re-put at wave 4. **Wave 3 OPEN** on `se-wave1`: `0005-artifact-homes.yaml`
  (21 homes · 3 template imports · 5 replacements · 19 rule mints · 1 reword), views
  regenerated, `KINDS_NOT_SHIPPED_YET` shrunk (the one licensed crate touch), cost median over
  20 runs vs the 100 ms trigger, budget table as a standalone ratification artifact
  (`wave3-budget-table.md`), cycle report; non-author grade after; no `plugin.json` bump.
- 2026-09-13 · QA contract-plan check: the crate fixture's `sections: []` bites only one row
  class — reports written *inside* a declared `reports/` dir — which the plan had missed (D2's
  open-by-name limb): `G-REPORT-OK` / `G-REPORT-BAD` added (34 rows). Ruled: rows resolve
  against the plugin's real log via `--plugin-root`, asserting structurally; crate fixture as
  fallback only. Relayed to wave 3: `report-envelope` ships with real sections + budgets.
- 2026-09-13 · **wave 3 BUILT** (`se-wave1`): `0005-artifact-homes.yaml` — 46 ops / 2,014
  lines: 20 homes (68 deliverables; `spec-prototype` became a deferred subdir), 3 template
  imports (`report-envelope` with sections+budgets, `architecture-spine`, `architecture-
  concerns`), 5 template replacements incl. the `tasks` skeleton fix, 17 `artifact-home` floor
  mints (two prose skills unreachable — no schema doc — home rule owed as a `.md` edit), 1
  reword. Validate 0 rejecting / 104 advisory; similar-rule clusters 0; `check` median 27.2 ms
  vs 100 ms trigger on the 14,077-line log (the wave-1 37 ms corrected). Seven homes moved to
  `bounds: elsewhere` with reasons. Crate/eval touches: `KINDS_NOT_SHIPPED_YET` emptied, a
  `home` view-kind arm, log-determined pin re-keys (docs 50→73, rules 1,022→1,039, floor
  index, similarity pins, producer-view fixtures, contract floor sets/README — `0004`
  precedent). Contract host cases green; 78 sandboxed cases deferred to wave 4's bump. The
  published-shape binary rejected `0005` loudly until the break-glass install (GI-020 as
  designed). Lead items: two unreachable skills · five `Home*` codes outside `Code::REJECTING`
  · `migrations/README.md` lacks `home`. `wave3-budget-table.md` (150 lines) is the user's
  ratification artifact. Non-author grade dispatched (`wave3-reports/migration-review.md`).
- 2026-09-13 · `report-envelope` confirmed shipping with sections (`Header` heading-less ·
  `## Failure narrative` · `## Notes of note` · `## Null-exit reasoning`, each 15 lines,
  `extra_headings: allow`, `required: [report]` + six-type enum — all verbatim from
  `templates/report-format.md`); both wave-4 report-row classes proven against the real log
  (7 writes: 3 allow / 4 deny, each deny naming its measure). First self-application: the
  wave-3 cycle report itself ran `## Notes of note` at 28 lines vs 15 and was split under
  `extra_headings: allow` — passes clean at its future `reports/` path.
- 2026-09-13 · `wave3-reports/migration-review.md` (`reviewer`): **PASS — 8 Minor, none
  blocking.** Validate 0 rejecting; state hash stable; views diff = exactly the 26 expected
  paths; census collapse reproduced 174/44/42/14/72 from the raw file (199 kinako + 147
  mochiko); ten sampled patterns run live (B53/reviews/landing/evidence/wave1-reports deny;
  desk/wave-file/benchmarks/root DECISIONS allow); 20 homes / 68 deliverables; all four
  deviations OK; item 10 corrected in the seat's favour (`Home*` codes reject at runtime —
  gap is the coverage manifest only). Three findings on the ratification artifact: A4p/A4l text
  lives in `artifact-format.md` rule 4 not the ADR (38 rows mis-attributed); whole-file 150/300
  carry no key; **rule 4 calls the numbers defaults with a justification escape — under the
  gate they become caps** (a change in kind the table must name). Fix round opened; user
  ratifies the corrected table after.
- 2026-09-13 · wave-3 fix round done (`se-wave1`, 8/8): table re-attributed to
  `artifact-format.md` rule 4 as amended, whole-file bounds keyed `S` (seat's call, no
  derivation), change-in-kind paragraph (defaults → caps under the gate; escapes structural);
  `Home*` codes in `Code::REJECTING` + five coverage probes (the wave's only `src/` line);
  README lists `home`; freeze provenance string records the re-key; gates green; migration
  hash/views untouched; the corrected table passes the gate as a fresh write. Delta-check
  dispatched to `reviewer`. **User ratification of the table = wave 3's gate.**

## Wave 2 — governance amend (D7)

- 2026-09-14 · **`/mochiko:setup` amend run (AM-3) — DONE; governance v3.0.3 → v3.1.0 (MINOR).**
  Input `wave2-amendments.md`; deck of 8 user-ruled "as recommended"; synthesis folded and
  frozen; solo cold intent review (`intent-reviewer`, devils-advocate on
  review-governance-intent, blind-map two-message dispatch, 28 angles → 10 survivors incl. 4
  emergent, `critical-gaps`: C1 clause (iv) unconditional while OQ1 open — user-ruled road (a),
  the budget-table condition; C5 the two owed supply-chain controls gate the wave-4 ship — yes;
  C6 MINOR re-put — stands as a recorded departure; C9 the cost cap a D10 watch; C10 this
  repo's violator pass before the local wave-4 install; C2/C3/C4/C7/C8 lead-repaired); verify
  NOT CLEAN 5 → delta-check 1 → closed; ratified. Producer `gov-producer` (tech-lead on
  authoring-constitution) on a lead-approved plan (FC-1…FC-4, FP-1…FP-5); validator
  `gov-validator` FAIL (B1 no rules file scoped the bump surfaces; M1 dormancy tier; M2 expiry
  route) → fix round → PASS 60/61 → two residual rounds → PASS, no residuals; user accepted all.
  Reports: `wave2-reports/intent-review.md` · `gov-producer.md` · `gov-validator.md`. Carried
  forward: the C1 measurement (`record.md` 150 vs 43/66 over) to the wave-3 table ruling · wave-4
  fix candidate: name a standing file-set violation in `additionalContext` · this repo's violator
  pass before the local wave-4 install · the ledger MAJOR limb's depth-flip clause at the next
  PATCH · DECISIONS row 2026-09-13 reworded ("strip" → recorded supersession, no strips entry).
  **WAVE 2 CLOSED.** `floor: tripped · seats: intent-reviewer (devils-advocate, cold) /
  gov-producer (tech-lead) / gov-validator (validator)`
- 2026-09-14 · **wave 2 DONE** — governance v3.1.0 (AM-3), commit `794cea8`, user-ruled MINOR as
  a recorded departure; GI-019 gains the admission + clause (iv); `rust-cli.md` bullet rewritten
  (prior text preserved in the ledger); `cli-schema-delivery` D7 narrowly superseded. Two
  consequences bind this build: **(a) wave-4 hook-ship precondition (review C5):** the
  `plugin.json` bump that ships the hooks MUST NOT land before the crate's first publish with
  all four controls (cargo audit in CI ✓ · sha256 assets ✓ · `cargo publish` behind a
  manual-approval environment — owed · signed release tags — owed); a break-glass install never
  substitutes for consumers — the lead's working assumption (b) is overruled; wave 4 may be
  BUILT but not bumped until then. **(b) clause (iv) condition (review C1, road (a)):** the
  budget table must admit honest content; a bound honest content cannot meet is a table defect
  corrected at the table — measured: the 150-line whole-file bound on brainstorm `record.md`
  vs 43 of 66 mochiko records over it (largest 1,536; the amend's own driver 738) and 7/7
  kinako records, with no in-run route for a fresh over-budget deliverable. Also: a crate gap
  found at the amend's verify pass — an `Edit` to an existing *undeclared* file name rides
  amnesty with a bare allow and no `additionalContext` → wave-4 crate fix list. The ≤ 60 s cost
  cap is a watch by ruling, never a bump gate (review C9). Previous reviewer seat hit its
  session limit; `reviewer-2` (`mochiko:tech-lead`) spawned for the wave-3 delta-check.
- 2026-09-14 · wave-3 delta-check **CLEAN 8/8** (`reviewer-2`): gates green on own run; state
  hash `cbb455a6…` identical pre/post fix round; `0005` untouched by the round (only
  `migrations/README.md` modified). Wave 3 now waits on the user's table ruling (the C1
  amendment: three whole-file rows) → `0005` re-keyed by the seat → bounded re-check → commit.
- 2026-09-14 · `wave4-plan.md` (`se-wave1`, 199 lines) read via the seat's summary and
  **approved**; five primitive edits (two prose-skill home rules additive; `executing-tdd-cycle`
  `description:` path, `analyst-report-template.md`, `techanalyst-report-template.md` →
  supersession strips), no command `.md` re-points (rules come from the log; `impl.reports-
  envelope` re-keyed in `0005`), gate wrapper parses no JSON. Lead calls: **(1)** implement D4e
  file-set amnesty as ratified (existing undeclared name editable with the violation named; new
  name still denies; path non-relaxable) — the built binary denied both, wider than the ledger's
  "bare allow" description, to be corrected in the bump's pre-authorized PATCH; **(2)** one
  un-narrowed `Write|Edit` handler (volume tiny), `if` narrowing on `Bash|PowerShell` only.
  **Wave 4 BUILD OPEN** on `se-wave1` (crate fix → wrappers + hooks.json → five primitive edits +
  three strips → CHANGELOG/marketplace/staged bump) and `qa-wave0` (contract cases under
  `evals/contract/`, real-log resolution, structural asserts). Bump NOT landed (AM-3 precondition:
  crate publish with four controls). Wave-3 commit waits on the user's table ruling.
- 2026-09-14 · `wave4-plan.md` read in full by the lead after the summary approval — holds.
  Accepted the plan's owed ruling: the two analyst report templates re-point to the spec home's
  `reports/` as `report: disclosure` (sixth migration declined). Lead calls (single un-narrowed
  `Write|Edit` handler; file-set amnesty as ratified) supersede plan §(a) and ladder row 3.
  Release mechanics staged: `plugin.json` 0.108.0 → 0.109.0 (MINOR), marketplace sync,
  CHANGELOG entry, gate 6 = cargo test + the full deterministic contract set (78 sandboxed +
  QA's new rows); precondition carried on the commit.
- 2026-09-15 · wave-4 contract cases in (`qa-wave0`): 5 cases, 38 host rows, host run 7/7
  green against the landed wrappers (`gate-input` 30 · `reminder-input` 6 · `if-placement` 2);
  fixtures at `evals/contract/fixture/artifact-hooks/` (a `load_captures()` first-sorted-wins
  trap avoided); report moved to the session dir's `reports/contract-report.md` — `wave4-reports/`
  is not a declared subdir (correct under the gate; `wave0-fixtures/`, `wave1-reports/`,
  `wave3-reports/` re-home in wave 5's pass). **Blocker, pre-existing:** the suite's sandbox
  half cannot start — `evals/contract/run.py` imports `SANDBOX`/`sbx_sh`/old `claude_args` from
  `evals/run.py`, removed at the skill runner's host-mode convergence (`8c27460`, 2026-09-11);
  77 of 82 cases (~151 metered sessions) unrunnable; gate 6 blocked. Lead ruling: the suite
  gets its own sandbox module under `evals/contract/` (lifted from `32c1ed5`), decoupled from
  the eval runner; smoke run only now, full run at the bump. Findings: PowerShell native
  cmdlets (`Set-Content`, `Out-File`, `Add-Content`, `New-Item`) allow — vocabulary gap → crate
  fix routed to `se-wave1`; stale `target/release` binary read as a gate regression → staleness
  guard; `R-LINE-EXACT` reads its golden from `seat-reminder.sh`.
- 2026-09-15 · **wave 4 BUILT** (`se-wave1`): `hooks.json` +2 registrations (un-narrowed
  `Write|Edit`; `Bash`/`PowerShell` narrowed `*.mochiko*`; `SubagentStart` no matcher); two POSIX
  wrappers (`artifact-gate.sh` parses nothing — exit 4 prints the deny, else explicit allow,
  wrapper exit always 0; `seat-reminder.sh` presence-tests the event, frozen line under
  `hookSpecificOutput.additionalContext`); crate fix in `conform.rs` (file-set amnesty as
  ratified, four cells; pre-fix behaviour captured: deny at exit 4, not the ledger's "bare
  allow" — correction owed in the bump's PATCH); five primitive edits + three strips (the
  analyst templates already carried `report: disclosure`; `executing-tdd-cycle` had 125 chars
  of headroom, not none); release files staged at 0.109.0, uncommitted. Gates: 455 tests, fmt/
  clippy/audit, validate 0 rejecting, contract host 7/7. Cost 31.5 ms/call → 6.6 s / 5.0 s per
  run vs the 60 s cap (the un-narrowed arm costs 0.16 s / 0.09 s extra). **Finding:** wave 3's
  17 mints put 11 skills 114–317 chars over their ledger budget — lead ruled the argued-
  overage path (graders rule HOLDS/FAIL per row; HOLDS rows stamped in
  `primitive-cost-budgets.md` at v0.109.0). Audit routing: two prose skills → `mochiko:validator`
  on coherence + preserved responsibilities. `evolution-roadmap.md` left undeclared. **Audits
  dispatched:** `validator` (5 pair/prose units + the 11-row overage ruling →
  `reports/wave4-audit.md`); `reviewer-2` (wrappers, hooks.json, crate fix →
  `reports/wave4-code-review.md`). QA's sandbox repair + smoke in flight. Tree frozen.
- 2026-09-15 · PowerShell write vocabulary landed (`src/hook.rs`, table keyed by `tool_name`;
  `Tee-Object` kept as the `tee` analogue); 457 tests; the suite's staleness guard fired for
  real (six cases red until a rebuild — a saved false green). Cycle report at
  `reports/wave4-cycle-report.md`; staging re-confirmed (0.109.0 uncommitted, untagged). Reviewer
  told the frozen state is the current tree; QA told to promote `G-PWSH-NATIVE` to an asserted
  deny.
- 2026-09-15 · **contract-suite sandbox repaired** (`qa-wave0`): `evals/contract/sandbox.py`
  (helpers lifted from `32c1ed5`, provenance + decoupling reason in the header), keyword
  `claude_args`, staleness guard that refuses (never rebuilds), `R-LINE-EXACT` parses the golden
  from `seat-reminder.sh`. **Smoke 5/5** (absence · skew · brainstorm-absence · `gate-live` ·
  `reminder-spawn`), 16 metered sessions. Two traps caught: the `gate-live` helper inherited the
  sandbox PATH that deliberately lacks the binary (would have blamed the gate) — `path_env` now
  required; `reminder-spawn`'s first shape ("quote your context") was refused by the lead seat
  as an extraction attempt — the case now reads the injection off the sidechain, model out of
  the evidence path (2 of 16 sessions spent on the refused shape). The report trimmed two
  sections to pass the gate on a fresh baseline — the gate biting its own author. Suite changes
  added to `reviewer-2`'s wave-4 review as item 5.
- 2026-09-15 · `reports/wave4-audit.md` (`validator`, `mochiko:validator`): units 3–5 PASS
  (`executing-tdd-cycle` pair — description 501/623, body byte-identical, floors read back;
  both analyst templates + strips verbatim); **unit 6: 11/11 HOLDS** — each mint priced at
  497–558 against a 0001–0004 log, every row under budget without it, no restored prose (small
  body deltas trace to the v0.107.0 reword strips); **units 1–2 FAIL** on rule text — F1 single
  home asserted for dual-home artifacts (`data-model.md`: spec + feature; `api.yaml`: both
  contracts homes), F2 a literal `<slug>` inside a fenced `mochiko-cli home` command that does
  not run (resolves to `specs-index`). Fix round opened (units 1–2 only) + the eleven
  standing-overage ledger lines owed at v0.109.0.
- 2026-09-15 · `reports/wave4-code-review.md` (`reviewer-2`): **PASS, 5 Minor** — gates green on
  own run (457 tests; host 7/7); pre-fix behaviour confirmed on the installed 2026-09-13 binary:
  deny at exit 4 on both limbs (the ledger's "bare allow" sentence corrected at the bump's
  PATCH); cost 30.2 ms median. Minors routed: W1 QA row (in flight) · **W2 reversal** — the
  frozen line is carried in the case as the golden and the script asserted against it (reading
  the golden from the script let a reword pass) · W3 gate exit-4 empty-stdout guard · W4 Edit
  amnesty cell asserts the file name · W5 three undeniable shell shapes (`cd &&`, variable path,
  cwd inside a home) disclosed — to the record at close as D1c/D9 stated limits. Gate 6's full
  sandboxed set and the bump precondition remain owed.
- 2026-09-15 · validator second pass: all eleven triples reproduce; **corrections** — no prior
  headroom (v0.106.0 re-seed, none); the 237 is an exact v0.107.0 render-format constant (six
  control skills); mint cost 497–558 gross; `executing-tdd-cycle` +317 = mint +512 + two-arm
  reword +42 − format 237; seven rows carry a `0003` two-arm residual. Ledger lines to be
  stamped from re-measurement (relayed to `se-wave1`). QA: PowerShell rows promoted
  (`G-PWSH-NATIVE` asserted deny, `G-PWSH-READ` allow control; 31 gate rows, 39 host rows, 7/7);
  the staleness guard is mtime-based (a content-identical rebuild trips it — kept strict for a
  release gate); `R-INJECT` reads as a hook regression when the guard refuses (self-clears).
- 2026-09-15 · wave-4 fix round (`se-wave1`): units 1–2 — `data-model.md` named in all four
  homes (spec · feature · epic · product), `api.yaml` in the `contracts/` home under each,
  `quickstart.md` explicitly not a feature deliverable; the three fenced `home` commands
  replaced by the ruled mints' phrasing; both files additive and under budget (14,355/16,835;
  11,877/13,412). **Eleven standing-overage ledger lines stamped v0.109.0** from re-measurement
  (no budget moved; seven body deltas named to the v0.107.0 reword — validator to settle six vs
  seven). Gates green (457; validate 0/104; host 7/7). Re-audit of units 1–2 + the ledger lines
  dispatched to `validator`; W3–W5 confirmation requested from the seat.
- 2026-09-15 · QA W2 landed: `REMINDER_GOLDEN` carried in `evals/contract/run.py`; `R-LINE-EXACT`
  four limbs (script line == golden · emitted line == golden · one line · names mochiko);
  `reminder-spawn`'s marker from the constant; proven negatively (a reworded script copy trips
  both limbs). Host 7/7, 39 rows. W1 in. Waiting: validator re-audit; seat's W3–W5.
- 2026-09-15 · re-audit (`validator`): **units 1–2 PASS** (deliverables resolved against the
  whole homes set — `data-model.md` in exactly spec/feature/epic/product; `api.yaml` in the
  four contracts homes, all `bounds: elsewhere`; `quickstart.md` absent from feature — a real
  exclusion). **Unit 7 FAIL** on one criterion: the eleven ledger rows reproduce every figure and
  move no budget, but omit the −237 format constant and the `0003` two-arm residual, so no
  row's arithmetic closes (GI-006). Fix = one clause per row. Body deltas settled at seven.
  Routed to `se-wave1` with the W3–W5 confirmation.
- 2026-09-15 · wave-4 round complete (`se-wave1`): ledger lines rebuilt from pristine HEAD rows
  + fresh measurement; identity closes on all eleven (overage = mint + body delta − 237 − `0003`
  render reduction); the "350–550 net" reading withdrawn; the two-arm residual has two limbs on
  two different sevens (render reduction 8–189; body delta +1–42), nine rows carry ≥ 1 limb,
  each row names only its own. W3 fixed + verified with a stub (exit 4, empty stdout → explicit
  allow); W4 fixed; W5 measured — `cd <home> && >`, a variable-held path, and a relative write
  from inside a home all allow (one root cause: the scanner reads literal tokens; `Write`/`Edit`
  unaffected) — to the record at close. Gates green. Unit-7 re-check (`validator`) and W1–W5
  delta-check (`reviewer-2`) dispatched.
- 2026-09-15 · reviewer-2 second pass (pre-delta-check): PASS; item 5 (suite) PASS — `sandbox.py`
  diffed name by name against `32c1ed5`, staleness guard exercised, every D10 clause has a row,
  `path_env` keyword-only with the four bare-PATH sites all absence cases; W1–W4 re-verified.
  Open: W5 (now disclosed in the cycle report — delta-check covers it) and **W6** new:
  `sandbox.py` header claims "unchanged in behaviour" while `claude_args` dropped `--bare` and
  gained two defaults (unconsumed) → header fix routed to QA. The reviewer's own report was
  denied twice by the gate (notes 18 → 16 → 15 lines) before passing.
- 2026-09-15 · seat confirmed W3 (`artifact-gate.sh:48` empty-stdout guard on the exit-4
  branch), W4 (`tests/hook.rs:232`, `:248` assert `notes.md` in `additionalContext`), W5 (cycle
  report section "The shell parse's known gaps, disclosed"). Ledger rows re-stamped a third
  time from pristine rows: −237 on all eleven + the two-arm limb split by row set with the
  `cli-schema-delivery` D9 anchor and the full `0003` filename; identity closes 11/11
  (`analysis-codebase` 9,088 − 237 − 146 + 523 = 9,228). Validator told the frozen state is the
  current tree.
- 2026-09-15 · **unit 7 PASS** (`validator`): identity recomputed from the validator's own
  figures closes 11/11; each row names only its limbs (render limb on seven, body limb on a
  different seven, two on neither); limb sourcing separates the migration's render change from
  the strip-recorded body reword (GI-006); no budget moved, prior history verbatim. **Every
  wave-4 audit unit PASS, `blocking: 0`.** Validator's note: an intermediate row version
  mis-credited the body delta to `0003` — the verdict is against the current text; re-run if
  edited again before the bump. QA W6 fixed (header names three deltas vs `32c1ed5`; argv
  byte-identical in sandbox mode, verified by import). Remaining on wave 4: `reviewer-2`'s
  W1–W6 delta-check.
- 2026-09-15 · reviewer-2 delta-check **CLEAN 5/5** (W2 proved by a one-word reword; W3 three
  stub cells; W5 gaps table reproduced). **WAVE 4 CHECKS CLOSED.** One stale cycle-report line
  (reminder golden "unfrozen") routed to the seat. Record gains a Build-trail transcription;
  BACKLOG item wave statuses touched. Committing waves 3 + 4 as built and reviewed (bump staged,
  not landed; table re-key pending).
  `floor: tripped · seats: se-wave1 (mochiko:staff-engineer) + qa-wave0 (mochiko:qa-engineer) / validator (mochiko:validator) + reviewer-2 (mochiko:tech-lead)`
- 2026-09-15 · **committed `5d8fc69`** — waves 3 + 4 as built and reviewed (104 files incl. the
  staged 0.109.0 release files; not a release: feature branch, no tag, the bump "lands" only
  under the AM-3 precondition). Tree clean. Wave 5 (violator pass) planning dispatched to
  `qa-wave0` — re-home half independent of the table ruling, size-split half waits on it; mochiko
  first (AM-3 C10), kinako on the user's word.
- 2026-09-15 · reviewer-2 delta-check extended: **CLEAN 6/6** (W6 verified by reconstructing
  the original `claude_args` from `32c1ed5` in isolation — argv identical across three modes).
  Cycle-report carried items 1 and 5 cleared as discharged (reminder golden frozen via
  `REMINDER_GOLDEN`; audit routing ruled and run). **WAVE 4 CLOSED (lead).**
- 2026-09-15 · `wave5-plan.md` (`qa-wave0`, 119 lines) read and **approved**. Inventory
  (fresh-write reading, no amnesty): mochiko 474 files / 143 deny (path 69 · size 58 · set 14 ·
  shape 2); kinako 499 / 360 (path 202 · size 114 · shape 32 · set 12). Path is the only
  non-relaxable measure → the re-home half (297 moves: mochiko 83, kinako 214; pointer fixes 204
  / 143) is mandatory, the size half is carried by amnesty (recommendation: rewrite nothing).
  Under the pending C1 amendment mochiko's size denies fall 58 → 6, kinako's 114 → 102 (67
  report-section, 33 whole-file 300–4,744). Findings for the user: 161 kinako evidence files
  (154 `.txt`) have no legal home under `.mochiko/` (recommend moving the tree out);
  `build-log.md` fits no amendment row (needs a `form: log` row); kinako's plugin is a
  project-scoped 0.103.0 pin. Lead calls by analogy: `B61` = R1; `EPIC-001/landing/` → that
  epic's `reports/`. **Mochiko re-home half OPEN** on `qa-wave0`, one class per commit.
- 2026-09-15 · **wave 5 class 1 committed `bd993c3`**: 51 reports out of nine `wave<n>-reports/`
  dirs into each session's `reports/` as `wave<n>-<original>`; 47 stamped with the envelope
  (22 `cycle` · 24 `review` · 1 `verification`), bodies byte-identical; mochiko path denies
  69 → 18 (13 held plans + `wave0-fixtures/`, both class 2). Surfaced and amnestied: two
  over-budget report sections (56/15, 90/15). **Finding:** the placeholder check fires on `<n>`
  inside a backticked path pattern quoted in a frontmatter value — a quotation, not a
  placeholder; crate fix (skip code spans, reuse the classifier) dispatched to `se-wave1`,
  review by `reviewer-2`. Dead `producer_report:` pointer noted for class 4. Class 2 open.
- 2026-09-15 · **wave 5 class 2 committed `73d1746`**: 29 files — 13 plans to their session root
  (`wave<n>-<name>.md`), 5 fixtures to `research/wave0-fixtures/`, 11 working files to
  `research/`; every `wave<n>-reports/`/`wave0-fixtures/` dir gone; mochiko **path denies 0**,
  set 3 (class 3). One path deny became a size deny (`wave1-p3-corpus-plan.md`, 368 lines —
  amnestied; the trade working as intended). Classes 3 + 4 opened together.
- 2026-09-15 · placeholder code-span fix built (`se-wave1`): `outside_code_spans` beside
  `heading_scan`, both placeholder haystacks read through it; unterminated backtick = literal
  (tail still scanned); kept segments joined with a space; 5 cells; 460 tests, gates green.
  The real file (`reports/wave3-migration-review.md`) now denies on `## Notes of note` 29/15 —
  a genuine size violation, amnestied. Non-author review dispatched to `reviewer-2`.
- 2026-09-15 · code-span fix reviewed **PASS** (`reviewer-2`; the splice guard proven on
  `<`x`n>`; the reproduction now denies on size, not `<n>`). Minors P1 (`is_multiple_of` is
  Rust 1.87+, no `rust-version` declared → `% 2 == 0`) and P2 (docstring: two stray backticks
  bracket everything between them — errs toward allow) routed. Reviewer's reading, adopted: this
  narrows where text is read, not what is checked — implementation, not a D4c amendment.
- 2026-09-15 · **committed `c56ca1c`** — the placeholder code-span fix (P1 as a scoped
  `#[allow(clippy::manual_is_multiple_of)]` with reason — the method is Rust 1.87+, no
  `rust-version` declared, raising the floor is a manifest policy call; P2 docstring). 460 tests.
  Classes 3–4 of the mochiko pass in progress (77 paths in the tree).
