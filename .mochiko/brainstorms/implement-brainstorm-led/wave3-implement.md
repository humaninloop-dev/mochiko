# Wave 3 — implement's design stage on the engine, and its architecture touches

**Opened:** 2026-10-11, after wave 2 closed (build log; committed `7d9d29e`) · **Lead:** the build
session lead · **Branch:** `engine-desk-build` · **Target version:** 0.120.0, the one bump after
this wave · **Records:** `.mochiko/brainstorms/implement-brainstorm-led/record.md` — D1, D3,
D5–D8 as review-amended, the *Defaults batch* (the *Implement — …* items, *Precedence*, *Slots
declared where*, *Change control*, *Sizing and skips*, *One provenance vocabulary*, *Greenfield
seed kept apart*), *Supersessions owed at the build*, *Touch set for the build* — and
`.mochiko/brainstorms/architecture-brainstorm-interview/record.md` — D5-visit-file-input (the
design-input line and the offer), D8-draw-before-code and D9-render-where-viewed (implement's
share), the S8 list of nine touches and the tenth stamped at V7. These are the source for every
rule text; this plan names carriers, never restates a decision.

Wave 3 is implement's design stage written straight onto wave 1's engine (implement keeps only
its domain part), the stage's home, the landing-table checkpoint, the ten architecture touches,
the three review-side touches (`review-plan-artifacts`' meaning-kept grade, `review-sufficiency`'s
re-grade of a user-added piece, `testing-gap-finding`'s first message), and the items wave 2
routed here.

## 0. Sound-loop wiring

As wave 2 §0, with a fresh re-plan counter (one, shared by this wave's producers; a second
consumption goes to the user). Plans go to one scratch file outside the tree, graded by path and
hash; returns kept short. A plan file is frozen once returned; a revision is a new file.

## 1. Sequence allocation (0075–0086; gaps legal)

- **`0075-design-stage-home.yaml`** — `template/design-stage` (`import-document`, the desk visit
  file's item shape: direction · seed · pieces · provenance, in the one provenance vocabulary);
  `home/feature`, `home/epic` and `home/product-lane` (`replace-document` each, gaining
  `design-stage.md` bound to that template, in each home's own bound form). The lane home is the
  user's ruling (2026-10-11, build log): a lane run that fires the stage writes its set there.
- **`0076-implement-design-stage.yaml`** — `command/implement`: the stage's moment, any condition
  it needs, and its domain rules across `roles` · `reserved` · `ways-of-working` · `boundaries` ·
  `fail-conditions` — never `impl.sec.tools`, never a seventh section.
- **`0077-implement-on-engine.yaml`** — `impl.thinking-stage`, the floor stub extending
  `common.thinking-stage` (the fourth extender; the shape of `0069-desk-on-engine.yaml`).
- **`0078-implement-architecture-touches.yaml`** — `command/implement`: the ten touches (§3.3),
  rewords of existing rules where a rule already carries the subject.
- **`0079-store-landing-diagrams.yaml`** — `skill/authoring-architecture-store` (§3.4).
- **`0080-plan-artifacts-meaning-kept.yaml`** — `skill/review-plan-artifacts` (§3.4).
- **`0081-sufficiency-user-added.yaml`** — `skill/review-sufficiency` (§3.4).
- **`0082-gap-finding-signed-set.yaml`** — `skill/testing-gap-finding` (§3.4).
- `0083`–`0086` held for a fix round or a split P1 proposes with its reason.

Header anchors: `2026-10-11 implement-brainstorm-led` on the stage, its home, the checkpoint and
the review-side touches; `2026-10-11 architecture-brainstorm-interview` on the ten touches and the
store skill. A rule carrying one ruled decision takes it as its rule-level anchor (`… D7`); a
confirmed default takes the bare session anchor. A reword or supersession of a `class: floor` rule
carries its anchor and names the superseded ruling — here `architecture-design-primitive`
D5-plan-run-artifact and D8-rendered-diagram-signoff, each in part.

## 2. Seats and ownership (disjoint)

| Seat | Owns | Depends on |
|---|---|---|
| **P1 schema** | migrations 0075–0082 (and any held number it uses) · `.mochiko/schema-views/**` (re-emit only) · `scripts/similar-rules-allowlist.yaml` (rows only for edges the sweep reports, each with its why) | wave 2 landed |
| **P2 prose** | `plugins/mochiko/commands/implement.md` (lines made false) · `SKILL.md` of `review-plan-artifacts` (and `references/ARTIFACT-CHECKLISTS.md`, `:80`/`:89` among the lines made false), `review-sufficiency`, `testing-gap-finding`, `authoring-architecture-store`, `patterns-system-design` (and its references) — lines made false only · the router `skills/mochiko/SKILL.md` (lines made false) · strips for every removal or supersession · ripple lines made false (`README.md`, `ARCHITECTURE.md`) · `.mochiko/memory/primitive-cost-budgets.md` · `CHANGELOG.md` (the `[lead: Wave 3 …]` slot only) | P1 landed |
| **P3 tests and kits** | crate frozen-census tests and fixtures under `crates/mochiko-cli/tests/` (figures and the new template's fixtures only) · `evals/contract/**` · `evals/plan/implement/**` | P1 landed |

Order: P1 plan → peer grade → approve → build → lead's read · then P2 and P3 in parallel, each plan
→ peer grade → approve → build · §4 audit. P3's build starts after P2 lands where a check reads
P2's surfaces (wave 2's order). No seat edits a persona (`plugins/mochiko/agents/**`): a persona
line this wave makes false goes to the lead, since a persona edit carries its own advisory grid.

## 3. Content

### 3.0 What every rule text must respect

- **Precedence.** The engine's asking rules hold; implement adds its domain part and fills its
  slots and never loosens a mechanic. A difference implement needs becomes a slot by ruling — a
  stop (§6), never a local rule.
- **The contract is on** in implement's own rule text (a stub inherits text only, so the mode
  never rides the shared line), as specify's story stage and the desk's focus stage name it.
- **The `contract-inside` latitude is granted by name.** The engine's floor lets a small technical
  extra no ruled item touches wait for a later sign-off only where the caller's own rules grant
  it; implement grants it for the landing table (D8's split), and nothing wider.
- **Section and size.** No new rule in `impl.sec.tools`; a reword there keeps its growth small,
  and its rendered block stays under 29,000 bytes (the 30,000-character `!`-output ceiling, with
  margin; BACKLOG records 25,469 at an earlier state, 26,200 B at this wave's open). P1 reports
  every block's bytes before and after.
- **Plain words, named carriers.** Every minted id has its why in P1's plan; every reword keeps its
  id and states what changed; nothing is minted where an existing rule carries the subject.

### 3.1 Migration 0075 — the stage's home (Defaults: *Implement — the set's home*; the user's lane ruling)

- `template/design-stage`: one item per ruled gap (and per epic spine item) — direction (the
  user's words) · seed ("I'd know it works if…", `open` allowed, carried as an open question) ·
  ruled pieces · defaults-batch rows with their suggestion and cost · provenance per item
  (`user's words` · `lead's guess, ratified` · `blind list` · `handed in`) · user-added gaps marked
  as such. It survives a stopped and resumed run. A `--check` form, as the store's templates have.
- `home/feature`, `home/epic`, `home/product-lane`: `design-stage.md` added; nothing else moves.

### 3.2 Migrations 0076 and 0077 — the stage (D5–D8 and the *Implement — …* defaults)

Carriers P1 proposes, ids and kinds its own (the names below are the lead's working labels):
- **The moment** — a `design-stage` moment: after the run-open confirmation, before any design
  seat drafts; the run-open stays one confirmation (D5). It fires only on gaps; on an epic it
  runs once over the joint gap list with the joint spine's items beside it (D5 as changed, S9).
- **The stage rule** (`roles`, a DM duty) — the stage runs on the engine with the contract on;
  the ruled set is the scope the design seats plan and draft inside (D5); zero-gap path: no
  stage, nothing changes.
- **The agenda and the outside need** (`reserved`) — the agenda is the graded gap list (D6); a
  need found outside it splits by kind: new behaviour never joins this run (to
  `/mochiko:specify`, `BACKLOG.md` or a feature-desk card); a design piece inside a story the spec
  carries is the user's *this run* or *later*, and if added the sufficiency seat re-grades it as a
  gap before it joins, marked user-added in its report; a lane run never adds
  (`impl.lane-never-widens`). Nothing outside the list is designed without that ruling.
- **Direction, seed, pieces** (`ways-of-working`) — per gap the user's direction and seed first;
  the owning design seat (technical-analyst for data and contract gaps, principal-architect for
  structural ones) proposes candidate pieces read-only, riding its plan-only dispatch, writing
  nothing; the lead puts them through the engine — own turn for a real or costly choice, put as
  what the user would notice with a worked example, the rest in the defaults batch (D7). The
  seat's plan is finished inside the ruled set once the stage closes, peer-graded as today
  (`impl.plan-approval-producers`).
- **The blind list** — once per design phase, a fresh persona-less seat on `model: opus`, given
  the spec, the gap list, the store index and any visit file, never the ruled set or the
  conversation; each difference is a candidate piece; skipped on a single-gap re-fire. Implement's
  cadence for the engine's `blind-second-list` slot.
- **Handed-in sets** — a desk visit file keyed to the feature, the spec's slate and Intent, and a
  named brainstorm record open the stage as facts, provenance `handed in`, never re-asked; a visit
  file covering some gaps leaves the stage to run on the rest (Defaults; carries architecture D5's
  design-input line at the stage — §3.3 item 1 rewords `impl.design-inputs` to match).
- **The desk offer** (architecture D5's carried default) — a structural gap with no visit file:
  the stage offers a desk visit first (`/mochiko:architecture <FEAT-ID>`, the run pausing) or
  proceeds; the user's choice at run time.
- **The set's file** (`ways-of-working`, a binding beside `impl.artifact-home`) — the lead
  transcribes the ruled set into `design-stage.md` at the run's home (feature · epic · lane) before
  the pen passes (the engine's `contract-pen`).
- **The seat addition split and the latitude** (`reserved` or `boundaries`) — D8 as changed (S2):
  a small technical extra no ruled item or default touches waits for the landing table; anything
  that changes a ruling or that the user would notice halts to the user through the lead before
  it is written. This is the by-name grant of `contract-inside`'s latitude, and only this.
- **Mid-run re-fire** — undesigned structure that halts a cycle re-fires a short stage scoped to
  the discovery, interrupting (the named exception to `impl.escalation-batching`), then the same
  review and landing-table sign-off. Carried by rewording `impl.midrun-refire` (a `tools` reword,
  small) or by a new rule outside `tools` — P1's call, with the bytes.
- **The checkpoint** — `impl.gate-design-checkpoint` (floor, `reserved`) reworded, its force kept:
  after a ruled stage it is asked as the landing table — each ruled item beside how it landed
  (kept, or changed with why) and every small extra needing the user's OK, filled from
  `review-plan-artifacts`' meaning-kept grade; the signed object per architecture D8 (the run's
  `architecture.md`, drawn from and listing the delta-styled store diagram files it edited, which
  signing it signs); D9's wording: Mermaid text that parses, rendered where the session can, else
  shown with the one-line degrade note (W2 carry-forward). Zero-gap path unchanged. The anchors
  name D8-checkpoint-shows-landing, architecture D8 and D9, and the superseded
  `architecture-design-primitive` D8-rendered-diagram-signoff (in part).
- **The fail** (`fail-conditions`, `kind: fail`, floor) — a design-phase write with no ruled set
  behind it, a write that changes a ruling without the user's word, or a small extra the user did
  not OK at the sign-off; scoped to runs where the stage fired (mirroring
  `spec.fail.story-outside-slate`); `enforces:` live local rules only.
- **The stub** (0077) — `impl.thinking-stage`, `extends: common.thinking-stage`, `class: floor`.

### 3.3 Migration 0078 — the ten architecture touches (architecture S8 list and V7)

1. The design-input line — `impl.design-inputs` reworded: a desk visit file keyed to the feature
   and the run's `design-stage.md` join the inputs (architecture D5).
2. The offer — §3.2's desk offer (one carrier, not two).
3. Drawing into the store's diagram files before code — the design phase edits
   `.mochiko/product/architecture/diagrams/` in place, delta-styled, touched elements
   `proposed (<key>)` (architecture D8).
4. `impl.design-outputs-home` reworded — the store diagram files join the in-place outputs; the
   run's `architecture.md` becomes the neighborhood view plus the checkpoint table listing the
   store diagram files it edited (architecture D8 as repaired, S6/V1, V9; supersedes
   `architecture-design-primitive` D5-plan-run-artifact in part).
5. `impl.deviation-gate` (floor) reworded — reads the store diagrams, held to the container and
   flow views; ER, state and other views checked at the landing diff, never as cycle stops (S7).
6. The built-vs-signed diff reads the store diagrams (`impl.store-landing`, a `tools` reword).
7. The landing flip clears the delta styling to plain and the elements to `built`, no redraw
   (`impl.store-landing`, the same reword; W2 carry-forward: implement's landing orphan check
   covers diagrams, per 0070's `orphan-rule`).
8. Gap-finding's first message carries the signed `architecture.md` and the store diagram files it
   lists (`impl.gap-finding-blind-dispatch`, a `tools` reword; 0082 on the skill side).
9. `impl.gate-design-checkpoint`'s "signed on a rendered diagram" — carried by §3.2's checkpoint
   reword (one reword per id; 0078 does not touch it again).
10. Implement rebuilds the derived index at every design checkpoint and every landing, structural
    or not, so the stale compare runs there (V7) — a rule outside `tools`.

Also `impl.design-seats-staffing` (principal-architect draws the store diagram edits) and
`impl.design-review-pair` (the meaning-kept grade beside conformance, a `tools` reword) — each
only if P1 finds its text false or incomplete against D8 as ruled.

### 3.4 Migrations 0079–0082 — the skill-side touches

- **0079 `authoring-architecture-store`** — `fold-duty`: the landing flip covers diagrams (delta
  styling to plain, the diagram's `**Lifecycle:**` key cleared) — W2 carry-forward; the health
  view's "orphan elements" label names diagrams too (W2 carry-forward, matching 0070's
  `orphan-rule`). Lines made false only otherwise.
- **0080 `review-plan-artifacts`** — the meaning-kept grade: the drafts against the ruled set in
  `design-stage.md`, every seat addition flagged by the D8 size split, the result filling the
  landing table; `gap-list-scope` and `material-divergence-autofail` read the grown list where
  the user added a gap (D6, D8).
- **0081 `review-sufficiency`** — the re-grade of a design piece the user adds inside an existing
  story, before it joins: the seat stays the report's one writer, the gap marked user-added (D6).
- **0082 `testing-gap-finding`** — `blindness-fence-inclusion-list`'s admissible inputs carry the
  signed `architecture.md` and the store diagram files it lists (touch 8), only where the skill's
  text names the drawing today; the fence otherwise unchanged.

### 3.5 Prose (P2)

- **`commands/implement.md`** — lines this wave makes false only; the Not-done line cites the
  CLI-printed pin, never a count.
- **The four skills and `patterns-system-design`** — lines made false by §3.2–§3.4 (the
  design-phase delta procedure in `patterns-system-design` reads against architecture D8);
  `ARTIFACT-CHECKLISTS.md:80`/`:89` re-keyed to D9 (W2 carry-forward).
- **Router, ripple, strips, budgets, `CHANGELOG.md`** — as wave 2 §3.5: lines made false only;
  `[v0.120.0]` strip entries for every removal and supersession, consumers assessed (implement's
  design phase, the desk, the store skill, the three review-side skills); each touched skill's
  payload re-measured as a skill pair, growth argued as the ruled obligations; the W3 slot.

### 3.6 Decision and default coverage

| Decision / default | Carrier |
|---|---|
| D1 · *Precedence* · *Slots declared where* (implement's share) | `impl.thinking-stage` · the contract named on in the stage rule · the by-name `contract-inside` grant |
| D3-four-move-now (implement) | 0076 · 0077 · the obligation map's implement rows |
| D5-stage-after-runopen (S9) | the `design-stage` moment · the stage rule · the epic agenda |
| D6-outside-need-asked (S1) | the outside-need rule · 0081 · 0080's grown list · `impl.lane-never-widens` unchanged |
| D7-direction-then-pieces (S8) | the direction-seed-pieces rule · the template's item shape |
| D8-checkpoint-shows-landing (S2) | `impl.gate-design-checkpoint` reworded · the seat-addition split · 0080's meaning-kept grade · the fail |
| *Implement — the blind list* · *handed-in sets* · *mid-run re-fire* · *the set's home* · *the fail* | the blind-list rule · the handed-in rule · the re-fire carrier · 0075 and the file binding · the fail |
| *Sizing and skips* · *Greenfield seed kept apart* · *Where TEST cases sit* | no size gate in any text; `impl.design-absent-baseline-seed` unchanged; no TEST-case rule (follow-up session) |
| *Change control* | §4 unit 5: the engine's render re-read against implement's |
| Architecture D5 (design-input line, offer) | `impl.design-inputs` reworded · the desk offer |
| Architecture D8 (touches 3–8, 10) | §3.3 items 3–8 and 10 · 0079 |
| Architecture D9 (touch 9) | the checkpoint reword · `ARTIFACT-CHECKLISTS.md` |
| Supersessions owed (implement's share) | the checkpoint's wording (force kept); `architecture-design-primitive` D5/D8 in part — strip entries and `DECISIONS.md` annotations at landing |

### 3.7 Tests and kits (P3)

Crate frozen-census figures from `migrate status`, the views and the renders, never by hand (no
crate source, no crate version); `render.rs`' `SHIPPED_TEMPLATES` gains `design-stage` with its
two fixtures captured from the binary · `evals/contract/`: `Expected["implement"]` gains
`impl.thinking-stage`, the new fail and any floor P1 mints; a skill's frozen floor set changes only
if its floors move; `TEMPLATE_NAMES` gains the template; README figures where a count moved ·
`evals/plan/implement/`: `observable.yaml` re-keyed, each minted id with its why, every id this
wave mints, rewords or retires placed; the preregistration recounted with one honest fill-log
amendment line, as wave 2's architecture kit did; the kit's static checks pass; no grid.

## 4. Audit (leg 2)

One gate grader for the wave: a fresh plain `general-purpose` seat on `model: opus`, explicit (a
new seat that authored nothing here), its brief the verbatim `mochiko-cli rules
validation-primitive-edit` render, the units, the paths, the pre-pass commands and any named
overage. One verdict block and one outcome line per unit, in `reports/audit-w3.md`:

1. schema content — 0075, the new template view and the three home view diffs
2. schema content — 0076 and 0077, and the implement view diff
3. schema content — 0078 and the implement view diff
4. schema content — 0079, 0080, 0081, 0082, each with its skill's view diff (4a–4d)
5. the `implement` command pair — run branch plus implement's own criterion; with *Change
   control* (the engine's render re-read against implement's: no implement rule loosens an engine
   mechanic) and the delivery check that `impl.thinking-stage` resolves to the engine's seven
   blocks, first-hand; `impl.sec.tools` under its ceiling
6. the skill pairs — `authoring-architecture-store`, `review-plan-artifacts`, `review-sufficiency`,
   `testing-gap-finding` (6a–6d), each with its named overage if any
7. prose — `patterns-system-design` if touched, the router, strips, ripple, ledger, changelog
8. crate tests, contract expectations, the implement plan kit, any allowlist rows (also the
   crate's non-author review)

On a FAIL the owning seat fixes and the same grader is resumed on the delta; a second FAIL on any
unit halts to the user (`common.gate-loop-bound`). Beside the audit, the lead reads the built rules
against §3.6 row by row; that read clears nothing on its own.

## 5. Facts the build checks

- **The `!`-output ceiling.** `impl.sec.tools` at 26,200 B at the open; P1 reports every
  implement block's bytes before and after; over 29,000 B is a stop.
- **D9's optional parse check.** Not built in this build: it is crate source, outside this build's
  line. The lead's call as D9 allows ("the build decides"); a BACKLOG item at landing.
- **OQ3** stands from wave 2: the user's GitHub check at push time.

## 6. Wave close and stop conditions

No bump at wave close; the bump, the full gates and the sandbox contract suite follow, then the
landing. The wave closes when §4 is all PASS; the gates (`migrate validate` · views ≡ replay ·
`cargo test` + fmt + clippy + audit · the full similarity sweep · char budgets) run as a
regression read.

Stops, each halting the wave to the user: plan FAILs from more than half the producers · any second
re-plan · a tree dirtied during planning · a rejecting `migrate validate` finding a seat cannot
resolve inside its ownership · any floor or fail retired or lowered · a grammar or crate-source
change found needed · an asking mechanic implement needs and the engine lacks (a slot by ruling) ·
`impl.sec.tools` over 29,000 B · a persona line made false · a second FAIL on any gate unit.
