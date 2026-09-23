# Hook enforcement field review — decision record

**Status:** **accepted 2026-09-23** (user: "accept") — review round 1 folded (16/16 survivors
dispositioned, user-ruled: C1–C3 one by one, C2 reversed once and re-ruled `Contested`; I/M batch
as recommended); verify round 1 NOT CLEAN (4 blocking + 8 nits) → V2 re-put and user-ruled, the rest
lead-repaired; delta-check CLEAN on blocking, 7 nits lead-folded (N2 and N4 tightenings disclosed);
reviewer's final status `ready` · **Landed:** `DECISIONS.md` row 2026-09-23 · `BACKLOG.md` build item
under *Hook-enforced artifact schema build* (the prior item's item 3 superseded; the 2026-09-23 audit
item's gaps 2 and 4 annotated) · `ROADMAP.md` Template-schema CLI Next row touched (cap held) ·
**Opened:** 2026-09-23 · **Lead:** session lead (inline questioning via
`mochiko:analysis-iterative`) · **Review:** solo cold review — `mochiko:devils-advocate` seat on
`mochiko:review-brainstorm`, blind two-message dispatch (message 1: topic + goal line only; the index
and the session directory fenced) ·
**Transport (patterns-transport-floor):** one reviewer seat, lead-relayed messaging, no cross-seat
mesh; single writer per file — the record is the lead's, `reports/angle-map.md` and
`reports/review.md` the reviewer's

## Topic

Driver ask (user, 2026-09-23): "i want to look broadly on the hooks schema enforcement and file
location based on the runs above" — the runs being kinako `/mochiko:implement FEAT-001` run 3
(sessions `b0eddd5e`, `8b0d2f34`, `d87ac459`; plugin 0.110.0 → 0.112.0; 2026-09-19 → 22) and run 4
(session `abdf4115`; plugin 0.112.0; 2026-09-22 → 23), audited earlier the same day by transcript
(the audit's six gaps are booked as one `BACKLOG.md` item under *Defects & empirical checks*).

**Goal line:** decide what the write-time artifact gate and the declared artifact homes should
enforce, and where run artifacts should live, now that the gate has met two real implement runs —
leaving one hardened decision record.

**Prior-session relations.** `hook-enforced-artifact-schema` (accepted 2026-09-13, D1–D11) is the
ruling under review; its gate shipped at v0.109.0 (2026-09-15). Its D10 names a "kinako first-live-run
watch" and its D11 wave 5 a kinako violator pass run on the pre-gate pin before the upgrade — this
session's evidence is, in effect, that watch, taken without the violator pass having run.
`cli-schema-delivery` (D3/D7) and GI-019/GI-020 bound what any gate may be.

## Ground facts (F)

*Sources: the run transcripts under `~/.claude/projects/-Users-deepeshadmin-Documents-GitHub-kinako/`
(deny census by script over main thread + every subagent sidechain; a deny is a tool result carrying
the gate's "a second deny on this path halts" sentence), the kinako tree at HEAD, `mochiko-cli home`
at plugin 0.112.0 / 0.114.0, and this repo's `BACKLOG.md`, `CHANGELOG.md`, and the prior record.*

- **F1 — The gate went live in kinako without its wave-5 preconditions.** `CHANGELOG.md` 0.109.0
  (2026-09-15) ships the `PreToolUse` gate on `Write|Edit` and `Bash|PowerShell` (the latter narrowed
  by `if: Bash(*.mochiko*)`). Runs 3 and 4 ran under it. The prior record's wave 5 kinako half — an
  evidence-tree ruling ("161 evidence files with no legal home under `.mochiko/`; recommend a
  repo-root `evidence/`"), 214 moves + 143 pointer repairs on the pre-gate pin, then upgrade, then the
  D10 watch — is still listed as owed in `BACKLOG.md` *Hook-enforced artifact schema build*, whose
  item also reads "0.109.0 … not released". *(Corrected at review, S10: that line is not stale — it
  records an unmet GI-012 gate. `mochiko-cli` is not on crates.io (API 404), no `mochiko-cli-v*` tag
  exists, the installed binary is a local-path `cargo install`; the bump reached kinako through the
  marketplace on public `main` without the first publish the ledger made a hard precondition.)*
- **F2 — Deny census.** Run 3: 40 denies (Write 20 · Bash 15 · Edit 5). Run 4: 39 (Write 26 · Edit 7
  · Bash 6). By kind, both runs: size 47 · shell-write 21 · frontmatter/envelope 7 · undeclared
  sub-directory 3 · outside-home report sniff 1. Size denies split: the report envelope's 15-line
  `## Failure narrative` 25 and `## Notes of note` 9; entry-class sections at 20 lines 4; the 300-line
  whole-file bound 9 (FEAT-001 `baseline-delta.md` seen at 2,316–2,323 lines;
  `constraints-and-decisions.md` 306–380; `architecture.md` 301–350; `plan.md` 301; a desk
  `derivation.md` 301–367).
- **F3 — The shell leg's false-positive rate is 13 of 21.** *(Counts confirmed at review; the
  mechanism in (a) corrected at S12 — the `-i` scan runs over every later token of the whole
  command and glued separators are not split, so `grep -i` alone does not deny; see § Review.)*
  True positives (8): heredoc `cat >` into
  a home, `sed -i` edits of a home file, `cp <scratch> <home file>`. False positives (13), three
  shapes: (a) `grep -i` read as `sed -i` in-place (e.g. `grep -n -i … .mochiko/product/contracts/plugin-bridge.md`);
  (b) `cp <home file> <scratch>` — the source read as the destination; (c) a home path merely named in
  a command whose only write goes elsewhere (`tee /tmp/…`, `git show …:<home path> > $S/pre.md`,
  `sed -i` on a memory file with a home path later in the line). This session's lead hit shape (a)
  twice more, on `.mochiko/memory/knowledge-management.md` and `.mochiko/brainstorms/index.md`. Each
  false positive carries the "second deny on this path halts" instruction.
- **F4 — Cumulative stores cannot take a graded fold under a whole-file bound.** At kinako HEAD the
  product baselines stand far over the `product` home's 300-line whole-file bound: `data-model.md`
  1,206 · `constraints-and-decisions.md` 733; FEAT-001's own `baseline-delta.md` is 2,314.
  (`contracts/plugin-bridge.md` at 1,276 lines sits in the `product-contracts` home, which declares
  `bounds: elsewhere` — no gate size bound applies; the earlier parenthetical here was wrong, S6.) A landing fold only adds lines, so D4e's first-touch amnesty (a
  non-worsening write is allowed) can never admit it; no seat and no lead can land it. Run 3 landed
  its folds through the user's own shell (`! zsh …/APPLY.sh`, `APPLY-2.sh` — user `!` input does not
  pass through `PreToolUse`); run 4 closed DONE with them "owed by the principal's pen" (kinako memory
  `feat-001-run-4-state.md`; no run-4 commit touches those files).
- **F5 — Evidence has no home, so it went where nothing checks.** `reports/evidence/` is not a
  declared sub-directory of the `feature` home (declared: `contracts`); the gate denied 3 run-4 writes
  there. Run 4 then wrote 166 raw evidence files to `.mochiko/archive/evidence/FEAT-001-run4/`, where
  `mochiko-cli home` answers "no declared home governs this path — nothing here is checked at write
  time"; `.mochiko/archive/` holds 9 further non-evidence files (4 groom snapshots, the backlog
  trail, 3 product-baseline snapshots, `spine-groom.md`) equally unchecked — all 175 files there are
  git-tracked (count corrected from 22 at review, S14).
  Run 3's earlier cycles had used `reports/evidence/` (68 files, pre-gate).
- **F6 — Worktree paths resolve inconsistently.** Reports under
  `.claude/worktrees/mochiko-run3/.mochiko/features/FEAT-001/reports/` took envelope size denies (4) —
  a home resolved — yet one write to `…/reports/gap-rework-2-report.md` there was denied as a report
  "under no declared home". Likely cwd-dependent resolution; cause not verified.
- **F7 — The open-name `reports/` door admits minted names.** Both runs kept their running log as
  `reports/run-log-2026-09-19.md` (95 edits) and `reports/run-log-run4-2026-09-22.md` (130 edits)
  rather than the declared `implement-log.md` (an append-only log bounded per entry); the gate's only
  objection was a missing `report:` frontmatter field (2 denies), after which the name passed.
- **F8 — The brainstorm home rule names a path that resolves to the wrong home.** *(Widened at
  review, S4: six of the seven `*.artifact-home` render targets misresolve the same way; the fix is
  the directory resolver, not one rule.)*
  `brainstorm.artifact-home` says render `mochiko-cli home .mochiko/brainstorms/<slug>`; that path
  (with or without a trailing slash) resolves to the `brainstorms-index` home as "NOT a declared
  deliverable"; only `.mochiko/brainstorms/<slug>/record.md` resolves to the `brainstorm-session` home.
- **F9 — The gate is phase-blind by design.** Mid-run 4, `f4c3ae5` rewrote the product architecture
  spine 646 → 227 lines (a groom, archived to `.mochiko/archive/product-baselines/`) — allowed, as a
  shrinking write is non-worsening; the gate holds no run state (prior D9), so "baselines change only
  at landing" (`impl.baselines-never-in-place`) is not a gate concern today.
- **F10 — Denies cost rework, not just friction.** Run-4 seats trimmed reports to the 15-line
  narrative budget at least three times; two seats cut mid-write by a usage limit had their recovered
  reports denied on YAML shape before the lead could land them (kinako memory
  `run4-paused-weekly-limit.md`).
- **F11 — The plugin already means raw evidence to be ephemeral.** `testing-end-user`'s
  `references/EVIDENCE-CAPTURE.md` (0.112.0): "Evidence files use a `verify-` prefix under a scratch
  directory (`/tmp/claude/`) … These are ephemeral runtime artifacts, cleaned up after the
  checkpoint"; `REPORT-TEMPLATES.md` truncation points at `Full log: /tmp/claude/verify-C{N}-….log`.
  The implement rules demand evidence be captured (`impl.craft-verify-bindings`,
  `impl.fail.no-evidence`, `testing-end-user.completion-set`), never that raw output be kept in the
  repo. (Tension: the report's "full log" pointer names a file the same skill deletes.)
- **F12 — What made evidence durable in run 4 was the lead's briefs plus run-3 precedent, over a
  real cross-session need.** From C1-15 on, the lead's seat briefs put "evidence files" in the
  verifier's write set and asked for "evidence pointers" per case, and later briefs said "Evidence
  beside the builder's" and gave the verifier the builder's evidence path to read. Run 3 had kept
  evidence git-tracked under `reports/evidence/` before the gate. When the gate denied that folder,
  the lead recommended `.mochiko/archive/evidence/FEAT-001-run4/` at the C1-15 checkpoint
  ("book a plugin migration for `reports/evidence/`"), ruled "as recommended". The need under it: the
  session scratchpad is per-session, run 3 spanned four sessions and run 4 a usage-limit pause, and
  seats read each other's output — the repo was the only place both cross-seat and cross-session.
- **F13 — The feature home declares no log; the epic home does.** `mochiko-cli home
  .mochiko/features/FEAT-001/implement-log.md` (0.114.0): "NOT a declared deliverable of this home"
  — the `feature` home's set is twelve deliverables plus `contracts/` and `reports/`, no log. The
  `epic` home declares `implement-log.md · append-only log · 60 lines per ## entry`. Yet
  `impl.artifact-home` (rendered to every implement run) says "`implement-log.md` is bounded per
  entry, never per file", and a `FEAT-XXX` run — the common case — had nowhere legal to keep its log.
  (The prior record's OQ2 left the log's home to the census; the census declared it for epics only.)
- **F14 — The run log lived in a report's frontmatter, where no budget reaches.** Run 4's
  `reports/run-log-run4-2026-09-22.md` (390 lines) carries `report: disclosure`; 385 of its lines are
  YAML frontmatter and its one `##` section is 3 lines. The report envelope bounds only `##` sections
  ("Line budgets, each counted from its `##` line to the next `##`"), the frontmatter has no line
  budget, and "Unknown extra fields are permitted" — so a running log of any length conforms as a
  `disclosure` report. This is F7's mechanism: not a name minted past the gate, but the only
  conforming shape a feature run's log could take.
- **F15 — Home resolution is textual and cwd-relative, which is F6's cause.** `mochiko-cli home` on
  `.claude/worktrees/mochiko-run3/.mochiko/features/FEAT-001/reports/a.md` (relative or absolute,
  from the repo root) answers "no declared home", and `check` denies a report there by the sniff
  ("sits under no declared home"); the same file written by a seat whose cwd is the worktree root
  resolves to the `feature` home (run 3's four worktree size denies). Every `impl.cold-verification`
  snapshot and every worktree-transport run therefore sees the gate differently per seat.
- **F16 — No rule requires a run log.** Across the 106 implement rules at 0.112.0, `implement-log.md`
  appears once — in `impl.artifact-home`, which only bounds it if written. What the rules do require
  durable are the typed reports (`sufficiency-report.md`, cycle / verification / final-validation /
  built-vs-signed reports), the `tasks.md` checkboxes as the progress surface, the landing rows
  (`DECISIONS.md`, `BACKLOG.md`, the feature entry), and evidence captured at verification.
  `impl.dm-surface-rounds` and `impl.dm-close-verdict` are duties to tell the user, not files. The
  log's observed jobs in runs 3 and 4 were the lead's working memory between checkpoints and the
  resume list after a pause (run 3 resumed from `run-log-2026-09-19.md`; run 4's
  `final_validation.round_1_checkpoint.resume_list`) — cross-session state, not record. The epic
  home's `implement-log.md` declaration came from the prior record's census of kinako's EPIC-001
  practice (prior F10/D2), not from a rule.
- **F3 (addendum)** — a third false positive in this session: `… | grep -i 'report'` after a
  `mochiko-cli home …/reports/x.md` read, denied as a shell write to `reports/x.md`.

## Constraints carried in

- GI-019 bright line: the gate stays mechanical — path · file set · headings/frontmatter · size — never
  judgment, never sequencing (prior D1/D7; AM-3 clause iv).
- GI-020: no schema file ships; homes live in the migration log (prior D3), no consumer-local
  declaration surface (prior D2/R2b).
- Hooks are fail-open and stateless (prior D7c/D9); a deny holds in every permission mode.

## Decisions (D)

*(recorded as ruled — statement · rationale · confidence)*

### D1 — Scope: one redesign of what each home allows, covering both file sprawl and the gate blocking legitimate work

**Statement:** The session treats the two symptoms — files landing in the wrong or unchecked places
(F5, F7) and the gate denying honest work (F3, F4) — as one problem: the home rules do not tell a
per-run output from a store that grows over the product's life. The outcome is a redesign of home
classes and what each class's gate checks, not a list of point fixes.

**Rationale:** F4 and F5 share a root: a single bound shape (whole-file lines, closed set) applied to
a product baseline that only grows and to a run's raw evidence alike. Fixing the landing deny alone
would leave evidence homeless; declaring an evidence folder alone would leave the landing impossible.

**Confidence:** Confident (user-ruled at Q1: "yes both", over the fix-sprawl-only and fix-denies-only
options).

### D2 — Cumulative stores take a per-entry size budget and no whole-file bound

**Statement:** A home whose file grows over the product's life — the product baselines
(`data-model.md`, `constraints-and-decisions.md`, `contracts/*`) and the architecture store — is
bounded per entry (one entity, one decision, one contract clause), the way an append-only log is
bounded per entry today; the file itself carries no line cap. A landing fold that adds an entry is
therefore a conforming write, and no seat needs a carve or the user's own shell to land it.

**Rationale:** F4 — every kinako product baseline already stands 2–4× over the 300-line whole-file
bound and a fold can only add lines, so D4e amnesty can never admit it. A per-entry budget keeps
verbosity pressure where the text is authored and needs no landing exception. Rejected: **no size
check** (verbosity unchecked on the stores that are read most) · **keep the file cap with a
landing-only carve** (keeps a number every store already breaks, and makes the gate phase-aware,
against prior D9's stateless posture).

**Confidence:** Confident (user-ruled "as recommended" at Q2). The entry grammar each store declares
and the budget numbers are open (OQ1).

**Amended at review (S6, user-ruled):** `contracts/*` leave D2's scope (no gate bound applies to
them today — `product-contracts` declares `bounds: elsewhere`); the guarantee "no seat needs a
carve" is **conditional on OQ1** — the entry budget must admit the largest honest entry (177 lines
observed) and the fold shape is ruled at the census; the split-into-per-entry-files road is the
recorded fallback. Full disposition under § Review.

### D3 — Closed world under `.mochiko/`: a write that resolves to no declared home is denied, and the deny names the run-scratch route

**Statement:** Inside `.mochiko/`, the gate answers every `Write`/`Edit` (and every shell write the
`Bash|PowerShell` leg parses) with a home or a deny: a path that resolves to no declared home is
refused — no more "no declared home governs this path — nothing here is checked". The reason names
the two honest routes: a declared deliverable or `reports/` file of the nearest home, or — for raw
output (captured console, logs, dumps) — the run-scratch location (Q5), stated as ephemeral.
`mochiko-cli home` says the same for such a path. The `evidence/` deny reason drops "a new
sub-directory takes a migration" (F12: that sentence is what the lead booked) for the scratch route.
Every real directory under `.mochiko/` that mochiko itself writes — `archive/` (the backlog trail,
groom snapshots, brainstorm archive), `schema-views/`, `strips/`, `decisions/`, `memory/`, `specs/`,
`features/`, `epics/`, `product/`, `brainstorms/` — is declared in the `home` migration first, from a
census of mochiko's own tree and kinako's. Outside `.mochiko/` nothing changes: prior D9's
frontmatter sniff stays, plain files elsewhere stay the product's business.

**Rationale:** F5 and F12 — the copying pattern did not defeat the gate, it walked around it into
the one region the gate did not watch, and the CLI's own wording confirmed the region as safe.
Prior D5 already scoped the registry to "the whole `.mochiko/` tree"; this closes the gap between
that scope and what `check` evaluates. Rejected: **also deny raw non-markdown files inside declared
homes** (B — `reports/` already refuses any file without the envelope, and deliverable sets are
closed, so it adds a second rule for the same effect) · **a narrow `evidence/` deny only** (C — the
next unplanned folder gets the same open door).

**Confidence:** Confident (user-ruled "yes A" at Q4, after asking that the hooks and CLI be hardened
against the copying pattern rather than a location chosen for it).

### D4 — Raw output lives in a git-ignored run folder inside the repo, deleted after final acceptance — ruled as `.claude/mochiko-runs/<run>/`, **amended at review to `.mochiko/runs/<run-id>/`** (see the amendment note below and § Review S2)

**Statement:** The route D3's deny names is a run-scoped scratch folder under
`.claude/mochiko-runs/<run-id>/` (beside `.claude/worktrees/`, whose ignore entry the implement
run already ensures — `impl.cold-verification`; the same ignore line covers it), shared by every seat
of the run, surviving session changes, pauses, and reboots, and outside git. Captured console, logs,
dumps, screenshots and any other raw output go there; a report quotes the decisive lines with the
command and commit that reproduce them, and its "full log" pointers point there. The lead deletes the
folder after the user's final acceptance (a landing step). `testing-end-user`'s `/tmp/claude/`
scratch path (F11) re-keys to this folder. The gate does not watch `.claude/`.

**Rationale:** F12's real need — cross-seat and cross-session reach — is what pushed evidence into
the repo; a per-session scratch (C) loses it at every pause, and the system temp folder (B) at every
reboot. A git-ignored folder inside the repo is the one place both properties hold, and the CLI can
print its exact path in the deny. Rejected: **B** (lost on reboot; runs have paused for days) · **C**
(a copy-forward step at every pause is the fragile hand-work the gate exists to remove).

**Confidence:** Confident (user-ruled "yes A" at Q5). The run-id form (`FEAT-001-run4`-style vs the
run log's own key) is a build detail.

**Amended at review (S2/S9/S13, user-ruled, `Contested`):** `.claude/` is a Claude Code protected
path (doc-verified), so the folder is now `.mochiko/runs/<run-id>/` — a declared raw-output home
inside the closed world, held by four mechanical controls (location · run-key name · write refused
unless `.gitignore` carries `.mochiko/runs/` · a `runs/` content rule: the report sniff on `.md`,
shell writes admitted for non-`.md` targets only) and one procedural (deletion at acceptance); the
folder is always the main tree's. The record's ignore claim ("the same ignore line covers it") was wrong and is
withdrawn. Full disposition under § Review.

### D5 — The shell leg denies only a home path in a write position

**Statement:** `mochiko-cli check`'s `Bash`/`PowerShell` parse treats a declared-home path as a write
target only where the command's grammar puts a write there: the right side of `>` / `>>` (heredoc
included), an argument of `tee`, the last argument of `cp` / `mv` / `install`, the file argument of
`sed -i` / `perl -i`, and the PowerShell write cmdlets already listed. A `-i` flag reads as in-place
only on `sed` and `perl`; a path in any other position — a `grep`/`sed -n`/`wc`/`cat` operand, the
*source* of `cp`, a `git show` object spec — is a read. The crate matrix gains one case per
false-positive shape from F3 (`grep -n -i … <home>`, `cp <home> <scratch>`, `tee /tmp/… ; sed -n … <home>`,
`git show <rev>:<home> > <scratch>`) and one per true positive. The three evasions the prior build
disclosed (`cd <home> && … > x.md`, a path held in a variable, a relative write from a cwd inside a
home) stay open and disclosed, as before.

**Rationale:** F3 — 13 of 21 shell denies across the runs, and 2 in this session, were reads; every
true catch had the path in a write position. Each false deny carries the "second deny halts"
instruction, so it costs a seat a halt-and-surface, not a retry. Rejected: **warn on ambiguous
shapes** (B — adds a third outcome the wrapper and the seat must interpret; the write-position rule
leaves nothing ambiguous in the observed set) · **keep as is** (C).

**Confidence:** Confident (user-ruled "as recommended" at Q6). **Amended at review (S12):** the
matrix is built from the 13 real denied commands and the 8 true positives, not minimal forms; the
observed mechanism is the `-i` scan over every later token and unsplit glued separators (`file;`);
`mv <home> <elsewhere>` stays a deny (a removal from the home). **(V1/N2):** one carve — a shell
write whose target sits under `.mochiko/runs/<run-id>/` and is not a `.md` file is allowed; every
other home keeps the deny.

### D6 — The run log is ephemeral working state in the run folder — ruled with a report-header line cap, **amended at review: cap dropped, standing in-run rulings route to `.mochiko/decisions/`** (see the amendment note below and § Review S1/S5)

**Statement:** The implement run's log lives at `.claude/mochiko-runs/<run-id>/implement-log.md`
(D4's folder), never under a declared home; it holds the lead's working memory between checkpoints
and the resume list, and goes with the folder after final acceptance. The `epic` home's
`implement-log.md` deliverable is withdrawn as a recorded supersession of the prior census ruling
(prior D2's "expected to be declared as an append-only log" and the wave-3 `0005` entry), and
`impl.artifact-home`'s log sentence re-points to the run folder. Every report type in the envelope
gains a **frontmatter line budget** (numbers at the census, tight posture per prior D6, sized from
the machine-first payload each type declares) so that a report's YAML header cannot carry what its
sections may not. What stays durable is what the rules already require (F16): the typed reports,
`tasks.md`'s checkboxes, the landing rows, the commits.

**Rationale:** F16 — no rule requires the log and nothing reads it after acceptance; F13/F14 — with
no legal home, the log took the one conforming shape left, a `disclosure` report with 385 header
lines, which shows the header is the budget's blind side. Rejected: **declare the log in the feature
home** (B — one more permanent file per run, 95–130 edits in the runs seen, that no later reader
opens) · **ephemeral log, headers uncapped** (C — leaves the loophole the log used).

**Confidence:** Confident (user-ruled "as recommended" at Q7, after asking whether the log is
required at all and answered from F16).

**Amended at review (S1, user-ruled):** the premise "nothing reads it after acceptance" was wrong
for in-run rulings — kinako's `DECISIONS.md` cited the log as their rationale home. Routing added: a
standing in-run ruling lands as a `.mochiko/decisions/` record at the landing; a checkpoint
acceptance gets no `DECISIONS.md` row (report + checkbox carry it); the log stays ephemeral. Existing
logs and their citations stand. **(S5, user-ruled):** the report-header line cap is dropped — honest
headers overlap the log's size, and with the routing above the loophole has no user; OQ4 closes.
The log's folder is D4 as amended (`.mochiko/runs/<run-id>/`).

### D7 — Home resolution never keys on cwd — ruled as "the last `.mochiko/` segment", **amended at review to the tree root (nearest `.git`), nested trees never homes** (see the amendment note below and § Review S3)

**Statement:** `mochiko-cli check` and `mochiko-cli home` locate a file's home by the last
`.mochiko/` segment in its path (after joining a relative path to the payload's `cwd`), treating
everything before that segment as the tree root; the root operating docs keep their by-name rule at
that root. A worktree copy (`.claude/worktrees/<name>/.mochiko/…`) therefore takes the same homes and
checks as the main tree for every seat, whatever its cwd. The run folder (D4) carries no `.mochiko/`
segment and stays outside the gate. One crate case per shape: relative from the root, relative from
inside the worktree, absolute, and the run folder.

**Rationale:** F15 — the cwd-relative textual match made the same file gated for one seat and
unwatched for another, and `impl.cold-verification` snapshots ride worktrees by rule. A pure string
rule is deterministic and free per call. Rejected: **walk up to the git root** (B — a filesystem read
on every call for the same answer) · **brief seats never to `cd`** (C — the run-4 briefs already said
so, and the seats' cwd is not the lead's to hold).

**Confidence:** Confident (user-ruled "yes A" at Q8).

**Amended at review (S3, user-ruled):** resolution keys on the tree root (nearest `.git` directory
or worktree file) and only `<root>/.mochiko/` is a home tree — the "last `.mochiko/` segment" rule
would have gated 354 nested fixture files in this repo, and the rejected-road reason ("the same
answer") was false. The root-operating-docs clause is withdrawn. Full disposition under § Review.

### D8 — Narrative budgets stay at 15; the prior D10 re-key trigger fired and is declined; the pre-write dry run is the seat's standard step

**Statement:** The report envelope's `## Failure narrative` and `## Notes of note` budgets stay at
15 lines. The prior record's D10 trigger ("a deny rate that stays high after the second run is the
budget table's re-key trigger") is recorded as fired by runs 3 and 4 (34 narrative denies) and
declined here: the observed overages were 16–20 lines in nearly every case (a few at 22–23, two at
29), trimmed to budget without loss. The authoring-time channel absorbs the cost instead: a seat
dry-runs its draft through `mochiko-cli check --hook-json -` (with `cwd`) before the `Write`, as
run 4's briefs already required — the producing skills' rules say so, and the `SubagentStart` line
carries it. A deny then marks a seat that skipped the dry run, not an honest draft.

**Rationale:** F2/F10 — the budget is doing what prior D6 chose it for; the cost was the re-emit,
which the dry run removes at zero gate change. Rejected: **re-key to 20** (B — ratifies the
overshoot; the two 29-line cases would still deny) · **warn instead of deny** (C — the size limb
becomes advisory, the posture prior D6 rejected).

**Confidence:** Confident (user-ruled "as recommended" at Q9). **Amended at review (S15):** "trimmed
without loss" is observed, not measured; the watch carries the displacement metrics and trigger;
wave 1 adds a direct dry-run form so the step needs no hand-built payload.

### D9 — Wrap-up: phase stays procedural · kinako clean-up · governance · four-wave build — **(b), (c), (d) amended at review** (a full-inventory pass; a ledger amendment replaces "no governance amend"; the bookkeeping and waves as in the amendment note and § Build surface)

**Statement:** **(a)** The gate stays phase-blind (prior D9); whether a store may be groomed mid-run
(F9) is an implement-rule matter, carried by the 2026-09-23 audit item in `BACKLOG.md` (its gap 3),
not by this record. **(b)** At the kinako violator pass, the 166 files under
`.mochiko/archive/evidence/FEAT-001-run4/` and run 3's 68 under `reports/evidence/` are deleted from
the tree (history keeps them); the 22 other `.mochiko/archive/` files are sorted into the declared
`archive/` set or out of `.mochiko/`. **(c)** No governance amendment: every check this record adds
or changes — closed world under `.mochiko/` (path), the epic log withdrawn (set), header caps and
per-entry budgets (size), write-position parse and segment resolution (evaluator mechanics) — sits
inside the mechanical class AM-3 clause (iv) admitted; the ledger pointer is untouched.
**(d) Build order — four waves, each under the primitive-edits ceremony and gate 6:**
**wave 1, crate** — closed-world resolution under `.mochiko/` (D3), write-position shell parse
(D5), `.mochiko/`-segment resolution (D7), frontmatter line budgets and per-entry budgets as
evaluable check kinds (D6, D2), the deny and `home` texts naming the run-scratch route (D3/D4),
matrices per shape from F3/F15, `mochiko-cli` publish; **wave 2, migration** — a census of every
real `.mochiko/` directory in mochiko's and kinako's trees → `archive/`, `schema-views/`, `strips/`
and any other live directory declared with their sets; the `epic` home's `implement-log.md`
withdrawn (recorded supersession of the wave-3 `0005` entry); the cumulative stores' entry grammar
(a template per store declaring its entry heading) and the budget table re-keyed with header caps and
per-entry numbers — **user-ratified as one table** (OQ1); **wave 3, prose** — `impl.artifact-home`
(log → run folder; render target unchanged), `brainstorm.artifact-home` (render target →
`<slug>/record.md`, F8), `testing-end-user`'s scratch path → `.claude/mochiko-runs/<run>/` and the
pre-write dry run as a rule (D8), the `SubagentStart` line extended with the run-scratch route, the
`evidence/` deny reason; strips + audits; `plugin.json` bump; **wave 4, kinako** — the violator pass
(b) on the upgraded plugin (the closed world denies nothing that already exists on disk — the gate
is write-time), the D10 first-live-run watch re-measured on the next implement run (deny count by
kind, false-positive share of the shell leg, files per run under `.mochiko/`). Bookkeeping: the
*Hook-enforced artifact schema build* item's stale "0.109.0 … not released" line corrected and its
item 3 re-pointed here; the 2026-09-23 audit item's gaps 2 and 4 annotated superseded by this record.

**Rationale:** (a) the stateless posture was argued and ruled once; (b) evidence is ephemeral by D3/D4
and the files are already in history; (c) the admission was scoped by check class, and no class is
new; (d) mirrors the prior record's crate → migration → prose → consumer order, with the census as
the ratification gate because the numbers land there.

**Confidence:** Confident (user-ruled "as recommended" at Q10, the batch put with each item's
recommendation named).

**Amended at review (user-ruled):** **(b)** the kinako pass is a full inventory (S11): every
evidence tree (68 + 166 + FEAT-002's 93), the six legacy undeclared directories, the 47 pointers;
the archive count is 9, not 22 (S14). **(c)** superseded by S8 — a PATCH ledger amendment in wave 3
re-words GI-019's "Reach of the gate", amnesty and "What the gate reads" paragraphs for the closed
world and annotates the C1 discharge with the field result; superseded within the S8 batch
disposition (it did not get its own line — V11). **(d)** the "0.109.0 … not released"
backlog line is not corrected — it is the honest trace of an unmet GI-012 publish gate, recorded as
a ledger exception entry; wave 1's publish names the two owed controls as hard preconditions (S10);
"render target unchanged" for `impl.artifact-home` is withdrawn — the directory-resolver fix covers
all seven `*.artifact-home` targets (S4); wave 1 also adds the direct dry-run form and wave 4's
watch carries the displacement metrics and trigger (S15); the run folder is `.mochiko/runs/`
(D4 as amended) with its ignore-guard check in wave 1 and its home declared in wave 2.

## Review (cold, round 1 — `reports/review.md`; blind map `reports/angle-map.md`, 53 angles / 8 classes)

**Reviewer:** `mochiko:devils-advocate` seat on `mochiko:review-brainstorm`, solo; two-message
dispatch, index fence held. **Verdict:** `critical-gaps` — 24 raised, 16 survived: 3 Critical
(S1 D6 · S2 D4 · S3 D7) · 8 Important (S4–S11) · 5 Minor (S12–S16); coverage survivors S4, S6
(part), S11, S16. **Cross-examination:** 10 questions, 7 answered "not in the record" (Q&A in the
report's `cross_exam`). **Lead's source re-reads (owed on S2/S10):** S2 confirmed verbatim at
code.claude.com/docs/en/permission-modes § Protected paths — "`.claude`, except for
`.claude/worktrees`"; `auto` → "Routed to the classifier", `dontAsk` → "Denied"; "`permissions.allow`
rules … do not pre-approve protected-path writes". S10 confirmed — crates.io API returns 404 for
`mochiko-cli`, no `mochiko-cli-v*` tag, the installed binary is a local-path `cargo install`. Also
re-checked: S1's 27 citing rows in kinako `DECISIONS.md`; S3's 354 tracked files under nested
`.mochiko/` trees in this repo; S8's ledger paragraphs. **Verification honesty (reviewer):** F2, F3,
F4 sizes, F5, F8, F9, F11–F13, F15, F16 confirmed; wrong: F4's contracts parenthetical, F5's "22"
(9), F3(a)'s mechanism, D4's three claims, D6/F16's "nothing reads it", D7's rejected-B premise, F1's
"stale"; unverified: F7 counts, F10, F6's count, D8's "without loss".

**Dispositions:**

- **S1 (Critical, D6) — user-ruled "as recommended" on the re-put.** The reviewer's premise holds
  in part: 27 kinako `DECISIONS.md` rows cite a run log or the epic log as their rationale home, and
  no implement rule names `DECISIONS.md` at all — the pointer comes from the KM contract ("session
  record, else a `.mochiko/decisions/` record"), which an implement run satisfied with the nearest
  file. The 27 split: 13 standing rulings (EPIC-001/002: the seat keychain rule, the frozen revision
  digest, `AX-007` amended …) and 14 checkpoint bundles from runs 3–4 ("C1-16 checkpoint — five
  rulings, all as recommended") that the KM contract never asked for (one row per ruled decision, not
  per gate). Nothing ruled here deletes an existing log, so today's pointers keep resolving; the
  exposure was future runs. **Fold (D6 amended):** a future run's standing in-run ruling lands as a
  `.mochiko/decisions/<date>-<slug>.md` record at the landing (the KM record-less path, minted into
  the implement rules at wave 3); a checkpoint acceptance gets no `DECISIONS.md` row — its
  verification report and the `tasks.md` checkbox carry it; the run log stays ephemeral (D6 holds).
  The epic home's `implement-log.md` withdrawal is a declaration change only — the files on disk and
  the 11 rows citing them stand. Rejected on the re-put: the reviewer's durable per-run rulings log
  (a third record surface beside reports and decisions) · rows per checkpoint pointing at reports.
- **S2 (Critical, D4) + S9 + S13 — user-ruled "yes B" on the third put (`Contested`).** Source
  re-read confirmed the finding verbatim. First put: a repo-root `.mochiko-runs/<run-id>/` (ruled
  "as recommended", then **reversed by the user** — "concerned that there will be more files dumped
  in this folder by agents once they see the pattern"). Second put: the plugin data directory
  `${CLAUDE_PLUGIN_DATA}/runs/<project>/<run-id>/` (doc-verified: `~/.claude/plugins/data/<id>/`,
  persistent across sessions and updates, exported to hooks; recommended) vs a declared scratch home
  inside `.mochiko/`; the user asked why the in-tree home could not be held by the CLI and hooks, and
  ruled it on the answer. **Fold (D4 amended):** the run folder is `.mochiko/runs/<run-id>/`, a
  **declared home** under the closed world with five controls, four mechanical: (1) location — the
  one raw-output home; every *undeclared* path under `.mochiko/` is denied by D3, so the pattern can
  spread only into declared places — which, after S7's fold, include four sub-directories open by
  location with content unchecked (`specs/<slug>/prototype/`, brainstorm `inputs/`, `research/`,
  `referents/`); *(corrected at verify V2 — the first wording "nowhere to spread" was false; the
  corrected basis was re-put to the user, see § Verify)*; (2) name — `<run-id>` must match the
  run-key form; (3) **ignore guaranteed** — `check` refuses any write under `runs/` while
  `.gitignore` lacks the `.mochiko/runs/` entry (one file read per call; run-open adds the entry), so
  nothing there can reach a commit or a cold-verification snapshot (S9 closed structurally); (4) a
  **`runs/` content rule** (new, since prior D9's sniff runs only outside homes): a `.md` opening
  with report frontmatter is refused there — no *report* hides in scratch (S13; the ephemeral
  `implement-log.md` of D6 carries no report frontmatter and is allowed by name); shape and size are
  otherwise unchecked, it is raw; **and `runs/` is the one home whose shell writes are admitted** —
  the `Bash`/`PowerShell` leg allows `>`/`>>`/`tee`/`cp`/`mv` into `runs/<run-id>/` **for non-`.md`
  targets only** (`.log`, `.txt`, `.json`, images …), since the gate cannot see what a shell write
  puts in a file and a heredoc could otherwise smuggle a report past control 4; a `.md` in `runs/`
  goes through `Write`/`Edit`, where the sniff reads it (raw capture is shell output by nature,
  F11's `cmd 2>&1 | tee …`), with matrix cases both ways *(verify V1; delta N2)*; (5)
  deletion is procedural — a landing step after the user's acceptance (`mochiko-cli run close <id>`
  or the lead's own removal), with a `SessionStart` line listing every `runs/*` folder present (no
  state needed; the lead knows which is current — *verify V12*). **The run folder is always the
  main tree's:** the deny and `home` print its absolute path under the main tree, and a write to a
  git worktree's own `.mochiko/runs/` is refused with that path as the reason (read from the
  worktree's `.git` pointer file — a third read the ledger paragraph names, *delta N4*) — so a
  worktree-transport seat's captures survive the worktree's removal and every seat shares one folder
  *(verify V3)*. An `impl.cold-verification` snapshot is a plain copy with no `.git`, so under D7 as
  amended its `.mochiko/` is a nested non-home tree the gate does not police; the snapshot seat's
  brief carries the main tree's `runs/` path and `reports/` home by absolute path, and its writes
  there take the ordinary checks *(delta N4)*.
  Reports cite the reproducing command and commit, never a "full log" path. What the gate cannot
  stop — dumping inside `runs/` during a run — is bounded to disk until deletion, in the folder raw
  output belongs to.
  Rejected: repo-root `.mochiko-runs/` (user-reversed: an undeclared in-repo folder invites copying)
  · `${CLAUDE_PLUGIN_DATA}` (shared across every project, deleted on uninstall, invisible from the
  repo; its one edge — nothing in the repo — is what control 3 makes moot for git) ·
  `.claude/worktrees/<run-id>/` (the platform's own store, undocumented pruning) · system temp by
  run (lost on reboot).
- **S3 (Critical, D7) — user-ruled "as recommended".** Confirmed: 354 tracked files under nested
  `.mochiko/` trees (`evals/*/fixtures/**`, `crates/mochiko-cli/tests/fixtures/genesis-corpus/`),
  outside every home today; D7's rejection of git-root resolution rested on a false premise.
  **Fold (D7 amended):** `check` and `home` resolve a file's home against the tree root it belongs
  to — the nearest ancestor holding `.git` (a directory, or the file a worktree carries) — and only
  `<root>/.mochiko/` is a home tree; a nested `.mochiko/` under that root (fixtures, eval
  workspaces) is never a home and stays outside the closed world. A worktree resolves to its own
  homes by construction. A path outside the cwd's tree resolves against its own root (so a write into
  another repo's `.mochiko/` is checked there). The root-operating-docs clause is dropped: they stay
  as today (prior R6, unwatched by the binary). Crate matrix: root-relative, worktree-relative,
  absolute, nested fixture, the run folder, and an out-of-tree path. Rejected: the segment rule with
  an exclusion list (B — a list to maintain, gated until it grows) · renaming 354 fixture files (C).
- **S4–S8, S10–S12, S14–S16 (Important + Minor batch) — user-ruled "as recommended".**
  - **S4 (coverage):** `home` resolves a directory path to the home it names (wave 1), one matrix
    case per `*.artifact-home` render target; D9d's "render target unchanged" for
    `impl.artifact-home` is withdrawn — all seven targets are covered by the resolver fix.
  - **S5:** the header cap is **dropped** from D6 — honest report headers (review median 256, max
    544) overlap the log's 384, and with S1's routing the log has no reason to hide in a report;
    OQ4 closes. D6 now: ephemeral log + the S1 routing, nothing on headers.
  - **S6 (part coverage):** F4 corrected — `contracts/*` sit in the `product-contracts` home with
    `bounds: elsewhere`, no gate size bound applies, and `contracts/*` leave D2's scope (`api.yaml`
    has no markdown entry grammar). D2's guarantee is **conditional on OQ1**: the per-entry budget
    must admit the largest honest entry observed (177 lines) and the fold shape — a dated fold block
    appended vs an in-place `[EXTEND]` of an entry — is ruled at the census table; an in-place
    amendment to an entry already over budget rides D4e amnesty per entry — a non-worsening edit is
    allowed, **a growing edit to such an entry is denied**, which is why the census must size the
    budget from the largest honest entry *(verify V9)*.
    The split-into-per-entry-files road (the `concerns/<AX-ID>.md` graduation pattern) is recorded
    as the **fallback** if the census finds an entry class one budget cannot fit — not the default,
    because it touches every store-authoring skill and every cross-reference into the stores.
  - **S7:** the closed-world census is taken from the primitives' write sets (rules and templates —
    incl. the KM module's nesting ruling that can place `ROADMAP.md` under `.mochiko/`), with both
    trees as cross-check; a declared sub-directory without its own home document (`prototype/`,
    `inputs/`, `research/`, `referents/`) is inside the closed world, allowed by location, unchecked
    for content — stated explicitly in the `home` migration. *(Verify V2, user-ruled: the report
    sniff extends into these four sub-directories — a `.md` opening with report frontmatter (the existing sniff test, no new check) is refused
    there; raw working material stays allowed.)*
  - **S8:** D9c amended — the ledger's GI-019 "Reach of the gate" and amnesty paragraphs, **and its
    "What the gate reads (review C7)" paragraph** *(verify V4: control 3 reads `.gitignore` on every
    `runs/` write and D7 walks ancestors for `.git`, so the gate now reads two things beyond the
    payload)*, are re-worded for the closed world under `.mochiko/` by a ledger amendment in wave 3,
    recorded in the amendment log. **Version class:** proposed PATCH — a text restatement of checks
    inside AM-3 clause (iv)'s admitted class, no floor removed, no admission widened; if the amend
    run reads the added filesystem reads as the ledger's "significantly expanded" limb (ledger line
    32), it is MINOR — the class is the user's call at the `/mochiko:setup` amend run, not this
    record's. The C1 "DISCHARGED" note gains the field result (the ratified table did not admit
    honest cumulative content — F4, nine whole-file denies in F2 — re-keyed at wave 2). D9c's
    no-amendment call, which rode the Q10 batch, is thereby superseded within this batch
    disposition *(verify V11: it did not get its own line)*.
  - **S10:** F1's "stale" is withdrawn — the backlog line "0.109.0 … not released" is the honest
    trace of an unmet GI-012 gate: `mochiko-cli` is not on crates.io (API 404), no `mochiko-cli-v*`
    tag exists, the hook bump reached kinako through the marketplace on public `main`. D9d's
    bookkeeping no longer "corrects" the line; instead a **ledger exception entry** records the gate
    as breached at 0.109.0 (2026-09-15) with this record as the finding source, and wave 1's publish
    names the signed-tag and crates.io manual-approval controls as hard preconditions.
  - **S11 (coverage):** D9b becomes a **full inventory** of kinako's `.mochiko/` tree at the pass:
    every evidence tree deleted (FEAT-001's 68 and 166, FEAT-002's 93), every legacy undeclared
    directory (`features/B53` 9, `features/B61` 1, `epics/EPIC-001/reviews` 15, `epics/EPIC-002/reviews`
    8, `epics/EPIC-001/landing` 4, `features/FEAT-006/reviews` 4) declared by the census or moved out,
    and the 47 pointers in 24 files annotated "in history at <commit>" or repaired.
  - **S12:** D5's matrix is built from the 13 real denied commands (and the 8 true positives) rather
    than minimal forms; F3(a)'s mechanism corrected — the `-i` scan runs over every later token of
    the whole command, and a separator glued to a word (`file;`) is not split, which is how a `tee`
    or `-i` elsewhere in the line reached a home path; `mv <home> <elsewhere>` stays a deny (a
    removal from the home, prior design), stated. Shell writes **into** `.mochiko/runs/<run-id>/`
    are the one admitted exception *(verify V1; D4 as amended)*.
  - **S14:** F5 and D9b corrected — `.mochiko/archive/` holds 9 non-evidence files (4 FEAT-001 groom
    snapshots, `backlog-trail.md`, 3 product-baseline snapshots of 2026-08-21, `spine-groom.md`), not
    22; all 175 files there are tracked, so "history keeps them" holds.
  - **S15:** D8's "without loss" is re-marked **observed, not measured**; the wave-4 watch restores
    prior D10's displacement metrics (per-home total volume · reports-per-run · section sizes
    before/after) with the same trigger (a deny rate still high after the second run re-keys the
    table); wave 1 adds a direct dry-run form (`mochiko-cli check --path <p> --content -`) so a seat
    need not hand-build a `PreToolUse` payload.
  - **S16 (coverage), one line each:** (i) the user's `!` shell is the recorded break-glass over a
    deny in every permission mode — sanctioned for the user, never a seat's route, and never a
    substitute for a fold the gate should admit (D2); (ii) a run in flight at a plugin upgrade reads
    the new homes at its next write, disclosed at its next checkpoint — no run pins a plugin
    version; (iii) per-run namespacing of `reports/` is declined now (a flat `reports/` with the
    run key in the name stays), revisited on the watch's files-per-run figure.

## Verify (round 1 — `reports/review.md` `verify_round_1`; NOT CLEAN: 4 blocking + 8 nits, all folds landed)

- **V1 (blocking, lead-repaired):** the `runs/` home collided with the shell leg's deny — raw
  capture is shell output (`tee`, `>`), and D5 denied every shell write into a home. Fold: `runs/`
  is the one home whose shell writes are admitted; matrix cases both ways (D4 control 4, D5).
- **V2 (blocking, user-ruled "as recommended"):** "the pattern has nowhere to spread" was false —
  S7 leaves `specs/<slug>/prototype/` and brainstorm `inputs/`, `research/`, `referents/` open by
  location. Re-put with the corrected basis: **C2 stands** (`.mochiko/runs/`), and the report sniff
  extends into those four declared sub-directories — a `.md` opening with report frontmatter (the existing sniff test, no new check) is refused
  there, raw working material stays allowed (it is what the folders are for). Rejected: leaving the
  four folders as they are · reversing to the plugin data directory (would not change the four).
- **V3 (blocking, lead-repaired):** a worktree's own `.mochiko/runs/` is not a home — the run folder
  is always the main tree's, printed by absolute path; a worktree-side write is refused with that
  path (D4).
- **V4 (blocking, lead-repaired):** the ledger amendment's scope gains GI-019's "What the gate
  reads" paragraph (the gate now reads `.gitignore` and walks ancestors for `.git`); version class
  stated as proposed PATCH with the MINOR reading named, the call the amend run's (S8).
- **V5–V12 (nits, lead-repaired):** amendment pointers on the D4/D6/D7/D9 headings (V5); F3 and F8
  annotated (V6); the `runs/` content rule names the report sniff only, the ephemeral log allowed by
  name (V7); OQ2 re-pathed (V8); OQ1 carries S6's census obligations and the over-budget-entry
  growth deny is stated (V9); build surface renumbered to waves 1–4 (V10); "re-ruled on its own
  line" withdrawn — it rode the batch (V11); the `SessionStart` line lists every `runs/*` folder,
  no state (V12).
- **Delta-check (`reports/review.md` `delta_check`): CLEAN on blocking — V1–V12 closed; 7 nits,
  lead-folded, no further round (reviewer's final status: `ready` once folded).** N1 the shell carve
  and content rule stated in D5, the D4 note and the build surface · **N2 (new, tightening inside
  the ruling):** the carve admits non-`.md` targets only, so a heredoc cannot smuggle a report past
  control 4 · N3 "log-shaped" withdrawn — the sniff's existing report-frontmatter test is the whole
  check, and its extension into the four sub-directories is on the build surface · **N4 (new):** an
  `impl.cold-verification` snapshot has no `.git`, so it is a nested non-home tree the gate does not
  police; its seat writes to the main tree's `runs/` and `reports/` by absolute path from its brief;
  the worktree `.git` pointer read joins the "What the gate reads" list · N5 the wave-3 line names
  the `/mochiko:setup` amend run, the third paragraph and the version-class note · N6 the D9 block's
  "re-ruled on its own line" withdrawn · N7 the Status line updated.

## Build surface (cold-buildable, by wave — see D9d, as amended at review)

1. Crate (wave 1): home resolution against the tree root (nearest `.git` dir or worktree file; only
   `<root>/.mochiko/` a home tree; nested fixtures never) · `home` resolves a directory to the home
   it names · closed-world verdict inside `.mochiko/` · the `runs/<run-id>/` home: run-key name
   check, ignore-guard (write refused unless `.gitignore` carries `.mochiko/runs/`), the report
   sniff on `.md` (the existing test) extended into `runs/` and the four declared sub-directories
   (`prototype/`, `inputs/`, `research/`, `referents/`), shape/size unchecked, main-tree path
   printed (worktree `.git` pointer read) · shell parse by write position (`>`/`>>`/heredoc · `tee`
   · `cp`/`install` last arg · `mv` both ends · `sed -i`/`perl -i` file arg · PowerShell cmdlets)
   **with the one carve: non-`.md` targets under `runs/<run-id>/` allowed** · per-entry budget
   kind (entry heading level + `max_lines`) · direct dry-run form `check --path --content -` · deny
   and `home` texts naming the run folder's absolute path · matrices from the 13 real false
   positives, 8 true positives, 6 resolution shapes, 7 render targets · **publish, gated on the
   signed-tag and crates.io approval controls (S10)**.
2. Migration (wave 2): `.mochiko/` census from the primitives' write sets, both trees as cross-check →
   declared sets incl. `archive/`, `runs/`, `schema-views/`, `strips/`; sub-directories without a
   home document stated allowed-by-location · epic `implement-log.md` withdrawn (files stand) ·
   store templates with entry headings (product baselines, feature-home entry-class files) · budget
   table re-keyed with per-entry numbers and the fold shape, `contracts/*` out of scope — one
   ratified artifact (OQ1).
3. Prose + governance (wave 3): `impl.artifact-home` (log → run folder; standing in-run rulings →
   `.mochiko/decisions/`; no row per checkpoint), `brainstorm.artifact-home` and the five other
   render targets, `testing-end-user` scratch path → `.mochiko/runs/<run-id>/` + the dry-run rule,
   the `SubagentStart` line, the `evidence/` deny reason, the run-close deletion step (OQ5) · strips
   + audits + bump · **ledger amendment through a `/mochiko:setup` amend run** (S8): GI-019's
   "Reach of the gate", amnesty, and "What the gate reads" paragraphs re-worded (the gate now reads
   `.gitignore`, walks ancestors for `.git`, and reads a worktree's `.git` pointer), the C1 discharge
   annotated with the field result, the 0.109.0 exception entry (S10); version class proposed
   PATCH, MINOR if the amend run reads the added reads as "significantly expanded" — the user's
   call there.
4. Kinako (wave 4): full-inventory violator pass (evidence trees deleted, legacy directories declared or
   moved, 47 pointers dispositioned) · watch re-measured with the displacement metrics and trigger.
   Bookkeeping: the hook-build backlog item's item 3 re-pointed here (its "not released" line kept);
   the 2026-09-23 audit item's gaps 2 and 4 annotated superseded.

## Open questions

- **OQ1 — Entry grammar and budgets for the cumulative stores (D2).** No product baseline has a
  template today (`mochiko-cli home` lists them "no template"); each store needs a declared entry
  heading (e.g. `###` per entity) before a per-entry budget can be counted, and the numbers need a
  census like the prior D6 table. Which feature-home files are entry-class too (`baseline-delta.md`
  at 2,314 lines accumulates across runs; `constraints-and-decisions.md` in the feature home) is
  ruled at the same table. *(S6 census obligations, added at verify V9:)* the table also fixes the
  entry heading per store, sizes each entry budget from the largest honest entry observed (177
  lines), rules the fold shape (dated fold block appended vs in-place `[EXTEND]`), and names the
  split-into-per-entry-files road as the fallback for any entry class one budget cannot fit.
- **OQ2 — Run-id form for `.mochiko/runs/<run-id>/` (D4 as amended; control 2's name check keys on
  it).** `FEAT-001-run4`-style vs a key
  the run mints at run-open; and who creates the folder (the lead at run-open is the natural owner).
- **OQ3 — The `archive/` set (D3/D9b).** What `.mochiko/archive/` legitimately holds in a consumer
  repo (the backlog trail, brainstorm archives, groom snapshots) is a census question; the KM
  module's "compress-and-move, never delete" needs a declared destination under the closed world.
- **OQ4 — Header cap numbers (D6).** *Closed at review (S5, user-ruled): the header cap is dropped;
  honest headers (review median 256, max 544) overlap the log, and S1's routing removes the log's
  reason to hide in a report.*
- **OQ5 — Run-folder deletion is procedural (D4 as amended, control 5).** A stateless hook cannot
  expire files; `mochiko-cli run close <id>` or the lead's own removal after acceptance, with a
  `SessionStart` reminder line, is the shape — whether the landing rule or the CLI owns it is a
  wave-3 detail.

## Question trail (Q)

- **Q1** — which problem first: sprawl (A) · gate blocking legitimate work (B) · both, as one redesign
  of home classes (C, recommended). → **C** ("yes both") → D1.
- **Q2** — size limits on cumulative stores: none (A) · per entry, file free (B, recommended) ·
  file cap with a landing carve (C). → **B** ("as recommended") → D2.
- **Q3** — where raw evidence goes: declared folder in the feature (A) · one bundled evidence report
  per cycle (B) · out of the repo, reports quote decisive lines (C, recommended). → user asked why the
  evidence is generated at all and whether it can be ephemeral → answered from F11/F12; Q3 re-put as
  where a run-scoped scratch lives (A git-ignored run folder in the repo, recommended · B system
  temp by run · C per-session scratch with carry-forward) → user redirected: "if it was supposed to
  be ephemeral and the copying pattern caused it, harden the hooks and cli" → Q4.
- **Q4** — closing the door: closed world under `.mochiko/` (A, recommended) · A plus no raw dumps
  anywhere under `.mochiko/` (B) · narrow `evidence/` deny only (C). → **A** ("yes A") → D3.
- **Q5** — the run-scratch route the deny names: git-ignored run folder in the repo (A, recommended)
  · system temp by run (B) · per-session scratch with carry-forward (C). → **A** ("yes A") → D4.
- **Q6** — shell-leg parse: write-position only (A, recommended) · deny clear writes, warn on unclear
  (B) · keep as is (C). → **A** ("as recommended") → D5.
- **Q7** — the run log: declare in the feature home + cap report headers (A) · declare only (B) ·
  reword only (C) → user asked why a run log is required and whether it can be ephemeral → answered
  from F16 → re-put: ephemeral in the run folder + header cap (A, recommended) · durable, declared
  (B) · ephemeral, headers uncapped (C). → **A** ("as recommended") → D6.
- **Q8** — home resolution: by the `.mochiko/` segment (A, recommended) · by the git root (B) · brief
  seats never to `cd` (C). → **A** ("yes A") → D7.
- **Q9** — the 15-line narrative budgets after the D10 trigger fired: keep 15 + standard dry run (A,
  recommended) · re-key to 20 (B) · warn not deny (C). → **A** ("as recommended") → D8.
- **Q10** — wrap-up batch: phase stays procedural · kinako clean-up · no governance amend · four-wave
  build, each with its recommendation. → **"as recommended"** → D9.
