# Verbosity, Caveman & Ops-vs-Shipped Separation — Decision Record

**Status:** converged — **frozen for cold review** (D1–D10 complete; pair spawned at sizing
gate; Review section excepted from freeze) · run-start declaration: default run (no
departures composed) — weight card user-ruled 2026-08-01: default accepted (pair review at
convergence), all three threads in this session · sizing ruling: heavyweight (10 decisions ·
8 Confident / 1 Contested + 1 partial / heavy fact load F1–F74) → **pair**, the declared
default, no trail line · counted unit: lead↔reviewer exchange per survivor (0 consumed) ·
fact-checker seat filled at start (probe seat, roster-confirmed), cadence-exempt, standing
for fact disputes

## Topic

Three threads, brought together by the user (2026-08-01):

1. **Verbose output & reports** — mochiko's user-facing output and its report layer are too
   verbose (BACKLOG: "Reduce mochiko output verbosity — explore caveman", ergonomics section).
2. **Caveman as part of the mochiko application** — the terse-output style currently lives
   only in this repo's `CLAUDE.md` as an operator preference; explore making it a shipped
   part of mochiko (dial, surface placement, seam with the token-reduction epic).
3. **Ops-vs-shipped separation** — mochiko's own operating primitives (ops agents, commands,
   skills used to build/maintain mochiko itself) versus what ships to end users in the
   plugin; whether they should be treated/packaged differently.

Related open items: "Plain-language sweep + internal-jargon leak" (ergonomics) ·
"Intensity modes" (open design decisions, 2026-06-27) · token-reduction epic (report layer).

## Run-start weight card

Lead's read of the four rigor factors (user rules the card):

- **Reversibility:** high — session record; builds land later; rework cheap.
- **Blast radius:** medium-high — output style touches every command's user surface;
  ops-vs-shipped touches plugin packaging that every install consumes.
- **Precedent:** process precedented (~10 pair-form sessions); threads 1–2 are recorded
  captures; thread 3 is a **new surface** with no backlog item.
- **Input confidence:** medium — user-named threads (ambiguity discounted), but thread 3
  introduces new surface → consistency risk scored on this record, not the ask.

Composed process: the stated default — fact-checker map at start, inline one-question-per-turn
discovery, sized end-stage review at convergence (default: heavyweight → pair).

## Fact map

*(fact-checker authored — lands verbatim below this scope note; lead writes around, never
restates. Delivered 2026-08-01 in one message, four sections F1–F69 + 19 "cuts both ways"
items. The mochiko-app shallow clone referenced in section 4 lives in session scratchpad.)*

# Reality map — verbosity / caveman / ops-vs-shipped (fact-checker, 2026-08-01)

All paths relative to `/Users/deepeshadmin/Documents/GitHub/mochiko` unless marked. Every fact Read or grepped this session. Quotes verbatim.

---

## Thread 1 — output & report verbosity

**F1.** `BACKLOG.md:294-296` — section exists, freshly captured: `## Ergonomics: output, language & run-hygiene`, then *"Raw captures (2026-08-01) — to triage/brainstorm; grooming may re-key into finer themes."* Four open items under it; two are this thread's.

**F2.** `BACKLOG.md:313-317`, verbatim: *"**Reduce mochiko output verbosity — explore "caveman"** (2026-08-01; provenance: capture session, to-brainstorm) — mochiko's user-facing output is too verbose; explore the "caveman" terse-output style and how to bring it into `setup` and mochiko generally. Brainstorm: where terseness helps vs. where detail is load-bearing, global dial vs. per-surface, and the seam with the token-reduction epic (which targets inter-agent/report tokens, not user-facing prose)."*

**F3.** `BACKLOG.md:318-325`, verbatim: *"**Plain-language sweep + internal-jargon leak to end users** (2026-08-01; provenance: capture session, to-brainstorm) — the language across mochiko is too complex; needs a plain-English sweep. Concrete leak: the plugin's end user is shown "Layer -2" (internal shape/architecture vocabulary) which means nothing to them. Sweep finding (2026-08-01): "Layer -2" appears in no shipped file — the leak is runtime lead prose, so a file sweep won't find it; the fix targets the shape's user-facing vocabulary ban (today an enumerated three-term list) and interacts with the R5 record-don't-build ruling. Brainstorm: the end-user vs. internal vocabulary boundary, and where the ban's term list should grow."*

**F4.** `plugins/mochiko/templates/command-shape.md:196-198` (Layer 1), the whole rule, verbatim: *"**The conversation is the production surface.** It belongs to the lead and the user, and it carries no machinery: no "phase", "round", or "gate" talk in user-facing prose. (The anatomy bans phase *headings*; this bans the vocabulary the user hears.)"* Three banned terms exactly — confirms F3's "enumerated three-term list".

**F5.** Sibling ban, anatomy level, `command-shape.md:43-44`: *"**Gates appear as ordered constraints, never as numbered procedural steps.** No `## Phase` heading, no `## The flow` heading, and no ordinal-step list inside **Constraints**."* Governs command file structure, not user-facing prose.

**F6.** Other shipped lines touching user-facing prose, complete set (grep `user-facing|plain language|plain English|narrat|jargon|in the user's terms` over `plugins/mochiko/`):
- `command-shape.md:55` — P1 goal line is *"what this run is for, in the user's terms"*.
- `command-shape.md:84-85` — user acceptance is *"plain blocking text, never a timed prompt."*
- `command-shape.md:158` — the lead owns *"the user-facing conversation."*
- `command-shape.md:266-269` (Layer 2, Seat legibility) — *"Announce each seat in one line when it is filled — an unexplained teammate spawn reads as a malfunction. Teammate housekeeping (idle notifications, acks) is never narrated and never replied to."*
- `authoring-user-stories/SKILL.md:43,89` — *"user journey in plain language — ≤ 2 lines"* (artifact content, not conversation).

**F7.** Absence: no shipped file prescribes tone, register, or length for lead→user prose beyond F4/F6. Greps run: `user-facing|user-visible|plain language|plain English|jargon|verbosity|verbose|brevity|terse|concise|succinct` over all of `plugins/mochiko/`. The only length rules found apply to artifacts and reports, never to conversation.

**F8.** `plugins/mochiko/templates/report-format.md:9-15`, the whole "Who reads a report" section, verbatim: *"The lead, forming a verdict or relaying a failure/gap list — a model, not a human. The user receives lead-authored summaries, never report files. So reports are **machine-first**: every consumed datum is a structured frontmatter field; prose exists only where structure cannot carry the content (see the conditional-prose rule). Do not write for a human-style reader; write fields a verdict can be formed from."*

**F9.** `report-format.md:36-68` — seven shared rules: machine-first · conditional prose (failure narrative mandatory on FAIL; *"A clean/passing report is frontmatter-only"*) · omit-empty · no self-verdict on disclosures · no restatement (cite IDs) · findings YAML schema · evidence captured not narrated. `report-format.md:71-74`: **Format version v1 (2026-07-23 — workflow-token-reduction wave 1)**, governed by the token-reduction epic record.

**F10.** `plugins/mochiko/templates/artifact-format.md` exists (69 lines). `:12-18`, "Who reads a deliverable", verbatim: *"Unlike a report (one consumer: the lead), a deliverable is read many times: by the human at its acceptance gate, by every mandated cold-reviewer read, and by every downstream producer — roughly ten model reads per feature. Every kilobyte is re-paid at each read. So deliverables are **dense by construction and human-legible**: not machine-first frontmatter, but no sentence that a field, a table row, or an ID citation could carry."*

**F11.** `artifact-format.md:34-37` rule 4 (size guidance): *"Overview / context / rationale prose defaults to ≤ 3 lines; list entries … are one line each. These are defaults, not caps on substance."* And `:46-50` rule 8: *"**Density is not a gap (the review rule).** Reviewers grade substance … never prose volume. Brevity is never itself a finding."* `:63-64`: **Format version v1 (2026-07-24 — workflow-token-reduction wave 2)**.

**F12.** Token-reduction epic scope, `.mochiko/brainstorms/workflow-token-reduction/record.md:9-14`. Reality surface = *"the plugin source (`plugins/mochiko/` commands, agents, skills, templates: what each run loads and spawns)"* plus prior records with token measurements, expanded to the kinako dogfood repo. User constraints (Q1, Confident): library-wide scope · latency not a constraint · *"Target is **low-hanging fruit** — high savings-to-effort ratio, not a redesign"* · *"Quality floor: no drastic quality hit."*

**F13.** The six decisions and what layer each targets (`record.md:252-279`): **D1** hunt order (pure waste, then quality-machinery sizing) · **D2** per-run recorded cost entry, manual baseline + OTel probe · **D3** slim implement's per-cycle + verification reports · **D4** reference-by-ID down the artifact chain · **D5** review sizing gates + verification-depth floor · **D6** four pure-waste fixes (reference splits, orphan refs, runtime HTML comment headers, round-report clean-by-default). **Every one targets context loads, report files, artifact files, or reviewer counts. None targets lead→user prose.**

**F14.** The who-reads-reports finding, `record.md:169-181` (F-c Part 1), verbatim conclusion: *"**The user gets a summary, never the report files:** implement.md Phase 3 (G5) — "Present the verified implementation (cycle / task / fix-pass counts, quality-gate results, an **evidence summary**, any noted gaps)." A lead-authored summary, not the reports. **This matches the user's report of never reading them** — the machinery never routes a cycle/verification report to the user."* Supporting quotes in the same block: `implement.md:119`, `:138`, `:173`, `executing-tdd-cycle/SKILL.md:28`, `:87`, `CYCLE-REPORT-FORMAT.md:3` — all name the lead as report reader.

**F15.** D3's rationale re-states it (`record.md:263`): *"the sole live consumer is the lead's verdict (frontmatter + evidence); the user never reads them (Q4)."*

**F16. Notable absence — the boundary F2 asserts is not a ruling in the record.** Grep `user-facing|user-visible|prose to the user|conversation` over `record.md` (311 lines) returns 3 hits, all unrelated (`:50` SKILL.md persistence "enters the conversation"; `:258` "user-visible usage figure" from `/usage`). The record contains **no statement scoping user-facing prose in or out**. The exclusion list `record.md:284-290` ("Considered and not pursued") names five items — wholesale reference loads · stopping cold-reviewer re-reads · humaninloop plugin manifest · lead-side doctrine reads · skill-description manifest trimming — and user-facing prose is not among them. So the boundary is **true by the avenue set's construction (F13) but never ruled**; BACKLOG's parenthetical is a summary, not a quotable ruling.

**F17.** Epic status: `record.md:5` — ACCEPTED 2026-07-23, *"**BACKLOG deliberately deferred** — the user re-examines the issues from more angles before locking build items."* Waves 1 and 2 shipped (`.mochiko/decisions/2026-07-23-token-reduction-wave-1.md`, `2026-07-24-token-reduction-wave-2.md`; the two format files' version stamps, F9/F11). `ROADMAP.md:21` Next: *"Token epic: D5 sizing-gate generalization + the one-shot OTel probe."*

**F18.** `ROADMAP.md:40`, under **Later *(non-committed)***: *"Ergonomics sweep: output verbosity (caveman) · plain-language / internal-jargon leak (e.g. "Layer -2") — validator worktree isolation built 2026-08-01 (v0.42.0, trail)"*. `ROADMAP.md:44` also lists Later: *"Token wave-3 candidates (governance/memory layer · brainstorm records)"*.

---

## Thread 2 — caveman as part of the mochiko application

**F19.** The caveman spec lives at `CLAUDE.md:3-34` — this repo's root operating manual, as the **first section**, above `## What this is` (`:38`). Not inside any marked region.

**F20.** Scope line, `CLAUDE.md:5-6`, verbatim: *"**Standing instruction. Applies to every response, every turn. Does not decay over a long session. Off only when the user says "stop caveman" or "normal mode".**"* It binds whichever agent session reads this repo's `CLAUDE.md` — i.e. the operator session working *on* mochiko. Nothing in it names a mochiko command, run, or end user.

**F21.** Core rule, `CLAUDE.md:8`: *"Respond terse like smart caveman. All technical substance stay. Only fluff die."*

**F22.** Drop list, `CLAUDE.md:10-13`, verbatim: *"**Drop:** articles (a/an/the), filler (just/really/basically/actually/simply), pleasantries (sure/certainly/of course/happy to), hedging, tool-call narration, decorative tables and emoji, long raw log dumps — quote the shortest decisive line instead. Fragments OK. Short synonyms: "fix", not "implement a solution for"."*

**F23.** Never-compress list, `CLAUDE.md:14-16`, verbatim: *"**Never compress:** code blocks, commands, file paths, identifiers, API names, error strings — verbatim. Technical terms exact. Standard acronyms (DB/API/HTTP) OK; never invent new ones (cfg/impl/req/res/fn) and never use arrows (→) — both save zero tokens and cost the reader clarity."*

**F24.** `CLAUDE.md:17-18`: *"**Never announce the style.** No "caveman mode on", no third-person caveman tags, no normal answer plus a caveman recap. Output caveman-only."* And `:19-20`: *"**Keep the user's language.** … Compress the style, not the language."*

**F25.** Exemptions, `CLAUDE.md:25-31`, verbatim, two clauses: *"**Drop caveman, write plainly, for:** security warnings · irreversible or destructive action confirmations · multi-step sequences where dropped conjunctions risk misordering · anywhere compression makes the technical meaning ambiguous · when the user asks to clarify or repeats a question. Resume after that part is done."* — and — *"**Written artifacts are exempt** — code, comments, commit messages, PR bodies, and every file in the repo are written normally."*

**F26.** Levels, `CLAUDE.md:33-34`, verbatim: *"Levels, default **full**: `lite` = no filler, keep articles and full sentences · `full` = drop articles, fragments OK · `ultra` = one word when one word enough, state each fact once."*

**F27.** `BACKLOG.md:25-26`, verbatim: *"**Intensity modes** (2026-06-27; provenance: unrecoverable — same source) — global `lite/full/ultra/off` dial vs per-rule. Defer until the pattern is clear from real runs."* Same level vocabulary as F26 plus `off`. The item does not say what the dial modulates; provenance is recorded unrecoverable.

**F28. Proven absence in shipped files.** Grep `caveman|terse|verbosity|verbose|brevity|concise|succinct` (case-insensitive) over all of `plugins/mochiko/`: **zero hits for `caveman`.** All other hits are non-style: session-slug provenance strings (`command-succinctness-strip` in `command-shape.md:49,75,396`, `agent-dispatch.md:59`, `sized-end-stage-review.md:52`), template field descriptors (*"Descriptive, concise"* ×5 in `authoring-technical-requirements/references/ARTIFACT-TEMPLATES.md`; *"Concise title"* in `patterns-technical-decisions/references/DECISION-RECORD.md:10`), `artifact-format.md:48` (*"Brevity is never itself a finding"*, F11), and one anti-pattern line, `authoring-commands/SKILL.md:113`: *"**Deleting what should relocate.** Verbosity in a command is usually mis-altitude, not…"*. **No terseness or output-style guidance ships today.**

**F29. Surfaces that exist where a dial could live** (what files show, no speculation):
- `command-shape.md` Layer 1, the F4 paragraph — the only shipped user-facing-prose rule, obligated-read by all six commands.
- The setup-authored **CLAUDE.md governance region** in a user project — `templates/governance-surfaces-template.md:24-53`: *"Regenerated in place between the markers"*, delimited `<!-- mochiko:governance:begin -->`, sections `### Principles` · `### Technology stack` · `### Quality gates` · `### Governance operations`. **No prose/style section in the shape.**
- Four shipped constitution modules, `templates/constitution-modules/`: `evolution-notes.md`, `knowledge-management.md`, `layer-rules.md`, `release-gates.md`. None governs response style.

**F30.** This repo's own `CLAUDE.md` has **no** `mochiko:governance:begin` marker (grep). Its `##` headings: Response style — caveman mode · What this is · Reference sources · Non-negotiable constraints · How to work in this repo · Skill-library conventions. So the caveman spec is hand-authored operator prose, not setup-generated, and no `/mochiko:setup` run has ever produced anything like it.

**F31. Claude Code output-style mechanism — not documented in this repo as a mechanism.** Grep `output.?style|outputStyle` over all `*.md`/`*.json`: hits are (a) `BACKLOG.md:315` (its own phrase "terse-output style"), (b) this session's `record.md:14` + `.mochiko/brainstorms/index.md:14`, (c) `.mochiko/brainstorms/team-lead-strategic-compaction/record.md:51` and `:144` — the compaction-survival table quoted verbatim from `code.claude.com/docs/en/context-window`: *"system prompt and output style → "Unchanged; not part of message history""*. That is the only citation of a platform output-style surface anywhere in the repo, and it is about compaction survival, not about authoring or configuring one.

---

## Thread 3 — ops-vs-shipped separation

**F32.** `plugins/mochiko/.claude-plugin/plugin.json` — **version `0.43.0`**; `"commands": "./commands/"`, `"skills": "./skills/"`, `agents` an explicit 10-entry array. Marketplace manifest is separate: `.claude-plugin/marketplace.json`, `metadata.version 0.10.0`, one plugin, `"source": "./plugins/mochiko"`.

**F33. What ships — 6 commands:** `brainstorm.md` · `implement.md` · `plan.md` · `setup.md` · `slice.md` · `specify.md`.

**F34. 10 agents:** `command-architect` · `devils-advocate` · `principal-architect` · `qa-engineer` · `requirements-analyst` · `staff-engineer` · `system-architect` · `task-architect` · `technical-analyst` · `validator`.

**F35. 30 skills:** analysis-codebase · analysis-iterative · authoring-architecture · authoring-commands · authoring-constitution · authoring-requirements · authoring-slices · authoring-technical-requirements · authoring-user-stories · brownfield-integration · executing-tdd-cycle · grooming-operating-docs · loop-discipline · **mochiko** (router) · patterns-api-contracts · patterns-entity-modeling · patterns-system-design · patterns-technical-decisions · patterns-vertical-tdd · review-brainstorm · review-feasibility · review-governance-intent · review-plan-artifacts · review-slices · review-specifications · review-task-artifacts · testing-end-user · testing-governance-injection · validation-command-shape · validation-constitution. Subdirectories: 18 `references/`, 6 `scripts/` (`detect-stack.sh`, `validate-model.py`, `validate-openapi.py`, `check-artifacts.py`, `validate-requirements.py`, `validate-user-stories.py`).

**F36. 20 templates** + `templates/constitution-modules/` (4 files, F29): advocate-report-template · agent-dispatch · analyst-report-template · artifact-format · codebase-analysis-template · command-shape · feasibility-report-template · governance-intent-template · governance-surfaces-template · plan-template · report-format · sized-end-stage-review · slicer-report-template · slices-template · spec-template · sysarchitect-report-template · taskarchitect-report-template · tasks-template · techanalyst-report-template · workflow-contract.

**F37.** Exactly **one** shipped skill is user-invoked: grep `disable-model-invocation: true` over `plugins/mochiko/skills/` returns `mochiko/SKILL.md:4` only. (The second grep hit, `validation-command-shape/SKILL.md:74`, is a *check line* about the command being graded — *"**Frontmatter** — `disable-model-invocation: true` present"* — not its own frontmatter.) So `validation-command-shape` and `authoring-commands` are model-invoked like every other non-router skill.

**F38. Stated consumers of the ops-flavored primitives:**
- `command-architect` (`agents/command-architect.md:4-8`): *"Framework smith who authors and converts orchestration commands to a codified shape, and executes ruled revisions of the shape's single-sourced home… Builds, converts, and revises; never grades its own output."* Its four worked examples all name mochiko's own commands.
- `authoring-commands` (`SKILL.md:4-14`): *"Author a mochiko command in the codified command shape… Use when creating a new `commands/*.md` supervisor… MUST BE USED when the task says "author a command"…"* Consumer named as whoever authors a mochiko command file.
- `validation-command-shape` (`SKILL.md:4-19`): *"Independently grade a mochiko command's conformance to the codified goal-shaped command shape… MUST BE USED when the task says "grade this command"…"*
- `grooming-operating-docs` (`SKILL.md:3`): fires when *"a knowledge-management invariant cap or bound trips at a command boundary"*; *"Resolves every cap, bound, and format from the project-pinned copy at `.mochiko/memory/knowledge-management.md`; **no copy → nothing to groom**"*; *"Attached to already-firing command boundaries (brainstorm open/close, setup/amend, specify/plan/implement landings)."* Consumer = **any project that adopted the KM module**, not mochiko specifically.
- `loop-discipline` (`SKILL.md:3`): *"MUST be invoked when designing, authoring, or reviewing any mochiko workflow, command supervisor, or agent loop… The doctrine skill behind every mochiko loop."* Doctrine for every workflow; not repo-ops-specific.

**F39. The router labels two of them, and only two, as framework-facing.** `skills/mochiko/SKILL.md:115` heading, verbatim: *"### Framework maintenance (model-invoked — reached when authoring or auditing mochiko's own commands)"* — the table holds exactly `authoring-commands` (`:118`) and `validation-command-shape` (`:119`). The `authoring-commands` row itself says *"running a strip wave with version-stamped notes (`.mochiko/strips/`, **repo-side**)"*. In the Agents table, `:143`: *"`command-architect` | **framework-facing producer** — authors/converts commands in the codified shape and first-passes strip waves."*

**F40. The router does NOT class the KM skills as framework maintenance.** `grooming-operating-docs` (`SKILL.md:51`) and `authoring-architecture` (`:52`) sit under **"### Setup cluster (model-invoked — auto-reached during a `/mochiko:setup` run)"** (`:43`) — i.e. indexed to end users as normal pipeline skills.

**F41. The strips convention is the repo's existing ops/shipped ruling — and it is explicit.** `.mochiko/strips/README.md:11-18`, verbatim: *"LOCATION (amended 2026-07-19, user ruling): `.mochiko/strips/` — repo-side, beside the other operational layers (.mochiko/transform/, .mochiko/brainstorms/). Strip notes are operational maintenance logs and must NEVER live under `plugins/` — the plugin directory is the shipped artifact, and anything inside it distributes with the plugin whether loaded or not. (The original D6 ruling placed them at plugins/mochiko/strips/; relocated out of the shippable tree once the first wave made the leak visible.) A future wave that writes a strip note anywhere under `plugins/` is a defect — fix on sight."* Same file `:4-9`: *"This directory is deliberately NON-LOADED… It is read at framework-maintenance time only (strip waves, dogfood re-add reviews, validation-command-shape audits)."* `authoring-commands/SKILL.md:83` re-states the constraint: *"repo-side, never under `plugins/`"*.

**F42.** `plugins/mochiko/strips/` **does not exist** (`ls` → No such file or directory). `.mochiko/strips/` exists with a README + ~35 per-primitive notes.

**F43. Repo-ops surfaces already outside the plugin:**
- `.claude/rules/mochiko/` — **2 files.** `operating-docs.md`, `paths`-scoped to `DECISIONS.md`, `ROADMAP.md`, `BACKLOG.md`, `ARCHITECTURE.md`, `GLOSSARY.md`, `.mochiko/brainstorms/index.md`, `.mochiko/decisions/**`, `.mochiko/archive/**`. `primitive-edits.md`, `paths`-scoped to `plugins/mochiko/commands/**`, `skills/**`, `agents/**`.
- `.mochiko/` subdirs: `archive/`, `brainstorms/`, `decisions/` (21 ADRs), `memory/` (1 file), `strips/`, `transform/`.
- `.mochiko/memory/` holds exactly one file: `knowledge-management.md` — the project-pinned KM copy.

**F44. `.claude/agents/` and `.claude/skills/` DO NOT EXIST** (`ls` → No such file or directory for both). `.claude/` holds `rules/`, `settings.json`, `settings.local.json`, `worktrees/`. **There is no out-of-plugin agent or skill layer today** — every agent and skill mochiko uses on itself is in the shipped plugin.

**F45. Ten shipped files point into the non-shipping `.mochiko/strips/`:** `templates/agent-dispatch.md:9` · `templates/command-shape.md:12` · `templates/sized-end-stage-review.md:6` · `commands/specify.md:77` · `skills/validation-command-shape/SKILL.md:43` and `:174` · `skills/mochiko/SKILL.md:118` · `skills/authoring-commands/SKILL.md:82` and `:83` · `skills/grooming-operating-docs/SKILL.md:22`. For an installed plugin these resolve to nothing.

**F46. The submodule-removal note**, `CLAUDE.md:52-58`, verbatim opening: *"**The `human-in-loop` and `agent-skills-research` submodules were removed on 2026-07-21** so the plugin installs cleanly for other users (git otherwise tries to fetch them on install)."* Rest of the block gives the `git submodule add` restore commands. Cross-references in `BACKLOG.md:19` and `:381`.

**F47. Prior rulings on packaging / install / distribution — near-absent.** Grep `packag|install|distribut|marketplace|end-user|repo-internal|ship` over `DECISIONS.md`: **no row about plugin packaging, install cleanliness, distribution, or an ops/shipped split.** The nearest rows are `DECISIONS.md:20` (2026-08-01 primitive-edit ceremony — about *editing* shipped primitives, not separating them) and `:14` (validator snapshot isolation). Grep over `.mochiko/decisions/` (21 ADRs): **one** hit, `2026-07-31-architecture-doc-authored.md:15` — *"the plugin is the shipped artifact users install — its system view is the one a…"*. **No ADR and no DECISIONS.md row records the 2026-07-21 submodule removal** — it lives only as the CLAUDE.md note (F46) and the two BACKLOG mentions.

**F48. The KM module is end-user-facing by explicit design.** `templates/constitution-modules/knowledge-management.md:3-8`, verbatim: *"Attach when: the knowledge-management dimension (dimension 7) elicited adoption of the operating-docs layer. **Offered DEFAULT-ON in every mode — the user must actively decline.** Adopted as CORE + ELECTIVES…"* And `:15-21`: *"AUTHORING SOURCE ONLY: this template is the module's single authoring-time source. At scaffold, setup writes a PROJECT-PINNED copy of the Document-contracts + Landing-ritual + Invariants sections to `.mochiko/memory/knowledge-management.md`; command landing steps and the groom skill resolve against the project copy at runtime, never against this file."*

**F49. Who scaffolds it:** `commands/setup.md:173-176` — *"**KM landing:** knowledge-management adopted → scaffold it at G5 per `templates/constitution-modules/knowledge-management.md` … **project-pinned copy** at `.mochiko/memory/knowledge-management.md`, which every command resolves…"*. Module registered in `skills/authoring-constitution/SKILL.md:234`. All five other commands carry a KM-landing line keyed to the pinned copy's existence: `specify.md:106`, `plan.md:154`, `implement.md:161`, `brainstorm.md:127`, `setup.md:27`.

**F50. Mochiko is a consumer of its own KM module.** `.mochiko/memory/knowledge-management.md` exists in this repo (F43). So `grooming-operating-docs` grooming mochiko's `ROADMAP.md`/`BACKLOG.md` and grooming a customer project's are **the same code path against a different pinned copy** — not two products.

**F51.** `ROADMAP.md` carries **no** ops/shipped-separation item in Now, Next, or Later. Nearest standing bet, `ROADMAP.md:43`: *"**Hard-require agent teams, no fallback** (`Contested`, 2026-07-04) — revisit: **distribution beyond the author's machines**."* Same revisit trigger appears in `command-shape.md:232-233`.

**F52.** Session bookkeeping is current: `.mochiko/brainstorms/index.md:11-14` carries this session's entry, `verbosity-caveman-ops-separation`, When 2026-08-01, **Status: open**, artifacts `record.md` (canonical, in progress), with all three threads described.

---

## Thread 4 — Driver evidence: the `author-navigate` run output (mochiko-app dogfood)

Source: `github.com/humaninloop-dev/mochiko-app`, shallow clone at `…/scratchpad/mochiko-app`, HEAD `1f35a68` (Sat Aug 1 18:57:36 2026 +1000, *"Merge pull request #6 from humaninloop-dev/ci-actions-budget-cut"*). Path mapped: `.mochiko/specs/author-navigate/`. Byte/word/line counts exact (`wc`); token figures chars/4 `est.`, the repo's own convention.

**F53. Full inventory — 20 files (19 content + 1 stray).** Bytes · words · lines:

| Path (under `.mochiko/specs/author-navigate/`) | B | words | lines | class |
|---|---|---|---|---|
| `constraints-and-decisions.md` | 105,740 | 16,113 | 1,278 | deliverable |
| `contracts/api.yaml` | 79,266 | 7,937 | 1,559 | deliverable |
| `slices/S1/tasks.md` | 76,415 | 10,360 | 655 | deliverable |
| `requirements.md` | 76,267 | 11,864 | 1,084 | deliverable |
| `data-model.md` | 52,139 | 7,651 | 658 | deliverable |
| `slices/S1/architecture.md` | 47,311 | 6,765 | 635 | deliverable |
| `spec.md` | 42,807 | 6,484 | 384 | deliverable |
| `slices/S1/reports/cycle-4-verification.md` | 41,278 | 6,047 | 537 | report |
| `slices/S1/reports/cycle-1-verification.md` | 37,507 | 5,558 | 627 | report |
| `slices/S1/reports/cycle-2-report.md` | 32,165 | 4,741 | 473 | report |
| `nfrs.md` | 31,199 | 4,629 | 461 | deliverable |
| `slices/S1/reports/cycle-3-report.md` | 28,559 | 4,158 | 421 | report |
| `slices/S1/reports/cycle-3-verification.md` | 27,299 | 3,913 | 367 | report |
| `slices/S1/reports/cycle-2-verification.md` | 25,985 | 3,732 | 362 | report |
| `slices/S1/reports/cycle-4-report.md` | 25,886 | 3,733 | 355 | report |
| `slices/S1/task-mapping.md` | 29,750 | 4,531 | 160 | deliverable |
| `slices.md` | 24,063 | 3,727 | 222 | deliverable |
| `slices/S1/reports/cycle-1-report.md` | 22,222 | 3,319 | 326 | report |
| `slices/S1/plan.md` | 10,743 | 1,546 | 162 | deliverable |
| `slices/.DS_Store` | 6,148 | — | — | stray (macOS metadata, committed) |

**F54. Totals (excluding `.DS_Store`): 816,601 B · 116,808 words · 10,726 lines · ~204,150 tok est.** across 19 files.

**F55. Split by class.** **Deliverables — 11 files, 575,700 B (70.5%), 81,607 words, ~143,925 tok est.** **Reports — 8 files, 240,901 B (29.5%), 35,201 words, ~60,225 tok est.** Classification basis: `templates/artifact-format.md:4-6` enumerates the deliverable chain (`spec.md · requirements.md · constraints-and-decisions.md · nfrs.md · data-model.md · contracts/api.yaml · quickstart.md · plan.md · task-mapping.md · tasks.md · slices.md`) — 10 of the 11 files here match it by name. `slices/S1/architecture.md` is the eleventh: **not named in artifact-format's list**, which stamps v1 at 2026-07-24, predating the architecture primitive (built 2026-07-30, v0.32.0 per `ROADMAP.md:22`). The 8 files under `reports/` are reports by their own `report:` frontmatter field (F57).

**F56. What is absent from the tree:** no `quickstart.md`; no `final-validation` report (4 cycle+verification pairs, no closing validation file); no S2/S3/S4 directories. This is **slice 1 of 4** (`slices.md:11-14` lists S1–S4) — so 816,601 B is one foundation slice of a four-slice feature, not the feature's total.

### Envelope conformance — reports

**F57. The `report-format.md` envelope landed, exactly.** All 8 open with YAML frontmatter whose `report:` value is from the sanctioned enum at `report-format.md:23` — `cycle` ×4, `verification` ×4 — plus `feature: author-navigate`, `slice: S1`, and `cycle:` + `attempt:` (the enum line's own carve-out: *"cycle reports carry `cycle:` + `attempt:` instead"*). Payload fields are richly structured YAML: `quality_gates:` maps with `{status, command, exit}`, `test_tasks:` inline-map lists, `attempt_history:`, `files_created:`, `failed_tasks:` with `{id, why}`. Example, `cycle-1-verification.md:8-9`: `attempt_history:` / `- {attempt: 1, status: fail, blocking: [VF-1, VF-3], evidence: "CI run 30675959694 — Coverage ratchet exit 127"}`.

**F58. The conditional-prose rule did not hold. All 8 reports carry `status: pass`; all 8 carry prose bodies.** `report-format.md:42-43`, verbatim: *"A clean/passing report is frontmatter-only."* Measured split (frontmatter bytes vs body bytes):

| Report | frontmatter ends | fm B | body B | body % |
|---|---|---|---|---|
| cycle-1-report | L76 | 4,892 | 17,330 | 77% |
| cycle-1-verification | L49 | 4,230 | 33,277 | 88% |
| cycle-2-report | L91 | 6,845 | 25,320 | 78% |
| cycle-2-verification | L48 | 3,711 | 22,274 | 85% |
| cycle-3-report | L96 | 7,969 | 20,590 | 72% |
| cycle-3-verification | L57 | 5,273 | 22,026 | 80% |
| cycle-4-report | L79 | 7,669 | 18,217 | 70% |
| cycle-4-verification | L68 | 7,742 | 33,536 | 81% |

Aggregate: **48,331 B frontmatter, 192,570 B prose body — 79.9% of all report bytes are prose.** Same rule in the two payload homes: `executing-tdd-cycle/references/CYCLE-REPORT-FORMAT.md:53-54` — *"a clean passing cycle is frontmatter-only"*; `testing-end-user/references/REPORT-TEMPLATES.md:73-75` — *"A passing report carries **no prose** — no evidence tables, no narration; the captured evidence stays in logs/scratch, pointed to by `evidence:` fields."* The verification reports carry both evidence tables and narration.

**F59. Cuts the other way — the reports have a textual defence.** Every cycle report's `failed_tasks:` is non-empty: each names one task with `why: "verifier-owned TEST: gate — deliberately not executed by the producer"` (`cycle-1-report.md:11-12`, `cycle-2-report.md:11-12`, `cycle-3-report.md:11-12`, `cycle-4-report.md:11-12`). `CYCLE-REPORT-FORMAT.md:69` makes the failure narrative *"mandatory when `status` is `fail` or `blocked`, **or any task failed**"*. By that clause a narrative is owed even at `status: pass`. Reporting the tension; not adjudicating it.

**F60. Where the prose sits differs sharply by report type.** Cycle reports use **2 H2 sections** each (c2/c3/c4) — `## Notes of note` (sanctioned by `CYCLE-REPORT-FORMAT.md:57`) and `## Blockers and flags for the checkpoint` — but at 5–14 KB apiece: c2 = 11,309 + 13,822 B · c3 = 13,323 + 7,136 B · c4 = 13,313 + 4,764 B. Verification reports fan into **7–15 H2 sections** each, most outside `report-format.md`'s conditional-prose set (failure narrative · notes of note · null-exit reasoning): e.g. cycle-1-verification carries `## Failures` 7,893 B · `## Judgment-call grades` 4,183 B · `## Deviation review` 3,397 B · `## Design review of the restructure` 2,261 B · `## Evidence classification` 947 B · two separate `## Checkpoint recommendation` blocks. cycle-4-verification carries `## Findings` 9,649 B.

**F61. No `findings:` YAML key exists in any of the 8** (grep `^findings:` → empty). Findings are prose subsections: `### VF-10 (important, non-blocking) — T4.20's Setup is not load-bearing for the result` (`cycle-4-verification.md:349`), with fenced command output beneath. Stated without prejudice: `report-format.md:53` scopes the structured findings schema to *"(reviewer reports)"*, and `verification` is a distinct enum member from `review` — so this is the observed form, not automatically a deviation.

### Envelope conformance — deliverables

**F62. The `artifact-format.md` ID-index rule landed, and four files label it in the file.** Rule 2 (`artifact-format.md:26-29`) requires each ID-bearing artifact to open with an ID index. Present and explicitly named: `requirements.md:8` — `## Traceability Summary  *(the ID index)*` · `constraints-and-decisions.md:12` — `### Constraint Summary  *(the ID index)*` · `data-model.md:9` — `## Data Sensitivity Summary  *(the coverage index)*` · `nfrs.md:38` (ID/Category/Target/Source table). Unlabelled but present: `spec.md:265` (Story / Requirements / Verified by) · `slices.md:9` (`## Slice order`).

**F63. Reference-by-ID (rule 1) is in heavy use — measured, not asserted.** Mechanical scan of every unbroken prose block ≥5 lines across the tree, counting artifact-ID tokens (`FR-`/`TR-`/`NFR-`/`SC-`/`C-`/`D-`/`US-`/`GI-`/`IP-`/`DS-`/`VF-`/`T<n>.<n>`): the four longest blocks in the tree are all in `constraints-and-decisions.md` and all are ID-dense — the longest, `constraints-and-decisions.md:940`, runs **22 lines / 320 words** and carries IDs at nearly every clause (`TR-025 c5`, `D-007`, `A-ARCH-2`, `100ms`). Density, not restatement.

**F64. Rule 4's size guidance is exceeded, on a rule that states it is a default.** `artifact-format.md:34-37`: *"Overview / context / rationale prose defaults to ≤ 3 lines … These are defaults, not caps on substance."* Observed: `spec.md`'s Overview is 9 lines; the tree's longest unbroken prose block is 22 lines (F63).

### The mechanically selected passage

**F65.** Two mechanical rankings were run over every unbroken prose block (excluding headings, tables, list items, blockquotes, code fences): **(a)** longest by word count → `constraints-and-decisions.md:940`, 22 lines / 320 words, but ID-dense; **(b)** highest words-per-ID among blocks ≥5 lines → the top four all score zero IDs. Of those four, the one that is also a **file-head preamble standing before any substantive fact** — the suggested criterion — is `spec.md:11-19`: the Overview, **9 lines / 121 words / 0 artifact IDs**, at the head of the run's human sign-off surface. Verbatim:

```
The author and navigate pillars of mochiko-app: a purpose-built surface for reading, editing,
scaffolding, and renaming mochiko's operational artifacts across all of an adopter's projects plus
one user-scope area. Editable are the **adopter pile** — user- and project-scope `.claude/` skills,
agents, commands, plus a project's `.mochiko/` artifacts, `.claude/rules/` files, and root operating
docs; read-only are the **shipped pile**, the `CLAUDE.md` governance region, the setup-owned
`.mochiko/memory/` files, and run kill-switch markers. Navigable — as areas with a tree and
quick-open — are each registered project, the user-scope area, and the shipped-pile area; markdown
is rendered, standalone YAML shown as plain text. Observe, control, deletion, and the planned mochiko↔app integration channel (record
D3) are out of scope; every surface satisfies the **artifact-binding test**.
```

Selection was mechanical (zero-ID + file-head position + longest such block). No judgment offered on whether the length is warranted.

### Audience: human-read surfaces vs machine hand-offs

**F66. Files carrying an explicit human-audience marker in their own text:**
- `spec.md:5` — `> Status: accepted (user, 2026-07-31)`. A user acceptance gate is recorded on the artifact.
- `slices/S1/architecture.md:4-11` — *"**Sign-off surface for the shape**; entity detail and IPC schemas are drawn downstream against the approved target"*, then *"**G3 record (2026-07-31): presented un-rendered.** Attended terminal session with no side-panel, published-artifact, or IDE render surface, and no local mermaid renderer (npm registry unreachable from the sandbox); diagram source + component register presented per the gate's…"* — an explicit human gate surface that also records how the gate degraded.
- `slices.md` — carries the spec stamp (`:3`) and three inline user rulings (`:43`, `:66`, `:185`, each marked `user-ruled`).
- `nfrs.md:28` — `**Ruled acceptance — no dogfood cut before S2 (user ruling, round 3).**`

**F67. Files with no stated audience marker** — their headers state provenance and accumulation rules instead: `requirements.md:3-6` (*"Shared artifact — it **accumulates** across slices per the `slices.md` Graduation contract"*), same pattern in `constraints-and-decisions.md:3-6`, `nfrs.md:3-6`, `data-model.md:3-7`; `task-mapping.md:3-4` (*"Source of truth for story→cycle decisions and slice rationale"*); `tasks.md:3-7` (generated-from list); `contracts/api.yaml`; `plan.md` (no audience line; `:38` carries a `**User-ruled**` decision-table cell). Absence of a marker is not absence of a human reader — `artifact-format.md:14-16` states every deliverable is read *"by the human at its acceptance gate, by every mandated cold-reviewer read, and by every downstream producer."*

**F68. All 8 reports read as machine hand-offs by their own markers** — `report:` frontmatter (F57) plus a closing `## Checkpoint recommendation` addressed to the lead, and lead-directed asides: `cycle-1-verification.md:413` — *"**TARGETED RETRY**, scoped to two producer-actionable items. *(Accepted by the lead 2026-08-01 as…"*; `cycle-4-report.md:61` — `- BACKLOG.md · DECISIONS.md               # the lead's records, carried (opening item 1)`; `cycle-1-verification.md:254` classifies its own content by grader — *"**Subjective** (my judgment, gradeable and contestable) | The two judgment-call grades · deviation materiality assessments · the checkpoint recommendation and its retry scope"*. Consistent with **F14**: the machinery routes no report file to the user.

**F69.** `slices/.DS_Store` (6,148 B) is committed inside the run's artifact directory — macOS metadata, no mochiko provenance.

---

## Cuts both ways / notable absences

1. **The token-epic boundary is real but unruled.** Every one of D1–D6 targets loads, report files, artifact files, or reviewer counts (F13), and the exclusion list never mentions conversation prose (F16) — but no sentence in the record scopes user-facing prose out. Anyone citing the BACKLOG parenthetical as a ruling is citing a summary.
2. **Two of the three verbosity layers are already ruled and shipped.** Reports went machine-first with clean reports frontmatter-only (F9, v1 2026-07-23); deliverables went dense-by-construction with per-section size guidance (F11, v1 2026-07-24). If "too verbose" points at reports or artifacts, it points at rulings already built — which makes the *conversation* layer (F4, F7) the untouched one.
3. **`artifact-format.md` rule 8 pushes the other way.** *"Brevity is never itself a finding"* is an explicit guard protecting density from reviewers — a shipped counterweight to any across-the-board terseness dial, at least on the artifact layer.
4. **Nothing caveman-shaped ships.** Zero `caveman` hits in `plugins/` (F28). The single shipped user-facing-prose rule is a three-term **vocabulary ban** (F4) — it says nothing about length, register, articles, or hedging, which is the whole of what caveman governs.
5. **Caveman's own text exempts most artifacts.** *"Written artifacts are exempt — code, comments, commit messages, PR bodies, and every file in the repo are written normally"* (F25) — so as written, caveman would touch only the conversation layer, i.e. exactly the layer the token epic did not touch, and none of the report/deliverable layers.
6. **The intensity-modes item shares caveman's level vocabulary but not its scope.** `lite/full/ultra/off` (F27) vs `lite/full/ultra` (F26). The BACKLOG item never says what the dial modulates, and its provenance is recorded unrecoverable — a vocabulary match, not evidence they are the same decision.
7. **No infrastructure exists for an out-of-plugin ops primitive.** `.claude/agents/` and `.claude/skills/` do not exist (F44). The only ops/shipped separation ever ruled moved **logs** (`.mochiko/strips/`, F41), never a command, agent, or skill.
8. **But that ruling's stated reason generalizes cleanly:** *"the plugin directory is the shipped artifact, and anything inside it distributes with the plugin whether loaded or not"* (F41) — a size/distribution argument, not a loading-cost one, so it applies to `command-architect` + `authoring-commands` + `validation-command-shape` on its own terms.
9. **A leak already exists in the other direction:** 10 shipped files reference `.mochiko/strips/` (F45), which does not ship. Installed users get dangling provenance pointers today.
10. **"KM = repo ops" is falsified.** The KM module is default-on at dimension 7 in every setup mode, the user must actively decline, and mochiko itself runs on a pinned copy (F48–F50). `grooming-operating-docs` is not a repo-ops primitive by any file in the repo.
11. **The router already discloses the ops/product split in prose, without acting on it** — heading *"Framework maintenance … reached when authoring or auditing mochiko's own commands"* (F39). End users see the two skills indexed and labelled; nothing hides or gates them.
12. **Packaging has one precedent and no ruling.** The 2026-07-21 submodule removal was done *"so the plugin installs cleanly for other users"* (F46) — but it never landed a `DECISIONS.md` row or an ADR (F47). It is the only install-cleanliness precedent, and by the repo's own subtractive-landing ritual it is under-recorded.
13. **Version surfaces disagree by design and are worth stating once:** plugin `0.43.0` vs marketplace `0.10.0` (F32).
14. **The dense forms half-landed, and the split is clean.** Structural conformance is complete — every report carries the sanctioned `report:` envelope with rich structured YAML payloads (F57), every ID-bearing deliverable carries its ID index (F62), reference-by-ID is in dense use (F63). The rule that did not hold is the **conditional-prose** one: 8 of 8 passing reports carry prose, 79.9% of report bytes (F58). D3's saving was specified as *dropping* the prose scaffold of passing reports; the frontmatter it specified was added, the prose it targeted stayed.
15. **The reports are the smaller half.** 240,901 B of 816,601 B (29.5%). Deliverables carry 70.5% (F55) — and per `artifact-format.md:14-16` those are the files re-paid at ~10 model reads per feature, versus reports' one lead read.
16. **The verbosity concentrates in files whose own envelope permits it.** The four longest prose blocks in the tree are all in `constraints-and-decisions.md` (F63), governed by `artifact-format.md`, whose rule 8 states *"Brevity is never itself a finding"* (F11) and whose rule 4 calls its size guidance a default, not a cap (F64).
17. **F59 is a genuine textual out for the reports.** Every cycle report has a non-empty `failed_tasks:` by design (the verifier-owned gate the producer must not run), and `CYCLE-REPORT-FORMAT.md:69` makes the narrative mandatory when *"any task failed"* — so the producer following the shipped text arrives at prose even on a passing cycle. That is a format-text interaction, not obviously an author's departure.
18. **This is one slice of four (F56).** Any per-feature extrapolation from 816,601 B is an extrapolation from S1 alone, and S1 is the foundation slice — the one that establishes shared artifacts (`requirements.md`, `constraints-and-decisions.md`, `nfrs.md`, `data-model.md` all say they *"accumulate across slices"*), so S2–S4 extend those files rather than duplicating them.
19. **`architecture.md` is 47,311 B and is governed by no envelope named in `artifact-format.md`** (F55) — the primitive postdates the v1 stamp. It is also one of the two files with an explicit human sign-off marker (F66).

---

## Fact map — second delivery (F70–F74, carrier verification)

*(fact-checker authored, verbatim — settling D2's pending-verification flag)*

**F70. All three D2 carriers exist under the names the record uses. Verified.**
- `plugins/mochiko/templates/command-shape.md:196-198` — the paragraph the record proposes to extend is titled *"**The conversation is the production surface.**"* (full text at F4). It sits in Layer 1, obligated-read by all six commands.
- `plugins/mochiko/templates/report-format.md` — exists, 74 lines, `**Format version:** v1 (2026-07-23 — workflow-token-reduction wave 1)`.
- `plugins/mochiko/templates/artifact-format.md` — exists, 69 lines, `**Format version:** v1 (2026-07-24 — workflow-token-reduction wave 2)`.

**F71. The reference graphs the three carriers already own** (grep over `plugins/mochiko/`), which is what "bound by reference at its three carriers" would inherit:
- `report-format` is referenced by **12 files**: 7 report templates (advocate · analyst · feasibility · slicer · sysarchitect · taskarchitect · techanalyst) · `command-shape.md` · `artifact-format.md` · the router · and **two skill references** — `executing-tdd-cycle/references/CYCLE-REPORT-FORMAT.md`, `testing-end-user/references/REPORT-TEMPLATES.md`.
- `artifact-format` is referenced by **22 files**: 5 artifact templates (spec · plan · tasks · slices · codebase-analysis) · 11 skills (authoring-requirements, authoring-user-stories, authoring-technical-requirements, authoring-slices, analysis-codebase, patterns-api-contracts, patterns-entity-modeling, patterns-system-design, patterns-vertical-tdd, review-plan-artifacts, review-slices, review-specifications, review-task-artifacts) · 3 skill references · the router.

**F72. Gap worth ruling before build — the reports' real authoring home is one layer below the named carrier.** The driver run's report prose (F58: 8/8 passing reports carry prose, 79.9% of report bytes) was authored against the two *payload* homes, not against `report-format.md` directly: `executing-tdd-cycle/references/CYCLE-REPORT-FORMAT.md` (which restates *"a clean passing cycle is frontmatter-only"* at `:53-54` and defines `## Notes of note` at `:57`) and `testing-end-user/references/REPORT-TEMPLATES.md` (*"A passing report carries **no prose**"* at `:73-75`). A style binding placed only at `report-format.md` reaches the producer by one more hop than the rule that already failed to hold there. Reporting the topology; the ruling is yours.

**F73. The switch's proposed home has no section that fits it today.** `templates/governance-surfaces-template.md:29-59` gives the region's full shape between `<!-- mochiko:governance:begin -->` and `<!-- mochiko:governance:end -->`: a `**Ratified:**` stamp line, then `### Principles` · `### Technology stack` · `### Quality gates` · `### Governance operations`. `### Governance operations` (`:53-59`) currently carries five line types — ledger pointer, amend-via-setup, the path-scoped-rules injection note, the KM operating-docs pointer, release gates — every one a pointer or a governance mechanic, none a response-style setting. So a default-on style line lands either as a sixth line in `Governance operations` or as a new section; neither exists yet. Both are additions to a setup-owned, regenerated-in-place region.

**F74. Two record claims already verified in the map, restated so D1/D2 are self-contained:** the `lite/full/ultra` level machinery is `CLAUDE.md:33-34` (F26, default `full`); and the operator spec's artifact exemption the record's Contested note flags is `CLAUDE.md:30-31` — *"**Written artifacts are exempt** — code, comments, commit messages, PR bodies, and every file in the repo are written normally"* (F25). The record's reading is correct: ruling artifacts at `full` does go beyond the operator spec, which exempts them entirely rather than assigning them a level.

## Decisions

### D1 — Diagnosis + per-surface caveman levels · `Confident` (artifacts-at-full: `Contested`)

**Statement:** The verbosity disease is **prose bloat** (too many words per fact), not fact
count — so the fix is substance-preserving compression, not content cuts, and it does not
fight the record-is-the-audit-trail doctrine. Caveman levels assign per surface: **chat
`full` · machine reports `ultra` · human-read artifacts `full`** — with the spec's ambiguity
guardrail riding (drop caveman wherever compression makes technical meaning ambiguous).

**Rationale:** Driver = the `author-navigate` run output (mochiko-app); user reports all
reading jobs affected and machine reports verbose too. Diagnosis A (words-per-fact) ruled by
user. Levels machinery already exists in the operator caveman spec (`lite/full/ultra`).

**Contested element:** lead recommended artifacts at `lite` (precision for cold builders in
FR/contract classes); user ruled `full` — "full is explanable enough" — after one challenge.
Note: this goes beyond the operator spec, which exempts written artifacts entirely.

### D2 — Adoption model: default-on, switch in the project's CLAUDE.md · `Confident`

**Statement:** The style ships **default-on across all surfaces** with a documented off
switch. The style text is single-sourced in one shipped home (small template), bound by
reference at its three carriers: `command-shape.md` Layer 1 (chat `full` — extends the
existing "conversation is the production surface" paragraph) · `templates/report-format.md`
(reports `ultra`) · `templates/artifact-format.md` + per-artifact templates (artifacts
`full`). The switch's persistent home is the **user project's `CLAUDE.md` governance region**
(setup writes the default-on line; user flips it there); the in-session phrases ("stop
caveman" / "normal mode") are honored on top, matching the operator spec. Pre-setup runs get
pure default-on from the shipped doctrine.

**Rationale:** User ruled A (default-on) over setup-elicitation and the split model — one
behavior, zero setup-agenda cost. Switch home A+B adopted on lead recommendation: the
governance region is the one project-local surface mochiko already owns that loads every
session; settings/env rejected as a surface mochiko doesn't own.

**Carrier verification (fact map):** all three carriers confirmed real — the Layer-1
conversation paragraph (F4/F70), `report-format.md` v1 (F8–F9), `artifact-format.md` v1
(F10–F11). The governance region exists with four sections and **no prose/style section**
(F29/F73) — the switch line is an addition (sixth `Governance operations` line or a new
section), a build obligation, not an edit to an existing shape. No platform output-style
mechanism is documented anywhere in the repo (F31) — D2 relies on none. Note F25/cut 5: the
operator caveman spec as written touches only the conversation layer; D1's extension to
reports and artifacts is deliberate new scope, not a port.

**Lead fold on F72 (formulation, flagged):** the reports binding reaches the **payload
homes too** — `executing-tdd-cycle/references/CYCLE-REPORT-FORMAT.md` and
`testing-end-user/references/REPORT-TEMPLATES.md` — not `report-format.md` alone: the driver
run's prose was authored against those homes one hop below the named carrier, where the
stricter frontmatter-only rule already failed to reach (F72). Same intent ("bind where
authored"), wider carrier set; consistent with the all-consumer edit guard.

### D3 — Plain-language principle folds into the style home; sweep stays open · `Confident`

**Statement:** The minted style home carries **both** rules: terse (caveman levels per D1)
**and plain-English-for-end-users** — including the growth mechanism for the user-facing
vocabulary ban (today the three-term list at `command-shape.md:196-198`, F4). The
plain-language **sweep work itself** (term hunting, "Layer -2"-class leaks) stays its own
BACKLOG item — per the sweep finding (F3), the leak is runtime lead prose, not file content,
so the fix is the ban's term list growing in the style home, exercised at runs.

**Rationale:** One style home, one landing; the home is being minted anyway (user ruled A,
Q8). Terse without plain risks denser jargon — the two rules are complements, not
substitutes. *(Lead's reading of the A ruling: the rule's home folds here, the sweep item
survives separately — flagged for user correction if the intent was a full fold.)*

### D4 — Reports fix: format repair + `ultra` + enforcement teeth · `Confident`

**Statement:** Three parts, one landing:
1. **Format repair** — close the F59 clause (a verifier-owned, deliberately-skipped task
   does not count as "failed" for the narrative-mandatory rule at
   `CYCLE-REPORT-FORMAT.md:69`); bound verification-report prose sections to the sanctioned
   set (driver run fanned into 7–15 H2 sections, most outside it — F60).
2. **`ultra` binding** — governs whatever prose legitimately remains (failure narratives,
   notes-of-note), reaching the payload homes per the F72 fold, not `report-format.md` alone.
3. **Teeth** — a prose-bearing clean report becomes a **findable defect**: the lead bounces
   it on sight and the relevant validator check names it. The report layer gains the missing
   twin of artifact rule 8 — verbosity *is* a finding here.

**Rationale:** The frontmatter-only rule was already stricter than `ultra` and still broke —
8/8 passing reports carried prose, 79.9% of report bytes (F58), partly forced by the format
text itself (F59). Unenforced format rules demonstrably don't land (same story as the kinako
dense-forms miss). A style rule added without repair + teeth would be a second wish on the
same shelf. User adopted the recommendation (Q9, option C).

**Boundary vs D5:** D4's "teeth" are a *structural binary check* — prose present on a clean
report, mechanically detectable — not a prose-quality review dimension. D5 rules out
verbosity *grading*; D4's check survives that ruling because it grades presence, not quality.

### D5 — Artifacts: no verbosity grading; enforcement by injection · `Confident`

**Statement:** Deliverables get **no verbosity grading** — `artifact-format.md` rule 8
("brevity is never itself a finding") stands, and no reviewer dimension for bloat is added.
Enforcement of artifacts-`full` (D1) is **injection-side** — the style rides the context
surfaces the producer already loads at authoring time:
1. **setup command** writes the style (default-on line + levels per surface) into the user
   project's `CLAUDE.md` governance region (D2's switch home; new section/line per F73);
2. **`paths`-scoped `.claude/rules/` style file** over artifact-producing paths (e.g.
   `.mochiko/specs/**`) delivers the artifact rule at touch time — precedent: OO-D3's
   SLO rules file over the same glob (audit-cleared pattern);
3. **shipped templates** carry the style by reference (D2's carriers + F72's payload homes).

**Rationale:** User-ruled with reasoning: grading verbosity *spends* the tokens caveman
exists to save — a big part of caveman is output-token reduction; a review dimension on
style invites mechanics-thrash rounds (trace-check watch precedent). Injection beats
inspection on this layer. **Dependency noted:** `paths`-scoped rules delivery has an open
empirical watch (BACKLOG "Fresh-session rules-loading test") — the mechanism this leans on
is precedented but not fully probed; watch rides the build.

### D6 — Framework-maintenance trio moves repo-side; ops leakage is a defect class · `Confident` (leakage boundary ruled at D7)

**Statement:** Two parts:
1. **The trio moves out of the plugin** — `command-architect` (agent), `authoring-commands`
   and `validation-command-shape` (skills) relocate to this repo's `.claude/agents/` +
   `.claude/skills/` (native surfaces, to be created — F44). They never ship; they keep
   working in-repo. Namespace change (`mochiko:` prefix lost) ripples through references —
   every touched shipped file goes through the primitive-edit ceremony; the router's
   "Framework maintenance" section comes out with them.
2. **Ops leakage into shipped content is ruled a defect class** (user-asserted, map-backed:
   F45's 10 dangling pointers into non-shipping `.mochiko/strips/`; ops provenance blocks
   and session-slug references in shipped templates). A **leakage sweep** is a build item:
   enumerate per class, disposition per line under the primitive-edit ceremony — this
   record supplies the class-level ruling; per-line removal still takes its strip entry +
   independent audit. Class boundary pending Q12.

**Rationale:** F41's ruled rationale generalizes verbatim — "the plugin directory is the
shipped artifact, and anything inside it distributes with the plugin whether loaded or not."
The trio's consumers are exclusively mochiko's own command authoring (F38–F39). Shipped
skill descriptions cost every end-user session context for skills they can never usefully
fire. A (repo-side) ruled over B (status quo) and C (`mochiko-dev` second plugin); C
re-opens if outside contributors materialize.

### D7 — Leakage boundary: full scrub, changelog-worthy detail preserved repo-side · `Contested`

**Statement:** **Full scrub** — both leakage classes leave the shipped tree: the one-line
provenance pointers *and* the fat version-history blocks. Hard constraint from the user:
**no changelog-worthy detail is lost** — every removed block relocates **verbatim** to its
repo-side home (the per-primitive `.mochiko/strips/` note, the existing provenance
convention), never deleted. Removal of protected/`DECISIONS`-traceable content rides this
record as the class-wide supersession ruling; each file's edit still takes its strip entry +
independent audit per the primitive-edit ceremony.

**Rationale:** User chose full scrub over the lead's B (pointers stay) — deliberate,
maintained after the pointer steelman; hence `Contested` on the pointer half. Mitigation
that makes it safe: the ceremony's walk-path survives via `.claude/rules/mochiko/`
`primitive-edits.md` (`paths`-scoped, fires on every `plugins/mochiko/**` touch and names
the strip-note home) — the in-file pointer was redundant for repo maintainers. Bonus: the
version blocks load in every command run (command-shape's v7 block ~2.5 KB, obligated-read),
so the scrub also shrinks the always-read floor — a token-epic win.

### D8 — Token-epic seam ruled explicit · `Confident`

**Statement:** The boundary the epic's record never stated (F16) is ruled here: **the
token-reduction epic owns context loads and machinery tokens; this session's style work owns
prose style on all three output layers** (conversation · reports · deliverables). D4's
format repair is explicitly **finishing epic D3's intent** — the conditional-prose rule D3
specified and the driver run shows never landed (F58, cut 14). One seam, no overlap; neither
work item re-opens the other.

**Rationale:** Without this ruling the BACKLOG parenthetical ("the epic targets inter-agent/
report tokens, not user-facing prose") stays a summary citing nothing. User confirmed in the
Q13 batch.

### D9 — "Intensity modes" backlog item superseded by D1+D2 · `Confident`

**Statement:** The open design decision **Intensity modes** (BACKLOG, 2026-06-27, provenance
unrecoverable) is superseded: D1 supplies the per-surface level pattern it deferred for
("defer until the pattern is clear from real runs" — the driver run is that evidence), and
D2 supplies the dial's home and off switch. Honest note: the item never stated what the dial
modulates — vocabulary match, not provable identity (F27, cut 6) — so this is a supersession
by covering, not by exact answer. Closes to the trail at landing.

**Rationale:** User confirmed in the Q13 batch.

### D10 — Operator caveman spec unchanged · `Confident`

**Statement:** This repo's own `CLAUDE.md` caveman spec keeps its written-artifacts
exemption (F25). D1's artifacts-`full` governs pipeline deliverables in mochiko-run
projects; if the pipeline ever produces such artifacts in this repo, the shipped injection
(D5) styles them by mechanism. The operator's hand-authored governance/audit layer (records,
decisions, strips) stays normal prose — exact wording is load-bearing there.

**Rationale:** Lead-recommended, user confirmed in the Q13 batch. Consistency-by-mechanism
beats consistency-by-spec-edit; operator artifacts are not the token sink.

## Review

*(sizing ruling on the Status line · survivor dispositions and verify outcome land here)*

### Fact-map errata (landed during review; checker-authored, verbatim)

**F43 ERRATUM (settled, 2026-08-01) — the record-integrity reviewer is correct. My error, not a repo defect.**

F43 stated that `.claude/rules/mochiko/primitive-edits.md` is `paths`-scoped to three globs. It carries **four**. The file's frontmatter in full, verbatim, lines 1–7:

```
---
paths:
  - "plugins/mochiko/commands/**"
  - "plugins/mochiko/skills/**"
  - "plugins/mochiko/agents/**"
  - "plugins/mochiko/templates/**"
---
```

**Cause:** I read the frontmatter with `head -5`, which terminated at line 5 — one line before `"plugins/mochiko/templates/**"`. The fourth glob was never in view. A truncated read reported as a complete enumeration.

**Corrected F43 clause:** `primitive-edits.md`, `paths`-scoped to `plugins/mochiko/commands/**`, `skills/**`, `agents/**`, **and `templates/**`**.

**The correction has a live consequence for this session's D2, in the session's favour.** All three D2 carriers — `templates/command-shape.md`, `templates/report-format.md`, `templates/artifact-format.md` — sit under `plugins/mochiko/templates/**`. Under the three-glob reading none of them was covered by the touch-time reminder; under the true four-glob scope **all three are**. So the primitive-edit ceremony (version-stamped entry in `.mochiko/strips/<primitive>.md` + the independent author ≠ grader audit) binds on every carrier the D2 build would touch, and the reminder fires at edit time on each. That is a build obligation the session should carry forward, and it was invisible under my erroneous F43.

**No other surface carries the wrong count — checked, not assumed.** The ruling's ADR states the scope correctly and completely: `.mochiko/decisions/2026-08-01-primitive-edit-ceremony-codified.md:23-25` — *"a new `paths`-scoped rule over `plugins/mochiko/{commands,skills,agents,templates}/**` that fires the checklist at edit time"*. The `DECISIONS.md:20` row names the file without enumerating globs, so it cannot disagree. The error existed only in my F43 line.

**The operating-docs.md half of F43 needs no re-check, and I verified that rather than asserting it.** That half was read with a range command covering the whole frontmatter, and re-reading it now confirms all eight globs exactly as reported, lines 3–10 with the closing `---` at line 11: `DECISIONS.md` · `ROADMAP.md` · `BACKLOG.md` · `ARCHITECTURE.md` · `GLOSSARY.md` · `.mochiko/brainstorms/index.md` · `.mochiko/decisions/**` · `.mochiko/archive/**`. Nothing after the last one.

**Self-audit for the same failure mode across the rest of the map.** The truncation risk applies wherever I reported an enumeration from a fixed-length head/sed window rather than a full read or a count. Re-checked and clean: F33 (6 commands), F34 (10 agents), F35 (30 skills), F36 (20 templates), F42 (`plugins/mochiko/strips/` absent), F44 (`.claude/agents`/`.claude/skills` absent), F53 (the 20-file dogfood inventory) — all came from `find`/`ls` over the full tree or from `wc`, not from windowed reads. F32's plugin.json was `cat`'d whole. The frontmatter enumerations in F43 were the only fixed-window reads I published as complete lists, and both are now settled above.
