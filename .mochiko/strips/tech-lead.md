# Strip notes — `agents/tech-lead.md`

Entry formats: `strips/README.md`. First entry (new file, created v0.78.0).

## [v0.91.0] Store-grading scope line: plan-run vocabulary re-keyed to design deltas — plan-stage retirement D1/D4

- **Disposition:** superseded → the reworded sentence: "Design deltas need no separate pass
  from you here — the feasibility and completeness reviews already grade them, and the user
  signs off on the design."
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/plan-stage-utility/record.md` D1 — `/mochiko:plan` retired, implement
  absorbs a conditional design phase; D4 — the design checkpoint replaces plan's package
  acceptance and the user signs the store delta there; `DECISIONS.md` 2026-08-26 row).
- **Content (superseded, verbatim):**

  > Plan-time deltas need no separate pass from you here — the feasibility review and the plan-artifact
  > review already grade them, and the user signs off on the diagram.

  Three dead references in one sentence: "Plan-time" named a run that no longer exists; "the
  plan-artifact review" named `review-plan-artifacts` by its retired subject rather than its
  re-scoped one (D5 re-points it onto the design-phase output); "the diagram" named plan's
  rendered-diagram sign-off gate, which died with the command — the surviving user signature is
  on the design phase's output, the store delta especially.
- **Kept deliberately:** the sentence's actual load — that this seat owes no *separate* grading
  pass over deltas another review already covers — survives unchanged; only the run-specific
  nouns moved. The following sentence ("you grade writes you did not author…") is untouched.
  The re-key is also a keystone improvement: the replacement names no workflow at all, where the
  original named a command's stage.
- **Consumers assessed:** persona body line, no other primitive quotes it. `review-feasibility`
  and `review-plan-artifacts` are re-scoped in the same wave (P3); the wording here points at
  the review *functions* (feasibility, completeness), not at either skill's name, so it stays
  correct across that re-scope.

## [v0.91.0] Skills-Available bullet: "plan analysis/design artifacts" → "design artifacts" — plan-stage retirement D1/D5

- **Disposition:** superseded → the re-keyed bullet: "**`mochiko:review-feasibility`** — the
  cross-artifact feasibility review of design artifacts (never the governance surface itself)."
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/plan-stage-utility/record.md` D1 + D5 — the plan package ceases to
  exist and `review-feasibility` re-scopes onto the in-run design phase's output;
  `DECISIONS.md` 2026-08-26 row).
- **Content (superseded, verbatim):** `the cross-artifact feasibility review of plan analysis/design artifacts (never the governance surface itself).`
- **Kept deliberately:** the bullet's carve — "never the governance surface itself" — survives
  byte-for-byte; it is the boundary the bullet exists to state. The `skills:` frontmatter mount
  `review-feasibility` is unchanged: the skill keeps its slug through its P3 re-scope.
- **Found-not-assigned note:** not in the build brief's three assigned lines; surfaced on a
  post-edit sweep of the same file and is the same defect from the same ruling — the persona
  would otherwise have pointed at "plan analysis" one screen above a paragraph this wave just
  re-keyed off it. Recorded rather than fixed silently.
- **Consumers assessed:** persona body, one bullet; no primitive quotes it. Body-class edit —
  agent bodies carry no budget row (descriptions only), so no pre-assert applies; net −8 chars.

## [v0.78.0] Delegating Cheap Reads retargeted — `mochiko:explorer` dispatch superseded by native `Explore` + `model: haiku` override

- **Disposition:** superseded → the reworded `## Delegating Cheap Reads` sentence: "spawn a
  disposable native `Explore` subagent with an explicit `model: haiku` override (the
  override makes the read cheap; a bare spawn inherits the session tier)".
- **Tier failed:** n/a — supersession by ruling (ADR
  `.mochiko/decisions/2026-08-19-explorer-retarget-native.md`; `DECISIONS.md` 2026-08-19
  row). Dogfood failure: agent-team teammates cannot spawn plugin-scoped agents, so the
  `mochiko:explorer` dispatch this section prescribed failed on exactly the transport the
  section was built for.
- **Content:** verbatim superseded span (identical across all ten personas): "spawn a
  disposable `mochiko:explorer` subagent (its `model: haiku` frontmatter makes the read
  cheap)".
- **Kept deliberately:** the rest of the `## Delegating Cheap Reads` section byte-for-byte —
  the class-key summary (locate/enumerate/targeted-read cheap; interpretive, absence-driven,
  completeness-sensitive kept), one-gap-per-spawn, the bulk-read-stays-out rule, and the
  closing pointer to `mochiko:patterns-model-tiering`.
- **Consumers assessed:** the section wording is shared across the ten personas; all ten
  edited in the same v0.78.0 wave (this entry mirrored in each persona's strip file). No
  command or skill names the section.

