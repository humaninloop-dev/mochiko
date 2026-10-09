---
report: review
seat: w4-crate-review
tier: opus
wave: human-readable-ids wave 4 — ids.yml CI step, CI exclude list, mochiko-cli 0.4.0
pinned: ids.yml 72b8b39c3f8e0a48 · ids-check-excludes.txt 752a5addaafe8c7a · Cargo.toml b6c438e83be22111 · Cargo.lock 3632f5de11bf8e1c — all match
rule: .claude/rules/mochiko/rust-cli.md (non-author review)
verdict: PASS · 4 findings (0 blocking)
layers: repo root, default target dir — cargo test --all 0 (765 passed, 0 failed, 0 ignored) · fmt 0 · clippy 0 · audit 0
findings:
  - "1 · .github/workflows/ids.yml:60 · advisory · Every non-zero exit is labelled findings. Tool error 2 ('not inside a git tree'), missing binary 127 and a panic 101 all print 'reported findings (exit N)' and the step stays green, so a broken check looks like a check that found drift. Fix: replace :60-62 with a case on $code — 0) nothing; 1) the current findings warning; *) echo '::error::ids --check did not run (exit $code); this run checked nothing'. Keep the step's final exit 0: an ::error:: annotation never fails a step, so this stays non-blocking."
  - "2 · .github/workflows/ids.yml:5 · advisory · The header says 'this job always succeeds'. Only the check step does. A checkout, toolchain or build failure turns the job red, for example a new stable toolchain that breaks the build would redden every pull request, prose-only ones included, while ci.yml's path filter skips them. Fix: reword to 'the check step always exits 0; a broken build still fails the job'."
  - "3 · .github/workflows/ids.yml:56 · advisory · The annotation drops the column the tool prints (path:line:col). Fix: col=${rest#*:}, then echo \"::warning file=${file},line=${lineno},col=${col}::${finding#* · }\"."
  - "4 · .mochiko/memory/governance-ledger.md:567 (also the 3.3.0 row, :787) · advisory · The stated reason holds: ci.yml's pull_request.paths filter skips a prose-only pull request. But the entry calls the ci.yml-to-ids.yml change a 'dead-pointer repair', and ci.yml exists, so the pointer was never dead. The ratified condition named a file the step never landed in. Fix: call it a correction of the condition's location (the user ruled on the CI step, not on the file), and raise it at the wave-4 sign-off as lead-hand text in ratified governance."
---

## Notes of note

- Shell: GitHub's default `bash -e {0}` (no pipefail). The `|| code=$?` capture holds. The read loop ends at EOF without tripping -e. The `printf | while` pipeline exits 0. `set -u` is safe (bash 5, args never empty). `bash -n` is clean. The excludes file ends in `\n`, with no CR and no trailing spaces.
- Annotation parse: the format is `{path}:{line}:{col} · {kind} · {token}[ · definition …]` (`src/ids.rs:1525`, summary at `src/cli.rs:1067`). The summary line `· 0 bare ·` never matches `' · bare · '`. Tracked paths containing `,`, `:` or `%`: 0, so no property-escape breakage today.
- Simulation (ruby-extracted step, `bash -e`, CI-like tree of tracked files plus the 2 new files, `.git` stub, debug binary): clean run gives `0 bare · 0 drift`, exit 0, summary written. Planted bare and drift give 2 warnings with the right file and line, step exit 0. Exit 2 and exit 127 reproduce finding 1.
- Excludes: the ruled wave-3 list (22 prefixes) plus `.mochiko/strips/validation-constitution.md`, with no governance-surfaces-template strip. Mechanical diff against `map-grader-a/excludes.txt`: only that one line added. `evals/.work/` has 0 tracked files, so it is inert in CI and harmless.
- Security: `contents: read`, no secrets referenced, trigger `pull_request` (not `_target`). A pull request can already run its own code in the build, so output-injected workflow commands add nothing for an attacker. Actions are tag-pinned like ci.yml; SHA-pinning is ci.yml's recorded follow-up.
- Cargo: `--locked` release and debug builds pass, and the lock hash is unchanged after the builds. `GRAMMAR_RANGE (1, 2)` at `src/migration.rs:21`; that file is untouched since `91b3838` (the 0.3.0 commit). `target/release/mochiko-cli --version` prints `mochiko-cli 0.4.0 · grammar 1..2`. The comment is accurate.
- Rulings: the workflow meets D18-build-done-check (every pull request, report only, the check step exits 0) and D6-minting-check-enforcement (exit code is the check's only signal; the annotations add no gate).
- Routing: no Explore spawn. The exclude-list equality is a completeness-sensitive enumeration and the rest were interpretive reads, both of which `patterns-model-tiering` keeps on the seat.

## Fix verify

- Round 1, findings 1 to 4 only. Pins match: `ids.yml` e1ccea91174c82b8, ledger 3bf1a2a4fcd64c7f, intent e58f0c78cb9e1276. Each file was diffed against its review-time copy (`ids.yml` was 72b8b39c3f8e0a48). Only the four fix hunks moved.
- 1 landed. `case "$code"`: 0 does nothing, 1 gives the findings warning, and anything else gives `::error::ids --check did not run (exit $code); this run checked nothing`. The final `exit 0` is kept.
- 2 landed. The header now says "the check step always exits 0" and that a checkout, toolchain or build failure still fails the job.
- 3 landed. `col=${rest#*:}` sets the column, and the warning carries `col=${col}`.
- 4 landed in all three places: ledger :567, the ledger 3.3.0 row :788, and intent :176. "dead-pointer repair" now occurs 0 times in all three files and in `ids.yml`. The phrase "put to the user at the wave-4 sign-off" becomes true only once the lead actually raises it there.
- Re-run first-hand (ruby-extracted step, `bash -n` clean, `bash -e`, the CI-like tree): clean gives `0 bare · 0 drift` with no annotation, exit 0. Planted bare and drift give "`::warning file=docs plant/p.md,line=1,col=20::bare · GI-019`" (planted, quoted verbatim) and `,col=32::drift · …`, plus the findings warning, exit 0. No git tree gives `::error::… (exit 2)`, exit 0. Missing binary gives `::error::… (exit 127)`, exit 0.
- No new defect found. Verdict: W4 FIX VERIFY PASS · 0 problems.
