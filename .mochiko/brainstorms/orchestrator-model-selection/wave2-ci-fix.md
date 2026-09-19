# Wave 2 — CI fix: `crates/mochiko-cli/tests/fidelity.rs` fixed expectations stale against migration 0007

**Trigger:** PR #37 (`worktree-seat-default-key` → `main`) CI job `crate` fails at the `Test`
step — `cargo fmt` and `cargo clippy` passed (they run before `Test` in `.github/workflows/ci.yml`
and the job stops on first failure, so their green is implied, not separately verified here; V1
re-confirms). Four test failures, all frozen-expectation drift against migration
`0007-seat-default-key.yaml`, which this wave's fidelity tests never anticipated because no
producer was assigned crate ownership.

**Governance:** `.claude/rules/mochiko/rust-cli.md` — "Every crate unit lands on a lead-approved
plan with an independent non-author code review — author≠grader extends to code." This is that
plan; P3 executes it verbatim (no design latitude — every substitution below is evidence-derived,
quoted from the actual render); V3 reviews.

## Root cause, each confirmed against the live render (not guessed)

1. `the_log_replays_into_a_deliverable_state` — expects `sequences() == vec![1,2,3,4]`; the log
   now has five migrations, sequences `1,2,3,4,7` (0005/0006 exist only on `primitive-evals-v2`,
   not this branch).
2. `the_sidecar_anchors_ride_their_rules` — walks the full log expecting every one of the 597
   sidecar-anchored rules to still be live. Migration 0007 `supersede-rule`s
   `patterns-model-tiering.rostered-seats-never-retier` under its own anchor — a protected exit
   the log's own anchor rule requires and permits. The test's own comment anticipates this
   ("a re-key here that names the rule") but no allowlist mechanism exists yet — this wave adds
   the first one.
3. `the_corpus_census_holds_through_the_log` — expects `skill_rules == 701`. Migration 0007 nets
   +4 on `patterns-model-tiering` (1 `supersede-rule` −1, 5 `mint-rule` +5) → 705.
4. `the_fourth_migration_added_the_worker_rung_to_the_tiering_floor` — reads the **full** log
   (`replay::load(&log_dir())`), not migration 4 in isolation, so a later rework of a rule it
   checks is in scope by the test's own design. Two needles are now stale: the
   `class-key-session-tier` needle `"model-tiered-seats D5"` (0007 reworded that clause to
   `"orchestrator-model-selection D1/D3"` — confirmed verbatim by render) and the skill's floor
   pin, `6` (now `7`, confirmed by the preamble's `class: floor · 7 rules`). The
   `override-is-the-pin` needles and the reserved-note needle are unaffected — confirmed by
   render/migration-source respectively, quoted below.

## P3 — the four edits, exact (file: `crates/mochiko-cli/tests/fidelity.rs`)

**Edit 1 — `the_log_replays_into_a_deliverable_state` (around line 171-177).**
```diff
     assert_eq!(
         replay.sequences(),
-        vec![1, 2, 3, 4],
+        vec![1, 2, 3, 4, 7],
         "genesis, wave 4's fail-conditions reword, wave 6's two-arm retirement, the sonnet \
-         worker rung"
+         worker rung, 0007's seat default key"
     );
```

**Edit 2 — `the_sidecar_anchors_ride_their_rules` (around line 519-533).** Add a documented
allowlist above the second loop and skip listed ids there, asserting they are genuinely retired
(never silently stale):
```diff
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
             .get(id.as_str())
             .unwrap_or_else(|| panic!("{id}: the sidecar's rule is no longer live"));
         match rule.anchor.as_deref() {
             Some(carried) => assert_eq!(carried, expected, "{id}: the anchor moved"),
             None => panic!("{id}: the sidecar's anchor was cleared"),
         }
     }
```
(Exact insertion point: the constant goes immediately before the `let live = replay::load(...)`
line or immediately before the `for (id, expected) in &anchors` loop — P3's call, either reads
correctly; keep it above the loop it serves.)

**Edit 3 — `the_corpus_census_holds_through_the_log` (around line 588-595).**
```diff
-    // `0004` (the sonnet worker rung, 2026-09-05) minted six skill rules on
-    // `patterns-model-tiering`, two of them floors; the command side is untouched.
+    // `0004` (the sonnet worker rung, 2026-09-05) minted six skill rules on
+    // `patterns-model-tiering`, two of them floors; `0007` (the seat default key,
+    // 2026-09-19) nets +4 on the same skill — one `supersede-rule` (−1) and five
+    // `mint-rule`s (+5), two of the five floors; the command side is untouched throughout.
     assert_eq!(command_rules, 321, "live command rules");
-    assert_eq!(skill_rules, 701, "live skill rules");
-    assert_eq!(command_rules + skill_rules, 1022, "live rules in total");
+    assert_eq!(skill_rules, 705, "live skill rules");
+    assert_eq!(command_rules + skill_rules, 1026, "live rules in total");
```
**`skill_floors` — confirmed by the lead against the pre-0007 render captured earlier this
session:** `patterns-model-tiering.rostered-seats-never-retier` was itself `class: floor`
before supersession (render: `[class: floor · labels: boundary]`). So `0007` removes one floor
(the supersession) and mints two more (`seat-default-key`, `seat-deviation-bounds`) — net **+1**
floor for this skill. The census's `skill_floors` therefore moves from its current value (as the
failing test would report it, not necessarily still 228 — this assertion never ran because
`skill_rules` panicked first) to **that value + 1**. P3 confirms the pre-edit value independently
(the read-back step below) before writing the post-edit assertion — do not trust this arithmetic
alone.

**Edit 4 — `the_fourth_migration_added_the_worker_rung_to_the_tiering_floor` (around line 928-948).**
```diff
     let reworded: [(&str, &[&str]); 2] = [
         (
             "patterns-model-tiering.class-key-session-tier",
             &[
                 "never tiered down",
                 "sonnet-worker-rung",
-                "model-tiered-seats D5",
+                "orchestrator-model-selection D1/D3",
             ],
         ),
         (
             "patterns-model-tiering.override-is-the-pin",
             &["`model: haiku`", "`model: sonnet`"],
         ),
     ];
```
and further down:
```diff
     let floors = schema.rules().filter(|rule| rule.is_floor()).count();
-    assert_eq!(floors, 6, "the skill's floor pin");
+    assert_eq!(floors, 7, "the skill's floor pin");
```
One-line doc-comment update above the test (currently says "the floor pin at six"):
```diff
-/// minted rules on `patterns-model-tiering`, two of them floors; the two reworded floors still
-/// floors, naming both rungs and the D5 clause; the floor pin at six; the reserved section's
-/// note naming its new reservation.
+/// minted rules on `patterns-model-tiering`, two of them floors; the two reworded floors still
+/// floors, naming both rungs (D5's text since superseded by `0007`'s seat default key — this
+/// test reads the full log, so the current wording is what it checks); the floor pin now seven
+/// after `0007`; the reserved section's note still naming its new reservation.
```

## Read-back P3 owes before reporting (do not skip)

1. Before editing: `cargo test -p mochiko-cli --test fidelity 2>&1 | grep -E "left:|right:|panicked"` — confirm the four failures reproduce locally exactly as CI reported (rules out an environment difference).
2. `skill_floors` — measure it directly (e.g. a throwaway `println!` in a scratch binary, or count by hand from the migration's mint/supersede ops: genesis had some N; 0004 added 2; 0007 adds 2 more and removes 0 — since `rostered-seats-never-retier` is `class: must`, not `class: floor`... **check this**: if that rule was NOT a floor, the floor count is unaffected by its supersession, and only the two new floor mints matter). Confirm from the migration source which of the five 0007 mints are floors, and whether the superseded rule was itself a floor, before writing the census assertion.
3. After editing: `cargo fmt --all --check` and `cargo clippy --all-targets -- -D warnings` — both must stay green; this is Rust source, not schema.
4. `cargo test --all` (not just `--test fidelity`) — full crate suite, in case another test also touches these numbers.
5. Nothing outside `crates/mochiko-cli/tests/fidelity.rs` changes. No schema file, no migration, no strip.

## Report

`.mochiko/brainstorms/orchestrator-model-selection/reports/p3-report.md` — the exact diff applied,
the read-back results, `cargo test --all` tail (pass count), `cargo fmt --check` and `cargo
clippy` exit codes. Message the lead the report path plus the decisive lines.

## Scope widening (lead, mid-wave — two more stale-pin files P3 correctly stopped at)

P3's read-back surfaced two more frozen-expectation files the original plan did not name, because
they carry the identical drift class as edits 1 and 3 above:

- `crates/mochiko-cli/tests/validate.rs:1103-1106`,
  `the_shipped_corpus_matches_its_recorded_census` — the same three numbers as Edit 3: `701 → 705`
  (rule count), `1022 → 1026` (total), `228 → 229` (skill floors). **Mechanical — approved,
  identical evidence already confirmed for Edit 3.** P3 applies it.
- `crates/mochiko-cli/tests/matrix_similar.rs:993-996`,
  `the_detector_reproduces_the_live_runs_figures_over_the_corpus` — the full-corpus similarity
  sweep (`MOCHIKO_FULL_SIMILAR=1`), which reads `rules scanned` (frozen 1022, corpus now 1026) and
  also, per P3, `scored` and `suppressed_hits`, gated behind `clusters == 0`. **Not blindly
  mechanical — `clusters` is a finding about migration 0007's text, not a count to refresh.**
  Corroborating evidence already in hand from this session's own release-gate run, same tree,
  same corpus, post-bump: `mochiko-cli migrate validate --report --plugin-root plugins/mochiko`
  printed `rules scanned: 1026 · in-kind pairs scored: 148160 · clusters: 0 (none) ·
  allowlist-suppressed edges: 169`. P3 verifies the crate test computes the *same* numbers from
  the *same* corpus (not merely trusts this quote) before writing anything, and reports every one
  of the four fields (`scanned`, `scored`, `clusters`, `suppressed_hits`) it observes. **If
  `clusters != 0`: STOP, do not write a number, message the lead — that is a near-dup finding
  needing a ruling under the near-dup-convergence doctrine (extraction bar / keep-distinct
  allowlist), not a fixture update.** If `clusters == 0` and every other field matches this
  session's quoted run: mechanical, update the four frozen constants, proceed.

Same governance as the original plan (P3 executes, V3 grades); this is a disclosed, evidence-cited
supersession of the plan's original file scope, not new design latitude.

## V3 — independent review

Grade P3's diff against this plan and against the actual migration/render (never against P3's
say-so): re-derive `skill_floors` and the `override-is-the-pin`/reserved-note needles
independently; confirm `RETIRED_SIDECAR_ANCHORS` is genuinely narrow (exactly the one id, not a
blanket skip); confirm no other test in the crate references the retired rule id or the stale
numbers; run `cargo test --all`, `cargo fmt --all --check`, `cargo clippy --all-targets -- -D
warnings` itself. Report `.mochiko/brainstorms/orchestrator-model-selection/reports/v3-audit.md`.
