# Quality Checklist

Before finalizing a governance surface set, verify all items below against the shapes in the
`governance-surfaces` schema — `mochiko-cli template governance-surfaces` for the shapes,
`mochiko-cli template governance-surfaces --check` for the mirror-checklist view; when the binary
is absent, Read `plugins/mochiko/schemas/governance-surfaces.yaml` raw. The Structure sections are
**module-parameterized**: read the synthesis's module selections first, then check core + exactly
the selected modules.

## Principle Quality

- [ ] Every principle's ledger record has an Enforcement section
- [ ] Every principle's ledger record has a Testability section
- [ ] Every principle's ledger record has a Rationale section
- [ ] Every principle carries a Trace stamp (`**Trace**: GI-XXX (…)` in the ledger; the region line's trace comment where the home is the region)
- [ ] All MUST statements have enforcement mechanisms
- [ ] All quantifiable criteria have specific thresholds
- [ ] No vague language without measurable criteria

## Traceability (cross-check against `.mochiko/memory/governance-intent.md` — deterministic)

- [ ] Every principle's Trace GI-ID **exists** in governance-intent.md
- [ ] Every trace points at a **principle-bearing** element (floor-asserted / deck-kept / minted / compliance-module obligation — not a waiver, exclusion, or template-module row)
- [ ] Every principle-bearing element in governance-intent.md is **realized** as a principle — or appears in the producer's flagged-proposals list
- [ ] No two principles claim the same GI-ID
- [ ] Waiver records match the synthesis's waiver elements one-for-one (each with its GI-ID)
- [ ] Attached template-module sections match the synthesis's module selections one-for-one — nothing extra, nothing missing (compliance modules are checked in Floor & Module Accounting below)

> Semantic fidelity of a stamped trace (does the principle's *content* faithfully realize the
> element's *intent*?) is judgment-grade — flag suspected mismatches in the fix list, but the
> deterministic checks above are the PASS/FAIL surface. Fidelity is guarded upstream by the
> synthesis-confirmation checkpoint and downstream by the acceptance gate's trace summary.

## Floor & Module Accounting (all modes)

- [ ] Governance Floor section present in the ledger: production floor + declared depth level (low/high) asserted · attached compliance modules with strata (or "none") · fact-profile trace (GI-001)
- [ ] Every Essential Floor category (Security, Testing, Error Handling, Observability) has a principle at the declared level's row of the floor card **or a recorded waiver** — neither is a FAIL
- [ ] Every waiver record carries: standard, justification, revisit trigger or "permanent (D4.1 pending)", trace
- [ ] **No waiver names a legal-mandate module obligation** (D4.2 — strata per [COMPLIANCE-MODULES.md](../../authoring-constitution/references/COMPLIANCE-MODULES.md)); one that does is a FAIL
- [ ] Attached compliance modules match the synthesis's fact profile one-for-one — every triggered module attached, none attached without a recorded trigger fact
- [ ] Module obligations are additive over the floor — no attached-module content loosens a floor principle
- [ ] Coverage thresholds and gate strictness sit at the declared level's row (the low row or the high row) of [the floor card](../../authoring-constitution/references/catalog/universal-floor.md) or carry a session override recorded in the synthesis
- [ ] No Quality-Gate row for a waived category (the waiver record covers the absence)

## Structure Quality — universal core (the surface set)

- [ ] CLAUDE.md governance region present between `<!-- mochiko:governance:begin -->` / `<!-- mochiko:governance:end -->`; no setup-owned content outside the markers
- [ ] Ratified stamp line: version · ratified date · production floor + declared depth level · attached modules (or "none")
- [ ] Principle index: one line per principle; index → home → ledger closes both ways
- [ ] Universal principles as short imperative RFC 2119 lines, floor principles first, marked `(NON-NEGOTIABLE)`
- [ ] Technology-stack lines with actual mandated choices
- [ ] Quality-gates summary with actual commands
- [ ] Governance-operations block: ledger pointer · amend route (fact-profile changes — module attach/detach — and un-waives are governance events)
- [ ] Preserved carve-outs survived this regeneration: the `mochiko:domain-registry` block and the `mochiko:output-style` pair (the region's switch line + its Shape-5 rules file). On an amend, each still carries the values that were there before the run — a carve-out regenerated back to its defaults silently reverts a user's ruling, so it is a FAIL, not a cosmetic diff
- [ ] Ledger complete per Shape 3: Governance Floor header · Waivers · Amendment policy · Exception registry · Three-Part records keyed by GI-ID · amendment log (version matching the region stamp)
- [ ] Trace summary manifest present (Shape 4): one row per principle-bearing GI element

## Structure Quality — selected modules

For **each template module the synthesis selects**, run the validator checklist fragment — for
`evolution-notes` / `layer-rules` / `release-gates`, from `mochiko-cli template <module> --check`
(or Read `plugins/mochiko/schemas/<module>.yaml` raw when the binary is absent); for
`knowledge-management`, embedded at the bottom of `templates/constitution-modules/knowledge-management.md`:

- [ ] `layer-rules` fragment (if selected)
- [ ] `release-gates` fragment (if selected)
- [ ] `evolution-notes` fragment (if selected — always selected in brownfield mode)
- [ ] `knowledge-management` fragment (if selected — adopted whole; never check it against a synthesis that records a decline)

**Cross-module check** (no single-module producer home — its truth depends on the *other* attached compliance modules, not on any one module's content; relocated verbatim from the `release-gates` module fragment 4 at the v0.77.0 template-schema ratchet, since it is not encoded in `release-gates.yaml` and the source `release-gates.md` retires):

- [ ] Gates consistent with the attached compliance modules (an attached module names its audit-evidence gate)

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

- [ ] Version follows semantic versioning (floor-level change / module attach or detach = MAJOR; new principle or waiver change = MINOR; clarification = PATCH)
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
