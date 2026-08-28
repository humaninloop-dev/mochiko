# Schema-header runtime kernel — top-of-file comments carry runtime-essential content only

**Date:** 2026-08-28 · **Status:** ruled + built (same-session wave; no session record — this
ADR is the rationale home) · **Ruled by:** the user, in-session: "in the schemas remove all
the comments that have been added at the top, if it not essential at runtime of schemas."

## Context

Every command content schema opened with the canonical full-grammar header comment (~68
lines), minted per command-content-schema D14 and copied verbatim across all six schemas.
The header restated the whole reading grammar — field semantics, ID minting ceremony,
referential closure, provenance sidecar, rule grain, checker pointer — plus decision
citations. The schemas are Read raw, in full, at every command fire (D1/D7), so the header
is paid for on every run. Meanwhile the runtime reader already receives the reading grammar
from the command `.md`'s "Rules — load the schema first" block (scaffold-standardized across
all six commands), and the edit-time grammar is single-sourced in
`.claude/rules/mochiko/primitive-edits.md` criterion 11 and the
`command-schema-ontology` / `command-content-schema` records. The header was therefore a
third restatement, almost entirely redundant at the one moment it is actually read.

Audit of what the `.md` Rules blocks do NOT restate found exactly two semantics that lived
only in the header: the `class:` value meanings beyond `floor` (`must`, `advisory`), and the
`conditions:` resolution-point vocabulary (`entry-derived` · `surface-presence` ·
`moment-resolved(<moment>)` · `user-ruled` · `standing-trigger`, and the
inapplicable-until-resolved rule).

## Rulings

**R1 — command content schemas open with the runtime-kernel header.** The six command
content schemas (`architecture` · `brainstorm` · `feature` · `implement` · `setup` ·
`specify`) open with a uniform 8-line kernel: identity line, the read-at-fire /
interpret-live note pointing at the `.md` Rules block for the reading grammar, and the two
semantics that live only here (`class:` meanings; `conditions:` resolution points). This is
a **recorded supersession-by-amendment of command-content-schema D14**: the canonical header
form is now the kernel, still one block copied verbatim with only `<cmd>` substituted. The
full grammar's homes are unchanged and already single-sourced — runtime: each command `.md`
Rules block; edit-time: `.claude/rules/mochiko/primitive-edits.md` criterion 11 plus the
`command-content-schema` and `command-schema-ontology` records.
`conversion-inventory.md` section G remains the historical authored form of the old header
but is no longer the mint source. The former header's per-command legal-self-reference
variance line (desk vs run vocabulary) is carried by the D13 checker's curated marker list
and the ontology record, not by the header.

**R2 — `common.yaml` opens with the binding-resolution kernel.** Identity plus the
stub-binding rule (citable ID stays the stub's; `text`/`labels`/`pointer` inherited only;
`${var}` substitutes from the binding schema's `vars:`, never from `common.yaml`). The
extraction bar and the file's history live in the near-dup convergence ADR
(`2026-08-28-near-dup-convergence.md` R1/R2) and the ontology record (D8 as amended by
C2/C3) — removed from the header as restatement.

**R3 — `command-labels.yaml` header keeps registry meaning plus the tombstone.** The
cross-command-link semantics stay (runtime: a reader of `labels:` values needs them); the
`fail-condition` retirement tombstone stays **verbatim** (record integrity — a tombstone
never leaves by cleanup). The registry-edit-first ceremony, the common.yaml-exception
narrative, and the Stage-1/goal-state roadmap note are removed as edit-time content: the
registry-edit-first ceremony and the one-line-of-meaning rule are homed in the
command-content-schema record D8, the common.yaml-exception narrative in the ontology record
(D8 as amended by C2), and the Stage-1/goal-state note in the command-content-schema record
(D3/D4). The clause "a label is a cluster name, never a summary" is deliberately dropped, not
relocated — its surviving limb is the D8 record's "one line of meaning" line, and the
closed-set rule's executable home is `scripts/check-command-schema.py` check 4.

**R4 — small data-schema headers stand.** The template data schemas (`architecture-store` ·
`codebase-analysis` · `feature-entry` · `features-index` · `governance-intent` ·
`governance-surfaces` · `spec` · `tasks`) and `architecture-shelf-backend.yaml` keep their
2–6-line headers: identity plus the raw-Read degraded-path orientation is itself
runtime-essential on the binary-absent path (GI-020), and the DOCTRINE-DENSE
verbatim-preservation lines govern the producing run. No trim.

## Consumers assessed

- The six command `.md`s — self-carry the reading grammar; unaffected.
- `.claude/skills/converting-command-to-schema/SKILL.md` — updated this wave: exemplar
  lines and procedure step 2 now mint the kernel header and point at the grammar's homes.
- `scripts/check-command-schema.py` — does not read header comments; post-edit run PASS,
  0 findings on all six pairs.
- `.mochiko/provenance.yaml` — unaffected (D16 anchors were never in the schemas).

## Delivery note (2026-08-28)

All eight shipped files trimmed (six kernels + common + labels; net −421 lines, ~−33k chars
across the eight files, ~−8.9k off each command fire's schema read). Strips: supersession-by-ruling entries in
`.mochiko/strips/{architecture,brainstorm,feature,implement,setup,specify,common,command-labels}.md`
citing this ADR. Checker `--all`: PASS, 0 findings ×6. Author≠grader audits dispatched per
the primitive-edit ceremony; ships with the next `plugin.json` bump.
