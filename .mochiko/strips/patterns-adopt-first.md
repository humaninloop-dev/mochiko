# Strip notes — `skills/patterns-adopt-first`

Entry formats: `strips/README.md`. Skill born at v0.73.0 (the build-vs-off-the-shelf wave,
D2–D5); this file opens with the first edit that superseded any of its shipped text.

## [v0.91.0] Fix round — the tooling-defaults carve-out aligned to this skill's own re-keyed Siblings line (advisory)

- **Disposition:** superseded → "not a design- or build-time decision (see Siblings)".
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D1 mechanic (d)). Raised as an
  **advisory** by the v0.91.0 wave audit: the main pass re-keyed this skill's Siblings line to
  "this skill the design- and build-time decision" but left this cross-reference to it reading
  "not a plan decision", so the bullet pointed at a line that no longer used its words.
- **Content (superseded text, verbatim):**

  ```
  - **Project tooling defaults** — "established, never hand-rolled" for linters, CI, and build
    tooling is governance-floor doctrine, not a plan decision (see Siblings).
  ```

- **Kept deliberately:** the carve-out's substance — "established, never hand-rolled" for
  linters, CI, and build tooling is **governance-floor doctrine** and therefore outside this
  discipline's remit, with the Siblings cross-pointer intact. The two-homes/no-merge relationship
  with `authoring-constitution/references/STACK-TOOLING.md` is untouched.
- **Budget:** unbudgeted (hard-cap-only). Body 7,390 → **7,407**; description unchanged at 610,
  inside the 1,536 cap.
- **Consumers assessed:** the Siblings line this bullet points at (re-keyed in the main pass,
  entry below) — pointer and target now use the same vocabulary.

## [v0.91.0] `description:` trigger re-keyed — fires at design-phase AND build-time-decomposition decisions — plan-stage retirement D1 (d)

- **Disposition:** superseded → the same trigger firing at a design-phase decision or a
  build-time decomposition decision in a commodity category, with the never-builder-decided
  rule carried in the field.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D1 carry-overs — "adopt-first
  re-homed to the design phase and build-time decomposition (gated per mechanics d)" — and
  mechanic (d): "A commodity-category adopt-first ruling or an IP-XXX provisioning call is never
  builder-decided: it halts the cycle to the user's checkpoint, where
  `mochiko:patterns-adopt-first`'s constraint-challenge mechanism keeps its firing site").
- **Content (superseded text, verbatim):**

  ```
  This skill MUST be invoked at a plan decision in a commodity category (storage, locking, serialization, queueing, caching, auth, search) — the alternatives name a real off-the-shelf candidate or state none exists; custom wins only in writing against it. SHOULD also invoke on
  ```

- **Budget:** description-class edit, measured with the canonical snippet
  (`.mochiko/memory/primitive-cost-budgets.md`): **497 → 610 chars**. The skill is deliberately
  unbudgeted (hard-cap-only, ≤1,536 delivery cap) — well inside it. The +113 buys the second
  firing site and the never-builder-decided gate, both ruled obligations.
- **Kept deliberately:** the seven presumptive commodity categories verbatim, the
  candidate-or-none-exists disclosure floor, the custom-wins-only-in-writing rule, all five
  SHOULD trigger phrases (unchanged so existing routing keeps firing), the
  in-process/self-hostable scope bound with its SaaS→IP-XXX carve-out, and the
  governs-CHANGING-the-stack / `analysis-codebase`-describes-it boundary.
- **Consumers assessed:** the router row (re-keyed same wave),
  `mochiko:patterns-plan-minimalism` rung 3 (cites adoption as a way something already exists —
  unchanged), `mochiko:review-plan-artifacts`'s adopt-first disclosure lens (BLOCKING at
  conformance strength — unchanged), `mochiko:review-feasibility` hunt class 7 (unchanged).

## [v0.91.0] Body: the "not at build time" carve-out superseded by the gated build-time firing site — plan-stage retirement D1 (d)

- **Disposition:** superseded → a narrowed carve-out (no reopening a mechanism the design phase
  already ruled) plus a new **build-time gate** section carrying mechanic (d)'s obligation.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D1 mechanic (d)).
- **Why this edit exceeded the re-point brief, recorded so the audit reads it as deliberate:**
  the description re-key alone would have left the body asserting that this discipline does
  **not** fire at build time while the description asserted it does — a direct self-contradiction
  inside one primitive. The wave lead ruled the body edit in scope on that ground before it was
  made.
- **Content (superseded fragments, verbatim):**

  1. Title: `# Adopt First — Build vs Off-the-Shelf at Plan Time`
  2. Overview firing sentence:
     `ourselves?** It fires at plan time, where whole mechanisms are still on the table, and it binds`
  3. Scope bound: `**Scope bound.** Plan seats own **in-process libraries and self-hostable components**.`
  4. When NOT to Use:

     ```
     - **Build time** — cards already carry the plan's commitment; the code ladder
       (`mochiko:patterns-code-minimalism`) shapes code, it does not reopen the mechanism.
     ```
  5. Constraint-collision opening: `A plan seat never silently overrides a ratified upstream constraint.`
  6. Constraint-collision close: `Only the colliding decision pauses — the plan proceeds elsewhere.`
  7. Siblings close: `home carries tooling defaults, this skill the plan-time decision — no merge, cross-pointers`

- **Budget:** the skill is unbudgeted (hard-cap-only). Body 6,493 → 7,390 chars (+897), of which
  the new build-time-gate section is ~590 and the rest is the re-keys above. No budget
  obligation; the description cap is the only bar and it holds (610 ≤ 1,536).
- **Kept deliberately:** the code ladder's boundary survives in full — a mechanism the design
  phase **already ruled** is still closed to reopening at build time, and
  `mochiko:patterns-code-minimalism` still shapes code without reopening mechanisms. What
  changed is only the *unruled* case, which mechanic (d) routes to the user rather than to the
  builder. Also untouched: the two-part obligation, the two-sided limb with its BE-DEP
  cross-read, the external-claim disclosure line, the retrofit-cost gate, the three-part
  constraint-challenge finding and its route-to-the-user rule (mechanic (d) explicitly preserves
  this firing site), the Who-grades-what table, and the whole Quality Checklist.
- **Consumers assessed:** `implement.md` (P1's rewrite carries the build-time decision gating and
  the `baseline-delta.md` path this section names), the router row (re-keyed same wave),
  `mochiko:executing-tdd-cycle` (build-time decomposition — the halt obligation lands on the
  builder's run; flagged to the wave lead as a coherence read for the audit, since that skill is
  outside this seat's writable scope).
