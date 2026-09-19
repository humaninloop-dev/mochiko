---
seat: V3
role: independent non-author review (author != grader, `.claude/rules/mochiko/rust-cli.md`)
plan: .mochiko/brainstorms/orchestrator-model-selection/wave2-ci-fix.md
graded: crates/mochiko-cli/tests/fidelity.rs, validate.rs, matrix_similar.rs
verdict: PASS
gates:
  cargo fmt --all --check: exit 0
  cargo clippy --all-targets -- -D warnings: exit 0
  MOCHIKO_FULL_SIMILAR=1 cargo test --all: 351 passed, 0 failed, exit 0
advisory: one documentation-only staleness outside crate scope (see Advisory)
register: ultra
---

# V3 — independent audit of the wave-2 CI fix

## Verdict

**PASS.** All seven mandated checks re-derived independently and all confirm. No assertion was
deleted or weakened beyond the one narrow, compile-time-bounded allowlist the plan approved. Blast
radius is exactly the three approved test files. Every gate green, run by V3, not read from P3.

Method note: P3's report was read for claims to falsify, never as evidence. Every number below
comes from a source V3 opened or a command V3 ran.

## Check 1 — `skill_floors` and the census, re-derived from the log

Derived by replaying the migration log from source (a standalone PyYAML walk over
`plugins/mochiko/migrations/`, independent of the crate entirely).

Genesis (`0001-genesis.yaml`), counting rules and `class: floor` under each `import-document`:

```
genesis command: docs=6  rules=321 floors=110
genesis skill:   docs=30 rules=695 floors=226
```

Rule-set deltas through the log — only two migrations move rule counts:

| migration | ops that move counts | skill rules | skill floors |
|---|---|---|---|
| `0002-fail-conditions-intent` | 6 × `reword-section` | 0 | 0 |
| `0003-two-arm-to-cli` | 14 × `reword-rule`, 3 × `set-rule-field`, 7 × `set-var` | 0 | 0 |
| `0004-sonnet-worker-rung` | 6 × `mint-rule`, 2 of them `class: floor` | +6 | +2 |
| `0007-seat-default-key` | 1 × `supersede-rule`, 5 × `mint-rule`, 2 of them `class: floor` | +4 | +1 |

The three `set-rule-field` ops in `0003` set `field: pointer` to `null` on three rules — they do
not touch `class`, so they cannot move the floor count. Verified at
`plugins/mochiko/migrations/0003-two-arm-to-cli.yaml:38-52`.

The superseded rule was itself a floor — verified at source, not inferred:

```yaml
# plugins/mochiko/migrations/0001-genesis.yaml:6469-6471
- id: patterns-model-tiering.rostered-seats-never-retier
  labels: [boundary]
  class: floor
```

So `-1` floor from the supersession, `+2` from the mints, net `+1`. The plan's step-2 hedge ("since
that rule is `class: must`, not `class: floor` ... check this") is wrong; P3's correction is right.

Arithmetic: skill rules `695 + 6 - 1 + 5 = 705`; skill floors `226 + 2 - 1 + 2 = 229`; total rules
`321 + 705 = 1026`. Command side untouched throughout at 321 rules / 110 floors, which matches the
two assertions the diff correctly left alone.

The pre-0007 values this derivation implies — 701 skill rules, 228 skill floors — are exactly the
constants the files carried before the fix, so the derivation reproduces the known-good prior state
as well as the new one.

**Confirms `701 → 705`, `1022 → 1026`, `228 → 229` in `fidelity.rs:611-614` and the identical twin
in `validate.rs:1105-1108`. Both correct.**

Fourth, independent corroboration of the floor figure: `evals/contract/expected-skills.json`, frozen
by the 0007 landing commit `3e43ef4` and never touched by this wave, records
`"floor_pin": 7` for `patterns-model-tiering` with exactly seven `floor_ids`, listing
`seat-default-key` and `seat-deviation-bounds` and no longer listing `rostered-seats-never-retier`.

## Check 2 — the `override-is-the-pin` and reserved-note needles are genuinely unaffected

Both rules ARE touched by `0007` (a `reword-rule` and a `reword-section` respectively), so "unaffected"
had to be verified against the new text, not assumed from the absence of a diff. Checked at the
migration source and against the replay semantics.

`override-is-the-pin` — `0007-seat-default-key.yaml:109-117` rewords it. The new text reads, verbatim:

> ... every tiered dispatch MUST pass its rung's model explicitly on the Agent tool call: `model:
> haiku` on the cheap rung, `model: sonnet` on the worker rung; ...

Both needles the test asserts — `` `model: haiku` `` and `` `model: sonnet` `` — survive verbatim.
The test also asserts `rule.is_floor()`. Genesis declares it `class: floor`
(`0001-genesis.yaml:6510-6513`), and `reword-rule` mutates text only —
`crates/mochiko-cli/src/replay.rs:486-491` assigns `rule.text = Some(text.clone())` and nothing
else — so the class is intact. **Correctly left unchanged.**

Reserved-note needle — the test asserts the `sec.reserved` note `contains("worker-seat-set-reserved")`.
`0007-seat-default-key.yaml:142-152` rewords that note; its closing clause reads:

> ... widening the worker rung's seat set rides `patterns-model-tiering.worker-seat-set-reserved`
> below (2026-09-05 sonnet-worker-rung).

The substring is present. **Correctly left unchanged.**

For completeness, the needle that DID change is genuinely stale and its replacement genuinely
present: `class-key-session-tier`'s new text (`0007:118-133`) contains `never tiered down`,
`2026-09-05 sonnet-worker-rung`, and `orchestrator-model-selection D1/D3`, and no longer contains
`model-tiered-seats D5`. It is also still `class: floor` (genesis `0001:6451-6453`), so the
`is_floor()` assertion above it still holds.

## Check 3 — `RETIRED_SIDECAR_ANCHORS` is narrow, and its assertion is real

`crates/mochiko-cli/tests/fidelity.rs:530-551`. Three properties, each checked:

1. **Exactly one id, enforced at compile time.** The type is `[(&str, &str); 1]` — a fixed-length
   array, not a `Vec` or a slice. A second entry cannot be added without also editing the length,
   which makes silent growth impossible. The single id is
   `patterns-model-tiering.rostered-seats-never-retier` — precisely and only the rule `0007`
   supersedes.
2. **Not a blanket skip.** The branch is entered only when the sidecar id matches an allowlist
   entry; every other one of the 597 anchors still takes the full positive walk (rule must be
   live, anchor must be present and equal). The genesis half of the test — `anchors.len() == 597`
   and `anchored == 597` — is untouched, so genesis must still carry all 597 anchors.
3. **The assertion inside the branch is real and correct.** It is
   `assert!(!live_rules.contains_key(id.as_str()), ...)` — a positive assertion of *absence*. If
   the supersession were reverted, or the rule re-minted, the rule would be back in `live_rules` and
   this assertion fails with a message naming the id. It converts "this rule is live" into "this
   rule is provably not live" rather than skipping the id. That is a narrowing, not a hole.

The entry is not dead: `rostered-seats-never-retier` is in the sidecar set — confirmed at
`crates/mochiko-cli/tests/fixtures/genesis-corpus/.mochiko/provenance.yaml:516`
(`patterns-model-tiering.rostered-seats-never-retier: "2026-08-16 model-tiered-seats"`) — so the
branch is genuinely reached and the negative assertion genuinely evaluated.

Minor, non-blocking: the tuple's second field (the ruling provenance string) is never read by the
code — it is documentation carried in the data. Clippy at `-D warnings` accepts it. Defensible; no
change requested.

## Check 4 — no other file in `crates/` was missed

Swept the whole tree for every stale constant and the retired id:

```
$ grep -rnE '\b(701|1022|228|148_353|148353)\b|model-tiered-seats D5' crates/
crates/mochiko-cli/tests/fixtures/genesis-corpus/plugins/mochiko/skills/patterns-model-tiering/schema.yaml:38
crates/mochiko-cli/tests/fixtures/genesis-corpus/plugins/mochiko/skills/patterns-model-tiering/schema.yaml:54

$ grep -rn "rostered-seats-never-retier" crates/
crates/mochiko-cli/tests/fidelity.rs:538                                      # the allowlist entry
crates/mochiko-cli/tests/fixtures/genesis-corpus/.../schema.yaml:48, :156
crates/mochiko-cli/tests/fixtures/genesis-corpus/.mochiko/provenance.yaml:516
```

Every remaining hit is inside `crates/mochiko-cli/tests/fixtures/genesis-corpus/` — the frozen
v0.103.0 input corpus that `genesis emit` regenerates `0001-genesis.yaml` from
(`fidelity.rs:48` and `fidelity.rs:128`, which spells the
`genesis emit --root crates/mochiko-cli/tests/fixtures/genesis-corpus` invocation). That fixture is
the log's *input*, not its live state. It must NOT be updated — editing it would break the
byte-for-byte genesis regeneration test. **Correctly untouched.**

Old floor pin `6`: the only `patterns-model-tiering` floor pin in the crate is
`fidelity.rs:981`, now `7`. The other `floor pin` sites — `render.rs:414-458` and
`matrix_skill.rs:543-559` — run against synthetic fixtures with their own counts (2 and 3) and are
independent of the live corpus. No miss.

Blast radius confirmed by `git status --porcelain`: three modified tracked files, all crate tests,
no data file, no migration, no schema, no strip, no view, no allowlist.

```
 M crates/mochiko-cli/tests/fidelity.rs
 M crates/mochiko-cli/tests/matrix_similar.rs
 M crates/mochiko-cli/tests/validate.rs
```

(Two untracked files — this report's siblings `p3-report.md` and `wave2-ci-fix.md` — are the wave's
own brainstorm artifacts, expected.)

Assertion accounting on the diff: `git diff -- crates/` shows three assertion values replaced in
each census test, one needle string replaced, one floor pin replaced, one sequence vector extended,
and the allowlist block added. **No assertion was deleted.**

## Check 5 — gates, run and observed by V3

Run from the worktree root, output captured to a file V3 read, not taken from P3's paste:

```
=== fmt ===
fmt exit: 0
=== clippy ===
clippy exit: 0
=== test full similar ===   (MOCHIKO_FULL_SIMILAR=1 cargo test --all)
test exit: 0
```

Per-binary results, all fourteen: `0, 0, 5, 40, 17, 2, 48, 3, 33, 39, 55, 98, 11, 0` — **351 passed,
0 failed, 0 ignored**, across every binary. The `fidelity` binary is the 17, `matrix_similar` the 48.

The opt-in sweep genuinely ran rather than being filtered out — the log carries its progress line:

```
test the_detector_reproduces_the_live_runs_figures_over_the_corpus has been running for over 60 seconds
test the_detector_reproduces_the_live_runs_figures_over_the_corpus ... ok
test result: ok. 48 passed; 0 failed; ... finished in 72.88s
```

These three commands are a superset of the four gating steps in the `crate` job of
`.github/workflows/ci.yml:43-59` (`Format check`, `Clippy`, `Test`, and
`Test (full-corpus similarity sweep)` under `MOCHIKO_FULL_SIMILAR: "1"`). `cargo audit` (step 5) was
not run: it scans the dependency graph, which this change does not touch.

## Check 6 — `matrix_similar.rs`, reproduced independently; `clusters` is genuinely 0

Ran the report myself, twice on two build paths, rather than rechecking a quote:

```
$ cargo run -q -p mochiko-cli -- migrate validate --report --plugin-root plugins/mochiko
=== similar-rule clusters (threshold 0.60) ===
none — no pair clears the threshold

=== stats ===
rules scanned: 1026 · in-kind pairs scored: 148160 · clusters: 0 (none)
allowlist-suppressed edges: 169
mochiko-cli migrate validate · 0 rejecting · 105 advisory
```

The installed binary on PATH (`mochiko-cli 0.1.0 · grammar 1..1`) printed the identical four fields,
so the figures are not an artefact of an uncommitted source build.

Against the four constants the crate test now asserts (`matrix_similar.rs:998-1001`):

| field | report printed | test asserts | |
|---|---|---|---|
| `rules scanned` | 1026 | `report.scanned, 1026` | match |
| `in-kind pairs scored` | 148160 | `report.scored, 148_160` | match |
| `clusters` | 0 (none) | `report.clusters.len(), 0` | match |
| `allowlist-suppressed edges` | 169 | `report.suppressed_hits, 169` | match |

**`clusters` is genuinely `0`.** No pair clears the 0.60 threshold, so the near-dup-convergence
doctrine is not reached, no ruling is owed, and the plan's gate opens: this is a fixture refresh, not
a finding. The allowlist was not touched (`suppressed_hits` unchanged at 169, and no allowlist file
appears in `git status`). The two constants that needed no change were correctly left alone.

The crate test and the CLI report are two paths over the same corpus, and they agree — the test
passed in V3's own sweep with exactly these values.

## Check 7 — P3's explanation for the falling `scored` is correct

The claim to falsify: `scored` drops 148,353 → 148,160 while the corpus gains four rules, because
pairs are scored *within a kind*, the retired rule declared no `kind:` and so sat in the large
`constraint` bucket, and the five mints land in small buckets.

Each limb checked at source:

1. **Pairs are bucketed by kind.** `crates/mochiko-cli/src/similar.rs:645-666` — `score_pairs`
   builds `buckets: BTreeMap<&str, Vec<usize>>` keyed on `rule.kind`, then iterates
   `for bucket in buckets.values()` scoring only within-bucket pairs. Confirmed.
2. **A kind-less rule defaults to `constraint`.** `similar.rs:494-499`:
   `fn rule_kind(rule) -> String { match rule.kind.as_deref() { Some(k) if !k.trim().is_empty() =>
   k.to_string(), _ => "constraint".to_string() } }`. Confirmed.
3. **The retired rule declared no `kind:`.** `0001-genesis.yaml:6469-6475` carries `id`, `labels`,
   `class`, `text`, `anchor` — no `kind:` field. Confirmed; it was in the `constraint` bucket.
4. **The five `0007` mints' kinds**, read at the lines the task named
   (`0007-seat-default-key.yaml`): `seat-default-key` `kind: binding` (:35); `seat-deviation-lane`
   `kind: latitude` (:55); `seat-deviation-bounds` `kind: bound` (:70); `seat-version-floor`
   `kind: bound` (:88); `seat-roster-disclosure` `kind: duty` (:103). Confirmed — `binding`,
   `latitude`, `bound` ×2, `duty`.

Then re-derived both pair counts arithmetically from the log, independent of the crate. Bucket sizes
from the same standalone replay used in check 1 (with same-`extends` pairs excluded, matching
`similar.rs:661-663`):

```
genesis kinds:     binding 127, bound 17, constraint 491, duty 125, fail 36,
                   gate 28, latitude 11, reservation 53, routing 128
genesis:           scanned 1016 · scored 146572
after 0004:        scanned 1022 · scored 148353     <- the old frozen constant
after 0007 kinds:  binding 129, bound 19, constraint 493, duty 127, fail 36,
                   gate 28, latitude 12, reservation 54, routing 128
after 0007:        scanned 1026 · scored 148160     <- the new constant
```

Both frozen constants are reproduced exactly by a path that never runs the detector. The signed
decomposition of the −193:

- Removing one member from the `constraint` bucket (494 → 493) costs **493** pairs.
- Adding to the small buckets gains: `binding` 128→129 = +128; `latitude` 11→12 = +11; `bound`
  17→18 = +17 and 18→19 = +18; `duty` 126→127 = +126. Total **+300**.
- `300 − 493 = −193`, and `148353 − 193 = 148160`.

**P3's explanation is correct in mechanism and in magnitude.** The comment written above the
constants states it accurately.

## Advisory — one documentation staleness, outside crate scope, non-blocking

`evals/contract/README.md:266` still reads:

> | floor ids | 110 across six | 228 across thirty (226 at the freeze; `patterns-model-tiering` 4 → 6
> by the 2026-09-05 sonnet-worker-rung ruling) |

The skill-side figure is now 229 across thirty, `patterns-model-tiering` 4 → 6 → 7. This is prose in
a README, not an assertion: a repo-wide sweep found no live pin of `228`/`1022`/`701` in
`run.py`, `freeze_expectations.py`, `diagnostic.py`, or `expected-skills.json`, and the suite's own
data was already refreshed by the 0007 landing commit (`floor_pin: 7`). The contract suite therefore
does not fail on this.

It is outside the three approved files and outside the crate, so V3 did not touch it and it does not
gate this fix. Flagging it for the lead as a separate, one-line fold — plausibly owed before the next
`plugin.json` bump, since the release gate reads that README's accounting.

`CHANGELOG.md:74` also names `695 → 701` and `226 → 228`; that is a historical entry describing the
0004 landing and is correct as written. No change owed.

## Fix list

None. Nothing blocking. The change is ready to commit.
