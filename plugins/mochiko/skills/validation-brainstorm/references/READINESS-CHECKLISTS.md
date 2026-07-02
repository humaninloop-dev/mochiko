# Destination-Readiness Checklists

The four checklists behind the pressure-test's destination-readiness lens (`validation-brainstorm`
Branch 2, lens 3). The caller names one destination; run that checklist, citing evidence from the
synthesis for **every** item — an item without cited evidence is unchecked, and any unchecked item
blocks `ready`.

Each checklist ends with a **depth-downgrade rule**: the condition under which the work demands
more than this destination provides. A tripped rule is a **wrong-depth** finding (scope-type):
report it with the shallower destination recommended — never stretch items to make the deeper
stamp fit. This file is the single source of truth for what each stamp means; consumers of a
stamped artifact rely on exactly these items, no more.

---

## 1. Feature-description readiness

*The conclusion will seed requirements discovery — an authoring loop that elicits stories and
requirements from it. It must be a rich description, not a spec.*

- [ ] **Who** — the affected actors/users are named specifically (roles, not "users")
- [ ] **Problem** — the problem is stated as a felt situation with a cost, not as a solution
- [ ] **Value** — what improves, for whom, and how you'd notice it
- [ ] **Out of scope** — at least the tempting-but-excluded adjacent work is named
- [ ] **Success sketch** — a rough, observable "we'll know it worked when…"
- [ ] **Session decisions carried** — the digest's settled decisions appear, with `Contested` /
      `Deferred` marks preserved (the authoring loop must know what was deliberate)

**Depth-downgrade rule:** none — this is the shallowest destination. If even the triad cannot be
evidenced, the finding is `critical-gaps` on the synthesis itself, not wrong depth.

---

## 2. Specification-equivalence

*The conclusion will stand in for an authored specification: technical design starts from it
directly, with no requirements-authoring loop in between. It must fill the spec shape honestly.*

- [ ] **Spec shape filled** — overview, user stories, edge cases, requirements, success criteria
      all present; no placeholder tokens or empty sections
- [ ] **Stories prioritized and testable** — P1/P2/P3 with Given/When/Then acceptance scenarios;
      each story independently testable
- [ ] **Requirements precise** — FR-XXX form with RFC 2119 keywords (MUST/SHOULD/MAY),
      technology-agnostic
- [ ] **Success criteria measurable** — SC-XXX with observable thresholds, not "fast" or "easy"
- [ ] **Edge cases surfaced** — empty/error/limit/permission cases, not just the happy path
- [ ] **Provenance header present** — states how this artifact was produced and reviewed, so a
      reader can tell it did not pass through a requirements-authoring loop
- [ ] **No unresolved preference questions** — every open "should we" is either decided (with a
      confidence mark) or explicitly deferred with a stated default

**Depth-downgrade rule:** if requirements are still discovering themselves — actors unclear,
stories unstable round-over-round, "it depends" answers load-bearing — the work needs requirements
discovery. Recommend **feature-description** depth.

---

## 3. Task-derivability

*The conclusion will be sliced directly into implementation tasks, skipping technical design. Only
design-light work qualifies — the checklist is half readiness, half proving the skip is legitimate.*

- [ ] **Design-light proven** — the work demands **no new entities, no new API endpoints, no new
      external integrations, and no new NFR targets**; it recomposes what exists (refactor,
      migration, extraction, consolidation)
- [ ] **Work items sliceable** — the goal decomposes into increments that each deliver an
      independently observable outcome
- [ ] **Affected areas named** — the files/modules/systems touched are listed from the actual
      codebase, not guessed
- [ ] **Constraints stated** — what must not break: preserved interfaces, compatibility bounds,
      untouchable areas
- [ ] **Done-criteria per item** — each work item has an observable completion check, not "improve X"
- [ ] **Verification method stated** — how the overall result is proven against real
      infrastructure (commands, checks, exit criteria)

**Depth-downgrade rule:** any new entity, endpoint, external integration, or NFR target ⇒ the work
needs technical design first. Recommend **specification-equivalence** depth (or shallower).

---

## 4. Direct-execution readiness

*The conclusion will be handed straight to a bounded implement-and-verify pass — no task
breakdown. The strictest stamp: an engineer must be able to code against it tomorrow.*

- [ ] **Small and bounded** — one focused change-set; a single implement-and-verify pass can
      plausibly complete it (guidance: if it wants multiple cycles or parallel tracks, it is not
      direct-execution work)
- [ ] **Goal unambiguous** — a one-paragraph statement a stranger could implement without asking
      a preference question
- [ ] **Affected files named** — the specific files/areas to touch, verified to exist in the
      codebase
- [ ] **Constraints stated** — interfaces to preserve, patterns to follow, areas not to touch
- [ ] **Observable done-criteria** — the behavior change that proves completion, stated as
      something a verifier can execute and observe
- [ ] **Verification method executable** — the exact checks (tests to run/write, commands, quality
      gates) a verifier runs against real infrastructure
- [ ] **Zero open questions** — no `Deferred` or unresolved `Contested` marks load-bearing to the
      change

**Depth-downgrade rule:** multiple independent increments, or any unnamed affected area, or any
open preference question ⇒ recommend **task-derivability** depth (or shallower).
