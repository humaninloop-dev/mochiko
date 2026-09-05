---
name: testing-gap-finding
description: This skill MUST be invoked when running the final-validation gap-finding pass — the blind, spec-derived hunt for promised behavior the builder and the test author both missed — or when authoring or folding a feature's durable gate set `.mochiko/features/FEAT-XXX/gates.md`. SHOULD also invoke on 'gap-finding', 'black-box exploration', 'blind explorer', 'exploratory probing', or 'mutation lens'. Owns the blindness fence, two-message dispatch, probe kit, mutation lens, finding-kind split, and fold-back. Selection-scope and epic runs only. Boundary: deterministic `**TEST:**` execution is mochiko:testing-end-user; the `**TEST:**` grammar is owned by mochiko:patterns-vertical-tdd — consume, never redefine.
allowed-tools: Bash(mochiko-cli *)
---

# Gap Finding — The Blind Exploratory Pass

**Expectations before sight — the diff between promised behavior and probed behavior is the hunt.**

## Overview

The deterministic layer answers *did the declared asserts hold*. It cannot answer *what did
nobody declare*. This pass is the discovery layer: one deep exploratory hunt at final
validation, by a seat that derives its own expected behaviors from the promise layer before
touching the running system. It finds what the builder and the test author both missed —
unreachable by any re-run of their own cases. This skill owns the pass: fence, derivation,
probes, lens, finding split, fold.

Sight of the declared cases anchors the hunt on existing coverage — the explorer probes
*around* what is already asserted instead of deriving independently. The fence and its
dispatch order exist to prevent exactly that.

## Rules — delivered by mochiko-cli

Your rules arrive below, rendered at fire by `mochiko-cli` from the migration log this plugin
carries — one block per section. Every block opens with a version-triple line
(`mochiko-cli rules testing-gap-finding · section <id> · binary <v> · grammar <g> · plugin <p>`) and
closes with an end line (`mochiko-cli rules end · testing-gap-finding · <id> · <N> rules`). **Proceed
only when every block carries both lines in that exact shape, from whichever channel delivered
it — this slot, or the plugin's dependency hook on a Skill-tool call.** Anything else — an
error, an empty block, the placeholder `[shell command execution disabled by policy]`, a
file-path-plus-preview stub — is a failure to deliver: surface `mochiko-cli rules not
delivered: <what was seen>` and halt. Never Read a schema file instead; there is no fallback.
The `legend` in the preamble block is the reading grammar; a `pointer:` binds you to that
file's or skill's procedure, referenced never restated.

!`mochiko-cli rules testing-gap-finding --section preamble --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules testing-gap-finding --section testing-gap-finding.sec.independence --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules testing-gap-finding --section testing-gap-finding.sec.scope --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules testing-gap-finding --section testing-gap-finding.sec.inputs --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules testing-gap-finding --section testing-gap-finding.sec.verdict --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules testing-gap-finding --section testing-gap-finding.sec.output --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules testing-gap-finding --section testing-gap-finding.sec.reserved --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`

Before the first procedural step, state back the floor count the preamble's `class: floor` pin
prints and the ids its `floors:` line lists; a blank or partial read-back is a skipped read —
halt and surface it.

## When NOT to Use

- **Accessibility probing** — declined; the a11y floor stays a build-time standard.
- **Property-based harnesses and metamorphic relations** — declined open threads.

## Expectation derivation

The derivation walk — five families:

1. **Happy path** — what the promise layer says the system does when used as intended.
2. **Negative and edge** — invalid, boundary, malformed, misuse; the declared edge cases and
   the ones the spec implies but never names.
3. **Abuse** — authz bypass (cross-user resource reach), privilege escalation, injection-class
   inputs, session and replay misuse. Derived from the spec's roles plus the DS-XXX classes:
   Confidential and Restricted attributes name what must not leak.
4. **Runtime NFR** — each `NFR-XXX` numeric target on the store's concern rows (p95,
   availability, limits) as a measurable expectation against the built system.
5. **Observability** — key flows leave logs and metrics; error paths produce actionable
   diagnostics.

## The probe kit

Seven families, all derivable from the inclusion list, all inside the fence — one seat, one
charter.

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
surviving mutants are measured holes in that suite. **Tool class per stack:** cargo-mutants
(Rust) · mutmut (Python) · Stryker (JS/TS and ports).

## Fold-back — the durable gate set

Folded findings land in the durable gate set — the grammar, authorship, and artifact rules
live in the schema; the artifact looks like this:

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
