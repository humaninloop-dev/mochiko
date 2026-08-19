---
name: testing-gap-finding
description: This skill MUST be invoked when running the final-validation gap-finding pass — the blind, spec-derived hunt for promised behavior the builder and the test author both missed — or when authoring or folding a feature's durable gate set `.mochiko/features/FEAT-XXX/gates.md`. SHOULD also invoke on 'gap-finding', 'black-box exploration', 'blind explorer', 'exploratory probing', or 'mutation lens'. Owns the blindness fence, two-message dispatch, probe kit, mutation lens, finding-kind split, and fold-back. Selection-scope and epic runs only. Boundary: deterministic `**TEST:**` execution is mochiko:testing-end-user; the `**TEST:**` grammar is owned by mochiko:patterns-vertical-tdd — consume, never redefine.
---

# Gap Finding — The Blind Exploratory Pass

**Expectations before sight — the diff between promised behavior and probed behavior is the hunt.**

## Overview

The deterministic layer answers *did the declared asserts hold*. It cannot answer *what did
nobody declare*. This pass is the discovery layer: one deep exploratory hunt at final
validation, by a seat that derives its own expected behaviors from the promise layer before
touching the running system. It finds what the builder and the test author both missed —
unreachable by any re-run of their own cases.

**Boundary.** Executing a declared `**TEST:**` case belongs to `mochiko:testing-end-user`. The
`**TEST:**` construct is owned by `mochiko:patterns-vertical-tdd` — consumed here when folding
findings back, never redefined. This skill owns the pass: fence, derivation, probes, lens,
finding split, fold.

## When NOT to Use

- **Per-cycle verification** — deterministic-only; this pass runs once, at the end.
- **Delta-scope and product-lane runs** — no spec layer; see the scope carve.
- **Re-running declared `**TEST:**` cases**, or authoring unit tests — not this layer.
- **Accessibility probing** — declined; the a11y floor stays a build-time standard.
- **Property-based harnesses and metamorphic relations** — declined open threads.

## Run scope and placement

One pass, at **final validation**, over the whole built feature — full surface visible, seams
live against real delivered sides, one cost per run. Probing runs against real infrastructure,
never mocks.

- **Selection-scope runs** — the pass runs.
- **Epic runs** — **once, over the union of member territories**, at the epic's single final
  validation.
- **Delta-scope and product-lane runs** — **skipped**, and the final-validation report **states
  the skip explicitly**. A silent no-op is a defect: their sole expectation source is the card,
  which the fence forbids.

## The blindness fence

The explorer is blind to code and cards. Admissible inputs are an **explicit inclusion list**,
not a layer label: `spec.md` (FR-XXX, SC-XXX, stories, declared edge cases) · the feature's
`requirements.md` · Screens & Flows (SCR-XXX, FLOW-XXX) · `data-model.md` (entities, state
machines, DS-XXX sensitivity) · `contracts/` (`api.yaml` and any sibling contract documents) ·
`nfrs.md`. All define externally-observable promised behavior, so the pass stays black-box.

**Excluded, structurally:** the code · the cycle cards (`tasks.md`) · the `**TEST:**` cases ·
cycle reports · the builder's tests.

**Delegated reads inherit the inclusion list.** A locate, enumeration, or targeted read over
the code, the cycle cards, or the `**TEST:**` cases is outside the fence and is never
delegated — a subagent's return carries code sight back to the blind seat exactly as a direct
read would.

**Two-message dispatch** — the fence is held by dispatch order, not trust:

1. **Message 1** carries inclusion-list references **only** — never a card, code, or
   `**TEST:**` path. The seat reads them and **states its derived expectations**.
2. **Message 2** opens probing, once those expectations are on the record.

Sight of the declared cases anchors the hunt on existing coverage: the explorer probes *around*
what is already asserted instead of deriving independently. That is what the ordering prevents.

**Seat:** `mochiko:devils-advocate` — the adversarial gap-hunt craft. Persona carries the
judgment, this skill carries the procedure.

## Expectation derivation

Before probing, enumerate expected behaviors as a **numbered list** — the numbering makes the
done condition's count auditable. Five families:

1. **Happy path** — what the promise layer says the system does when used as intended.
2. **Negative and edge** — invalid, boundary, malformed, misuse; the declared edge cases and
   the ones the spec implies but never names.
3. **Abuse** — authz bypass (cross-user resource reach), privilege escalation, injection-class
   inputs, session and replay misuse. Derived from the spec's roles plus the DS-XXX classes:
   Confidential and Restricted attributes name what must not leak.
4. **Runtime NFR** — each `nfrs.md` numeric target (p95, availability, limits) as a measurable
   expectation against the built system.
5. **Observability** — key flows leave logs and metrics; error paths produce actionable
   diagnostics. **Advisory-only findings, always**: by construction no clause exists to cite.

## The probe kit

Seven families, all derivable from the inclusion list, all inside the fence — one seat, one
charter. Runs at **both depth levels**; breadth is invariant.

| Family | Probes |
|---|---|
| Adversarial inputs | invalid, boundary, malformed, misuse |
| Illegal state-transition walks | transitions the `data-model.md` state machines forbid |
| Contract probes | wrong types, missing fields, status-code and pagination edges |
| Concurrency / idempotency | parallel submits, replays, double-fire |
| Security / abuse | authz bypass, escalation, injection, session/replay |
| NFR measurement | each numeric target measured against the built system |
| Observability | logs, metrics, diagnostic quality on error paths |

## The mutation lens

A grey-box lens: a mutation tool mutates the built code and runs the builder's suite;
surviving mutants are measured holes in that suite. It needs code sight, so it runs
**alongside, never inside, the blind explorer** — on the existing verification seat, which
already holds code sight.

- **HIGH depth only**, **this feature's diff only** (changed-code mutation,
  `cargo-mutants --in-diff` class), **timeboxed**.
- **Tool class per stack:** cargo-mutants (Rust) · mutmut (Python) · Stryker (JS/TS and ports).
- **Tool absent = lens skipped AND noted** — never silent. Routine on the mobile and desktop
  shelves, where mutation tooling is sparse; note it there too.
- **Flaky suite detected during the run = skipped, skip noted.**
- **Surviving mutants are beyond-spec advisory findings** — never blocking.

The tool is an advisory post-hoc checker read as an optional exit-code signal: it never gates
progress, never dispatches agents, never holds judgment this skill owns.

## Findings — split by kind, never by severity

- **Spec-violation — blocking, final validation fails.** Spec-required behavior demonstrably
  broken, with **evidence captured** and the **spec clause cited**. A broken `nfrs.md` numeric
  target qualifies.
- **Beyond-spec — advisory to the final checkpoint.** Robustness gap, undeclared edge behavior,
  surviving mutant, observability hole. The user rules fix-now / backlog / accept.

**Adjudication.** The finder **proposes** the kind; the **lead confirms** the blocking
classification at the checkpoint verdict against the cited clause; a **disputed kind defaults
advisory** and goes to the user, who rules. The finder never gates alone.

**Rework bound.** Gap-rework carries a **whole-run bound, default 2 rounds, redeclarable only
at run open**. A finding localizing to one cycle's territory charges that cycle's remaining
attempts instead. Bound exhaustion or a no-progress round halts the run; the disposition is
**reserved to the user**.

**Out-of-territory routing.** A gap in a previously delivered feature's territory — via the
accumulated gates or a seam — is **not this run's rework**. It routes to a `/mochiko:feature`
delta card, cited in the report.

## Done condition and disclosure

Complete when **every derived expectation has been probed, or explicitly marked unprobeable
with a reason, within the charter's timebox**. The **final-validation report** discloses the
expectation count, the probed count, each unprobeable expectation with its reason, and the
findings by kind.

**Zero findings is a clean pass** — no never-zero rule, no quota. The disclosure is the honesty
mechanism, not the finding tally.

## Fold-back — the durable gate set

Every gap the user rules **fix-now or backlog** is authored as a `**TEST:**` case, in the
grammar owned by `mochiko:patterns-vertical-tdd`, into the feature's durable gate set.
Authored by the QA craft (`mochiko:qa-engineer`), never the exploratory seat. Findings
accepted **as-designed do not fold**.

**The artifact:** `.mochiko/features/FEAT-XXX/gates.md` — minted at first fold (or at plan
time, when the cards are authored) and **surviving graduation**: work rows vanish, `gates.md`
persists. It is the named read source of the "accumulated territory `**TEST:**` gates"; at a
later final validation that read is the **union of the territory features' `gates.md` plus
their cards' cases**.

```markdown
# FEAT-014 — Durable gates
<!-- Folded gap findings. Persist past graduation; read at every later final validation. -->

**TEST:** Rejects a session token replayed after logout
- **Setup** / **Action** / **Assert** / **Capture** per the TEST grammar
- Source: gap-finding pass, FEAT-014 final validation (spec-violation, SC-003)
```

## Anti-patterns

| Anti-pattern | Why it breaks the pass |
|---|---|
| Peeking at cards or code "to target the hunt" | Anchors on existing coverage; kills what the fence buys |
| Probing the happy path only | The declared gate covers it; discovery lives elsewhere |
| The finder ruling its own finding blocking | Gate boundaries are the lead's, the user's when disputed |
| Skipping the mutation lens silently | An unnoted skip reads as a clean lens |
| Padding findings to avoid a zero | Zero is clean; padding corrupts the disclosure |

## Quality Checklist

- [ ] Scope checked — selection, or epic once over the union; a delta/lane skip stated in the report
- [ ] Message 1 carried inclusion-list references only — no card, code, or `**TEST:**` path
- [ ] Delegated reads stayed inside the inclusion list — no locate over code, cards, or `**TEST:**` cases delegated
- [ ] Expectations numbered and stated **before** probing, across all five families
- [ ] Probe kit run at the run's depth level — breadth invariant at both
- [ ] Mutation lens run at HIGH depth, or skipped **and noted** (tool absent / flaky suite)
- [ ] Every finding carries a proposed kind; spec-violations cite a clause and carry evidence
- [ ] Blocking confirmed by the lead; disputes defaulted advisory and put to the user
- [ ] Out-of-territory gaps routed to a `/mochiko:feature` delta card and cited
- [ ] Every expectation probed or marked unprobeable with a reason, inside the timebox
- [ ] Report discloses expectation count, probed count, unprobeable reasons, findings by kind
- [ ] Fix-now / backlog rulings folded into `gates.md`; as-designed not folded
