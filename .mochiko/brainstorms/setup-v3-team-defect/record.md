# Setup Dogfood Ran Subagents Again — Diagnosing the Team-Mandate Failure

- **When:** 2026-07-18 · **Status:** accepted (2026-07-18, user: "close and build") · record un-reviewed (bare session — direct skill invocation, no cold pass)
- **Form:** `mochiko:analysis-iterative` direct invocation — lead + user, no team seats; reality map lead-gathered at open; docs fact-check via a `claude-code-guide` subagent (F2)
- **Topic:** a dogfood run of `/mochiko:setup` executed its agents as one-shot subagents rather than the mandated agent team (contrast: `/mochiko:brainstorm`, which runs team-form). If the run was on v3, this is the D7 defect recurring *after* the recorded mitigation shipped.

## Reality map (lead-gathered at session open)

- `plugins/mochiko/commands/setup.md:30-38` hard-requires teams: env check (`CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS`) → refusal when unset; first teammate spawn is the authoritative probe; "Running the loop on one-shot subagents is not a fallback — it is the defect this section exists to forbid."
- `setup.md:62-68` carries the seat idiom v3 adopted from brainstorm: named standing seats, message-the-same-seat continuity, "A fresh spawn per round is the subagent anti-pattern, not a team."
- `BACKLOG.md:126` ("Investigate the D7 substrate defect") pre-registered this exact diagnostic: was the env **unset with a silent fallback** (command-discipline failure) or **set and ignored** (instruction non-compliance)? It records v3's seat/messaging idiom ("dispatch vocabulary removed") as the *likely-mitigation hypothesis*, to be confirmed or refuted on a re-run.
- `BACKLOG.md:129` (v3 dogfood item) names "team-form observed (standing producer seat messaged across rounds)" as the still-pending evidence the ROADMAP substrate item is waiting on. The defect run did not count; this run is the re-run the hypothesis was staked on.
- Plugin at v0.9.0 (setup v3 merged in PR #14, 2026-07-18). The dogfood project's environment (env var state, installed plugin version) is not yet established — first discovery target.
- Known contrast case: `/mochiko:brainstorm` has completed team-form runs on the user's machines (index: v2.x runs with live seats), so the substrate itself has worked here before.

## Findings

**F1 — OT-1 resolved: fork B, checked-then-ignored (user-supplied transcript, kinako project, 2026-07-18).**
Claude Code v2.1.214; plugin cache path shows `mochiko/0.9.0` (setup v3, not stale); the lead itself ran the env probe and echoed `AGENT_TEAMS=1`; it stayed script-compliant to the letter up to the spawn point — G1 mode-select, inline interrogation, G3 ratification stamp, kill-switch check immediately before the spawn, even the seat-announcement beat ("Ratified. Stamping the synthesis, then bringing in the author.") — and then dispatched `mochiko:principal-architect(Author governance surface set)` as a one-shot Agent-tool subagent. User interrupted at that point. **BACKLOG:126's mitigation hypothesis is refuted: v3's seat/messaging idiom did not prevent the defect.** The failure is not instruction decay — every surrounding instruction was followed; the "teammate" framing simply did not translate into team-form mechanics at the tool-call moment.

**F2 — fact-check: the defect is a documented substrate failure mode (claude-code-guide vs official agent-teams docs, 2026-07-18).**
- Since v2.1.178 there is no distinct team-creation step: `TeamCreate`/`TeamDelete` "no longer exist"; `team_name` on the Agent tool is "accepted but ignored"; the session has one implicit team. A teammate is spawned via the **same Agent tool** the subagent path uses — distinguished essentially by a `name:` parameter, its own session with `SendMessage`/shared-task access, and message-the-same-seat follow-ups.
- The env var gates **availability, not choice**: "Without that variable … Claude does not spawn or propose teammates." With it set, nothing forces team-form.
- The docs name this exact failure: *"Claude may sometimes use subagents instead of creating a team. Subagents appear in the same agent panel as teammates, so the panel alone doesn't confirm a team formed. If Claude spawned subagents instead, ask again and explicitly request an agent team."*
- Churn window (v2.1.178→214): implicit-team model stable; reliability fixes only. Unconfirmed from docs: whether team tools (e.g. SendMessage) are context-visible or deferred at spawn time in a given session.
- Implication for the substrate bet: BACKLOG:161's "name capabilities, not version-specific mechanics" kept the command surface small, but the capability-level instruction ("spawn the producer seat") leaves the teammate/subagent fork to a substrate that documentedly resolves it wrong "sometimes." Brainstorm's successful team runs (2026-07-04→16) sit in the same churn window — team-form there may owe as much to luck/version as to its text.

**F3 — side observation, out of scope unless pulled in:** the interrogation ran as `AskUserQuestion` structured prompts rather than conversational turns. One-question-per-turn was honored; whether the form matches the "interrogation as a session, not a form" intent is a separate watch item (BACKLOG:141 names that check).

## Decisions

**D1 — Fix altitude: shared transport recipe + spawn self-check (option A).** `Confident` — lead-recommended with steelmanned alternatives, user-adopted.
- **Statement:** the seat-spawn mechanics land **once** in `templates/agent-dispatch.md` (the template every spawn brief in both team-form commands already cites): a seat = one Agent call carrying `name:`; use the official docs' trigger vocabulary ("create an agent team," "spawn a teammate") alongside mochiko's "seat"; round > 1 = `SendMessage` to the same name, never a fresh spawn. Both team-form commands' hard-requirement sections gain two sentences: the discriminating line (**a spawn without a `name:` is a subagent — the forbidden form**) and the post-spawn addressability check (**verify the seat can be messaged; if not, kill it and respawn as a teammate before proceeding**).
- **Rationale:** the only option that both *prevents* (mechanics legibility + docs-native vocabulary at the decision moment) and *enforces* (the addressability check catches the substrate's documented "may sometimes use subagents" residue), while staying single-sourced against future harness churn where every current and future team-form command already looks.
- **Rejected:** B (in-command mechanics — duplicated, churn-exposed, erodes the command-altitude ruling) · C (self-check only — detection without prevention) · D (reopening D9 — its Contested revisit trigger hasn't fired, and forfeiting the pilot with zero compliant team-form runs observed would decide the experiment by default).

## Open threads

- OT-1: **resolved** → F1 (fork B).
- OT-2: **resolved into D1** — root cause characterized as both mechanism illegibility (teammate spawn is one thin parameter away from subagent dispatch) and documented substrate nondeterminism; D1's two halves address them respectively.
- OT-3: **resolved** (user, Q3) — the run was stopped at the interrupt and never continued. The "completed to acceptance" recap is residue, probably summarizing the earlier D7-era run in the same project (consistent with `.mochiko/memory` pre-existing and detection finding no CLAUDE.md governance region — the v2-era form wrote `constitution.md`, not a region). Consequences: **no full subagent-transport datapoint was acquired**; kinako sits mid-run at a defined recovery state (synthesis ratified, surface set absent → resume at *loop (produce)* per setup.md's recovery table), so a compliant re-run is cheap.
- OT-4: **resolved** → D1.

## Consequences (bookkeeping owed on acceptance)

- `BACKLOG.md:126` (D7 investigation) **closes**: diagnostic ran, fork B (env set and verified, instruction non-compliance at the tool-call level), v3 seat-idiom mitigation refuted, fix ruled (D1).
- `BACKLOG.md:129` (v3 dogfood) **stays open** and inherits verification: D1's acceptance test is the kinako re-run (resume at loop/produce) showing its already-named check — *team-form observed: standing producer seat messaged across rounds* — which also feeds the ROADMAP substrate item.
- Ruling lands in `ROADMAP.md` (Key Decisions or Decision Trail) with a pointer to this record.
- Build surface for D1: `templates/agent-dispatch.md` (transport recipe) + `commands/setup.md` and `commands/brainstorm.md` hard-requirement sections (discriminating line + addressability check).

## Landed (built 2026-07-18, same session, on acceptance — plugin v0.9.1)

- `plugins/mochiko/templates/agent-dispatch.md` → **v3**: new "Seat transport (team-form commands only)" section — seat = one Agent call carrying `name:` in the docs' idiom; round > 1 = `SendMessage` to the same name; post-spawn addressability check with kill-and-respawn remedy; explicit non-applicability to the one-shot-dispatch commands.
- `plugins/mochiko/commands/setup.md` + `commands/brainstorm.md` — hard-requirement sections gained the discriminating line (spawn without `name:` = the forbidden subagent form) + the addressability check, each pointing at the agent-dispatch recipe.
- `BACKLOG.md` — D7 investigation item closed `[x]` with the fork-B resolution; the v3 dogfood item now carries D1's acceptance test (kinako resume at *loop (produce)*); the substrate item gained the 2026-07-18 datapoint ("the team mandate alone doesn't produce teams").
- `ROADMAP.md` Key Decisions — "Team-transport legibility" row (2026-07-18), D9 no-fallback standing, pointer to this record.
- **Verification pending:** the kinako re-run under BACKLOG's v3 dogfood item — team-form observed (standing producer seat messaged across rounds) is both that item's named check and D1's acceptance test.
