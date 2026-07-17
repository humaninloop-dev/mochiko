<!--
MODULE: knowledge-management
============================
Attach when: the knowledge-management dimension (dimension 7) elicited adoption of the
operating-docs layer. Offered DEFAULT-ON in every mode — the user must actively decline — and
adopted or declined WHOLE: a fixed four-part bundle, no inner menu, no layout parameterization
(layout mirrors mochiko's own dogfooded shape). A project for which any part feels heavy
declines the module, not a fragment. The ruling is recorded in the synthesis either way; a
recorded decline is durable — amend runs never re-offer a ruled module.
Trace: the GI module-selection element that names `knowledge-management`.

DISAMBIGUATION: this module's repo-root `ROADMAP.md` (an operating doc — the project's durable
decision log) is NOT the per-project `.mochiko/memory/evolution-roadmap.md` (the brownfield
improvement plan, produced by the unported roadmap cluster and referenced by the
`evolution-notes` module). Same word, different artifact — a constitution may carry both.
-->

## Knowledge Management

The operating-docs layer: where design sessions, open threads, and durable rulings live, and how
they circulate. Adopted as **one four-part system**:

| Artifact | Role |
|----------|------|
| `.mochiko/brainstorms/<topic-slug>/` | One directory per brainstorm/design session; the session's `record.md` (and any derived `synthesis.md`) lives there — never at repo root |
| `.mochiko/brainstorms/index.md` | The session index — newest first, one entry per session: when, status (open / accepted / superseded), review state, what it's about, where the outcome landed. Carries the maintenance contract: read the index before opening any session; opening adds an entry; concluding updates it |
| `BACKLOG.md` (repo root) | Open threads: design decisions awaiting a ruling, scoped-but-unbuilt work. Not an issue tracker |
| `ROADMAP.md` (repo root) | The durable decision log: accepted session rulings graduate here as decision rows, each pointing back to its session record |

**Circulation is the point:** session rulings graduate to `ROADMAP.md`; open threads land in
`BACKLOG.md`; the index tracks every session's status. An index whose rulings cannot graduate is
a dead ledger. Note: `ROADMAP.md` here is the operating-doc decision log, **not**
`.mochiko/memory/evolution-roadmap.md` (the brownfield improvement plan) — same word, different
artifact.

**Enforcement** (three carriers, adopted as a chain):

- **Command bookkeeping** — session-producing commands (e.g. `/mochiko:brainstorm`) MUST carry
  the index contract as executable steps: read `index.md` before opening a session, add the
  entry on open, update its status at acceptance/supersession — and MUST run this module's
  invariants at session open/close, fixing violations on sight.
- **CLAUDE.md sync** — this module's rows in the Mandatory Sync Artifacts table keep the
  operating-manual text in the repo-root `CLAUDE.md`, in front of every agent session
  (stub-backed until the `syncing-claude-md` cluster ports).
- **Setup/amend re-audit** — the constitution validator re-checks the invariants whenever
  `/mochiko:setup` runs on this project.

**Testability** (mechanical invariants — run at session open/close, re-audited at setup/amend):

- Pass: every directory under `.mochiko/brainstorms/` has an `index.md` entry AND every index
  entry's status matches its record's Status line AND all four bundle artifacts exist AND every
  index entry marked accepted names where its outcome landed (a `ROADMAP.md` row, or an explicit
  no-graduation).
- Fail: a session directory without an index entry; an index status contradicting its record; a
  missing bundle artifact; an accepted entry with no named landing.
- The invariants are vacuously satisfied at zero sessions — adopting the module never obligates
  *running* sessions. Content **quality** of `BACKLOG.md`/`ROADMAP.md` is explicitly exempt from
  mechanical enforcement — that boundary is declared, not implied.

**Rationale**: the layer keeps knowledge management organized, and governance is what makes the
structure *enforced* rather than merely documented — a maintenance contract living only in prose
is a declaration without a carrier. The operating docs sit at repo root because they are for
humans (and renderers); `.mochiko/` is machinery. Uniform layout preserves cross-project
familiarity.

### Scaffolding & collisions

Setup scaffolds the adopted bundle at finalize on a hard **never-overwrite floor**: existing
content is never overwritten or renamed — explicitly covering the repo-root writes this module
introduces. On brownfield, an existing doc whose semantics fit is **codified** into the module
role by default; a true semantic collision (name taken, different meaning — e.g. a product
roadmap at `ROADMAP.md`) was ruled by the user in-session — adopt the existing doc into the
role, or nest the module's artifact under `.mochiko/`.

**Collision rulings:** [none — clean names | e.g. "`ROADMAP.md` is the product roadmap; the
module's decision log lives at `.mochiko/ROADMAP.md`" — from the synthesis, GI-XXX]

**Trace**: GI-XXX (module selection)

<!-- ── Validator checklist fragment (checked only when this module is attached) ──
- [ ] All four bundle parts named with their roles and the circulation rules (rulings graduate to ROADMAP · threads land in BACKLOG · index tracks session status)
- [ ] All three enforcement carriers present (command index-bookkeeping + open/close invariants · CLAUDE.md sync rows · setup/amend re-audit)
- [ ] Invariants stated mechanically (index↔session-dirs↔status coherence · four-artifact presence · accepted entries name their landing) with the content-quality exemption and the vacuous-at-zero-sessions note declared
- [ ] `ROADMAP.md` explicitly disambiguated from `.mochiko/memory/evolution-roadmap.md`
- [ ] Never-overwrite floor stated; collision rulings recorded (or "none — clean names") and matching the synthesis
- [ ] Sync-table rows for this module present in the CLAUDE.md Synchronization section
- [ ] Re-audit (the module's third carrier, run against the repo, not the document): every directory under `.mochiko/brainstorms/` has an index entry · index statuses match their records' Status lines · all four bundle artifacts exist · accepted entries name their landing (vacuously PASS at zero sessions)
-->
