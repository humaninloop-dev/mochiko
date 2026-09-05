---
name: authoring-technical-requirements
description: This skill MUST be invoked when authoring `constraints-and-decisions.md` — hard constraints (C-XXX), technology decisions (D-XXX), infrastructure provisioning (IP-XXX), and the thin INT-XXX / DS-XXX declarations — plus the NFR-XXX grammar the architecture store's concern rows carry, each traced to a business source. Fires in `/mochiko:implement`'s design phase, or at build time through the gated `baseline-delta.md` path. SHOULD also invoke on 'C-', 'D-', 'NFR-', 'IP-', or 'technical constraints'. Owns the artifact structure — NOT the decision technique (mochiko:patterns-technical-decisions).
allowed-tools: Bash(mochiko-cli *)
---

# Authoring Technical Constraints and Decisions

Translate business specifications into the design-time constraint layer: the
`constraints-and-decisions.md` artifact and the NFR rows the architecture store carries.

## Rules — delivered by mochiko-cli

Your rules arrive below, rendered at fire by `mochiko-cli` from the migration log this plugin
carries — one block per section. Every block opens with a version-triple line
(`mochiko-cli rules authoring-technical-requirements · section <id> · binary <v> · grammar <g> · plugin <p>`) and
closes with an end line (`mochiko-cli rules end · authoring-technical-requirements · <id> · <N> rules`). **Proceed
only when every block carries both lines in that exact shape, from whichever channel delivered
it — this slot, or the plugin's dependency hook on a Skill-tool call.** Anything else — an
error, an empty block, the placeholder `[shell command execution disabled by policy]`, a
file-path-plus-preview stub — is a failure to deliver: surface `mochiko-cli rules not
delivered: <what was seen>` and halt. Never Read a schema file instead; there is no fallback.
The `legend` in the preamble block is the reading grammar; a `pointer:` binds you to that
file's or skill's procedure, referenced never restated.

!`mochiko-cli rules authoring-technical-requirements --section preamble --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-technical-requirements --section authoring-technical-requirements.sec.independence --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-technical-requirements --section authoring-technical-requirements.sec.scope --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-technical-requirements --section authoring-technical-requirements.sec.inputs --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-technical-requirements --section authoring-technical-requirements.sec.artifact --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-technical-requirements --section authoring-technical-requirements.sec.output --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-technical-requirements --section authoring-technical-requirements.sec.reserved --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`

Before the first procedural step, state back the floor count the preamble's `class: floor` pin
prints and the ids its `floors:` line lists; a blank or partial read-back is a skipped read —
halt and surface it.

## The artifact walk

Author the layers in order, each block to its field set: Part 1 hard constraints (C-XXX),
Part 2 technology decisions (D-XXX), Part 3 infrastructure requirements (IP-XXX), and
Part 4 the thin INT-XXX / DS-XXX declarations — all on `constraints-and-decisions.md` —
then the NFR-XXX rows the architecture store's concern rows carry. Field definitions,
document templates, and worked examples live in
[ARTIFACT-TEMPLATES.md](references/ARTIFACT-TEMPLATES.md); cross-reference patterns,
dependency chains, and the completeness procedures live in
[TRACEABILITY-PATTERNS.md](references/TRACEABILITY-PATTERNS.md).

## Common Rationalizations

The Reality column answers each excuse as it arises mid-authoring.

| Excuse | Reality |
|--------|---------|
| "NFR targets can be refined later during design" | Targets set during design are reverse-engineered from implementation, not derived from business needs. Define now. |
| "Only a few integrations, formal mapping is overkill" | Few integrations with undocumented failure modes cause the worst outages. Catalogue every one. |
| "Data classification is a security team concern" | Every constraint that touches data needs classification before design. Security reviews supplement, not replace. |
| "Constraints are well-known to the team" | Implicit constraints cause the costliest mid-implementation discoveries. Make every one explicit. |
| "This is a simple system" | Simple systems with missing constraints become complex debugging sessions. Follow the full process. |
