---
name: authoring-commands
description: |
  Author a mochiko command in the codified command shape, convert an existing command to it
  (incl. one-shot → team-form), run a minimalism/strip pass over a command cluster with
  version-stamped strip notes, or execute a ruled shape-home revision — a rulings-driven
  edit to command-shape.md itself. Use when creating a new commands/*.md supervisor,
  converting a command's orchestration form, stripping/relocating command or primitive
  prose, or encoding session rulings into the shape home.
  MUST BE USED when the task says "author a command", "convert this command to team form",
  "apply the command shape", "run a strip wave / minimalism pass", or "revise the command
  shape / shape home". Producer craft only — the authored/converted/revised artifact is
  graded independently (validation-command-shape), never by its author.
---

# Authoring Commands — Build, Convert, Strip, Revise

## Overview

Four jobs, one discipline: a command states its per-workflow parameters and references
everything else. The shape's single authoritative home is
`${CLAUDE_PLUGIN_ROOT}/templates/command-shape.md` (Layer 1 form-agnostic core · Layer 2
team transport) — **Read it first, every run**; this skill never restates its content.
Dispatch briefing + seat transport: `templates/agent-dispatch.md`. Loop soundness:
`mochiko:loop-discipline`.

## Job 1 — author a new command

1. **Read** `command-shape.md` (both layers), plus the workflow's design record or accepted
   artifacts — the command's requirements source.
2. **Decide the form** (Job 2's assessment, run pre-emptively): team-form or one-shot, on
   recorded command-specific grounds.
3. **Fill only parameters.** Work through the shape's `[PARAM]` tags — deliverable artifact
   + ID scheme, seat roster (agent × skill × spawn timing × standing/cold), sizing-gate
   keying, verify-pass owner, fact substrate, gates, bounds, recovery mapping — and write
   the command as: frontmatter → goal → lead framing + the obligated Read of
   `command-shape.md` → the per-workflow sections → Contract (authoring-time fill) →
   Recovery.
4. **Altitude check before handoff:** every line is (i) unique to this workflow, (ii) a
   reference, or (iii) a marked exception (`<!-- shape-exception: why -->`). If a sentence
   would be true of every conformant command, it belongs in the shape home — propose it
   there instead (a template edit is a shared-primitive change: flag it, never slip it in).
5. **Hand off for independent audit** (the `validation-command-shape` bar). Never grade
   your own output.

## Job 2 — convert an existing command

The conversion assessment — recorded, per command, never blanket:

1. **State the command-specific rationale** for the target form. Team-form is the declared
   default preference where longer-horizon context retention and lead↔teammate
   collaboration pay (the lead orchestrates and interfaces with the human); one-shot
   subagents stay right for surgical, bounded work. No pre-existing dogfood evidence is
   required to convert — none can exist before the team variant does — but the rationale
   must be the command's own, not the doctrine restated.
2. **Name the confirm-or-revert checkpoint:** the first post-conversion dogfood run
   confirms the conversion or reverts it. Write that into the conversion note.
3. **First conversion only — home-revision checkpoint:** before authoring, re-open
   `command-shape.md` against the needs this conversion reveals; a shape gap found here is
   a template revision (plus a re-audit of conformant commands), never a command-local
   workaround.
4. **Convert as Job 1** (steps 3–5), preserving every workflow-specific responsibility;
   anything dropped or relocated follows Job 3's logging.

## Job 3 — strip pass (a minimalism wave)

Unit: one command **plus its related agents and skills** — the cluster. Division of labor:
you propose, the user rules contested lines, an independent audit closes the wave.

1. **Tier 1 — altitude (structural; you propose):** per line — unique content, a
   reference, or a marked exception survives; restated doctrine/pattern is proposed
   **relocated** to its single-sourced home (the dominant disposition — relocation, not
   deletion) or deleted only when it has no home to earn.
2. **Tier 2 — purpose-proof (you draft, the user ratifies):** for Tier-1 survivors, draft
   a per-line rationale — the behavior it produces or the failure it prevents, with
   provenance where it exists (dogfood defect, session ruling). The user ratifies
   **contested/uncertain lines**, not every line.
3. **Shared primitives** (consumed by more than one cluster): a strip runs its Tier-2 test
   against **all consumers**, and its entry names the consumers assessed. **≥3 consumers →
   raise the strip in-wave but do not rule it there**: it escalates to a scheduled
   all-consumer pass.
4. **Log every outcome** per the entry formats in `.mochiko/strips/README.md` — one note
   per primitive at `.mochiko/strips/<primitive>.md`, **repo-side, never under `plugins/`**
   (why: the README) — strip entries, survivor-provenance entries for contested keeps,
   re-add entries later — each stamped with the plugin version. One version bump per wave.
5. **Hand the wave to the independent audit** before it ships.

## Job 4 — shape-home revision

A ruled edit to `command-shape.md` itself — standalone, unlike the revisions that ride a
first conversion (Job 2 step 3) or a strip wave (Job 3). The home's own header carries the
protocol: one edit here plus a re-audit of the conformant commands.

1. **Read** the shape home and the ruling source (session record or ADR) this run. The
   rulings are the requirements — nothing beyond them enters the revision; a gap the
   rulings don't cover goes back to the user, never gets invented.
2. **Edit at shape altitude:** doctrine true of every conformant command lives in the
   home; per-command variance stays a `[PARAM]` tag. Rewritten or removed content follows
   Job 3's logging (version-stamped strip-note entry); pure additions are recorded in the
   revision's decision row, not the strip note (the v3 run-cost precedent).
3. **Stamp the footer:** bump the shape version with date + ruling source, preserving the
   prior version history.
4. **Name the re-audit set:** every conformant command whose text the revision affects,
   listed in the handoff — the re-audit is its own graded step, never skipped.
5. **Hand off for independent audit** (`validation-command-shape`, shape-revision mode).
   Never grade your own output.

## Common mistakes

- **Restating the shape "for safety."** The bet that a referenced home fires is the
  design — backstopped by the audit's deterministic floor, not by a second copy that will
  drift.
- **Deleting what should relocate.** Verbosity in a command is usually mis-altitude, not
  dead weight; the home must exist before the line leaves.
- **Blanket conversions.** The doctrine is a default, not a ruling — each command's
  assessment stands on its own recorded grounds.
- **An unlogged strip.** If it is not in the strip note with a version stamp, it did not
  happen honestly.
