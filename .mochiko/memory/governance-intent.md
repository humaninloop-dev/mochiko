# Governance Intent — mochiko

**Session date:** 2026-08-06 · **Mode:** brownfield
**Confirmed at synthesis checkpoint:** 2026-08-06 by Deepesh
**Governs:** the governance surface set v2.0.0 (CLAUDE.md governance region · `.claude/rules/mochiko/` · `.mochiko/memory/governance-ledger.md`)

## Fact profile

- **GI-001 — Facts:** industry: none (developer tool) · data classes: none (no user data, no PII, no services, no DB) · jurisdictions/markets: none · contractual commitments: none · **Mark:** Confident
- **Modules triggered (mechanical):** none — negatives confirmed: no data classes → no privacy/security-regulatory module; no jurisdiction → no audit-trail module; no contracts → no contractual-compliance module; **no customer-facing UI → no accessibility module** (confirmed per-negative at review fold S3, 2026-08-06 — prose primitives consumed inside Claude Code, no UI ships). Blanket consequence stated and confirmed by user ("yes, this is developer tool"); the no-UI negative re-confirmed explicitly.
- **Brownfield cross-check:** consistent — `codebase-analysis.md` detects no services, no DB, no user data; integrations limited to GitHub hosting/marketplace, context7 MCP, and a local Anthropic gateway config.

## Project identity & type

- **GI-002 — Type:** none of the shelf taxonomy fits ("it doesn't fit, so don't try to fit" — user) → shelves dealt: universal floor only, expressed procedurally; no backend/service shelf. · **Mark:** Confident
- **Identity:** mochiko — kernel-free agent-skill framework for Claude Code; markdown primitive library (5 commands · 9 agents · 28 skills · 14 templates, plugin v0.53.0). Currently personal tooling for Deepesh, with a planned trajectory toward a public product ("currently B with plan for A sometime"). Governance follows today's reality; public-product compat obligations are a recorded future amend trigger.
  *AM-1 (2026-08-16):* the "kernel-free" identity phrase is superseded by the D11-softened position — **markdown-first primitive library with kernel-class tooling admissible by recorded ruling** (GI-019); the first admitted instance is the ruled (not yet built) template-schema Rust CLI, strictly additive to the install path (GI-020).
- **Risk surface:** flawed primitives propagate into downstream user projects; wasted design sessions; record-layer corruption destroying provenance. No money, no user data, no compliance exposure.
  *AM-1 (2026-08-16, review fold I3):* the ruled (not yet built) Rust CLI adds a **shipped-executable vector** — a compiled binary running on user machines is a materially different propagation/trust class than prose, incl. a supply-chain/dependency surface. Interrogated at AM-1; **no module attached now** (no binary ships yet, n=0; user-ruled). Revisit triggers: the crate's first public release · the existing public-product transition trigger.
- **Team reality:** solo maintainer; review culture = author≠grader validator audits (structural independence, not human reviewers).

## Convergence skips

- Dimension 4 (risk) — settled by dimensions 1–3 + analysis: propagation/provenance risk, stated to user with no objection.
- Dimension 5 (team) — settled by dimension 1: solo maintainer, audits as review culture.
- Dimension 6 (practices) — pre-filled by `codebase-analysis.md`: no lint/test/build/CI commands exist; quality floor is procedural (audits · strips ledger · landing ritual · human gates).

## Real commands (dimension 6/8 → the validator's placeholder bar)

| Purpose | Command | Source |
|---------|---------|--------|
| Lint | none exists | detected |
| Test | none exists | detected |
| Build | none exists | detected |
| Release | manual: `plugin.json` semver bump + git push (marketplace metadata sync manual) | detected |

The validator's placeholder bar adapts: no principle may cite a fictional command; enforcement clauses cite procedural gates (audit PASS, strip entry present, landing complete) instead.

## Floor expression & deck rulings

Floor cards enter asserted — rows record *expression* (type translation), never a level ruling:

| GI-ID | Card | Layer | Ruling / Expression | Mark |
|-------|------|-------|---------------------|------|
| GI-003 | FLOOR-SEC | floor-asserted | Secrets out of repo: `.claude/settings.local.json` (live token, detected) added to `.gitignore` — **MUST-fix this session**; no credentials in primitives/records. Secret-scanning clause **narrowed**, not waived: gitignore + pre-commit vigilance now; no CI obligation while no CI exists. Revisit trigger: CI arrives. | Confident |
| GI-004 | FLOOR-TEST | floor-asserted | Translated: every shipped-primitive edit passes the author≠grader audit before a version bump (the live ratchet); no coverage percentage — prose has none. *Subsumed as inapplicable (GI-007): coverage thresholds, smoke test (no runtime critical path).* Helper scripts carved out by waiver GI-008. | Confident |
| GI-005 | FLOOR-ERR | floor-asserted | Translated: no silent corruption of the record layer — protected content leaves only by recorded ruling (strips/supersession); dead pointers are defects caught by the KM dead-pointer scan. *Subsumed as inapplicable (GI-007): API/UI error surfaces, correlation IDs, stack-trace leakage (no runtime).* | Confident |
| GI-006 | FLOOR-OBS | floor-asserted | Translated: traceability is the observability surface — strips ledger + `DECISIONS.md` + version stamps; every primitive edit reconstructible from the record layer. *Subsumed as inapplicable (GI-007): structured logs, health checks, no-PII-in-logs (no runtime, no logs, no PII anywhere — GI-001).* | Confident |

Arbitrated deck: **empty by ruling** — no shelf dealt (GI-002); no architecture-opinion cards apply to a prose library.

## Minted principle intents

- **GI-017 — Pointer-only region (none minted):** ruled by user ("leave these out"): the repo's existing constraints (no-kernel · author≠grader · landing ritual · single-sourcing · protected-content-by-ruling) stay in their current homes (CLAUDE.md prose, rules files). The governance region **points at** those homes; it never restates them. This is a producer-binding selection constraint — restating an existing constraint on a surface is a trace violation against this element. · **Mark:** Confident
  *Elicited from:* dimension 9 — candidates presented for codification, user ruled "leave these out"
  *AM-1 note (2026-08-16):* the "no-kernel" home named here now carries the D11-softened text (see GI-019, AM-1); the pointer-only mechanic itself is unchanged.

- **GI-019 — Kernel-class tooling admission (D11 bright line):** the no-kernel position is softened per the `schema-based-template-guidance` ruling D11 (2026-08-16): skills and agents remain the primary quality surface; kernel-class executable tooling is admissible **only by recorded ruling**, and such tooling never gates pipeline progress, never dispatches or sequences agents, never holds judgment that skills own. **Definition (review fold I1):** kernel-class ≡ executable tooling whose output primitives *depend on* to do their work — source-of-truth delivery, composition, or any standing infrastructure role. Advisory post-hoc checkers consumed as optional exit-code signals are **not** kernel-class; the 6 existing scripts (5 `.py` validators, 1 `.sh` detector) land there, unpainted by this element and still carried by waiver GI-008. First admitted instance: the template-schema Rust CLI (foundation seed for future native tooling, Tauri-bound), with the recorded concession that template delivery alone would not carry it. Constraint home stays CLAUDE.md prose (per GI-017 the region points, never restates). · **Mark:** Confident *(evidence basis n=0 — the concession above names it; review fold M2)*
  *Elicited from:* brainstorm `schema-based-template-guidance` D11, user-ruled at review disposition; setup amend invoked by the user 2026-08-16; definition + script placement user-ruled at the AM-1 review disposition batch

- **GI-020 — Additive-CLI install constraint (user-declared):** the plugin's install path stays exactly as it is today — a markdown plugin, no install-time build step, no binary dependency, no submodule-class fetch burden; the CLI is **strictly additive**: the plugin MUST install and function without the binary present, with the schema data files readable raw as the first-class degraded path (record D8). Any distribution mechanism for the binary that would make plugin install heavier violates this intent. · **Mark:** Confident
  *Elicited from:* the user's setup invocation, 2026-08-16 — "I want to retain the current way plugin is installed, cli is additional"

- **GI-021 — Depth level declared: high (legacy default).** The production floor's depth level is `high` — set up under the single floor pre-adaptive-depth, already conformed to full depth (#7 fold, 2026-08-11). Minted at AM-1, discharging the ledger's "formal GI-row minting rides the next amend run" pointer. No ceremony; one-way ratchet applies (high never returns to low). · **Mark:** Confident
  *Elicited from:* ledger legacy-default line (2026-08-11); minting obligation discharged at the AM-1 review disposition (C1)

## Waivers

| GI-ID | Standard | Justification | Revisit trigger | Mark |
|-------|----------|---------------|-----------------|------|
| GI-008 | FLOOR-TEST as applied to the 6 helper scripts (1 bash, 5 python) | Scripts are thin standalone validators/detectors; no shared deps; testing/lint infrastructure absent and not worth erecting for them today | Script count grows, or a script becomes load-bearing in a shipped flow *(lead-composed, user-ratified at review fold S5, 2026-08-06)* | Confident |

(FLOOR-SEC secret-scanning is a **narrowing** recorded in GI-003, not a waiver.)

## Module selections

| GI-ID | Module | Ruling | Because | Mark |
|-------|--------|--------|---------|------|
| GI-009 | knowledge-management (core) | **adopted — pin ratified** | Offered default-on at dimension 7; core already hand-pinned 2026-07-25 and live; this run is the pin's recorded revisit trigger, now discharged: pinned core ratified as the ruled core. All four existing root docs (`ROADMAP.md` · `BACKLOG.md` · `DECISIONS.md` · `ARCHITECTURE.md`) codified into module roles — analysis confirms semantics fit; **no collisions**. ARCHITECTURE.md deferral in the pin is **retired** (doc gained content); GLOSSARY.md deferral **carried** as recorded deviation — scaffold when it gains content. | Confident |
| GI-010 | knowledge-management elective: `CHANGELOG.md` | adopted | Release-shaped project (semver, marketplace) | Confident |
| GI-011 | knowledge-management elective: `RUNBOOK.md` | **declined — durable** | Nothing deployed, nothing operated | Confident |
| GI-012 | release-gates | adopted | User ruling at dimension 8 ("okay adopt, release gate"): codify what blocks a `plugin.json` bump — audits PASS · strip entries recorded · landing ritual complete · marketplace metadata synced (current 0.10.0-vs-0.53.0 lag becomes a tracked defect). *AM-1 (2026-08-16) dormant conditional:* when the template-schema Rust crate lands — `cargo test` PASS joins the blocking gates; schema-data/binary consistency joins the marketplace-sync gate; dormant until the crate exists. | Confident |
| GI-013 | layer-rules | **declined — durable** (AM-1 once-offer, 2026-08-16) | No layered architecture exists — prose library + one future crate; re-openable by explicit ruling if the Rust codebase grows layers. *(Was: not offered at v1.0.0, bookkeeping only.)* | Confident |
| GI-014 | evolution-notes | **declined — durable** (AM-1 once-offer, 2026-08-16) | The Rust/Tauri trajectory already lives in ruled homes — the D11 record, ROADMAP standing surface, BACKLOG build item; a separate evolution artifact would duplicate them (GI-017 pointer-only spirit). *(Was: not offered at v1.0.0, bookkeeping only; reviewer flagged newly-substantive given the Tauri trajectory — offered and declined with that in view.)* | Confident |

## Domain-dependency seeds

Not applicable — `layer-rules` not adopted.

## Deliberate exclusions (dimension 10)

- **GI-007:** Application-shaped enforcement machinery (CI pipelines, coverage gates, runtime health checks, log schemas) — excluded as *inapplicable in kind*, with each floor category retained via translated expression (GI-003–006), never dropped. Helper-script carve-out rides waiver GI-008, not exclusion. · **Mark:** Confident
  *AM-1 dormant note (review fold I2):* the "no runtime critical path" basis goes stale when the Rust crate lands — the CLI is a runtime with a critical path (emits authoring guidance). At the crate landing, GI-004/GI-007's inapplicability clauses and GI-002's tech-stack statement are re-expressed alongside the GI-012 gate activation — same dormant treatment, one consequence set.
- **AM-1 scope exclusion (review fold M1):** the GI-019 softening licenses **no general kernel** and **no orchestration/brain-code** — the recorded-ruling door plus the bright line are the whole grant; template conversion stays scoped to D3's 8 pipeline artifact templates, not all templates. A future reader must not read GI-019 as "kernel tooling is now generally fine." · **Mark:** Confident

## Confrontation rulings (brownfield)

- **GI-015 — Live token exposure:** `.claude/settings.local.json` holds a live `ANTHROPIC_AUTH_TOKEN`, untracked but absent from `.gitignore` — one `git add -A` from being committed. Ruling: MUST-fix in this session's finalize (add to `.gitignore`); folded into GI-003's expression. · **Mark:** Confident
- **GI-016 — Marketplace metadata lag:** `marketplace.json` at 0.10.0 vs plugin 0.53.0. Ruling: becomes a release-gates tracked obligation (GI-012); sync lands as a gate, not a one-off fix. · **Mark:** Confident
- **GI-018 — ARCHITECTURE.md version-lag accepted:** analysis inconsistency #3 (header cites v0.48.0 vs plugin v0.53.0) accepted as intentional — the doc updates only at component-changing landings, per its own contract; not a defect, not a release gate. · **Mark:** Confident

## Review

**2026-08-06 — first ratification**

- **Sizing:** lead stated weight at sizing time: 16 elements · marks all-Confident (audited; post-fold: 18 elements, 16 Confident + 2 bookkeeping) · reality-surface load moderate (brownfield analysis + live KM pin + two confrontations); default pair on first ratification; **lead sized: single** (solo seat, both lenses) — below default: departure reason is the session's narrow scope (KM-centered, empty arbitrated deck, no minted principles beyond the pointer-only ruling) — departure-trail line carried here.
- **Review:** solo reviewer, coverage + coherence lenses; **tally 9 raised → 6 merged survivors**; recommended status needs-revision.
- **Survivor dispositions:**

  | # | Sev | GI element(s) | Finding | Disposition |
  |---|-----|---------------|---------|-------------|
  | S1 | Medium | GI-013, GI-014 | "not offered" rows read as durable rulings, would foreclose amend offers | resolved — rows re-marked as bookkeeping, no ruling recorded, offerable on amend |
  | S2 | Medium | GI-017 | pointer-only directive lacked a GI-ID, invisible to trace string-match | resolved — minted as GI-017 with elicitation quote |
  | S3 | Minor | GI-001 | no-UI negative (a11y trigger) never named | resolved — user confirmed per-negative at fold |
  | S4 | Minor | GI-004–006 | dropped floor sub-clauses vanished implicitly | resolved — subsumed sub-clauses named per row |
  | S5 | Minor | GI-008 | revisit trigger lead-composed, not user-ruled | user-ruled — trigger ratified as user's own, provenance noted |
  | S6 | Minor | (analysis #3) | ARCHITECTURE.md version-lag uncarried | resolved — GI-018 accepts as intentional |

- **Verify pass:** PASS — all six folds confirmed by the sole reviewer from disk (2026-08-06); one sizing-line figure nit fixed post-verify.

## Amendment Log

**AM-1 — 2026-08-16 — no-kernel softening + additive CLI** *(driver: brainstorm
`schema-based-template-guidance` D1–D11, accepted 2026-08-16; setup amend invoked by the
user same day)*

- **Scope (as amended at review):** GI-019 minted (D11 bright line — kernel-class tooling
  by recorded ruling only, never gates / never orchestrates agents / never holds
  skill-owned judgment; kernel-class defined by the output-dependency test, the 6 existing
  scripts placed outside it) · GI-020 minted (additive-CLI install constraint,
  user-declared) · GI-021 minted (depth level `high`, legacy default — ledger pointer
  discharged) · GI-002 identity annotated (the "kernel-free" identity phrase superseded by
  the D11-softened position; markdown-first identity survives) + risk surface extended
  (shipped-executable/supply-chain vector interrogated, no module now, revisit triggers
  named) · GI-012 gains a **dormant conditional clause** (activates when the Rust crate
  lands: `cargo test` PASS joins the blocking release gates; schema-data/binary
  consistency joins the marketplace-sync gate) · **crate-landing consequence set widened
  (review fold I2):** GI-002 tech-stack statement + GI-004/GI-007 inapplicability clauses
  re-expressed at the same landing, same dormant treatment; ledger CI-arrival trigger
  scope widened to match · GI-003's secret-scanning revisit trigger noted as expected to
  fire there — no change now · GI-013/GI-014 once-offer run: both **declined durable** ·
  AM-1 scope-exclusion line added (no general kernel, no orchestration) · CLAUDE.md prose
  rewording (core-bet sentence + no-kernel non-negotiable) user-approved to land **in this
  run**, ahead of the build, per the user's timing ruling.
- **Semver:** governance surface set v1.0.0 → **v2.0.0** (MAJOR — a non-negotiable's
  meaning changes; user-ruled).
- **Review:** solo cold intent review via blind-map two-message dispatch (27-angle map,
  topic-only spawn; synthesis + ledger withheld until the map returned). Verdict
  **critical-gaps** — 12 raised, 5 killed on read, 7 survived (C1 depth-row unminted ·
  I1 kernel-class undefined/scripts unplaced · I2 half-sequenced crate-landing set ·
  I3 stale risk surface/supply-chain uninterrogated · I4 module once-offer skipped ·
  M1 missing scope exclusion · M2 bare Confident mark). 7/7 dispositioned 2026-08-16:
  I4's two offers + I3 user-ruled individually, C1/I1/I2/M1/M2 one user-ruled batch "as
  recommended"; all folds landed in this synthesis. Lead re-verified the C1 ledger quote
  against the file before disposition.
- **Ratified:** 2026-08-16 by Deepesh — after the CLEAN verify pass over the folded
  dispositions; 4 non-blocking verify nits carried to the producer checklist (header
  version line · ledger touch set · GI-002 tech-stack re-expression home · optional
  "today" tightening).
- **Accepted:** 2026-08-16 by Deepesh — surface set v2.0.0 authored by the producer seat
  on a lead-approved plan (two content-pinned amendments: named-species retention ·
  Workflows/agent-teams sentence), graded PASS round 1 by an independent
  `validation-constitution` seat (trace closed both directions; 3 non-blocking
  advisories: ROADMAP thesis line closed at this same acceptance · index-line marker
  left per GI-017 precedent · trace manifest served by distributed stamps).
  `floor: tripped · seats: gov-producer (tech-lead) / gov-validator + intent-reviewer`.
