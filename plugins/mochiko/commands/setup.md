---
description: Create or update the project constitution — an interrogation session elicits the user's governance intent (tier, type, risk, values) before anything is authored, closing on a ratified synthesis that becomes a traceable contract on the producer; a principal-architect teammate authors from it, an independent validator teammate grades from the files (trace-IDs, tier, waivers, modules), and the user accepts with a trace summary in hand — brownfield-aware (greenfield | brownfield | amend). Requires agent teams (CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS); refuses without them.
disable-model-invocation: true
---

# Setup — Constitution From Interrogated Intent

**Goal:** establish or update `.mochiko/memory/constitution.md` so it follows the user's declared
intent — never a fixed baseline. The session elicits *what to govern and how strictly* before any
authoring; the ratified synthesis (`.mochiko/memory/governance-intent.md`) is a **traceable
contract** on the producer: selection belongs to the session, only formulation to the producer.
`$ARGUMENTS` = optional setup request; empty is fine — detection proposes the mode.

**You are the lead.** You run the interrogation inline via `mochiko:analysis-iterative` (one
question per turn) against the agenda in
`${CLAUDE_PLUGIN_ROOT}/skills/authoring-constitution/references/INTERROGATION-AGENDA.md`, write
the synthesis, own the loop (round counter, verdict, escalations), and hold every gate. The
machinery holds two seats — a producer and a cold validator — and beyond those seats the
conversation is you and the user. This is a `mochiko:loop-discipline` sound loop; the Contract
section below is its authoring-time fill — **no per-run contract is written**.

## Hard requirement — agent teams

Check `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` in the environment before anything else; unset →
stop and tell the user how to enable it (settings/env; Claude Code ≥ v2.1.178). The env check is
a proxy — the **first teammate spawn is the authoritative probe** (the producer): if it fails,
stop with the same instructions. Never proceed teamless — no fallback transport (the same
deliberate dogfood-pilot bet as `/mochiko:brainstorm`, marked `Contested`; revisit when mochiko
distributes beyond the author's machines).

## Session constraints

- The conversation belongs to you and the user. **Never narrate machinery** — no "phase",
  "round", or "gate" talk; teammate housekeeping is never narrated and never replied to.
- Announce each seat in one line when you fill it — an unexplained teammate spawn reads as a
  malfunction. Tell the user they can watch or message any teammate directly.
- Workspace: `mkdir -p .mochiko/memory`. Kill-switch: stop and escalate if
  `.mochiko/memory/SETUP_STOP` exists — check before each dispatch.
- **Mode is yours** (greenfield | brownfield | amend) — it selects which stages run and the
  producer's branch. It is carried in-session + by workspace evidence; there is no context file.

## The team

Teammates do **not** load `skills:` frontmatter — every spawn prompt must name the skill and
role itself, plus what to Read (see `templates/agent-dispatch.md` for the briefing fields).

- **producer** — `mochiko:principal-architect`, one standing seat spanning both of its jobs, so
  analysis context flows into authoring and fix-list context survives across rounds. Brownfield:
  spawn at analysis (runs `mochiko:analysis-codebase`, mode setup-brownfield, on the detect-stack
  output → writes `.mochiko/memory/codebase-analysis.md`, returns summary + Essential-Floor table
  + clarifications). All modes: after synthesis ratification it authors via
  `mochiko:authoring-constitution` → `.mochiko/memory/constitution.md` — brief it to Read the
  synthesis (its contract; formulation only, flagged proposals for anything beyond) and, in
  brownfield, the analysis; in amend, the existing constitution (preserve untouched principles;
  semver bump). It **returns** a report + any flagged proposals + clarifications; it never grades.
- **validator** — `mochiko:validator`, spawned **cold at first validation**, never in the room
  before that and never messaged by the producer. Brief it to run
  `mochiko:validation-constitution` **from the files** (constitution + synthesis, never the
  producer's report): three-part + trace-stamp structure, the deterministic trace-ID cross-check
  both ways, tier + waiver-format + floor-accounting checks, module-parameterized section
  checklist, anti-pattern + placeholder scans, semver call — brownfield adds the
  tools/versions↔analysis cross-check. Returns binary PASS/FAIL + fix list; **the fix list flows
  through you** to the producer — that routing is the independence the loop rests on.

## The flow

**Detect & mode-select** *(gate G1)* — run
`bash ${CLAUDE_PLUGIN_ROOT}/skills/analysis-codebase/scripts/detect-stack.sh .` (an input, not the
quality gate), count source files, check for an existing constitution (present → suggest
**amend**; >5 source files + framework detected → suggest **brownfield**; else **greenfield**).
The user chooses. Brownfield → analysis; greenfield/amend → interrogation.

**Brownfield analysis** *(gate G2)* — spawn the producer for the analysis, then present its
summary: **confirm** (→ interrogation) / **edit** (corrections, bounded re-run) / **reject**
(fall back to greenfield, or abort). The analysis is an intermediate input gated by this human
checkpoint + the deterministic detect-stack baseline — no machine validator (the constitution is
the deliverable, and gets one).

**Interrogation** *(inline — you, not a teammate)* — run the agenda's nine dimensions adaptively;
an early low-tier declaration licenses pruning (skips named, never silent). Brownfield: the
analysis pre-fills the existing-practices dimension; **confront detected-reality-vs-declared-intent
conflicts in the open** ("you declared production tier; the codebase has no tests") — never
silently resolve. Amend: micro-session scoped to the delta (a tier bump or un-waive is a
governance event and gets its agenda slice). Then deal the catalog
(`…/authoring-constitution/references/catalog/` — shelves by declared type, cards filtered and
parameterized by tier), arbitrate card by card (recommend-then-arbitrate; the user keeps /
tightens / drops / re-ranks), collect minted intents, and take waiver rulings where the tier
permits.

**Synthesis confirmation** *(gate G3, all modes)* — assemble
`templates/governance-intent-template.md` → `.mochiko/memory/governance-intent.md` (GI-IDs on
every element; **amend updates the persisted file delta-wise** — untouched elements keep their
IDs; an amend where no synthesis exists yet backfills one from the existing constitution first,
each standing principle landing as a kept element). Present it: **confirm** (record the stamp →
constitution loop) / **edit** (fold corrections, re-present) / **reject** (back to
interrogation). This checkpoint is the synthesis↔intent fidelity gate — nothing is authored
before it clears.

**Constitution loop** *(you own the counter; the deliverable is FAIL until proven otherwise)* —
**produce** (dispatch the producer; on round > 1 pass the validator's fix list verbatim for
targeted revision — don't regress passing items; if it returns clarifications it cannot resolve,
ask the user and feed answers forward — an in-loop human gate, never the done-condition) →
**validate** (spawn/dispatch the validator) → PASS: record verdict + version bump → acceptance.
FAIL: increment round; cap **3** / fix list unchanged round-over-round / kill-switch → **escalate**
(last fix list + stop reason; give-guidance-and-retry / accept-with-noted-gaps / abort — the run
stays FAIL unless the user explicitly accepts); else loop.

**Acceptance** *(gate G4 — reachable only on validator PASS)* — present the validated
constitution (version, tier, principle count, floor accounting incl. waivers) **with a trace
summary**: which principles trace to which synthesis elements (deck-kept / minted /
floor-preset), waiver records, and **every flagged proposal from the producer, each needing the
user's ruling** — proposals fold in only by the user's word (then re-validate). **accept** (done)
/ **amend** (changes become the fix list, re-enter the loop — must PASS again) / **reject**
(abort; the draft stays in `.mochiko/memory/`).

**Finalize** *(gate G5)* — report from the accepted artifacts: `constitution.md` +
`governance-intent.md` (both durable — the synthesis is the amend baseline and the traceability
surface; never offer to delete either), brownfield `codebase-analysis.md` (offer retain /
remove), the PASS + acceptance trail, a suggested commit, next step (`/mochiko:specify`).
Cross-cutting stubs, referenced not dispatched: CLAUDE.md sync (`syncing-claude-md`, unported)
and evolution roadmap (`authoring-roadmap`, unported).

## Contract (authoring-time fill — governed by `mochiko:loop-discipline`)

- **Done-condition:** default **FAIL**; clears only when the validator returns PASS graded from
  the files **and** G3 synthesis confirmation cleared (all modes) **and** G4 acceptance cleared
  (**and** G2 confirmed, in brownfield). Out of rounds = escalate, never done.
- **Producer ↔ validator:** `principal-architect` (authoring-constitution, analysis-codebase)
  authors, never grades; `validator` (validation-constitution) grades from files, never authors —
  disjoint agents, disjoint skills, structurally separated (validator cold-spawned, fix list
  lead-routed, no producer↔validator messaging). Validator standing: Tier-2 with deterministic
  sub-checks (placeholder scan, trace-ID cross-check, three-part presence; brownfield
  tools/versions↔analysis).
- **Bounds:** ≤3 produce↔validate rounds (you count) · no-progress exit · kill-switch
  `.mochiko/memory/SETUP_STOP`. The interrogation is bounded by user-driven convergence — a
  human-attended session, not an agent loop.
- **Out of scope, explicitly:** drift detection between invocations. Setup is on-demand tooling —
  waiver revisit triggers fire on re-invocation only; a project that never re-runs setup is never
  re-checked, **by design**, not oversight.
- **Human gates:** G1 mode-select · G2 analysis checkpoint (brownfield) · the interrogation
  itself + the in-loop clarification sub-gate · G3 synthesis confirmation · G4 acceptance with
  trace summary + flagged-proposal rulings · G5 cleanup · escalation on any guard trip.

## Recovery

Teams do not survive `/resume`. Resume from workspace evidence, respawning what the stage needs
(the standing producer rereads the synthesis + fix list cheaply; a validator respawn is cold by
design):

| Evidence in `.mochiko/memory/` | Resume at |
|--------------------------------|-----------|
| missing or empty | detect & mode-select |
| brownfield chosen, `codebase-analysis.md` missing | analysis |
| `codebase-analysis.md` present, unconfirmed | G2 |
| mode set, `governance-intent.md` missing | interrogation |
| `governance-intent.md` present, no confirmation stamp | G3 |
| synthesis confirmed, `constitution.md` missing/stale | loop (produce) |
| `constitution.md` present, no recorded PASS | loop (validate) |
| PASS recorded, not accepted | G4 |
| accepted | G5 |
| `SETUP_STOP` present | escalate |

---

**What you own (not the agents):** the interrogation and the synthesis pen; the loop; the
verdict against the default-FAIL done-condition; the human gates; the mode; and never letting
producer and validator collapse into one seat. Stay kernel-free; brief every spawn per
`agent-dispatch`; do not modify git or push. Full rules: `mochiko:loop-discipline`.
