# Strip notes — `templates/governance-surfaces-template.md`

Entry formats: `strips/README.md`.

**Wave context (v0.65.0 — the production-floor adaptive-depth landing).** The asserted production
floor gains a **two-row `low`/`high` depth level** (one project-wide dial, user-declared, one-way
`low`→`high`). Ruling: `production-floor-adaptive-depth`, ratified 2026-08-11, D1–D8 —
`.mochiko/brainstorms/production-floor-adaptive-depth/record.md`; `DECISIONS.md` 2026-08-11
adaptive-depth row. On this template the two entries below carry **level state** into the region
stamp (Shape 1) and the ledger's Governance-Floor line (Shape 3) — both are identity-carrying,
`DECISIONS.md`-traceable stamps, so they are supersessions **by ruling**. The rest of this
template's depth work is **pure addition** and takes no strip entry: the semver MAJOR list gains
the `low`→`high` flip event, and the amendment-policy Route line gains the flip as a governance
event recorded via amendment-log rows (no new ledger structure).

## [v0.74.0] Template retired — superseded by schema-based template guidance (D1/D3/D8)
- **Disposition:** superseded → plugins/mochiko/schemas/governance-surfaces.yaml + mochiko-cli template governance-surfaces
- **Tier failed:** n/a — supersession by ruling (schema-based-template-guidance D1/D3/D8; record `.mochiko/brainstorms/schema-based-template-guidance/record.md`; `DECISIONS.md` "Template-schema CLI ruled")
- **Content (superseded template, full verbatim below):**

````markdown
<!--
GOVERNANCE SURFACES TEMPLATE — the canonical shapes of the dissolved constitution
=================================================================================
There is NO constitution.md. Governance is a SET, authored by `authoring-constitution` and graded as one deliverable
by `validation-constitution`:

  1. A marked governance region in CLAUDE.md          (always-on; short-form only)
  2. `paths`-scoped rules files under .claude/rules/mochiko/   (scope-on-touch)
  3. Skill pointers                                    (procedure lives in skills)
  4. The governance ledger at .mochiko/memory/governance-ledger.md  (setup/amend + validator only)
  5. The trace summary manifest                        (the validator's grading surface)

Ownership (D8): the region between the markers — and everything under .claude/rules/mochiko/ and
the ledger — is setup-owned and idempotently REGENERATED on re-runs and amends. Content outside
the markers is user territory: NEVER touched. TWO carve-outs, both preserved verbatim across
regenerations: (1) a domain-dependency registry block (`mochiko:domain-registry` markers) inside
a layer-scoped rules file — implement-time additions live there
(`authoring-constitution/references/DOMAIN-DEPENDENCIES.md`); (2) the output-style switch line
(`mochiko:output-style` markers) in Governance operations — setup writes it default-on ONCE, and
every later regeneration keeps whatever per-surface values the user set. Regenerating it back to
the defaults silently reverts a user's ruling, which is the failure the carve-out exists to
prevent (style home: `templates/output-style.md`). HTML comments in CLAUDE.md are
stripped before context injection (doc-confirmed) — trace stamps there are context-free; comment
handling in rules files is undocumented, so the ledger is always the canonical metadata record.
-->

# Shape 1 — the CLAUDE.md governance region

Regenerated in place between the markers. Everything short-form: one line per entry, detail in
the ledger, module detail behind pointers.

```markdown
<!-- mochiko:governance:begin -->
## Governance

**Ratified:** v[X.Y.Z] · [YYYY-MM-DD] · production floor · depth: [low | high] · modules: [attached compliance modules, or "none"] <!-- GI-001 (fact profile) · GI-0XX (depth level) -->

### Principles

<!-- Universal principles: the operative line IS the governance. Scope-bound and
     procedure-shaped principles: the line is an index entry pointing at the home. -->
- [Imperative universal principle, RFC 2119, one line] [(NON-NEGOTIABLE) for floor principles] <!-- GI-XXX -->
- [Imperative universal principle] <!-- GI-XXX -->
- [Concern name] — see `.claude/rules/mochiko/[file].md` <!-- GI-XXX -->
- [Procedure name] — follow skill `[skill-name]` when [trigger] <!-- GI-XXX -->

### Technology stack

- [Language/runtime + version] · [framework] · [key mandated choices, one line each] <!-- GI-XXX -->

### Quality gates

- `[actual command]` MUST pass before merge <!-- GI-XXX -->
- Coverage ≥ [floor card's coverage threshold, session-overridable]% on new code (`[actual command]`) <!-- GI-XXX -->

### Governance operations

- Ledger (waivers · amendment policy · exceptions · Three-Part metadata): `.mochiko/memory/governance-ledger.md`
- Amend via `/mochiko:setup` (fact-profile changes — module attach/detach — and un-waives are governance events)
<!-- mochiko:output-style:begin -->
- Writing style — conversation: `full` · reports: `ultra` *(internal agent hand-offs)* · documents: `full`. Terse and plain-English by default. Set any of the three to `off`, `lite`, `full`, or `ultra` and your choice is kept when this section is regenerated; "normal mode" turns it off for one session.
<!-- mochiko:output-style:end -->
[- Path-scoped rules inject on **Read**, not Write (observed behavior, kinako dogfood 2026-07-19) — before creating a new file under [the actual scoped paths, e.g. `src/`, `tests/`], read the matching `.claude/rules/mochiko/` file or read back the file you created <!-- standing line: emit whenever the set includes any rules file -->]
[- Operating docs (knowledge-management module): sessions in `.mochiko/brainstorms/` + `index.md`; rulings land in `DECISIONS.md`; open threads in `BACKLOG.md`; direction in `ROADMAP.md`; landing ritual + invariants at `.mochiko/memory/knowledge-management.md`; groom: `mochiko:grooming-operating-docs` <!-- GI-XXX -->]
[- Release gates: [one-line summary] — detail in the ledger <!-- GI-XXX -->]
<!-- mochiko:governance:end -->
```

# Shape 2 — a scope-bound rules file (`.claude/rules/mochiko/<concern>.md`)

One concern per file. The `paths` globs must be honest to the concern — and honest cuts both
ways: a glob that matches everything is a universal principle wearing a costume (those belong in
the region), while globs that stop at the mechanism's home layer silently exempt code that can
violate the concern. Cover **every path whose code can violate the rule** — including layers that
orchestrate the governed operation through ports/interfaces, not just the layer that implements
it (kinako dogfood 2026-07-19: storage invariants scoped to domain+infrastructure missed the
application layer's use cases persisting through ports).

```markdown
---
paths:
  - "[honest glob, e.g. src/api/**/*.py]"
  - "[second glob if the concern truly spans]"
---

# [Concern name] <!-- GI-XXX -->

- [Operative rule, RFC 2119] 
- [Operative rule]
- [Operative rule]

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-XXX.
```

# Shape 3 — the governance ledger (`.mochiko/memory/governance-ledger.md`)

Read by setup/amend runs and the validator only — never force-loaded into working sessions.

```markdown
# Governance Ledger

**Governance Floor:** production (asserted) · **Depth level:** [low | high] (user-declared, one-way; `high` terminal) · **Modules:** [attached compliance modules with strata, or "none"] · **Trace:** GI-001 (fact profile) · GI-0XX (depth level)
**Version:** [X.Y.Z] (must match the region stamp)

## Waivers

Any asserted standard, with a recorded justification (D4); permanent pending the D4.1 revisit.
**Legal-mandate module obligations are unwaivable (D4.2)** — a waiver row naming one is a
validator FAIL.

| Standard | Justification | Revisit trigger (optional) | Trace |
|----------|---------------|----------------------------|-------|
| [floor category / card / non-legal module obligation, or "None."] | [recorded reason] | [or "permanent (D4.1 pending)"] | GI-XXX |

## Amendment policy

- Route: `/mochiko:setup` amend mode; fact-profile changes (module attach/detach), un-waives,
  and the depth-level flip (a `high`-mode rerun) are governance events — the declaration and any
  later flip are recorded as amendment-log rows, no new ledger structure.
- Semver: MAJOR — principle removal / incompatible redefinition / floor-level change /
  depth-level flip (`low`→`high`) / module attach or detach · MINOR — new principle or waiver
  change · PATCH — clarification.
- Approvers: [from the synthesis's team reality]

## Exception registry

| Exception | Principle (GI-ID) | Granted | Expires/revisit |
|-----------|-------------------|---------|-----------------|
| [none yet] | | | |

[## Domain-dependency policy (only when `layer-rules` is adopted)]
[Qualification criteria (domain-relevance filters first, then ubiquity) · trust-signal
hierarchy: `authoring-constitution/references/DOMAIN-DEPENDENCIES.md` · add-process + gate
(human ruling before registry entry; `domain_deps_added` cycle-report disclosure — the
checkpoint never auto-approves while it is non-empty). The list itself lives ONLY in the domain
rules file's `mochiko:domain-registry` block — preserved across regenerations, no ledger copy.]

## Principles (Three-Part metadata, keyed by GI-ID)

### GI-XXX — [Principle Name] · home: [CLAUDE.md | rules/mochiko/<file>.md | skill:<name>]

**Enforcement**:
- [How compliance is verified — specific commands or processes]

**Testability**:
- Pass: [criterion] · Fail: [criterion]

**Rationale**: [Failure mode prevented; success enabled.]

**Trace**: GI-XXX (floor-asserted: CARD-ID | deck-kept: CARD-ID | minted | module: <module>-<obligation>)

[## Evolution notes (brownfield — the evolution-notes module's ledger section)]
[Floor status table · gap references · session confrontation rulings.]

## Amendment log

| Version | Date | Change | GI delta |
|---------|------|--------|----------|
| [X.Y.Z] | [date] | ratified | [elements] |
```

# Shape 4 — the trace summary manifest

Emitted by the producer with every authoring round; presented at the acceptance gate; the
validator's grading surface. One row per principle-bearing GI element:

```markdown
## Trace summary

| GI-ID | Principle | Source | Primary home | Companions present |
|-------|-----------|--------|--------------|--------------------|
| GI-003 | [name] | deck-kept: CARD-ID | CLAUDE.md region line | index ✓ · ledger ✓ |
| GI-004 | [name] | minted | rules/mochiko/api.md | index ✓ · ledger ✓ |
| GI-005 | [name] | floor-asserted: CARD-ID | skill:mochiko:executing-tdd-cycle | index ✓ · ledger ✓ |

Flagged proposals: [none | list — each awaiting the user's ruling at acceptance]
Waivers: [none | GI-IDs]
```

# Shape 5 — the output-style rules file (`.claude/rules/mochiko/output-style.md`)

A Shape-2 file with fixed content, emitted every run rather than routed from a principle, scoped
over the paths where pipeline deliverables are authored. It is **edit-time reinforcement, never
the carrier**:

- **For:** a producer *editing* an existing deliverable, which reads the file at touch time and
  writes in the right register.
- **NOT for:** reaching file **creates**. `paths` rules inject on **Read, not Write** — the same
  observed behavior the region's standing new-file line records — so a file written from scratch
  never triggers this one. The always-loaded governance region is what reaches creates; this file
  is the second delivery, never the only one. (Anyone re-opening that question: it was ruled, not
  overlooked.)
- **Create-or-join:** the `.claude/rules/mochiko/` scaffold is shared. Whichever concern lands
  first creates the directory and the region's index line; every later concern joins it, one file
  per concern.

```markdown
---
paths:
  - ".mochiko/specs/**"
---

# Writing style

- Documents here are written `full` by default — dense, articles droppable, fragments fine. The
  **Writing style** line in `CLAUDE.md`'s Governance section is the authority whenever it differs.
- Never compress: IDs, identifiers, paths, commands, contract clauses, numeric targets and error
  strings — verbatim.
- Stop compressing wherever it would leave a requirement, criterion or constraint ambiguous. A
  human signs these off, so plain English beats terse every time the two pull apart.
```

Preserved like the switch line: written once, refreshed only to track the region's Writing-style
values, and any line the user adds to it survives every regeneration. Full rules ship with
mochiko (the `output-style` template; deliverable envelope `artifact-format.md` rule 11) — this
file cites them and never restates them.
````
- **Kept deliberately:** Doctrine-dense, multi-shape canonical reference — every operative line is protected / `DECISIONS.md`-traceable governance doctrine. All of it was carried **verbatim** into `plugins/mochiko/schemas/governance-surfaces.yaml` (shape-blocks preserved over uniform per-section fields, per plan §3 I3) and renders through `mochiko-cli template governance-surfaces`; the `.yaml` ships in the plugin as the raw-Read first-class degraded path (D8, GI-020, no install regression). Net-new per-section `check` lines were authored under D7 (disclosed, not lifted). V2 confirmed **no doctrine dropped** — nothing removed.
- **Consumers assessed:** `skills/authoring-constitution/SKILL.md` (re-pointed by P5) · `skills/validation-constitution/references/QUALITY-CHECKLIST.md` (D7 re-key — governance-surfaces structure cites the `--check` view, re-pointed by P5) · `templates/output-style.md` (contextual pointer reword, re-pointed by P5). V2 fidelity PASS 2026-08-16 (schema graded 8/8 at the M3 gate).

## [v0.65.0] Shape 1 region stamp — single-floor Ratified line superseded (carries depth level)
- **Disposition:** superseded → the `**Ratified:**` region stamp in Shape 1 of `templates/governance-surfaces-template.md`; preserved verbatim here.
- **Tier failed:** n/a — supersession by ruling (production-floor-adaptive-depth D1/D2, ratified 2026-08-11; `DECISIONS.md` 2026-08-11 adaptive-depth row).
- **Content (verbatim):**
```
**Ratified:** v[X.Y.Z] · [YYYY-MM-DD] · production floor · modules: [attached compliance modules, or "none"] <!-- GI-001 (fact profile) -->
```
- **Also reworded for disambiguation (lead ruling #2, same citation):** the Shape 1 Quality-gates line `Coverage ≥ [asserted floor level, session-overridable]%` → `Coverage ≥ [floor card's coverage threshold, session-overridable]%`. "Asserted floor level" there named the coverage-threshold number, not the new depth level; reworded so "level" is reserved for the depth declaration.
- **Kept deliberately:** the stamp's every other field verbatim — version, ratified date, `production floor`, the modules field, and the `GI-001 (fact profile)` trace comment; the depth field and its `GI-0XX (depth level)` trace are additions within the same line.
- **Consumers assessed:** grep across `plugins/` — `authoring-constitution/SKILL.md` mandatory-content-inventory item 1 (Ratified stamp) re-keyed to name the declared depth level this same wave (cluster B); `validation-constitution` grades the stamp fields (Cluster C re-keys it to the two-row form). No removed anchor.

## [v0.65.0] Shape 3 ledger Governance-Floor line — single-floor line superseded (carries depth level)
- **Disposition:** superseded → the `**Governance Floor:**` line in Shape 3 (the ledger) of `templates/governance-surfaces-template.md`; preserved verbatim here.
- **Tier failed:** n/a — supersession by ruling (production-floor-adaptive-depth D1/D2, ratified 2026-08-11; `DECISIONS.md` 2026-08-11 adaptive-depth row).
- **Content (verbatim):**
```
**Governance Floor:** production (asserted) · **Modules:** [attached compliance modules with strata, or "none"] · **Trace:** GI-001 (fact profile)
```
- **Kept deliberately:** `production (asserted)`, the Modules field with its strata note, and the `GI-001 (fact profile)` trace, all verbatim; the **Depth level** field (user-declared, one-way, `high` terminal) and its `GI-0XX (depth level)` trace are additions within the same line.
- **Consumers assessed:** the ledger is read by setup/amend runs and `validation-constitution` only; the amendment-policy semver + Route lines below it gained the flip event as pure additions this wave. No removed anchor.

**Wave context (v0.44.0 — the D7 leakage scrub).** `verbosity-caveman-ops-separation` D7 as
folded at review (S4): **full scrub** of ops leakage from the shipped tree, with no
changelog-worthy detail lost — every removed block is preserved verbatim below. Ruling:
`DECISIONS.md` 2026-08-01 "Output verbosity, caveman & ops separation ruled" row.

**The leak test this wave used, recorded so a future sweep inherits it: *whose artifact does the
pointer name?*** Mochiko's own ops records — `.mochiko/strips/`, `.mochiko/brainstorms/`,
`.mochiko/decisions/`, `.mochiko/archive/` — are leaks: they resolve to nothing in an installed
plugin. Adopter runtime paths (`.mochiko/specs/`, `.mochiko/memory/`) and the KM module's
document contracts are the **user's** artifacts and are untouchable. A prefix-based sweep on
`.mochiko/` would gut the KM module and the brainstorm command; 101 of this tree's 146
`.mochiko/` references were correctly left alone on that test.

## [v0.44.0] Design-record citation in the ownership header
- **Disposition:** superseded → deleted from the shipped file; preserved verbatim here.
- **Tier failed:** n/a — supersession by ruling (`verbosity-caveman-ops-separation` D7 + S4; the `DECISIONS.md` 2026-08-01 row above)
- **Content (verbatim):**
```
(design: .mochiko/brainstorms/constitution-native-surfaces/record.md,
D1–D8)
```
- **Kept deliberately:** the operative assertion — there is NO constitution.md, and governance is a SET authored by `authoring-constitution` and graded by `validation-constitution`.
