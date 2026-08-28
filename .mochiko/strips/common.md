# Strip notes — `plugins/mochiko/schemas/common.yaml`

Entry formats: `strips/README.md`. The shared block library was born at v0.98.0 (the
command-schema ontology wave, D8). Block additions are pure additions and ride their
decision row (five blocks added at v0.99.0: `no-acceptance` · `model-tiering` ·
`author-grader-default-fail` · `tools-referenced-never-restated` ·
`plan-approval-producers`); only superseding edits take entries — the two widened texts at
v0.99.0, the header supersession at v0.100.0.

<!-- Wave context: the schema-header runtime-kernel wave (v0.100.0) — shipped schema
top-of-file header comments trimmed to runtime-essential content. Ruling for every
[v0.100.0] entry below: `.mochiko/decisions/2026-08-28-schema-header-runtime-kernel.md`
(a recorded supersession-by-amendment of command-content-schema D14) + `DECISIONS.md`
2026-08-28 row. Pre-edit verbatim text: `git show e44b33d:plugins/mochiko/schemas/<file>`. -->

## [v0.100.0] `schemas/common.yaml` header — narrative superseded by the binding-resolution kernel
- **Disposition:** superseded → the 4-line binding-resolution kernel; the extraction bar lives in
  `.mochiko/decisions/2026-08-28-near-dup-convergence.md` R1/R2 and the ontology record (D8 as
  amended by C2/C3); the file's history lives in those same records
- **Tier failed:** n/a — supersession by ruling (`2026-08-28-schema-header-runtime-kernel.md` R2)
- **Content:** the 52-line header, faithfully compressed: D8/C2 supersession narrative ·
  extraction bar (3+-command near-identical, strongest-wording-wins, allowlist edge) · full
  resolution rule (inherited vs always-local fields, stub `class:` obligation, ${var} source) ·
  read-alongside instruction · ID-minting note · two history paragraphs (prototype screening;
  v0.99.0 near-dup wave). Verbatim: `git show e44b33d:plugins/mochiko/schemas/common.yaml`
  (lines 1–52).
- **Kept deliberately:** identity + the binding-resolution kernel (stub ID citable ·
  text/labels/pointer inherited only · ${var} from the binding schema's vars:, never this file).
- **Consumers assessed:** the six command `.md`s (each restates stub resolution in its Rules
  block; unchanged) · `.claude/skills/converting-command-to-schema/SKILL.md` (updated this wave) ·
  `scripts/check-command-schema.py` (post-edit: all blocks bound, PASS).


<!-- Wave context: the near-dup convergence wave (v0.99.0). Ruling for every [v0.99.0]
entry below: `.mochiko/decisions/2026-08-28-near-dup-convergence.md` R1–R6 + wave flags →
`DECISIONS.md` 2026-08-28 row. A common-block text edit changes the RESOLVED text of every
binding stub, so each entry's consumers list is the stub set at edit time. -->

## [v0.99.0] `common.transport-floor` — text widened with the desks' enumeration (flag A)

- **Disposition:** superseded → the widened text on the same block, gaining "Trigger test,
  floor legs, composition-safe shapes, and disclosure live there" from the arch/feat
  wording (strongest member, flag A).
- **Tier failed:** n/a — supersession by ruling (near-dup ADR R1/R2 + flag A).
- **Content:** the text as shipped at v0.98.0, verbatim —

  ```
      mochiko:patterns-transport-floor governs composition and messaging under a
      split trigger — message legs on any multi-seat messaging, topology legs on
      shared writes — non-waivable once triggered; referenced, never restated.
  ```

- **Kept deliberately:** every prior clause survives word for word; the enumeration is
  added, nothing removed.
- **Consumers assessed:** binding stubs at edit time — `brainstorm.transport-floor` ·
  `setup.transport-floor` · `spec.transport-floor` (all three floors; their resolved text
  gains the enumeration — R4 supersession carried by this entry; the brainstorm omission
  was a slip caught by the V3 audit, repaired pre-bump), joined at the same wave by
  `arch.transport-floor` · `feat.transport-floor` · `impl.transport-floor` (entries in
  their own strip files).

## [v0.99.0] `common.acceptance-plain-text` — text widened to "rulings and acceptance" (wave move 7)

- **Disposition:** superseded → the widened text on the same block: "User rulings and
  acceptance are plain blocking text, never a timed prompt." — so the desks' "rulings"
  vocabulary binds through the same block.
- **Tier failed:** n/a — supersession by ruling (near-dup ADR R1/R2, wave move 7).
- **Content:** the text as shipped at v0.98.0, verbatim —

  ```
      User acceptance is plain blocking text, never a timed prompt.
  ```

- **Kept deliberately:** the acceptance clause survives word for word; "rulings and" is
  added, nothing removed.
- **Consumers assessed:** binding stubs at edit time — `brainstorm.acceptance-plain-text` ·
  `setup.acceptance-plain-text` (floor — R4 supersession carried by this entry) ·
  `spec.acceptance-plain-text`, joined at the same wave by `arch.rulings-plain-text` ·
  `feat.rulings-plain-text` (entries in their own strip files). `impl.acceptance-plain-text`
  stays local by flag D and is unaffected.
