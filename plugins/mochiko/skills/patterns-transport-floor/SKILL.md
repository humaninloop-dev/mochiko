---
name: patterns-transport-floor
description: This skill MUST be invoked before composing or running any multi-seat work — cross-seat or lead-relayed messaging, or any shared write surface — the transport floor against message races and write collisions. Each lane is non-waivable once fired. SHOULD also invoke on 'transport floor', 'message race', 'single writer', 'fan-in confirmation', or 'mesh hold'. Governs transport use, never the neutral transport choice; sibling of patterns-sound-loop.
---

# Transport Floor — Message Races and Write Collisions

**The message arrives; the work does not start until the lead opens it.**

## Overview

The sound-loop floor governs *who produces and who reviews*; this floor governs *how seats
talk and write while they do it*. It is kind-keyed on the transport axis: whenever a
multi-seat run carries messaging or a shared write surface, a set of legs becomes
non-waivable for the hazard class it fires against. Transport *choice* stays neutral — a seat
may be a teammate or a subagent, the lead's per-seat call (realignment D5). Transport *use* is
what this floor disciplines: message races (an order lands stale, a supersession arrives out
of sequence, an idle ping fires without its deliverable) and write collisions (two seats
overwrite one file, a grader reads a surface still moving).

## When NOT to Use

- **Solo run** — one actor, no messaging, no shared surface: nothing to race.
- **Single seat, no cross-seat messaging** — no trigger fires; chartered freedom stands.
- **Transport-choice questions** — teammate vs subagent is the neutral per-seat call
  (realignment D5); this floor governs *use*, never the choice.
- **Sizing questions** — whether an artifact should exist or how small it should be belongs to
  the three minimalism siblings, another axis.

## The trigger — two lanes, each non-waivable when it fires

- **Message legs (3, 4, 6, 7)** fire on **any multi-seat run with cross-seat or lead-relayed
  messaging** — shared writes or not. Review-pair and cross-examination exchanges are
  message-leg territory: a lead relaying a mid-flight ruling to a single recipient already
  trips them.
- **Topology legs (1, 2, 5)** fire when a **shared write surface** exists — two or more seats
  able to write one artifact.

When a lane fires, its legs bind: the trigger scopes the obligation, and an untriggered run
keeps its chartered freedom. Neither lane is waivable once fired — a lead cannot legally
depart the floor mid-crisis, which is exactly the shape a waivable floor would sanction.

## The seven legs

Each leg carries its lane; a leg binds only when its lane's trigger has fired.

1. **Composition steer** — *[topology, binding — D4]* when a lead is about to compose
   concurrent writers on one surface, only two shapes are in-floor: **worktree-isolated
   writers** (isolation makes the surface non-shared) or **one pen-holder seat** with the
   other seats routing deltas to it. Any other composition is out of floor — re-shape at
   composition time, never police with use-discipline a shape that need not exist.
2. **Single writer per surface per wave** — *[topology]* every artifact has exactly one seat
   holding the pen at any moment; pen hand-offs are explicit.
3. **Mesh hold** — *[message]* a received message is a hand-off, never a start signal; work
   starts on the lead's explicit open.
4. **Content-pinned supersession** — *[message]* an order quotes the exact text it lands, and
   a superseding order names what it voids; no order asserts disk state or relies on message
   sequence.
5. **Quiesce before cold grade** — *[topology]* a grader reads a declared-frozen state;
   writers hold until the verdict.
6. **No ritual sends; never re-send** — *[message]* idle-ping-with-queued-inbox is normal; a
   resend exists only as a supersession that names what it voids.
7. **Fan-in confirmation** — *[message]* the lead counts expected deliverables and confirms
   each **arrived** before treating a fan-out complete or converging; an idle signal without
   its deliverable means pull the output, never assume it.

## The platform floor — version and transport facts

- **Version floor ≥ v2.1.224.** Below it, `SendMessage` reported success when the write to a
  teammate's inbox had actually failed (fixed in the v2.1.224 changelog) — masked-failure
  sends. Agent-teams work assumes ≥ v2.1.224.
- **Teammate transport:** delivery is documented-automatic (send success = the mailbox write
  succeeded, ≥ v2.1.224); ordering is **undocumented** — the floor designs around it rather
  than trusting it. Doc anchor: the agent-teams best-practices ownership-split line — "Two
  teammates editing the same file leads to overwrites. Break the work so each teammate owns a
  different set of files" (code.claude.com/docs/en/agent-teams), which directly grounds legs
  1–2.
- **Cross-session messaging** is a different transport: neither delivery nor ordering is
  guaranteed. Cite it only when cross-session work is in scope.

## Sibling

`mochiko:patterns-sound-loop` — the ritual floor (who produces, who reviews) on the same
kind-keyed pattern, a different axis. That floor's neutrality line points here: the transport
*choice* stays neutral, transport *use* carries this floor.

## Quality Checklist

- [ ] Trigger lanes evaluated before the run — messaging present? shared write surface present?
- [ ] Message lane fired → legs 3, 4, 6, 7 held
- [ ] Topology lane fired → legs 1, 2, 5 held
- [ ] Concurrent writers composed only as worktree-isolated or single pen-holder (leg 1)
- [ ] Every superseding order quotes what it lands and names what it voids (leg 4)
- [ ] No re-sends except a void-naming supersession; idle-with-queued-inbox treated as normal (leg 6)
- [ ] Every expected deliverable confirmed arrived before convergence; idle-without-deliverable pulled (leg 7)
- [ ] Agent-teams run on ≥ v2.1.224
