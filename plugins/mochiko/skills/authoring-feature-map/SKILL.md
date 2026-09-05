---
name: authoring-feature-map
description: This skill MUST be invoked when deriving or updating the repo-level feature map — the FEATURES.md index of durable capabilities plus per-capability FEAT-XXX entry files carrying transient work rows — during a specify or /mochiko:feature touch. SHOULD also invoke on 'feature map', 'FEATURES.md', 'FEAT-XXX', 'capability', 'work row', 'feature derivation', 'extend beats mint', or 'map delta'. Boundary: authors the MAP — NOT user stories (mochiko:authoring-user-stories), NOT architecture, NOT selection; mint/extend discipline lives in mochiko:patterns-map-minimalism. Never grades its own output.
allowed-tools: Bash(mochiko-cli *)
---

# Authoring the Feature Map

The feature map is the **broad view of the whole system expressed as capabilities** — the
primary capability lens on the product, the way the architecture store is the component
view; together the two are the central source of truth. Capability delivery and spec
delivery are independent axes: one spec can surface several capabilities and build only a
subset of their rows. The map is durable; specs are delivery events. This skill carries
the authoring craft — the derivation flow (frame at intent, stories authored inside it,
filter and selection after), entry authoring, and the write mechanics.

## Rules — delivered by mochiko-cli

Your rules arrive below, rendered at fire by `mochiko-cli` from the migration log this plugin
carries — one block per section. Every block opens with a version-triple line
(`mochiko-cli rules authoring-feature-map · section <id> · binary <v> · grammar <g> · plugin <p>`) and
closes with an end line (`mochiko-cli rules end · authoring-feature-map · <id> · <N> rules`). **Proceed
only when every block carries both lines in that exact shape, from whichever channel delivered
it — this slot, or the plugin's dependency hook on a Skill-tool call.** Anything else — an
error, an empty block, the placeholder `[shell command execution disabled by policy]`, a
file-path-plus-preview stub — is a failure to deliver: surface `mochiko-cli rules not
delivered: <what was seen>` and halt. Never Read a schema file instead; there is no fallback.
The `legend` in the preamble block is the reading grammar; a `pointer:` binds you to that
file's or skill's procedure, referenced never restated.

!`mochiko-cli rules authoring-feature-map --section preamble --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-feature-map --section authoring-feature-map.sec.independence --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-feature-map --section authoring-feature-map.sec.scope --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-feature-map --section authoring-feature-map.sec.inputs --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-feature-map --section authoring-feature-map.sec.artifact --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-feature-map --section authoring-feature-map.sec.output --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-feature-map --section authoring-feature-map.sec.reserved --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`

Before the first procedural step, state back the floor count the preamble's `class: floor` pin
prints and the ids its `floors:` line lists; a blank or partial read-back is a skipped read —
halt and surface it.

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
