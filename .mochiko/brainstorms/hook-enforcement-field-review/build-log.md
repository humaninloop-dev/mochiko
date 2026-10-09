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
- State read before planning: governance v3.2.0 (AM-5-field-review-fold) already carries this gate's GI-019-kernel-tooling-admission text and
  the GI-012-release-gates-module exception row the bump ships under (R4); plugin 0.115.0 took migrations 0019–0023;
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
  passwords) — any hit redacted and disclosed (GI-003-repo-secret-hygiene) · Q4 agreed, the 23 out-of-set denies are a
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
  The **`## Header` signature limb** was ruled (`hook-enforced-artifact-schema` D9-stateless-deny-discipline, record :543)
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
- **Lead ruling on the no-stat gap:** accepted and disclosed, no directory stat (a read GI-019-kernel-tooling-admission
  does not list). The carve never bounded content in the first place — `cp report.md
  runs/<id>/x.txt` already lands a report's text under a non-`.md` name, the limit control 4 states
  ("the gate cannot see what a shell write puts in a file") — so the gap adds no new exposure.
- Next: G1 (plain `general-purpose`, `model: opus`) reviews the whole wave-1 diff.

## 2026-09-29 · G1 code review round 1 — FAIL (2 blocking, 10 advisories) · fix round ruled

- G1 ran the four layers itself (556 passed; fmt, clippy, audit exit 0), `migrate validate` 0
  rejecting, views identical, and drove HEAD's 0.2.0 binary beside the new one on write-shaped
  commands. All S1 clauses PASS; S1's not-red-first tests confirmed to fail at HEAD; `shell.rs`
  intact after the fmt incident; no read outside GI-019-kernel-tooling-admission's list, no network call.
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
  test edits take an independent non-author code review (`rust-cli.md`, GI-004-primitive-audit-ratchet) before the wave
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

## 2026-09-29 · wave 2 committed · wave 3 planned, approved, opened

- User: "Commit and plan wave 3". Wave 2 committed `5558fd7` (67 files; the scan clean). Nothing
  merged to `main`.
- Plan: [wave3-joint-rewords-ceremony.md](wave3-joint-rewords-ceremony.md) — S6 crate fixes · S7
  rule rewords 0036–0039 · S8 prose, hooks, README, strips · S9 pins, evals, contract suite (after S6
  and S7) · S10 the pre-ruled governance strike PATCH v3.2.1 · G3 crate review · V1–V3 the gate
  audit · the lead's ceremony and landing; the bump 0.116.0 under the AM-5-field-review-fold exception row.
- User at plan approval: the "smaller two" → "Base + diff fingerprint (Recommended)", recorded as
  seam R10 (`.mochiko/decisions/2026-09-29-uncommitted-evidence-citation.md`, new `DECISIONS.md`
  row) · "Approve and start (Recommended)".
- S6 (`staff-engineer`), S7, S8, S10 (`tech-lead`) spawned plan-only; fresh peers grade.
- S10's plan (`s10/plan.md`, 184 lines): the trigger fires at 0.116.0; E1–E12 in the ledger, E13
  `rust-cli.md`'s build-state parenthetical, E14 `CLAUDE.md`'s Ratified line; all PATCH. Drift
  found beyond the three known: (d) the sniff places listed where the binary sniffs every declared
  sub-directory without its own home, `features/desk/` included · (e) "never wedged" false for an
  undeclared sub-directory · (f) the stated-limits pointer stale (evasion 3 open end to end under the
  hooks' `if: Bash(*.mochiko*)` narrowing; a command-substitution shape; no `.git`, no gate) · (g)
  the crate test matrices omit `tests/shell.rs`. Depends on S6 landing first (RA3, P1). One
  installed-gate false deny on a read (`sed -n …; echo`), not retried.
- Ruled: Q1 the PATCH states evasion 3's end-to-end gap; no `hooks.json` change this wave; BACKLOG
  with the latency/coverage trade · Q2 the build-state line kept as a struck note (the v3.0.3 idiom)
  · Q3 the missing whole-file bounds taken as drift into `rust-cli.md` · Q4 no GI-019-kernel-tooling-admission text for the
  SessionStart listing unless the ledger enumerates what that hook prints.
- S10 v2 frozen (203 lines, `79484692…`); P10 grading.
- S7's plan (`s7/plan.md`, 246 lines): 0036 field D4 (mints `impl.run-folder` — R5, R6, the
  `.gitignore` line — and `impl.evidence-citation` — R10; `testing-end-user` rewords) · 0037 field
  D6 (`impl.artifact-home` — R1, S1; `decision-technique-routing`) · 0038 field D8 (mints
  `testing-end-user.pre-write-dry-run`) · 0039 field D2 (per-entry bound pointers) · 0040 field D3
  (N1d names). Probes: all seven render targets resolve (F8 closed by the wave-1 resolver, no
  reword); R2's final-validation line already in 0026; the R10 fingerprint (`git diff --binary`
  plus untracked paths and `git hash-object --stdin-paths`, hashed) is repeatable, covers untracked
  files and only reads. One installed-gate false deny on a read (`grep -n -i … | head`), not retried.
- Ruled: Q1 slot 0040 granted (wave plan §1 amended) · Q2 the D8 dry-run clause also in
  `impl.reports-envelope`, building on 0027's text · Q3 N1d's D-XXX source named
  `constraints-and-decisions.md`, the home replace in 0040 inside S7's set · Q4 `arch.artifact-home`'s
  stale spine pointer rides 0039 · Q5 the epic home has never declared `tasks.md` (0005) while
  `patterns-vertical-tdd.artifact-home` sends cycle cards to "the declared feature or epic home";
  kinako's epics never wrote one; neither record ruled on it — BACKLOG, for the epic design's owner ·
  Q6 the run-open step adds the `.gitignore` line when absent (field S2 fold governs).
- S7 v2 frozen (253 lines, `762f1ea1…`); P7 grading.
- S6's plan (`s6/plan.md`, 201 lines): P1 the preamble counted as one entry at `entry_max_lines`,
  key `size:preamble` (kinako's five preambles 29/25/12/42/63, matching the census) · RA3
  reproduced read-only (the unchanged driver-fix report denies, "52 lines against 15"); fix pairs
  standing and new faults by rank within a key · A4 re-pointed, the silent skip removed (a
  read-only pre-probe renders all 287 blocks, exit 0) · deny wording: an empty home never named as
  route 1, its title printed; `.mochiko/runs` paths resolve only against the runs home, restoring
  the run-key reason; D9's evidence ask already met in wave 1; `home` prints "per deliverable" ·
  R2 `((` read as arithmetic only when its inner `)` closes right before the outer (bash and zsh
  agree) · G1 R1/R3/R4/R5/A8 to BACKLOG · no version move for the unpublished 0.3.0. Two cargo
  slots, predicted red counts, any unpredicted red a stop.
- Ruled: Q1 take c′ (the `reports/evidence/` reason) · Q2 `form: log` preambles to BACKLOG (outside
  P1's ratified scope) · Q3 rank pairing denying a new over-budget section under an existing
  heading confirmed as the D4e reading; slot B's sweep must show no unchanged rewrite flipping to
  deny in either tree.

## 2026-09-29 · S8 plan returned · Q1–Q6 ruled · O2/O3 routed as graded addenda

- S8's plan (`s8/plan.md`, 399 lines, plan-only): 11 skills (15 files), `seat-reminder.sh` and
  `session-start.sh`, the migrations README, 12 strip files at [v0.116.0] (one new:
  `migrations-readme.md`); no agent edits. One installed-gate false deny on a read (`sed -n …; ls`),
  not retried.
- Ruled: Q1 the third Q8 prose skill is `patterns-technical-decisions` (it writes D-XXX content into
  `constraints-and-decisions.md`; `patterns-system-design` writes no baseline) · Q2 option A: the
  entity template re-levels to `### Entity:` entries under `## Entities` with the `**Lifecycle:**`
  first line, the brownfield tags and the Status column out, superseded by delta D1/D2 and census S1
  (both v0.27.0-KEPT, so recorded supersessions); S8's set widens by
  `patterns-entity-modeling/scripts/validate-model.py`, the one `Entity:` regex · Q3 the reminder
  line carries D8's dry-run sentence with the run-scratch route; S9 re-pins `REMINDER_GOLDEN`
  (`evals/contract/run.py`) · Q4 agreed: the lead relays S7's landed `truncation-bounds` /
  `cleanup-protocol` text and S8 writes item 3.8 after S7 lands · Q5 the lead owns the budget
  ledger: S8 reports each skill's body delta (characters of the parsed value) with its obligation;
  the lead sweeps every wave-touched skill at the quiesced tree with the canonical snippet, names the
  overage components (render per migration into V1/V2, body into V3) and writes the rows at the
  ceremony (`.mochiko/memory/primitive-cost-budgets.md` joins the lead's set, wave plan §1) · Q6 the
  README goes last, after S6's diff lands.
- Routed: O2 (replay keeps the last file's grammar, so `migrate status`, `home` and the
  session-start line print grammar 1 over a grammar-2 log) to S6 · O3
  (`impl.fail.unmarked-baseline-write` reads a run's own summary-table or index-row hunk as
  unmarked, against delta D2) to S7. Each goes in as a separate addendum file, graded by the same
  peer after its current verdict; the frozen plans stay frozen. O4 (`mochiko-cli home` in the
  reminder needs `--plugin-root` or `MOCHIKO_MIGRATIONS` outside the plugin root; this predates the
  wave) to BACKLOG · O5 already with the rehoming brainstorm.

## 2026-09-29 · addenda frozen · P10 FAIL on S10 (1 blocking) · fix round 1

- S7's O3 addendum (`s7/plan-addendum.md`, 40 lines, `fdff066e…`): `impl.fail.unmarked-baseline-write`
  (0026) keeps id, class, kind and anchor and gains a third exception under delta D2 I3 — a sweep hunk
  outside every entry is excused only when each changed line names an id the same diff marks for the
  run's key, or is a count a recount at the diff's head confirms; every case that failed before still
  fails. Rides 0037 (field D6, delta D2 named second, the 0027 precedent). +309 characters on the
  implement command's fail render only; no skill payload moves. For P7 after its verdict.
- S6's O2 addendum (`s6/plan-addendum.md`, 39 lines, `001e08d0…`): `replay.rs:261` assigns each
  file's grammar in turn; the fix keeps the maximum. The range check stays per file at parse
  (`migration.rs:578`, exit 3), so a 0.2.0 binary still halts on 0033. `src/replay.rs` and
  `tests/replay.rs` join S6's set; two of S6's own pins move 1 to 2; slot A predicts 13 red. For P6
  after its verdict.
- P10 on S10 (`p10/verdict.md`): FAIL, 1 blocking, 11 advisory, 25 claims checked (1 inexact,
  cosmetic). B1: plan §4 re-words the governance text to fit whatever S6 lands, where §6 stops — a
  new repository read is the conformance tier's Fail limb, and describing it would widen the
  admission (MINOR at AM-5-field-review-fold, never a PATCH); a P1 built otherwise contradicts the ratified census row;
  RA3 has no not-landed branch; the re-read set omits `home.rs` and the `Cargo.toml` version line.
  Items 1–4 and 6 pass; E1–E14 are PATCH-class. Read-only probes: the report sniff's reach confirmed;
  "never wedged" false today (census L3); evasion 3 closed at the decision, open end to end.
- Fix round 1 sent to S10: B1 as P10 states it (one stop before any write), with A1, A3, A4 and A7
  folded. A4: the Reach line on eval workspaces must be true after S9's `.git` fix, so S10 sequences
  that line after S9 lands or states the conditional. P10 re-reviews; a second FAIL goes to the user.
- S8 v2 frozen (443 lines, `9b2e9a12…`), Q1–Q6 folded; linter baseline on the current template read-only
  (`['User', 'Session']`, 8/8). P8 (fresh `tech-lead`) grading.
- P7 on S7 (`p7/verdict.md`): FAIL, 4 blocking, 8 advisory. B1 the plan re-points full-log pointers
  into the run folder, against D4 as amended (record `:486`, "never a 'full log' path"; R10) — lead
  confirmed at the record · B2 two planned texts call the run folder "never a declared home" where 0033
  declares `runs` · B3 0039 gives every store file a per-`##` bound, taking in `concerns/<AX-ID>.md`
  (whole-file 177, census S5b) · B4 no attempt bound cited (wave plan §6). Verified clean: F8, R2 in
  0026, R10's fingerprint (hash stable, index untouched, untracked covered, ignored excluded), 0027 as
  base, prefix order. P7's Skill call failed twice (the classifier returned no verdict); it read the
  pair by path. Fix round 1 sent to S7, the O3 addendum folded into v3 so P7 grades one plan.
- S10 v3 frozen (252 lines, `ccd92ed5…`), B1 and A1–A11 folded; P10 re-review **PASS**
  (`p10/verdict-r2.md`), 5 new advisories (N1 the §4 stop reads S6's whole `src/` diff, `render.rs`
  included · N2 pin §4 to the post-G3 diff · N3 list the expected history hits · N4 E5's antecedent ·
  N5 E6b's qualifier), folded as execution notes. GO held: S10 writes only after S6 lands and G3
  passes. Its landed diff takes a fresh grader (V4, wave plan §6), since a PATCH is the user's to
  approve (ledger amendment policy) and the author does not grade it.
- S7 v3 frozen (`plan-v3.md`, 335 lines, `a3cdafdf…`), B1–B4, the O3 addendum (§3b, 0037 op 3) and
  A1–A7 folded; P7 re-reviewing. P7's A8 (246 against 253) needs no change: 246 was v1, 253 the v2
  it graded.

## 2026-09-29 · P7 re-review FAIL (second) · to the user · seam R11 ruled · S7 GO for 0036–0040

- P7 on S7 v3 (`p7/verdict-r2.md`): B1–B4 closed and verified; 2 new blocking, both in §3b (the O3
  addendum, graded for the first time): N1 the per-line test has no domain, "another level" takes in
  `####` sub-headings (the crate's `entry_spans` runs an entry to the next heading at its level or
  above, `conform.rs:903–906`), and `home` prints no entry level for three baselines · N2 "nothing
  leaves" is false, and D2 I3 supports the class but rules no exception to D3's test, which makes it
  a reconciliation of two user rulings, the kind R8 was. Advisory: AR1 0036 op 5's brief fingerprint
  could go stale · AR2 0037 would carry two rulings · AR4 V2's range.
- To the user (a second FAIL): rule it now, defer to BACKLOG, or narrow D3 to lines inside entries.
  Ruled **"rule it now"** as recommended: seam R11
  (`.mochiko/decisions/2026-09-29-sweep-hunk-exception.md`; `DECISIONS.md` row; delta row and index
  entry annotated). Its own record because the seams record sits at 146 of 150 lines.
- Consequences: §3b leaves 0037 (AR2 closed) and becomes its own migration 0041, anchored to delta D2
  and D3 with R11 named in the intent (wave plan §1 widened). S7 writes the 0041 section to R11's four
  conditions; P7 grades that section once, and a FAIL goes to the user. V2 now covers 0032–0041 (AR4).
- S7 GO for 0036–0040 as v3 without 0037 op 3, AR1 folded (the fingerprint computed when the report
  is written, never at dispatch). One migrations slot open now; S6 runs no cargo while it is open.
  Views regenerate after 0036–0040 and again after 0041.

## 2026-09-29 · P6 FAIL on S6 (2) · P8 FAIL on S8 (7) · fix rounds 1 with lead rulings

- P6 on S6 (`p6/verdict.md`): FAIL, 2 blocking, 6 advisory. B1 slot A is fail-fast and tailed, so at
  most one test binary's reds show · B2 the sweep's before and after runs read a moving log (S7's
  0036–0040) and tree, and slot B's build overwrites the E0 binary. Verified: RA3 reproduced; a full
  sweep of 1,356 files (931 allow, 411 deny, matching S3's A6); R2 matches bash and zsh.
- Ruled for S6's round: fold its O2 addendum into v3 so P6 grades one plan · B2 by a scratch copy of the
  E0 binary run beside the new one on one log snapshot and one file list · slot A runs after S7's slot
  closes, its reds predicted against the post-0040 log · A1 (a same-heading swap allows): Q3 is
  re-affirmed — sections sharing a heading are told apart by rank only, so a write that grows no
  rank's standing fault and adds none settles, as D4e reads; the plan's "growing any section denies"
  claim is corrected, not the design · A5 the P1 reason prints the store's own entry level.
- P8 on S8 (`p8/verdict.md`): FAIL, 7 blocking, 9 advisory, 26 claims checked (2 false or incomplete).
  B1 sweep misses (entity `SKILL.md:298`, `RELATIONSHIP-PATTERNS.md:63`, the spec-home data-model paths
  in `ARTIFACT-CHECKLISTS.md`, ATR's "no gaps", `REPORT-TEMPLATES.md` full-log lines) · B2 the
  per-feature quickstart cap and presence check survive · B3 item 3.1 restates `lifecycle-statuses` ·
  B4 `patterns-technical-decisions:56` mirrors a rule 0037 rewords · B5 the runs listing is unbounded
  (15,000 folders take 11.2 s under `/bin/sh`) · B6 "never into a home" is false for `runs` · B7 stale
  lines outside every seat's row.
- Ruled for S8's round: B2 needs no user question — v0.23.0 T3's ≤150-line cap and presence check bind
  a per-feature `quickstart.md` that delta D4/D6(b) withdrew (census H5), and the product file's bound
  is census Q1 (177 per `##` entry, user-ratified), so both leave by recorded supersession citing those
  rulings · B5 one reminder line per field OQ5 ("a `SessionStart` reminder line"): the count of run
  folders and at most five names, the version line printed first, measured under `/bin/sh` at 15,000
  folders to finish within 1 s (wave plan §4's "every" amended) · B7 S8's set widens by
  `templates/constitution-modules/knowledge-management.md` (the pre-0025 orphan rule, `:133–136`),
  `templates/report-format.md` (`:7`, "the design-phase deltas") and `commands/architecture.md`
  (`:52`, the orphan wording); each re-points to `authoring-architecture-store.orphan-rule` as 0025
  worded it, with strips, and the command edit makes V3's unit a command pair · B4 waits for S7's
  landed 0037 text, like 3.8 · B6 "never into any other home", with S9 re-pinning the corrected line.
- S6 v3 frozen (`plan-v3.md`, 263 lines, `2e919802…`), the O2 addendum folded as §2 I7, B1/B2 and A1–A5
  folded, A6 declined with a reason. Lead execution note on A2: past the 64-answer budget, `((` falls
  back to the subshell reading, so the gate fails closed and sees the write, where v3 said arithmetic
  (fail-open). P6 re-reviewing v3 with the note. S6's reading: the guard carries a `tee HOME` target
  so the deny can show; the new test `a_double_paren_past_the_lookahead_budget_reads_as_subshells`
  (three rows); slot A's predicted reds move from 13 to 14.
- S8 v3 frozen (`plan-v3.md`, 687 lines, `bfb86ca1…`), B1–B7 folded; the scratch pre-probe of the
  runs line under `/bin/sh` took 0.245 s at 15,000 folders; the corrected reminder line is 352
  ASCII characters. Ruled: A1 S8's set widens by `validate-model.py:41–45` and `:144` (stale with the
  regex) · the KM template's `:182` checklist twin accepted · O6 mochiko's pinned
  `.mochiko/memory/knowledge-management.md:67–70` (the same pre-0025 orphan line) to the lead's
  ceremony set, kinako's copy to wave 4. P8 re-reviewing.
- P8 re-review **PASS** (`p8/verdict-r2.md`): B1–B7 closed, 1 condition (C1, the linter docstring and
  message per ruling (a)), 7 advisories, 22 claims checked with none false. Its scratch probes: the
  version line first in every case; 0.465 s at 15,000 folders under `/bin/sh` (0.34 s of it `migrate
  status`); `home .mochiko/runs/FEAT-001-run5` prints `home: runs`; the linter case 1 gives 8/8, the
  controls 7/8. S8 GO with C1 and A1–A7 folded; 3.8 and `patterns-technical-decisions:56` held for
  S7's landed text, the README for S6's diff.

## 2026-09-29 · P6 re-review FAIL (second) · to the user · S6 GO with fixes as conditions

- P6 on S6 v3 (`p6/verdict-r2.md`): B1's mechanics and B2 closed; 2 new blocking, 3 advisory. N1 the
  baseline set counts any red in a test S6 "does not touch", but S6 edits shared helpers (`tests/hook.rs`
  `state`, split to `state_from`, used by 29 tests; `cli.rs` `shipped_log()`), so an S6-caused red could
  pass slot B as S9's · N2 v3's only guard is write-free and pins neither fallback; the fail-closed
  row belongs in the already-red I5 test. Graded sound: I7 (the range check still halts 0.2.0 on 0033),
  gate 0 against the landed log (1..35, `79aadc14…`) and S7's staged `s7/log/` (1..40, `73483c82…`,
  1139 rules), the A2 bound under the fail-closed note.
- To the user (a second FAIL): go with the fixes as conditions, one more plan round, or pause S6.
  Ruled **"go with fixes"** as recommended. S6 GO on v3 with N1 (the baseline holds only reds in test
  files S6 does not edit; any unpredicted red in its six is a stop), N2 as P6 wrote it (the row
  `"((a); " × 65 + "((echo a); tee HOME)"` → `[HOME]` in I5, the 20,000-opener guard at `[]`, S6's
  boundary rows allowed in I5; 13 reds by name), and R2-A1–A3 binding (slot B re-checks gate 0's state
  hash, and 0041's slot opens only after slot B closes). G3 reviews the diff as planned.
- P6's r3 note (`p6/verdict-r3.md`, after the ruling): S6's reading of the A2 note closes N2 — the
  boundary rows traced exact against `src/shell.rs` (×63 → `[]`, passing today; ×64 → `["2"]`, red
  today; 20,000 around `tee HOME` → `[HOME]`, red today), and no existing test spends the budget; N1
  stays open on paper and is closed by the binding condition. The rows ride I5 (13 reds by name), and
  the disclosure wording is corrected to "64 non-arithmetic `((` answers in one `tokenize` call".

## 2026-09-29 · S7 landed 0036–0040 · lead-verified · S6 gate 0, S8's holds released, P7 on 0041

- S7 landed 0036 (87 lines, `a22e822e…`) · 0037 (43, `bc4fb3c1…`; no op 3, no delta-D2 intent) ·
  0038 (39, `a63f2897…`) · 0039 (36, `178cc8e2…`) · 0040 (41, `671aeb0f…`), each byte-equal to its
  scratch copy; every prefix 0 rejecting · 113 advisory · clusters 0. Views: 6 files, +87/−23; 3 rules
  added (`impl.run-folder`, `impl.evidence-citation`, `testing-end-user.pre-write-dry-run`), 9 texts
  changed, none removed; allowlist-suppressed edges 183 to 182, no new unsuppressed edge.
- Lead-verified first-hand (branch binary): `sequences 1..40 (38 migrations)` · `state
  sha256:73483c82a7d916302e6446534f93523c76394ad7bfa4b871a68921061299956a · 87 documents · 1139 rules`
  · `0 rejecting · 113 advisory` · a fresh emit identical to `.mochiko/schema-views/` · the five hashes.
- Render deltas for the budget sweep: `testing-end-user` `sec.output` +794 (render 12,353 to 13,147;
  its standing overage grows from +239 to about +1,033, a new obligation under field D4/D8) · ATR
  `sec.scope` +158 · store `sec.artifact` +142 · the implement tools section 23,282 to 25,469 (ceiling
  30,000).
- Disclosed by S7: its rows diff ran `uv run --with pyyaml`, which may have fetched pyyaml from PyPI —
  a public package fetch carrying no personal identifier; no repo effect. Seats keep to installed tools.
- Relayed: S6 the landed hash (gate 0, then slot A granted on its pass; slot B on an explicit grant) ·
  S8 the landed `truncation-bounds` and `cleanup-protocol` texts, items 3.8 and `:56` released · P7 to
  grade `s7/plan-0041.md` (60 lines, `696bad7e…`) once, a FAIL to the user; S7's Q7 (spine's marker is
  the row's Status cell) inside that grade.
- S8 phase 1 landed (before the relay reached it): 20 plugin files, +196/−148; 12 strip files, 44
  entries at [v0.116.0]. Held: 3.8, `:56`, the README. Bodies (characters, canonical snippet): store
  5,391 to 5,797 · epic 3,129 to 3,200 · system-design 9,583 of 11,047 · entity 14,592 of 16,835 ·
  api-contracts 12,278 of 13,412 · technical-decisions 5,377 of 5,783 (before `:56`) · router 47,358
  (unbudgeted). C1 done, with two same-format comments (`:47`, `:102`) accepted; A1: no delivered rule
  governs NFR- numbering, so the cell cites none and the strip says so. Linter proof decisive (case 1
  8/8; the controls 7/8), taken on scratch copies, though after the template edit (a disclosed order
  slip). A `py_compile` check made a gitignored `scripts/__pycache__/`, removed. Hook probes: exit 0
  in every case, the version line first, 15,000 folders in 0.470 s; the reminder line 352 characters.
- P7 on plan-0041 **PASS** (`p7/verdict-0041.md`), 0 blocking, 7 advisory: the text states R11's four
  conditions and no wider (342 to 1,108 characters); `home` prints `###` for data-model and constraints,
  `##` for quickstart, spine and concerns, and `entry_spans` (`conform.rs:903–955`) agrees; the intent
  names what leaves; S7's Q7 answered (spine's row-Status marker satisfies conditions 2–4; no text
  change); only `fidelity.rs`'s sequence list moves. A1–A4 folded at write time (the intent's condition 4
  wording, an entity's id is its heading name, fenced `#` lines are not headings, results taken after
  slot B). To BACKLOG: a count inside `## Elements` fails before and after (outside R11); hunks
  straddling an entry and a sweep row; another open key's own sweep rows. 0041's slot follows S6's slot B.

## 2026-09-29 · S8 phase 2 landed · lead budget sweep · V1, V3a, V3b dispatched

- S8 phase 2: item 3.8 (`EVIDENCE-CAPTURE.md` to `$RUN/`, the `rm -f` pair withdrawn;
  `REPORT-TEMPLATES.md` full-log pointers withdrawn, the v0.44.0-KEPT prose-on-clean check kept) and
  `patterns-technical-decisions:56` (matching 0037's landed `decision-technique-routing` clause for
  clause), against state `73483c82…`. One line beyond the plan list accepted: `REPORT-TEMPLATES.md:47`,
  the `evidence` path field, twin of `:29`. Totals: 37 files, +1,340/−186; 14 strip files, 51 entries at
  [v0.116.0]. testing-end-user body 9,596 to 9,593; technical-decisions 5,118 to 5,554 of 5,783. The
  hook probes re-run at this state: exit 0 throughout, 15,000 folders in 0.470 s. The README waits for S6.
- Lead budget sweep (`scratchpad/lead-budget/components.md`; canonical snippet plus each `!` block
  rendered with the branch binary; testing-end-user's render matched S7's 13,147 exactly). Prefixes
  ending at 0024 or 0025 do not validate alone (5 rejecting at 0024, forward cites 0026 resolves), so
  0024–0026 are one component. Over budget, components summing exactly: store +2,058 · review-sufficiency
  +1,377 · testing-end-user +1,030 · ATR +926 · feature-map +375 · epic +141 · sound-loop +208 ·
  gap-finding +3,196; each carries its standing ruled overage plus this wave's renders (0024–0026,
  0027–0029, 0036–0039) and S8's bodies (store +406, epic +71, testing-end-user −3). Body-only
  budgets and descriptions all under. Re-run at the gate after 0041 and S6's diff.
- Dispatched (fresh plain seats, `opus`; the contract rendered from the branch with the branch binary,
  never through the Skill tool, which would render the installed 0.112.0 copy): V1, schema content
  0024–0029 with its render components · V3a, the skill pairs (store, epic, ATR, testing-end-user with
  its references, technical-decisions, the router) with the body components · V3b, system-design,
  entity-modeling with its linter, api-contracts, the two checklists, the KM template, report-format,
  the `architecture` command pair and both hooks. V3 is split in two for size; the README is a later
  unit. V2 waits for 0041.
- S6 gate 0 PASS (`s6/gate0.txt`, read-only): the relayed hash `73483c82…`, 287/287 blocks render,
  all 10 render targets with and without a trailing slash, the `home` head and tail as expected.
  Slot A (`cargo test --all --no-fail-fast`, full output in `s6/slot-a.txt`): the 13 predicted reds
  are exactly the actual reds in S6's six files, each for its predicted cause; both guards green.
  Baseline, 4 reds in files S6 does not edit, each the first failed assert (later asserts unverified,
  for S9): `fidelity.rs` sequence list without 36–40 (`:173`) · `fidelity.rs` and `validate.rs`
  census, live command rules 335 against 333 (`:731`, `:1153`) · `matrix_similar.rs` command-family
  figures (335, 13050, 0, 54) against (333, 12884, 0, 54) (`:947`). The pre-fix binary is kept at
  `s6/mochiko-cli-head` (`4d688d03…`). Slot A closed; S6 edits `src/` with no cargo, then asks for
  slot B.
- S6's `src/` edits done without cargo: 11 files (5 src, 6 tests), +526/−55, rustfmt on exactly
  those, `--check` exit 0. Slot B granted, with one execution change: every cargo command runs with
  `CARGO_TARGET_DIR` in `s6/target`, so the auditors' `target/debug/mochiko-cli` is not swapped
  mid-audit; the lead rebuilds `target/` at the gates after G3. The sweep compares `s6/mochiko-cli-head`
  with `s6/target/debug/mochiko-cli`, about 14 minutes by S6's smoke run.
- Slot B stopped on one unpredicted red (576 passed, 5 failed; all 13 predicted now green, both guards
  green, the 4 baseline reds unchanged by name): `matrix_similar.rs
  a_log_whose_own_tree_carries_no_allowlist_keeps_walking_up` (`:1172`). The lead confirmed the
  cause: the test's own comment (`:1168`) assumes its scratch tree sits inside the repository, which
  holds only for an in-repo target dir, and the lead's `CARGO_TARGET_DIR` change moved it out;
  `src/similar.rs` and the test file carry no diff. Ruled environment-caused; the in-repo rebuild at
  the lead gate must show it green (binding). S6 continues slot B. BACKLOG: the test depends on the
  target dir's location.
- S6 slot B continued: fmt checks, clippy `-D warnings` and `cargo audit --deny warnings` exit 0; the
  new binary's `migrate status` prints `grammar 2` over the same state `73483c82…`; the repo's
  `target/` untouched. The side-by-side sweep (1,357 files) running.

## 2026-09-29 · V3 gate audits, round 1 — outcome lines · 5 units FAIL · one fix round to S8

Outcome lines (the auditors write nothing in the repo, so the lead records them; verdicts in
`v3a/verdict.md` and `v3b/verdict.md`; each file count includes the unit's strip file):

```
audit: authoring-architecture-store (skill pair) · V3a (V3 split with V3b for size) · opus · 2 files · 1 rounds · 2 blocking
audit: authoring-epic (skill pair) · V3a (V3 split with V3b for size) · opus · 2 files · 1 rounds · 1 blocking
audit: authoring-technical-requirements (skill pair + ARTIFACT-TEMPLATES.md) · V3a (V3 split with V3b for size) · opus · 3 files · 1 rounds · 0 blocking
audit: testing-end-user (skill pair + EVIDENCE-CAPTURE.md + REPORT-TEMPLATES.md) · V3a (V3 split with V3b for size) · opus · 4 files · 1 rounds · 0 blocking
audit: patterns-technical-decisions (prose) · V3a (V3 split with V3b for size) · opus · 2 files · 1 rounds · 0 blocking
audit: mochiko router (prose) · V3a (V3 split with V3b for size) · opus · 2 files · 1 rounds · 1 blocking
audit: patterns-system-design (SKILL.md + DIAGRAM-CONVENTIONS) · V3b · opus · 3 files · 1 rounds · 2 blocking
audit: patterns-entity-modeling (SKILL.md + RELATIONSHIP-PATTERNS + validate-model.py) · V3b · opus · 4 files · 1 rounds · 0 blocking
audit: patterns-api-contracts (SKILL.md + OPENAPI-TEMPLATE) · V3b · opus · 3 files · 1 rounds · 0 blocking
audit: review-plan-artifacts ARTIFACT-CHECKLISTS.md · V3b · opus · 2 files · 1 rounds · 1 blocking
audit: review-feasibility FEASIBILITY-LENS.md · V3b · opus · 2 files · 1 rounds · 0 blocking
audit: knowledge-management constitution module · V3b · opus · 2 files · 1 rounds · 0 blocking
audit: report-format template · V3b · opus · 2 files · 1 rounds · 0 blocking
audit: architecture command pair · V3b · opus · 2 files · 1 rounds · 0 blocking
audit: hooks seat-reminder.sh + session-start.sh · V3b · opus · 3 files · 1 rounds · 0 blocking
```

- V3b: pre-pass 0 rejecting · 113 advisory; budgets inside and matching the lead's figures; the
  linter `['User','Session']` 8/8 (controls 7/8); both hooks clean, 15,000 folders in 0.58 s.
  system-design — B1 "every box appears in the table and vice versa" cannot hold now the table lists
  only changed elements while collaborators stay drawn plain; B2 the altitude check keyed to every
  spine element fails a delta adding a boundary or flow · ARTIFACT-CHECKLISTS — B3, B1's root cause in
  the coverage row and Key Question. A6 (supersession notes on the 2026-08-13 architect-role and T3
  `DECISIONS.md` rows) to the lead's landing; A7 (`REMINDER_GOLDEN`) is S9's.
- V3a: its measurements match the lead's exactly. store — `:73` "the three keyed statuses" is false
  against `lifecycle-statuses` (four, `proposed` too) while its strip calls it kept verbatim; the
  `proposed` bullet (+86) repeats the diagram line above it (body +406: 320 HOLDS, 86 FAILS) · epic —
  the +71 one-pen clause restates the floor `shared-baseline-single-pen-holder` (FAILS; fix: end the
  list at "and the ordering") · router `:143` — "flips … in-flight-class elements to `built`"
  contradicts `fold-duty` and `impl.store-landing` (a removal goes to its `removed` stub). Budget
  verdicts for store, epic, ATR and testing-end-user are conditional on V1/V2's render rulings.
- One fix round to S8 for all five FAIL units, as each auditor stated the fix; then V3a and V3b each
  re-audit their units once, and a second FAIL goes to the user.

## 2026-09-29 · V1 gate audit, round 1 — 0029 FAIL on its budget component · fix round 0042 to S7

```
audit: schema 0024-delta-baselines-in-place · V1 · opus · 6 files · 1 rounds · 0 blocking
audit: schema 0025-delta-lifecycle-marker · V1 · opus · 6 files · 1 rounds · 0 blocking
audit: schema 0026-delta-pinned-base-review · V1 · opus · 4 files · 1 rounds · 0 blocking
audit: schema 0027-delta-drawing-not-copies · V1 · opus · 6 files · 1 rounds · 0 blocking
audit: schema 0028-delta-epic-one-pen · V1 · opus · 3 files · 1 rounds · 0 blocking
audit: schema 0029-delta-proposed-until-signoff · V1 · opus · 9 files · 1 rounds · 1 blocking
```

- V1 (`v1/verdict.md`): pre-pass 0 rejecting · 113 advisory, 0 clusters; views equal replay; across
  0024–0029, 16 views move, 9 ids out (all tombstoned), 10 in, none reused; implement pins unchanged
  (fail 15, floor 37); its budget measurement matches the lead's table to the character; 13 of 14
  render components HOLD.
- Blocking (0029, budget): review-sufficiency's +1,031 fails in part — the D7 reader sentence renders
  three times in one payload (`lifecycle-marker-read`, again in `clause-in-flight`, 156 characters,
  and as `clause10-carve`'s added sentence, 204), so about 360 characters are restatement. Fix: a
  reword migration citing `review-sufficiency.lifecycle-marker-read` in both places, keeping D6d's
  "never proposed"; the residual (about +700) HOLDS. Fix round to S7 as migration 0042 (wave plan §1
  widened), written under a lead slot after S6's slot B with 0041; V1 re-audits it once.
- Prefixes ending at 0024 or 0025 not validating alone: ruled acceptable by V1 — `replay.rs:276` runs
  the hard set on the finished state only, and the six files landed together in `5558fd7` with
  consecutive sequences, so no state or release carries 0024 without 0026. Recorded here in place of
  editing the landed intents.
- To BACKLOG: the store template's concern-ledger `check:` asks a stance of every row while its
  contract excepts rows cut for removal · no seat is named to run the duplicate-id check and the
  unmarked-write test (D3a names the landing diff reviewer) · `sequential-ids` names no sequence for a
  spec-home file. 0026's unmarked-write fail needs 0041 (R11) before the bump — planned.
- S8's fix round (all five units, one pass; strip entries extended in place at [v0.116.0], none new):
  system-design and the checklists as V3b stated (styled boxes and `container` rows only; the altitude
  check keyed to `kind: container`; body 9,991 of 11,047) · store — the `proposed` bullet deleted, the
  in-flight-class line names the three sign-off statuses, the strip no longer claims it verbatim (body
  5,767, net +376) · epic — the list ends at "and the ordering", the strip records a deletion (body
  3,101, net −28) · router `:143` points at `impl.store-landing` and `impl.landing-verifier-folds`.
  V3a's A1–A3 folded (A3 touches the passed ATR unit's `ARTIFACT-TEMPLATES.md:215`, re-read by V3a);
  A4–A6 declined with reasons (A4 an accurate line, A5 predates the wave, A6 V1's schema content).
  V3a and V3b re-auditing.

## 2026-09-29 · V3a re-audit PASS on all three units

```
audit: authoring-architecture-store (skill pair) · V3a (V3 split with V3b for size) · opus · 2 files · 2 rounds · 0 blocking
audit: authoring-epic (skill pair) · V3a (V3 split with V3b for size) · opus · 2 files · 2 rounds · 0 blocking
audit: mochiko router (prose) · V3a (V3 split with V3b for size) · opus · 2 files · 2 rounds · 0 blocking
```

- V3a round 2 (`v3a/verdict-r2.md`): state `73483c82…` unchanged, 0 rejecting · 113 advisory; bodies
  match S8's (store 5,767, epic 3,101, router 47,326). Store body +376 HOLDS in all four parts, the
  pair's overage now conditional only on V2's 0039 (+142) · epic overage +42, covered by its standing
  ruled +261 (the wave's net −219), settled · router `:143` points at two live rules that both carry
  a removal to its `removed` stub. ATR's `:215` line breaks nothing; its round-1 PASS stands (its
  outcome line stays `1 rounds`, round 2 re-read one line). The declined A4 holds. To BACKLOG: A5
  (`AX-XXX-<slug>.md` against `<AX-ID>.md` naming, predates the wave) · A6 (a build-raised store
  element against `sign-off-flips-proposed`) · R2-A1 (`impl.baseline-diff-review` is the exact home
  of the baseline flip; a pointer refinement).
- V3b round 2 (`v3b/verdict-r2.md`): both units 0 blocking; pre-pass 0 rejecting · 113 advisory;
  system-design body 9,991 of 11,047, description 625 of 677.

```
audit: patterns-system-design (SKILL.md + DIAGRAM-CONVENTIONS) · V3b · opus · 3 files · 2 rounds · 0 blocking
audit: review-plan-artifacts ARTIFACT-CHECKLISTS.md · V3b · opus · 2 files · 2 rounds · 0 blocking
```

- Stall caught (the user asked for status and saw nothing running): S6, S7 and V3b had gone idle with
  finished background runs unreported. S6's sweep had ended at 18:28 exactly as predicted (1,357 rows,
  176 changed: RA3 1, contracts 14, c′ 161, flips 0, other 0; gone 0, moving 0; the same state hash on
  both sides). All three were prompted to report, and the standing rule was restated to every seat:
  wait for a background command and report, never go idle mid-run.
- S6's diff is final (no re-edit allowed after slot B), so G3 is dispatched on it now (a fresh
  `general-purpose` `opus` seat; its cargo runs under `CARGO_TARGET_DIR=target/g3`, in-repo, only on a
  lead slot, since S7's 0041/0042 slot is next) and S8's README is released.

## 2026-09-29 · S6 slot B closed, every check as predicted · S7's 0041/0042 slot opened

- S6 hand-off: 11 files, +529/−56, `Cargo.*` untouched. Slot B: 576 passed, 5 failed — the 4 baseline
  reds of slot A by name plus the ruled environment-caused `matrix_similar.rs` red; none of the 13, both
  guards green. Sweep (`s6/classify.txt`): 1,357 rows, gone 0, moving 0, the same state on both sides;
  old 946 allow · 411 deny, new 947 · 410; 176 changed, all predicted (RA3 1 deny to allow · contracts 14
  route wording · c′ 161 reason wording); flips 0, other 0, mochiko none. clippy, both fmt checks and
  `cargo audit` exit 0. The new binary `s6/target/debug/mochiko-cli` (`5ac86b42…`) prints `grammar 2 ·
  sequences 1..40` over `73483c82…`. For S9: the 4 baseline tests unverified past their first failed
  assert (`fidelity.rs:173`, `:731`, `validate.rs:1153`, `matrix_similar.rs:947`).
- S7's scratch stack (pre-slot-B binary): 1..41 and 1..42 each 0 rejecting · 113 advisory · clusters 0
  (states `959d0467…`, `9d162ed7…`). 0041 renders R11's four conditions with A2/A3 folded (342 to
  1,237 characters; the implement fail section 3,628 to 4,523). 0042 (anchor delta D7; the intent names
  V1's finding and what leaves): `clause-in-flight` 710 to 632 and `clause10-carve` 493 to 392, each
  citing `lifecycle-marker-read`, D6d's "never a `proposed` one" kept. review-sufficiency render 13,538
  to 13,359 (−179, not −360: the citations and the kept D6d clause take back about 180); payload 16,621,
  still +1,198 over, 0029's component now +852 for V1 to re-judge. One more installed-gate false deny
  on a read (`grep -n -i` on the delta record), not retried.
- S7's slot opened with S6's rebuilt binary for every step (A4); the repo's `target/` stays pre-slot-B
  until the lead's gate rebuild. G3's cargo waits for the slot to close.
- S9 dispatched plan-only (a fresh `staff-engineer`): the 4 baseline reds and whatever 0041/0042 move,
  `REMINDER_GOLDEN` to S8's 352-character line read from the landed script, the stale eval content (P5
  A1, G2 A1–A2, anything S8's entity template made stale), the gate workspaces' `.git` (G1 R5), and the
  contract suite's deterministic run (gate 6; a SKIPPED suite blocks). P9 grades its plan.
- S8's README landed (+85/−13) with its new strip file `migrations-readme.md` (2 entries at
  [v0.116.0]: the grammar re-keyed to the highest per O2 · the grammar-range and bump-criterion
  paragraph superseded, citing wave 1's approved §2.8 and field D2/D4). Every statement was checked
  against S6's source with file:line (the repo binary predates S6's diff); one claim was narrowed to
  what the crate bears out (an older binary reads the run folder as an ordinary home). `migrate
  validate` unchanged. V3b audits it as unit 10, the last prose unit.

## 2026-09-29 · S7 landed 0041 and 0042 · lead-verified · G3 cargo slot · V2 and V1's re-audit dispatched

- S7 landed 0041 (39 lines, `9b700a9f…`) and 0042 (37 lines, `0ab9fb26…`), every step on S6's slot-B
  binary (`5ac86b42…`, A4); scratch prefixes 1..41 and 1..42 each 0 rejecting · 113 advisory ·
  clusters 0. Views: this slot moved only `commands/implement.yaml` (`impl.fail.unmarked-baseline-write`)
  and `skills/review-sufficiency.yaml` (`clause10-carve`, `clause-in-flight`); cumulative against HEAD
  7 files, +107/−34. Renders: review-sufficiency 13,538 to 13,359, payload 16,621 (+1,198 over 15,423)
  · the implement fail section 3,628 to 4,523.
- Lead-verified first-hand (S6's binary): `grammar 2 · sequences 1..42 (40 migrations)` · `state
  sha256:9d162ed77bdd1ed39ed30147c909dfb2f6d77567b85fcdbeaad991d9b06d7d8d · 87 documents · 1139 rules`
  · 0 rejecting · 113 advisory · a fresh emit identical to `.mochiko/schema-views/` · both hashes.
- Dispatched: G3's cargo slot (`CARGO_TARGET_DIR=target/g3`) · V2 (a fresh plain `opus` seat) on
  0032–0041 with the render components 0036, 0037, 0038, 0039 · V1's bounded re-audit of 0029's budget
  item with 0042 applied (0029 now nets +852) and 0042 itself. S9 has the final hash for its plan.
- V3b unit 10 (`v3b/verdict-readme.md`) PASS, 0 blocking — every README statement checked against the
  code, and the S6-dependent ones probed with S6's binary on scratch trees (a preamble of 177 lines
  allows and 178 denies; entries at 177/178; a bad run key denies "not a run key"; only 0033 and 0034
  at grammar 2). V3b complete: 10 of 10 units PASS. Advisories to BACKLOG, not folded into an audited
  file: R-A1 mark `entry_heading` Required · R-A2 the grammar row's kept "grammar version" wording ·
  R-A3 declared run-folder names match at the top level only · R-A4 "(0033 and 0034 are grammar 2)"
  will date. R-A5 (the repo binary predates S6) is the lead's gate rebuild.

```
audit: migrations README (plugins/mochiko/migrations/README.md) · V3b · opus · 2 files · 1 rounds · 0 blocking
```

- V1 round 2 (`v1/verdict-r2.md`, S6's binary verified): 0029's budget item PASS with 0042 applied, and
  0042 PASS as schema content — one view file and two rule texts move, every rule field kept, the
  floor pin stays 8; the reader sentence renders once, down from three. review-sufficiency 16,621
  (+1,198) = standing +220 · 0026 +126 · 0029 +852 net, the +852 ruled HOLDS (`lifecycle-marker-read`
  about 561, a new D7 obligation; `clause-in-flight` +161, the read's new home plus D6d's "never a
  `proposed` one"; `clause10-carve` +115, a relocation; `clause-structural-trigger` +15); the citations
  are the minimum in substance. V1 complete: 7 of 7 units PASS; its 11 round-1 advisories stand.

```
audit: schema 0029-delta-proposed-until-signoff · V1 · opus · 10 files · 2 rounds · 0 blocking
audit: schema 0042-marker-read-cited · V1 · opus · 2 files · 1 rounds · 0 blocking
```

## 2026-09-29 · G3 FAIL on S6's crate diff (1 blocking, a shell-parse regression) · fix round 1

- G3 (`g3/review.md`, cargo under `target/g3`): fmt, clippy and `cargo audit` exit 0; `cargo test`
  577 passed, 4 failed — exactly the 4 expected pins, unmoved by 0041/0042; the environment-caused
  `matrix_similar.rs` test green on the in-repo target, closing that binding check. P1, RA3 (grows and
  new faults deny; the swaps are the ruled residual), A4, I4a–d and O2 hold; the range check per file,
  no diff; no new filesystem read and no network call in `src/`; the render head/tail changes only in
  the grammar digit.
- B1 (blocking, `src/shell.rs:753–766`, `:874–887`, doc `:35–40`): R2's subshell reading hides writes
  HEAD denied — a `((` read as two subshells exposes a `<<`, taken as a heredoc, and
  `skip_heredoc_bodies` swallows every later line. Three shapes deny at HEAD and allow now before a
  write into a home: 64 non-arithmetic `((` answers then `(( x = 1 << 2 ))` (bash and zsh) ·
  `(( x = $(case … esac) << 2 ))` (zsh) · the backtick form (bash 3.2). Fix, as G3 recommends: the
  union of the exact reading and HEAD's always-arithmetic reading (two linear scans), so no command
  yields fewer targets than at HEAD; the three shapes as red rows; the doc corrected.
- Advisory: A1 `settle` is O(k² log k) per key (3.44 s at 20,000 repeated sections, so a standing file
  of about 25,000 sections passes the 5 s hook timeout and fails open; pathological) · A2 no floor on
  the raw-output literal prefix (a hypothetical log) · A3 the RA3 deny rows pin no reason (folded) ·
  A4 `echo $[1<<2]` then a write is allowed at HEAD too (pre-existing) · A5 `closing_paren`'s gaps
  now decide the reading. A1, A2, A4 to BACKLOG; A5 by S6's call under the union.
- Fix round 1 to S6 with a cargo slot under `CARGO_TARGET_DIR=target/s6` (in-repo, gitignored; V2's
  scratch binary untouched). G3 re-reviews once; a second FAIL goes to the user.
- The lead's A2 "fails closed" note rested on the subshell reading only ever exposing writes; G3 showed
  it can also hide them (the exposed `<<` taken as a heredoc). The union below replaces it.
- S6's fix round (`s6/fix-round.diff`, +84/−28; `src/shell.rs`, `tests/shell.rs`, `tests/conform.rs`
  only): a `DoubleParen { Exact, Arithmetic }` switch, the Arithmetic arm being HEAD's `closing_paren`
  arm verbatim; `write_targets` returns the exact reading plus every arithmetic-reading target it
  lacks, in two linear scans; the doc claims only that no write either reading sees is missed. Red
  first: the three shapes failed, the ×63 control held; green: 577 passed, 4 failed (the baseline). Both
  fmt checks, clippy and `cargo audit` exit 0. End to end (`check --hook-json -`, head against new): the
  three shapes deny on both; `((echo a); tee HOME)` allow to deny. The new binary is
  `target/s6/debug/mochiko-cli` (`41c1de22…`). The file sweep was not re-run, since only `decide_shell`
  calls `write_targets` (`hook.rs:365`). A3 folded; A5 moot under the union; A1, A2 and A4 to BACKLOG.
  G3 re-reviewing, with a cargo slot.

## 2026-09-29 · usage-limit pause · G3 re-review PASS · V2: 0037 FAIL on its budget · 0043 fix round

- A usage limit stopped every seat at about 19:20 (reset 21:30). G3, S9 and V2 were mid-turn; nothing
  was running at the reset. Each was resumed or re-prompted from its last written state.
- G3 round 2 (`g3/review-r2.md`) **PASS**, 0 blocking: B1 and A3 closed; its own build under
  `target/g3` — fmt, clippy and `cargo audit` exit 0; `cargo test` 577 passed, 4 failed (the baseline,
  unchanged); the round's diff recomputed from S6's pre-round copies and identical. Advisory: R2-A1 the
  module doc's "the scan before G1 R2" should name the scan at `5558fd7` (BACKLOG, doc-only) · R2-A2
  the booked A1, A2 and A4 need BACKLOG rows (the lead's landing). S6's crate diff is final.
- V2 (`v2/verdict.md`, S6's slot-B binary): 9 PASS, 1 FAIL; 0 rejecting · 113 advisory; views equal
  replay; the view diff split per migration by stepped replays; no id lost or reused; the pins unchanged
  (implement fail 15, floor 37; architecture 1, 23); every home agrees with its census row, and all 87
  views, 102 strip files and 4 archive files resolve as declared deliverables. Budget: testing-end-user
  0036 +490 and 0038 +304 HOLD · store 0039 +142 HOLDS · ATR 0037 +158 FAILS in part — 79 characters
  (", in the spec's file or in place in the product file under its lifecycle marker") restate the
  floor `authoring-technical-requirements.artifact-home`; R1's +79 HOLDS. Pressed cases hold: 0036's
  pointer withdrawal (D4 `:486`), 0037's `impl.artifact-home` narrowing (D6, R1), 0041 no wider than
  R11. Advisory: `impl.sec.tools` renders 25,469 of 30,000 (A3) · 0040 archives the constraints file
  "unchanged" while it stays live in reduced form (A4).

```
audit: schema 0032-closed-world-homes · V2 · opus · 11 files · 1 rounds · 0 blocking
audit: schema 0033-run-folder-home · V2 · opus · 5 files · 1 rounds · 0 blocking
audit: schema 0034-store-entry-budgets · V2 · opus · 7 files · 1 rounds · 0 blocking
audit: schema 0035-home-sets-after-delta · V2 · opus · 9 files · 1 rounds · 0 blocking
audit: schema 0036-run-folder-and-evidence · V2 · opus · 10 files · 1 rounds · 0 blocking
audit: schema 0037-run-log-and-in-run-rulings · V2 · opus · 6 files · 1 rounds · 1 blocking
audit: schema 0038-pre-write-dry-run · V2 · opus · 6 files · 1 rounds · 0 blocking
audit: schema 0039-store-entry-bound-pointers · V2 · opus · 6 files · 1 rounds · 0 blocking
audit: schema 0040-brownfield-archive-names · V2 · opus · 6 files · 1 rounds · 0 blocking
audit: schema 0041-sweep-hunk-exception · V2 · opus · 6 files · 1 rounds · 0 blocking
```

- Fix round to S7 as migration 0043 (a new file, the 0042 pattern, so landed hashes stay stable):
  `decision-technique-routing` ends at "…lives here as a D-XXX entry."; slot open, on S6's final build
  `target/s6/debug/mochiko-cli` (`41c1de22…`). V2 re-audits once. S10 GO (S6 final, G3 PASS): its §4
  stop check first, against the final diff. S9 resumes its plan against 1..43.
- G3 confirmed `review-r2.md` final (PASS; slot closed). Its evidence adds an in-process comparison of
  1,000,000 random commands with 0 where the new parse names fewer targets than the old; the second
  scan costs a few milliseconds on 60,000-character hostile input.
- S10 landed the v3.2.1 PATCH (3 files, +54/−47: ledger 90, `rust-cli.md` 7, `CLAUDE.md` 2). The §4
  stop check against S6's post-G3 diff: no new filesystem, process or network read in `src/`
  (`render.rs` included); P1 as ratified; RA3 landed; `Cargo.toml` 0.3.0; S8's reminder one line; its
  11 probe verdicts reproduced on the final build. Edits E1–E14 landed as planned with the N-notes; the
  N3 grep is clean against the expected-hit list; the amendment-log row is ledger `:717` (2,762
  characters: the trigger evidence, the strikes, each correction with its ground, "mints no principle;
  no fresh `/mochiko:setup` amend"). Open: the Testability case list (`:562–563`) depends on S9's
  contract cases; the row cites build-log lines `:47–48` and `:853–856`. V4 (a fresh plain `opus`
  seat) grades the diff, pointer durability included. The DECISIONS row is the lead's at the landing.

## 2026-09-29 · S7 landed 0043 · lead-verified · the log final at 1..43 · V2 re-auditing

- 0043 (`0043-routing-restatement-cut.yaml`, 27 lines, `7b89c80c…`; anchor field D6; the intent names
  V2's finding and the 79 characters leaving, R1's boundary staying) rewords
  `decision-technique-routing` to end "…lives here as a D-XXX entry." One view file moves
  (`skills/authoring-technical-requirements.yaml`). ATR render 17,663 to 17,584, payload 21,622; 0037's
  component now +79, V2's HOLDS residual.
- Lead-verified on the final crate build (`target/s6/debug/mochiko-cli`, `41c1de22…`): `grammar 2 ·
  sequences 1..43 (41 migrations)` · `state sha256:71c099ee000b3ac9473e98fb3806b0b3d0e716539ad77b0b271d6464a1fed62c
  · 87 documents · 1139 rules` · 0 rejecting · 113 advisory · a fresh emit identical · the file hash.
  No more migrations are planned this wave. V2 re-audits 0037's budget with 0043 and 0043 itself; S9
  freezes its plan against this state.
- Lead ceremony, begun while seats work: `.gitignore` gains `.mochiko/runs/` (the line
  `home::ignores` matches). The budget sweep re-run on the final build at 1..43
  (`lead-budget/final.txt`) matches the components exactly; the ledger rows are written for store
  (+2,028), review-sufficiency (+1,198), testing-end-user (+1,030), feature-map (+375), epic (+42, now
  falling), sound-loop (+208) and gap-finding (+3,196), and the four body-only rows plus two
  description rows note their [v0.116.0] re-measures inside budget; ATR's row waits for V2 on 0043. The
  pinned `.mochiko/memory/knowledge-management.md` orphan rule re-keyed to 0025's wording (S8's O6).
  V3b's A6: the 2026-08-13 architect-role row and the 2026-07-23 workflow-token-reduction row (T3)
  annotated "superseded in part 2026-09-29" in `DECISIONS.md` and the index. Release note owed: plugin
  0.116.0 needs `mochiko-cli` 0.3.0 (grammar 1..2) — the installed 0.2.0 halts loudly at 0033 — so the
  maintainer reinstalls from `main` with the merge (seam R4: only the maintainer installs).

## 2026-09-29 · V2 round 2 PASS · V4 PASS on the PATCH · S9's plan to P9 · the lead's Q-rulings

- V2, round 2 (`scratchpad/v2/verdict-r2.md`): 0037 PASS with 0043 applied, 0043 PASS as schema content;
  all eleven V2 units pass. ATR payload 21,622 = body 4,038 + render 17,584 (render 17,505 at 0036,
  17,663 at 0037, 17,584 at 0043), +847 over 20,775, summing exactly: standing +263 · 0024–0026 +257 ·
  0027 +83 · 0029 +165 · 0037 net +79 (R1's boundary text, HOLDS). The ledger's ATR row is written.
  Outcome lines, replacing round 1's 0037 line:
  `audit: schema 0037-run-log-and-in-run-rulings · V2 · opus · 6 files · 2 rounds · 0 blocking`
  `audit: schema 0043-routing-restatement-cut · V2 · opus · 3 files · 1 rounds · 0 blocking`
  V2 hit one installed-gate false deny on a read (`tee` to scratch, then `grep` on
  `.mochiko/memory/primitive-cost-budgets.md`, read as a write target); not retried.
- V4 (fresh plain `opus`, `scratchpad/v4/verdict.md`): S10's PATCH v3.2.1 PASS, 0 blocking, 8
  advisory; 36 dry-run probes on the final build match the text; a PATCH, not a MINOR (both wider
  denies grounded in rulings the row discloses).
  `audit: governance PATCH v3.2.1 · V4 · opus · 3 files · 1 rounds · 0 blocking`
  Lead: S10 applies A1 (date the struck note at `:494`), A2 (entry headings beside the build-log line
  cites), A3 (the `runs/` Reach sentence to the build), A4, A6, A8; V4 confirms the delta. A5 is the
  bump (the lead's); A7 is a plan note, skipped. After S9, S10 re-checks the Testability case list.
- S9's plan (`scratchpad/s9/plan.md`, 369 lines, `ed2fb3d4…`), measured at 1..42: census 335/804/1139
  (floors 119/264, 36 fails hold) · the command family (335, 13050, 0, 54) · the opt-in corpus (1139,
  178832, 0, 182); the suppressed-edge drop 183 to 182 is 0036's `truncation-bounds` reword leaving
  cluster [26], allowlist row `:538` kept (both ids resolve). Host probe: `gate-input` 17/31 red (no
  `.git` in the workspaces) and `reminder-input` red (the stale golden) as the file stands; all seven
  host cases green with the two planned edits simulated. Nothing written, no `cargo` run.
- Lead rulings on S9's questions: Q1 gate 6 with `MOCHIKO_GATE_VERSION` unset, the sandbox building
  from this tree (R4) · Q2 (a) — the legacy `FEAT-004/baseline-delta.md` fixture deleted, the s3 prose
  and two preregistration phrases fixed, with a dated note that no grid ran before the change · Q3
  the three `review-plan-artifacts` g1–g3 goldens booked to BACKLOG · Q4 `impl.run-folder` and
  `impl.evidence-citation` observable · Q5 S9 builds `target/release` in its Docker slot; order: S9's
  non-Docker slot, the lead's `plugin.json`/`marketplace.json` edits, S9's Docker slot on the
  quiesced tree, the lead's gates · Q6 blocker: `sbx ls` returns 401, not authenticated to Docker —
  the login goes to the user.
- P9 (a fresh `staff-engineer` peer) grades the plan per `mochiko:review-seat-plan`, with the rulings.
- Lead ceremony: `BACKLOG.md` build items re-stated (waves 1–3 built at 0.116.0, wave 4 owed) and
  four residual groups booked · `ROADMAP.md` Next row marked built · `target/debug` rebuilt in-repo
  (hash differs from `target/s6` by path; same source, identical status and validate output) ·
  the CHANGELOG entry drafted in scratch, gate figures pending.
- S10 applied A1, A2, A3, A4, A6 and A8 (ledger only; `rust-cli.md` holds none of the phrases). V4's
  round 2 on the delta: PASS, 0 blocking — twelve hook-json probes on the final build match the A3
  sentence; a word diff against round 1 shows no reflow change.
  `audit: governance PATCH v3.2.1 advisory delta · V4 · opus · 1 file · 2 rounds · 0 blocking`
  A5 stands as the landing condition: the PATCH lands with the 0.116.0 bump.
- S9 re-froze its plan at 1..43 (`eb0f02f0…`, 372 lines; figures unchanged, E0 done) while P9 held
  the earlier copy; P9 stopped on the hash move and the lead ruled it grades `eb0f02f0` in full.
- `sbx login` done by the user; `sbx ls` lists sandboxes, the 401 gone — Q6 cleared.
- P9 (`scratchpad/p9/verdict.md`): S9's plan FAIL, one blocker — B1, the Q2(a) dated preregistration
  note not in the plan (§4, §7's anchor, "pending Q2"). Everything else passes: every pin traced and
  re-run byte-identical on the final build at 1..43, the scope inside the five items, the Docker slot
  last, no contract case renamed or added (97 hold, the GI-012-release-gates-module list). Fix round to S9 with B1 and
  advisories A1–A4, A6, A7 folded; A5's four BACKLOG items booked by the lead (g1–g3,
  `sf-direction-checks`, the architecture g3 path, a session-start contract row). One re-review.

## 2026-09-29 · P9 PASS on S9's plan · GO on plan-v2 · slot A open

- P9 re-graded `eb0f02f0` PASS once the lead's Q-rulings stood as execution notes (B1 downgraded to
  A0), its message crossing the fix round. S9 wrote the fix round as a separate `plan-v2.md` (436
  lines, `98bba14a…`, `make_v2.py` — 27 asserted replacements), leaving the passed file intact. P9's
  delta re-review: PASS, no blocking — `make_v2.py` on `eb0f02f0` reproduces v2 byte for byte; B1 (the
  dated note at preregistration `:257`, the ruling's three parts, the s3 scenario change named),
  A1–A4, A6 (D0 records the `plugin.json` version and stops unless it reads 0.116.0) and A7 (97 cases,
  none renamed) met; D-A1 accepted (a true reword of the JSON-escapes line); D-A2 applied at
  execution (the note's pointer reads `:131 (s1) and :133 (s3)`). Verdict `scratchpad/p9/verdict.md`.
- GO on v2. Slot A (pins, eval edits, `cargo test --all`, `CARGO_TARGET_DIR=target/s9`) is S9's alone.
  Then: a fresh non-author review of the crate test edits · the lead's `plugin.json` /
  `marketplace.json` bump · slot D, the contract suite in the sandbox on the quiesced tree · the
  lead's gates.

## 2026-09-29 · S9 slot A closed green · the bump edits · slot D open · G4 reviewing

- S9 slot A (`scratchpad/s9/s9.diff`, `27341ea4…`, 12 files, +97 −62): E0 matched §1 at `71c099ee`;
  E1 showed exactly the four predicted reds (`fidelity.rs:173`, `:731`, `validate.rs:1153`,
  `matrix_similar.rs:947`) plus the opt-in corpus red at `:1122`, and `a_log_whose_own_tree_carries_no_allowlist_keeps_walking_up`
  green in-repo. E2: the pins moved (sequences through 43; census 335/804/1139; the command family
  (335, 13050, 0, 54); the corpus (1139, 178832, 0, 182)); `REMINDER_GOLDEN` generated from the
  landed script; the `.git` added to `case_gate_input`'s workspaces and `case_gate_live`'s seed; the
  implement, feature and architecture eval text re-keyed; the preregistration's dated note at
  `:257`; the legacy `FEAT-004/baseline-delta.md` fixture deleted (its empty directory with it).
  E3: `cargo test --all` 0 failed across 18 binaries (validate 105 · replay 64 · cli 62 · conform
  60 · matrix_similar 48 · home 44 · shell 42 · hook 41 · render 39 · migration 38 · fidelity 17 ·
  views 11 · anchor_grammar 5 · matrix_skill 3 · matrix_command 2); the opt-in similarity run 48
  passed; rustfmt clean; host probe 7/7 with `R-LINE-EXACT`; `check-rubric` implement uncovered only
  on the five booked 0013 ids. No gate deny. Deviations, none changing content: a relative binary
  path in the first probe run (rerun absolute) · an em-dash escape in S9's scratch edit script
  (fixed, `feature/evals.json` keeps its `—` escapes).
- Lead: `plugins/mochiko/.claude-plugin/plugin.json` and `.claude-plugin/marketplace.json` to
  **0.116.0**; nothing else under `plugins/`, `crates/` or `evals/` pins 0.115.0.
- Dispatched in parallel: S9's slot D (the contract suite in the sandbox, `MOCHIKO_GATE_VERSION`
  unset, D0 reading 0.116.0) · G4 (a fresh plain seat) reviewing `s9.diff` as the non-author crate
  and harness review, cargo in `target/g4` · S10 re-checking the Testability case list. A G4 fix to
  `run.py` re-runs slot D.
- S10: the Testability case list holds — the five cases the ledger names are registered in `run.py`
  (`:4954`–`:5025`), no gate case added or renamed, every `tests/` file modified only, `hooks.json`
  without a diff. S10's unit is complete pending the landing.
- S9 slot D: D0 `plugin.json` and `marketplace.json` read 0.116.0, `MOCHIKO_GATE_VERSION` unset · D1
  `cargo build --release` exit 0, `0.3.0 · grammar 1..2` (`0c55d0b3…`) · D2 `--host-only` 7/7 · D3,
  the full suite, started 22:49 with `run.py` at `ad5deb33…` (a G4 fix to it re-runs D3).
- Lead gates on the quiesced tree, `CARGO_TARGET_DIR=target/lead`: views emit ≡ `.mochiko/schema-views`
  (87 documents, 0 diff lines) · `migrate validate` 0 rejecting · 113 advisory at `71c099ee…` ·
  `cargo fmt --all --check` 0 · `cargo clippy --all-targets -- -D warnings` 0 · `cargo audit --deny
  warnings` 0 (1,277 advisories, 31 dependencies) · `cargo test --all` 0, 581 tests across 18 targets ·
  `MOCHIKO_FULL_SIMILAR=1` `matrix_similar` 48 passed · secret scan clean — the CI pattern over tracked
  and untracked files, gitleaks over the working diff and each untracked file, no personal address.

## 2026-09-29 · G4 FAIL on S9's diff (2 blocking, both in one preregistration) · fix round 1

- G4 (a fresh plain `opus` seat, `scratchpad/g4/verdict.md`): the diff equals the working tree; every
  crate pin re-derived and true, not just green — census 333 to 335 and 803 to 804 by exactly three
  mints, no class or kind moved; the command family +166 by bucket; the corpus +805; the 183 to 182
  edge proven row `:538`'s by single-row allowlists on 1..35 and 1..43; asserts only re-valued, none
  weakened. `run.py` minimal and correct: `REMINDER_GOLDEN` 352/352 against `seat-reminder.sh:34`; the
  `.git` fixture masks nothing (no `.git` allows by the stated limit, ledger `:428`); 97 cases; host
  7/7. Cargo in `target/g4`: fmt 0 · 581 passed across 18 targets · clippy 0 · full similarity 48.
  `review: crate+harness S9 wave-3 pins · G4 · opus · 12 files · 1 rounds · 2 blocking`
- Blocking: B1 `evals/plan/feature/preregistration.md:178` still names "the delta beside the
  baseline", the retired ledger model · B2 the dated note at `:257` cites `evals/plan/README.md:31–32`
  where the text sits at `:32–33` (a wrong pointer in a recorded amendment).
- Lead: fix round 1 to S9 — B1, B2, and A4 (feature `observable.yaml:53` names s3's orphan fix) plus
  a widened sweep for "delta beside". The contract suite reads nothing under `evals/plan`, so D3 stays
  valid and keeps running. A1–A3 touch `run.py` (a recorded slice, two comments) and would stale D3:
  booked to BACKLOG. A5 needs no edit. G4 re-reviews once.
- S9's fix round (`s9-r2.diff`, `5d745695…`, 424 lines; three lines in two files): `:178` reads "the
  contract touch named on the card"; the `:257` note names `:178` too and cites README `:32–33`;
  feature `observable.yaml:53` names the store's orphan `SPN-005` fix for s3. `run.py` untouched
  (`ad5deb33…`). The widened sweep's new retired-model hits outside `plan/` (the `review-plan-artifacts`
  preregistration `:37`, `technical-analyst` `evals.json:103`, the `principal-architect` p2 fixture's
  `architecture-delta.md`) booked by the lead.
- G4 round 2 (`scratchpad/g4/verdict-r2.md`): PASS, 0 blocking — both fixes true (README lines and the
  s3 fixture read by G4); the working diff equals `s9-r2.diff`; nothing new under `crates/`, the log,
  the views or `evals/contract/`; the feature, architecture and implement kit checks as before.
  `review: crate+harness S9 wave-3 pins · G4 · opus · 12 files · 2 rounds · 0 blocking`

## 2026-09-29 · contract suite 97/97 · landing ritual done · wave 3 closed, the bump to the user

- S9's D3 (`scratchpad/s9/contract-full.txt`), run on the quiesced tree with `plugin.json` at 0.116.0
  and a 0.3.0 release build of this tree: `contract suite: 97/97 cases passed, 97 ran, 327
  measurement(s) recorded and not asserted`, exit 0; no case skipped. `run.py` unchanged since the
  run began (`ad5deb33…`); G4's fix round touched only `evals/plan`, which the suite does not read.
- Final lead checks on the landing tree: `migrate validate` 0 rejecting · 113 advisory · the views
  equal the replay · the secret scan clean (the CI pattern over tracked and untracked files, gitleaks
  over the working diff) · no `.mochiko/runs/` folder left · `target/` ignored.
- Landing ritual: `CHANGELOG.md` 0.116.0 entry (cites the AM-5-field-review-fold exception row and seam R4, the
  `mochiko-cli` 0.3.0 reinstall from `main`, R3) · `plugin.json` and `marketplace.json` at 0.116.0 ·
  `DECISIONS.md` — the five 2026-09-29 rows, the delta and field-review rows "built at v0.116.0" with
  wave 4 owed, the v3.2.0 row noting PATCH v3.2.1, and the setup-agnostic row's stale "build queued"
  transcribed to built at v0.115.0 (status-agreement, fix on sight) · the brainstorms index's delta
  and field-review Landed lines · `ROADMAP.md`'s Next row · `BACKLOG.md`: both build items stay open
  on wave 4, four residual groups booked (every item within 15 lines) · the budget ledger's rows.
- Wave 3 closed. The bump commit goes to the user; nothing merges to `main` before the user says so.
  After the merge the maintainer reinstalls `mochiko-cli` 0.3.0 from `main` (R4). Wave 4 (kinako)
  follows, seam R3 first.

## 2026-09-30 · wave 3 committed and merged · wave 4 planned

- User: "Commit it" (wave 3 as `591f31e`), then "merge to main and plan wave 4". Before the merge the
  five 2026-09-29 decision records still read "build pending" or "build started" against
  `DECISIONS.md`'s "built" — fixed on sight as `853ce88` (status agreement). PR #40: CI `crate` green
  on both heads (`591f31e` push and PR, `853ce88` PR, 4m04s); merged as `7b15fd4`. `main` carries
  plugin 0.116.0 and crate 0.3.0. Wave 4's mochiko records ride branch `joint-hook-delta-w4`.
- Planning reads: the installs are split — the user scope holds plugin 0.112.0, kinako's project
  scope 0.114.0, a stale 0.110.0 entry names the removed `mochiko-run3` worktree, and the binary on
  PATH is 0.2.0. A session binds its gate to the plugin copy it started with, so the pass runs in a
  session started after the upgrade. Kinako: the user's checkout sits on `console-consistency-sweep`
  with an uncommitted `.gitignore` line and holds no `.mochiko` commit `origin/main` lacks; `main`
  (`44635f1`) is ahead by two releases — the pass branches from it in its own worktree.
- Plan: [wave4-kinako-pass.md](wave4-kinako-pass.md) — preconditions (merge · the user's reinstall
  and plugin update · a fresh session · R3 first) · K0 inventory and op probe · K1 D5(i) · K2
  D5(ii)–(iii) with D9(b)'s evidence deletions · K3 D5(iv) with census item 4 · K4 D5(v) with S11's
  pointers · a non-author diff read after each step · K5 close checks · refused shell shapes routed
  to one reviewed script the user runs with `!` · the landing, with the D10 watch booked for kinako's
  next implement run. Put to the user for approval.
- User: "Approve (Recommended)" · the wave runs in this session resumed after the upgrade ("Resume
  this one") · kinako's work in a new worktree off kinako `origin/main` ("New worktree"). Next: the
  user's reinstall and plugin update at both scopes, then the resume; the lead confirms the
  SessionStart line before K0 is dispatched.
- Upgrade, by the user: `cargo install --force --git …` installed 0.3.0 at `7b15fd42` but resolved
  fresh dependencies (no `--locked`); reinstalled with `--locked` on the lead's security note, the
  build now matching `Cargo.lock` (clap 4.6.6, syn 3.0.3) — README's missing `--locked` booked to
  BACKLOG. Marketplace refreshed; the user scope updated 0.112.0 to 0.116.0; `/reload-plugins --force`.
- The user questioned the quit-and-resume. A live probe settled it: in a scratch git tree, a Write
  into `.mochiko/runs/FEAT-000-run1/` was allowed (0.112.0's log denies it, 0.116.0's allows) and a
  control Write of an undeclared `.mochiko/` file was denied with 0.3.0's run-folder route in the
  text — the running session's gate is on plugin 0.116.0 and binary 0.3.0. No restart; plan
  precondition 4 re-worded. Kinako's project scope (0.114.0) is still owed its update.
- R3 pre-check by the lead: kinako has no `.mochiko/runs/`, and no worktree carries a `.mochiko`
  commit `origin/main` lacks; whether a feature run is still open is K0's read. Worktree created per
  the user's ruling: `kinako/.claude/worktrees/w4-cleanup`, branch `mochiko-0.116-cleanup` at
  `44635f1`, clean (`.claude/worktrees` ignored by kinako). K0 (a fresh `tech-lead`) dispatched
  read-only: R3 evidence, the record figures re-counted, the move list per step with each target's
  home, the op probe of every write shape against the live gate logic, anything the plan missed.
- User: kinako's project scope updated 0.114.0 to 0.116.0. Kinako's two live peer sessions still hold
  their old hooks until each reloads (`/reload-plugins --force`); neither writes during the pass.

## 2026-09-30 · K0 inventory returned · PK0 verifying · W1–W4 ruled by the user, L1–L11 by the lead

- K0 (`scratchpad/k0/inventory.md`, 249 lines, `3781b1c7…`; link list `raw-links.tsv`, `1d0ffaa9…`):
  R3 none open (every sufficiency report paired with a passed final validation or an epic close; no
  run folder; no store run key). Every record figure within a recount: evidence 68/166/93 · legacy
  directories as recorded · 9 archive files · pointers 47 in 24 at `5167f36` (48 in 25 at HEAD) ·
  63 ledger-naming files · ledgers 8,068 lines (237 entries, not 238: a fenced `## 2.1.47`) · fold
  blocks at the recorded lines · 17 headings (15 in the file + 3 from §A2, less `C-002 · AMENDED`) ·
  high-water C-008/D-049, no collision. Budgets: the largest entry after K1/K3 is 85 (data model) and
  126/77 (constraints), under 177. Op probe: only the ledger `git mv` is refused among the needed
  shapes (`git rm`, `rm -r`, Write, Edit, `git add`, `git commit` allowed); the whole-tree unchanged
  rewrite denies exactly the 327 evidence files and 41 closed-record files. No stop item. Nineteen
  gaps (§5); K4's real scope 124 ledger · 131 copy · 69 evidence · 11 live fold-heading links, and
  236 line cites.
- PK0 (a fresh `tech-lead`) verifies K0 first-hand; its FAIL on a fact reopens the ruling on it.
- User, from plain options, each "as recommended": W1 preamble notes kept as dated lines · W2 the
  D-050…D-061 write-ups moved unchanged into the product entries · W3 the three EPIC-002 contract
  fold blocks re-homed · W4 the Rust comment links left and booked. The lead's L1–L11 readings and
  W1–W4 are recorded in `.mochiko/decisions/2026-09-30-wave4-kinako-pass-rulings.md`, with a new
  `DECISIONS.md` row.
- PK0 (`scratchpad/pk0/verdict.md`): FAIL, 4 blocking. What holds, first-hand: R3 none open, every
  record figure, every sampled probe verdict (C8/F2 off by one line, a construction difference), the
  draft budgets re-measured, the keep list, every §5 anchor, 30 sampled link rows. Blocking: B1 the
  copy class misses the short `FEAT-00x/…` form (61 lines in 30 files; 15 live), so the code-comment
  count behind W4 is 12 lines in 8 files, not 8 in 6 · B2 the evidence class drops two grep lines
  (one of the record's 47) · B3 the K4 file probe counted from a superseded list (113 · 98 · 15) ·
  B4 §A is stale beyond the refusal count (five moved code line cites; B137's eight marker keys
  against A4's "six → seven"). Missed: nine cites W3 orphans; `contracts/README.md:21`'s counts.
- Lead: fix round 1 to K0 (B1–B4, the missed items, the `C-002 · AMENDED` target entry, the
  package-to-product id map for W2, the ledger-internal lines). W4's count corrected in the record,
  its ground unchanged and the user told; L1 widened to "§A applied as current truth — every count
  and cite re-verified at HEAD, a later landed change wins, noting the run-4 figure". PK0 re-reviews once.
- K0's fix round (`inventory-v2.md`, 294 lines, `f57d6eaf…`; `raw-links-v2.tsv`, 1,070 lines,
  `f171a8db…`): B1 C re-derived at 192 in 68, holding PK0's 61; code comments 12 in 8 · B2 E and
  E-dead cover the whole grep set (the two drops explained: a worktree-path exclusion, a never-created
  folder) · B3 136 files · 120 allow · 16 deny · B4 27 §A cites checked, 5 moved, 3 counts superseded.
  L3's stop fired (the product home binds no template); four new findings (W2's bodies carry
  colliding package ids, short paths and stale cites; AppSettings 13 in code; D-057's "private" false
  as graded; 202 contract line cites).
- Lead readings L12–L17 recorded: the `## Declarations` heading after the Part 4 name · W2's bodies
  id-mapped by §A2's F-01, paths and cites corrected, one dated provenance line each, each its own
  `###` entry (the user told, W2 reopenable) · AppSettings "eleven at run 4" with a dated B137 note ·
  D-057's fact corrected with a note · L5 widened to the contracts · §A's links written in final form.
  PK0 re-reviews v2.

## 2026-09-30 · PK0 PASS on K0's v2 · K1 dispatched plan-only

- PK0 round 2 (`scratchpad/pk0/verdict-r2.md`): PASS, 0 blocking — B1–B4 closed and re-checked
  first-hand (C 192 in 68 with 0 missing; E ∪ E-dead the whole 54-line grep set; the file probe 136 ·
  120 · 16; 27 cites, 5 moved, the counts checked in code); K0's new findings true; nothing v1 had
  right broken; no ruling W1–W4 or L1–L17 rests on a wrong fact. Advisories to carry: D-055's body
  also cites `architecture.md:182` (L13's re-point) · W1's worst case about 102 lines, inside 177.
  `review: inventory K0 wave-4 · PK0 · claude-opus-5-5 · 1 file · 2 rounds · 0 blocking`
- K1 (a fresh `technical-analyst`) dispatched plan-only: §A applied in place as current truth (L1,
  L2, L14, L15, L17), the README and ipc counts, and W2's twelve write-ups moved per L13 — each its
  own `###` entry, id-mapped, re-pointed, re-verified, one provenance line. A fresh peer grades the
  plan; GO follows a PASS; a non-author diff read after execution.
- K1's plan (`scratchpad/k1/plan.md`, 223 lines, `536adbaf…`): full-replace Writes of five files
  (`data-model.md`, `constraints-and-decisions.md`, `contracts/ipc.md`, `contracts/plugin-bridge.md`,
  `contracts/README.md`), the final bodies pre-assembled in `k1/final/` and each dry-run ALLOW on the
  live gate; budgets: the data model 1,317 lines with a largest entry of 85, constraints 969 lines
  with 126, the twelve W2 bodies their own entries at about 30 at most. One more stale cite found
  beyond PK0's list (D-056's `cli.rs:1516-1519`, now `:1658`). Lead on K1's two questions: A2's Part 4
  "none" writes nothing (the `## Declarations` heading is K3's, for the older blocks) · A4's four
  unanchored pieces placed at the section each extends, a plan choice the grader judges. PK1 (a fresh
  `technical-analyst`) grades the plan.
- PK1 (`scratchpad/pk1/verdict.md`, on `sonnet`, the persona's default): FAIL, item 3 — the reads
  list names a non-existent `corpus/sitting.rs`, and corrections row 1 resolves only against the
  top-level `settings.rs`; rows 1–5 match K0 §5.1 verbatim while the plan claims no reliance on K0.
  A disclosure defect, not content: items 1, 2 and 4–6 PASS, with the hunk counts, baselines, five
  dry runs, budgets, four W2 bodies word for word and the 29 id tokens all re-checked first-hand.
  `plan-grade: K1 · PK1 · claude-sonnet-5 · 1 rounds · 1 blocking`. Fix round 1 to K1 (the reads,
  the attribution, a check that no final text names a wrong path); PK1 re-reviews once.
- K1's revision (229 lines, `8b57a1b8…`): the reads corrected and the attribution split (rows 1–5 to
  K0 §5.1; the W2 cite fixes and D-056's `:1658` K1's own). PK1 round 2 (`verdict-r2.md`): PASS —
  the round-1 read reconstructed to its exact sha and diffed, three hunks only; `final/*` sizes and
  times unchanged since round 1. `plan-grade: K1 · PK1 · claude-sonnet-5 · 2 rounds · 0 blocking`.
- GO K1 (the worktree clean at `44635f1`; K1 the one writer): five full-replace Writes byte-identical
  to `k1/final/*`, `cmp` after each, the step's diff saved; no commit before the non-author read.

## 2026-09-30 · K1 executed in kinako · the lead verified · D1 reading the diff

- K1: the five files written, no gate deny, nothing outside them, no commit. Method deviation,
  disclosed: 20 exact-match Edits reproducing the graded hunks, not five whole-file Writes (a
  1,200-line retype risks drift; an Edit fails loudly on a miss — one miss on a dropped backtick,
  re-read and applied). Outcome as graded: each file `cmp`-identical to `k1/final/*`, verified
  first-hand by the lead; `git diff` sha `2500e838…` (5 files, +474 −4; saved `k1/k1.diff`).
- D1 (a fresh `tech-lead`) reads the diff against §A, the W2 source and the rulings: conservation,
  current truth, all twelve W2 bodies, the duplicate-id check, final-form links, the gate, scope.
  The lead commits the step on D1's PASS.
- D1 (`scratchpad/d1/verdict.md`): FAIL, 17 blocking — ids (C-011/D-061/V-51, no duplicate), gate
  and scope PASS, and every value K1 changed true; the fails lie elsewhere:
  - scaffolding landed (Part 3's "none" body, A3's trailer);
  - three insertions split a note or a sentence, and C-009…C-011 were written `####`, nesting them
    inside another entry;
  - A4 rewritten beyond L1 (dropped sentences, one miscounted row, `bind-record-conduct-below-floor`
    left unrecorded);
  - two line cites K1's own insertions made false;
  - report names, paths and feature-only § anchors not in final form;
  - two W2 changes outside the allowed kinds (D-060's cite, D-055's added clause);
  - D-061's cite and "not yet realized" claim false;
  - three derived counts stale in their own sections.
  Not committed.
- Lead: fix round 1 to K1 with every F item. Rulings on D1's "your call" items:
  - A5 made blocking under W1 (run 4's id map lands as a dated note);
  - A7 new text qualified per the baselines' rule;
  - A8's older stale counts booked to kinako BACKLOG;
  - F9 as D1 places it;
  - advisories A1, A3, A4, A6 and A9 taken.
  D1 re-reads the whole diff once; a second FAIL goes to the user.
- K1's fix round: F1 and F4–F18 fixed; A1, A3, A4, A5, A6, A7 and A9 taken; A11 skipped. Its first
  report claimed F2 closed, but the lead found A3's trailer still in the `ipc.md` diff (K1 had looked
  in the other file) and sent it back in the same round. Now deleted. The lead verified: `git diff`
  sha `8a685bcc…` (5 files, +483 −6), no trailer, five files only. D1 re-reads the whole diff.
- D1 round 2 (`scratchpad/d1/verdict-r2.md`): FAIL, 6 blocking. Checked fixed first-hand: F1, F2,
  F4–F16, the A5 map, and all 12 W2 bodies by word diff. Ids, gate and scope PASS. Blocking:
  - B1: D-052's FR-034 is qualified with the wrong spec;
  - B2: the entity count note says one where two were added;
  - B3: D-061's correction cites the wrong test;
  - B4: F7(d)/(e) left unfixed (GAP-10 and the bind.md note re-worded and re-tensed);
  - B5: a false § SCR-036 pointer, caused by D1's own round-1 wording, disclosed;
  - B6: A7 not applied in `plugin-bridge.md`.

  `review: diff K1 kinako wave-4 · D1 · opus-5.5 · 5 files · 2 rounds · 6 blocking`
- The second FAIL went to the user, who chose "One more round". K1 applies D1's six fixes word for
  word, plus advisories a1 (the landing date 2026-09-23), a2 ("at HEAD" → `44635f1`), a3, a4 (bare
  mochiko ids → the rulings record's path) and a5. D1 checks the delta. A third FAIL goes back to
  the user.

## 2026-09-30 · D1 PASS on K1's third round · K1 committed in kinako · K2 dispatched plan-only

- K1 round 3 (`k1-r3.diff`, `25b99866…`, 5 files +487 −6; 80 lines changed from r2): D1's six fixes
  applied word for word, and a1–a5 taken. D1 round 3 (`scratchpad/d1/verdict-r3.md`): PASS, 0 blocking.
  Each fix was re-checked against the specs, the tests and §A4; nothing outside the named hunks moved;
  the gate allows all five files; the id set is unchanged.
  `review: diff K1 kinako wave-4 · D1 · opus-5.5 · 5 files · 3 rounds · 0 blocking`.
  Advisories: the mochiko rulings record must land on the mochiko branch, because kinako now cites it,
  and K5's dead-pointer scan must honour the `mochiko:` prefix.
- Lead: gitleaks clean on the staged diff, and no personal address. The staged diff's sha equals the
  graded `25b99866…`. Committed on kinako `mochiko-0.116-cleanup` as `e5e2e15`, tree clean.
- K2 (a fresh `staff-engineer`) dispatched plan-only. It covers:
  - the three ledgers to `archive/ledgers/<FEAT-ID>/`, by the user's one `!` script (L8);
  - the feature copies deleted: FEAT-001 and FEAT-006's data models, constraints and contracts, and
    FEAT-002's contracts. Each is verified folded first, the W2 write-ups already having landed in K1;
  - the 327 evidence files deleted;
  - the keep list honoured (census items 2–3, delta I9).

  A fresh peer grades the plan, and a non-author diff read follows execution.

## 2026-09-30 · K2 planned · FEAT-006 stop · W5 ruled · PK2 grading

- K2's plan (`scratchpad/k2/plan.md`, 212 lines, `22510028…`) and the user script
  (`k2/move-ledgers.sh`, `76e7e8f3…`).
  - The script guards HEAD `e5e2e15`, a clean tree, pinned sha256s and absent targets, then runs
    `git -C <W> mv` ×3 with absolute paths and re-checks each sha256.
  - Every seat op dry-runs allow. The inner `git mv` is denied, as K0 found.
  - K2 observed that the gate allows a seat to run `bash <script>`: it reads no script contents, so
    "the user runs it" rests on seat discipline.
  - End state, simulated in a scratch index: held branch 337 files, −33,094 lines (R100 ×3, D ×334);
    full 342, which matches K0.
- **The stop.** FEAT-001 and FEAT-002 are folded: 60 units are anchored, with 0 missing. FEAT-006 is not.
  - EPIC-001's `ac21ce8` changed no product baseline; the lead confirmed the stat.
  - The product names FEAT-006's copies as the home of content it omits. The lead confirmed c&d
    `:8–10`, `:561` and `:600–601`, dm `:8–11` and `:984`, and README `:133`.
  - Delta D5(iii)'s FEAT-006 premise ("the evidence of its epic landing") is false in part.
- **User ruling W5** ("as recommended"): W2's move applies to what the product names. That is the
  in-force rows' rationale, IP-001…005, `CorpusLocator` and corpus-format § 1.1. The superseded
  decisions, v1 §§ 1–9 and the engine-port/ipc residue go to history.
- **Lead reading L18:** K2 commits the held branch, and K2b (the W5 move plus FEAT-006's five
  deletions) is its own commit before K3. Both are in the rulings record, now 140 lines.
- PK2 (a fresh `staff-engineer`) is grading K2's plan and the script, read-only.

## 2026-09-30 · PK2 FAIL on K2's plan (2 blocking) · K2 fix round

- PK2 (`scratchpad/pk2/verdict.md`, `2c3d29e6…`):
  `review: plan K2 kinako wave-4 · PK2 · opus-5.5 · 2 blocking`.
  - **B1.** O5 was "run if ruled". With W5 and L18 on disk, a GO would read that trigger as met and
    delete FEAT-006's copies before W5's move. Fix: strike O5 and the full branch from K2.
  - **B2.** §4.3's "cited nowhere else" is false: D-013 is in 14 files (GI-033 waiver row), IP-003…005
    in `ci.yml:1`, and D-014 in `console.rs:151`. W5 is unaffected, because it rests on the six
    pointer lines the lead confirmed, not on this claim, and the user's question did not state it.
  - Verified live:
    - the script's paths and sha pins;
    - a `/bin/bash` 3.2 replica on a scratch linked worktree, which gave 3 × R100 with nothing else,
      and all five guards STOP;
    - every op's gate verdict;
    - every count;
    - `end-state.py`, 10/10 on simulated commits;
    - 24 of 60 fold units read.
  - Advisories:
    - A1: a mid-run failure leaves some ledgers moved without a STOP line, so add an ERR trap and a
      user-side recovery;
    - A2: re-check the script's sha at H3;
    - A3–A5: text fixes;
    - A6: carry the GI-033 row into K2b's W5 inventory.
- The lead sent B1, B2, A1, A3, A4 and A5 to K2 for its one fix round. A2 and A6 stay with the lead.
  PK2 re-reads the delta.

## 2026-09-30 · PK2 PASS on K2's revision 2 · GO · H3 handed to the user

- K2's revision 2 files:
  - `plan.md`: 259 lines, `bd415693…`;
  - `move-ledgers.sh`: `e6f52eaf…`. It adds an ERR trap and an `archive/ledgers`-absent guard;
  - `undo-ledgers.sh`: `d6ff0cc0…`. New: a user-run recovery that touches the six ledger paths only
    and refuses if any other path changed;
  - `commit-msg.txt`: `12bc1a9e…`.
- PK2 round 2 (`pk2/verdict-r2.md`, `077602cb…`): PASS, 0 blocking.
  `review: plan K2 kinako wave-4 · PK2 · opus-5.5 · 2 rounds · 0 blocking`.
  - B1 and B2 are closed.
  - PK2's own replica (its own object store) covered undo from every partial state, run twice, and
    refusing on stray files or a moved HEAD.
  - Kinako's index and object store were untouched.
  - Advisories:
    - A1: a Finder `.DS_Store` in a target dir stops undo's `rmdir`, after git is already restored.
    - A2: undo discards edits on the six paths. Accepted: only the move writes them.
    - A3: check both scripts' shas at H3.
    - A5: after `== undone`, the user reports to the lead and nothing re-runs without the lead.
- Lead: re-hashed both scripts, the plan and the commit message; all equal the graded bytes. Read both
  scripts whole. `bash -n` is clean. GO sent to K2 (O0). H3: the user is handed the `!` line.

## 2026-09-30 · user ran the ledger move · K2 staged the deletions · D2 reading the diff

- O0 (K2): HEAD `e5e2e15`, 0 status lines, and both script shas unchanged.
- **The user ran `move-ledgers.sh` with `!`.** It ended `== done`. The before and after sha256s are
  equal for all three ledgers, with 3 `R` lines. The output was transcribed to
  `scratchpad/k2/raw-user-script.txt`. The lead's own `diff --cached -M --name-status` shows exactly
  3 R100. Shape: L8, the recorded break-glass (field review S16(i)).
- K2:
  - O2: 3 R100, nothing else.
  - O3: `git rm` of the 7 FEAT-001/FEAT-002 copies, giving 10 lines.
  - O4: `git rm -r` of the 3 evidence trees, giving 337 lines (3 R and 334 D, no untracked).
  - O6: `end-state.py --verify` PASS. `337 files changed, 33094 deletions(-)`, matching the plan's
    §5.
  - No gate deny, no stop, not committed. FEAT-006's five copies are still tracked, for K2b.
- D2 (a fresh `qa-engineer`) is reading the staged diff against the plan's §1, the fold evidence and
  the keep set, read-only.

## 2026-09-30 · D2 PASS on K2's staged diff · O7 GO · K2b dispatched plan-only

- D2 (`scratchpad/d2/verdict.md`, `88bd220b…`): PASS, 0 blocking.
  `review: diff K2 kinako wave-4 · D2 · opus-5.5 · 1 round · 0 blocking`.
  - The expected set, built independently from `ls-tree -r e5e2e15`, equals the staged name-status.
  - Numstat is +0, and the ledger blobs are equal.
  - The keep set is untouched, and FEAT-006's five copies are still tracked.
  - Fold evidence: 43 of 60 units read, 33 of them outside PK2's sample. The 12 D bodies are present
    once each, with only L13 tokens differing.
  - None of the 327 evidence files is referenced by a product, spec, map entry or code.
  - Advisories:
    - A1: K2's omitted-content column was incomplete, but each item has another home.
    - A2 and A4: commit-message scope. The lead applied both; the message is now `1effec51…`.
    - A3: the rulings record is still untracked in mochiko, and lands in the wave-4 commit.
    - A5: counts for K4. Ledger links are 130 lines in 52 files, matching K0. Copy links are at least
      123 in 41, 18 of them in the product, 2 of those stale quickstart GAP-6 cites.
- Lead: gitleaks ran on the staged diff with no leaks (a vacuous scan, since it inserts nothing), and
  no personal address. GO for O7.
- K2b (a fresh `technical-analyst`) was dispatched plan-only while D2 read. It reads only product
  files and FEAT-006's copies, which the staged change leaves alone. It covers:
  - W5's moves;
  - the product lines the move makes false, rewritten to the new home;
  - the five `git rm`s.

  Two W5-unnamed items, dm § Amendment and the per-entity sensitivity detail, are resolved from the
  tree, or stop to the user if a current fact exists only in the copy.
- **K2 committed** in kinako as `7d0fbef` (parent `e5e2e15`): `337 files changed, 33094 deletions(-)`.
  - The message sha was checked (`1effec51…`) before the commit.
  - `end-state.py --verify --rev HEAD` PASS.
  - Tree clean; the lead confirmed with `log` and `--shortstat`.
  - K2b's base is `7d0fbef`.

## 2026-09-30 · K2b planned · sensitivity-table stop · W6 ruled · K2b revising

- K2b's plan (`scratchpad/k2b/plan.md`, 298 lines, `6e66d254…`, with its bodies in
  `k2b/bodies/final/`). Findings:
  - FEAT-006's ids are the product's own, so no remap is needed.
  - The bodies carry no file:line code cites.
  - About 180 bare FR/SC ids get the "(corpus-stewardship)" qualifier, and two short paths are
    re-pointed.
  - D-003/008/009 move as their Amendment re-issue texts.
  - What moves:
    - 10 c&d entries: C-001 extends its existing entry, and the other nine are new, placed before
      `### D-050`;
    - IP-001…005 as 5 new entries;
    - `CorpusLocator`, whole;
    - one corpus-format section.
  - Seven false product lines are rewritten.
  - All four whole-file edits dry-run allow. The largest entry is 112 lines, within the budget of
    177.
  - dm § Amendment is resolved, not a stop: the product's `WriteLock`, `InFlightTurnMarker` and
    V-rules already carry the current facts, re-verified from code.
  - For K4: `features/FEAT-006-corpus-stewardship.md:36/37/44` links to the copies.
- **Stop.** The per-entity `Sensitivity Details` tables (retention, access, deviations) for 8 entities
  exist only in the copy, and the product lacks them. The lead's check: the copy's tables at dm
  `:130`…`:464` run 6–8 lines each, and the product uses `#### Sensitivity Details` for FEAT-001's
  entities (dm `:615`).
- **User ruling W6** ("as recommended"): they move too, each under its entity in the FEAT-001 shape,
  within K2b. The rulings record is now 144 lines.
- K2b is revising the plan for W6. A fresh peer (PK2b) grades it next.
- K2b's revision (`plan.md`, 300 lines, `9f9c096f…`; 20 body files) moves 25 bodies, W6's 8
  `#### Sensitivity Details` tables among them. No stop remains open.
  - It self-caught two defects: the placement had to follow the Live-rows prose (K1's F4 class), and
    `CorpusLocator`'s sub-headings had to be `####`.
  - Expected end state: 9 files changed, +704 −5688 (4 M, 5 D).
  - Dry-runs: all four files allow, and the `git rm` allows.
- PK2b (a fresh `technical-analyst`) is grading it, read-only.

## 2026-09-30 · PK2b FAIL on K2b's plan (2 blocking) · K2b fix round

- PK2b (`scratchpad/pk2b/verdict.md`): FAIL, 2 blocking.
  - **B1.** `proposed/dm-final.md` omits Corpus's `#### Sensitivity Details`: only 7 of the 8 W6
    tables are in. The dm numstat is 118/9, not 127/9, so the plan's +704 does not match its own
    artifact.
  - **B2.** c&d `:375–376` cites FEAT-006's `engine-port.md`, which K2b deletes, and it was missing
    from the item-2 rewrites.
  - Verified clean:
    - all 49 c&d/dm units disposed;
    - 21 bodies diffed against their sources, with only L13 tokens differing;
    - D-013 agrees with GI-033;
    - placement (no F4-class swallow) and budgets;
    - the gate dry-runs and scope.
- The lead sent both fixes and the advisory to K2b for its one fix round. The totals are to be
  re-derived from the rebuilt artifact by numstat, not by arithmetic. The possessive "FEAT-006's …"
  mentions are to be re-grepped in the product. PK2b re-reads the delta.
- A K2b message crossed with the fix round. Plan `7914f884` left out Corpus's table on purpose, calling
  it a contradiction: `rootPath` is not in Corpus's Attributes.
- **Lead reading L19:** it is not a contradiction. The product keeps `rootPath` on Corpus as in memory
  and Confidential (dm `:36`, `:74`, `:132`), and the copy's row states the same facts. So the table
  moves, as W6 rules, which also closes B1.
- The rulings record is now 147 lines. K2b is doing the full fix round (B1, B2 and the advisory) on
  top.
- K2b's revisions 3 and 4 (`plan.md` `0cbb2834…`, 300 lines):
  - B1: the Corpus table is folded in, giving dm 127/9 and 23 `####` headings.
  - B2: c&d `:375–376` is the eighth rewrite. A product re-grep for paths and for the possessive
    "FEAT-006's …" found `contracts/ipc.md:437`.
  - The lead ruled that line in scope, under item 2's "any others you find", because the deletion
    makes it false. It is the ninth rewrite, pointing to `client.ts` with the copy in history, and
    `ipc.md` becomes the fifth modified file.
  - New totals: 10 files changed, +710 −5693 (5 M, 5 D).
  - Lead: hashed every `proposed/*` file, and all five `*-current.md` are `cmp`-equal to HEAD
    `7d0fbef`.
- PK2b is re-reading the delta; a second FAIL goes to the user.

## 2026-09-30 · PK2b PASS on K2b's revision 4 · GO

- PK2b round 2 (`pk2b/verdict-r2.md`): PASS, 0 blocking.
  `review: plan K2b kinako wave-4 · PK2b · opus-5.5 · 2 rounds · 0 blocking`.
  - B1 is fixed: the Corpus table's cells match dm `:36`, `:74` and `:131–132`, per L19.
  - B2 and the ipc rewrite are fixed: `client.ts` is named in the same file as "the single typed
    client".
  - PK2b's own `git grep -n "FEAT-006"` of the product found no further line that goes false. The
    remaining hits cite FEAT-006 files this pass keeps, or record past divergences.
  - Totals reproduced in a fresh scratch repo: +710 −5693, 10 files.
- GO to K2b:
  - Edit the five product files, with each `cmp`-equal to its `proposed/*-final.md` or stop;
  - `git rm` the five copies;
  - stage everything; no commit before D3's read.
- **K2b executed** on `7d0fbef`: five Edits, each file `cmp` MATCH against its `proposed/*-final.md`;
  `git rm` ×5; staged. `10 files changed, 710 insertions(+), 5693 deletions(-)` (5 M, 5 D), with
  nothing unstaged or untracked.
- Lead: every staged product blob (`git show :<path>`) is `cmp`-equal to its graded final, and the
  shortstat matches. D3 (a fresh `qa-engineer`) is reading the staged diff, read-only.

## 2026-09-30 · D3 FAIL on K2b's staged diff (6 blocking) · K2b diff fix round

- D3 (`scratchpad/d3/verdict.md`, `a03852a6…`, 139 lines): FAIL, 6 blocking, 11 advisories.
  - **B1.** Nine cites in moved bodies name FEAT-006 copy sections that the product lacks, and the
    step deletes their only target (D1's F12 class).
  - **B2.** Corpus SD's line cites have drifted by +2 (F10 class).
  - **B3.** dm `:16` and `:34`'s modelled counts are stale, and the Data Sensitivity Summary lacks
    CorpusLocator's `rootPath` row.
  - **B4.** Three lines go false after the move: the c&d "silent" list (D-012 now carries it), c&d
    `:8`'s rationale-home line, and corpus-format `:7–11` against § 1.1.
  - **B5.** C-001's rationale was not moved, though W5 names it.
  - **B6.** D-001, D-010 and D-011 differ beyond L13, in heading text and restated bullets (F15
    class).
  - Held, checked first-hand:
    - staged equals the graded finals;
    - all other bodies match their source exactly;
    - placement and budgets;
    - the W6 levels;
    - D-013 against GI-033;
    - the gate allows.
- Lesson, carried to K3/K4's briefs:
  - PK2b's plan grade passed what D3's diff read caught: cross-file cite targets, derived counts and
    "silent"-list truth.
  - Plan grades need the D1 defect-class list (F10/F12/F15, stale counts) as an explicit checklist.
- K2b is fixing all six in the staged files. The r1 finals are kept for D3's delta read.
- K2b's round 2 is re-staged: `10 files changed, 804 insertions(+), 5700 deletions(-)`, with
  `r1-to-r2.diff` at `303da433…`.
  - All six fixes are made. Eight advisories were applied and three declined with reasons.
  - One new K4 item: `architecture/spine.md:26`.
- Lead: the staged blobs are `cmp`-equal to the new finals, and the tree scope is 5 M + 5 D. D3 is
  re-reading the delta; a second FAIL goes to the user.

## 2026-09-30 · D3 FAIL on K2b's round 2 (2 blocking) · user: one more round

- D3 round 2 (`d3/verdict-r2.md`, `52895e87…`): FAIL, 2 blocking.
  - **R1.** B3 is only half cured: dm `:16` and `:35` keep the stale counts, with notes giving the true
    ones beside them.
  - **R2.** A B1 re-point made D-011's disposition claim that § The export rendering is "verbatim".
    D3 says the fault is its own round-1 mapping; the right form is L6.
  - Cured, checked first-hand:
    - B1: 8 of the 9 targets hold the cited fact;
    - B2, B4(a–c) and B6;
    - B5: verbatim match to the source;
    - the declines are sound;
    - the delta sets are equal;
    - budgets and the gate.
- This was the second FAIL, so it went to the user. The user chose "One more round (Recommended)".
- K2b is applying D3's exact wording, with nothing else touched. D3 checks only those hunks.
- K2b round 3 (`r2-to-r3.diff`, `26d18ffd…`, 41 lines): R1 and R2 applied, plus D1 provenance
  disclosures. `824 insertions(+), 5701 deletions(-)`.
- D3 round 3 (`d3/verdict-r3.md`, `1f8fe354…`): R1 and R2 cured, but FAIL with 2 new blocking.
  - **Q1.** D-012's new provenance line lists a § V2.9 re-point that D-012 lacks; it is D-014's.
  - **Q2.** The unchanged commit message says all nine cites are re-pointed, but D-011's § 8 is now
    kept "in history".
- This was the third FAIL, so it went to the user. The user chose "Apply D3's wording
  (Recommended)": two exact phrases, nothing else, and advisories E1/E2 not applied. D3 checks those
  two lines.

## 2026-09-30 · D3 round 4: product files PASS · commit message fixed by the lead · K2b commit GO

- K2b round 4 (`r3-to-r4.diff`, `d7655b9d…`): Q1 and Q2 applied verbatim. The lead confirmed the delta
  is exactly those phrases.
- D3 round 4 (`d3/verdict-r4.md`, `2ea2291c…`):
  - Q1 and Q2 cured. D3's own r3-to-r4 diff is one hunk, and the other four files are byte-unchanged.
  - **P1, the commit message only.** Its second-sweep sentence cited round-1 working-copy line
    numbers, true at neither the parent nor the commit. D3 says it missed this in rounds 2 and 3.
  - Advisory F1: the "every false line" claim holds because K4's list carries `ipc.md:7`, README
    `:113–125` and `spine.md:26`, as it does.
  - `review: diff K2b kinako wave-4 · D3 · opus-5.5 · 4 rounds · 0 blocking on the product files`.
- The lead applied D3's exact by-entry replacement text to `commit-msg.txt`, which is now
  `96ca8c74…`, the previous version kept as `.r4.txt`. The commit message is the lead's own ceremony
  text, as at K2, under the user's standing "apply D3's wording" ruling.
- Gitleaks ran on the staged diff (74 KB): no leaks, and no personal address. GO to K2b to commit.
- **K2b committed** in kinako as `e45b47e` (parent `7d0fbef`):
  `10 files changed, 824 insertions(+), 5701 deletions(-)`. The HEAD, shortstat and message shas
  were checked before the commit, and the tree is clean.
- **K3 dispatched plan-only** (a fresh `technical-analyst`), based on `e45b47e`.
  - Inputs:
    - the fold blocks: c&d `:1231`/`:1407`, dm `:1142`/`:1315`, and W3's three contracts;
    - c&d's 15 `####` at HEAD against the census's 17, to reconcile or stop;
    - K0's `drafts-v2`, marked stale (`44635f1`) and used as method only.
  - The brief carries the defect-class checklist from D1 and D3 (F4, F10, F12, F15, stale counts,
    silent lines, sentence splits, commit-message claims), plus a mechanical conservation proof.

## 2026-09-30 · K3 planned · heading count reconciled · PK3 grading

- K3's plan (`scratchpad/k3/plan.md`, 155 lines, `469178cb…`):
  - five `current`/`final` pairs, each final allowed by the gate;
  - conservation scripts with zero unexplained line differences;
  - 18 in-file re-points to removed fold headings, done by entry name;
  - 2 out-of-scope items for K4: `FEAT-002/requirements.md` and `contracts/README.md`.
- **Heading reconciliation.** 15 `####` in c&d at HEAD: 14 raised, 1 kept (`C-002 · AMENDED`, L3).
  - The lead confirmed c&d already had 15 at `44635f1`.
  - The census's "17" assumed run 4's C-009…C-011 would land at `####`. K1 wrote them at `###`, so
    item 4's "every id its own entry" is met.
- Flagged for the grader:
  - 8 `### Part N ·` fold scaffold headings dropped;
  - W1's notes paraphrased with a Source cite (K1's convention), not moved verbatim.
- **Gate observation.** K3 built a scratch git repo with `.mochiko/product/…` paths for numstat and
  was denied on a shell redirect. The gate was correct: it resolves homes against any repo root. K3
  rebuilt under other names, which is a route-around even in scratch.
  - The lead told K3 not to repeat the shape.
  - Seats use `git diff --no-index --numstat` from here on.
  - This is a note for the gate's scope docs.
- PK3 (a fresh `technical-analyst`) is grading, briefed on all four flags and the defect-class list.
- PK3 (`pk3/verdict.md`, 70 lines): FAIL, 2 blocking.
  `review: plan K3 kinako wave-4 · PK3 · claude-sonnet-5 · 2 blocking`.
  - **B1.** Dropping `### Part 2 · Decisions` leaves the EPIC-002 ratification table (D-038…D-044)
    unheaded after `### D-014` (F4 class).
  - **B2.** The plan claims five conservation scripts passing 0/0, but only one exists, and it prints
    10/7. PK3's own symmetric rebuild shows the content is in fact conserved.
  - Advisories:
    - a blank line before `## Declarations`;
    - the notes lack K1's `Source:` line;
    - the NFR/AX-021 preamble fact is dropped;
    - the heading reconciliation, numstat and gate are confirmed.
  - Not swept in full: the ~30 span boundaries for sentence splits.
- K3 fix round: B1 and B2, plus A1–A3 and a full boundary sweep. PK3 re-reads.
- K3's revision 2 (`plan.md` `50227944…`, 183 lines):
  - B1: 2 of the 8 Part-N drops needed an anchor, and each gets a new `###`.
  - B2: `conserve_all.py` covers all 5 files, giving 0/0 and PASS.
  - A1–A3 applied.
  - The sweep found 2 more missing blank lines (dm `HarnessSessionFile`, pb `SCR-037`).
  - `--no-index` numstat: 5 files, +808 −869.
- The lead hashed every final and the script. PK3's re-grade is to test the script's whitelist
  against a deliberately broken copy.
- PK3 round 2 (`pk3/verdict-r2.md`): PASS, 0 blocking.
  `review: plan K3 kinako wave-4 · PK3 · claude-sonnet-5 · 2 rounds · 0 blocking`.
  - B1's anchors are minimal. PK3 swept all 5 finals for any heading glued to the text before it,
    with 0 hits.
  - B2: the whitelist is narrow, PASS is reproduced byte for byte, and a tampered sentence in a
    scratch copy FAILs.
  - A3's fact matches the HEAD preamble.
  - Only the disclosed plan sections changed.
  - The gate, numstat and commit message are re-checked.
- GO to K3: write the five files, each `cmp`-equal to its final; stage; no commit before D4's read.
- K3's turn was cut off mid-write when the machine slept ("API Error: Your computer went to sleep
  mid-response").
  - The lead checked the tree: HEAD `e45b47e`, nothing staged. dm, engine-port, ipc and plugin-bridge
    are `cmp`-equal to their finals, and c&d is `cmp`-equal to its current, untouched, with no
    partial file.
  - K3 was resumed to write c&d only, then `cmp` all five and stage.

## 2026-09-30 · K3 executed and staged · D4 reading the diff

- **K3 executed and staged:** `5 files changed, 808 insertions(+), 869 deletions(-)`. K3 caught one
  transcription slip in c&d (`measured."*` against `measured"*.`) by `cmp`, and fixed it by Edit.
  - Lead: every staged blob is `cmp`-equal to its graded final, and the finals' shas are unchanged
    from PK3's grade. 5 staged, 0 unstaged, 0 untracked.
  - D4 (a fresh `qa-engineer`) is reading the staged diff. The brief asks for an independent
    conservation check and a W1 fact-by-fact check, plus the defect classes.
- The gate denied a build-log Edit that would have made the previous entry 64 lines, against the
  bound of 60. The remedy is the bound's own: a new entry for the new event (this one). The shape
  was not retried.
- D4 (`d4/verdict.md`, `ee85c9cd…`, 116 lines): FAIL, 7 blocking, 9 advisories.
  - **B1 (F4).** Three EPIC-001 seed quotes now head the wrong entry: c&d `:128`, dm `:628` and dm
    `:854`.
  - **B2.** Two direction words ("below", "above") are false.
  - **B3.** One re-point was missed, at ipc `:749`.
  - **B4.** "§ Entity: BoundSession" is ambiguous.
  - **B5.** Three section counts are stale: "Two rows", "Seventeen rows" and "Seven rows".
  - **B6.** "Heading levels are demoted one level" is now false.
  - **B7.** The commit message says four fold headings were deleted, but it was seven.
  - Held:
    - scope, and blobs equal to the finals;
    - D4's own conservation check (18 re-points and 42 preamble lines, all accounted for);
    - 14 raised and 1 kept;
    - budgets, the largest 157;
    - no glued headings;
    - the W1 facts;
    - the gate;
    - the K4 list complete.
  - `k3/raw-conserve.txt` now prints a pb FAIL. PK3's break test appears to have overwritten the
    author's evidence file, which is a grader writing into the author's folder.
- Lesson: PK3's plan grade passed, and D4's diff read found F4 in a new shape (a provenance quote cut
  off from its heading) and stale counts again. Plan graders read proposed whole files less closely
  than diff readers read the diff.
- K3's diff fix round covers all seven. The r1 finals are kept for D4's delta read.
- K3's round 2 (`r1-to-r2.diff`, `431296e4…`): `816 insertions(+), 877 deletions(-)`. K3 reports all
  seven fixed, and 6 advisories applied and 3 declined.
- The lead checked before D4's re-read. Two items were not done as briefed, so K3 was sent back within
  the same round, before D4 spends a read:
  - **B5.** The three section leads still state the stale counts, with notes stacked beside them.
    `:264`'s note gives no current total at all.
  - **A6.** Three pre-existing stale cites were re-pointed to new line numbers, against L5's rule
    to cite by name.
- K3 corrected both:
  - The leads now read "Ten rows are delivered", "Forty-two rows exist. Thirty-seven are live; five
    are superseded" and "Nine rows are delivered", each id listed, with the old text in dated notes.
  - The A6 cites now go by name.
  - `r1-to-r2.diff` is `021958b7…`, and the shortstat is unchanged.
- The lead verified the staged blobs against the finals. D4 is re-reading the delta; a second FAIL
  goes to the user.

## 2026-10-01 · D4 FAIL on K3's round 2 (3 blocking) · user: apply D4's fixes

- D4 round 2 (`d4/verdict-r2.md`, `a4d4f6db…`, 92 lines): all 7 round-1 blockers cured. FAIL, with 3
  new blockers, each one sentence:
  - **R1.** c&d `:264` keeps K1's "pre-run-4" note, which is now false, and its new note credits
    17 to 42 to the re-home alone.
  - **R2.** A3's new dm `:718` text points at the header's high-water paragraph, which is stale
    (dm `:31` "continues at V-46 and DS-012").
  - **R3.** The commit message's "text unchanged" omits five edited moved lines.
  - Checked first-hand:
    - D4's own delta equals K3's (62 lines);
    - 20 seed quotes re-paired by script;
    - the recounts are true;
    - the A6 targets are right;
    - the declines are sound;
    - conservation, budgets (max 157), glued headings, the fold greps and the gate.
- This was the second FAIL, so it went to the user. The user chose "Apply D4's fixes
  (Recommended)": exactly three sentences, and no advisories.
- Booked for kinako's BACKLOG (the stale-counts item): dm `:31`'s high-water line, which predates K3.
  Revised by the lead: it is a live product fact made stale by K1, so it goes to K4's scope (L1) and
  is not booked.
- K3 round 3 (`r2-to-r3.diff`, `365ec531…`): R1–R3 applied exactly. The lead confirmed a 3-hunk delta.
- D4 round 3 (`d4/verdict-r3.md`, `f0d194d6…`): PASS, 0 blocking.
  `review: diff K3 kinako wave-4 · D4 · opus-5.5 · 3 rounds · 0 blocking`.
  - D4's own delta has 3 hunks and equals K3's.
  - The recount, the minted ids and the whole message are true.
  - The gate and budgets hold.
- The lead edited the commit message (`1f9ff5b9…`):
  - D4's advisory v1 quote ("the fold" and "this fold");
  - a rulings pointer;
  - the attribution line set to this session's form. K3's draft carried its own seat's model line.
- Gitleaks: clean. GO to K3 to commit.

## 2026-10-01 · K3 committed · K4 dispatched plan-only

- **K3 committed** in kinako as `1313c9e` (parent `e45b47e`):
  `5 files changed, 816 insertions(+), 877 deletions(-)`. The message sha was checked, the
  attribution is this session's form, and the tree is clean.
- Kinako merges PRs with merge commits (`44635f1`, `88da637`, `f2c8c24`, `cab3aaa`), so the branch
  commits cited "in history at …" (`7d0fbef`) survive on `main`. The kinako PR must be merged the
  same way, never squashed.
- **K4 dispatched plan-only** (a fresh `technical-analyst`), based on `1313c9e`.
  - Scope: every link class in live, writable files (L, C, C6 for FEAT-006's copies, E, H, N, N3,
    and the dead links), plus dm's stale high-water line (L1).
  - Method: a table-driven transform with a line-diff proof. The seat may split K4 into sub-steps.
  - A dated report's path links are the seat's to read against D5(v)/L4–L7, or to stop on.
  - The brief carries the defect-class checklist.

## 2026-10-01 · K4 planned · dated-report links · W7 ruled · K4 revising

- K4's plan (`scratchpad/k4/plan.md`, 212 lines, `f323f108…`): 131 lines in 25 live files, split
  into 5 file-disjoint sub-steps (K4a–K4e), all gate-allow.
  - Readings:
    - L6's historical-instruction test extends to `tasks.md:174`;
    - "dated" is judged by substance (`sufficiency-report.md` counts as dated).
  - Gap: 48 C/C6 tokens were left "resolved at execution", which cannot be graded.
- K4 left every dated report untouched, and listed "460 dated path-links" as K5 residue.
  - The lead's count from K0's TSV: 222 path-link lines in 73 files still present at HEAD (L 81,
    C 70, E 59, E-dead 3, H 9). The 460 includes dated line cites.
  - L5 exempts only line cites. D5(v), S11 and K5's clean scan call for the path links.
- **User ruling W7** ("as recommended"): dated reports' path links are handled. Ledger links are
  re-pointed; copy, evidence and fold-heading links gain "(in history at `44635f1`)", with the text
  otherwise unchanged.
  - The rulings record is now 149 lines, against a bound of 150. W4 and W5 were re-wrapped, with the
    text unchanged. A further ruling needs a new record.
- K4 is revising: W7's rows as extra file-disjoint sub-steps, every row resolved in the table, and
  `44635f1` verified for each annotation.
- K4's revision 2 (`plan.md` `bbedce98…`, 201 lines): 353 rows in 98 files, 11 file-disjoint
  sub-steps (K4a–e live, K4f–k dated), all gate-allow.
  - Tables: core 45 · c-c6 42 · n-n3 56 tokens · dated 222.
  - K4 re-derived the dated count as 222 in 73 files, matching the lead's. `44635f1` is unchanged up
    to each deletion.
  - New: 6 FEAT-002 rows re-point rather than annotate, and one more pre-existing dead link repaired
    (`FEAT-006/requirements.md:26`).
- PK4 (a fresh `technical-analyst`) is grading. The brief asks for an independent link census, a
  script check of all 222 dated rows, and every live row read against its target.

## 2026-10-01 · PK4 FAIL on K4's plan (4 blocking) · K4 fix round

- PK4 (`pk4/verdict.md`, 139 lines): FAIL, 4 blocking.
  `review: plan K4 kinako wave-4 · PK4 · claude-sonnet-5 · 4 blocking`.
  - **B1.** The deleted-copy total is 12, not 17.
  - **B2 and B4.** The row counts are hand-derived and wrong: 139 live / 361 in all, not 131 / 353.
  - **B3.** A `spine.md:26` row was retyped with ASCII hyphens where the file has en dashes.
  - Held:
    - the 222 dated rows exact;
    - the file partition;
    - the high-water, `B86` and `FEAT-006/requirements.md:26` targets;
    - `44635f1` unchanged up to each deletion.
  - `apply.py` is "not yet written", so there are no proposed files to grade.
  - PK4's verdict does not report the brief's independent completeness census (item 1) or its
    every-live-row read (item 2).
- Fix round sent to K4:
  - counts regenerated by script;
  - every `old` column byte-copied from `1313c9e`;
  - `apply.py` with a pre-apply verbatim assertion, `conserve.py`, the proposed whole files for all
    98 targets, and the gate dry-run on each.
- PK4 was told its re-grade must cover items 1 and 2 with evidence.
- K4 stalled idle, waiting on a gate dry-run monitor that had stopped: an empty `gate-results.tsv`,
  and no process running. The lead nudged it, and the re-run in batches completed.
- K4's revision 3 (`plan.md` `d797e498…`, 195 lines):
  - 366 rows (easy 277, hard 33, n-n3 56), every `old` sliced from `1313c9e`;
  - `apply.py` asserts each `old` verbatim once; run 1 found 32 anchor mismatches, including 9
    fabricated fold-heading anchors, which K4 fixed; run 2 applied 366/366;
  - `conserve.py` proves 98 files, 333 lines;
  - the gate allows all 98;
  - live 143 rows in 25 files, dated 223 in 73.
- PK4 is re-grading. The brief asks for the census, all 143 live rows, a re-run of `apply.py` into
  PK4's own scratch space, a break test, and a script check of the dated rows.

## 2026-10-01 · PK4 round 2: B5 · W8 ruled · K4 adds one row

- PK4 round 2 (`pk4/verdict-r2.md`, `08843219…`): B1–B4 confirmed fixed. The mechanics reproduce:
  `apply.py` is byte-identical in PK4's own scratch run, `conserve.py` FAILs on an injected break,
  and the gate allows 11 of 11.
- **New finding, B5, from the census.** PK4 found 312 relative-form mentions of the five deleted
  contract names, in 67 files, in no table.
- The lead's check (`lead-ceremony/relcontracts.py`, at `1313c9e`):
  - 28 are dead markdown hyperlinks. K4's tables cover 27; one is missing,
    `FEAT-001/reports/records-groom-review-2026-09-22.md:295`.
  - The rest are backtick names in table cells. They name product contracts of the same names that
    still exist, and 19 are in the product itself.
- This was the second FAIL, so it went to the user.
- **User ruling W8** ("as recommended"): bare names stay, like L4's bare ledger names; the one
  missed hyperlink gets a row; K5 scans resolvable links only.
  - The first rulings record was at its 150-line bound, so W8 is in a new record,
    `.mochiko/decisions/2026-10-01-wave4-kinako-pass-rulings-2.md`.
- K4 is adding the row, plus PK4's two factual advisories, then re-running apply, conserve and the
  gate. PK4 re-checks the delta.
- K4's revision 4 (`plan.md` `282d609d…`): the `:295` row, annotated per W7 (the copy existed at
  `44635f1`; the link's relative depth was wrong); the `concerns.md` label and href fixed together;
  367 rows; apply 367/367; conserve 98 files, 334 lines; gate 2/2.
- PK4 round 3 (`pk4/verdict-r3.md`, `599a470d…`):
  - the delta passes, and the other 96 files are byte-unchanged;
  - one new item: `docs/quickstart.md:19–21`, 4 hyperlinks to `contracts/*.md` relative to `docs/`;
  - the lead checked: `docs/contracts` exists in no commit, so these links have never resolved;
    they predate the pass and lie outside K5's scan.
- This was the third FAIL, so it went to the user. **User ruling W9** ("as recommended"): repair them
  in K4, re-pointed to `../.mochiko/product/contracts/<name>.md` as L6 did for `tasks.md:114`.
  Recorded in `…-rulings-2.md`.

## 2026-10-01 · PK4 PASS on K4's revision 5 · GO

- K4's revision 5 (`plan.md` `5698db7b…`, 267 lines): 4 W9 rows added to K4a, giving 371 rows in 99
  files; apply 371/371, conserve 99 files and 337 lines, gate 99/99 allow. The lead diffed
  `docs/quickstart.md`: lines 19–21 only, labels kept.
- PK4 round 4 (`pk4/verdict-r4.md`, `f6f98659…`): PASS, 0 blocking.
  `review: plan K4 kinako wave-4 · PK4 · claude-sonnet-5 · 4 rounds · 0 blocking`.
  - The mechanics reproduce byte for byte. The only `apply.py` change is the K4a cluster entry.
  - The census inside and outside `.mochiko/` is clean: 26 links, 24 covered, and 2 that already
    resolve.
- GO to K4:
  - apply by Edit, one row per Edit, never a whole-file Write, since a retyped file is how K3's slip
    happened;
  - `cmp` each sub-step against its proposed files;
  - stage nothing. D5 reads each sub-step's diff, then the sub-steps commit one at a time.
- **K4 executed** all 11 sub-steps by Edit. Every touched file is `cmp`-equal to its proposed file,
  and no deny was raised. Unstaged: `99 files changed, 337 insertions(+), 337 deletions(-)`.
  - K4 disclosed two deviations, with content byte-equal either way: in K4a, two same-line pairs went
    as one Edit each; in K4h, short unique anchors were used inside two very long run-log lines.
  - Lead: an independent `cmp` of all 99 live files against `proposed/`, 0 differences. HEAD is
    `1313c9e`, nothing is staged, 99 ` M` and nothing untracked.
- D5 (a fresh `qa-engineer`) is reading the diff per sub-step: an independent hunk-to-row map, every
  live hunk read, the dated rows by script plus a third read, and the gate.

## 2026-10-01 · D5 FAIL on K4's diff (8 blocking) · L20 · K4 diff fix round

- D5 (`d5/verdict.md`, `464937ff…`, 200 lines): FAIL, 8 blocking.
  `review: diff K4 kinako wave-4 · D5 · claude-opus-5-5 · 8 blocking`.
  - **B1.** All 56 N/N3 rows wrote the resolver's description as their new text: line numbers (F10,
    56/56), `#` markup, nested backticks, and at least 14 wrong targets. PK4's grade passed them.
  - **B2.** 18 annotations sit inside code spans.
  - **B3.** 3 new hrefs are dead relative to their own files.
  - **B4.** A sentence split at `spine.md:26`.
  - **B5.** `tasks.md:1045`, L6's named example, was re-pointed instead of annotated.
  - **B6.** A wrong D-id target.
  - **B7.** Two anchors that exist only in the deleted copy (F12).
  - **B8.** Commit messages untrue: they say ledger links are annotated, and carry line numbers.
  - Held:
    - D5's own parser maps 371 rows onto 337 lines;
    - the dated rows are exact;
    - the repairs resolve;
    - the high-water fix is true;
    - W8 is respected;
    - the gate allows all 99.
- **Lead reading L20:** `FEAT-002/sufficiency-report.md` is dated by substance (`report: review`,
  2026-09-10), so its 30 line cites stay under L5. This is K4's own §2 reading, which its table
  contradicted. Recorded in `…-rulings-2.md`.
- Lesson: the plan grade read the tables. The diff read read the text the tables wrote. Only the
  second sees what lands.
- K4's fix round: tables fixed, then `proposed/` regenerated from `1313c9e`, then the live files
  brought to it by Edit and `cmp`; every href resolved by script.
- K4 reported the fix round: all 8 fixed, N/N3 down to 22 (4 false positives into a rules file
  dropped), 330 rows in all, `97 files changed, 306 insertions(+), 306 deletions(-)`, 0 new dead
  hrefs.
- The lead checked before D5's re-read and sent it back again.
  - `BACKLOG.md:477`/`:481` (B93) cite the signed EPIC-002 contract's `:53`, which still exists,
    unchanged.
  - K4 re-pointed them to product `ipc.md:253`, a heading saying that statement is false. That
    inverts the cite and writes a line number. K4 also kept a line number there on purpose.
  - Every kept N/N3 row is now to be re-verified for which file the cite meant, with the evidence in
    `k4/n-n3-verify.tsv`.
- K4's re-verify: B93's pair dropped, and one more of the same class found
  (`FEAT-001/tasks.md:647`, a cite into EPIC-001's unchanged `data-model.md:378`) and dropped.
  - N/N3 is now 19, and 327 rows in all.
  - `97 files changed, 303 insertions(+), 303 deletions(-)`.
  - Lead: all 99 proposed files are `cmp`-equal to the live tree. The added lines carry no new line
    number; the 4 hits are pre-existing frontmatter text that only gained an annotation.
- D5 is re-reading the delta. A second FAIL goes to the user.

## 2026-10-01 · D5 round 2: all 11 diffs PASS · commit messages fixed by the lead · K4 commit GO

- D5 round 2 (`d5/verdict-r2.md`, `414f214d…`, 153 lines): all 11 diffs PASS.
  - B1 holds: the 19 kept rows were checked by the cited fact at the commit that wrote each cite, and
    the 37 dropped rows are byte-equal to HEAD.
  - 0 annotations sit in code spans, 0 new dead links, and B4–B7 are fixed.
  - The hunk map lands all 327 rows. The gate allows 97 of 97.
  - **R2-B1, messages only:** five messages were untrue of their sub-steps (K4b, c, d, f, g). D5
    gave the exact replacement text.
  - `review: diff K4 kinako wave-4 · D5 · claude-opus-5-5 · 2 rounds · 0 blocking on the diffs`.
- The lead applied D5's exact text, as the lead's own ceremony text, as at K2b. It added a trailer
  to all 11: the rulings pointers and the attribution. The lead checked the text matches and that no
  message carries a `file:line`.
- Gitleaks on the full diff: no leaks, and no personal address. GO to K4 to commit K4a…K4k in order,
  each staged set's shortstat matching its reported figure.
- Booked for kinako's BACKLOG and K5's residue:
  - D5's A2: `FEAT-001/architecture.md:61`'s bare `:1204` line cite, which no census caught.
  - D5's A10, K5 residue: 22 annotated dead hrefs; the old ledger path, annotated, at
    `tasks.md:174`/`:1045`; and the reverted fenced fold-heading text.

## 2026-10-01 · K4 committed · K5 close checks by the lead: clean

- K4 committed K4a…K4k in order (`94434e0`…`73dd17d`): 97 files, +303 −303. The tree is clean.
  Branch total `44635f1..73dd17d`: 15 commits, 443 files, +2438 −39989.
- K5 ran by the lead, read-only, kinako at `73dd17d` (scratchpad `k5/k5-report.md`):
  - **Ledger grep:** 0 unannotated old feature-directory ledger paths outside L7's exclusion set.
    The rest are 7 hits inside the archived ledgers, 30 in excluded closed or frozen files, and 157
    bare names, `BD-` cites and annotated or re-pointed paths (L4, W7). Crates hits are W4's.
  - **Dead-pointer scan** (markdown hrefs and path-qualified backtick paths, fenced blocks
    skipped, W8): 0 dead pointers caused by the pass left unannotated. 3 apparent hits are
    provenance lines whose commit id wraps to the next line. 31 dead targets carry the in-history
    annotation by design.
  - **Pre-existing:** 81 dead pointers in 24 files were already dead at `44635f1` (wrong relative
    depths in text copied from EPIC-001, retired `specs/` and product paths, plugin-cache paths).
    They are outside D5(v)'s moved-or-deleted scope. Booked for kinako's `BACKLOG.md`, not fixed.
    The first count, 135 in 34, was inflated: the scan read a backtick path's `:line` suffix as
    part of the path. The lead fixed the scan (`k5/deadscan2.py`) and re-ran it. Blame shows the
    pass wrote none of the 81; one line, c&d's NFR rows, K3 moved unchanged.
  - **Figures** against `git diff -M --name-status 44635f1 HEAD`: 3 R100 (the ledgers), 339 D
    (327 evidence files and 12 copies: FEAT-001 5, FEAT-002 2, FEAT-006 5) and 101 M. They match
    K0's list.
  - **Gate no-op:** one file per touched home, current content, under `mochiko-cli` 0.3.0 on
    0.116.0: 16 of 16 allow.
  - `.mochiko/runs/` is absent (L11).
- Next: the kinako landing (B119 closed, B121 narrowed, B132 updated, the new items, the ROADMAP
  run-4 row; `brainstorms/index.md` untouched, L10). Then the PR, on the user's word, merged with a
  merge commit. Then the mochiko landing.

## 2026-10-01 · Kinako landing written by the lead · D6 FAIL (4 blocking) · fix round

- The lead wrote kinako's landing, uncommitted:
  - a per-decision record, `.mochiko/decisions/2026-10-01-mochiko-0116-cleanup-pass.md`, and its
    `DECISIONS.md` row;
  - B119 moved to the trail, superseded;
  - B121 narrowed to its three open limbs, since 0034 discharges the bounds limb;
  - B132's README limb done;
  - B144 (code comments, W4/L17), B145 (stale counts and the `:1204` cite) and B146 (the 81
    pre-existing dead pointers) opened;
  - the ROADMAP run-4 row and stamp touched. The brainstorms index is untouched (L10).
- D6 is a fresh non-author diff read (`d6/verdict.md`). FAIL, with 4 blocking and 9 advisory:
  - B1: B144 booked only `corpus/settings.rs`'s two ids, but L17 covers run 4's ids in at least
    14 files;
  - B2: the ROADMAP gave §A1–A4's date as 2026-10-01, but K1 is dated 2026-09-30;
  - B3: B146 named a `specs/FEATURES.md` that no commit ever held;
  - B4: the untracked record escaped the scan, and its two mochiko cites missed the `mochiko:`
    form.
- The lead fixed all four and took A1–A6 and A8:
  - B132's BD-236 limb now reads as the principal's call;
  - B145 names BoundSession's 8 fields;
  - B138 gains a dated 0034 note;
  - the trail names the three evidence trees, with D3 and census row R1;
  - B146's dated-report handling is marked unruled.
- A7 goes to the user: commit the mochiko rulings records before the kinako PR merges. A9 is not
  taken, with B132 as the precedent.
- Checks after the fixes:
  - the scan stays at 115, and the record alone scans 0;
  - every item is ≤15 lines;
  - the gate allows all five files.
- Round 2 is with D6.

## 2026-10-01 · D6 round 2 PASS · kinako landing committed `345ff0f`

- D6 round 2 (`d6/verdict-r2.md`): PASS, 0 blocking.
  - Every fix checks out first-hand.
  - The scan, widened to the untracked record, reads 115 with no hit in it.
  - The gate allows all five files, and every touched item is ≤15 lines.
  - N1 taken: the ROADMAP stamp adds "B138 noted". N2 (C-003 is reserved, not minted) needs no
    change.
- Gitleaks on the landing diff and the record: no leaks, and no personal address.
- Committed `345ff0f` on `mochiko-0.116-cleanup`: 5 files, +139 −24, tree clean. Branch total from
  `44635f1`: 16 commits.
- Owed: the PR to kinako `main` (the user's word; a merge commit, never squash), and the mochiko
  landing, with the two rulings records committed before the PR merges (D6's A7).

## 2026-10-01 · Kinako PR #24 opened · mochiko landing written by the lead

- The user ruled "Open PR now" and "Commit + open PR", both as recommended.
- Kinako `main` was still at `44635f1` after a fetch. The lead pushed `mochiko-0.116-cleanup` and
  opened humaninloop-dev/kinako#24, with 16 commits: 446 files, +2572 −40008 (3 R, 339 D, 1 A,
  103 M). The body's first line asks for a merge commit, never a squash. The user merges.
- The mochiko landing is uncommitted on `joint-hook-delta-w4`:
  - `DECISIONS.md`: rows R9, R10, R11, the census and the seams now read "wave 4, kinako, built
    2026-10-01", kinako PR #24. The delta row's D5 and the field review's wave 4 read the same.
    The 2026-09-30 rulings row is widened to W1–W7 and L1–L19, built. A 2026-10-01 row is added
    for rulings-2 (W8, W9, L20), built.
  - `BACKLOG.md`: both build items go to the trail as DONE. The publish limb stays at *what is
    left* item 1, whose item 3 now reads done. New: the D10/S15 watch item, and a process-findings
    item (the `bash <script>` gap, the scratch-repo deny, plan grades missing diff-read defects).
  - Both index entries and the ROADMAP Next row now read wave 4 BUILT 2026-10-01.
  - Both rulings records' Status lines now read "Built 2026-10-01". The first stays at 149 lines.
- The gate allows all eight files. *What is left* is 16 lines, one under its pre-landing 17; it was
  already over the cap.
- Next: D7, a fresh non-author read of the mochiko landing diff. Then the commit and the PR.

## 2026-10-01 · D7 FAIL on the mochiko landing (5 blocking) · fix round

- D7 (`d7/verdict.md`): FAIL, with 5 blocking and 10 advisory findings.
  - B1/B2: the seams and census records' Status lines still read wave 4 as owed or future, against
    their rows.
  - B3: the trail's "every link re-pointed" was false, because W4 left the code-comment links.
  - B4: OQ1's entry sizes were off by one; the gate counts 122 and 85.
  - B5: *what is left*'s "0.109.0 … not released" line, which D9(d) ruled corrected, was never
    corrected; 0.109.0 has been on `main` since `2f92b09` (2026-09-19).
  - D7 found the 2026-09-30 row's rewording acceptable: it corrects a false count and adds no
    ruling.
- The lead fixed all five, with D7's text. It took A1–A7:
  - finding (3) scoped to K1, K2b, K3 and K4;
  - finding (2) restated as a write deny and a note, not a rule;
  - the trail line names the write steps and the full `-2.md` path, and separates evidence
    deletion from the moves;
  - *what is left* item 3 is one line, so the item is 15;
  - the watch's baseline is kinako `main` after the merge commit, its title says "S15's metrics
    restored", and it covers "0.116.0 or later".
- A8 is left as ruled, since the trail reconciles the 17. A9: `wave4-kinako-pass.md` will be
  committed with the records. A10 is not taken, to keep item 1 within its lines.
- The gate allows every edited file. No "kinako, owed" phrase is left in DECISIONS, the index,
  ROADMAP or the records. Round 2 is with D7.

## 2026-10-01 · D7 round 2 FAIL (1, its own text) · user-ruled delta check · round 3 PASS · commit

- D7 round 2 (`d7/verdict-r2.md`): every round-1 fix holds, but one blocking finding remains.
  - The fault was D7's own B3 text: "every link in kinako's records". L7's closed and frozen records
    still carry 30 old ledger links, by design.
- A second FAIL goes to the user. The user ruled "Apply D7's text + delta check", as recommended.
  - The lead applied the text word for word ("live, writable records … closed and frozen records
    left as written, L7").
  - The lead also rewrapped process finding (2)'s split code span.
- D7 round 3 (`d7/verdict-r3.md`): PASS, 0 blocking. The diff hashes are unchanged elsewhere.
  - `review: diff mochiko-landing wave-4 · D7 · claude-opus-5-5 · 3 rounds · 0 blocking`.
- Gitleaks on the full mochiko landing (diff and untracked files): no leaks, and no personal
  address. There is no `plugin.json` bump, since the pass found no primitive defect.
- The lead commits the landing on `joint-hook-delta-w4`. The commit carries `wave4-kinako-pass.md`
  and both rulings records, so kinako's cites resolve before PR #24 merges. The lead then opens the
  mochiko PR. **Wave 4 is closed** except for the user's merges and the D10 watch.
