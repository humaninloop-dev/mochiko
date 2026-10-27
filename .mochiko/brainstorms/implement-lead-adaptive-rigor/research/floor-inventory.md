# Floor inventory — implement's 40 floors under D1's four-question test

Author: the session lead · date 2026-10-27 · input: the implement rules rendered 2026-10-25
from `plugins/mochiko` (0.122.0) via `mochiko-cli rules implement --section <id> --plugin-root
plugins/mochiko`, the 40 rules carrying `class: floor`. Test applied: a rule stays floor only if
departing from it would let the lead (1) take someone else's call, (2) misstate a result, (3) hide
a skip, or (4) break a machine check; a rule with no depth dimension is marked invariant. This is
the lead's reading, to be confirmed by the user under Q3 and read cold at review.

## Result

| Outcome | Count | Rule ids |
|---|---|---|
| Keep — question 1, someone else's call | 17 | `impl.gate-design-checkpoint` · `impl.gate-card-confirm` · `impl.gate-final-acceptance` · `impl.design-first-write` · `impl.deviation-gate` · `impl.constitution-supremacy` · `impl.constraint-challenge` · `impl.attempt-exemption-user-only` · `impl.epic-member-halt` · `impl.gap-rework-bound` · `impl.thinking-stage` · `impl.lane-never-widens` · `impl.fail.design-skipped` · `impl.fail.deviation-unresolved` · `impl.fail.no-acceptance` · `impl.fail.design-outside-ruled-set` · `impl.fail.cards-outside-order` |
| Keep — question 2, misstate a result | 9 | `impl.author-grader-default-fail` · `impl.sound-loop-floor` · `impl.gates-never-triaged` · `impl.fail.card-independence` · `impl.fail.quality-gate` · `impl.fail.no-evidence` · `impl.fail.regression` · `impl.fail.spec-gap-unresolved` · `impl.fail.unreviewed-baseline-diff` |
| Keep — question 3, hide a skip | 4 | `impl.fail.skip-unstated` (widened to every stated skip at every scope) · `impl.fail.sufficiency-unrecorded` · `impl.fail.card-unchecked` · `impl.no-progress-stop` |
| Keep — question 4, break a machine check | 6 | `impl.artifact-home` · `impl.baselines-in-place-marked` · `impl.baseline-diff-review` · `impl.fail.unmarked-baseline-write` · `impl.fail.store-landing-incomplete` · `impl.transport-floor` |
| Keep — invariant, no depth to adapt | 3 | `impl.minimalism-advisory` · `impl.design-audit-advisory` · `impl.attempt-per-grade` (the count is already the user's at run-open) |
| Re-class | 1 | `impl.fail.gap-finding-missing` — gap-finding is a means toward "promised behaviour not missed"; departing it usurps no call, misstates nothing if stated, hides nothing if trailed, breaks no machine check. Re-keyed on the `skip-unstated` model: a run without its gap-finding pass *and without the skip stated at run-open and in the final-validation report* fails. |

Stakes-invariance check on the 39 kept: each holds the same for a one-line cosmetic fix and a
payments rewrite — a user gate is the user's either way, a failing gate fails either way, a
marker is machine-read either way. None varies with stakes.

## Reading

The 40 floors are almost entirely what D1 keeps: gates, honesty, the trail, and the record layer.
The depth the user feels is not in the floor layer; it is in the `class: must` layer — the steps
that run at full breadth on every run whatever the stakes. The departable set is therefore a
re-keying of must-class rules, not a stripping of floors; only one floor moves, and it moves to a
stated-skip form, not away.

## Candidate departable steps (must-class today) — the lead's list before the blind list

Each: the step · the defect it was added to prevent (lead's reading, provenance to be cited on
the ruling) · the thinner form in the band · what still catches the defect.

1. **Per-cycle verification-seat grade** — builder self-clearing, green that proves nothing. Thin:
   one grade over several cards (blind map D6), never zero — the non-author read before
   acceptance (D1) stays. Catch: the smoke probe plus the final non-author read.
2. **Card review before confirm** — cards cut wrong before the user confirms. Thin: a lighter
   form of leg 2 per Q4's menu, never the user's confirm alone (`patterns-sound-loop.leg-2-non-author-review`,
   F17, is the collision — resolved at Q5 where the lever lands in the skill).
3. **Full gate suite every cycle** (`impl.gates-full-suite`) — a cycle breaking an earlier one.
   Thin: touched-module gates per cycle, the full suite once in the smoke. Catch: the smoke's
   suite run; `impl.fail.quality-gate` stays floor.
4. **Cold verification** (`impl.cold-verification`) — per-cycle grades miss integration. Thin:
   the smoke probe is the cold look in the band. Catch: the smoke; the final non-author read.
5. **Regression sweep** (`impl.regression-sweep`) — a delivered feature broken. Thin: the touched
   territories' gates plus the suite in the smoke, not every delivered gate. Catch:
   `impl.fail.regression` stays floor as an end state.
6. **Gap-finding pass** (`impl.gap-finding-scope`) — promised behaviour both builder and test
   author missed. Thin: skipped in the band, stated at run-open and in the report; the smoke's
   hand probes are its lighter form. Catch: the non-author read; `impl.fail.spec-gap-unresolved`
   stays floor for anything found.
7. **Design review pair** (`impl.design-review-pair`) — an infeasible or incomplete design signed.
   Thin: a solo reviewer in the band. Catch: the user's design signature (floor) over a reviewed
   package.
8. **Plan-approval leg** (sound-loop leg 1, `mochiko:review-seat-plan`) — seats drifting scope
   before producing. Thin: skipped in the band for a seat whose brief fences one file set
   (blind map E3). Catch: leg 2's read of the produced surface.
9. **Design audit** (`ux_bearing`) — already advisory; thin: skipped in the band, stated.

Never move (beyond the floors): the sufficiency check at entry — the independent read the
stakes read sits beside, never replaces (blind map C4); the landing verification seat — it grades
the record layer (question 4) and judgment entries (question 2); the mutation lens — already keyed
on `depth=high` and left as is (out-of-scope fence 3).
