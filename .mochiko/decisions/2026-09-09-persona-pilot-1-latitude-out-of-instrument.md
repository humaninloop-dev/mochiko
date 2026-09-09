# Persona eval pilot 1 — latitude-conditional claims are out of instrument for the plan-only target

- **Status:** ruled
- **Date:** 2026-09-09
- **Context:** `primitive-eval-harness-v2` (accepted 2026-09-08) made the persona eval target
  plan-only (D2, `Contested`) and pre-registered a positive control for the first pilot: the
  v0.108.0 `staff-engineer` section (`## Delegating Bounded Work`, the Sonnet worker rung)
  must read absent before the edit and `reflected` under pass^k after it, on a golden sized so
  a seat would delegate (D11 as folded by C2). Pilot 1 ran to its read on 2026-09-09
  (`evals/agents/staff-engineer/runs/{probe,pilot1-prune,pilot1}`; pre-registration fill log):
  the control **failed** — on the delegation-forcing golden the seat delegated to a
  `model: sonnet` worker in one of three `post` replicates and declined in two ("briefing
  costs more than doing"); `pre` never delegated; the noise guard tripped at a 35 % `post`
  flaky share, almost entirely on the seven delegation claims; no common-claim regression on
  any golden; judge calibration 0.875 against the lead's arm-blind labels. The section grants
  latitude ("whether to delegate is your call per task"), so the edit changes plan behaviour
  *sometimes*; the instrument sees the behaviour when it occurs and cannot see "sometimes"
  under pass^k. The pre-registered remedies (one extra replicate; one instrument re-key) were
  not applied: the finding is about the shape of the edit under a plan-only observable, not a
  mechanical defect. The user ruled (2026-09-09): accept the finding.
- **Decision:**
  1. **Latitude-conditional claims are `out-of-instrument` for the plan-only persona target.**
     A claim whose observable behaviour exists only when the persona takes a path its own text
     leaves to its discretion (the seven delegation claims conditional on `whether-delegate-call-per`)
     carries partition `out-of-instrument`, reason `latitude-conditional`, unless a golden makes
     the path non-optional by construction. The partition vocabulary gains that reason.
  2. **Pilot 1 closes on this reading.** The persona target stands for non-latitude claims: the
     read of common claims was clean (no regressions, `pre` 15 % flaky), calibration passed, the
     mechanics held on every assert. Its ship bar (a) is recorded as *not met for latitude
     edits by construction*; (b) and (c) are met for the non-latitude read.
  3. **The second pilot proceeds on `validator`** (D9), whose claims are `must`-shaped.
  4. Two instrument watches carry into pilot 2: the Haiku judge credits an `Explore` read
     dispatch as worker delegation and credits declined or hypothetical conditional paths
     (fold I9's failure mode — a judge-prompt re-key at pilot 2's mint); Sonnet pairwise is
     position-biased (1 of 12 consistent), advisory only.
- **Rationale:** This is D2's `Contested` cost materialising exactly where the record predicted
  it (contact-dependent behaviour); the ruling names the boundary rather than tuning the
  instrument until the control passes (M5: a control gates the instrument, never the persona,
  and re-keying a golden until a "may" reads as a "must" would grade the golden, not the
  persona). Rejected: **re-key once more** (~$4; a stronger golden would test whether the seat
  can be made to delegate, not whether the edit changed it); **re-shape to trace-based
  execution for producers** (reopens D2 — a legitimate future ruling, not this pilot's).
- **Alternatives considered:** read `added` latitude claims at pass^1 (presence in any
  replicate) — rejected as a post-hoc read-rule change that the pre-registration forbids and
  that would hide the variance the pilot found.
