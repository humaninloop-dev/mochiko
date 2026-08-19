# Strip notes — `agents/technical-analyst`

Entry formats: `strips/README.md`. Wave context: the plan cluster wave (v0.15.0). Plan-cluster-only
agent (the analysis+design producer, mounted on `plan`) — strips ruled in-wave (single consumer).

## [v0.81.0] Skill-pointer artifact list: `nfrs.md` → the store's concern rows — product-architecture-schema D12

- **Disposition:** superseded → the architecture store. The persona's pointer to
  `mochiko:authoring-technical-requirements` named the three analysis artifacts by filename; D12
  kills `nfrs.md` as a file and homes NFR-XXX on the store's concern rows. Rather than deleting
  the filename and leaving a silent gap, the line now **states the new home**, so an analyst
  reading the persona learns where an NFR goes rather than merely not being told.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/product-architecture-schema/record.md` D12; `DECISIONS.md` 2026-08-19).
- **Content (superseded, verbatim):**

  ```
  - **`mochiko:authoring-technical-requirements`** — the analysis artifacts (`requirements.md`,
    `constraints-and-decisions.md`, `nfrs.md`) and their TR/C/NFR/IP traceability.
  ```
- **Kept deliberately:** the persona's whole shape — the reach-for-the-skill-whose-artifact-is-in-
  front-of-you framing, the scope-lives-in-the-skill rule, all four skill pointers, and the
  **TR/C/NFR/IP traceability** clause: D12 moves the NFR's path, never the trace chain, so the
  traceability obligation is unchanged. The persona's other NFR mentions (the load-testing
  motivation at :36, the constraint-to-infrastructure tracing at :99) are home-agnostic and were
  verified correct as written, not edited.
- **Consumers assessed:** the router's `technical-analyst` row carried the same NFRs limb and was
  re-keyed in the same pass (P4). The agent's `description:` names artifacts generically
  ("entity models, API contracts, technology decisions") and needed no edit — verified, and its
  char count is unchanged, so no ledger movement. `plan.md` (P2) dispatches this seat.
  Routed to P4 at the V4 delta pass as an unowned ripple (B2-extension).

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

## [v0.64.0] Frontmatter `description:` examples stripped → prose-only agent description
- **Disposition:** superseded → prose-only agent description (Wave 2 editorial extension of the agents-arm ruling); the `<example>` blocks were removed from the frontmatter `description:` block scalar, the prose framing (routing content) kept verbatim.
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail agents-arm user ruling (b) 2026-08-10, extended to the untested agents by the Wave 2 user ruling; `DECISIONS.md` 2026-08-11 build row Wave 2 residual; `report/final-verdict.md` — 0 route misses over 20+ staffings).
- **Content:** faithfully compressed. **4 `<example>` blocks removed** from the `description:` value:
  1. Business spec needs full technical breakdown and design — analysis-to-design translation in one workflow.
  2. "System should be fast" with no measurable targets — translation into measurable NFRs.
  3. External services without integration details — integration mapping with failure modes.
  4. Requirements locked, data model + contracts needed — design artifacts built on the analysis.

  Description parsed-value char delta: **2,259 → 401** (chars of the parsed block-scalar value; block-scalar parse, not `wc -c` bytes). Verbatim removed text survives in git history of `plugins/mochiko/agents/technical-analyst.md` (pre-v0.64.0).
- **Kept deliberately:** the prose framing of the `description:` (the routing content that staffs the agent — "Senior systems engineer who bridges the gap between business specifications and technical implementation through requirements analysis AND concrete design decisions. Decomposes business intent into precise, traceable technical requirements, then transforms those requirements into entity models, API contracts, and technology decisions. Authors the technical artifacts; does not grade its own output.") — and the entire agent body, byte-for-byte untouched (verified against git HEAD).
- **Consumers assessed:** grep of `plugins/mochiko/commands/` and `plugins/mochiko/skills/` for `technical-analyst`: referenced only by the router `plugins/mochiko/skills/mochiko/SKILL.md`; no command references the agent by name. Routing/staffing contract intact — the agent name and the description's prose framing are unchanged; only the illustrative `<example>` blocks were removed.
- **Standing watch:** an F-X1-class route miss on the untested agents re-opens ruling (b).
- **KEPT reconciliation:** the prior [v0.15.0] KEPT entry protects the "Quality Standards" persona *body* enumeration, and the [v0.15.0] strip relocated a "Skills Available" *body* bullet. Neither touches the frontmatter `description:` value or any `<example>` block. No overlap with this edit.

## [v0.15.0] "Skills Available" scope duplication
- **Disposition:** scope enumeration relocated → the four skills themselves
  (`mochiko:authoring-technical-requirements`, `mochiko:patterns-technical-decisions`,
  `mochiko:patterns-entity-modeling`, `mochiko:patterns-api-contracts`), which already single-source
  it — each skill's `description:` carries its scope; the skill **names + a one-line reach-for-it hint
  are kept** (they carry the team-form function — teammates ignore `skills:` frontmatter, so the
  in-body names are how a spawned teammate learns its skills)
- **Tier failed:** 1 (a second home for each skill's scope — the bullets restated what each skill's
  `description:` already single-sources)
- **Content:** the four full scope descriptions in the "Skills Available" bullets — "Guidance on
  writing technical requirements, constraints, non-functional requirements, and data sensitivity
  classifications with proper traceability and measurability standards.", "Evaluate technology
  alternatives and document decisions in ADR format with criteria weighting, trade-offs, and
  consequences.", "DDD-style entity extraction including attributes, relationships, state machines,
  and validation rules.", and "RESTful API design with endpoint mapping, schema definition, error
  handling, and OpenAPI specification."
- **Consumers assessed:** plan only (technical-analyst is plan-only). One instance of the 7-agent
  library-wide "Skills Available" pattern; ruling in-wave is D9-authorized (single consumer) and noted
  so the escalation lands consistently across all seven agent personas — the `devils-advocate` instance
  is raised for the ≥3-consumer escalation, while `task-architect` (v0.14.0, 2 consumers) and
  `principal-architect` (this wave, 2 consumers) were ruled in-wave under the 2-consumer allowance.

## [v0.15.0] KEPT: the "Quality Standards" persona enumeration (Traceable / Measurable / … / Realistic)
- **Tier-2 evidence:** the agent↔skill composition axis blesses persona (what the agent cares about)
  in the agent; "Quality Standards" is the analyst's *taste*, explicitly disclaimed — "this is the
  *taste* you bring, not the format spec. The concrete formats and procedures live in your skills,
  which are the single source of truth" — and closed with the single-source reference to the four
  skills (the IP-XXX format, the ≥2-alternatives ADR evaluation, the entity schema, the OpenAPI
  structure "live in [the four skills] … Consult them there rather than a copy in this persona"). Persona
  altitude, not a behavior-driving procedure restatement — distinct from the "Skills Available" strip
  above, which was a scope catalog pointing at the very homes it copied. Matches the `task-architect`
  "What You Produce" persona keep.
