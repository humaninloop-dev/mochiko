# Strip notes — `hooks/scripts/dependency-halt.sh`

Entry formats: `strips/README.md`.

## [v0.120.0] The delivery-check comment's prose-skill count — "the seven prose skills" superseded by six

- **Disposition:** superseded → `dependency-halt.sh`:76, as rewritten: "A primitive with no `!`
  line has no rules to deliver — the six prose skills and the router carry procedure only, and
  never had a rule set —".
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/implement-brainstorm-led/record.md` D2-iterative-becomes-engine —
  `analysis-iterative` converted from a prose skill to a pair whose rules `mochiko-cli` renders
  from the log, wave 1, `0058-thinking-engine.yaml`; the `DECISIONS.md` 2026-10-11 "Shared
  thinking engine ruled" row). The skills with no `!` line are now six —
  `grooming-operating-docs`, `patterns-api-contracts`, `patterns-entity-modeling`,
  `patterns-system-design`, `patterns-technical-decisions`, `testing-governance-injection` —
  plus the router.
- **Content (superseded):** the changed line at `7d9d29e`, verbatim, as `<file>:<line>: <text>`
  (files under `plugins/mochiko/hooks/scripts/`):

  ```text
  dependency-halt.sh:76: # The delivery check. A primitive with no `!` line has no rules to deliver — the seven prose
  ```

- **Kept deliberately:** the rest of the comment block (`:76–79`) byte for byte — "and the router
  carry procedure only, and never had a rule set" and "The primitive's own file is the truth here,
  so there is no list to keep in sync"; no reflow (the line shortens by two characters); the
  `grep -q -F` check at `:80` untouched.
- **Consumers assessed:** the hook's behaviour is unchanged — its gate keys on the primitive's own
  `` !`mochiko-cli rules `` line (`:80`), never on the count, and `analysis-iterative` has carried
  seven such lines since `963a3d6`, so it has been gated since wave 1; `evals/contract/README.md`'s
  leave-alone row names "the seven prose skills that carry no rules" with the router among its
  seven, consistent as a count of primitives (the wave-3 test seat's write set);
  `.claude/rules/mochiko/primitive-edits.md:75` and `:242` carried the same "seven prose
  skills" count, both corrected from seven to six in this landing by the lead's edit (the
  build's Q-D item).

## [v0.118.0] Wave 3 back-fill: the kernel-tooling-admission cite joined

- **Disposition:** superseded → `dependency-halt.sh`:5 now cites GI-019-kernel-tooling-admission.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/human-readable-ids/record.md` D11-cross-session-qualifier,
  D14-live-layer-backfill and D15-protected-line-rewrites — every live-layer mention joined to its
  definition's slug, cross-session mentions qualified, verbatim spans masked; wave plan
  `.mochiko/brainstorms/human-readable-ids/wave3-backfill.md` items 7–9)
- **Content (superseded):** every changed line at `0b8982a`, verbatim, as `<file>:<line>: <text>`
  (files under `plugins/mochiko/hooks/scripts/`):

  ```text
  dependency-halt.sh:5: # The gate is on dependency absence only, never on behavior or judgment (GI-019). It fires for
  ```

- **Kept deliberately:** `dependency-halt.sh`:2's "(record D7b)": it names its ruling only as
  "record", with no qualifier or link, and "D7b" is a clause pointer (wave plan S8); left as
  written.
