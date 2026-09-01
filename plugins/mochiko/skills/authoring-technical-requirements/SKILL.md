---
name: authoring-technical-requirements
description: This skill MUST be invoked when authoring `constraints-and-decisions.md` — hard constraints (C-XXX), technology decisions (D-XXX), infrastructure provisioning (IP-XXX), and the thin INT-XXX / DS-XXX declarations — plus the NFR-XXX grammar the architecture store's concern rows carry, each traced to a business source. Fires in `/mochiko:implement`'s design phase, or at build time through the gated `baseline-delta.md` path. SHOULD also invoke on 'C-', 'D-', 'NFR-', 'IP-', or 'technical constraints'. Owns the artifact structure — NOT the decision technique (mochiko:patterns-technical-decisions).
---

# Authoring Technical Constraints and Decisions

Translate business specifications into the design-time constraint layer: the
`constraints-and-decisions.md` artifact and the NFR rows the architecture store carries.

## Rules — load the schema first

Your first action, before any authoring: **Read `schema.yaml` (this skill's own directory)
and `../../schemas/skill-authoring-common.yaml` raw, in full, in the same declared first
action** — schema, then common. The schema is the source of truth for this skill's binding
rules, nested in six sections, each addressable by its section ID:
`authoring-technical-requirements.sec.independence` ·
`authoring-technical-requirements.sec.scope` ·
`authoring-technical-requirements.sec.inputs` ·
`authoring-technical-requirements.sec.artifact` ·
`authoring-technical-requirements.sec.output` ·
`authoring-technical-requirements.sec.reserved`. Interpret it live: a rule's `kind:` names
what it is, and an absent `kind:` reads `constraint`; a `pointer:` rule binds you to that
file's or skill's procedure, referenced never restated; labels come from
`plugins/mochiko/schemas/skill-labels.yaml`. A rule carrying
`extends: authoring-common.<slug>` inherits text/labels/pointer from
`skill-authoring-common.yaml` only — `class` and every absence-meaningful field are local —
and the stub's `authoring-technical-requirements.*` ID stays the citable ID. The floor pin:
the 8 rules of `class: floor` are non-waivable. Before the first authoring step, state the
floor count back — a skipped or partial read leaves that count blank: halt and surface it,
and halt likewise if the schema's `class: floor` count disagrees with the pin.

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
