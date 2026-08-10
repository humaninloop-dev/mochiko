---
name: authoring-prototype
description: This skill MUST be invoked when authoring a clickable low-fidelity UX prototype for a feature specification — the static HTML app under `.mochiko/specs/<feature>/prototype/` and the spec's Screens & Flows section (SCR-XXX, FLOW-XXX). SHOULD also invoke on 'mock the UX', 'clickable prototype', 'screens and flows', 'SCR-XXX', or 'FLOW-XXX'. Boundary: authors the PROTOTYPE — NOT the user stories it renders (mochiko:authoring-user-stories), NOT production UI code; never grades its own output.
---

# Authoring a Clickable Low-Fi Prototype

**Violating the letter of the rules is violating the spirit of the rules.**

## Overview

A text spec cannot show what the experience will be — surprises surface at build time, when they
are expensive. A **clickable low-fidelity prototype** fixes this: static HTML screens, wired with
real links and buttons, that a user clicks through while the stories are still wet. The output is
two coupled artifacts:

1. **The prototype app** — `.mochiko/specs/<feature>/prototype/`: plain static HTML/CSS (+ minimal
   inline JS for navigation only), servable with `bun` (e.g. `bunx serve prototype/`) and — the
   degrade path — openable directly from the filesystem with no server at all. No build step, no
   framework, no install *required* to view (serving is convenience, never a prerequisite).
2. **The Screens & Flows section of `spec.md`** — the manifest, in the shape
   [`spec-template.md`](../../templates/spec-template.md) defines: `SCR-XXX` rows (screen, purpose,
   data shown, FEAT tag) and `FLOW-XXX` rows (click-path steps, the story acceptance scenario each
   keys to, FEAT tag). The manifest is the contract surface downstream stages trace to; the HTML is
   its clickable rendering. IDs per the deliverable envelope
   ([`artifact-format.md`](../../templates/artifact-format.md)) — sequential, three-digit padded,
   cited never re-quoted.

**Authority split — binding flows, advisory pixels.** What screens exist, what each shows, and
what the user can do (the manifest) is binding: downstream design must serve every screen's data
and route every action. How it looks is deliberately rough and advisory: layout, styling, and
copy may all improve at build time without ceremony.

## When NOT to Use

- **The feature has no UX surface** — the intent ruling says not UX-bearing; the spec carries the
  waiver line instead. Never manufacture screens for an API or batch job.
- **Authoring the stories themselves** — `mochiko:authoring-user-stories`; the prototype renders
  stories, never invents or rewrites them
- **Grading the prototype** — graded with the spec by `mochiko:review-specifications`, run by an
  independent reviewer, never the author
- **Production UI work** — the prototype is a specification artifact; its code is throwaway by
  design and never migrates into the product

## The invariants (hard rules)

1. **Skeleton first.** Before any story screen: a nav frame — the app shell, navigation structure,
   and route stubs for every anticipated screen. Story screens fill into a stable frame; this is
   what keeps later stories from thrashing earlier screens.
2. **Lockstep, story by story.** Screens are authored with their story, as one unit of thought —
   never batched after all stories are drafted. Each story's screens land while that story is
   under discussion.
3. **Every flow keys to a scenario.** Each `FLOW-XXX` names the story acceptance scenario it
   renders (Given/When/Then → the click path that walks it). A flow keyed to nothing is scope
   invention; a P1 scenario with no flow is a manifest gap.
4. **Manifest ↔ HTML agreement.** Every `SCR-XXX` is a reachable page; every `FLOW-XXX` is
   clickable end-to-end in the served app. Drift between the tables and the HTML is a defect,
   mechanically checkable by walking the app.
5. **Low-fi discipline.** Grey boxes, system fonts, placeholder data with honest *shape* (realistic
   fields, realistic cardinality — five rows, not one). No polish, no animation, no pixel
   perfection — over-fidelity invites sign-off on looks the build won't honor, and makes the
   advisory half read as binding.
6. **Design system honored when one exists.** If the project has a design system or component
   library, use its tokens/primitives (colors, spacing, component names) so screens read as the
   product's family — at low fidelity, not faithful reproduction. No design system → neutral
   grey-box defaults; never invent a new visual language.
7. **FEAT tags carried — a re-tag pass at derivation.** Tags cannot exist during lockstep
   authoring: feature derivation runs after stories, so FEAT tags land as a **re-tag pass over
   the SCR/FLOW manifest** once derivation completes. Every row then carries the FEAT tag of the
   feature its story homes to; screens outside the current selection stay present but visibly
   greyed **coming-soon** — the app stays a coherent whole, not a stub maze.
8. **Rejected stories stay visible.** A filter-rejected story's screens are kept, greyed with
   the same coming-soon grammar, and marked **rejected** with a pointer to the rejection recorded
   in the story file — never silently deleted; the walkable record of what was considered and
   declined survives.

## Structure

```
.mochiko/specs/<feature>/prototype/
├── index.html          # entry — nav frame, links to every screen
├── screens/
│   ├── scr-001-<slug>.html
│   └── scr-002-<slug>.html
├── assets/             # shared css, placeholder images (optional)
└── README.md           # one-pager: how to serve (bun), degrade path, manifest pointer
```

- One file per screen, named by its `SCR-XXX` id. Navigation is plain `<a href>`/`<form action>` —
  a flow is walkable by clicking, no JS state machine.
- Coming-soon screens: the real page at reduced opacity with a banner, or a stub page carrying the
  FEAT tag (rejected screens: the rejection mark and pointer) — either way reachable, so
  navigation never dead-ends.

## Quality checklist

Before handing off:

- [ ] Skeleton nav frame exists; every screen reachable from `index.html`
- [ ] Every SCR-XXX row has a page; every page has a row (no drift, either direction)
- [ ] Every FLOW-XXX clickable end-to-end; each keyed to a named story acceptance scenario
- [ ] Every P1 story scenario has a flow
- [ ] Placeholder data has honest shape (realistic fields and cardinality)
- [ ] Design system tokens/components used where one exists; noted in README
- [ ] FEAT tags on every row after the re-tag pass; out-of-selection screens greyed, reachable; rejected stories' screens greyed, marked, pointed at the recorded rejection
- [ ] Serves with bun AND opens file:// with no server (degrade path)
- [ ] No build step, no framework, no dependency install required to view
- [ ] Stories, requirements, criteria untouched — the prototype renders them, never edits them

## Red Flags — STOP

- "I'll build all the screens once the stories settle" — lockstep is the point; batching re-creates
  the late-surprise problem the prototype exists to kill
- "This looks rough, let me polish it" — over-fidelity makes pixels read as binding; rough is a
  feature
- "The story doesn't cover this, but the screen obviously needs it" — scope invention; surface it
  as a story finding, never silently render it
- "I'll use React, it's faster for me" — the reader's cost is the constraint: static HTML, no
  toolchain
- "The flow works, I don't need to key it to a scenario" — an unkeyed flow is untraceable
  downstream; the key is the contract
- "Skip the greyed screens, they're not in this selection" — dead-end navigation breaks the
  clickable whole; coming-soon is cheap
- "The story was rejected, delete its screens" — rejected screens stay, greyed and marked with
  the rejection pointer; deletion erases the record of what was declined

## Common Rationalizations

| Excuse | Reality |
|--------|---------|
| "Low-fi means low effort — one row of fake data is fine" | Dishonest data shape hides layout and flow problems. Realistic cardinality is exactly what low-fi must get right. |
| "The design system slows me down; grey boxes everywhere" | Where a system exists, its absence in the mock *is* a surprise deferred to build. Tokens at low fidelity, not reproduction. |
| "Manifest and HTML say the same thing twice" | The manifest is what downstream traces to; the HTML is what humans click. Two consumers, two surfaces — agreement is the invariant. |
| "Pixels will change anyway, so flows can change too" | The split is ruled: flows/data/actions binding, pixels advisory. A flow change is a spec amendment, not a build-time whim. |
| "This prototype is good enough to ship as the real UI" | It is a specification artifact — unreviewed, untested, throwaway by design. It never migrates. |

## Related

- [`spec-template.md`](../../templates/spec-template.md) — owns the Screens & Flows section shape this skill fills
- [`artifact-format.md`](../../templates/artifact-format.md) — the deliverable envelope (ID grammar, citation rules)
- `mochiko:review-specifications` — grades the prototype with the spec (independent reviewer, never the author)
- `mochiko:authoring-user-stories` — upstream: the stories and acceptance scenarios the flows render
- `mochiko:authoring-feature-map` — the feature derivation whose FEAT tags the re-tag pass carries onto the manifest (single source of the map machinery)
