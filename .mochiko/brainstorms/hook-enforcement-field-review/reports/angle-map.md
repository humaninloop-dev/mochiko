---
report: review
round: 0
session: hook-enforcement-field-review
phase: blind angle map (Phase 0, review-brainstorm.blind-map-before-record-contact)
reviewer: cold-reviewer
record_contact: none — built from the topic statement and goal line only; no session artifact, index entry, or other brainstorm record opened
grounding: plugin 0.112.0 hooks.json and hooks/scripts/*.sh; migrations 0005-artifact-homes.yaml (home documents and every *.artifact-home rule); crates/mochiko-cli/src/hook.rs and conform.rs doc comments; mochiko-cli home renders against the kinako tree; kinako .mochiko/ file census and git log; mochiko BACKLOG.md (hook build item; the 2026-09-23 kinako run-audit defect item; feature-entry path grammar item); DECISIONS.md rows 2026-09-13 and 2026-09-14
angle_count: 53
class_counts:
  A-jurisdiction: 6
  B-homes-and-placement: 12
  C-budgets-and-shape: 8
  D-mechanism-integrity: 8
  E-discovery-and-ergonomics: 5
  F-governance-and-change: 5
  G-evidence-base: 4
  H-rejected-roads: 5
  total: 53
angles:
  A-jurisdiction:
    - "A1 · Purpose in field terms: what the gate is for (structural validity of artifacts, reviewer read cost, findability) stated as observable outcomes, and whether runs 3 and 4 show those outcomes or different ones."
    - "A2 · Bright line (GI-019 clause iv, never gates pipeline progress): does a budget deny that blocks a landing fold or a lead's-pen write act as de facto pipeline gating; every proposed hardening checked against sequencing and judgment."
    - "A3 · Per-check disposition: for each of the six checks (path, file set, frontmatter, headings, placeholders, size) a keep-deny, downgrade-to-context, or drop ruling, each resting on its own field evidence."
    - "A4 · Middle lane: allow-plus-additionalContext (already used by amnesty) as a ruled outcome for some checks, given `ask` is excluded; which checks, if any, move there."
    - "A5 · Beneficiary: consumer repository hygiene versus mochiko's own review economy; naming it sets the tolerable false-deny cost."
    - "A6 · Sanctioned override: the gate holds in every permission mode, so the user's only escape was out-of-band (APPLY.sh); is a recorded break-glass needed, and how is its use made visible."
  B-homes-and-placement:
    - "B1 · Census of what an implement run actually writes (cycle and verification reports, evidence captures, run logs, seat plans, fix briefs, landing fold texts, probe notes, sweep plans, gap-finding reports, groom archives) against the feature home's declared set; each kind gets a ruled home or a ruled not-persisted."
    - "B2 · Evidence captures: non-markdown, shell-produced, voluminous (68 files under reports/evidence in run 3, 166 in .mochiko/archive/evidence in run 4); home, retention, git status, and whether a capture must carry the report envelope."
    - "B3 · Shell-arm collision with evidence: the Bash deny forbids redirect capture into any home, yet redirect is how a TEST case captures output; carve, relocate, or accept."
    - "B4 · Run log: impl.artifact-home names implement-log.md but the feature home does not declare it (epic home only); both runs minted run-log-*.md as report: disclosure. Declare it, retire the rule text, or rule per-run versus per-capability logs."
    - "B5 · Report type set versus actual kinds: disclosure used as catch-all (390-line run log, 619-line fold text, probes); report: fix present outside the closed set. Widen the type set or give non-report artifacts (briefs, fold texts, plans) their own declared files."
    - "B6 · reports/ nesting: a sub-directory under reports/ resolves as an undeclared sub-directory (never relaxed), and any file in reports/ must carry the envelope; rule nesting and non-markdown in reports/."
    - "B7 · Scale of a flat reports/: 168 files for one capability after two runs; per-run or per-cycle namespacing, name grammar, and the discovery cost for later graders."
    - "B8 · Multi-run identity: runs 3 and 4 (and earlier epic runs) share one capability home; accumulating files (tasks.md, baseline-delta.md, gates.md) versus a run-scoped sub-home versus close-time archival."
    - "B9 · Archive: .mochiko/archive/ is no declared home, so it is the ungated sink for evidence and groom overflow; declare it, gate it, delete-and-rely-on-git, or keep it as the deliberate valve."
    - "B10 · Standing undeclared structures in a consumer tree (features/B53, features/B61, epics/*/reviews, epics/*/landing, FEAT-006/reviews, specs/*/stress-test.md): violator pass or migration path, and whether a re-homing tool is kernel-class creep."
    - "B11 · Path grammar: feature-entry and features-index templates say FEAT-XXX-slug.md, the home says FEAT-ID.md and matches FEAT-001-sparring-sessions.md; one canonical grammar and how strict the matcher is."
    - "B12 · Parity: whatever is ruled for the feature home (evidence, log, run namespacing) applied to the epic, product-lane, and desk homes, and to declared sub-directories with no home document (spec prototype/)."
  C-budgets-and-shape:
    - "C1 · Whole-file caps on accumulating design artifacts: requirements.md 476 and baseline-delta.md 2314 against 300, constraints-and-decisions.md sitting at 300; tests GI-019's condition that the ratified table admits honest content."
    - "C2 · Folds into over-budget baselines: amnesty allows only non-worsening writes, so any additive fold denies; fold text stayed owed and was applied out of band. Landing-fold carve, groom-first, log form, or raised budget."
    - "C3 · Displacement under pressure: mid-run grooms (spine 646 to 227) and groom overflow moved to the ungated archive; does the gate compress content or move it out of sight, and how would loss be detected."
    - "C4 · Asymmetry in reports: the envelope bounds only its three prose sections (15 lines), payload is unbounded, so reports became the unbounded valve; intended or a hole."
    - "C5 · Budget unit: lines are gameable by long lines and inflated by tables and code blocks; lines versus characters versus a countable unit."
    - "C6 · Budgets that scale with work: tasks.md at 1658 lines grows with card count; per-section budgets tied to countable units (as concerns.md is tied to concern count) versus fixed numbers."
    - "C7 · Amnesty semantics: non-worsening per measure, the recorded bare-allow-on-file-set gap, and whether a baseline reset or an in-fold allowance is needed."
    - "C8 · Shape evasion: extra ## headings denied and prose sections capped; did authors shift to ### headings, frontmatter payload, or unstructured prose to pass, and does that degrade the artifact."
  D-mechanism-integrity:
    - "D1 · Bypass census: writes that reached homes outside the gate (user-run scripts, relative redirects after cd that do not name .mochiko, interpreter scripts, git mv or checkout, NotebookEdit and MCP writers); close, accept as named holes, or detect post hoc."
    - "D2 · Shell-arm accuracy: false denies on commands that only read .mochiko, false allows the scanner misses; field counts."
    - "D3 · Fail-open windows: binary absent, unreadable log, timeout, missing cwd; any silent fail-open observed in either run."
    - "D4 · Plugin version change inside a run: run 3 spanned 0.110.0 and 0.112.0; homes or budgets changing under a live run, and whether a run pins its log version."
    - "D5 · Deny-loop and hook cost: denies per run, turns burned retrying, the advisory halt sentence's effect, the per-call cost of the un-narrowed Write|Edit arm across every product-code edit, against the 60 s watch."
    - "D6 · Background-subagent decisions: the explicit-allow requirement held on teammate transports, or silent denies appeared."
    - "D7 · Working-directory scope: paths outside cwd are allowed, so worktree seats or sub-directory cwds escape the gate."
    - "D8 · Out-of-home report sniff (D9): did it fire, did it false-positive on product docs, and does it cover .mochiko/archive."
  E-discovery-and-ergonomics:
    - "E1 · Addressing contract of mochiko-cli home: impl.artifact-home instructs rendering the FEAT-XXX directory, which resolves to the parent features-index home and reports FEAT-001 as not a declared deliverable, with or without a trailing slash; the discovery surface misleads the seat it serves."
    - "E2 · SubagentStart reminder: did seats render their home before the first write; keep, strengthen, or drop the one-line reminder, and its context cost across many seats."
    - "E3 · Deny text actionability: does a deny name the home, the fault, the budget, and the admissible next move; measured by first-retry success."
    - "E4 · Raise-never-mint in practice: seats minted names (run-log-*.md, fix-*.md, report: fix) instead of raising; mid-run the lead cannot declare a file, only a plugin release can. Is a sanctioned mid-run path needed."
    - "E5 · Consumer-side extension: a project-local overlay of homes or budgets versus plugin-only declaration, weighed against the single-source-of-truth and no-readable-schema constraints (GI-020)."
  F-governance-and-change:
    - "F1 · Change vehicle and sequencing: new homes or budgets ship as a migration plus bump; ordering against the unreleased crate publish, release gate 6, and the pending pre-authorized PATCH."
    - "F2 · Governance weight: does any change touch GI-019 clause (iv) or the budget-table condition, needing a /mochiko:setup amend rather than a plugin PATCH."
    - "F3 · Primitive-edit ritual: changes to impl.artifact-home and home documents are shipped-primitive edits needing strips entries, the author-not-grader audit, and protected-content checks."
    - "F4 · Consistency sweep: every *.artifact-home rule, the feature-entry templates, and testing-gap-finding's gates.md path aligned with any new home in one move."
    - "F5 · Test coverage: contract-suite goldens and kinako-shaped fixtures (long runs, many files, evidence, multi-run homes) for whatever is ruled."
  G-evidence-base:
    - "G1 · Sample limits: two runs, one capability, one consumer, one operator, both multi-day; which conclusions are safe from that base and which are provisional."
    - "G2 · Attribution: separating gate-caused friction from co-occurring causes (plan-grade failures named as the main time sink, store grooms driven by store budgets)."
    - "G3 · Transcript-audit provenance: deny and bypass counts taken exhaustively or sampled, and whether load-bearing counts are checkable against transcripts."
    - "G4 · Re-evaluation watch: deny rate, bypass count, out-of-home file count, reports-per-run, archive growth; thresholds and a re-review trigger."
  H-rejected-roads:
    - "H1 · Retire the write-time gate for an advisory post-hoc checker (a lint at landing, optional exit-code signal), the non-kernel shape GI-019 already admits."
    - "H2 · Path-and-file-set only: drop shape and size denies, keep location enforcement."
    - "H3 · Ephemeral run directory: run artifacts (evidence, logs, drafts) out of the governed tree or gitignored, only durable surfaces gated."
    - "H4 · Per-run sub-home (features/FEAT-XXX/runs/RUN-ID/) with its own open set, versus the flat capability home."
    - "H5 · Size as advisory only: budgets reported as context at write time and graded by the reviewing seat, never denied."
---

## Notes of note

- Grounding facts that shaped classes B, C and E were read first-hand, not from any session artifact:
  `mochiko-cli home .mochiko/features/FEAT-001` resolves to `features-index`;
  `reports/evidence/…` resolves as an undeclared sub-directory; `reports/a.txt` resolves as a report
  bound by the envelope; the feature home declares no `implement-log.md`.
- Pairing, lens and pass are not stated in message 1; the map is lens-neutral.
