---
name: patterns-model-tiering
description: This skill MUST be invoked when dispatching exploration or fact-finding work in any mochiko run — routing each read by the class key. locate/enumerate/targeted-read gaps go to a native `Explore` subagent spawned with an explicit `model: haiku` override; interpretive reads, decision-driving absences, and completeness-sensitive enumerations stay on the session tier. SHOULD also invoke on 'model tiering', 'cheap explorer', 'which model', 'explore the code', 'targeted read', or 'fact-find dispatch'. Governs dispatch tier only — rostered seats never change model (model-tiered-seats D5); third sibling of patterns-sound-loop and patterns-transport-floor.
---

# Model Tiering — The Class-Keyed Dispatch Floor

**Every read rides the lowest tier where its answer can be trusted.**

## Overview

Rostered mochiko personas run on the strong tier and stay there; this floor governs the
*reads they and the lead dispatch along the way*. The economics are documented, not
assumed: Haiku is ~5× cheaper than Opus and ~10× cheaper than Fable per token both
directions, and on subscription seats cheap-model work preserves Opus-cap headroom
(model-tiered-seats D1).

## Rules — load the schema first

Your first action, before any dispatch: **Read `schema.yaml` (this skill's own directory)
raw, in full, as one declared first action.** The schema is the source of truth for this
skill's binding rules, nested in six sections, each addressable by its section ID:
`patterns-model-tiering.sec.trigger` · `patterns-model-tiering.sec.scope` ·
`patterns-model-tiering.sec.discipline` · `patterns-model-tiering.sec.inputs` ·
`patterns-model-tiering.sec.disclosure` · `patterns-model-tiering.sec.reserved`.
Interpret it live: a rule's `kind:` names what it is, and an absent `kind:` reads
`constraint`; a rule of `class: floor` is always read and always delivered; a `pointer:`
rule binds you to that file's or skill's procedure, referenced never restated; labels come
from `plugins/mochiko/schemas/skill-labels.yaml`. The floor pin: the 4 rules of
`class: floor` are non-waivable. Before the first dispatch, state the floor count back — a
skipped or partial read leaves that count blank: halt and surface it, and halt likewise if
the schema's `class: floor` count disagrees with the pin.
