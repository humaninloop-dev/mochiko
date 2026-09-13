# Hook-enforced artifact schema and location — decision record

**Status:** accepted 2026-09-13 (user: "accept") · amended post-acceptance 2026-09-13 (wave-0 results: D1b/D1c/D3/D8/OQ3/OQ4/F10 — user rulings R4/R5/R6) · **Opened:** 2026-09-13 · **Lead:** session lead (inline questioning via
`mochiko:analysis-iterative`) · **Review:** round 1 folded (cold, blind-map two-message dispatch); verify round 1 NOT CLEAN → 14 repaired; delta-check 3 residuals → lead-repaired, closed

## Topic

Driver ask (user, 2026-09-13): "get context from claude hooks documentation, pretool and
posttool use. I want to ensure the schema of the markdown files and the location to be enforced
by cli and hooks. The situation currently what happens is, claude sessions scans the repo and
markdown in the repos and mochiko plans to use that files as schema rather than what is in the
instructions."

**Goal line:** decide whether, where, and how `mochiko-cli` plus Claude Code hooks enforce the
schema and the location of the markdown artifacts mochiko commands produce, so that a session's
repo scan can no longer substitute an existing markdown file for the CLI-delivered rules and
templates — leaving one hardened decision record.

## Ground facts (F)

- **F1 — Shipped hooks today (plugin v0.108.0).** `plugins/mochiko/hooks/hooks.json` registers
  three: `SessionStart` (presence report, never blocking), `UserPromptExpansion` matched
  `^mochiko:` and `PreToolUse` matched `Skill` (both `dependency-halt.sh`: block on binary
  absence or grammar skew only; otherwise one-line presence context). The script's own header
  states the gate is "on dependency absence only, never on behavior or judgment (GI-019)".
- **F2 — The standing ruling this session reopens from zero.** `cli-schema-delivery` D7
  (accepted 2026-09-03): "Behavior-gating hooks are declined: a `PreToolUse` deny on `Agent`
  spawn until rules are rendered, or any hook that judges a seat's work, would cross the bright
  line's judgment and sequencing clauses […]; **if ever wanted it takes its own ruling from
  zero.**" D7(c) admits a maintainer-side-only `PostToolUse` advisory check on the migration
  paths, never shipped. `producer-plan-enforcement` D1 (2026-09-03) likewise ruled "detection
  plus review, no hook gate, no worktree".
- **F3 — The bright line (GI-019).** Kernel-class tooling "never gates pipeline progress, never
  dispatches or sequences agents, and never holds judgment that skills own"; "advisory post-hoc
  checkers used as optional exit-code signals are not kernel-class".
- **F4 — CLI surface today.** `mochiko-cli rules <primitive>` (per-section renders), `mochiko-cli
  template <name> [--check]` (producer or checklist view) for eight artifact templates:
  `architecture-store · codebase-analysis · feature-entry · features-index · governance-intent ·
  governance-surfaces · spec · tasks`; `doc`, `migrate`, `views`, `genesis`. No `check <file>`
  or `validate <path>` subcommand exists; `--check` renders a checklist for a model to apply, it
  does not read an artifact.
- **F5 — Open backlog threads this session touches.** *Prose vs. gate allocation* (2026-06-27,
  Open design decisions): "which behaviors earn graded anti-rationalization prose vs a hard
  `PreToolUse` hook? Kernel excluded; allocate between prose and hook." *Agents-as-schema
  channel probe* (`primitive-eval-harness-v2` D4 door): does a plugin hook fire for `--agent`,
  `Agent` subagent, and teammate, with injected text landing before the first turn — three legs.
- **F6 — Agent bodies are not `!`-preprocessed** (`primitive-eval-harness-v2` F12, probe
  2026-09-05, CLI 2.1.258): commands and skills get shell expansion; agents do not. Any
  CLI-served content reaching a persona needs a hook channel.
- **F7 — Hooks platform facts (`cli-schema-delivery` F9, source-verified 2026-09-03):** hooks
  are fail-open (a hook that cannot start or times out renders no decision and the action
  proceeds); hook input carries `session_id · transcript_path · cwd · permission_mode ·
  hook_event_name · agent_id · agent_type`; hooks ship to every consuming project. Fresh
  PreToolUse/PostToolUse field-level facts: F8.
- **F8 — Hooks platform, field level (fact-finding seat over code.claude.com/docs/en/hooks-guide
  + /hooks, 2026-09-13; items marked *unverified* are the seat's inference, to be re-read cold
  before any decision leans on them).**
  - Event set now includes, beyond F7's: `PostToolUseFailure · PostToolBatch · SubagentStart ·
    SubagentStop · FileChanged · InstructionsLoaded · Stop · UserPromptSubmit`, plus
    `PermissionRequest / PermissionDenied`, `TaskCreated / TaskCompleted`, compaction and
    worktree events.
  - **PreToolUse** input: `tool_name`, `tool_input` (the tool's arguments — `file_path` for
    Edit/Write; Read/Glob/Grep field names *unverified* — the docs show no example). Matcher is a
    tool-name regex (`Edit|Write`); an `if` field narrows on arguments (`"if": "Bash(git *)"`).
    Exit 2 blocks with stderr fed back to the model; exit 0 + JSON
    `hookSpecificOutput.permissionDecision` `allow|deny|ask|defer` with
    `permissionDecisionReason` (fed to the model) and `additionalContext`. A deny reaches the
    model as text it can read and route around — a hook constrains one call, never the plan.
  - **PostToolUse** input adds `tool_response`; exit 2 / `decision: "block"` + `reason` feeds
    the model but cannot undo the write ("PostToolUse hooks can't undo actions"). PostToolUse
    carries top-level `additionalContext` and `systemMessage` (verified at review, I9/M3 —
    the seat's second doc read). PreToolUse also carries a top-level `updatedInput` that
    rewrites the call silently — this design never uses it (D1: deny, never correct).
  - **Turn-level alternative the docs name outright:** "If your hook must see every file
    change, such as for compliance scanning or audit logging, add a `Stop` hook that scans the
    working tree once per turn."
  - Hooks apply to subagents (`agent_id`, `agent_type` on input; `SubagentStart`/`SubagentStop`
    events); subagent-frontmatter hooks run only for that subagent's life. Plugin
    `hooks/hooks.json` applies when the plugin is enabled. **Merge is documented** (corrected
    at review, I9): "Hook entries merge across settings levels rather than replacing each
    other"; a consumer may set `disableAllHooks: true`, and an enterprise may set
    `allowManagedHooksOnly` — so any gate is one consumer setting away from off (D7/D9 scope
    line). `permissionDecision` values are `allow | deny | ask | defer` (`defer` documented for
    non-interactive `-p` runs — M3 verified). **A `deny` fires "before any permission-mode
    check, in every permission mode … even in `bypassPermissions` mode or with
    `--dangerously-skip-permissions`"** (docs, verified at review).
  - **A `Write|Edit`-matched hook does not see a file written through the Bash tool** (docs,
    verified at review — C5; source `code.claude.com/docs/en/hooks-guide`, the "Block edits to
    protected files" section, fetched 2026-09-13): "Claude can also create or modify files by
    running shell commands. If your hook must see every file change, such as for compliance
    scanning or audit logging, add a `Stop` hook that scans the working tree once per turn."
    and "To run a hook when a specific file changes on disk, whatever wrote it, use a
    `FileChanged` hook." and, same section, verbatim (re-located at the delta-check, V6): "For
    per-call coverage instead, also match `Bash|PowerShell` and have your script list modified
    and untracked files with `git status --porcelain`." followed by "The PowerShell hook input
    section explains why matching `Bash` alone is not enough." `FileChanged`'s `matcher` is a
    list of literal filenames, not a pattern (verify pass). The `if` field
    narrows a matcher by arguments and is best-effort for Bash.
  - Default `command` timeout 600 s (30 s for `UserPromptSubmit`), per-hook `timeout` override;
    placeholders `${CLAUDE_PROJECT_DIR}`, `${CLAUDE_PLUGIN_ROOT}`, `${CLAUDE_PLUGIN_DATA}`.
  - **No documented pattern for schema or content validation via hooks**; the closest examples
    are path-pattern blocks on Edit/Write (`.env`, lockfiles) and post-edit formatters.
- **F9 — Artifact census (Explore seat, 2026-09-13; locate-only, paths as declared in
  primitives).** Roughly thirty markdown artifact paths are declared across commands and skills
  in prose (`.mochiko/specs/<spec>/{spec,tasks,data-model,constraints-and-decisions,quickstart}.md`
  + `stories/US-*.md` + `contracts/api.yaml` · `.mochiko/brainstorms/<slug>/record.md` + `index.md`
  · `.mochiko/features/FEAT-XXX-<slug>.md` + `FEAT-XXX/gates.md` · `.mochiko/epics/EPIC-XXX/` ·
  `.mochiko/product/architecture/{spine,concerns}.md` + `concerns/AX-XXX-<slug>.md` ·
  `.mochiko/product/{data-model,quickstart,constraints-and-decisions}.md` ·
  `.mochiko/memory/{governance-intent,governance-ledger,codebase-analysis,knowledge-management}.md`
  · `.mochiko/decisions/<date>-<slug>.md` · root `FEATURES.md · ARCHITECTURE.md · DECISIONS.md ·
  BACKLOG.md · ROADMAP.md · GLOSSARY.md`). **Only eight carry a CLI-served template** (F4);
  the rest have their shape in skill prose, `references/*.md`, or the `templates/*.md`
  envelopes (`artifact-format.md`, `report-format.md`, four report templates) — none machine-
  checkable today. Location is nowhere machine-checked: no hook, no CLI subcommand, no
  `paths:`-rule reads a produced artifact's path against a declared pattern. The four
  `.claude/rules/mochiko/*.md` files are `paths:`-scoped and inject on Read (CLAUDE.md observed
  behavior) — advisory prose, not a gate.
- **F10 — The concrete instance: kinako EPIC-002 (run 2026-09-09..12 on plugin v0.108.0,
  closed `closed-partial`; user-named at Q1; lead's own read 2026-09-13).**
  - *File set undeclared, so precedent supplies it.* Of the EPIC-002 spine's files, no mochiko
    primitive (commands, skills, migrations, templates) names `build-order.md`,
    `screens-and-flows.md`, `implement-log.md`, `reports/`, `reviews/` (0 mentions each); in
    the feature homes likewise `plan.md` and `design-closure.md` (0 mentions; corrected at
    review I8 — they are feature files, not spine files); named: `cycle-report` (6 files),
    `baseline-delta` (5), `requirements.md` (3), `sufficiency-report` (2). EPIC-001's tree carried
    `proposal.md`, `contest-brief.md`, `landing/{checkpoint,close,fold,fold-grade}.md` — also
    undeclared; EPIC-002 dropped those and minted `reports/`, `build-order.md`,
    `screens-and-flows.md`. The shape drifts run to run with the prior epic as the source:
    `card-review.md:320` cites ".mochiko/epics/EPIC-001/reviews/card-review.md (report shape
    only)"; `design-phase-plan-artifacts.md:248` lists five EPIC-001 files as "read-only seed
    sources"; the implement log says "as EPIC-001" / "EPIC-001's precedent" at R6, R7, K5 and the
    outage handling.
  - *A delivered rule lost to precedent, disclosed.* `impl.reports-envelope` (rendered at fire):
    "All reports land in `.mochiko/features/FEAT-XXX/`". The log (line ~329): "Reviews land at
    `.mochiko/epics/EPIC-002/reviews/` (the EPIC-001 precedent), disclosed against
    `impl.reports-envelope`'s per-member default because one joint package is graded." Rule
    read, precedent chosen, deviation disclosed — nothing checked it.
  - *`tasks.md` home is split across sources.* `executing-tdd-cycle` SKILL.md prose declares
    `.mochiko/specs/<feature>/tasks.md`; the migration log's 13 `tasks.md` mentions carry no
    path; kinako writes `.mochiko/features/FEAT-XXX/tasks.md` (at HEAD 2026-09-13: FEAT-001
    1,003 lines, FEAT-002 1,731 — the log's own 651→836 / 1175→1452 were card-writing-time
    figures; corrected at review I8 — with an appended `## EPIC-002 cycles — 2026-09-10`
    section).
  - *Template shape partially held, never checked — and partly the template's own defect
    (annotated 2026-09-13, user-ruled R6 on the wave-1 readiness finding).* `mochiko-cli
    template tasks` declares the sections `Header · Overview · Cycle Format · Cycle Cards`, but
    its own skeleton places `### - [ ] Cycle 1: …` directly under `## Cycle Format` with no
    `## Cycle Cards` heading and treats `## Header` as a meta section; kinako's `FEAT-002/tasks.md`
    (no `## Header`, no `## Cycle Cards`, cards under `## Cycle Format`, ids `C2-n`) therefore
    followed the skeleton faithfully — the section list and the skeleton disagree, and that drift
    is mochiko's. The log confirms `template tasks` rendered at run open; `--check` was never run
    against the produced file. Wave 3's census reconciles the two (`wave1-template-readiness.md`).
  - *The report envelope held.* Reports and reviews open with `report: … · feature: EPIC-002 ·
    round: n` frontmatter per `templates/report-format.md`; extra fields permitted.
  - *Manifest status vocabulary strained.* The manifest's own comment: `closed-partial` chosen
    as "the only value that is not false", with the vocabulary gap "raised as a finding".
- **F11 — `--check` is a model-applied checklist, not a machine grammar.** `mochiko-cli
  template tasks --check` renders four `- [ ]` questions of the form "Does the Cycle Format
  explain …" — judgment questions for a validator seat. Nothing in the log today states a
  template's required heading sequence, required frontmatter fields, forbidden placeholder
  tokens, or a size bound in a form a binary could evaluate against a file. D1's write-time gate
  therefore needs a **new machine-checkable conformance layer** per kind, alongside the
  producer and checklist views.
- **F12 — The verbosity-envelope ADR (2026-08-22) already named this gap, prose-only.** Root
  cause 1: "Unenveloped artifact classes … the epic-run artifacts that ballooned — proposal,
  contest brief, epic architecture delta, plan-run reviews, derivation — had no format home, no
  template, no cap." Its fix widened `artifact-format.md` and `report-format.md` scope and made
  overage an advisory finding. EPIC-002 (F10) shows the advisory did not hold the line: the
  spine is 11,713 lines, the implement log 3,653.
- **F13 — The symptom is library-wide, not implement's (kinako `.mochiko/` census, lead's
  read 2026-09-13; user's framing at Q5: "It was pointed as a symptom of a bigger problem").**
  **288 markdown files, 115,757 lines** under `.mochiko/` at HEAD 2026-09-13 (per-dir:
  features 156 files / 66,283 lines · epics 56 / 30,807 · product 9 / 6,321 · brainstorms 17 /
  5,975 · specs 30 / 2,944 · memory 4 / 1,810 · decisions 12 / 675; the remainder `archive/`
  — totals corrected at review I8, lead re-counted). Undeclared homes and names, by workflow:
  - **feature/specify:** `.mochiko/features/desk/2026-09-09-sandbox-runner/` (desk home —
    0 primitive mentions of `features/desk`); `.mochiko/features/B53/` — a non-`FEAT-XXX` id
    holding nine `cycle-NN-report.md` files (0 mentions); the specify desk output landed as
    `specs/atom-extraction-via-sparring/derivation.md` in one run and
    `specs/corpus-stewardship/map-delta/` in the next (each name has exactly one primitive
    mention — two homes for one output).
  - **implement (feature runs):** `features/FEAT-006/reviews/` beside `reports/`;
    `features/FEAT-00x/reports/evidence/` sub-dirs (0 mentions).
  - **brainstorm:** review artifacts beside `record.md` named per run — `review-A-map.md`,
    `review-B-crossexam.md`, `review-survivors.md`, `review-verify.md`, `probes.md` (0 mentions
    of any; mochiko's own repo uses `review-lens-a.md`); the migration log declares no review
    home for the brainstorm command at all.
  - **architecture/product:** conforming (`spine.md`, `concerns.md`, the baselines).
  Every command minting artifacts shows the same pattern: the home is named in prose or not at
  all, the file set is never declared, and the run fills the gap from a sibling run.

## Constraints carried in

- GI-019 bright line (F3) — any gate proposed here must be argued against it, not around it.
- GI-020 clone-only install; every command depends on `mochiko-cli`; no schema file ships.
- Hooks ship to every consuming project and execute the plugin author's code (D7 ratified).
- Fail-open platform: a hook can add loudness or a deterministic deny, never a guarantee.

## Decisions (D)

*(recorded as ruled — statement · rationale · confidence)*

### D1 — Three channels, mechanical deny before the write: authoring-time delivery of the kind's conformance block · a read-time reminder · a write-time `PreToolUse` gate on location · file set · section shape · size — matcher `Write|Edit|Bash`

**Statement (as amended at review — C1/R1, C5/R3, I1, I2, I3, M1, M2, M6):** Three channels,
one evaluator (D3). **(a) Authoring-time delivery:** the kind's conformance block — declared
home, file set, required `##` sections in order, per-section line budgets, placeholder tokens —
is part of the `mochiko-cli template <kind>` **producer view** for the templated kinds, and
**for a kind with no template yet, the producing command's or skill's rendered rules carry the
home, its file set, and the whole-file bound from `mochiko-cli home <path>`** (the subcommand
D3 mints) — so every kind, templated or not, has an authoring-time channel before the seat
drafts a line (V1); this is the channel that serves the token driver directly. **(b) Per-seat reminder at `SubagentStart` (amended post-acceptance 2026-09-13, user-ruled
R5 on wave-0 leg 8):** a `SubagentStart` hook returns `hookSpecificOutput.additionalContext`
— one self-identified line, landing in every spawned seat's context before its first turn
(named and unnamed spawns alike; wave-0 proof: `attachment: hook_additional_context` in both
sidechains): "mochiko gate: artifacts under declared homes take their shape from `mochiko-cli
home <path>`; existing files are not templates." Never a deny. The lead session needs no
injection — channel (a) reaches it through its own command render. *Superseded by this
amendment:* the `PreToolUse`-on-`Read` per-read reminder as originally ruled — wave 0 showed
one seat reading the unsourced per-read line as a prompt injection and ignoring it, and the
per-seat form fires once instead of on every home read (the Read term was ≈ 1.0–1.6 s per run
under the `if` narrowing, so the ground is delivery shape, not cost). The D10 watch metric
"reminder lines per run" becomes "SubagentStart injections per run". **(c) Write-time gate:** `PreToolUse` matched on `Write|Edit|Bash|PowerShell` (the
`PowerShell` leg added at the delta-check, V6 — the docs state a `Bash` match alone leaves
the Windows shell tool uncovered, and Windows with Git Bash is a supported environment under
GI-020; a lead repair inside R3's intent, disclosed for acceptance), narrowed with `if` to the
declared homes where the tool carries a path — the `if` field sits on the **handler object**,
never the matcher group (wave 0: on the group it is silently ignored); for `Bash` the
narrowing is a path-substring pattern (`Bash(*.mochiko*)`), which wave 0 measured as the
primary cost lever (60–64 % of gated calls removed; redirect-shaped patterns do not fire). For `Write`/`Edit` the wrapper
pipes the raw hook payload to `mochiko-cli check --hook-json -`; the CLI answers allow, or
**deny** — never a silent `updatedInput` correction — with a reason carrying the declared home,
the allowed file set, the required skeleton, and the section budget, whichever failed. For
`Bash` and `PowerShell` the CLI denies a command whose text carries a write operator (`>`, `>>`, `tee`, `sed
-i`, `cp`/`mv` destinations, heredoc redirects) aimed at a path under a declared home, reason
"artifacts under declared homes are written with Write/Edit" — best-effort string parsing,
stated as such (F8: the docs' own remedy for shell writes). `NotebookEdit` and any MCP file
writer a consumer enables are **knowingly outside** the matcher set. The four checks are
mechanical only: path against the home's pattern · file name against the home's declared set ·
required headings and frontmatter fields present · per-section size within budget. No check
reads meaning, grades quality, or sequences seats. A deny holds in every permission mode,
including `bypassPermissions` (F8, doc-verified).

**Rationale:** At `PreToolUse` on `Write` the content is already in `tool_input.content`, so
the deny **bounds persistence, not generation cost** — a failing draft is re-emitted once
(I2, stated honestly); channel (a) is what shapes the draft before tokens are spent, and it
rides the delivery channel the thirty skills and six commands already use. Channel (b) fires
at the moment the model reaches for a sibling artifact as its shape source (F10: EPIC-001 read
as "report shape only"), at zero re-emit cost. Channel (c) is the floor: F10 shows a delivered
rule read and overridden by precedent, disclosed, and F13 shows ~22 of ~30 kinds with nothing
delivered at all — (a) closes the second class for every kind (templated kinds get the full
conformance block, template-less kinds the home/set/bound line from `mochiko-cli home`; the
section shape of the untemplated ~22 arrives as their templates land, D4f), (c) holds the
first. The shell legs exist
because a `Write|Edit` matcher provably never sees a shell write (F8) and a denied seat's
cheapest evasion is a heredoc.

**Rejected roads:** **declare-and-measure first** (C1 — land the registry and channel (a),
run one measured kinako run, scope the deny to the residual by a later ruling; **user declined
at R1**: the driver is no further generate-then-fix run, a measurement run is one more ~30k-line
drift run at kinako's cost, and channel (a) ships beside the gate at no extra cost — the road's
argument stands recorded, the fork was put with both costs named) · **deny on path/set/size
only, shape reminder-only** (leaves the class that most tempts sibling-copying unprevented) ·
**reminder everywhere, no deny** (stays inside `cli-schema-delivery` D7 as ruled but prevents
nothing) · **a `Stop` hook scanning the tree once per turn** and **a `PostToolUse` advisory**
(I1 — both post-generation, the class the driver excludes; neither reaches a hook-disabled
consumer either, so neither is a hole mitigation — the holes' mitigation is the procedural
author≠grader review, D9) · **`updatedInput` silent path correction** (hides the drift from
the user and the model; a corrected path is a fact the seat never learns) · **a `FileChanged`
hook** (V7 — fires after the file is on disk, whatever wrote it, so post-write by construction;
and its `matcher` is a list of literal filenames, not a pattern, so it cannot watch a tree of
homes — verified at the verify pass).

**Supersession (narrow):** `cli-schema-delivery` D7's "behavior-gating hooks are declined"
stays for judgment and sequencing; **mechanical conformance gates on artifact writes are
admitted** by this ruling — the from-zero ruling D7 reserved. GI-019's ledger detail and the
`rust-cli.md` line "hooks MUST block only on the binary's absence or a log outside its grammar
range, never on behavior" take the carve at D7.

**Confidence:** Confident — re-marked at R1 (I11): the user chose the deny-now road over the
review's declare-and-measure road with both costs in view, not by adoption alone; R3's Bash
extension user-ruled "yes".

### D2 — File set: closed per home for deliverables, open-by-name for reports under one `reports/` dir per home

**Statement:** Every declared home (`.mochiko/epics/EPIC-XXX/` · `.mochiko/features/FEAT-XXX/`
· `.mochiko/specs/<spec>/` · `.mochiko/product/` (+ `architecture/`) · `.mochiko/memory/` ·
`.mochiko/brainstorms/<slug>/` · `.mochiko/decisions/` · the root operating docs) declares its
deliverable file names (and sub-dirs); a `Write` to an undeclared name in a home is denied,
the reason listing the set and naming the real route (amended at review C4, **user-ruled R2b**):
"`<name>` is not a declared deliverable of `<home>`. If it is a report, give it a `report:`
type from the envelope's enum and put it under this home's `reports/` (any name). Otherwise
raise it upstream — a new deliverable kind takes a migration in the plugin's log and a plugin
release." (V4: the `reports/` route is open only to content that honestly carries an enum
type; non-report content ends at the upstream ask, which R2b accepts as the only route.) There is **no consumer-local declaration
surface**: the registry ships only in the plugin's migration log (D3), and a readable
consumer-side homes file would be the schema-file road GI-020 closed. Each run-unit home
carries exactly one `reports/` dir: any file **name** is allowed there, the content must open
with the `templates/report-format.md` envelope (`report:` from the format's enum · `feature:` ·
`round:` or `cycle:`+`attempt:`), and each report **type** carries its own section budgets —
names free, types enumerated (M8). `reviews/` is not a home — reviews are reports
(`report: review`) and land in `reports/`; this holds for every command, **including the
brainstorm command's cold-review artifacts**, which land at
`.mochiko/brainstorms/<slug>/reports/` (I5; D5 amended to match — this session's own
`review.md` beside `record.md` predates the gate and is re-homed at the census). Where a
delivered rule already enumerates report kinds (`impl.reports-envelope`: sufficiency, cycle,
verification, final-validation, built-vs-signed diff), the enum in `report-format.md` becomes
the binding type list and the rule's path re-keys to `reports/` at wave 4 (M8). A new
deliverable kind therefore needs a migration entry and a plugin release; that friction is the
point. The build
runs a **census** of every file name in kinako's EPIC-001, EPIC-002, and FEAT dirs (F10): each
is declared with a kind and envelope, or forbidden — `implement-log.md` expected to be declared
as an append-only log bounded per entry, not per file.

**Rationale:** The drift class in F10 is undeclared names minted from precedent
(`build-order.md`, `screens-and-flows.md`, `reports/` vs EPIC-001's `proposal.md`,
`contest-brief.md`, `landing/`); a closed set is the only shape under which a deny can name
what is allowed. Reports are run-time products whose count and names legitimately vary per
run, and they already carry a machine-checkable envelope (F10: the frontmatter held), so the
check there is the envelope, not the name. Rejected: **fully closed incl. reports** (every
review name pre-declared — brittle for no drift gain, the envelope already binds them);
**open set with required core** (declines the drift fix; the mint continues).

**Confidence:** Confident (user-ruled: "as recommended" at Q4; the consumer-escape amended
by user ruling R2b at review — the local-surface road declined against GI-020).

### D3 — Source of truth: a `home` document kind in the migration log, CLI-served; `mochiko-cli check` is the one evaluator both hooks call

**Statement:** Homes, their file sets, the kind→template binding, the per-kind machine
conformance block (D4), and the size envelopes are one new migration-log document kind
(`home`), replayed at fire like every other kind — no schema file ships (GI-020 holds), the
derived views regenerate repo-side; the CLI resolves one log (flag · `MOCHIKO_MIGRATIONS` ·
plugin root) and merges nothing consumer-local (D2). The crate gains **`mochiko-cli check
--hook-json -`** — it consumes the raw `PreToolUse` payload on stdin and parses `tool_name`,
`tool_input.file_path`, `tool_input.content`, `old_string`/`new_string`, and `command` in Rust
(I4: the shipped wrapper's grep-and-sed `field()` cannot carry multi-line escaped content, and
`jq` is not a dependency the plugin may add) — and `mochiko-cli home <path>` (renders a path's
home, set, kind, and budgets for a model or a human). Both D1 hooks are thin POSIX `sh`
wrappers that pipe stdin through and emit the decision — the scripts hold no rule, the same
posture as `dependency-halt.sh`. **Exit-code contract (C2):** the wrapper **always exits 0**
and speaks JSON; only a conformance verdict from `check` (its own exit code, minted as a fifth
code beside 0/1/2/3 or documented as a reuse at wave 1) becomes `permissionDecision: deny`;
CLI exit 1 (log unsound), 2 (usage — e.g. an installed binary that predates `check`), and 3
(version skew) are **pass-through: the write proceeds, and the wrapper emits an explicit
`permissionDecision: allow`** — never empty stdout, because the platform denies a background
subagent's call when no hook returns a decision (wave-0 platform fact; a lead repair inside
D7's fail-open intent, 2026-09-13) — absence and skew are already
the shipped halt hooks' job, and a usage error must never deny a consumer's every write. One
contract case per exit code (D10). **Per-call cost:** `check` replays the log per call as
`rules` does today (35 ms cold at wave 1); D8 measures the aggregate per run, and the
`${CLAUDE_PLUGIN_DATA}` replay cache `cli-schema-delivery` D1 deferred "on measured need" is
the named mitigation if the per-run budget trips (I3).

**Rationale:** Derived, not chosen: GI-020 forbids a readable schema file in the plugin, the
migration log is the ruled truth for every other rule class, and a second store for homes
would be the divergence `cli-schema-delivery` D1 closed. One evaluator keeps the reminder and
the gate from disagreeing on what a home is.

**Confidence:** Assumed (derived from GI-020 + `cli-schema-delivery` D1; not put to the user
as a fork — surfaced for acceptance; exit-code and interface repairs C2/I4 folded at review).

### D4 — Conformance is mechanical: ordered required `##` headings with extra `##` denied · required frontmatter and enums · placeholder tokens absent · per-kind size · Edit checked on the in-memory result · no shape check without a template

**Statement:** Per kind, the `home` document carries a conformance block the binary evaluates:
**(a)** required `##` headings present in the declared order; an undeclared `##` heading is a
deny; `###` and deeper are free — **a section's line count runs from its `##` line to the next
`##`, nested content included** (I7: nesting under `###` is not an escape). **(b)** required
frontmatter fields present; a field declared as an enum holds a listed value. **(c)** the
template's placeholder tokens are absent, checked **only in frontmatter values and heading
text, by the exact token spelling the template declares** (`[entity]`, `<feature-id>`, a
literal `FEAT-XXX` as a heading's id) — never as a body substring, since honest prose names
`FEAT-XXX`/`EPIC-XXX` as patterns (M4). **(d)** size within the kind's per-section budgets
(D6); an append-only log (`implement-log.md` and its class) is bounded per entry, never per
file. **(e)** on `Edit`, the CLI applies `old_string` → `new_string` to the on-disk file in
memory and runs (a)–(d) on the result, so an append that adds an undeclared `##` or crosses a
budget is denied before it lands — **with first-touch amnesty (C3, V3) on any write over an
existing file, `Write` and `Edit` alike:** the on-disk file is the baseline (for `Write`,
`tool_input.content` is the result; for `Edit`, the applied result), and where the baseline
already violates a measure, a write that does not worsen that measure is **allowed** with the
standing overage reported as `additionalContext`; only a worsening write is denied. A new file
at a declared name takes the budget outright. A file cannot be wedged by its own history, and
the rewrite that fixes it is itself an allowed non-worsening write. **(f)** a kind with no template in the log (today ~22 of ~30 homes) takes
location + set + whole-file size checks only; no shape rule is invented from prose — a template
lands first, then its conformance block. **Exemption (V2, clarified 2026-09-13 at wave-1
review G8):** a kind whose bounds already live in another named constraint home takes **no
size check** here, that home cited; the shape checks (a)–(c) still run where a template binds
the file, and location + set always bind — the root operating docs were the motivating case
(their caps and bounds are the KM invariants, D5/I6; at wave 3 they are not a home at all,
R6), and `.mochiko/memory/` is the live one. The earlier "location + set only" wording is
superseded by this clause; D6 already read it this way.

**Rationale:** Each check is decidable by string and count, which is what keeps the gate on
the admitted side of GI-019 (D1). Extra-`##`-denied is the one setting that catches F10's
`## EPIC-002 cycles` and F13's per-run inventions; leaving `###` free keeps content structure
the producer's. Edit-on-result is what makes appends honest — most drift in F10 arrived by
append (FEAT-002 `tasks.md` grew by an appended `## EPIC-002 cycles` section to 1,731 lines at
HEAD; F10 carries the qualifier). Rejected: **extra `##` allowed** (the observed drift
passes); **Edit path+size only** (appended drift passes).

**Confidence:** Confident (user-ruled: "as recommded" at Q5; counting rule, token scope, and
first-touch amnesty folded at review I7/M4/C3 — lead repairs under the user's batch ruling).

### D5 — Scope: every mochiko workflow's artifact homes, the whole `.mochiko/` tree plus the root operating docs; the census runs over kinako's full tree and mochiko's own

**Statement:** The `home` registry covers every artifact home any command or skill mints —
brainstorm (`record.md`, `index.md`, `synthesis.md`; the cold review's artifacts are reports
under `reports/` per D2 — no separate review home, I5), setup (`.mochiko/memory/*`;
`.claude/rules/mochiko/*.md` **location and file name only** — their content is the
consumer's Claude Code configuration; **`CLAUDE.md` is outside the write-time gate entirely**,
its governance region has its own ceremony through `/mochiko:setup` — I10), specify and
feature (`specs/<spec>/*`, `stories/`, `prototype/`, the desk output — one home, not two —
`FEATURES.md`, `FEAT-XXX` entries, `features/desk/`), architecture (`product/architecture/*`,
`ARCHITECTURE.md`), implement (feature dirs, epic spines, product baselines, `gates.md`), and
the root operating docs (**location only** — their caps and bounds already live in the KM
invariants at `.mochiko/memory/knowledge-management.md` with `mochiko:grooming-operating-docs`
as the responder; the budget table cites those bounds and restates no number, GI-017 — I6). The D2
census runs over kinako's whole `.mochiko/` tree and root docs (F13: 288 files) and over
mochiko's own `.mochiko/` as a second sample; every observed name is declared, merged into a
declared name, or forbidden — `B53`-class ids and the two desk-output homes resolved by ruling
at the census, not silently.

**Rationale:** F13: the same pattern in every workflow — home in prose or absent, set never
declared, sibling run fills the gap. A registry that covered implement alone would move the
drift, not end it. The user named EPIC-002 as a symptom.

**Confidence:** Confident (user-ruled at Q5: "ensure we are focussing on all the workflows of
mochiko, not just the ones analysed in implement run of EPIC-002").

### D6 — Size budgets are tight and live per section in the template schema, not per file; logs per entry; template-less kinds take a whole-file bound until their template lands

**Statement:** Every template's conformance block (D4) carries a `max_lines` per declared
`##` section; the write-time gate evaluates each section against its own budget and denies
naming the section and its overage. There is no separate whole-file cap for a templated kind —
the file's bound is the sum of its sections. Report kinds carry the same per-section budgets
over their envelope payload; an append-only log is bounded per entry (one entry = one dated
`##` block). A kind with no template yet (D4f) takes a single whole-file bound as its
placeholder until its template lands — **except a kind whose bounds live in another named
constraint home, which takes no size bound here at all** (D4f's exemption; the root operating
docs, bounded by the KM invariants — V2). Posture is **tight**: each budget is set at the compact
target the 2026-08-22 verbosity-envelope ADR named for that class, never derived from kinako's
observed sizes — the census measures the gap, it does not set the line. The full budget table
(kind × section × lines) is proposed at the census as one artifact and user-ratified before
the build ships; the first dogfood run is expected to trip denies on **new** writes, which is
the signal working — existing files are never wedged (D4e first-touch amnesty, C3), and the
live violators are split or re-homed by a named pass before size denies go live in a repo
(D11 wave 5 precondition). Displacement — prose moved under free `###`, into extra `reports/`
files, or into log entries — is measured, not assumed away: D10's watch carries a per-home
total-volume figure and a reports-per-run count (I7).

**Rationale:** The user's ask ("limit be at section level by schema, rather than the entire
file. We already have template schemas"): a per-file cap lets one bloated section starve the
others and tells the producer nothing about where to cut; a per-section budget points at the
overage and reuses the section grammar the templates already declare. Tight over generous:
F12 — the advisory envelope did not hold; a generous line would ratify F13's sizes. Rejected:
**per-file caps** (blind to where the bloat is); **generous budgets at ~2× observed** (catches
only pathological files, verbosity unchanged).

**Confidence:** Confident — marked from the answer, not the adoption (I11/V13): the
section-level shape is the user's own wording, and the tight posture was chosen after a
plain-language re-put of both sides.

### D7 — Governance routing: narrow supersession of `cli-schema-delivery` D7 and a GI-019 ledger amend, through a `/mochiko:setup` amend run before the hooks ship

**Statement:** The admission in D1 lands as a governance event, not a build side-effect: a
`/mochiko:setup` amend run records **(a)** `cli-schema-delivery` D7 superseded narrowly —
"behavior gating declined" stands for judgment and sequencing; mechanical conformance gates
on artifact writes (path · file set · headings/frontmatter/placeholders · per-section size)
are admitted; **(b)** GI-019's ledger detail gains that carve with this record as rationale;
**(c)** the hook floor from D7 carries over unchanged — 5-second `timeout`, fail-open when the
hook cannot run, hooks ship to every consuming project executing the plugin author's code —
re-ratified knowingly for hooks that now fire on every home `Read` and on every
`Write`/`Edit`/`Bash` call under a home. **(d)** the `.claude/rules/mochiko/rust-cli.md` line
"Its hooks MUST block only on the binary's absence or a log outside its grammar range, never on
behavior" and its "MUST NOT grade an artifact" clause are edited as a **recorded strip** under
the same amend — "validates an artifact's mechanical conformance against its own data" is
the admitted reading, grading stays forbidden. **(e) Scope of the guarantee (I9):** the gate is
a floor for consumers who keep the plugin's hooks enabled; a project that sets
`disableAllHooks`, or an enterprise under `allowManagedHooksOnly`, keeps only the procedural
author≠grader ceremony — a ratified consequence, not a defect. `producer-plan-enforcement` D1
(no hook gate on plan mode) is untouched. Version bump class
per the ledger's amendment policy (expected MINOR: an admission widened, no floor removed).

**Rationale:** `cli-schema-delivery` D7 said the behavior gate "takes its own ruling from
zero" — this record is that ruling, and the same session's GI-019/GI-020 changes went through
an amend run (wave 2), so the route is precedented. Landing the carve silently in a hook script
would be the record-layer corruption GI-005 forbids.

**Confidence:** Confident (drafted by the lead as derived; user-confirmed as drafted at Q7 — the amend run itself stays the user's).

### D8 — Transport coverage: a wave-0 probe with an abort criterion; the reminder channel's `Read` field name, the Bash leg, permission modes, dead-gate behavior, and per-run cost measured there too

**Statement:** Before the crate wave, one probe session establishes, for each transport the
runs use — the lead session, an `Agent` subagent, an agent-team teammate, and a `--agent`
headless session — whether the plugin's `PreToolUse` hooks fire on `Write`/`Edit`/`Read` and
whether a deny reaches that seat as text. It also records: the exact `tool_input` field name
for `Read` (F8: undocumented); **whether a `Bash` heredoc write to a gated path fires the
`Bash`-matched hook and whether a denied seat retries through Bash** (C5); **whether a deny
holds under `bypassPermissions`, `acceptEdits`, and `auto`** — the docs say every mode
(F8), the probe confirms it on the modes the runs actually use, and a gate inert in those
modes is the same abort as one inert for subagents (I12); **what the platform does when the
wrapper is unexecutable or `${CLAUDE_PLUGIN_ROOT}` fails to resolve** (a non-blocking error
that silently proceeds — how a dead gate is noticed, I12); and the **aggregate per-run cost**
of `check` and the reminder over one representative kinako run, **counting every `Bash` call**
— the `Bash` matcher fires on every shell command, not only on writes, so shell-call volume is
part of the measure (V12) — the abort line is a stated per-run budget, not the 5-second
per-call timeout (I3/M7). **Abort criterion:** if the gate does not fire for
`Agent` subagents (the transport every producing seat rides), the write-time gate is not built
and the session reopens on the channel question; a teammate-only miss is recorded as a
disclosed hole, not an abort. This probe **absorbs the backlog's agents-as-schema channel
probe** (F5) — same three legs, same session.

**Rationale:** F8: hooks apply to subagents per the docs, with `agent_id` on input; teammates
and headless `--agent` sessions are undocumented, and the eval-v2 record already owes the same
probe. A gate that fires only for the lead would enforce nothing the producing seats do.

**Result (wave 0, 2026-09-13 — `wave0-probe-report.md`, reviewed PASS after one fix round;
user ruled PROCEED at R4):** abort not tripped. All four transports fire on every channel and
the deny reaches the seat as text; `Read` carries `tool_input.file_path`; deny held under
`dontAsk` · `acceptEdits` · `bypassPermissions` · `auto` on lead and subagent; a dead gate
(exit 126/127) proceeds silently with an operator-only stream event; the shipped grep/sed
payload parser produced one false allow (a `printf >` write created the file — D3/I4 settled
by experiment); Bash is 85–86 % of gated volume, the `if` narrowing removes 60–64 %; per-call
49 ms, projected 9.2–27.6 s per run on two non-implement transcripts (no EPIC-002 implement
transcript exists — the first dogfood run re-measures). Leg 8 (lead-added): `SubagentStart`
injection lands before the first turn → D1b amended (R5). Disclosed holes: D8's `Edit` limb was
exercised once, not per transport (same matcher arm and key as `Write`); the `PowerShell` arm
is unverifiable on macOS (registration valid); the spawn tool's wire name is unverified (`Task`
in the init array, `Agent` on every call) — moot for this design, which registers the
`SubagentStart` event and no spawn matcher. A denied seat escalates to delegation; the spawned
subagent inherits the gate. OQ3 closed (teammates are gated; no fan-in mitigation owed).

**Confidence:** Confident (drafted by the lead as derived; user-confirmed as drafted at Q7).

### D9 — Deny discipline: stateless hooks; the owned mechanisms are the `Bash`-matched deny and the every-permission-mode floor; the two-strike halt sentence is advisory; frontmatter sniff outside homes; no gate on `.md` writes outside the declared tree

**Statement:** The hooks hold no state. Every deny reason ends with the same instruction: "a
second deny on this path halts — surface the check's text to the user, do not rewrite around
it." **That sentence is advisory** (C5) — the mechanisms the hook itself owns are the
`Bash`-matched deny (D1c), which closes the cheapest route around a `Write` deny, and the
platform fact that a deny holds in every permission mode (F8). A `Write` outside every declared
home is gated only when its content opens with mochiko report frontmatter (`report:` in the
enum) or a template's `## Header` signature — the frontmatter sniff — so a report smuggled to
`docs/` is caught; a plain `.md` elsewhere is not mochiko's business (product repos have their
own docs). Escapes past the sniff, writes through tools outside the matcher set (D1c), and
hook-disabled consumers (D7e) are the reviewer's to catch under the existing author≠grader
ceremony; the gate is a floor, not the whole fence.

**Rationale:** A model denied twice is in a loop the hook cannot see (stateless, fail-open);
the halt sentence is advisory — it costs nothing and helps an obedient seat — but the
backstops the record relies on are the `Bash` deny (closing the cheapest evasion), the
permission-mode floor (a deny cannot be switched off by mode), and the reviewer's read of the
file set at fan-in (V9). Gating every `.md` in a product repo would be a plugin overreach the
consuming project never ratified.

**Confidence:** Confident (drafted by the lead as derived; user-confirmed as drafted at Q7).

### D10 — Test regime: crate matrices for `check` per home, deterministic contract cases for both hooks, a kinako first-live-run watch measuring denies and section sizes

**Statement:** Crate: an allow/deny fixture matrix per declared home (path · set · headings ·
frontmatter · placeholders · per-section size · Edit-on-result · first-touch amnesty · the
Bash write-operator parse), replay-determinism of the `home` kind, render goldens for
`mochiko-cli home`, the template producer view's conformance block, and the minted
authoring-time rule as it renders in a producing primitive's section (D1a both arms — V1), and **one case
per CLI exit code proving the wrapper's mapping** (C2: only a conformance verdict denies; 1, 2,
3 proceed). Plugin contract suite (`evals/contract/`):
deterministic cases for both hook scripts — the `SubagentStart` reminder returns the exact
frozen line under `hookSpecificOutput.additionalContext` with no `permissionDecision` key, is
fail-open with no binary, and stays silent on a wrong-event payload; a `Read` payload fed to
the gate wrapper is a plain allow (R5: the gate owns no Read behaviour — amended 2026-09-13,
superseding "reminder fires on a home read and not elsewhere"); deny fires on each of the four
failures and allow on a conforming write; **a denied `Bash`
heredoc write to a home and an allowed ordinary `Bash` command** (V12); fail-open when the
binary is absent (the hook must never break a session — D7 floor). Dogfood: the next kinako
run records deny count by kind, section sizes before/after, **per-home total volume and
reports-per-run** (I7 displacement watch), and the reminder's injected line count per run
(M2); a deny rate that stays high after the second run is the budget table's re-key trigger,
not the gate's.

**Rationale:** GI-012 gate 6 — a `plugin.json` bump ships nothing without the contract suite
green; the hooks are shipped code and take the same gate as the rest.

**Confidence:** Confident (drafted by the lead as derived; user-confirmed as drafted at Q7).

### D11 — Build order: five waves, the census and the budget table user-ratified before anything ships

**Statement:** **Wave 0** — D8 probe (abort or proceed). **Wave 1** — crate: the `home`
document kind and grammar bump, `check --hook-json -` and `home` subcommands, per-section
conformance evaluation, the conformance block in the template producer view, the D10
matrices; no shipped file touched; **exit condition: the crate is published** (`mochiko-cli-v*`
tag, crates.io + the tap — the owed wave-2 publish tail, C4: a consumer whose binary predates
`check` gets silent pass-through, never a deny, but gets no gate either). **Wave 2** — D7
governance amend run incl. the `rust-cli.md` strip.
**Wave 3** — the D2/D5 census over kinako's and mochiko's `.mochiko/` trees → one migration
carrying every home, set, kind→template binding, conformance block, and the budget table;
the two desk-output homes and `B53`-class ids ruled at the table; the brainstorm home's
`reports/` dir and its report types declared (no separate review home, I5/V11); **the same
migration mints the authoring-time rule into every producing command's and skill's rule set**
— "before the first write to `<home>`, render `mochiko-cli home <path>` and hold its file set
and bound" for template-less kinds, the template's conformance block riding the producer view
for templated ones (D1a's second arm — V1: this is a rule mint, neither a `home` document nor
a re-point); **user ratifies the table as one artifact**. **Wave 4** — hooks ship
(`hooks/hooks.json` gains the two registrations, two thin scripts), contract cases, the
`Not-done` and `Tools` lines of each command's `.md` re-pointed where a home changed
(`impl.reports-envelope` re-keyed to `reports/`), strips + audits per the primitive-edits
ceremony, `plugin.json` bump. **Wave 5** — in order (V5): **(1) the violator pass over the live repo** (kinako's oversized
and mis-homed files split or re-homed by ruling — C3) **runs with the consumer's plugin still
at the pre-gate version**, so no registration exists yet to deny a `mv` or a rewriting
`Write`; **(2)** the plugin is upgraded in that repo; **(3)** the kinako dogfood run under the
gate; D10 watch. Any later re-home under the gate rides D4e's amnesty (a `Write` at a declared
name whose result does not worsen the baseline is allowed) — a `Bash` `mv` into a home is
denied by D1c by design and is done with `Write`. **Rejected order
(C1, user-declined at R1):** registry + channel (a) first, one measured run, deny scoped to the
residual — recorded in D1.

**Rationale:** Mirrors the six-wave shape `cli-schema-delivery` ran, with the census as its
own ratification gate because the budget table is where the user's verbosity ruling (D6)
becomes numbers; the census re-counts at its own date and states it (I8) rather than
inheriting F13's figures.

**Confidence:** Confident (drafted by the lead as derived; user-confirmed as drafted at Q7).

## Open questions

- **OQ1 — Section budgets for prose-shaped deliverables.** A brainstorm `record.md` or a
  governance-intent synthesis has sections whose honest length varies by session; the D6
  table must decide per kind whether such sections carry a budget or a `max_lines: none`
  disclosure — ruled at the census table, not here.
- **OQ2 — `implement-log.md`'s status.** D2 expects it declared as an append-only log; whether
  the log belongs in the epic home at all (vs the run record living in reports) is a census
  ruling.
- **OQ3 — Teammate transport.** *Closed 2026-09-13 by wave 0:* teammates are gated on their
  resumed turn too; no mitigation owed.
- **OQ4 — Performance budget (re-stated at review M7/I3).** *Closed 2026-09-13 by wave 0:*
  one cap — **≤ 60 s aggregate hook cost per run** (gated calls × per-call median), with the
  wall-clock share reported as a watch metric (stream `duration_ms` headless, transcript span
  interactive); replay-cache trigger: per-call median > 100 ms or a projection > 60 s. The
  2 % wall-clock limb was dropped (unmeetable on short runs, never binding on long ones). The
  5-second timeout stays a separate correctness floor.

## Question trail (Q)

- **Q1 (2026-09-13)** — where was the failure seen? **User:** "checkout the latest run of
  kinako. EPIC-002 was run on the latest version […] look at the artifact generated for it
  compared to what the current version of mochiko should have." → F10.
- **Q2** — which failure class first: location · file set · section shape? **User:** "both,
  location + file set and section shape is important. I am strugging to control the verbosity.
  we need to have pre tool use base enforcement reminder of schema i think which will be
  preventative. I dont want the files to be generated and then we fix it . token wastage" →
  scope = all three plus a **size envelope** (verbosity is a fourth failure class: EPIC-002's
  spine is 11,713 lines, `implement-log.md` alone 3,653); moment = **before the write**
  (`PreToolUse`), never post-hoc repair — the driver is token cost, not tidiness.
- **Q3** — hard deny or reminder? Three options, (1) recommended. **User:** "as recommeded" →
  D1.
- **Q4** — file-set policy: closed/open? Three options, (1) closed deliverables + open-by-name
  reports recommended. **User:** "as recommeded" → D2; D3 derived alongside.
- **Q5** — conformance semantics package; three options, (1) recommended. **User:** "as
  recommded. also ensure we are focussing on all the workflows of mochiko, not just the ones
  analysed in implement run of EPIC-002. It was pointed as a symptom of a bigger problem" →
  D4 + D5; F13 census run to ground D5.
- **Q6** — size posture tight vs generous; user asked for plain language, re-put. **User:** "as
  recommded. I want the limit be at section level by schema , rather than the entire file. We
  already have template schemas" → D6.
- **Q7** — confirm derived D7–D11 as drafted, or name the one to change? **User:** "confirm
  D7-D11 as drafted" → D7–D11 confidence re-marked; record frozen for cold review.

## Review

**Composition (transport-floor disclosure, 2026-09-13):** one review seat, dispatched via
the `Agent` tool as `subagent_type: mochiko:devils-advocate`, name `cold-review`, carrying
`mochiko:review-brainstorm`, session tier; the seat reported not perceiving the persona (M5)
— recorded as dispatched and as reported, unresolved, not load-bearing. Blind-map two-message
dispatch — message 1 topic + goal line only, no record path, no index; message 2
the frozen record. Message lane fired (lead ↔ seat); topology lane not fired — `record.md` is
the lead's pen only, `review.md` the seat's pen only, no shared write surface. Record frozen
before message 2 (quiesce); fan-in: the map is confirmed arrived before message 2 is sent,
the review file confirmed on disk before it is read.

**Round 1 (2026-09-13):** map 55 angles (in `review.md`); 30 raised → **25 survivors: 5
Critical · 12 Important · 8 Minor**; status `critical-gaps`. Three survivors challenged user
rulings and were put to the user as forks with both costs named; the user ruled **R1b** (deny
ships with the registry — C1/I11), **R2b** (no consumer-local surface — C4), **R3 yes** (Bash
matcher — C5/M6), and **"batch as proposed"** for the lead repairs. Every survivor
dispositioned:

| id | sev | disposition | landed at |
|---|---|---|---|
| C1 | C | ruling — user declined the declare-and-measure order (R1b) | D1 rejected roads · D11 |
| C2 | C | repair | D3 exit-code contract · D10 |
| C3 | C | repair | D4e first-touch amnesty · D6 · D11 wave 5 |
| C4 | C | ruling — no local surface (R2b) | D2 deny text · D3 · D11 wave 1 exit |
| C5 | C | ruling — matcher `Write|Edit|Bash` (R3) | D1c · D8 · D9 |
| I1 | I | fold | D1 rejected roads |
| I2 | I | fold | D1a authoring-time channel · rationale |
| I3 | I | repair | D1 `if` narrowing · D3 cache · D8 · OQ4 |
| I4 | I | repair | D3 `check --hook-json -` |
| I5 | I | repair | D2 · D5 (brainstorm reviews under `reports/`) |
| I6 | I | repair | D5 operating docs location-only |
| I7 | I | repair | D4a counting rule · D6 · D10 watch |
| I8 | I | repair — figures re-counted by the lead | F10 · F13 · D11 |
| I9 | I | repair | F8 · D7e |
| I10 | I | repair | D5 |
| I11 | I | ruling — fork re-put, confidences re-marked | D1 confidence · R1 |
| I12 | I | fold | D8 legs |
| M1 | M | repair | D1b `Read` only |
| M2 | M | fold | D1b repetition accepted · D10 |
| M3 | M | verified — `defer` and PostToolUse `additionalContext` are documented | F8 |
| M4 | M | repair | D4c |
| M5 | M | repair | this block |
| M6 | M | fold | D1c matcher boundary |
| M7 | M | repair | OQ4 |
| M8 | M | fold | D2 |

**Verify round 1 (2026-09-13):** NOT CLEAN — 14 residuals (V1 Critical · V2–V5 Important ·
V6–V14 Minor; in `review.md` § Verify pass), every disposition confirmed landed where the
table says, both fold-introduced doc quotes confirmed. All 14 repaired by the lead: V1 —
D1(a) extended to template-less kinds via `mochiko-cli home`, so R1's basis ("channel (a)
ships beside the gate") holds for every kind; V2 — D4f exemption for kinds whose bounds live
in another constraint home; V3 — amnesty on `Write` over an existing file too; V4 — deny text
routes only report-typed content to `reports/`; V5 — wave 5 re-ordered (violator pass before
the plugin upgrade); V6 — F8 quotes pinned, the unlocated clause marked as the seat's reading;
V7 — `FileChanged` added to rejected roads; V8 — figures propagated; V9 — D9 re-titled, halt
advisory; V10 — D8 title; V11 — wave 3 wording; V12 — Bash contract cases and Bash-call volume;
V13 — D6 confidence re-marked from the answer; V14 — header line. **Delta-check (2026-09-13):**
NOT CLEAN — 3 residuals of 14 (V1 sequencing: no wave landed the template-less authoring arm
→ wave 3 mints the rule, D10 golden added; V2: D6 lacked the D4f exemption pointer → clause
added; V6: the "per-call coverage" clause is verbatim on the pinned page, the hedge was wrong,
and the docs' next sentence says a `Bash` match alone is not enough → clause restored, D1c
matcher widened to `Write|Edit|Bash|PowerShell` as a lead repair inside R3's intent, disclosed
for acceptance). Eleven held. The three repairs are the lead's, closed without a third seat
round — disclosed as lead-repaired, seat-unverified.

**Acceptance (2026-09-13):** user accepted the record as it stands, the `PowerShell` matcher
widening included. **Landed:** `DECISIONS.md` row 2026-09-13 · `BACKLOG.md` section
*Hook-enforced artifact schema build* (waves 0–5 as one bounded item; the *Prose vs. gate
allocation* open design decision closed → trail as ruled by D1/D7; the eval-v2
*Agents-as-schema channel probe* annotated as absorbed into D8) · `ROADMAP.md` Next: folded
into the template-schema/CLI row (cap 7 held) · brainstorms index entry → accepted.
