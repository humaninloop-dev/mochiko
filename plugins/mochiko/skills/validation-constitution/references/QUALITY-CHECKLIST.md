# Quality Checklist

Before finalizing a governance surface set, verify all items below against the shapes in the
`governance-surfaces` schema — `mochiko-cli template governance-surfaces` for the shapes,
`mochiko-cli template governance-surfaces --check` for the mirror-checklist view. The Structure
sections are **module-parameterized**: read the synthesis's module selections first, then check
core + exactly the selected modules.

## Principle Quality

- [ ] Every principle's ledger record has an Enforcement section
- [ ] Every principle's ledger record has a Testability section
- [ ] Every principle's ledger record has a Rationale section
- [ ] Every principle carries a Trace stamp (`**Trace**: GI-XXX (…)` in the ledger; the region line's trace comment where the home is the region)
- [ ] All MUST statements have enforcement mechanisms
- [ ] All quantifiable criteria have specific thresholds
- [ ] No vague language without measurable criteria

## No Product Instance (all modes)

Governance holds the rule, never the product's instance of it: a principle passes only if it can be
checked from the code, the pipeline, or the stack without knowing what the product does for its
users.

- [ ] No surface restates a product's instance of a rule — its boundary list, its data's name, its behaviour toward its users; where a principle needs the instance, it points at the instance's home (GI-017: point, never restate)
- [ ] No elicited intent that failed the test is authored as a principle or flagged as a proposal — it sits in the synthesis's Handed off list under Deliberate exclusions

Worked cases — a principle as elicited, and what the set may hold of it:

| Elicited | The set holds |
|---|---|
| Dependencies pinned and justified | the whole principle — a stack rule |
| UI components come from a named component library | the whole principle — a stack choice |
| A named engine MUST be reached only through a port | the rule (external systems through ports, at the scope the project ratified); the engine's name leaves — it is an architecture-store row, and choosing it was a product ruling |
| The UI never touches the named store or engine | the layer rule (the UI layer reaches no persistence or external system directly); the store and engine names leave |
| Validate input at every boundary, followed by the product's boundary list | the rule; the boundary list leaves — the architecture store holds it |
| The product's named data is never silently lost | the floor's no-silent-data-corruption rule; the data's name leaves |
| A message envelope between two product components carries no user-facing sentences | nothing — a product architecture boundary |
| The assistant proposes and the user rules | nothing — product behaviour |

A failing principle is fixed by cutting the instance and keeping the rule, or — when nothing
engineering is left — by moving the intent to the Handed off list.

## Traceability (cross-check against `.mochiko/memory/governance-intent.md` — deterministic)

- [ ] Every principle's Trace GI-ID **exists** in governance-intent.md
- [ ] Every trace points at a **principle-bearing** element (floor-asserted / deck-kept / minted — not a waiver, exclusion, or template-module row)
- [ ] Every principle-bearing element in governance-intent.md is **realized** as a principle — or appears in the producer's flagged-proposals list
- [ ] No two principles claim the same GI-ID
- [ ] Waiver records match the synthesis's waiver elements one-for-one (each with its GI-ID)
- [ ] Attached template-module sections match the synthesis's module selections one-for-one — nothing extra, nothing missing
> Semantic fidelity of a stamped trace (does the principle's *content* faithfully realize the
> element's *intent*?) is judgment-grade — flag suspected mismatches in the fix list, but the
> deterministic checks above are the PASS/FAIL surface. Fidelity is guarded upstream by the
> synthesis-confirmation checkpoint and downstream by the acceptance gate's trace summary.

## Floor Accounting (all modes)

- [ ] Governance Floor section present in the ledger: production floor + declared depth level (low/high) asserted
- [ ] Every Essential Floor category (Security, Testing, Error Handling, Observability) has a principle at the declared level's row of the floor card **or a recorded waiver** — neither is a FAIL
- [ ] Every waiver record carries: standard, justification, revisit trigger or "permanent (D4.1 pending)", trace
- [ ] Coverage thresholds and gate strictness sit at the declared level's row (the low row or the high row) of [the floor card](../../authoring-constitution/references/catalog/universal-floor.md) or carry a session override recorded in the synthesis
- [ ] No Quality-Gate row for a waived category (the waiver record covers the absence)

## Structure Quality — universal core (the surface set)

- [ ] CLAUDE.md governance region present between `<!-- mochiko:governance:begin -->` / `<!-- mochiko:governance:end -->`; no setup-owned content outside the markers
- [ ] Ratified stamp line: version · ratified date · production floor + declared depth level
- [ ] Principle index: one line per principle; index → home → ledger closes both ways
- [ ] Universal principles as short imperative RFC 2119 lines, floor principles first, marked `(NON-NEGOTIABLE)`
- [ ] Technology-stack lines with actual mandated choices
- [ ] Quality-gates summary with actual commands
- [ ] Governance-operations block: ledger pointer · amend route (the governance events the template's amendment policy lists)
- [ ] Preserved carve-outs survived this regeneration: the `mochiko:domain-registry` block and the `mochiko:output-style` pair (the region's switch line + its Shape-5 rules file). On an amend, each still carries the values that were there before the run — a carve-out regenerated back to its defaults silently reverts a user's ruling, so it is a FAIL, not a cosmetic diff
- [ ] Ledger complete per Shape 3: Governance Floor header · Waivers · Amendment policy · Exception registry · Three-Part records keyed by GI-ID · amendment log (version matching the region stamp)
- [ ] Trace summary manifest present (Shape 4): one row per principle-bearing GI element

## Structure Quality — selected modules

For **each template module the synthesis selects**, run the validator checklist fragment embedded
at the bottom of that module's file in `templates/constitution-modules/`:

- [ ] `layer-rules` fragment (if selected)
- [ ] `release-gates` fragment (if selected)
- [ ] `evolution-notes` fragment (if selected — always selected in brownfield mode)
- [ ] `knowledge-management` fragment (if selected — adopted whole; never check it against a synthesis that records a decline)

## Rules-File Scope & Delivery (when any rules file exists)

- [ ] Each rules file's `paths` globs cover every path whose code can violate the concern — per-layer violation test against the kept architecture card, not just the mechanism's home layer
- [ ] The governance region carries the standing new-file read line (rules inject on Read, not Write — read the matching rules file, or read back the created file, before creating a file under a scoped path)

## No Placeholders Rule

- [ ] Technology stack has NO `[PLACEHOLDER]` syntax - all actual tool names
- [ ] Quality gates have NO `[COMMAND]` placeholders - all actual commands
- [ ] Coverage thresholds are numeric (e.g., "≥80%", NOT "[THRESHOLD]%")
- [ ] Security tools are named (e.g., "Trivy + Snyk", NOT "[SECURITY_COMMAND]")
- [ ] Test commands are complete (e.g., "`pytest --cov`", NOT "`[TEST_COMMAND]`")
- [ ] Trace stamps are real IDs (e.g., "GI-007", NOT "GI-XXX")

## Governance Quality

- [ ] Version follows semantic versioning (floor-level change = MAJOR; new principle or waiver change = MINOR; clarification = PATCH)
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
