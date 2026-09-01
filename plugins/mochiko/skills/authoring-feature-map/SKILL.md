---
name: authoring-feature-map
description: This skill MUST be invoked when deriving or updating the repo-level feature map — the FEATURES.md index of durable capabilities plus per-capability FEAT-XXX entry files carrying transient work rows — during a specify or /mochiko:feature touch. SHOULD also invoke on 'feature map', 'FEATURES.md', 'FEAT-XXX', 'capability', 'work row', 'feature derivation', 'extend beats mint', or 'map delta'. Boundary: authors the MAP — NOT user stories (mochiko:authoring-user-stories), NOT architecture, NOT selection; mint/extend discipline lives in mochiko:patterns-map-minimalism. Never grades its own output.
---

# Authoring the Feature Map

The feature map is the **broad view of the whole system expressed as capabilities** — the
primary capability lens on the product, the way the architecture store is the component
view; together the two are the central source of truth. Capability delivery and spec
delivery are independent axes: one spec can surface several capabilities and build only a
subset of their rows. The map is durable; specs are delivery events. This skill carries
the authoring craft — the derivation flow (frame at intent, stories authored inside it,
filter and selection after), entry authoring, and the write mechanics.

## Rules — load the schema first

Your first action at invoke, before any derivation or map write: **Read `schema.yaml`
(this skill's own directory) and `../../schemas/skill-authoring-common.yaml` raw, in
full, in the same first action.** The schema is the source of truth for this skill's
binding rules; this body carries identity and teaching only. Its rules are nested in six
sections, each addressable by its section ID: `authoring-feature-map.sec.independence`
(who grades the produced artifacts) · `authoring-feature-map.sec.scope` (jurisdiction and
routing) · `authoring-feature-map.sec.inputs` (read duties before deriving) ·
`authoring-feature-map.sec.artifact` (the map's binding grammar, invariants, and write
mechanics) · `authoring-feature-map.sec.output` (the acceptance batch and the selection
card) · `authoring-feature-map.sec.reserved` (decisions reserved to the user).

Read the rule grammar along with the rules: a rule's `kind:` names what it is, and an
absent `kind:` reads `constraint`. Where a rule carries `extends: authoring-common.<slug>`,
the stub inherits `text` / `labels` / `pointer` only from `skill-authoring-common.yaml` —
`class` and `kind` are always this schema's own, and the stub's `authoring-feature-map.*`
ID stays the citable ID. `${var}` placeholders substitute from this schema's `vars:` at
read time. Labels come from `../../schemas/skill-labels.yaml`. A `pointer:` rule binds you
to that file's or skill's content, referenced never restated.

The schema carries **the 16 rules of `class: floor`**. State the floor count back before
the first procedural step; a skipped or partial schema read is a halt-and-surface, never a
silent continue.

## Vocabulary — capability vs the units around it

| Term | Level | What it is | Owner |
|------|-------|------------|-------|
| **Capability** (a *feature*) | Product | The durable thing the product does, in its own language; permanent, honest extents; the only entry called a feature | **this skill** (map entry) |
| **Work row** | Delivery increment | A transient row under a capability — one built increment, story-shaped allowed; `pending` (cut, undelivered) or `live` (in a run); folds into the capability's extent at its landing and vanishes | **this skill** (map entry) |
| **User story** | Spec | User value in user language; informs capabilities, never defines them | `mochiko:authoring-user-stories` |
| **Cycle** | Implementation, within one capability-batch run | A test-first increment delivering one observable behavior | `mochiko:patterns-vertical-tdd` (downstream) |

A **capability-batch** is the pipeline unit: each capability with selected work rows gets one implement run covering exactly those rows.

## Common Rationalizations

| Excuse | Reality |
|--------|---------|
| "Rejecting a story feels like losing requirements" | The rejection is recorded with its why — nothing is lost. Homing every story inflates the map until it stops describing the system. |
| "The map is right here, writing one entry early can't hurt" | A rejected spec must leave the truth layer clean. One early write breaks that guarantee for every future reader. |
| "Status on the story file too — easier to read" | Two status homes drift into two sources of truth. Stories derive status through their FEAT-ID; that is the design. |
| "The row will obviously fold, no need to name its run" | An unnamed row is unauditable; a work row whose spec or lane run died is invisible rot. The grammar exists to make both checkable. |
| "First capability has no journey, so I'll mint a core feature" | A pseudo-capability poisons the map permanently to save one run's ordering. Carry the core as far as a real capability's extent honestly reaches. |
| "A filled-out stub saves derivation time later" | A stub is name + hook only. Extent and rows are derivation's to fill — a pre-filled stub fakes ratification and anchors the deriver on unratified text. |
| "Defects and refactors belong on the map — they're work on features" | The map states what the product does, not what needs fixing. Defects, tooling, and process live in `BACKLOG.md`; only extent-growth ideas ride the map. |

## Related

- `features-index` / `feature-entry` schemas — own the repo-root `FEATURES.md` index shape and the per-capability entry shape this skill fills (bindings in the schema's artifact section)
- `mochiko:patterns-map-minimalism` — the derivation discipline this skill applies
- `mochiko:review-specifications` — grades spec + stories + feature derivation + map delta in one pass
- `mochiko:authoring-user-stories` — upstream: the story quality the derivation reads
- `mochiko:patterns-vertical-tdd` — downstream: cuts one capability-batch run's scope into cycle cards
- `mochiko:authoring-architecture-store` — the peer view: the topology that realizes capabilities; the entry's architecture link points there
