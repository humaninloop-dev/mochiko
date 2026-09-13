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
