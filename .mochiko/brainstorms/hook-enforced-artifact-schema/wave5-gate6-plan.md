# Gate 6 — the full contract run before the bump (plan; awaiting lead approval)

**Author:** QA seat · **Date:** 2026-09-15 · **Ruling home:** GI-012 gate 6 · record D10 · AM-3's
wave-4 hook-ship precondition. **Plan only. No run until the lead opens it.**

## (a) Which binary each half uses

**Host rows — `target/release/mochiko-cli`.** `host_binary()` refuses a binary older than any `.rs`
under `crates/mochiko-cli/src/`, by mtime, and prints the rebuild command. A source touched without
a content change stops the run; that is the right side to err on for a release gate.

**Sandbox sessions — today a source build, not the release.** `build_binary()` runs `cargo build
--release -p mochiko-cli --manifest-path <repo>/Cargo.toml --target-dir /home/agent/mochiko-target`
inside the sandbox, because the host is macOS and the sandbox is Linux, and proves it by parsing
`--version`. **That tests the worktree, not the artifact a consumer installs** — the gap gate 6
exists to close.

**What it needs.** `cargo` is in the image and `build_binary()` already checks for it, so the
smallest change is `cargo install mochiko-cli --version <x> --root /home/agent/.cargo-gate` with
`binary_dir` pointed at its `bin/`; the `--version` check and `sandbox_path()`'s absence check are
unchanged. Fetching the release asset and verifying its sha256 is the stronger option if the tap is
the supported route. Roughly ten lines either way. **Neither exists today** — pick one before the
tag, not after.

## (b) Invocation, sessions, cost

```
python3 evals/contract/run.py            # the full deterministic set
python3 evals/contract/run.py --host-only  # the 7 host cases, no session, no sandbox
```

87 cases: 7 host, 80 sandboxed. Sessions: wave 6 measured **151** over the 82 cases that existed
then, my two cases add **3**, the preflight authentication probe adds **1** per invocation —
**155 metered sessions**. Cost is measured, not guessed: 194 result events in `evals/.work/` carry
`total_cost_usd`.

| figure | per session | × 155 |
|---|---|---|
| mean | $0.081 | ~$12.5 |
| median | $0.098 | ~$15.2 |

**Estimate $13–16.** Model time is small — mean 6.4 s per session — so wall clock is dominated by
the in-sandbox build (30-minute timeout) and `sbx exec` overhead. Budget an hour, not a spend.

## (c) Pass criterion

Exit 0 with every case green. Two failures block equally: exit 1, an assertion failed; exit 3, the
sandbox never started — GI-012 says a SKIPPED suite is not green, and so does the suite: *"the
sandbox cases did not run, so they are evidence of nothing."* `report()` rows never gate.

## (d) What a red case means for the bump

The bump blocks; fix the cause, never the assertion, then re-run.
**Re-running only the affected subset is impossible today** — the suite takes `--list` and
`--host-only` and nothing else, so any sandbox-side fix costs the full 155 sessions again. A
`--case <name>` filter over `CASES` is a few lines and turns each iteration from ~$14 into cents.
Recommendation: add the filter, then run the gate. If the runner should not be touched before a
release gate, the fallback is batching fixes at ~$14 per iteration. A host-side red is free to
re-run.
