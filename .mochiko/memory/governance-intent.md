# Governance Intent — mochiko

**Session date:** 2026-08-06 · **Mode:** brownfield
**Confirmed at synthesis checkpoint:** 2026-08-06 by Deepesh
**Governs:** the governance surface set v1.0.0 (CLAUDE.md governance region · `.claude/rules/mochiko/` · `.mochiko/memory/governance-ledger.md`)

## Fact profile

- **GI-001 — Facts:** industry: none (developer tool) · data classes: none (no user data, no PII, no services, no DB) · jurisdictions/markets: none · contractual commitments: none · **Mark:** Confident
- **Modules triggered (mechanical):** none — negatives confirmed: no data classes → no privacy/security-regulatory module; no jurisdiction → no audit-trail module; no contracts → no contractual-compliance module; **no customer-facing UI → no accessibility module** (confirmed per-negative at review fold S3, 2026-08-06 — prose primitives consumed inside Claude Code, no UI ships). Blanket consequence stated and confirmed by user ("yes, this is developer tool"); the no-UI negative re-confirmed explicitly.
- **Brownfield cross-check:** consistent — `codebase-analysis.md` detects no services, no DB, no user data; integrations limited to GitHub hosting/marketplace, context7 MCP, and a local Anthropic gateway config.

## Project identity & type

- **GI-002 — Type:** none of the shelf taxonomy fits ("it doesn't fit, so don't try to fit" — user) → shelves dealt: universal floor only, expressed procedurally; no backend/service shelf. · **Mark:** Confident
- **Identity:** mochiko — kernel-free agent-skill framework for Claude Code; markdown primitive library (5 commands · 9 agents · 28 skills · 14 templates, plugin v0.53.0). Currently personal tooling for Deepesh, with a planned trajectory toward a public product ("currently B with plan for A sometime"). Governance follows today's reality; public-product compat obligations are a recorded future amend trigger.
- **Risk surface:** flawed primitives propagate into downstream user projects; wasted design sessions; record-layer corruption destroying provenance. No money, no user data, no compliance exposure.
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
| GI-012 | release-gates | adopted | User ruling at dimension 8 ("okay adopt, release gate"): codify what blocks a `plugin.json` bump — audits PASS · strip entries recorded · landing ritual complete · marketplace metadata synced (current 0.10.0-vs-0.53.0 lag becomes a tracked defect) | Confident |
| GI-013 | layer-rules | not offered this run — **no ruling recorded; remains offerable on amend** | agenda's layered-architecture beat did not fire (no layered card kept, no layered intent minted) | — (bookkeeping, not a ruling) |
| GI-014 | evolution-notes | not offered this run — **no ruling recorded; remains offerable on amend** | no evolution-roadmap artifact; brownfield gaps landed as confrontation rulings (GI-015, GI-016) instead | — (bookkeeping, not a ruling) |

## Domain-dependency seeds

Not applicable — `layer-rules` not adopted.

## Deliberate exclusions (dimension 10)

- **GI-007:** Application-shaped enforcement machinery (CI pipelines, coverage gates, runtime health checks, log schemas) — excluded as *inapplicable in kind*, with each floor category retained via translated expression (GI-003–006), never dropped. Helper-script carve-out rides waiver GI-008, not exclusion. · **Mark:** Confident

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

(Empty on first ratification.)
