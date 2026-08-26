# Strip notes — `templates/techanalyst-report-template.md`

Entry formats: `strips/README.md`. Wave context: workflow-token-reduction wave 1 (rulings
ratified 2026-07-23: producer disclosures machine-first).

## [v0.91.0] Usage Note 2's ID list drops the dead `TR-` class (second site, same ruling)

- **Disposition:** superseded → the surviving id classes. Usage Note 2 told the producer to cite
  IDs rather than restate artifact content, and named `TR-` first among the classes to cite. The
  same ruling that removed `requirements.md` from this template's `produced:` field (the entry
  below, same version) killed the artifact that defines `TR-XXX`, so the instruction pointed a
  producer at an id class nothing in the pipeline mints. A second site of one ruling, found after
  the first was written — recorded as its own entry rather than folded into the earlier one, so
  the sweep history stays honest about what was caught when.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/plan-stage-utility/record.md` D3 — the FR→TR layer dies as a mandatory
  artifact and the real technical decisions land as `D-XXX`, `C-XXX`, and `IP-XXX` rows in
  `constraints-and-decisions.md` plus the store's `NFR-XXX` concern rows; `DECISIONS.md`
  2026-08-26 row.)
- **Content (superseded, verbatim, one site — Usage Note 2's closing parenthetical):** "(cite
  TR-/C-/NFR-/D- IDs)". Now: "(cite C-/D-/IP-/NFR- IDs)".
- **Kept deliberately:** the note's whole substance — machine-first framing, the
  frontmatter-only routine round, `## Notes of note` for genuine difficulties, and the
  cite-never-restate rule itself — is untouched; only the class list changed. `IP-` was added
  rather than the list merely shortened, because D3 routes infrastructure-provisioning decisions
  to the same surviving home as `D-`/`C-`, and this template's `produced:` comment (the entry
  below) already names all three; leaving `IP-` out would have made the two fields disagree.
- **Consumers assessed:** the `mochiko:technical-analyst` persona fills this template; the id
  classes named here are minted by `mochiko:authoring-technical-requirements`, whose subject D3
  partly kills and whose retire-or-re-scope ruling is another seat's build-wave call — this edit
  neither pre-empts nor blocks it, since it only stops pointing at a class no artifact defines.
  Sweep note for the audit: this site survived my earlier cluster sweep because that grep matched
  `TR-0` (the digit form, as in the `TR-012` example fixed in `artifact-format.md`) and this
  occurrence is the bare slash form `TR-/`. Re-swept with a bare `TR-` pattern across all of
  `plugins/mochiko/templates/` and `plugins/mochiko/schemas/`: this was the only remaining hit.

## [v0.91.0] `produced:` example drops `requirements.md` — the FR→TR artifact dies with the plan stage

- **Disposition:** superseded → `constraints-and-decisions.md` alone, with the row classes named.
  The frontmatter's `produced:` example headed its analysis-round list with `requirements.md`,
  which is no longer authored: D3 kills the per-feature FR→TR layer as a mandatory artifact and
  routes the real technical decisions to the rows this field's surviving member already carries.
  The comment now names those row classes, so a producer disclosing an analysis round still has a
  precise thing to list rather than an artifact that no longer exists.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/plan-stage-utility/record.md` D3 — "the FR→TR layer dies as a mandatory
  artifact … Real technical decisions (D-XXX, C-XXX, IP-XXX) land where they already live —
  `constraints-and-decisions.md` and the store"; `DECISIONS.md` 2026-08-26 row.)
- **Content (superseded, verbatim, one site):** `produced: [requirements.md,
  constraints-and-decisions.md]   # + store-delta NFR rows when touched; design rounds:
  [data-model.md, contracts/api.yaml, quickstart.md]`. Now: `produced:
  [constraints-and-decisions.md]   # the D-XXX/C-XXX/IP-XXX rows this round wrote; + store-delta
  NFR rows when touched; design rounds: [data-model.md, contracts/api.yaml, quickstart.md]`.
- **Kept deliberately:** the store-delta NFR arm added at v0.81.0 survives verbatim — the reason
  it was added (an NFR-touching round would otherwise be invisible in the disclosure) is
  untouched by this ruling. The design-round list is unchanged: all three of its members survive
  the retirement with their homes intact (record D4). Every other field, all eight usage notes,
  the no-self-verdict rule, and the output location are untouched.
- **Consumers assessed:** the `mochiko:technical-analyst` persona fills this template, and the
  dispatching command seeds and collects it; the plan run that used to be one such dispatcher is
  deleted this wave, while specify's use (output location `.mochiko/specs/<feature>/`) is
  unaffected — its rounds never produced `requirements.md`. Note for the audit round, not repaired
  here: the `phase: analysis | design` field still frames a two-phase producer sequence, which is
  a shape question for the `implement.md` rewrite rather than a dead-artifact reference, so it is
  left for a ruling rather than changed unilaterally.

## [v0.81.0] `produced:` example drops `nfrs.md`, gains the store-delta NFR arm — product-architecture-schema D12

- **Disposition:** superseded → the same field with the store named. The frontmatter's
  `produced:` example listed the analysis artifacts a round emits; `nfrs.md` is no longer one.
  The comment now carries "+ store-delta NFR rows when touched", so a producer whose round did
  change an NFR target still has somewhere honest to disclose it — dropping the filename alone
  would have made NFR work invisible in the disclosure.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/product-architecture-schema/record.md` D12; `DECISIONS.md` 2026-08-19).
- **Content (superseded, verbatim):**

  ```
  produced: [requirements.md, constraints-and-decisions.md, nfrs.md]   # design rounds: [data-model.md, contracts/api.yaml, quickstart.md]
  ```
- **Kept deliberately:** the design-rounds arm of the same comment verbatim; the machine-first
  frontmatter shape; the `changed_this_round:` example, which cites **NFR-003 given a numeric
  latency target** — still correct, since D12 preserves NFR-XXX ids and the numeric-target
  grammar, and only the row's home moved.
- **Consumers assessed:** `mochiko:authoring-technical-requirements` owns the grammar this
  template discloses and was re-keyed in the same wave (P4). The sibling
  `feasibility-report-template.md` carried the same listing defect and was fixed in the same pass
  (logged in `strips/review-feasibility.md`). `report-format.md` owns the envelope and names no
  artifact filenames — verified clean. Routed to P4 at the V4 delta pass (B2-extension).

## [v0.46.0] "the shape's producer-authored branch" re-pointed (audit finding 1)
- **Disposition:** superseded → "the dispatching command's producer-authored branch"
- **Tier failed:** n/a — supersession by ruling (ADR `.mochiko/decisions/2026-08-02-doctrine-purge-wave-1.md`; DECISIONS.md 2026-08-02 "Doctrine purge wave 1" row) — audit-caught consumer, fixed at re-grade
- **Content:** rule 6's parenthetical "(the shape's producer-authored branch)" — the shape home was deleted this wave; the ADR's re-point list missed this consumer, caught by the wave audit.
- **Consumers assessed:** the producing seat's briefs unchanged.

## [v0.22.0] Prose disclosure sections → frontmatter fields
- **Disposition:** contracted in place (template rewritten)
- **Tier failed:** consumption evidence (epic F-c part 2)
- **Content:** `## What Was Produced` prose → the `produced:` artifact list (the artifacts themselves are what reviewers grade); `## What Changed This Round` prose → `changed_this_round:` ID-cited list; `## Governance Alignment` prose → `governance_alignment:` one-liner; `## Open Questions` prose → `open_questions:` list; `## Handoff to Review` prose → `handoff:` one-line pointer; the optional `## Artifacts Produced` table (folded into `produced:`). The "Foreground prose… no parser" usage note reversed to machine-first. Preserved: no-self-verdict (no Completion/ready field), phase disclosure semantics, handoff-is-a-pointer-not-a-claim, output location.
