<!--
MODULE: knowledge-management
============================
Attach when: the knowledge-management dimension (dimension 7) elicited adoption of the
operating-docs layer. Offered DEFAULT-ON in every mode — the user must actively decline.
Adopted as CORE + ELECTIVES: the core bundle is adopted or declined WHOLE (a project for
which the core feels heavy declines the module, not a fragment); the electives
(`CHANGELOG.md`, `RUNBOOK.md`) are per-doc opt-in, elicited by project type. The ruling is
recorded in the synthesis either way; a recorded decline is durable — amend runs never
re-offer a ruled module.
(Redesign ruled 2026-07-25 — deliberately supersedes the prior four-part "no inner menu" bundle rule.)
Trace: the GI module-selection element that names `knowledge-management`.

AUTHORING SOURCE ONLY: this template is the module's single authoring-time source. At
scaffold, setup writes a PROJECT-PINNED copy of the Document-contracts + Landing-ritual +
Invariants sections to
`.mochiko/memory/knowledge-management.md`; command landing steps and the groom skill resolve
against the project copy at runtime, never against this file. A plugin upgrade that changes
this template reaches pinned projects only as an amend OFFER through the ledger/version-bump
machinery — never silent enforcement of unratified governance.

DISAMBIGUATION: nothing in this module is `.mochiko/memory/evolution-roadmap.md` (the
brownfield improvement plan, produced by the unported roadmap cluster and referenced by the
`evolution-notes` module). `ROADMAP.md` here is the THIN FORWARD VIEW (four pieces, one
screen) — NEVER a decision archive (the fat-roadmap failure this module was redesigned to
kill); durable rulings live in `DECISIONS.md` + records.
-->

## Knowledge Management

The operating-docs layer: where design sessions, open threads, durable decisions, the living
system view, and the project's forward direction live — and the carriers that keep each doc
fit in active use. **Admission rule:** a doc enters this module only with a named
**read-job**, a **writer moment**, and a **carrier** — no carrier, no scaffold.

**Core** (adopted or declined whole):

| Artifact | Read-job | Writer moment · carrier |
|----------|----------|-------------------------|
| `.mochiko/brainstorms/<topic-slug>/` + `index.md` | which session records are current, superseded, un-reviewed | session open/close · brainstorm command steps + the invariants |
| `BACKLOG.md` (repo root) | the **complete open-set detail store** — scannably complete, never curated; bounded entries with resume-cold context in theme-keyed sections | item opened/closed · landing ritual + groom |
| `.mochiko/archive/backlog-trail.md` | resume-cold on a reopened item + provenance lookup | item close (append-only) · landing ritual |
| `ROADMAP.md` (repo root) | the one-glance view of current work + future direction — the sole **curated** scan surface | landings + groom · horizon caps |
| `DECISIONS.md` (repo root) + `.mochiko/decisions/` | which decisions rule, which are superseded, where each rationale lives | ruling landed · landing ritual |
| `ARCHITECTURE.md` (repo root) | the living system view — components, boundaries, data flow; decisions record *changes*, this records the *resulting system* | plan/implement landings on structural change · `mochiko:authoring-architecture` |
| `GLOSSARY.md` (repo root) | the project's domain language | spec landing when new terms mint · the term format below |

**Plus the enforcement surfaces — core, not separately declinable** (documents never scaffold
without their carriers): the **project-pinned copy** at
`.mochiko/memory/knowledge-management.md`, a **`paths`-scoped rules file**
(`.claude/rules/mochiko/operating-docs.md`, scoped over the docs above) injecting the shape
contracts for ad-hoc edits — touch-time edit quality only; rules are structurally blind to
omission, so omission-class drift is caught **only** at the boundary invariants — and
**CLAUDE.md pointers** (doc pointers + the groom-skill pointer; never again a sole carrier).
Rules-file delivery is probe-verified via `mochiko:testing-governance-injection`, never
assumed.

**Electives** (per-doc opt-in at setup, elicited by project type): `CHANGELOG.md`
(release-shaped projects), `RUNBOOK.md` (deployed services). A recorded decline is durable.

### Document contracts

**`ROADMAP.md` — four pieces, one screen, nothing else:** (1) **Thesis** — 2–3 lines on
what the project is becoming and the core bet; (2) **Now / Next / Later** — one line per
item, each linked to its BACKLOG item or session record (*Later* is non-committed and exempt
from the link rule until promoted to *Next*); (3) **Standing bets & revisit conditions** —
the few strategic contested/provisional bets, each with the condition that would reopen it;
(4) **nothing else** — decision rows, trails, and rationale prose live in the decisions
layer. Dates on Now/Next items and bets; a last-groomed stamp line carries the groom date +
baseline figures.

**`BACKLOG.md` — open items only:** one bounded entry per item (title, date, provenance
pointer, resume-cold context) in theme-keyed sections that merge on groom — never
provenance-keyed-forever. Closing an item never deletes it: it compresses to the one-line
DONE + pointer form and **moves to the trail**. Dead provenance gets the terminal stamp
`provenance: unrecoverable (<what it was>, removed <date>)` — satisfying the pointer
requirement and the dead-pointer scan at once.

**Decisions layer — a thin index over records:** one `DECISIONS.md` line per **ruled
decision** — date · title · status (`ruled` / `superseded by <pointer>`) · pointer to the
rationale home: a session record where one exists, else a per-decision record at
`.mochiko/decisions/<YYYY-MM-DD>-<slug>.md`. Record schema (record-less decisions only):
**Status · Date · Context · Decision · Rationale · Alternatives considered**. At groom, a
superseded row compresses to one line **per superseded decision, status preserved** —
retirement without deletion.

**`GLOSSARY.md` term format:** `**<term>** — <definition> *(minted <date>, <source>)*`, one
line per term.

### Landing ritual (subtractive — enforced floor, with the invariants)

Closing **or superseding** work is ONE move with three parts, executed in the same moment at
the command landing step (brainstorm close · setup/amend · specify/plan/implement landings
where those commands run):

1. append the decision row (and any per-decision record);
2. move the closed `BACKLOG.md` item to the trail as its one-line DONE + pointer;
3. touch `ROADMAP.md` Now/Next — and, on supersession, update **both** indexes
   (brainstorms + decisions) so statuses agree.

A landing that only adds is incomplete: subtraction is part of the same move, not a later
groom's job.

### Invariants (run at command boundaries under fix-on-sight; project-pinned at scaffold)

- **Bijection:** every directory under `.mochiko/brainstorms/` has an `index.md` entry;
  every accepted entry names its landing (a `DECISIONS.md` row, or an explicit
  no-graduation).
- **Status-agreement:** wherever the same decision appears, brainstorms-index status ↔
  record `Status` line ↔ decisions-index status agree.
- **Open-only:** no `[x]` item in `BACKLOG.md` — done items live in the trail.
- **Horizon caps:** `ROADMAP.md` Now ≤ 5 · Next ≤ 7 · Later ≤ 10; every *Now* item points
  at live work.
- **Item bounds:** per-open-item size bound (default ≤ 15 lines) + an open-item-count watch
  against the last-groomed baseline (the baseline figures live on the stamp line).
- **Dead-pointer scan:** every `ROADMAP.md` / `DECISIONS.md` / `BACKLOG.md` pointer
  resolves, or carries the `provenance: unrecoverable` terminal stamp.
- **Presence:** all core artifacts exist (electives only when adopted).
- Vacuously satisfied at zero sessions / zero items — adopting the module never obligates
  running sessions.

A tripped cap or bound invokes `mochiko:grooming-operating-docs` on sight (theme-section
merges, Next/Later re-ranks, supersession compression, staleness stamps, the
expansion-heavy-surface watch); the groom attaches to already-firing boundaries, never to
user initiative. Prose quality beyond these invariants is groom territory, not mechanical
enforcement.

**Rationale**: every operating doc in the redesign's evidence base that failed, failed the
same way — write obligations with no subtractive obligation anywhere, and prose-only
carriers that never fired. The one healthy layer (the brainstorms index) had both structural
view/archive separation and command-boundary invariants. So: view and archive are separate
artifacts by design, subtraction is part of the landing move itself, and every carrier here
is either an executable command step or a probe-verified injection — never prose alone. The
living read surfaces sit at repo root because they are for humans (and renderers);
`.mochiko/` holds machinery, records, and archives. Uniform layout preserves cross-project
familiarity.

### Scaffolding & collisions

Setup scaffolds the adopted set at finalize on a hard **never-overwrite floor**: existing
content is never overwritten or renamed — explicitly covering the repo-root writes this
module introduces. On brownfield, an existing doc whose semantics fit is **codified** into
the module role by default; a true semantic collision (name taken, different meaning — e.g.
a product roadmap squatting a module name) was ruled by the user in-session — adopt the
existing doc into the role, or nest the module's artifact under `.mochiko/`.

**Collision rulings:** [none — clean names | e.g. "`ROADMAP.md` is a product feature
roadmap; the module's forward view lives at `.mochiko/ROADMAP.md`" — from the synthesis,
GI-XXX]

**Trace**: GI-XXX (module selection)

<!-- ── Validator checklist fragment (checked only when this module is attached) ──
- [ ] Core artifacts named with read-job, writer moment, and carrier (the admission rule holds for every scaffolded doc); electives present only when adopted, declines recorded
- [ ] All three enforcement surfaces present (project-pinned copy at `.mochiko/memory/knowledge-management.md` · `paths`-scoped rules file · CLAUDE.md pointers) and the five carrying commands' landing steps reference the PROJECT copy, not this template
- [ ] Landing ritual stated as one three-part move covering closing AND supersession
- [ ] Invariants stated mechanically (bijection · status-agreement · open-only BACKLOG · horizon caps Now ≤5 / Next ≤7 / Later ≤10 · item bounds + count watch · dead-pointer scan with the `provenance: unrecoverable` terminal state · presence) with the vacuous-at-zero note
- [ ] Decision-record schema + glossary term format present; superseded rows compress one line per decision, status preserved
- [ ] Disambiguation present (`ROADMAP.md` = the thin forward view, never a decision archive; `DECISIONS.md` = the thin index over records; `evolution-roadmap.md` unrelated)
- [ ] Never-overwrite floor stated; collision rulings recorded (or "none — clean names") and matching the synthesis
- [ ] Re-audit (run against the repo, not the document): the invariants above pass (vacuously PASS at zero sessions/items)
-->
