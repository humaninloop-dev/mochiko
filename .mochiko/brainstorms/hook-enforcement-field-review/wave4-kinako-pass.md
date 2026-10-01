# Wave 4 — the kinako pass (joint hook/delta build)

The consumer-side wave of the joint build. It executes these rulings in kinako's tree on the
upgraded plugin (0.116.0, `mochiko-cli` 0.3.0):
- the field review's D9(b) as amended (S11, S14);
- the field review's D9(d) wave 4 and its S15 watch;
- the delta record's D5 (i)–(v);
- the census ratification's items 2–4;
- seams R3 and R4.

Mochiko's own records land on a branch cut from `main` after PR #40. Kinako's work lands on its own
branch, which merges to kinako `main` on the user's word.

**Amended 2026-09-30, after K0's inventory:** the steps below also carry user rulings W1–W4 and the
lead's readings L1–L11, recorded at `.mochiko/decisions/2026-09-30-wave4-kinako-pass-rulings.md`.
- K3 keeps the fold blocks' notes as dated lines, and also re-homes the three EPIC-002 contract fold
  blocks.
- K1 applies the A3 count as it stands today, and places A3/A4 entry by entry.
- Before K2, the D-050…D-061 write-ups move into the product file.
- K4 covers every link class. It re-points line cites only in live files and leaves the Rust
  comments.
- The ledger move is the user's one script.
- Then W5 and L18, at K2's stop. FEAT-006's copies go in their own step, K2b, before K3. First, what
  the product names them as the home of (in-force rationale, IP-001…005, `CorpusLocator`, the locator
  format) moves into the product entry by entry.

## 0. Preconditions, in order

1. **PR #40 merged** (lead, on CI green). `main` then carries plugin 0.116.0 and crate 0.3.0.
2. **Binary reinstalled by the user** (seam R4: the maintainer installs, from `main`). Run
   `! cargo install --git https://github.com/humaninloop-dev/mochiko mochiko-cli`, then check that
   `mochiko-cli --version` reads `0.3.0 · grammar 1..2`.
3. **Plugin updated by the user at both scopes it is installed at.** Today the user scope holds
   0.112.0 and kinako's project scope holds 0.114.0. A third entry, 0.110.0, belongs to the removed
   `kinako/.claude/worktrees/mochiko-run3` worktree and can go. Refresh the marketplace, then update
   both scopes to 0.116.0.
4. **The running session's gate is rebound to 0.116.0.** `/reload-plugins --force` rebinds the
   hooks without a restart. On 2026-09-30 a live probe showed it: in a scratch git tree, a Write
   into `.mochiko/runs/FEAT-000-run1/` was allowed, which the 0.112.0 log denies and the 0.116.0 log
   allows, and a Write of an undeclared `.mochiko/` file was denied with 0.3.0's run-folder route in
   the deny text. The lead repeats that probe after any further plugin change, before any kinako
   write. Kinako's project-scope install still gets updated in step 3, for kinako's own sessions.
5. **Seam R3, first and read-only:** no implement run is open in kinako. The evidence is:
   - no `.mochiko/runs/` folder;
   - no feature or epic whose run opened and never landed. That means no sufficiency report without
     its final validation, and no map row still reading in-flight for a run. K0 defines the exact
     reads.

   An open run is a stop, and it goes to the user.

## 1. Where kinako's work happens

- **Base:** kinako `origin/main` (`44635f1` at planning). The checkout the user works in
  (`console-consistency-sweep`, with an uncommitted `.gitignore` line) holds no `.mochiko` commit
  that `main` lacks. The pass leaves that checkout untouched.
- **Tree:** a new worktree at `kinako/.claude/worktrees/w4-cleanup` on branch
  `mochiko-0.116-cleanup`. The 0.3.0 gate resolves homes against a worktree's own root, because it
  reads the `.git` pointer.
- **Git commands:** every git command is `git -C <path>`, run from scratchpad scripts. Worktree
  sessions refuse computed inline git commands.
- **One writer at a time** in the kinako tree. Every seat runs plan-only first. A fresh peer grades
  the plan, the lead gives GO, there is one fix round, and a second FAIL goes to the user.

## 2. Seats and steps

The steps run in D5's order, because (i) must land before (iii) deletes the copies it reads.

**K0 — inventory and op probe** (read-only; a fresh `tech-lead`, graded by a fresh peer):
- A full inventory of kinako's `.mochiko/` against the 0.116.0 homes (`mochiko-cli home`, the
  closed world). It re-counts every figure the records carry:
  - the evidence trees — FEAT-001's 68 under `reports/evidence/`, the 166 under
    `archive/evidence/FEAT-001-run4/`, FEAT-002's 93;
  - the legacy directories;
  - the nine archive-root and snapshot files;
  - the 47 evidence pointers in 24 files;
  - the 63 `.mochiko` files naming a ledger;
  - the run-4 fold text's §A;
  - the fold blocks at product `constraints-and-decisions.md` `:371`/`:547` and `data-model.md`
    `:907`/`:1080`;
  - the 17 `####` headings.
- The per-step move list, with each target named.
- **The op probe:** every write shape the pass needs, dry-run against the live 0.3.0 gate with the
  direct form (`check --path <p> --content -`) and a hook-json Bash payload for each shell shape
  (`git rm`, `git mv`, `rm -r`). Each shape gets allow or deny with its text. §3 routes the denied
  ones.
- The R3 evidence reads.

**K1 — delta D5(i): run 4's fold applied in place:**
- `reports/landing-fold-text-run4-2026-09-23.md` §A, graded PASS at revision 1, goes entry by entry
  into the sections it extends.
- The product high-water becomes `C-011` / `D-061`, so the ids the map entry and the fold text
  already cite resolve.
- No `Lifecycle:` marker is added; the entries read `built`, as delta verify N5 rules.
- Writes use Edit on the product baselines. Each entry stays within its 177-line budget, the watch on
  delta OQ1.

**K2 — delta D5(ii)–(iii) and field D9(b)/S11:**
- The three `baseline-delta.md` ledgers (FEAT-001, FEAT-002, FEAT-006; 8,068 lines) move unchanged
  into their declared archive home (migration 0032).
- The feature-directory baseline copies are deleted: `data-model.md`, `constraints-and-decisions.md`
  and `contracts/`. FEAT-001's go because runs 3 and 4 are folded; FEAT-002's and FEAT-006's on their
  epic landings (`77ea3fb`, `ac21ce8`).
- The evidence trees are deleted: 68, 166 and 93 files, which history keeps.
- These stay in place as closed record, per census item 2 (never written): `features/B53`,
  `features/B61` and `features/FEAT-006/reviews`.
- These stay in place per census item 3 (never moved): the archive root's four groom snapshots and
  `spine-groom.md`.
- `EPIC-001` and `EPIC-002` are closed record (delta I9) and are not touched.

**K3 — delta D5(iv) and census item 4:**
- The two fold blocks in each of the two product files move entry by entry into the sections they
  extend, and the `## The … landing fold` headings go.
- The 17 `####` constraint and decision headings are raised to `###`, `D-025` and `D-037` included.
- The move is mechanical: every entry present once, text unchanged, nothing else moved.

**K4 — delta D5(v) and field S11's pointers:**
- Every link to a moved or deleted path is re-pointed, with no stub left at an old name.
- `BD-` ids stay valid as citations into the archived ledgers.
- Each of the 47 evidence pointers is annotated "in history at `<commit>`" or repaired to a live path.

**Dk — the non-author diff read after each step:**
- A fresh seat reads `git diff` for that step alone, against the step's source (K1 against the fold
  text's §A; K3 against the pre-move blocks), and grades it before the next step opens.
- This is delta D5's "each write graded by a non-author seat reading the diff".

**K5 — close checks** (lead):
- `git -C <worktree> grep` finds `baseline-delta.md` only in the archive and in `BD-` citations.
- A dead-pointer scan over kinako's `.mochiko/` comes back clean.
- Every figure from K0 reconciles against the diffs.
- A dry-run of one no-op write per touched home returns allow under 0.3.0.
- `.mochiko/runs/` is absent.

## 3. Writes the gate refuses

Moves out of a home, deletions and shell writes into homes may be refused by the 0.3.0 gate
(field review S12: `mv <home> <elsewhere>` stays a deny). A seat never routes around a deny, and the
user's `!` shell is the only recorded break-glass (field review S16(i)).

For each denied shape from K0's probe, the lead writes one reviewed script in scratch. It lists every
path it deletes or moves, and a fresh seat grades it against K0's move list. The user runs it once
with `!`, and its output is kept. Content writes (K1, K3, K4, the archive copies) go through
Write/Edit under the gate, as usual.

## 4. Landing

**Kinako:**
- One commit per step on `mochiko-0.116-cleanup`.
- Kinako's own `BACKLOG.md` consumer items for this pass are closed, following kinako's own ritual.
- A PR to kinako `main`; the user merges.

**Mochiko:**
- A build-log entry per event.
- `BACKLOG.md`:
  - the field-review and delta build items go to the trail as DONE;
  - a new **watch item** records the D10 first-live-run watch, re-measured on kinako's next implement
    run with S15's displacement metrics and trigger. The watch is not part of this pass.
- The `DECISIONS.md` statuses drop "wave 4 owed", and the brainstorms index and the `ROADMAP.md` row
  are updated to match.
- No `plugin.json` bump unless the pass finds a primitive defect, which would go to a separate bump.

## 5. Stops

The pass stops and goes to the user when any of these happens:
- the R3 check finds an open run;
- a SessionStart line does not read 0.3.0 and 0.116.0;
- a K0 figure disagrees with a record's figure by more than a recount explains;
- a fold entry exceeds its 177-line budget;
- a second plan FAIL;
- a second gate deny on one path.
