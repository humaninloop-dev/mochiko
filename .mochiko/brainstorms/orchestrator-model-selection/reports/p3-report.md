---
seat: P3
plan: .mochiko/brainstorms/orchestrator-model-selection/wave2-ci-fix.md
scope: >-
  crates/mochiko-cli/tests/fidelity.rs (plan), plus validate.rs and matrix_similar.rs
  under the lead's recorded scope widening
status: complete — all three files fixed, every gate green
gates:
  cargo fmt --all --check: exit 0
  cargo clippy --all-targets -- -D warnings: exit 0
  cargo test --all: 351 passed, 0 failed, exit 0
  MOCHIKO_FULL_SIMILAR=1 cargo test --test matrix_similar: 48 passed, 0 failed
blocker: none — cleared by the scope widening
grader: V3 (author != grader)
register: ultra
---

# P3 — wave 2 CI fix, fidelity.rs

## Bottom line

All three stale-pin files are fixed and every gate is green, including the opt-in full similarity
sweep CI runs in its own step. The plan's four fidelity.rs edits went in first; the plan's own
read-back step 4 then uncovered the same stale expectations in two further test files, which the
lead's recorded scope widening authorised and which are covered in "Widened scope" below.

The judgment-bearing pin in the widened set came back clean: the similarity detector reports
`clusters: 0`, so 0007's five minted rules are not near-duplicates of anything in the corpus and
the refresh is a fixture update rather than a near-dup finding.

## Read-back 1 — pre-edit reproduction

`cargo test -p mochiko-cli --test fidelity` before any edit reproduced exactly the four CI
failures, ruling out an environment difference:

```
test result: FAILED. 13 passed; 4 failed; 0 ignored; 0 measured; 0 filtered out
the_corpus_census_holds_through_the_log — left: 705, right: 701 (live skill rules)
the_fourth_migration_added_the_worker_rung_to_the_tiering_floor —
  patterns-model-tiering.class-key-session-tier: the reword names "model-tiered-seats D5"
the_log_replays_into_a_deliverable_state — left: [1, 2, 3, 4, 7], right: [1, 2, 3, 4]
the_sidecar_anchors_ride_their_rules —
  patterns-model-tiering.rostered-seats-never-retier: the sidecar's rule is no longer live
```

## Read-back 2 — `skill_floors`, confirmed not assumed

The lead's hypothesis (`pre-edit value + 1`) is confirmed, and the hedge in the plan's step 2
("since that rule is `class: must`, not `class: floor` ... check this") is wrong: the superseded
rule was a floor.

Three independent confirmations, none relying on the others:

1. **Genesis source.** `plugins/mochiko/migrations/0001-genesis.yaml:6469-6471` mints
   `patterns-model-tiering.rostered-seats-never-retier` with `labels: [boundary]` and
   `class: floor`. Its supersession therefore removes one floor.
2. **Migration 0007 ops.** One `supersede-rule` (the floor above) and five `mint-rule`s, of which
   exactly two declare `class: floor` — `patterns-model-tiering.seat-default-key` (line 34) and
   `patterns-model-tiering.seat-deviation-bounds` (line 69). The other three
   (`seat-deviation-lane`, `seat-version-floor`, `seat-roster-disclosure`) are `class: must`.
   Net for the skill: -1 + 2 = **+1 floor**, and -1 + 5 = **+4 rules**.
3. **Measured against the live render.** `mochiko-cli views emit` into a scratch directory, then
   counting rule-level (indent-8) `class: floor` lines: skills **229**, commands **110**. The
   command figure matches the untouched `command_floors` assertion exactly, which validates the
   counting method itself. A naive `grep -c` gives 112 on the command side; the two extra matches
   are prose inside rule text, which `validate.rs:1099-1101` documents independently.

So `skill_floors` moves 228 to **229**. The post-edit assertion passing is the fourth
confirmation.

Also confirmed directly against the render before writing edit 4, rather than trusted from the
plan:

- `class-key-session-tier` is still a floor and now reads `orchestrator-model-selection D1/D3`;
  the `never tiered down` and `sonnet-worker-rung` needles both survive.
- `override-is-the-pin` still carries `` `model: haiku` `` and `` `model: sonnet` `` and is still
  a floor — untouched by 0007's reword.
- The reserved section's note still names `worker-seat-set-reserved`, so that assertion needed no
  change despite 0007's `reword-section` on `sec.reserved`.
- `rostered-seats-never-retier` appears in the live render only under `tombstones:`, never as a
  live rule — which is what makes the new allowlist's negative assertion meaningful.

## The diff applied

```diff
@@ fn the_log_replays_into_a_deliverable_state
-        vec![1, 2, 3, 4],
+        vec![1, 2, 3, 4, 7],
         "genesis, wave 4's fail-conditions reword, wave 6's two-arm retirement, the sonnet \
-         worker rung"
+         worker rung, 0007's seat default key"

@@ fn the_sidecar_anchors_ride_their_rules
+    /// Sidecar-anchored rules retired by a recorded ruling since genesis — each entry is a
+    /// protected exit the log's own anchor rule required (`supersede-rule` under an anchor,
+    /// `plugins/mochiko/migrations/README.md` "The anchor rule"). A rule listed here MUST be
+    /// genuinely absent from the live state, or the walk below still fails — this list only
+    /// narrows which absence is expected, never which text is skipped.
+    const RETIRED_SIDECAR_ANCHORS: [(&str, &str); 1] = [(
+        "patterns-model-tiering.rostered-seats-never-retier",
+        "0007-seat-default-key.yaml, 2026-09-19 orchestrator-model-selection D1",
+    )];
+
     for (id, expected) in &anchors {
+        if RETIRED_SIDECAR_ANCHORS
+            .iter()
+            .any(|(retired_id, _)| retired_id == id)
+        {
+            assert!(
+                !live_rules.contains_key(id.as_str()),
+                "{id}: listed in RETIRED_SIDECAR_ANCHORS but still live — update the list or \
+                 the migration"
+            );
+            continue;
+        }
         let rule = live_rules

@@ fn the_corpus_census_holds_through_the_log
     // `0004` (the sonnet worker rung, 2026-09-05) minted six skill rules on
-    // `patterns-model-tiering`, two of them floors; the command side is untouched.
+    // `patterns-model-tiering`, two of them floors; `0007` (the seat default key,
+    // 2026-09-19) nets +4 on the same skill — one `supersede-rule` (−1) and five
+    // `mint-rule`s (+5), two of the five floors; the command side is untouched throughout.
+    // The superseded rule was itself a floor, so the skill's floor count nets +1.
     assert_eq!(command_rules, 321, "live command rules");
-    assert_eq!(skill_rules, 701, "live skill rules");
-    assert_eq!(command_rules + skill_rules, 1022, "live rules in total");
-    assert_eq!(skill_floors, 228, "skill floors");
+    assert_eq!(skill_rules, 705, "live skill rules");
+    assert_eq!(command_rules + skill_rules, 1026, "live rules in total");
+    assert_eq!(skill_floors, 229, "skill floors");

@@ doc comment above the_fourth_migration_added_the_worker_rung_to_the_tiering_floor
-/// the two reworded floors still floors, naming both rungs and the D5 clause; the floor pin at
-/// six; the reserved section's note naming its new reservation.
+/// the two reworded floors still floors, naming both rungs (D5's text since superseded by
+/// `0007`'s seat default key — this test reads the full log, so the current wording is what it
+/// checks); the floor pin now seven after `0007`; the reserved section's note still naming its
+/// new reservation.

@@ fn the_fourth_migration_added_the_worker_rung_to_the_tiering_floor
                 "never tiered down",
                 "sonnet-worker-rung",
-                "model-tiered-seats D5",
+                "orchestrator-model-selection D1/D3",

-    assert_eq!(floors, 6, "the skill's floor pin");
+    assert_eq!(floors, 7, "the skill's floor pin");
```

Two deliberate departures from the plan's literal text, both additive and neither a design call:

- The plan's edit 3 diff did not show the `skill_floors` line, but its prose requires the
  measured `+1`. The line moves 228 to 229, and the comment gains one sentence saying why.
- The `RETIRED_SIDECAR_ANCHORS` constant is placed immediately before the loop it serves (the
  plan left the insertion point to P3's call between two named options).

## Read-back 3 — format and lint

```
cargo fmt --all --check           exit 0
cargo clippy --all-targets -- -D warnings   exit 0
```

The `///` doc comment on the function-local `const` raises no `unused_doc_comments` warning: a
`const` in a function body is an item, so the comment is accepted as written.

## Read-back 4 — full suite

`cargo test -p mochiko-cli --test fidelity`:

```
test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.67s
```

`cargo test --all`, before the scope widening, caught the first out-of-plan file:

```
test result: FAILED. 97 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
---- the_shipped_corpus_matches_its_recorded_census stdout ----
crates/mochiko-cli/tests/validate.rs:1104: left: 705, right: 701 (live skill rules)
```

`cargo test --all`, after the widened fix, across all fourteen binaries:

```
351 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out   (exit 0)
```

## Read-back 5 — blast radius

Three changed tracked files, all of them crate tests, none of them data:

```
 crates/mochiko-cli/tests/fidelity.rs       | 46 +++++++++++++++++++++++-------
 crates/mochiko-cli/tests/matrix_similar.rs | 11 +++++--
 crates/mochiko-cli/tests/validate.rs       | 11 ++++---
```

No schema file, no migration, no strip, no view, no allowlist. The `views emit` run used for
measurement wrote only into the session scratchpad, never the repo.

## Widened scope — the two files outside the plan

The plan pinned P3 to fidelity.rs alone, so on finding these P3 stopped and asked. The lead
ruled (recorded in `wave2-ci-fix.md` under "Scope widening") that both may be touched.

### `crates/mochiko-cli/tests/validate.rs:1101-1106`

Test `the_shipped_corpus_matches_its_recorded_census`, mechanical — the same three substitutions
already derived and measured for fidelity.rs, plus the matching comment:

```diff
     // `0004` (the sonnet worker rung, 2026-09-05) minted six skill rules on
-    // `patterns-model-tiering`, two of them floors; the command side is untouched.
+    // `patterns-model-tiering`, two of them floors; `0007` (the seat default key,
+    // 2026-09-19) nets +4 on the same skill — one `supersede-rule` (−1) and five
+    // `mint-rule`s (+5), two of the five floors; the command side is untouched throughout.
+    // The superseded rule was itself a floor, so the skill's floor count nets +1.
     assert_eq!(command_rules, 321, "live command rules");
-    assert_eq!(skill_rules, 701, "live skill rules");
-    assert_eq!(command_rules + skill_rules, 1022, "live rules in total");
-    assert_eq!(skill_floors, 228, "skill floors");
+    assert_eq!(skill_rules, 705, "live skill rules");
+    assert_eq!(command_rules + skill_rules, 1026, "live rules in total");
+    assert_eq!(skill_floors, 229, "skill floors");
```

The neighbouring comment at lines 1107-1109 corroborates this report's read-back 2 independently:
it documents that a naive `grep -c 'class: floor'` returns 112 on the command side against 110
declared floors, because two matches are prose inside rule text.

### `crates/mochiko-cli/tests/matrix_similar.rs:989-996`

Test `the_detector_reproduces_the_live_runs_figures_over_the_corpus`, the judgment-bearing one.
Verified before writing, not bumped blind.

**The four-field readout.** `mochiko-cli migrate validate --report --plugin-root plugins/mochiko`,
run here rather than taken from the lead's quote:

```
=== similar-rule clusters (threshold 0.60) ===
none — no pair clears the threshold

=== stats ===
rules scanned: 1026 · in-kind pairs scored: 148160 · clusters: 0 (none)
allowlist-suppressed edges: 169
```

The crate test's detector then reproduced all four from the same corpus — the sweep passes with
the constants set to exactly these values, which is what says the two paths agree rather than
merely both being plausible:

```
test the_detector_reproduces_the_live_runs_figures_over_the_corpus ... ok
test result: ok. 48 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 71.33s
```

**`clusters == 0`, so this is a fixture refresh and not a near-dup finding.** The
near-dup-convergence doctrine is not reached: no pair clears the 0.60 threshold, and the
allowlist was not touched.

```diff
+    // Re-measured after `0007` (the seat default key): the scan moves from 1,022 to 1,026 — one
+    // rule superseded, five minted. The pair count falls while the rule count rises because pairs
+    // are scored within a kind: the retired rule declared no kind and so sat in the large
+    // `constraint` bucket, while the five mints land in the small `binding`, `latitude`, `bound`
+    // and `duty` buckets, so the pairs lost from the big bucket outnumber the pairs gained in the
+    // small ones. No new cluster surfaced and the suppressed set is unchanged — the allowlist was
+    // not touched.
-    assert_eq!(report.scanned, 1022, "rules scanned");
-    assert_eq!(report.scored, 148_353, "in-kind pairs scored");
+    assert_eq!(report.scanned, 1026, "rules scanned");
+    assert_eq!(report.scored, 148_160, "in-kind pairs scored");
     assert_eq!(report.clusters.len(), 0, "clusters");
     assert_eq!(report.suppressed_hits, 169, "allowlist-suppressed edges");
```

**One figure needed explaining rather than recording.** `scored` *falls* from 148,353 to 148,160
even though the corpus *gains* four rules, which would read as a contradiction left unexplained.
It is not: pairs are scored within a kind. The retired
`patterns-model-tiering.rostered-seats-never-retier` declared no `kind:` and so defaulted into the
large `constraint` bucket, where removing one member costs that bucket `n-1` pairs; the five
mints declare `binding`, `latitude`, `bound`, `bound` and `duty`, small buckets where each gain
adds only a few pairs. Verified against the migration source (`0007-seat-default-key.yaml` lines
35, 55, 70, 88, 103) and the retired rule's genesis declaration. The comment above now carries
this, so the next reader does not re-derive it.
