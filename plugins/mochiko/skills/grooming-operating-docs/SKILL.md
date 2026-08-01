---
name: grooming-operating-docs
description: This skill MUST be invoked when a knowledge-management invariant cap or bound trips at a command boundary — a ROADMAP.md horizon cap (Now/Next/Later), the BACKLOG.md per-open-item size bound or open-item-count watch, a dead pointer, a status disagreement across the brainstorms index / record / decisions index, or a `[x]` item found in BACKLOG.md — restoring the operating docs to their shape contracts under fix-on-sight. SHOULD also invoke on "groom the operating docs", "merge backlog sections", "compress superseded decisions", "re-rank Now/Next/Later", or an "are these items already delivered?" delivery sweep over open BACKLOG.md items. Attached to already-firing command boundaries (brainstorm open/close, setup/amend, specify/plan/implement landings) — never dependent on the user remembering to groom. Resolves every cap, bound, and format from the project-pinned copy at `.mochiko/memory/knowledge-management.md`; no copy → nothing to groom. Compresses and moves, never deletes; the subtractive landing ritual itself is the commands' job, not this skill's.
---

# Grooming Operating Docs

## Overview

Restore the operating docs to their shape contracts after a cap or bound trips. The runtime
source for every invariant, cap, bound, and format is the **project-pinned copy** at
`.mochiko/memory/knowledge-management.md` — read it first; this skill carries the craft, never
the numbers.

## Procedure

1. **Read the pinned copy** — invariants, caps, bounds, and the last-groomed baselines (on the
   stamp lines of `ROADMAP.md` and `BACKLOG.md`).
2. **Fix the tripped invariant first**, then sweep the rest of the list once.
3. **Delivery sweep** *(on an "already delivered?" doubt, or a tripped open-item-count
   watch)* — verify each open item's **stated obligation** against the delivered state: the
   trail, `.mochiko/strips/`, the decisions layer, git history, and the primitives on disk.
   Adjacent work never counts; run-gated items stay open without run evidence. Per-item
   verdict — delivered / partial / open — with citations, every delivered claim
   independently re-verified before it is presented. Delivered → closure candidates the
   **user ratifies** (then the done-item move below); partial evidence and stale item text
   (wrong counts, dead-provenance claims, uncredited run evidence) fold into the item
   fix-on-sight.
4. **BACKLOG.md** — merge provenance-keyed sections into theme-keyed ones; any done item →
   one-line DONE + pointer, moved to the trail (append-only); over-bound entries compressed
   in place, resume-cold context preserved; dead provenance stamped
   `provenance: unrecoverable (<what it was>, removed <date>)`.
5. **ROADMAP.md** — enforce the horizon caps: overflow demotes (Now → Next → Later →
   BACKLOG); promotion into *Next* mints the required link; drop any *Now* item not pointing
   at live work (back to BACKLOG, never deleted).
6. **DECISIONS.md** — superseded rows compress to one line per decision, status preserved.
7. **Stamp** — update both last-groomed lines: date + baseline figures (open-item count,
   entry-size figure) the count watch reads next time.
8. **Expansion-heavy-surface watch** — check whether any operating doc's writes have become
   mechanical derivations from artifacts (`CHANGELOG.md` first, when adopted). A hit is the
   recorded re-open trigger for the report-writer ruling (`model-tiered-seats` D3): log it as
   a BACKLOG item for the user — never act on it here.

## Boundaries

- Compress and move, never delete — the trail and archives are append-only.
- Never edit session records or decision records; only indexes, views, and stamps.
- A judgment call the docs can't settle (which bet to drop, a contested demotion) goes to the
  user; everything mechanical is fix-on-sight.
