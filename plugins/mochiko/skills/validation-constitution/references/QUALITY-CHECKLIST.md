# Quality Checklist

Before finalizing a constitution, verify all items below. The Structure section is
**module-parameterized**: read the synthesis's module selections first, then check core + exactly
the selected modules.

## Principle Quality

- [ ] Every principle has Enforcement section
- [ ] Every principle has Testability section
- [ ] Every principle has Rationale section
- [ ] Every principle has a Trace stamp (`**Trace**: GI-XXX (…)`)
- [ ] All MUST statements have enforcement mechanisms
- [ ] All quantifiable criteria have specific thresholds
- [ ] No vague language without measurable criteria

## Traceability (cross-check against `.mochiko/memory/governance-intent.md` — deterministic)

- [ ] Every principle's Trace GI-ID **exists** in governance-intent.md
- [ ] Every trace points at a **principle-bearing** element (deck-kept / minted / floor-preset — not a waiver, exclusion, or module row)
- [ ] Every principle-bearing element in governance-intent.md is **realized** as a principle — or appears in the producer's flagged-proposals list
- [ ] No two principles claim the same GI-ID
- [ ] Waiver records match the synthesis's waiver elements one-for-one (each with its GI-ID)
- [ ] Attached module sections match the synthesis's module selections one-for-one — nothing extra, nothing missing

> Semantic fidelity of a stamped trace (does the principle's *content* faithfully realize the
> element's *intent*?) is judgment-grade — flag suspected mismatches in the fix list, but the
> deterministic checks above are the PASS/FAIL surface. Fidelity is guarded upstream by the
> synthesis-confirmation checkpoint and downstream by the acceptance gate's trace summary.

## Tier & Floor Accounting (all modes)

- [ ] Governance Tier section present: named tier + graduation path + trace stamp
- [ ] Every Essential Floor category (Security, Testing, Error Handling, Observability) has a principle **or a recorded waiver** — neither is a FAIL
- [ ] Every waiver record carries: floor category, waiving tier, revisit trigger, trace
- [ ] No waiver at a tier whose posture forbids it (`production`/`regulated` — see [the floor cards](../../authoring-constitution/references/catalog/universal-floor.md))
- [ ] Coverage thresholds and gate strictness are consistent with the declared tier (or carry a session override in the synthesis)
- [ ] No Quality-Gate row for a waived category (the waiver record covers the absence)

## Structure Quality — universal core

- [ ] SYNC IMPACT REPORT present as HTML comment
- [ ] Overview section with project description
- [ ] Governance Tier section (incl. Waivers table or "None.")
- [ ] Core Principles numbered with Roman numerals
- [ ] Technology Stack table complete with rationale
- [ ] Quality Gates table with measurement commands
- [ ] Governance section with amendment process (incl. tier-bump/un-waive routed through amend)
- [ ] CLAUDE.md Sync Mandate with mapping table (incl. Governance Tier row + a row per attached module the sync mandates)
- [ ] Version footer with dates in ISO format

## Structure Quality — selected modules

For **each module the synthesis selects**, run the validator checklist fragment embedded at the
bottom of that module's file in `templates/constitution-modules/`:

- [ ] `layer-rules` fragment (if selected)
- [ ] `release-gates` fragment (if selected)
- [ ] `evolution-notes` fragment (if selected — always selected in brownfield mode)

## No Placeholders Rule

- [ ] Technology Stack has NO `[PLACEHOLDER]` syntax - all actual tool names
- [ ] Quality Gates has NO `[COMMAND]` placeholders - all actual commands
- [ ] Coverage thresholds are numeric (e.g., "≥80%", NOT "[THRESHOLD]%")
- [ ] Security tools are named (e.g., "Trivy + Snyk", NOT "[SECURITY_COMMAND]")
- [ ] Test commands are complete (e.g., "`pytest --cov`", NOT "`[TEST_COMMAND]`")
- [ ] Trace stamps are real IDs (e.g., "GI-007", NOT "GI-XXX")

## Governance Quality

- [ ] Version follows semantic versioning (tier change = MAJOR; waiver change = MINOR)
- [ ] Amendment process is actionable
- [ ] Exception registry format defined
- [ ] Compliance review expectations set

## Brownfield-Specific (if applicable)

- [ ] All four Essential Floor categories have principles **or recorded waivers**
- [ ] Existing good patterns identified and codified
- [ ] Gap references included where codebase lacks capability (and no gap for waived categories)
- [ ] Technology stack matches codebase analysis
- [ ] Quality gates reflect current + target state
- [ ] Evolution Notes present (via its module fragment above) and documents brownfield context
