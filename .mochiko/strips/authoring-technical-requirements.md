# Strip notes — `skills/authoring-technical-requirements`

Entry formats: `strips/README.md`. Plan-cluster-only skill (the technical-requirements authoring layer,
mounted on the `technical-analyst` producer — sole consumers: `agents/technical-analyst.md` +
`commands/plan.md`, verified by grep). Single-consumer; in-wave rulings D9-authorized. Skill body + two
references (`ARTIFACT-TEMPLATES.md`, `TRACEABILITY-PATTERNS.md`) touched this wave.

## [v0.19.0] Wave context — token-reduction build, wave 1 (D4 reference-by-ID)

Token-reduction epic **D4** (reference-by-ID; generalizes `authoring-slices` invariant 6 — cite IDs,
one-line gloss max, never re-quote). **Additions (not strips — logged for provenance):** SKILL.md
Traceability Rules gained the explicit "cite IDs, never re-quote text" rule and named the
**Traceability Summary** table (`references/ARTIFACT-TEMPLATES.md`) as the coverage surface reviewers
grade against; the Quality Checklist gained two size / ID-cite items; `references/TRACEABILITY-PATTERNS.md`
homed the global "the link is the ID, not the quoted text" rule beside The Traceability Web. The two
strips below are quoted-text **examples** de-modeled to the ID-cite form the rule now teaches. No
`command-shape.md` touch this wave → no cross-command re-audit.

## [v0.19.0] De-model: quoted-FR decomposition example (`references/ARTIFACT-TEMPLATES.md`)
- **Disposition:** de-modeled in place → ID-cite form (the example modeled the re-quote pattern D4 forbids)
- **Tier:** 1-adjacent (altitude — the example *modeled* re-quoting upstream text; D4 makes the ID the link)
- **Content (verbatim, replaced):** `**Business FR:** "Users must be able to sign in to their account"
  (FR-001)` → now: `**Business FR:** FR-001 (users sign in) — cited by ID with a one-line gloss, never
  the re-quoted spec sentence`.
- **Consumers assessed:** plan-only (technical-analyst producer; single-consumer, in-wave per D9).

## [v0.19.0] De-model: quoted glosses in the traceability-chain diagrams (`references/TRACEABILITY-PATTERNS.md`)
- **Disposition:** de-modeled in place → ID-cite form (minimal glosses, no reproduced source text)
- **Tier:** 1-adjacent (altitude — chain-head glosses reproduced upstream text; sibling lines were already ID + short gloss)
- **Content (verbatim, replaced):**
  - Full Traceability Chain head: `FR-001 (business: "users can sign in")` → `FR-001 (business: user sign-in)` (quotes dropped)
  - Constraint Impact Chain head: `C-002 (regulatory: GDPR Art. 17 right to erasure)` → `C-002 (regulatory: right to erasure)` (source-text gloss trimmed to the concept)
- **Consumers assessed:** plan-only (technical-analyst producer; single-consumer, in-wave per D9).
