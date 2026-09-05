# Wave 1 — seat P3 (corpus) independent code audit

**FAIL** — one blocking finding. Criterion 4's name-level probe accounting holds for the command
and skill matrices (both independently verified verbatim against the live Python) but is absent
for the 48-probe detector matrix, and the cycle report claims it holds for all three. Everything
else in the unit passes, most of it by independent reproduction rather than by reading the
author's tests: genesis regenerates byte-identically, all 50 views are semantically equal to the
shipped corpus under a comparator written for this audit, and the detector's corpus pin was
reproduced against the Python detector itself.

**Unit graded:** `a1e4275` on top of `07a39b4` (P2). Author: seat P3. Grader: this seat, which
wrote none of it. Default posture FAIL.

---

## 1. Verdict per criterion

| # | criterion | verdict |
|---|---|---|
| 1 | genesis complete and honest | **pass** |
| 2 | fidelity, field-by-field over 50 | **pass** |
| 3 | views, semantic equality and the out-dir fence | **pass** |
| 4 | matrix port, every Python probe claimed exactly once | **FAIL** (B1) |
| 5 | `similar.rs`, difflib parity and the corpus pin | **pass** |
| 6 | release machinery and the CI filter | **pass** |
| 7 | contract-suite skeleton | **pass** |
| 8 | hygiene | **pass** |
| 9 | what a skeptic would refuse | **pass with advisories** (A1–A3) |

---

## 2. Blocking

### B1 — the 48-probe detector matrix has no name-level accounting, and the report says it does

`crates/mochiko-cli/tests/matrix_similar.rs` — whole file; the accounting that exists is at
`:305`, `:332`, `:359`, `:403`, `:521`, `:695`, `:772` (per-section counts in doc comments) and
`:938` (`not_applicable_under_d6`, which asserts only `not_applicable.len() == 3`).

**What.** The command and skill matrices each carry a verbatim `PYTHON_PROBES` array and run
`matrix::accounted_for`, which does the set algebra: every Python probe claimed exactly once, and
a claimed name that is not a Python probe fails as loudly as a missing one. I extracted both
Python lists by executing each file's own `probes()` and diffed them against the Rust arrays —
**both identical, 134 and 114, no reordering, no paraphrase.** That is the strongest part of the
unit.

`matrix_similar.rs` has neither. There is no list of the 48 `check("…")` names, no ledger, and no
set-algebra test. The split the report gives — 45 ported / 3 not applicable — rests on a hand
count in section comments (5 + 3 + 3 + 7 + 5 + 9 + 4 = 36, plus seven untallied end-to-end tests).

**Why it is wrong, not merely unverified.** Mapping all 48 Python check names onto the Rust tests
by hand, **three have no named Rust referent and appear in no ledger**:

```
e2e: exit 0 by default
skill e2e: exit 0 by default
authoring e2e: exit 0 by default
```

They are the same class as the three `--exit-signal` / `--json` probes that *were* dispositioned
at `:922-936`: assertions about the Python script's exit-code surface, which the Rust detector
does not have. The true split is 42 ported / 6 not applicable, not 45 / 3.

**The contradicted claim.** `p3-corpus.md` §3: "Each matrix's ledgers are asserted to account for
its Python list exactly, by name." False for the third matrix. §3's table row `similar | 48 | 45 |
— | 3 | —` is off by three in both columns.

**Why blocking.** Record D6 makes the matrix port the retirement gate for the Python scripts — "the
port is the retirement gate, never a parallel period." The seat itself found and fixed exactly
this failure mode for the other two matrices (report §2, cycle 4: "my first ledger summed to 136 of
134, because two probes were claimed twice and two were claimed nowhere. Counting by hand is how
that happens"). The third matrix is still counted by hand.

**Mitigating, and stated so the lead can scale the fix.** The *substance* is not lost. The three
unaccounted checks assert that the detector exits 0 on clusters, and the Rust equivalent — the
report carries no severity and no exit code — is asserted at `matrix_similar.rs:946-951` and holds
in the live binary (`migrate validate --report` exits 0 with 92 advisory findings; I ran it). The
defect is the accounting mechanism and the report's claim about it.

**Fix.** In `matrix_similar.rs`: add `const PYTHON_PROBES: [&str; 48]` verbatim from
`scripts/test-find-similar-rules.py`, a per-test mapping to the Python name(s) each covers, a
not-applicable ledger carrying all six exit-surface checks with reasons, and a set-algebra test in
the shape of `matrix::accounted_for`. Correct `p3-corpus.md` §3's table row and the "by name"
sentence.

---

## 3. Advisory, ranked

### A1 — `migrate validate --report` silently reports 76 clusters instead of 0, depending on the working directory

`crates/mochiko-cli/src/cli.rs:415` — `crate::similar::default_allowlist(Path::new("."))`.
`crates/mochiko-cli/src/similar.rs:961` — `default_allowlist` returns `None` when the file is
absent, and `clusters()` then suppresses nothing and says nothing.

The allowlist is resolved against the **process working directory**, not the plugin root and not
the repository holding `--log-dir`. Run from the repo root the report is `clusters: 0 ·
allowlist-suppressed edges: 181`. Run the identical command on the identical log from any other
directory:

```
rules scanned: 1016 · in-kind pairs scored: 146572 · clusters: 76 (COMMON-CANDIDATE 17, CROSS-PAIR 42, INTRA-SCHEMA 17)
```

The `allowlist-suppressed edges:` line simply vanishes. A maintainer gets 76 adjudicated-and-closed
clusters back as fresh signal with nothing telling them why. Both runs are in §5 below.

Advisory-only surface, exit code unaffected, so not blocking — but this is the one finding a
skeptical reader would refuse on sight.

**Fix.** Resolve the allowlist against the resolved plugin root or the log dir's repository, and
print an explicit line when none is found (`no allowlist at <path> · 0 edges suppressed`) so a
76-cluster report can never be mistaken for a clean read.

### A2 — the view writer loses trailing newlines on any string carrying two or more

`crates/mochiko-cli/src/views.rs:275-290` (`literal_block`).

For text ending in *n* newlines the writer picks `|+` (keep) at `n >= 2`, then computes
`body = &text[..text.len() - (trailing - usize::from(trailing == 1))]` and additionally strips one
more newline — so the body it emits has no trailing blank lines for `|+` to keep. Emission and
read-back, verified:

```
"a\n\n"   -> emitted 'key: |+\n  a\n'  -> reads back as 'a\n'
"a\n\n\n" -> emitted 'key: |+\n  a\n'  -> reads back as 'a\n'
```

Unreachable on today's corpus — I confirmed all 50 documents round-trip under an independent
PyYAML comparator — so no live corruption. But this module is the GI-006 reconstruction surface and
its own doc asserts a lossless projection; the neighbouring hazards (a leading-space first line, a
trailing-space body line) each have a guard at `:284-289` and this one does not.

**Fix.** Emit `trailing - 1` blank lines after the body in the `|+` branch, or fall back to
`double_quoted` when `trailing >= 2`, matching the guards already there.

### A3 — quoted scalars are never folded, so the views are measurably longer-lined than the corpus they mirror

`crates/mochiko-cli/src/views.rs:318-325` (`foldable`) requires `!needs_quote(text, false)`. A
string containing `: `, a leading `-`, or a quote is therefore emitted as one unwrapped line
however long. Inside a `>-` block those characters are literal, so most of these would fold
safely; the guard is over-conservative.

| | shipped corpus | generated genesis |
|---|---|---|
| lines over 120 chars | 89 | 330 |
| longest line | 696 | 970 |

The hand-rolled writer's entire justification (`views.rs:7-12`) is that readability is a D6
constraint, so this is worth closing.

### A4 — half the ported probes assert the finding code without the offending id

`Expect::Reject(code)` asserts only "some rejecting finding of this code". Counts:

| matrix | `Expect::Reject` | `Expect::RejectOn` |
|---|---|---|
| command | 29 | 20 |
| skill | 34 | 19 |

Most are document- or section-level mutations with no rule id to name, and the baseline is clean
so a single mutation makes a code match near-decisive. Still weaker than the criterion asks.

### A5 — the skew case asserts on text the binary writes only to stderr

`evals/contract/run.py:333` — `assert_message(text, "cargo install mochiko-cli")`. Verified
locally: the D5 message goes to **stderr only**, stdout is empty, exit 3.

```
0001-skew.yaml: the migration log is written in grammar 99, and this binary reads grammar 1..1. Update the binary: cargo install mochiko-cli
```

Whether Claude Code's `!` preprocessing surfaces stderr to the model is not settled by the wave-0
probes for the non-denial path (probe (a) records only that a *denial* arrives as
`<local-command-stderr>`). This case has never executed, so the assumption is untested. The halt
itself is safe either way — the fixture's clause treats an empty block as a delivery failure — only
this one assertion is at risk. Worth confirming at the first authenticated run.

### A6 — `main()` returns exit 0 when no case runs

`evals/contract/run.py:349-364`. With `CASES` empty, `ran = 0`, `failures = 0`, and the function
returns `EXIT_OK` after printing "0/0 cases passed". `CASES` is a two-entry constant so this cannot
fire today, but the file's own contract is "exit 0 means every declared case ran — never 'nothing
happened'". A `if not CASES: return EXIT_SKIP` makes that structural.

### A7 — the release archive name doubles the crate prefix

`.github/workflows/release.yml:70` — `stage="mochiko-cli-${{ github.ref_name }}-${{ matrix.target }}"`
with `ref_name` already `mochiko-cli-vX.Y.Z`, producing
`mochiko-cli-mochiko-cli-v0.1.0-aarch64-apple-darwin.tar.gz`. Harmless for manual download; will
not match `cargo binstall`'s default URL template, which the header comment names as a consumer.
Wave 2 can settle it with `[package.metadata.binstall]` or a simpler stage name.

### A8 — the detector test dominates the suite

`matrix_similar.rs` is 100.7 s of a 137 s `cargo test --all`. Inherited, not introduced: the same
scoring pass takes 80.4 s in the Python it replaces and 10.0 s in a release build of this port. No
action asked; noted because CI now runs it on every `migrations/**` touch.

### A9 — small report drifts

- `p3-corpus.md` §6 says "the eight that stay malformed"; `tests/anchor_grammar.rs:57-67` lists
  nine (`D2a3` is the extra).
- §10.1 (`migrations/README.md` carries the pre-delta anchor grammar) is discharged — the lead's
  two sentences landed in this commit. The open item can close.

---

## 4. What passed, and how it was checked

**Genesis (criterion 1).** 50 `import-document` ops, one per document, kinds summing correctly
(6 command · 1 command-common · 1 command-labels · 30 skill · 2 skill-common · 1 skill-labels ·
8 template · 1 shelf). 597 `anchor:` fields at rule depth against 597 sidecar entries — counted
independently by indentation, not by the crate. Header carries `grammar: 1`, `sequence: 1`,
`anchor: 2026-09-03 cli-schema-delivery D2`, and a hash that is real: a one-character tamper in the
copy produced `hash-mismatch · 1 rejecting`. Regenerated with `genesis emit` and `cmp`-identical
at 598,626 bytes. `.mochiko/provenance.yaml` untouched (`git status` clean after every run). Both
`note:` lifts are present, verbatim from the source comments, and generic — the corpus carries
exactly two `# D6 empty-with-reason:` blocks and both are lifted, with the ids asserted by name at
`fidelity.rs:37-49`.

**Fidelity (criterion 2).** `fidelity.rs:145` compares the shipped YAML decoded straight from disk
against the replayed document, field by field, accumulating every divergence rather than stopping
at the first. Coverage checked against the model: `RuleSchema::rules()` at `model.rs:465` chains
sections' rules with `blocks`, so common-library blocks are compared too. The 9 opaque documents go
through `canonical_hash`. Both deltas are asserted, not excused: the notes against a table of the
literal comment text, the anchors against the sidecar the test reads for itself.

**Views (criterion 3).** I emitted all 50 from the log and compared them against the shipped files
with a PyYAML comparator written for this audit — **0 of 50 diverge** once the two declared deltas
(folded anchors, the two lifted notes) are excluded; before excluding them, exactly the 36
anchor-bearing documents differ, which is the expected shape. Output is genuinely readable: block
scalars, corpus key order, inline `labels:`/`enforces:`/`when:`, and the 8-line command kernel
header reproduced exactly. No default `--out` (`cli.rs:97-103`), and `emit_to` writes only under
it.

**`similar.rs` (criterion 5).** The corpus pin was reproduced against the Python detector, not
taken on trust — `uv run scripts/find-similar-rules.py` and `migrate validate --report` return
**identical** figures: 1016 scanned · 146,572 pairs · 0 clusters · 181 suppressed. 17 golden
vectors asserted at `< 1e-12`, including the autojunk witness at 0.0667 where a port without it
returns ~0.95. The `find_longest_match` port is faithful, including the `j < blo` / `j >= bhi`
window and the two extension loops; the scratch-buffer optimisation is correct across the swap.
Advisory throughout — `migrate validate --report` exits 0.

**Release and CI (criterion 6).** Trigger `mochiko-cli-v*`; four targets with arm64 Linux through
`cross`; strip, tar, `shasum -a 256`, both uploaded. `publish` present, `if: false`, with the
wave-2 lift named in a comment. Actions tag-pinned, matching `ci.yml` (`actions/checkout@v4`,
`dtolnay/rust-toolchain@stable`, `taiki-e/install-action@v2`, `softprops/action-gh-release@v2`);
SHA-pinning disclosed as the same follow-up `ci.yml` carries. `ci.yml` gains exactly
`migrations/**` and `evals/contract/**` on both triggers and nothing else.

**Contract skeleton (criterion 7).** `run.py:63` imports `evals/run.py` through `importlib`, never
forks it; `claude_args(prompt, model, max_turns, stream, plugin)`, `sbx_sh(script, timeout)` and
`SANDBOX` all match the real signatures. All six D8 assertions exist as functions. Preflight ladder
is ordered and honest, and the authentication rung runs a real cheap headless probe rather than
inferring. Ran it: case list printed, then `SKIPPED … exit 3`, matching report §9 verbatim. README
carries the gate split and the ToS caveat.

**Hygiene (criterion 8).** `git diff --stat 07a39b4 HEAD -- plugins/` empty. No `unsafe`. No
network symbol anywhere in `src/` or `tests/`. `Cargo.toml` and `Cargo.lock` untouched — no new
dependency. Every P3 test write lands under `CARGO_TARGET_TMPDIR`. The `model.rs` delta is 9
insertions / 1 deletion inside `is_decision_segment` plus its doc paragraph, and nothing else;
`anchor_grammar.rs` pins it hard, including reading the two forcing anchors out of the sidecar by
rule id so the test dies if the corpus stops needing the widening. `RULING_RE` in both Python
checkers is `D\d+.*`, so the report's parity claim is accurate and the Rust stays the stricter of
the two. GI-019 intact: the detector proposes and never merges, genesis is a generator, nothing
dispatches or gates.

**Report honesty.** The seat disclosed more against itself than the diff would have forced: the
stale 86-probe census (114 confirmed by execution), the 81 probes outside the Rust hard set, the
severity change on a missing sidecar, its own arithmetic error in an earlier tally, and the refusal
to normalise `D2a` into the validator. The 231/0 tally reproduces exactly, per binary.

---

## 5. Commands run

```
cargo test --all --no-fail-fast          231 passed, 0 failed, 13 binaries (137 s)
cargo fmt --all --check                  clean
cargo clippy --all-targets -- -D warnings  clean
cargo audit --deny warnings              clean (31 crates, 1239 advisories)

cargo run -q -- migrate validate --log-dir migrations
  mochiko-cli migrate validate · 0 rejecting · 92 advisory              exit 0
cargo run -q -- migrate validate --log-dir migrations --report
  rules scanned: 1016 · in-kind pairs scored: 146572 · clusters: 0      exit 0
  allowlist-suppressed edges: 181

uv run scripts/find-similar-rules.py
  rules scanned: 1016 · in-kind pairs scored: 146572 · clusters: 0 (none)
  allowlist-suppressed edges: 181                                      (80.4 s)

# A1 — same log, same binary, different cwd
(cd <scratch>) mochiko-cli migrate validate --log-dir <repo>/migrations --report
  rules scanned: 1016 · in-kind pairs scored: 146572 · clusters: 76
  (COMMON-CANDIDATE 17, CROSS-PAIR 42, INTRA-SCHEMA 17)   [no suppression line]

mochiko-cli genesis emit --out <scratch>/genesis-audit.yaml --root .
  598626 bytes · 10479 lines ; cmp vs migrations/0001-genesis.yaml -> identical

mochiko-cli views emit --out <scratch>/views-audit --log-dir migrations
  50 documents ; independent PyYAML comparison vs plugins/: 0 of 50 diverge
  (excluding the two declared deltas; 36 of 50 diverge on anchors alone, as expected)

# hash is load-bearing
perl -pi -e 's/^    name: architecture$/    name: architecturX/' <copy>
  hash-mismatch · 0001-genesis.yaml: body hash mismatch — header records
  sha256:361cf5d6… body canonicalises to sha256:16428146…       1 rejecting

# probe-list extraction, executed not transcribed
test-check-command-schema.py probes() -> 134, unique 134  == Rust PYTHON_PROBES  (diff empty)
test-check-skill-schema.py   probes() -> 114, unique 114  == Rust PYTHON_PROBES  (diff empty)
test-find-similar-rules.py            -> 48 checks, passed: 48 · failed: 0
  ledger coverage in matrix_similar.rs: none

python3 evals/contract/run.py
  SKIPPED: the sandbox 'claude-mochiko' is not reachable: ERROR: Not authenticated to Docker
  exit 3 — the suite did not run, so nothing here is evidence of anything.     exit 3

mochiko-cli rules specify --section roles --log-dir <skew>
  stdout: empty ; stderr: "… grammar 99 … reads grammar 1..1. Update the binary:
  cargo install mochiko-cli"                                            exit 3
```

Git state at close: only `wave1-reports/p3-corpus.md` modified in the tree (an author-side line-count
correction, 115 → 117, unrelated to this audit). No file was edited by this seat.

---

## 6. Method note

I graded the code, the data and the tests, and treated the cycle report as a claim to be checked
rather than as evidence. Where the seat's own test asserts a property, I re-derived that property
by a route the crate does not own: the two Python probe lists were extracted by executing each
script's `probes()` and diffed against the Rust arrays rather than eyeballed; the 50 views were
compared to the shipped corpus with a PyYAML comparator written for this audit rather than through
`canonical_hash`; the detector's corpus pin was reproduced by running the Python detector itself;
the genesis hash was proved load-bearing by tampering with a copy; and the `|+` round-trip loss was
confirmed by emitting what `write_scalar` would emit and reading it back. The one blocking finding
came from asking the question the seat's own §2 cycle-4 story invites — *is the third matrix counted
the same way the first two now are* — and the answer is no. I did not edit any file under
`crates/`, `plugins/`, `migrations/`, `.github/` or `evals/`; every run wrote to a scratch
directory outside the repository.

---

## Delta re-grade — fix round 1

**PASS.** All ten findings confirmed fixed in code and pinned by tests that would fail without
the fix. Every claim below was re-derived independently rather than read off `p3-corpus.md` §12.
Two report-accuracy nits and one pen question for the lead; neither is a code defect.

Graded: the uncommitted working tree on `a1e4275`, 13 files, +2,449 / −393. I re-ran the whole
genesis and views chain rather than only the ten sites, because the folding fix rewrote
`migrations/0001-genesis.yaml` by 1,826 lines.

### Gates

```
cargo test --all --no-fail-fast          242 passed, 0 failed, 13 binaries (~32 s)
                                         matrix_similar 100.7 s -> 5.2 s ; views 8 -> 10 tests
cargo fmt --all --check                  clean
cargo clippy --all-targets -- -D warnings  clean
MOCHIKO_FULL_SIMILAR=1 cargo test --test matrix_similar
                                         48 passed, 0 failed (68.9 s) — the 1,016 / 146,572 /
                                         0 / 181 pin reproduces

mochiko-cli genesis emit --out <scratch> --root .
  618122 bytes · 11701 lines ; cmp vs migrations/0001-genesis.yaml -> BYTE-IDENTICAL
  hash: sha256:361cf5d61ee69a856dcb49c7014c305cd00595fa8eef9cdcaa87de2239ef39cd  (unchanged)
mochiko-cli migrate validate --log-dir migrations     0 rejecting · 92 advisory
mochiko-cli views emit --log-dir migrations           50 documents
  independent PyYAML comparison vs plugins/: 0 of 50 diverge (36 on anchors alone, as expected)
python3 evals/contract/run.py                          SKIPPED … exit 3
```

### Per finding

**B1 — CONFIRMED (blocking, closed).** `matrix_similar.rs:1100` carries `PYTHON_PROBES: [&str;
48]`. I extracted the script's own `check("…")` names again and diffed **order-sensitively**:
identical, all 48, no paraphrase. `:1156` `macro_rules! ported` binds each row's test as
`$test as fn()`, so a rename or deletion is a build error, not a quiet unclaim — stronger than
the transcribed arrays the other two matrices still use, which P3 names as a follow-up.
`:1248` `NOT_APPLICABLE` carries six rows including the exact three I found unaccounted
(`e2e: exit 0 by default` and its skill and authoring siblings), each with a reason.
`:1280` `EXTRA` fences the six Rust-only tests out of the coverage claim.
`:1327` `the_recorded_python_names_are_the_scripts_own` re-derives the list from the script, so
the array cannot go stale in silence. `:1344` asserts unclaimed, twice-claimed and invented names
are all empty and pins the split at `(42, 6)` — the split I derived by hand in §2. P3 quoted the
red naming exactly the three probes; the guard is real.

**A1 — CONFIRMED.** `similar.rs:981` `find_allowlist` walks the log directory's ancestors and
never consults the process working directory; `cli.rs:413` passes `dir`. `similar.rs:948`
`render_report` now always prints one of two lines. Re-ran my own reproduction — the repository's
log, from a scratch directory that demonstrably has no `scripts/`:

```
rules scanned: 1016 · in-kind pairs scored: 146572 · clusters: 0 (none)
allowlist-suppressed edges: 181
```

Was 76 clusters with the suppression line absent. And the other half, a copy of the log outside
any tree carrying an allowlist:

```
rules scanned: 1016 · in-kind pairs scored: 146572 · clusters: 76 (COMMON-CANDIDATE 17, CROSS-PAIR 42, INTRA-SCHEMA 17)
allowlist: none (181 edges unsuppressed)
```

Six tests pin it, not the five §12 claims. `the_repositorys_allowlist_resolves_without_the_process_cwd`
(`matrix_similar.rs:1037`) asserts the cwd lacks the file **before** resolving, so it fails under
the old implementation rather than passing vacuously.

**A2 — CONFIRMED.** `views.rs:290` appends `trailing - 1` blank lines in the `|+` branch.
`tests/views.rs:104` `a_multiline_scalar_keeps_every_trailing_newline` runs eight cases through a
real parse, including `"a\n\n"` and `"a\n\n\n"` — the two I proved returned `"a\n"`. It fails
without the fix.

**A3 — CONFIRMED.** `views.rs:301` `foldable` drops the `needs_quote` gate and excludes control
characters instead (tab still caught — it is `Cc`). Measured myself, both units:

| | lines over 120 | longest |
|---|---|---|
| shipped corpus (bytes) | 89 | 696 |
| generated log, before (bytes) | 330 | 970 |
| generated log, after (bytes) | 39 | 349 |
| shipped corpus (chars) | 76 | 693 |
| emitted views, after (chars) | 26 | 341 |

The views are now shorter-lined than the corpus they mirror. `tests/views.rs:134` pins folding
for dash-leading, colon-space, hash-carrying and `yes:`-leading strings, asserts the width, and
round-trips each — every case fails under the old guard. Semantic equality survives the change:
0 of 50 diverge under my own comparator.

**A4 — CONFIRMED in substance; the §12 table is wrong.** Measured, comments excluded:

| matrix | `Expect::Reject` before → after | `Expect::RejectOn` before → after |
|---|---|---|
| command | 29 → 21 | 20 → 28 |
| skill | 34 → 26 | 19 → 27 |

**Sixteen** probes moved, not the fourteen §12 claims, and neither row of its table (command
23/25, skill 25/27) matches the file. The fix over-delivers; only the arithmetic is off. Four
exceptions are argued in-file (`matrix_command.rs:176-178`): the canonical-section-absent family
reports a finding with no id, because the node it would name is the one that is not there.

**A5 — CONFIRMED.** The fixture's `!` line is now
`` !`mochiko-cli rules brainstorm --section preamble 2>&1` `` and `run.py:234`
`assert_skew_halt_on_stderr` asserts exit 3, empty stdout and the message on stderr, wired into
`case_skew` at `:340`. I re-confirmed the binary's behaviour directly. The residual — that a
redirect needs no separate `allowed-tools` grant — is stated in the fixture body itself and
deferred to the first authenticated run, which is the honest disposition.

**A6 — CONFIRMED.** `run.py:383` `if not CASES: return EXIT_SKIP`, before any case runs.
Contract suite still exits 3 with the case list printed first.

**A7 — CONFIRMED, with a residual.** `release.yml:80` `stage="${{ github.ref_name }}-${{
matrix.target }}"`; the upload list and the header comment match. The doubled prefix is gone.
Residual: the release tag is `mochiko-cli-vX.Y.Z`, so the download **path** segment is still
neither `{version}` nor `v{version}`, which is what `cargo binstall` derives by default. The
filename now matches its `{name}-v{version}-{target}` form but the directory does not, so §12's
"no `[package.metadata.binstall]` override is needed" is stronger than the evidence. A wave-2
check, not a wave-1 defect.

**A8 — CONFIRMED, with two notes.** The substitute pin is real, not a weaker restatement: I ran
`uv run scripts/find-similar-rules.py --schemas-dir plugins/mochiko/schemas --allowlist
scripts/similar-rules-allowlist.yaml` and it returns `321 · 12154 · 0 · 60`, exactly what
`the_detector_reproduces_its_figures_over_the_command_family` asserts. The full pin still
reproduces under the env var. Default suite 137 s → 32 s.

- *Note 1 — the skip is not visible.* `cargo test` captures a passing test's stdout, so the gated
  test reports `ok` and its `println!` never appears. I ran it alone to confirm: `1 passed`, no
  skip line. §12's "prints its skip rather than passing silently" holds only under `--nocapture`.
  `#[ignore]` would show it in the tally as `ignored`; worth a follow-up either way.
- *Note 2 — the CI step exceeds the ask, and the stated reason is not mine.* A8 said in terms
  "**No action asked**". §12's pen note says "Taken because A8 asks for it in those words", which
  it does not. The step itself (`ci.yml:49-57`) is sound: env scoped to the step, `--test
  matrix_similar` resolves in this single-crate workspace, no rebuild, and CI wall time falls
  rather than rises (137 s of `cargo test --all` becomes 32 s plus a 69 s step). Nothing else in
  the file moved. But it is a disclosed widening of a path-filter-only pen, and the lead should
  ratify it on its merits rather than as an audit-mandated fix.

**A9 — CONFIRMED.** `anchor_grammar.rs` lists nine malformed cases and the report now says nine;
the two `migrations/README.md` sentences landed and §10.1 is closed.

### Final

**PASS.** The blocking finding is closed by a mechanism stronger than the one it was measured
against, and the nine advisories are closed by real code changes, each pinned. Outstanding for
the lead, none of them code defects: the two §12 arithmetic corrections (A4's table and moved
count; A7's binstall claim), the invisible opt-in skip (A8 note 1), and the `ci.yml` pen widening
to ratify (A8 note 2). The follow-ups P3 names itself — transcribed probe arrays in the command
and skill matrices, and the `MOCHIKO_FULL_SIMILAR` switch belonging in
`.claude/rules/mochiko/rust-cli.md` — are both worth taking.
