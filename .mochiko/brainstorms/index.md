# Brainstorm Sessions — Index

One entry per session directory under `.mochiko/brainstorms/<topic-slug>/`, **newest first**.

**Maintenance contract (enforced by `CLAUDE.md`):** opening a session adds an entry at the top (status: open); acceptance, supersession, or abandonment updates it — status, review state, and where the outcome landed. A session directory without an index entry, or an entry whose status contradicts its record, is a defect to fix on sight.

**Artifact naming:** `record.md` is the canonical decision record (as-you-go audit trail + end-stage review). `synthesis.md` is the session deliverable for pre-v2 sessions; from v2.2 on it is an on-request digest stamped *derived — record canonical*.

---

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
