---
report: review
feature: hook-enforced-artifact-schema
round: 1
verdict: PASS
verdict_history:
  first_pass: FAIL — 10 findings, 1 Critical, 3 Important, 6 Minor; graded before the supersession notice
  frozen_tree: FAIL — 9 open, 1 withdrawn (G1, the fmt gate); blocking G2 and G4; see the `## Re-grade — the frozen tree` section
  after_fix_round: PASS — all 10 held, 0 not held; G8 repaired by the lead in the record; see the `## Re-grade — after the fix round` section
note_on_item_grades: >
  The `item_grades` and `findings` blocks below are the first-pass record and are left unrevised.
  Item 1 flips to PASS on the frozen tree and G1 is withdrawn; the per-finding frozen-tree
  dispositions live in the re-grade section at the end of this file.
graded_artifact: the uncommitted wave-1 crate work under crates/mochiko-cli/
graded_against: wave1-plan.md (approved, §3 steps 3/8 superseded by the explicit-allow open) + record.md D3, D4, D6, D9, D10 + .claude/rules/mochiko/rust-cli.md
reviewer_role: standing non-author reviewer; authored none of the graded code
producer_report: wave1-reports/cycle-report.md
fix_list_count: 10
blocking: [G1, G2, G4]
nothing_edited_under: [crates/, plugins/]
note_on_the_producer_report: >
  The cycle report was revised while this review was running — tests_added 94 to 98, tests_total 445
  to 449, and a new "The shared fixture log" section. This review grades the tree and the report as
  they stand after that revision. The fmt failure at G1 lives in the four tests that revision added.

item_grades:
  - id: 1
    name: gates
    grade: FAIL
    verdict: Three of four gates are green on my own run; `cargo fmt --all --check` fails, and the report records it as PASS.
    commands_run:
      - "cargo test --all — exit 0, 17 suites, 449 passed, 0 failed (matches the revised report's 449)"
      - "cargo fmt --all --check — FAILS: `Diff in crates/mochiko-cli/tests/home.rs:1154` and `:1189`, 29 lines of diff"
      - "cargo clippy --all-targets -- -D warnings — exit 0, no warnings"
      - "cargo audit --deny warnings — exit 0, 1243 advisories loaded, 31 crate dependencies scanned"
      - "git status --short plugins/mochiko — 0 lines; git diff --stat plugins/ — 0 lines (byte-identity holds)"
    detail: >
      The two fmt diffs are both inside `the_shared_fixture_log_covers_every_branch_the_contract_suite_reads`,
      one of the four tests the mid-review revision added, so the report's PASS was true when it was
      written and is false now. Nothing else about the gate story moved: no dependency was added, the
      audit surface is unchanged at 31 crates, and no file under `plugins/mochiko/` differs by a byte.
    findings: [G1]
  - id: 2
    name: contract conformance to D3/D4/D6
    grade: FAIL
    verdict: The eight-step algorithm, the exit-code contract and five of the six checks are built as ruled; the placeholder check is wider than D4c and false-denies a conforming artifact.
    verified_myself:
      algorithm_order: >
        `run_check` (cli.rs:721-767) parses the payload before loading the log, so a malformed payload
        reports as the usage error it is — plan §3 steps 1 and 2 in that order. `decide` (hook.rs:101)
        branches Write/Edit, Bash/PowerShell, and everything else to an explicit allow, so `Read`
        takes no branch as the plan requires.
      exit_4_minted_and_sole: >
        `EXIT_CONFORMANCE = 4` (hook.rs:37) is returned only by `Outcome::Deny` (hook.rs:83), reached
        only through `run_check`. Grep over src/ finds no other path to 4. Empirically: exit 4 on an
        undeclared name, exit 0 on a conforming write, exit 1 on an empty log dir, exit 2 on a
        non-JSON payload, exit 3 on grammar skew (the last from the suite's own case).
      explicit_allow_and_silence: >
        Every exit-0 path renders `{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"allow"}}`
        — I ran the conforming-write, no-home, ordinary-shell, other-tool and outside-cwd shapes. Exits
        1 and 2 wrote 0 bytes to stdout on my own runs, with the message on stderr.
      edit_in_memory: >
        `apply_edit` (conform.rs:127) replaces the first occurrence only and the applied result is what
        the checks grade; an append that crosses the whole-file bound denies and one that stays inside
        allows (tests/hook.rs:222-242, re-read).
      amnesty: >
        `settle` (conform.rs:228) excuses a candidate fault only where the baseline carries the same
        `key` and `not_worsened_from` holds, and reports the standing overage as `additionalContext`.
        A new file has no baseline, so it takes the budget outright. Worsened denies, improved allows,
        and the `Write` and `Edit` legs reach the same function — all four asserted and re-read.
      counting_rule: >
        `heading_spans` (conform.rs:558) counts from a `##` line to the next `##` with `###` inside the
        span, and toggles a fence flag so a `##` inside ``` or ~~~ is not a heading. Both legs tested,
        the fence case with an unfenced control.
      placeholders: exact-token containment, no pattern matching — but see G2 for the scope defect.
      bounds_elsewhere: >
        Size is skipped for `Bounds::Elsewhere` on both the templated path (conform.rs:346) and the
        whole-file path (conform.rs:283). Proven end to end against the shared fixture: a 5,000-line
        file at a declared name allows, an undeclared name still denies.
      form_log: >
        `log_faults` (conform.rs:304) measures each `##` entry against `entry_max_lines` and applies no
        whole-file bound — six entries allow, one fat entry denies.
      d9_sniff: >
        `sniff` (conform.rs:189) fires only when the frontmatter's `report:` value is in the union of
        every home's envelope enum; a plain `.md`, a non-report frontmatter, and an unlisted type all
        allow. Proven through the hook as well as at the unit.
      shell_parse: >
        The wave-0 probe line denies. I also confirmed `>`, `>>`, `tee`, `tee -a`, `sed -i`, heredoc
        with its redirect, `cp` into a home, `mv` out of a home, `2>`, `&>`, `dd of=` and `install` all
        deny, while `git status --porcelain`, a `cat`, a `grep`, a redirect to `/tmp` and a redirect of
        grep output outside the tree all allow. The `mv`-out-of-a-home deny looks over-broad but is
        exactly what the approved plan's §6 matrix asks for, so it is correct as ruled.
    findings: [G2, G5]
  - id: 3
    name: deviations from the plan
    grade: PASS
    verdict: All five named deviations are sound; two further deviations exist that the report does not name.
    rulings:
      kinds_not_shipped_yet: >
        OK. tests/validate.rs — the constant is asserted in both directions, so the day wave 3 lands a
        home document the second assert trips and the constant must shrink. This is stronger than the
        assert it replaced, not weaker.
      home_shape_fifth_code: >
        OK, and required. `Homes::load` skips an undecodable home at delivery time by design
        (home.rs:299), so without a rejecting finding at authoring time a malformed home would be
        silently absent — the GI-005 class. `validate_homes` reports it with the decoder's own error.
      six_resolution_variants: >
        OK. Each variant is a distinct sentence a deny reason must be able to say, and `Deferred` vs
        `UndeclaredSubdir` is a real distinction: one is a declared sub-directory awaiting its own home
        (allowed, D4f), the other is a denial.
      dispatch_io: >
        OK. `dispatch` keeps its signature and delegates to `dispatch_io` with the process stdin, so the
        exit-code contract is driven in-process like every other CLI test and no binary is spawned.
      serde_norway_over_serde_json: >
        OK, and I checked both halves the brief named. Parsing: an escaped forward slash resolves to the
        plain path, proven discriminatingly — a `\/`-escaped path to an *undeclared* name inside a home
        exits 4, which it could only do by resolving into that home. Multi-line escaped content with
        quotes and backslashes parses, and unknown fields are ignored. Emitting: `escape` (hook.rs:139)
        covers the JSON short forms and every remaining code point below 0x20 as `\uXXXX`, which is
        exhaustive over the control range; multi-byte UTF-8 passes through literally and the output is
        one line. A deny reason carrying a raw control character round-trips through `json.load`.
      seven_of_nine_tasks_kept: >
        OK. The stated reason holds against the dispatch rule: tasks 5 and 6 share the `Fault` type and
        the fault key *is* the amnesty mechanism, so the brief would have cost more than the code. Both
        dispatches that did go down are disclosed with their read-back, and one of them names a real
        blocker the worker found.
    findings: [G6, G7]
  - id: 4
    name: test honesty
    grade: FAIL
    verdict: The matrices are broad and the disclosed self-corrections check out, but two rows pass for a reason other than the rule they name.
    verified_myself:
      disclosed_corrections_are_real: >
        The report admits two bodies initially tripped the size budget instead of the rule under test.
        Both now carry the comment that proves the fix was a tightening rather than a loosening —
        tests/conform.rs:269-271 ("the whole `## Intent` span is 3 lines, inside its budget of 4") and
        :281 ("Four lines from `## Intent` to EOF: inside the budget"). No assertion was weakened.
      unfenced_control_legs: >
        Present. tests/conform.rs:291-297 states why it exists — without it the fence case would pass
        on a checker that ignored headings entirely — and asserts the deny names `## Invented`.
      exit_code_matrix: >
        One case per code, 0 through 4, plus a property test that stdout carries JSON only when the
        binary could read its log (tests/cli.rs, five named cases re-read).
      amnesty_matrix: new-file, unchanged, worsened, improved, standing-heading and both-legs rows all present and discriminating.
    findings: [G4]
  - id: 5
    name: bright line
    grade: PASS
    verdict: Every check is a string or a count against log data, and no code path reads a schema file.
    evidence: >
      Grep over the three new modules for `read_to_string`, `include_str`, `include_bytes`,
      `File::open` and `fs::read` returns exactly one hit: hook.rs:174, reading the on-disk artifact as
      the amnesty baseline — the artifact under test, not a rule source. No `schemas/` path and no
      `.yaml` literal appears in home.rs, conform.rs or hook.rs. Homes, templates, conformance blocks,
      budgets and the report enum all come off the replayed state. Nothing in `conform` reads meaning,
      grades quality, or sequences a seat: the six checks are set membership, key presence, list
      membership, string containment and line counts.
  - id: 6
    name: fixture log covers the four contract-suite branches
    grade: PASS
    verdict: All four branches present, each able to both allow and deny, with a structural test that fails the build if one is dropped.
    evidence: >
      tests/fixtures/home-log/0001-homes.yaml declares five homes and two templates and replays clean.
      Branch 1 is `feature/spec.md` bound to `demo-spec` with per-section `max_lines`, required and
      enumerated frontmatter and three placeholder tokens; branch 2 is `feature/gates.md` at
      `max_lines: 40`; branch 3 is `feature`'s `reports.envelope` with a `report:` enum; branch 4 is
      `memory` at `bounds: elsewhere` with its `bounds_cite`. `the_shared_fixture_log_covers_every_branch_the_contract_suite_reads`
      asserts each by its distinguishing property rather than by name, and
      `the_shared_fixture_log_denies_and_allows_on_each_branch` exercises an allow and a deny on each —
      including the 5,000-line file that branch 4 must allow. I ran my own checks against this log for
      every exit code and both shell verdicts.
  - id: 7
    name: cost
    grade: PASS
    verdict: The 36 ms figure reproduces; the sentence explaining it does not, and the correction matters for wave 3.
    my_measurement: >
      Release binary, 40 to 50 invocations per row, median, same box:
      `check --hook-json -` against the real 4-migration log — 35.5 ms and 37.5 ms on two passes
      (report: 36) · against the 1-migration shared fixture log — 3.4 and 3.7 ms ·
      `home <path>` against the fixture log — 3.5 ms · `migrate status` as a referent — 41.2 ms ·
      `mochiko-cli --version`, which is process start and nothing else — 3.3 ms ·
      `/usr/bin/true` spawn floor — 2.1 ms. CPU for one real-log check, via `/usr/bin/time -p`:
      real 0.04, user 0.03.
    reading: >
      The absolute number is right and the 100 ms cache trigger is unmet, so "no cache this wave"
      stands. But the cost is replay, not process start, and it scales with log size — see G3.
    findings: [G3]

findings:
  - {id: G1, type: gate, sev: Critical,
     at: "crates/mochiko-cli/tests/home.rs:1154 and :1189; cycle-report.md frontmatter `gate_cargo_fmt: PASS`",
     gap: "`cargo fmt --all --check` exits non-zero on the current tree, 29 lines of diff across two closures in `the_shared_fixture_log_covers_every_branch_the_contract_suite_reads` (a chained `.iter().any()` rewrap at 1154, a nested-closure rewrap at 1189). Both are in the four tests added while this review ran, so the report's PASS predates them. rust-cli.md requires all four layers green, and the release gate treats a red layer as blocking.",
     fix: "Run `cargo fmt --all`, re-run all four gates, and correct the cycle report's gate line if it still reads PASS from the earlier run."}
  - {id: G2, type: correctness, sev: Important,
     at: "crates/mochiko-cli/src/conform.rs:493-497 (`heading_lines` inside `placeholder_faults`)",
     gap: "The placeholder haystack is every line whose trimmed start is `#`, with no fence awareness and no heading-level restriction. So a `#` comment inside a fenced code block counts as heading text, and a conforming artifact is denied. Reproduced against the shared fixture: a spec whose `## Requirements` section carries a ```sh block containing `# FEAT-XXX is the pattern` exits 4 with \"the placeholder token `FEAT-XXX` survives in a frontmatter value or a heading\" — the line is neither. The control, the same token in plain prose, allows. This is the exact trap the report says it found and closed for `heading_spans`, reintroduced one function away, and it fails in the worst direction: a false deny blocks honest work, where a false allow only misses drift. It also widens D4c: the plan scopes the check to `##`/`###` heading text, and this accepts `#` and `####` too.",
     fix: "Build the haystack from a fence-aware pass — reuse the fence state `heading_spans` already tracks, or return heading lines from it — and restrict the level to `##` and `###` per plan §3.7. Add a row for a placeholder token inside a fenced block (allow) beside the existing prose row."}
  - {id: G3, type: measurement, sev: Important,
     at: "cycle-report.md, Cost section: \"CPU is 1.2 ms per call, so the 36 ms is process start, not replay\"",
     gap: "Both limbs are wrong, and the conclusion drawn from them points the lead the wrong way. Process start alone is 3.3 ms (`--version`), a check against a 1-migration log is 3.7 ms, and a check against the real 4-migration log (640 KB, 12,063 lines) is 37.5 ms with 30 ms of user CPU. So roughly 34 of the 37 ms is log-size-dependent replay work. The cost therefore scales with the log, and wave 3's census migration declares homes for a 288-file tree — the figure will move, and the 100 ms trigger comes into range rather than staying irrelevant.",
     fix: "Restate the cost note: give the process-start floor and the two log sizes measured, say that the median scales with log size, and state what that means for the wave-3 re-measure against the 100 ms trigger. The four-row table also reads as though `check` costs 36 ms unconditionally; label each row with its log size."}
  - {id: G4, type: test-honesty, sev: Important,
     at: "crates/mochiko-cli/tests/conform.rs:391-402 and :313-320",
     gap: "Two rows pass for a reason other than the rule they name. (a) `a_home_whose_bounds_live_elsewhere_takes_no_size_check` grades `gates.md` in a fixture home declared `bounds: template`, with a 3-line body against a 6-line bound — it would allow with the `elsewhere` branch deleted. The real coverage is tests/home.rs's branch-4 leg, which this row does not cite. (b) `a_placeholder_token_in_heading_text_is_denied` mutates `## Notes` into `## Notes for [entity]`, which also makes the heading undeclared; `settle` returns the first fault and headings are graded before placeholders, so the row passes on the extra-heading rule. Its own comment concedes the ambiguity and the assert is a bare `denied`.",
     fix: "(a) Add a `bounds: elsewhere` home to the conform fixture log and grade an over-budget body against it, or delete the row and point its name at the home.rs branch-4 leg. (b) Put the token in a `###` heading, which is in the placeholder scope but is not a `##` span, or assert the reason names the placeholder rather than only that a deny happened."}
  - {id: G5, type: correctness, sev: Minor,
     at: "crates/mochiko-cli/src/hook.rs:315 (the redirect arm of `write_targets`)",
     gap: "The clobber redirect `>|` is not scanned. `printf x >| .mochiko/features/FEAT-001/gates.md` allows, where the same command with a plain `>` denies — verified against the shared fixture. Stripping `>` leaves `|file`, which resolves to nothing. Inside a scan the deny reason already calls best-effort, so this is a gap rather than a contract break, but it is a one-character evasion of the arm the record calls load-bearing.",
     fix: "After `strip_prefix(\">>\")`/`strip_prefix('>')`, also strip a leading `|`, and add the `>|` row to the shell table beside `>` and `>>`. While there, `dd of=` and `install` are implemented but carry no test row."}
  - {id: G6, type: disclosure, sev: Minor,
     at: "crates/mochiko-cli/src/hook.rs:188 and tests/hook.rs:245-257, against wave1-plan.md §3 step 6 and its §6 matrix row",
     gap: "An `Edit` whose `old_string` is absent from the file returns an explicit allow. The approved plan says exit 2 in both the algorithm and the test matrix (\"`old_string` absent → exit 2\"). The behaviour is the better of the two — an unmatched `old_string` is not a usage error in the record D3 sense, and the platform rejects the call itself — but it is an undeclared change to a contract row, and the wave-4 contract suite may be written against the plan's exit 2.",
     fix: "Add it to the cycle report's deviation list with the reason, and fold the plan's step 6 and matrix row so the contract suite inherits the built behaviour."}
  - {id: G7, type: disclosure, sev: Minor,
     at: "wave1-plan.md §5's test list, against the tree",
     gap: "The plan names `tests/matrix_home.rs` and extensions to `tests/render.rs` and `tests/views.rs`. None exists: the home matrix is in tests/home.rs, the `home` render goldens are in tests/cli.rs, and render.rs and views.rs are unmodified in `git status`. The consolidation is fine; its absence from the report's file table is not.",
     fix: "One line in the cycle report saying the home matrix and the render goldens landed in home.rs and cli.rs instead of the three files §5 named."}
  - {id: G8, type: contract-drift, sev: Minor,
     at: "record.md D4f's V2 exemption against D6's `bounds: elsewhere` clause; built behaviour at conform.rs:283 and :346",
     gap: "The two record clauses disagree and the build can only follow one. D4f/V2 says such a kind \"takes location + set only, that home cited\"; D6 says it \"takes no size bound here at all\". The approved plan chose the D6 reading, and the build matches it — frontmatter, headings and placeholders still run under `bounds: elsewhere`. Not a build defect, but wave 3 binds the root operating docs to this branch, and the stricter reading would then gate their shape.",
     fix: "Fold the record so D4f and D6 state the same rule, naming which checks survive `bounds: elsewhere`, before the census migration declares the operating-docs home."}
  - {id: G9, type: clarity, sev: Minor,
     at: "crates/mochiko-cli/src/conform.rs:171 (`let _ = home;`) and :371 (`let _ = name;`)",
     gap: "Two parameters exist only to be discarded, which is how a signature keeps a field nobody needs. Both sit in the report path, where the reason text arguably *should* name the home and the file.",
     fix: "Either drop the parameters, or use them — a report deny that named its home and file name would read better than one that names neither."}
  - {id: G10, type: coverage, sev: Minor,
     at: "crates/mochiko-cli/src/conform.rs:353-373 (`report_faults`)",
     gap: "A report is graded on the envelope's frontmatter plus, when `reports.by_type` names a template for its type, that template's section budgets. The envelope template's own declared headings, placeholder tokens and section budgets are never applied. D6 says report kinds \"carry the same per-section budgets over their envelope payload\", which reads as the envelope's own sections. The fixture envelope declares `sections: []`, so no test can see the difference.",
     fix: "Either apply the envelope template's headings, placeholders and section budgets on the report path, or state in the plan and record that `by_type` is the sole budget carrier for reports and D6's phrase means that."}

strengths: the amnesty is built on a comparable fault key rather than a re-run of the whole check, which is what makes improve-allow and worsen-deny fall out by construction; the fence-and-nesting counting rule is right and carries an unfenced control leg; exit 4 is reachable from exactly one place and codes 1/2/3 are provably silent on stdout; the shell scan denies every operator I could think to try, including the wave-0 probe line verbatim and the `2>`/`&>`/`tee -a`/`dd of=`/`install` shapes the plan never named; the shared fixture log is defended by a structural test that fails the build if a contract branch is dropped; `KINDS_NOT_SHIPPED_YET` is written to fail when the gap closes rather than to rot; the plan-minimalism reversal mid-build (the leaked-static cache replaced by an owning `Homes`) is disclosed with its rung; no dependency added and the audit surface is unchanged at 31 crates; `plugins/mochiko/` is byte-identical.
---

## Failure narrative

Three things block, and none of them is the design. The eight-step algorithm, the exit-code contract,
the amnesty and the shell scan are built as ruled, and I verified each against the running binary
rather than against the report's word. What fails is one gate, one check, and two test rows.

**The fmt gate is red.** `cargo fmt --all --check` exits non-zero with 29 lines of diff in
`crates/mochiko-cli/tests/home.rs`, at lines 1154 and 1189. Both sit inside the fixture-branch test
that the mid-review revision of the cycle report added, which is why the report's `gate_cargo_fmt:
PASS` was honest when written and is wrong now. The other three gates are green on my own run: 449
tests pass across 17 suites, clippy is clean under `-D warnings`, and audit scans 31 dependencies
with no advisory. `plugins/mochiko/` is byte-identical by both `git status` and `git diff`. The fix is
one command, but the gate is a release-blocker under `rust-cli.md`, so it has to run before the tag.

**The placeholder check denies conforming artifacts.** This is the finding I would not ship without.
`placeholder_faults` collects its haystack from every line whose trimmed start is `#`, with no
awareness of fenced blocks and no restriction to `##` and `###`. The module one function below it
takes deliberate care to get exactly this right for headings, and the cycle report calls that out as
one of two traps found and closed. The placeholder scan reintroduces it. I reproduced the consequence
end to end: a spec that carries a shell fence containing `# FEAT-XXX is the pattern` is denied with
the reason "the placeholder token `FEAT-XXX` survives in a frontmatter value or a heading", and the
line is neither a frontmatter value nor a heading. The control case, the same token in plain prose,
allows. The direction matters: a false allow misses drift the reviewer still catches, while a false
deny stops honest work at the write and teaches the seat that the gate is noise. A template that
documents its own placeholder spellings inside a fenced example is the obvious first victim, and
wave 3 lands conformance blocks on exactly such templates.

**Two test rows pass for the wrong reason.** The row named for the `bounds: elsewhere` size skip
grades a body that is inside its bound anyway, in a fixture home declared `bounds: template` — it
would pass with the branch deleted. The real coverage exists, in the shared-fixture branch-4 leg,
which allows a 5,000-line file and still denies an undeclared name; the misnamed row neither tests
nor cites it. The row named for a placeholder in heading text mutates a heading into one the template
does not declare, so the heading fault is raised first and the placeholder rule is never what the
assertion sees. Its own comment concedes this. Both are repairable without touching the
implementation, and the rest of the matrix is honest work: the two bodies the report admits it had to
tighten now carry the comment that proves they were tightened rather than loosened, and the fence row
has the unfenced control leg that stops it passing on a checker which ignores headings entirely.

**One number needs restating before the lead plans wave 3.** The 36 ms median reproduces — I measured
35.5 and 37.5 ms on two passes against the real four-migration log. But the report explains it as
process start, and that is measurably false: the binary's own start is 3.3 ms, the same check against
the one-migration fixture log is 3.7 ms, and the real-log call burns 30 ms of user CPU. The cost is
replay and it scales with the log. The conclusion that no cache is built this wave survives, since
100 ms is still untripped, but the reason inverts what the lead should expect when wave 3's census
migration lands homes for a 288-file tree.

Everything else I was asked to grade holds. Nothing in `check` reads meaning: one filesystem read
exists in the whole of the new code, and it reads the artifact under test as the amnesty baseline.
No schema file is read, embedded, or referenced, so GI-020 is intact. The five disclosed deviations
are each sound, and the two I found beyond them are disclosure gaps rather than defects — the
`Edit`-with-absent-`old_string` path returns an allow where the plan said exit 2, which is the better
behaviour and needs the plan folded to match, and the plan's `matrix_home.rs` became part of
`tests/home.rs`. The `serde_norway` choice checks out on both halves the brief named: an escaped
forward slash resolves discriminatingly, and the hand-written emitter is exhaustive over the control
range, verified by round-tripping a deny reason carrying a raw control byte through a JSON parser.

## Notes of note

**The `mv`-out-of-a-home deny looks like over-reach and is not.** A `cp` or `mv` whose *source* sits
in a home is denied along with one whose destination does, which reads as stricter than D1c's "aimed
at a path under a declared home". It is what the approved plan's §6 matrix asks for, row by row, and
the code carries the reason: a re-home under the gate is done with `Write` by design. Recorded here so
the next reviewer does not re-litigate it.

**The fixture log is doing a job the contract suite cannot do for itself.** The wave-4 suite is
downstream and would fail silently if a branch vanished, so the defence lives in the crate: a test
asserts each of the four branches by its distinguishing property rather than by name, and a sibling
proves each can both allow and deny. That is the right place for it, and it is worth keeping when the
suite lands.

**One thing about the review itself.** The graded artifact changed under me — the cycle report gained
four tests, a fixture row and a whole section while I was reading. I re-ran every gate afterwards and
this review grades the post-revision tree, which is how the fmt failure surfaced at all. A producer
revising a report mid-review is not a problem, but the gate lines in it are a snapshot, and this one
went stale within the hour.

## Re-grade — the frozen tree

Re-graded after the supersession notice: the shared fixture log plus four tests, and `wave1-plan.md`
§3 corrected in four places. The corrected §3 is the referent below. Every gate re-run and every
empirical finding re-tested against the frozen tree; the three source files under review are
unchanged (`conform.rs` 17:58, `hook.rs` 17:50), so the code findings were re-confirmed by running
them, not by assumption.

**One correction to my own first pass. G1 is withdrawn: the gates are green.** `cargo fmt --all
--check` now exits 0 with zero diffs, and `tests/home.rs` carries exactly the formatting rustfmt
wanted at both 1154 and 1189. The file has not been written since 18:12:49 by either mtime or ctime,
so my first-pass failure — whose verbatim diff I captured — almost certainly raced a `cargo fmt
--all` the seat was running in the same minute: my check read the pre-format bytes, and the
post-format bytes are what stands now. The finding was true when measured and is not true of the
frozen tree, so it is withdrawn rather than carried. The cycle report's `gate_cargo_fmt: PASS` is
correct as it stands.

Gates on the frozen tree, my own run: `cargo test --all` 449 passed, 0 failed, 17 suites ·
`cargo fmt --all --check` exit 0 · `cargo clippy --all-targets -- -D warnings` exit 0 ·
`cargo audit --deny warnings` exit 0, 31 dependencies · `git status --short plugins/mochiko` and
`git diff --stat plugins/` both 0 lines.

**The step-6 change rules OK.** An `Edit` whose `old_string` is absent from the file returns an
explicit allow, where the plan's draft said exit 2. Three things make it the right call rather than a
convenient one. It is correct on the merits: record D3 scopes exit 2 to a usage error, an unmatched
`old_string` is not one, and the platform rejects that edit itself, so a deny from the gate would be
a second rejection the seat cannot act on. It is disclosed at every layer — the plan now states the
built behaviour and marks the supersession in the same sentence that names what the draft said, the
cycle report names it as one of the four places the plan text lagged, and the test that asserts it
carries the reason in its own name. And it is the safe direction: the gate declines to deny where it
cannot be sure, which is the posture every other fail-open path in this module takes. Verified
running: exit 0 with an explicit allow decision.

Per-item grades on the frozen tree: **1 gates PASS** (was FAIL) · 2 contract conformance FAIL ·
3 deviations PASS · 4 test honesty FAIL · 5 bright line PASS · 6 fixture branches PASS ·
7 cost PASS. Verdict unchanged: **FAIL**, now driven by G2 and G4 alone.

Item 6 was checkable directly and passes. The fixture log declares five homes and two templates,
replays clean, and carries all four branches: a templated `spec.md` bound to `demo-spec` with
per-section budgets, enumerated frontmatter and three placeholder tokens; a template-less `gates.md`
at 40 lines; a `reports/` directory whose envelope enumerates six types; and a `memory` home at
`bounds: elsewhere` with its citation. Each allows and denies under its own rule, the `bounds:
elsewhere` leg proven by a 5,000-line file that allows beside an undeclared name that denies. The
append-only log shape and the nested `stories` home against the literal `desk` home give the
per-entry bound and the longest-literal race something real to bind to. A structural test asserts
each branch by its distinguishing property, so dropping one fails the build here rather than
stranding a row in the downstream suite. I also ran my own end-to-end checks against this log for all
five exit codes and both shell verdicts.

Findings on the frozen tree — nine open, one withdrawn:

- **G1 — WITHDRAWN.** The gates are green; see above.
- **G2 — STANDS, Important, blocking.** Re-tested on the frozen tree: a spec whose `## Requirements`
  section carries a ```sh block containing `# FEAT-XXX is the pattern` exits 4 with "the placeholder
  token `FEAT-XXX` survives in a frontmatter value or a heading". The line is neither. The control,
  the same token in plain prose, exits 0. The corrected plan did not move this: §3 step 7 still
  scopes the check to "frontmatter *values* and `##`/`###` heading text only", and `conform.rs:493`
  accepts any line whose trimmed start is `#`, fences included. Fix as filed.
- **G3 — STANDS, Important.** The cycle report's cost section still reads "1.2 ms per call, so the
  36 ms is process start, not replay" at line 106. Measured on the frozen tree: process start alone
  3.3 ms, a one-migration log check 3.7 ms, the real four-migration log 37.5 ms with 30 ms of user
  CPU. The cost is replay and it scales with the log, which is what wave 3's census migration will
  move.
- **G4 — STANDS, Important, blocking.** Both rows are unchanged. `a_home_whose_bounds_live_elsewhere_takes_no_size_check`
  still grades a within-bound body in a `bounds: template` home, and `a_placeholder_token_in_heading_text_is_denied`
  still mutates the heading into an undeclared one with a bare `denied` assert and a comment
  conceding the ambiguity. Fix as filed.
- **G5 — STANDS, Minor.** Re-tested: `printf x >| <home>/gates.md` exits 0 while the same command
  with a plain `>` exits 4.
- **G6 — RESOLVED as filed, re-scoped to a residual, Minor.** The disclosure I asked for exists in
  both the plan and the report. What remains is one line inside the corrected plan that now
  contradicts it: §6's test matrix at line 175 still reads `` `old_string` absent | deny · deny ·
  exit 2 ``. The wave-4 contract suite reads that matrix. Fix: correct the matrix row to the explicit
  allow, so the plan agrees with itself.
- **G7 — HALF RESOLVED, Minor.** §5 now names the shared fixture log, which was half the gap. It
  still names `tests/matrix_home.rs`, which does not exist — the home matrix is in `tests/home.rs`
  and the render goldens in `tests/cli.rs`. Fix: drop the name or say where it landed.
- **G8 — STANDS, Minor.** Record D4f's V2 exemption ("location + set only") and D6's clause ("no
  size bound here at all") still disagree, and the build follows D6 via the plan. Wave 3 binds the
  operating docs to this branch.
- **G9 — STANDS, Minor.** `conform.rs:171` and `:371` still discard a parameter each.
- **G10 — STANDS, Minor.** The report path still applies only the envelope's frontmatter plus a
  `by_type` template's section budgets; the envelope's own headings, placeholders and budgets go
  unused.

verdict: FAIL
items_open: 9
items_withdrawn: 1
blocking: [G2, G4]

## Re-grade — after the fix round

Bounded to the ten items of round 1. Tree frozen at the fix-round state; every line below is my own
run against it, and the two test-honesty rows were checked by deleting the rule they name in a
scratch copy of the crate rather than by reading them.

**Gates, my own run:** `cargo test --all` 453 passed, 0 failed, 17 suites · `cargo fmt --all --check`
exit 0 · `cargo clippy --all-targets -- -D warnings` exit 0 · `cargo audit --deny warnings` exit 0,
31 dependencies · `git status --short plugins/mochiko` and `git diff --stat plugins/` both 0 lines.

- **G1 — already withdrawn, and the gates stay green.** The report's own correction row agrees with
  my reconstruction: the post-revision `cargo fmt --all` had landed before this round, so my round-1
  check read pre-format bytes. Gate lines now carry the post-fix figures with the measurement point
  named.
- **G2 — HELD.** One fence-aware `heading_scan` classifies every line once, and both callers consume
  it: `heading_spans` takes the `##` boundaries, `heading_texts` takes the text. `heading_of`
  restricts to `##` and `###` and rejects `##text` with no space. Reproduced through the binary: the
  fenced `# FEAT-XXX is the pattern` now exits 0; the same token in a real `### FEAT-XXX notes`
  heading exits 4 and the reason reads "a `##`/`###` heading", so the message matches the scope; and
  a body carrying the token in a `#` title, a `####` heading and a glued `##FEAT-XXX` exits 0. The
  structural point is what closes this properly — a second implementation was how the trap got
  reintroduced, and there is now only one.
- **G3 — HELD.** The cost section is restated, not patched. Each row carries its log size, the
  process-start floor and the spawn floor are separate rows, and the CPU line reads user 0.03. My
  re-measure agrees row for row. The report also names why the first figure was wrong — it timed a
  subshell loop, which did not attribute child CPU — and draws the consequence I asked for: the
  figure tracks the log, so the cache seam is likely at wave 3 rather than hypothetical, while the
  conclusion that no cache is built this wave survives.
- **G4 — HELD, mutation-tested.** I copied the crate to a scratch tree and deleted each rule in turn.
  Removing the `bounds: elsewhere` size exemption fails `a_home_whose_bounds_live_elsewhere_takes_no_size_check`
  with "`## Intent` is 25 lines against a budget of 4"; short-circuiting `placeholder_faults` to empty
  fails `a_placeholder_token_in_heading_text_is_denied` on its `denied` assert. Both rows now depend
  on the rule they are named for, which is what round 1 said they did not. Each also gained controls:
  the elsewhere row denies the identical 25-line body at a `bounds: template` home and still denies an
  undeclared name under the exempt home, and the placeholder row routes the token into a `###`
  sub-heading where no heading rule can fire and asserts the reason names ``placeholder token
  `[entity]` ``. Nothing in the repository was mutated; the working tree is unchanged.
- **G5 — HELD.** Both clobber forms now deny — spaced `>|` and glued `x>|path`. I ran twelve write
  shapes into a home and all twelve deny: `>|` spaced and glued, `>` spaced and glued, `>>`,
  `tee -a`, `2>`, `&>`, `dd of=`, `install`, `sed -i`, and a heredoc with its redirect. Four controls
  outside every home allow, including `>|` to `/tmp`.
- **G6 — HELD.** The plan is now self-consistent: §3 step 6 and §6's matrix row both read explicit
  allow at exit 0, each carrying the reason and marking what the draft said. The report lists it as a
  sixth deviation. The wave-4 contract suite inherits the built behaviour from either surface it reads.
- **G7 — HELD, with a residual worth one line.** `tests/matrix_home.rs` is gone from §5 and the
  fixture log is described there instead. §5 still lists `render.rs` and `views.rs` among the extended
  test files, and neither carries a test change — but the report's correction row says exactly that
  and names where the work landed, so the disclosure I asked for exists. Tidying the plan's own list
  is optional.
- **G8 — HELD, the lead's repair.** D4f's exemption now reads "no size check here, that home cited;
  the shape checks (a)–(c) still run where a template binds the file, and location + set always
  bind", and it supersedes the earlier "location + set only" wording in its own sentence. That is the
  build's reading and D6's, so the three surfaces agree.
- **G9 — HELD.** Both discarded parameters are used: every report fault's message is prefixed with
  the home and the file. Verified through the binary — a report deny reads
  ``` `.mochiko/features/<FEAT-ID>/reports/round-7.md`: frontmatter `report: invented` is not one of … ```
  and a test asserts both halves.
- **G10 — HELD.** The report path now grades the envelope in full: frontmatter, its declared headings,
  its placeholder tokens and its section budgets, with a `by_type` template's budgets added on top and
  the whole size leg still under the `bounds: elsewhere` guard. The new row proves each of the four
  binds — a missing `## Findings` denies naming the heading, five lines under a budget of four denies,
  an undeclared `## Invented` denies, and a `### FEAT-XXX` token denies — against an allow baseline.
  The check I was most interested in: the two pre-existing report rows were **re-payloaded**, each
  gaining `## Findings` with a comment saying the payload conforms so only the rule under test can
  fire. The envelope was not relaxed to keep them green.

One forward note, not a finding. The shared fixture log's envelope is still `sections: []`, so the
wave-4 contract suite sees only the frontmatter half of the envelope binding. The mechanism is
complete and covered in the crate's own matrix; if the suite is meant to exercise the heading,
placeholder and budget legs on a report, that envelope needs sections.

verdict: PASS
items_not_held: 0
