# A landed removal leaves a one-line stub — joint-build seam R9

**Date:** 2026-09-29
**Status:** ruled (user) 2026-09-29, "as recommended", during wave 2's plan round of the joint build
(branch `joint-hook-delta`); **built 2026-09-29 at plugin v0.116.0** (migrations 0025 and 0026)
**Driver:** the plan grader P4, on seat S4's wave-2 plan (its advisory A10). This is the ninth seam
ruling of the joint build; R1–R8 are in
[the seams record](2026-09-29-joint-hook-delta-build-seams.md), which is at its size bound.

## Context

The delta record's D2 marks a removal `removing (<key>)` and says the landing sweep flips every
`in-flight` / `modifying` / `removing` of the landed key to `built`. That fits a new or changed
entry. It does not fit a removal: nothing is left to be built. R8 (seams record) settled what a
removal reads before and after sign-off; nothing says what it becomes at landing.

## Decision

**R9 —** At landing a `removing (<key>)` entry keeps its heading and becomes
`**Lifecycle:** removed`, with no body and no key. Its id stays taken: the next-free-id read of delta
D3a counts stubs, so no later run mints the id again, and a citation of it still lands on the stub.
Every reader treats `removed` as absent. One form across the architecture store and every baseline,
as D2's I4 has it. It lands in the grammar rule, `lifecycle-statuses`, the store template's legend
and the landing sweep (migration 0025 and the landing rules seat S4 rewords).

## Rationale

Ids are cited from reports, commits and other entries. A deleted entry that held the high-water id
would be minted again by the next run's high-water read, and old citations would then name a
different entry. A two-line stub is the cheapest way to keep the id taken.

## Alternatives considered

- Delete the entry at landing, git history keeping the text (the id can be minted again when it was
  the high-water mark, and old citations then point at a different entry).
