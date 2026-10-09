---
report: review
wave: 1
seat: G1
reviewer: plain general-purpose seat (opus), independent non-author code review per `.claude/rules/mochiko/rust-cli.md`; authored nothing in the diff
reviewed: >-
  `crates/mochiko-cli` 0.2.0 to 0.3.0 on branch `joint-hook-delta`, the uncommitted tree over
  `bbbcf10`: the tracked diff (21 files) plus untracked `src/shell.rs`, `tests/shell.rs` and
  `reports/w1-shell-census.md`; against the wave plan §2, §3, §5, §7, §8, S1 `plan-v3.md`, S2
  `plan-v2.md`, every 2026-09-29 build-log ruling, record D3/D4/D5/D7 as amended, S3, S4, S6/V9,
  S7/V2, S12, S15, V1–V4, V7, N2, N4, the seams record R5/R7, ledger GI-019-kernel-tooling-admission "Reach of the gate" and
  "What the gate reads", and `rust-cli.md`
verdict: FAIL
blocking_count: 2
advisory_count: 10
gates_run_by_g1:
  cargo_test_all: "18 `test result: ok` lines (15 test files, 2 unit targets, doc-tests), 556 passed, 0 failed (CARGO_TARGET_DIR=target/g1)"
  cargo_fmt_check: "exit 0, no output"
  cargo_clippy: "Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.44s; exit 0"
  cargo_audit: "Scanning Cargo.lock for vulnerabilities (31 crate dependencies); exit 0"
  migrate_validate: "mochiko-cli migrate validate · 0 rejecting · 113 advisory (today's log, built binary)"
  version: "mochiko-cli 0.3.0 · grammar 1..2"
  views: "views emit · 80 documents; diff -rq against .mochiko/schema-views empty"
  head_baseline: >-
    HEAD's crate (`git archive HEAD`, built under target/g1/head, `mochiko-cli 0.2.0 · grammar
    1..1`) driven side by side with the built binary over the real log, for the differential below
blocking:
  - id: B1
    where: "crates/mochiko-cli/src/shell.rs:119-122, :220-230 (`target_directory`)"
    wrong: >-
      A `-t` inside a short-option cluster is not read as the target directory: `cp -rt <home-dir>
      a.md b.md`, `cp -vt <home-dir> a.md`, `cp -at <home-dir> a.md` and `install -Dt <home-dir>
      a.md` return only the last source, so the write into the home is allowed. HEAD denied all
      four (its cp/install arm took the first non-flag argument too), confirmed by running both
      binaries on the same payloads. The lead's Q2 ruling brought `-t DIR` in precisely because
      "(i) avoids a regression"; the built form covers `-t DIR`, `-tDIR` and the long option only,
      so the regression the ruling meant to prevent survives in the clustered spelling (GNU `cp`
      and `install` accept it). A silent deny-to-allow change on a real write, not in any plan.
    fix: >-
      In `target_directory`, also scan a single-dash cluster: at its first `t`, the rest of the
      cluster, or when empty the next word, is the directory; stop the scan at a value-taking flag
      first (`install`'s `m` `o` `g` `S`, `cp`'s `S`). Add rows returning exactly the directory for
      `cp -rt D a b`, `cp -vtD a`, `install -Dt D a`, and a read row for `install -m 644 -t D a`.
  - id: B2
    where: >-
      crates/mochiko-cli/src/shell.rs:148-162 (the closed wrapper set), :563-566 with :470-472 (an
      unquoted paren ends the simple command), and reports/w1-shell-census.md:333-335 (Notes)
    wrong: >-
      Two classes of write the old any-word scan caught are now allowed, and the census discloses
      neither; its Notes list only tightenings and call the two residual misses "the same before
      and after", which reads as no catch lost. (a) A wrapper outside the ruled set hides its
      command word: `watch tee <home>`, `strace -o /tmp/x tee <home>` and `parallel tee <home> :::
      a` were denied at HEAD and are allowed now. (b) A process substitution among `tee`'s
      arguments ends `tee`'s run: `echo x | tee >(cat) <home>` was denied at HEAD and is allowed
      now, because the `(` after `>` is a separator and the home path lands in a new simple command
      with no arm. Both follow from ruled design (the named wrapper set, the paren-as-operator
      ruling), but the losses are silent.
    fix: >-
      Either close (b) in the tokenizer (lex `>(`/`<(` to the matching `)` as its own simple
      command and resume the outer word run after it) or record it; and record (a) as a class. One
      Notes line in the census for each, and pinning rows in tests/shell.rs asserting today's
      result for `watch tee <home>` and `tee >(cat) <home>`, so a later change is visible.
advisories:
  - id: A1
    where: "crates/mochiko-cli/src/shell.rs:264-280"
    note: >-
      `find_exec_targets` recurses through `arm_targets` without a bound; 20,000 nested `find .
      -exec` words overflow the stack and abort (5,000 do not). The wrapper fails open, so the cost
      is an allow on crafted input only. Fix: cap the depth or walk iteratively.
  - id: A2
    where: "crates/mochiko-cli/src/conform.rs:352-372"
    note: >-
      For `.mochiko/runs` itself, and for a run folder named as a file (`.mochiko/runs/FEAT-001-run5`),
      the closed-world reason prints "`` is not a run key: a run folder's name must match ``".
      Skip the run-key sentence when the next segment is absent or matches its pattern.
  - id: A3
    where: "crates/mochiko-cli/src/hook.rs:366-372"
    note: >-
      The cwd join that closes evasion 3 has a mirror false deny, because `cd` is not followed:
      from a cwd inside `.mochiko/`, `cd /tmp && echo x > y.log` and `(( 3 > 2 ))` now deny (HEAD
      allowed). Inherent to the ruling; disclose it beside evasion 1 in the census Notes or the
      wave-3 prose.
  - id: A4
    where: "crates/mochiko-cli/src/shell.rs:503"
    note: >-
      `#` comments are not lexed: an apostrophe in a comment line (`# don't`) opens a quote that
      swallows every later line, so a real home write after it is allowed; a `>` in a comment reads
      as a redirect. Same at HEAD. Fix: a word-initial `#` in the POSIX dialect ends the line.
  - id: A5
    where: "crates/mochiko-cli/src/hook.rs:219-227"
    note: >-
      The report sniff now reaches every path on disk: a report-frontmatter `.md` Write to the
      session scratchpad or under `~/.claude/` denies, where HEAD allowed any path outside the cwd.
      Planned (S1 plan §2.1 step 5, "sniff only"), not in the build log's disclosures; wave-3 prose
      should point drafts at `check --path --content -`.
  - id: A6
    where: "crates/mochiko-cli/src/hook.rs:366-372"
    note: >-
      A shell payload with no `cwd` now allows a relative home target (HEAD denied). The platform
      always sends `cwd`, and the contract fixtures carry it; plan step 6's "as today" held for the
      file leg only.
  - id: A7
    where: "crates/mochiko-cli/src/shell.rs:391"
    note: >-
      A perl cluster that opens with `l` or `0` (`perl -lpi -e … <home>`) stops the scan before its
      `i`, so the in-place edit is allowed. `-l` and `-0` take only digits; skip those, not the
      rest of the cluster.
  - id: A8
    where: "crates/mochiko-cli/src/home.rs:530-535"
    note: >-
      Segment matching is case-sensitive, so on macOS's case-insensitive filesystem
      `.MOCHIKO/features/FEAT-001/x.md` escapes both the homes and the closed world. Pre-existing
      for homes; the closed world inherits it.
  - id: A9
    where: "crates/mochiko-cli/src/hook.rs:175"
    note: >-
      The on-disk file is read for every Write before resolution, now for any path on disk (HEAD:
      only under the cwd). Inside the ledger's read class (drift (b) is booked), but the read is
      needed only for Edit and for in-home amnesty; reading after resolution trims its reach.
  - id: A10
    where: "crates/mochiko-cli/src/render.rs:595-598"
    note: >-
      `home <home>/reports/` renders "`reports` is NOT a declared deliverable" rather than the
      home's reports directory. Not a render target; cosmetic.
clauses_walked:
  D7_S3_resolution: >-
    PASS. `home::locate` walks `ancestors()` for `.git` (dir or pointer file); only a first segment
    of `.mochiko` is a home tree; `check` and `home` share `absolute` + `locate`. The eight-shape
    matrix (tests/hook.rs) covers root-relative, absolute from a sub-directory cwd, relative from a
    sub-directory, relative inside a worktree home, absolute, nested fixture (allow), other tree
    (deny), no `.git` (allow); `home` relative via `run_binary` and absolute.
  S4_directory_home: "PASS. Ten targets against the real log, each with and without `/`, tail line asserted."
  D3_closed_world: >-
    PASS. Write, Edit on an existing file, Bash and PowerShell deny with both routes; `home` prints
    "`.mochiko/` is closed, so a write here is refused" and never "nothing here is checked" under
    `.mochiko/`; openers "not a declared sub-directory" / "not a declared deliverable" pinned; the
    "takes a migration" sentence gone from the sub-directory reason; no run folder invented when the
    log declares none.
  D4_controls: >-
    PASS. `<run-id>` per R5 (accept and reject rows); four ignore spellings, comment, negation,
    glob and parent rejected; report sniff on `.md` in `runs/` and on every Deferred path (Q2 B);
    `implement-log.md` allowed by name; worktree write refused naming the main tree's folder from
    the pointer (absolute, relative, submodule and junk pointers pinned).
  D5_write_positions: >-
    PASS except B1 and B2. Command-word matching, the twelve-word wrapper set, reserved words,
    POSIX-only parens, `$` `{}` `~` dropped, sed/perl script words dropped, `mv` both ends, `git mv`,
    the four tightenings; the 25 census rows asserted exactly (17 name no home, 8 keep theirs).
  runs_carve: "PASS. Non-`.md` allowed; `.md`, `implement-log.md`, `dir/` and the run folder itself denied; guard and main-tree rule kept."
  evasions: "PASS. Evasion 3 denies at `decide`; 1 and 2 allow, 2 confirmed on the landed parse."
  V9_entries: >-
    PASS. `##`/`###` boundaries, fence, exempt fields (both spellings), `section_max_lines`, no
    whole-file bound, amnesty per entry (grow denies, add passes, rename is a new key and says so),
    template shape kept and its section budgets dropped (Q6).
  grammar_2: "PASS. Range (1, 2); grammar 3 halts; grammar-1 home change carrying any of the five fields rejected by name; validate arms incl. Q7 and the duplicate raw-output home."
  dry_run: "PASS. `check --path --content -`: exit 0/4, one-line JSON, relative path from the process cwd, both exclusive-form usage errors exit 2."
  render_shape: "PASS. home head and tail format strings unchanged; byte test on a directory path; other renders untouched."
  reads: >-
    PASS. The check path reads only the payload, the log, the on-disk file (A9), `<ancestor>/.git`
    existence, the worktree pointer (only when a raw-output home is declared) and `.gitignore` (only
    inside `run_folder_refusal`). No network call; dependencies unchanged.
disclosures_checked:
  not_red_first: >-
    Confirmed against `git show HEAD:` source: the T3 hook closed-world tests fail at HEAD (Write of
    `.mochiko/evidence/console.log` is Outside, sniffed, allowed; the sub-directory reason has no
    run folder), the cli closed-world test fails (HEAD prints "nothing here is checked"), the
    shell-deny run-folder test fails (HEAD's shell reason names no run folder). The worktree-pointer
    pin and the corrected fence test can fail: without the fence skip `### User` splits into 2 and
    3 lines and the test's expected "5 lines" deny disappears.
  build_time_choices: >-
    Accepted as reasonable: a log with no home closes no world (matches the shell leg's early
    return); `home`'s directory reading yields only to File and Report; worktree and guard checks on
    `implement-log.md` follow V3 and control 3 literally.
  single_writer: >-
    `src/shell.rs` mtime 08:54:03 is after S1's 08:47:39 fmt; the landed file carries every S2 plan
    v2 item and each later ruling (A1 wrappers, A2 suffix, A4 expansions, A6 POSIX-only parens); no
    old table function remains in `hook.rs`; `rustfmt --check` clean. Nothing lost.
  no_stat_gap: "Confirmed as built and disclosed (`cp x <run>/sub` without a slash allowed); lead-accepted."
re_review_round_1:
  date: 2026-09-29
  bound: "`common.gate-loop-bound`: the one re-review after the one fix round, same seat"
  scope: >-
    The fix delta on the same uncommitted tree: `src/shell.rs`, `tests/shell.rs`, `src/hook.rs`,
    `src/conform.rs`, `tests/hook.rs` and `reports/w1-shell-census.md`, read whole against the round-1
    cites and the build-log entries "G1 code review round 1", "fix round — A3 refined on the sweep"
    and the S1/S2 closes; every round-1 probe, the HEAD-vs-new differential and new probes aimed at
    the `cd` join and the `.mochiko`-first keep, re-run on the rebuilt binary.
  verdict: PASS
  gates_run_by_g1:
    cargo_test_all: "18 `test result: ok` lines, 570 passed, 0 failed (CARGO_TARGET_DIR=target/g1)"
    cargo_fmt_check: "exit 0, no output"
    cargo_clippy: "Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.33s; exit 0"
    cargo_audit: "Scanning Cargo.lock for vulnerabilities (31 crate dependencies); exit 0"
    migrate_validate: "mochiko-cli migrate validate · 0 rejecting · 113 advisory"
    version: "mochiko-cli 0.3.0 · grammar 1..2"
    views: "views emit · 80 documents; diff -rq against .mochiko/schema-views empty"
    census_dry_run: "`check --path` on w1-shell-census.md: allow; K02/K05/K11 `targets_now` match their tests/shell.rs rows"
  closed:
    B1: >-
      Closed. `target_directory` (shell.rs:332-361) scans a single-dash cluster for `t` and stops at
      `m` `o` `g` `S`; the differential no longer lists `cp -rt`, `-vt`, `-at` or `install -Dt`, and
      the probe's `cp -rt <home-dir> a.md b.md` denies. Rows at tests/shell.rs:684-694, including
      `install -m 644 -t` and `install -mt`.
    B2: >-
      Closed. (b) `Token::Substitution` lexes `>(…)`/`<(…)` as its own command and resumes the outer
      run: `tee >(cat) <home>` names the home again (tests/shell.rs:697-710). (a) Recorded as a class
      in the census Notes, pinned by `watch tee <home>` returning none (:713-716); the differential
      still shows `watch`, `strace -o f` and `parallel` allowing, as disclosed.
    A1: "Closed. `MAX_DEPTH = 64` on `find -exec` and process substitution; 20,000 nested levels allow with exit 0, no abort."
    A2: "Closed. No segment past the run key prints route 1 against the raw-output home, not empty backticks."
    A3: >-
      Closed as re-ruled. `cd /tmp && echo x > y.log` and `(( 3 > 2 ))` from a cwd inside `.mochiko/`
      allow; an absolute literal `cd` joins later relative targets; `cd $X` keeps only
      `.mochiko`-first targets; a `cd` inside `( … )` holds to its `)`.
    A4: "Closed. `# don't` then a home write denies; a `>` after a word-initial `#` names nothing; `a#b` stays literal."
    A5: "Closed as ruled. A report-shaped `.md` Write to the scratchpad or `~/.claude/` allows; a path in a tree outside `.mochiko/` is still sniffed."
    A7: "Closed. `perl -lpi -e` and `-0777pi` are in place; `perl -l -ne` is a read."
    A9: "Closed. The on-disk file is read only for an `Edit` or a File/Report/UndeclaredFile resolution; a no-tree path returns before any read."
    accepted_no_change: "A6, A8 and A10, by lead ruling."
  new_blocking: none
  new_advisories:
    - id: R1
      where: "crates/mochiko-cli/src/shell.rs:187-209 (`change_directory`)"
      note: >-
        Three `cd` shapes are misread, each a narrow false allow: `pushd /abs && popd && echo x >
        <home>` (popd leaves the directory unknown, so the target drops; disclosed in the census Notes
        and lead-confirmed); `cd /tmp | tee <home>` and `cd /tmp & …` (a pipeline or background `cd`
        read as holding; disclosed only in the module doc); `cd -` after an absolute `cd`. None
        appears in the 48,284-command sweep.
    - id: R2
      where: "crates/mochiko-cli/src/shell.rs:746-751"
      note: >-
        Every `((` is lexed as arithmetic, so bash's nested-subshell reading, `((echo a); tee <home>)`,
        becomes one word and its `tee` is lost (round 1 denied it). An unterminated `((` swallows the
        rest of the line, but bash refuses that line anyway, so nothing is written.
    - id: R3
      where: "crates/mochiko-cli/src/shell.rs:187-209"
      note: >-
        A relative literal `cd` could be stacked on the start directory (return `<operand>/<target>`
        for the hook's cwd join) rather than dropped. That would close evasion 1 for a relative operand
        and the `cd crates && … > ../.mochiko/…` shape, which today skips even the run folder's ignore
        guard and the `.md` carve. Ruled as a drop; recorded for a later wave.
    - id: R4
      where: "crates/mochiko-cli/src/hook.rs:366-372 with shell.rs:172-175"
      note: >-
        A worktree seat running `cd $MAIN && cmd | tee .mochiko/runs/<run-id>/t.log` is refused by
        the V3 worktree rule, because the kept target joins the worktree cwd. This is a case of the
        accepted `cd $X` false deny, and the reason names the main tree's absolute path to use.
    - id: R5
      where: "crates/mochiko-cli/src/hook.rs:171-176"
      note: >-
        With A5 on top of D7, a consumer project with no `.git` anywhere gets no gate at all, where
        HEAD gated it through the cwd. This follows from N4 and the ruling. It belongs with the
        booked wave-3 note that the contract workspaces need a `.git`.
---

## Failure narrative

The four crate layers pass as run here, `migrate validate` rejects nothing on today's log, and every
clause S1 built (resolution, closed world, run-folder controls, entry budgets, grammar 2, dry run,
render shape) meets its ruling. The FAIL is in the shell parse. Running HEAD's binary beside the
built one on write-shaped commands into a declared home turned up writes HEAD denied that are now
allowed. One contradicts a ruling outright: a clustered `-t` (`cp -rt`, `install -Dt`), the flag
Q2 brought in to avoid exactly this regression (B1). The rest follow from ruled design but go
unrecorded: wrappers outside the named set, and a process substitution among `tee`'s arguments
(B2). The census tells a reader no catch was lost. B1 needs a code fix plus rows; B2 needs
disclosure and pinning rows, or a tokenizer fix for the process-substitution case. Both are in S2's
write set. S1's files need no change for this verdict.

## Notes of note

- The differential used today's real log and a scratch tree with its own `.git`, under the
  session scratchpad; no binary was installed. Besides this file, the only repo writes are under
  the gitignored `target/g1/` (build outputs, HEAD's extracted crate, the views emit).
- The reads audit found nothing outside GI-019-kernel-tooling-admission's list. A9 (Write reads the file before it is
  resolved) sits inside drift (b), which is already booked for the wave-3 text-vs-build check.
- The census count holds: 25 rows, each asserted on its exact targets. The re-extraction from the
  transcripts was not repeated; P2 re-derived 79/21 in the plan round.
- The fix round is bound by `common.gate-loop-bound`: one round, then a re-review by this seat.
