# Validator Worktree Isolation in Implement — Decision Record

- **Session:** `.mochiko/brainstorms/validator-worktree-isolation/` · 2026-08-01
- **Status:** **accepted** (user, 2026-08-01) — pair-reviewed, 19/19 survivors dispositioned incl. the U1–U7 batch; verify round 1 NOT CLEAN (B1/B2 repaired same round + 7 polish) → round 2 CLEAN. Landed: DECISIONS.md row (2026-08-01) · BACKLOG build item + Layer-2 working-tree capture. Synthesis: not generated (on request). **Built 2026-08-01 at v0.42.0** — audit PASS (fix round folded); build item → trail, U5 watches open in BACKLOG; Open threads 1/5/7 dispositioned below. Sizing trail: pair, user-sized at convergence on the lead's weight statement (6 decisions at freeze · 58-fact surface at freeze · supersedes a shipped ruled constraint). *(Resume: re-read this record; if the verify pass has not reported, re-dispatch it to reviewer-integrity; then user acceptance.)*
- **Driver (BACKLOG › Ergonomics, 2026-08-01, to-brainstorm):** give the implement command's
  validation/QA step the option to run in a git worktree for cleaner separation from the
  producer's working tree — isolate the grader from uncommitted producer state / avoid
  cross-contamination. Scope per the item: which validation gates warrant it · per-gate
  worktree setup cost · interaction with the cold-checkout step already ruled into implement's
  final validation (2026-07-31).
- **Problem sharpening (user, Q1):** the driving pain is **mutual interference** — producer
  and verifier are "not aware of each other, and stepping on each other's toes" in the one
  shared working tree. Bidirectional (not only inbound pollution of qa's read, not only qa's
  side effects dirtying the producer's tree), and framed as suspicion from watching runs
  ("I think"), not yet a pinned incident.
- **Q2 (timing):** unsure whether true overlap or alternation residue — but an incident WAS
  observed: "I saw it happen, and producer and validators were confused." Mechanism unpinned;
  whether the shipped flow permits temporal overlap routes to the fact-checker.
- **Q3 (incident trace):** the observed incident is that a run's **validator seat itself
  recommended worktree isolation** — the seat's own recommendation is the capture behind the
  BACKLOG item. Which run / where that report lives: unpinned by the user; routed to the
  fact-checker as a disk search (verification/cycle reports mentioning worktrees).
- **Questioning trail (stems, for traceability — added at review, S5):** Q1 driver (inbound
  pollution / outbound contamination / parallelism / observed incident) · Q2 timing (true
  overlap / alternation residue / unsure) · Q3 incident trace (capture exists / run named /
  memory only) · Q4 scope — (a) validator-only, (b) symmetric separation, (c) facts-decide →
  ruled (a), D1 · Q5 commit boundary — (a) user-commit gate before final validation, (b)
  diff-transport, (c) demote cold checkout → (a) rejected, D2 · Q6 evidence posture / gate
  scope — (a) fixed core + composable option, (b) always-on per-cycle, (c) fact-backed core
  only → ruled (a), D3 · Q7 batch — mechanism card (→ D4) + per-cycle trigger card ((a) lead
  judgment / (b) deterministic triggers / (c) run-start ask → (a)) — both as recommended ·
  U1–U7 — the review ruling batch, stems in the Review section.
- **Reality surface (fact-checker seat filled — probe):**
  `plugins/mochiko/commands/implement.md` (verifier seat, cycle checkpoint + devolved branch,
  cold-checkout constraint, final validation) · `qa-engineer` × `testing-end-user` ·
  `executing-tdd-cycle` (the producer report qa reads) · the cold-checkout ruling (ADR
  `2026-07-31-team-method-escalations-closed`; origin `team-method-vs-command-shape`) ·
  command-shape Layer 1 ground rules (the git-mutation ban) · git-worktree semantics + the
  harness's native isolation surfaces.

## Decisions

*(D1… as ruled — statement + rationale + confidence mark each; user corrections and reversals
logged where they happen.)*

**D0 — The null option, killed on the record.** `Confident` (user-ruled at review, U6)
Build-versus-don't-build was never dealt during questioning (review finding S10); ruled at
review: the map's against-column (F7 no concurrent writers · F19 independence never required
filesystem separation · F35 "ergonomics alone can't justify" · F49 the one prior worktree
primitive abandoned for no demand) loses to one fact alone — **F24: the shipped cold-checkout
constraint, executed as written against an uncommitted implementation, gates a tree that does
not contain the work under validation.** That is a defect in ruled doctrine requiring repair
regardless of the incident's evidence grade; the per-cycle option rides as optional
ergonomics (D3(ii), D5) and would not have justified the build alone (consistent with F35).

**D1 — Scope: validator-only isolation.** `Confident` (user-ruled, Q4)
The design separates the **qa/verifier side** only; the producer keeps owning the working
tree, which stays the single source of truth. Symmetric seat-per-worktree separation (each
seat in its own tree with a merge point) is out of scope for this session — it attacks the
driver more directly but opens merge/ownership semantics against the sequential-cycle
doctrine, and the **filed capture** was validator-side (F47 for provenance; the Driver
line's own scope — "the implement command's validation/QA step" — for the validator-side
reading) — the incident itself is `Assumed`, per D3(iii) (qualified at review, S9).
Revisit trigger (S12): if the sequential-only deferral (F7 — an explicit
`deliberate-shortcut-ledger` deferral, not a permanent property) is ever lifted for parallel
cycle execution, this rejected road re-opens.

**D2 — No user-commit gate.** `Confident` (user-ruled, Q5 — rejecting the lead's recommended
option (a))
A mid-run commit checkpoint ("pause, you commit, then cold validation runs on the commit") is
**out**: "will create friction with too much approval." Binding consequence for everything
downstream: every isolation mechanism must work on the **uncommitted** working state with no
user action added to the run — which also rules out closing the F24 cold-checkout gap by
commit-gating it.

**D3 — Gate scope: fixed core + composable option.** `Confident` (user-ruled, Q6 — adopting
the lead's recommendation; (i) enumerated and (ii) amended at review, S1 / S15(U5))
(i) **Final validation always runs against a cold tree built from the uncommitted working
state** — superseding the 2026-07-31 cold-checkout ruling's "fresh clone" **mechanism** while
preserving its **intent**, closing the F24 gap (a clone/worktree of HEAD does not contain the
uncommitted work; F22–F24). *What is preserved, enumerated:* the works-warm-only catch
(F14/F15 — gitignored dirs absent either way, F32/F57c) and G5 evidence status. *What
changes, enumerated:* the clone gated **what the repository contains**; the snapshot gates
**what the disk contains** — a strict superset (untracked non-ignored files ride, F57b).
Residual, accepted under D2: a selective user commit that later omits a file the snapshot
contained can still ship the works-warm class — uncatchable without a commit boundary; the
tracked-only snapshot variant (F59/F60: commit-equivalent; hard-fails loudly on deleted
tracked files) is recorded as **available, not canonical**, since it drops exactly what a TDD
cycle creates (new files). *Carrier set for the build, one disposition each (the strip
entry's `Consumers assessed` source):* `implement.md:105-107` — edited under
supersession-by-ruling ceremony citing this record · the ADR
`.mochiko/decisions/2026-07-31-team-method-escalations-closed.md:35-38` (F13) — gains a
superseded-mechanism marker pointing here, intent standing · `DECISIONS.md:20` — the
cold-checkout index row annotated in the same landing.
(ii) **Per-cycle qa isolation is an option the lead composes per run** — never always-on;
the BACKLOG item's own word ("the option"). The per-cycle cost is only paid when composed in
— and it is unmeasured (F33 UNVERIFIED). **Ruled at review (U5):** the cost is measured at
the first composed-in run, and the triggering interference *observation* is recorded on the
run whether or not the option is composed in — the design generates the citable evidence
this session lacked.
(iii) The driving incident ("producer and validators confused, stepping on toes") is marked
**`Assumed`** — user recollection, corroborated in class by F44 (qa graded uncommitted
working-tree state in kinako) and F48 (three authoring-side working-tree collisions), with no
citable run artifact (incident-trace dispatch, F41–F49 and its verdict block).

**D4 — Cold-tree mechanism: the git-semantics filtered snapshot.** `Confident` (user-ruled,
Q7 batch — adopting the lead's recommendation; caveat (i) resolved by U1 at review, scope
bound added per S17)
The canonical mechanism for every cold tree this design creates is a **filtered snapshot
driven by `git ls-files -co --exclude-standard`** (working-tree file list → copy to a
disposable location): git-read-only, so it performs none of the writes the F18 ban could even
arguably cover (no `.git/worktrees` metadata write, F31 avoided); carries the uncommitted
work per D2/D3 (F57a/b); drops gitignored dirs, preserving the works-warm-only catch
(F14/F15, F57c); cheapest of the three measured mechanisms **at the measured 5.9M sample** —
F56's own bound rides: it copies real bytes and scales with working-tree size, so the
ordering can invert on a large tree; matches git's own ignore semantics where the rsync
variant fails two cases (F58.1/2). Native harness isolation is ruled out **by docs**: all
paths are commit-based/tracked-only, and `.worktreeinclude` copies exactly the gitignored set
— it would defeat the catch (F50–F53). The tracked-only variant (F59/F60) is available, not
canonical (D3(i)) — measured (0.023s) but in a different repo against a different question,
so outside F56's three-way comparison. The listing always **explicitly excludes the whole
`.claude/worktrees/` parent** (e.g. the `:!.claude/worktrees` pathspec on the `ls-files`
call) — covering the snapshot home and the D4(i) fallback replica alike, independent of the
target repo's ignore state (D6; B1 repair, round-2 notes 2–3). Caveats:
(i) the snapshot is **not a git repository** (F58.5). For a stack whose quality gates shell
out to git (version-from-describe, changed-files selectors), the lead composes the
**fallback, fully specified under the U1 ban ruling**: `git worktree add --detach` under
`.claude/worktrees/` — an ephemeral, self-removed verification worktree, **ruled not a
banned mutation (U1: the F18 ban protects refs, index, tracked content, and history)** —
plus an overlay of the D4 snapshot file-set over the worktree and removal of the
`git ls-files -d` deleted set, yielding a working-tree replica with a functional `.git`.
The composition is recorded on D5's carrier.
(ii) a snapshot-only gate failure is checked against the **dangling-symlink** class (F58.4)
and, per-cycle, against **carry-set gaps** (D7) before being read as a real cold bug — the
snapshot is retained until that disposition (D6, teardown).

**D5 — Per-cycle option trigger: lead judgment, guidance recorded.** `Confident` (user-ruled,
Q7 batch; triggers re-derived from F7 and the visibility carrier re-ruled at review —
S12/U7)
The interference model the facts leave standing (S12): doctrine permits no concurrent
writers (F7 — sequential-only, verification paired per round), so the open channels are
**alternation residue** — state one seat's run leaves for the other's next turn — and
**off-doctrine runs** (the class F48 evidences). The lead composes per-cycle qa isolation in
on those signals: GUI/subjective verification in the cycle set (residue-sensitive; the
carry-set question is live there, D7) · state-heavy stacks with generated-file churn ·
interference observed mid-run — this trigger fires *after* the suspect cycle, so
already-cleared cycles are backstopped by the always-cold final validation (D3(i)), not
re-verified · a prior recorded incident in this codebase. Never mechanically, never via an
added run-start approval (rejected as friction, consistent with D2; deterministic named
triggers rejected as brittle before a citable incident exists). **Visibility carrier (U7 —
implement is v6-form, P19/P20 out of its scope, F65/F66):** every verification report,
per-cycle and final, **names the tree that produced its evidence** — warm working tree, or
snapshot path + variant (filtered snapshot, tracked-only, or the D4(i) worktree-replica
fallback) + carry-set; at implement's v7 conversion the composed choice
additionally lands per P19/P20. This build touches implement, so the convert-on-touch
decision (F66) fires at build scoping — the user's call there.

**D6 — Composition details.** `Confident` (user-ruled at review, U3 — promoted from the
lead's `Assumed` formulation; teardown and location amended per S13/S14; citations repaired
per S16)
- **Report routing and reads:** qa always **writes** verification reports into the **real**
  tree's `.mochiko/specs/<feature>/…` by absolute path, never into the snapshot — and
  **reads** its inputs (`tasks.md`, design inputs, prior reports) from the real tree too.
  The snapshot's own copy of `.mochiko/specs/**` (carried per F57b) is dead weight: never a
  read or write surface.
- **Dependency posture:** the final-validation snapshot is always dependency-cold — install
  from lockfiles; the coldness **is** the catch (F14/F15, F32/F57c; F33/F54 price it, they
  do not justify it). Per-cycle dependencies follow D7's carry-set policy: **copy or fresh
  install, linking forbidden** — `npm ci` measurably wipes the dependency dir (F70), so a
  linked dir turns qa's cold install into destruction of the producer's real
  `node_modules`, the exact contamination this design exists to prevent.
- **Snapshot location + friction:** the snapshot lands under **`.claude/worktrees/<name>/`**
  — in-repo and **made ignored: a build obligation, not existing behavior (B1 repair)**.
  Implement's scaffolding step, which today only creates missing ignore files (F9), is
  extended at build to ensure the target repo's `/.claude/worktrees` ignore entry — F38
  establishes that line in *this* repo only, and F54 records it as a manual docs tip; and
  the D4 listing excludes the snapshot home explicitly besides, so a missing ignore line
  cannot recurse retained snapshots into the `-co` set. In-repo placement also sits inside
  the `acceptEdits` auto-accept path scope (F62). The qa seat runs gates **by absolute
  path** (plain shell, no `EnterWorktree`, no session-CWD move — F55 never fires). D2's
  no-added-approval posture holds on the documented permission model (F61/F62:
  command-pattern prompts, path-scoped auto-accept only under `acceptEdits` — both closed by
  the in-repo home); the honest residual is stack-specific command patterns a repo has not
  yet approved — and the cold path adds install patterns (`npm ci`) a warm run may never
  have invoked: a one-time approval per repo per pattern (F61), paid at the first cold run.
- **Teardown (S13):** the snapshot is torn down only after its evidence is captured **and**
  any snapshot-only failure is dispositioned — the F58.4 symlink check and the D7
  carry-set-gap check run against the live snapshot, before teardown; on the per-cycle path
  a failed cycle's snapshot is retained until the retry's disposition.

**D7 — Per-cycle carry-set policy.** `Confident` (user-ruled at review, U2)
A per-cycle snapshot's purpose is interference isolation, not coldness — so it MAY carry
warm gitignored items, but only as a **lead-declared carry-set**, named in the composition
record (D5's carrier): runtime config (`.env` — F53's class, without which a subjectively
verified app will likely not boot) and generated dirs a gate consumes (codegen output,
`dist/`). Dependencies enter by **copy or fresh install — linking forbidden** (F70: `npm ci`
wipes the dependency dir; a link would propagate that wipe into the real tree). Carrying
warm items defeats the cold catch **by design and by ruling** — acceptable per-cycle, never
at final validation, whose snapshot stays exactly D4's filtered set plus nothing.

**D8 — Independence reading.** `Confident` (user-ruled at review, U4)
Snapshot isolation is **ergonomics + evidence fidelity**, not part of the shape's
independence definition — F19's "who fills the seats and when they arrive… the whole of it"
stands untouched. No independence claim rides a snapshot-produced report, and a lead who
declines the option composes a fully independent verification regardless.

## Open threads & revisit triggers

1. **Shape-home clarification of the F18 ban reading** — U1 ruled the reading for this
   design; whether `command-shape.md`'s ground-rule line gains the clarifying sentence is
   decided at build scoping (a pure addition rides the decision row; any rewording is
   ceremony). **Dispositioned at build (2026-08-01):** user ruled yes — the sentence landed
   in Layer 1 Ground rules as a pure addition (audit-verified zero deletions), citing U1.
2. **Per-cycle cost is unmeasured** (F33 UNVERIFIED) — measured at the first composed-in run
   (U5); until then D3(ii)'s optionality is priced by judgment.
3. **No citable incident artifact exists** (F41–F47) — every future interference observation
   is recorded on the run (U5); trigger: the next observed interference.
4. **Non-npm dependency behavior** (F71 — UNVERIFIED beyond npm) — checked at the first
   composed-in run on a non-npm stack (kinako/Flutter is the live candidate).
5. **implement's v7 conversion** — this build touches implement, so convert-on-touch (F66)
   fires at build scoping; until conversion, U7's report-provenance line is the visibility
   carrier. **Dispositioned at build (2026-08-01):** user ruled **defer** to a dedicated
   wave — this wave stayed surgical v6-form; the deferral is recorded in the `[v0.42.0]`
   strip entry and the F66 trigger stays live.
6. **Layer 2 says nothing about seats sharing a working tree** (F19) while F48 documents
   three real authoring-side collisions — out of D1's scope; captured as a new BACKLOG item
   at close (wrap ruling).
7. **`.claude/worktrees/` is harness-managed** (F50/F54: the default worktree home; F52
   documents a periodic sweep) — whether the sweep touches non-worktree directories there is
   unestablished (verify-pass polish note 3). Fact-checked at build; if it does, the
   snapshot home moves to a sibling ignored directory — the D6 ruling carries the property
   (in-repo + ignored + explicitly excluded), not the literal path. **Dispositioned at
   build (2026-08-01):** F72–F75 — the sweep is worktree-registry-scoped; a plain snapshot
   directory is not a target (small version-sensitive residual, honestly marked). The real
   hazard is F76 name collision (+ F77: harness background sessions share the directory) —
   resolved by the **`mochiko-` name prefix** on snapshot dirs (`implement.md`; recorded in
   the `[v0.42.0]` strip entry). Home unchanged.

## Fact-checker map (verbatim)

> **Lead scope note (review fold S6):** the map below folds **four dispatches** — F1–F40
> (initial map: its "40 facts" header, its cutting-both-ways summary, and its UNVERIFIED
> tally are scoped to that dispatch alone) · F41–F49 (incident trace; the `#### verdict`
> block is *that dispatch's* verdict, the one D3(iii) cites) · F50–F58 (isolation
> mechanics) · F59–F71 (review-stage fact routes). Checker text is untouched; the dispatch
> headings and this note are the lead's only additions. Total: **71 facts**, sequence
> verified gapless (F1–F58 by the integrity reviewer, F59–F71 by the lead).

REALITY MAP — validator worktree isolation in implement. 40 facts, all path-cited; git semantics established empirically in a scratch repo (git 2.53.0) and on this repo, not from memory.

### implement today

**F1** — Two standing seats share one working tree: producer `staff-engineer` × `executing-tdd-cycle`, `brownfield-integration`; verifier `qa-engineer` × `testing-end-user`. Plus two disposable `principal-architect` seats (arch-diff at final validation, arch-scribe at finalize). (`plugins/mochiko/commands/implement.md:39-42`)

**F2** — Verification happens at three moments: per-cycle verification by qa (executes the cycle's `**TEST:**` tasks, runs the quality gates, captures evidence), the quality gates themselves inside that, and the whole-implementation final validation. (`implement.md:40`, `:22-23`)

**F3** — Verifier lifecycle: "cold at the first cycle verification, standing after"; the override recycles it per **slice** boundary, and its final-validation incarnation is "additionally briefed from the on-disk verification reports." (`implement.md:40`, `:49-52`)

**F4** — Cycle-checkpoint devolved branch, exact conditions: skipped "**exactly** when every verification in the cycle is a deterministic CLI check at 100% pass **and** no deviation is reported **and** `domain_deps_added` is empty: the cycle then clears on qa's PASS-with-evidence, unread by you, counted from its one-line clearance notice." (`implement.md:69-74`)

**F5** — Reports live under `.mochiko/specs/<feature>/` (or `slices/<slice>/` when slice-scoped): `cycle-report.md` and a verification report per cycle, the final-validation report, the built-vs-approved diff report. (`implement.md:124-126`)

**F6** — Fact route: "real infrastructure — executed `**TEST:**` tasks and quality-gate exit codes." (`implement.md:133`)

**F7** — **Sequential-only today.** "parallel cycle execution is a `deliberate-shortcut-ledger` deferral, not a capability drop"; every produced cycle is paired with a verification in the same round. One producer at a time — there are no concurrent writers in the tree. (`implement.md:101-104`)

**F8** — Under slice scope the quality gates still run the "**full repository suite**." (`implement.md:113-114`)

**F9** — A scaffolding step creates missing ignore files (`.gitignore` / `.dockerignore` / lint-ignore) once before the cycle loop — a filesystem write, no git command. (`implement.md:108-109`)

**F10** — On G5 reject, "the work remains under `.mochiko/specs/<feature>/` and in the working tree" — the warm working tree is the assumed home of the deliverable. (`implement.md:92-93`)

### cold-checkout ruling

**F11** — Shipped constraint, verbatim: "**Cold checkout:** the final validation builds and runs the quality gates from a **fresh clone** of the repository, never only the warm working tree; that clone's results are part of G5's evidence." (`implement.md:105-107`)

**F12** — "a warm-only final validation" is a named not-done state. (`implement.md:31`)

**F13** — Ruling, verbatim: "**User ruling — cold checkout lands in implement's final validation.** The final validation builds and runs the quality gates from a fresh clone, never only the warm working tree; the clone's results are part of G5's evidence, and a warm-only final validation is a named not-done state." (`.mochiko/decisions/2026-07-31-team-method-escalations-closed.md:35-38`)

**F14** — Motivating bug, verbatim from the kinako note: "`dart compile exe` doesn't create its `-o` parent, `build/` is gitignored, so the new CI step passed six cycles and independent qa verification — because every machine involved was *warm* — and failed only on a fresh checkout. **Team verification is warm-machine verification.** It does not substitute for a cold-checkout gate." (`.mochiko/brainstorms/team-method-vs-command-shape/inputs/kinako-mvp-h1-team-note.md:108-112`)

**F15** — kinako rule 7 states the bug class: "any step depending on a gitignored directory passes locally and fails only on a fresh clone." (same file, `:127-128`)

**F16** — Placement rationale: "cheapest deterministic catch for the works-warm-only bug class, placed once per run at final validation rather than per cycle." Audit-charter placement was rejected by user ruling — "audit is unscoped, ROADMAP Later" — with "the step migrates if audit takes feature-close." (ADR `:55-56`, `:39-40`)

**F17** — The ADR never says HOW the fresh clone is produced, never reconciles it with the git-mutation ban, and never addresses uncommitted producer state. Searched the ADR, the originating record, and `command-shape.md`: no reconciliation text exists anywhere. UNVERIFIED that any was ever written.

### the git-mutation ban and what the shape does/doesn't say

**F18** — Layer 1 Ground rules, complete text: "Stay kernel-free — no Python/MCP brain code, no capability catalogs, no DAG-mediated orchestration. A command suggests commits; it never runs git mutations and never pushes." (`plugins/mochiko/templates/command-shape.md:228-230`) This is the only git sentence in the shape.

**F19** — **Layer 2 has no text about seats sharing a filesystem, working tree, or CWD.** Grepping `command-shape.md` for filesystem / working tree / concurrent / contested / CWD / parallel returns only unrelated `Contested` confidence-mark hits. Mochiko's independence is context-scoped by definition: "Independence is carried by **who fills the seats and when they arrive** — disjoint agents, disjoint skills, no seat grading its own output… That is what **structural separation** means here, and it is the whole of it" (`:285-288`), and "**Cold arrival is a property of the stage, not of the traffic:** a seat is cold when it is not in the room before its own stage" (`:288-290`). Filesystem separation appears nowhere in the independence definition.

**F20** — Across all of `plugins/`, the only git verbs instructed anywhere are `git status` (in `testing-governance-injection/SKILL.md:80`, verifying stub cleanup) and the ban sentence itself. `grep -rno "\bgit [a-z-]*" plugins/` returns exactly three unique hits. No primitive instructs `git add/commit/checkout/clone/stash/diff/worktree`.

### commit-state at verification — the load-bearing finding

**F21** — **Nothing in the implement flow ever commits.** The only "commit" in `implement.md` is `:160`, "a suggested commit (`feat: implement <feature>`)" in the finalize row — i.e. after G5 acceptance. `executing-tdd-cycle` has zero commit instructions (its one hit is a domain example about a DB write). `testing-end-user` has zero. Every other command is the same shape: "a suggested commit" at finalize (`setup.md:179`, `plan.md:166`, `specify.md:107`, `slice.md:113`).

**F22** — Therefore at per-cycle verification time **and** at final-validation time, the producer's work is uncommitted in the working tree — modified tracked files plus new untracked files.

**F23** — Verified empirically: a worktree checks out a committed ref, so uncommitted and untracked producer state is invisible in it. Test repo with ` M src/a.txt`, `?? src/b.txt`, `?? .mochiko/` → `git worktree add ../wt HEAD --detach` → `../wt/src/a.txt` reads `committed` (edit absent), `src/b.txt` absent, `.mochiko/specs/` absent entirely.

**F24** — **The same is true of the already-ruled fresh clone.** `git clone . ../clone1` → `../clone1/src/a.txt` reads `committed`; new files absent. So the cold-checkout constraint as shipped, executed today against an uncommitted implementation, builds and gates a tree that does not contain the work under validation. This is a property of the ruled step as written — not a new risk that worktrees would introduce. Flagging as a **live gap in the shipped constraint**; disposition is yours.

**F25** — `.mochiko/specs/**` is untracked-and-not-ignored. This repo's `.gitignore` has no `.mochiko` entry except one stale brainstorm dir; `git check-ignore -v .mochiko/specs/foo/tasks.md` returns nothing. `git ls-files .mochiko | wc -l` = 240 tracked, of which zero under `.mochiko/specs/` (no specs dir exists here). No mochiko primitive instructs adding `.mochiko/` to a target repo's ignore files. Net: spec artifacts are untracked until someone commits them, hence invisible in a worktree or clone (F23/F24).

### git semantics (empirical, git 2.53.0)

**F26** — Worktree = checkout of a committed ref only (F23).

**F27** — Object DB is shared: the worktree's `.git` is a file containing `gitdir: <repo>/.git/worktrees/wt3`. Sizes in the toy repo: source `.git` 220K vs worktree 16K.

**F28** — Creation cost on this repo (16M `.git`, 376 tracked files): `git worktree add --detach` = **0.63s**; `git clone -q . <dst>` = **1.63s** (a local clone hardlinks objects by default). Toy repo: worktree add 0.010s, `git worktree remove --force` 0.003s, the `-b <branch>` form 0.007s. Clone from a *remote URL* is network-bound — UNVERIFIED, not measured.

**F29** — Branch-checkout exclusivity is real: `git worktree add ../wt2 main` → `fatal: 'main' is already used by worktree at <repo>`. A second worktree must use `--detach` or `-b <newbranch>`.

**F30** — `git worktree add` succeeds from a dirty tree; there is no clean-tree precondition (added successfully with ` M` and `??` entries present).

**F31** — Mechanical asymmetry against the F18 ban: `git worktree add` **writes into the source repo** — `.git/worktrees/<name>/` appears after the call. `git clone <src> <dst>` does not (source `.git/worktrees/` unchanged after cloning). Whether either counts as a "git mutation" under the ban is a reading, not a measurement; I report only what each command writes.

**F32** — **Gitignored build artifacts are absent in both.** With `build/` and `node_modules/` gitignored and populated warm, neither `../wt3/build` nor `../clone1/build` exists. A worktree of HEAD reproduces the cold-checkout property (the F14/F15 bug class) exactly as a clone does — the catch does not depend on clone-vs-worktree.

**F33** — Corollary: quality gates in either a worktree or a clone start with no `node_modules` and no build output, so a full dependency install plus build is the per-gate setup cost. Stack-dependent; UNVERIFIED as a number (not measured — this repo has no build).

### prior rulings and mentions

**F34** — **Zero prior mochiko ruling on worktrees.** The token `worktree` appears in `plugins/` **not at all**. In `.mochiko/` it appears only in: this session's index entry and record, the `vertical-graduation` synthesis, and the `architecture-design-primitive` record's scope line.

**F35** — The one prior worktree ruling is a rejection, scoped to pipeline parallelism: "Parallelism and stage-shepherding ergonomics were explicitly *not* drivers — so no worktree-parallel clusters in v1, and ergonomics alone can't justify a new orchestrator." (`.mochiko/brainstorms/vertical-graduation/synthesis.md:12`)

**F36** — Author-side precedent, never doctrine: a mochiko design session itself ran in a worktree — "Scope: worktree at `.../worktrees/brainstorm-architecture-design`" (`.mochiko/brainstorms/architecture-design-primitive/record.md:15`), whose `:83` also notes that worktree held primitives the installed plugin lacked.

**F37** — `DECISIONS.md:20` is the cold-checkout index row; `ROADMAP.md:40` carries "validator worktree isolation" in the ergonomics sweep line.

### harness surfaces

**F38** — This repo's `.gitignore` last line is `/.claude/worktrees`, and `.claude/worktrees/` exists on disk (currently empty). That is the harness's worktree location, already ignored; no mochiko doc explains or references it.

**F39** — Native isolation exists at the harness layer: the Agent tool exposes `isolation: "worktree"` ("gives the agent its own git worktree (auto-cleaned if unchanged)"), and `EnterWorktree` / `ExitWorktree` are available tools in this session.

**F40** — **No mochiko doctrine knows about any of it.** Grepping `.claude/`, `plugins/`, and `CLAUDE.md` for `isolation|EnterWorktree|ExitWorktree`: every hit is the unrelated phrase "in isolation" (feasibility review, unit tests) plus `command-shape.md`'s "cold isolation" (context, per F19). Zero mentions of Agent-tool worktree isolation, `EnterWorktree`, or `.claude/worktrees` in any primitive. `agent-dispatch.md`, the caller-side briefing home, has no isolation or worktree field — its ninth and newest field is peer-edge/hold (ADR 2026-07-31 §2).

### cutting-both-ways summary (facts only)

Against needing it: F7 (no concurrent writers today), F19 (independence is defined as context-scoped, filesystem separation is not part of it), F32 (a clone already delivers the cold property), F35 (the one prior worktree ruling was a rejection).
For it: F28/F31 (worktree is ~2.6× cheaper than a local clone and its metadata write is the only source-repo effect), F38/F39 (the harness already has the machinery and the ignore entry), F24 (the currently-ruled clone mechanism has an unaddressed commit-state gap that any worktree design would inherit and must answer).

Two UNVERIFIED items: remote-URL clone cost (F28) and per-gate install/build cost in a cold tree (F33). One flagged live gap in shipped doctrine: F24.

### incident trace (second dispatch)

Headline: **the artifact is UNVERIFIED — no validator/qa output on this machine recommends worktree isolation.** But the search turned up three things worth having: kinako's final-validation report explicitly confirms qa graded uncommitted working-tree state, this repo's own strip notes record three real working-tree contamination incidents, and HIL shipped-then-abandoned a `using-git-worktrees` skill.

#### where I looked

**F41** — `/Users/deepeshadmin/Documents/GitHub/` contains exactly two entries: `CLAUDE.md` and `mochiko`. No kinako, no mochiko-app, no other project dir. The lead's premise of sibling repos does not hold on this machine.

**F42** — Machine-wide `find` for `.mochiko/specs` trees returns four: three read-only clones of `humaninloop-dev/kinako` in prior sessions' scratchpads (`/tmp/claude-1000/-Users-deepeshadmin-Documents-GitHub-mochiko/{e8c5793a…,3323c2c4…,e02f3d3b…}/scratchpad/kinako`, `git remote -v` → `https://github.com/humaninloop-dev/kinako.git`), plus my own throwaway test repo from this run. The `e8c5793a` copy is the fullest: 12 S1 cycle-reports, 12 S1 verification reports, `final-validation.md`, and S2 reports.

#### what the kinako artifacts say

**F43** — **Zero `worktree` hits in any kinako artifact.** `grep -rniE "worktree|separate (working )?tree|clean tree|dirty tree|cross-contam"` over every `.md` under kinako's `.mochiko/specs/` returns nothing at all. Widening to `worktree|working tree|contaminat|isolat` over kinako's whole `.mochiko/` yields only domain-level hits — capture-path failure isolation (FR-014/TR-013), injection-path failure isolation (FR-049/TR-045), the Dart "UI isolate" (NFR-004) — plus two incidental mentions: `dogfood-runbook.md:69` "`git status` # reconcile the working tree first (S4 files are uncommitted)" and `constraints-and-decisions.md:668` (writing hook config into each repo's working tree). None is a verifier recommending isolation.

**F44** — The kinako final-validation report **does** independently corroborate the warm/uncommitted condition behind F22/F24. Verbatim, `…/slices/s1/verification-reports/final-validation.md:11`: "**Code under validation:** working-tree state (S1 cycle edits; C11/C12 uncommitted at read time)." Alongside `:6` "**Verifier:** standing QA seat (independent; ran reality, trusted no producer summary)" and `:8-10` "**Environment:** real on-disk store / spool / run-store / error journal (env-var overridden temp dirs), real `dart run` tool CLIs… No mocks." Its "Recommendation to the lead" (`:112-120`) is "**AUTO-APPROVE the CI-verifiable scope of S1**" plus a standing sandbox-gate note — **no mention of worktrees, tree separation, or contamination.** Corroboration for the commit-state facts; not the incident artifact.

#### transcripts

**F45** — **No implement run's transcripts exist on this machine.** One transcript root only, `/home/agent/.claude/projects/-Users-deepeshadmin-Documents-GitHub-mochiko/` (98 session `.jsonl` files). Enumerating every subagent seat name across all sessions yields only mochiko authoring/design/audit seats — `command-architect`, `validator`, `shape-architect`/`shape-auditor`, `fact-checker`, `decision-reviewer`, `doctrine-auditor`, `integrity-reviewer`, `wave*-architect`/`-auditor`, `sweep-*`, `author-implement`, `advocate`, `grounder`, `verify-conformance`/`-fidelity`/`-wiring`. **There is no `qa-engineer`, verifier, producer, or `staff-engineer` seat in any session.** (The `verify-*` trio in session `5e2c7929` verifies authoring conformance, not code.) This matches the standing note that the kinako dogfood's artifacts are available but its transcripts never were.

**F46** — The item's own phrasing — "cross-contamination", "cleaner separation", "producer's working tree", "run in a git worktree", "uncommitted producer" — appears in exactly two transcripts: this session (`31e8c385…`) and `39de1ea3-1e2b-4a4d-892d-97021e066aa1.jsonl` (2026-08-01). That second session's seat roster is `decision-reviewer`, `doctrine-auditor`, `fact-checker`, `integrity-reviewer`, `shape-architect-2`, `shape-auditor-2` — a shape/doctrine wave with **no qa seat**. Extracting every assistant/user text block containing "worktree" from it returned no text-content hit (matches sit in tool-result and system payloads), so no authored recommendation is recoverable there either.

**F47** — The BACKLOG item was captured in commit `28e43af` (2026-08-01, "Add lead-owned process-flexibility brainstorm") and is present at HEAD. Its provenance field reads "provenance: capture session, to-brainstorm" — it cites a session, not an artifact, and names no report path.

#### the contamination class IS on disk — authoring-side, three times

**F48** — This repo's strip notes record three real working-tree collisions between mochiko's own seats. All three were resolved by treating **HEAD as the trusted baseline** — the property a validator worktree would give for free.

- `.mochiko/strips/implement.md:355-362` — "## [v0.35.0] Collision note — an unledgered orphan draft occupied the working tree… While the wave was in flight a since-terminated seat, executing a superseded instruction, overwrote the working-tree `implement.md` at ~23:26 with a **different** goal-shaped draft (1,934 w / 13,919 B, never committed, no strip entry)." Recovery: "**this rewrite and its ledger derive from HEAD**, not from the working tree… Every row of the CS-D8 ledger above was re-derived against `git show 7898d86:…`."
- `.mochiko/strips/specify.md:30-36` — "**Baseline provenance — read this before auditing the ledger.** The working tree held a partial rewrite by a since-stopped seat executing a superseded instruction. **This ledger is derived from `HEAD` (the authoritative 146-line baseline), not from that draft**." It cost one genuine fidelity gap that the audit had to restore.
- `.mochiko/strips/setup.md:128-133` — "**Build note — concurrent draft.** A goal-shaped draft of this file by another agent was found in the working tree mid-task (2,393 w; ceiling FAILs on preamble 147/130, Goal 156/150, Seats 391/235, Constraints 990/810…). It was preserved, not overwritten blind."

Caveat carried honestly: all three are **authoring-wave** collisions in mochiko's meta-work, not implement-run producer↔validator contamination. They evidence the class, not the driving incident.

#### prior art

**F49** — HIL shipped a `using-git-worktrees` skill; its description (recovered from a transcript's skill listing) read: "This skill MUST be invoked when the user says 'create worktree', 'isolated workspace', 'parallel branch work', 'git worktree', 'feature isolation', or 'branch workspace'. SHOULD also invoke when starting feature work that needs isolation from current workspace." mochiko's retired registry files it as a **cross-cutting utility** (`.mochiko/archive/REGISTRY.md:99`) and records it **abandoned** — "`using-git-worktrees` · `using-github-issues` | **abandoned** (no consumer demand recorded; re-open on demand)" (`.mochiko/archive/REGISTRY.md:16`). Never ported, never validator-scoped. This strengthens F34: the token is absent from `plugins/` because the one worktree primitive that ever existed was dropped for lack of demand.

### isolation mechanics — uncommitted state (third dispatch, F50–F58)

Headline against D2: **native harness worktree isolation cannot reproduce uncommitted working state — established from the official docs, not inferred.** A gitignore-filtered snapshot can, at 0.16s on this repo, with four measured sharp edges. Scratch copies deleted; mochiko repo untouched (`git status` shows only your record edits; `git worktree list` shows main only).

#### (1) native worktree vs dirty state — VERIFIED BY DOCS

**F50** — The page exists: "Run parallel sessions with worktrees", https://code.claude.com/docs/en/worktrees. Decisive, verbatim: "Claude Code's defaults for creating worktrees cover most sessions: it creates them under `.claude/worktrees/`, branches them from your repository's default branch, and **checks out only tracked files.**" And: "**A worktree is a fresh checkout, so untracked files like `.env` or `.env.local` from your main repository are not present.**"

**F51** — Both `worktree.baseRef` settings are commit-based; neither carries working-tree state. Verbatim: "`"fresh"` (default): branch from the repository's default branch **on the remote**, usually `main`, so the worktree starts from a clean tree matching the remote." / "`"head"`: branch from your current local `HEAD`, so the worktree carries your **unpushed commits** and feature-branch state. Use this when isolating subagents that need to operate on in-progress work." Note the default is the *remote* default branch — further from the producer than even local HEAD. `"head"` is the closest available and it is still commits, not edits.

**F52** — The native subagent mechanism is the documented `isolation: worktree` frontmatter field: "Subagents can run in their own worktrees so parallel edits don't conflict. Ask Claude to 'use worktrees for your agents', or make the isolation permanent for a custom subagent by adding `isolation: worktree` to its frontmatter." Lifecycle: "Each subagent gets a temporary worktree that Claude Code removes automatically when the subagent finishes without changes; a worktree with changes stays on disk until the periodic sweep… can remove it without losing work." Base branch: "**Subagent worktrees use the same base branch as `--worktree`**, so they branch from your repository's default branch unless `worktree.baseRef` is set to `"head"`." So the exact surface mochiko would reach for inherits F51 unchanged.

**F53** — The one documented file-carrying escape hatch, `.worktreeinclude`, **cannot** carry the producer's work and would actively defeat cold checkout. Verbatim: "To copy them automatically when Claude creates a worktree, add a `.worktreeinclude` file to your project root. The file uses `.gitignore` syntax. **Only files that match a pattern and are also gitignored are copied**, so tracked files are never duplicated." Two consequences: (a) it cannot carry a modified *tracked* file nor an untracked-but-not-ignored new file — precisely the two things F22 says the producer leaves at hand-off; (b) what it *can* carry is exactly the gitignored set (`build/`, `node_modules/`, `.env`) — the warm artifacts whose absence is the whole catch behind F14/F15/F32. Using it to make a validator worktree runnable would reintroduce the bug class cold checkout exists to detect.

**F54** — Other documented mechanics bearing on the design. Worktrees share `.git`: "git commands in a worktree write to the main repository's shared `.git` directory, and sandboxing allows those writes, so commands such as `git commit` work from inside a worktree with the sandbox enabled." Project-scope plugins load in worktrees without reinstall (v2.1.200+); permission approvals granted in a worktree save to the main checkout (v2.1.211+). Default path `.claude/worktrees/<name>/` on branch `worktree-<name>` — and the docs' own tip, "Add `.claude/worktrees/` to your `.gitignore`", is verbatim the origin of F38's line in this repo. Dependency cost is explicitly the caller's: "A worktree is a fresh checkout, so initialize your development environment there: ask Claude to install dependencies, or run your project's setup yourself in the worktree directory" — documentary confirmation of F33.

**F55** — Placement friction, relevant to D2's no-added-approval constraint: "When Claude enters a path outside the repository's `.claude/worktrees/` directory, Claude Code asks for your approval first, because the move takes the session's working directory, write access, and project configuration such as `CLAUDE.md` and settings to that location. An `EnterWorktree` permission rule or choosing 'don't ask again' doesn't suppress this prompt; only `bypassPermissions` mode skips it." Also: while an agent runs, "Claude runs `git worktree lock` on its worktree so that concurrent cleanup cannot remove it."

**Net: no empirical corroboration needed — the docs answer it outright.** Every native path (default `--worktree`, `isolation: worktree`, both `baseRef` values, `.worktreeinclude`) is commit-based or gitignored-only. None reproduces uncommitted tracked edits plus untracked non-ignored files.

#### (2) filtered-snapshot cost — the mechanism that CAN carry dirty state

**F56** — Measured on this repo (5.9M working tree excluding `.git`; 376 tracked files), copied to a scratch dir outside the repo and then deleted:
- `rsync -a --filter=':- .gitignore' --exclude '.git/' ./ <dst>` — **0.161s**, 378 files.
- `git ls-files -co --exclude-standard -z | tar --null -cf - -T - | tar -xf - -C <dst>` — **0.163s**, 378 files.
Both equal `git ls-files -co --exclude-standard | wc -l` = 378 exactly. Against F28's numbers on the same repo (worktree add 0.63s, local clone 1.63s), the snapshot is the cheapest of the three — but it copies real bytes, so it scales with working-tree size rather than with object-DB size, and 5.9M is a small sample.

**F57** — Content semantics, verified in the toy repo (the mochiko repo cannot demonstrate this, having no dirty tracked file):
- **(a) dirty tracked edit — present with uncommitted content.** `src/a.txt` in the snapshot reads `committed` / `UNCOMMITTED EDIT`, i.e. the working-tree version, not HEAD's.
- **(b) untracked non-ignored files — present.** Both `src/b.txt` and `.mochiko/specs/feat/cycle-report.md` copied.
- **(c) gitignored dirs — absent.** `build/` and `node_modules/` do not exist in the snapshot; on the mochiko repo, `.claude/worktrees/` and the ignored `.mochiko/brainstorms/setup-workflow-rewrite/` are likewise absent.

So the filtered snapshot holds a property neither a worktree nor a clone has: it carries uncommitted producer state **and** still drops the warm gitignored artifacts — satisfying D2/F22 and preserving the F14/F15 cold-checkout catch at the same time.

**F58 — four measured sharp edges, plus one structural:**
1. **Negation patterns break under rsync.** With `.gitignore` holding `*.log` and `!keep.log`: the git-based method copies `keep.log`; **rsync does not.** rsync's merge-file syntax reads a leading `!` as "clear the filter list", not "negate". A repo using `!` re-inclusions silently loses those files under the rsync method.
2. **`.git/info/exclude` diverges.** A file listed there was **copied by rsync** (a leak) and **correctly excluded by `git ls-files --exclude-standard`**. rsync reads only `.gitignore` files; git reads the full standard exclude set (`.gitignore` + `.git/info/exclude` + `core.excludesFile`).
3. **Nested `.gitignore` is safe in both.** A `sub/.gitignore` ignoring `sub/secret.txt` excluded it under both methods while `sub/ok.txt` came through — rsync's `:- .gitignore` is a genuine per-directory merge. Not a hazard.
4. **Symlinks preserved as links by both** (`rsync -a` does not follow; tar stores the link). A link pointing outside the tree therefore lands **dangling** in the snapshot — a gate reading through it fails in the snapshot but passes warm, which manufactures false positives distinct from the real cold-checkout bug class.
5. **Structural: the snapshot is not a git repository** (`.git` excluded by both methods). Any quality gate that shells out to git — version-from-describe, a lint reading tracked-file lists, a changed-files test selector — fails there. Including `.git` would make it one, but then the copy is a clone/dirty-files hybrid whose HEAD and index describe a different tree than its files.

On edges 1 and 2 the two methods diverge in the same direction: the `git ls-files` variant matches git's own ignore semantics because it asks git; the rsync filter reimplements them and gets two cases wrong, at identical wall time.

#### verdict

**UNVERIFIED — the incident artifact is not on this machine.** Exhausted: all of `/Users/deepeshadmin/Documents/GitHub/`, every `.mochiko/specs/**` tree on the filesystem, this repo's entire `.mochiko/`, and all 98 session transcripts plus their subagent transcripts. If a qa seat did recommend worktree isolation, it happened in a session whose transcript is not retained here — most plausibly the kinako dogfood run, whose artifacts are present but whose transcripts are not (F45). Two consequences for the session's evidentiary posture, stated as fact and not as advice: the driver currently rests on the user's recollection rather than a citable artifact, and the nearest on-disk support is F44 (qa demonstrably grading uncommitted working-tree state) plus F48 (three authoring-side collisions of the same class).

### review-stage fact routes (fourth dispatch, F59–F71)

Four headlines: (1) yes, a tracked-only snapshot variant exists and reproduces a commit-of-tracked-changes exactly, with one hard-failure edge; (2) Bash permissions are matched on the **command string, not the path**, so out-of-repo-ness is not itself what prompts — but my probe's clean run is weak evidence and I say why; (3) **implement is v6-form and P19/P20 do not reach it** — no declaration or trail carrier binds today; (4) dependency dirs are **not** read-mostly — `npm ci` wipes the tree, and I measured it. All probes deleted; scratchpad empty; mochiko repo shows only your record edits and no stray worktrees.

#### Route 1 — tracked-only snapshot variant

**F59 — yes, and it is exactly the commit-equivalent tree.** `git ls-files -z | tar --null -cf - -T - | tar -xf - -C <dst>` (no `-o`) copies tracked paths with **working-tree content**. Measured in the toy repo: **0.023s**. Verified: **(a)** the dirty tracked edit is present with its uncommitted bytes — `src/a.txt` reads `committed` / `UNCOMMITTED EDIT`, not HEAD's version; **(b)** untracked files excluded — `src/b.txt`, the untracked `.mochiko/` tree, and untracked `keep.log` all absent. That is precisely what a hypothetical `git add -u && git commit` would capture. `--exclude-standard` is indeed a no-op without `-o`, since ignore rules only ever filter untracked candidates.

**F60 — one sharp edge, and it fails loudly rather than silently: a tracked file deleted from the working tree breaks the pipe.** `git ls-files` still lists a tracked-but-deleted path (after `rm src/a.txt`, `git ls-files | grep -c src/a.txt` = 1), and tar then errors: `tar: src/a.txt: Cannot stat: No such file or directory` / `tar: Exiting with failure status due to previous errors`. Any cycle that deletes a tracked file — a refactor moving a module — trips this. (`git ls-files -d` enumerates the deleted set; stated as mechanism, not as a recommendation.)

How it differs from the D4 snapshot (F56/F57): the `-co` form carries **untracked new files**, which is most of what a TDD cycle *creates* — new test files, new modules, and the `.mochiko/specs/**` reports themselves (F25). The tracked-only form drops all of those. The two variants answer different questions: "what would a commit of tracked changes contain" versus "what does the producer's tree actually hold".

#### Route 2 — build/install at an out-of-repo path

**F61 — docs: Bash permissions match the command string, not the paths touched.** The permission-system table: "Bash commands | Shell execution | **Yes, except a built-in set of read-only commands** | Permanently per repository and command." Rule syntax is command-pattern only — `Bash(npm run build)`, `Bash(npm *)`, `Bash(* install)`, "A single `*` matches any sequence of characters including spaces." **No Bash rule form takes a path argument.** The working-directory section scopes *file access*, not Bash: "By default, Claude has access to files in the directory where you launched it… Files in additional directories follow the same permission rules as the original working directory: they become readable without prompts, and file editing permissions follow the current permission mode." So in `default` mode the prompt fires on first use of a **command pattern**, not because a path is outside the repo.

**F62 — the one path-scoped exception is `acceptEdits`.** Verbatim: "`acceptEdits` | Automatically accepts file edits and common filesystem commands such as `mkdir`, `touch`, `mv`, and `cp` **for paths in the working directory or `additionalDirectories`**." So under acceptEdits a `mkdir` at an out-of-repo path falls outside the auto-accept and returns to prompting — the one place the out-of-repo-ness itself matters. Related: `bypassPermissions` "Skips permission prompts" except explicit `ask` rules and the `rm -rf /` / `rm -rf ~` circuit breaker; `dontAsk` "Auto-denies tools unless pre-approved". `Cd` rules are irrelevant to a seat — "**`Cd` is not a model-invocable tool: Claude can't call it**, and the rules apply only when you run `/cd` yourself." Sandboxing is a separate layer: "Sandbox restrictions prevent Bash commands from reaching resources outside defined boundaries, even if a prompt injection bypasses Claude's decision-making."

**F63 — observed: `npm init -y && npm install left-pad` ran at an out-of-repo absolute path with no permission prompt**, exit 0, `node_modules/left-pad` created, with a real registry fetch ("added 1 package, and audited 2 packages in 546ms"). Earlier `rsync`/`tar` writes to out-of-repo scratch paths likewise never prompted. **Caveat, stated because it materially weakens the probe:** I cannot read my own session's permission mode, and this sandbox grants broad Bash latitude, so a clean run here is **not** evidence about a `default`-mode user session. The load-bearing part is F61 from the docs, not my probe. The compile half is **UNVERIFIED** — no `cc`/`gcc` in this image — though it rides the same Bash surface.

#### Route 3 — shape v7 carriers and implement's form

**F64 — P19/P20 exist and are precisely the departure-visibility carriers.** Slots: "**P19** run-start declaration home + counted unit · **P20** departure-trail home" (`plugins/mochiko/templates/command-shape.md:152-153`). The governing principle: "The pipeline is a **default, never an obligation**… **Departure is by record, never by silence:** one trail line per departure (P20), against a run-start declaration (P19) the user rules. Outside the floor (Layer 1) nothing obliges the lead." (`:22-27`). Homes: "P19 — the declaration's home: one line on the named deliverable for a default run… P20 — the departure trail's home on this workflow's artifact" (`:128-130`).

**F65 — implement is v6-form, so P19/P20 do not bind it today.** The marker rule: "**Form is declared, never inferred.** A command authored or converted at v7 carries the literal marker `<!-- shape-form: v7 -->` in its preamble; **a file without it is v6-form** and is graded on the v6 slot set… The marker is the audit's branch key, and it retires when the last command converts." (`:45-48`). I grepped every file in `plugins/mochiko/commands/` and `plugins/mochiko/templates/`: the string `shape-form` occurs **exactly once in the whole plugin**, at `command-shape.md:46` — the rule defining it. **No command carries the marker; `implement.md` has none.** All six commands are v6-form. And the slot text is explicit: "**P18–P20 are v7-form slots** — a v6-form file (no marker) neither binds nor states them." (`:153-154`)

**F66 — the interim note, verbatim, is what governs:**
> "**Interim note (v7 — `Contested`, ruled over a pilot-first recommendation).** Shape v7 lands now and the six commands convert **when next touched or needed** (`lead-owned-process-flexibility` D4). Until a command carries the v7 marker it is **v6-form and fully conformant**, and three v7 clauses do not reach it: its gate lines and bounds stand exactly as written — **read as this command's obligations, not yet as departable defaults** · its Goal may name the checks and clearances the v7 spec calls process residue · and P18–P20 are out of its scope rather than silently absent from it. It is graded on the v6 slot set (P1–P17). The library runs mixed-form in the interim; the audit branches on the marker, never on the auditor's judgment, and each converted command's first live run is its own checkpoint." (`:156-164`)

Two facts follow directly, stated without design intent. First: on the note's own terms a v6-form implement has **nothing to depart from** — its gate lines and bounds are "obligations, not yet… departable defaults". Second: **no run-start-declaration or departure-trail carrier is available to it**, P19/P20 being "out of its scope rather than silently absent". The note's own conversion trigger is "when next touched or needed". I found no other v6 surface designated for run-level declarations — **UNVERIFIED** that one exists.

**F67** — `workflow-contract.md` is the shape's per-run carrier, listed in Pairs-with as "conditional — the per-run carrier a **departing** run instantiates" (`:417`), with `templates/workflow-contract.md:20` "Delete the HTML comments before committing the filled copy alongside the workflow." Whether a v6-form command may instantiate it is not addressed anywhere I could find: the departure concept that triggers it is a v7 clause, and F66 holds that a v6-form run has no departable default. **UNVERIFIED** that the shape rules this case explicitly.

#### Route 4 — are dependency dirs read-mostly?

**F68 — the documented heavy writes are install-lifecycle-scoped, not gate-time.** npm docs: "For commands like `npm ci` and `npm install`, the `preinstall` script runs before dependencies are fetched or unpacked… Scripts like `install` and `postinstall` are used for setup that depends on installed packages, as `preinstall` runs before packages are available in `node_modules`" (docs.npmjs.com/cli/v12/using-npm/scripts). `prepare` is likewise a lifecycle script (docs.npmjs.com/cli/v11/using-npm/scripts). None of these fire on a later `npm test` / `npm run build`.

**F69 — `node_modules/.bin` is read at gate time, not written.** "npm adds `node_modules/.bin` to the PATH" for scripts (docs.npmjs.com/cli/v11/commands/npm-run): a gate invoking a local binary reads the shim; shims are created at install.

**F70 — but "read-mostly" fails three ways, two of them measured:**
1. **`npm ci` is destructive to the entire tree — measured decisively.** I planted `node_modules/.sentinel` and `node_modules/.cache/x`, ran `npm ci`, and **both were removed**. npm's framing: `npm ci` "is used to install dependencies based on a `package-lock.json`… It ensures that the exact versions of dependencies specified in the lock file are installed" and is "recommended… in CI environments" (docs.npmjs.com/cli/v11/commands/npm-ci). The very command a cold gate would run **wipes** the dependency dir rather than reading it.
2. **A no-op `npm install` still writes into `node_modules` — measured.** Re-running `npm install` with dependencies already satisfied left `package-lock.json` byte-identical (same md5) but **changed the mtime fingerprint of `node_modules`** (`find node_modules -type f -printf '%T@ %p\n' | md5sum` differed before and after). "Install once, then read" is not idempotent at the filesystem level.
3. **The lockfile is conditionally rewritten by `install`, never by `ci`.** "When `npm install` is run without arguments, it compares `package.json` and `package-lock.json`. If the lockfile's versions satisfy `package.json` ranges, the exact versions from the lockfile are used… If they conflict, `package.json` ranges are used, **and `package-lock.json` is updated**" (docs.npmjs.com/cli/install). `package-lock.json` is normally a **tracked** file, so that write lands inside the producer's tracked working set — the same set F57(a) and F59 carry.

Counter-observation worth having: a plain read-only gate did **not** touch the tree. `node -e "require('left-pad')"` left the `node_modules` mtime fingerprint byte-identical. So the read-mostly intuition holds for *execution*; it breaks on the *install/refresh* commands a cold environment must run first.

**F71 — stack variance and probe limits, flagged.** All of F68–F70 is npm-specific. I established nothing equivalent for pip/site-packages, Dart's pub cache, or pnpm's content-addressable store, and their layouts differ in ways that matter (pnpm's store is global with symlinks into `node_modules`; Dart's pub cache is user-global with a project-local `.dart_tool/`; pip typically installs into a venv outside the repo or gitignored inside it). **UNVERIFIED for every non-npm stack** — and kinako, the only dogfood datapoint, is Flutter/Dart. Also honest: I *created* `node_modules/.cache/x` by hand to test destruction; I did **not** observe a real bundler writing there, so "build tools write into `node_modules/.cache`" is **UNVERIFIED** by my probe despite being common convention. One cross-stack generality I can state from measured repo facts rather than vendor docs: whatever the layout, the dependency dir is normally **gitignored** (F32 measured `node_modules/` and `build/` absent from both worktree and clone), so it is reconstructed rather than carried by any of the four isolation mechanisms mapped so far — which is exactly why F70(1)'s wipe-and-reinstall is on the critical path for every one of them.

### post-acceptance build-scoping dispatch (F72–F77 — Open thread 7)

Headline: **the sweep is worktree-registry-scoped, not directory-scoped — a plain snapshot directory under `.claude/worktrees/` is not a sweep target.** That conclusion rests on the mechanism plus one near-miss doc sentence, not on an explicit statement, so I mark the enumeration mechanism UNVERIFIED and give you the discriminator the docs imply. A *different* collision hazard at that path is real and measured (F76). Scratch probes deleted; scratchpad empty.

**F72 — the sweep's documented scope is worktree-and-provenance-scoped, never directory-scoped.** Verbatim (worktrees page, "Clean up subagent and background-session worktrees"): "A periodic sweep removes **worktrees that Claude created for subagents and background sessions** once they are older than your `cleanupPeriodDays` setting. The sweep skips a worktree that still holds work: changed or untracked files, or unpushed commits. It never removes worktrees you create with `--worktree`." The settings page ties the cadence and names the target class: "The same age cutoff applies to automatic removal of **orphaned worktrees** at startup" (`cleanupPeriodDays`, default 30 days, minimum 1).

**F73 — every documented sweep predicate and remedy is a git-worktree-registry operation, not a filesystem one.** The skip predicate is git state: "changed or untracked files, or **unpushed commits**" — and unpushed-commits is only meaningful for a registered worktree on a branch. The protection mechanism is a git command: "While an agent is running, Claude runs **`git worktree lock`** on its worktree so that concurrent cleanup cannot remove it." The manual remedy is a git command: "To clean up a worktree that the sweep keeps, run **`git worktree remove`**, adding `--force` if the worktree has uncommitted changes or untracked files." And the enumeration instruction the docs give users is literally the registry command (agent-view): "List leftover entries with **`git worktree list`** in the project directory and remove each with `git worktree remove <path>`."

**F74 — the closest thing to a direct statement, and it points the safe way** (agent-view, marked min-version 2.1.211): "**A worktree directory that git no longer recognizes is left on disk when its session is deleted, so remove leftover directories you don't need by hand.**" Git recognition is what determines whether Claude Code removes a directory; where recognition is absent the directory is *left*, and cleanup falls to the user by hand. Precision caveat, stated because it matters: that sentence governs **session deletion**, not the periodic sweep, so it is strong adjacent evidence rather than a statement about the sweep itself.

**F75 — verdict: UNVERIFIED as an explicit statement; git-worktree registration is unambiguously the discriminator the docs imply.** No sentence on either page says the sweep enumerates via `git worktree list`, and none says it scans `.claude/worktrees/` as a filesystem tree. But nothing in the documented sweep has anything to act on for an unregistered directory: it has no branch, no commits, no lock, and no registry entry. Confirmed empirically in a scratch repo (git 2.53.0): a plain directory at `.claude/worktrees/snap/` does **not** appear in `git worktree list` (only the main checkout is listed), and `git worktree prune -v` leaves its contents intact (`file.txt` still present afterwards). On the docs plus that behavior a plain snapshot directory is not a sweep target — but since this rests on inference from the mechanism rather than a doc sentence, treat the residual risk as small-but-not-zero and version-sensitive: the sweep's implementation is unspecified and could change without the docs changing.

**F76 — a different collision hazard at that path IS real, and it cuts the other way.** Measured, git 2.53.0: `git worktree add --detach .claude/worktrees/snap HEAD` onto a **non-empty** existing directory **fails** — `fatal: '.claude/worktrees/snap' already exists`. Onto an **empty** existing directory it **succeeds**. So a snapshot occupying `.claude/worktrees/<name>/` would block a later worktree creation at that same name. Compounding it, the docs' name-reuse rule keys on directory existence and does not distinguish a registered worktree from a plain directory: "**Passing `--worktree` a name whose directory already exists opens that existing worktree instead of creating a new one.**" That rule is documented for the user-facing `--worktree` flag; whether subagent-isolation naming can collide the same way is **UNVERIFIED**.

**F77 — two adjacent settings surfaced that the earlier map lacked.** `worktree.bgIsolation` set to `"none"` disables worktree isolation for background sessions: "Background sessions then edit your working copy directly without moving into a worktree first." And background sessions isolate into the same directory the design is considering: "Before editing files, Claude moves the session into an isolated git worktree under `.claude/worktrees/`, so parallel sessions can read the same checkout but each writes to its own." Its documented skip list contains one entry bearing directly on this design — Claude skips the worktree when "**The write is outside the working directory**" — alongside skipping when the session is already inside a linked worktree, and when the directory isn't a git repository with no `WorktreeCreate` hook configured.

Net for build scoping: the sweep is not the threat to a snapshot parked at `.claude/worktrees/<name>/`; **name collision with git's own refusal to create a worktree over a non-empty directory is** (F76).

## Review

**Sizing ruling:** pair (user, at convergence, over the lead's weight statement above).
Lens split: decision-quality · record-integrity. Record frozen from reviewer spawn until
every disposition landed (this section excepted).

**Tallies:** decision-quality 13 raised → 12 survived cross-exam (2 Critical · 9 Important ·
1 Minor) · record-integrity 13 raised → 12 survived (1 Critical · 7 Important · 4 Minor) ·
cross-exam closed at four messages, no unresolved counterpart objections on either side ·
**lead cross-set merge: 24 → 19 distinct survivors** (S1–S19: 3 Critical · 12 Important ·
4 Minor; 5 cross-lens duplicates merged). Both lenses recommended **needs-revision**, each
stating its own escalation conditions (restated per B2): decision-quality's
flip-to-critical-gaps condition — the F18 ban reading left unruled — closed in-session by
U1; record-integrity's two caveats — I2 meeting the critical-gaps letter ("an unowned
decision"), dissolved by U3's promotion at S4 · the C1+C2 merge possibly warranting a
Critical neither half carried alone, honored by rating S1 Critical. Fact disputes: none;
four fact **routes** opened instead (answered as F59–F71).

**User ruling batch (all adopted as recommended, 2026-08-01):**
- **U1** — F18 ban reading: the ban protects refs, index, tracked content, and history; an
  ephemeral, self-removed verification worktree (`git worktree add --detach` + `remove`) is
  **not** a banned mutation (→ D4(i)).
- **U2** — per-cycle carry-set policy (→ D7).
- **U3** — D6 promoted lead-`Assumed` → user-ruled; reads pinned to the real tree; snapshot
  home `.claude/worktrees/<name>/` (→ D6).
- **U4** — independence reading: ergonomics + evidence fidelity, shape untouched (→ D8).
- **U5** — falsifiability: interference observations always recorded; F33 cost measured at
  first composed-in run; the BACKLOG item closes to an open watch, never silently (→ D3(ii),
  Open threads 2–3).
- **U6** — the null option killed on F24 alone (→ D0).
- **U7** — visibility carrier: every verification report names its evidence tree now;
  P19/P20 take over at implement's v7 conversion; convert-on-touch fires at build scoping
  (→ D5, Open thread 5).

**Survivor dispositions (19/19):**
- **S1** (int-C1 + dec-C2, **Critical**) — supersession under-executed: carriers
  unenumerated, "substance preserved" asserted → **resolved**: D3(i) rewritten — three
  carriers with one disposition each, the disk-vs-repo delta enumerated, the F59/F60
  variant recorded, `Consumers assessed` sourced.
- **S2** (dec-C1 + int-I5, **Critical**) — F18 reading unruled, fallback mechanism unnamed,
  deferral unmarked inside a `Confident` decision → **user-ruled (U1)**; D4(i) rewritten
  with the fully specified fallback.
- **S3** (dec-C3, **Critical**) — no carry-set policy; purpose≠mechanism; deps clause
  pointed the wrong way → **user-ruled (U2)** → D7; the gate-flip risk absorbed into
  D4(ii) + D6 teardown.
- **S4** (int-I2 + dec-I7) — D6 unowned / mis-credited marks → **resolved**: U3 promotion,
  header mark present.
- **S5** (int-I3) — Q4–Q7 stems missing; "(a)" untraceable → **resolved**: questioning-trail
  block added to the header.
- **S6** (int-I4 + dec-M1) — map header/scope/verdict-anchor confusion, both lenses'
  substrate at stake → **resolved**: lead scope note over the map; D3(iii) citation widened
  to F41–F49.
- **S7** (int-I6) — no open-threads register → **resolved**: section added (six items at
  fold; a seventh added at verify polish 3), a trigger each.
- **S8** (int-I7 + dec-I4) — the v7 visibility carrier load-bearing and unmapped →
  **fact-routed (F64–F67) + user-ruled (U7)** → D5.
- **S9** (int-I8) — D1 stated the `Assumed` incident as established → **resolved**: clause
  qualified ("filed capture", F47), cross-referenced to D3(iii).
- **S10** (dec-I1) — the null option never argued, the against-column unengaged →
  **user-ruled (U6)** → D0.
- **S11** (dec-I2) — F19's closed independence definition never ruled against →
  **user-ruled (U4)** → D8.
- **S12** (dec-I3) — F7 never folded back → **resolved**: D5 carries the interference model
  and re-derived triggers; D1 carries the parallel-cycles revisit trigger; the
  final-validation backstop named.
- **S13** (dec-I5) — teardown destroyed the F58.4 check surface → **resolved**: D6 binds
  teardown to disposition.
- **S14** (dec-I6) — "holds mechanically" upgraded confidence across an inference →
  **fact-routed (F61–F63) + resolved**: the U3 location ruling closes the `acceptEdits`
  path caveat; the residual stated honestly in D6.
- **S15** (dec-I9) — the option unpriced and unfalsifiable → **user-ruled (U5)** → D3(ii) +
  Open threads 2–3.
- **S16** (int-M1) — deps bullet cited cost facts as the catch → **resolved**: D6 cites
  F14/F15/F57c for the catch, F33/F54 for the price.
- **S17** (int-M2) — "cheapest" unbounded → **resolved**: F56's scale bound carried in D4.
- **S18** (int-M3) — resume line unactionable → **resolved**: Status line rewritten to the
  live state.
- **S19** (int-M4) — the snapshot's read path unruled → **user-ruled (U3)** → D6's reads
  clause.

Withdrawn during cross-exam (retrievable from the reviewers' reports): int-I1 entire ·
dec-I8 entire · five single legs across both lenses.

**Verify pass (owner: reviewer-integrity):** round 1 — **NOT CLEAN**: 19/19 folds confirmed
landed with evidence quoted; 2 blockers introduced by the folds themselves, both repaired
same round — **B1**: D6's snapshot-home safety cited F9/F38 for a property neither
establishes in a target repo → repaired as an explicit **build obligation** on implement's
scaffolding step (ensure the `/.claude/worktrees` ignore entry) plus an explicit
snapshot-home exclusion in D4's listing; **B2**: the tallies paragraph misstated the
reviewers' escalation conditions as one → restated per lens above. All 7 non-blocking
polish notes taken (D1 driver-scope cite · D3 header S15 cite · D3(i) ADR line range · D4
F59-comparison note · D5 fallback named in the carrier · D6 residual qualifier · Open
thread 7, harness-managed home). Round 2 — **CLEAN**: both blockers confirmed closed (B1 on
both legs — the build obligation and the mechanical exclusion; B2 faithful per lens), all
seven polish landings verified, no new defect. Three round-2 cosmetic notes, all taken (S7
count updated · the D4 exclusion given a concrete form · the exclusion widened to the
`.claude/worktrees/` parent, covering the fallback replica).
