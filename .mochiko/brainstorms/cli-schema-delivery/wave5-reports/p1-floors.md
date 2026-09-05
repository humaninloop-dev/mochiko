# P1 — the `floors:` preamble line (report)

**Status: complete, four layers green, nothing committed.** Two files touched, exactly the seat's
scope: `crates/mochiko-cli/src/render.rs` (+25/−4) and `crates/mochiko-cli/tests/render.rs` (+289).

## What landed

In `render::preamble`, after the pins block and before the legend. The pin's count and the index's
ids come from one iterator — `schema.rules().filter(|rule| rule.is_floor())` — collected once, so
the two cannot disagree by construction rather than by a test's vigilance. `RuleSchema::rules()`
walks sections in declared order and rules in section order, which is render order. The block is
blank-line separated like every other preamble block, one line however long, `floors: none` when
the set is empty. The legend const and its 845-byte pin are untouched, and the end line still reads
`preamble · 0 rules`.

## Tests — 7 added, 37 pass in `tests/render.rs`, 338 across the crate

All seven written and confirmed red before the render change; the failure in each was the missing
line, not a fixture defect.

| test | what it holds |
|---|---|
| `the_preamble_indexes_every_floor_id_in_render_order` | the fixture command's three floors, drawn from three different sections, in order |
| `a_skill_preamble_indexes_its_floor_ids_too` | the same for the fixture skill |
| `the_floor_index_sits_between_the_pins_and_the_legend` | placement, and that the index is not a rule |
| `a_primitive_with_no_floor_rule_indexes_none` | the `none` branch |
| `the_shipped_floor_index_carries_the_recorded_sets` | `implement`'s 34 ids and `review-brainstorm`'s 9, written out |
| `every_shipped_floor_index_matches_its_pin` | index length equals pin, all 36 primitives |
| `the_widest_shipped_floor_index_is_the_size_the_wave_recorded` | `implement` at 945 chars / 978 bytes |

The `none` branch needed a fixture: no shipped primitive has an empty floor set, the corpus running
from 2 floors (`patterns-plan-minimalism`) to 34 (`implement`). It arrives as a second migration
importing a floorless review skill, following the precedent of the anchor test rather than
perturbing the shared `RULES_LOG` that some thirty tests read. That fixture carries all six
canonical review sections, which the validator's census requires.

## The two lines, verbatim from the shipped log

```
floors: impl.gate-design-checkpoint · impl.gate-card-confirm · impl.gate-final-acceptance · impl.graded-fold · impl.author-grader-default-fail · impl.baselines-never-in-place · impl.deviation-gate · impl.constitution-supremacy · impl.constraint-challenge · impl.attempt-per-grade · impl.attempt-exemption-user-only · impl.no-progress-stop · impl.epic-member-halt · impl.gap-rework-bound · impl.gates-never-triaged · impl.minimalism-advisory · impl.lane-never-widens · impl.sound-loop-floor · impl.transport-floor · impl.fail.sufficiency-unrecorded · impl.fail.design-skipped · impl.fail.card-independence · impl.fail.card-unchecked · impl.fail.quality-gate · impl.fail.no-evidence · impl.fail.regression · impl.fail.baseline-in-place · impl.fail.deviation-unresolved · impl.fail.store-landing-incomplete · impl.fail.ungraded-fold · impl.fail.gap-finding-missing · impl.fail.skip-unstated · impl.fail.spec-gap-unresolved · impl.fail.no-acceptance
```

```
floors: review-brainstorm.never-in-the-room · review-brainstorm.blind-map-before-record-contact · review-brainstorm.author-grader · review-brainstorm.contested-needs-new-angle · review-brainstorm.never-default-ready · review-brainstorm.unverifiable-claim-is-finding · review-brainstorm.evidence-floor · review-brainstorm.verdict-is-input · review-brainstorm.findings-through-leads-pen
```

Each rendered with `mochiko-cli rules <name> --section preamble --log-dir plugins/mochiko/migrations`
off a fresh `cargo build --release`. `implement`'s pin reads `- class: floor · 34 rules` beside 34
ids; `review-brainstorm`'s reads 9 beside 9.

## Preamble byte deltas — measured over all 36 primitives

The added block is the line plus its separating newline, so each primitive pays
`len("floors: …") + 2` bytes on every preamble render.

| | bytes |
|---|---|
| `implement` preamble, before → after | 3,485 → 4,465 (+980, the corpus's largest) |
| `review-brainstorm` preamble, before → after | 1,972 → 2,366 (+394) |
| smallest delta (`patterns-plan-minimalism`, 2 floors) | +99 |
| mean delta across 36 primitives | +386 |
| total added across the corpus | 13,881 |

The largest preamble is now 4,465 bytes against the 30,000-character inline ceiling, so the
headroom the per-section chunking exists to protect is untouched. The largest section render is
unchanged; `no_shipped_section_renders_past_the_inline_ceiling` still passes without edit.

## Verification

`cargo test --all`, `cargo fmt --all --check`, `cargo clippy --all-targets -- -D warnings` and
`cargo audit --deny warnings` all exit 0.

Ripple checked and none found. The only callers of `render::preamble` and `render::section` are the
`rules` subcommand at `crates/mochiko-cli/src/cli.rs:321`; `mochiko-cli views emit` never reaches
the render, so the derived views cannot move on this change. The contract suite's `parse_preamble`
reads only the `sections` block, and `tests/cli.rs` asserts head and tail lines alone — both are
unaffected, and both belong to P3 in any case.

## Deviations

None from the approved plan. Two incidental notes for the lead:

- The wave plan's §0 measurement and §2 both describe `implement`'s line as "~1 KB"; the measured
  figure is 945 characters / 978 bytes, so the estimate held.
- `mochiko-cli views emit` output still differs from the hand-authored files under
  `plugins/mochiko/schemas/`, in comment headers and YAML block-scalar style. That is the
  pre-wave-6 state, unrelated to this change, and noted only so the diff is not read as a
  regression from it.
