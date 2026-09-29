# Build log — the joint build of the hook field review and the delta-files retirement

## 2026-09-29 · joint build opened — seams ruled, wave 1 plan approved

- User: "i want to implement together the last two brainstorming session. first find if they have
  decision that clash with each other", then "Can we implement these two session together".
- Cross-read of both records: no direct clash (the delta record already supersedes in part the two
  field-review clauses that disagreed: OQ1's fold shape and the `baseline-delta.md` entry class);
  seven seams and two smaller ones found. Four seams user-ruled R1–R4, then OQ2/OQ5 as R5/R6 at the
  plan approval, each "as recommended":
  [the seams record](../../decisions/2026-09-29-joint-hook-delta-build-seams.md). Landed: its
  `DECISIONS.md` row, annotations on both sessions' rows and index entries, a pointer in the delta
  `BACKLOG.md` item and the Template-schema CLI `ROADMAP.md` Next row (cap held).
- State read before planning: governance v3.2.0 (AM-5) already carries this gate's GI-019 text and
  the GI-012 exception row the bump ships under (R4); plugin 0.115.0 took migrations 0019–0023;
  installed binary `mochiko-cli 0.2.0 · grammar 1..1`; installed plugin cache 0.112.0–0.114.0.
- Plan: [wave1-joint-build.md](wave1-joint-build.md). User: GO. Branch `joint-hook-delta` off
  `main` at `be46e16`; the records above committed there on the user's GO.
- Live field evidence this session: the installed gate denied a read-only
  `sed -n 276,312p .mochiko/memory/governance-ledger.md; echo ----; grep …` as a shell write (the
  glued-`;` shape the field review's S12 names). Not rewritten around; the read went through the
  Read tool. Added to S2's matrix.
- Next: S1 and S2 (`staff-engineer`, default `opus`) spawned plan-only; P1/P2 fresh peers grade.

## 2026-09-29 · plan round — S1 v1 (199 lines, 8 questions) · S2 v1 (138 lines, 4 questions)

- Both plans returned truncated in the message channel; each seat wrote its plan unchanged to the
  session scratchpad (`s1/plan.md`, `s2/plan.md`), read whole by the lead. Tree clean after both
  (`git status --short` empty at `bbbcf10`).
- **Disclosure (S2, self-reported):** during the adopt-first probe S2 sent the user's email address
  as the `User-Agent` of four read-only `curl` calls to `crates.io/api/v1`. Not reversible; told to
  the user plainly the same turn; standing rule sent to both seats (no personal identifier in any
  network request, a neutral agent string if a probe needs one).
- The harness flagged S2's message as instruction-shaped (`settings-json`). Cause, on the full read:
  sample (c) quotes a kinako command verbatim (`shasum -a 256 ~/.claude/settings.json | tee …`). A
  quoted command, not a directive; nothing to act on.
- S2's probe reproduced the census exactly: 79 denies, 21 shell (13 false, 8 true), plus 4 added
  (field review ×3, this session ×1). Finding: F3(c)'s `git show … > $S/pre.md` cases were denied on
  the `cp` source, shape (b) by mechanism; the `git show` spec still gets its own case.
- **Lead rulings on S2's questions (inside D5, disclosed):** Q1 yes, the `[shell]` doc bullet and the
  `[hook]` bullet's reword are both S2's (it owns `lib.rs`) · Q2 all four tightenings in: `-t DIR` /
  `--target-directory`, zsh `>!`/`>>!`, `mv` every non-flag argument, BSD `sed -I` (write positions
  the old scan caught or zsh opens; (i) avoids a regression) · Q3 verbatim commands with heredoc
  bodies trimmed, **after a secrets scan** of all 25 (tokens, keys, `Bearer`, `ANTHROPIC_`,
  passwords) — any hit redacted and disclosed (GI-003) · Q4 agreed, the 23 out-of-set denies are a
  count on one Notes line, no matrix cases.
- **Lead rulings on S1's questions:** Q2 (B) the V2 sniff on every `Resolution::Deferred`, keeping
  sub-directory names in the log (adds a file directly under `features/desk/`, disclosed) · Q3 a
  shell write to a directory under `runs/` denies · Q4 yes, a grammar-1 home change carrying a
  grammar-2 field is rejected at parse · Q5 the `Cargo.lock` version line joins S1's set · Q6 S1's
  reading: with `form: entries` a bound template's frontmatter, heading and placeholder checks run
  and entry budgets replace its per-section `max_lines` · Q7 yes, `section_max_lines` with
  `entry_heading: "##"` rejected as `home-bounds` · Q8 yes, ten render-target cases replace "seven".
- **S1 Q1 split:** drifts (a) the ledger's "a `.md` write" vs every extension sniffed and (b) `Write`
  also reading the on-disk file are text corrections, carried to the wave-3 text-vs-build check.
  The **`## Header` signature limb** was ruled (`hook-enforced-artifact-schema` D9, record :543)
  and never built — striking it removes ruled content, so it is put to the user; S1 builds nothing
  for it meanwhile.
- Hand-off booked for the wave-3 plan (S1): the contract suite's gate workspaces
  (`evals/contract/run.py:4531`) carry no `.git`, so under tree-root resolution every deny row flips
  to allow unless each workspace gains one.
- Next: P1/P2 fresh `staff-engineer` peers grade the plans with these rulings attached.
- **User ruling (R7, "as recommended"):** the `## Header` signature limb is struck, never built;
  the ledger sentence is corrected at the wave-3 text-vs-build check with drifts (a) and (b).
  Recorded in the seams record; the 2026-09-13 `DECISIONS.md` row and index entry annotated
  superseded in part. S1's plan unchanged by it (it built nothing for the limb).

## 2026-09-29 · plan grades round 1 — P1: S1 FAIL(1) · P2: S2 FAIL(1)

- Graders: fresh `staff-engineer` peers on `mochiko:review-seat-plan` (installed render 0.114.0,
  binary 0.2.0); read-only, tree clean after. **Disclosure (P1):** the plans reached the graders by
  scratchpad path, not verbatim in the brief — a departure from `review-seat-plan.plan-verbatim`,
  forced by the message channel truncating both plans; the files are persisted in the scratchpad.
- **P1 on S1, FAIL item 1:** §2.3's route 1 (the nearest home's deliverable or `reports/` file) and
  "stated as ephemeral" had no design and no test on the deny side. Items 2–6 PASS; 20 cites
  checked; advisories: migration.rs qualifier and `Cargo.lock` line, two missed `home_view` callers,
  cite offsets, `entry_exempt_fields` list-item form disclosed as interpretation, close hands to the
  lead, a root-operating-doc regression case.
- **P2 on S2, FAIL item 1:** the command arms fired on every word (`grep -n tee <home>` read as a
  write), against §3.2/D5; fix: command-word matching after `NAME=value`, a wrapper set (`sudo`
  `env` `command` `nohup` `time` `xargs`, `find -exec`/`-execdir`), rows both ways. Items 2–6 PASS;
  P2 re-derived 79/21 and the 6/5/2+8 split from S2's outputs.
- **Lead rulings on the advisories:** sed/perl script words dropped from the targets (P2 A2 — they
  would become false denies under the closed world) · S1's cwd join **closes disclosed evasion 3**
  at the decision level, ruled in with `decide`-level cases both ways; evasions 1 and 2 stay open
  (P2 A3, the wave plan's §3.2 sentence superseded on this point) · the rest folded as given.
- Both seats resumed for v2 (`s1/plan-v2.md`, `s2/plan-v2.md`); the same graders re-grade
  (`review-seat-plan.same-grader-regrades`).

## 2026-09-29 · plan grades round 2 — P1: S1 v2 FAIL(1) · user ruled re-plan

- P1 on S1 v2 (306 lines): the v1 fix list closed (`Homes::nearest`, both routes, deny-side tests,
  all seven advisories folded); **new FAIL item 1** — `home` on a closed-world path still prints
  "nothing here is checked at write time" (`render.rs:561`), the line D3 rules out and F5 names as
  the door the copying pattern walked through; v2's test asserted only route substrings.
- Second consumption of the re-plan bound, so put to the user (`review-seat-plan.approval-is-the-leads`):
  re-plan again / re-staff / narrow the scope. **User: re-plan again.** S1 resumed for v3 with the
  one fix; lead ruling on P1's advisory: keep the contract suite's reason openers ("not a declared
  sub-directory", "not a declared deliverable", `evals/contract/run.py:4311,4314,4357,4848`) in the
  rewrite; an allow case pins disclosed evasion 2.
- S2 v2 (133 lines): command-word matching after `NAME=value` and the wrapper set, `find -exec` /
  `-execdir` fed to the same lookup; prototype holds 25 census rows, 53 rule rows, 24 carried cases.
  Secrets scan over all 25 commands: no credential (4 hits — a SHA-256 of `~/.claude/settings.json`
  twice, "credential" in prose twice — all in heredoc bodies the trim removes; re-scanned on the
  exact committed text). The harness flagged S2's message as `settings-json` again: the same quoted
  path, not a directive. Out-of-set count corrected 23 → 30 (7 in kinako `fix-sandbox-check`
  `5cde9fb6`, after run 4); Q4 ruling stands.
- **Lead rulings on S2 v2's questions:** build both regression closers — shell reserved words
  (`do`, `then`, `else`, `{`, `(`, `!` …) are skipped before the command word, and `git mv` is read
  as `mv` (both ends targets) · a `perl` program file (first non-flag with no `-e`/`-E`) is a read
  and is dropped, as `-e` values are; with `-i`, the file arguments after it stay targets · an
  unquoted `(`/`)` is lexed as an operator, so `(tee <home>)` and `x=$(tee <home>)` find their
  command word (an unlisted miss today, closed; rows both ways). S2's prototype: 9 paren rows
  pass, every earlier row holds. Residual, disclosed in the census Notes, not closed: a
  substitution inside double quotes (`echo "$(tee <home>)"`) is one quoted word and is never lexed
  (today's code misses it too).
- S1 v3 (333 lines, v2 plus targeted edits): the closed-world `resolved:` line and its red test,
  the reason openers pinned, the evasion-2 allow case. With P1.

## 2026-09-29 · plan grade round 3 — P1: S1 v3 PASS · S1 approved, executing

- P1 re-graded v3: **PASS**, all seven items; `diff` v2→v3 confirmed targeted edits only; the v2
  fix list closed (`resolved:` line "`.mochiko/` is closed, so a write here is refused" plus the
  routes; struck wording asserted absent; outside `.mochiko/` byte-identical). Advisory carried to
  T9: confirm the evasion-2 pin against S2's landed `shell.rs`.
- Lead approved on the PASS; S1 resumed to execute T8, T1–T7, T9 held for the S2 relay. S2's v2
  still with P2. Tree clean of seat writes at the approval.
- P2 re-graded S2 v2 as amended: **PASS**, all seven items; P2 re-ran the prototypes read-only
  (53 rule rows 0 failures, 24/24 carried, census 17 FP name no home, 8 TP exact) and re-derived
  the out-of-set 30. **Lead rulings on its advisories:** A1 `timeout` `nice` `stdbuf` `exec`
  `builtin` `doas` join the wrapper set (today's any-word arms catch them) · A2 a sed backup suffix
  is `.` with no `/`, a word with `/` is a file, two rows returning [home] · A3 `( tee <home> )` a
  row; the glued forms closed by the paren ruling, double-quoted substitution the one residue · A4
  targets beginning `$`, `{}` or `~` dropped (unresolvable expansion, fail-open, no regression;
  would otherwise be closed-world false denies under the cwd join; S1 told) · A5 test renamed.
- Lead approved; S2 resumed to execute. **plans: S1:PASS(2) · S2:PASS(1)**, no dirty tree.
- P2 checked the paren ruling against v2: PASS unchanged; none of the 25 census rows changes (all
  their parens are quoted). Folded: the operator split is POSIX-flavor only (A6); the census Notes
  list the double-quoted and backtick substitution misses (A7).

## 2026-09-29 · execution — single-writer breach (self-disclosed) · S1 T8 done

- **Transport-floor breach, disclosed by S1:** its `cargo fmt --all` (08:47:39) likely rewrote S2's
  `src/shell.rs` (same-second mtime as S1's `hook.rs`); layout-only, but a concurrent S2 edit could
  have been lost. Relayed to S2 to re-read and re-apply. Rule for both seats from here: format
  only one's own files (`rustfmt --edition 2021 <files>`); workspace-wide only
  `cargo fmt --all --check`. The G1 review reads `shell.rs` against S2's account.
- S1: T8 done (grammar 1..2, crate 0.3.0, `Cargo.lock` line); `migrate validate` on today's log 0
  rejecting under the new binary; T1 in. `cargo test --all` does not compile at `tests/shell.rs`
  while S2's module is in flight; S1 tests per file meanwhile.
- **S2 landed:** `src/shell.rs` (707 lines), `tests/shell.rs` (29 tests, the 25 census rows exact),
  `lib.rs` (module line, `[shell]` bullet, `[hook]` reword), `reports/w1-shell-census.md` (Write
  tool, dry-run allow). Final extraction 21. TDD trail: 21 of 25 census rows red on the old code
  (17 FP + 4 TP where the old scan returned extra words), 23 rule tests red, all 29 green after.
  Both scans over 48,284 transcript commands, 0 panics. Secrets second pass: none. S2's gate tails
  (shared tree, `CARGO_TARGET_DIR=target/s2`): test all ok (shell 29, hook 30) · clippy exit 0 ·
  audit exit 0 · `fmt --all --check` exit 1 on S1's in-flight test files only, S2's three clean.
  Disclosed: an environment-only `matrix_similar` failure with a target dir outside the repo; the
  `$((1<<2))` heredoc-opener residue (added to the census Notes on the lead's ask); a few gate
  captures under `/tmp/claude-501/` outside the scratchpad. S1 relayed: T9 unblocked.
- **Breach resolved, nothing lost:** S1's fmt (08:47:39) rewrote S2's T1 verbatim copy only; S2's
  full rewrite replaced the file later (08:54:03, its own `rustfmt`). S2 checked all 28 items
  present and none of the T1 copy's names left. A6 in place (`shell.rs:563`, POSIX-gated split,
  a PowerShell test pins it); A7 in the census Notes and pinned by the backtick test cases.
- S2 closed: the `$((1<<2))` residue added to the census Notes as checked on the real module (a
  newline after it hides later lines until one reads `2`; the same line is still scanned). Notes
  10 lines. S2 idle, awaiting G1.

## 2026-09-29 · S1 T1–T8 done — four layers green · T9 started

- S1 changed `src/{home,conform,hook,cli,render,migration,validate}.rs`, `Cargo.toml` (0.3.0), six
  test files, the root `Cargo.lock` version line; no committed fixtures (test trees built under
  `CARGO_TARGET_TMPDIR`). Tails: `cargo test --all` 551 passed 0 failed · `fmt --all --check` exit
  0 · clippy exit 0 · audit exit 0 (31 deps) · `migrate validate` 0 rejecting · 113 advisory ·
  `mochiko-cli 0.3.0 · grammar 1..2` · views emit ≡ `.mochiko/schema-views` (`diff -rq` empty).
- **Disclosed, carried to G1:** tests not observed red first (the T3 hook/cli closed-world cases,
  a compile error ending that run; two written after their code; three pins by design; one T5
  fence test first written unfailable, corrected) · build-time choices inside the plan (a log with
  no home closes no world; `home`'s directory reading wins over any resolution but a file or a
  report, since `features/desk/<date-slug>` resolves Deferred; the no-prefix sentence only where
  true; worktree and guard checks also on `implement-log.md`) · existing tests changed (grammar
  out-of-range list from 3, `.git` in the scratch helpers, absolute paths in `home` tests, one
  rename, fixture logs at grammar 2 with the `runs` home) · until T9 a shell write into `runs/`
  still denies.
- S1 had not seen the S2 relay (messages crossed); re-sent, T9 started.

## 2026-09-29 · S1 T9 done — wave-1 build complete, to G1

- T9: `hook.rs` calls `crate::shell`; the old tables, `cmdlet_targets`, `bare_command`,
  `non_flag_args`, `tokenize` deleted; the `runs/` carve (main tree, guard, directory target
  denies, `.md` denies incl. `implement-log.md`, other files allowed); 4 decision tests red on the
  old tables then green; evasion 3 closed since T1, 1 and 2 pinned allow (2 confirmed on the landed
  `shell.rs`: `write_targets` returns `[]`). Tails: 556 passed 0 failed · fmt/clippy/audit exit 0 ·
  `migrate validate` 0 rejecting · views identical.
- **S1 disclosures:** a `runs/` sub-directory named without its trailing slash reads as a non-`.md`
  file and is allowed (no stat) · the `tests/hook.rs` `shell()` helper now JSON-escapes newlines ·
  a `mv` of a `.md` out of `runs/` denies (a removal is a write).
- **Lead ruling on the no-stat gap:** accepted and disclosed, no directory stat (a read GI-019
  does not list). The carve never bounded content in the first place — `cp report.md
  runs/<id>/x.txt` already lands a report's text under a non-`.md` name, the limit control 4 states
  ("the gate cannot see what a shell write puts in a file") — so the gap adds no new exposure.
- Next: G1 (plain `general-purpose`, `model: opus`) reviews the whole wave-1 diff.

## 2026-09-29 · G1 code review round 1 — FAIL (2 blocking, 10 advisories) · fix round ruled

- G1 ran the four layers itself (556 passed; fmt, clippy, audit exit 0), `migrate validate` 0
  rejecting, views identical, and drove HEAD's 0.2.0 binary beside the new one on write-shaped
  commands. All S1 clauses PASS; S1's not-red-first tests confirmed to fail at HEAD; `shell.rs`
  intact after the fmt incident; no read outside GI-019's list, no network call.
  [reports/w1-code-review.md](reports/w1-code-review.md).
- **B1** (S2): a `-t` inside a short-flag cluster (`cp -rt`, `-vt`, `-at`, `install -Dt`) is missed —
  a deny-to-allow regression the Q2 ruling meant to prevent. **B2** (S2): writes HEAD denied now
  allowed and undisclosed — wrappers outside the set (`watch`, `strace -o f`, `parallel`) and a
  process substitution among `tee`'s arguments (`tee >(cat) <home>`).
- **Lead rulings for the fix round (one round, `common.gate-loop-bound`):**
  - S2: B1 as G1 gives it · B2(b) closed in the tokenizer (a `>(`/`<(` group lexed as its own
    command, the outer run resumed) · B2(a) recorded as a class in the census Notes with pinning
    rows, the set not widened · A3 relative targets after a `cd`/`pushd` in the same command are
    dropped (the cwd is unknown; removes the mirror false deny, evasion 1 unchanged) · `((…))` and
    `$((…))` lexed as one word (no redirect, no heredoc; closes A3's `(( 3 > 2 ))` deny and the
    `$((1<<2))` residue) · A7 perl `-l`/`-0` take digits only · A1 a recursion depth cap, fail-open
    past it · A4 a word-initial `#` ends the line in the POSIX dialect.
  - S1: A2 the run-key sentence skipped when the next segment is absent or matches · A5 a path in no
    git tree allows with no sniff (D3: nothing changes outside `.mochiko/`; the scratchpad and
    `~/.claude/` stay the seat's) · A9 the on-disk file read only after resolution, where needed.
  - Accepted and disclosed, no change: A6 (a shell payload with no `cwd`; the platform always
    sends one) · A8 (case-sensitive segments on a case-insensitive filesystem; pre-existing, booked
    for BACKLOG at the wave close) · A10 (`home <home>/reports/` wording; cosmetic).
- **S2 stop on A3 (correct):** dropping every relative target after a `cd` flipped three census true
  positives (K02, K05, K11 — `cd <abs repo root>` then a relative home write) to allow. **Re-ruled
  (S2's option ii, plus scoping):** after `cd`/`pushd` with one absolute literal operand, later
  relative targets join that directory; after a relative operand, none, `-` or an expansion, they
  drop; a `cd` inside `( … )` holds only to its `)`. All 8 true positives hold; the mirror false
  deny goes. Side effect accepted and disclosed: **evasion 1 closes for an absolute literal `cd`
  into a home**, stays open for a relative operand and for expansions. S1 told to re-check its
  evasion-1 case's operand form.
- S1 fix round: A2, A5, A9 built test first (3 new tests red, then hook 40/40, conform 56, cli 62,
  home 42 at the last compile before S2's edit; a fifo test proves a Write to Outside never opens
  the file). **Disclosed and accepted:** the path-class verdict now precedes `Edit`'s early allow,
  so an `Edit` on a missing file or an absent `old_string` at a closed-world or refused run-folder
  path denies where it allowed (the platform rejects that edit anyway; the verdict does not depend
  on the body). Re-confirmed for wave 3: the contract suite's gate workspaces need a `.git` each,
  or every home row and `G-SNIFF` resolves no-tree and allows.
- S2 fix round interim: seven items built, each red first (5 new tests failed on the old code,
  the depth test overflowed the stack; the `watch` pin passed as a pin should); isolated build 36
  passed. B1 cluster scan · B2(b) a `Token::Substitution` group · `((…))` one word ·
  A7 · A1 `MAX_DEPTH = 64` · A4 comments. Sweep over 48,284 commands: 0 panics, 0 targets lost,
  1 gained — a real `perl -0pi` in-place write A7 now catches. A3 pending (the ruling crossed).

## 2026-09-29 · fix round — A3 refined on the sweep

- S2 built A3 as ruled and swept 48,284 commands: of 705 naming a home, 261 changed form only (a
  relative target joined to an absolute literal `cd`), 26 gained a target (evasion-1 closures), and
  **35 lost theirs**, all after a `cd` to an expansion (`cd $MAIN` 26, `cd $W` 8, `cd $R` 1) —
  real writes into homes the pre-A3 join denied.
- **Lead refinement:** after a `cd`/`pushd` to an expansion, a relative target whose first segment
  is `.mochiko` is kept (joined to the payload cwd as before A3); other relative targets drop. The
  35 deny again, and A3's mirror false deny stays gone. Residual accepted and disclosed: `cd $X`
  outside the tree followed by a `.mochiko/…` write denies (no census row has that shape). S2's
  choices confirmed: `popd` leaves the directory unknown; a process substitution is its own scope.
- S2's round report crossed the refinement: 8 items built, 39 shell tests, full-tree four layers
  green, census Notes 13 lines (dry-run allow, sha matches). S2 resumed once more for the
  `.mochiko`-first keep rule and a re-sweep. Field evidence: the installed 0.2.0 hook denied S2's
  heredoc-built scratch script once (the old scan read a body token as a target); nothing ran,
  scripts rewritten with Write and run by path.
- **S2 fix round closed:** the refinement built test first (1 new test, 12 rows, red then
  green; shell 40 tests). Sweep: against unrefined A3, 35 gained with exactly their pre-A3
  targets, 0 lost, 0 changed; against the pre-fix-round scan, 0 lost, 27 gained, 261 pure joins.
  S2's reading, accepted: leading `./` segments are skipped before the `.mochiko` check. Census
  Notes 13 lines, dry-run allow, file equals dry-run text, secrets 0. Full-tree four layers green
  (S2's run). S1 relayed for its evasion edits and the full-tree gates.
- **S1 fix round closed:** A2 (`conform.rs`), A5 and A9 (`hook.rs`), the evasion test reworked —
  3 closed at decide, 1 closed for an absolute literal `cd`, 1 open for a relative operand, 2 open,
  the `cd $D` + `.mochiko/…` keep denies; each deny asserts the home-shell reason text. Full tree:
  570 passed 0 failed · fmt/clippy/audit exit 0 · `migrate validate` 0 rejecting · views identical
  (80 files). S1 side note, no change: a shell deny names the target as written, not its resolved
  path. To G1 for the re-review (same seat, resumed; reads the fix delta).

## 2026-09-29 · G1 re-review PASS · lead gates green · wave 1 closed

- **G1 re-review: PASS** (`re_review_round_1:` block in `reports/w1-code-review.md`). Re-ran the four
  layers (570 passed), `migrate validate` 0 rejecting, views identical; B1 and B2 closed (B2(a)
  recorded and pinned); A1–A5, A7, A9 fixed; A6, A8, A10 unchanged as ruled; the HEAD-vs-new
  differential shows only the recorded wrapper class allowing; probes at the `cd` joining, the
  `.mochiko`-first keep, subshell scope and large inputs: no crash, each under 0.3 s.
- **Five new advisories, none blocking, booked for the wave close (not fixed in wave 1):** R1
  narrow `cd` false allows (`popd`, a `cd` in a pipeline or background, `cd -` after an absolute
  `cd`; none in the 48,284 sweep) · R2 `((echo a); tee <home>)` read as arithmetic, a miss that
  was a deny before the fix round · R3 a relative `cd` could be appended rather than dropped
  (would close evasion 1 fully) · R4 `cd $MAIN` from a worktree then a `runs/` write refused by the
  worktree rule, inside the accepted false deny, the reason names the path to use · R5 a consumer
  with no `.git` at all gets no gate — joins the wave-3 contract-workspace `.git` note.
- **Lead gates, run first-hand:** `cargo test --all` 570 passed 0 failed (15 suites plus doc) ·
  `cargo fmt --all --check` exit 0 · `cargo clippy --all-targets -- -D warnings` exit 0 ·
  `cargo audit --deny warnings` exit 0 (31 deps) · `mochiko-cli 0.3.0 · grammar 1..2` ·
  `migrate validate · 0 rejecting · 113 advisory` · `views emit · 80 documents`, `diff -rq` empty.
- Wave-3 carries: the contract suite's gate workspaces each need a `.git` (S1; G1 R5) · the three
  ledger text corrections (R7's struck limb, drifts (a) and (b)) · R1–R3 and A8 to BACKLOG at the
  joint bump unless the wave-3 plan takes them.
- `floor: tripped · seats: S1, S2 / P1, P2, G1 · plans: S1:PASS(2) · S2:PASS(1)` — no dirty tree
  at any plan return; one single-writer breach (S1's `cargo fmt --all`), disclosed, nothing lost.
- Wave 1 is uncommitted on `joint-hook-delta`; the commit is put to the user.

## 2026-09-29 · wave 1 committed · wave 2 opened — plan-only dispatch

- User: "Commit and plan wave 2". Wave 1 committed `91b3838` on `joint-hook-delta` (secrets and
  email scan over the new files first: none). Nothing merged to `main`.
- Plan: [wave2-joint-census.md](wave2-joint-census.md). Sequence ranges: S4 0024–0031 (the delta
  rule supersessions, one decision per file), S3 0032–0035 (home documents, after the user
  ratifies the census table). S5 (`staff-engineer`) plans after both land.
- S3 and S4 (`tech-lead`, default `opus`) spawned plan-only; fresh `tech-lead` peers grade.

## 2026-09-29 · S3 plan returned (plan-only, clean tree) · Q1–Q4 ruled by the lead

- S3's plan: scratch `s3/plan.md` (322 lines). Tree at return: only the lead's two files; kinako
  shows only its pre-existing `M .gitignore`.
- **Q1 (withdrawing `feature-contracts` and `epic-contracts`): accepted — replace each with an
  empty `deliverables` set.** The log has no op that removes a document, and dropping `contracts`
  from the parent's `subdirs` withdraws nothing (the child home resolves first). `render.rs:707`
  already renders an empty set. Conditions: the replaced document's text says it is withdrawn by
  delta D4 and names `product-contracts`; the table quotes the deny a `contracts/` write then gets,
  so the user sees whether it names a route. If `migrate validate` rejects the empty set, that is
  the §7 stop. A `withdraw-document` crate op is not built this wave; it goes to BACKLOG at the
  joint bump as a candidate.
- **Q2 (the epic home takes two rulings): accepted — one `replace-document` in 0035**, anchored
  delta D4, its intent naming field review D6.
- **Q3 (boundary with S4): S3 plans no template change, accepted.** Delta D2 (`:280–282`) leaves
  which fields each store's entries gain to the census table. So S4's reword of
  `impl.baseline-delta-grammar` states the marker values, the key set and "a field of its own in the
  entry", and points at the store's home document for the per-store field set and placement; it
  fixes no placement. If the ratified table needs rule wording, the lead relays it to S4 as a
  follow-up inside 0024–0031.
- **Q4 (the 177): reading confirmed** — `entry_max_lines ≥ max(177, largest honest M-variant
  entry)`. Source: delta record `:601` ("kinako's largest entity entry is 177 lines"), field review
  `:206`, `:516`, `:654`. S3 traces which entity, level and commit gave 177. If the trace shows a
  mis-measure, the floor still stands until the user rules on it as a table row.
- The spec home (`.mochiko/specs/<slug>`, `0005:72`) is distinct from the feature home, so S3's
  choice row for its baseline copies is in scope.
- Lead's own shell append to this log was denied by the installed gate (a shell redirect into a
  declared home); written with Edit, the route the deny names. Correct deny, not a false positive.
- Next: P3 (fresh `tech-lead`) grades S3's plan as ruled. No GO before a PASS.

## 2026-09-29 · S4 plan returned (plan-only, clean tree) · Q1, Q2, Q4–Q7 ruled · Q3 to the user

- S4's plan: scratch `s4/plan.md` (251 lines, with an addendum folding the lead's `Lifecycle:`
  placement boundary). Sweep first pass: 234 hits on 204 nodes; all 27 D6d ids and all 6
  untouched-list ids hit; 144 hits in another sense. Planned: 0024–0029, one per decision (D1, D2,
  D3 with seam R2, D4, D6b, D7); 0030–0031 left as a legal gap.
- **Q1 (strips): none — accepted.** Rules and templates are schema content, recorded by the log
  (`primitive-edits.md:17–24`, `strips/README.md:73–87`); the wave plan's strip line was wrong and
  is corrected in `wave2-joint-census.md` §1 and §3b.
- **Q2 (the §7 stop, 15 in-scope sites D6d missed): all 15 ruled in**, each riding its decision's
  file; D6d's `Assumed` completeness is discharged by this sweep. Sites 7 and 8 (the D4 clause of
  `authoring-technical-requirements.artifact-home` and `authoring-epic.artifact-home`) are S4's;
  any wave-3 render-target edit to those rules builds on S4's text. Site 15 mints
  `review-sufficiency.lifecycle-marker-read` (D7 `:494–496`, M6 `:279–281`).
- **Q3 (what a changed or removed entry reads after sign-off): put to the user.** D7 rules that
  every pre-checkpoint baseline write is `proposed`, and that sign-off flips every `proposed` to
  `in-flight`; D2 says an amendment reads `modifying` and a removal `removing`. The two texts
  disagree on the flip target. Five texts wait: the grammar rule, `lifecycle-statuses`, the store
  template legend, the store gate's successor, and the checkpoint flip.
- **Q4 (R2 timing): accepted** — the run's lead pins the base at entry, before spawning the
  sufficiency seat; the brief carries it, the seat writes it, the run-open confirmation restates it.
- **Q5 (the absent-baseline seed): accepted** — D2 applied as written (an entry written during a
  run is marked), so seed entries are `proposed (<key>)`; no third exclusion to the unmarked-write
  test; the card lists them apart from the design's entries.
- **Q6 (reword vs supersede): accepted** with one condition — a reword keeps every responsibility
  the rule had except what the anchored ruling removes, and the file's intent names what left; a
  lost responsibility with no ruling is a supersession, not a reword. The wave-3 gate audit's
  preserved-responsibilities check grades it.
- **Q7 (a baseline one epic member alone touches): accepted** — keyed to that member's `FEAT-XXX`
  (D2: "the key is the run's owner"; `EPIC-XXX` is for a joint write).
- Next: P4 (fresh `tech-lead`) grades S4's plan as ruled, with Q3 open. No GO before a PASS.

## 2026-09-29 · S4's Q3 ruled by the user — seam R8

- User: "Flip by kind (Recommended)". Before the checkpoint every run write reads `proposed
  (<key>)`; at sign-off the flip reads the pinned-base diff per entry — absent at the base
  `in-flight`, changed `modifying`, cut to its heading `removing`. Recorded as R8 in the seams
  record; `DECISIONS.md` rows (2026-09-29, 2026-09-24) and the brainstorms index annotated.
- S4's five waiting texts (grammar rule, `lifecycle-statuses`, store template legend, store gate
  successor, checkpoint flip) are now unblocked; relayed to S4 and to P4 mid-grade.

## 2026-09-29 · plan grades: S3 FAIL (P3, 5 blocking) · S4 FAIL (P4, 4 blocking) · re-plan 1 ruled

- Verdicts: scratch `p3/verdict.md` (5 blocking, 11 advisory), `p4/verdict.md` (4 blocking, 10
  advisory; revised mid-grade for R8). Both graded v1 as briefed; the seats' v2 folds are ungraded.
  No gate denied either grader; neither wrote outside its scratch folder.
- **Two lead rulings corrected by the grades.** (1) Q1's "every write under them is denied" is
  false for files already on disk: the file-set amnesty allows them (`conform.rs:173–187`); kinako
  tracks 14. Restated: new files under the emptied homes deny; existing files stay editable until
  D5(iii) deletes the feature copies. The deny prints "Declared: none" and never shows the title,
  so the `product-contracts` pointer lives in `home`'s title line; the table quotes both verdicts
  and both surfaces. (2) The `Lifecycle:` boundary's "pointer to the store's home document" points
  at nothing: a grammar-2 home carries only exempt field names and cannot express a field set or
  placement (`home.rs:92–125`, `render.rs:538–564`). Re-ruled: the ratified census table's
  per-store field set and placement are written into S4's grammar rule (0025) as a re-cut after
  ratification; no pointer and no placeholder in the interim; homes carry only grammar-2 fields; no
  per-store template. S4's "states only" addendum (P4 B4) falls with it: the grammar rule keeps all
  its ruled content (D2, N5, D3a, D3b, D7's build-raised clause, R8).
- **S11 vs delta I9 (P3 B1), lead reading:** I9 governs. It is the later user ruling (2026-09-24,
  "user-ruled inline") and names the epic directories specifically: `EPIC-001` and `EPIC-002` are
  closed record, untouched, exempt from the census — including the three S11 sub-directories inside
  them. The table marks them `kind: ruled` (I9); consequence: existing files editable under
  amnesty, new files denied. Kinako's evidence trees are `kind: ruled` too (deleted at wave 4,
  D9(b)/S11). Choice rows remain for `features/B53`, `features/B61`, `features/FEAT-006/reviews`
  and the archive's non-evidence files. The field review's `DECISIONS.md` row takes the annotation
  at the joint landing.
- **S4 additions (P4 B1): both ruled in** — `authoring-feature-map.in-flight-territory-read` (0029,
  D7) and `impl.design-seats-staffing` (0024, D1). S4 re-walks the 144 other-sense hits on P4's
  test (does a ruling change what the rule directs?). Ruled in by class, no round-trip: a reader of
  another run's planned contract (D7); "design(-phase) deltas" or "folds" meaning the per-feature
  copies or the baseline fold (D1); the package-drafted store delta of the prior D10 step (D1/D7).
  Anything else comes to the lead before execution.
- **P4 B2:** `authoring-technical-requirements.store-write-at-sign-off` is superseded with a new-id
  floor (0029, D7), per the Q6 ruling; `expected-skills.json:229` joins S5's hand-off.
- **P3 B2, B3; P4 B3:** the seats take the graders' fixes (the id-based new-entry clause; the next
  honest writes tested at their destinations; every R8-dependent text worded over the in-flight
  class).
- Transport (P3 A10): no seat runs `cargo build` or `cargo test` without a slot from the lead —
  `target/` is shared and `fidelity.rs` clears `target/tmp`.
- Re-plan 1 for each seat; the same graders re-review. A second FAIL goes to the user.

## 2026-09-29 · P4's revised verdict (still FAIL) · seam R9 ruled by the user · a gate deny

- P4 revised after R8: 4 blocking, item 5 now PASS. B3 now reads: R8's post-sign clause has no
  text, and the two sufficiency readers leave signed `modifying` / `removing` unclassified — both
  already in the lead's re-plan ruling. New advisory A10: what a `removing` entry becomes at landing
  is unstated (D2's sweep "flips to `built`").
- A10 put to the user: "One-line stub (Recommended)". At landing a `removing (<key>)` entry keeps
  its heading as `**Lifecycle:** removed`, no body, no key; its id stays taken (the next-free-id read
  counts stubs); readers treat `removed` as absent.
- **Gate deny, first on its path:** adding R9 to the seams record denied — "`<date-slug>.md` is 153
  lines against a whole-file bound of 150." The three smaller R9 edits sent in the same batch had
  landed, leaving R9's rationale and status without its decision; the lead reverted them (the record
  back to R1–R8). Surfaced to the user, who ruled "Its own record (Recommended)": R9 now lives at
  `.mochiko/decisions/2026-09-29-landed-removal-stub.md`, the seams record carries a two-line
  pointer (146 lines), a new `DECISIONS.md` row, the delta row and the brainstorms index point at it.
  Lesson: an edit batch that grows a bounded file lands the decision paragraph first, alone.
- R9 relayed to S4 (into its re-plan) and to S3 (a stub is an entry for the census's count).

## 2026-09-29 · S3 re-plan 1 returned (`s3/plan-v3.md`) · one lead correction · to P3

- S3's v3 (563 lines) takes P3's five blocking fixes and all eleven advisories, each with a
  disposition; no execution, both trees unchanged, git reads with `--no-optional-locks`.
- **Lead ruling corrected again, on S3's probe:** the I9 consequence "existing files editable under
  amnesty" holds only for undeclared *files*. An undeclared *sub-directory* denies outright with no
  amnesty (`conform.rs:190–195`), so every write into `epics/EPIC-001/reviews`, `EPIC-001/landing`
  and `EPIC-002/reviews` denies — and equally into `features/B53`, `B61`, `FEAT-006/reviews` unless
  the table declares them. Confirmed: the I9 rows read "every write denied (path class); closed
  record, no next write". Whether any live writer targets `epics/<EPIC-ID>/reviews/` is A1's to
  show; if one does, the epic home's set is a table row.
- P3 re-reviews v3 (re-plan 1 of 1 before the user).
- S3 folded R9 and A1 step 7 (the open-epic check) into v3 in place while P3 held it (574, then 581
  lines); P3 told of each addition; v3 frozen for the grade.

## 2026-09-29 · S4 re-plan 1 returned (`s4/plan-v3.md`) · Q8, Q9, A8 ruled · to P4

- v3 (395 lines) takes P4's four blocking fixes and ten advisories. The re-walk of the 144
  other-sense hits adds four by the lead's classes, all 0029: `patterns-sound-loop.
  governing-surface-table` (b), `review-sufficiency.clause-structural-trigger` (b),
  `review-feasibility` `conditions.store_delta.note` (c, a set-condition), `review-feasibility.
  architecture-pass-gate` (c); 137 stand with grouped reasons. Totals: 9 supersede · 10 mint · 35
  reword · 1 set-condition · 4 template replace.
- **Q8 (the marker grammar reaches only the store's writer at write time): accepted.** One clause
  in `impl.baseline-entry-grammar`: every brief to a seat that writes or amends a product baseline
  carries the rule as an obligated read (the `impl.briefs-name-rules-files` pattern; CLAUDE.md axis
  4). Prose pointers in the three prose skills are wave 3's.
- **Q9 (`authoring-technical-requirements.sequential-ids` starts ids at `001`, against D3a):
  accepted** as a 0026 reword — continue the product file's own sequence, a `removed` stub's id
  counting as taken; the intent names the `001` start as what leaves.
- **A8 (`evals/plan/implement/observable.yaml` pins five superseded ids, outside every set):** S5's
  set widens to `evals/plan/**` where a pinned id moves; `wave2-joint-census.md` §1 amended.
- **A2 (the designer-only-writer clause vs the design template's open truth writer):** flagged, not
  settled here — it belongs to the queued design-truth rehoming brainstorm (setup-product-agnostic
  OQ1); the lead carries it to that item at the joint landing.
- **A10 reading, confirmed:** a removed store element is a table row kept with its id, kind, name and
  status `removed`, and is not drawn — R9's one form, in the store's row shape.
- P4 re-reviews v3 (re-plan 1 of 1 before the user).

## 2026-09-29 · P3 re-review PASS on S3 v3 · GO S3 phase A

- P3 `re_review_round_1` (in `p3/verdict.md`): PASS — B1–B5 closed, no new blocking defect, 9
  advisories. Its header cites 563 lines, but the section grades R9 and the open-epic condition
  (`:169`), the two in-place additions; lead checked. P3 re-ran S3's epic `reviews/` probe (denies)
  and the counter parity (28 entries, 10 sections).
- Taken into execution by ruling: RA1 (the constraints S row carries one budget per option of the
  `####`-to-`###` re-level choice) · RA2 (a rule places run-4 §A2's "Part 2 · Decisions" body, about
  37 lines, D-050…D-061) · mochiko's `.mochiko/archive/**` files are frozen (operating-docs rule),
  so their rows offer "declare" only, never "move". The other six at S3's judgment.
- **RA3, a latent crate defect, routed to the wave-3 plan** (BACKLOG if wave 3 declines): `settle`
  compares a standing fault against the first fault of its key, so a later, larger entry sharing a
  heading makes a no-change rewrite deny — a file wedged by its own history, against D4e. P3's probe:
  kinako's constraints file at HEAD, budget 0 at `###`, denies on "### Part 2 · Decisions" (42, then
  108 lines). Not biting this census once budgets are at least 177.
- **GO S3, phase A only** (facts and table; phase B after the user ratifies). `target/debug/
  mochiko-cli` 0.3.0 as built at `91b3838`; no rebuild, no `cargo` without a slot.
- `plans: S3:PASS(2)`.
- P3 then re-read the frozen file whole (581 lines, md5 `ff6d6564…`): PASS, 0 blocking, 8
  advisories; its RA4 withdrawn (it had stopped at line 563); RA5 narrowed to a label nit (A11
  listed both as a lead ruling and as "taken").

## 2026-09-29 · S3 phase A: installed-gate false deny on a read (`cp` source) · S4 v3 to P4

- S3 disclosed a deny from the installed 0.2.0 gate on a read: `cp .mochiko/product/architecture/
  $f.md <scratch>` inside a `for` loop, the source read as the write target (the field review's F3
  shape b). Nothing written; not retried. S3 now reads those two files through `git show HEAD:<path>`
  piped to `check --content -` or the Read tool — a read by another tool, as the lead did for the
  `sed -n` false deny in wave 1, not a rewrite around a write deny. Accepted and surfaced to the user.
- Lead probe of the same payload against the branch binary (`check --hook-json -`, 0.3.0, working
  tree log): `permissionDecision: allow`. The wave-1 write-position parse closes it.
- S4 v3 frozen at 409 lines (sha256 `1986314c…e2ec`, lead-verified, read-only) after one late fold
  (the 0026 `impl.landing-verifier-folds` reword naming the R9 stub); sent to P4 with the corrected
  hash.

## 2026-09-29 · P4 re-review PASS on S4 v3 · GO S4

- P4 `re_review_round_1` (in `p4/verdict.md`): PASS — B1–B4 closed, 0 blocking, 10 advisories,
  items 1–6 PASS. Graded the 409-line file whole (hash `1986314c…`, matching the log; the lead's
  corrected-hash message had not reached it, it caught the change itself). The four by-class
  additions fit the classes; the 137 standing nodes' reasons hold; totals re-counted.
- Taken into execution by ruling: RA1 (the fence, its dispatch mirror and `clause10-carve` name the
  signed `architecture.md` drawing where "design-phase deltas" leaves) · RA2
  (`in-flight-territory-read` names where the planned contract is read: the baselines' in-flight
  class and the owning run's drawing) · RA7 (NFR-only store changes land through the
  `impl.baseline-diff-review` flip, not gated on `structure_built`) · RA6 is a sequencing rule the
  lead adopts: the post-ratification 0025 re-cut re-runs stamp, validate and the view diff before
  S3 regenerates the views and before S5 pins. The rest at S4's judgment.
- **GO S4, execution** (plan v3 §4). Transport: S4 authors in its scratch log copy; the copy of the
  six files into `plugins/mochiko/migrations/` and the in-place `migrate validate` wait for a slot
  from the lead, who clears it with S3 (whose phase-A reads pin the log state), so no reader ever
  sees a partial set (0024 alone rejects). No `cargo`.
- `plans: S3:PASS(2) · S4:PASS(2)`.

## 2026-09-29 · S4 steps 1–3 done · three items ruled · step-4 slot staged

- Sweep report written once: `reports/w2-delta-sweep.md` (367 lines), dry-run allowed first on the
  branch `check --path` and on the live gate's `--hook-json` Write payload (the session's live
  plugin is 0.112.0, user scope; S4 checked 0.114.0 too). 0024–0029 authored and stamped in S4's
  scratch log: 9 supersede · 10 mint · 35 reword · 1 set-condition · 4 template replace; the final
  set 0 rejecting · 113 advisory; after each file 5, 4, then 0 rejecting (forward cites to rules
  0025/0026 mint), so the six land together; `impl.sec.tools` renders 22,008 of 30,000 characters;
  view diff: 16 files move (9 ids out, 10 in, 35 texts, 1 condition), nothing else.
- Ruled: **RA5** — `sequential-ids` reads "an id is never reused; a gap left when an abandoned run's
  reverted entries were minted over is legal" (no stub for abandoned runs: their entries were
  `proposed`, nothing promised); "gap-free" named as what leaves, under D3a · **`impl.sufficiency-
  report`** gains the run-open pin, one more 0026 reword under R2 (reword total 36) · the concern
  ledger's stance exemption for a row cut for removal before landing accepted as R8's direct
  consequence, disclosed as beyond the plan text.
- Step 4 (copy into `plugins/mochiko/migrations/`, validate, status) waits on S3's "hold"; after the
  copy S3 re-pins A0 to the post-S4 log — the census reads the write sets after the delta rewrites.
- S4 folded the rulings into 0026 (re-stamped; the other five byte-identical): 36 rewords, 0
  rejecting · 113 advisory, `impl.sec.tools` 22,137 characters. S4 scoped RA5 to "a landed id is
  never reused" — a reverted `proposed` id leaves the file, so an unscoped "never reused" could not
  be followed; accepted. The sweep report takes a second write (Edit, in the slot) for the moved
  figures.

## 2026-09-29 · S3 phase A done · slot to S4 · routing

- S3 wrote `reports/w2-census-facts.md` and `reports/w2-census-table.md` (33 rows plus a consequence
  list per pattern), each once, dry-run first against the branch log and both installed logs; no
  deny, no §7 stop, kinako clean after every batch. Its phase A crossed the lead's hold request, so
  it read the pre-S4 log (0001–0023 plus its scratch 0032–0035 drafts); a post-S4 re-take follows.
- Headline figures: under the branch's closed world with log 0001–0023, 605 of 1,342 files deny an
  unchanged rewrite (all 101 strips and 80 views in this repo among them); under the recommended rows
  411 (evidence 327, I9 epic dirs 27, legacy 56, one RA3 file). Budget 177 fits every store;
  constraints re-level option I (17 `####` to `###`) gives a largest entry of 77, option L 169 with
  the next decision denying — a user row. The 177 floor rests on a mis-measure (83 by the gate's
  count) — row M1 for the user. Every D5 merge variant allows under the drafted 0034. Ledger move
  route: Write at the destination, then `git rm`.
- **Wave-4 constraint recorded:** crate 0.3.0 must not reach a machine before the plugin carrying
  0032–0035 — with the old log its closed world denies edits to every strip and view here.
- Routing: (a) `authoring-epic.artifact-home`'s implement-log clause (field D6, no owner) to S4's
  0027 before its copy · (b) two deny-wording issues (the emptied contracts home named as "the
  nearest home"; a bad run id losing its run-key reason under the root nested-docs row) to wave 3 ·
  (c) neither repo's `.gitignore` carries `.mochiko/runs/`, so every run-folder write denies until
  it does: mochiko's line in wave 3, kinako's in wave 4, and wave 3 checks that the run-open step
  names the line · (e) row H5 re-checked against the landed 0027.
- Slot opened to S4 (S3 idle, not reading). After the landing S3 re-takes what the post-S4 log can
  move, then R1 reviews the table.

## 2026-09-29 · S4 landed 0024–0029 · lead-verified · log settled

- S4 folded routing item (a) into 0027 first (the epic implement-log clause withdrawn, field D6 named
  in the intent; anchor delta D4; re-stamped, scratch set 0 rejecting), then copied the six files,
  ran validate and status, and applied the sweep report's second write (382 lines, 11 Edit hunks,
  dry-run first; byte-identical to the draft).
- **Lead verification, first-hand:** the six sha256s match S4's (`86a0350a…`, `6faccc7c…`,
  `45e41eb8…`, `a3384a24…`, `1db2a01e…`, `7a9f7ebe…`) · `migrate validate --report` `0 rejecting ·
  113 advisory`, clusters 0, 183 allowlist-suppressed edges · `migrate status` `sequences 1..29 (29
  migrations)` · state `sha256:6836ba96…` · 80 documents · 1,136 rules.
- Totals landed: 9 supersede · 10 mint · 36 reword · 1 set-condition · 4 template replace. The
  committed views are stale until S3 regenerates them at the wave close; S5's pin list is in S4's
  report (render.rs floor index, three fidelity.rs tests, the two contract pins, observable.yaml).
- "Log settled" sent to S3 for its post-S4 re-take.
- Second installed-gate false deny on S3 (first on its path): a heredoc writing a scratch probe
  script whose body held a `cp <home ledger> <archive>` string as dry-run payload data; the 0.2.0
  parse read the string as a write. Nothing under any `.mochiko/` written or attempted. S3 wrote the
  scratch script with the Write tool, the payload string built inside the script — a probe harness
  for `check --hook-json`, not the denied write performed another way; accepted, disclosed in the
  report's notes. Lead probe of the same heredoc against the branch binary: `allow`.

## 2026-09-29 · S3 post-S4 re-take: no figure moved · RA8 ruled · R1 dispatched

- Pinned `91b3838` plus 0024–0029 (hashes in the table's `post_s4_retake.pin`). No table figure
  moved: the 21 home views byte-identical before and after S4; unchanged-rewrite verdicts over 1,342
  files byte-identical (737/605 on the committed log, 931/411 under the drafted homes); 194
  destination and next-write checks identical. Row H5 holds (no stop 8). Facts moved and recorded in
  a `post_s4_correction` note: A1 hits 567 to 574; withdrawn-name citations 4 to 1 (the one left,
  `impl.artifact-home`'s `implement-log.md`, is wave 3's).
- **RA8 (marker spelling), lead ruling:** landed 0025's `impl.baseline-entry-grammar` spells the
  field `Lifecycle:` unbolded, while the delta record, R9's record and the gate's exempt matcher
  (`conform.rs:916–921`, `**Name:**` lines only) use `**Lifecycle:**`. The post-ratification 0025
  re-cut — the one re-cut, not an extra file — also aligns every marker field in S4's six files to
  the bold `**Name:**` form (`**Lifecycle:**`, `**Raised:**`, `**Weighed:**`). 0025 is uncommitted
  and unshipped; the re-cut re-stamps and re-validates (RA6 order).
- S3's Q2 row reason (the design truth writer) is partly overtaken by 0024/0027 naming in-place
  writers for design-phase and build-time decisions; the wording is fixed in S3's revision round,
  recommendation unchanged.
- Routed to wave 3: `authoring-technical-requirements.decision-technique-routing` ("only
  feature-scope D-XXX records live here") — read whether it still holds with decisions edited in
  place in the product file.
- R1 (fresh `tech-lead`, wrote nothing this wave) dispatched on the table, per wave plan §4.
- S3's third table write (a notes bullet disclosing the heredoc deny) landed as R1 started; R1 told;
  both census reports frozen until R1's findings.
- S4's grep for the re-cut: unbolded marker fields in 0025 (7 sites), 0026 (2), 0029 (2). Ruled:
  store sites keep the store's own `**Status**: removed` form (colon outside the bold); baseline
  fields take `**Lifecycle:**` · `**Raised:**` · `**Weighed:**`; the constraints stance reads
  `**Status:**`; the sweep report's quotes aligned in the same slot. The exempt matcher reads only
  `**Name:**`, so a store `**Status**:` line is never exempt — S3's store E rows say so in its
  revision round (a template form change or a crate matcher widening in wave 3, or no exemption).

## 2026-09-29 · R1 review of the census table: FAIL (7 blocking, 12 advisory) · all folded

- `reports/w2-census-review.md` (R1, written once after a trimmed dry run). Checks 1 and 5 PASS;
  every sampled verdict matched (19 files, 13 kinako and 6 mochiko, plus 21 destination checks);
  kinako clean; no installed-gate deny. The defects are in what rows decide and say.
- **Lead folds all 19 findings into S3's one revision round; none rejected.** Blocking: B1 a choice
  row for the lane home's drawing (declare `architecture.md`, or scope `impl.design-outputs-home`) ·
  B2 every store row states how its preamble and non-entry text are bounded, with a choice to accept
  it unbounded or bound it by crate work in wave 3 · B3 F2 spelled as ruled, and `text_moved` and E1
  state RA8 as ruled · B4 S5 becomes a choice row (150, a whole-file 177, or entries) · B5 a spine
  element's why on its own row, or an explicit supersession in part of seams R1, put as a choice ·
  B6 a contracts entry heading and api.yaml's marker form (or no marker, with how the diff finds
  changes), carried by the 0025 re-cut · B7 `ledgers/` and `product-baselines/` declared as homes
  with file sets, the root narrowed or N1a's full cost put to the user, L1/L2 re-probed.
  Advisories A1–A12 folded as R1 wrote them; A7 (readability: plain words for every internal code,
  options on every choice row, a short list of choices at the top) weighs most, since the user
  rules from the table.
- **Store field form, lead ruling (B3):** inside the architecture store every field keeps the
  store's `**Name**:` form (`**Status**:`, `**Raised**:`, `**Weighed**:`); the baselines take
  `**Name:**`. Relayed to S4 for the 0025 re-cut.
- Routed to wave 3: A11 (`authoring-architecture-store.store-home` sends seats to spine budgets the
  gate no longer applies, under S3 (a)).
- After the revision R1 re-reviews once (the gate-loop bound); anything still open goes to the user
  with the table.

## 2026-09-29 · S3 revision round done · R1 re-review requested

- Table rewritten once (823 lines; dry-run allowed on the branch root and both installed roots;
  byte-equal to the draft); facts took one Edit (A9). 27 choice rows (from 21), 11 ruled. New at the
  top for the user: `how_to_read`, `choices_at_a_glance`, `ruled_at_a_glance`, `terms`.
- New rows: P1 (the store preamble: unbounded, a wave-3 crate bound at 177 recommended, or 60) · F3
  (a build-raised spine element's why: two columns recommended, one column, or none as a named
  supersession in part of seams R1) · F4 (contracts: the changed clause's heading at its siblings'
  level, `**Lifecycle:**` first; `x-lifecycle` in api.yaml recommended) · N1d (product-baselines as
  a home with file names) · H0a (kinako's 16 legacy files under undeclared names: keep, recommended).
  H4 (lane drawing, declare recommended) and S5 (graduated concern file, whole-file 177
  recommended) became choices. The archive root drops `<slug>.md`: a 400-line report dump and raw
  console output are refused at every archive path (probed).
- L1/L2 (kinako's B53, B61, FEAT-006/reviews) now recommend "keep in place as closed record",
  which amends S11 in part — named as the user's call beside S2 option I (amends D5(iv) in part)
  and F3 (c) (would amend seams R1 in part).
- Consequences unchanged in total (931 allow / 411 refused under the drafts; 737 / 605 today). S3's
  drafts: `s3/probe/log904` (0 rejecting · 113 advisory); nine option logs, each 0 / 113.
- Routed: to wave 3 — P1's crate bound if ratified; the brownfield rule naming a file for
  "structural D-XXX rows" (N1d) · to S4's 0025 re-cut — F1–F4 and the E1 spelling, the spine
  element-table columns (F3) and the api.yaml `x-lifecycle` form (F4).
- R1 re-reviews once; then the table goes to the user.

## 2026-09-29 · R1 re-review: FAIL (B1–B7 and A1–A12 closed; 1 new blocking) · table to the user

- R1 `re_review_round_1` (appended to `reports/w2-census-review.md`): all 19 first-round findings
  closed and re-probed under S3's drafts; every view and strip still allows. New blocking RB1: N1a's
  recommendation (keep kinako's four groom snapshots in place) and the ruled N1d (leaving
  `spine-groom.md` undeclared) amend field D9(b) in part — "sorted into the declared archive/ set
  or out of `.mochiko/`" (record `:369`) — and no row says so. Five advisories: RA1 the glance list
  does not mark the recommendations that amend a user ruling (L1, L2, S2, N1a) · RA2 N1d refuses
  `ARCHITECTURE.md` under its own name (renaming is the route) · RA3 the strips home's door stays
  open in consumer repos · RA4 bare lead-ruling ids ambiguous across seats · RA5 `how_to_read`
  calls 0024–0029 committed.
- The gate-loop bound is reached (one fix round, one re-review): no further revision round. RB1
  goes to the user with the table as a choice, and the lead marks every recommendation that amends
  an earlier user ruling when putting the table.

## 2026-09-29 · census table RATIFIED by the user · phase B sequenced

- Put as a plain list of the 27 choices, the four that amend an earlier user ruling asked apart.
  User: old reports "Keep as frozen (Recommended)" · groom files "Keep as frozen (Recommended)"
  (RB1) · heading levels "Raise them (Recommended)" · "Approve all 23 (Recommended)".
- Recorded: `.mochiko/decisions/2026-09-29-census-table-ratified.md` (amends in part field S11,
  field D9(b), delta D5(iv)); new `DECISIONS.md` row; the field-review and delta rows and both
  brainstorms-index entries annotated.
- R1's RA2–RA5 are wording advisories on the table; the table stands as ratified, and they are
  disclosed here rather than rewritten. RA3 (the strips home's door open in consumer repos) goes
  to BACKLOG at the joint landing unless wave 3 takes it.
- **Sequence (RA6):** (1) S4's one 0025 re-cut — the per-store field set, placement and
  contracts/api.yaml form from rows F1–F4, the E1 spelling, the bold-marker alignment across its
  files — re-stamped, validated, view-diffed; (2) S3's phase B, 0032–0035 from its `log904`
  drafts, validated; (3) S3 regenerates `.mochiko/schema-views/`; (4) S5 plans the pins. Each
  write into `plugins/mochiko/migrations/` takes a slot from the lead.

## 2026-09-29 · S4's 0025 re-cut landed · S3's phase B slot opened

- S4 re-cut 0025 (and the bold spelling in 0026, 0029): the per-store sentence in
  `impl.baseline-entry-grammar` from rows F1, F2, F4 (api.yaml `x-lifecycle`; a removed operation
  keeps its summary and `x-lifecycle: removed`); Raised and Weighed columns in both spine element
  tables and `- **Raised**:` / `- **Weighed**:` in the concern ledger (F3, F2); every baseline
  marker bold, the store's `**Status**: removed`. Counts unchanged. S4's validate: 0 rejecting ·
  113 advisory; state `sha256:158a73a9…`; `impl.sec.tools` 23,282 characters. Sweep report
  re-aligned (389 lines, 3 Edits, dry-run first).
- Lead check: the three changed hashes match S4's (`1ec1bcb8…`, `b12a1ddc…`, `cb31a9a4…`); 0024,
  0027, 0028 unchanged; no unbolded `Lifecycle:` / `Raised:` / `Weighed:` left in the six files.
  The lead's own full validate runs after S3's four land.
- Gap noted, not a stop: the table places no marker for the design baseline's truth part (Q2 keeps
  section budgets; its writer is the rehoming brainstorm's), so only the grammar rule's generic
  sentence applies there — carried to the rehoming item with A2's tension. S3's wave-3 nit: `home`
  prints "bounds: per template section" for the architecture home whose files are per entry.
- S3's phase B finalised in scratch (0032 grammar 1, 0033 and 0034 grammar 2, 0035 grammar 1;
  bodies byte-identical to the tested drafts; 0 rejecting; the 1,342-file rewrite check identical
  to the table's). Slot opened to S3.

## 2026-09-29 · S3 landed 0032–0035 · lead-verified · views and S5 dispatched

- S3 copied the four (each byte-equal to its source): 0032 `b2928ae7…` · 0033 `cc8cd5bf…` · 0034
  `61e8ee15…` · 0035 `90345929…`. Its in-place A6 re-run over 1,342 files: 931 allow · 411 refused,
  every verdict, reason and fault identical to the ratified table's consequence column — spine.md
  and concerns.md unmoved by the 0025 template re-cut. Kinako clean; only the ten new migrations
  untracked under `plugins/`.
- **Lead verification, first-hand:** the four hashes match · `migrate validate --report` `0
  rejecting · 113 advisory`, clusters 0 · `sequences 1..35 (33 migrations)` · state
  `sha256:79aadc14…` · 87 documents · 1,136 rules.
- Next: S3 regenerates `.mochiko/schema-views/` through `views emit` (the lead diffs a fresh emit);
  S5 (`staff-engineer`) spawned plan-only for the pins, P5 grades.
- S3 regenerated the views (no installed-gate deny): 87 documents — 7 homes added, 9 homes and 16
  command/skill/template views changed, none removed, no stale file. **Lead check:** a fresh emit
  to scratch, `diff -rq` against `.mochiko/schema-views` — identical; 87 files. S3's wave-2 work is
  done.

## 2026-09-29 · S5 plan returned (plan-only, clean) · Q1–Q5 ruled · to P5

- S5's plan: scratch `s5/plan.md` (229 lines), every value read from the landed files or the branch
  binary. Pins moving: `fidelity.rs` ×3 · `validate.rs` census (80/332/1135 to 87/333/1136) ·
  `views.rs` ×3 (80 to 87) · `render.rs` floor index, widest line and six template fixtures ·
  `matrix_similar.rs` command family and opt-in corpus · `evals/contract/run.py` implement set ·
  `expected-skills.json` ×2 · `observable.yaml`; `RETIRED_SIDECAR_ANCHORS` 3 to 8.
- Ruled: **Q1** the contract suite's G-SIZE and G-AMNESTY fixtures break under 0034's entry budgets
  (S5's dry run) — in S5's set as a moved census figure; the tests keep asserting a size deny, only
  the fixture size and expected wording move (pad 40 to 200, "per-entry bound of"). The suite itself
  runs at wave 3's gate · **Q2** `impl.baseline-entry-grammar` and `impl.base-pins` both
  `observable` in `observable.yaml` · **Q3** the five 0013 `impl.*` ids missing from
  `observable.yaml` since before this wave stay out of scope; BACKLOG at the joint landing · **Q4**
  one re-key sentence in `expected-skills.json`'s provenance, yes · **Q5**
  `MOCHIKO_FULL_SIMILAR=1 cargo test --test matrix_similar` joins the lead's wave-2 gates.
- P5 (fresh `staff-engineer`) grades; S5's single cargo slot (E1–E3: a run before the edits, one
  after) opens only on a PASS.
- **P5: FAIL, 1 blocking, 4 advisory** (`p5/verdict.md`; plan hash verified). Blocking (item 5):
  the post-edit run has no attempt bound — fix: any E3 red is a stop to the lead with its output, no
  re-edit and no second cargo run inside the grant; drop "two slots also work". Pin completeness
  verified first-hand; the G-SIZE/G-AMNESTY re-key does not weaken either row.
- **Routed to the wave-3 plan** (BACKLOG if wave 3 declines): A1 — plan-eval content 0024/0026/0035
  made stale, outside S5's field: `evals/plan/feature/evals.json:7,11,35` (asserts a
  `baseline-delta.md` and "the graded fold"), `evals/plan/feature/observable.yaml:52,53,61` why
  texts, `evals/plan/implement/evals.json:7` and `observable.yaml:33`, and fixtures carrying files
  0035 withdrew (`evals/plan/feature/fixtures/s3-groom-at-cap/…/FEAT-004/baseline-delta.md`,
  `evals/review-plan-artifacts/fixtures/g2-cancellation-refunds/…/FEAT-034/{data-model,
  constraints-and-decisions}.md`) · A4 — a pre-existing crate test defect: `cli.rs`
  `shipped_log()` resolves `<repo>/migrations`, gone since the log moved, so
  `the_shipped_log_renders_every_section_of_every_primitive` and
  `the_shipped_log_is_reachable_through_the_binary` return early and skip silently.
- Re-plan 1 for S5; P5 re-reviews.
- S5 re-plan (264 lines, sha256 `98481214…`, lead-verified): the §6 attempt bound (any E3 red is a
  stop to the lead with its output; a repair takes a new slot), one slot for E1–E3, A1 handed to
  the lead. **P5 re-review: PASS**, 0 blocking, 0 new advisory. `plans: S5:PASS(2)`.
- **GO S5 with its one cargo slot** (E1–E3). S3 and S4 idle, no other cargo in flight. S5's crate
  test edits take an independent non-author code review (`rust-cli.md`, GI-004) before the wave
  closes; that reviewer also checks the command-family `scored` figure against S5's E1 output (A3).

## 2026-09-29 · S5 done (E1–E3 green) · lead gates green · G2 reviewing

- S5, one cargo slot: E1 failed exactly the 12 predicted tests on their predicted values; 14 files
  changed (five crate test files, six template fixtures, `run.py`, `expected-skills.json`,
  `observable.yaml`); E3 `cargo test --all` 570 passed · 0 failed, the opt-in `matrix_similar` 48
  passed, rustfmt clean on its files, `check-rubric implement` uncovered only the five 0013 ids
  (Q3). No stop, one post-edit run.
- **Lead gates, run first-hand:** `cargo test --all` 570 passed · 0 failed ·
  `MOCHIKO_FULL_SIMILAR=1 cargo test --test matrix_similar` 48 passed · `cargo fmt --all --check`
  exit 0 · `cargo clippy --all-targets -- -D warnings` exit 0 · `cargo audit --deny warnings` exit 0
  (31 dependencies; re-run alone for its own exit code) · `mochiko-cli 0.3.0 · grammar 1..2` ·
  `sequences 1..35 (33 migrations)`, state `sha256:79aadc14…`, 87 documents, 1,136 rules ·
  `migrate validate · 0 rejecting · 113 advisory` · views: a fresh emit identical to the tree.
- G2 (fresh, `general-purpose` on `opus`, read-only, no cargo) reviews S5's crate test diff per
  `rust-cli.md`.

## 2026-09-29 · G2 PASS · wave 2 closed

- **G2: PASS**, 0 blocking, 2 advisory (`g2/review.md`). Every pin re-measured with the branch
  binary: 87 documents; sequences 1..29 and 32..35; the census; the five retired sidecar anchors are
  exactly the sidecar-anchored ids among the nine superseded; `IMPLEMENT_FLOORS` equals the rendered
  floors; all 24 template fixtures byte-identical to the binary's output; the command-family figure
  equals S5's E1 line; the corpus arithmetic holds; the G-SIZE/G-AMNESTY re-key needed (pad 40 now
  allows) and not weakening; write set exactly the 14 files.
- Routed to wave 3 with P5's A1: G2 A1 (`observable.yaml:32`, `:44`, `:53` why texts stale, no
  score impact) · G2 A2 (`run.py:4500` docstring tense).
- Pre-commit scan over the 67 changed and new paths (email, token and key patterns): no hit.
- **Wave 2 carries to wave 3** (from this log): the store preamble bound (P1) and `settle`'s
  first-fault-per-key defect (RA3) and the two silently skipping `cli.rs` tests (P5 A4) — crate
  work · the stale plan-eval content and fixtures (P5 A1, G2 A1–A2) · `impl.artifact-home`'s
  `implement-log.md` · `authoring-technical-requirements.decision-technique-routing` ·
  `authoring-architecture-store.store-home`'s spine-budget pointer (R1 A11) · the brownfield rule's
  file name for structural D-XXX rows (N1d) · two deny-wording issues and the `home` "per template
  section" line · the `.mochiko/runs/` `.gitignore` line (mochiko now; kinako at wave 4) · the
  migrations README (grammar 2) · the prose and ceremony already planned. To the design-truth
  rehoming item: the designer-only-writer tension (S4 A2) and the truth part's marker placement. To
  BACKLOG at the joint landing: the five 0013 `impl.*` ids missing from `observable.yaml` · the
  strips home's consumer-side door (R1 RA3) · a `withdraw-document` op candidate.
- **Wave-4 constraint:** crate 0.3.0 reaches no machine before the plugin carrying 0032–0035.
- `floor: held · seats: S3, S4, S5 / P3, P4, P5, R1, G2 · plans: S3:PASS(2) · S4:PASS(2) ·
  S5:PASS(2)` — no single-writer breach; every write into the log under a lead slot; the census
  review loop reached its bound and its last finding went to the user, who ratified the table.
- Wave 2 is uncommitted on `joint-hook-delta`; the commit is put to the user.
