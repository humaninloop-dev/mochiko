# Brainstorm Sessions — Index

One entry per session directory under `.mochiko/brainstorms/<topic-slug>/`, **newest first**.

**Maintenance contract (enforced by `CLAUDE.md`):** opening a session adds an entry at the top (status: open); acceptance, supersession, or abandonment updates it — status, review state, and where the outcome landed. A session directory without an index entry, or an entry whose status contradicts its record, is a defect to fix on sight.

**Artifact naming:** `record.md` is the canonical decision record (as-you-go audit trail + end-stage review). `synthesis.md` is the session deliverable for pre-v2 sessions; from v2.2 on it is an on-request digest stamped *derived — record canonical*.

---

## `model-tiered-seats` — Haiku scribe + explorer seats: model tiering as token-epic angle 2
- **When:** 2026-07-24 · **Status:** accepted · record pair-reviewed (lens-split per sizing gate: 13 raised → 12 survived cross-exam → 10 merged survivors → 10/10 dispositioned incl. a 5-item user-accepted amendment batch; verify pass CLEAN after one fold-contradiction fix; clearing verdict ready)
- **Artifacts:** [record.md](model-tiered-seats/record.md) (canonical)
- **About:** second re-examination angle feeding the workflow-token-reduction epic's deferred BACKLOG scoping, seeded by a free-form conversation proposing Haiku scribe/explorer seats (recorded as unverified propositions P-scribe/P-explorer/P-ladder/P-caution). Fact-grounded headlines: every mochiko persona is `model: opus` with no model axis anywhere in doctrine; Explore silently stopped being guaranteed-cheap at Claude Code v2.1.198 (Opus-capped inherit) — un-cheapening the gap-resolver at 10+ sites; the scribe's designed target no longer exists post-wave-1/2; a plugin cannot shadow native Explore (scoped names, lowest precedence). Ruled (D1–D6 + 9 review folds): cost-weighted usage on the two documented channels with context-health as standing test and segments named; scribe closed as a non-avenue (re-open condition recorded); class-keyed cheap/session gap routing at `loop-discipline` dispatched to a disposable-per-gap scoped `model: haiku` explorer (honest build: the rule + ~10 site edits); no rostered seat changes tier — seat-tiering deferred to a dedicated brainstorm gated on reliability evidence (steelmans preserved); three probe items ride the epic's OTel/dogfood watch.
- **Landed:** ROADMAP Key Decisions (model-tiered-seats row, 2026-07-24), third row of the token-epic cluster; **BACKLOG stays deferred per the epic's ruling** — build scoping re-opens from the three records together.

## `standing-seat-lifecycle` — standing-seat context accumulation and strategic producer recycling
- **When:** 2026-07-23 · **Status:** accepted · record pair-reviewed (lens-split per sizing gate: 16 raised → 13 carried → 11 merged survivors → 11/11 dispositioned incl. 2 in-review user rulings; verify pass CLEAN, 4 optional polish notes applied; clearing verdict ready)
- **Artifacts:** [record.md](standing-seat-lifecycle/record.md) (canonical)
- **About:** second angle feeding the workflow-token-reduction epic's deferred BACKLOG re-examination, triggered by a live sighting (a team-form implement run's staff-engineer seat at "300K tokens" at a cycle kickoff). Fact-grounded headlines: doctrine is silent on seat lifetime and books standing-seat context only as a benefit; the accumulation concern concentrates on implement's two seats (~15–45+ turns vs ≤6 library-wide); the platform documents seat shutdown/replacement and advises it; cache expiry × human gates inverts "standing saves the re-read" for long loops. Ruled (D1–D4 + 11 review folds): conditioned checkpoint recycling for implement's producer (cycle floor ~≥3 + gate-pause check, relay-at-dispatch retry briefs, same-name successors) and per-slice verifier recycling; respawn briefs from existing artifacts only with an honestly-open sufficiency watch-item; the transport-vs-context-lifecycle doctrine reframe (command-shape Layer 2 rewrite v2→v3, anti-pattern line retargeted at transport, end-of-need shutdown norm gated on re-summons improbability); per-seat measurement riding the epic's OTel probe. A3 (pointers-not-payloads — already built per F-e(b)) and A5 (occupancy thresholds — no lead-visible surface) dissolved on facts.
- **Landed:** ROADMAP Key Decisions (standing-seat-lifecycle row, 2026-07-23), adjacent to the epic's row as its first completed re-examination angle; **BACKLOG stays deferred per the epic's ruling** — build scoping re-opens from both records together.

## `workflow-token-reduction` — avenues for reducing token usage across mochiko workflow runs
- **When:** 2026-07-23 · **Status:** accepted · record pair-reviewed (lens-split per sizing gate: 14 raised → 11 merged survivors → 11/11 dispositioned incl. 4 in-review user rulings, verify pass CLEAN, clearing verdict ready)
- **Artifacts:** [record.md](workflow-token-reduction/record.md) (canonical)
- **About:** where mochiko workflow runs spend tokens and which reductions are worth their trade-offs, triggered by the kinako dogfood (all workflows heavy, zero telemetry). Fact-grounded headline: the reporting layer outweighs the design layer (31 reports ~102k tok est., never read by the user, sole consumer the lead's verdict). Ruled (D1–D6 + 11 review folds): pure-waste-first frame; slim per-cycle reports to their verified consumers (failed cycles keep narrative); reference-by-ID down the artifact chain; sizing gates generalized with verification depth floored never-zero; four small waste fixes; a per-run recorded cost entry (manual baseline, OTel probe-then-graduate — config-only telemetry confirmed, F-d). Cleared non-avenues recorded (conditional reference loads, method-mandated reviewer reads, doctrine tax).
- **Landed:** ROADMAP Key Decisions (workflow-token-reduction row, 2026-07-23) as an **epic-weight item**; **BACKLOG deliberately deferred by user ruling** — the issues get re-examined from more angles before build items lock; build scoping re-opens from the record. **Wave 1 built 2026-07-23 at plugin v0.22.0** (report layer machine-first: D3-extended-to-all-report-formats + D2 manual baseline + D6a/b/c/d; four wave rulings + edit surface in ROADMAP's Decision Trail wave-1 entry). **Wave 2 built 2026-07-24 at plugin v0.23.0** (deliverable/artifact layer — D4 closed: `templates/artifact-format.md` envelope + dense artifact forms + conditional quickstart; rulings R1–R4 + trades T1–T4 and the edit surface in ROADMAP's Decision Trail wave-2 entry; remaining items in BACKLOG's Token-reduction waves 1–2 section).

## `domain-dependency-allowlist` — pragmatic domain-layer dependency allowlists in `/mochiko:setup`
- **When:** 2026-07-21 · **Status:** accepted · record reviewed (user-sized solo cold subagent + lead pressure-test of every finding against cited files: 9 raised → 7 confirmed / 1 weakened / 1 reframed / 0 refuted → 9/9 dispositioned incl. 3 user rulings)
- **Artifacts:** [record.md](domain-dependency-allowlist/record.md) (canonical)
- **About:** the Flutter dogfood run authored a domain layer restricted to the Dart SDK only. BE-HEX and the `layer-rules` module already carry an "approved domain dependencies registry" concept, but nothing in setup ever elicits or populates it. Ruled (D1–D5 + review folds): seed-eagerly-grow-lazily allowlist arbitration keyed to `layer-rules` attachment (card kept **or minted**); domain-relevance filters candidates before a codified trust-signal hierarchy ranks them (Flutter Favorites / golang.org/x / blessed.rs …, live-verified); tier-keyed add-gate with a cycle-report visibility floor; registry list in a preserve-in-place block of the paths-scoped domain rules file with policy in the ledger. Multi-stack registries and a mobile/app shelf deferred to BACKLOG.
- **Landed:** ROADMAP Key Decisions (domain-allowlist row, 2026-07-21); BACKLOG (`approved-domain-deps.md` item closed; Domain-allowlist follow-ups added — mobile/app shelf, multi-stack registries, kinako amend dogfood). **Built 2026-07-21 at plugin v0.18.0** — `references/DOMAIN-DEPENDENCIES.md` (single source) + the layered-architecture beat (agenda + setup.md) + BE-HEX/`layer-rules` registry-block machinery + ownership carve-outs (SKILL.md + surfaces template) + synthesis seeds section + implement-side visibility floor (`domain_deps_added` + confidence-gate hook).

## `pattern-codification-and-minimalism` — codify the team-form build pattern; strip the library to minimum
- **When:** 2026-07-18 · **Status:** accepted · record cold-reviewed (pair, lens-split: 17 raised → 13 merged survivors → 13/13 dispositioned incl. 3 user rulings, verify pass PASS) · **codification built 2026-07-19 at plugin v0.11.0**
- **Artifacts:** [record.md](pattern-codification-and-minimalism/record.md) (canonical)
- **About:** two coupled goals: (1) codifying the way `/mochiko:brainstorm` and `/mochiko:setup` were built — per the latest sessions — into primitives (agents + skills, possibly a team; count unconstrained) applicable to all other commands; (2) a library-wide minimalism pass: justify every line, strip commands/agents/skills to bare minimum with a strip log, then dogfood and iteratively re-add under documentation with version mapping. Ruled (D1–D9 + 13 review folds): codify the artifact *shape*, not the build lifecycle; a layered pattern-surface template as the shape's sole authoritative home; a two-seat keeper (new `command-architect` authors/converts, existing `validator` audits with a revived deterministic conformance floor); tiered strip criterion; per-primitive non-loaded strip notes with version stamps; evidence-gated re-adds with marked overrides; execution in cluster waves with an all-consumer guard on shared primitives.
- **Landed:** ROADMAP Key Decisions (pattern-codification + minimalism row, 2026-07-18); BACKLOG (Pattern-codification build items — the template, `command-architect` + authoring skill, the `validation-*` audit skill, the `strips/` convention, the `agent-dispatch.md` demotion edit, the `specify.md` frontmatter fix, the wave plan). **Built 2026-07-19 at plugin v0.11.0** — `templates/command-shape.md` (the shape's sole home, layered) · `command-architect` + `authoring-commands` · `validation-command-shape` on `validator` · `plugins/mochiko/strips/` (README + first two notes) · the `agent-dispatch.md` one-shot-line demotion · the `specify.md` frontmatter fix · the brainstorm/setup pre-shrink (strip-logged, independently audited PASS); the cluster strip waves remain open in BACKLOG.

## `setup-adversarial-review` — an adversarial gap review in `/mochiko:setup`
- **When:** 2026-07-18 · **Status:** accepted · record cold-reviewed (pair, lens-split — dogfooding the design on itself: 18 raised → 13 merged survivors → 13/13 dispositioned incl. 2 user rulings, verify pass PASS) · **built same day at plugin v0.10.0**
- **Artifacts:** [record.md](setup-adversarial-review/record.md) (canonical)
- **About:** whether and where `/mochiko:setup` gets an adversarial gap-finding review analogous to `/mochiko:brainstorm`'s end-stage cold review — setup's existing validator seat is a Tier-2 checklist grader, while the interrogation→synthesis path (G3) had no adversarial pressure beyond the user confirming their own synthesis. Ruled (D1–D7): a sized cold review of the confidence-marked synthesis before G3 (tier-keyed sizing gate, new `review-governance-intent` skill on `devils-advocate`, survivor routing, verify pass, Review section in `governance-intent.md`, event-scaled amend sizing, G3-edit delta-pass). Plus a user-initiated library-wide split of review-shaped skills into `validation-*` vs `review-*` by *who owns the clearing* — re-ruled on corrected facts after the review's Critical finding (S1).
- **Landed:** ROADMAP Key Decisions (setup-G3-review + validation/review split row, 2026-07-18); BACKLOG (Setup-adversarial-review build items — the skill, template deltas, `setup.md` propagation, seven-skill rename set). **Built 2026-07-18 at plugin v0.10.0** — `review-governance-intent` (coverage/coherence lens split, verify on coherence; Contested-rationale audit; cross-exam single-sourced at `review-brainstorm/references/CROSS-EXAM.md`) + template deltas + `setup.md` propagation + the seven-skill rename set; dogfood rides the setup-v3 dogfood item.

## `setup-v3-team-defect` — setup dogfood ran subagents again: diagnosing the team-mandate failure
- **When:** 2026-07-18 · **Status:** accepted · record un-reviewed (bare session — direct skill invocation) · **built same day at plugin v0.9.1**
- **Artifacts:** [record.md](setup-v3-team-defect/record.md) (canonical)
- **About:** the post-v3 dogfood run of `/mochiko:setup` (kinako) executed its producer as a one-shot subagent despite the hard team requirement — fork **B, set-and-ignored**: the lead probed `AGENT_TEAMS=1` itself, followed every surrounding instruction, then dispatched via the Agent tool. Refutes BACKLOG's seat-idiom mitigation hypothesis; docs-grounded cause: since v2.1.178 the teammate/subagent fork is one `name:` parameter on the same Agent tool, and the docs admit Claude "may sometimes use subagents instead of creating a team." Ruled (D1): seat-transport mechanics single-sourced in `agent-dispatch.md` + discriminating line and post-spawn addressability check in both team-form commands; D9's no-fallback mandate stands.
- **Landed:** ROADMAP Key Decisions (team-transport row, 2026-07-18); BACKLOG D7-investigation item closed, v3-dogfood item inherits D1's acceptance test, substrate item datapoint added; build: `agent-dispatch.md` v3 + `setup.md`/`brainstorm.md` hard-requirement addenda. Verification pending: the kinako re-run (resume at *loop (produce)*) showing team-form observed.

## `constitution-native-surfaces` — replacing constitution.md with native context surfaces
- **When:** 2026-07-18 · **Status:** accepted · record cold-reviewed (pair per sizing gate: 16 raised → 8 merged survivors → 8/8 dispositioned, verify pass PASS incl. 2 propagation fixes)
- **Artifacts:** [record.md](constitution-native-surfaces/record.md) (canonical)
- **About:** `constitution.md` dissolves entirely into native Claude Code surfaces: `CLAUDE.md` becomes the thin ratified core (stamp, index, universal principles as short lines, stack, gates summary) inside a marked setup-owned governance region; scope-bound principles go to `paths`-scoped `.claude/rules/`; procedure to skills; governance ledger + Three-Part metadata to `.mochiko/memory/` (D1–D8). Grounded by a fact-checker map (docs + repo), an inheritance check, and an empirical rules-delivery probe (negative → universal principles retreated into CLAUDE.md). Second thread (dogfood setup ran subagents, not teams) dispositioned as an out-of-scope defect (D7).
- **Landed:** ROADMAP Key Decisions (constitution-dissolution row, 2026-07-18); BACKLOG (D7 substrate-defect item); build started same day — setup v3 rewrite adopting the brainstorm command's team system (post-acceptance user ruling, ROADMAP Decision Trail).

## `setup-operating-docs-scaffolding` — setup scaffolds the operating-docs layer
- **When:** 2026-07-17 · **Status:** accepted · record cold-reviewed (single reviewer per sizing gate: 12 findings → 9 survivors → 9/9 dispositioned, verify pass PASS) · **built 2026-07-17 at plugin v0.8.0**
- **Artifacts:** [record.md](setup-operating-docs-scaffolding/record.md) (canonical) · [synthesis.md](setup-operating-docs-scaffolding/synthesis.md) (post-acceptance digest, fidelity-checked PASS)
- **About:** the operating-docs layer (brainstorm session org + `index.md`, `BACKLOG.md`, `ROADMAP.md`) becomes an elective **default-on knowledge-management constitution module** — adopted or declined whole; elicited by a new interrogation dimension #7 (after existing-practices), scaffolded at G5 finalize on a never-overwrite floor with per-project rulings on true collisions; enforced via three carriers (command index-bookkeeping, CLAUDE.md-sync rows, testable invariants run at session open/close); offered once to existing constitutions on amend (D1–D7 + 5 build items).
- **Landed:** record accepted 2026-07-17; all five build items built same day (the user landed the changes directly from the record post-session, as ruled): `constitution-modules/knowledge-management.md` (new) · `INTERROGATION-AGENDA.md` dimension #7 (agenda now ten) · `constitution-template.md` sync rows · `setup.md` (G5 scaffolding + brainstorm next-step + amend module-offer) · `brainstorm.md` (index bookkeeping + open/close invariants) · ripple edits (governance-intent template module-ruling declines, authoring/validation module rows, router). Ruling: ROADMAP Key Decisions (KM-module row, 2026-07-17); open thread + dogfood watches: BACKLOG (module-elicitation scaling item + KM-module follow-ups).

## `brainstorm-v2-2-revision` — sized lens-split review (brainstorm v2.2)
- **When:** opened 2026-07-05, concluded 2026-07-16 · **Status:** accepted; built at plugin v0.6.0 · record un-reviewed (bare session)
- **Artifacts:** [record.md](brainstorm-v2-2-revision/record.md)
- **About:** token-efficiency revision of the v2.1 end-stage review after its first completed run measured ≈654k out, dominated by the review pair triple-reading a reality surface the fact-checker had already mapped. Ruled: review sizing becomes a named human gate (pair / single / none with waiver), the pair splits by lens (decision-quality vs record-integrity), the fact-checker's map lands verbatim in the record, cross-exam compresses to one four-message exchange, verify runs on the integrity reviewer only, and `synthesis.md` is codified as a derived on-request artifact.
- **Landed:** `/mochiko:brainstorm` v0.6.0 + `validation-brainstorm`; ROADMAP Key Decisions (v2.2 row); BACKLOG v2.1-dogfood item closed.

## `setup-constitution-flexibility` — setup rewrite: constitution flexibility & deep elicitation
- **When:** 2026-07-05 · **Status:** accepted (review: 15 findings → 11 survivors → 11/11 folds verified) · **built 2026-07-16 at plugin v0.7.0** (stage 1 — frontend catalog shelf deferred to stage 2, tracked in BACKLOG's Setup-v2 follow-ups)
- **Artifacts:** [record.md](setup-constitution-flexibility/record.md) (canonical) · [synthesis.md](setup-constitution-flexibility/synthesis.md) (post-acceptance digest)
- **About:** fundamental rewrite of `/mochiko:setup` — the constitution should follow the user's declared intent, not a fixed backend-flavored baseline: a rigorous interrogation session precedes and governs authoring, sensitive to project type and scope.
- **Landed:** `/mochiko:setup` v2 (team-form, per two build-time rulings recorded in ROADMAP's Decision Trail: brainstorm-v0.6.0 command shape + team-form substrate) + the constitution cluster rewrite (catalog, modules, synthesis contract, trace-checking validator); ROADMAP Key Decisions (setup v2 row); REGISTRY setup rows. Doubles as the first completed end-to-end dogfood run of brainstorm v2.1 (cost + behavior numbers feed the v2.2 revision above).

## `fact-checker-seat` — the fact-checker seat: role, value, and the rename
- **When:** 2026-07-05 · **Status:** ruled and built at plugin v0.5.1 · record un-reviewed (bare session)
- **Artifacts:** [record.md](fact-checker-seat/record.md)
- **About:** opened as "why did the grounder spawn at session start"; converged to a keep-or-remove ruling on the seat via transcript forensics on both grounder runs. Kept: the seat is a neutral fact-checker (maps what IS at spawn, checks claims on demand, arbitrates fact disputes at review); renamed grounder → fact-checker; the command's spawn-timing language fixed.
- **Landed:** `/mochiko:brainstorm` v0.5.1 (rename + spawn semantics).

## `brainstorm-v2-revision` — end-stage review pair (brainstorm v2.1)
- **When:** 2026-07-05 · **Status:** accepted; built at plugin v0.5.0 · record un-reviewed (bare session)
- **Artifacts:** [record.md](brainstorm-v2-revision/record.md)
- **About:** revision of v2's team engagement after transcript forensics on its first run measured 3:1 machine-to-user traffic and consent-free folds into user-ruled decisions. Retired the standing episodic advocate; all adversarial pressure moved to convergence — two cold reviewers form findings independently, then cross-examine and return only survivors with a tally.
- **Landed:** `/mochiko:brainstorm` v0.5.0; ROADMAP Key Decisions (v2.1 row); BACKLOG I-4 (cost question) closed.

## `brainstorm-command-rewrite` — brainstorm v2 as an agent-team framework
- **When:** 2026-07-04 · **Status:** accepted; built at plugin v0.4.0 · record cold-reviewed (1 devils-advocate pass: 1 Critical / 8 Important / 3 Minor, all dispositioned) · engagement model since revised by v2.1/v2.2
- **Artifacts:** [record.md](brainstorm-command-rewrite/record.md)
- **About:** comprehensive rewrite of `/mochiko:brainstorm` from a 7-phase/5-gate dispatch pipeline into a minimal framework on native Claude Code agent teams — lead questions inline and writes `record.md` as-you-go; advocate, conditional grounder, cold pressure-tester at convergence. The pilot for team-form mochiko workflows (D9: hard-require teams, `Contested`).
- **Landed:** `/mochiko:brainstorm` v0.4.0; ROADMAP Key Decisions (v2 row); the substrate BACKLOG item's team datapoints.

## `vertical-graduation` — slice workflow / vertical graduation substrate
- **When:** 2026-07-02 · **Status:** acted on — substrate designed and built; `/mochiko:graduate` wrapper deferred to dogfooding
- **Artifacts:** [synthesis.md](vertical-graduation/synthesis.md)
- **About:** the pipeline's unit was the whole feature, so P1 stories couldn't reach verified code until the entire spec was planned and tasked. Resolved: a post-spec decomposition command producing a `slices.md` overlay (spec stays whole) plus slice-scoped entry variants on plan/tasks/implement; foundation slice, extend-mode design artifacts, feature-done declared at decomposition.
- **Landed:** `/mochiko:slice` + `authoring-slices` / `validation-slices` skills; ROADMAP Key Decisions (slice row).

## `brainstorm-command` — `/mochiko:brainstorm` v1 design
- **When:** 2026-07-02 · **Status:** **superseded** by brainstorm v2 (2026-07-04) after the first dogfood run — adversarial substance kept, phase/gate ceremony killed
- **Artifacts:** [synthesis.md](brainstorm-command/synthesis.md)
- **About:** first design of a brainstorm command wrapping `analysis-iterative` in a sound loop — two artifact-grounded advocate gates and altitude-aware exit ramps. Historically useful for why v1's shape existed; do not build from it.
- **Landed:** superseded — rationale preserved here and in git history; supersession recorded in ROADMAP (v1 row struck through).

## `command-altitude` — command altitude / single-sourcing
- **When:** 2026-06-30 · **Status:** acted on — recipe fixed, `specify` (329→66) and `setup` (385→78) retrofitted, both independently verified PASS
- **Artifacts:** [synthesis.md](command-altitude/synthesis.md)
- **About:** why the first transformed commands came out verbose: they restated discipline that `loop-discipline` + `workflow-contract` already single-source. A command's irreducible job is to stitch a team to a goal under a contract — declare the team, state per-workflow parameters, reference shared doctrine, name the human gates, stop. Root cause fixed in the transform recipe; thinness enforced as `verify-output`'s altitude gate (grep floor + keystone ceiling).
- **Landed:** transform recipe + `verify-output` altitude gate; ROADMAP Key Decisions; every later command port's contract cites this rule.

## `playbook-design` — design of the transformation playbook
- **When:** 2026-06-27 · **Status:** acted on — the playbook and the transform cluster were built from this spec
- **Artifacts:** [synthesis.md](playbook-design/synthesis.md)
- **About:** a repeatable, inspectable way to turn a human-in-loop primitive into mochiko form (kernel-free, five conventions, sound-loop placement). Defining twist: the playbook is dogfooded — it is itself the first mochiko cluster, and must pass its own done-condition.
- **Landed:** `PLAYBOOK.md`, `/mochiko:transform-cluster`, the `transform-producer` agent, and the assess/reconcile/transform/verify skills.

## `agent-decoupling` — agent ↔ workflow decoupling
- **When:** 2026-06-27 · **Status:** acted on — doctrine validated on the `specify` port (2026-06-27: HELD)
- **Artifacts:** [synthesis.md](agent-decoupling/synthesis.md)
- **About:** migrated personas carried their workflow coupling forward (sibling names, supervisor assumptions, baked-in loop roles). Doctrine: decoupling is proven by the *absence* of coupling, not a declaration of it — a persona is a self-contained professional with no trace of any workflow; enforced by the keystone test and a grep-checkable deny-list.
- **Landed:** agent personas, skill-library conventions 4 & 5, `loop-discipline`'s keystone-test section, `verify-output`'s decoupling scan.

---

*Not indexed:* `setup-workflow-rewrite/` — the aborted first v2 run's stale artifacts, gitignored (see `.gitignore`); the topic was re-run fresh as `setup-constitution-flexibility`.
