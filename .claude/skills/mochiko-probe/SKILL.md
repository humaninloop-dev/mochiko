---
name: mochiko-probe
description: This skill MUST be invoked when a mochiko framework-maintenance run is probing whether a repo-side skill fires at all — the throwaway continuity probe for the D6 trio move. SHOULD also invoke on "run the repo-side probe" or "does the out-of-plugin skill load". Carries no doctrine and produces no artifact; firing is the whole result.
---

# Mochiko Probe (throwaway)

## Overview

This skill exists to answer one question before the framework-maintenance trio
(`command-architect`, `authoring-commands`, `validation-command-shape`) leaves the shipped
plugin for `.claude/`: **does a repo-side skill get discovered and model-invoked the way a
plugin skill does?** Absence of `.claude/skills/` proved nothing about its semantics
(`verbosity-caveman-ops-separation` F44), so the S14 fold made a working probe a
precondition of the move.

Delete this directory once the probe result is recorded — it is scaffolding, not a primitive.

## What to do when invoked

State, in one line each:

1. That `mochiko-probe` fired, and whether it was model-invoked or named explicitly.
2. Whether this file's body was readable in full.

Nothing else. No artifact, no report file, no follow-on work.

## Probe result contract

The run that spawned this skill records the outcome in the wave-3 dispatch, not here. A
firing proves discovery only — the agent half of the probe (`.claude/agents/mochiko-probe.md`)
is what proves a repo-side agent can be spawned as a teammate.
