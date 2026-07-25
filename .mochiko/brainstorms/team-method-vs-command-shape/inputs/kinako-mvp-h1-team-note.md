# Design note — running a lead + agent team

> Session input, pasted by the user at session open (2026-07-25). Stored verbatim so the
> record reads standalone. Authored outside this session from the kinako MVP-H1 transcripts.

**Subject:** the MVP-H1 build session (shipped `kinako-hook` binary), 2026-07-25 00:15–02:03 UTC.
**Outcome:** PR #10 / commit `504766b` — 56 files, +5,507 / −286, six TDD cycles, all gates green.
**Source:** session `de55b447-5e50-4d46-9b90-a2140f5bd2ff` + its four teammate transcripts under
`~/.claude/projects/-Users-deepeshadmin-Documents-GitHub-kinako/de55b447-*/subagents/`.

This records *how the team worked*, not what it built. It is a working-method note, not a ruling.

## The cast

| Seat | Agent type | Stream | Model |
|---|---|---|---|
| lead | (main session) | ruling, structuring, unblocking, commit/PR | opus |
| architect | `mochiko:task-architect` | authored the task doc, then idle | opus |
| core-engineer | `mochiko:staff-engineer` | cycles C1 → C2 → C3 → C6 | opus |
| ui-engineer | `mochiko:staff-engineer` | cycles C4 → C5 | opus |
| qa | `mochiko:qa-engineer` | verifies both streams, final whole-suite | opus |

All four ran as `in_process_teammate`, `bypassPermissions`, in **one shared working tree**.

## Lifecycle

| Time (UTC) | Phase |
|---|---|
| 00:36 | Lead puts the one architectural fork to the founder via `AskUserQuestion` (app-less vs app-served hook). Ruling: app-less. |
| 00:38 | Lead creates 5 tasks with descriptions detailed enough to be handed off. |
| 00:42–00:49 | **architect** spawned *synchronously* (lead waits 7 min). Produces one artifact: a task doc in scratchpad with cycles, per-task file paths, fixed seam signatures, and team-coordination rules. |
| 00:50 | Lead spawns core-engineer, ui-engineer, qa *in one message* (concurrent), then sets task owners and `blockedBy` edges. |
| 00:50–01:39 | **The lead goes quiet.** Engineers build; qa verifies; all three talk to each other. Lead sends exactly 2 messages in 108 minutes. |
| 01:39 | Both streams report complete → lead commissions qa's final whole-suite validation. |
| 01:43 | qa: **PASS** — 635/635 tests, 232/232 integration, coverage 95.2% vs baseline 95, fresh `dart compile exe` end-to-end under `env -i`. |
| 01:44–02:03 | Lead alone: commit, branch, PR #10, CI red, root-cause, fix, CI green. |

## Communication topology

The striking number: **30 peer-to-peer messages, 2 from the lead.**

| From → | core | ui | qa | lead | total sent | inbox |
|---|---|---|---|---|---|---|
| core-engineer | — | 4 | 8 | 0 | 12 | 3 |
| ui-engineer | 5 | — | 6 | 0 | 11 | 5 |
| qa | 3 | 3 | — | 1 | 7 | 11 |
| lead | 1 | 0 | 1 | — | 2 | — |
| architect | 0 | 0 | 0 | 0 | **0** | 1 (its spawn) |

This is the design: **verification traffic never routes through the lead.** Engineers hand cycles
straight to qa and continue without waiting; qa replies PASS-with-evidence or FAIL-with-command.
The lead is an exception handler, not a switchboard.

## The three mechanisms that made concurrency safe

**1. Seam signatures fixed before anyone starts.** The task doc §3 froze three files — `hook_health.dart`,
`hook_binary_port.dart`, `check_hook_health.dart` — down to enum members and constructor shapes.
ui-engineer landed them 3 minutes after spawn and messaged core-engineer: *"the C4 seam is on disk and
green. Your C3 adapter is unblocked… no signature deviations, nothing to renegotiate."* Both streams
then ran flat out against a contract neither could drift from.

**2. Contested files have exactly one owner.** `app_composition.dart`, `composition_root.dart`,
`main.dart` were declared ui-engineer's, with a binding rule for core-engineer: *"If you believe an edit
to a contested file is unavoidable: SendMessage first, never edit-and-tell."* core-engineer instead sent
a **wiring snippet** (constructors, call sites, launch ordering) for ui-engineer to apply. Zero write
conflicts on the three highest-contention files in the repo.

**3. Every wait has a named non-blocking fallback.** ui-engineer's prompt said: if core's T3.8 snippet
hasn't arrived when C4 finishes, *"work is NOT blocked-idle: prepare C5's view-model tests that depend
only on C4, and SendMessage to ask for status."* It asked at 00:58 and got the answer at 01:08 — ten
minutes that cost nothing.

## What worked

- **Independent re-verification caught real defects.** qa re-ran every `TEST:` task itself rather than
  trusting reports, and checked *premises* rather than accepting them — when ui-engineer argued a sibling
  S4 view-model didn't need the same fix, qa went and measured both probe bounds and poll intervals
  before agreeing.
- **Cross-stream critique found bugs neither owner would have.** core-engineer, reviewing nothing more
  than a failing shared suite, warned twice about a poll-interval / probe-timeout overlap in ui's code.
  ui-engineer's reply: *"your overlap warning was not just a caution — it was a live bug"* — a 3 s poll
  against a 10 s probe timeout, spawning overlapping processes indefinitely.
- **Peers reversed themselves on evidence.** qa withdrew its own earlier advice when core-engineer's
  alternative was better (*"I am withdrawing my C2 advice… byte comparison is strictly stronger than
  SHA-256 here"*). ui-engineer conceded a test-count dispute: *"my 49 was wrong, yours is right."*
- **Two lead messages, both high-leverage.** The first killed an out-of-policy dependency before it
  reached the tree (GI-018/GI-034) and told core-engineer to record it as a *reported deviation*, not a
  silent fix. The second commissioned final validation. Nothing else needed the lead.
- **Self-critique after handoff.** ui-engineer re-probed its own shipped C5 at smaller window sizes
  *after* declaring done, found a `RenderFlex` overflow at 800×600, fixed it, and sent qa an addendum
  telling it to re-verify against the new state.

## What it cost, and what it missed

- **Duplicated gate runs.** Three agents in one tree ran **29 full `flutter test --coverage`**, 37
  `flutter analyze`, 42 `dart format`, 73 targeted test runs, 11 `dart compile exe` — concurrently, on the
  same files. `dart format` twice normalized one agent's files from another's run, generating message
  traffic purely to explain the churn. A load-sensitive perf test flaked under the contention.
- **The architect was a one-shot document generator.** 7 minutes, one artifact, then zero traffic in
  either direction — despite all three teammate prompts naming it as the address for structural
  questions. The one structural question that actually arose went to the lead instead. Either fold
  structuring into the lead, or give the architect an explicit "you will be consulted" checkpoint.
- **qa drifted into being the coordination hub** (inbox 11, the highest of anyone). It absorbed
  cross-stream routing — flake warnings, divergence rulings, "you should know" notices — that isn't
  verification work and has no reason to sit with the verifier.
- **~623k output tokens and ~199M cache-read tokens** across five agents for a 5.5k-line change, in
  1h48m wall clock. The lead alone spent 204k output tokens — more than either engineer — mostly on
  writing three long spawn prompts and on its solo commit/PR/CI phase.
- **The team could not catch the one bug that shipped.** `dart compile exe` doesn't create its `-o`
  parent, `build/` is gitignored, so the new CI step passed six cycles and independent qa verification —
  because every machine involved was *warm* — and failed only on a fresh checkout. **Team verification is
  warm-machine verification.** It does not substitute for a cold-checkout gate.

## Rules to reuse

1. **Freeze the seam before you spawn.** Concurrency is safe exactly to the extent that the interface
   between streams is unnegotiable. Signature-level, not prose-level.
2. **Name contested files and give each one owner**, with *message-first, never edit-and-tell*. Pass
   wiring snippets across the boundary instead of edits.
3. **Give every dependency wait a named fallback task.** "Blocked" should never mean idle.
4. **Route verification peer-to-peer.** The lead rules on policy, deviations, and scope — nothing else.
   Two messages in two hours is the target, not an accident.
5. **Require deviations to be reported, not resolved silently.** Every deviation in this run surfaced in
   a cycle report and survived review.
6. **Designate one runner for the expensive whole-suite gate** (qa). Engineers run targeted tests only.
   That alone would have removed ~20 of the 29 full-coverage runs and most of the format churn.
7. **Add a cold-checkout gate.** The warm-tree blind spot is a *class* of failure — any step depending on
   a gitignored directory passes locally and fails only on a fresh clone.
