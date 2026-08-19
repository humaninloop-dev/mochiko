# Strip notes — `templates/techanalyst-report-template.md`

Entry formats: `strips/README.md`. Wave context: workflow-token-reduction wave 1 (rulings
ratified 2026-07-23: producer disclosures machine-first).

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
