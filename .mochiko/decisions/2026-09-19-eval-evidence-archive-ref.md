# Eval run evidence moves to an immutable archive ref

- **Status:** ruled
- **Date:** 2026-09-19
- **Context:** Every eval kit's `.gitignore` ignores its `runs/` directory, because run output is
  regenerated locally on each grid. The evidence was nonetheless carried on the working branch, by
  force-adding each file past that rule. The cost came due on 2026-09-19: two commits staged
  updates to already-tracked run files and silently dropped every file their grids had created for
  the first time — twenty-two sessions across the `validation-constitution` and
  `review-plan-artifacts` kits, including every session of their third goldens. The repair
  (`ee9545e`) force-added the missing files and recorded the mechanism, but left the mechanism in
  place. Separately, the `primitive-evals-v2` branch had grown to 2,190 files and 222,314
  insertions, of which 757 files and 151,336 insertions were run evidence — 68 % of the diff, in a
  pile no reviewer reads line by line. A third defect was found while measuring: the `**Evidence:**`
  pointer at `.mochiko/strips/review-brainstorm.md:268` names
  `evals/review-brainstorm/runs/20260826-110222/`, which was never committed at all. That pointer
  resolved only on the machine that ran the grid, which is a dead pointer under GI-005 for every
  other reader.
- **Decision:**
  1. **Run evidence lives on an archive ref, not the working branch.** The raw sessions behind a
     published read are committed as a parentless (orphan) commit and tagged
     `eval-evidence-<date>` — `eval-evidence-2026-09-19` for this first archive: 795 evidence files
     plus a README carrying provenance and resolution instructions.
  2. **A tag, not a branch.** The branch `eval-evidence` exists for browsing, but citations name
     the tag. Tags are immutable and survive branch cleanup; a pointer that can be swept away is
     the defect this ruling exists to fix.
  3. **The ignore rule now stands rather than being worked around.** The 757 tracked evidence files
     are removed from the index (`git rm --cached`) and left on disk, where `evals/.gitignore`
     already covers them. No future grid needs `git add -f`, so the silent-drop failure cannot
     recur.
  4. **Citations are re-keyed, never left bare.** Each kit's `preregistration.md` carries an
     `**Evidence.**` block at its head binding its kit-relative `runs/…` citations to the tag; the
     four persona records under `.mochiko/decisions/` carry the same pointer; `evals/README.md`
     gains a "Where the run evidence lives" section.
  5. **Archives supersede, never replace.** A later grid's evidence lands under a new
     `eval-evidence-<date>` tag. Existing citations keep resolving against the tag they name.
  6. **Scope is the evidence this branch adds.** The 171 run files already on `main` under
     `evals/commands/…` are untouched — moving them would turn a quiet rename into a loud deletion
     and would remove from `main` content `main` already carries.
  7. **The review-brainstorm grids join the archive.** All 38 previously untracked files under
     `evals/review-brainstorm/runs/` are carried, which repairs the strips pointer rather than
     merely documenting it.
- **Rationale:** The evidence earns its keep — `ee9545e` argued correctly that the raw sessions are
  "the part a later reader needs to check a verdict rather than take it on faith," and nothing here
  contradicts that. What fails is the *carrier*. Keeping regenerable output on a branch whose own
  ignore rule excludes it requires a manual override at every commit, and a discipline that must be
  remembered every time is a discipline that will be forgotten once — as it already was. An
  immutable ref keeps every verdict auditable while letting the ignore rule mean what it says, and
  makes the working branch reviewable as a side effect rather than as the goal. The
  `review-brainstorm` case shows the upgrade is not merely neutral: a pointer that resolved on one
  machine now resolves for everyone.
- **Alternatives considered:**
  - **Leave it as it is, but un-ignore `runs/`.** Honest, and it kills the force-add foot-gun, but
    it keeps 151,336 insertions of machine output on the reviewed branch permanently and grows with
    every grid. Rejected: it fixes the trap and not the weight.
  - **Drop the raw per-session artifacts, keep `report.md` and `summary.json`.** Saves most of the
    weight for 16 fewer files of work. Rejected: it contradicts `ee9545e` on the merits — the
    per-session `.plan.md`, `.artifact.txt` and `.result.md` files are exactly what makes a verdict
    checkable rather than trusted, and a read nobody can audit is a read nobody should rely on.
  - **A separate repository.** Cleanest separation, and the natural home if the evidence ever
    outgrows this one. Rejected for now: it needs its own creation, permissions, and lifecycle, and
    an orphan ref in the same repository gives the same resolution guarantee at none of that cost.
  - **Rewrite the branch history to drop the blobs entirely.** The only option that shrinks `.git`.
    Rejected: PR #36 is open and the branch is pushed, so a rewrite breaks every existing checkout
    and review link. A squash-merge keeps the blobs out of `main` without touching published
    history.
