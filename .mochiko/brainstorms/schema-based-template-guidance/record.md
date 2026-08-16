# Schema-Based Template Guidance — Decision Record

**Status:** accepted (2026-08-16) — solo cold-reviewed via blind-map two-message dispatch
(26-angle map, topic-only spawn); verdict critical-gaps → 7/7 survivors dispositioned
(spine user-ruled "own the real driver" → D11); bounded verify round CLEAN; user accepted
on the record as folded.
**Opened:** 2026-08-16
**Topic:** Move the plugin's `.md` template files from example-file guidance to schema-based
guidance delivered at runtime. Output artifacts stay `.md`; the schema (structure, section
contracts) would be read from a CLI invocation instead of a static `.md` exemplar, so that
runtime parameters — verbosity, depth of content, types — can be injected per run.

**Goal line:** decide whether, and in what form, template guidance moves from static `.md`
files to runtime-parameterized schema delivery, leaving one hardened decision record.

---

## Ground facts

- **F1** — `plugins/mochiko/templates/` holds 17 files: 6 seat report templates
  (`advocate-`, `analyst-`, `architect-`, `feasibility-`, `techanalyst-report-template.md`,
  `report-format.md`), pipeline artifact templates (`spec-template.md`, `plan-template.md`,
  `tasks-template.md`, `feature-entry-template.md`, `features-index-template.md`,
  `codebase-analysis-template.md`, `governance-intent-template.md`,
  `governance-surfaces-template.md`), register/format guidance (`output-style.md`,
  `artifact-format.md`), and `constitution-modules/`. (verified: `ls`, 2026-08-16)
- **F2** — Template-shaped guidance also lives inside skills as references:
  `authoring-technical-requirements/references/ARTIFACT-TEMPLATES.md`,
  `testing-end-user/references/REPORT-TEMPLATES.md`,
  `review-plan-artifacts/references/ARTIFACT-CHECKLISTS.md`, and ~20 SKILL.md files
  reference `templates/` paths. (verified: grep, 2026-08-16)
- **F3** — Repo constraint (CLAUDE.md, non-negotiable): no kernel infrastructure — no
  Python/MCP brain code; skills and agents are the quality surface. Shell-script precedent
  exists inside a skill (`analysis-codebase` ships `detect-stack.sh`). **As amended at
  review (I2):** five Python advisory validators ALSO ship today inside skills —
  `validate-requirements.py`, `validate-user-stories.py`, `validate-openapi.py`,
  `validate-model.py`, `check-artifacts.py` — deterministic, advisory, exit-code-style,
  coexisting with the non-negotiable unamended. This fact was absent when D2 was ruled.
  (verified: `ls`, 2026-08-16)
- **F4** — A runtime depth dial already exists at the governance layer: the production
  floor's user-declared low/high depth level (adaptive-depth D1–D8, 2026-08-11), plus
  the writing-style dial (conversation/reports/documents · off/lite/full/ultra) in
  CLAUDE.md's governance section.
- **F5** — Prior evidence of template-verbosity pain (kinako dogfood): `quickstart.md`
  authored unconditionally at 17.5k bytes → made conditional ≤150 lines;
  `task-mapping.md` freehand → ballooned to 45.9k bytes for one slice → canonical compact
  table form ruled. **As amended at review (M5):** both fixes were static-guidance edits —
  F5 is neutral context on delivery mechanism, not pro-CLI evidence.

## Decisions

- **D1 — Delivery mechanism is a single plugin CLI.** One CLI tool shipped with the plugin
  (working shape: `mochiko-template <name> --depth=... --verbosity=...`) owns all template
  schemas and emits schema + guidance at runtime; authoring skills invoke it instead of
  reading a static `.md` exemplar. Chosen over script-per-template, no-executable
  structured data, and an external tool. `Confident` (user-ruled at Q1, options presented
  with the no-kernel tension named). *Reconciliation with the no-kernel constraint pending
  — see Q2.*

- **D2 — The no-kernel constraint is amended, not argued around.** The single plugin CLI
  is treated as a real collision with CLAUDE.md's "No kernel infrastructure" non-negotiable.
  The constraint text is amended by recorded ruling — a governance event — to carve out
  non-orchestrating executable tooling, keeping the constraint text honest rather than
  resting on a compatibility interpretation. Lead recommended "compatible, record why";
  user chose amendment. `Contested` (deliberate user choice against recommendation).
  *Amendment wording and its boundary line (what the carve-out permits and still bans) to
  be settled in this session; the CLAUDE.md edit itself lands at build, via the governance
  amendment path.* **As amended by D9:** the amendment takes the form of a recorded
  this-CLI-only exception line on the non-negotiable, not a general carve-out category.
  **As amended at review disposition:** superseded by D11's softening — the
  non-negotiable is reworded (skills-first, kernel-class tooling admissible by recorded
  ruling), not annotated with a one-off exception line.

- **D3 — Scope: pipeline artifact templates first.** The conversion covers the pipeline
  artifact templates (`spec-template.md`, `plan-template.md`, `tasks-template.md`,
  `feature-entry-template.md`, `features-index-template.md`,
  `codebase-analysis-template.md`, `governance-intent-template.md`,
  `governance-surfaces-template.md`). Seat report templates, format/register guidance,
  `constitution-modules/`, and in-skill reference templates stay `.md` until the mechanism
  proves itself; later ratchet explicitly available. `Confident` (user accepted
  recommendation at Q3).

- **D4 — CLI output = schema + example + normative guidance; the agent never selects
  depth.** Invoking the CLI for a template (working shape: `mochiko-cli template spec`)
  returns three things: the structural schema (sections, types), an example/skeleton, and
  guidance on what good and bad look like — verbosity norms, depth-of-content norms
  included. Agent-side parameter flags (`--depth=...`) are ruled out as misguided: the
  choice of verbosity/depth is not the agent's to make; the CLI guides it. Runtime
  injection means the CLI is the single guidance authority at authoring time, not that the
  caller tunes knobs. `Confident` (user-authored reframe at Q4). *Where the CLI's own
  variation input comes from (project config vs baked-in) → Q5.*

- **D5 — Guidance variation is baked into the CLI release, not read from project
  config.** The CLI carries its verbosity/depth/type norms internally; they change by
  shipping a new plugin version, never by reading per-project state. No coupling to the
  governance ledger's low/high depth dial or the writing-style dial — foregone
  alternative recorded (lead recommended project-config reads; per-project adaptation is
  deliberately given up). The "runtime injection" value lands as: one central, versioned
  guidance authority decoupled from skill and template text — tune once, all templates
  follow, no 20-skill edit wave. `Contested` (deliberate user choice against
  recommendation).

- **D6 — Runtime: Rust, compiled binary.** The CLI is written in Rust and ships as a
  compiled binary. User-stated rationale: the Rust codebase is a forward bet — a Tauri
  desktop UI is hoped for later, sharing this foundation. So the CLI is not just a
  template tool; it seeds mochiko's first non-markdown codebase. Priced consequences on
  record: per-platform distribution (macOS arm64/x64 · Linux · Windows) in a plugin that
  today ships only markdown; a build pipeline where the repo has none — its arrival trips
  CLAUDE.md's standing governance amend trigger "CI arrival"; the Python ban's substance
  stays covered — D11's bright line bans the brain-code/orchestration class regardless of
  language. `Confident` (user-authored with reason at Q6).
  **As amended at review (I3/I4 disposition):** rationale made honest — Rust is chosen as
  the foundation seed for mochiko's future native tooling (Tauri-bound), decided openly
  here rather than implied; the template CLI is that foundation's first workload. The
  reviewer's decoupling route (decide delivery on the template problem's own needs, let a
  future session pick the stack) was put to the user and declined.

- **D7 — Validators grade against the same schema, role-shaped.** The CLI grows a grading
  view (working shape: `mochiko-cli template spec --check`) emitting the checklist form of
  the same schema the producer received. Producer and grader consume one truth source in
  role-shaped views; the hand-written artifact checklists covering in-scope templates
  re-key to it. Template↔checklist drift dies structurally. Author≠grader is untouched —
  independence lives in seats, not sources. `Confident` (user accepted recommendation
  at Q7).

- **D8 — Schemas live as structured data files; the binary is a renderer; raw Read is the
  fallback.** Schema + example + guidance live as structured data (YAML/JSON) shipped in
  the plugin; the Rust binary renders and composes views over them. When the binary is
  unavailable (unsupported platform, sandbox, pre-build clone), the authoring skill Reads
  the data files directly — degraded but functional. Single source of truth survives every
  environment; no `.md` template fallback, no dual maintenance. `Confident` (user accepted
  recommendation at Q8).

- **D9 — No-kernel position: this-CLI-only exception; the broader move-away is
  deferred.** At Q9 the user stated willingness to move away from the no-kernel position
  entirely; on the explicit scope pin (Q10) the ruling is: the template CLI — including
  its mechanical-lint capability — is approved as a **one-off recorded exception** with
  the boundary: permitted = rendering guidance views + deterministic, advisory structural
  conformance checks on artifacts (exit-code style, feeding the grader seat); banned =
  gating pipeline progress, dispatching or sequencing agents, holding judgment that
  skills own. The general retirement/softening of the no-kernel non-negotiable is
  **`Deferred`** to its own session, trigger: when the Tauri work gets real. The stated
  willingness to move away stands on record as context; the session it anticipated was
  dissolved into D11 at review disposition. `Confident` on
  the exception + boundary (user-ruled after explicit scope pin); `Deferred` on the
  general position. **As amended at review disposition (spine ruling "own the real
  driver"):** the deferral dissolves — superseded by D11; the boundary line survives
  inside D11 as the standing bright line for kernel-class tooling.

- **D10 — Build stages as one wave: CLI + all 8 conversions in a single landing.** The
  Rust CLI, all 8 schema data files, every skill re-point, and the checklist re-keys land
  together — no split-brain period where some in-scope templates are `.md` and some
  schema. Costs on record: the first Rust code, the first build pipeline, and 8 template
  supersessions land untested together (untested = no live dogfood run yet — distinct
  from M6's unit-test gate on the crate). Lead recommended a spec-template pilot first;
  user chose one wave. `Contested` (deliberate user choice against recommendation).

- **D11 — No-kernel position ruled now: softened, skills-first, foundation owned.**
  Born from the review's spine disposition (dissolving D9's deferral): CLAUDE.md's
  no-kernel non-negotiable is **softened by recorded ruling** — skills and agents remain
  the primary quality surface; kernel-class executable tooling is admissible where a
  recorded ruling justifies it, under the standing bright line carried over from D9
  (such tooling never gates pipeline progress, never dispatches or sequences agents,
  never holds judgment that skills own). The Rust CLI is ruled in openly as mochiko's
  **foundation seed for future native tooling (Tauri-bound)**, with template guidance as
  its first workload. Honest concession on record (from Critical C1): after D4/D5,
  template delivery alone would not carry this machine — shared-reference `.md` files
  plus the existing `.py` lint could serve it; the machine is carried by the foundation
  bet. Exact CLAUDE.md rewording lands at build via the governance amendment path.
  `Confident` (user-ruled at review disposition, option presented with full reviewer
  steelman).

## Session trail

- **Q1 (mechanism):** What machine delivers the schema at runtime? Options: script per
  template (detect-stack.sh precedent) · one plugin CLI · no real CLI (structured data +
  Read) · external CLI. **A:** one plugin CLI → D1.
- **Q2 (constraint):** Reconcile CLI with no-kernel non-negotiable — compatible-with-
  recorded-reasoning (lead recommendation) · amend constraint (governance event) · rethink
  mechanism. **A:** amend constraint → D2, `Contested`.
- **Q3 (scope):** Which templates convert — pipeline artifacts first (recommended) ·
  everything · artifacts + seat reports. **A:** pipeline artifacts first → D3.
- **Q4 (output shape):** What does the CLI emit — schema+skeleton (recommended) · rendered
  guidance only · structured schema only. **A (user-authored):** schema + example +
  guidance (good/bad, verbosity norms); `--depth` flag rejected — agent never selects
  depth, CLI guides it → D4.
- **Q5 (variation source):** Project config (recommended) · baked into CLI · artifact
  context. **A:** baked into CLI → D5, `Contested`.
- **Q6 (runtime):** Node (recommended) · bash · compiled binary · Python. **A
  (user-authored):** "why not rust? i am hoping to use rust so i can build a Tauri UI
  later" → D6.
- **Q7 (validators):** Role-shaped grading view (recommended) · same output · keep
  separate checklists. **A:** role-shaped → D7.
- **Q8 (fallback):** Data readable raw (recommended) · hard fail · keep `.md` fallback.
  **A:** data readable raw → D8.
- **Q9 (boundary):** Carve-out reach — emit + mechanical lint (recommended) · emit only ·
  general wording. **A (user-authored):** "i am okay to move away from no kernel
  position" — reframe above every offered option.
- **Q10 (scope pin, plain-language):** How far does moving away go — soften ·
  retire fully · this CLI only, decide later. **A:** this CLI only, decide later → D9
  (exception carries the Q9-recommended mechanical-lint boundary).

- **Q11 (staging):** Pilot spec-template first (recommended) · one wave all 8 · CLI first
  convert later. **A:** one wave, all 8 → D10, `Contested`.
- **Q12 (review disposition):** Spine route — own the real driver · explore null road now
  (reviewer-recommended) · route out and re-decide · sustain as ruled; minors batch.
  **A:** own the real driver → D11 + D2/D6/D9 amendments; minors "as recommended" →
  F3/F5 amendments + M6/M7 build-surface additions.

## Cold review + dispositions

Solo cold review, blind-map two-message dispatch (26-angle map, topic-only spawn; map
returned before the record path was sent). Reviewer source-verified its repo claims; the
lead re-verified the two load-bearing ones (`ls` of skill scripts; `artifact-format.md`
referenced from 25 files) before disposition. **Verdict: critical-gaps.** ~10 candidates
raised, 3 reviewer-killed, 7 survived: 1 Critical, 3 Important, 3 Minor. 7/7
dispositioned — spine (C1/I2/I3/I4) user-ruled **"own the real driver"**; minors + fact
repairs user-ruled in one batch "as recommended."

**Verify round 1 (reopen-born D11, bounded): CLEAN** — all three checks PASS (post-fold
consistency · D11 record-fitness · 7/7 traceability); 5 non-blocking nits, 4
lead-repaired same round (D6 stale D2-clause re-pointed to D11's bright line · D9
dangling "that session" closed · figure-basis note below · D10 "untested" word-clarified),
1 left by convention (topic header preserves the original runtime-parameterization
framing; D11 records the divergence). Figure basis: `artifact-format.md` is consumed by
25 files (grep -l); the reviewer's 17× counted literal `templates/artifact-format.md`
path occurrences — one metric, two denominators, C1 unaffected; canonical figure for
downstream docs: **25 consuming files**.

| # | Sev | Finding (target) | Disposition |
|---|-----|------------------|-------------|
| C1 | Critical | Machine's justification removed mid-session (D4/D5); null road — shared-`.md` + existing `.py` lint, no CLI — never on a menu (targets D1/D4/D5/D6) | Dissolved by D11: driver owned openly as the foundation bet; concession recorded in D11 that template delivery alone would not carry the machine |
| I2 | Important | F3 omitted the 5 `.py` advisory validators — strongest fact for "no amendment needed"; D2 ruled without it (targets F3/D2/D9) | F3 amended; the amendment question mooted by D11 (softening happens for the owned driver, not the lint capability) |
| I3 | Important | Rust sized for a deferred Tauri hope, not the template problem (targets D6) | D6 amended: coupling made open and user-ruled; decoupling route declined |
| I4 | Important | "One-off exception" (D9) contradicts "foundation seed" (D6) | Resolved by D11: deferral dissolved, the strategic decision made here — contradiction gone |
| M5 | Minor | F5 bloat evidence cuts against the CLI (fixed statically both times) | F5 demoted to neutral context |
| M6 | Minor | No quality gate for inbound Rust code | Build surface gains the Rust quality gate (below) |
| M7 | Minor | No rollback path for a one-way wave on n=0 | Build surface gains the rollback statement (below) |

## Build surface

*(lead-drafted from the rulings; build-time detail stays builder's room)*

- New Rust crate — mochiko's first non-markdown code (layout builder's room, e.g.
  `crates/mochiko-cli/`); first build pipeline; **CI arrival trips CLAUDE.md's standing
  governance amend trigger** — governance amendment due at the same landing.
- CLAUDE.md no-kernel non-negotiable is **reworded per D11** (softened: skills-first,
  kernel-class tooling by recorded ruling, standing bright line) — governance event via
  the amendment path; the core-bet sentence updates to match. *(Supersedes the earlier
  exception-line shape.)*
- **Rust quality gate (M6):** the crate lands with its own test suite; an independent
  non-author seat reviews the code (author≠grader extends to code artifacts); tests wire
  into the arriving CI as the repo's first executable gate.
- **Rollback statement (M7):** trigger — the first-live-run watch shows CLI-delivered
  guidance underperforming the `.md` baseline on artifact quality; cost — revert the 8
  template supersessions (reconstructible from strips per GI-006) and re-point skills
  back; the crate itself may survive independent of template delivery under D11's
  foundation ruling.
- 8 schema data files (format builder's room: YAML/JSON) shipped in the plugin (D8).
- 8 `.md` template deletions = supersessions-by-ruling — strip entries per
  `.mochiko/strips/` ceremony, author≠grader audits per the primitive-edits rule.
- Skill/command re-points: every surface referencing the 8 in-scope templates (grep at
  build; ~20 files reference `templates/` paths today, not all in scope).
- `review-plan-artifacts/references/ARTIFACT-CHECKLISTS.md` re-keys to the `--check`
  view (D7).
- Release gates as always: `plugin.json` bump · CHANGELOG · marketplace sync · audits
  PASS · strips recorded · landing ritual complete.

## Open threads

- **Binary distribution mechanism** — build-from-source on install vs release-artifact
  download vs committed binaries; builder's room at plan time, but the chosen mechanism
  must keep D8's raw-Read fallback honest.
- **Schema data format** (YAML vs JSON vs TOML) — builder's room.
- **Later ratchet** (D3): seat report templates, in-skill reference templates,
  `constitution-modules/` — each conversion its own recorded landing.
- ~~**Deferred session** (D9): general no-kernel retirement/softening; trigger — Tauri
  work gets real.~~ **Closed at review disposition — D11 pulled it into scope and ruled
  it here.**
- **Evidence honesty:** n=0 — no run yet demonstrates CLI-delivered guidance outperforms
  `.md` exemplars; F5 is neutral (M5); the null-road concession stands in D11 — the
  machine rides the foundation bet, not template-side evidence. First-live-run watch owed
  after build.
- **D5 reopen condition:** if a real per-project depth need emerges (e.g. the governance
  depth dial and template guidance visibly diverge), the baked-in ruling reopens by
  explicit user ruling only.
