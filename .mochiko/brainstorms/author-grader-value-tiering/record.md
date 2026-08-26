# Record — author≠grader questioned: value-tiered verification

**Status:** open
**Opened:** 2026-08-26
**Topic:** Question the author≠grader philosophy wholesale and find a pragmatic split: independent
verification reserved for high-value surfaces, author self-verification accepted for low-value
ones.

---

## Ground facts

- **F1 — Where author≠grader binds today (five sites).** (1) GI-004, NON-NEGOTIABLE: every
  shipped-primitive edit passes the author≠grader audit before the `plugin.json` bump that ships
  it. (2) Skill-library axis 5: every reviewable artifact is graded by a structurally independent
  validator. (3) `patterns-sound-loop` floor: judgment-authored write × governing surface obliges
  an independent non-author review leg (mechanical/transcription/fix-on-sight exempt; desk delta
  cards lost their exemption by a `Contested` ruling). (4) Brainstorm end-stage cold review with
  the sizing gate (pair/single/none; "none" = recorded waiver). (5) Pipeline producer↔validator
  pairs (`mochiko:validator`, the `review-*`/`validation-*` skills, qa-engineer's independence).

- **F2 — Prior retention ruling.** `architect-role-pushback-and-abstraction` D3 (2026-08-13)
  explicitly retained author≠grader: the additive bias the session hunted was traced to charter
  asymmetry, not to independence itself. A wholesale change supersedes that ruling — recorded
  supersession, user-ruled.

- **F3 — Wave audit outcomes on record (index, v0.65.0 → v0.81.0).** Zero-fix-round waves:
  v0.65.0 (all PASS round 1) · v0.70.0 (2/2) · v0.71.0 (2/2) · v0.72.0 (8/8) · v0.73.0 (13/13)
  · v0.75.0 (12/12). Catch waves: v0.67.0 (1 of 4 clusters FAIL → fix) · v0.68.0 (desk audit
  caught the lead's unrecorded coordination ruling — a GI-006 breach, user-ratified mid-wave)
  · v0.76.0 (one fix round: CI cargo-audit flag) · v0.79.0 (V2 FAIL → fix round; V2's F1 catch
  killed a false premise in the review) · v0.81.0 (all 4 clusters FAIL round 1 → fix rounds →
  4/4 PASS). Roughly half the recorded waves produced zero blocking findings; the catch waves
  include one governance breach and several substantive defects.

- **F4 — Value-tiering already exists in three ruled forms.** Sound-loop's kind key exempts
  mechanical/transcription/fix-on-sight work; the brainstorm review sizing gate lets the user
  choose pair/single/none with a recorded waiver; the two-reviewer overlap finding (kinako,
  review-brainstorm eval) showed only 2 of 6 Important findings found by both reviewers — naive
  halving of reviewers loses Important-class findings.

- **F5 — Cost side is unmeasured.** The platform exposes no session-readable token totals; cost
  evidence is seat counts and wall time only. Every validator seat is a fresh session-tier agent
  reading the full cluster cold.

- **F6 — GI-004 is governance.** Amending or softening it is a governance event (fact-profile
  amendment via `/mochiko:setup` per CLAUDE.md governance operations). This record can rule the
  intent; landing requires the governance amend path.

## Decisions

*(none yet)*

## Session trail

- **Q1 — driving pain.** Offered cost / latency-ceremony / low-yield / other. User: **(a) cost and
  (b) latency/ceremony primary**. Yield not disputed — the catches are real; the price per catch
  is the problem. Frames the goal: keep catch capability where expected value is high, cut spend
  and ceremony where it is low.
