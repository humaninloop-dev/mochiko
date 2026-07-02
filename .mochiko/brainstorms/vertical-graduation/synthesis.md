# Vertical Graduation Substrate — Design Synthesis

**Session**: 2026-07-02 · 9 structured questions; every design recommendation accepted without contest; user directed two extra dimensions before wrap-up
**Method**: `analysis-iterative` (brainstorm mode)

## Problem Statement

The pipeline's unit is the whole feature: every user story in `spec.md` crosses `plan → tasks → implement` together (horizontal batching), so P1 stories can't reach verified working code until the entire spec has been planned and tasked, and whole-spec artifacts spread producer and reviewer attention thin. The session opened with two candidate fixes — a cross-cutting *graduate* command that walks one story cluster end-to-end, and decomposing the spec into groups the existing commands consume — and resolved that the second **is** the build: the substrate (a post-spec decomposition step plus cluster-scoped entry variants) delivers both drivers by itself, and the graduate wrapper is deferred until dogfooding proves shepherding pain.

## Context & Constraints

- **Both primary drivers point at smaller pipeline units**: time-to-working-code (P1 cluster ships end-to-end early) and artifact focus (tighter context per stage). Parallelism and stage-shepherding ergonomics were explicitly *not* drivers — so no worktree-parallel clusters in v1, and ergonomics alone can't justify a new orchestrator.
- **Design artifacts are inherently cross-cutting**: `data-model.md`, `contracts/api.yaml`, constraints — the `User` entity belongs to no single cluster. Any per-cluster planning scheme must keep them coherent.
- **Nothing is dogfooded**: all five commands passed structural verification only; six dogfood items are open in BACKLOG. The substrate compounds on an untested pipeline.
- **Reuse surface**: the clustering judgment is the `task-architect`'s story→cycle craft lifted one level; extend-mode is mochiko's existing brownfield muscle applied to design artifacts; `devils-advocate` already reviews four workflows; stage state follows the workspace-as-state memory model.
- **Thin-command altitude + kernel-free are non-negotiable** — the deferred graduate wrapper was also the kernel-shaped risk (commands invoking commands).

## Key Decisions

| Decision | Choice | Confidence | Rationale |
|----------|--------|------------|-----------|
| Primary drivers | Time-to-working-code **+** artifact focus; parallelism/ergonomics secondary | Confident | Both selected explicitly; both are served by smaller pipeline units, which fixed the design direction. |
| Cluster origin | **Post-spec decomposition step** — `spec.md` stays whole; clusters live in a reviewed overlay `clusters.md` (ordered, named groups) | Confident | The advocate stress-tests the FULL spec; clustering is a distinct judgment deserving its own review; specify stays untouched; works on pre-existing specs. |
| Shared design | **Foundation cluster + extend-mode** — single accumulating design artifacts at feature root; decomposition designates the foundation cluster; later clusters' plans treat existing artifacts as brownfield input | Confident | Reuses the brownfield muscle one level up; avoids resurrecting a whole-spec upfront pass. Accepted cost: a later cluster can strain an early design. |
| Command shape | **Substrate first; graduate deferred** to the shortcut ledger with a dogfooding trigger | Confident | Both drivers are fully served without the wrapper; ergonomics wasn't a driver; a super-command on five undogfooded workflows is the kernel-shaped risk. |
| Conflict policy | **Graded amendment-in-place**: additive extensions routine; breaking changes = reviewed design amendment + migration tasks inside the *current* cluster's cycles; repeated breaking amendments (cap) → halt and re-decompose | Confident | Cost lands where the need arose — exactly how brownfield code changes already work; escalation valve reserved for thrash. |
| Decomposition home | **New tiny standalone command** (working name `/mochiko:cluster` — naming open): `task-architect` proposes via a new story-clustering skill, `devils-advocate` challenges the grouping, human gate accepts | Confident | Follows the one-artifact-one-loop pattern exactly; honors "specify stays untouched"; serves old specs. Cost: a seventh command. |
| Feature-done | **Declared at decomposition, verified at close**: `clusters.md` carries a Feature-Done section (SC-XXX → verifying-cluster coverage map + named cross-cluster seams); a feature-close verification pass executes it when all clusters ship — qa-engineer against real infra, human accepts FEATURE-DONE | Confident | loop-discipline demands the done-condition be declared before the loop runs, not invented after; feature-level SCs and seams are exactly what per-cluster verification misses. **This pass is the deferred `audit` workflow's charter.** |
| Cross-cutting stories | **Earliest meaningful home + explicit extend obligations** recorded in `clusters.md` on later clusters; a story whose independent test can't pass in ANY single cluster → spec-amendment escalation (usually an NFR wearing a story costume) | Confident | Keeps the exactly-one-home invariant while making the spread visible, not silent; graded like the conflict policy. |

### Proposed defaults (Assumed — stated in session, never contested)

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Artifact layout | `.mochiko/specs/<feature>/clusters.md` + `clusters/<name>/{plan.md, task-mapping.md, tasks.md, cycle-reports/}`; shared accumulating `data-model.md`, `contracts/`, `constraints-and-decisions.md`, `nfrs.md` at feature root | Shared artifacts are the "design codebase" clusters extend; narrative/per-stage artifacts scope per cluster. |
| Stage tracking | Workspace-as-state — a cluster's pipeline stage derived from artifact presence in its subdir; no status field | 4th-confirmed memory-model pattern; no new state machinery. |
| Gates | Per-command gates unchanged; decomposition adds one (grouping acceptance); feature-close adds one (FEATURE-DONE) | Substrate-first means no gate collapsing was ever needed. |
| Regression safety | Amendments touching shipped foundation code are caught by construction — earlier clusters' tests live in the repo; implement's quality gates run the full suite | No new machinery; Tier-1 deterministic. |
| Clustering checklist invariants | Dependency-closure (plannable given only foundation + prior clusters); exactly-one-home; ordering = dependency beats priority; soft size 2–4 stories, must-justify above; foundation cluster = P1 core journey that *also* establishes shared design (pure-plumbing clusters forbidden — plumbing lives as foundation *cycles*); **null exit** (spec wouldn't yield ≥2 clusters with distinct value seams → recommend whole-spec and stop); Feature-Done authoring duty | The judgment core of the new skill; foundation-value rule follows from the time-to-value driver. |
| Entry variants | `plan` gains extend-mode reading of accumulated artifacts + a scope guard (designing beyond the cluster's stories → halt); `tasks` reads the cluster's plan; `implement` mostly inherits (path-scoped) | Precedent: the tasks design-light entry variant with its mid-loop guard. |

## The Composed Flow

```
/mochiko:specify ──→ spec.md (whole, accepted)                    [unchanged]
        │
/mochiko:cluster (working name) ──→ clusters.md                   [NEW thin command]
        │  task-architect proposes (story-clustering skill)
        │  devils-advocate challenges grouping · human accepts
        │  · ordered clusters, foundation designated
        │  · extend obligations for cross-cutting stories
        │  · Feature-Done section: SC coverage map + seams
        │  · null exit: small spec → "run whole-spec", stop
        │
        ▼  per cluster, in clusters.md order:
   /mochiko:plan      --cluster <c>   extend-mode; shared artifacts accumulate;
   /mochiko:tasks     --cluster <c>   breaking needs → graded amendment
   /mochiko:implement --cluster <c>   full quality gates catch regressions
        │
        ▼  all clusters shipped:
   feature-close verification (= the audit charter):
   qa-engineer executes Feature-Done against real infra → FEATURE-DONE gate
```

## Decision Trail

### Command shape — the original lead idea got deferred
- **Options considered**: substrate first / build graduate now / graduate as the golden path
- **Recommendation was**: substrate first — **chosen**
- **Key reasoning**: the session's read-back showed both primary drivers fully served by the substrate alone; graduate adds only ergonomics (explicitly not a driver), each stage's human gate makes end-to-end graduation attended anyway, and commands-invoking-commands on an undogfooded pipeline is the kernel-shaped accretion risk. Deferral is a ledger entry with a trigger, not a rejection.

### Shared design — the crux trade-off
- **Options considered**: foundation cluster + extend-mode / thin upfront whole-spec design pass / cluster only after plan
- **Recommendation was**: foundation + extend-mode — **chosen**
- **Key reasoning**: the upfront pass partially resurrects horizontal batching ("thin" passes grow); clustering only after plan abandons the dilution driver at the stage where it was named. Extend-mode's honest cost — mid-pipeline design rework — was accepted and then contained by the graded amendment policy.

### Feature-done — the gap that scoped `audit`
- **Options considered**: declare-at-decomposition + verify-at-close / last cluster's gate absorbs it / done = all clusters shipped
- **Recommendation was**: declare + close — **chosen**
- **Key reasoning**: vertical graduation destroys the free whole-feature ending the horizontal model had; a done-condition invented at the end is post-hoc (loop-discipline violation); an SC no cluster covers becomes a decomposition-time gap the advocate catches. Side-effect: the deferred `audit` workflow acquires its concrete charter.

## Risks

- **Mid-pipeline design rework** (accepted cost of extend-mode): a later cluster strains shipped foundation design; contained by graded amendments + the re-decompose valve, but migration work will land inside feature clusters.
- **Compounding on an undogfooded pipeline**: the substrate multiplies invocations of five never-run-end-to-end commands. Dogfooding sequencing was deliberately deferred — decide it before building.
- **`clusters.md` staleness**: spec.md amendments mid-flight silently orphan the overlay. Ripple policy deferred; cheapest first guard is a version-stamp check at every cluster-scoped entry.
- **Silent extend-obligation pileup**: cross-cutting stories load later clusters with inherited scope; `clusters.md` must render obligations visibly and the advocate must check them at decomposition.
- **Vocabulary collision**: "cluster" already means *primitive cluster* in transform-land; shipping `/mochiko:cluster` overloads it.

## Open Questions

- **Naming** — command + artifact (candidates: `/mochiko:decompose`, `/mochiko:slice`; `clusters.md` vs `increments.md`). Resolve before build; blocks file and variant names.
- **Spec-change ripple policy** (Deferred) — amend-mode for the decomposition command; what re-placement is allowed without touching shipped clusters; staleness-guard shape.
- **Dogfooding sequencing** (Deferred) — run the existing whole-spec pipeline once on a real feature *before* building the substrate, or build first and dogfood the whole stack together.
- **Graduate's ledger trigger** — what observed pain counts as "earned"? (e.g., N per-cluster runs where every stage handoff was mechanical and no gate intervened.)
- **Audit scope** — is feature-close the whole `audit` workflow or one branch of it? Audit's scoping session should decide; it inherits the Feature-Done section as input either way.

## Recommended Next Steps

1. **Record the scoping** — BACKLOG.md entry pointing here, plus the audit-charter datapoint on the audit item (done this session).
2. **Resolve naming** — one decision, unblocks all artifact/command/variant names.
3. **Build in workflow-first order**: story-clustering skill (checklist + Feature-Done authoring + null exit) → decomposition command (thin by construction, contract from `workflow-contract`) → `plan` extend-mode entry variant (the deepest edit — scope guard + accumulating-artifact reads) → `tasks`/`implement` cluster scoping → router + REGISTRY updates.
4. **On landing, record in ROADMAP Key Decisions** — task-architect gains a second affinity (decomposition producer) and devils-advocate a fifth mount (grouping review): both touch the agent↔skill composition axis the operating manual requires recording.
5. **Ledger entries**: graduate deferral (with its trigger) and the spec-ripple deferral — deliberate shortcuts, not silent drops.
6. **Decide dogfooding sequencing first**, then dogfood the substrate on a real multi-story feature end-to-end.
