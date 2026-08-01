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

**Ratified:** v[X.Y.Z] · [YYYY-MM-DD] · production floor · modules: [attached compliance modules, or "none"] <!-- GI-001 (fact profile) -->

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
- Coverage ≥ [asserted floor level, session-overridable]% on new code (`[actual command]`) <!-- GI-XXX -->

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

**Governance Floor:** production (asserted) · **Modules:** [attached compliance modules with strata, or "none"] · **Trace:** GI-001 (fact profile)
**Version:** [X.Y.Z] (must match the region stamp)

## Waivers

Any asserted standard, with a recorded justification (D4); permanent pending the D4.1 revisit.
**Legal-mandate module obligations are unwaivable (D4.2)** — a waiver row naming one is a
validator FAIL.

| Standard | Justification | Revisit trigger (optional) | Trace |
|----------|---------------|----------------------------|-------|
| [floor category / card / non-legal module obligation, or "None."] | [recorded reason] | [or "permanent (D4.1 pending)"] | GI-XXX |

## Amendment policy

- Route: `/mochiko:setup` amend mode; fact-profile changes (module attach/detach) and
  un-waives are governance events.
- Semver: MAJOR — principle removal / incompatible redefinition / floor-level change / module
  attach or detach · MINOR — new principle or waiver change · PATCH — clarification.
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
