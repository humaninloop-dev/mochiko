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
