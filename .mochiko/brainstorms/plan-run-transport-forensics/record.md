# Plan-run transport forensics — the S1 `author-navigate` run

**When:** run 2026-07-31 (12:52 local start), analysis 2026-08-01 · **Status:** accepted —
bare session (solo lead forensic over a captured transcript, un-reviewed; the
`team-method-vs-command-shape` precedent); R1–R5 user-ruled 2026-08-01, build landed same day
at v0.38.0 (shape v6; audit FAIL→PASS in one fix round + two sweeps — see Landing).
**Input:** [inputs/2026-07-31-plan-s1-run-transcript.txt](inputs/2026-07-31-plan-s1-run-transcript.txt)
— the full 1,844-line session capture of `/mochiko:plan author-navigate --slice S1` in
`mochiko-app` (Claude Code v2.1.220). Line cites below are into that file.
**Version note:** the user labels the run **v0.38.0**; every version surface reachable from this
repo says **v0.36.0** (`plugin.json` on `main` and `origin/main`, the installed-plugins manifest,
the plugin cache: 0.7.0 / 0.28.0 / 0.36.0). Unresolved — possibly a host-side install this
sandbox cannot see. The transcript's observed behavior matches the v0.36.0 `plan.md` +
shape-v5 `command-shape.md` structure exactly (G1–G7, seat roster, 3-round caps,
degrade-with-record G3, no-devolved-branch), so the analysis is graded against those texts.

## Scope

Two questions, both the user's: **why did the run take so long**, and **why did it not follow
Layer 2** (`command-shape.md` team transport). Out of scope: the quality of the produced package
(by the transcript's own evidence it is high — F1/GI-009, F7 plugin-cache-in-user-scope, and the
TM5 reasoning-correction are genuine catches the loop was built to make).

## Fact base — measured

- **44 completed seat tasks**, summed runtime **≈ 285 min (4h45m)**; lead turns sum
  **≈ 74 min** (one 32m45s turn at line 241 — the lead's own GI-009/F1 ledger verification).
  The transcript ends **before the run does** (assembly + G7 still pending at line 1843), so
  total cost is higher than measured.
- Stage × round shape: analysis **3 rounds + a user-authorized touch-up** (cap+1) ·
  architecture 2 rounds + a micro-round (A4) · detailed design 2 rounds + a lead-added round
  (OQ-DM-5) · mapping 3 rounds (at cap) · tasks 3 rounds (at cap). ~14 produce↔review rounds
  total, every one lead-opened and lead-adjudicated.
- The completeness-seat lineage ran **14+ passes** (10 on the original incarnation, 4+ on the
  respawn) — matching TC-D4's "plan's completeness reviewer at ≤15 passes" governed-set ranking
  almost exactly.
- The original completeness seat reached a panel reading of **836k tokens** (user report, line
  1553; F-e(a) caveat — the figure's live-vs-cumulative meaning is undetermined) before the
  user ordered the kill/respawn.
- **Resume-tax evidence:** a one-cell mapping alignment took the late-run task-architect
  **14m32s** (line 1774) against 3m25/3m27 for comparable bounded rounds earlier (lines
  1617, 1827); one-line stamp/verdict resumes cost 20s–1m36s each, every one re-materializing
  the seat's full transcript (every send logs "resumed from transcript", e.g. lines 353, 400,
  607, 1228).
- **Ritual cost with no durable effect:** two compact waves + one mid-review compact (lines
  1266–1330, 1429–1495) = seven full-transcript resumes; the lead itself later concedes
  "compaction doesn't reclaim the transcript's token accounting for a long-lived agent" (line
  1556). A ~5-minute npm-install timeout hunting a mermaid renderer at G3 (lines 966–983),
  then the correct degrade-with-record.
- User rulings were **batched well**: six AskUserQuestion batches covering ~12 rulings.

## F1 — the team never existed (the Layer-2 headline)

Every seat in the run was a **background subagent**, not a teammate. The harness says so in its
own voice: name resolution reports "completeness [514357] — **subagent**, in this session"
(lines 1622–1631), and every SendMessage lands as "had no active task; **resumed from
transcript** in the background" rather than delivery to a live teammate. Layer 2 is explicit:
"a spawn without a `name:` is a one-shot subagent — in a team-form command, **the forbidden
transport**", and the command's frontmatter says it *refuses* without agent teams.

The run did not catch this because **the probe's discriminator is broken against the current
harness**: shape v5 says confirm the first spawn yielded "an addressable teammate (a named
agent you can `SendMessage`…)" — but named *subagents* are SendMessage-addressable now, so the
probe passes on exactly the transport it exists to reject (line 144: "the first-spawn probe,
which succeeded"). A doctrine defect, not only a lead-conduct one.

Everything else in this record is mostly downstream of F1.

## F2 — the mesh never ran → full serialization

Layer 2's default: "Producers hand work to the verifying seat **directly**; the lead is the
exception handler, not the switchboard." `plan.md`'s own P5 rows bind the peer edges
("hands finished artifacts to completeness directly"). In the run, **zero peer edges were
exercised** — all ~44 hand-offs routed producer → lead → reviewer → lead → producer. On
subagent transport peer messaging is structurally unavailable, so the mesh *could not* run;
the consequence is that ~74 min of lead turns plus every notification-wake sits on the
critical path, and reviewer↔producer hand-offs that the mesh would overlap were strictly
sequential. (Reviewer *pairs* were correctly spawned in parallel — line 167 — the one
concurrency the run did get.)

## F3 — no context lifecycle → the resume tax compounds

The shipped shape v5 has **no standing-seat lifecycle machinery**: nothing told the lead to
recycle a governed seat, so the completeness seat stood through 14+ passes and the user had to
become the trigger — twice by compact request, finally by ordering the kill ("i am happy for
you to kill this one and spawn a new one", line 1553). This is precisely the mechanism
`standing-seat-lifecycle` D1–D3 diagnosed ("a standing seat re-processes its whole transcript
every round, and every human gate is a probable cache-expiry point re-paying that transcript")
and `team-lead-strategic-compaction` TC-D1–D6 parameterized (counted loop-unit cadence, recycle
at ~≥3 at gate pauses; respawn-as-reset; versioned-name successors) — **ruled the same day as
the run, still unbuilt** (the open BACKLOG "Standing-seat build items", D3-first). The run is
the live dogfood evidence for that queued build:

- The respawn worked and the fresh seat was immediately sharper *and cheaper* (its TM5
  re-grade overturned the author's reasoning, line 1589–1597; bounded verifies at 2m28s).
- The name-takeover on respawn caused one **silently-failed send** ("Nothing was sent",
  line 1623) needing an explicit ref — the exact failure TC-D5's versioned-name successors +
  name-refusal check exist to prevent.
- The compact-request ritual was worse than nothing: seven resumes at full prefill, zero
  durable reclamation, plus narrated housekeeping turns. TC-D1's finding (the model can
  neither invoke nor observe compaction; kill-and-respawn is the only real lever) was
  re-proven at the user's expense.

## F4 — conduct deviations independent of transport

- **Machinery vocabulary in user-facing prose** — banned by Layer 1 ("no 'phase', 'round', or
  'gate' talk") and breached in nearly every lead message: "Round 2 is open" (359), "round 3 —
  the analysis stage's final revision round" (613), "the stage is at its three-round cap"
  (703, 735), G3/G7 by name throughout, "kill-switch check" (809).
- **Housekeeping narrated** — compact acknowledgements relayed turn-by-turn (1301–1330,
  1461–1477) against "teammate housekeeping is never narrated".
- **G3 renderer hunt** — a 5-minute npm timeout inside a firewalled sandbox before degrading;
  the degrade-with-record itself was textbook (988–996, stamp at 1073).

## What held

Credit where the doctrine worked: probe-first ordering; seat announcements + the watch/message
notice; parallel reviewer spawns; author≠grader preserved at every seat; caps counted and the
cap-exit escalated to the user rather than looped (735–742); feasibility re-fired only on
structural change; batched rulings; as-you-go artifacts with DECISIONS/BACKLOG/slices touches
landing mid-run; kill-switch checks before sends; the G3 degrade-with-record path. The
*content* quality of the loop (finding rates 27 → 24 → 8 → 6 → 0 across analysis rounds;
verified closures; honest self-corrections) is the production-only depth bet doing its job.

## Cost attribution — ranked

1. **Volume as designed** (~285 min agent runtime; 5 serial stages × up-to-3 default-FAIL
   rounds × dual reviewers × lead adjudication reads). Plan is *supposed* to be expensive;
   even on perfect transport this run is hours. Not a defect.
2. **Serialization through the lead** (F1→F2): every hop waits on a lead turn; ~74 min of
   lead time inline on the critical path, including one 33-minute verification turn.
3. **The resume tax** (F3): full-transcript prefill on every send to a standing seat,
   compounding with transcript growth — measurable at 3–4× on like-for-like bounded tasks
   (14m32 vs 3m25), and the user's 836k intervention.
4. **Ritual waste** (F3/F4): seven no-op compact resumes, stamp-only resumes, the npm
   timeout — each small, all pure loss.
5. **Round inflation at the margin**: closures introducing new findings (F10/F11 "introduced
   by round 3's own closures") pushed analysis to cap+1; fresh-hunting each round is
   doctrinally required, so this is the depth bet's price, mitigated only by better first
   drafts.

## Recommendations (ruled 2026-08-01 — outcomes in Landing)

**R1 — execute the queued Layer-2 context-lifecycle rewrite and the transport repair as ONE
shape revision** (v5→v6, one re-audit ceremony, per the combine-precedented-waves practice):
the already-ruled TC-D5/TC-D6 + SSL D1–D3 content (per-seat lifecycle [PARAM]; counted
loop-unit cadence ~≥3 at gate pauses for governed standing multi-unit seats; respawn-as-reset
briefed from artifacts; versioned-name successors + name-refusal check; end-of-need shutdown)
**plus** this record's transport findings, which touch the same Layer-2 section and would
otherwise force a second bump. Then per-command parameter lines (plan's completeness seat
governed; the implement re-add) and the `validation-command-shape` teeth, per the BACKLOG item.

**R2 — repair the probe discriminator** *(new ruling needed)*: "addressable via SendMessage"
no longer proves teammate. The check must key on evidence the harness actually gives (the
resolve output's own classification, or a team-roster query), phrased against the current docs
— to be verified against official Claude Code documentation at build time, not invented.
*(Superseded at build: the doc check found a **positive** discriminator — the team config's
`members` array at `~/.claude/teams/session-<first-8-of-session-id>/config.json`; the resolve
strings this paragraph names shipped only as observed-not-documented corroboration.)*

**R3 — rule the degraded-transport branch** *(new ruling needed)*: when only subagent
transport exists, does a team-form command **refuse** (the letter of the no-fallback bet,
marked `Contested`) or **proceed degraded-with-record** (mesh off; lead-as-switchboard
acknowledged; micro-sends batched; recycle cadence tightened)? Evidence for the second: this
run completed on the forbidden transport and produced a strong package — the no-fallback bet's
own revisit trigger names distribution, but this is dogfood evidence that the degraded mode is
survivable *when recorded*. Evidence for the first: the degrade went unnoticed until the token
pane forced it, which is exactly what silent fallbacks do.

**R4 — micro-send hygiene line** *(new ruling needed, small)*: no ritual sends to standing
seats — no compact requests (TC-D1: not a real lever), no stamp-only resumes; fold one-line
confirmations into the next real dispatch. One Layer-2 sentence; token-justified by ~30 min
of measured pure loss in this run.

**R5 — vocabulary ban: record, don't build** *(recommendation)*: the breach is real but a
conversation cannot be mechanically graded; adding machinery would be invented enforcement.
Keep the ban, log this run as the breach evidence, revisit only if it recurs post-R1 (the
rewrite's dispatch-brief already re-states the register).

## Landing (2026-08-01)

**Rulings (user, 2026-08-01):** R1 adopted — one combined v5→v6 revision · R3 adopted as
**keep refuse + fix probe** (no degraded branch; the Contested no-fallback bet's letter holds,
its revisit trigger unchanged) · R4 adopted — the no-ritual-sends line · R5 adopted — vocab
ban recorded, no machinery. R2 executed as part of R1.

**Built (plugin v0.38.0, shape v6; command-architect authored, independent
`validation-command-shape` audit):** Layer 2 re-framed into team transport + per-seat context
lifecycle (TC-D1–D6 · SSL D1–D3 as amended encoded; SSL D4 and TC-D5's cost rider deliberately
not, logged); the first-spawn probe rebuilt on the **documented positive roster check** (team
config `members` array — found by an independent doc check after the audit's Critical caught
round 1 re-encoding proof-by-absence), transcript strings demoted to corroboration; **P17**
lifecycle-override slot (override-only — silence is conformant) + grader lockstep with the
`+60` ceiling term; lifecycle lines in brainstorm (fact-checker cadence-exempt), implement
(cycle unit · per-slice verifier override · retry relay), plan (unit counted **cumulatively
across stages** — the exact counter-reset failure this run measured). Audit: FAIL (1 Critical,
3 Important, 4 Minor) → fix round → PASS + two footer/figure sweeps. Run cost stated, not
offset: shared always-read floor 32,836 → 39,610 B (**+6,774 B/run**).

**Banked from the doc sweep:** TC-D5's P26 watch-item settled — v2.1.199's documented
name-refusal ("SendMessage… refuses the send rather than delivering it to the wrong agent")
confirms versioned-name successors as the right shipped default. TC-D4's governed-set ranking
was field-confirmed by this run within one pass (≤15 predicted, 14+ observed).

**Landed:** DECISIONS.md row (2026-08-01) · BACKLOG "Standing-seat build items" → trail ·
ROADMAP Next token-epic touch · index status updates (this entry; the compaction entry's
build state).

## Open questions

- Where does the **v0.38.0** label come from? No surface this sandbox can read carries it.
  *(Landing note: this wave itself now occupies plugin v0.38.0 — 0.37.0 was consumed by the
  concurrent `@`-reference wave mid-session. Coincidence with the user's run label; the label's
  origin stays unexplained.)*
- Whether `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` was set in the `mochiko-app` session — the
  transcript never shows the env check, only the (falsely-passing) probe.
- TC-D4's governed-set ranking predicted this seat's pass count within one — worth citing in
  the rewrite's rationale as its first field confirmation.
