# Command Succinctness Strip + Structure Rethink — Decision Record

**Status:** **accepted** (2026-07-30) — pair-reviewed: 24→18 merged findings, 18/18 dispositioned (user batch U1–U5); verify 16-clean + V1 user-ruled/folded/re-verified CLEAN + V2 resolved via checker corrigendum. Landed: `DECISIONS.md` CS-D1–D10 · `BACKLOG.md` §Command goal-shape rebuild · `ROADMAP.md` Now.
**When:** 2026-07-30
**Session:** `/mochiko:brainstorm` (team-form, shape v4)
**Topic:** assess every line of every mochiko command for succinctness or removal — a ≥70% per-command reduction ambition — and question/rethink the commands' structure.

## Session setup

- **Reality surface:** `plugins/mochiko/commands/*.md` — six commands, 1308 lines total at open (brainstorm 47 · specify 146 · slice 165 · setup 265 · implement 292 · plan 393; line counts flagged non-comparable, see Corrections C2) — plus the shape home (`templates/command-shape.md` v4) and its paired templates, `.mochiko/strips/`, and the prior doctrine sessions (`skill-succinctness-strip`, `command-altitude`, `pattern-codification-and-minimalism`).
- **Seats:** fact-checker (filled at open; map lands verbatim below) · cold reviewers at convergence per the sizing gate.
- **Decision namespace:** D1… — statement + rationale + confidence mark (`Confident / Assumed / Contested / Unsure / Deferred`); user corrections and reversals logged where they happen.
- **KM open invariants:** run at open — see Session log.

## Fact-checker map — command surface (verbatim)

**Measured 2026-07-30 at plugin v0.32.0** (`plugins/mochiko/.claude-plugin/plugin.json`), shape home at **v4** (`templates/command-shape.md:173–180`). All counts are `wc`-measured or grep-counted on the files, not estimated. Git-reconstructed history uses `git show <commit>:<path> | wc`.

### 0. Two corrections to premises stated in the session record

- **The strips directory is `.mochiko/strips/`, not `plugins/mochiko/strips/`.** `record.md:10` and the fact-checker brief both name `plugins/mochiko/strips/`; that path does not exist. `.mochiko/strips/README.md:13,17` states the location was amended 2026-07-19 by user ruling *precisely* out of `plugins/`: "must NEVER live under `plugins/` — the plugin directory is the shipped artifact… A future wave that writes a strip note anywhere under `plugins/` is a defect — fix on sight." 52 files live at `.mochiko/strips/`.
- **The "1308 lines total" baseline is arithmetically correct but not comparable across the six commands.** See §2.

### 1. Per-command size (measured)

| command | lines | words | chars | chars/line | max line len | `description:` chars |
|---|---|---|---|---|---|---|
| brainstorm | 47 | 1,376 | 9,547 | **203.1** | 1,076 | 769 |
| specify | 146 | 1,273 | 9,390 | 64.3 | 412 | 399 |
| slice | 165 | 1,611 | 11,968 | 72.5 | 527 | 514 |
| setup | 265 | 2,768 | 20,731 | 78.2 | 868 | 855 |
| implement | 292 | 3,230 | 23,873 | 81.7 | 882 | 869 |
| plan | 393 | 4,439 | 33,833 | 86.0 | 993 | 980 |
| **TOTAL** | **1,308** | **14,697** | **109,342** | 83.5 | — | — |

Descriptions against the 1,536-char delivery truncation boundary (`CLAUDE.md`, skill-library conventions axis 3): brainstorm 50.0% · implement 56.5% · plan 63.8% · setup 55.6% · slice 33.4% · specify 25.9%. **No command description is at or past the truncation boundary.** All six carry `disable-model-invocation: true`.

### 2. The line-count / wrap artifact (bears directly on a per-line reduction target)

Five commands are hard-wrapped at 64–86 chars/line. **brainstorm.md is not wrapped at all** (203.1 chars/line; longest line 1,076 chars). Consequences, measured:

- brainstorm (47 lines, 9,547 chars, 1,376 words) and specify (146 lines, 9,390 chars, 1,273 words) are **the same size in chars and words** despite a 3.1× line-count gap. In words brainstorm is the **larger** of the two.
- Normalized to the five-command ~80 chars/line convention, the surface is **1,366 line-equivalents**, distributed: brainstorm ~119 · specify ~117 · slice ~149 · setup ~259 · implement ~298 · plan ~422. brainstorm's 47 → ~119 is a 2.5× restatement of its share.

The wrap convention changed *inside* the strip history, and the change alone doubled line counts with no content added — verified by measuring both sides of the v0.13.0/v0.14.0/v0.15.0/v0.17.0 commits:

| command | before | after | Δ lines | Δ chars |
|---|---|---|---|---|
| specify | `ffd8e16` 67 lines / 8,715 chars / 130.0 c/l | `e5dfbee` 138 / 8,805 / 63.8 | **+106%** | **+1.0%** |
| slice | `dfe51ac` 70 / 10,923 / 156.0 | `fbb9a17` 161 / 11,644 / 72.3 | +130% | +6.6% |
| implement | `022a404` 82 / 14,503 / 176.8 | `d3964b9` 229 / 17,674 / 77.1 | +179% | +21.9% |
| plan | `dfe51ac` 84 / 13,935 / 165.8 | `81bfe3a` 229 / 17,150 / 74.8 | +173% | +23.0% |

specify's doubling is a **pure rewrap** (+1.0% chars, words 1,204 → 1,191, i.e. down). implement's and plan's carry ~22% real growth under a ~175% line jump.

### 3. Composition at coarse grain

Section inventory and line counts per command:

- **brainstorm** (7 sections): Team-form parameters 4 · Session parameters 7 · The seats 5 · Convergence 4 · Done-condition and acceptance 6 · Contract 7 · Recovery 3. No `## The flow`, no `## Phase` sections.
- **specify** (6): Team-form params 7 · Session constraints 11 · The seats 23 · The flow 44 · Contract 21 · Recovery 22.
- **slice** (6): 7 · 21 · 23 · The flow 43 · 25 · 25.
- **setup** (6): 8 · 17 · 44 · The flow 100 · 32 · 30.
- **implement** (10): 8 · 15 · 37 · Phase 0 36 · Phase 1 41 · Phase 2 30 · Phase 3 10 · Phase 4 11 · Contract 40 · State recovery 43.
- **plan** (12): 8 · 23 · The seats **84** · Phase 0 32 · Phase 1 25 · Phase 2 40 · Phase 3 28 · Phase 4 21 · Phase 5 17 · Phase 6 15 · Contract 36 · State recovery 40.

**[PARAM] denominator.** `command-shape.md` contains **14 `[PARAM]` occurrences** (line 7 is the tagging rule itself; 13 are actual slots): artifact+path+ID scheme (:30) · uncertainty carrier (:31) · reviewed artifact / in-loop-critique declaration (:42) · sizing default keying (:47) · reviewer agent × skill × lens briefs (:50) · fact seat (:68) · verify-pass owner (:73) · pause location (:106) · evidence→resume-at mapping (:108) · first-spawn seat (:115) · seat roster incl. peer edges (:128) · clearing unit (:160) · checkpoint keying (:166). **No `[PARAM]` slot covers the flow/phase machinery** — the Phase and The-flow sections are per-workflow content with no declared shape slot.

Splitting each command by whether its sections map to a declared `[PARAM]` slot (Team-form params + Session constraints + The seats + Contract + Recovery) or to unslotted flow machinery:

| command | flow/phase lines (share) | param-slot sections (share) | remainder (frontmatter + goal preamble + footer) |
|---|---|---|---|
| brainstorm | 10 (21.2%) | 27 (57.4%) | 10 |
| specify | 44 (30.1%) | 62 (42.4%) | 40 |
| slice | 43 (26.0%) | 69 (41.8%) | 53 |
| setup | 100 (37.7%) | 93 (35.0%) | 72 |
| implement | 128 (43.8%) | 100 (34.2%) | 64 |
| plan | 178 (45.2%) | 123 (31.2%) | 92 |

Reference density (backticked `*.md`/`*.yaml`/`*.sh` paths + `mochiko:*` skill names): brainstorm 18 refs on 11 lines · specify 38/32 · slice 55/42 · setup 43/40 · implement 63/53 · plan **158/108** (27.5% of plan's lines carry at least one reference).

Elaboration proxies (em-dash asides, parentheticals, bold spans): brainstorm 43 / 38 / 35 · specify 35 / 52 / 41 · slice 46 / 73 / 54 · setup 74 / 113 / 102 · implement 70 / 124 / 120 · plan 97 / 204 / 156.

`**What you own (not the seats/agents)**` footers: specify 4 lines/40 words · slice 6/76 · setup 5/58 · plan 12/157 · implement **19/255** · brainstorm has none.

Recovery-table rows: specify 8 · slice 9 · implement 13 · setup 14 · plan 18 · brainstorm 0. Numbered gates: specify G1–G3 (11 mentions) · slice G1–G5 (14) · implement G1–G5 (24) · setup G1–G5 (25) · plan G1–G7 (**53 mentions**) · brainstorm **none — 0 numbered gates**.

**`<!-- shape-exception -->` markers: 2 total, in 2 of 6 commands.**
- `plan.md:227` — D8/R5, the un-rendered-diagram degrade-with-record fallback at the G3 sign-off.
- `setup.md:100–101` — "setup carried the library's most explicit statement of the falsified routing=independence claim; the correction is stated at the seat so the deletion is not silent."
brainstorm, implement, slice, specify carry zero. The only other HTML comments in commands are the five literal `<!-- mochiko:governance:begin -->` marker citations (implement:92, slice:79, setup:53, specify:65, plan:146).

### 4. The single-sourced homes — role and size

| home | lines | words | chars | role | referenced by |
|---|---|---|---|---|---|
| `templates/command-shape.md` **v4** | 180 | 1,849 | 12,502 | sole authoritative home of the command pattern; Layer 1 form-agnostic core + Layer 2 team transport; declares the 13 `[PARAM]` slots | **all 6** commands (hard obligated read) |
| `templates/agent-dispatch.md` v4 | 72 | 800 | 5,183 | caller-side 8-field briefing checklist + the "one hard line" independence check + **Seat transport** (team-form spawn mechanics, `name:` discriminator, addressability probe) | **all 6** commands |
| `templates/workflow-contract.md` v1 | 58 | 484 | 3,272 | per-run fill-in contract form | **0 commands.** Referenced only from `command-shape.md:86` ("stays the form for loops whose values genuinely vary per run"), `agent-dispatch.md:72`, `skills/loop-discipline`, `skills/mochiko` |
| `templates/report-format.md` v1 | 74 | 558 | 3,919 | machine-first report envelope (YAML frontmatter, findings schema, conditional-prose rule) | **0 commands.** Referenced by the 8 report templates, `skills/mochiko`, `executing-tdd-cycle/references/CYCLE-REPORT-FORMAT.md`, `testing-end-user/references/REPORT-TEMPLATES.md` |
| `templates/artifact-format.md` v1 | 69 | 647 | 4,678 | deliverable envelope, 10 shared rules incl. "density is not a gap" and the self-containment floor | **0 commands.** Referenced by the artifact templates + the authoring/review skills |

**Three of the five named homes (`workflow-contract`, `report-format`, `artifact-format`) are referenced by zero commands** — they bind templates and skills, not the command layer.

**One shape v4 Layer 1 element is bound in no command.** The Run-cost entry (`command-shape.md:93–102`: at finalize ask the user for the visible usage figure and append a row to `run-costs.md`) carries **no `[PARAM]` tag** and appears in **0 of 6 commands** (grep for `run-cost` / `/usage` across `commands/` = 0 hits; the only plugin hits are `command-shape.md` itself and `skills/authoring-commands/SKILL.md`).

### 5. The runtime read chain — true loaded-context cost of one run

Lead-side obligated reads, measured. `command-shape.md` is obligated in all six by the literal instruction "Read `${CLAUDE_PLUGIN_ROOT}/templates/command-shape.md` (both layers) before anything else". `agent-dispatch.md` is named in all six for transport + briefing. `loop-discipline` is invoked in all six ("This is a `mochiko:loop-discipline` sound loop").

| command | command file | + shape 12,502 | + dispatch 5,183 | + loop-discipline 11,926 | + per-command reads | **TOTAL chars** | command's share |
|---|---|---|---|---|---|---|---|
| brainstorm | 9,547 | | | | 14,810 (analysis-iterative 12,366 + CROSS-EXAM 2,444) | **53,968** | 17.7% |
| specify | 9,390 | | | | 13,137 (analysis-iterative 12,366 on sparse input + spec-template 771) | **52,138** | 18.0% |
| slice | 11,968 | | | | 6,512 (slices-template) | **48,091** | 24.9% |
| setup | 20,731 | | | | 20,384 (analysis-iterative 12,366 + INTERROGATION-AGENDA 8,018) | **70,726** | 29.3% |
| implement | 23,873 | | | | 6,512 (slices-template, slice-scoped via the Graduation contract) | **59,996** | 39.8% |
| plan | 33,833 | | | | 9,378 (plan-template 2,866 + slices-template 6,512 slice-scoped) | **72,822** | 46.5% |

Obligated-file sizes: `command-shape.md` 180 lines/12,502 chars · `agent-dispatch.md` 72/5,183 · `loop-discipline/SKILL.md` 138 lines/1,899 words/11,926 chars · `analysis-iterative/SKILL.md` 196/1,829/12,366 · `INTERROGATION-AGENDA.md` 101/1,183/8,018 · `CROSS-EXAM.md` 39/376/2,444 · `slices-template.md` 135/964/6,512 · `plan-template.md` 76/381/2,866 · `spec-template.md` 58/89/771.

The three shared reads (shape + dispatch + loop-discipline) total **29,611 chars** and are paid by every command. Not counted above: the seats' own skill loads, which are teammate-side.

### 6. Strip history per command

No command strip note records a before/after line or word count. (Skill strip notes do — e.g. `.mochiko/strips/patterns-vertical-tdd.md:8` "body 368 → 204, −45%, in-band"; `analysis-codebase.md:5` "313 file / 309 body lines at v0.23.0 → 175 file"; `authoring-requirements.md:9` "201 → 125, −38%". The command notes carry disposition + tier + verbatim stripped content only.) Entry counts and version stamps:

| command | strip entries | version stamps |
|---|---|---|
| brainstorm | 8 | v0.11.0 only |
| specify | 11 | v0.13.0, v0.31.0 |
| slice | 12 | v0.14.0, v0.31.0 |
| setup | 9 | v0.11.0, v0.13.0, v0.31.0 |
| plan | 10 | v0.15.0, v0.31.0, v0.32.0 |
| implement | 11 | v0.17.0, v0.31.0, v0.32.0 |
| tasks (**retired**) | 10 | v0.16.0, v0.31.0, v0.32.0 |

Reconstructed before/after from git:

**The 2026-06-30 altitude retrofit (`c09d4da`, "Enforce command altitude; thin specify/setup") covered only specify and setup.** `command-altitude/synthesis.md:5` records "specify (329→66) and setup (385→78)" — both line figures verify exactly. In words: specify 3,080 → 1,154 (−62.5%); setup 2,853 → 1,344 (−52.9%). plan, implement, slice, brainstorm were never in that retrofit.

Word-count trajectory since:

| command | earliest | altitude floor | at team-form conversion | today | today vs floor | today vs earliest |
|---|---|---|---|---|---|---|
| specify | 3,080 (`07c9dbc`, 2026-06-28) | 1,154 | 1,191 (v0.13.0) | 1,273 | **+10.3%** | 41.3% |
| setup | 2,853 (`e26ec97`, 2026-06-27) | 1,344 | 2,509 (v0.13.0) | 2,768 | **+105.9%** | **97.0%** |
| plan | 1,597 (`d24cd3f`, 2026-07-01) | n/a | 2,241 (v0.15.0) | 4,439 | — | 278% |
| implement | 1,768 (`f97cc0a`, 2026-07-02) | n/a | 2,404 (v0.17.0) | 3,230 | — | 183% |
| slice | 1,480 (`b53b57c`, 2026-07-02) | n/a | 1,562 (v0.14.0) | 1,611 | — | 109% |
| brainstorm | 1,730 (`4ee1407`, 2026-07-02) | n/a | 1,383 (v0.11.0) | 1,376 | — | **79.5%** |

Growth since each command's own team-form conversion, in words: brainstorm −0.5% · slice +3.1% · specify +6.9% · setup +10.3% · implement +34.4% · **plan +98.1%**. plan's two v0.32.0 jumps are recorded builds, not drift: `21cb75e` (architecture-design primitive + plan-absorbs-tasks merge) took plan 255→361 lines / 2,498→4,010 words, and the surface went 7 commands → 6 with `/mochiko:tasks` retired (`.mochiko/strips/plan.md:10–19`, `.mochiko/strips/tasks.md` v0.32.0).

Recurring strip-entry types across the six command notes: the sound-loop paragraph + four-requirement enumeration (relocated → shape Layer 1, in specify/slice/plan/implement) · the per-run contract fill (→ shape Layer 1 Contract, in specify/slice/plan/implement) · verdict-ownership triplication (deduped to once, in specify/slice/plan/implement) · footer ground rules + Task-tool transport line (→ shape Layer 1 Ground rules, in specify/slice/setup/plan/implement) · the recovery memory-model parenthetical (→ shape Layer 1 Recovery, in specify/slice/plan/implement) · the "why this done-condition differs from HIL's" blockquote (Tier-2 **deleted**, in specify/plan/implement/tasks) · at v0.31.0, the lead-routed-gap-list supersession (all six).

Two prior audit catches are on record, both about restatement hiding beneath a self-declaration: `.mochiko/strips/plan.md:193–211` (the slice-scoped entry "declared 'the single source … do not restate it' and then restated most of it — the D1 churn liability", caught by the `validation-command-shape` audit, fixed in-wave with no version bump) and `.mochiko/strips/implement.md:197–222` (the same defect, fixed proactively; implement was the last restating consumer). `.mochiko/strips/plan.md:96–97` records a two-round in-wave audit correction where a narrowing "was applied to the roster bullet and Phase 1 step 1 but **not propagated**, leaving three sites still asserting the blanket edge."

### 7. Standing doctrine a new wave must reckon with

**Shape v4's param-vs-shape rule** (`command-shape.md:6–12`): "an obligated read — the lead Reads this file up front; the command states only its per-workflow parameters. Everything tagged **[PARAM]** below is a parameter — it lives in the command; everything else is shape — it lives here and only here. A command line that must restate shape content (rare) carries an inline marked exception — `<!-- shape-exception: why -->` — the audit's deterministic floor keys on that marker." Also: "a shape revision is one edit here plus a re-audit of the conformant commands."

**Tiered strip criterion** (`.mochiko/strips/README.md:24–43`): **Tier 1** = altitude, restated doctrine/pattern → relocate to the single-sourced home. **Tier 2** = no behavior/failure named → delete. Four entry types: strip · **supersession-by-ruling** (ground is a decision, never a tier; carries a *Kept deliberately* field because "a doctrine reversal usually spares part of what it touches, and an unrecorded survivor reads to the next auditor as an oversight") · **survivor-provenance** (`KEPT:` + Tier-2 evidence) · **re-add** (`RETURNED:` + evidence link or the literal `override` marker — "override clusters are a hunt signal"). Entries are newest-first, one file per primitive, each version-stamped. The directory is "deliberately NON-LOADED: no command or runtime surface references it — the log must never itself cost context."

**The skill-succinctness-strip rulings** (`.mochiko/brainstorms/skill-succinctness-strip/record.md`, accepted 2026-07-25, batch R1–R7 ratified — the skills analog, itself opened on a "reduce size by 30–70%" instruction):

- **D1 calibration-bar-not-quota** (:62–71): "30–70% is the expected outcome and a tripwire, not a quota. Tiers apply line-by-line with the burden of proof on each line… **no line is cut while its evidence stands. No quota-override strips.**" Rejected: hard per-skill quota ("forces cuts past standing Tier-2 evidence") and an aggregate library target ("big skills carry the number while already-stripped skills coast").
- **D2 scope, three surfaces each under its own rule** (:75–88): bodies under the tier criterion; frontmatter descriptions as a dedicated **measure-first** sub-pass under a trigger-fidelity criterion; references stripped only where a body pointer dies or content duplicates a home. Fold I6 added a **wrong-fire** watch: "disambiguating boundary clauses ('use X instead', 'does NOT cover Y') are *negatively-graded triggers*, protected alongside positive trigger phrases; cut only genuine elaboration."
- **D3 wave production, per-primitive ratification** (:90–107): architects fan out per cluster, one proposal per primitive; the lead walks the user through proposals **one at a time** (proposed strips, contested lines with drafted Tier-2 rationale, projected reduction %); user rules per primitive; **one independent audit (author ≠ grader) + version bump per wave**. Rejected: fully sequential end-to-end, and one combined ceremony ("batch ratification reads 'one by one' out of the instruction"). Fold I5: a **pre-wave measurement pass enumerates the ≥3-consumer set by citation count**; shared primitives are ruled at a scheduled all-consumer escalation, never inside one cluster's wave.
- **D4 true-reductions-only / sham-cut ban** (:109–122): "Reduction credit = deletes + body→reference moves where the reference is genuinely conditional (the strip note must name the invocation path that skips it — audit-checkable). **Always-read content moved to a reference is a sham cut, forbidden** — and `templates/` is named a forbidden relocation destination (templates are read at authoring time, i.e. always-read)." R4a: cross-primitive **dedup** into a genuinely conditional home earns Tier-1 credit; mere flagging earns nothing.
- **D5 close shape** (:124–137): cold review before anything executes, then conclude with full bookkeeping, waves execute in later sessions. The user ruled out launching wave 1 immediately: "untested doctrine should not drive cuts across the framework's quality surface. The review's `critical-gaps` verdict vindicated the ruling."
- **R2 denominator** (:191): the 30–70% denominator = **per-skill SKILL.md body lines**; description and reference reductions ride separate ledgers with no headline credit.
- **R3 bands by prior status** (:194–196): "never-stripped 30–70%; **previously-stripped 10–40%, tripwire at the lower bound**." Numbers marked **provisional until the R6 pilot calibrates them**.
- **R6 pilot** (:204–206): execution opens with a **one-primitive pilot** — full loop (propose → ratify → strip → audit → measure) then a D1–D4 confirm-or-revise checkpoint before the wave plan is treated as settled. Pilot skill was `analysis-codebase`.
- **R5 exclusions** (:201–203): scripts, `OPENAPI-TEMPLATE.yaml`, and **`plugins/mochiko/templates/` were ruled out of scope** for that pass ("templates carry fresh wave-2 design"), with a revisit trigger.

**The audit floor a new wave must clear** (`skills/validation-command-shape/SKILL.md`): binary PASS/FAIL, **default FAIL**, "Read the graded file and the shape home this run — grading from a summary or the author's report is a FAIL by itself." Deterministic floor, checks 1–5 (:36–64): **(1)** the file contains `loop-discipline` AND `agent-dispatch`; a team-form file also contains `command-shape`; the five KM-carrying commands (brainstorm · specify · plan · implement · setup) also contain `.mochiko/memory/knowledge-management.md`, never the module template's path. **(2)** `disable-model-invocation: true` present, `description:` non-empty. **(3)** five named signature lines must not appear un-excepted: `the forbidden form`/`forbidden transport` · `load`+`skills:`+`frontmatter` on one line · `do not survive` · a transcription of the four-message cross-exam sequence · `reads as a malfunction`. **(4)** every intentional restatement carries `<!-- shape-exception: ... -->` with a non-empty justification — an unjustified marker is a floor FAIL. **(5)** every entry touched this wave carries a `[v` version stamp; every re-add carries an evidence link or the literal `override`. Judgment ceiling, checks 6–10 (:66–85): altitude ("A sentence that would be true of every conformant command is mis-homed") · **parameter completeness** ("every `[PARAM]` the shape home declares for the command's form is actually bound… An unbound parameter is a gap, not a style choice") · contract-fill soundness · **preserved responsibilities** ("nothing workflow-specific was dropped without a strip entry, and every relocation points at a home that actually contains the content — Read the home to confirm") · strip-note quality. Checks 11–14 apply only to a shape-home revision.

I verified the floor's check-1 grep against the current six: all six contain `loop-discipline`, `agent-dispatch`, and `command-shape`; all five KM-carrying commands contain `.mochiko/memory/knowledge-management.md`. Check-3's five signature lines: `run-cost` aside, I did not find un-excepted hits.

**The prior command-thinning session's own doctrine** (`command-altitude/synthesis.md`, 2026-06-30, status "acted on"): the diagnosis was "**not** 'verbose prose': it is that the command **duplicates discipline that `loop-discipline` + `workflow-contract` already single-source** — a `single-source-rule-fanout` violation" (:9). "A command's irreducible job is to **stitch a team to a goal under a contract**: declare the team, state the goal + the per-workflow contract parameters, reference the shared doctrine, name the human gates — and stop there." Its enforcement ruling (:27) is the layered "deterministic grep floor under a grounded keystone-test ceiling" that became `validation-command-shape`. Two recorded risks bear on a new wave: **invocation reliability** (:47) — "A thin command bets `loop-discipline` reliably *fires and is obeyed* mid-loop under rationalization pressure", with an inline backstop licensed **only** "if dogfooding shows a gate being rationalized" (:25); and **retrofit regression** (:49) — "`specify.md`/`setup.md` are landed, independently-verified artifacts whose verbosity encodes hard-won fixes (the `DROPPED` notes: the `@`-reference recovery, the non-auto-resolved constitution prerequisite, the 'advocate status is input, not the gate' reversal of HIL). Thinning must preserve every workflow-specific responsibility."

### 8. Facts that cut AGAINST the ≥70% ambition

1. **brainstorm.md is 47 lines, and what is in it is almost entirely bound `[PARAM]` slots.** Section breakdown: Team-form params 4 (first-spawn probe + transport pointer + `Contested` provenance) · Session parameters 7 (the artifact + ID namespace, slug/reality-surface scoping, KM index bookkeeping, the seat announcement) · The seats 5 (fact-checker + reviewers, with lens-split briefs) · Convergence 4 (weight statement, default keying, fact-dispute route, verify-pass owner) · Done-condition and acceptance 6 · Contract 7 · Recovery 3. Of the 13 declared `[PARAM]` slots, brainstorm binds at least 10. It has **0 numbered gates and 0 recovery-table rows** — there is no gate-numbering or table scaffolding to cut. A 70% cut takes it to ~14 lines against 10+ mandatory parameter bindings.
2. **Two commands have already absorbed an ~80%-line / ~55–63%-word thinning** (`c09d4da`, 2026-06-30: specify 329→66 lines, setup 385→78). R3's band doctrine, applied by analogy, puts previously-stripped primitives at **10–40%**, not 70%.
3. **All six carry v0.31.0 or later strip/supersession entries** — every command was line-audited within the last ~11 days of repo time (v0.31.0 shape-v4 conform, 2026-07-30). specify and slice's most recent entries are supersessions-by-ruling, not minimalism strips, but the v0.31.0 pass read every line of all six (`.mochiko/strips/command-shape.md:12` names the re-audit set: "all seven commands read this run").
4. **Line count is the wrong denominator for four of six commands** (§2). specify's own history contains a **+106% line change at +1.0% chars**. A line-denominated 70% target measured against the current 1,308 is measuring a wrap convention.
5. **The shape's own rule caps how much can leave.** `command-shape.md:9–11` says everything untagged already lives in the home "and only there" — so Tier-1 relocation of shape doctrine out of the commands has largely already run (the recurring v0.11.0–v0.17.0 entries in §6 are exactly that work: sound-loop paragraph, per-run contract, ground rules, recovery preamble, transport mechanics, review generics — six classes, all already relocated).
6. **D4's sham-cut ban forecloses the cheapest route.** Moving command content into `templates/` earns no credit and is explicitly forbidden as a destination (`skill-succinctness-strip/record.md:113–114`), and a body→reference move only counts where the strip note "must name the invocation path that skips it". The command file itself is always-read at invocation, so any relocation out of a command into another always-read home is a sham cut by that definition.
7. **plan's and implement's recent growth is recorded build work, not drift.** plan 255→393 lines across `21cb75e`+`b5335bd` carries the architecture stage (AD-D1–D9: a new `system-architect` seat, `architecture.md`, the G2 baseline gate, the G3 rendered-diagram sign-off) plus the absorbed `/mochiko:tasks` structuring loop; implement 255→292 carries AD-D6 (briefed architecture input, the diagram-anchored deviation self-check at cycle open and close, the built-vs-approved diff, seam N1). Each addition has a `DECISIONS.md` row.
8. **plan's largest section is its seat roster (84 lines, 21.4%)** for 3 producers + 2 reviewers + 1 disposable scribe = 6 seats, which is the shape's `[PARAM]`-mandated content (`command-shape.md:128` — "the seat roster — agent, skill, spawn timing, standing or cold, and each seat's peer edges").
9. **Two `KEPT:` survivor-provenance entries with standing Tier-2 evidence exist on the command surface** and under D1 cannot be cut while the evidence stands: `.mochiko/strips/specify.md:74–75` (the lead-inline enrichment boundary — "without it the natural reading is to hand enrichment to the producer seat, coupling input conditioning into authoring") and `.mochiko/strips/slice.md:111–115` (the "No G2" note — "without it, an auditor/reader seeing G1/G3/G4/G5 reads the G2 gap as a dropped gate").
10. **The command file is a minority of a run's loaded context in 4 of 6 cases** (§5): 17.7% for brainstorm, 18.0% specify, 24.9% slice, 29.3% setup. Even eliminating brainstorm.md entirely cuts its run's loaded context by 17.7%. The three shared obligated reads are 29,611 chars — 27.1% of the command surface's own total size — and are out of the commands' reach.

### 9. Facts that SUPPORT the ≥70% ambition

1. **Measured cross-command repetition.** Phrase-level, whitespace-flattened, over the six files:

| appears in | phrase / clause |
|---|---|
| **6/6** | `Hard-require CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS per the shape` · `team-form command in the mochiko command shape` · `before anything else` (the obligated shape read) · `the shape's rules … are not restated; this file carries only <cmd>'s parameters` · `authoritative first-spawn probe` · `Transport mechanics + the addressability check: templates/agent-dispatch.md (Seat transport)` · `Pause posture (per the shape)` |
| **5/6** | `no-fallback bet` + the `Contested` pointer · the kill-switch sentence · the governance-region prerequisite (`<!-- mochiko:governance:begin -->`) · the one-line governance obligated-read line · `Resume from workspace evidence` · `respawn is cold by design` · `Out of rounds = escalate, never done` · `lead-adjudicated input` (the `review-*` family boundary) · the KM landing ritual + invariants under fix-on-sight · `named standing seat` · `structurally separated` · Contract `default **FAIL**` · the `What you own (not the seats)` footer · `Full rules: mochiko:loop-discipline` · the `@`-reference drop bug |
| **4/6** | `the bounded in-loop critique` (validation model) · `No devolved branch` · `hold the revision targeted (fix the flagged gaps; don't regress passing sections)` · `disjoint agents, disjoint skills` · `no-progress exit` · `status is input, never the gate` · `check(ed) before each seat send` · `governance reaches … natively` · the `accept-with-noted-gaps` escalation menu · `a suggested commit` |
| **3/6** | `the shape's in-loop-critique branch` · `(the shape's producer-authored branch)` · `It never grades` · `run stays FAIL unless the user explicitly accepts` · `Round reports are cleaned by default` · `verifying each dispatch wrote its expected files (a missing output → log and ask retry/abort)` · `No copy → skip` · `most recent in-progress feature under .mochiko/specs/` |

The 6/6 set alone is the goal preamble + the whole `## Team-form parameters (shape Layer 2)` section, which runs 7–8 lines in five commands and 4 in brainstorm — **42 lines / ~44 lines of near-identical text**, of which only the probe-seat identity varies per command.

2. **The `## Team-form parameters` section binds exactly one `[PARAM]`** (`command-shape.md:115`, which seat spawns first). Its other three sentences — the env hard-requirement, the transport pointer, the no-fallback `Contested` provenance — are the same in all six and are shape content per `command-shape.md:112–121`.

3. **Doctrine is restated in the Contract sections at scale.** `default **FAIL**` (5/6), `Out of rounds = escalate, never done` (5/6), `structurally separated` (5/6), `disjoint agents, disjoint skills` (4/6), `No devolved branch` (4/6), `the bounded in-loop critique … no sized end-stage review (the shape's in-loop-critique branch)` (4/6, ~4 lines each), `no-progress exit when the gap set is unchanged round-over-round` (4/6). Contract sections total **161 lines** across the six (brainstorm 7 · specify 21 · slice 25 · setup 32 · plan 36 · implement 40) = **12.3% of the surface**. `command-shape.md:78–86` states what the Contract must carry; the per-command not-done states and gate lists are the parameters, the framing sentences are not.

4. **The `What you own (not the seats)` footers are declared duplicates of content already in the body.** Five commands carry them, 586 words total (implement 255 · plan 157 · slice 76 · setup 58 · specify 40). `.mochiko/strips/specify.md:44–47`, `slice.md:55–62`, `plan.md:154–163`, `implement.md:156–167` each record a **verdict-ownership triplication** already deduped once at v0.13.0–v0.17.0, with the footer named as one of the three sites — yet the footers survive and restate the gate list, the counter ownership, the verdict ownership, and the collapse prohibition. brainstorm has no such footer and is the smallest command by chars.

5. **Recovery sections restate a shared preamble plus a table.** 5/6 carry `Pause posture (per the shape)` + `Resume from workspace evidence, respawning what the stage needs` + `a <X> respawn is cold by design` before the table. Recovery totals 143 lines (brainstorm 3 · specify 22 · slice 25 · setup 30 · plan 40 · implement 43) = 10.9% of the surface; 82 of those are table rows (8+9+13+14+18 = 62 rows plus headers/separators). Only the evidence→resume-at mapping is a `[PARAM]` (`command-shape.md:108`).

6. **Elaboration density is high and uneven.** 204 parentheticals and 97 em-dash asides in plan (0.52 and 0.25 per line); 124 and 70 in implement. `plan.md` averages 86.0 chars/line with 27.5% of lines carrying a reference — i.e. references and their surrounding gloss, not bare pointers.

7. **Three commands' flow sections exceed their param-slot sections** (setup 100 vs 93; implement 128 vs 100; plan 178 vs 123) — and the shape declares **no `[PARAM]` slot for flow/phase machinery** at all, so the altitude bar for that 449-line body of text (34.3% of the surface) is set only by check 6's judgment ceiling, not by a declared slot.

8. **Two prior audits found restatement surviving beneath an explicit "do not restate" self-declaration** in this exact command set (`.mochiko/strips/plan.md:193–211`, `implement.md:197–222`), and a third found a correction applied to 2 of 5 sites (`plan.md:96–97`, "Substance was upheld; only propagation failed"). The self-declaration is not evidence the restatement is absent.

9. **Three of the five templates the brief names as single-sourced homes are referenced by zero commands** (§4), and one shape v4 Layer 1 element (the run-cost entry) is mentioned by zero commands — so the commands' reference surface is narrower than the template surface suggests, and the six commands' 158+63+55+43+38+18 = 375 reference tokens point at a small set.

10. **The surface already contracted once by command count.** 7 commands → 6 at v0.32.0 with `/mochiko:tasks` retired into plan's Phase 4 (`.mochiko/strips/plan.md:10–19`, `.mochiko/strips/tasks.md` v0.32.0); tasks' standalone acceptance gate dissolved into plan's G7, and two separately-spawned reviewers became one standing `devils-advocate` seat running two skills.

### 10. Measurement notes

- Word/char/line counts are `wc -lwc` on the files as of 2026-07-30, plugin v0.32.0, on branch `main` at `c47684d`.
- "Commands containing X" counts are `grep -l` over whitespace-flattened copies of the six files (hard wrapping otherwise splits phrases across lines and undercounts — my first pass reported `authoritative first-spawn probe` as 1/6 when it is 6/6, and `Out of rounds = escalate, never done` as 2/6 when it is 5/6). All counts in §9 are the flattened, corrected ones.
- Section line counts are measured from `## ` heading to the next heading (or EOF), so each includes its own heading line; the "remainder" column in §3 is total minus (flow + param-slot) and holds frontmatter, title, goal preamble, the `---` rule, and the footer.
- The `[PARAM]`-slot vs flow classification in §3 is mechanical (by section heading, against the shape's tag list), not a judgment about whether a given line is at altitude.
- Git-reconstructed counts use the commit that touched each file; `b5335bd` and `21cb75e` are merge/build commits on `main`.

### Map errata (checker-authored, verbatim — issued 2026-07-30 on the review pair's findings; supersedes the map's §3 columns, §5 rows, §9.5 totals, §9.7, and the 449 figure)

**Recomputed 2026-07-30 at plugin v0.32.0.** The reviewers are right on all six items. Every **primary** measurement in the map holds — I re-measured the per-command section inventory directly from the files and it reproduces the map's §3 inventory line for line. Every error below is in an **aggregate I computed on top of** that inventory: five of six param-slot sums silently dropped the Recovery section (and in three cases more), and three stated totals do not follow from their own components. Directions of correction are stated per item; they do not all cut the same way.

---

**(1) §3's param-slot and remainder columns — reviewer-ri's figures confirmed; the map's are wrong in all six rows.**

Derivation: param-slot = Team-form parameters + Session constraints/parameters + The seats + Contract + Recovery, summed from the map's own §3 section inventory (re-measured heading-to-heading: identical). remainder = total − param-slot − outside.

| command | total | param-slot (map said) | share | outside | share | remainder (map said) | share | sums to |
|---|---|---|---|---|---|---|---|---|
| brainstorm | 47 | **26** (27) | 55.3% | 10 | 21.3% | **11** (10) | 23.4% | 47 ✓ |
| specify | 146 | **84** (62) | 57.5% | 44 | 30.1% | **18** (40) | 12.3% | 146 ✓ |
| slice | 165 | **101** (69) | 61.2% | 43 | 26.1% | **21** (53) | 12.7% | 165 ✓ |
| setup | 265 | **131** (93) | 49.4% | 100 | 37.7% | **34** (72) | 12.8% | 265 ✓ |
| implement | 292 | **143** (100) | 49.0% | 128 | 43.8% | **21** (64) | 7.2% | 292 ✓ |
| plan | 393 | **191** (123) | 48.6% | 178 | 45.3% | **24** (92) | 6.1% | 393 ✓ |
| **TOTAL** | **1,308** | **676** | **51.7%** | **503** | **38.5%** | **129** | **9.9%** | 1,308 ✓ |

Component sums: brainstorm 4+7+5+7+3=26 · specify 7+11+23+21+22=84 · slice 7+21+23+25+25=101 · setup 8+17+44+32+30=131 · implement 8+15+37+40+43=143 · plan 8+23+84+36+40=191. The map's figures for specify (62), implement (100) and brainstorm (27) are the sums with Recovery/State-recovery omitted or off by one; slice (69), setup (93) and plan (123) match no subset of their own components. None of the map's rows summed to the file's line count — an arithmetic check the map did not run.

**Direction: this correction cuts AGAINST the ≥70% ambition, materially.** Param-slot sections are the **majority of every command** (48.6%–61.2%; 51.7% of the surface, 676 of 1,308 lines), not the 31.2%–57.4% minority the map reported. The map understated the shape-mandated share of every command, worst on plan (123 → 191, +55%).

---

**(2) §9.7 — retracted in full. Flow exceeds param-slot in ZERO of six commands.**

The map's §9.7 claimed "three commands' flow sections exceed their param-slot sections (setup 100 vs 93; implement 128 vs 100; plan 178 vs 123)". With the corrected columns: setup 100 vs **131** · implement 128 vs **143** · plan 178 vs **191**. All three reverse. Closest approaches are plan (178 = 93.2% of its param-slot lines) and implement (128 = 89.5%); no command's flow section reaches its param-slot total.

**Surviving true fact, restated:** flow/phase machinery is 26.1%–45.3% of each command that has it (specify 30.1% · slice 26.1% · setup 37.7% · implement 43.8% · plan 45.3%) and **38.5% of the whole surface as measured, 37.7% counting genuine flow only** — and `command-shape.md`'s 13 `[PARAM]` slots include **no slot for flow/phase machinery**, so the altitude bar for that body of text is set only by `validation-command-shape` check 6's judgment ceiling, not by a declared slot. That claim was never load-bearing on the comparison and stands unchanged.

---

**(3) The "449 lines" figure — wrong; ri's 503 and 406 both confirmed.**

The map wrote "that 449-line body of text (34.3% of the surface)" for setup+implement+plan's flow sections. Measured: 100 + 128 + 178 = **406**. 449 corresponds to no sum of these components. Correct denominators, all three stated because the map conflated them:

- **406 lines** = setup + implement + plan flow sections = **31.0%** of the 1,308-line surface, or **42.7%** of those three commands' own 950 lines.
- **503 lines** = all six commands' sections outside the five named groups = **38.5%** of the surface (ri's figure, confirmed).
- **493 lines** = genuine flow/phase machinery across all six = **37.7%** of the surface — brainstorm's 10 are excluded here per item (6).

The map's "34.3%" matches none of these.

---

**(4) §5 corrected — named lead-side reads of measured plugin files only.**

Three charges in the map's §5 were unfounded and two lead-side reads were omitted. Verified by grep against the command files:

- **Removed, implement −6,512 and plan −6,512** (`slices-template.md`). Neither file contains the string `slices-template`. Both reference the **per-feature artifact**: `implement.md:105` / `plan.md:159` "If `.mochiko/specs/<feature>/slices.md` exists (accepted)… apply that file's own **Graduation contract** section". A filled per-feature `slices.md` is not the 6,512-char template and its size is **unmeasured**. Only `slice.md` references the template (3×).
- **Removed, brainstorm −2,444** (`CROSS-EXAM.md`). No command references it — `grep -rn CROSS-EXAM plugins/mochiko/commands/` returns nothing. Its only reference is `command-shape.md:56`, and the load is **teammate-side**, reached by the reviewer seats through `review-brainstorm`, not by the lead.
- **Added, setup +8,127** (`templates/governance-intent-template.md`, 157 lines). `setup.md:135` — "assemble `templates/governance-intent-template.md` → `.mochiko/memory/governance-intent.md`". The lead writes the synthesis, so this is a lead-side read the map omitted entirely.
- **Added, setup +11,418 conditional** (`templates/constitution-modules/knowledge-management.md`, 163 lines). `setup.md:188` — fires when knowledge-management is adopted.
- **Not charged, stated:** `setup.md:107` runs `bash …/detect-stack.sh` — that is **executed, not Read**; its output enters context and is unmeasured.

Shared unconditional lead-side reads, all six: `command-shape.md` 12,502 + `agent-dispatch.md` 5,183 + `loop-discipline/SKILL.md` 11,926 = **29,611 chars**.

| command | command file | + shared | + named per-command reads | **total (measured)** | command's share | unmeasured on top |
|---|---|---|---|---|---|---|
| brainstorm | 9,547 | 29,611 | 12,366 (`analysis-iterative`, uncond. — :10 "You run the questioning inline via") | **51,524** | **18.5%** | `governance-ledger.md` (:19, cond.) · `knowledge-management.md` (:20, cond.) · `brainstorms/index.md` (:20) |
| specify | 9,390 | 29,611 | 771 (`spec-template`, uncond. — :29 lead seeds it) · +12,366 `analysis-iterative` **conditional, sparse input only** (:73) | **39,772 – 52,138** | **23.6% – 18.0%** | `knowledge-management.md` (:100, cond.) |
| slice | 11,968 | 29,611 | 6,512 `slices-template` **arguable** — named in the producer brief (teammate-side) *and* in Contract clause (1) the lead verifies conformance against | **41,579 – 48,091** | **28.8% – 24.9%** | per-feature `spec.md` (:34) |
| setup | 20,731 | 29,611 | 12,366 + 8,018 (`INTERROGATION-AGENDA`) + 8,127 (`governance-intent-template`) = 28,511 uncond.; +11,418 KM module conditional | **78,853 – 90,271** | **26.3% – 23.0%** | `codebase-analysis.md` (brownfield) · `detect-stack.sh` output |
| implement | 23,873 | 29,611 | none | **≥53,484** | **≤44.6%** | **7 per-feature design inputs** obligated at Phase 0 step 4 (`plan.md`, `architecture.md`, `task-mapping.md`, `data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`, `requirements.md`) + per-feature `tasks.md`, `slices.md` |
| plan | 33,833 | 29,611 | 2,866 (`plan-template`, :288 lead assembles) | **≥66,310** | **≤51.0%** | repo-root `ARCHITECTURE.md` (:156) · `governance-intent.md` (:152) · `codebase-analysis.md` (:153) · per-feature `spec.md` (:150), `slices.md` (:159) |

Net effect on the map's §5 totals: setup was **under**-counted by 8,127–19,545; implement and plan were **over**-counted on the measured part while their largest lead-side loads (8 and 5 per-feature artifacts respectively) were omitted entirely; brainstorm over-counted by 2,444.

**Direction: mixed, and it weakens the map's §10 item 10 as stated.** The map claimed the command file is "a minority of a run's loaded context in 4 of 6 cases". Corrected, it is a minority in 4 of 6 on measured floors (brainstorm 18.5% · specify 18.0–23.6% · slice 24.9–28.8% · setup 23.0–26.3%), and implement's 44.6% / plan's 51.0% are **upper bounds** that fall as their unmeasured per-feature inputs are counted — so the honest statement is that the command file may be a minority in all six, but implement's and plan's true shares are unmeasurable from the plugin alone. The shared-read floor of 29,611 chars is unchanged and correct.

---

**(5) §9.5 — Recovery totals corrected; both the map's and ri's table figures are wrong, the measured value is 67.**

- **Components sum to 163, not 143.** 3 + 22 + 25 + 30 + 40 + 43 = **163** (ri confirmed). Share = 163/1,308 = **12.5%**, not the map's 10.9%.
- **Table lines: 67 measured.** The map wrote "82 of those are table rows (8+9+13+14+18 = 62 rows plus headers/separators)". Both figures are wrong, and so is ri's 72. The map's per-command counts (8·9·13·14·18) came from `grep -cE '^\| '`, which **matches the header row but not the `|---` separator** — so 62 already includes the 5 headers, and ri's 72 (62+10) double-counts them. Measured directly with `grep -cE '^\|'`: specify 9 · slice 10 · setup 15 · implement 14 · plan 19 = **67 total table lines** = **57 data rows + 5 headers + 5 separators**. 82 follows from no component set.
- **Consequent correction to §3:** "Recovery-table rows: specify 8 · slice 9 · implement 13 · setup 14 · plan 18" are header-inclusive. Data rows are **specify 7 · slice 8 · implement 12 · setup 13 · plan 17 = 57**.
- **New measured fact:** 163 − 67 = **96 lines of Recovery are prose, 58.9% of the section** — the shared preamble (`Pause posture (per the shape)` 6/6 · `Resume from workspace evidence` 5/6 · `<X> respawn is cold by design` 5/6) plus per-command gloss. The table (41.1%) is the only part the shape declares a `[PARAM]` for (`command-shape.md:108`, evidence→resume-at mapping).

**Direction: cuts BOTH ways.** The section is 20 lines larger than the map said (against the map's accounting, and Recovery is a larger share of the surface than reported), but the prose/table split is new evidence that the majority of Recovery is un-slotted preamble rather than the mandated mapping.

---

**(6) §3's brainstorm classification — ri's slot citations confirmed; the column is relabelled.**

`brainstorm.md` has **no flow/phase section at all** (no `## The flow`, no `## Phase`). The two sections the map filed under "flow/phase" bind declared slots:

- **Convergence — this workflow's review bindings** (4 lines). `brainstorm.md:30`: "run the shape's sized end-stage review with these bindings: **weight statement** = decision count, confidence-mark mix, reality-surface load; **default keying** = a heavyweight record defaults to the full pair; **fact disputes** → the fact-checker (or an Explore subagent); **verify pass** = the record-integrity reviewer's (the sole reviewer's, in single mode)." This binds `command-shape.md:47` (default keying), **:68** (fact seat), and **:73** (verify-pass owner) — three of the 13 slots, exactly as ri cites. The weight statement answers the shape's :46 instruction, which carries no `[PARAM]` tag.
- **Done-condition and acceptance** (6 lines). Carries the done-condition the Contract section then references back to (`brainstorm.md:40` — "**Done-condition:** as above"), so it *is* brainstorm's Contract-fill of the shape's :78–86 requirement, and `:34` binds the :42 reviewed-artifact declaration ("the review ran per the sizing ruling… or the ruling was **none** and the waiver is recorded").

**Column relabelled honestly: "sections outside the five named groups"** — that is what the measurement is. For the five phase-bearing commands it coincides with flow/phase machinery; for brainstorm it does not, and brainstorm's flow/phase figure is **0**.

**Direction: cuts AGAINST the ≥70% ambition, and strengthens §8 item 1.** With Convergence and Done-condition counted as slot-binding, **36 of brainstorm's 47 lines (76.6%)** sit in sections that bind a declared `[PARAM]` or fill the Contract requirement, leaving 11 lines (23.4%) of frontmatter, title, goal preamble and the `---` rule. The map reported 27 lines (57.4%).

---

**Scope of this errata.** Items (1)–(6) are the only aggregates the reviewers challenged and the only ones I have recomputed. Unaffected and unretracted: §1 sizes and description lengths · §2 the wrap artifact and all git-reconstructed before/after figures · §3's section inventory, `[PARAM]` enumeration, reference density, elaboration proxies, footer sizes, gate inventory and the two `shape-exception` locations · §4 the five homes and the zero-reference findings · §6 the strip history · §7 the doctrine quotations · §9's repetition counts (the flattened, corrected set). §10's measurement notes stand, with one addition: **the map stated no arithmetic check that its three columns sum to each file's line count; they did not, in any of the six rows.** All corrected columns above sum to 47/146/165/265/292/393 and to 1,308.

### Errata corrigendum (checker-authored, verbatim — issued 2026-07-30 on the verify pass's residues)

1. **§8 item 10 is superseded by errata item 4 — in part, not whole.** Its four command-share figures fall: brainstorm 17.7% → **18.5%** · specify 18.0% → **18.0–23.6%** · slice 24.9% → **24.9–28.8%** · setup 29.3% → **23.0–26.3%**; and "eliminating brainstorm.md cuts its run's loaded context by 17.7%" reads **18.5%**. Its claim of "a minority in 4 of 6" survives on measured floors, with implement's and plan's shares now upper bounds (errata item 4). **The 27.1% figure is not superseded and stands** — it is 29,611 / 109,342 = 27.08%, and errata item 4 changed neither term (the shared-read floor and the six files' char total are both unchanged), so that sentence needs no edit.

2. **Errata item 4's direction paragraph mis-cites its own target: "weakens the map's §10 item 10" should read "§8 item 10".** §10 is Measurement notes and carries five unnumbered bullets, no item 10. The figures quoted in that paragraph (17.7% / 18.0% / 24.9% / 29.3%) are §8 item 10's.

3. **The governance-intent-template read is at `setup.md:133–134`, not :135.** Verified: `grep -n governance-intent-template plugins/mochiko/commands/setup.md` returns line **134** for the path, and the obligating sentence opens on **:133** ("**Synthesis review** *(sizing gate + cold review, all modes — before G3)* — assemble"). The reference, its 8,127-char size, and its lead-side classification were correct as stated; only the line number was off by one. The errata's two other setup cites re-verify exact: `constitution-modules/knowledge-management.md` at **:188**, `detect-stack.sh` at **:107**.

## Driver (Q3, user-stated)

The commands have **too many instructions**; the user wants their structure aligned to the Claude Code `/goal` doc (https://code.claude.com/docs/en/goal). Doc content verified by lead fetch (2026-07-30): `/goal` is a **runtime mechanism**, not a document format — "Set a completion condition with /goal and Claude keeps working across turns until the condition is met"; a small fast model evaluates the condition after each turn. Its condition anatomy: "**One measurable end state** … **A stated check** … **Constraints that matter**"; conditions up to 4,000 chars; the evaluator "doesn't run commands or read files independently." The doc prescribes no file structure — so "align the structure" needs an interpretation ruling (→ D3).

## Decisions

**D1 — Scope: the whole loaded surface, with the command surface itself open to question.** `Confident` (recommendation adopted)
Restructuring scope covers the command files AND the shape architecture — `command-shape.md`'s two layers, the paired templates, and the runtime read chain a command obligates the lead to Read. The six-command surface (merge/kill/split) is also questionable where evidence warrants ("B, leaning D").
*Rationale:* the commands were already stripped hard once (specify 329→66 in the command-altitude wave; brainstorm at 47 lines today), so a ≥70% cut measured on command files alone may have little left to take; the true loaded cost of a run is command + obligated reads, so the read chain must be in scope for the reduction to be real.
*Review fold (M-I5 + M-M2, user-ratified batch 2026-07-30) — decode of "B, leaning D":* A (command files only) **rejected** — already-stripped files alone cannot carry the ambition; B (files + shape architecture + read chain) **adopted**; C's command-surface question (merge/kill/split) **absorbed evidence-gated** — no cardinality change ruled in-session, reopening governed by D10's named trigger; D (everything at once) **declined as a blanket grant**. Option letters do not survive undecoded hereafter.

**D2 — The 70% measures the loaded run surface, under true-reductions-only accounting.** `Confident` (recommendation adopted) — **AMENDED by user at Q8, see D2′**
~~The ≥70% target is measured on the total context a run loads — the command file plus every read it obligates — not per file.~~ The sibling session's *true-reductions-only* rule attaches and survives the amendment: moving lines from a command into the shape home (or any still-loaded file) counts as zero.
*Rationale:* the loaded surface is what a run actually pays; per-file quotas are gameable by relocation. The sibling skill ruling (calibration bands, quota rejected) noted as skills-scoped.

**D2′ — 70% was indicative, not a rule: the success criterion is goal-shape conformance, not a percentage.** `Confident` (user correction at Q8)
User verbatim: "I think 70% was more of a indication of extent of the cut i wanted, not really a hard rool. What i want is goal shape to the commands." The wave's done-criterion is structural — every command conforms to the D5 anatomy with aggressive cutting under D4's survivor discipline. Reductions are still **measured and reported** (words/chars; owned surface and run-level both) with ~70% as the ambition signal, but no percentage is pass/fail. This dissolves the Q8 fork (owned-surface vs run-level bar) — neither is a bar; both are reporting lines. True-reductions-only accounting (D2) still governs what counts as a cut.
*Rationale:* the fact-grounded arithmetic showed a run-level 70% unreachable without cutting shared skills that D6 walled off; rather than re-key the bar, the user demoted the number to intent — aligning the session with its actual driver ("too many instructions" → goal-shaped structure).
*Review fold (M-I10, user-ratified batch):* the wave's **not-done states**, structural and non-numeric — the wave is not done while: any command retains a `## Phase` or `## The flow` heading; any v4 `[PARAM]` slot is unmapped (no v5 block home and no supersession entry); any Tier-2-evidenced or `DECISIONS.md`-traceable line is neither translated nor superseded; the revised grader lacks its per-check disposition table; or any reduction claim reports against an uncorrected denominator.

**D3 — "Align to /goal" = goal-shaped documents; runtime `/goal` adoption rejected.** `Confident` (user narrowed the lead's recommendation)
Each command is restructured **condition-first** on the `/goal` doc's anatomy — one measurable end state, stated checks, constraints that matter — and the step-by-step instructional posture dies. The runtime `/goal` mechanism is NOT adopted: the lead recommended keeping it alive for later per-command assessment (option C); the user ruled it out — likely impossible for command files (`/goal` is user-typed, session-scoped), and decisively **additive** even if possible, against a session whose whole point is subtraction. Lead concurs; logged as a user narrowing, not a deadlock.
*Rationale:* "too many instructions" is the driver; the map shows the instruction mass (flow/phase machinery, restated framing) is exactly what a condition-first structure eliminates, while the goal-shaped content (done-condition, checks, gates) already exists in every command's Contract section — today at the bottom, under the instructions.

**D4 — Flow/phase narrative dies aggressively; the essentials restructure into the condition anatomy.** `Confident` (user-stated: "remove most of it, be aggressive in cutting, and structuring the essential")
The flow/phase narrative (corrected per errata: **493 genuine flow lines** across the six; 503 counting all sections outside the five named groups — the original "449" matched no derivable grouping) is not compressed or relocated wholesale — the connective procedure is deleted, and what survives is *restructured*: gates, checks, and artifact bindings take their place inside the goal-shaped anatomy (→ D5). Survivor discipline still applies: a line with named Tier-2 evidence (a behavior/failure it prevents) survives as a `KEPT:` entry; over-deletion is recoverable via the evidence-gated `RETURNED:` re-add path.
*Rationale:* the user wants a reimagining, not a trim; the rejected Q5 road (compress prose, keep phases) is measurably what previous waves did.
*Review fold (M-C1/U1, **user re-affirmed on corrected evidence**, batch 2026-07-30):* the original rationale cited plan's +98% growth, which the map's own §8.7 disqualifies as recorded build work (the on-point figures — specify +6.9%, setup +10.3% since their floors — are weak support); and the errata shows param-slot sections are the **majority of every command** (51.7% of the surface). The ruled aggression therefore lands through BOTH channels: the flow narrative dies AND the param-slot mass restructures through the D5 anatomy (84-line roster → table, Contract → the document's head, Recovery → table-only). "Flow dominance" was never load-bearing for D5 and is retracted as a premise.

**D5 — The goal-shaped command anatomy: five blocks, nothing else.** `Confident` (user: "I like the suggested structure")
A command file is: frontmatter · one-line goal + obligated-reads/probe-seat preamble · **Goal** (end state: artifact set + checks passed + gates ruled + KM landing + user acceptance; initial state FAIL, named not-done states) · **Seats & checks** (ONE table: seat · agent × skill(s) · produces/grades · spawn · peer edges — replaces roster prose AND the Contract's producer↔validator clause; independence visible structurally, no row grades its own output) · **Constraints** (the gates in order, one line each: opening evidence · who rules · what it decides; bounds stated once for all loops; workflow invariants + `KEPT:` survivors with evidence pointers) · **Bindings** (artifacts + paths + ID namespaces · fact route · verify-pass owner · checkpoint keying) · **Recovery** (one line + the evidence→resume-at table).
**The Contract section disappears as a section because it becomes the document** — `loop-discipline`'s four requirements are the file's skeleton (Goal = done-condition, table = producer↔validator, Constraints = bounds + human gates), not its appendix. Sub-fold accepted: Seats and Checks merged (roster prose and the P↔V clause carried the same information twice).
*Rationale:* demonstrated on plan.md:188–195 — the Phase-1 verdict narration distills to three constraint lines with every routing decision and trigger preserved; the killed remainder is the lead's job description and shape content, narrated. Projection (estimate, to be measured at build): surface 14,697 → ~5–6k words before read-chain work.
*Review folds (M-I7, M-I8, M-I9, user-ratified batch 2026-07-30):*
(a) **The graded exemplar** — plan.md:188–195 (~15 discrete rules) distills to exactly these three constraint lines, recorded so the pilot and its auditor grade against a fixed preservation standard (every routing decision and trigger survives; narration dies):
> — Analysis advances on `feasible` + `ready` + no blocking gap; otherwise findings route per `loop-discipline` gap-routing (knowledge → Explore / the G5 research branch · preference → G5 · scope → G6), architect concerns → G4, advocate gaps → G5, architect `infeasible` → escalated as a business-level scope decision.
> — The architect re-grades only on structural change — new/changed constraints, expanded requirement scope, or modified NFR targets; a clarification-only revision returns to the completeness pass alone.
> — Bounds (once, for every loop, in Constraints): round cap lead-counted · no-progress exit · kill-switch · out of rounds = escalate, never done.
(b) **Conformance defined** (this is D2′'s done-criterion): five blocks present per their bindings; each block carries only its class of content; gates appear as ordered constraints, never as numbered procedural steps; every v4 `[PARAM]` maps per the slot map (→ D10 step 1).
(c) **Empty blocks are conditional, not mandatory** — a command with no gates or no recovery rows omits or one-lines the block per the shape's existing conditional-binding vocabulary; brainstorm (0 gates, 0 recovery rows, 76.6% slot-bound per errata) is the worked case. The grader's presence floor keys on the binding, not the heading.

**D6 — Read chain: goal-shape the shape home, absorb Seat transport, assess dispatch line-by-line; `loop-discipline` ruled elsewhere.** `Confident` (recommendation adopted) — **AMENDED by user at Q10, see D7** <!-- review fold M-I3: amendment marker per the D2/D2′ convention -->
`command-shape.md` gets the same condition-first rewrite as the commands (a shape v5 revision — which the anatomy change requires anyway, one edit + re-audit of conformant commands). The **Seat transport** section moves out of `agent-dispatch.md` into shape Layer 2 (command-layer-only content currently sitting in a file every skill dispatch also references); the rest of `agent-dispatch.md` is assessed line-by-line. ~~**`loop-discipline` is NOT cut in this wave** — it is a shared primitive with consumers beyond commands; its size is ruled at the already-open all-consumer pass in BACKLOG (mesh-rewrite ADR escalation), per the sibling session's shared-primitive escalation doctrine (fold I5).~~ *(Superseded by D7: editable this wave under the edit-time all-consumer guard.)*
*Rationale:* the three always-read files cost 29.6k chars per run; the shape home narrates as much as it specifies; the transport merge kills a cross-file boundary that costs a reference hop on every run.

**D7 — The shared-primitive wall is reopened, scoped: skills are editable in service of goal-shape delivery.** `Confident` (user-stated: "i am open about edits to any skills to help with goal shape command delievery")
Amends D6's wall. Any skill — including `loop-discipline` and `analysis-iterative` — may be edited this wave **where goal-shaped command delivery genuinely needs it**. The *reason* for the old wall survives as an edit-time guard: before editing a multi-consumer skill, enumerate its consumers and hold their contracts (the all-consumer check moves from scheduling to edit time). Preferred first move unchanged in substance: **drop the obligated `loop-discipline` read at the command layer** (the D5 anatomy carries the four requirements structurally) — an edit to the reference, not the file. *(Amended at verify V1, user-ruled 2026-07-30: the drop lands via the **step-4 ceremony, checkpoint-gated** — never via the initial shape-v5 edit; see D10.)* Edit the shared file itself only where delivery still needs it after that.
*Review fold (M-I4) — the Q10 constraint inventory, dispositioned in full:* (1) sham-cut accounting — standing (D2). (2) shared-primitive wall — reopened and re-scoped by this decision. (3) execution ceremony — ruled at D10. (4) KEPT survivor re-grading — ruled at D8. (5) author≠grader — standing, untouched.
*Review folds (M-C2 in part, M-I13, user-ruled batch):* the read-drop carries a **replacement guarantee** — it executes only checkpoint-gated on pilot evidence that a bounded loop held with the goal-shaped structure alone (see D10's re-ruled order); if the pilot shows gate rationalization, the prior session's licensed inline backstop is the answer, never a silent re-add. Any shared-primitive edit this wave is **graded by `mochiko:validator` against an edit-specific checklist whose first section enumerates the primitive's consumers and the contract each holds**, and is surfaced per-primitive for user ratification — closing the no-referent gap (no `validation-skill-*` primitive exists).
*Rationale:* the user's driver is delivery of the goal shape, not doctrinal purity of the wave boundary; the guard keeps the wall's protective function without its scheduling cost.

**D8 — KEPT survivors and Tier-2-evidenced lines are re-graded against the new anatomy, never auto-carried.** `Confident` (recommendation adopted)
At authoring time, each `KEPT:` survivor (specify's lead-inline enrichment boundary, slice's "No G2" note) and every Tier-2-evidenced line is asked: *does the failure this line prevents still have a path in the goal-shaped file?* Still a path → the line survives, translated into the Constraints block. Structurally prevented by the new anatomy → it strips, with a **supersession-by-ruling** entry naming the structural prevention (existing strip-note vocabulary; the Kept-deliberately field covers partial survivals).
*Rationale:* the survivors' evidence was recorded against the old structure; auto-carry would embed old-shape prose in new-shape files on stale evidence, while plain deletion would discard evidence that may still bind — re-grading is the only path that honors both.
*Review fold (M-I11/U4, user-ruled batch 2026-07-30):* the protection set extends beyond Tier-2-evidenced lines to **every command line traceable to a `DECISIONS.md` row** — the architecture gates, deviation checks, and absorbed structuring loop carry rows (~11 repo-days old) but name no prevented failure, making them Tier-2-deletable by the letter of the criterion. Such lines enter the re-grade set and survive unless their decision is superseded. Enumeration procedure: the authoring pass greps the per-command `DECISIONS.md` row trace before any cut.

**D9 — The grader is revised with the shape: `validation-command-shape` lands alongside shape v5, before any command is re-authored.** `Confident` (user-ruled at review batch 2026-07-30 on the reworked form below; originally lead-proposed and unobjected — see M-I6)
The current grader's floor keys on the v4 anatomy and would FAIL every conformant goal-shaped file. **Reworked per M-C3 (user-ratified):** the revision ships with a **per-check disposition table covering all ten existing checks** (kept / dropped / re-keyed, with why), which the validator audits for completeness — silence on any check is itself a FAIL. Known dispositions: checks 2, 5 **kept**; 3's signature list **re-keyed** to the new forbidden markers; 7 **re-keyed** to the v5 slot list; 8 **re-keyed** (Contract-section fill → Goal/Constraints soundness); **1 re-keyed by ruling** — its reference set changes with D7's read-drop, so its disposition lands with the D10 checkpoint, never silently; **4 re-keyed** — both live `shape-exception` markers re-justified against v5; **6 (altitude) and 9 (preserved responsibilities) carried forward unchanged**, with 9 gaining a clause **grading D8's structural-prevention claims** (a supersession entry claiming "structurally prevented by the new anatomy" is verified against the anatomy, never taken on the author's say-so); **10 (strip-note quality) kept** <!-- verify-pass precision fix: restored from the pre-rework D9 text -->. New floor checks: block presence per binding (not per heading — D5 fold (c)); forbidden markers `## Phase` / `## The flow`; **a forbidden ordinal-step pattern inside Constraints plus a per-block line ceiling keyed to gate count** (M-I7's teeth — ordering narrative cannot return under a permitted heading); the Seats & checks table's mechanical no-row-grades-its-own-output check. Author≠grader intact: command-architect authors both revisions, validator grades.
*Rationale:* the new anatomy is more deterministically auditable than the old — block presence, forbidden headings, and table independence are all greppable; the wave must have a working independent grader from day one.

**D10 — Execution: pilot-first, plan pilots the full loop.** `Confident` (recommendation adopted)
Order *(re-ruled at review batch 2026-07-30, M-C2/U2 + M-I8 + M-I9 + M-I12/U5)*: (1) shape v5 (D5 anatomy + D6 transport merge — **without the read-drop**) and the D9 grader revision land together, publishing the **v4→v5 slot-by-slot map** (every v4 `[PARAM]` → its v5 block, or retired by supersession entry — never by omission) and the **per-command parameter-floor arithmetic** (slots + gates × facts + seat rows + recovery rows vs the projection; a floor exceeding the projection changes the anatomy or the ambition *before* the pilot). (2) **plan** — heaviest file, every content class (6-seat roster, 7 gates, slice-scoping, architecture stage) — is re-authored through the full loop **with the `loop-discipline` read retained**: author → independent audit → measure → **confirm-or-revise-the-anatomy checkpoint with the user**, which additionally grades whether the goal-shaped structure held the loop (gates un-rationalized, bounds honored). (3) The **checkpoint rules the read-drop on that evidence** (D7's replacement guarantee). (4) The remaining five re-author in one wave, per-command ratification, one audit ceremony + version bump — **and the approved read-drop lands inside this same ceremony** *(verify fold V1, user-ruled 2026-07-30)*: one shape-home edit riding the wave's version bump, its mandated re-audit satisfied by this ceremony's audit of the five **plus a named delta re-audit of pilot plan** (one file, one clause — named so it cannot be silently skipped), and the shape-home edit itself graded under `validation-command-shape`'s shape-revision checks (11–14), same as step 1's v5. **Cardinality (M-I12/U5, user-ruled): deferred with a named trigger** — the question reopens at the pilot checkpoint if goal-shaped plan exceeds ~2× the median goal-shaped command or its gate set resists a single Goal statement; otherwise the six-command surface stands. This record's cold review preceded all execution (this section).
*Rationale:* the anatomy is unprecedented — no audit-cleared precedent exists for goal-shaped commands, so the one-ceremony shortcut doesn't apply; a defect caught at the pilot costs one file, not six.

## Corrections & reversals

- **C1 (2026-07-30, from checker map §0):** the lead's session-setup line named `plugins/mochiko/strips/` as the strip-note home. Wrong path, and doctrinally loaded — strip notes were ruled *out* of `plugins/` 2026-07-19 ("a strip note anywhere under `plugins/` is a defect — fix on sight"). Corrected to `.mochiko/strips/`. Lead formulation error, fixed on sight.
- **C2 (2026-07-30, from checker map §2):** the "1308 lines total" baseline is arithmetically correct but non-comparable across commands — five files are hard-wrapped at ~64–86 chars/line, brainstorm.md is unwrapped (203 chars/line), and a past rewrap doubled line counts at +1.0% chars. Session measurements proceed in **words/chars**; line counts are kept as raw fact only.
- **C3 (2026-07-30, review fold M-M2 — label discipline):** two collisions found in cross-exam: "option C" named different rejected roads in Q4 (runtime-`/goal`-later) and Q5 (compress-prose-keep-phases), and the map's §3 cites plan.md:227 as "D8/R5" — that is the *architecture-design session's* D8 (AD-D8/R5), not this record's D8. Conventions adopted: in-chat option letters never enter the record undecoded (D1 fixed under M-I5); cross-session decision labels carry their session prefix.

## Review

**Sizing gate (2026-07-30):** weight stated — 11 rulings (D1–D10 + D2′), all `Confident`; heavy reality-surface load carried by the verbatim map; adoption-provenance note given to the user (D1, D2, D6, D8, D10 recommendation-adoptions; D2′, D3, D4, D7 engaged rulings). Lead recommended **pair**; **user ruled: pair.** Record frozen at spawn; lens split per the command — decision-quality vs record-integrity; map named as fact substrate in both briefs; counterparts withheld until findings formed.

**Cross-examination (2026-07-30):** completed per protocol, four messages. Per-reviewer tallies: reviewer-dq 12 raised → 12 survived (2C/10I after in-exam concessions/upgrades); reviewer-ri 12 raised → 12 survived (1C/9I/2M, three partial withdrawals). Both recommend `critical-gaps`. No fact disputes routed during the exam; both reviewers' substrate note agrees: **every map failure is in a derived aggregate; every primary measurement holds.**

**Lead cross-set merge — combined tally: 24 raised → 24 survived → 18 merged findings (3 Critical · 13 Important · 2 Minor):**

- **M-C1** (RI-C1 + DQ-C1 + RI-I1): map §3's param-slot/remainder columns wrong in all six rows; §9.7 inverts once corrected (flow exceeds param-slot in ZERO commands); D4's "449 lines" object matches no derivable grouping (503 all-six / 406 three-named); D4's rationale cites plan's +98% growth, which the map itself disqualifies as recorded build work. → fact-checker errata + **user re-confirms the "aggressive" ruling on corrected evidence** (U1).
- **M-C2** (DQ-C2): D7's obligated-`loop-discipline`-read drop never engages the prior session's recorded invocation-reliability risk; D10's ordering lands the drop before the pilot can test it. → D7 names a replacement guarantee; **D10 re-order is the user's** (U2).
- **M-C3** (DQ-I2 upgraded in-exam): D9's grader revision silent on old checks 1, 4, 6, 9 (incl. both over-deletion catches); nothing audits the revised grader. → per-check disposition table (kept/dropped/re-keyed + why), validator-audited; check 9 carries forward grading D8's structural-prevention claims.
- **M-I1** (RI-I2): §3 assigns brainstorm 10 "flow" lines that are slotted content per §8.1 — reclassify/relabel (rides errata).
- **M-I2** (RI-I3): §5 charges implement/plan for a template neither references and brainstorm for teammate-side CROSS-EXAM.md — re-issue rows (rides errata).
- **M-I3** (RI-I4): D6 reversed by D7 without the amendment-marker convention D2 established — apply marker + strikethrough to every amended decision.
- **M-I4** (RI-I5; DQ-M1 dropped as duplicate): D7's constraint open-list stale in both directions and the Q10 inventory absent — reconcile against D8/D10, disposition all five constraints.
- **M-I5** (RI-I6): D1's "B, leaning D" un-decoded — expand into named alternatives with why each lost.
- **M-I6** (RI-I7 + DQ-I9 merged): 11/11 rulings `Confident`; D9 textually never-confirmed; record silent on which adoptions the user confirmed — **per-decision confirmation is the user's** (U3).
- **M-I7** (RI-I8 + DQ-I6 folded): Constraints block carries ordering the deterministic floor can't police, and "goal-shape conformance" (D2′'s done-criterion) is defined nowhere beyond five heading names — D5 ordering fix or D9 floor-with-teeth; define conformance + how the pilot tests it.
- **M-I8** (RI-I9 + DQ-I4 folded): no v4→v5 slot-by-slot map (:31/:47/:160 unclaimed by any block); empty-block question (brainstorm: 0 gates, 0 rows) unruled — publish the map before the pilot, conditional-binding vocabulary, brainstorm as worked case.
- **M-I9** (RI-I10 + DQ-I5 composed): D5's demonstrated three lines absent from the record (the 8 source lines carry ~15 discrete rules); no anatomy-floor arithmetic — paste the graded exemplar; compute per-command parameter floors before the pilot.
- **M-I10** (DQ-I1): the wave's own done-criterion has no not-done states while D5 obliges every command to carry them — add structural, non-numeric not-done states.
- **M-I11** (DQ-I3): operative build instructions traceable to `DECISIONS.md` rows but naming no failure are Tier-2-deletable — **extending D8's protection set is the user's** (U4).
- **M-I12** (DQ-I7): D10 calibrates on plan, the surface's most obvious split candidate; D1 licensed cardinality change — **rule or defer cardinality with a named trigger before the pilot, user's** (U5).
- **M-I13** (DQ-I8): D7 moves the all-consumer check to the interested author's judgment and no `validation-skill-*` grader exists — name the grader (agent × skill) for any shared-primitive edit, or per-primitive user ratification.
- **M-M1** (RI-M1): §9.5 Recovery total 143→163 (10.9%→12.5%), row arithmetic 82→72 (rides errata).
- **M-M2** (cross-exam-derived label cluster, uncounted by both reviewers, taken by the lead at Minor): "option C" collides across D3/D4; map §3's "D8/R5" collides with this record's D1… namespace — adopt disambiguating label conventions.

**User batch ruling (2026-07-30):** U1–U5 presented with per-item recommendations and confidence levels; **user adopted the batch** ("i will go with your recommendation"). Per M-I6, this batch adoption — given in direct response to the finding that five decisions were unconfirmed recommendation-adoptions — constitutes the user's explicit confirmation of **D1, D2, D6, D8, D10** (marks stay `Confident` with this provenance) and the explicit ruling on **D9's reworked form**. D3, D4 (re-affirmed on corrected evidence at U1), D7, and D2′ already carried engaged user rulings.

**Dispositions — 18/18 landed:**

| finding | disposition | where it landed |
|---|---|---|
| M-C1 | **user-ruled** (U1) | Map errata items 1–3 + D4 re-key (object → 493/503 corrected denominators; the original 449 matched no *stated* grouping — it is slice+setup+implement+plan, which the map never named; +98% premise retracted) |
| M-C2 | **user-ruled** (U2) | D10 re-ruled order (checkpoint rules the drop at step 3) + D7 replacement guarantee *(amended at V1 — the drop lands at step 4's ceremony)* |
| M-C3 | **resolved** (user-ratified in batch) | D9 reworked: per-check disposition table, checks 6+9 carried, 9 grades D8 claims |
| M-I1 | **resolved** | Errata item 6 (brainstorm flow/phase = 0; column relabelled) |
| M-I2 | **resolved** | Errata item 4 (§5 re-issued, named-reads-only, ranges + unmeasured marked) |
| M-I3 | **resolved** | D6 amendment marker + strikethrough (D2/D2′ convention applied) |
| M-I4 | **resolved** | D7 constraint inventory dispositioned in full (1–5) |
| M-I5 | **resolved** | D1 decode of "B, leaning D" |
| M-I6 | **user-ruled** (U3) | Batch-adoption provenance above; D9 mark re-grounded |
| M-I7 | **resolved** | D5 fold (b) conformance definition + D9 ordinal-pattern/line-ceiling floor teeth |
| M-I8 | **resolved** | D10 step 1 slot-by-slot map + D5 fold (c) conditional blocks (brainstorm worked case) |
| M-I9 | **resolved** | D5 fold (a) graded exemplar (3 lines, ~15 rules) + D10 step 1 floor arithmetic |
| M-I10 | **resolved** | D2′ structural not-done states |
| M-I11 | **user-ruled** (U4) | D8 extension to `DECISIONS.md`-traceable lines |
| M-I12 | **user-ruled** (U5) | D10 cardinality deferral with named trigger |
| M-I13 | **resolved** | D7: `mochiko:validator` × edit-specific consumer checklist + per-primitive ratification |
| M-M1 | **resolved** | Errata item 5 (163/12.5%; table lines 67 = 57 data + 5 + 5) |
| M-M2 | **resolved** | C3 label conventions + D1 decode |

Zero recorded-open; zero overruled (no element newly marked `Contested`). **Verify pass:** dispatched to reviewer-ri (the record-integrity reviewer) over all 18 folds — grading that each landed where claimed, quoting evidence, plus any contradictions the folds themselves introduce.

**Verify-pass outcome (2026-07-30):** 16 landed clean · 2 landed-with-contradiction · 0 not-landed — **not clean on first pass**.
- **V1 (blocking, from M-C2's fold):** D7's "via shape v5" clause contradicted D10's re-ruled order, hiding an unaccounted second shape revision (re-audit + version bump). **User-ruled:** the checkpoint-approved read-drop lands **inside step 4's ceremony** — one shape-home edit on the wave's bump, re-audit satisfied by the ceremony's audit of the five plus a **named delta re-audit of pilot plan**. D7 and D10 amended accordingly; bounded re-verify dispatched.
- **V2 (editorial, from M-I2's fold):** errata residues (stale §8 item 10 shares, the §10 mis-pointer, one setup line-cite) — resolved by the checker-authored **corrigendum**, which also settled one fact dispute **against** the reviewer: the 27.1% figure stands (29,611/109,342, neither term changed).
- **Precision fixes (verify-suggested, lead-penned):** D9's check-10 disposition restored ("10 kept"); M-C1's "449" phrasing corrected to "no *stated* grouping".

**Bounded V1 re-verify (2026-07-30): CLEAN.** The three sites (D7, D10, Review) agree verbatim; the four-step chain is internally consistent (v5 without the drop → pilot with the read → checkpoint rules → ceremony lands); the second-shape-revision accounting is closed on both dimensions (conformant coverage complete: five by ceremony + plan by named delta; one bump, not two). Two non-blocking precision clauses applied on the reviewer's note: the step-4 shape edit's own grade named (checks 11–14) and the M-C2 row tagged with the V1 amendment. The reviewer also placed on the record that the corrigendum was right against its residue (27.1% stands) and right in correcting its 72→67 table-line figure.
