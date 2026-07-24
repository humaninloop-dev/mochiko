# Backlog

Lightweight capture of open design decisions and workflow scoping work. Not a full issue tracker — just things that need a decision before building can proceed, or ideas that need to be held without cluttering REGISTRY.md or ROADMAP.md.

Items close when a decision is made and recorded in `ROADMAP.md`, or when the work lands in `REGISTRY.md`.

> **Note (2026-07-18):** the HIL→mochiko **transformer cluster was retired** (`transform-cluster` command, `transform-producer` agent, and the `assess-primitive` / `reconcile-cluster` / `transform-recipes` / `verify-output` skills — see `ROADMAP.md` Key Decisions). The `…-port follow-ups (from the /mochiko:transform-cluster … run)` sections below are **historical records** of how each workflow was ported; the run tool no longer exists, but the follow-up items themselves remain valid where still open.

---

## Open design decisions

These carry forward from `agent-skills-research/synthesis/my-framework.md` — unresolved questions that will surface during workflow builds.

- [ ] **Prose vs. gate allocation** — which behaviors earn graded anti-rationalization prose in skills vs. a hard `PreToolUse` hook interceptor? The synthesis has three poles (persuasion, command-hook, kernel) but the kernel is excluded; allocate between prose and hook for mochiko.

- [ ] **Claude-Code portability** — `rfc2119-invocation-trigger` and `invocation-axis-taxonomy`'s `disable-model-invocation` flag are CC-specific. Decide: adopt the principle and bind to CC, or abstract the mechanism? Likely surfaces when the router skill is built.

- [x] **Memory model** — RESOLVED 2026-06-27 (setup, confirmed by specify; ROADMAP Key Decisions). In-session + `.mochiko/memory/`, workspace-as-state, context-handoff absorbed into the lead.

- [ ] **Intensity modes** — global `lite/full/ultra/off` dial vs. per-rule feature? (`[[intensity-modes]]` still `maybe` in synthesis.) Defer until at least two workflows are built and the pattern is clear.

- [x] **`implement` orchestration** — RESOLVED 2026-07-01 (implement port; ROADMAP Key Decisions + Decision Trail). Sequential-first thin lead; native parallel + kernel/DAG deferred to dogfooding as a shortcut-ledger entry (tracked under Implement-port follow-ups).

- [ ] **Command orchestration substrate — agent teams vs. `Task`-subagents** — DEFERRED from the 2026-06-30 altitude work (`.mochiko/brainstorms/command-altitude/synthesis.md`). The thin commands are substrate-agnostic and currently use one-shot `Task`-subagent dispatch under a prose lead, though the ROADMAP thesis says "agent teams handle orchestration" — a thesis↔build gap. Inputs already gathered: agent teams give producer↔validator independence for free (isolated context) **but** are experimental/flag-gated (`CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS`), ephemeral, **ignore the `skills:` frontmatter for teammates** (hits convention axis 4), and their direct teammate↔teammate messaging can *dilute* the artifact-grounded independence an adversarial pair needs. A thin command now exists to evaluate against. Relates to the Claude-Code portability item. **Datapoint (implement, 2026-07-01):** implement — the DAG-heaviest workflow — ported cleanly on one-shot `Task`-subagent dispatch under a thin prose lead; **5th confirmation**. **Datapoint (brainstorm v2, 2026-07-04): the deliberate agent-team pilot now exists.** `/mochiko:brainstorm` v2 runs on a live team by design, and answers each gathered input concretely: flag-gated → hard-require, no fallback (`Contested`, revisit at distribution); `skills:` frontmatter ignored → every spawn prompt names skill + role; messaging-dilutes-independence → structural sequestration (the pressure-tester spawns cold at convergence and reads the record before any teammate contact; the in-room advocate is *additional pressure*, never the validator). **Datapoint (v2 first run + v2.1 revision, 2026-07-05):** the standing-teammate form used teammate↔teammate messaging **zero** times and measured **more** expensive than v1's four dispatches (I-4 closed — numbers in the Brainstorm-v2 follow-ups below); the v2.1 end-stage revision (two cold reviewers cross-examining each other) is the first shape that exercises the capability that distinguishes teams from `Task` dispatch. **Datapoint (setup v2, 2026-07-16): generalization begun by user ruling, ahead of the v2.2 dogfood this item was waiting on.** The rewritten `/mochiko:setup` is the second team-form command — deliberately, eyes open: its bet is **persistent seat context** (one producer spans brownfield analysis → authoring → ≤3 fix rounds without cold re-reads), NOT teammate↔teammate messaging (explicitly unused — the fix list routes through the lead, designing out the independence-dilution risk this item warned about). This item stays open until a team-form setup run is observed (see the Setup-v2 follow-ups below); specify/plan/tasks/implement remain `Task`-dispatch pending that evidence. **Datapoint (setup v3 dogfood, 2026-07-18): the team mandate alone doesn't produce teams.** Second defect run — env set and probed by the lead itself, v3's seat/messaging idiom in the command — and the producer still dispatched as a one-shot subagent. Docs since v2.1.178: no separate team-creation step; the teammate/subagent fork is one `name:` parameter on the same Agent tool, and *"Claude may sometimes use subagents instead of creating a team."* Fix: transport mechanics + addressability check (`agent-dispatch.md` v3 + both team-form commands' hard-requirement sections); the team-form setup observation is still pending. Record: `.mochiko/brainstorms/setup-v3-team-defect/record.md`.

- [ ] **Module-elicitation scaling pattern** — does the interrogation agenda grow one dimension per future constitution module (as the knowledge-management module did — dimension #7, agenda now ten), or should module elicitation consolidate into a single modules beat? Raised in the `setup-operating-docs-scaffolding` session as the fold-into-values steelman; deferred as not that session's to settle. **Revisit trigger: the next constitution module design.** Record: `.mochiko/brainstorms/setup-operating-docs-scaffolding/record.md` (Open / Deferred). **Datapoint (2026-07-21, domain-allowlist build):** the `layer-rules` elicitation landed as a *beat* attached to deck arbitration (kept-or-minted trigger), not a new dimension — the agenda stays ten; first evidence for the consolidate-into-beats pole.

---

## Domain-allowlist follow-ups (from the `domain-dependency-allowlist` session, 2026-07-21)

Machinery built same day at plugin v0.18.0 (see ROADMAP Key Decisions). Record:
`.mochiko/brainstorms/domain-dependency-allowlist/record.md`.

- [ ] **Mobile/app shelf** — a properly-flavored clean-architecture-for-apps card (Flutter / Compose / SwiftUI) selecting the `layer-rules` module, so app sessions deal flavored layered material instead of minting from nothing. **Acceptance test (from the dogfood run):** a Flutter setup session deals flavored layered-architecture material. Carries the review's F6 caution: until this lands, a *greenfield* app session filling `layer-rules` placeholders risks backend-flavored `src/domain/**` paths globs that never match `lib/…` code (brownfield is covered by codebase-analysis + the validator's paths checks).
- [ ] **Multi-stack / monorepo registries** — the built design covers the single detected stack by explicit ruling (review F5): a monorepo with two domain layers in two ecosystems needs per-stack registries, paths scopes, and seeding. **Revisit trigger: the first multi-stack setup run.**
- [ ] **Dogfood: kinako amend run** — run `/mochiko:setup` amend on the Flutter dogfood project: the layered-architecture beat should fire (minted layered principle → module ruling + seed arbitration), replacing "Dart SDK only" with a seeded registry block. Doubles as the acceptance check for the minted-path trigger (review F4).

---

## Token-reduction waves 1–2 (from the `workflow-token-reduction` epic)

Wave 1 (report layer, v0.22.0, 2026-07-23) and wave 2 (deliverable/artifact layer, D4 closed,
v0.23.0, 2026-07-24) both built & independently audited **PASS** (one audit-round fix apiece). Full
record: ROADMAP Decision Trail (wave-1 + wave-2 entries) + strip notes `[v0.22.0]`/`[v0.23.0]`-stamped
in `.mochiko/strips/` + REGISTRY rows (`report-format`, `artifact-format`). Deliberately untouched
(scope calls, not omissions): brainstorm's record/`synthesis.md` and the governance surfaces —
carried as wave-3 candidates below. Still open from the epic:

- [ ] **Waves 1–2 dogfood (next kinako run).** The acceptance check for both strips: seats author
  the slim report formats (frontmatter-only passing reports; failure narrative on fails) AND the
  dense artifact forms (statement-line blocks, sensitivity rows, compact mapping, 2-3 one-line
  scenarios; quickstart only when an integration surface exists, its null path in plan.md); the
  lead's confidence gate reads the frontmatter fields; reviewers converge without density-as-gap
  findings; the first `run-costs.md` rows land (D2 manual baseline — usage figure + run-shape
  counts) and the artifact-layer disk sizes are compared against the kinako baseline (~555k B
  design layer); no consumer starves (a starved consumer re-adds via the strip notes'
  evidence-gated path with a marked override, never silently). **Calibration watches riding this
  run:** (a) the envelope's size defaults (≤ 3-line rationale, one-line entries) and the ≤ 150-line
  quickstart cap were set by judgment, not evidence — a form that squeezes out real substance is a
  starved consumer and re-adds through the same evidence-gated path; (b) T3 has two branches and
  one feature may not exercise both — the null path (no integration surface → one-line record in
  plan.md, no stub file) needs a feature without an external integration surface before it counts
  as verified.
- [x] **D4 — reference-by-ID down the artifact chain** — DONE (wave 2, 2026-07-24, v0.23.0; `templates/artifact-format.md`; ROADMAP Decision Trail wave-2 entry).
- [ ] **D5 — review sizing gates generalized + the floored verification-depth gate**: command texts
  of the five pipeline stages.
- [ ] **D2 upgrade path — the one-shot OTel probe** (probe-then-graduate, epic S1/F-d): enable the
  documented config in a dogfood run; observe console behavior, per-run aggregation, teammate
  attribution; automation graduates on probe evidence only.
- [ ] **Wave-3 candidates — the layers R1 deliberately scoped out (2026-07-24; candidates, not
  commitments — each needs its own ruling before any strip).** (1) **The governance/memory layer:**
  kinako's `governance-intent.md` 26.2k B + `governance-ledger.md` 47.4k B + `trace-summary.md`
  12.9k B (~21.6k tok est. combined) — excluded because they carry their own fresh design
  (constitution-native-surfaces ruled the ledger metadata-primary, read only by setup/amend + the
  validator; the intent is the amendable ratified contract), so any densification must be designed
  inside the setup cluster, not imposed by an artifact wave. **Revisit trigger:** a kinako amend
  run showing the setup-side read cost mattering, or the intent/ledger growing superlinearly with
  amendments. (2) **Brainstorm `record.md` / `synthesis.md`:** excluded because record length IS
  the audit-trail function and the session doctrine (v2.2) already governs synthesis as a derived
  on-request digest — only re-examine if a completed session's transcript forensics show record
  *authoring* (not review reading, already optimized) as a material pool. Neither item enters
  scope silently; a wave-3 opens the way waves 1–2 did — rulings ratified in-session first.
- [ ] **Standing-seat build items stay deferred** (`standing-seat-lifecycle` record D1–D4 —
  conditioned checkpoint recycling, respawn briefs, the Layer-2 transport-vs-lifecycle rewrite,
  per-seat measurement). **Version-number handoff:** that record's D3 targeted "shape v2→v3"; this
  wave took v3 (run-cost + header), so the lifecycle rewrite lands as **v4+**. Its D2
  respawn-briefs-from-artifacts ruling is a named consumer of this wave's slimmed reports — the
  sufficiency watch-item rides the wave-1 dogfood above.

---

## Kinako-dogfood workflow changes (mvp-thin-loop oversight trace, 2026-07-24)

The first field evidence of the pipeline's blind spots, from the kinako mvp-thin-loop dogfood
(2026-07-24): five UI gaps traced root-cause through the full artifact chain — evidence layer at
`humaninloop-dev/kinako` → `.mochiko/specs/mvp-thin-loop/oversight-trace.md` (findings F1–F5,
first-origin + missed-gates + classification per finding). Unifying diagnosis: every gate
verified artifact-internal consistency; none exercised the launched artifact's user journey, and
app-internal user actions had no machine-traceable artifact. Each item below was
**pressure-tested in-session for single-project overfit** (n = instantiating findings; 2 of 9
raw proposals cut, 2 slimmed) and is **awaiting founder ratification** — ratified items land as
one edit wave (author ≠ grader: `validator` audits touched primitives), ruling promoted to
ROADMAP Key Decisions, entries closed. **Confirm-or-revert vehicle: the kinako UI fix wave**
(the same next-kinako-run the token-reduction waves 1–2 dogfood item above rides) — each change
either fires during that run (confirmed) or sits unexercised (flagged, revisit). **Meta-rule
riding the ratification:** single-project-retro changes default to checklist/hunt-line edits in
existing skills; new artifacts, stages, or hard gates require n≥2 across projects.

- [ ] **Surface-task rule (adopt; n=2).** Owner: `patterns-vertical-tdd` (rule) +
  `review-task-artifacts` (check). Every acceptance scenario in which the developer performs an
  action MUST map to a task that builds the user-reachable surface where the action happens;
  entity/port/use-case/CLI tasks do not discharge it — the reviewer asks, per scenario, "name
  the task where the user does this in the shipped surface." Evidence: kinako F2 (allowlist +
  injection-scope UI never tasked in any S1/S2 cycle — first origin s1/task-mapping C7
  "Delivers"; the S2 mirror deleted the word "UI") and F3 (the only setup-UI task scoped to
  readiness display). Confirm: the kinako allowlist-UI and setup-integrate fixes run through
  `/mochiko:tasks` and this check produces surface tasks.
- [ ] **Journey-gate class, graded form (adopt; n=5).** Owner: `testing-end-user` (doctrine) +
  `qa-engineer`; touches the implement entry variant and the audit charter. The scenario's actor
  and entry surface are part of "real infrastructure": a `**TEST:**` whose spec text has the
  developer acting in the app cannot be discharged by a CLI/fixture except as an explicit,
  logged substitution with open residue; ≥1 launched-artifact journey gate per slice; journey
  gates **block feature-close** (not slice-close — kinako's slice-close honesty wasn't the
  failure, the compounding silence was) and open deferrals re-surface at every subsequent slice
  start. Evidence: all five kinako findings passed through "no gate exercised the launched app"
  (T6.8 `[ ]` across S1→S4; "developer adds repo" = fixture store write; "watch the run in the
  app" = CLI script; the browse gate test called `refresh()` itself). Cross-refs: the `audit`
  scoping item (feature-close verification charter) and the slice-build "Feature-close
  verification has no owning workflow" item — this rule gives that pass its hard gate class.
  Confirm: the kinako fix wave closes through a launched-app journey gate.
- [ ] **Two spec-review hunt classes (adopt; n=2).** Owner: `review-specifications` (the
  `devils-advocate` mount). (a) **Display meaningfulness** — for every MUST-display requirement:
  what does the user literally see, and can they act on it? (kinako F5: "provenance" satisfied
  by raw UUIDs; TR-025 ratified ID-sufficiency; no artifact required "can tell which
  repo/session"). (b) **Lifecycle twins** — for every background state-change: when does each
  surface reflect it; for every elicited lifecycle edge case, generate its twin (kinako F4:
  elicitation reached app-*closed* catch-up — edge case 4/FR-013 — and stopped one step short of
  app-*open* staleness). Guard ratified with it: future hunt-class additions require n≥2 or
  high severity — hunt lists must not grow one class per retro. Confirm: the next
  `/mochiko:specify` review exercises both.
- [ ] **Split-gate assert-union invariant (adopt; n=1, zero-cost).** Owner: `loop-discipline`.
  When a gate is split (e.g. CI half / human half), the union of the halves' asserts MUST equal
  the original gate's — splitting may never shrink; the reviewer diffs asserts-before vs
  asserts-after. Evidence: kinako F3 — the tasks advocate's round-1 CI-testability gap produced
  the T6.7/T6.8 split that dropped the config-write assert from the app-journey half
  (task-mapping still carried "a pre-seeded foreign MCP/hook entry survives the write"), then
  round 2 blessed the result: **the review loop itself caused the shrink**. Adopted despite n=1
  because the mechanism is the scariest found and the check costs nothing.
- [ ] **Runbook walked-stamp (adopt; n=1 doc).** Owner: convention — landing surface decided at
  ratification (likely a `testing-end-user` reference or the implement/audit command side; no
  runbook-authoring skill exists today). Any doc claiming "as it actually runs today" carries,
  per step, either an executed-date stamp or `UNVERIFIED`; a doc with `UNVERIFIED` steps may not
  claim "today." Evidence: kinako's `dogfood-runbook.md` §0–2 — claims "the loop as it actually
  runs today" while instructing unbuilt UI; its overclaims cover four of the five detonations.
- [ ] **Residues route to a tracked surface (slimmed adopt; n=1–2).** Owner:
  `executing-tdd-cycle` (rule) + `authoring-slices` (Feature-Done condition). Any
  flagged/deferred/remaining line in a cycle report MUST also land in an existing tracked
  surface (the feature's backlog/obligations section) at cycle close; Feature-Done requires that
  surface empty. Deliberately **no new artifact** (overfit guard). Evidence: kinako F4 — the C10
  cycle report's "plus a background refresh trigger" flag dropped at transcription into the
  app-shell obligation (which cited other sources); F3's write-trigger wiring bundled invisibly
  under a sandbox-only deferral rationale. Jointly with the journey-gate item this also covers
  the cut "no out-of-pipeline integration steps" proposal: feature-close wiring executes against
  the tracked surface and closes through journey gates.
- [ ] **Plan designs its surfaces (slimmed adopt; n≈4 paths).** Owner: plan cluster
  (`technical-analyst` craft; `review-plan-artifacts` check). The plan MUST design every
  user-facing surface it commits to build; an api-contracts "app-internal, deliberately not
  endpoints" note is an omission note, not a design — app-internal user actions need a designed
  home. Evidence: kinako s1/plan.md designed no browse or setup surface at all; four of the five
  findings pass through that vacancy (F2 no surface to task; F3 no write-action in the surface;
  F4 no load-timing design; F5 no display-form design). The full surfaces-inventory *artifact*
  was deliberately NOT adopted (overfit guard) — revisit at the next GUI-project run.
- [ ] **Watch-line (deferred; n=1 each — upgrade on a second instance, per the meta-rule):**
  (a) **scaffolding→production promotions re-open design review** — for now one prompt line in
  `patterns-technical-decisions`' promotion-decision guidance ("which constraints did the
  scaffolding context never face — launch context, inherited environment, packaging?"; kinako F1
  / D-011 promoted the terminal orchestrator examining only CI-testability); (b) **out-of-pipeline
  integration steps** — cut as a standalone rule (covered by the journey-gate + residues items);
  re-raise if a feature-close wiring step again bypasses gates.

---

## Workflow scoping

Notes for upcoming workflows, to be fleshed out before building starts.

- [x] **`plan`** — DONE (ported 2026-07-01; ROADMAP Decision Trail; plan-port follow-ups below). techspec merge excluded (plan IS the merged form); analyst/task-architect overlap resolved.

- [x] **`tasks`** — DONE (ported 2026-07-01; tasks-port follow-ups below + `.mochiko/transform/tasks/report.md`). Cluster = task-architect + patterns-vertical-tdd + validation-task-artifacts + templates; strategy/state-analyst not consumed.

- [x] **`implement`** — DONE (ported 2026-07-01; implement-port follow-ups below + `.mochiko/transform/implement/report.md`). Cluster = staff-engineer + qa-engineer + executing-tdd-cycle + testing-end-user + brownfield-integration; strategy-implementation dissolved; sequential-first.

- [x] **`brainstorm` (net-new, not a HIL port)** — BUILT 2026-07-02, since rewritten v2/v2.1/v2.2 (REGISTRY brainstorm row is current; brainstorm-build + brainstorm-v2 follow-ups below).

- [x] **`slice` / vertical-graduation substrate (net-new, not a HIL port)** — SCOPED + BUILT 2026-07-02 (synthesis: `.mochiko/brainstorms/vertical-graduation/synthesis.md`; slice-build follow-ups below). Per-slice graduation through plan→tasks→implement; `/mochiko:graduate` wrapper deferred to dogfooding (tracked below).

- [ ] **`audit`** — lowest priority; scope after `implement` is landed. **Datapoint (2026-07-02):** the vertical-graduation scoping hands audit a concrete charter — the feature-close verification pass (qa-engineer executes the `slices.md` Feature-Done section — SC-XXX coverage map + cross-slice seams — against real infra; human FEATURE-DONE gate). Now that `/mochiko:slice` is built, the section exists in every decomposed feature and is deliberately executable by hand until audit owns it. Audit's scoping should decide whether that pass is the whole workflow or one branch.

---

## Setup-port follow-ups (from the `/mochiko:transform-cluster setup` run, 2026-06-27)

Concrete deferred work left by the setup-cluster port (**core-only** scope). Full run record + rationale in `.mochiko/transform/setup/` (`report.md`, `reconcile.md`, per-primitive assessments). Each item has enough context to resume cold. Shipped in PR #4 (`port-setup-cluster`).

- [ ] **Wire the cross-cutting stubs when their clusters port.** The setup port left `syncing-claude-md` (CLAUDE.md governance sync) and `authoring-roadmap` (evolution-roadmap) as *documented reference stubs*, not live skill mounts — mounting an unported skill would dangle. They were removed from `principal-architect`'s `skills:` list this run. Locations to update later: `plugins/mochiko/skills/authoring-constitution/SKILL.md` (Related + the CLAUDE.md-sync section spec), `plugins/mochiko/commands/setup.md` (the finalize/G5 stub — v2 renamed the old Phase 5), `plugins/mochiko/agents/principal-architect.md` (re-add to `skills:`). When those clusters land, re-mount and replace the stub prose.

- [ ] **Port `codebase-inventory-schema.json` with the spec/plan cluster.** Orphan in setup (no consumer after mode-scoping `analysis-codebase` to its setup-brownfield slice). Its `collision_risks`/`spec_item` shape is the contract for `analysis-codebase`'s *collision / spec-plan mode*, which was tagged moved-to-other-cluster. Port it alongside that mode when spec/plan is built and wire its consumer then. HIL source: `human-in-loop/plugins/humaninloop/templates/codebase-inventory-schema.json`.

- [x] **Confirm `brownfield-integration`'s home (REGISTRY mis-file)** — DONE (closed during grooming 2026-07-24): home confirmed **implement** at the implement port 2026-07-01; REGISTRY now files it under the implement cluster ("mis-file corrected — home confirmed implement").

- [x] **Decide `approved-domain-deps.md`'s fate** — RESOLVED 2026-07-21 (`domain-dependency-allowlist`; v0.18.0). No standalone doc: list → domain-rules file `mochiko:domain-registry` block, policy → ledger, craft → `authoring-constitution/references/DOMAIN-DEPENDENCIES.md`. Record: `.mochiko/brainstorms/domain-dependency-allowlist/record.md`.

- [x] **Dogfood `/mochiko:setup` v1** — SUPERSEDED by the v2 rewrite (2026-07-16); behavioral check moved to the Setup-v2 / Constitution-dissolution follow-ups below.

- [x] **Confirm the two empirical structural calls in `specify`** — DONE 2026-06-27 (ROADMAP Key Decisions): human-gate placement (on gated dispositions + escalations) and the memory model both confirmed.

- [x] **Validate the agent↔workflow decoupling doctrine by porting `specify`** — DONE 2026-06-27: doctrine HELD (no new deny-list token; rules landed in personas + conventions 4 & 5). Record: `.mochiko/brainstorms/agent-decoupling/synthesis.md`; ROADMAP Key Decisions + Decision Trail.

## Specify-port follow-ups (from the `/mochiko:transform-cluster specify` run, 2026-06-27)

Deferred work from the specify-cluster port (**core-only** scope). Full run record in `.mochiko/transform/specify/` (`report.md`, `reconcile.md`, per-primitive assessments). All 14 primitives PASSed independent verification in a single round.

- [x] **Re-mount `devils-advocate`'s deferred validation skills** — DONE (plan + tasks ports, 2026-07-01); advocate is now the cross-workflow reviewer, last stubbed mount closed.
- [x] **`strategy-implementation` dissolved into `loop-discipline`** — DONE (implement port, 2026-07-01); 3rd/final strategy dissolution, strategy family fully dissolved.
- [ ] **`ui-designer` + design track remain deferred** (out of specify-core; the specify catalog never invokes them). Scope with a dedicated design cluster or `plan`. HIL sources: `agents/ui-designer.md` + `skills/{analysis-screenshot, authoring-design-system, patterns-flow-mapping, patterns-interface-design}`.
- [ ] **Dogfood `/mochiko:specify` for real (behavioral validation).** The port passed *structural* verification but has not run end-to-end. Run it on a real feature to confirm the produce→critique loop converges, the advocate's gap list flows back as a targeted revision, the gap-classification routing (Explore / clarification / halt) fires sensibly, and the G3 acceptance gate works. Pairs with the still-open dogfood-`/mochiko:setup` check. **Since the 2026-07-19 team-form conversion (v0.13.0), this run is also the D2/S4 confirm-or-revert checkpoint:** it confirms the conversion or reverts to one-shot Layer-1 form. Team-form named checks: the producer probe fires the addressability check; the standing producer seat is messaged (not respawned) on round > 1 with the gap list verbatim; the critic spawns cold at first critique and never contacts the producer; a revert (if earned) is logged as a `RETURNED:` entry in `.mochiko/strips/specify.md`.

## Plan-port follow-ups (from the `/mochiko:transform-cluster plan` run, 2026-07-01)

Deferred work from the plan-cluster port (**core-only** scope). Full run record in `.mochiko/transform/plan/` (`report.md`, `reconcile.md`, per-primitive assessments + traces). All 15 primitives + the wiring PASSed independent verification in a single round, zero required fixes.

- [x] **Re-mount `validation-task-artifacts` on `devils-advocate` when `tasks` ports** — DONE (tasks port, 2026-07-01); no stubbed mounts remain on the advocate.
- [ ] **Reclaim `review-plan-artifacts`'s (né `validation-plan-artifacts`, renamed 2026-07-18) Phase-A0 codebase-discovery review when the brownfield/discovery track ports.** Tagged `moved-to-other-cluster` (out of plan-core); the review capability is *parked* (documented in the skill's `references/ARTIFACT-CHECKLISTS.md`), not dropped — the receiving cluster must reclaim it.
- [ ] **`evolution-roadmap-template` + `authoring-roadmap` remain deferred** (roadmap / setup-brownfield track — REGISTRY-filed under plan but never invoked by `/plan`). Port with the roadmap track.
- [ ] **Design track (`patterns-flow-mapping`, `patterns-interface-design`, `ui-designer`) + `patterns-vertical-tdd` remain deferred** — they belong to the design track and the tasks cluster, not plan-core. (The specify-port design-track deferral already tracks the first three.)
- [ ] **Dogfood `/mochiko:plan` for real (behavioral validation).** The port passed *structural* verification but has not run end-to-end. Run it on a real feature (after `/mochiko:specify` produces a spec) to confirm: the producer→two-reviewer loop converges, the feasibility-once-then-completeness ordering + skip-architect routing fire sensibly, the `infeasible`→business-escalation gate works, the Phase-2 incremental review works, and the G5 `plan.md` acceptance gate fires. Pairs with the still-open dogfood-`/mochiko:setup` + `/mochiko:specify` checks. **Since the 2026-07-19 team-form conversion (v0.15.0), this run is also the D2/S4 confirm-or-revert checkpoint:** it confirms the conversion or reverts to one-shot Layer-1 form. Team-form named checks: the producer probe fires the addressability check; the standing producer seat is messaged (not respawned) across rounds **and across the phase boundary** (Phase-1 analysis rationale carried into Phase-2 design); the completeness advocate spawns cold and is messaged in Phase 2 for incremental mode (never respawned); the feasibility architect spawns cold, fires once post-Phase-1-produce, and re-fires only on a structural change; neither reviewer contacts the producer; a revert (if earned) is logged as a `RETURNED:` entry in `.mochiko/strips/plan.md`.
- [x] **Quickstart / Integration Guide label aligned** — DONE: `technical-analyst` deliverable #6 relabeled to "Quickstart / Integration Guide" (file stays `quickstart.md`).

## Tasks-port follow-ups (from the `/mochiko:transform-cluster tasks` run, 2026-07-01)

Deferred work from the tasks-cluster port (**core-only** scope). Full run record in `.mochiko/transform/tasks/` (`report.md`, `reconcile.md`, per-primitive assessments + realized traces). All 6 primitives + wiring PASSed independent verification in a single round, zero required fixes.

- [x] **Reclaim the qa `**TEST:**` runtime-classification** — DONE (implement port, 2026-07-01); reclaimed into `testing-end-user`, drives the confidence-based per-cycle gate.
- [x] **Confirm the `patterns-vertical-tdd` ↔ `executing-tdd-cycle` boundary** — DONE (implement port, 2026-07-01): confirmed disjoint (design-time structuring vs runtime execution; 0 `**TEST:**` overlap).
- [ ] **Roadmap track stays deferred; the task-architect's `[GAP:XXX]` read is a documented stub.** Core-only scope left `authoring-roadmap` + `evolution-roadmap-template` deferred (as setup/specify/plan did). The `task-architect` keeps the brownfield-marker *craft* ([NEW]/[EXTEND]/[MODIFY]/[GAP:XXX]) but the `evolution-roadmap.md` file read is a reference stub, not a live mount. Wire it when the roadmap track ports.
- [ ] **Dogfood `/mochiko:tasks` for real (behavioral validation).** The port passed *structural* verification but has not run end-to-end. Run it on a real feature (after `/mochiko:plan` produces a plan) to confirm: the producer→reviewer loop converges, the two-phase Mapping→Tasks sequence + the early mapping-review gate fire, the cumulative review (tasks.md ↔ task-mapping.md) works, the "Research this" knowledge→Explore branch fires, and the G5 `tasks.md` acceptance gate works. Pairs with the still-open dogfood-setup/specify/plan checks. **Since the 2026-07-19 team-form conversion (v0.16.0), this run is also the D2/S4 confirm-or-revert checkpoint:** it confirms the conversion or reverts to one-shot Layer-1 form. Team-form named checks: the producer probe fires the addressability check; the standing producer seat is messaged (not respawned) across rounds **and across the Mapping→Tasks phase boundary** (Phase-1 slicing judgment carried into the Phase-2 expansion); the reviewer spawns cold at the first mapping review, is messaged in Phase 2 for **cumulative** mode (both artifact sets supplied — never respawned), and never contacts the producer; a revert (if earned) is logged as a `RETURNED:` entry in `.mochiko/strips/tasks.md`.
- [x] **Decoupling doctrine proven on an adversarial input** — DONE: `task-architect` (the worst coupled case) transformed with zero residual deny-list tokens; 4th consecutive pass.
- [x] **Optional cosmetic** — `tasks-template.md` L21 em-dash vs hyphen `**TEST:**` gloss; no drift, left as-is (scrub if a future pass touches the file).

## Implement-port follow-ups (from the `/mochiko:transform-cluster implement` run, 2026-07-01)

Deferred work from the implement-cluster port (**core-only** scope). Full run record in `.mochiko/transform/implement/` (`report.md`, `reconcile.md`, per-primitive assessments + realized traces). All 7 primitives + wiring PASSed independent verification in a single round, zero required fixes.

- [ ] **Dogfood `/mochiko:implement` for real (behavioral validation).** The port passed *structural* verification but has not run end-to-end. Run it on a real feature (after `/mochiko:tasks` produces a `tasks.md`) to confirm: the sequential cycle loop (foundation-before-feature) executes, each staff cycle is followed by a qa verification, the confidence-based per-cycle gate (CLI-100%-pass auto-approve vs GUI/subjective/failure checkpoint) fires, the targeted-retry + fix-pass bounds hold, and the named final-acceptance gate works. Pairs with the still-open dogfood-setup/specify/plan/tasks checks. **Since the 2026-07-19 team-form conversion (v0.17.0), this run is also the D2/S4 confirm-or-revert checkpoint:** it confirms the conversion or reverts to one-shot Layer-1 form. Team-form named checks: the producer probe fires the addressability check (the foundation-cycle-1 implement); the standing producer seat is messaged (not respawned) across cycles, targeted retries, **and the cycle→fix-pass boundary** (whole-implementation knowledge carried into a cross-cycle fix pass); the qa seat spawns cold at the first cycle verification, is messaged per-cycle and for the whole-implementation final validation (never respawned), and never contacts the producer; a revert (if earned) is logged as a `RETURNED:` entry in `.mochiko/strips/implement.md`.
- [ ] **The parallelism deferral is now a live `deliberate-shortcut-ledger` candidate.** Sequential-first was a documented deferral, not a silent drop — the shortcut is: no native `pipeline()`/`parallel()` and no artifact-DAG/kernel this run. Revisit (add native parallel dispatch, or reopen the lightweight kernel/hook question) **if dogfooding shows sequential cycle execution is too slow** on a real dependency-rich feature. This is the concrete drift-trigger the "code-free until dogfooding" kernel decision was waiting for.
- [ ] **`approved-domain-deps` + cross-cutting utilities remain deferred** (out of implement-core scope). `approved-domain-deps.md` follows the governance/constitution track (reference-stub); `using-git-worktrees`, `using-github-issues`, `syncing-claude-md` are cross-cutting utilities, not implement-specific. Port with their own tracks.
- [ ] **`qa-engineer` still owes its `audit`-cluster affinity.** The agent is REGISTRY-filed for both `implement` (done) and `audit` (pending). When `audit` ports, reclaim the affinity — confirm `qa-engineer` (or its verification skill) serves the audit workflow without a produce+grade leak.

## Setup-v2 rewrite follow-ups (from the constitution-flexibility rebuild, 2026-07-16)

The rewrite of `/mochiko:setup` per the accepted `setup-constitution-flexibility` record (design:
`.mochiko/brainstorms/setup-constitution-flexibility/record.md`; build decisions: ROADMAP Key
Decisions + Decision Trail, 2026-07-16). Stage 1 (everything except the frontend shelf) landed at
plugin v0.7.0.

- [ ] **Stage 2 — author the frontend catalog shelf (A1, user-ruled in-scope).** `catalog/frontend.md`: type principles (accessibility, bundle/performance budgets, component/state discipline, release gates) **plus frontend-appropriate floor examples** (the universal shelf's worked examples are backend-flavored — RFC 7807, `/health` — and the catalog README carries an honesty note pointing here). CLI/library shelves stay empty by the same ruling (mint-driven via the D9 seam) — do NOT author them speculatively.
- [x] **Dogfood `/mochiko:setup` v2** — SUPERSEDED by the v3 dissolution rebuild (2026-07-18); behavioral check moved to the Constitution-dissolution follow-ups below (v2 named checks carried forward).
- [x] **Watch the amend-backfill path** — MOOT under v3 (2026-07-18): no constitution.md is supported (D6); amend operates on persisted `governance-intent.md` directly.
- [ ] **Watch the trace-check ergonomics — re-targeted at v3's manifest grading.** The validator's surface grew: trace closure both ways across CLAUDE.md region + rules files + ledger (companion-entry completeness), not one file. Same watch: false FAILs on trace mechanics (GI-XXX placeholders, IDs drifting on G3 edits, index↔file mismatches from renames) vs substance; if the fix-list loop thrashes on mechanics, tighten the templates before touching the check.
- [ ] **D9 graduation seam stays deferred.** After a few real sessions mint principles, harvest candidates from the trace stamps and design the catalog-graduation pass (curation authority, admission bar, versioning) — from real artifacts, not speculation.

## Constitution-dissolution follow-ups (setup v3, 2026-07-18)

The dissolution of `constitution.md` into native surfaces, built from the accepted
`constitution-native-surfaces` record (design: `.mochiko/brainstorms/constitution-native-surfaces/record.md`;
ruling: ROADMAP Key Decisions + Decision Trail, 2026-07-18).

- [x] **Investigate the D7 substrate defect** — RESOLVED 2026-07-18: root cause = teammates ride the same Agent tool as subagents (docs: "Claude may sometimes use subagents instead of creating a team"). Fix (D1): seat-transport recipe single-sourced in `agent-dispatch.md` v3 + post-spawn addressability check in both team-form commands. Verification rides the v3 dogfood item below. Record: `.mochiko/brainstorms/setup-v3-team-defect/record.md`.
- [ ] **Defect: `validation-constitution/references/QUALITY-CHECKLIST.md` "Structure Quality — universal core" block is pre-dissolution stale.** Surfaced by the wave-1 audit (2026-07-23, out-of-scope observation): the block still checks the dissolved `constitution.md` document shape — "SYNC IMPACT REPORT present as HTML comment" (the concept whose orphaned reference D6b just deleted), Overview section, Roman-numeral Core Principles, Technology Stack / Quality Gates tables — none of which exist in the v3 surface set (CLAUDE.md region + rules files + ledger). The SKILL.md's manifest/trace-closure grading is current; only this reference section went stale. Rewrite the block against the surface set (or delete it if the SKILL.md checks supersede it) — a latent false-FAIL generator for the setup validator. Needs setup-cluster context; deliberately not fixed inside the token-reduction wave.
- [ ] **Empirical: HTML-comment stripping in `.claude/rules/` files.** Docs confirm stripping for CLAUDE.md, are silent for rules files (fact-check 2026-07-18). The ledger is metadata-primary regardless (D2 as amended); if a test shows rules-file comments are stripped, in-file trace comments become a free option. Method the checker suggested: an `InstructionsLoaded` hook or `/context` check on a comment-bearing rule file.
- [ ] **Empirical: fresh-session rules-loading test.** The session's probe (marker rules file → freshly spawned agent → marker absent) carried a stated confound: the file was created mid-session, so the negative can mean spawned agents don't get project rules on that path, or rules snapshot at main-session start. Re-run with the rule file present **before** session start. Either result stands: universal principles stay in CLAUDE.md (doc-confirmed); this only decides how much the scope-bound obligated-read must carry.
- [ ] **Dogfood `/mochiko:setup` v3 for real (supersedes the v2 dogfood check).** Named checks carried over from v2 (interrogation-as-session, deck arbitration, G3 fidelity catch, flagged proposals reaching G4 un-folded, teams-unset refusal) **plus v3's own:** the producer authors the full surface set; the validator's manifest grading closes the trace both ways and catches a seeded missing-companion; the governance region regenerates idempotently around hand-edited CLAUDE.md content; a superseded `constitution.md` is deleted on sight; a downstream command's producer demonstrably receives CLAUDE.md governance natively (no read-and-inject) and obeys the obligated read for scoped rules; **team-form observed** (standing producer seat messaged across rounds — feeds the substrate item). **This check is now also D1's acceptance test for the team-transport fix (2026-07-18):** resume the paused kinako run at *loop (produce)* per the recovery table — synthesis is ratified, surface set absent. Record: `.mochiko/brainstorms/setup-v3-team-defect/record.md`.
- [x] **Productize the governance-injection probe suite** — BUILT 2026-07-19 (v0.12.0; ROADMAP Key Decisions). Landed as the standalone `testing-governance-injection` skill + a G5 offer when the set carries rules files (REGISTRY row is current). The two empirical items above run as probe cases; residual dogfood watch (G5 offer fires with honest cost, probes spawn as subagents, clean tree, seeded scope gap caught) rides the setup-v3 dogfood item above.

## Setup-adversarial-review build items (from the accepted `setup-adversarial-review` record, 2026-07-18)

The G3 adversarial review + the `validation-*`/`review-*` split, designed and accepted 2026-07-18
(record: `.mochiko/brainstorms/setup-adversarial-review/record.md` — D1–D7 + 13 review folds;
ruling: ROADMAP Key Decisions, 2026-07-18). **Built 2026-07-18 at plugin v0.10.0** — the four
items landed as one build; the dogfood item below stays open.

- [x] **Author `review-governance-intent` (D4, D7.3) — the sixth `devils-advocate` mount** — BUILT 2026-07-18 (REGISTRY row + `agents/devils-advocate.md` + `review-governance-intent` SKILL are current; cross-exam protocol single-sourced from `review-brainstorm/references/CROSS-EXAM.md`). Record: `.mochiko/brainstorms/setup-adversarial-review/record.md`.
- [x] **Template deltas (D2, D7.7)** — BUILT 2026-07-18: `governance-intent-template.md` confidence marks + per-run Review section (REGISTRY row is current).
- [x] **`setup.md` propagation (D7.8/S2)** — BUILT 2026-07-18: synthesis-review stage + reshaped G3 + intent-reviewer seat + recovery rows all landed.
- [x] **The seven-skill rename set (D5/S5)** — BUILT 2026-07-18: `validation-*`/`review-*` split across all six dirs + every live ref; zero old-name refs remain under `plugins/` (REGISTRY Skills note is current).
- [ ] **Dogfood rides the setup-v3 dogfood item.** Once built, the review stage's named checks fold into the existing v3 dogfood run (Constitution-dissolution follow-ups above): the sizing gate fires with a tier-keyed recommendation, a waiver actually lands in the Review section, survivor routing honors the user-territory buckets, and the G3-edit delta-pass triggers on a material edit.

## Pattern-codification build items (from the accepted `pattern-codification-and-minimalism` record, 2026-07-18)

The team-form shape codification + library minimalism machinery, accepted 2026-07-18 (record:
`.mochiko/brainstorms/pattern-codification-and-minimalism/record.md` — D1–D9 + 13 folds; ROADMAP
Key Decisions). Items 1–6 (codification) built 2026-07-19 at v0.11.0; item 7's cluster waves ran
through v0.17.0 (see below).

- [x] **Author the pattern-surface template (D3/D8, S2/S8)** — BUILT `templates/command-shape.md` (v0.11.0); the shape's sole authoritative home (form-agnostic core + team-transport layer). S8 home-revision checkpoint resolved at the specify conversion (v2). REGISTRY row is current.
- [x] **Author `command-architect` + its authoring skill (D8, S6)** — BUILT `agents/command-architect.md` + `skills/authoring-commands/` (v0.11.0); references the template, never restates it. REGISTRY rows are current.
- [x] **Author the `validation-*` audit skill on `validator` (D3/D8, S1)** — BUILT `skills/validation-command-shape/` (v0.11.0); grep floor beneath prose judgment; apply and audit on different agents. REGISTRY row is current.
- [x] **Establish the strip-notes convention (D6, S10)** — BUILT (`.mochiko/strips/README.md`); one non-loaded note per primitive, repo-side never under `plugins/`; strip + survivor-provenance entry types.
- [x] **Demote `agent-dispatch.md`'s one-shot line (D2 implication)** — BUILT: one-shot dispatch is now a rebuttable default pending per-command assessments.
- [x] **Fix `specify.md`'s missing `disable-model-invocation` frontmatter** — BUILT.
- [ ] **Run the strip pass in cluster waves (D5, D7, D9, S7/S13).** All five one-shot-command waves
  DONE 2026-07-19 (specify v0.13.0 · slice v0.14.0 · plan v0.15.0 · tasks v0.16.0 · implement v0.17.0,
  audit closed 2026-07-20) — every command now team-form in the codified shape, each independently
  audited **PASS**; per-wave records in `.mochiko/strips/*` + REGISTRY command/agent/skill rows + the
  Decision Trail. **Remaining:** the brainstorm/setup light residual pass — reads as already discharged
  by the v0.11.0 pre-shrink (grooming flag 2026-07-24: confirm whether this closes the item). **Re-add
  protocol (D7, S5 — still live):** user supplies dogfood logs; re-adds identified collaboratively
  against the primitive's strip notes, evidence-linked by default with marked overrides; any version
  bump with re-adds triggers the validator audit on touched primitives; an override-count threshold
  flags a primitive for audit regardless.
- [ ] **≥3-consumer escalation queue (raised across the five waves, not yet ruled — D9's guard).**
  Rule each at the scheduled all-consumer pass, Tier-2-tested against every consumer; per-wave
  provenance is in the `.mochiko/strips/*` notes. Net open queue:
  1. **`devils-advocate`'s six per-mount "Skills Available" paragraphs** — each restates its mounted
     skill's own `description:`; dedupe across all six clusters. The other five agents' instances were
     ruled in-wave (`task-architect` · `principal-architect` · `technical-analyst` · `staff-engineer` ·
     `qa-engineer`; `requirements-analyst` already at altitude) — `devils-advocate` is the sole
     remaining agent instance.
  2. **`devils-advocate`'s "What You Hunt For" generic gap taxonomy** — its own closing line points to
     `review-specifications` yet copies the catalog; persona-generic-vs-copied-catalog dedupe, rule at
     the same all-consumer `devils-advocate` pass.
  3. **`slices-template.md`'s Graduation-contract section** (consumed by the plan/tasks/implement
     slice-scoped entries, now all locally de-restated) — its Staleness-guard/Regression-safety rules
     overlap `authoring-slices`' spec-stamp duty + the regression-by-construction default; rule against
     all three consuming entries at once.
  4. **The letter/spirit aphorism** — 11 skill copies + the canonical `loop-discipline` home; a
     library-wide keep-or-strip ruling (if kept, survivor-log the set; if stripped, replace copies with
     a `loop-discipline` reference).

## Knowledge-management-module follow-ups (from the operating-docs-scaffolding build, 2026-07-17)

The operating-docs layer landed as an elective, default-on constitution module at plugin v0.8.0
(design: `.mochiko/brainstorms/setup-operating-docs-scaffolding/record.md`; ruling: ROADMAP Key
Decisions, 2026-07-17). Build surface: `constitution-modules/knowledge-management.md` (new) ·
`INTERROGATION-AGENDA.md` (dimension #7, agenda now ten) · `constitution-template.md` (sync rows)
· `governance-intent-template.md` (module rulings record declines; dimension renumbers) ·
`setup.md` (G5 scaffolding + amend module-offer) · `brainstorm.md` (index bookkeeping + open/close
invariants) · router + validator checklist rows.

- [ ] **Dogfood the KM path inside the setup-v2 dogfood run.** Named checks: dimension #7 fires after existing-practices and reads as a session beat, not a form; a decline records durably in `governance-intent.md` and memorializes in dimension #10; G5 scaffolds all four artifacts never-overwriting (the module's first repo-root writes); the brownfield collision beat fires only on real dimension-#6 evidence and its ruling lands in the module's collision-rulings slot; an amend run offers unruled modules once (and never re-asks a recorded decline).
- [ ] **Watch brainstorm's index bookkeeping + invariants on a KM-adopted project.** Read-index-before-open fires; the entry lands on open and updates at acceptance with a named landing; the open/close invariants catch a seeded defect under fix-on-sight; the no-module branch stays silent on a declined project. Honest residual accepted at review: index drift on a project that opens no further sessions is caught only at the next setup/amend re-audit (kernel-free tradeoff, deliberate).
- [ ] **Wire the KM sync rows live when `syncing-claude-md` ports.** The module's Mandatory-Sync-Artifacts rows are honestly stub-backed until then (extends the existing "wire the cross-cutting stubs" setup-port item above).

## Brainstorm-build follow-ups (from the net-new `/mochiko:brainstorm` build, 2026-07-02)

Deferred work from the first net-new workflow build. Design record: `.mochiko/brainstorms/brainstorm-command/synthesis.md`. **v1 was rewritten 2026-07-04 (see the v2 section below)** — these items resolved as follows:

- [x] **Dogfood `/mochiko:brainstorm` v1** — DONE 2026-07-03/04: substance ✓, ceremony ✗ (illegible flow, unreachable done-condition, ≈620k tokens); drove the v2 rewrite. Evidence: `.mochiko/brainstorms/setup-workflow-rewrite/dogfood-handover.md`.
- [x] **Watch the plan ramp's spec quality** — MOOT (2026-07-04): v2 removed the ramps.
- [x] **Watch the `/mochiko:tasks` entry variant for scope leak** — RETIRED (2026-07-04): v2 removed the stamp + variant + `handoff-brief-template.md`.

## Brainstorm-v2 follow-ups (from the agent-team rewrite, 2026-07-04; revised to the end-stage review pair, 2026-07-05; sized to the lens-split review, 2026-07-16)

Design records: `.mochiko/brainstorms/brainstorm-command-rewrite/record.md` (v2, D1–D9 incl. the `Contested` hard-require and its revisit trigger) · `.mochiko/brainstorms/brainstorm-v2-revision/record.md` (v2.1 — the end-stage revision, decided from the first run's transcript forensics) · `.mochiko/brainstorms/brainstorm-v2-2-revision/record.md` (v2.2 — the sized lens-split review, decided from the first *completed* run's transcript forensics).

- [x] **Dogfood `/mochiko:brainstorm` v2 (agent-team pilot)** — RUN 2026-07-04 (partial, paused at account limit). Verdict: substance ✓, standing-advocate model ✗ (continuous-loop UX, zero teammate↔teammate messaging, not cheaper). Drove the v2.1 revision (forensics + cost table in the v2.1 record).
- [x] **Dogfood the end-stage review pair (v2.1)** — RUN 2026-07-05 to completion (`setup-constitution-flexibility`; first completed run). Verdict: tail ✓, pair coverage ✓, reading cost ✗ (triple-read of the reality surface; ≈654k total). Drove the v2.2 sized lens-split review. Artifacts: `.mochiko/brainstorms/setup-constitution-flexibility/`.
- [ ] **Dogfood the sized lens-split review (v2.2).** The review stage is now sized by the user at a named convergence gate (**pair / single / none** — "none" writes a review waiver into the record's Review section; single mode names the sole reviewer as verify owner), the pair is **lens-split** (decision-quality vs record-integrity) over the fact-checker's **verbatim-landed map** as shared fact substrate, cross-exam is **one-shot** (four messages, fact-route dedup), cross-set merge/tally is lead-owned, verify runs on the integrity reviewer only, and `synthesis.md` is sanctioned on-request post-acceptance (fidelity-sampled by the verify reviewer; "derived, unchecked" when review was waived). Named checks for the next run: **lens-split coverage bet** — does any Important-class finding arrive via the "wrong" lens, or get missed for lack of double coverage? (the found-by-both convergence signal is deliberately gone) · **one-shot exchange bet** — does severity calibration (the withdrawals/downgrades/merges class) survive a single attack/response round? (as-ran took 6 messages) · **argument-cap watch** — 0-for-2; if 0-for-3, decide dead weight vs the findings keep being simply right · the sizing gate fires sensibly, the "none" waiver text actually lands in the record, single-mode self-verify reads clearly · fact-route dedup honored · the synthesis fidelity check runs when requested and the "derived, unchecked" stamp appears on the waived path · **cost re-measure from transcripts:** target pair ≈150–170k (from 270k), total materially under 654k. Note: the v2.2 design record is itself **un-reviewed** (bare session, no cold pass) — a single-reviewer cold pass can be requested per its own D3 logic.
- [ ] **Flag-off posture revisit (the `Contested` D9 trigger).** v2 hard-requires agent teams with no fallback transport — deliberately, for dogfood fidelity. When mochiko ships beyond the author's machines (marketplace path), reopen the fallback question with real dogfood data on what degrades acceptably.
- [ ] **Watch the experimental-API churn.** Agent-teams behavior changed across five recent point releases (v2.1.178–199). If a Claude Code update breaks spawn/messaging semantics the command relies on, re-verify against the docs page and adjust — the command names capabilities (direct user↔teammate talk, teammate↔teammate messaging, cold spawn at convergence), not version-specific mechanics, to keep this surface small.

## Slice-build follow-ups (from the net-new `/mochiko:slice` build, 2026-07-02)

Deferred work from the vertical-graduation substrate build. Design record: `.mochiko/brainstorms/vertical-graduation/synthesis.md` (Risks + Open Questions name these triggers).

- [ ] **Dogfood `/mochiko:slice` + the slice-scoped pipeline for real (behavioral validation).** The build passed independent structural verification but has not run end-to-end. Run it on a real multi-story feature (after `/mochiko:specify`) to confirm: the produce→review loop converges, the null exit fires sensibly on a small spec, the foundation slice's plan genuinely establishes the shared artifacts, a later slice's plan honors extend-mode (extends, never re-derives), the staleness guard trips on a spec edit, the graded-amendment path surfaces a breaking change, and the G5 two-shape acceptance works. Pairs with the still-open dogfood-setup/specify/plan/tasks/implement/brainstorm checks — **decide the dogfooding sequencing first** (whole-spec pipeline once before slice-scoped, or the whole stack together; deliberately deferred at scoping). **Since the 2026-07-19 team-form conversion (v0.14.0), this run is also the D2/S4 confirm-or-revert checkpoint:** it confirms the conversion or reverts to one-shot Layer-1 form. Team-form named checks: the producer probe fires the addressability check; the standing producer seat is messaged (not respawned) on round > 1 with the gap list verbatim; the reviewer spawns cold at first review and never contacts the producer; the null exit and a wrong-depth flip exercise the standing seat's cross-round memory; a revert (if earned) is logged as a `RETURNED:` entry in `.mochiko/strips/slice.md`.
- [ ] **`/mochiko:graduate` stays a ledger entry.** The one-invocation-per-slice wrapper (sequences plan→tasks→implement for a slice, pausing at each existing gate) is deferred until dogfooding shows real shepherding pain — e.g. repeated manual per-slice stage runs where every handoff was mechanical and no gate intervened. Build it as a thin sequencer over the existing commands if earned; never a re-implementation of their loops.
- [ ] **Spec-change ripple / amend-mode is deliberately unsupported.** `/mochiko:slice` halts when any slice has graduated (Phase 0 guard); the staleness stamp is the only protection. If dogfooding hits a real mid-flight spec amendment, design amend-mode then: re-place changed/new stories without touching shipped slices, escalating when a shipped slice's stories changed. A recorded deferral, not an oversight.
- [ ] **Feature-close verification has no owning workflow.** `slices.md`'s Feature-Done section (SC coverage map + cross-slice seams) is declared at decomposition and deliberately executable by hand, but no command runs it — the `audit` scoping inherits it as its charter (see the audit item above). Until then the implement entry variant surfaces "declared, not verified" after the last slice — never claims feature completion.
- [ ] **Watch the two-level "slice" vocabulary in practice.** Graduation slice (spec-level, `authoring-slices`) vs vertical slice (cycle-level, `patterns-vertical-tdd`) is an explicit boundary in both skills + the router; if dogfooding shows the task-architect conflating them (e.g. `slices.md` groups shaped like cycles), sharpen the trigger descriptions or rename one level.

## Command-altitude pass (2026-06-30)

The first two command ports were re-transformed to the thin "stitch a team to a goal under a contract" altitude, and the transform recipe was fixed so future commands come out thin by construction. Full record: `.mochiko/brainstorms/command-altitude/synthesis.md`; decision in ROADMAP (Key Decisions + Decision Trail).

- [x] **Fix the transform recipe for altitude** — DONE 2026-06-30 (transformer cluster since retired).
- [x] **Retrofit `specify` + `setup`** — DONE 2026-06-30 (specify 329→66, setup 385→78; verified PASS).
- [x] **Apply the altitude shape to `plan`/`tasks`/`implement`** — DONE 2026-07-01 (82/77/80 lines, thin by construction); rollout complete.

## Ideas / candidates

Things worth holding but not yet scoped.

- [ ] **Brownfield onboarding path** — `[[brownfield-first-onboarding]]` from the synthesis; HIL has `brownfield-constitution` and `brownfield-integration` skills. Worth keeping as a distinct entry path into `setup`.

- [ ] **Context handoff document** — `[[context-handoff-document]]` for serializing in-flight work across session boundaries; pairs with the router skill. Low priority until other workflows are landed. **Data point from `setup`:** that cluster's HIL context-handoff template (`constitution-context-template`) was *absorbed into the lead* (in-session + `.mochiko/memory/`) rather than kept as a serialized doc — so a cross-session handoff doc, if pursued, is a deliberate add-on, not a carried-over HIL primitive.

- [ ] **Deliberate shortcut ledger** — `[[deliberate-shortcut-ledger]]` for tracking deferral decisions with upgrade triggers; relevant once `implement` is built and shortcuts start accumulating.
