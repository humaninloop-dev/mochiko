# Second list — the floor fork (what stays non-departable at any stakes, and the test that sorts it) · 2026-10-25

Drawn blind by a fresh seat from the session frame and the question only; the lead's options were not seen.

## Facts on the terrain

- **`class: floor` is a delivery class today, not a departability class.** The render legend says a floor rule "is always delivered whatever its when:". Yet the Boundaries section calls itself "the non-waivable floor", and floor-tagged rules also sit in the reserved, ways-of-working and fail-conditions sections. The tag does two jobs at once.
- **The 2026-08-01 ruling had a four-invariant floor.** It held user gates, author≠grader, self-declared bounds, and an honest trail of skips. Rigor was read on four factors: reversibility, blast radius, precedent, and input confidence.
- **The 2026-08-02 realignment dropped three of those carriers.** It dropped counted bounds, the run-start weight card, and the departure trail, and put producer plan approval in their place.
- **Implement today carries counted bounds as floor again.** `impl.attempt-per-grade` and `impl.gap-rework-bound` are both floor. A skip trail exists only for the gap-finding pass on delta and lane runs (`impl.fail.skip-unstated`).

## Roads

1. **Keep the declared floor as it stands.** The 40 `class: floor` rules stay non-departable, and every `class: must` rule becomes departable by recorded line. The test is the existing tag.
   - *Example:* `impl.attempt-per-grade` stays fixed, while `impl.delta-reverification`'s scoping becomes the lead's to widen or narrow.
   - *Cost:* The lead gets almost no new latitude, because the depth-bearing rules are already floor (gap finding, bounds, gates never triaged). The tag also keeps meaning two things.

2. **Ownership floor.** Only rules that keep a call with its rightful owner are floor: a user's ruling stays the user's, and a grade stays with a non-author. Which steps run, and how deep, is departable. The test asks whether departing the rule would let the lead take a call reserved to someone else.
   - *Example:* `impl.gate-card-confirm` and `impl.author-grader-default-fail` stay floor. `impl.fail.gap-finding-missing` becomes departable.
   - *Cost:* Evidence and the record layer are not protected by ownership. Real-infrastructure evidence and unmarked baseline writes would need a second pin, or they fall out of the floor.

3. **Revive the 2026-08-01 four invariants.** The floor is user gates, author≠grader, declared bounds, and an honest trail of skips. The test asks whether the rule guards one of four failure modes: self-acceptance, self-grading, a runaway loop, or invisible corner-cutting.
   - *Example:* `impl.no-progress-stop` stays floor because it guards runaway loops. `impl.fail.gap-finding-missing` becomes departable under a trail line, as `impl.fail.skip-unstated` already does for delta runs.
   - *Cost:* It reverses part of the 2026-08-02 drop of the trail and the weight card, a ruling the user ratified deliberately. A run-start stakes read likely returns as a stop.

4. **Must-not-break floor.** The floor is exactly the session's five must-not-breaks: user gates, real-infrastructure evidence wherever verification runs, an honest skip trail, the engine rulings of 0.120 and 0.121, and clone-only install with CLI delivery. The test asks whether departing the rule breaks one of the five.
   - *Example:* `impl.fail.no-evidence` stays floor. `impl.gap-rework-bound`'s default of 2 becomes departable.
   - *Cost:* The engine-rulings bucket is not itemized, so each rule needs an inventory pass to know whether 0.120/0.121 pins it. Author≠grader is absent from the list and survives only in a form the lead rules.

5. **End-state floor, means departable.** The `kind: fail` set, the end states a finished run must show, stays floor. The process rules each fail rule `enforces:` become departable means. The test asks whether the rule is a state the finished run must exhibit or a step toward it.
   - *Example:* `impl.fail.regression` stays floor, but `impl.regression-sweep` may be scoped by stakes as long as no regression stands.
   - *Cost:* Several fail rules are step-shaped ("a run without its gap-finding pass"), so the fail set itself must be re-cut. An end-state claim without a fixed check also invites thin evidence.

6. **Verdict-honesty floor.** Whether a step runs, and how deep, is departable. What a step's result is allowed to mean is floor. The test asks whether the rule governs presence or depth (departable) or the meaning of an outcome (floor).
   - *Example:* `impl.gates-never-triaged`, `impl.minimalism-advisory`, `patterns-sound-loop.only-real-seats` and default FAIL stay floor. The full-suite re-sweep becomes the lead's call.
   - *Cost:* A lead could skip whole verification kinds on a selection run and stay honest about it. Author≠grader as a concept needs its own pin, because honesty alone does not force any review to happen.

7. **Stakes-invariance test.** A rule is floor when the harm it prevents does not shrink as stakes fall, because the damage lands on the record or on trust rather than on the product. The test asks whether a low-stakes run makes the breach cheaper.
   - *Example:* `impl.fail.unmarked-baseline-write` stays floor, since silent baseline corruption costs the same on a one-line fix. The count of 3 in `impl.attempt-per-grade` becomes departable, since its harm scales with stakes.
   - *Cost:* The test is judgment-heavy and two readers will split on some rules. It needs a one-time ruled inventory, never a run-time call.

8. **Detection test.** A rule is floor when no later check would catch its breach. A rule backstopped by a downstream gate is departable. The test asks whether anything after this point would notice if it were skipped.
   - *Example:* `impl.author-grader-default-fail` stays floor, since a self-cleared output leaves nothing for anyone to catch. The seat-plan grade in `impl.plan-approval-producers` becomes departable, since leg 2 still grades the artifact.
   - *Cost:* The floor shifts whenever backstops thin. A lead departing two backstops at once can expose a third rule that was departable only because the others held.

9. **Cheap-to-keep test.** Floor is whatever costs near nothing to keep at any stakes, since departing it saves nothing. This was the 2026-08-01 rationale for its four invariants. The test asks whether keeping the rule on a low-stakes run costs more than it protects.
   - *Example:* `impl.fail.skip-unstated` costs one line and stays floor. The full regression sweep is expensive and becomes departable in depth.
   - *Cost:* Cheapness is not importance. An expensive but vital check, such as the real-infrastructure `**TEST:**` gate, would land departable unless a second test pins it.

10. **Presence floor, depth dial above a stated minimum.** Every verification kind is always present, and each skill states its own minimum depth. The lead dials depth upward from that minimum by stakes. The test separates a rule's presence clause, which is floor, from its depth clause, which is departable down to the minimum.
    - *Example:* Gap finding on a selection run always happens, but at low stakes it is one blind probe pass instead of the full probe kit and mutation lens of `mochiko:testing-gap-finding`.
    - *Cost:* Each `testing-*` and `review-*` skill needs a new minimum-depth clause. A stated minimum per skill also edges toward the codified tier ladder that 2026-08-01 rejected.

11. **Library floor by pointer.** Floor is what the core skills own as library-wide invariants: the six sound-loop floors, the transport floor, and default FAIL in `review-*`. Implement's own rules shrink to its user gates plus pointers. The test asks whether the rule is a library-wide invariant owned by a skill or a rule local to implement.
    - *Example:* `impl.sound-loop-floor` stays as a pointer. Implement-local counts such as `impl.gap-rework-bound` become departable.
    - *Cost:* Implement-local user gates such as the card confirm need an explicit keep. A change to one skill's floor moves every command at once, which is the reach the frame wants but also the blast radius.

12. **Three-tier authority.** Each rule declares who may lift it: nobody, the user at run-open, or the lead by trail line. The test asks whose cost the breach lands on. Record integrity and secrets go to nobody, verification presence to the user, and depth and counts to the lead.
    - *Example:* `impl.fail.unmarked-baseline-write` is lifted by nobody. Skipping gap finding on a selection run becomes the user's call at run-open, written into the sufficiency report as `impl.user-runopen-rulings` already does. The attempt count becomes the lead's.
    - *Cost:* It adds a run-open card, close to the weight card the 2026-08-02 realignment dropped. It also hands part of the rigor call back to the user, against "the lead rules pragmatically."

13. **Label-keyed sort.** Use the existing labels registry as the test. `user-gate`, `independence`, `evidence` and `landing` rules are floor, while `attempt-economy`, `reporting`, `seats` and `scope-entry` rules are departable.
    - *Example:* `impl.fail.no-evidence` is labelled `evidence` and stays floor. `impl.attempt-per-grade` is labelled `attempt-economy` and becomes departable.
    - *Cost:* The legend says labels bind nothing, so this repurposes them into binding. Multi-label rules conflict, for example `impl.gap-rework-bound`, which carries both `attempt-economy` and `user-gate`.

14. **Trail-only minimal floor.** The floor is user gates, no self-clearing, an honest trail of every departure, and "verification that runs uses real infrastructure." Everything else is departable, presence of any check included. The test asks whether the departure can be recorded and audited after the fact.
    - *Example:* Even the regression sweep (`impl.regression-sweep`) may be skipped with a trail line. `impl.fail.no-evidence` binds only when verification is claimed.
    - *Cost:* The 2026-08-01 evidence that every review found something on 18 of 18 runs says skipped reviews will bite. Record-layer protections such as unmarked-baseline writes become departable, which collides with GI-005.

15. **Asymmetric departure, a modifier for any road above.** Departing upward, adding depth or a step, is free and needs no trail. Only departing downward meets the floor test.
    - *Example:* The lead may add gap finding to a delta run without a ruling, while dropping it on a selection run meets whatever test the floor road applies.
    - *Cost:* It settles nothing alone. It needs a downward test from one of the roads above, and an upward step still spends attempts under `impl.attempt-per-grade`.
