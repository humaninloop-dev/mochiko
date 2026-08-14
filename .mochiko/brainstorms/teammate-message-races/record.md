# Teammate-Transport Message Races — Decision Record

**Status:** accepted (2026-08-14 — acceptance explicitly covered the recommended-adoption streak, D7's `Assumed` mark, and the review-batch amendments to the user's own D5/D6 rulings)
**When:** 2026-08-14
**Topic:** message races in multi-teammate command runs — fix doctrine; verify the regression claim against official Claude Code docs.
**Driver:** the kinako FEAT-006 `/mochiko:specify` run (Claude Code v2.1.229) hit repeated message races during its grader fix waves; the lead converged only by inventing concurrency protocols live. The user reads this as a regression and wants official-docs research plus a doctrine fix.

---

## Ground facts

Verified this session unless marked otherwise.

- **F1 — five race classes observed in the kinako run** (transcript, 2026-08-14):
  1. Superseding orders did not land last — one FR flipped between two ruled forms 4+ times ("my bridging commit and dual pin arrived at RA in reverse order").
  2. Lead disk-state assertions went stale — "three consecutive orders of mine asserted a disk state that was one edit behind."
  3. Two-writer clobber — one seat overwrote another's edit on a shared file ("the two-writers failure our own spec exists to prevent"); three seats wrote one workspace concurrently.
  4. Grader raced the writers — "the grader launched against a snapshot that has since moved."
  5. Stale idle pings drove re-sends — "teammate finished" fired with orders still queued; re-sent orders multiplied in-flight duplicates that crossed later.
- **F2 — doctrine carries zero concurrency discipline.** Grep over `plugins/mochiko/` and `.mochiko/memory/`: no hits for single-writer rules, message-ordering assumptions, supersession/ack protocol, or freeze-before-grade. `specify.md` Harness: "teammates or subagents per seat is your call" — no discipline attached to the teammate branch.
- **F3 (corrected in-session; original claim wrong) — the v7 harness DID carry structural concurrency protection, deleted whole at v0.48.0.** Original F3 claimed the dead machinery covered only team formation/addressability; the user challenged it ("regression in mochiko for sure, look at strip notes") and `.mochiko/strips/specify.md` proves them right. Verbatim from the superseded v7 file (pre-edit v0.47.0 tree):
  - **Mesh hold:** "A peer-routed gap list is a **hand-off, not a start signal** — the producer revises only when you open the next round, and your brief carries that hold." (message arrival ≠ act signal — kills race #1 by construction)
  - **Single writer by roster:** spec.md authored by "one **named standing seat** across rounds"; the critic "never authors" (kills #3)
  - **Serialized counted rounds:** "cap **3** rounds, you count them" — work moves at round boundaries (kills #2, #4)
  - **No ritual sends:** "no ritual sends — fold confirmations into the next real dispatch" (kills #5's amplifier)
  All superseded in one move at v0.48.0 — strip entry: "the entire v7-form file superseded — … seat lifecycle/recycling · … ordering invariants, ground-rules block" (D5 transport-neutral + D6 v8 shape; D1 `Contested` — no relocation, no new skills/shared-mechanics files). `patterns-sound-loop` (v0.70.0) declares "Transport stays neutral" and rebuilt only the RITUAL floor (produce/review/user-gate), not a transport floor.
- **F4 — prior platform datapoint on record.** 2026-08-01 (`plan-run-transport-forensics` trail, BACKLOG.md brainstorm-dogfood item): "a cross-exam message delayed, not lost — sender receipts prove neither delivery nor timeliness."
- **F5 — races clustered in grader fix waves.** The main authoring phase (seats on approved plans, rulings at user gates) ran clean; crossings began when the lead fan-out-relayed mid-flight rulings to three seats in parallel. Fan-out of lead-relayed rulings is the race surface.
- **F6 — the kinako lead's live-invented protocols converged**, and are candidate doctrine:
  content-pinned rulings (orders quote exact text, never disk state or message sequence) · token-gated supersession (new order names and voids prior tokens; stale arrivals discarded on receipt) · single writer per surface per wave (other seats send deltas to the writer) · drain-and-lock before grading (queue drained acting on nothing, disk locked, grader reads locked state) · file-over-message collision rule · idle-ping-with-queued-orders is normal, never re-send.
- **F7 — official-docs research (seat-reported 2026-08-14; sources cited; load-bearing quotes verify-at-review per ER-D3):**
  - **Ordering:** no guarantee. Explicit quote "Ordering is not guaranteed…sort by created_at if order matters" — scope nuance flagged by the lead: that quote sits in the *cross-session-messaging events* doc (code.claude.com/docs/en/cross-session-messaging), not teammate inbox semantics; teammate inbox ordering has no documented guarantee either way [NOT FOUND + adjacent evidence].
  - **Delivery (corrected at review, C3):** "Delivery isn't guaranteed in every configuration" is a **cross-session-transport** quote — that page self-scopes to "messages between your independent sessions." The **agent-teams (teammate) page says the opposite for delivery**: messages "delivered automatically to recipients," and ≥v2.1.224 send success means the mailbox write succeeded. Teammate ordering is **undocumented (silent)**, not documented-unguaranteed. Reviewer verified both pages at source.
  - **True platform regression existed and is already fixed:** v2.1.224 changelog — "Fixed `SendMessage` reporting 'Message sent' when the write to a teammate's inbox had actually failed" — masked-failure sends. Kinako ran v2.1.229 (fix in). Five send-path fixes v2.1.199→v2.1.225 show the delivery layer was unstable through the window.
  - **Idle notifications:** fire independently of queued inbox messages; idle-with-non-empty-inbox is expected behavior (v2.1.199 idle-row change; v2.1.225 duplicate-idle fix).
  - **Concurrent writes:** no file locking; official best practice IS ownership split — "Two teammates editing the same file leads to overwrites. Break the work so each teammate owns a different set of files" (code.claude.com/docs/en/agent-teams#best-practices). **Directly corroborates D4/D5 legs 1–2.** Task claiming has file locking, `.claude/tasks/` only.
  - **Open issues (unresolved at v2.1.229):** #78338 queued messages destroyed while sender holds success · #34668 teammates stop receiving after extended polling · #25254 VS Code delivery deadlock · #58762 team_name routing mismatch · #55586 duplicate worker multiplication.
  - **Seat's bottom line (scoped at review, C3):** cross-session transport — neither delivery nor ordering guaranteed; teammate transport — delivery documented-automatic (success = mailbox write ≥v2.1.224), ordering undocumented; feature experimental with documented limitations.
- **F8 — regression mechanism: ruled supersession with an unpriced side effect.** The v7 protections (F3) were framed as rigor/independence machinery, never as concurrency discipline — so the realignment ruling priced the choreography cost, not the race cost; D5's own rationale cited only team-formation failures (v0.9.1, v0.38.0). Precedent for the fix shape: `charter-ritual-balance` (2026-08-13) — v8 freedom deleted the ritual floor, kinako dogfood exposed it, the fix was a kind-keyed floor skill (`patterns-sound-loop`) + charter pointer lines, not restored choreography.

## Decisions

### D1 — Frame: harden doctrine regardless; research settles the platform half `Confident`
"Regression" is not pre-assigned. Mochiko doctrine gains concurrency discipline either way — races are possible under any async transport, and F2 shows nothing covers them. The official-docs research (F7) decides whether Claude Code v2.1.x also changed messaging behavior; if yes, that lands as a filed-upstream note or version pin, never as the fix itself.
*Rationale:* fixing only the platform half leaves doctrine bare against the same class on any version; fixing only doctrine leaves a possible real platform defect unreported.
*Amended in-session:* the mochiko half is no longer hypothesis — user-asserted and strip-verified (F3-corrected, F8): regression by ruled supersession at v0.48.0.

### D2 — Fix bites at both layers: topology AND message semantics `Confident`
Topology rules (who may write, when the grader reads) and message rules (what an order means, how supersession works) each kill the race classes the other cannot. Evidence split: #3/#4 are topology (writer clobber, grader racing writers); #1/#2/#5 are message semantics and hit even with ONE recipient seat — the kinako FR flip-flops were lead→single-seat crossings.
*Rationale:* topology-only leaves the flip-flop class alive; protocol-only leaves clobber to luck (the kinako clobber was caught by a seat's own whitespace sweep, not by any rule).

### D3 — Carrier: a new kind-keyed floor skill, sound-loop's sibling `Confident`
A new `patterns-*` floor skill carries the transport floor — firing per the split trigger ruled at C1: message legs on any multi-seat messaging, topology legs on shared writes — with pointer lines in the orchestrating commands, per the `charter-ritual-balance` precedent (F8). It amends `patterns-sound-loop`'s "Transport stays neutral" line: the transport *choice* stays neutral (D5 intact), transport *use* gains a floor. Fold-into-sound-loop rejected (two keying schemes in one skill: per-write vs per-run-composition); command-lines-only rejected (three restated copies that drift).
*Supersession owed at build:* scope annotation on the realignment D5 row + a strip-referenced amendment to sound-loop's neutrality line.

### D4 — The floor names safe shapes: composition steer is binding `Confident`
User-ruled at a no-recommendation fork, deliberately narrowing their own D5 (transport-neutral, 2026-08-02): when a lead is about to compose **concurrent writers on one write surface**, doctrine names the sanctioned shapes — **worktree-isolated writers** (isolation makes the surface non-shared) or **one pen-holder seat** (other seats route deltas to it). Any other composition is out of floor. Transport choice stays neutral everywhere writes do not collide.
*Rationale:* the kinako run would have been re-shaped at composition time — three teammates writing one workspace was the root topology error; use-discipline alone polices a shape that need not exist.
*Supersession:* deepens the D3-owed annotation on the realignment D5 row (choice neutral *except where writes collide*).

### D5 — The seven floor legs, non-waivable when triggered `Confident` *(as amended at review: C1/CV1/CV2)*
User-confirmed as drafted below; trigger and leg count amended at review disposition (2026-08-14, user-ruled).
**Split trigger (C1 amendment):** the original single trigger (">1 seat + shared write surface") un-covered the headline race — message races hit single-recipient, no-shared-write compositions, and leg 1 compliance (worktree isolation) deletes the shared surface, switching the message legs off. Now:
- **Message legs (3/4/6/7)** fire on **any multi-seat run with cross-seat or lead-relayed messaging** — shared writes or not. Review-pair/CROSS-EXAM messaging is covered (CV2).
- **Topology legs (1/2/5)** keep the shared-write condition.
When a trigger fires, its legs bind (sound-loop form — the trigger scopes the obligation; untriggered runs keep chartered freedom). Waivable form rejected (the kinako lead would have legally departed mid-crisis); trimming legs 5–6 rejected (races #4/#5 would keep only informal cover). **Leg 7 added (CV1 amendment):** no liveness protection existed — official docs name lead-stops-early as a known limitation, and this session live-missed two deliverables behind idle pings, caught only by manual pull. Skill name and exact wording are build-time craft.

### Ruled floor legs (D5, as amended)
Split trigger (C1): **message legs 3/4/6/7** fire on any multi-seat run with cross-seat or lead-relayed messaging, shared writes or not; **topology legs 1/2/5** fire when a shared write surface exists. When a trigger fires, its legs are **non-waivable** (sound-loop form: the trigger scopes the obligation; no trigger → chartered freedom stands).
1. **Composition steer** (D4): concurrent writes on one surface → worktree-isolated writers, or a single pen-holder with delta routing.
2. **Single writer per surface per wave** — every artifact has exactly one seat holding the pen at any moment; pen hand-offs are explicit.
3. **Mesh hold** (teammate transport): a received message is a hand-off, never a start signal — work starts on the lead's explicit open. (Restores the v7 line the strips preserved, F3.)
4. **Content-pinned supersession**: an order quotes the exact text it lands; a superseding order names what it voids. No order asserts disk state or relies on message sequence.
5. **Quiesce before cold grade**: a grader reads a declared-frozen state; writers hold until the verdict.
6. **No ritual sends; never re-send**: idle-ping-with-queued-inbox is normal; a resend exists only as a supersession that names what it voids.
7. **Fan-in confirmation** *(added at review, CV1)*: the lead counts expected deliverables and confirms each **arrived** before treating a fan-out as complete or converging; an idle signal without its deliverable → pull (request the output), never assume. Detects silent drops (#78338 class), died seats, and the narrated-not-dispatched defect class.
*Deliberately not promoted:* kinako's interim "file-over-message" collision rule — it conflicts with leg 4 (disk can carry a straggler's write; the content pin, not the disk, is authoritative). Kinako's terminal protocol agreed.

### D6 — Platform half: version floor only, no upstream filing `Confident` *(premise corrected at review: C2/C3; conclusion survives on corrected grounds)*
**Corrected premise, per-transport:** teammate transport — delivery documented-automatic, send success = mailbox write ≥v2.1.224 (a genuine teammate delivery LOSS would contradict the docs and be reportable); ordering **undocumented**, designed around, never claimed guaranteed-absent. Cross-session transport — delivery and ordering both explicitly unguaranteed (that page's quotes no longer ground the teammate claim). **#78338 reconciled (C2):** OPEN, same masked-failure class as the .224 fix but on the *background-agent send path*, not the teammate inbox path; may generalize; leg 7 fan-in confirmation is its detector either way — so "masked-failure fully closed at .224" is softened to "closed on the teammate path; open upstream on the background path."
**Disposition (unchanged):** kinako's observed races land on ordering (undocumented) and clobber (documented no-locking) — no NEW bug to file (#78338 already filed covers the loss class). The floor skill encodes **≥v2.1.224 as the agent-teams version floor** and cites the **agent-teams** page's ownership-split line as its doc anchor (correct transport's page, per C3).
*Rationale:* undocumented behavior is designed-around, not reported; the masked-failure hazard is version-keyed and real below .224; a *proven* teammate delivery loss is the one future observation that flips this into an upstream report.

### D7 — Verification: standing first-live-run watch only; no dedicated re-run probe `Assumed` *(limitation named at review, CV4)*
Default applied, flagged for acceptance: the build lands with the standard first-live-run watch (next multi-seat run exercises the floor); no dedicated kinako re-run probe. Revisit trigger: the watch's first live run shows a race despite the floor.
**Limitation (named):** one clean run is weak evidence for a nondeterministic class — absence-of-race is not proven by it; the watch verifies the floor *fired and was followed*, not that races are extinct. Accepted with that scope.

## Build surface (draft — confirmed at acceptance)

- **New floor skill** (`patterns-*`, name at build): split trigger + the seven D5 legs + When-NOT + the D4 composition-steer shapes + the D6 version floor (≥v2.1.224, agent-teams doc anchor); cross-referenced as sound-loop's transport sibling.
- **Pointer lines** in the multi-seat-capable command surfaces (specify · plan · implement · feature · brainstorm · setup — per-command judgment at build, sound-loop precedent) **plus the review clusters** (CV2: `review-brainstorm`/CROSS-EXAM's paired exchange is message-leg territory).
- **`patterns-sound-loop` amendment:** "Transport stays neutral" gains the floor pointer — supersession-by-ruling strip citing this record.
- **Bookkeeping:** DECISIONS.md row · supersession annotation on the realignment D5 row (choice neutral *except where writes collide*) · router row · BACKLOG build item + ROADMAP touch · first-live-run watch (next multi-seat run, CV4 scope named).
- **BACKLOG reconciliation (CV3):** "Working-tree ownership gap" (2026-08-01) closed via D4/legs-1-2 overlap, trail note citing this record · "Teammate hand-off narrated as text, never dispatched" (2026-08-04) related — leg 7 named its detector · "Experimental-API churn watch" (v2.1.220 baseline) carries the version-floor re-verify; no parallel watch.
- **Ceremony:** full primitive-edit ritual — strips, author≠grader audits, char-budget pre-asserts, plugin.json bump, CHANGELOG, marketplace sync.

## Open questions

- ~~What does "regression" name?~~ → D1: both halves; strips settled the mochiko one (F3/F8), research the platform one (F7/D6).
- ~~Platform disposition~~ → D6: version floor ≥v2.1.224 in the skill; no upstream filing.
- ~~Probe shape~~ → D7 `Assumed`: standing first-live-run watch only.

## Cold review (2026-08-14, solo, blind-map dispatch)

Reviewer spawned topic-only (23-angle blind map returned before the record path went out). Verdict: **critical-gaps**. Tally: 11 formed, 7 survived, 4 dropped at materiality (retrievable). Source verification: F3's four v7 strip quotes CLEAN against `.mochiko/strips/specify.md` · v2.1.224 changelog line and ownership-split quote VERIFIED EXACT at source · the delivery quote caught as cross-session-scoped (→C3) · #78338 VERIFIED OPEN (→C2). Hunt class 6 clean: every leg maps to an observed race class — no unpaid leg.

| # | Sev | Finding | Disposition (all user-ruled 2026-08-14, one batch, as recommended) |
|---|---|---|---|
| C1 | Critical | Single trigger un-covers message legs (worktree compliance deletes the shared surface; single-recipient races legal-but-unprotected) | **Split trigger** — message legs 3/4/6/7 on any multi-seat messaging; topology legs 1/2/5 keep shared-write. D5 amended |
| CV1 | Critical | No liveness/fan-in leg; silent drops and died seats undetected; twice self-demonstrated this session | **Leg 7 added** — fan-in confirmation before convergence; idle-without-deliverable → pull |
| C2 | Important | D6 dismissed platform half while #78338 (same masked-failure class) verified OPEN | D6 reconciled — .224 closes the teammate path; background path open upstream; leg 7 the detector |
| C3 | Important | F7/D6 conflated cross-session quotes with teammate transport | F7 corrected + D6 premise restated per-transport; doc anchor re-pointed to the agent-teams page; conclusion survives |
| CV2 | Important | Review-pair/CROSS-EXAM surface outside trigger and build surface | Covered by the C1 split; build surface gains the review clusters |
| CV3 | Important | Three related BACKLOG items unreconciled | Landing reconciles: working-tree item closed via D4 overlap · narrated-not-dispatched related (leg 7 detector) · churn watch carries the version-floor re-verify |
| CV4 | Important | D7's single watch is weak evidence for a nondeterministic class | Limitation named on D7's face; watch scope = floor-fired-and-followed, not races-extinct |

Dropped at materiality (reviewer's kill pass): lead-turn-boundary · human-gate-in-race-window · resume-from-transcript subtlety · proportionality/parallelism cost (single-writer-per-surface preserves cross-surface parallelism).

## Session trail

- Opened 2026-08-14 from the lead's race diagnosis of the kinako FEAT-006 specify run (same-session context). Research seat dispatched before questioning.
- Q1 frame (D1, recommended-adopted) → Q2 fix layers (D2, recommended-adopted) → user interjection "regression in mochiko for sure, look at strip notes" → strip verification corrected F3, added F8, amended D1 → Q3 carrier (D3, recommended-adopted; adoption streak flagged) → Q4 D5-edge posed recommendation-free (D4, user-ruled binding steer) → Q5 floor legs confirm (D5) → research landed as F7 (in-session irony: the seat's own idle ping arrived without its deliverable — race #5 live; pulled via direct request) → Q6 platform disposition (D6, recommended-adopted) → D7 default flagged `Assumed`.
- Cold review: blind-map dispatch (two-message protocol, topic-only spawn; 23-angle map). Verdict critical-gaps; 7 survivors → 7/7 dispositioned in one user-ruled batch "as recommended" (C1 trigger split · CV1 leg 7 · C2/C3 D6-premise repair · CV3/CV4 fold). Folds executed by lead.
- Verify round 1: NOT CLEAN — 2 stale-pre-fold-text defects (split trigger un-propagated to the authoritative legs block + D3 echo, blocking; F7 bottom line still conflated, non-blocking). Lead-repaired same round, propagation-only. Bounded round 2 on the three edit sites: CLEAN.
- Live race instances DURING this session (the class under repair, self-demonstrating): (1) research seat's idle ping without deliverable → pulled; (2) reviewer's verdict behind a bare idle ping → pulled; (3) reviewer's re-send of the already-dispositioned report crossing the verify order → deduplicated by content, duplicate voided by name; (4) reviewer's round-2 message void-named its own stale NOT-CLEAN before re-asserting — leg 4 practiced both directions.
- **Accepted 2026-08-14.** Landing executed in the same moment: DECISIONS.md row · BACKLOG "Teammate-transport floor build" section (cold-buildable) + the three CV3 reconciliations (working-tree ownership gap closed → trail · narrated-not-dispatched related, leg 7 its detector · churn watch carries the D6 version-floor re-verify) · ROADMAP Next touched — sound-loop row and the new transport-floor row merged into one floor-builds row to hold the cap at 7/7 (merge precedent: the 2026-08-12 production-only-narrowing merge) · index entry updated to accepted.

## Build (2026-08-14, v0.71.0 — same day as acceptance)

The wave ran under the floor it ships. Composition: two producer seats with **disjoint file
ownership** (skills cluster: new skill + sound-loop amendment + router + strip file; commands
cluster: six command pointer bullets + CROSS-EXAM.md) — no shared write surface existed, so
the topology legs were satisfied by construction. Message legs practiced: plan-first with
lead approval before any write (mesh hold) · lead-pinned skill name relayed to both seats
content-pinned (`patterns-transport-floor`) · one stale idle ping correctly held without
re-send (leg 6) · fan-in confirmation on both plans, both completion reports, and both audit
verdicts (leg 7) · auditors dispatched only after both producers confirmed done and held pens
(leg 5 quiesce).

- **Producer catches worth the record:** the commands seat chose CROSS-EXAM.md (references/,
  budget-exempt) over SKILL.md despite 2,630 chars of disclosed headroom — the pointer covers
  both review skills at the shared pair-protocol home; the skills seat self-reverted an
  unapproved body addition mid-write (leg-4 discipline — approved text only), and the lead's
  one plan amendment (the description's MUST clause under-covered the topology lane) landed
  at 450 chars.
- **Audits:** two fresh author≠grader validator seats on the quiesced tree. Skills cluster:
  budget pre-asserts clean (450/5,398 new · 500/5,849 sound-loop post-amend · router 206) ·
  ruling fidelity including the C3 per-transport correction verified against the record ·
  strip entry complete with the replaced sentence verbatim, both citations resolving.
  Commands cluster: +31/−0 pure additions proven per-file · GI-017 pointer discipline held ·
  charter-form exception applied to feature/plan/implement · CROSS-EXAM note correctly
  message-legs-only. **2/2 PASS round 1, zero fix rounds.**
- **Ripple (lead, mechanical):** cost-budgets ledger notes (transport-floor at birth;
  sound-loop re-measured) · DECISIONS 2026-08-14 row → ruled + built · realignment-D5 row
  annotated (choice neutral *except where writes collide*, with the unpriced-race-cost
  provenance) · plugin.json + marketplace 0.71.0 · CHANGELOG entry · BACKLOG build item →
  trail, replaced by the first-live-run watch (D7 scope on its face) · ROADMAP floor-builds
  row annotated DELIVERED, stamp to 32 skills / v0.71.0 · index Built note.
