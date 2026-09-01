---
name: authoring-prototype
description: This skill MUST be invoked when authoring a clickable low-fidelity UX prototype for a feature specification — the static HTML app under `.mochiko/specs/<feature>/prototype/` and the spec's Screens & Flows section (SCR-XXX, FLOW-XXX). SHOULD also invoke on 'mock the UX', 'clickable prototype', 'screens and flows', 'SCR-XXX', or 'FLOW-XXX'. Boundary: authors the PROTOTYPE — NOT the user stories it renders (mochiko:authoring-user-stories), NOT production UI code; never grades its own output.
---

# Authoring a Clickable Low-Fi Prototype

A text spec cannot show what the experience will be — surprises surface at build time,
when they are expensive. A **clickable low-fidelity prototype** fixes this: static HTML
screens, wired with real links and buttons, that a user clicks through while the stories
are still wet. Structure and flows are precise; pixels are deliberately rough.

## Rules — load the schema first

Your first action at invoke, before any screen or manifest row: **Read `schema.yaml`
(this skill's own directory) and `../../schemas/skill-authoring-common.yaml` raw, in
full, in the same first action.** The schema is the source of truth for this skill's
binding rules; this body carries identity and teaching only. Its rules are nested in six
sections, each addressable by its section ID: `authoring-prototype.sec.independence`
(who grades the produced prototype) · `authoring-prototype.sec.scope` (jurisdiction
lines) · `authoring-prototype.sec.inputs` (empty by design) ·
`authoring-prototype.sec.artifact` (the app, the manifest, the invariants) ·
`authoring-prototype.sec.output` (what surfaces upward) ·
`authoring-prototype.sec.reserved` (empty by design).

Read the rule grammar along with the rules: a rule's `kind:` names what it is, and an
absent `kind:` reads `constraint`; a rule carrying `when:` binds only where its terms
hold against the schema's declared `conditions:`, except that a `class: floor` rule is
always read and always delivered — `when:` gates when its obligation applies, never
whether it reaches you. Where a rule carries `extends: authoring-common.<slug>`, the
stub inherits `text` / `labels` / `pointer` only from `skill-authoring-common.yaml` —
`class` and `kind` are always this schema's own, and the stub's `authoring-prototype.*`
ID stays the citable ID. `${var}` placeholders substitute from this schema's `vars:` at
read time. Labels come from `../../schemas/skill-labels.yaml`. A `pointer:` rule binds
you to that file's or skill's content, referenced never restated.

The schema carries **the 4 rules of `class: floor`**. State the floor count back before
the first procedural step; a skipped or partial schema read is a halt-and-surface, never
a silent continue.

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

Coming-soon screens render as the real page at reduced opacity with a banner, or as a
stub page carrying the FEAT tag; rejected screens carry the rejection mark and pointer —
the greying grammar the schema's re-tag and rejected-story rules bind.

## Common Rationalizations

| Excuse | Reality |
|--------|---------|
| "Low-fi means low effort — one row of fake data is fine" | Dishonest data shape hides layout and flow problems. Realistic cardinality is exactly what low-fi must get right. |
| "The design system slows me down; grey boxes everywhere" | Where a system exists, its absence in the mock *is* a surprise deferred to build. Tokens at low fidelity, not reproduction. |
| "Manifest and HTML say the same thing twice" | The manifest is what downstream traces to; the HTML is what humans click. Two consumers, two surfaces — agreement is the invariant. |
| "Pixels will change anyway, so flows can change too" | The split is ruled: flows/data/actions binding, pixels advisory. A flow change is a spec amendment, not a build-time whim. |
| "This prototype is good enough to ship as the real UI" | It is a specification artifact — unreviewed, untested, throwaway by design. It never migrates. |

## Related

- `spec` schema — owns the Screens & Flows section shape this skill fills (binding in the schema's artifact section)
- [`artifact-format.md`](../../templates/artifact-format.md) — the deliverable envelope (ID grammar, citation rules)
- `mochiko:review-specifications` — grades the prototype with the spec
- `mochiko:authoring-user-stories` — upstream: the stories and acceptance scenarios the flows render
- `mochiko:authoring-feature-map` — the feature derivation whose FEAT tags the re-tag pass carries onto the manifest (single source of the map machinery)
