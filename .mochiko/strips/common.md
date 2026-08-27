# Strip notes — `plugins/mochiko/schemas/common.yaml`

Entry formats: `strips/README.md`. The shared block library was born at v0.98.0 (the
command-schema ontology wave, D8); this file opens with the first edits that superseded any
of its shipped text. Block additions are pure additions and ride their decision row (five
blocks added at v0.99.0: `no-acceptance` · `model-tiering` · `author-grader-default-fail` ·
`tools-referenced-never-restated` · `plan-approval-producers`); only the two widened texts
below take entries.

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
