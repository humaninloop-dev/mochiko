# Constitution → Native Surfaces — Decision Record

> **Status:** **ACCEPTED** (user, 2026-07-18) — pair review, 16 raised → 8 merged survivors → 8/8 dispositioned, verify pass PASS (all folds + 2 propagation fixes confirmed). Eight decisions D1–D8, all `Confident`. This record is the deliverable and stays canonical in place. **Landed:** ROADMAP Key Decisions (constitution-dissolution row) · BACKLOG (D7 substrate-defect item + build follow-ups) · build started same day (setup v3 rewrite). Post-acceptance build-time ruling (user): setup adopts the brainstorm command's agent-team system — recorded in ROADMAP's Decision Trail.

**Topic:** Can the `constitution.md` that `/mochiko:setup` authors be replaced by native Claude Code surfaces — agent skills, path-specific rules, and `CLAUDE.md`?

## Context

- Prompted by a dogfood run of setup v2 in a separate dogfood project (not this repo).
- **Second observed thread:** that run reportedly executed via subagents, not the agent-team form ruled at build time (ROADMAP Decision Trail: team-form substrate). Sent to the fact-checker (what does `setup.md` actually mandate?); whether this session *rules* on it or just logs it is an open scope question.
- Prior art: `setup-constitution-flexibility` (accepted 2026-07-05, built at v0.7.0) — that session ruled the constitution follows declared intent via interrogation → governance-intent → authored constitution. This topic revisits the *artifact form* of its output.
- This repo has no `.mochiko/memory/constitution.md`; index bookkeeping applies via the CLAUDE.md operating manual. Open invariants checked 2026-07-18: pass.

## Problem statement (user, Q1)

The constitution fails on **context economics**, not content:

1. **Not naturally loaded.** `constitution.md` lives outside Claude Code's native context surfaces — unlike `CLAUDE.md`, nothing auto-loads it. Every command (`specify`, etc.) must force it into context by reading the whole file.
2. **Monolithic, no progressive disclosure.** It grows as one giant file; every principle added raises the cost of *every* load. There is no way for a principle to be called upon only when relevant.
3. **Cost to govern more.** Because of 1+2, adding rules has a marginal context cost that punishes exactly the behavior governance wants (more, better-scoped principles).

The proposed direction: let the content live where Claude Code natively loads it — `CLAUDE.md` (always-on), path-specific rules (scope-on-touch), agent skills (load-on-trigger).

## Fact-checker map

_(checker-authored, pasted verbatim 2026-07-18)_

### Fact-checker map (2026-07-18)

**Scope note:** all `file:line` citations are in `/Users/deepeshadmin/Documents/GitHub/mochiko`. Doc claims are quoted from `code.claude.com/docs` (memory + skills pages), fetched today.

---

#### 1. Setup's mandated substrate

- **Setup mandates agent-team form, hard, no fallback.** Frontmatter: "Requires agent teams (CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS); refuses without them" (`plugins/mochiko/commands/setup.md:2`). Body: "Check `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` in the environment before anything else; unset → stop… the **first teammate spawn is the authoritative probe** (the producer): if it fails, stop… **Never proceed teamless — no fallback transport**" (`setup.md:24-29`).
- **Two standing seats, lead-routed, no producer↔validator messaging.** "producer — `mochiko:principal-architect`, one standing seat" (`setup.md:50`); "validator — `mochiko:validator`, spawned **cold at first validation**, never in the room before that and never messaged by the producer… the fix list flows through you" (`setup.md:58-63`). Contract: "structurally separated (validator cold-spawned, fix list lead-routed, no producer↔validator messaging)" (`setup.md:139-141`). This is teammate/`SendMessage` vocabulary ("spawn", "standing seat", "in the room"), not `Task()`-subagent vocabulary.
- **ROADMAP's team-form ruling (2026-07-16, `setup` v2 row + Decision Trail).** "**Substrate: team-form by user ruling (2026-07-16)** — the second agent-team command after brainstorm: standing producer seat (analysis→authoring context continuity), validator spawned cold at first validation, fix list lead-routed (no producer↔validator messaging), hard-require teams (`Contested` extended)" (`ROADMAP.md:75`). Rationale narrows the payoff: "what team-form buys setup is **persistent context**… **not** teammate↔teammate messaging (explicitly unused: the fix list routes through the lead)… The item stays open until a team-form setup run is observed" (`ROADMAP.md:186`).
- **On the second thread (a separate-project dogfood reportedly run via subagents):** un-verifiable from this repo — no run transcript for another project exists here. What this repo fixes is unambiguous: subagent execution would contradict the mandated form (`setup.md:29` forbids teamless fallback) and is the exact observation the still-open BACKLOG substrate item is waiting on (`ROADMAP.md:75,186`).

---

#### 2. What the constitution artifact carries

**Template sections** (`plugins/mochiko/templates/constitution-template.md`), universal core: SYNC IMPACT REPORT header (`:12-42`); Overview; **Governance Tier** + **Waivers** table (category / waiving tier / revisit trigger / trace) (`:57-79`); **Core Principles** — each with Enforcement / Testability / Rationale / `**Trace**: GI-XXX` (`:83-109`); Technology Stack (`:111-125`); Quality Gates (`:127-144`); a modules-attach point (`:146-149`); Governance (approvers, amendment process, version policy, exception registry, compliance review) (`:151-201`); **CLAUDE.md Synchronization** (mapping table + process + enforcement) (`:203-244`); Version/Ratified footer (`:248`).

**Attachable modules** (`plugins/mochiko/templates/constitution-modules/`): `layer-rules.md`, `release-gates.md`, `evolution-notes.md`, `knowledge-management.md`. Each carries its own validator checklist fragment; validator checks core + exactly the selected modules (`authoring-constitution/SKILL.md:266-283`; `validation-constitution/SKILL.md:41-46`).

**Enforcement carriers the artifact names** (all external to Claude Code's own loading): CI automation, code review, tooling/pre-commit hooks, periodic audit (`authoring-constitution/SKILL.md:130-138`); branch protection / PR approval (`constitution-template.md:144`); and the **CLAUDE.md sync** carrier — the constitution mandates CLAUDE.md replicate principles summary, tech stack ("MUST match exactly"), quality gates ("MUST match exactly"), governance (`constitution-template.md:217-227`). That sync's *operational* half is **stub-backed / unported**: `syncing-claude-md` "**Not ported**… referenced as a planned stub, not a live mount" (`authoring-constitution/SKILL.md:368-370`).

**Downstream consumers of `.mochiko/memory/constitution.md`** (grep over `plugins/mochiko/`):
- **Producer:** `setup.md` authors it (`setup.md:8,52`); `validator.md:10` grades it; `analysis-codebase` checks for its presence (`skills/analysis-codebase/references/CONTEXT-GATHERING.md:47`).
- **Read as "governing context" by five downstream commands, none blocking:** `specify.md:27` ("Constitution prerequisite… carry its constraints into the producer's brief"), `plan.md:28`, `tasks.md:30`, `slice.md:30`, `implement.md:30` (all "Read… carry its principles into the producer's brief… Missing → surface it… not a blocking gate — do not auto-resolve"), and `brainstorm.md:21` ("Read… if present and carry it as governing context — never a blocking gate").
- **Templates referencing it:** `agent-dispatch.md:43-44` (example input/output), `techanalyst-report-template.md:64` and `taskarchitect-report-template.md:77` (a "Constitution Alignment" report section), `patterns-technical-decisions/references/DECISION-RECORD.md:145`.
- **Consumption mode in every case is an explicit `Read` by a mochiko command that injects the text into a producer brief.** Nothing consumes it via native Claude Code loading — `.mochiko/memory/constitution.md` is not a native memory path (see §3), so absent a mochiko command reading it, no agent session sees it. This matches the brainstorm record's own grep finding: "no downstream workflow reads the floor… no downstream mechanical dependency… none exists today" (`setup-constitution-flexibility/record.md:49,137` — decision D1 / finding A4) *[erratum applied — see Review]*.

---

#### 3. What native Claude Code surfaces actually exist (per official docs)

**(a) CLAUDE.md / memory files** — `code.claude.com/docs/en/memory`:
- Scopes, broadest→most specific: managed policy → user (`~/.claude/CLAUDE.md`) → project (`./CLAUDE.md` or `./.claude/CLAUDE.md`) → local (`./CLAUDE.local.md`).
- Load semantics: "CLAUDE.md and CLAUDE.local.md files in the directory hierarchy above the working directory are loaded in full at launch. Files in subdirectories load on demand when Claude reads files in those directories." Subdirectory files "are included when Claude reads files in those subdirectories."
- **It is context, not enforcement:** "Both are loaded at the start of every conversation. Claude treats them as context, not enforced configuration. To block an action regardless of what Claude decides, use a PreToolUse hook instead." And "CLAUDE.md content is delivered as a user message after the system prompt… there's no guarantee of strict compliance."
- Size: "target under 200 lines per CLAUDE.md file… This limit applies only to MEMORY.md. CLAUDE.md files are loaded in full regardless of length, though shorter files produce better adherence."
- `@path` imports load at launch (max depth four hops); "imported files still load and enter the context window at launch." Block-level HTML comments are stripped before injection.
- Auto memory is a separate system at `~/.claude/projects/<project>/memory/`; MEMORY.md's "first 200 lines… or the first 25KB" load every session.

**(b) Path-specific rules — the feature EXISTS.** `code.claude.com/docs/en/memory` §"Organize rules with `.claude/rules/`": "Place markdown files in your project's `.claude/rules/` directory… All `.md` files are discovered recursively." Path scoping: "Rules can be scoped to specific files using YAML frontmatter with the `paths` field. These conditional rules only apply when Claude is working with files matching the specified patterns." Priority: "Rules without a `paths` field are loaded unconditionally… loaded at launch with the same priority as `.claude/CLAUDE.md`." Trigger: "Path-scoped rules trigger when Claude reads files matching the pattern, not on every tool use." User-level `~/.claude/rules/` also exists; symlink-sharing supported. (Third-party corroboration: introduced ~v2.0.64.)

**(c) Agent skills** — `code.claude.com/docs/en/skills`:
- Frontmatter fields (all optional; only `description` recommended): `name`, `description`, `when_to_use`, `disable-model-invocation`, `user-invocable`, `allowed-tools`, `disallowed-tools`, `paths`, `shell`, `arguments`.
- **Description length limit:** "the combined `description` and `when_to_use` text is truncated at 1,536 characters in the skill listing to reduce context usage." Body: "Keep SKILL.md under 500 lines. Move detailed reference material to separate files."
- Loading / trigger: "In a regular session, skill descriptions are loaded into context so Claude knows what's available, but full skill content only loads when invoked." `disable-model-invocation: true` → "Description not in context, full skill loads when you invoke"; `user-invocable: false` → Claude-only, description stays in context.
- Skills can be **path-scoped**: the `paths` field "limit[s] when this skill is activated… Uses the same format as path-specific rules."
- Persistence: once invoked, "the rendered SKILL.md content enters the conversation… and stays there for the rest of the session" (recurring token cost). Locations: personal (`~/.claude/skills/`), project (`.claude/skills/`, incl. nested/on-demand), plugin.

---

#### 4. Overlaps and gaps, both directions (facts only)

**What native surfaces already do that the constitution duplicates:**
- CLAUDE.md's stated purpose — "Coding standards, workflows, project architecture" — overlaps the constitution's Technology Stack, Quality Gates, and Principles-Summary. The constitution's own sync table concedes this: it requires CLAUDE.md to carry those sections, tech stack + quality gates "MUST match exactly" (`constitution-template.md:217-227`). Today that copy is manual/unported (`authoring-constitution/SKILL.md:368-370`).
- `.claude/rules/` with `paths` frontmatter does path-scoped rule injection natively — the mechanism the type/tier modules of `setup-constitution-flexibility/record.md` decision D8 (`layer-rules` for backend layers, a planned frontend/accessibility shelf) reach for. *[erratum applied — see Review]* Docs' worked example ("All API endpoints must include input validation… standard error response format… OpenAPI documentation comments") is the same shape as `layer-rules`/API-contract principles.
- Skills carry repeatable procedure natively (the "how"); mochiko already uses them for that. Constitution principles are "what/governance," not procedure — limited overlap.

**What the constitution does that no native surface does (per docs):**
- **Three-Part structure (Enforcement / Testability / Rationale) with trace-IDs** to a ratified `governance-intent.md` — no native surface carries enforcement-metadata or provenance stamps; CLAUDE.md/rules/skills are plain instruction text.
- **Governance metadata:** tier declaration, waiver records with revisit triggers, amendment process, semver version policy, approvers, exception registry (`constitution-template.md:57-201`). Native memory has no version/amendment/waiver/tier concept.
- **Binds parties beyond the Claude session:** the constitution's enforcement carriers are CI, branch protection, human code review, periodic audit — it governs humans + pipelines, not only an agent's in-session behavior. Native CLAUDE.md/rules are explicitly "context, not enforced configuration" scoped to what Claude decides in-session; hard native enforcement is hooks/permissions only. (Symmetric fact: the constitution is *also* not enforced by Claude Code — its teeth are external CI/review, same as CLAUDE.md relies on external mechanisms.)
- **Not natively loaded at all:** `.mochiko/memory/constitution.md` sits outside every native memory path (`./CLAUDE.md`, `./.claude/CLAUDE.md`, `~/.claude/CLAUDE.md`, `.claude/rules/`, auto-memory dir). It reaches a session only when a mochiko command Reads it into a brief, or via the unported CLAUDE.md sync — i.e. its distribution is entirely mochiko-command-mediated, not native.
- **Author→cold-validator→human-acceptance loop with trace summary** (`setup.md`, default-FAIL, GI trace-ID cross-check) — a production process, not a file format; no native surface has an authoring/validation pipeline.

**Load-order / limits facts that cut across the decision:** CLAUDE.md loads in full every session at all ancestor levels (200-line *target*, not cap); `.claude/rules/` non-`paths` files load at launch at CLAUDE.md priority, `paths` files only on matching-file reads; skill descriptions (≤1,536 chars) always in context unless `disable-model-invocation`, bodies on invoke and then persistent. A constitution's content moved into any of these would incur those load/priority/limit behaviors, which the single `.mochiko/memory/constitution.md` file (read on demand into a brief) currently does not.

**Sources:** [Claude Code — Memory](https://code.claude.com/docs/en/memory), [Claude Code — Skills](https://code.claude.com/docs/en/skills), [.claude directory](https://code.claude.com/docs/en/claude-directory).

### Fact-check addendum: what spawned agents inherit (2026-07-18)

_(checker-authored, pasted verbatim)_

Docs-grounded, per surface and per spawn path. Two distinct spawn paths behave differently; I keep them separate.

**Threshold fact that governs the whole question:** the constitution lives at `.mochiko/memory/constitution.md`, which is **not** a native surface (not CLAUDE.md, not `.claude/rules/`). Nothing below auto-loads it on either path. Native inheritance only helps if the constitution content is **moved to** CLAUDE.md or `.claude/rules/` (or pulled into CLAUDE.md via an `@.mochiko/memory/constitution.md` import — "@path imports load at launch," memory page). As it stands today, the read-and-inject step is the *only* thing delivering it to a producer.

**(a) Subagents (Task/Agent tool)** — sub-agents page §"What loads at startup":
- **CLAUDE.md — YES.** "**CLAUDE.md files**: every level of the CLAUDE.md hierarchy the main conversation loads, including `~/.claude/CLAUDE.md`, project rules, `CLAUDE.local.md`, and managed policy files. The built-in Explore and Plan agents skip this." (sub-agents.md, "What loads at startup").
- **`.claude/rules/` — YES (named as "project rules").** Same sentence above lists "project rules" among what loads; the memory page titles `.claude/rules/` "project rules." `paths`-scoped rules still follow their normal trigger — "Path-scoped rules trigger when Claude reads files matching the pattern, not on every tool use" (memory page) — so inside a subagent a scoped rule reaches context only if that subagent reads a matching file; unconditional rules load at its startup.
- **Skills listing — NO by default.** The startup list does **not** include an always-on skills listing; it includes only "**Preloaded skills**: full content of any skill named in the agent's `skills` field. Built-in agents don't preload skills." The subagent "doesn't see your conversation history, the skills you've already invoked, or the files Claude has already read." Skills remain reachable but only actively: "without it, the subagent can still discover and invoke project, user, and plugin skills through the Skill tool during execution." So a subagent does not get the main session's always-in-context skill descriptions; it must preload (`skills:` frontmatter) or invoke via the Skill tool.
- **Auto memory — NO.** "The main conversation's auto memory isn't loaded into subagents" (memory page).

**(b) Agent-team teammates** — agent-teams page §"Context and communication" + best practices + limitations:
- **CLAUDE.md — YES.** "When spawned, a teammate loads the same project context as a regular session: CLAUDE.md, MCP servers, and skills. It also receives the spawn prompt from the lead. The lead's conversation history does not carry over." Reinforced in Limitations: "**CLAUDE.md works normally**: teammates read CLAUDE.md files from their working directory."
- **Skills listing — YES (regular-session behavior).** Same sentence: teammates load "the same project context as a regular session: CLAUDE.md, MCP servers, and skills." Confirmed against the subagent-definition path: "The `skills` and `mcpServers` frontmatter fields in a subagent definition are not applied when that definition runs as a teammate. **Teammates load skills and MCP servers from your project and user settings, the same as a regular session.**" This is the key divergence from subagents: a teammate gets the always-in-context skill listing; a plain subagent does not.
- **`.claude/rules/` — NOT NAMED; covered only by "same project context as a regular session."** The agent-teams page enumerates "CLAUDE.md, MCP servers, and skills" and says teammates load "the same project context as a regular session." A regular session loads `.claude/rules/`, so this implies rules load for teammates too — but the agent-teams page does **not** name `.claude/rules/` explicitly (unlike the sub-agents page, which does name "project rules"). Docs are silent on the exact case; I won't assert it as explicit.
- **Auto memory:** the agent-teams page doesn't address the lead's auto memory reaching teammates; it only states conversation history does not carry over.

**Bottom line for the "drop the read-and-inject machinery" question:**
- The machinery is droppable **only if constitution content moves to a native surface.** Relocated into project `CLAUDE.md` (or `@`-imported into it): reaches teammates AND custom subagents at spawn, per the quotes above. Relocated into `.claude/rules/`: reaches subagents explicitly ("project rules"), reaches teammates by the "same-as-regular-session" statement (not named). A `paths`-scoped rule reaches a producer only when the producer reads a matching file — so it would not be present at brief time for a producer that authors before reading matching files.
- If the constitution **stays** at `.mochiko/memory/constitution.md`, neither path auto-loads it; the commands still need to read-and-inject (or add a CLAUDE.md `@import`).
- Independent of relocation, one surface does **not** free-ride on either path: skill *descriptions* are absent from plain subagents by default (present for teammates). That's orthogonal to the constitution but relevant if any downstream command relies on model-invoked skills firing inside a subagent producer.

**Sources:** [sub-agents](https://code.claude.com/docs/en/sub-agents), [agent-teams](https://code.claude.com/docs/en/agent-teams), [memory](https://code.claude.com/docs/en/memory), [skills](https://code.claude.com/docs/en/skills).

### Fact-check addendum: HTML comment stripping in `.claude/rules/` files (2026-07-18)

_(checker-authored, pasted verbatim)_

**Answer: the docs are silent on comment handling for rule files.** The stripping guarantee is stated by name only for CLAUDE.md (and, separately, for the MEMORY.md auto-memory index). No official page extends it to `.claude/rules/`.

- **CLAUDE.md — explicit (scoped to "CLAUDE.md files").** Memory page, §"How CLAUDE.md files load": "Block-level HTML comments (`<!-- maintainer notes -->`) in CLAUDE.md files are stripped before the content is injected into Claude's context. Use them to leave notes for human maintainers without spending context tokens on them. Comments inside code blocks are preserved. When you open a CLAUDE.md file directly with the Read tool, comments remain visible." The subject is "CLAUDE.md files" — the sentence names no other surface.
- **MEMORY.md — explicit, but a different mechanism/scope.** Same page, §"How it works": "YAML frontmatter and block-level HTML comments are stripped before the index is loaded, so they don't count toward the limits." Scoped to the auto-memory `MEMORY.md` index and framed around its 200-line/25KB limit, not `.claude/rules/`.
- **`.claude/rules/` — silent, both pages checked.** I read the memory page's entire rules section (Organize rules / Set up rules / Path-specific rules / Share rules via symlinks / User-level rules) — none of it mentions HTML comment stripping or preservation. The dedicated `.claude directory` page (`code.claude.com/docs/en/claude-directory`) describes rules loading ("A rule without `paths:` frontmatter loads at session start like CLAUDE.md; a rule with `paths:` loads only when Claude reads a matching file") but likewise says nothing about comment handling.

**Caveat against inferring parity:** the `.claude directory` page says a no-`paths` rule "loads at session start **like CLAUDE.md**" — a *loading* analogy only. It does not state that rule files inherit CLAUDE.md's comment-stripping. Since Claude Code documents this behavior explicitly for CLAUDE.md and MEMORY.md but not for rules, whether rule-file HTML comments are stripped, preserved, or counted toward context is **undocumented** — treat it as unknown rather than assumed-stripped. (An `InstructionsLoaded` hook or `/context` check on a rule file carrying a comment would settle it empirically, but that's a test to run, not a doc fact.)

**Sources:** [memory](https://code.claude.com/docs/en/memory), [.claude directory](https://code.claude.com/docs/en/claude-directory).

## Decisions

_(confidence marks: Confident / Assumed / Contested / Unsure / Deferred)_

### D1 — Complete dissolution; CLAUDE.md is the thin ratified core · `Confident`

**Statement:** `.mochiko/memory/constitution.md` ceases to exist. `CLAUDE.md` takes over the thin-ratified-core role: it carries the ratified index of principles, carries universal principles' enforceable content directly as short governance-region lines *(verify-pass propagation C2, per the M3 relocation)*, and points to where the rest lives — agent skills and `paths`-scoped `.claude/rules/` files. Content loads via native disclosure tiers (CLAUDE.md always-on, `paths`-scoped rules on matching-file reads, skills on trigger).

**Rationale:** The constitution's failure is context economics, not content (Problem statement). The lead's recommendation was a thin ratified core as a *separate* small manifest (option B); the user ruled the stronger form — the manifest role belongs on the surface Claude Code natively always loads, so the core needs no mochiko-command-mediated Read at all. Consistent with the map: nothing native loads `.mochiko/memory/constitution.md` (§2), while CLAUDE.md loads in full every session and `.claude/rules/` provides native path-scoped injection (§3).

> *Rationale completed at review (M6/F1, lead formulation): the "needs no mochiko-command-mediated Read" reason alone is defeated by the map's @import fact — an @-imported separate file also auto-loads at launch. The decisive reason is **single-source ownership**: the core lives ON CLAUDE.md as one human-visible surface, not a transcluded second file.*

**Ruled:** user, Q2 (2026-07-18). Open follow-ons at time of ruling: where the governance metadata (version, tier, waivers, amendment, trace-IDs) lands; content→surface mapping; what happens to setup's authoring/validation loop and the five downstream constitution-reads.

### D2 — Governance layer splits by audience · `Confident`

**Statement** *(amended at review — M2+M3 user-ruled, M8 lead formulation)***:** `CLAUDE.md` carries what every session needs, all short-form: the ratified stamp (version + date + tier, one line), the principle index with pointers, the universal principles as short imperative lines, the technology stack, and the quality-gates summary. Module detail (knowledge-management operating manual, release-gate detail) sits behind pointers, on-demand. The governance ledger — waiver table with revisit triggers, amendment process, exception registry — lives in `.mochiko/memory/` beside `governance-intent.md`, read only by setup/amend runs and the validator. **Three-Part metadata and GI trace-IDs live ledger-primary, keyed by trace-ID**; HTML trace comments are an optional optimization in CLAUDE.md only (comment-stripping documented there; for rules files checked 2026-07-18: **undocumented**).

> *Original form read "only the stamp and the index"; superseded at the survivor gate by the probe-driven retreat of universal principles into CLAUDE.md (M3) and the always-on budget reconciliation (M2). Trace sentence tightened ledger-primary (M8).*

**Rationale:** The same context-economics logic that motivated D1, applied to governance itself: always-on surfaces carry only what every session acts on; rare governance operations read their ledger on demand. Rejected: all-in-CLAUDE.md (re-imports the verbosity problem into the always-on surface); dissolve-the-ceremony (deletes the amendment/ratification story the setup-v2 session ruled central, and amend runs lose their diff-base).

**Ruled:** user, Q3 (2026-07-18) — adopted the lead's recommendation.

### D3 — Content→surface mapping · `Confident`

**Statement:** Every constitution-template section gets the following destination (user kept the proposed table whole):

| Constitution content today | New home | Reasoning / caveat |
|---|---|---|
| Ratified stamp (version, date, tier) | `CLAUDE.md` — one line | D2 |
| Principle index (one line each + pointer) | `CLAUDE.md` | D1 — the thin core |
| Universal principles (every session) | `CLAUDE.md` — short imperative lines in the governance region | *Amended at review (M3, user-ruled):* was `.claude/rules/` unconditional files; the empirical probe showed rules delivery to spawned producers can't be assumed, while CLAUDE.md is doc-confirmed for both spawn paths |
| Scope-bound principles (layers, API standards, test discipline, frontend shelf) | `.claude/rules/` — **`paths`-scoped**, one file per concern | Docs' worked example is this shape. Caveat: fires on matching-file *read* — pure-authoring producers may need the brief to name it |
| Procedure-shaped standards (how-tos) | Skills | Existing mochiko convention; dispatch briefs keep naming them |
| Technology stack | `CLAUDE.md` | Docs name this as CLAUDE.md's purpose |
| Quality gates | `CLAUDE.md` summary; teeth stay CI/hooks | The teeth were never in the file |
| Three-Part metadata (Enforcement/Testability/Rationale) + GI trace | `.mochiko/memory/` ledger keyed by trace-ID (fallback made primary — rules-file comment stripping is undocumented); HTML comments usable in CLAUDE.md (stripping documented there); empirical stripping test optional at build time | Fact-check addendum 2 |
| Governance ledger (waivers, amendment, exceptions) | `.mochiko/memory/` | D2 |
| SYNC IMPACT report + CLAUDE.md Synchronization section | **Dies** | Nothing left to sync — the copy problem dissolves with the artifact; the stub-backed `syncing-claude-md` dependency disappears |
| Module: layer-rules | `paths`-scoped rules | Direct fit |
| Module: knowledge-management | Pointer from `CLAUDE.md`; operating-manual detail on-demand + command carriers unchanged | *Amended at review (M2, user-ruled):* detail dropped behind a pointer to protect the always-on budget |
| Module: release-gates | Pointer from `CLAUDE.md`; detail on-demand | *Amended at review (M2, user-ruled)* |
| Module: evolution-notes | Stays a `.mochiko/memory/` artifact; pointer from `CLAUDE.md` | On-demand brownfield material |

**Rationale:** Each destination follows the surface's native disclosure tier (always-on / scope-on-touch / on-trigger) per the map §3, with the two spawn-path caveats from the inheritance addendum acknowledged in-row.

> *Review annotation (M2/F4, user-ruled):* the marginal-context-cost win (Problem #3) holds **fully only for scope-bound principles**. Universal principles remain always-on — now as CLAUDE.md lines — so their wins are native delivery (Problem #1) and one-touch modular amendment, not marginal load. The always-on budget is bounded by keeping every CLAUDE.md governance entry short-form with detail behind pointers.

**Ruled:** user, Q4 (2026-07-18) — kept the lead's proposed table whole, including the flagged rows (universal principles as rules files; sync-section death; metadata to ledger after the stripping check came back undocumented).

### D4 — Setup loop survives with a distributed target; trace is the spine · `Confident`

**Statement:** Interrogation and `governance-intent.md` ratification are untouched. The producer authors the full surface set (CLAUDE.md core, rules files, skill pointers, ledger). The cold validator grades the *set* using the trace summary as its instrument: every ratified GI element lands on **one primary enforceable home** (CLAUDE.md governance-region line, rule file, or skill) **with its required companion entries present and complete** (index line, ledger metadata); every surface element traces back to a GI element; the CLAUDE.md index points only at files that exist. *(Amended at review, M4, lead formulation: "exactly one surface" restated — D3 deliberately decomposes one principle across index + home + ledger; the validator grades the full decomposition, which also catches missing companions.)* The CLAUDE.md version stamp is the governance release version. Human acceptance gate unchanged.

**Rationale:** The gradable unit stops being "one file" and becomes "the trace closes both ways" — the GI trace machinery setup v2 already built is exactly the cross-check a distributed artifact needs. Rejected: grade-the-core-only (ships unaudited governance surfaces); drop-the-validator (removes the independent grade loop-discipline treats as structural — dissolution changed the container, not the content's worth grading).

**Ruled:** user, Q5 (2026-07-18) — adopted the lead's recommendation.

> **Streak note (lead, logged where it happened):** D2, D3, D4 are three consecutive unelaborated adoptions of lead recommendations. Per the questioning discipline this was flagged to the user after D4; the next fork (Q6) was posed steelmanned, with no marked recommendation. Outcome: the user engaged and ruled actively at D5 ("option B actually not A") — streak broken.

### D5 — Downstream briefs: native loading plus a one-line governance pointer · `Confident`

**Statement:** The five downstream commands (`specify`, `plan`, `tasks`, `slice`, `implement`) delete their constitution read-and-inject step. Native loading carries the governance content to producers via CLAUDE.md at spawn — the surface doc-confirmed for both spawn paths; scope-bound rules arrive via the obligated read, since the empirical probe showed `.claude/rules/` delivery to spawned producers cannot be assumed. *(Verify-pass propagation C1: former "+ unconditional rules at spawn" clause struck — stale after M3 and contradicted by the probe.)* Each command's dispatch brief carries a one-line **obligated read**: it names the governance surfaces relevant to that producer's work (scoped rule files it will not trigger by reading, skills it should lean on) and instructs the producer to Read the named rule files before authoring — no lead-side file-reads, no text duplication. *(Amended at review, M3/F8, user-ruled: pointer upgraded from awareness to obligated read.)*

**Rationale:** Closes the two documented spawn-path holes — `paths`-scoped rules fire on matching-file *read* (a from-scratch author never triggers them at exactly the moment governance matters), and plain subagents don't see skill descriptions — using the dispatch-briefing convention mochiko already has. Rejected: trust-native-entirely (its failure mode is silent; the sentence it saves is not a subsystem).

**Ruled:** user, Q6 (2026-07-18) — posed without a marked recommendation (streak protocol); user chose B over A explicitly.

### D6 — No backward compatibility; no migration machinery · `Confident`

**Statement:** No migration path, no amend-time offer, no legacy read-and-inject support. Mochiko is in dogfooding stage with no external adopters; existing dogfood constitutions are simply superseded — re-run setup under the new form (or discard). The D5 machinery deletion lands fully and immediately.

**Rationale:** User's own grounds, overriding the lead's amend-time-offer recommendation: backward compatibility is a cost paid for adopters, and there are none. All three offered options (amend-time offer / dual-form coexistence / standalone migration command) built machinery for a constituency that doesn't exist. Also dissolves by the same logic: the amend-time module-offer concern for old constitutions.

**Ruled:** user, Q7 (2026-07-18) — rejected all offered options with a simpler stage-grounded ruling.

> *Amended at review (M7, user-ruled): new-form setup deletes a superseded `.mochiko/memory/constitution.md` on sight — no orphaned second governance source.*

### D7 — Subagents-vs-teams observation: defect thread, out of scope · `Confident`

**Statement:** The dogfood run's subagent execution is dispositioned as a **defect observation, out of this session's design scope**. It lands as a BACKLOG item at acceptance: reproduce in the dogfood project; determine whether `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` was unset with a silent fallback (command-discipline failure — `setup.md:24-29` mandates refusal, not fallback) or set and ignored (instruction non-compliance); harden the refusal if needed. The ROADMAP team-form substrate item stays open — the observed run was not team-form, so the observation it waits on has still not occurred. Nothing in D1–D6 alters setup's substrate mandate.

**Rationale:** The defect is real and fact-grounded (map §1), but the evidence — the dogfood run's transcript — lives outside this repo; ruling on a fix here would be diagnosis without the patient. Dropping it would forfeit the exact observation the open ROADMAP item is waiting on.

**Ruled:** user, Q8 (2026-07-18).

### D8 — Setup owns a marked governance region in CLAUDE.md; trace summary is the validator's manifest · `Confident`

**Statement:** Setup writes its CLAUDE.md content inside a **marked, setup-owned governance region**, idempotently regenerated on setup/amend runs; content outside the region is never touched. For that region only, this deliberately replaces setup's never-overwrite floor — a governance region must *update* on amend. The trace summary is the validator's **complete grading manifest**, bounding what it grades across the CLAUDE.md region, the rules files, and the ledger.

**Rationale:** Dissolution makes setup a co-author of a human-edited file (review finding F2): regeneration must be surgical or D6's "just re-run setup" clobbers user content, and the validator — which previously graded one bounded file — needs a manifest to separate setup-authored governance from user content across N surfaces.

**Ruled:** user, review survivor M6 (2026-07-18) — adopted at the survivor gate.

## Corrections & reversals

- **Review-stage amendments (2026-07-18), all user-authorized at the survivor gate:** D1 rationale completed (M6/F1); D2 statement re-scoped and made ledger-primary (M2+M3, M8); D3 three rows re-routed + Problem-#3 annotation (M2, M3); D4 invariant restated (M4); D5 pointer upgraded to obligated read (M3/F8); D6 stale-file deletion added (M7); D8 added (M6). Each amendment is annotated in place at its decision; dispositions in the Review section.
- **Checker erratum applied to the map's self-citations** (M1) — the bare `record.md` references point into `setup-constitution-flexibility/record.md`; inline citations re-pointed with erratum tags, checker-verified line numbers unchanged.

## Review

**Sizing:** user ruled **pair** at the convergence gate (2026-07-18). Reviewers: `reviewer-dq` (decision-quality lens) and `reviewer-ri` (record-integrity lens; ran the map sample-audit — external file-facts confirmed true, setup.md line numbers drifted 1–3). Cold reads independent (counts reported before introduction: 9 and 7); one-shot four-message cross-examination, closed clean, no unresolved counterpart objections.

**Combined tally (lead's merge):** 16 raised → 2 owner-withdrawn (dq-F5 ceremony-vs-stage; ri-RI-7 provenance) → 14 survived → deduplicated cross-set: **8 merged survivors** (6 Important, 2 Minor). Both reviewers independently recommend **needs-revision**; all items session-resolvable.

| # | Merged from | Sev | Target | Finding (compressed) | Route |
|---|---|---|---|---|---|
| M1 | RI-1 | Important | Fact map | Map self-citations (`record.md:49,137`, "D1/A4", "D8") don't resolve in this record — self-containment fitness item blocked; external facts confirmed true | fact-checker (erratum) — dispatched |
| M2 | RI-2 + F4 | Important | D2/D3 | D2 "CLAUDE.md carries *only* stamp+index" vs D3 routing tech stack, gates, KM manual, release gates into CLAUDE.md; always-on load never sized; Problem #3 (marginal cost of governing more) solved only for scope-bound principles | user |
| M3 | F3 + RI-5 + F8 | Important (→Critical if probe fails) | D3/D5 | Rules-loading for **teammates** undocumented (checker won't assert); D5 pointer covers scoped rules + skills but not unconditional rules; pointer-read is voluntary | empirical probe (running) + user |
| M4 | F6 + RI-4 | Important | D4 | "Every GI element lands on exactly one surface" contradicts D3's deliberate index+rule+ledger spread; literal reading flags correct authoring, loose reading misses missing companions | lead formulation fix (fold) |
| M5 | F7 + RI-6 | Important | D2/D3/D4 marks | Three `Confident` marks vs the record's own streak note (unelaborated adoptions); fitness reserves `Assumed` for these | user (re-confirm or re-mark) |
| M6 | F2 + F1 | Important | D1/D4/D6 | Setup becomes co-author of human-edited CLAUDE.md: needs governance-region ownership markers + idempotent regeneration (never-overwrite is the wrong discipline for a region that must update on amend); trace summary should be the validator's complete grading manifest; D1's written rationale defeated by the @import fact (decisive single-source reason unstated) | user (new decision) + lead rationale fix |
| M7 | F9 | Minor | D6 | No hygiene step removes a superseded `constitution.md` — orphaned second governance source | user (quick ruling) |
| M8 | RI-3 | Minor | D2 | Trace-comment main clause overstates; ledger-primary parenthetical should be the main clause | lead formulation fix (fold) |

### Checker erratum — map §2/§4 self-citations (M1 resolution, pasted verbatim 2026-07-18)

Confirmed: the bare `record.md` in my map's §2 closing bullet and the "D1/A4"/"D8" references point into **`.mochiko/brainstorms/setup-constitution-flexibility/record.md`** (the prior, 2026-07-05 accepted session), **not** this session's `constitution-native-surfaces/record.md`. The reviewer is right that a bare `record.md` is ambiguous once the map is pasted inside this session's record.

The line numbers are correct **for that prior file** — verified by re-reading it just now:
- `setup-constitution-flexibility/record.md:49` — D1's rejected-alternatives line: "…its value is the governance contract with the user, **not a downstream mechanical dependency — none exists today.** *Rationale softened during review, A4.*"
- `setup-constitution-flexibility/record.md:137` — finding A4 (Minor): "**A4 — D1 rationale leg unsupported:** **no downstream workflow reads the floor** (grep-verified)…"
- `D1` = that record's line 48 decision "The floor survives, scope-tiered"; `A4` = the Minor review finding at line 137; `D8` (referenced in my §4) = that record's "Template layer: universal core + attachable modules" decision. All three live in `setup-constitution-flexibility/record.md`.

My quoted fragment "no downstream workflow reads the floor… no downstream mechanical dependency… none exists today" is a splice of that file's line 137 and line 49; both halves are accurate to it.

**Corrected citation (paste-ready):** replace `(record.md:49,137, decision D1/A4)` with `(setup-constitution-flexibility/record.md:49,137 — decision D1 / finding A4)`, and the §4 `D8` reference with `setup-constitution-flexibility/record.md, decision D8`. No factual claim changes; only the file the citation names. My external file-facts (the `plugins/mochiko/` citations) are unaffected — they already carry full paths.

### Empirical probe — rules delivery to spawned agents (lead-run, 2026-07-18)

Method: wrote a marker file `.claude/rules/governance-probe.md` (no `paths` frontmatter → unconditional), then spawned a fresh agent via the same spawn path this session's teammates use, instructed to answer tool-free from loaded context only. **Result: NO — the marker was absent from the spawned agent's context.** Confound stated plainly: the rule file was created mid-session, so the negative can mean spawned agents don't receive project rules on this path, or that rules are snapshotted at main-session start; a fresh-session test would separate the two. Either way, for the environment mochiko actually runs in: **native `.claude/rules/` delivery to spawned producers cannot currently be assumed** — reviewer-dq's F3 escalation condition is met.

**Dispositions:** _(logged per survivor as resolved / user-ruled / recorded-open)_

- **Verify pass (reviewer-ri):** 6/8 folds verified clean on first pass (M1, M4, M5, M6, M7, M8 — evidence quoted); M2/M3 landed at target but left two propagation misses, both lead-pen fixes of the already-ruled relocation: **C1** — D5's "+ unconditional rules at spawn" struck (stale post-M3, contradicted the probe); **C2** — D1's statement now names CLAUDE.md governance-region lines as universal principles' direct home. Both applied 2026-07-18; reviewer-ri quote-checked and **closed both** — "no new inconsistency introduced; nothing else reopened. My verify pass now clears clean… All 8 folds now verified, plus these two propagation fixes." Non-blocking observations (D8 numbering collision with the prior record's D8 — disambiguated by full paths; D1 correction-in-place style; index-line redundancy for CLAUDE.md-resident principles) noted, not gating.
- **M1 — resolved.** Checker erratum obtained and pasted above; the map's self-citations point into `setup-constitution-flexibility/record.md` with line numbers verified against that file; no factual claim changed. Citation re-point applied at fold time.
- **M7 — user-ruled (2026-07-18).** New-form setup deletes a superseded `.mochiko/memory/constitution.md` on sight. Amends D6. Fold at freeze-lift.
- **M4 — resolved (lead formulation, accepted without argument).** D4's invariant restated: one primary enforceable home + required companion entries; validator grades the full decomposition. Fold at freeze-lift.
- **M8 — resolved (lead formulation, accepted without argument).** D2's trace sentence made ledger-primary in the main clause. Fold at freeze-lift.
- **M5 — user-ruled (2026-07-18): D2/D3/D4 actively re-confirmed** in their amended form (post M2+M3 folds), item-by-item at the survivor gate. Marks stay `Confident`; this disposition line is the recorded re-confirmation the fitness standard requires.
- **M6 — user-ruled (2026-07-18): adopted as new decision D8.** Marked, setup-owned governance region in CLAUDE.md, idempotently regenerated on amend; user content outside the region never touched (deliberately replaces the never-overwrite floor for that region only); trace summary = the validator's complete grading manifest across CLAUDE.md + rules + ledger. D1-rationale completion (single-source reason vs @import) is a lead formulation fix, folded with it.
- **M2 + M3 — user-ruled (2026-07-18).** Universal principles move into CLAUDE.md as short imperative lines (probe-driven retreat from unconditional rules files); KM operating-manual and release-gate detail drop behind pointers; tech stack + gates summary stay in CLAUDE.md; D3 records that the marginal-context-cost win (Problem #3) holds fully only for scope-bound principles; D5's pointer becomes an **obligated read** of the named scoped rules before authoring. Amends D2 (index+stamp "only" → index+stamp+universal principles+stack+gates summary, all short-form), D3 (two rows re-routed), D5 (obligated read). Fold at freeze-lift.
