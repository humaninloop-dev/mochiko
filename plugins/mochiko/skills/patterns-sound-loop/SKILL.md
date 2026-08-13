---
name: patterns-sound-loop
description: This skill MUST be invoked before a judgment-authored write to a governing surface — capability map, product baselines, specs, `ARCHITECTURE.md`, governance, plugin primitives, product code — running the floor: a seat produces on a lead-approved plan (never the lead), a non-author seat reviews, the user rules. No size gate; desk delta cards take the review leg. SHOULD also invoke on 'sound loop', 'ritual floor', or 'seat wiring'. Single source of the floor; fourth sibling of the minimalism trio.
---

# Sound Loop — The Ritual Floor

**The entry door never lowers the review.**

## Overview

The floor is kind-keyed and library-wide: whenever a judgment-authored artifact is about to
land on a governing surface — any session, any command, whatever door the work entered
through — three rituals become non-waivable: a seat produces it on a lead-approved plan, a
non-author seat reviews it, and the user rules. When the trigger does not fire, the lead's
inline freedom stands as chartered. Transport stays neutral — a seat may be a teammate or a
subagent, the lead's per-seat call; what dies above the floor is the lead absorbing the seat.

## When NOT to Use

- **Trigger not fired** — either part of the test false: lead-inline freedom stands as
  chartered; the floor is not a blanket dispatch mandate.
- **Above the user** — the floor never adds a machine gate over a ruling reserved to the
  user; it works beneath the gate only.
- **Sizing questions** — whether an artifact should exist or how small it should be belongs
  to the three minimalism siblings, another axis.

## The trigger — two parts, no size threshold

The floor fires when **both** are true of the output about to be produced:

1. **Judgment-authored** — producing it required real judgment (the exemptions below never
   trip).
2. **Governing surface** — downstream work will read it as truth: a member of the table
   below.

Both true → the three legs are obligatory. Either false → lead-inline freedom stands. There
is **no size threshold**: a small judgment-authored write on a governing surface trips the
floor as surely as a 10-file rewrite — magnitude never gates it.

## Governing surfaces — per-member regime

| Governing surface | Existing regime | Net-new obligation |
|---|---|---|
| Plugin primitives (`plugins/mochiko/`) | primitive-edits ceremony (strip → record → author≠grader audit) | none — satisfied by construction |
| Product code | `/mochiko:implement` seats | none — by construction |
| Specs (`.mochiko/specs/`) | `/mochiko:specify` loop | none — by construction |
| Governance surfaces | `/mochiko:setup` loop | none — by construction |
| Capability map (`FEATURES.md` + entries) | specify-side derivation graded; desk-side writes carried no loop | full floor — net-new bite |
| Product baselines (`.mochiko/product/`) | no loop outside pipeline runs | full floor — net-new bite |
| `ARCHITECTURE.md` folds outside landings | landing-time diff only (`mochiko:authoring-architecture`) | full floor — net-new bite |

## The three legs

1. **Production sits with a seat, never the lead** — the producing seat plans first; the
   lead gives feedback, approves the plan, and distributes work to seats.
2. **Independent review** — the produced artifact is graded by a non-author seat before the
   user's gate; the user's ruling alone never substitutes for the review leg.
3. **The user gate stays** — rulings reserved to the user remain theirs; the floor adds
   review beneath the gate, never a machine gate above it.

## Exemptions — and the one that does not exist

Only three kinds of write never trip: **mechanical execution of an existing ruling** ·
**transcription of user decisions** · **fix-on-sight integrity repairs** (typos, status
agreement, dead pointers).

**There is no delta-card exemption.** Every desk-authored delta card — bug and improvement
alike — takes the review leg at the desk before dispatch. The review leg only: card
authorship stays with the desk.

## Default seat wiring

| Work class | Produces | Reviews |
|---|---|---|
| Map work (capability map) | `product-manager` | `devils-advocate` |
| Architecture / baseline touches | `principal-architect` | `tech-lead` |
| Desk delta cards | desk-authored (review leg only) | `devils-advocate` |
| Specs · product code | existing command wiring — unchanged | — |

Map review runs **spec-less**: capability tests per `mochiko:patterns-map-minimalism`,
entry/delta legality per `mochiko:authoring-feature-map`, derivation honesty against
whatever stories exist — no spec required. The lead may swap personas for cause — disclosed
at close.

## Out-of-remit hosting — the door moves, the ritual never drops

When the user explicitly asks a command to host work outside its key responsibilities, it
adapts rather than refuses: it **names the boundary crossing**, then serves the demand with
the home command's rituals imported — the three legs at minimum. Adaptation moves the door,
never lowers the ritual.

## Disclosure and honesty

Every visit/run close report carries one line, pinned grammar:

`floor: tripped|clear · seats: <who produced> / <who reviewed>`

When clear: `floor: clear` alone — no seats segment, since no seats existed to name. That is
the honesty rule itself: **a report may only name a seat that actually existed; otherwise
the lead speaks as itself.** The known under-classification risk — calling judgment work
"mechanical" to stay inline — is what this line makes auditable, visit by visit.

## Sibling

`mochiko:patterns-plan-minimalism`, `mochiko:patterns-code-minimalism`, and
`mochiko:patterns-map-minimalism` — same family, different axis: the three ladders size the
artifact, the floor governs who produces and who reviews.

## Quality Checklist

- [ ] Trigger evaluated — both parts, before the write, no size reasoning
- [ ] Tripped → all three legs ran: seat produced on an approved plan, non-author seat reviewed, user ruled
- [ ] Exemption claims honest — no judgment work relabeled "mechanical"
- [ ] Every desk delta card reviewed before dispatch
- [ ] Wiring defaults used, or the swap disclosed with cause
- [ ] Close report carries the disclosure line in pinned grammar
- [ ] Only seats that actually existed are named
