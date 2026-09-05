---
name: authoring-prototype
description: This skill MUST be invoked when authoring a clickable low-fidelity UX prototype for a feature specification — the static HTML app under `.mochiko/specs/<feature>/prototype/` and the spec's Screens & Flows section (SCR-XXX, FLOW-XXX). SHOULD also invoke on 'mock the UX', 'clickable prototype', 'screens and flows', 'SCR-XXX', or 'FLOW-XXX'. Boundary: authors the PROTOTYPE — NOT the user stories it renders (mochiko:authoring-user-stories), NOT production UI code; never grades its own output.
allowed-tools: Bash(mochiko-cli *)
---

# Authoring a Clickable Low-Fi Prototype

A text spec cannot show what the experience will be — surprises surface at build time,
when they are expensive. A **clickable low-fidelity prototype** fixes this: static HTML
screens, wired with real links and buttons, that a user clicks through while the stories
are still wet. Structure and flows are precise; pixels are deliberately rough.

## Rules — delivered by mochiko-cli

Your rules arrive below, rendered at fire by `mochiko-cli` from the migration log this plugin
carries — one block per section. Every block opens with a version-triple line
(`mochiko-cli rules authoring-prototype · section <id> · binary <v> · grammar <g> · plugin <p>`) and
closes with an end line (`mochiko-cli rules end · authoring-prototype · <id> · <N> rules`). **Proceed
only when every block carries both lines in that exact shape, from whichever channel delivered
it — this slot, or the plugin's dependency hook on a Skill-tool call.** Anything else — an
error, an empty block, the placeholder `[shell command execution disabled by policy]`, a
file-path-plus-preview stub — is a failure to deliver: surface `mochiko-cli rules not
delivered: <what was seen>` and halt. Never Read a schema file instead; there is no fallback.
The `legend` in the preamble block is the reading grammar; a `pointer:` binds you to that
file's or skill's procedure, referenced never restated.

!`mochiko-cli rules authoring-prototype --section preamble --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-prototype --section authoring-prototype.sec.independence --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-prototype --section authoring-prototype.sec.scope --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-prototype --section authoring-prototype.sec.inputs --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-prototype --section authoring-prototype.sec.artifact --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-prototype --section authoring-prototype.sec.output --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-prototype --section authoring-prototype.sec.reserved --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`

Before the first procedural step, state back the floor count the preamble's `class: floor` pin
prints and the ids its `floors:` line lists; a blank or partial read-back is a skipped read —
halt and surface it.

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
