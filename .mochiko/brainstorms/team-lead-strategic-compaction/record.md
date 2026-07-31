# Team-Lead Strategic Compaction — Decision Record

**Status:** accepted (2026-07-31) — pair-reviewed, 23/23 dispositioned, verify round 1 NOT CLEAN → repaired → round 2 CLEAN; landed: `DECISIONS.md` TC-D1–D6 + the standing-seat supersession clause · BACKLOG standing-seat surface re-specified + probe riders · ROADMAP Next touch
**When:** 2026-07-31
**Session:** `/mochiko:brainstorm` (goal-shaped command, shape v5)
**Topic:** In all mochiko agent teams led by commands, context compaction should be strategic —
and the team lead's responsibility. This session decides whether/how that becomes doctrine:
what "strategic compaction" means for the lead's own context and for its seats', where it
binds (command-shape Layer 2, per-command parameters, briefs), and how it relates to the
already-ruled seat-recycling doctrine (`standing-seat-lifecycle`).

**Driver (user, Q1):** long-running sessions with context filling to ~80%; known context-rot
degradation as the window fills; token-usage cost. Not a single-incident post-mortem —
a recurring condition of long team-form runs.

---

## Reality map (fact-checker, verbatim)

_The priority answer landed first (its own P1–P26 namespace, preserved below as landed); the
full map (sections A–D, **F1–F86** — renumbered from F78 after the hook/env-var facts were
folded into sections C and D rather than appended) follows it. Every P-fact also appears in
the map under its F-number._

### Priority answer — lead self-compaction (fact-checker, verbatim)

PRIORITY FACT — lead self-compaction. Bottom line first: **the model cannot invoke compaction, for itself or anyone — confirmed. But "no mechanism lets the lead *time* its own compaction" is partly falsified: three documented timing levers exist, none of them held by the lead-as-model mid-run.** Details below, all quotes verbatim from code.claude.com/docs.

**Q1 — Is `/compact` user-invocable only? YES for the model; no self-invocation path exists.**

- **P1** — No tool. `code.claude.com/docs/en/tools-reference` lists 38 built-in tools (`Agent`, `AskUserQuestion`, `Bash`, `EndConversation`, `Read`, `SendMessage`, `Skill`, `TaskStop`, `TodoWrite`, `ToolSearch`, …). The word "compact" appears **0 times on the page**, and **there is no `SlashCommand` tool**. Nothing in the tool surface reaches compaction.
- **P2** — Slash commands are user input by definition. `code.claude.com/docs/en/commands`: *"A command is only recognized at the start of your message."*
- **P3** — The platform **deliberately removed** the model's ability to fire slash commands. Same page: *"`/verify` and `/code-review` run only when you invoke them. **Before v2.1.215, Claude could also run them on its own.**"* This is the closest thing to an explicit ruling, and it cuts against self-invocation.
- **P4** — **NEW, and it moves the premise: `PreCompact` and `PostCompact` hooks exist.** `code.claude.com/docs/en/hooks`. `PreCompact` — *"Fires before context compaction begins. Can block compaction with exit code 2 or `decision: \"block\"`. Useful for logging, cleanup, or **preventing compaction at inopportune times**."* Matchers are `manual` (*"User ran `/compact`"*) and `auto` (*"Automatic compaction"*). Block form: `{"decision": "block", "reason": "Compaction blocked: waiting for long-running task to complete"}`. Input carries `session_id`, `transcript_path`, `cwd`, `permission_mode`, `hook_event_name`, `trigger`. `PostCompact` fires after and has **no** decision control — side effects only.
- **P5** — **And mochiko could ship them.** Same page: *"Hooks can also be defined in [plugin](/docs/en/plugins) `hooks/hooks.json` when the plugin is enabled, and they merge with your user and project hooks."* (Flagging without arguing it: a shipped hook is harness config, which sits near CLAUDE.md's "no kernel infrastructure" constraint. That's a design call, not a fact — I only report that the mechanism exists and is plugin-shippable.)
- **P6 — Absence:** no hook *triggers* compaction. `PreCompact` can only **block/defer**; there is no `RequestCompact` or equivalent. So even via hooks, compaction can be postponed, never fired.

**Q2 — What auto-compact does, its threshold, and whether it can be timed.**

- **P7** — What it does. `code.claude.com/docs/en/how-claude-code-works`: *"Claude Code manages context automatically as you approach the limit. **It clears older tool outputs first, then summarizes the conversation if needed.** Your requests and key code snippets are preserved; detailed instructions from early in the conversation may be lost."* And `code.claude.com/docs/en/context-window`: *"Claude Code compacts automatically as you approach the limit, so a full context window doesn't end your session."*
- **P8** — **The threshold is model- and config-dependent, and there is no single number.** `code.claude.com/docs/en/env-vars`, `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE`: *"Set the percentage (1-100) of the auto-compaction window at which auto-compaction triggers. Use lower values like `50` to compact earlier. This variable only causes earlier compaction when Claude Code compacts **proactively**: when `CLAUDE_CODE_AUTO_COMPACT_WINDOW` is set, in cloud sessions, and on Sonnet 4.6 and Opus 4.6 without extended context, which compact at the 200K boundary by default. On Sonnet 5, proactive compaction applies at the model's default threshold. **In other cases, such as a local session on Opus 4.8, auto-compaction triggers when the conversation reaches the model's context limit.** The override can only lower the threshold, so values above the default have no effect."*
- **P9** — **The threshold IS configurable, ahead of the run.** `CLAUDE_CODE_AUTO_COMPACT_WINDOW`: *"Set the context capacity in tokens used for auto-compaction calculations. Defaults to the model's context window, 200K for standard models or 1M for extended context models… Use a lower value like `500000` on a 1M model to treat the window as 500K for compaction purposes… Setting this variable **decouples the compaction threshold from the status line's `used_percentage`**, which always uses the model's full context window."* One published default: `code.claude.com/docs/en/model-config` — Sonnet 5 *"auto-compact before the window fills, **at about 967K tokens by default**"*.
- **P10** — **`CLAUDE_AUTOCOMPACT_PCT_OVERRIDE` is the ONLY documented sentence tying compaction config to sub-sessions.** Its entry ends: *"**Applies to both main conversations and subagents**"*. Note it says *subagents*, not *teammates*; the docs nowhere restate it for team-form teammates. The other two variables carry no subagent/teammate applicability statement.
- **P11 — So compaction CAN be timed, by three parties, none of which is the lead-as-model mid-run:** (a) **the human user**, by typing `/compact` at a chosen moment — and the docs explicitly recommend this over waiting: `code.claude.com/docs/en/prompt-caching`, *"To choose when its overhead happens, run `/compact` at a natural break in your work, such as between tasks, instead of waiting for auto-compaction to trigger mid-task."* (b) **a `PreCompact` hook**, by blocking an auto pass at a bad moment (P4). (c) **environment config**, set before the run (P8/P9). The lead can *recommend* (a) — and every mochiko command is human-attended with named gates, so the human is present at exactly the "natural break" the docs describe.
- **P12** — Auto-compaction can also fail: `how-claude-code-works` — *"If a single file or tool output is so large that context refills immediately after each summary, Claude Code **stops auto-compacting after a few attempts and shows an error instead of looping**."*

**Q3 — Influencing WHAT survives (this the lead has real reach over).**

- **P13** — `/compact` takes an instructions argument. `code.claude.com/docs/en/commands`: *"`/compact [instructions]` — Free up context by summarizing the conversation so far. **Optionally pass focus instructions for the summary.**"* User-typed, so lead-recommendable, not lead-invocable.
- **P14** — **Standing instructions can be pre-authored in CLAUDE.md, and these apply to the automatic pass too.** `code.claude.com/docs/en/costs`: *"You can also customize compaction behavior in your CLAUDE.md file at the root of your project"* with the block `# Compact instructions / When you are using compact, please focus on test output and code changes`. `how-claude-code-works`: *"To control what's preserved during compaction, add a 'Compact Instructions' section to CLAUDE.md or run `/compact` with a focus."* **This is the one lever that works with no human in the loop and no hook.**
- **P15** — The survival table, verbatim (`code.claude.com/docs/en/context-window`, "What survives compaction"): system prompt/output style *"Unchanged; not part of message history"* · project-root CLAUDE.md and unscoped rules *"Re-injected from disk"* · auto memory *"Re-injected from disk"* · `paths:`-scoped rules *"Lost until a matching file is read again"* · nested CLAUDE.md *"Lost until a file in that subdirectory is read again"* · invoked skill bodies *"Re-injected, **capped at 5,000 tokens per skill and 25,000 tokens total; oldest dropped first**"* · hooks *"Not applicable; hooks run as code, not context"*.
- **P16 — Load-bearing for mochiko: the table has NO row for file reads or tool results, and the command body is not on it either.** Mechanism sentence (same page): *"Path-scoped rules and nested CLAUDE.md files load into message history when their trigger file is read, so **compaction summarizes them away with everything else**."* And `code.claude.com/docs/en/prompt-caching`: *"Skills and commands **inject their instructions as user messages** at the point of invocation."* So a compacted lead loses, to a summary: the command file's own body, and the obligated reads (`command-shape.md` both layers, `agent-dispatch.md`, `sized-end-stage-review.md`) — while the project-root CLAUDE.md operating manual is re-injected from disk and skill bodies come back truncated (start-of-file kept: *"Truncation keeps the start of the file"*). **Disk artifacts and the as-you-go record survive by being re-readable, not by surviving.**
- **P17** — Compaction is not free: `prompt-caching`, "Compacting the conversation" — *"To produce the summary, Claude Code sends a **separate request with the same system prompt, tools, and history as your conversation**, plus a summarization instruction… **After a break longer than the cache lifetime, there is no cache left to read, so the summarization request reprocesses the full history as uncached input.**"* `costs`: *"When you want a fresh start instead of continuity, `/clear` costs nothing."*

**Q4 — Teammates: can a lead trigger or instruct a teammate's compaction? NO documented path, for the model OR the human.**

- **P18** — **The agent-teams page never mentions compaction at all.** Full page read this session (`code.claude.com/docs/en/agent-teams`): zero occurrences of "compact", "compaction", "auto-compact", or "context limit". Its whole context treatment is: *"Each teammate has its own context window… **The lead's conversation history does not carry over.**"*
- **P19 — Decisive, and it surprised me: a `/compact` typed while viewing a teammate compacts the LEAD.** agent-teams, "Talk to teammates directly": *"While you're viewing an in-process teammate, plain text and skills go to that teammate, but **built-in commands still run in the lead's session**."* Corroborated by the adjacent behavior: *"A teammate's model and fast mode are fixed when it spawns, so `/model` and `/fast` only change the lead's settings. As of v2.1.199, typing either command while viewing a teammate shows a notice that the change applies to the lead."*
- **P20 — Caveat I will not overstate:** P19 is documented **for in-process mode**, which is the default (*"The default is `\"in-process\"`"*). For split panes the page says *"click into a teammate's pane to interact with their session directly. Each teammate has a full view of their own terminal"* — and **the docs never state whether a built-in command typed into a tmux/iTerm2 teammate pane runs in that teammate or in the lead.** Undocumented, not confirmed either way.
- **P21** — The complete documented set of lead→teammate controls is: spawn (name, model, plan-approval requirement), `SendMessage` by name, plan approve/reject, task assignment, shutdown. **Nothing about context.** `SendMessage`'s own tool entry — *"Sends a message to an agent team teammate, or resumes a subagent by its agent ID or name"* — has no context or compaction parameter.
- **P22 — Absence:** whether `PreCompact` fires per-teammate is **undocumented**. The hooks page confirms hooks run inside subagents for *tool* events — *"tool events such as `PreToolUse` and `PostToolUse` fire the same configured hooks as in the main conversation, and the input carries the `agent_id` and `agent_type`"* — and lists `TeammateIdle`, `SubagentStart`, `SubagentStop` as the agent-lifecycle events. It never says `PreCompact` fires for a teammate. So a `PreCompact` hook as a per-seat instrument is **unverified**, and `PreCompact` cannot trigger compaction anyway (P6).
- **P23 — Absence:** no per-teammate context or token display exists anywhere. `code.claude.com/docs/en/agent-view` (fetched this run) has zero mention of context usage, token counts, or compaction for individual agents; the agent panel is name/status rows plus transcript navigation. This re-confirms `standing-seat-lifecycle` F-e(a) as still current — **the lead has no surface on which to observe a seat's occupancy**, which is exactly the fact that dissolved thresholds in that record (`record.md:193`).
- **P24 — Absence:** no teammate summarize / handoff / context-export primitive is documented. agent-teams describes information sharing exhaustively as automatic message delivery, idle notifications, the shared task list, and messaging by name.
- **P25 — So the answer to Q4 is: teammates only auto-compact (inferred, never documented — see P18 and `standing-seat-lifecycle` §3c/§5, which already marked this UNKNOWN), or they die and get respawned.** Both death paths are documented: graceful — *"The lead sends a shutdown request. The teammate can approve, exiting gracefully, or reject with an explanation"* (limitation: *"Shutdown can be slow — teammates finish their current request or tool call before shutting down"*); hard — `TaskStop` *"also accepts an agent-team teammate… by agent ID or name"*. Replacement is the documented recovery: *"Spawn a replacement teammate to continue the work."*
- **P26 — One warning for the already-ruled recycling design:** `standing-seat-lifecycle` D1's same-name respawn rests on "latest wins", sourced to a pre-v2.1.199 SendMessage doc. The current tool doc says: *"As of v2.1.199, a send to a name that now resolves to a different agent than it did earlier in the conversation is **refused instead of delivered**."* A lead that respawns a seat under its old name may find sends to that name refused. Worth a check before any cadence rule leans on name reuse.

*(Lead note at review, RI-18: the three paragraphs below are the checker's interpretive
commentary over its own facts — P1–P26 and F1–F86 are the fact surface; what follows is
commentary, not mapped fact. Its "the seats have **zero** for anyone" phrasing is the
equivocation D1's Resolution corrects — review fold #7.)*

**What this means for D1, stated as facts and not as a recommendation:**

- The strict reading of your conditional — *can the lead invoke its own compaction?* — is **NO** (P1, P2, P3, P6). On that reading D1 stands as written.
- The rationale's second clause — *"no platform mechanism lets the lead invoke **or time** its own compaction"* — is **partly falsified**. Timing levers exist (P11): the human user typing `/compact` at a lead-named moment, a plugin-shippable `PreCompact` block, and pre-run threshold config. What the lead lacks is a lever it can pull **unilaterally, mid-run**. Whether "the lead recommends and the human types, at a gate the command already owns" counts as the lead holding the responsibility is a judgment for you and the user — I'm reporting that the mechanism exists, not that it should be used.
- D1's asymmetry is **inverted from what the record assumes**, and this is the sharpest finding: the lead's own context has **three** documented influence paths (P11 timing, P13/P14 what-survives) — while the seats have **zero** for anyone, human or model (P18–P25). The one seat-side lever that exists is kill-and-respawn, which `standing-seat-lifecycle` D1–D4 already rules and which `BACKLOG.md:199` records as unbuilt. So a doctrine scoped to "seats only" targets the half of the team where **no compaction mechanism exists at all**.

*(end verbatim priority answer)*

### Full reality map — sections A–D, F1–F86 (fact-checker, verbatim)

#### A. The team-form command surface

- **F1** — The command surface is **six files**, all under `plugins/mochiko/commands/`: `brainstorm.md` (109 lines), `implement.md` (154), `plan.md` (163), `setup.md` (180), `slice.md` (114), `specify.md` (108). No other command files exist.
- **F2** — **All six are team-form; there are zero one-shot commands.** Each binds Layer 2 by the identical obligated-read line — "Read `${CLAUDE_PLUGIN_ROOT}/templates/command-shape.md` (**both layers**)" (`brainstorm.md:12-13`, `implement.md:14-15`, `plan.md:13-14`, `setup.md:13-14`, `slice.md:14-15`, `specify.md:13-14`) — and every frontmatter `description:` ends "Requires agent teams (CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS); refuses without them."
- **F3** — **Correction to the prior records' count: seven team-form commands is now six.** `workflow-token-reduction/record.md:22-36` and `model-tiered-seats/record.md:60` both enumerate a seventh, `tasks.md`; no such file exists on disk today — the mapping and tasks stages are stages inside `plan.md` (`plan.md:38`, `plan.md:90-91`). Any session arithmetic inherited from those maps is off by one command.
- **F4** — `brainstorm.md` seats (`:35-38`): **fact-checker** — "a neutral empiricist, no skill mounted", spawned "at start, conditional on the topic having a reality surface", standing for the whole session; **reviewer(s)** — `devils-advocate` × `review-brainstorm`, "cold at convergence only, never in the room before it; count per the sizing ruling". Run length (`:61-64`): "per reviewer one cold read, plus (pair only) the one-shot four-message cross-exam, plus one verify pass; lead↔reviewer argument **max two exchanges per survivor** … one fact-checker dispatch per fact." No numbered gates; Recovery (`:105`) states "No resume table — the record is the whole state."
- **F5** — `specify.md` seats (`:33-34`): **producer** `requirements-analyst` — "one **named standing seat** across rounds; **probe seat**"; **critic** `devils-advocate` — "cold at first critique, standing after". Run length: "cap **3** rounds, you count them" (`:61`). Gates G1/G2/G3 plus escalation.
- **F6** — `slice.md` seats (`:38-39`): **producer** `task-architect`, "one **named standing seat** across rounds; **probe seat**"; **reviewer** `devils-advocate`, "cold at first review, standing after". Run length: "cap **3** rounds" (`:74`). Gates G1–G4.
- **F7** — `plan.md` seats (`:36-41`) — **six rows, the largest roster**: **producer** `technical-analyst` "standing across analysis + detailed design; **probe seat**"; **system-architect** "standing, architecture stage"; **task-architect** "standing across mapping + tasks"; **feasibility** `principal-architect` "cold once the analysis is authored; **lead-gated** thereafter"; **completeness** `devils-advocate` "cold at first review, standing after"; **architecture scribe** "disposable, at finalize". Run length (`:90-93`): "cap **3** produce↔review rounds **per stage** (analysis · architecture · detailed design · mapping · tasks)" — up to **15 produce↔review rounds**, with the completeness reviewer standing through all of them. Gates G1–G7.
- **F8** — `implement.md` seats (`:39-42`): **producer** `staff-engineer` — "**standing across the cycle sequence and the fix-pass loop**; **probe seat**, foundation cycle 1"; **verifier** `qa-engineer` — "cold at the first cycle verification, **standing after**", spanning every cycle plus the final validation; **arch-diff** and **arch-scribe** disposable. Run length (`:90-95`): "**targeted retry** … **max 3 attempts per cycle**; **fix pass** … **max 3 passes**", with the cycle count set by `tasks.md`, not by the command. Gates G1–G5.
- **F9** — **`implement` is the longest-running team by construction** — its producer's lifetime is bounded by the feature's cycle count, which the command file leaves unbounded; every other command's standing seat is bounded at ≤3 rounds (`specify`, `slice`), ≤15 rounds split across five stages among three different producer seats (`plan`), or ≤3 authoring rounds plus analysis (`setup`). This matches the measurement carried at `standing-seat-lifecycle/record.md:54`: implement's producer at "~15–45+ turns vs ≤6 everywhere else".
  *(Lead note at review, verify B1: the "≤6 everywhere else" half of this fact is defective — superseded by FD-2's corrected F9 in the fact-dispute answers below.)*
- **F10** — `setup.md` seats (`:37-39`): **producer** `principal-architect` — "standing across both jobs — at analysis in brownfield (**probe seat**), else first produce"; **intent reviewer(s)** `devils-advocate` — "cold at the synthesis review, count per the sizing ruling"; **validator** — "cold at first validation, **messaged after**". Run length (`:112-117`): "cap **3** produce↔validate rounds … review caps: one cold read per reviewer, one four-message cross-exam, a two-exchange lead↔reviewer cap per survivor, one verify pass, plus one bounded delta-pass on a material G3 edit", plus an **unbounded** stretch: "The interrogation is bounded instead by user-driven convergence — a human-attended session, not an agent loop."
- **F11** — **Two commands run a long, unbounded, human-attended inline stretch in the lead's own context**, before or between seat work: `setup`'s interrogation ("*yours, inline — no seat runs it*", `setup.md:59` — ten agenda dimensions plus a card-by-card catalog deck) and `brainstorm`'s questioning ("You run the questioning inline via `mochiko:analysis-iterative` — one question per turn", `brainstorm.md:16-17`). `specify` runs a bounded inline enrichment ("**yours, inline, and once**", `specify.md:64`).
- **F12** — Every command's **Recovery** block prescribes respawn: "resume from workspace evidence, respawning what the stage needs" (`specify.md:97-98`, `plan.md:142-143`, `implement.md:137`, `setup.md:163-164`, `slice.md:101-102`); `brainstorm.md:107-109` names "respawning the fact-checker mid-session or the reviewers per the sizing ruling". The trigger for every one of these is a **pause/resume**, never a context condition.

#### B. Existing doctrine touching context accumulation, compaction, or seat lifecycle

- **F13** — `templates/command-shape.md` is **shape v5, 2026-07-30**, 240 lines (`:223-240` carries the version block).
- **F14** — Layer 2 "**Seats, not dispatches**" (`command-shape.md:176-179`) is the standing-seat continuity claim, verbatim: *"A teammate's plain text is invisible to the lead: reports arrive as **messages**, and every follow-up goes to the **same named seat**, which is the continuity a standing seat exists to buy."*
- **F15** — Layer 2 "**Seat transport**" (`command-shape.md:165-173`), verbatim: *"**A spawn without a `name:` is a one-shot subagent — in a team-form command, the forbidden transport.**"* · *"**Every later round** is a `SendMessage` to that same name. A fresh spawn per round is the subagent anti-pattern wearing a team's clothes."* · *"Not addressable → kill it and respawn, explicitly requesting an agent team."* The only "kill and respawn" in doctrine is **addressability-failure recovery**.
- **F16** — The seat-roster parameter **P5** (`command-shape.md:75-79`) enumerates: *"seat · agent × skill(s) · produces or grades · spawn (standing / cold / disposable, and when — the probe seat marked) · peer edges"*. Three lifecycle values exist as spawn descriptors, but **no line anywhere states when a standing seat should stop standing**, and no parameter carries a lifecycle policy, context budget, or recycling cadence.
- **F17** — The **P1–P16 slot index** (`command-shape.md:112-117`) is the audit's parameter-completeness set. **No slot covers context lifecycle, compaction, context budget, seat recycling, or run cost.** The version block records that a cost slot once existed and was removed: *"**the Run-cost entry element dropped** by user ruling (step-1 adjudications), retiring v3's manual-baseline carrier"* (`:229-230`).
- **F18** — Layer 1 "**One lead**" (`command-shape.md:123-126`) is the exhaustive statement of the lead's duties, verbatim: *"The body addresses a single lead, who owns the loop's counters, every verdict, every escalation, every human gate, and the user-facing conversation."* **Context management is not in that list.**
- **F19** — **Recovery** (`command-shape.md:105-110`), verbatim: *"One line of pause posture [PARAM: where resume state is noted — **on the deliverable** by default …]. Sessions and teams do not survive `/resume`, and a shared account limit can throttle the team and the main session together — escalation then has nowhere to go but pause. Resume from **workspace evidence**, never a context `phase` field, respawning only what the stage needs."* This is the **only** doctrine line naming a resource ceiling, and its remedy is pause, not compaction.
- **F20** — Layer 2 "**Clearing under the mesh**" (`command-shape.md:211-219`) is the only doctrine that bounds what enters the lead's context: *"The lead reads the escalations and the endgame, not every clean result."* Its stated justification is judgment and independence — *"wherever judgment exists, the verifying seat's status is **input, never the gate**"* — **never context or token cost**.
- **F21** — `templates/agent-dispatch.md` is the caller-side briefing checklist, **eight fields** (`:11-21`): Skill(s) to lean on · Role this run · Input(s) to read · Where the output goes · What good looks like · Prior feedback (retries) · Independence framing · Return vs. write. **None concerns context, compaction, budget, or seat lifetime.** Its one hard line (`:26-39`) is independence only.
- **F22** — **Grep absence across the whole plugin** (`plugins/mochiko/`, all file types), run this session: `recycl` **0** · `context window` **0** · `context growth` **0** · `seat lifetime` **0** · `shut down` **0** · `shutdown` **0** · `token budget` **0** · `auto-compact` **0** · `microcompact` **0** · `context health` **0** · `/usage` **0**.
- **F23** — **`compact` now appears zero times in `plugins/mochiko/commands/` and zero times in `plugins/mochiko/agents/`.** The only two plugin hits are adjectival and unrelated to context: `templates/spec-template.md:4` ("entities conceptual and compact") and `templates/artifact-format.md:26` ("a compact ID index"). This **moves the `standing-seat-lifecycle` §7 erratum** (`record.md:144`), which recorded exactly one lexically-incidental hit at `commands/brainstorm.md:36` ("compact digest") — that line no longer exists after the v5 goal-shape rewrite, so the commands' lexical hit count for `compact` has gone from 1 to 0.
- **F24** — **Live `compact` hits in `.mochiko/` (excluding `archive/` and `transform/`), complete list — all in session records or strip notes, none in shipped doctrine:** `brainstorms/index.md:11,13,14` · `brainstorms/team-lead-strategic-compaction/record.md:1,6,8` · `brainstorms/standing-seat-lifecycle/record.md:40,64,94,114,115,116,144,148,179,186,188,192,194,199,244,292` · `brainstorms/workflow-token-reduction/record.md:267` (adjectival) · `brainstorms/ops-observability-hardening/record.md:429,449,1198` · `brainstorms/brainstorm-command-rewrite/record.md:52,124` (adjectival) · `brainstorms/brainstorm-v2-2-revision/record.md:26` (adjectival) · `strips/*.md` ×5 (all adjectival). **Archive plus transform:** 1 file, 4 hits, not re-examined. **Net: "compaction" as a context concept exists in mochiko only inside `standing-seat-lifecycle/record.md`, `ops-observability-hardening/record.md:1198`, and this session's own files — nowhere in any command, template, agent, or skill.**
- **F25** — `standing-seat-lifecycle/record.md` **D1** (`:262-268`): implement's **producer** recycles at cycle checkpoints under a *"**conditioned cadence**"* — *"recycle when the transcript has plausibly outgrown a fresh brief **or** the cache has gone cold — approximated by two lead-countable conditions, a **cycle floor (~≥3 …)** and a **gate-pause-duration check**"*; the **verifier** recycles *"per slice boundary"*; the dying seat shuts down *"when its cycle work is on disk — before the gate pause"*; the successor spawns *"under the same seat name"*. Scope: **implement's two seats only** — *"short-loop seats keep standing"*.
- **F26** — **D2** (`:270-276`): a fresh incarnation is briefed *"from the existing artifact set alone … No new handoff artifact; no lead-authored state summary."* Kill-safety precondition: *"a seat is recyclable at a boundary iff this set is current on disk."* Sufficiency is a **named dogfood watch-item**, not a settled fact.
- **F27** — **D3** (`:280-288`): Layer 2 is to be rewritten on **two axes** — *"**Team transport** … is unchanged. **Per-seat context lifecycle** — standing / stage-scoped / boundary-recycled — becomes a per-seat choice keyed to loop length, declared in each command's seat-roster [PARAM]."* The sentence *"that continuity is what a standing seat buys"* is retired; the anti-pattern line is **retargeted at transport**. Also ruled in: an **end-of-need shutdown norm** — *"a seat shuts down when its remaining work is zero **and re-summons is improbable**"* — explicitly gated because *"the pure-idle token cost is **unknown**"*.
- **F28** — **D4** (`:290-292`): per-seat measurement rides the epic's OTel probe, with the map's unknowns as probe questions — *"teammate compaction (whether, when, cost, what it drops), pure-idle seat burn, SendMessage cache behavior across rounds"* — plus *"a `/usage` / status-line `used_percentage` reading on the heavy seats"* at gates.
- **F29** — **D1–D4 are ruled but NOT BUILT.** `BACKLOG.md:199-201` carries the open item verbatim: *"**Standing-seat build items (deferred)** — conditioned checkpoint recycling · respawn briefs from artifacts · the Layer-2 transport-vs-lifecycle rewrite (**v4+** — coordinate with the team-method mesh rewrite above) · per-seat measurement. Record D1–D4."* `DECISIONS.md:79` records the ruling; the shape is now v5 and its version block (`command-shape.md:223-236`) attributes every v4/v5 change to `command-succinctness-strip` and `team-method-vs-command-shape`, **not** to standing-seat D3 — and F14 confirms the sentence D3 retires is still in force.
- **F30** — **A recycling addition to `implement` was authored and then reverted by user ruling, one day ago.** `.mochiko/decisions/2026-07-30-goal-shape-wave-ceremony.md:19`, verbatim: *"implement's seat-recycling addition **reverted** (`RETURNED:` with re-add trigger — standing-seat build items shipping, D3 first)"*. The doctrine order is on record: **D3 must land before any per-command recycling text.**
- **F31** — **The lead's own accumulation has exactly one disposition in the repo, and it accepts it.** `standing-seat-lifecycle/record.md:192` (the S3 review fold), verbatim: *"the lead's read-accumulation across a run is **accepted** — bounded by epic-D3's per-unit slimming and, at the limit, lossy session auto-compaction (§3c) — because the lead is structurally un-recyclable: under the one-lead shape its context *is* the session, and no cheaper disposal exists without changing the command shape itself."* Same wording at `:244`.
- **F32** — **Thresholds were considered and dissolved for want of an instrument.** `standing-seat-lifecycle/record.md:193`: *"F-e(a): no documented surface lets the *lead* see a teammate's context occupancy — thresholds have nothing to trigger on."* The residue is a **user escape hatch** — *"the user, who *can* see per-seat panes, may order a recycle at any gate — as a **coarse, instrument-blind override**"*.
- **F33** — `standing-seat-lifecycle/record.md:64` (§3c) records teammate compaction as **inference, not fact**: *"**Context window fills → auto-compaction — INFERENCE for teammates; the agent-teams page never mentions compaction.**"* And `:94` (§5): *"**Teammate auto-compaction: UNKNOWN.**"*
- **F34** — `model-tiered-seats/record.md:136` (**D1**) makes context health a standing test, verbatim: *"**Worker-context health** — what stays out of strong seats' contexts — remains the **mandatory secondary test** every tiered-seat design must also pass, composing with angle 1's lifecycle rulings."* Applied at `:163`: *"the bulk read is isolated inside the disposable subagent and the return is a terse answer, never a raw dump; the dump staying out of the dispatcher's context is the test's point."* It is a **design-review test on proposals**, not a runtime duty on any lead.
- **F35** — `model-tiered-seats/record.md:161` (**D4 fold F5**) forecloses a standing cheap seat partly on accumulation grounds: *"a standing seat re-pays its transcript across gate pauses (angle-1 §3); the disposable subagent's frontmatter `model:` is the **confirmed** mechanism."*
- **F36** — `workflow-token-reduction/record.md` headline spend map (§2, `:38-44`): the **lead-side doctrine tax** is ~7,193 tokens est./run (`command-shape.md` + `agent-dispatch.md` + `loop-discipline`); the **heaviest single seat load** is plan's `technical-analyst` at ~16.8k tokens est. of skills alone, ~43k with all reference bundles. §3 (`:46-52`) names the repeat-load surfaces: the doctrine tax recurs on every run **and on every `/resume`**; pipeline artifacts are re-Read cold into ~10 agent contexts per feature; and *"once invoked, 'the rendered SKILL.md content enters the conversation… and stays there for the rest of the session'"*.
- **F37** — **D2** (`workflow-token-reduction/record.md:258-260`): *"each workflow run ends with a recorded cost entry in the feature directory. **Baseline mechanism … a manual protocol** — at run end the lead records the user-visible usage figure (e.g. from `/usage`, supplied by the user) plus the run-shape counts the lead itself observes"*. Load-bearing constraint at `:260`: *"**the platform exposes no session-readable cumulative token total** (only a USD estimate + a live context snapshot), and an automated transcript parse would breach the kernel-free rule."*
- **F38** — **That cost entry was subsequently dropped from the shape** (F17), so nothing in any shipped command asks a lead to record or observe usage today. `BACKLOG.md:191-193` keeps the OTel probe open: *"Standing-seat D4 + model-tiered D6 probe questions ride it."*
- **F39** — `ops-observability-hardening/record.md:1198` carries the probe forward: *"**D4** per-seat measurement rides the epic's OTel probe (per-seat attribution, the teammate-compaction/idle/cache unknowns as probe questions, a manual per-seat `/usage` reading at gates)."*
- **F40** — **Absence, stated:** no line in any of the six commands, in `command-shape.md`, in `agent-dispatch.md`, in `sized-end-stage-review.md`, or in any skill assigns **anyone** — lead, seat, or user — a duty to observe, budget, compact, or reset a context. The only lifecycle instructions in shipped doctrine are "respawn on resume" (F12) and "kill and respawn on addressability failure" (F15).

#### C. Platform affordances

- **F41** — **The agent-teams page never mentions compaction.** Full page read this session (`https://code.claude.com/docs/en/agent-teams`): zero occurrences of "compact", "compaction", "auto-compact", or "context limit". Its entire context treatment is: *"Each teammate has its own context window. When spawned, a teammate loads the same project context as a regular session: CLAUDE.md, MCP servers, and skills. It also receives the spawn prompt from the lead. **The lead's conversation history does not carry over.**"* This confirms F33 as still current.
- **F42** — Teammates are full sessions — agent-teams: *"Each teammate is a full, independent Claude Code session."* This is the sole basis for inferring session-level compaction behavior applies inside a teammate; **no doc states it.**
- **F43** — **Auto-compaction exists and fires without user action.** `context-window`: *"Claude Code compacts automatically as you approach the limit, so a full context window doesn't end your session. The automatic pass works the same way as the `/compact` step in the timeline."* `how-claude-code-works`, "When context fills up": *"Claude Code manages context automatically as you approach the limit. **It clears older tool outputs first, then summarizes the conversation if needed.** Your requests and key code snippets are preserved; detailed instructions from early in the conversation may be lost."*
- **F44** — **The trigger is model- and config-dependent; there is no single number.** `env-vars`, `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE`: *"Set the percentage (1-100) of the auto-compaction window at which auto-compaction triggers. Use lower values like `50` to compact earlier. This variable only causes earlier compaction when Claude Code compacts **proactively**: when `CLAUDE_CODE_AUTO_COMPACT_WINDOW` is set, in cloud sessions, and on Sonnet 4.6 and Opus 4.6 without extended context, which compact at the 200K boundary by default. On Sonnet 5, proactive compaction applies at the model's default threshold. **In other cases, such as a local session on Opus 4.8, auto-compaction triggers when the conversation reaches the model's context limit.** The override can only lower the threshold, so values above the default have no effect."*
- **F45** — **The only documented sentence tying compaction config to sub-sessions.** Same entry, final clause, verbatim: *"**Applies to both main conversations and subagents**"*. It says *subagents*, not *teammates*; the docs nowhere restate it for team-form teammates. `CLAUDE_CODE_AUTO_COMPACT_WINDOW` and `CLAUDE_CODE_DISABLE_1M_CONTEXT` carry **no** subagent or teammate applicability statement.
- **F46** — The compaction window is configurable process-wide. `env-vars`, `CLAUDE_CODE_AUTO_COMPACT_WINDOW`: *"Set the context capacity in tokens used for auto-compaction calculations. Defaults to the model's context window, 200K for standard models or 1M for extended context models… Use a lower value like `500000` on a 1M model to treat the window as 500K for compaction purposes… Setting this variable **decouples the compaction threshold from the status line's `used_percentage`**, which always uses the model's full context window."*
- **F47** — One published default: `model-config`, "Sonnet 5 context window": *"Sessions auto-compact before the window fills, **at about 967K tokens by default**; set `CLAUDE_CODE_AUTO_COMPACT_WINDOW` to choose a different threshold."*
- **F48** — **`PreCompact` and `PostCompact` hooks exist.** `hooks`. `PreCompact`: *"Fires before context compaction begins. Can block compaction with exit code 2 or `decision: \"block\"`. Useful for logging, cleanup, or **preventing compaction at inopportune times**."* Matchers: `manual` (*"User ran `/compact`"*) and `auto` (*"Automatic compaction"*). Input fields: `session_id`, `transcript_path`, `cwd`, `permission_mode`, `hook_event_name`, `trigger`. Block form: `{"decision": "block", "reason": "Compaction blocked: waiting for long-running task to complete"}`. `PostCompact` fires after, same matchers, and has **no decision control** — *"Exit codes and JSON output are for side effects only (logging, cleanup). The hook cannot block or prevent any action."*
- **F49** — **Hooks are plugin-shippable.** `hooks`: *"Hooks can also be defined in [plugin](/docs/en/plugins) `hooks/hooks.json` when the plugin is enabled, and they merge with your user and project hooks."* Mochiko is a plugin, so this is a surface it could occupy. (Fact only; whether a shipped hook sits inside CLAUDE.md's "no kernel infrastructure" constraint is a design call.)
- **F50** — **Absence: no hook, tool, or command triggers compaction.** `PreCompact` can only block or defer; there is no `RequestCompact` or equivalent event. Compaction can be postponed by machine, never fired by machine.
- **F51** — **Absence: whether `PreCompact` fires per-teammate is undocumented.** The hooks page confirms hooks run inside subagents for *tool* events — *"tool events such as `PreToolUse` and `PostToolUse` fire the same configured hooks as in the main conversation, and the input carries the `agent_id` and `agent_type`"* — and lists `TeammateIdle` (*"When an agent team teammate is about to go idle"*, blockable), `SubagentStart`, and `SubagentStop` as the agent-lifecycle events. It never says `PreCompact` fires for a teammate.
- **F52** — **`/compact` takes an instructions argument.** `commands`: *"`/compact [instructions]` — Free up context by summarizing the conversation so far. **Optionally pass focus instructions for the summary.**"* `costs`: *"**Add custom compaction instructions**: `/compact Focus on code samples and API usage` tells Claude what to preserve during summarization."*
- **F53** — **Compaction instructions can be made standing, in CLAUDE.md, and they apply to the automatic pass too.** `costs`: *"You can also customize compaction behavior in your CLAUDE.md file at the root of your project"*, with the block `# Compact instructions / When you are using compact, please focus on test output and code changes`. `how-claude-code-works`: *"To control what's preserved during compaction, add a 'Compact Instructions' section to CLAUDE.md or run `/compact` with a focus."*
- **F54** — **What survives compaction** — `context-window`, "What survives compaction" table, verbatim rows: system prompt and output style → *"Unchanged; not part of message history"* · project-root CLAUDE.md and unscoped rules → *"Re-injected from disk"* · auto memory → *"Re-injected from disk"* · `paths:`-scoped rules → *"Lost until a matching file is read again"* · nested CLAUDE.md → *"Lost until a file in that subdirectory is read again"* · invoked skill bodies → *"**Re-injected, capped at 5,000 tokens per skill and 25,000 tokens total; oldest dropped first**"* · hooks → *"Not applicable; hooks run as code, not context"*.
- **F55** — The mechanism sentence for everything not in that table — `context-window`: *"Path-scoped rules and nested CLAUDE.md files load into message history when their trigger file is read, so **compaction summarizes them away with everything else**."* The table has **no row for file reads or tool results**; they are message history.
- **F56** — Skill truncation is start-biased. `context-window`: *"Skill bodies are re-injected after compaction, but large skills are truncated to fit the per-skill cap, and the oldest invoked skills are dropped once the total budget is exceeded. **Truncation keeps the start of the file**, so put the most important instructions near the top of `SKILL.md`."*
- **F57** — **Compaction is cache-invalidating and costs a full read of what it summarizes.** `prompt-caching`, "Compacting the conversation": *"Compaction replaces your message history with a summary. By design, this invalidates the conversation layer … To produce the summary, Claude Code sends a **separate request with the same system prompt, tools, and history as your conversation**, plus a summarization instruction appended as a final user message. While the cache is warm, that request reads your prefix from the cache, so a mid-session `/compact` costs a fraction of what the context size suggests … **After a break longer than the cache lifetime, there is no cache left to read, so the summarization request reprocesses the full history as uncached input.**"* `costs`: *"When you want a fresh start instead of continuity, `/clear` costs nothing."*
- **F58** — **The docs recommend deliberate timing.** `prompt-caching`: *"To choose when its overhead happens, run `/compact` at a natural break in your work, such as between tasks, instead of waiting for auto-compaction to trigger mid-task."* And *"Pick your model and effort level at the top of a session, then **save `/compact` for natural breaks between tasks**."* `context-window`: *"**Compact with a focus**: run `/compact` with instructions … before starting a long new task. The summary keeps what you choose instead of what the automatic pass guesses is important."*
- **F59** — **Auto-compaction can fail.** `how-claude-code-works`: *"If a single file or tool output is so large that context refills immediately after each summary, Claude Code **stops auto-compacting after a few attempts and shows an error instead of looping**."*
- **F60** — **Absence: "microcompact" is not a documented feature.** Zero occurrences across `commands` (the full built-in slash-command list), `context-window`, `costs`, `how-claude-code-works`, `prompt-caching`, `model-config`, `env-vars`, `hooks`, and `agent-teams` — every page fetched this session.
- **F61** — **Absence: no tool lets a model trigger compaction.** `tools-reference` lists 38 built-in tools (`Agent`, `Artifact`, `AskUserQuestion`, `Bash`, `CronCreate/Delete/List`, `Edit`, `EndConversation`, `EnterPlanMode`, `EnterWorktree`, `ExitPlanMode`, `ExitWorktree`, `Glob`, `Grep`, `ListMcpResourcesTool`, `LSP`, `Monitor`, `NotebookEdit`, `PowerShell`, `PushNotification`, `Read`, `ReadMcpResourceTool`, `RemoteTrigger`, `ReportFindings`, `ScheduleWakeup`, `SendMessage`, `SendUserFile`, `ShareOnboardingGuide`, `Skill`, `TaskCreate/Get/List/Output/Stop/Update`, `TodoWrite`, `ToolSearch`). **The word "compact" appears 0 times on the page, and there is no `SlashCommand` tool.**
- **F62** — **Slash commands are user-typed, and the model's ability to fire them was removed.** `commands`: *"A command is only recognized at the start of your message."* And: *"`/verify` and `/code-review` run only when you invoke them. **Before v2.1.215, Claude could also run them on its own.**"*
- **F63** — **A `/compact` typed while viewing a teammate compacts the LEAD.** agent-teams, "Talk to teammates directly", verbatim: *"While you're viewing an in-process teammate, plain text and skills go to that teammate, but **built-in commands still run in the lead's session**."* Corroborating: *"A teammate's model and fast mode are fixed when it spawns, so `/model` and `/fast` only change the lead's settings. As of v2.1.199, typing either command while viewing a teammate shows a notice that the change applies to the lead."*
- **F64** — **Caveat on F63:** it is documented **for in-process mode**, which is the default (*"The default is `\"in-process\"`"*). For split panes the page says *"click into a teammate's pane to interact with their session directly. Each teammate has a full view of their own terminal"* — and **the docs never state whether a built-in command typed into a tmux or iTerm2 teammate pane runs in that teammate or in the lead.** Undocumented, not confirmed either way.
- **F65** — **Absence: no documented way for a lead to trigger, instruct, or observe a teammate's compaction.** The complete documented set of lead→teammate controls is: spawn (name, model, plan-approval requirement), `SendMessage` by name, plan approve/reject, task assignment, and shutdown. Nothing about context. `SendMessage`'s tool entry — *"Sends a message to an agent team teammate, or resumes a subagent by its agent ID or name"* — has no context or compaction parameter.
- **F66** — **Absence: no per-teammate context or token display is documented anywhere.** `agent-view` (fetched this session) has zero mention of context usage, token counts, compaction, or context health for individual agents; the agent panel is described in agent-teams as name/status rows with transcript navigation only. Unchanged from `standing-seat-lifecycle/record.md:111`: *"**no per-teammate token number is documented in the agent panel** … interactive-mode has **zero** mention of a token/context figure per teammate."*
  *(Lead note at review, verify B1: too strong — corrected by FD-1's erratum FD1-8 below: the agent panel's default subagent row shows a per-agent token count, human-facing.)*
- **F67** — The one context-occupancy instrument that exists is **per-session**, and each teammate is a session. Carried from `standing-seat-lifecycle/record.md:108` (statusline doc): `context_window.total_input_tokens` = *"Token counts **currently in the context window**, from the most recent API response"*; `context_window_size` = *"200000 by default, or 1000000 for models with extended context"*; plus `used_percentage` and `exceeds_200k_tokens`. Re-confirmed via F46: `used_percentage` *"always uses the model's full context window"* even when the compaction threshold is decoupled.
- **F68** — **What happens when a teammate nears or hits its limit is undocumented.** No agent-teams sentence covers it. The general session behavior (F43/F44) plus F42 is the only available reasoning, and F41 confirms the docs never make that connection.
- **F69** — **The lead can end a teammate two ways, both documented.** Graceful — agent-teams: *"The lead sends a shutdown request. The teammate can approve, exiting gracefully, or **reject with an explanation**."* Limitation: *"**Shutdown can be slow** — teammates finish their current request or tool call before shutting down."* Hard — `tools-reference`, `TaskStop`: *"Stops a running background task by ID. As of v2.1.198, it also accepts an **agent-team teammate** or a named background agent by agent ID or name."*
- **F70** — **Replacement is the documented recovery path, but name takeover has tightened since the record was written.** agent-teams troubleshooting: *"**Spawn a replacement teammate to continue the work.**"* `tools-reference`, `SendMessage`: *"As of v2.1.199, a send to a name that now resolves to a different agent than it did earlier in the conversation is **refused instead of delivered**."* **This narrows `standing-seat-lifecycle` D1's same-name respawn mechanic** (`record.md:264`, sourced to a pre-2.1.199 doc): a lead that respawns a seat under its old name may find sends to that name refused rather than silently rerouted.
- **F71** — **Cost guidance the docs give for long-running teams** — `costs`, "Agent team token costs": *"Use Sonnet for teammates."* · *"Keep teams small. Each teammate runs its own context window, so token usage is roughly proportional to team size."* · *"**Keep spawn prompts focused.** Teammates load CLAUDE.md, MCP servers, and skills automatically, but everything in the spawn prompt adds to their context from the start."* · *"**Shut down teammates when their work is done.** Each active teammate continues consuming tokens until it exits or the session ends."* agent-teams: *"**Too large**: teammates work too long without check-ins, increasing risk of wasted effort"* and *"Letting a team run unattended for too long increases the risk of wasted effort."* **None of these is framed as context management; all are framed as token cost or wasted effort.**
- **F72** — **Absence: no mechanism for a teammate to summarize itself, hand off state, or transfer context to a successor.** agent-teams describes information sharing exhaustively as automatic message delivery, idle notifications, the shared task list, and teammate messaging by name.
- **F73** — Teammate caches are separate and shorter-lived than the lead's. `prompt-caching`, "Subagents and the cache": *"A subagent starts its own conversation with its own system prompt and tool set, separate from the parent's. It builds its own cache, starting with no cache hits on its first call … **Subagents use the five-minute TTL even on a subscription**, since the automatic one-hour TTL applies to the main conversation. The parent's cache is unaffected."* (Stated for subagents; not restated for teammates.)
- **F74** — Cache lifetime, which sets the cost of any pause: `prompt-caching` — *"On a Claude subscription, Claude Code requests the one-hour TTL automatically"*; on API or cloud *"the TTL stays at the cheaper five minutes by default"*; drawing on usage credits, *"Claude Code automatically drops to the shorter one."* `costs`: *"your first message after a break longer than the cache lifetime misses the cache and reprocesses your full context."*
- **F75** — Teammates do not inherit lead settings uniformly: agent-teams — *"Teammates don't inherit the lead's `/model` selection by default"*, but *"Teammates inherit the lead's effort level"*, and *"The `skills` and `mcpServers` frontmatter fields in a subagent definition are **not applied** when that definition runs as a teammate."* Relevant because effort level is part of the cache key: *"each effort level has its own cache for the same model."*

#### D. The lead's own context

- **F76** — **Doctrine assigns the lead no context duty whatsoever.** F18 gives the exhaustive duty enumeration; F17 confirms no slot in P1–P16 carries one; F22 confirms zero lexical hits for every context-management term across the plugin. **The absence is total and it is at the shape level, not a per-command omission.**
- **F77** — **The only pause posture doctrine binds is resume-state location, and the only ceiling it names is an account limit** (F19). Per-command fills: `brainstorm.md:105-109` (on the record's `Status` line; no resume table), `specify.md:97` (on `spec.md`'s `Status` header), `slice.md:101`, `plan.md:142`, `implement.md:137`, `setup.md:163`. **No command's pause posture is keyed to context fill, token usage, or compaction proximity.**
- **F78** — **The repo's one statement about the lead's context accepts the risk rather than managing it** — F31: accumulation is *"accepted … bounded … at the limit, lossy session auto-compaction"* because *"the lead is structurally un-recyclable."* That reasoning was reviewed (survivor S3, DQ4, "Important (concurred)") and cleared as a disposition, not deferred.
- **F79** — **Auto-compaction will fire in the lead's session during a long team run, and nothing exempts a team run from it** (F43/F44); F41 confirms the agent-teams page never discusses it. **Absence:** no doc states what a lead's compaction does to team state — whether the roster, seat names, task assignments, or in-flight gate state survive the summary. What *is* documented is that team state lives on disk, not in the lead's context: agent-teams — *"Team config: `~/.claude/teams/{team-name}/config.json` · Task list: `~/.claude/tasks/{team-name}/`"*, and *"Teammates can read this file to discover other team members."*
- **F80** — **A lead cannot self-invoke `/compact`.** F61 (no tool; "compact" appears 0 times in the tools reference) and F62 (commands recognized only at the start of a user message; the model's slash-command ability removed in v2.1.215).
- **F81** — **But compaction of the lead's session IS timeable — by three parties, none of them the lead-as-model mid-run.** (a) The **human user** typing `/compact` at a chosen moment, which the docs explicitly recommend over waiting (F58) — and every mochiko command is human-attended with named gates, i.e. the "natural break" the docs describe. (b) A **`PreCompact` hook** blocking an auto pass at a bad moment (F48), plugin-shippable (F49). (c) **Environment config** setting the threshold before the run (F44/F46). **This partly falsifies the formulation that "no mechanism lets the lead time its own compaction": what the lead lacks is a lever it can pull unilaterally, mid-run — not the existence of levers.**
- **F82** — **The lead has real reach over WHAT survives, even without reach over WHEN.** Three surfaces: the `/compact [instructions]` argument (F52, user-typed, lead-recommendable); the standing "Compact instructions" section in the project-root CLAUDE.md (F53) — **the one lever that works with no human in the loop and no hook**, since it applies to the automatic pass; and the as-you-go artifact doctrine, since disk artifacts survive by being re-readable rather than by surviving the summary.
- **F83** — **What a lead loses to compaction is exactly the material mochiko commands are built from.** The obligated reads — `command-shape.md` (both layers), `mochiko:loop-discipline`, `agent-dispatch.md`, conditionally `sized-end-stage-review.md` (`command-shape.md:48-55`) — enter as file reads, i.e. message history, and the survival table (F54) has **no row for file reads**; F55 states message history is summarized away. The command file's own body is also message history: `prompt-caching` — *"Skills and commands **inject their instructions as user messages** at the point of invocation."* By contrast the project-root CLAUDE.md operating manual is *"Re-injected from disk"*, and invoked skill bodies return truncated under the 5,000/25,000-token caps, oldest dropped first, start-of-file kept (F54, F56).
- **F84** — **A compacted lead is a re-reading lead, and the doctrine tax is already the measured per-run floor.** F36's ~7,193 tokens est. is paid once per run and, per `workflow-token-reduction/record.md:48`, again on every `/resume` — *"teams don't survive resume … so a paused run re-reads it."* Compaction is a third occasion for that re-read that no mochiko record has costed.
- **F85** — **The lead can see its own context; it cannot see any seat's.** `context-window`: *"To see your actual context usage at any point, run `/context` for a live breakdown by category with optimization suggestions"*; plus the per-session status-line fields (F67). Against F66's total absence on the seat side. **No mochiko doctrine line references either surface.**
  *(Lead note at review, verify B1: "the lead" here is the **human viewing the session**, not the model — corrected by FD-1 part (b) below.)*
- **F86** — **Absence: no mochiko record, decision, or backlog item has ever proposed a lead-side compaction practice.** F24's complete live hit list contains no such proposal; F31 is the only lead-context disposition and it is acceptance; F32's threshold dissolution was about **teammate** occupancy, not the lead's own.

*(end verbatim reality map)*

### Fact-dispute answers (fact-checker, verbatim)

#### FD-1 — can a model observe its own context occupancy? (routed from the review; settles RI-14)

FD-1 SETTLED. **D3's claim holds for the model: no documented mechanism lets a model — lead or teammate — read its own context occupancy. But the reviewers are right that the map didn't carry the fact, and settling it surfaced a documented surface I had missed, which forces two errata (F66, F85) and softens one inherited claim.** Facts only, no recommendation.

**(a) Can the model read its own context occupancy? — NO documented mechanism, on four checked routes.**

- **FD1-1 — No tool.** `code.claude.com/docs/en/tools-reference`, re-grepped this run for `token` / `context window` / `used_percentage` / `occupancy`: the only hits are the `Agent` row (*"Spawns a subagent with its own context window"*), the Agent-behavior prose, a WebSocket "subprotocol token", and `Read`'s file-size token limit. **No tool returns, reports, or queries context occupancy.** `TaskGet` is *"Retrieves full details for a specific task"*; `TaskOutput` is *"Retrieves output from a background task."* Neither carries usage.
- **FD1-2 — The status line is display, not context.** `code.claude.com/docs/en/statusline`: *"The status line is a customizable bar at the bottom of Claude Code that runs any shell script you configure. It receives JSON session data on stdin and **displays whatever your script prints**."* And, under "How status lines work": *"Claude Code runs your script and pipes JSON session data to it via stdin. Your script reads the JSON, extracts what it needs, and prints text to stdout. **Claude Code displays whatever your script prints.**"* The data flows *harness → script → terminal*. **The model is never named as a consumer anywhere on the page** — an absence, not a denial.
- **FD1-3 — Hook inputs carry no occupancy field.** `code.claude.com/docs/en/hooks`, common input fields, complete list: `session_id`, `prompt_id`, `transcript_path`, `cwd`, `permission_mode`, `effort`, `hook_event_name`. **No token count, no context-window usage, no occupancy field on any hook event** — including `PreCompact`, whose input is just `session_id` / `transcript_path` / `cwd` / `permission_mode` / `hook_event_name` / `trigger`.
- **FD1-4 — Hooks CAN inject context, but would have to derive the number.** `hooks`: *"For `UserPromptSubmit`, `UserPromptExpansion`, and `SessionStart`, stdout is added as context that Claude can see and act on"*, plus the `additionalContext` field inside `hookSpecificOutput` (also available on `SessionStart`, `Setup`, `SubagentStart`, the tool events, `Stop`, `SubagentStop`). So an injection channel into the model exists — but **with no occupancy value in the hook input to inject.** A script would have to compute one from `transcript_path`, which the docs caveat: *"The transcript file is written asynchronously and may lag the in-memory conversation, so it may not yet include the current turn's most recent messages when a hook fires."* **This is a constructible estimate, not a documented occupancy read.**
- **FD1-5 — `/context` is user-typed, and whether its output enters model history is undocumented.** `commands`: *"`/context [all]` — Visualize current context usage as a colored grid."* It is a built-in slash command, so F62 applies (recognized only at the start of a user message; the model's ability to fire slash commands was removed in v2.1.215). **Notable contrast:** the docs are explicit about this for a different command — `prompt-caching` on `/recap`: *"it **appends the summary as command output** rather than replacing your message history, so the cached prefix stays intact."* No equivalent sentence exists for `/context`. **Undocumented either way; I will not infer it.**

**(b) F85 is the HUMAN, not the model — erratum.** The map's F85 reads "the lead can see its own context", citing `/context` and the status-line fields. Per FD1-2 and FD1-5 both are **human-facing surfaces**: `/context` is user-typed and renders a grid; the status line prints to the terminal bar. **Corrected F85 wording:** *"The **human viewing the lead's session** can see the lead's context occupancy — `/context` for a live breakdown, and a configured status line for a continuous read. The **model** has no documented route to either. No mochiko doctrine line references either surface."* The substantive point the map was making — a visibility asymmetry between lead-side and seat-side — survives, but see (c): the asymmetry is smaller than F66 claimed.

**(c) No mechanism lets a model state its own fill non-invented — BUT a documented per-subagent token surface exists for the human, and I missed it. Erratum on F66.**

- **FD1-6 — `subagentStatusLine` exists and carries per-agent context data.** `statusline`, "Subagent status lines", verbatim: *"The `subagentStatusLine` setting renders a custom row body for each [subagent](/docs/en/sub-agents) shown in the agent panel below the prompt. Use it to replace the **default `name · description · token count` row** with your own formatting."* And: *"The command runs once per refresh tick and receives all visible subagent rows as a single JSON object on stdin. The input includes the base hook fields, a `columns` field with the usable row width, and a `tasks` array. Each task has `id`, `name`, `type`, `status`, `description`, `label`, `startTime`, `model`, `effort`, **`contextWindowSize`**, **`tokenCount`**, **`tokenSamples`**, and `cwd`."*
- **FD1-7 — And a plugin can ship one.** Same section: *"The same trust and `disableAllHooks` gates that apply to `statusLine` apply here. **Plugins can ship a default `subagentStatusLine` in their [`settings.json`](/docs/en/plugins-reference#standard-plugin-layout).**"*
- **FD1-8 — Erratum to F66.** F66 stated "no per-teammate context or token display is documented anywhere", grounded in `agent-view` and `interactive-mode`. That is **too strong**: the agent panel's **default** subagent row already displays a **token count**, and `subagentStatusLine` exposes `contextWindowSize` + `tokenCount` + `tokenSamples` per row. **Corrected F66:** *"`agent-view` and `interactive-mode` carry no per-agent context display, but `statusline`'s `subagentStatusLine` does: the agent panel's default row is `name · description · token count`, and a custom row script receives `contextWindowSize`, `tokenCount`, and `tokenSamples` per task. This is a **human-facing terminal surface**, readable by a shell script, **not by the lead model.**"*
- **FD1-9 — Two limits on FD1-6, both undocumented rather than negative.** (i) The doc says *"each **subagent** shown in the agent panel"*; **it never states whether agent-team teammates appear in that `tasks` array.** Teammates do share that panel (agent-teams: *"The lead's terminal lists teammates in the agent panel below the prompt input"*), so coverage is plausible — but unconfirmed, and I am not asserting it. (ii) `tokenCount` is **not defined** on the page — whether it means live context occupancy or cumulative usage is unstated, which is precisely the live-vs-cumulative ambiguity `standing-seat-lifecycle` F-e(a) left open on the 300K sighting. `contextWindowSize` being alongside it is suggestive of a fill ratio, not proof.
- **FD1-10 — Net answer to (c).** For **a model stating its own fill**: no documented mechanism — direct read impossible (FD1-1 through FD1-3), and the one injection channel (FD1-4) has no occupancy value to carry, only a lagging transcript to estimate from. **So the record's D3 phrase — "a seat cannot see its own token count either, so a self-reported 'context-health' line is an invented number" — is CORRECT AS STATED for the model.** What changes is the *neighbouring* claim: a **human**, and a **script**, can see a per-agent token count today, and a plugin can ship the row that displays it.

**One inherited claim this softens, flagged because a ruling rests on it.** `standing-seat-lifecycle/record.md:193` dissolved threshold-triggered recycling on the finding *"no documented surface lets the **lead** see a teammate's context occupancy — thresholds have nothing to trigger on"*, and D1's escape hatch was worded as a *"coarse, **instrument-blind** override"* because F-e(a) found the pane figure's meaning undetermined. **Both remain true of the lead-as-model** (FD1-1–FD1-5), and the meaning of `tokenCount` is still undefined (FD1-9ii). **But "instrument-blind" is now the weaker half of the claim:** `context_window.used_percentage` is documented as *"Pre-calculated percentage of context window used"*, `context_window_size` as the window ceiling, and `total_input_tokens` as *"Token counts currently in the context window, from the most recent API response"* — so a human reading a configured status line has a **calibrated** fill figure, not a bare number. That resolves F-e(a)'s ambiguity **for the session-level status line**; it does not resolve it for the panel's `tokenCount`.

*(end verbatim FD-1 answer)*

#### FD-2 — plan's completeness reviewer lifetime (routed from the review; settles RI-15, unblocks the RI-7/DQ-6 merge)

FD-2 SETTLED — **the reviewers are correct and F9 is defective.** Plan's completeness reviewer reaches **≤15 review passes**, two-and-a-half times plan's longest producer, and the inherited "≤6 everywhere else" figure **excludes it by construction on two independent grounds**. Facts and a corrected F9; no recommendation.

**(a) Plan's completeness reviewer's maximum exposure, from `plan.md` alone: ≤15 review passes, standing across the whole run.**

- **FD2-1 — It grades five stages, named explicitly in the Goal.** `plan.md:22-24`, verbatim: *"`devils-advocate` returned `ready` on **the analysis, the architecture coverage, the detailed design, the mapping and the tasks**, each grounded in the files"*. Five gradings, enumerated in the done-condition itself.
- **FD2-2 — Its seat row confirms all-stage scope and standing lifetime.** `plan.md:40`: *"| completeness | `devils-advocate` × `review-plan-artifacts`, then `review-task-artifacts` | grades coverage / measurability / consistency **at every stage**, architecture coverage, and conformance to the approved architecture; never authors | **cold at first review, standing after** | peer-edged with the active producer; grades only when you open the pass |"*
- **FD2-3 — The cap multiplies against those same five stages.** `plan.md:90-91`: *"**Bounds:** cap **3** produce↔review rounds **per stage** (analysis · architecture · detailed design · mapping · tasks), you count each"*. **5 stages × 3 rounds = ≤15 review passes**, and the seat is standing from the first review through the last, i.e. spanning the entire run after analysis.
- **FD2-4 — Every producer in plan is bounded well below that.** `plan.md:36` producer `technical-analyst` — *"standing across analysis + detailed design"* = 2 stages, **≤6**. `plan.md:37` `system-architect` — *"standing, architecture stage"* = 1 stage, **≤3**. `plan.md:38` `task-architect` — *"standing across mapping + tasks"* = 2 stages, **≤6**. The feasibility seat is the shortest: `plan.md:39` *"cold once the analysis is authored; **lead-gated** thereafter"*, grading analysis feasibility then the architecture pass, with `plan.md:96-97` re-firing it *"only on a structural change"*.
- **FD2-5 — Unit caveat, stated so the number isn't over-read.** ≤15 counts **review passes**, not messages. The record's implement figure (*"~15–45+ turns"*) counts turns including retries. A review pass is at least one turn, so ≤15 passes is a **floor** on the reviewer's turn count, not a like-for-like substitute for the implement figure. The two are the same order of magnitude; I am not claiming they are the same measurement.

**(b) The "≤6 everywhere else" figure EXCLUDES plan's completeness reviewer — on two grounds, either of which is sufficient.**

- **FD2-6 — Ground one: the measurement is producer-keyed, in its own words.** `standing-seat-lifecycle/record.md:54`, verbatim: *"Both dwarf the ≤3–6-round standing **producers** of every other command."* The "≤6 everywhere else" shorthand used later at `:181` and inside **D1's own rationale at `:266`** is a compression of that producer-scoped sentence. `model-tiered-seats/record.md:81` removes all doubt: *"All non-implement standing **producers** are ≤3–6 rounds."* **Reviewers were never in the denominator.**
- **FD2-7 — Ground two: the plan row listed the reviewer as standing but gave it no round figure.** `standing-seat-lifecycle/record.md:48`, verbatim: *"| `plan.md` | **producer** (technical-analyst) across **both phases**; **completeness reviewer** standing both phases | feasibility reviewer cold, **once** (re-fire only on structural change) | producer up to **~6** (≤3/phase × 2) |"*. The seat is named as standing; the rounds column reports **only the producer**. The reviewer's own exposure was never computed. `model-tiered-seats/record.md:70` repeats the pattern — its completeness-reviewer row's rounds cell is a bare dash.
- **FD2-8 — And the figure was computed against a plan that no longer exists.** Both records say *"both phases"* / *"≤3/phase × 2"* — plan had **two** phases on 2026-07-23, and `tasks` was a **separate command** with its own two phases (`standing-seat-lifecycle/record.md:49`). Under today's file plan has **five** stages (FD2-1/FD2-3), because tasks merged in (map F3) and an architecture stage was added. **So the reviewer's own lifetime grew from ≤6 to ≤15 independently of the scoping error.** Even had the original derivation counted reviewers, it would now be stale.

**(c) Corrected fact — supersedes the relevant half of F9.**

> **F9 (corrected).** **`implement` is the longest-running team by construction** — its producer's lifetime is bounded by the feature's cycle count, which the command file leaves unbounded, and the one built feature ran 15 cycles at up to 3 retries each plus up to 3 fix passes (`standing-seat-lifecycle/record.md:54`: *"~15–45+ turns"*). **But the runner-up is not a producer: it is `plan`'s completeness reviewer at ≤15 review passes** (`plan.md:22-24`, `:40`, `:90-91` — five stages × cap 3), standing from first review to last, which is **2.5× plan's longest producer** (`technical-analyst`, ≤6) and longer than every standing seat in the library except implement's two. The inherited *"≤6 everywhere else"* (`standing-seat-lifecycle/record.md:54`, `:181`, `:266`) is **a producer-only measurement** — stated as such at `:54` and `model-tiered-seats:81` — **computed when plan had two phases**; it does not bound reviewer seats and does not survive the tasks-into-plan merge. Corrected library ordering by bounded exposure: implement producer (unbounded by cycle count) · implement verifier (~15 verifications + final validation) · **plan completeness reviewer (≤15 passes)** · plan `technical-analyst` and `task-architect` (≤6 each) · setup producer (analysis + ≤3) · specify, slice producers (≤3).

**One consequence I am flagging as a fact about the record's own text, not as advice.** The session's **D4** scopes the cadence rule to *"standing seats whose lifetime spans multiple units"* and enumerates *"implement's producer and verifier, setup's producer, a long session's fact-checker"*, exempting *"cold end-stage seats (e.g. reviewers spawned at convergence)"*. Plan's completeness reviewer is **not** a cold end-stage seat — `plan.md:40` spawns it *"cold at first review, standing after"*, in-loop across five stages — so **it satisfies D4's stated criterion while being absent from D4's enumeration**, and D4's rationale cites the very *"≤6 library-wide"* figure that FD2-6/FD2-7 show excludes it. Whether the enumeration or the criterion governs is a ruling, not a fact; I report only that the two disagree on this seat.

*(end verbatim FD-2 answer)*

#### FD-3 — devolved-branch / P14 scoring across the six commands (lead-routed; sizes DQ-7)

FD-3 SCORED. **One command of six fills P14. `implement` is the only one that binds the devolved branch or names a clearing unit; four explicitly state the absence; `brainstorm` forecloses it a level higher.** A cadence keyed to "the command's clearing unit (P14)" therefore has a denominator in exactly one command. Facts and quotes per command; no recommendation.

**Per-command scoring**

- **FD3-1 — `implement`: BINDS the branch, FILLS P14.** The only `Clearing unit` line in the entire command surface (grep for `clearing unit|checkpoint keying` across all six returns one hit). `implement.md:128-130`, verbatim: *"**Clearing unit + checkpoint keying:** the **cycle**; a surfaced architecture deviation **de-devolves** it, and a non-empty `domain_deps_added` **always** forces the escalated human checkpoint — never auto-approved, no stamp read."* The qualifying condition is at `implement.md:65-70`: the cycle checkpoint *"carries the shape's **devolved branch**, skipped **exactly** when every verification in the cycle is a deterministic CLI check at 100% pass **and** no deviation is reported **and** `domain_deps_added` is empty"*. Its frontmatter advertises it: *"A per-cycle checkpoint carries the shape's deterministic-clean devolved branch."* Note `implement.md:100` carves out the endgame — *"The final validation is lead-routed, never devolved."*
- **FD3-2 — `plan`: NO branch, no P14.** `plan.md:101-102`, verbatim: *"**No devolved branch** — every review here is a judgment grade, so no gate is skipped and no unit clears unread."* Bindings carries no clearing-unit line.
- **FD3-3 — `specify`: NO branch, no P14.** `specify.md:72-73`, verbatim: *"**No devolved branch** — the critique is a judgment grade, never all-deterministic-CLI, so no gate is skipped and every verdict is yours."*
- **FD3-4 — `slice`: NO branch, no P14.** `slice.md:79-81`, verbatim: *"**No devolved branch** — the review is a judgment grade, never all-deterministic-CLI, so no gate is skipped and every verdict is yours."*
- **FD3-5 — `setup`: NO branch, no P14 — and it fails the test for an instructive reason.** `setup.md:132-134`, verbatim: *"**No devolved branch:** every verdict is a Tier-2 judgment grade **with deterministic sub-checks inside**, never all-CLI, so no gate is skipped and no unit clears unread."* Deterministic checks **do** exist inside setup's validation; the branch still doesn't bind, because Layer 2 requires *every* verification in the unit to be a deterministic CLI check (`command-shape.md:213-215`). Partial determinism does not qualify.
- **FD3-6 — `brainstorm`: no P14, and the word "devolved" does not appear in the file at all.** A grep for `devolved` across all six commands returns hits in five; `brainstorm.md` has **zero**. Its Bindings block carries Artifacts · Uncertainty carrier · Fact route · Verify-pass owner · KM landing — **no clearing unit**. What it does state, at `brainstorm.md:40-41`, is the absence one level up: *"**Validation model:** the sized end-stage review of `record.md`; there is no in-loop critique seat."* Since the devolved branch is a property of in-loop clearing, no in-loop critique seat forecloses it by construction. **Whether that satisfies the shape's stated-absence rule** (`command-shape.md:31-34`) **is a grading judgment, not a fact I can settle.** Two facts bearing on it: P14 is declared *"(devolved branch only)"* at `command-shape.md:116-117`, i.e. conditional rather than universally required; and `brainstorm.md` was graded **PASS** in the 2026-07-30 wave ceremony (`.mochiko/decisions/2026-07-30-goal-shape-wave-ceremony.md:21`: *"**Ceremony audit: all five PASS; wave verdict PASS.**"*), so an independent grader already accepted the file as written.

**FD3-7 — The blast radius, stated plainly.** **P14 fill rate: 1 of 6.** A shape-wide cadence keyed to the clearing unit binds `implement` alone and is a **no-op in the other five**, which have no clearing unit to count. `implement` is also the one command whose seat-recycling text was authored and then **reverted** on 2026-07-30 pending D3 (map F30) — so a P14-keyed rule would land exactly, and only, where a reverted mechanism is already queued.

**FD3-8 — What the other five do have that is lead-countable, since the denominator question is really "count what instead".** These are structural units the command files already make the lead count, none of them P14: **rounds** — `specify` and `slice` cap 3 (`specify.md:61`, `slice.md:74`); `setup` caps 3 produce↔validate (`setup.md:112`); `plan` caps *"**3** produce↔review rounds **per stage**"* across five stages (`plan.md:90-91`). **Stages** — `plan`'s five. **Named gates** — G1–G7 (`plan`), G1–G5 (`implement`, `setup`), G1–G4 (`slice`), G1–G3 (`specify`). **`brainstorm` has neither rounds nor numbered gates**: its bounds are per-reviewer, per-survivor, and per-fact (*"per reviewer one cold read … lead↔reviewer argument **max two exchanges per survivor** … one fact-checker dispatch per fact"*, `brainstorm.md:61-64`), and its gates are named rather than numbered (review sizing · survivor rulings · tie-break · acceptance). **So `brainstorm` is the hardest case for any structural cadence, not merely for a P14-keyed one** — and per map F4 its fact-checker is a whole-session standing seat, i.e. it has the lifetime problem without a countable unit.

*(end verbatim FD-3 answer)*

---

## Decisions

### D1 — Scope: the lead manages the seats' compaction, not its own — `Contested` (held at Q8, re-affirmed at review U2; mark per U5 — held-through-challenge ⇒ `Contested`, the same convention as D3)

**Statement (amended at review — original second clause superseded by the fact map):** The
lead's compaction strategy governs the **seats' contexts only**. The lead's own (main-session)
context is out of scope because **no platform lever exists that the lead can pull
unilaterally, mid-run** — ~~"no platform mechanism lets the lead invoke or time its own
compaction"~~ was the original clause, and its timing half is falsified: timing and
what-survives *influence* exists (F81/F82 — human-typed `/compact` at lead-named breaks,
pre-run env config, standing Compact Instructions), but every path is human- or
config-mediated (F80).

**The pre-review conditional (user-declared at Q2; since tested — marked stale at verify
N2):** *if* lead self-compaction is platform-possible → whole team; otherwise seats only.
The condition was tested by the checker: strict self-invocation **not possible** (F80), the
timing half of the premise falsified (F81) — resolved seats-only at Q8 and re-affirmed at
U2; the provisional choice became this ruling.

**Rationale (amended at review):** `/compact` is user-invoked and the model cannot fire it
(F80/F62). ~~"auto-compact fires on its own threshold, not at a chosen moment"~~ —
superseded: the threshold is configurable ahead of the run (F44/F46) and the docs recommend
human-timed `/compact` at natural breaks (F58); the operative fact is narrower — the
lead-as-model holds no lever. A responsibility doctrine can't assign the lead a lever it
doesn't hold. The lead still *shapes what survives* (as-you-go artifacts, the record) —
hygiene, not a lever.

**Resolution (Q8, user-ruled on the full fact map, no lead recommendation):** the checker's
verdict split the conditional — strict self-invocation **not possible** (P1–P3, P6/F80), but
the "no mechanism to *time* it" clause partly falsified (F81: human-typed `/compact` at
lead-named gates · plugin-shippable `PreCompact` block · pre-run env config). **Corrected
inventory (review fold — the Q8 phrasing "seats have zero compaction mechanisms for anyone"
equivocated, RI-9/DQ-3):** the seats have zero *compaction* mechanisms (F65–F72) and exactly
**one reset lever — kill-and-respawn (F69/F70) — the only context lever anywhere in the
system the lead can pull unilaterally**; respawn is a reset, not compaction — it discards
what is not on disk where compaction summarizes and retains. The lead's own session has
timing and what-survives *influence* (F81/F82), none of it unilateral. One counter-fact was
omitted at Q8 and is recorded (DQ-2): `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE` is documented for
"both main conversations and subagents" (F45) — teammate applicability undocumented, inert in
ordinary local sessions (F44); **record-open dogfood probe item per U3.** Presented with the
inversion and both steelmans, the user **held seats-only (A)** at Q8 — and **re-affirmed it
at review (U2) on the corrected inventory**: the doctrine governs exactly the one unilateral
lever the lead holds. This re-affirms the `standing-seat-lifecycle` S3 acceptance of lead
accumulation (F31/F78) with knowledge the prior ruling lacked.

**Rejected at Q8 (steelman preserved):** whole-team scope, levers-as-they-are — lead names the
compaction moment for the human at gates (the docs' own recommended pattern, F58), a standing
CLAUDE.md Compact-Instructions section shaping the auto pass (F53, the no-human lever),
post-compaction re-read discipline for the summarized-away obligated reads (F83/F84), and the
`PreCompact` hook (F48/F49, near the no-kernel line). Rejected on duties-without-unilateral-
levers grounds and scope discipline — **scope discipline is the operative ground for the two
levers needing no human** (post-compaction re-read; the Compact-Instructions section), which
the unilateral-lever ground does not reach (DQ-5 residue, recorded); available to a future
session — this record's map (notably F79–F86) is the ready evidence base.

### D2 — Binding site: `command-shape.md` Layer 2 — `Confident`

**Statement:** The compaction-responsibility doctrine lives in **Layer 2 of
`templates/command-shape.md`** — every team-form command inherits it; a command adds a
per-command parameter line only where its compaction moments genuinely differ from the
shape-generic rule.

**Rationale (amended at review):** the user's framing is "*all* the mochiko agent teams led
by commands" — shape-level by intent. Layer 2 already owns seat transport, standing seats,
and the mesh. ~~"the compaction rule will key on structures Layer 2 already names (gates,
units, checkpoints), so it can be stated once in workflow-generic terms"~~ — superseded
(RI-6 residue / FD-3): Layer 2 names the clearing unit only where the devolved branch binds,
which is 1 of 6 commands, so the generic statement keys instead to **the command's own
counted loop unit** per D6 as re-keyed (U6a), with `brainstorm`'s parameter line the one
override. Single-sourcing: commands reference, never restate.

**Known limit (DQ-4, recorded — unmitigated; corrected at verify B5):** the Layer-2 home
enters the lead's context as message history and is summarized away by the lead's own
compaction (F83). ~~Survival rides the existing resume-from-workspace-evidence Recovery
discipline~~ — that discipline triggers on pause/resume, never on a context condition
(F12/F19), and a post-compaction re-read duty was a **rejected lead-side lever at Q8** (D1
Rejected), so the exposure stands **unmitigated, an accepted risk under D1's scope** — the
future lead-side session D1 names inherits it, with F84 pricing the re-read it would cost.

**Rejected (RI-13, recorded at review):** the project CLAUDE.md governance region (survives
compaction re-injected from disk, but setup-owned and per-project — a plugin cannot land
doctrine there); skill-body carriage (re-injected but truncated under the 5k/25k caps, oldest
dropped — F54/F56); per-command restating (violates single-sourcing and meets the same
message-history fate).

### D3 — Trigger: structural cadence at boundaries, plain — `Contested`

**Statement:** The lead acts on a seat's context at **named structural moments only** — every
N units (cycles, rounds), at gate pauses, at review start — a cadence the lead counts. No
occupancy measurement, no seat self-report, no load-weighted counting.

**Rationale (user's, as argued by the lead's steelman):** neither the lead nor the seat can
*measure* seat occupancy — `standing-seat-lifecycle` dissolved thresholds on the no-lead-visible-
surface fact, and a seat cannot see its own token count either, so a self-reported
"context-health" line is an invented number. A cadence in structures the lead already counts is
deterministic — the same reason loop bounds are counted, not felt. Extends the ruled recycling
floor (implement producer ~≥3 cycles + gate-pause check) rather than forking it.

**Rejected:** B (self-report-driven) and C (cadence floor + self-report override) — recommended
by the lead, rejected for resting on an unmeasurable signal; A′ (cadence counted in lead-side
dispatch/round load, offered as a deterministic refinement) — rejected for simplicity: cadence
counts structural units only. User held A through one challenge → `Contested` per convention.

**Review notes (recorded):** FD-1 grounded the rationale — no documented mechanism lets a
model, lead or teammate, read its own context occupancy (FD1-1–FD1-5); "a self-reported
context-health line is an invented number" stands as stated. The absorbed **user escape
hatch** is restated here so it has a home under the cadence (DQ-8 residue): the user may
order a recycle at any gate; per FD-1 a configured status line gives the human a
**calibrated** fill figure (`used_percentage`), and the agent panel's default row shows a
per-agent token count (FD1-6/FD1-8) — no longer wholly instrument-blind for the human,
though teammate coverage of `subagentStatusLine` and the meaning of `tokenCount` are
**recorded-open probe items**.

### D4 — Governed seats: standing multi-unit seats only — `Confident`

**Statement (enumeration corrected at review per FD-2; criterion unchanged, user re-affirmed
U1):** The cadence rule governs **standing seats whose lifetime spans multiple units** — the
**criterion governs, not any list**. Corrected ranking by bounded exposure (FD-2's corrected
F9, reproduced as cited): implement's producer (unbounded by cycle count; the only *measured*
seat, ~15–45+ turns) · implement's verifier (~15 verifications + final validation, derived; unmeasured — RI-16) ·
**plan's completeness reviewer (≤15 review passes)** · plan's technical-analyst and task-architect (≤6 each) ·
setup's producer (analysis + ≤3) · specify's and slice's producers (≤3). **Beyond FD-2's
ordering, D4 adds (verify B3/B4):** a long session's **fact-checker** — no unit bound exists
for it; it **meets the criterion but is cadence-exempt** for want of a countable unit,
governed by the user's gate-time recycle override per D6's brainstorm parameter line.
"Cold at first review, standing after" seats are **governed when multi-unit** —
cold arrival guarantees freshness only at the first round, not the fifteenth (RI-7). Exempt:
cold end-stage seats (reviewers spawned at convergence). Edge: setup's producer (≤3 rounds)
meets the criterion but a ≥3-unit cadence fires at most once, at end of life — effectively
ungoverned in practice (DQ-6). ~~The original enumeration ("implement's producer and
verifier, setup's producer, a long session's fact-checker") and its "≤6 library-wide"
citation~~ are superseded — that figure was producer-only and pre-dated the tasks-into-plan
merge (FD2-6/FD2-8).

**Rationale:** cold arrival is a freshness guarantee at the seat's own stage — Layer 2
defines a respawn as cold by design — so cadence-recycling a convergence-stage seat spends a
respawn on freshness it already has. The accumulation problem concentrates in standing
multi-unit seats (measured for implement's producer; bounded-derived for the rest, FD-2).
Per-command exceptions ride D2's escape hatch (a parameter line), not the generic rule.

**Re-affirmed at review (U1)** with the corrected enumeration in view.

**Rejected:** A (uniform all-seats) — pays for no-op respawns; C (per-command marking as the
rule) — D2 already provides it as the exception path.

### D5 — One doctrine: the lifecycle rulings generalize into Layer 2 — `Confident`

**Statement (amended at review):** The `standing-seat-lifecycle` recycling machinery becomes
the **generic Layer-2 mechanism** of this doctrine: cadence-triggered (D3), scoped to
standing multi-unit seats (D4), **recycling-by-respawn as the default move** — respawn briefs
built from existing artifacts only, cold-by-design freshness, successors under a **versioned
name with the lead re-announcing the seat** (~~same-name successors~~ superseded pending the
v2.1.199 name-refusal check, F70/P26 — same-name reuse only if that check passes at build).
Implement's ruled numbers (~≥3-cycle floor + gate-pause-**duration** check — a cache-warmth
condition on the trigger, not a per-gate observation duty (RI-10); per-slice verifier
recycling) survive as that command's parameter values under D2.

**Marking (amended at review, U4a):** the lifecycle rulings are **absorbed with one ruled
exception: D6 supersedes standing-seat D3's clause "declared in each command's seat-roster
[PARAM]"** — the lifecycle-policy *location* moves to the Layer-2 default with per-command
overrides. Every other clause is absorbed unchanged: their record stays the rationale home
for the machinery; this session's record is the home for its promotion to shape-wide
doctrine. (KM landing: the supersession wording lands in **both indexes** and the
`DECISIONS.md` standing-seat row at close — statuses agreeing.)

**Rationale (amended at review, RI-8/DQ-3):** the lifecycle rulings are ~~the proven shape of
exactly this move~~ **ruled but unbuilt** (F29; the implement edit was authored and reverted,
F30) — and respawn is a **reset lever, not compaction**: it discards everything not on disk
where compaction summarizes and retains (DQ-3), so the doctrine's value rests on
respawn-brief sufficiency (a named dogfood watch-item, F26) and extends to seats the original
ruling never measured (FD-2). **The generalization is a deliberate, named bet (user
re-affirmed, U1):** two parallel doctrines about the same act would drift; one doctrine with
per-command parameters is the shape's existing pattern; the risks ride the watch-items and
the probe rider below.

**Sequencing invariant (U4c, honoring the 2026-07-30 revert's gate):** the Layer-2 lifecycle
rewrite (standing-seat D3 as amended by this session) lands **first**; no per-command
recycling text ships before it. **Cost rider (DQ-11, recorded-open):** no per-respawn token
figure exists for any seat; costing one recycle rides the OTel/dogfood probe (F28/F38/F39),
which may tune D6's default value — mirroring the original ruling's own gating discipline
(F27).

**Rejected (RI-13, recorded at review):** keeping the machinery implement-scoped with Layer 2
pointing at it (recreates the two-doctrine drift this ruling exists to prevent); a fresh
shape-native mechanism ignoring the ruled machinery (discards ruled work, violates the
absorption pattern).

**Open under this ruling (fact-blocked):** whether instruct-the-seat-to-compact exists as a
second instrument beside respawn — pending the fact-checker's item 4 (teammate compaction
mechanisms). If it exists, a later decision places it; respawn remains the default either way.
*Resolved by the priority answer (P18–P25): no such instrument exists for anyone — respawn is
the only seat-side move.*

**Watch-item (fact-sourced, P26; aligned at verify N1):** the same-name-successor design the
absorbed ruling assumed rests on a pre-v2.1.199 "latest wins" behavior; current docs say a
send to a name that now resolves to a different agent is **refused instead of delivered**.
Per the amended Statement, the ruling's **default is versioned names (`producer-2`) with the
lead re-announcing the seat**; same-name reuse is the conditional upgrade, available only if
the name-refusal check passes at build.

### D6 — Cadence default lives in Layer 2, command-overridable — `Confident`

**Statement (re-keyed at review, U6a — the original shape-slot-P14 keying superseded by
FD-3; re-worded at verify B2/N4):** Layer 2 states the default cadence — **at each gate
pause, count each governed seat's completed loop units and recycle at ~≥3** — counting,
never per-seat observation (D3/RI-10), with D5's gate-pause-**duration** (cache-warmth)
condition composing on the same trigger — the unit being the one the command's Bounds
already make the lead count (implement: the **cycle** · specify: the **produce↔critique
round** · slice: the **produce↔review round** · setup: the **produce↔validate round** ·
plan: the **produce↔review round**, counted across its five stages) — and a command writes a
parameter line only to override. ~~"recycle at ~≥3 of the command's own clearing units (P14)"~~ had a
denominator in exactly **1 of 6 commands** (FD3-7: shape slot P14 is devolved-branch-only,
filled by implement alone) — a silent no-op in the other five. `brainstorm` counts no sequential loop
unit (FD3-8) and carries the one required parameter line: its standing fact-checker is
cadence-exempt, governed by the user's gate-time recycle override (D3's escape hatch).
~~"the only current heavyweight needs zero edits"~~ superseded (RI-2, U4b): implement
conforms on the producer floor but carries **one explicit override as a parameter value —
its verifier keeps the ruled per-slice cadence** (standing-seat D1, F25).

**Rationale (amended at review):** defaults live once in the shape; forced per-command
explicitness (rejected B) adds a mandatory line to every command while a differing command
writes the same override line under either option. The counted-loop-unit key preserves that
while giving five of six commands a real denominator (FD3-8); the **mechanism**
(default-in-shape + overrides) is what the user re-affirmed (U1), the content being the U6a
ruling. The default *value* (~≥3) is probe-tunable per D5's cost rider.

**Note:** D4–D6 were flagged as a three-adoption streak (RI-3/DQ-9); at review the user
**explicitly re-affirmed all three** (U1) with the repairs in view — the `Confident` marks
are genuine as of that re-affirmation; D6's rides the U6a re-key.

---

## Review

**Sizing (user-ruled at convergence):** momentarily sized **single**, immediately reversed by
the user to **pair** before any reviewer spawned — the pair runs: two cold reviewers
(`devils-advocate` × `mochiko:review-brainstorm`), lens-split **decision-quality** /
**record-integrity**, mutually withheld until findings are formed, one four-message
cross-exam, verify pass owned by the record-integrity reviewer. **Record frozen from reviewer
spawn until every disposition lands (this section excepted).**

**Reports (cross-exam closed at four messages):** decision-quality **14 raised → 12 survived**
(DQ-5, DQ-8 withdrawn as filed, each leaving a Minor residue); record-integrity **18 raised →
17 survived** (RI-6 withdrawn on evidence). Both independently recommend **critical-gaps**.
**Cross-set merge (lead-owned):** 7 duplicate pairs merged (RI-3+DQ-9 · RI-4+DQ-13 · RI-5+DQ-12
· RI-7+DQ-6 · RI-9+DQ-3 · RI-12+DQ-14 · RI-17+DQ-10), the merged RI-17/DQ-10 row then folded
into the RI-3 repair as an eighth reduction (29 − 7 − 1) → **21 substantive merged survivors** (9
Critical · 7 Important · 5 Minor, lead-classified) **+ 2 reviewer fact disputes** (FD-1
model-self-observability · FD-2 plan-reviewer lifetime) **+ 1 lead-routed fact question**
(devolved-branch scoring across the six commands, DQ-7's blast radius) — all three with the
standing fact-checker. One reviewer severity disagreement (RI-17/DQ-10, Minor vs Important)
resolved by the lead: folded into RI-3's repair at Important. Dispositions land below as
ruled; full per-survivor table with the folds.

### Dispositions (all 23; user batch U1–U6 ruled 2026-07-31)

**User batch:** **U1** D4/D5/D6 explicitly re-affirmed (D5 as a named bet on unbuilt
machinery; D6 as the mechanism over the U6a content) · **U2** D1 stands on the corrected
inventory · **U3** the omitted F45 fact → record-open probe · **U4** adopted whole
(supersession clause + "zero edits" struck with the verifier override stated + D3-first
sequencing invariant) · **U5** D1 → `Contested` · **U6** option (a), the counted-loop-unit
re-key.

| # | survivor(s) | sev | disposition |
|---|---|---|---|
| 1 | RI-1 | C | user-ruled U4a — supersession clause in D5's Marking; both indexes + decisions row at close |
| 2 | RI-2 | C | user-ruled U4b — "zero edits" struck; verifier per-slice override stated in D6 |
| 3 | RI-3 + DQ-9 + RI-17/DQ-10 (folded per lead merge) | C | user-ruled U1 — explicit re-affirmation; D6's Note replaced; D5's same-name clause made conditional (the RI-17/DQ-10 repair) |
| 4 | RI-4 + DQ-13 | C | resolved — D1 Statement/Rationale amended in place, superseded wording struck through |
| 5 | DQ-1 | C | user-ruled U4c — sequencing invariant in D5 |
| 6 | DQ-2 | C | user-ruled U3 — omission recorded in D1's Resolution; probe record-open |
| 7 | DQ-3 + RI-9 | C | resolved — equivocation ruled: respawn = reset lever, not compaction; one inventory statement in D1; D1 re-affirmed on it (U2); null-option risk named in D5's rationale |
| 8 | DQ-4 | C | recorded-open — D2 known-limit recorded **unmitigated** (verify B5: Recovery fires on pause, never a context condition; the re-read duty was Q8-rejected) — accepted risk under D1's scope |
| 9 | DQ-7 (+RI-6 residue) | C | user-ruled U6a — D6 re-keyed to counted loop units; D2's workflow-generic premise corrected |
| 10 | DQ-6 + RI-7 | I | resolved via FD-2 — D4 enumeration corrected; cold-then-standing class classified governed-when-multi-unit |
| 11 | DQ-11 | I | recorded-open — cost rider in D5; D6 value probe-tunable |
| 12 | RI-8 | I | resolved — "proven" → "ruled but unbuilt"; the bet named |
| 13 | RI-10 | I | resolved — "duration" restored as a cache-warmth condition in D5 **and** composed into D6's Statement (verify B2) |
| 14 | RI-11 | I | user-ruled U5 — D1 `Contested`; convention stated on D1's header |
| 15 | RI-12 + DQ-14 | I | resolved — Status line reset; index refresh rides the close ritual |
| 16 | RI-13 | I | resolved (pairs with #8) — Rejected roads recorded on D2 and D5 |
| 17 | RI-5 + DQ-12 | M | resolved — D6's live citations qualified "shape slot P14" (the struck original quote kept as written); P-namespace distinct (verify N3) |
| 18 | RI-16 | M | resolved — measurement attributed to the producer; verifier marked unmeasured (D4) |
| 19 | RI-18 | M | resolved — commentary marker inserted before the priority answer's closing paragraphs |
| 20 | DQ-5 residue | M | resolved — operative-conjunct note in D1's Rejected |
| 21 | DQ-8 residue | M | resolved — F32 escape hatch restated under D3 with FD-1's calibration note |
| 22 | RI-14 / FD-1 | fact | settled — D3 grounded; F66/F85 errata landed verbatim |
| 23 | RI-15 / FD-2 | fact | settled — F9 corrected verbatim; fed #10 |

**Recorded-open items for the landing:** teammate applicability of
`CLAUDE_AUTOCOMPACT_PCT_OVERRIDE` (U3) · `subagentStatusLine` teammate coverage + `tokenCount`
semantics (FD-1) · cost-one-recycle (DQ-11) — those three riding the existing OTel/dogfood
probe watch — plus **D2's compaction exposure (DQ-4), an accepted risk inherited by the
future lead-side session D1 names, not a probe item** (verify nit 2). **Reviewer severity disagreement (RI-17/DQ-10)** resolved by lead merge into #3 at
Important. **Verify pass:** owner reviewer-ri (record-integrity).

**Round 1 — NOT CLEAN:** 14/23 verified clean; **5 blocking** (B1 three superseded map facts
unmarked at their sites, F9/F66/F85 · B2 D5↔D6 gate-pause contradiction · B3 D4↔D6
fact-checker contradiction · B4 D4's FD-2 citation drift · B5 D2's mitigation cited a
trigger that never fires on compaction) + **5 non-blocking** (N1 watch-item default/fallback
inversion · N2 stale D1 conditional paragraph · N3 P14 qualifiers + a false row claim · N4
wrong unit labels · N5 merge-arithmetic step unstated, RI-17 untraceable from the table) +
1 observation. **All repaired same round** — pointer notes at the three fact sites; D6
re-worded (count-not-observe, duration condition composed, per-command loop names); D4
ranking reproduced as cited with the fact-checker a marked D4 addition, cadence-exempt per
D6; D2's limit recorded **unmitigated** (the Q8-rejected duty not resurrected), row #8 →
recorded-open; watch-item aligned; the D1 conditional marked tested; shape-slot qualifiers +
row #17 corrected; merge arithmetic stated (29 − 7 − 1) with RI-17 traced in row #3; the
commentary marker gains the equivocation clause.

**Round 2 — CLEAN** (bounded to the repairs and their interaction with the clean 14):
**10/10 repairs verified landed**, no regression in the clean 14; 3 non-blocking nits
returned and applied with this result — the implement-verifier derived bound carried into
D4's ranking · DQ-4 added to the recorded-open register marked non-probe · the Status line
refreshed to the live resume state. Reviewer's closing: *"with 23/23 dispositioned and the
repairs verified, nothing from my lens now blocks acceptance."*

**Clearing verdict (lead): ready.**
