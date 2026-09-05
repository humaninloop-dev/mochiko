# Wave 1 — seat P1 (core) independent code audit

**FAIL**

Unit frozen at `f34d48e`. Reviewer authored none of it. Default FAIL; three blocking findings
survive, twelve advisory. Gates all green — the failures are semantic, not mechanical.

Blocking: 3 · Advisory: 12.

The unit is strong work. The typed model, the canonical encoder, the op set, the corpus smoke and
the census pins are all better than the plan asked for. What fails is the one guarantee the whole
wave is built to make mechanical: **protected content does not, in fact, leave only by ruling.**
A single migration retires a floor rule, a fail rule and an anchored rule with no anchor, raising
zero findings.

---

## Blocking

### B1 — Protected-exit is bypassable in one migration

**Where:** `crates/mochiko-cli/src/replay.rs:586` (`set_field`, no protection guard) with
`crates/mochiko-cli/src/replay.rs:556` (`retire` reads `is_protected()` *after* the downgrade).

**What.** `set-rule-field` may write or clear `class`, `kind` and `anchor` — the exact three
fields `Rule::is_protected()` (`src/model.rs:375`) reads. Ops inside one `changes:` list apply in
order, so a migration downgrades then retires:

```yaml
- {op: set-rule-field, schema: command/demo, id: demo.lead, field: class, value: advisory}
- {op: tombstone-rule, schema: command/demo, id: demo.lead, disposition: gone}
```

**Evidence.** Out-of-tree probe against the frozen crate (a two-file log; genesis imports a floor
rule, a fail rule and an anchored rule; migration 0002 downgrades and tombstones all three):

```
=== PROBE A: downgrade-then-tombstone ===
findings: 0
is_deliverable: true
live rules after: []
tombstones after: [("demo.lead", "gone"), ("demo.fail.unplanned", "gone"), ("demo.anchored", "gone")]
load() Ok? true
```

**Why it fails.** Wave-plan §3 requires that a `class: floor` rule, a `kind: fail` rule, or an
anchored rule "leaves only via `supersede-rule` with a resolving anchor". Record D2 makes the
schema-rule limb of GI-005 mechanical on that basis; D6 lists it in the hard set. The seat's own
`migrations/README.md` states the claim outright — "a floor cannot be dropped quietly, because the
tool will not write the state in which it has been" — and Probe A writes exactly that state. The
suite already contains the mirror case: `tests/replay.rs`'s
`protected_content_leaves_only_by_a_ruling_anchored_supersession` *adds* an anchor with
`set-rule-field` and asserts the tombstone is refused. Clearing one is untested.

**Fix.** Make protection sticky rather than re-derived. Seed a per-document protected-id set in
`State` at `import-document` and at `mint-rule`, add to it whenever `set-rule-field` writes
`class: floor`, `kind: fail` or an `anchor`, and have `retire` consult that set. Only
`supersede-rule` with a resolving anchor clears an entry. Cheaper alternative if the sticky set is
unwanted: reject a `set-rule-field` that removes protection unless the same op carries an anchor.
Either way the owed test is the inverse of the one above — clear the field, then tombstone, assert
`protected-exit`.

### B2 — The body hash is optional, so the anchor it protects stays editable

**Where:** `crates/mochiko-cli/src/migration.rs:711`.

**What.** The `hash:` header is checked only when present and non-empty. Three spellings skip the
check entirely: the key absent, `hash: ""`, and `hash: ~`.

**Evidence.**

```
=== PROBE B: no hash: key ===
parsed OK with no hash. computed body_hash would be sha256:d5cf0d88…
parsed OK with `hash: ""`
parsed OK with `hash: ~`
```

**Why it fails.** Wave-plan §2 lists `hash: sha256:<hex>` among the header fields and marks only
`anchor:` optional ("optional; **required** when …"), so `hash:` reads as required. The seat's Q1
amendment put the anchor *inside* the hash on the stated ground that "the anchor … would be
editable after the fact" otherwise (report §4.1). Optionality nullifies that: an editor need not
forge a hash, only delete one line. `migrations/README.md` discloses the choice
("optional, and binding once written"); the report's deviation list (§4) does not, so it reached
the lead only through the README.

**Fix.** Require `hash:` — return `ParseError::Header` when absent or empty. If an authoring
escape hatch is genuinely wanted, make it explicit and loud (an `--unhashed` flag on the authoring
command, never a silently-absent field), and disclose it as a §2 deviation.

### B3 — `replay::load` never runs the D6 hard set

**Where:** `crates/mochiko-cli/src/replay.rs:184`. `validate::validate`
(`crates/mochiko-cli/src/validate.rs:401`) has no caller anywhere in `src/`.

**What.** `load` refuses on rejecting findings raised *during replay* only. The hard set is a
separate pass nothing invokes.

**Evidence.**

```
=== PROBE C: does load() run the hard set? ===
load() returned Ok; the hard set then finds 5 REJECTING findings:
  section-set · command/demo · - · canonical section demo.sec.boundaries absent …
  (+4 more)
```

**Why it fails.** Wave-plan §3 puts the hard set "on the state after every migration". P1's report
§1 hands P2 a stronger contract than the code keeps: "`Ok` is a state safe to render from … P2's
`rules` and `template` should call `load`, never `replay_dir`." P2 following that ships renders
off unvalidated state. Both modules are P1's, so the seam is P1's.

**Fix.** Fold `validate(&state)`'s rejecting findings into `load`'s refusal (or add
`replay::validated_load` and point the report at it). Pin it: a log that replays cleanly but fails
the hard set must not return `Ok`.

---

## Advisory

Ranked. None blocks on its own.

**A1 — a non-conforming `.yaml` in `migrations/` is silently skipped.**
`crates/mochiko-cli/src/replay.rs:112`. `is_migration_file` requires a leading digit, and a file
failing it is dropped with no finding — so `genesis.yaml`, or `O001-genesis.yaml` typed with a
letter O, replays as if it did not exist. Silent partial delivery is the class D5's no-fallback
posture exists to rule out. Report any `.yaml` that is not `NNNN-<slug>.yaml`.

**A2 — `every_rejecting_code_is_reachable` proves nothing.**
`crates/mochiko-cli/tests/validate.rs:940`. It asserts
`state_level.len() + log_level.len() == Code::REJECTING.len()`, which is true by construction
(`state_level` is `REJECTING` minus `log_level`), plus one `contains` check. Coverage is in fact
complete — every one of the 34 rejecting codes has at least one probe, verified by grepping the
three suites — but the guard that would keep it complete as codes are added is a tautology.
Collect the codes actually raised across the suite and assert set equality with `Code::REJECTING`.

**A3 — `is_anchor` is materially looser than the checker it ports.**
`crates/mochiko-cli/src/model.rs:1157` vs `scripts/check-command-schema.py:145` (`RULING_RE`,
anchored at both ends). Probe E: `"9999-99-99 x"`, `"2026-09-03 slug and a whole sentence of junk
here"` and `"2026-09-03 slug [NOT-A-DECISION]"` all pass. The `[D#]` segment the error message
advertises is never checked. `!slug.is_empty()` at `model.rs:1168` is dead — `split_whitespace`
never yields an empty token. Anchor the tail and range-check month and day.

**A4 — a far-future grammar misses the D5 halt message.**
`crates/mochiko-cli/src/migration.rs:446` caps every header integer at
`u32::from(u16::MAX) as u64 * 2`, an undocumented 131070. Probe D: `grammar: 999999` returns
`grammar-header` ("out of range"), not `grammar-version`, so the install line D5 requires never
prints. `grammar: 2` is handled correctly. Parse `grammar` before the cap, or raise
`GrammarVersion` for any out-of-range whole number.

**A5 — `mint-section` silently drops inline rules.**
`crates/mochiko-cli/src/replay.rs:333` (`rules: Vec::new()`). Probe F: a `mint-section` carrying a
floor rule applies with zero findings and the rule is gone. Reject a section value carrying
`rules:` rather than discarding it.

**A6 — the deixis port is incomplete and boundary-free.**
`crates/mochiko-cli/src/validate.rs:382` carries 8 markers;
`scripts/check-command-schema.py:140` also matches `see below` and `there is no <X> section`, and
uses `\b` boundaries where the Rust does a substring `contains` on lowercased text (so
`this sectional` matches `this section`). Wave-plan §3 pins the list to the Python's.

**A7 — the "unused declared moments" advisory is unported.**
Wave-plan §3's advisory set names "unused vars · unused declared conditions/moments".
`validate.rs` reports `UnusedVar` and `UnusedCondition`; the moments half
(`scripts/check-command-schema.py:910`, check 11) has no Rust equivalent.

**A8 — `encode_canonical` recurses with no depth bound.**
`crates/mochiko-cli/src/model.rs:1039`. Rule-bearing documents are shape-bounded by the decoder,
but `Document::Opaque` (templates, shelf) holds arbitrary-depth YAML, so `State::content_hash` on
a deeply nested document aborts the process instead of raising a finding. A depth counter that
returns a finding past a generous limit costs three lines.

**A9 — every change-level parse error reports as `op-unknown`.**
`crates/mochiko-cli/src/migration.rs:102`. A well-known op missing a required field
("`content:` missing") is filed under the code for an unrecognised op, which sends a maintainer to
the wrong place. Split `ParseError::Change` or give it a code field.

**A10 — silent coercions in `set-rule-field`.**
`crates/mochiko-cli/src/replay.rs:618` (a non-string `when:` dimension key becomes `""`) and
`crates/mochiko-cli/src/replay.rs:662` (`string_list` maps a non-string item to `""`). Both should
be findings; as written they corrupt quietly and the hard set then reports a confusing downstream
symptom.

**A11 — the round trip is content-lossless, not order-lossless.**
`crates/mochiko-cli/tests/validate.rs:1045` compares `canonical_hash`, which sorts mapping keys,
so field and declaration order are not asserted. The claim holds for A1 as written (no field
normalises), and order survives structurally via `Ordered`, but P3's derived views depend on key
order and nothing pins it. One additional assertion comparing `to_value()`'s key sequence with the
original's would close it.

**A12 — section ids never get the `id-format` check.**
`crates/mochiko-cli/src/validate.rs:395` (`check_ids`) runs over `schema.rules()` only. A
malformed section id surfaces indirectly, as a `section-set` "extra". Wave-plan §3 names "a
rule/section ID". Functionally covered for the canonical six; worth closing when the set check
cannot reach (a schema whose prefix will not derive).

---

## Criterion-by-criterion

| # | criterion | verdict |
|---|---|---|
| 1 | every §3 hard-set clause has a rejecting code **and** a mutation test | **pass on substance** — all 34 rejecting codes carry a probe; the guard that enforces it is a tautology (A2) |
| 2 | family/prefix derivation matches the Python exactly | **pass** — `Family::of` (`validate.rs:284`) is character-for-character `family_of` (`check-skill-schema.py`); `derive_prefix` (`validate.rs:362`) matches `derive_prefix` (`check-command-schema.py:1037`) including the empty and disagreeing branches; `common_prefix_of` and the registries check out |
| 3 | migration grammar per §2 as amended | **fail** — hash over `{id, sequence, anchor, changes}` ✓ (`migration.rs:678`, pinned by `the_body_hash_covers_the_anchor_and_the_sequence_but_not_the_intent`); mint-once ✓; tombstone integrity ✓; `supersede-rule` requires an anchor ✓; `tombstone-section` rejects while rules remain ✓; `import-document` rejects on an existing doc ✓; `registry-retire` records, never deletes ✓ — but the hash is optional (**B2**) |
| 4 | replay determinism and the deliverability signal | **partial** — `replaying_twice_yields_an_identical_content_hash` is real and green; `is_deliverable` correctly tracks rejecting findings; but `load`'s `Ok` does not mean hard-set-clean (**B3**) |
| 5 | corpus smoke: 50 / 321 / 695 / 226, the 110-vs-112 difference, the two exemptions pinned | **pass** — `the_shipped_corpus_matches_its_recorded_census` pins all five figures plus 36 fail nodes; the instrument difference is explained in a code comment (`tests/validate.rs:1153`) and in report §7, and the explanation checks out (`architecture.yaml` and `implement.yaml` write `class: floor` inside rule prose); `the_shipped_corpus_validates_with_no_rejecting_finding` allows exactly the two `setup.yaml` rules and then asserts `findings.len() == 2`, so a third cannot hide; its sibling sets both `note:` fields and asserts the corpus is clean outright |
| 6 | lossless round trip over all 50 (A1) | **pass** — read the test, not the report: `every_shipped_document_round_trips_through_the_model` (`tests/validate.rs:1045`) decodes and re-encodes each shipped file and compares canonical hashes; caveat at A11 |
| 7 | hygiene | **pass** — `Cargo.toml` gained `sha2 = "0.10"` and nothing else; `Cargo.lock` gained sha2 plus 9 transitives, all expected (`digest`, `block-buffer`, `crypto-common`, `generic-array`, `typenum`, `cpufeatures`, `libc`, `cfg-if`, `version_check`); no `unsafe` anywhere; no network or subprocess in the new code; tests write only under `CARGO_TARGET_TMPDIR`; `git show --stat f34d48e -- plugins/` is empty; no `plugin.json` bump; GI-019 intact — the crate grades no artifact and dispatches nothing, and the hard set is structural validity on the store's own data per D11(ii). Deviations: six disclosed in report §4; the hash optionality (B2) is not among them |
| 8 | what a skeptical senior would refuse | **B1, B2, B3 plus A1, A5, A8, A10.** No panic path on malformed input found (`replay.rs:407`'s `.expect` is guarded nine lines above; the decoders return `Result` throughout). No O(n²) that will not scale — `check_extends`'s library scan is rules × blocks, 0.19s over the real corpus. No dead code beyond `model.rs:1168`. Error messages are specific and name the offending value. Grammar helpers are shared, not duplicated — `decode_rule` deliberately routes through the `import-document` path (`replay.rs:667`) so the two cannot diverge |

---

## Gate outputs

Run by the reviewer, in the worktree, at `f34d48e`.

```
$ cargo test --all
   tests/migration.rs   16 passed; 0 failed
   tests/render.rs      12 passed; 0 failed
   tests/replay.rs      33 passed; 0 failed
   tests/validate.rs    38 passed; 0 failed
   unittests + doc-tests 0 passed; 0 failed
   exit 0

$ cargo fmt --all --check
   (no output) exit 0

$ cargo clippy --all-targets -- -D warnings
   Checking mochiko-cli v0.1.0
   Finished `dev` profile
   exit 0

$ cargo audit --deny warnings
   Loaded 1239 security advisories
   Scanning Cargo.lock for vulnerabilities (25 crate dependencies)
   (no advisories) exit 0
```

99 tests total, 87 of them new. Clippy was re-run after touching `src/lib.rs` to defeat the
cache, so the clean result is a real compile, not a replay.

---

## What was checked, and how

I read the wave plan §2/§3/§6/§8, record decisions D1, D2, D5, D6 and D11, `rust-cli.md`, and
P1's cycle report, then read all five source files and all three new test files in full before
forming a view — the report's claims were treated as claims and checked against the diff, not
accepted. Family and prefix derivation were compared line-by-line against `check-skill-schema.py`
and `check-command-schema.py`, including the deixis regex, the resolution set and the ruling-anchor
regex, which is where A3, A6 and A7 came from. For the three blocking findings I did not rely on
reading: I built a probe crate outside the repository that takes `mochiko-cli` as a path
dependency, wrote adversarial migration logs against it, and ran them, so B1, B2, B3 and A4, A5
carry executed output rather than an argument. Nothing in the repository was edited; the probe
lives in the session scratchpad. The corpus pins were checked by reading the assertions and
confirming the 110-vs-112 explanation against the two named files. Every gate above was executed
by me rather than quoted from the report, and they agree with it.

---

# Delta re-grade — fix round 1

**PASS**, with one documentation item owed (D-3 below).

Scope: the fifteen findings above and the two lead-granted deltas, nothing else. Fifteen of
fifteen CONFIRMED, both deltas CONFIRMED. Every blocking fix was verified twice — once by
re-running the audit's original adversarial probes against the fixed crate, and once by reverting
the fix in a scratch copy of the crate and watching the named test go red. A fix whose test does
not fail without it is not a fix, and all three were checked that way.

## Blocking

**B1 — protected exit — CONFIRMED.** `lowers_protection` at `crates/mochiko-cli/src/replay.rs:614`
asks whether a `set-rule-field` removes `class: floor`, `kind: fail`, or an `anchor:`, and the
call site at `crates/mochiko-cli/src/replay.rs:462` demands the migration's own header anchor when
it does. The authority is filtered through `is_anchor` at `crates/mochiko-cli/src/replay.rs:265`,
so a malformed header anchor authorises nothing. The audit's original probe, replayed against the
fixed crate, now returns four `protected-exit` findings where it previously returned none:

```
protected-exit · command/demo · demo.lead · `class: floor` would become `class: advisory` …
protected-exit · command/demo · demo.lead · a `class: floor` rule leaves only by `supersede-rule` …
protected-exit · command/demo · demo.fail.unplanned · `kind: fail` would become `kind: constraint` …
protected-exit · command/demo · demo.anchored · the rule's ruling anchor would be cleared …
is_deliverable: false · load() Ok? false · live rules: all three still live
```

With a well-formed header anchor the same migration applies and the floor leaves; with a malformed
one it is refused. Both branches probed. Neutralising the guard in a scratch copy turns four tests
red: `lowering_protection_without_a_ruling_is_a_protected_exit`,
`clearing_protection_without_a_ruling_is_a_protected_exit`,
`clearing_or_changing_an_anchor_without_a_ruling_is_a_protected_exit`, and
`a_malformed_migration_anchor_does_not_authorise_lowering_protection`.

I checked the other write paths for the same class of hole. `reword-rule` touches text only;
`move-rule` changes no field; `replace-document` is refused for rule-bearing kinds;
`import-document` is refused over an existing document. `set-rule-field` was the only door.

**Lead ruling (1) — per-migration authority — implemented as ruled.** The residual is real and
behaves as designed: an anchored migration may downgrade a floor, and a later unanchored migration
may retire what is by then an ordinary rule, the ruling having been recorded at the downgrade.
P1's report states it plainly rather than hiding it. See D-3 for the documentation half.

**B2 — optional body hash — CONFIRMED.** `crates/mochiko-cli/src/migration.rs:812` rejects an
absent or empty `hash:` as `grammar-header`. All three bypasses are closed and a tampered body
still reports `hash-mismatch`:

```
no hash    -> grammar-header ("header field `hash:` missing …")
hash: ""   -> grammar-header
hash: ~    -> grammar-header
tampered   -> hash-mismatch
correct    -> parses
```

`migration::with_hash` (`crates/mochiko-cli/src/migration.rs:690`) is the sanctioned stamping
path, documented in `migrations/README.md`. Restoring the optional-hash branch in a scratch copy
turns `a_migration_with_no_hash_is_rejected` red. Worth recording that writing the helper exposed
a second bug in the seat's own code — the stamping path validated the hash it was about to
overwrite, so it could not repair a stale one — caught by
`with_hash_replaces_an_existing_hash_rather_than_duplicating_it`. That is the test suite working.

**B3 — `load` did not run the hard set — CONFIRMED.** `replay()` runs
`crate::validate::validate` over the finished state at `crates/mochiko-cli/src/replay.rs:271` and
stores it in `Replay::validation`; `rejecting()` and `is_deliverable()` consult both passes.
Running the hard set over the finished state rather than each intermediate one is correct: a
migration may pass through a shape the hard set would reject so long as it does not leave one.
The audit's probe now refuses:

```
load() refused with 6 findings; section-set · command/demo · canonical section … absent (×5)
```

Replacing the validation pass with an empty vector in a scratch copy turns
`load_refuses_a_log_that_replays_cleanly_but_fails_the_hard_set` and
`a_state_carrying_a_rejecting_finding_is_never_deliverable` red. The report's earlier over-claim to
P2 is now true of the code, and `migrations/README.md` states the two-part contract.

## Advisory

| # | verdict | evidence |
|---|---|---|
| A1 | CONFIRMED | `Code::LogFileName` raised at `src/replay.rs:157`; probed with both `genesis.yaml` and `O001-typo.yaml`, each reported. Pinned by `a_yaml_file_that_is_not_a_migration_is_reported_never_skipped` |
| A2 | CONFIRMED | The tautology is gone. `every_rejecting_code_is_raised_by_some_probe` runs 21 state-level mutations and builds 13 real migration logs, then asserts set equality with `Code::REJECTING` in both directions. Verified it bites: deleting one probe from the list fails the test with `these rejecting codes are declared but no probe raises them: ["label-unknown"]` |
| A3 | CONFIRMED | `is_anchor` at `src/model.rs:1203` is anchored both ends, range-checks month and day, and validates one decision segment. Every counter-example from the first round now returns false: `9999-99-99 x`, `2026-13-45 x`, trailing prose, `[NOT-A-DECISION]`, and a doubled `D3 D4`. Both `D3` and `[D3]` accepted, which the corpus requires — all 597 sidecar anchors write the tail bare, so bracket-only would have rejected the sidecar. The dead `!slug.is_empty()` branch is gone |
| A4 | CONFIRMED | `read_grammar` at `src/migration.rs:484` parses before any integer cap. Probed at 2, 99, 999999 and `u32::MAX`: all four reach `grammar-version` with the install line. A value past `u64` reports `grammar-parse`, which is a YAML-level rejection and still loud |
| A5 | CONFIRMED | `src/replay.rs:378` rejects a section value carrying `rules:`; probed, and pinned by `mint_section_carrying_inline_rules_is_rejected` |
| A6 | CONFIRMED | Ten markers with the `there is no … section` wildcard, matched on word boundaries by `deixis_marker` at `src/validate.rs:411`. Two tests: all ten phrases fire, and `this sectional` / `there is no shortcut` do not |
| A7 | CONFIRMED | The unused-moment advisory at `src/validate.rs:671`, including the shipped checker's deliberate prose-substring weakness, which P1 names as an under-report rather than papering over. Pinned by `a_declared_moment_nothing_names_is_reported` |
| A8 | CONFIRMED | `MAX_CANONICAL_DEPTH` 64, encoder depth-threaded from `src/model.rs:1078`, `canonical_depth` saturating. A 5,000-level value built in memory hashes without aborting and raises `depth-exceeded`. Note for the record: serde's own parser stops near 512 levels, so a file could not have reached the old bug — the exposure was through in-memory opaque documents only, which is what the fix and its test cover |
| A9 | CONFIRMED | `ParseError` split into `UnknownOp` and `MalformedChange` (`src/migration.rs:43`), mapping to `op-unknown` and the new `op-malformed`. Pinned by `a_known_op_missing_a_field_is_malformed_not_unknown` |
| A10 | CONFIRMED | No coercion left: a non-text label item and a non-text `when:` key each raise `op-inapplicable`, probed directly. Pinned by `a_non_string_when_key_or_label_is_a_finding_never_a_silent_coercion` |
| A11 | CONFIRMED as partial, per the lead's ruling | `the_round_trip_preserves_declaration_order` asserts document key order, section key order, and the declaration order of `vars`, `conditions`, `moments` and registry `labels` over all 50 shipped files. Rule *field* order normalises, and the report's A11 row discloses exactly that with its reason. The test's scope and the disclosure agree — it stops above rule fields rather than asserting something it does not check |
| A12 | CONFIRMED | Section ids get `id-format` directly at `src/validate.rs:848`. One precision nit, not a defect: the report says a malformed section id "is still caught when the prefix will not derive" — true, but by `derive_prefix`'s `id-prefix` finding, not by this check, which sits after the early return at `src/validate.rs:461` |

## Deltas

**D-1 — CONFIRMED.** `Replay::grammar()` (`src/replay.rs:99`) and `load_full` (`src/replay.rs:222`)
are additive; `load` is re-expressed on `load_full` and keeps its signature. Pinned by
`the_replay_reports_the_grammar_it_applied` and `an_empty_log_reports_no_grammar`; probed, and
`grammar()` returns `Some(1)`.

**D-2 — CONFIRMED, and it is the right call.** `ResolvedRule`, `resolve_extends` and `placeholders`
are made public by rename and visibility only, with no change to their logic. The reason P1 gives
is the one that matters: a renderer resolving `extends:` or `${var}` through a second
implementation would show guidance the hard set never graded.

## D-3 — the one item owed

`migrations/README.md` documents the lowering-protection rule clearly, but not its corollary. A
reader reaches "a floor cannot be dropped quietly, because the tool will not write the state in
which it has been" without being told that the authority is per migration, so an anchored
downgrade in one file and an unanchored tombstone in a later one is the sanctioned path. P1's
report states the residual; the README, which is the artifact a future maintainer reads, does not.
One sentence in the anchor-rule section closes it. Documentation only, no code change, and not a
reason to hold the unit.

## Gate outputs

Run by the reviewer against the working tree.

```
$ cargo test --all
   tests/migration.rs   25 passed; 0 failed     (was 16)
   tests/render.rs      12 passed; 0 failed     (untouched)
   tests/replay.rs      46 passed; 0 failed     (was 33)
   tests/validate.rs    44 passed; 0 failed     (was 38)
   exit 0 — 127 total, up from 99

$ cargo fmt --all --check          exit 0 (no output)
$ cargo clippy --all-targets -- -D warnings
                                   exit 0 (re-run after touching lib.rs, so not a cached result)
$ cargo audit --deny warnings      exit 0 — 25 crate dependencies, no advisories
```

Unchanged and re-verified: `Cargo.toml` and `Cargo.lock` have no new dependency this round; no
file under `plugins/` is touched and all 20 shipped schemas are intact; no `plugin.json` bump; no
`unsafe`; no network or subprocess in the crate; tests write only under `CARGO_TARGET_TMPDIR`. The
corpus pins are unmoved and still green — 50 documents, 321 command rules, 695 skill rules, 1,016
total, 226 skill floors, 110 declared command floors, 36 fail nodes — and the round trip still
passes over all 50 shipped files.

## Method

I re-graded only the fifteen findings and the two deltas, and did not re-open the design questions
the lead ruled. For each item I located the change in the source, read it, then found the test
that names it. For the three blocking findings I did not stop at reading: I re-ran the original
adversarial probes from an out-of-tree crate that takes `mochiko-cli` as a path dependency, and
then took a scratch copy of the crate, reverted each fix in turn, and confirmed the named tests go
red — four for B1, one for B2, two for B3 — and did the same for A2's new coverage guard. The
scratch copy has been deleted and the repository was not edited; the shipped schemas were checked
intact afterwards. Every gate above I executed myself.
