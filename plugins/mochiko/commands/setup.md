---
description: Establish or update the project's governance on native Claude Code surfaces — an interrogation session elicits the user's governance intent (tier, type, risk, values) before anything is authored, closing on a synthesis that a tier-sized cold intent review stress-tests before the user ratifies it (pair / single / recorded waiver — the user rules at the gate); the ratified synthesis becomes a traceable contract on the producer; a principal-architect teammate authors the surface set (a marked CLAUDE.md governance region, paths-scoped rules files, skill pointers, and a governance ledger), an independent validator teammate grades trace closure from the files, and the user accepts with a trace summary in hand — brownfield-aware (greenfield | brownfield | amend). Requires agent teams (CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS); refuses without them.
disable-model-invocation: true
---

# Setup — Governance From Interrogated Intent, On Native Surfaces

**Goal:** establish or update the project's governance so it follows the user's declared intent —
never a fixed baseline — and lives where Claude Code natively loads it. There is **no
`constitution.md`**: the deliverable is a **surface set** — a marked, setup-owned **governance
region in `CLAUDE.md`** (ratified stamp · principle index · universal principles as short
imperative lines · tech stack · quality-gates summary · module pointers), **`paths`-scoped
`.claude/rules/` files** for scope-bound principles, **skill pointers** for procedure-shaped
standards, and a **governance ledger** at `.mochiko/memory/governance-ledger.md` (Three-Part
metadata keyed by GI-ID, waivers with revisit triggers, amendment process, exception registry —
read only by setup/amend and the validator). The ratified synthesis
(`.mochiko/memory/governance-intent.md`) remains the **traceable contract** on the producer:
selection belongs to the session, only formulation to the producer. Design record:
`.mochiko/brainstorms/constitution-native-surfaces/record.md` (D1–D8).
`$ARGUMENTS` = optional setup request; empty is fine — detection proposes the mode.

**You are the lead.** You run the interrogation inline via `mochiko:analysis-iterative` (one
question per turn) against the agenda in
`${CLAUDE_PLUGIN_ROOT}/skills/authoring-constitution/references/INTERROGATION-AGENDA.md`, write
the synthesis, own the loop (round counter, verdict, escalations), and hold every gate. The
machinery holds three kinds of seats — a producer, cold intent reviewer(s) at the pre-G3
synthesis review, and a cold validator — and beyond those seats the conversation is you and the
user. This is a `mochiko:loop-discipline` sound loop; the Contract
section below is its authoring-time fill — **no per-run contract is written**.

## Hard requirement — agent teams

Check `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` in the environment before anything else; unset →
stop and tell the user how to enable it (settings/env; Claude Code ≥ v2.1.178). The env check is
a proxy — the **first teammate spawn is the authoritative probe** (the producer in brownfield;
otherwise the intent reviewers at the synthesis review, or the producer when that review was
waived): if it fails, stop with the same instructions. Never proceed teamless — **no fallback transport** (the same
deliberate dogfood-pilot bet as `/mochiko:brainstorm`, marked `Contested`; revisit when mochiko
distributes beyond the author's machines). Running the loop on one-shot subagents is not a
fallback — it is the defect this section exists to forbid. **Transport is discriminable at the
call:** a seat is spawned by one Agent call carrying **`name:`** — in the docs' idiom ("create
an agent team", "spawn a teammate") — and every later round is a `SendMessage` to that name; **a
spawn without a `name:` is a one-shot subagent, the forbidden form.** After the first spawn,
**verify the seat is addressable** (you can message it by name — the agent panel alone doesn't
confirm) before proceeding; not addressable → kill it and respawn, explicitly requesting an
agent team. Recipe: `templates/agent-dispatch.md` (Seat transport).

## Session constraints

- The conversation is the production surface, and it belongs to you and the user. **Never narrate
  machinery** — no "phase", "round", or "gate" talk; teammate housekeeping (idle notifications,
  acks) is never narrated and never replied to.
- Tell the user at the start that they can watch or message any teammate directly. Announce each
  seat in one line when you fill it — an unexplained teammate spawn reads as a malfunction.
- Workspace: `mkdir -p .mochiko/memory`. Kill-switch: stop and escalate if
  `.mochiko/memory/SETUP_STOP` exists — check before each producer, reviewer, or validator send.
- **Mode is yours** (greenfield | brownfield | amend) — it selects which stages run and the
  producer's branch. It is carried in-session + by workspace evidence; there is no context file.
- **Hygiene (all modes):** a `.mochiko/memory/constitution.md` found on disk is a superseded
  artifact from the pre-dissolution form — **delete it on sight** (no migration, no offer; the
  dogfooding-stage ruling D6). Say so in one line.
- **Ownership boundary (D8):** the CLAUDE.md governance region — delimited by
  `<!-- mochiko:governance:begin -->` / `<!-- mochiko:governance:end -->` markers — is
  setup-owned and **idempotently regenerated**: re-runs and amends replace the region in place.
  Everything outside the markers is user content and is **never touched**. All other writes
  (rules files under `.claude/rules/mochiko/`, the ledger, the synthesis) follow the same
  regenerate-what-setup-owns rule; the knowledge-management bundle scaffolding keeps its hard
  never-overwrite floor.

## The team

Teammates do **not** load `skills:` frontmatter — every spawn prompt must name the skill and
role itself, plus what to Read (see `templates/agent-dispatch.md` for the briefing fields). A
teammate's plain text is not visible to you: its reports arrive as **messages**, and your
follow-ups go to the **same named seat** via message — that continuity is what the standing seat
buys. A fresh spawn per round is the subagent anti-pattern, not a team.

- **producer** — `mochiko:principal-architect`, one **named standing seat** spanning both of its
  jobs, so analysis context flows into authoring and fix-list context survives across rounds:
  spawn it once, then **message the same seat** for every later round or clarification.
  Brownfield: spawn at analysis (runs `mochiko:analysis-codebase`, mode setup-brownfield, on the
  detect-stack output → writes `.mochiko/memory/codebase-analysis.md`, messages you a summary +
  Essential-Floor table + clarifications). All modes: after synthesis ratification, brief it (by
  message; spawn now if greenfield/amend never needed analysis) to author via
  `mochiko:authoring-constitution` → the **surface set** (CLAUDE.md governance region ·
  `.claude/rules/mochiko/*.md` · skill pointers · `governance-ledger.md`) **plus the trace
  summary** (each GI element → its primary enforceable home + companion entries) — Read the
  synthesis (its contract; formulation only, flagged proposals for anything beyond) and, in
  brownfield, the analysis; in amend, the current surfaces (preserve untouched principles;
  bump the region's semver). It **messages you** a report + flagged proposals + clarifications;
  it never grades.
- **intent reviewer(s)** — `mochiko:devils-advocate`, spawned **cold at the synthesis review**
  (never in the room during interrogation), count per the user's sizing ruling. Brief each to run
  `mochiko:review-governance-intent` against the frozen synthesis + the interrogation agenda +
  (brownfield) `codebase-analysis.md`. In a pair, **split the hunt by lens in the spawn briefs**
  — one **coverage** (missed dimensions, prune audits, the card-acceptance and waiver/module
  sweeps), one **coherence** (tier↔risk↔ruling alignment, the mark/echo-rationale audit,
  reality-conflict resolutions, cross-element contradictions) — the skill stays one document; the
  brief sets the emphasis. **Withhold each reviewer's counterpart from its spawn prompt**:
  findings form cold, then you introduce them for the one-shot four-message cross-exam (the
  single-sourced protocol in `review-brainstorm`'s `references/CROSS-EXAM.md`). A **single**
  reviewer gets the whole surface, no cross-exam, and runs the verify pass itself; in a pair the
  verify pass is the **coherence** reviewer's. These seats produce **lead-adjudicated input
  before G3** — never the authoritative grade (that stays the validator's, on the authored
  surface set); they message you, never the user, and never contact the producer or validator.
- **validator** — `mochiko:validator`, spawned **cold at first validation**, never in the room
  before that and never in contact with the producer. Brief it to run
  `mochiko:validation-constitution` **from the files** (the surface set + synthesis + trace
  summary, never the producer's report): trace closure both ways over the manifest, region-marker
  integrity, index→file existence, ledger completeness (Three-Part per principle), tier + waiver
  + floor-accounting checks, module-parameterized checks, anti-pattern + placeholder scans,
  semver call — brownfield adds the tools/versions↔analysis cross-check. It **messages you**
  binary PASS/FAIL + a fix list; round > 1 is a **message to the same validator seat**. **The fix
  list flows through you** to the producer — that routing is the independence the loop rests on.

## The flow

**Detect & mode-select** *(gate G1)* — run
`bash ${CLAUDE_PLUGIN_ROOT}/skills/analysis-codebase/scripts/detect-stack.sh .` (an input, not the
quality gate), count source files, check for an existing governance region in `CLAUDE.md`
(present → suggest **amend**; >5 source files + framework detected → suggest **brownfield**; else
**greenfield**). Apply the constitution-hygiene rule if the old artifact is on disk. The user
chooses. Brownfield → analysis; greenfield/amend → interrogation.

**Brownfield analysis** *(gate G2)* — the producer runs the analysis, then present its summary:
**confirm** (→ interrogation) / **edit** (corrections, bounded re-run) / **reject** (fall back to
greenfield, or abort). The analysis is an intermediate input gated by this human checkpoint + the
deterministic detect-stack baseline — no machine validator (the surface set is the deliverable,
and gets one).

**Interrogation** *(inline — you, not a teammate)* — run the agenda's ten dimensions adaptively;
an early low-tier declaration licenses pruning (skips named, never silent). Brownfield: the
analysis pre-fills the existing-practices dimension; **confront detected-reality-vs-declared-intent
conflicts in the open** — never silently resolve. Amend: micro-session scoped to the delta (a
tier bump or un-waive is a governance event and gets its agenda slice); additionally offer,
**once**, every module the synthesis records no ruling on — record the answer in
`governance-intent.md` either way (a recorded decline is permanent until the user reopens it;
never re-ask). Then deal the catalog (`…/authoring-constitution/references/catalog/` — shelves by
declared type, cards filtered and parameterized by tier), arbitrate card by card
(recommend-then-arbitrate), collect minted intents, and take waiver rulings where the tier
permits.

**Synthesis review** *(sizing gate + cold review, all modes — before G3)* — assemble
`templates/governance-intent-template.md` → `.mochiko/memory/governance-intent.md` (GI-IDs **and
a confidence mark** on every element; **amend updates the persisted file delta-wise** — untouched
elements keep their IDs and marks). Open the **sizing gate**: state the synthesis's weight
(element count, mark mix, reality-surface load) — its purpose is informing the user's *elevation*
of the tier-keyed default recommendation: `poc`/`internal` → **single**, `production`/`regulated`
→ **pair**; on amend, **event-scaled** — a governance event (tier bump, un-waive, floor/waiver
change) gets the full tier-keyed default, lighter deltas recommend single at
`production`/`regulated` and none-with-waiver below (the gate always opens: every amend records a
sizing ruling or a waiver — audit-complete). The user rules **pair / single / none**; `none` →
record the waiver in the synthesis's Review section and go to G3. Otherwise spawn per the ruling
— the synthesis is **frozen** from spawn until dispositions land — and route each survivor by
answer-owner: user rulings (deck / tier / waiver) and challenged dimension-prunes / scope
decisions → **the user, directly**; your formulation → argue with the finding's owner, **two
exchanges max (you count)**, then tie-break; **reality-surface** fact disputes → checked against
the analysis or an Explore subagent, never argued; **user-declared facts** (team size, risk
posture, lifespan) → **the user, as confirmation**. An overruled survivor marks its element
`Contested`. Land every disposition in the Review section, then the **verify pass** (the
coherence-lens reviewer, or the sole reviewer) confirms the folds → G3. On a **tier-bump amend**,
re-deal tier-loosened deck rulings on the bump's agenda slice (or force-re-mark them) — a stale
`Confident` never carries a lower-tier loosening upward unexamined.

**Synthesis confirmation** *(gate G3, all modes)* — present the reviewed synthesis (tally +
dispositions, or the waiver): **confirm** (record the stamp → authoring loop) / **edit** (fold
corrections, re-present — a **material** post-review addition or change gets a bounded
delta-pass by the still-seated reviewer before confirmation) / **reject** (back to interrogation,
re-entering the full flow — fresh sizing and review included). This checkpoint is the
synthesis↔intent fidelity gate — nothing is authored before it clears, and everything ratified
was stress-tested (or its waiver recorded).

**Authoring loop** *(you own the counter; the deliverable is FAIL until proven otherwise)* —
**produce** (brief the producer seat; on round > 1 message it the validator's fix list verbatim
for targeted revision — don't regress passing items; if it messages clarifications it cannot
resolve, ask the user and feed answers forward — an in-loop human gate, never the done-condition)
→ **validate** (spawn the validator cold on round 1; message the same seat after) → PASS: record
verdict + version bump → acceptance. FAIL: increment round; cap **3** / fix list unchanged
round-over-round / kill-switch → **escalate** (last fix list + stop reason;
give-guidance-and-retry / accept-with-noted-gaps / abort — the run stays FAIL unless the user
explicitly accepts); else loop.

**Acceptance** *(gate G4 — reachable only on validator PASS)* — present the validated surface set
(region version + tier, principle count by home — CLAUDE.md lines / rules files / skill
pointers — floor accounting incl. waivers) **with the trace summary**: which principles trace to
which synthesis elements (deck-kept / minted / floor-preset), each principle's primary home +
companions, waiver records, and **every flagged proposal from the producer, each needing the
user's ruling** — proposals fold in only by the user's word (then re-validate). **accept** (done)
/ **amend** (changes become the fix list, re-enter the loop — must PASS again) / **reject**
(abort; drafts stay in place, marked unaccepted in the region stamp).

**Finalize** *(gate G5)* — report from the accepted artifacts: the CLAUDE.md governance region,
the rules files, `governance-ledger.md`, and `governance-intent.md` (the last two durable — the
synthesis is the amend baseline and the traceability surface; never offer to delete either),
brownfield `codebase-analysis.md` (offer retain / remove), the PASS + acceptance trail, a
suggested commit, next step (`/mochiko:specify`; when knowledge-management was adopted, also
`/mochiko:brainstorm`). **Knowledge-management adopted → scaffold the bundle here**
(`.mochiko/brainstorms/` + `index.md`, `BACKLOG.md`, `ROADMAP.md`), honoring the session's
collision rulings, on the hard never-overwrite floor. Downstream delivery is native: CLAUDE.md
(region included) loads for every session and spawned agent; dispatch briefs in downstream
commands carry the **obligated read** for `paths`-scoped rules their producers won't trigger by
reading.

## Contract (authoring-time fill — governed by `mochiko:loop-discipline`)

- **Done-condition:** default **FAIL**; clears only when the sized intent review ran (or its
  waiver is recorded) **with every survivor carrying a disposition** **and** G3 synthesis
  confirmation cleared (all modes) **and** the validator returns PASS graded from the files
  **and** G4 acceptance cleared (**and** G2 confirmed, in brownfield). Out of rounds = escalate,
  never done.
- **Producer ↔ validator:** `principal-architect` (authoring-constitution, analysis-codebase)
  authors, never grades; `validator` (validation-constitution) grades from files, never authors —
  disjoint agents, disjoint skills, structurally separated (validator cold-spawned, fix list
  lead-routed, no producer↔validator contact). Validator standing: Tier-2 with deterministic
  sub-checks (placeholder scan, two-way trace-closure over the manifest, region-marker + index→file
  existence, Three-Part presence in the ledger; brownfield tools/versions↔analysis). Intent
  reviewer(s): `devils-advocate` (review-governance-intent), cold-spawned at the synthesis
  review, no contact with producer or validator — they produce **lead-adjudicated input** before
  G3, never the authoritative grade (the `review-*`/`validation-*` family boundary).
- **Bounds:** ≤3 produce↔validate rounds (you count) · no-progress exit · kill-switch
  `.mochiko/memory/SETUP_STOP` · review caps: one cold read per reviewer · one four-message
  cross-exam · a two-exchange lead↔reviewer cap per survivor · one verify pass (+ one bounded
  G3-edit delta-pass when triggered). The interrogation is bounded by user-driven convergence — a
  human-attended session, not an agent loop.
- **Out of scope, explicitly:** drift detection between invocations (waiver revisit triggers fire
  on re-invocation only, by design) · backward compatibility with the retired `constitution.md`
  form (D6: none — delete on sight).
- **Human gates:** G1 mode-select · G2 analysis checkpoint (brownfield) · the interrogation
  itself + the in-loop clarification sub-gate · the review sizing gate + survivor rulings on
  user territory · G3 synthesis confirmation · G4 acceptance with trace summary +
  flagged-proposal rulings · G5 cleanup · escalation on any guard trip.

## Recovery

Teams do not survive `/resume`, and a shared account limit can throttle the team and the main
session together — pause posture: note the resume stage in one line at the top of
`governance-intent.md` (or in the region stamp once it exists). Resume from workspace evidence,
respawning what the stage needs — the producer seat re-reads the synthesis + fix list cheaply; a
validator respawn is cold by design:

| Evidence | Resume at |
|----------|-----------|
| `.mochiko/memory/` missing or empty and no governance region in `CLAUDE.md` | detect & mode-select |
| brownfield chosen, `codebase-analysis.md` missing | analysis |
| `codebase-analysis.md` present, unconfirmed | G2 |
| mode set, `governance-intent.md` missing | interrogation |
| `governance-intent.md` present, Review section empty (no sizing ruling) | synthesis review (sizing gate) |
| sizing ruled, survivors undispositioned (sized-not-run or mid-review) | synthesis review (respawn reviewers per the ruling) |
| dispositions folded, verify pass unrecorded | verify pass |
| Review verified or waived, no confirmation stamp | G3 |
| synthesis confirmed, surface set missing/stale (no region, missing named rules files, no ledger) | loop (produce) |
| surface set present, no recorded PASS | loop (validate) |
| PASS recorded, not accepted | G4 |
| accepted | G5 |
| `SETUP_STOP` present | escalate |

---

**What you own (not the agents):** the interrogation and the synthesis pen; the review sizing
gate, survivor routing, and the cross-set merge; the loop; the verdict against the default-FAIL
done-condition; the human gates; the mode; the governance-region ownership boundary; and never
letting producer, reviewer, and validator collapse into one seat. Stay
kernel-free; brief every spawn per `agent-dispatch`; do not modify git or push. Full rules:
`mochiko:loop-discipline`.
