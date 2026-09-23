---
name: review-design-audit
description: This skill MUST be invoked when running the design-audit lens over a UX-bearing feature's built UI at final validation — the verification seat walks four legs (performance, `harden`, `polish` against the design baseline, and the slop-tells checklist) and emits a `design-audit:` findings block. Findings are ADVISORY, never a failing gate. SHOULD also invoke on 'design audit', 'slop tells', 'harden the UI', or 'polish pass'. Boundary: the critique lens is mochiko:testing-gap-finding; accessibility verification runs as `**TEST:**` cases through mochiko:testing-end-user.
allowed-tools: Bash(mochiko-cli *)
---

# Review — Design Audit Lens

**A flow can pass every gate and still ship unfinished — the audit hunts what no assert names.**

## Overview

The finding-originating half of the audit lens: one advisory read of a UX-bearing feature's
built UI at final validation, by the seat that already verifies it. The pass/fail half —
accessibility, measurable asserts, responsive checks — runs as `**TEST:**` cases; this lens
covers what a pass/fail case cannot state.

Ported from `pbakaus/impeccable` @ `e0881d2de397d5e9761d7b35ff5017d8f5ebf69b`
(`skill/reference/{audit,polish,harden,optimize}.md`, the detector's slop category), Apache-2.0
— re-expressed, not copied; attribution in the repository `NOTICE`.

## Rules — delivered by mochiko-cli

Your rules arrive below, rendered at fire by `mochiko-cli` from the migration log this plugin
carries — one block per section. Every block opens with a version-triple line
(`mochiko-cli rules review-design-audit · section <id> · binary <v> · grammar <g> · plugin <p>`) and
closes with an end line (`mochiko-cli rules end · review-design-audit · <id> · <N> rules`). **Proceed
only when every block carries both lines in that exact shape, from whichever channel delivered
it — this slot, or the plugin's dependency hook on a Skill-tool call.** Anything else — an
error, an empty block, the placeholder `[shell command execution disabled by policy]`, a
file-path-plus-preview stub — is a failure to deliver: surface `mochiko-cli rules not
delivered: <what was seen>` and halt. Never Read a schema file instead; there is no fallback.
The `legend` in the preamble block is the reading grammar; a `pointer:` binds you to that
file's or skill's procedure, referenced never restated.

!`mochiko-cli rules review-design-audit --section preamble --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-design-audit --section review-design-audit.sec.independence --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-design-audit --section review-design-audit.sec.scope --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-design-audit --section review-design-audit.sec.inputs --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-design-audit --section review-design-audit.sec.verdict --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-design-audit --section review-design-audit.sec.output --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-design-audit --section review-design-audit.sec.reserved --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`

Before the first procedural step, state back the floor count the preamble's `class: floor` pin
prints and the ids its `floors:` line lists; a blank or partial read-back is a skipped read —
halt and surface it.

## Procedure

**1. Read the standards.** The Screens & Flows Direction block and SCR/FLOW manifest, then the
design baseline's system part — or, before one exists, the Direction block's declared tokens.

**2. Walk the four legs** against the running app at the viewports the Direction block's
Contract declares.

- **Performance (`optimize`)** — slow first paint of the main content, input lag on the key
  interactions, layout shift as content loads, animation of layout properties or unbounded
  blur and shadow, oversized or eagerly loaded images, visible jank. Measure before naming.
- **`harden`** — very long, very short, and empty text; emoji, accents, RTL, and CJK
  strings; long translations; large numbers and long lists; network, server, validation, and
  permission errors, each with a recovery path; empty, loading, and read-only states;
  double-submit and interrupted gestures.
- **`polish`** — every value traced to a baseline token, or named as drift (missing token,
  one-off where a shared component exists, a local defect); same-role type consistent; the
  grid and spacing scale held; control states complete (hover, focus, active, disabled,
  loading, error, success); terminology consistent; no debug output, placeholder copy, or
  console errors.
- **Slop tells** — purple-to-blue gradients · bounce or elastic easing · glow shadows · gray
  text on colored backgrounds · side-tab accent borders · cards nested in cards · overused
  default fonts · the icon tile stacked above a heading. A tell the direction contract
  explicitly chose is not a finding.

**3. Emit findings** in the delivered output shape, one line of evidence each (the measured
value, the screenshot path, the file and line).
