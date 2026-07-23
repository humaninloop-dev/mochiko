# Workflow Token Reduction — Decision Record

**Session:** 2026-07-23 · `/mochiko:brainstorm`
**Topic:** Avenues for reducing token usage when running mochiko's workflows.
**Status:** ACCEPTED (2026-07-23) — pair review complete (14 raised → 11 merged survivors → 11/11 dispositioned, verify pass CLEAN, clearing verdict ready); user accepted the record. **Landing (user-ruled):** ROADMAP Key Decisions row (workflow-token-reduction, epic-weight) landed 2026-07-23; **BACKLOG deliberately deferred** — the user re-examines the issues from more angles before locking build items; build scoping re-opens from this record. This record is the session deliverable.

## Scope

- **Reality surface:** yes — the plugin source (`plugins/mochiko/` commands, agents, skills, templates: what each run loads and spawns) plus prior session records carrying real token measurements (the v2.1 brainstorm run measured ≈654k out; the v2.2 revision was itself a token-efficiency pass on one workflow). Fact-checker seat filled at start. **Expanded at Q1:** the kinako dogfood repo (`github.com/humaninloop-dev/kinako`) — its `specs/` folder holds the artifacts the token-heavy runs actually produced; fact-checker dispatched to map it.
- **Session constraints (user, Q1 — Confident):**
  1. Trigger: the kinako dogfood runs — *all* workflows ran token-heavy there; scope is library-wide, not one command.
  2. Latency is not a constraint — trades that make a run slower but cheaper in tokens are acceptable.
  3. Target is **low-hanging fruit** — high savings-to-effort ratio, not a redesign.
  4. Quality floor: no drastic quality hit; modest, understood quality trades may be offered but the user arbitrates each.
- **Decision namespace:** D1…
- **Prior rulings that bound this session** (not re-litigated): brainstorm v2.2's efficiency folds — review sizing gate (pair/single/none), lens-split pair, map-lands-verbatim, one-shot cross-exam, verify-on-integrity-reviewer-only, synthesis on request (`.mochiko/brainstorms/brainstorm-v2-2-revision/record.md`).

## Fact-checker map (verbatim — fact-checker seat, 2026-07-23)

**Token formula:** chars/4 (= bytes/4 for ASCII source), applied consistently. Byte/line/word counts are exact (`wc`); every token figure is marked `est.` All paths under `plugins/mochiko/` unless noted.

### 1. Workflow inventory

All 7 commands in `commands/` are **team-form** (frontmatter `disable-model-invocation: true`; each hard-requires `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` and refuses without it — verified: 7/7 Read `templates/command-shape.md`, 7/7 name `mochiko:loop-discipline`). No one-shot commands exist. Seats (agent × review/author skill · spawn timing · standing vs cold):

| Command | B / lines | Seats spawned |
|---|---|---|
| `brainstorm.md` | 9,603 / 47 | **fact-checker** (conditional on reality surface, at start, spawned `general-purpose` per v2.1 record — no persona file, no skill) · **reviewer(s)** `devils-advocate`×`review-brainstorm`, **at convergence only**, 1–2 per user sizing. Lead runs `analysis-iterative` inline. |
| `specify.md` | 8,805 / 138 | **producer** `requirements-analyst`×(`authoring-requirements`+`authoring-user-stories`), standing, first seat · **critic** `devils-advocate`×`review-specifications`, cold at first critique. Lead runs `analysis-iterative` inline for sparse enrichment. |
| `plan.md` | 17,150 / 229 | **producer** `technical-analyst`×4 skills, standing both phases, first seat · **feasibility reviewer** `principal-architect`×`review-feasibility`, cold, **once** (Phase 1; re-fire only on structural change) · **completeness reviewer** `devils-advocate`×`review-plan-artifacts`, standing both phases, cold at first review. |
| `tasks.md` | 15,239 / 207 | **producer** `task-architect`×`patterns-vertical-tdd`, standing both phases, first seat · **reviewer** `devils-advocate`×`review-task-artifacts`, standing both phases, cold at first (mapping) review. |
| `implement.md` | 18,026 / 233 | **producer** `staff-engineer`×(`executing-tdd-cycle`+`brownfield-integration`), standing across all cycles + fix-pass, first seat · **verifier** `qa-engineer`×`testing-end-user`, standing across all cycles + final validation, cold at first cycle verification. (Cycles run sequentially — N cycles, still 2 seats.) |
| `setup.md` | 18,942 / 245 | **producer** `principal-architect`×(`authoring-constitution`+`analysis-codebase`), standing, first seat in brownfield · **intent reviewer(s)** `devils-advocate`×`review-governance-intent`, cold at synthesis review, 1–2 per sizing · **validator** `mochiko:validator`×`validation-constitution`, cold at first validation. Lead runs `analysis-iterative` inline (interrogation). |
| `slice.md` | 11,644 / 161 | **producer** `task-architect`×`authoring-slices`, standing, first seat · **reviewer** `devils-advocate`×`review-slices`, cold at first review. |

Spawn-count per run: brainstorm 1–3 · specify 2 · plan 3 · tasks 2 · implement 2 · setup 2–4 · slice 2.

### 2. What a single run loads

**Lead-side doctrine tax — paid on every run.** `command-shape.md` is a guaranteed read ("Read … before anything else"): **10,160 B / 148 lines (~2,540 tok est.)**. `agent-dispatch.md` near-certain for team-form (seat transport/briefing): **6,543 B / 98 lines (~1,636 tok est.)**. `loop-discipline` SKILL.md is cited as governing doctrine but the Contract section is its authoring-time fill — loads only if the lead invokes the skill (not guaranteed per run): **12,069 B / 138 lines (~3,017 tok est.)**. All three: **28,772 B (~7,193 tok est.)**. Per command-shape.md:130 + agent-dispatch.md:24, these are **lead-only** — teammates don't load `skills:` frontmatter and agent-dispatch is a caller-side checklist, so seats do **not** re-read them.

**Heaviest single seat load — plan's `technical-analyst`:** persona `agents/technical-analyst.md` 13,243 B + 4 SKILL.md (`authoring-technical-requirements` 15,955 · `patterns-technical-decisions` 7,429 · `patterns-entity-modeling` 16,873 · `patterns-api-contracts` 13,707) = **67,207 B SKILL-only (~16,802 tok est.)**; if every `references/` bundle loads (+105,354 B) → **172,561 B (~43,140 tok est.)**. Next: setup producer persona 8,606 + `authoring-constitution` 25,087 + `analysis-codebase` 14,008 = 47,701 B SKILL-only (~11,925 tok est.); `authoring-constitution/references/` alone is 63,803 B (heaviest reference bundle, catalog cards filtered by type/tier). implement verifier `qa-engineer` 5,131 + `testing-end-user` 14,231 + refs 25,832 = 45,194 B (~11,299 tok est.).

**Runtime report/format templates the lead fills or reads:** analyst-report 2,209 · advocate-report 1,725 · techanalyst-report 3,565 · feasibility-report 5,439 · taskarchitect-report 4,403 · slicer-report 3,530 · plan-template 1,680 · tasks-template 8,225 · spec-template 431 · slices-template 6,252 · governance-intent-template 8,127 · governance-surfaces-template 7,626 · codebase-analysis-template 4,362 · constitution-modules/ (4 files, ~13,264). Scripts (`detect-stack.sh` 13,040, `validate-openapi.py` 17,361, `validate-model.py` 16,540, etc.) are **executed via Bash, not context-loaded** unless an agent Reads them.

### 3. Repeat-load surfaces

1. **The doctrine tax recurs on every run of every command** (~7,193 tok est. lead-side) and again on every `/resume` — teams don't survive resume (command-shape.md:108), so a paused run re-reads it. Not repeated *within* a run (lead-only, read once).
2. **Pipeline input artifacts, re-Read cold by each downstream non-standing seat.** `spec.md` is authored by specify, then Read by plan's producer + 2 reviewers, tasks' producer + reviewer, implement's producer + verifier — ~10 agent contexts across one feature. Same for `data-model.md`, `contracts/api.yaml`, `plan.md`. Standing seats carry the file across rounds (no re-read); cold reviewers Read fresh at each spawn.
3. **SKILL.md persistence.** Per `constitution-native-surfaces/record.md:76`: once invoked, "the rendered SKILL.md content enters the conversation… and stays there for the rest of the session" — a recurring cost. This is exactly why the standing-seat design is cheaper than re-spawn, and why plan's producer holding 4 heavy skills is a sustained per-round cost, not a one-time one.
4. **The brainstorm reality-surface triple-read is already measured AND fixed** (see §4, F2 / v2.2). The identical cold-reviewer-re-reads-the-artifacts pattern is **unmeasured** in specify/plan/tasks/implement/slice, whose cold reviewers Read the artifacts fresh at each spawn.

### 4. Prior measurements and standing efficiency rulings

**Every hard token number the repo holds is from `brainstorm`.** A grep for measured run costs of specify/plan/tasks/implement/slice/setup-team-form returned **empty** — those are all recent conversions (git log 2026-07-19→07-21); a team-form `setup` run is explicitly "still pending" observation (`BACKLOG.md:25`). The only non-brainstorm datapoint is the governance-injection probe.

Measured runs (transcript forensics, output tokens):
- **brainstorm v1: ≈620k** across four cold dispatches, "each re-read everything" (`brainstorm-command-rewrite/record.md:16`; `BACKLOG.md:196`; `ROADMAP.md:170`).
- **brainstorm v2** (standing episodic advocate, died pre-convergence): **not cheaper than v1** — advocate 118k out / 2.70M cache-write / 11.2M cache-read; grounder 25k; lead 502k (`brainstorm-v2-revision/record.md:21`). **3:1 machine-to-user traffic**; "every message costs a full teammate turn (context reload, usually a full record re-read, a reply)" (`…v2-revision/record.md:18`).
- **brainstorm v2.1** (first *completed* run, setup-constitution-flexibility, 2026-07-05): **≈654k total out** — lead 355k / fact-checker 29k / reviewer-a 144k / reviewer-b 126k (`brainstorm-v2-2-revision/record.md:17`). **Review pair alone ~270k out + 6.2M cache-write — the dominant pool.**
- **F2 — reality surface read three times over:** fact-checker Read 14 files, reviewer-a 16, reviewer-b 11; "each reviewer rebuilt the file context the fact-checker already held" (`…v2-2-revision/record.md:18`).
- **fact-checker seat cost ~5% of run** (~25–29k out), "the cheapest seat by 5–20×" (`fact-checker-seat/record.md:14`).
- **governance-injection probe (kinako dogfood, 2026-07-19): ~220k subagent tokens** (`ROADMAP.md:82`; `REGISTRY.md:89`; `BACKLOG.md:143`).

Standing rulings — **already decided; not to re-litigate** (`workflow-token-reduction/record.md:11` names these as binding; source `brainstorm-v2-2-revision/record.md` D1–D5):
- Reviewers consume the fact-checker's **verbatim-landed map** instead of re-reading the tree (kills F2); cross-exam → **one-shot four-message**; **verify pass on the integrity reviewer only**. **"Est. pair cost ~270k → ~150–170k"** (`…record.md:32`), total target materially under 654k (`…record.md:68`).
- Review **sizing gate** (pair / single / none-with-waiver); **lens-split** pair (decision-quality vs record-integrity); `synthesis.md` on request only.
- Shared team-form prose is **single-sourced** to `command-shape.md`; commands reference, never restate it (pattern-codification-and-minimalism ruling; per-command relocation logs in `.mochiko/strips/`). This is a completed minimalism ruling.
- Standing-seat over re-spawn: the setup + specify→implement team-form conversions bet on **persistent seat context** (no cold re-reads across rounds), teammate↔teammate messaging deliberately unused (`ROADMAP.md:194`, `BACKLOG.md:25`).

### 5. Other file-grounded facts (cut both ways)

- **Total plugin footprint on disk: 117 files, 1,020,160 B (~255k tok est.)** — but a single run loads a small subset; footprint ≠ per-run cost. By category: **skills 759,320 B (80 files, ~190k tok est.)** dominate; commands 99,409 B (7); templates 91,313 B (20); agents 69,349 B (9).
- **Skill-description manifest overhead — the one fixed per-session tax paid regardless of which command runs:** 27 mochiko skills; description frontmatter totals **25,874 chars (~6,468 tok est.)**, auto-loaded into every session. (Separately, ~37 `humaninloop:` skills from another installed plugin also auto-load descriptions — additional per-session cost, not mochiko's footprint.)
- **CLAUDE.md auto-load:** `mochiko/CLAUDE.md` 5,542 B / 71 lines (~1,386 tok est.). **No governance region present** (0 `mochiko:governance` markers) → no governance auto-load in this repo currently. (Parent `GitHub/CLAUDE.md` 19,131 B is the sandbox project file, not mochiko's.)
- **Largest single files:** `skills/mochiko/SKILL.md` 27,156 B / 152 lines (the discoverability router — 206-char description but heavy body) · `authoring-constitution/SKILL.md` 25,087 · `patterns-api-contracts/references/OPENAPI-TEMPLATE.yaml` 23,943 · `setup.md` 18,942 · `patterns-vertical-tdd/references/CYCLE-STRUCTURE.md` 18,589 · `analysis-iterative/references/ADAPTIVE-EXAMPLES.md` 18,436 · `implement.md` 18,026.
- **Runtime vs authoring-only templates:** `workflow-contract.md` (3,272 B) is an authoring form only — command-shape.md:102 states "No per-run contract file is written"; not loaded at runtime. **`.mochiko/strips/` (17 files, 975 lines) is deliberately NON-LOADED** — README: "no command or runtime surface references it — the log must never itself cost context." Zero runtime cost.
- **HTML-comment headers in the two lead-obligated templates load in full:** `command-shape.md` header 1,373 B (~343 tok est., 14% of file); `agent-dispatch.md` header 1,783 B (~445 tok est., 27% of file). Comment stripping applies to `@path`/CLAUDE.md imports (`constitution-native-surfaces/record.md:66`), **not** to Read-tool template loads — so these headers cost context every time the lead reads the template.

### 6. Kinako dogfood output forensics (addendum)

Source: `github.com/humaninloop-dev/kinako` (cloned read-only into the scratchpad; not on local disk). A Dart project. One feature run to date, **slice-scoped**: `.mochiko/specs/mvp-thin-loop/`, decomposed into 3 slices (s1 foundation, s2, s3). Sizes exact `wc`; token figures chars/4 `est.`

**Caveat, load-bearing:** these are **on-disk artifact sizes, not tokens consumed by the runs.** Real run cost is a large multiple — each artifact was authored across ≤3 produce↔review rounds (drafts + revisions), read by the producer's upstream inputs, and read again by each cold reviewer that graded it. No transcripts are in the repo. **No run telemetry exists in the artifacts** — a grep for token/cost/timing across all specs + brainstorms returned only domain content (credential/commit "tokens"), zero run self-instrumentation. So kinako gives **artifact-volume** forensics, not token-consumption telemetry — consistent with mochiko's own finding that run cost is reconstructable only from transcripts (§4).

**Full residue:** `.mochiko/` = **70 files, 1,302,916 B (~325,729 tok est.)**. Of that, `.mochiko/specs/mvp-thin-loop/` = **64 files, 1,167,512 B (~291,878 tok est.)** — one partially-complete feature.

**Per-stage artifact volume (feature root + 3 slices):**

| Stage / location | Bytes | ~tok est. |
|---|---|---|
| Feature-root design (spec 42,557 · requirements 61,024 · constraints-and-decisions 66,809 · data-model 42,207 · api.yaml 45,837 · nfrs 27,496 · quickstart 17,519) | 303,449 | 75,862 |
| Feature-root reports (slices.md 17,221 · slicer-report 14,695 · analyst-report 6,407 · advocate-report 9,066) | 47,389 | 11,847 |
| **s1 foundation:** plan-artifacts 109,640 · round-reports 46,086 · **12 cycle-reports 176,070** · **13 verification-reports 158,221 B (12 per-cycle 149,715 + final-validation 8,506)** | 490,017 | 122,504 |
| s2: plan-artifacts 75,654 · round-reports 58,894 · 3 cycle-reports 43,311 · 3 verification-reports 29,036 | 206,895 | 51,724 |
| s3 (planned+tasked, not implemented): plan-artifacts 66,324 · round-reports 53,438 | 119,762 | 29,940 |

**Headline: reporting about the build outweighs the design of the build.** The 15 cycle-reports + 16 verification-reports total **406,638 B (~101,659 tok est.)** — larger than the entire feature-root design artifact set (303,449 B / ~75,862 tok est.). s1 alone (foundation, 12 cycles) is **490,017 B (~122,504 tok est.)** — the single largest cost center.

**Fan-out (report artifacts per feature): 47** — 3 feature-root (analyst / advocate / slicer) + 13 round-reports (s1: 5, incl. both a plan-stage `advocate-report.plan.md` and a tasks-stage `advocate-report.md`; s2: 4; s3: 4) + **15 cycle-reports** (s1:12, s2:3) + **16 verification-reports** (s1: 13 = 12 per-cycle + 1 final-validation; s2: 3). Cycles run: 12 + 3 + 0 = **15**, each producing a cycle-report AND a verification report (the execute→verify pairing, §1).

**Repetition surfaces:**
- **Traceability-ID re-quoting is pervasive.** Across the 64-file set: FR- IDs appear **1,496×** (57 files), C- **894×** (54), TR- **870×** (40), D- **721×** (45), NFR- **626×** (43), GI- **453×** (56), SC- **335×** (43) — **~5,395 ID mentions total.** A single requirement is re-quoted down the chain: requirements → plan artifacts → task-mapping → tasks → cycle-reports → verification-reports → advocate-reports.
- **Per-report boilerplate.** The 15 cycle-reports share a rigid skeleton — 15/15 carry `### What Was Done` + `### Decisions Made`, 14/15 `### Notes for Next Cycle`, 12/15 `### Local gate results`. The 16 verification-reports share `Setup / Actions / Asserts` + `Quality gates (deterministic exit-code checks)` + `Checkpoint recommendation` + `Classification & recommendation`. The scaffold repeats per report, ×15 / ×16.

**Governance surfaces auto-loaded on a kinako run:**
- `CLAUDE.md` **8,990 B**; its **governance region 5,523 B / 55 lines (~1,380 tok est.)** (2 markers present) loads for every session + every spawned agent (setup.md:180).
- `.claude/rules/mochiko/` — **8 files, 14,675 B (~3,668 tok est.)**, `paths`-scoped (load when a matching file is touched): architecture-layers 6,901 · storage-invariants 1,990 · sandbox-boundary 1,355 · dependencies 1,122 · module-boundaries 884 · telemetry-privacy 832 · config-writes 816 · capture 775.
- `.mochiko/memory/` — **86,469 B (~21,617 tok est.)**: governance-ledger 47,386 (read by setup/validator only), governance-intent 26,174, trace-summary 12,909.
- Brainstorm residue: `.mochiko/brainstorms/kinako-product-architecture/` — record.md 42,975 B + synthesis.md 5,063 B (a brainstorm dogfood also ran).

**Largest single artifacts:** constraints-and-decisions.md 66,809 B · requirements.md 61,024 · s1/tasks.md 53,785 · governance-ledger.md 47,386 · s1/task-mapping.md 45,919 · contracts/api.yaml 45,837 · brainstorm record.md 42,975 · data-model.md 42,207.

- **Q2 (user — Confident):** no per-stage cost view — "they all were token expensive." No felt ranking exists; stage ranking must come from the kinako forensics + structural analysis. Consequence: the hunt targets patterns common to all workflows, not one command.
- **Q3 (user — Confident):** the kinako runs used **only mochiko — no humaninloop plugin**. The legacy plugin's ~37 auto-loading skill descriptions were not part of the observed cost; dropped as an avenue for the observed heaviness (remains one-line environment hygiene wherever it *is* still installed, e.g. the mochiko dev repo itself).
- **Q4 (user — Confident):** the user **never read** the kinako cycle-reports or verification-reports. User-side, the per-cycle report volume (~102k tok est. on disk, the largest cost center) is write-only residue. Machinery-side consumption (retry, fix-pass, next-cycle handoff, checkpoint presentation) dispatched to the fact-checker (F-c) before the slimming avenue is sized.
- **Q5 (user — Confident)** *(logged at review, S3 — the exchange happened pre-review but was not captured as an event)*: asked whether the avenue set should include a minimal per-run measurement row; lead recommended yes. **User: "i will go with your recommendation."** → became D2.
- **Avenue-map arbitration (user — Confident)** *(logged at review, S3)*: the full avenue map (five rows + recommended order D3 first, D4 second, then D5/D6; plus the cleared non-avenues) was presented for keep/drop/re-rank. **User: "i will go with your recommendation"** — wholesale adoption, no demotions, no re-ranking. This event is the ratification basis of D3–D6's `Confident` marks; its three embedded quality-relevant trades were then individually walked back and confirmed at Q6 ("no softening").

## Dispatched fact-checks (verbatim — fact-checker seat)

### F-a. Reference-load gating

**No skill instructs a blanket read of its `references/` dir** (grep for "read all/every references" etc. → zero hits). The dominant instruction is the **conditional pointer** `See [X](references/…) for detailed/complete Y` — **25 such "See" lines** across the target skills, vs **1 "Load"**, **2 "Read the format"**, **3 "Execute all/the checklist"**. So: authoring/pattern (producer) skills are **conditional/discretionary**; reviewer/validator skills are **obligated** (the checklist/lens *is* the method).

**Producer/authoring skills — every reference is a conditional "See X for details" pointer:**
| Skill · reference (B) | Class · instruction line |
|---|---|
| authoring-technical-requirements · ARTIFACT-TEMPLATES 14,775 | COND — "See … for complete field definitions and examples" (L41) |
| · TRACEABILITY-PATTERNS 11,537 | COND — "See … for detailed cross-reference patterns" (L138) |
| patterns-technical-decisions · EVALUATION-MATRIX 4,782 | COND (L55) |
| · DECISION-RECORD 5,012 | COND — "See … for full ADR format" (L94) |
| patterns-entity-modeling · DATA-SENSITIVITY 7,320 | COND, only when Confidential/Restricted attrs (L156, L199) |
| · RELATIONSHIP-PATTERNS 5,737 | COND (L162) |
| · STATE-MACHINES 4,297 | COND + "if applicable" (L181, L332) |
| · VALIDATION-RULES 6,845 | COND (L195) |
| patterns-api-contracts · ERROR-PATTERNS 9,949 | COND (L130) |
| · PAGINATION-PATTERNS 11,157 | COND (L149) |
| · OPENAPI-TEMPLATE.yaml 23,943 | COND — "See … for a complete, copy-ready template" (L246) |

**plan-producer specifically (the sizing question):** all four skills' references total **105,354 B (~26,338 tok est.)** and **every one is behind a conditional "See X" pointer — zero mandated blanket read.** The producer pulls each on demand per artifact section (OPENAPI-TEMPLATE when assembling `api.yaml`, DATA-SENSITIVITY only for Confidential/Restricted attrs, STATE-MACHINES "if applicable"). The §2 "if every reference loads → ~43k tok est." figure is a genuine **ceiling the text does not mandate**; realistic load is the subset the feature exercises. Caveat: the SKILL.md bodies deliberately omit the single-sourced detail (field definitions, the OpenAPI template), so a producer authoring *well* will functionally pull the big ones even though no imperative forces it.

**authoring-constitution (9 refs, 63,803 B) — mostly mode/type-gated conditional:**
- catalog/ (17,171 B): COND, dealt during interrogation, **filtered by declared type/tier** ("both → references/catalog/", L97) · ESSENTIAL-FLOOR 7,318: COND (L100) · EMERGENT-CEILING-PATTERNS 11,435: COND **brownfield-only** (L103) · DOMAIN-DEPENDENCIES 5,159: COND **layered-architecture beat only** (L119, L286) · RFC-2119-KEYWORDS 6,181: COND (L239).
- **SYNC-IMPACT-FORMAT 8,983 and INTERROGATION-AGENDA 7,556 are UNMENTIONED in the SKILL.md body** (16,539 B). INTERROGATION-AGENDA is read by the **lead** directly (setup.md:28), not this producer skill.

**Skills where a reference IS obligated** (not discretionary):
- executing-tdd-cycle · CYCLE-REPORT-FORMAT 4,688 — "Produce cycle-report.md following the format in …" (L81). (TASK-PARSING 3,305 COND L40; TDD-ANTI-RATIONALIZATION 2,920 COND L136.)
- testing-end-user · TASK-PARSING 8,664 — near-obligated, the parse step (L55). (EVIDENCE-CAPTURE 5,844 COND L69; REPORT-TEMPLATES 6,880 COND L87; **TESTING-EVIDENCE 4,444 UNMENTIONED as obligated** — index-listed provenance doc L262.) Cross-skill: it **consumes** the TEST: grammar from `patterns-vertical-tdd/references/CYCLE-STRUCTURE.md` **18,589 B** (a *different* skill's reference — L16/47/63/73).
- patterns-vertical-tdd · CYCLE-STRUCTURE 18,589 — near-obligated ("This section and CYCLE-STRUCTURE.md teach how to fill it", L113). (SLICE-IDENTIFICATION 5,627 COND L92.)
- analysis-codebase · CONTEXT-GATHERING 5,505 COND (L110); also cross-reads `authoring-constitution/references/ESSENTIAL-FLOOR.md` for the canonical floor def (L124).
- **Reviewer skills (obligated — the checklist is the method):** review-feasibility · FEASIBILITY-LENS 7,817 — "**Load** … and look through each of the six lenses in turn" (L65) · review-plan-artifacts · ARTIFACT-CHECKLISTS 15,553 — "**Execute all** applicable checks from …" (L126) · review-task-artifacts · PHASE-CHECKLISTS 11,557 (L61) · validation-constitution · QUALITY-CHECKLIST 5,223 "**Read** …" (L49) + ANTI-PATTERNS 2,529 "**Compare against** …" (L123) · review-brainstorm · RECORD-FITNESS 1,624 (protocol step, L34) + CROSS-EXAM 2,444 (**pair-mode only**, L42). Their ISSUE-TEMPLATES refs stay COND ("See … for classification").

### F-b. Reviewer read scope — upstream re-reads are SKILL-MANDATED, not seat judgment

Most pipeline review skills **explicitly instruct reading upstream artifacts beyond the one graded** (for traceability / coverage / consistency). Quoted instruction lines:

- **review-plan-artifacts** (SKILL.md:80-83): *"Read and understand: The artifact being reviewed · **The spec / upstream requirements it should satisfy** · **Prior artifacts** (for consistency checks) · **Constitution principles** (for compliance)."* → grades a plan artifact but mandated to read spec + prior artifacts + constitution.
- **review-task-artifacts** (SKILL.md:91-95): *"Read and understand: The task artifact being reviewed (`task-mapping.md` and/or `tasks.md`) · **The spec / user stories** the tasks should deliver · **The plan artifacts** the mapping draws on · `task-mapping.md` when reviewing `tasks.md`."* → tasks review mandated to read spec + plan artifacts + mapping.
- **review-slices** (SKILL.md:61, 42): *"Read **both** files — `slices.md` *and* the `spec.md` it indexes. Coverage cannot be verified from [slices alone]"*; *"this review **reads `spec.md`** (it must, to verify coverage)"* (also L107, L131). → graded slices.md + upstream spec.md, mandated.
- **review-feasibility** (SKILL.md:61): *"Read the actual artifacts under review… For an analysis review: **requirements, constraints-and-decisions, NFRs**. For a design review: **add data-model and contracts**."* → cross-artifact by nature: grades across 3 (analysis) or 5 (design) plan artifacts. (No separate spec read named.)
- **review-governance-intent** (SKILL.md:42-45): *"Read the frozen synthesis, **the interrogation agenda** [+ brownfield] `.mochiko/memory/codebase-analysis.md`."* → graded synthesis + agenda (7,556 B) + codebase-analysis (brownfield), mandated.
- **testing-end-user** (verification role, SKILL.md:142): *"Identify quality gate commands from the `## Quality Gates` section of `tasks.md` and/or the build configuration in **`plan.md`**."* → reads tasks.md + upstream plan.md (for gate commands) + runs against the real codebase.
- **validation-constitution**: grades the surface set from the files + synthesis + trace summary (setup.md brief) + reads matching rules files (SKILL.md:74) — cross-file by construction.
- **review-specifications** (SKILL.md:95): *"Read the full specification before identifying gaps."* → **the only one that reads a single artifact** (spec.md) — because nothing is upstream of the spec.

**Net:** the cold-reviewer upstream re-read surface flagged in §3 is **text-required by the review skills themselves** (coverage/traceability/consistency checks name the upstream sources as mandatory context) — not discretionary seat behavior. The lone exception is review-specifications (spec is the pipeline's first artifact). Reporting the fact; the design implication is the session's to draw.

### F-c. Report consumption paths (cycle / verification / round reports)

## Part 1 — cycle-reports & verification-reports: who is instructed to read them

**The lead reads them, to form the verdict / confidence gate** (not the user):
- implement.md:119 — *"Confidence gate + verdict (you). **Read `cycle-report.md` + the verification report + qa's evidence.**"*
- implement.md:138 — *"Verdict (you). **Read the report** + confirm the done-condition's end state."* (final validation)
- implement.md:173 (done-condition) — *"**you Read the cycle-reports + verification reports** and confirm no blocking gap remains."*
- executing-tdd-cycle/SKILL.md:28 — *"Evaluating checkpoint or validation reports… **the lead Reads the reports** and owns that verdict."*
- executing-tdd-cycle/SKILL.md:87 — *"The frontmatter's structured fields are your self-report — **the lead reads them when deciding the cycle checkpoint**, and verifies independently."*
- executing-tdd-cycle/references/CYCLE-REPORT-FORMAT.md:3 — *"two consumption patterns: structured YAML frontmatter **the lead reads** when it decides the cycle checkpoint, and prose sections that carry human-readable context **for the lead and the next cycle**."*

**The user gets a summary, never the report files:** implement.md Phase 3 (G5) — *"Present the verified implementation (cycle / task / fix-pass counts, quality-gate results, an **evidence summary**, any noted gaps)."* A lead-authored summary, not the reports. **This matches the user's report of never reading them** — the machinery never routes a cycle/verification report to the user.

**Fix / retry mode: the producer reads the failure list it's handed, not the report file:**
- executing-tdd-cycle/SKILL.md:94 (fix mode) — *"Read the reported failures (**from the checkpoint or verification report you were given**)."*
- executing-tdd-cycle/SKILL.md:107 (retry) — *"Read the reported failures."*
- implement.md:112 — *"on round > 1 the message **carries the checkpoint's failed tasks** for targeted retry."* → the lead extracts/relays the failed tasks; the producer isn't sent to re-read the verification file.

**"Notes for Next Cycle" — authored for the next cycle, but nothing instructs reading the prior cycle-report file:**
- CYCLE-REPORT-FORMAT.md:53 / :65-67 declare the section is *"Information the next cycle's implementation needs to know."*
- But the producer is **one standing seat across the whole cycle sequence** (implement.md:47), and implement.md:53-56 states it *"carries the accumulating implementation forward… rather than re-reading the growing codebase cold each cycle."* So next-cycle context flows via the standing seat's **in-session memory**, not a file re-read. No line in implement.md or executing-tdd-cycle instructs a producer to Read a prior `cycle-report.md`; the recovery table's respawn re-reads *"the cycle's tasks + design inputs (+ any failed-task list)"* (implement.md:202) — prior cycle-reports are not on that list.

## Part 2 — round-reports (analyst / advocate / techanalyst / feasibility / taskarchitect / slicer): terminal once the round closes

**Consumed in-round only** — by the lead (verdict: plan.md:119 *"Read the artifacts + both reports"*, plan.md:141) and by the producer as a **relayed gap list**, not the report file (plan.md:56 / tasks.md:55 / specify.md:44 / slice.md:56 — *"carrying the reviewer's gap list verbatim"*).

**No downstream stage reads them** — the downstream input lists name **deliverables only, zero reports:**
- implement.md:83 — *"Read the design inputs (`plan.md`, `task-mapping.md`, `data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`, `requirements.md`)."*
- tasks.md:85-86 — *"Read plan's design outputs (`spec.md`, `requirements.md`, `constraints-and-decisions.md`, `nfrs.md`, `data-model.md`, `contracts/api.yaml`)."*

**Explicitly offered for cleanup as "intermediate"** (the terminal signal): plan.md:173-174, tasks.md:153-154, specify.md:96, slice.md:111 — *"Offer a … retain/clean choice for the intermediate round reports; never offer to delete [the deliverables]."*

## Part 3 — kinako actual-artifact evidence of consumption

Only two kinds of report cross-reference exist in the 64-file set, **neither a downstream content read:**
1. **Provenance-stamp summary lines inside the `plan.md` deliverables** — e.g. `slices/s1/plan.md:98-100`: *"Review trail (this slice): `feasibility-report.md` — feasible (round 2…) · `advocate-report.md` — ready (analysis round 2, design round 4…) · `techanalyst-report.md` — producer disclosure across 4 rounds."* (also s2/plan.md:87-89, s3/plan.md:84-86). One-line **outcome stamps**, not consumption of report content.
2. **Intra-stage citations** — `s1/advocate-report.plan.md:15` cites *"the round-4 `techanalyst-report.md`"* (a reviewer citing the same-round producer report); `s2/tasks.md:77,257` cite *"`taskarchitect-report.md`"* for the deltas the tasks.md itself discloses.

**Negative findings (no consumption):**
- **No cycle-report cites a prior cycle's "Notes for Next Cycle."** All 15 cycle-reports contain the `### Notes for Next Cycle` section (authored), but a grep for any back-reference ("as noted in cN", "prior cycle's note") returns **zero**. The section is written; nothing in the artifacts shows a later cycle reading it — consistent with the standing-seat design carrying that context in-session instead.
- **No final-validation or fix-pass back-cites the per-cycle verification reports.** `final-validation.md` does not reference `c1-verification`…`c12-verification`. The 12 per-cycle verification files (s1: 158,221 B) are not re-read as files after their cycle closes.

**Net (fact):** cycle/verification/round reports are consumed **by the lead in-round/in-cycle for verdicts**, and by the fix-mode producer only as a **lead-relayed failure list** — never presented to the user (summary only) and never read by downstream stages (which read deliverables) or, on the artifact evidence, by the next cycle. The "for the next cycle" purpose of the `Notes for Next Cycle` section is unverified by any artifact back-reference, given the standing-seat design that makes a file re-read unnecessary. The design implication is the session's to draw.

**Erratum — fact-checker, 2026-07-23 (F-c Part 3 byte attribution).** F-c Part 3 stated: *"The 12 per-cycle verification files (s1: 158,221 B) are not re-read as files after their cycle closes."* The byte figure is misattributed. Corrected: the **12 per-cycle** verification files (c1–c12) sum to **149,715 B**; **158,221 B is the 13-file figure** — the 12 per-cycle (149,715 B) **plus final-validation.md (8,506 B)**. The finding is otherwise unchanged: per-cycle verification reports are not re-read or back-cited after their cycle closes (the conclusion never depended on the byte figure). Map §6 totals are unaffected — the s1 "13 verification-reports 158,221 B" and the all-report "406,638 B (~101,659 tok est.)" both already count all 13 files, final-validation included; no total changes and there is no double-count.

### F-d. Claude Code OpenTelemetry (user-commissioned at the S1 ruling)

OTel fact-check, from the official doc **https://code.claude.com/docs/en/monitoring-usage** (quoted verbatim). Bottom line up front: **this materially upgrades my earlier hooks/statusline settlement** — Claude Code natively emits per-run, per-type, AND per-subagent token+cost data by config alone. But the "the run writes its own totals into the feature dir at run end" gap only *partly* closes. Facts:

**1. Native support, config-only (env or settings.json), zero mochiko code — YES.**
- `CLAUDE_CODE_ENABLE_TELEMETRY` — *"Enables telemetry collection (required)"* (set to `1`).
- `OTEL_METRICS_EXPORTER` — *"Metrics exporter types, comma-separated. Use `none` to disable"* → `console` / `otlp` / `prometheus` / `none`.
- `OTEL_LOGS_EXPORTER` — *"Logs/events exporter types"* → `console` / `otlp` / `none`.
- `OTEL_METRIC_EXPORT_INTERVAL` (*"default: 60000"* ms), `OTEL_LOGS_EXPORT_INTERVAL` (*"default: 5000"*), plus OTLP protocol/endpoint/headers vars.
- **settings.json:** the doc shows these in a `.claude/settings.json` `"env": { … }` block — *"Administrators can configure OpenTelemetry settings … through the managed settings file."* So it's declarative config, not a hook, script, or brain code.

**2. Metrics — per-run token counts by type, cost, per-model — YES, documented names.**
- **`claude_code.token.usage`** (unit: tokens) — *"Number of tokens used"*; *"Incremented after each API request"*; attributes: `type` (*"input" / "output" / "cacheRead" / "cacheCreation"*), `model`, `query_source` (*"main" / "subagent" / "auxiliary"*), `speed`.
- **`claude_code.cost.usage`** (USD) — adds `agent.name`, `skill.name`, `plugin.name`, `effort`.
- Also `claude_code.session.count`, `lines_of_code.count`, `commit.count`, `active_time.total`.

**3. Local dump without a collector — console exporter YES, but stdout, not a file.**
- `OTEL_METRICS_EXPORTER=console` (+ `OTEL_LOGS_EXPORTER=console`) prints to **stdout/console — no separate collector required**. Quick-start: `CLAUDE_CODE_ENABLE_TELEMETRY=1` · `OTEL_METRICS_EXPORTER=console` · `OTEL_METRIC_EXPORT_INTERVAL=1000`.
- **Nuance:** there is **no native "file exporter."** Console = stdout. Getting it into `.mochiko/specs/<feature>/usage.log` needs either redirecting the Claude Code process's stdout **at launch** (a shell/invocation step) or OTLP→collector / Prometheus scrape (`http://localhost:9464/metrics`) — infrastructure. Export is also **interval-based (60 s metrics / 5 s logs), not run-end**, so a clean per-run total is a post-aggregation of interval emissions.

**4. Per-seat attribution — YES at the subagent level (documented).**
- Token/cost metrics and the `api_request` event carry `query_source` (*"main" / "subagent" / "auxiliary"*, or *"a subagent name"*) and `agent.name` (*"Subagent type that issued the request"*), plus `session.id`, `user.*`, `organization.id`, `model`, `terminal.type`. So token/cost **is** attributable per request-source, not session-aggregate only.
- **Residual I can't close from this doc:** whether agent-TEAM teammates (the records call them "full sessions") surface as `query_source=subagent`+`agent.name` on one stream, or as **independent sessions each with its own `session.id`** (needing cross-session correlation). The doc documents per-subagent attribution; the team-teammate→attribute mapping isn't stated here. I can check the agent-teams doc if you want that pinned.

**5. Logs/events — YES, a per-API-call token+cost event.**
- **`claude_code.api_request`** — *"Logged for each API request to Claude"* — carries `input_tokens`, `output_tokens`, `cache_read_tokens`, `cache_creation_tokens`, `cost_usd`, `cost_usd_micros`, `model`, `duration_ms`, `request_id`, `query_source`, `agent.name`, `skill.name`. Correlatable via `prompt.id` — *"UUID … linking all events produced while processing a single user prompt."* Enabled with `OTEL_LOGS_EXPORTER=console` (or `otlp`).

**Sizing verdict for D2 (facts, not advocacy):**
- The **data** D2 wants — per-run tokens split input/output/cacheRead/cacheCreation, cost, and per-subagent attribution — **is emitted platform-native, by env/settings.json config alone: no mochiko code, no hooks, no MCP/brain.** Strictly better than the hooks/statusline finding (which had neither a cumulative token total nor a per-seat split).
- The **gap** for "the run records its own totals to the feature dir at run end": (a) console export is **stdout, not a file** — a file needs a launch-time stdout redirect (out-of-band wrapper) or a collector/scraper; (b) emission is **interval-based, not run-end** — a per-run total is an aggregation; (c) the workflow **command never sees the stream** — it can't read its own telemetry into context and write it, the same structural limit as the earlier paths.
- So D2 is **automatable "in-constraint" only if "config + a launch-time stdout redirect + a small parse step" counts as in-constraint.** Pure env/settings.json config gets the data flowing (and it's rich, incl. per-seat); turning it into a per-run number in the feature dir needs **one out-of-band consumer** (a redirect+parser, or a collector). Whether that consumer reads as "kernel infrastructure" (forbidden) or "harness config" (allowed) is the session's judgment; the doc facts are above.

**Reconciliation with my earlier settlement:** the narrow claims there stand — no hook carries usage (only `transcript_path`), and no command-readable usage value exists. What I under-weighted was the OTel pipeline, which *does* emit the richer per-run/per-seat data out-of-band. So "the platform can't expose it" was too strong for the OTel path; "the *command* can't capture-and-write it without an out-of-band consumer" remains correct.

## Decisions

### D1 — Hunt order: pure waste first, quality-machinery sizing second — `Confident`
*(Mark provenance — reconciled at review, S2: initially `Assumed` (lead-inferred); the avenue map built on this frame was adopted wholesale (see the Q5 and Avenue-map arbitration entries in the Q-series); the review flagged that ratification as implicit and inconsistently recorded; **explicitly endorsed by the user at the S2 survivor ruling, 2026-07-23: "d1 yes."** `Confident` on an explicit basis.)*
**Statement:** Avenues are classed and worked in two tiers. **Class A: pure waste** — tokens that buy nothing (repeat cold reads of already-digested context, wholesale reference loads where a subset is used, runtime-loaded comment headers, manifest overhead from an unused plugin). The waste portion is cut exhaustively — it carries no quality trade by construction; where a Class-A cut exposes a residual trade-bearing edge (as D3's Notes-for-Next-Cycle removal did), that edge is surfaced for explicit user arbitration, never silently cut — the Q6 walk-back is this rule in practice (tightened at review, S9). **Class B: quality-machinery sizing** — tokens that buy adversarial pressure (reviewer counts, review depth). Never hard-cut; where trimmed, trimmed via the already-ruled sizing-gate pattern (pair/single/none + recorded waiver) so the user chooses per run.
**Rationale:** follows directly from the user's Q1 constraints (low-hanging fruit; no drastic quality hit; user arbitrates quality trades).

### D2 — Every workflow run ends with a recorded cost entry — `Confident` (re-scoped at review, S1)
**Statement (re-scoped):** each workflow run ends with a recorded cost entry in the feature directory. **Baseline mechanism (user-ruled at S1): a manual protocol** — at run end the lead records the user-visible usage figure (e.g. from `/usage`, supplied by the user) plus the run-shape counts the lead itself observes (seats spawned, rounds, cycles, review sizing); deeper transcript forensics stay an on-demand, human-initiated step — the same method that produced every number this session used. Not automated, recorded with that limitation stated. **Upgrade path (ruled at the S1 close, 2026-07-23 — probe-then-graduate):** the F-d fact-check confirmed the data D2 wants is emitted platform-native by config alone (per-type tokens, cost, per-seat attribution — env vars / `settings.json` only), but with three gaps (console export is stdout, not a file; interval-based, not run-end; the session cannot read its own stream) and two doc-unclosable unknowns (console-exporter behavior in an interactive session; whether agent-team teammates attribute onto one stream or fragment into separate sessions). **Ruling: the manual baseline stands now; the build list gains a one-shot empirical OTel probe** — enable the documented config in a dogfood run, observe live console behavior, per-run aggregation, and teammate attribution — and the automated mechanism graduates from probe evidence, not speculation (the rules-delivery-probe precedent, constitution-native-surfaces). User adopted the recommendation.
**Rationale:** the repo's only hard numbers are brainstorm's; all six other workflows ran unmeasured (fact map §4) — this session had to rank avenues from artifact sizes as a proxy (map §6 caveat). Recorded numbers are also the only way to verify that any reduction shipped from this session actually worked. User adopted the measurement row at Q5. **Re-scope provenance (S1, Critical, fact-checked):** the original "captures its usage numbers" scoping was falsified — the platform exposes no session-readable cumulative token total (only a USD estimate + a live context snapshot), and an automated transcript parse would breach the kernel-free rule. The S1 cascade concern (D3+D6d strip the disk-residue proxy this session itself ranked by) is met by this floor: D3's structured frontmatter (counts, gate results) retains a structural proxy, and the forensics path stays available on demand.

### D3 — Slim implement's per-cycle reports to their verified consumers — `Confident` (priority 1)
**Statement:** `cycle-report.md` contracts to its consumed surface — the structured YAML frontmatter (checkpoint fields, gate results) plus one **deviations & notes-of-note** prose block; the unconsumed skeleton sections (`What Was Done` restating tasks.md, routine `Decisions Made` narrative, `Notes for Next Cycle`) are dropped, with non-obvious decisions and genuine next-cycle warnings folding into the deviations block. Verification reports contract to the evidence table (Setup/Actions/Asserts results), quality-gate results, and verdict + classification; the narrative scaffold is dropped. **Refinement (S8, user-accepted at review):** cycles that FAIL — a failed checkpoint or a verification reporting a blocking defect — keep the fuller narrative, since debug value concentrates exactly there; the slim format is the passing-cycle format. (This does not restore `Notes for Next Cycle`; its Q6-confirmed removal stands.)
**Rationale:** F-c — the sole live consumer is the lead's verdict (frontmatter + evidence); the user never reads them (Q4); the next cycle demonstrably never reads the file (15/15 authored `Notes for Next Cycle`, 0 back-references; standing seat carries the context); fix/retry consumes a lead-relayed failure list, not the file. ~102k tok est. on-disk per feature — the largest cost center **by artifact type** (the largest single *location* is the s1 slice at ~122.5k est.; and all such figures are disk-volume proxies for authoring + read cost, not measured tokens — tightened at review, S4/IF5) — plus the authoring cost paid in full-context seats. Honest reporting survives: the frontmatter self-report, deviations, and evidence are exactly what remains.
**Build surface:** `executing-tdd-cycle/references/CYCLE-REPORT-FORMAT.md`, `testing-end-user/references/REPORT-TEMPLATES.md`, the two skills' report sections.

### D4 — Reference-by-ID: kill restatement down the artifact chain — `Confident` (priority 2)
**Statement:** downstream artifacts cite upstream IDs (FR-/TR-/C-/NFR-/D-/SC-) without re-quoting their text — one-line context max where a bare ID would be unreadable; authoring templates and skills gain explicit no-restatement rules and per-section size guidance; each artifact keeps/gains a compact ID index as the coverage surface reviewers verify against.
**Rationale:** the multiplier avenue — kinako shows ~5,400 ID mentions and 43–67k B design artifacts for an MVP; F-b proves reviewers are *mandated* to read upstream artifacts, so every artifact kilobyte is re-paid at ~10 reads per feature. Traceability is the ID link, not the quoted text. Cheapens authoring, every mandated review read, and every downstream producer read at once.
**Ordering (composed at review, S4):** the D3-first / D4-second order survives the savings-to-effort composition the review demanded: D3 concentrates a large saving (largest artifact-type volume, authored ~once per cycle) in the set's smallest build (two formats + two skill sections) at low risk — the best ratio; D4's gross saving is likely larger (it rides the ~10× mandated-read multiplier) but costs the set's largest build and carries the moderate standalone-readability risk — high ratio, second. Both sides of the ratio are disk-based estimates (§6 caveat); the D2 measurement floor is what makes post-build validation of either possible.
**Build surface:** the authoring-* skills + artifact templates (largest build of the set).

### D5 — Review sizing gates generalized; verification depth split out with a floor — `Confident` (priority 3; split at review, S5)
**Statement:** every review-bearing stage gains the brainstorm-pattern sizing gate **for reviewer count** — pair / single / none-with-recorded-waiver, defaults keyed to feature weight — covering plan's two reviewers and the single reviewers of specify/tasks/slice. **Implement's per-cycle verification depth is a separate gate with a floor (S5, user-accepted):** depth may thin on light cycles (e.g. quality gates + spot evidence instead of full evidence capture), but never to zero — no cycle closes without real-infrastructure evidence. "None" exists only for reviewer count, never for verification.
**Rationale (regrounded at review, S7):** extends the already-ruled v2.2/setup sizing-gate pattern to the pipeline stages — opt-in per run, waiver-audited, quality trades user-ruled. The v2.1 measurement ("review machinery ~270k of 654k") is brainstorm-pair-specific evidence of what review weight *can* be, not a measurement of the mostly-single-reviewer pipeline stages (unmeasured, §4); the decision rests on the pattern-extension leg, with the split protecting verification's distinct risk class (a skipped second opinion loses judgment; a skipped verification ships unevidenced code).
**Build surface:** command texts of the five pipeline stages.

### D6 — Four small pure-waste fixes — `Confident`
**Statement:** (a) split the TEST: task grammar out of `patterns-vertical-tdd/references/CYCLE-STRUCTURE.md` (18,589 B) into its own small reference so implement's verifier loads only the grammar; (b) the two orphaned references (`SYNC-IMPACT-FORMAT.md` 8,983 B, `TESTING-EVIDENCE.md` 4,444 B) get linked from their skill bodies or archived to strips; (c) the runtime-loaded HTML comment headers of `command-shape.md` + `agent-dispatch.md` (~0.8k tok/run) relocate to the strip-note convention; (d) the existing end-of-stage retain/clean offer for round reports defaults to **clean**.
**Rationale:** F-a (a, b), map §5 (c), F-c part 2 (d — round reports are terminal; provenance stamps already live in the deliverables). Each trivial to build; (d) mildly thins audit residue and is flagged for the follow-through check.

### Q6 follow-through — the three embedded trades confirmed, no softening — `Confident`
The map adoption's three quality-relevant trades were walked back past the user once, each with a named softer variant on offer: (1) D3's removal of `Notes for Next Cycle` as a mandatory section (softer: optional field); (2) D6d's clean-by-default for round reports (softer: retain-by-default on foundation slices); (3) D5's added per-stage sizing interaction (softer: weight-keyed auto-sizing with override). **User ruling: "no softening" — all three stand as written.** Logged as the recommend-then-arbitrate follow-through; nothing amends these without the user's word.

### Considered and not pursued — `Confident`
Named to the user during the map presentation and at the wrap; recorded so the exclusions are choices, not omissions:
- **Wholesale reference loads** — cleared by F-a: no skill mandates blanket `references/` reads; the ~43k plan-producer figure is an unmandated ceiling.
- **Stopping cold-reviewer upstream re-reads** — F-b: the re-reads are the review method (coverage/traceability), not seat discretion. The v2.2 map-lands-verbatim precedent (named binding in Scope) does not transfer here (reworded at review, S6): pipeline reviewers are per-stage cold seats spawned into separate contexts that cannot share one landed copy, and once D4 shrinks the artifacts, a coverage-preserving digest has nothing further to compress out. Addressed via D4 (shrink what they must read), not by changing what reviewers verify against.
- **humaninloop plugin manifest** — not installed where the heavy runs happened (Q3); one-line environment hygiene elsewhere, no mochiko design work.
- **Lead-side doctrine reads** (~7.2k tok est./run) — small and already single-sourced; only the comment-header slice is actioned (D6c).
- **Skill-description manifest trimming** (~6.5k tok est./session, fixed) — deliberately excluded: trims trade against the model-invocation-reliability axis (graded trigger phrases) for a marginal once-per-session gain. Named as the loose end at the wrap; user accepted the wrap with it out.

## Review

- **Sizing gate (at convergence, 2026-07-23):** weight stated to the user — 6 decisions, all `Confident` (none contested/deferred), heavy fact surface (6-section map + F-a/F-b/F-c), build-consequential D3/D4. Options priced (pair ~150–170k est. / single ~half / none-with-waiver). **User ruling: pair** — lens-split per the command (decision-quality / record-integrity), cross-examination before findings land.
- **Record frozen at reviewer spawn** — edits suspended outside this section until every disposition lands.
- **Independent cold reads complete:** reviewer-decision formed 9 (1 Critical / 5 Important / 3 Minor); reviewer-integrity formed 5 (incl. sample audit of the map against plugin tree + kinako clone). Contents held until both counts were in; cross-examination opened 2026-07-23; closed 4/4 messages + one fact routed to the fact-checker (D2 platform capability — settled, decisive).
- **Reviewer tallies:** reviewer-decision 9 raised / 9 survived (recommended: critical-gaps, driven by F1). reviewer-integrity 5 raised / 5 survived (recommended: needs-revision alone; trending critical-gaps combined). **Lead's cross-set merge: 14 raised → 11 distinct survivors** (F3≈IF1 and F9≈IF3 merged as duplicates; IF5 folded into F4 as one compound). Map sample-audit: ~18 load-bearing figures verified exact; one arithmetic slip (S11).
- **Merged survivor set (lead's numbering · severity · source · answer-owner · disposition):**
  - **S1 · Critical · F1 (fact-checker-settled, both reviewers concur)** — D2 falsified as scoped: no session-readable per-run/per-seat token total exists without transcript parsing (platform exposes only a USD estimate + live context snapshot), and the kernel-free non-negotiable bars an automated parse; cascade: D2's rationale claimed to verify D3–D6, and D3+D6d strip the disk-residue proxy. **Disposition: user-ruled (2026-07-23), CLOSED** — "manual is fine": D2 re-scoped to the manual baseline protocol (folded into D2), cascade met by the floor + D3's structured residue. The user-commissioned OpenTelemetry fact-check (F-d, landed verbatim) confirmed config-only per-run/per-seat data with a stdout/interval/self-read gap; **final close: user adopted probe-then-graduate** — baseline stands, build list gains the one-shot OTel probe, automation graduates on probe evidence.
  - **S2 · Important · IF1+F3 (merged dup)** — D1's mark self-contradicted in three locations; Confident rested on inferred ratification. **Disposition: user-ruled + resolved** — user explicitly endorsed the frame ("d1 yes"); all three locations reconciled (header provenance note rewritten, rationale's "Mark Assumed" line removed, status refreshed).
  - **S3 · Important · IF2** — ratification events unlogged (no Q5 entry, no avenue-sort event). **Disposition: resolved** — both events logged verbatim in the Q-series (Q5 + Avenue-map arbitration), named as the ratification basis of D3–D6's marks.
  - **S4 · Important · F4+IF5 (compound)** — priority order asserted without composing savings-to-effort; "largest single center" wrong on unit and axis. **Disposition: resolved** — ratio composed in D4 (Ordering note; D3-first order survives), D3's wording tightened with the type/location qualifier and disk-proxy caveat.
  - **S5 · Important · F5** — D5 conflated verification-depth with review-count sizing. **Disposition: user-ruled (2026-07-23)** — split accepted: reviewer-count gate keeps pair/single/none-with-waiver; verification depth is its own floored gate, never zero real-infra evidence (folded into D5).
  - **S6 · Minor · F2 (pair-converged)** — map-landing precedent dismissed with the wrong reason. **Disposition: resolved** — bullet reworded: precedent named, real non-transfer reasons stated (per-stage cold seats; post-D4 nothing left to compress).
  - **S7 · Minor · F6** — D5's ~270k citation misapplied. **Disposition: resolved** — D5 rationale regrounded on pattern-extension; the figure qualified as brainstorm-pair-specific.
  - **S8 · Minor · F7** — slim reports' failure-debug sufficiency untested. **Disposition: user-ruled (2026-07-23)** — refinement accepted: failed cycles keep the fuller narrative; slim is the passing-cycle format (folded into D3; Q6's Notes-for-Next-Cycle removal untouched).
  - **S9 · Minor · F8** — D1's "no quality trade by construction" wording leaked. **Disposition: resolved** — Class-A definition tightened: waste portion trade-free; residual trade-bearing edges surfaced for arbitration (Q6 as the practiced rule).
  - **S10 · Minor · IF3+F9 (merged dup)** — status line stale. **Disposition: resolved** — status rewritten to the review-state form; will track through close.
  - **S11 · Minor · IF4** — map internal inconsistency on verification-report bytes. **Fact territory — resolved by double-check:** the fact-checker's per-file recount refuted the finding's arithmetic (12 per-cycle = 149,715 B, not 158,221; §6 totals correct, no exclusion, no double-count) but confirmed the finding's substance — the F-c Part 3 line misattributed 158,221 B to the 12 per-cycle files. **Disposition: resolved** — attributed erratum landed under F-c; §6 s1 row carries the checker's clarifying label; all totals unchanged (D3's basis intact). Arithmetic relayed to reviewer-integrity (finding owner) by the checker; **the owner then amended IF4 on the record** — owned the glob error (its 12-file sum had matched final-validation.md too), retracted the two consequence claims (§6 totals sound; no D3 understatement), confirmed the retarget to the F-c Part 3 label. Tallies unchanged (5 raised / 5 survived, IF4 retargeted); the map-audit's reliability conclusion stands reinforced.
- **Verify pass (reviewer-integrity, 2026-07-23): all 11 folds VERIFIED CLEAN** — per-survivor PASS with quoted evidence; the new-inconsistency hunt found none blocking; all five in-review user rulings verified recorded-as-ruled. Two optional non-blocking nits were flagged and folded by the lead post-verify (D1's arbitration-pointer wording; D2's title aligned to the re-scoped manual baseline — "records its own token cost" → "ends with a recorded cost entry"), each a one-line touch in verified sections per the reviewer's note.
- **Clearing verdict (lead, 2026-07-23): the record CLEARS — ready.** Both reviewers' pre-disposition recommendations (needs-revision / critical-gaps) were driven by findings now dispositioned: the critical-gaps driver (S1/D2) closed by user re-scope + the probe-then-graduate ruling; every Important folded and verified. Combined tally: **14 raised → 11 merged survivors → 11/11 dispositioned (4 user rulings in-review + Q6 standing) → verify PASS.** Fallen findings: none (0 of 14 died in cross-exam; dups/folds are merges, not kills; retrievable from the reviewers on ask).
