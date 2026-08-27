## Action Plan — `/mochiko:setup add a principle covering how long we keep customer data, and tighten the pre-release checks`

**Grounding already done (read-only):** schema `plugins/mochiko/schemas/setup.yaml` (fail-condition count confirmed = 6, matching the Not-done hard-code); workspace state — `CLAUDE.md`, `.mochiko/memory/governance-intent.md`, `.mochiko/memory/governance-ledger.md`, `.mochiko/memory/constitution.md`, `.claude/rules/mochiko/money-handling.md`, `.claude/rules/mochiko/output-style.md`, `FEATURES.md`. Finding: this is **not** a fresh project — a ratified governance surface set already exists (v1.2.0, greenfield-originated, production floor, depth `high`, modules: none), plus a stale, superseded `.mochiko/memory/constitution.md` (v0.9) still on disk.

---

### Phase 1 — Mode proposal and gate
**Does:** Proposes run mode from the workspace evidence rather than asking blind.
**Reads:** the five files already read above (sufficient to establish state).
**Writes:** none.
**Gate (user's ruling, non-negotiable):** *"An existing ratified governance set (v1.2.0) is present, plus a stale `constitution.md` I'd flag as superseded and remove on sight. I'm proposing mode = **amend**. Confirm, or rule greenfield/brownfield instead?"*
- **If amend (expected):** proceed to Phase 2 against the existing synthesis as the amend baseline.
- **If user overrides to greenfield/brownfield:** the run restarts under that mode's full protocol (full fact-profile re-interrogation or codebase reconstruction) instead of the lighter amend path below — out of scope for this plan unless the user actually rules it that way.

---

### Phase 2 — Inline interrogation on the two requested topics
**Does:** Runs the interrogation myself, inline, adaptively (via the `mochiko:analysis-iterative` procedure) on exactly the two topics the user named, then a card-by-card catalog-deck pass for anything the topics trigger, recommend-then-arbitrate.
**Reads:** `${plugin_root}/skills/authoring-constitution/references/INTERROGATION-AGENDA.md`, its `catalog/` deck, `DOMAIN-DEPENDENCIES.md`; re-reads `GI-001` (fact profile: EU/UK, customer contact details, invoice line items, bank account identifiers) and `GI-007` (existing quality gates) as anchors.
**Topic A — data retention principle (new):** how long each data class in GI-001 is kept, deletion trigger (account closure vs. rolling window), interaction with UK/EU statutory invoice-retention minimums vs. GDPR erasure rights, whether backups/archives are in scope, exceptions.
**Topic B — tighten pre-release checks (amends GI-007):** what's insufficient today (currently: tests green + ≥70% coverage only), candidate additions — migration dry-run, security/lint gate, staging smoke test, manual sign-off, changelog discipline; note the superseded `constitution.md` Article III ("tag Fridays only if someone's around") as a candidate topic to surface, not silently adopt or silently drop.
Locate/enumerate reads here (e.g. checking for an existing deletion job or CI config) route to a `haiku`-model `Explore` subagent; interpretive judgment (does this satisfy "tightened") stays on session tier.
**Writes:** none yet — findings accumulate in working memory toward the synthesis draft.
**Gate (every card/module ruling is the user's, ongoing through this phase):** each catalog card and any module trigger (a data-retention/compliance module could plausibly fire given EU/UK + customer data) is put to the user individually — adopt, waive, or defer. No card auto-resolves.

---

### Phase 3 — Draft the synthesis amendment
**Does:** Drafts the updated `governance-intent.md` content in place: a new `GI-00X — Customer data retention` principle, an amended `GI-007 — Quality gates` (or a new `GI-00Y` for the tightened pre-release checks, depending on how Phase 2 resolves it), and a new Amendment-log row.
**Reads:** current `governance-intent.md` (already read) as the amend baseline.
**Writes (deferred until after Phase 4/5, not yet committed):** the draft content for `.mochiko/memory/governance-intent.md`.
**Gate:** none yet — this is drafting, not ratification.

---

### Phase 4 — Pre-ratification stress test (cold seat)
**Does:** Spawns an independent seat (via the `mochiko:review-governance-intent` skill, e.g. a `mochiko:devils-advocate`-style reviewer) to stress-test the draft **before** ratification, using the mandated two-message blind dispatch:
1. Message 1 — setup topic + project identity/goal only ("add a data-retention principle; tighten pre-release checks", Peartree/invoicing/EU-UK/production-floor/depth-high) — **no synthesis path**. Seat returns a blind Phase 0 angle map.
2. Message 2 — sends the synthesis path (`.mochiko/memory/governance-intent.md`); cold read begins against the actual draft.
**Reads (by the seat):** the drafted `governance-intent.md`, the existing ledger, the CLAUDE.md region, the rules files.
**Writes:** none (grading seat, not an author).
**Gate:** *user's recorded waiver* can skip the cold seat entirely — if the user explicitly waives it, this phase is skipped and the waiver is recorded in the ledger; otherwise the cold seat runs as above.
**Follow-on gate (survivor routing, user's ruling):** any coverage finding that survives is presented as a candidate topic, never silently folded in or dropped:
- *Explore now* → re-enters Phase 2's interrogation on that angle, the re-elicited intent lands as a further GI-ID, loop back to Phase 3.
- *Rule inline* → user decides on the spot, synthesis updated directly.
- *Defer* → noted, excluded from this run's ratification.

---

### Phase 5 — Ratification gate
**Does:** Presents the final draft synthesis (post-stress-test edits, if any) to the user as plain blocking text — never a timed prompt.
**Gate (floor, user-only — this is the hard ratification line):**
- **Ratified as-is:** proceed to Phase 6; this is the moment after which the fail-condition "surface authored before intent was ratified" becomes checkable — nothing gets authored before this point.
- **Requested changes:** loop back to Phase 3, redraft, then either a full or bounded delta re-pass of Phase 4 depending on how material the edit is, then re-present.
- **Rejected/cancelled:** run halts here; no surfaces are touched; the existing v1.2.0 set stands untouched.

---

### Phase 6 — Author the governance surfaces (post-ratification only)
**Does:** A producing seat (via `mochiko:authoring-constitution`) plans the concrete file diffs first and gets my approval on that plan before writing anything (producer seats never self-approve).
**Would write, pending the plan-approval gate below:**
- `CLAUDE.md` — inside the `<!-- mochiko:governance:begin/end -->` region only: new retention-principle bullet, amended quality-gates/pre-release bullets, bumped `Ratified:` version stamp (MINOR bump per the ledger's own semver policy — new principle + tightened, non-breaking check — e.g. 1.2.0 → 1.3.0). The `mochiko:output-style` carve-out block is regenerated verbatim, untouched in content.
- `.claude/rules/mochiko/` — a new scoped file (e.g. `data-retention.md`, `paths:` scoped to customer-data/deletion code) if the retention principle is detailed enough to need its own home, and/or an expanded quality-gates rules file for the pre-release checklist if it grows beyond a CLAUDE.md bullet — placement decided by the authoring skill's own home-selection rule, not asserted here.
- `.mochiko/memory/governance-ledger.md` — new Three-Part metadata blocks (Enforcement/Testability/Rationale/Trace) for each new/amended GI-ID, and a new Amendment-log row matching the CLAUDE.md version bump.
- `.mochiko/memory/constitution.md` — **deleted**, flagged in one line as superseded (non-negotiable per the schema; not optional cleanup).
- **Not touched:** `FEATURES.md` / `.mochiko/features/` (already scaffolded and confirmed from the prior run — the never-overwrite floor applies and this amend doesn't re-trigger reconstruction), `.mochiko/product/architecture/` (already scaffolded; this run authors no ruled architecture content regardless of path).
**Gate (plan approval, producer-seat floor):** the exact file list and diff shape above is presented for approval before any write.
- **Approved:** proceed to write, then to Phase 7.
- **Wants changes:** replan and re-present.
- **Declines:** authoring halts; run cannot close (would trip the "authored surfaces don't trace" / "no acceptance" fail conditions if forced past this point).

---

### Phase 7 — Independent grading (author ≠ grader, default FAIL)
**Does:** A seat that authored none of Phase 6's output (via `mochiko:validation-constitution`, e.g. a `mochiko:tech-lead`/`mochiko:validator` seat) grades the **actual files on disk** — never the authoring seat's self-report — against the quality checklist: trace closure from every ratified GI-ID to its authored surface, version-stamp match between the CLAUDE.md region and the ledger, the new data-retention floor category carrying either a principle or a recorded waiver (not silently uncovered), carve-outs (`output-style`) byte-identical, superseded `constitution.md` actually gone. Default posture is FAIL until every item explicitly clears.
**Reads:** `CLAUDE.md`, the new/changed rules file(s), `governance-ledger.md`.
**Writes:** a findings/fix list only, no file edits.
**Gate:** none directly to the user here, but the result routes:
- **PASS:** proceed to Phase 8.
- **FAIL:** loop back to Phase 6 with the fix list (same ratified intent, no re-ratification needed unless a fix would materially change the intent itself, in which case it routes back to Phase 3/5 instead).

---

### Phase 8 — Trace summary and final acceptance
**Does:** Assembles the trace summary (ratified GI-IDs → their landed surface locations → the independent grade result) and presents final acceptance as plain blocking text, flagged proposal by flagged proposal (not one bulk yes/no).
**Gate (floor, user-only):**
- **Accepts all:** run closes; I'd suggest a commit message describing the change but never run the git mutation myself.
- **Accepts some, rejects others:** only the accepted deltas land; rejected ones loop back to Phase 3 (if the intent itself is contested) or Phase 6 (if just the authored wording is contested), then re-grade only the changed subset.
- **Rejects entirely:** halts with no net change; v1.2.0 stands.

---

### Phase 9 — Close-out audit against the 6 fail-conditions
**Does:** Checks each of the 6 `setup.fail.*` conditions explicitly before declaring done — no surface authored pre-ratification (Phase 5 gate enforced this), trace closes (Phase 7 confirmed), grading seat ≠ author (Phase 7 wiring), the new retention floor category is covered by principle or recorded waiver (not silently dropped — tracked since Phase 2/4), user acceptance was given (Phase 8), feature map present at close (already true from the prior setup run, unaffected by this amend). Any one still standing would fail the run regardless of how far the phases above got.
**Writes:** none beyond what Phases 6–8 already committed.
**Reports:** result in `templates/output-style.md` register, and the advisory next-step pointer (`/mochiko:specify`, `/mochiko:architecture` — peer doors; `/mochiko:brainstorm` omitted since Modules: none, no knowledge-management adopted).