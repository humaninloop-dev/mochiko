# Wave 1 — the design-integration build (`impeccable-design-integration` D1–D14 · build surface items 0–11)

**Opened:** 2026-09-23 · **Lead:** session lead (Opus 5.5) · **Branch:** `impeccable-design-integration`
off `main` at `6e4b264` · **Target version:** 0.114.0 (MINOR — three new skills, a persona rename with
no alias (D4), a new product-baseline home) · **Audited under:** the gate form (`author-grader-consolidation`
D3 · D6 · D7 · D11) — one plain seat, rendered contract, one re-audit by the same seat.

Ruling anchor for every migration header: `2026-09-19 impeccable-design-integration` with the decision
segment the migration executes. BACKLOG item: "Impeccable design-integration build → Design-integration wave".

## 0. Pre-wave (item 0) — done by the lead before any dispatch

Mechanical execution of D14 (sound-loop exemption: execution of an existing ruling):

- **Upstream pinned:** `pbakaus/impeccable` `main` at **`e0881d2de397d5e9761d7b35ff5017d8f5ebf69b`**
  (2026-09-22), cloned to the session scratchpad. Every provenance line names this SHA. Drift since
  the record's read (2026-09-19): fix commits only; `skill/reference/visualize.md` (present since
  2026-07-22, not in the SKILL.md command table) is the comp-led build's reference — covered by D9's
  comp-first exclusion, no new verb to allocate.
- **Root `LICENSE`** (MIT, HumanInLoop) and **root `NOTICE`** (Impeccable attribution, the pinned SHA,
  Impeccable's own `NOTICE.md` chain for the `ehmo/platform-design-skills` MIT material, and the full
  Apache-2.0 text retained in-file) — D14 (i)–(ii). Item 9 checks presence.
- **Budget abort gate (R16) and description cap (R18)** — cannot run before drafts exist, so it runs as
  a **named stop** inside S1 and S2 (§3): each drafts its new skill(s), measures the delivered payload
  (SKILL.md body + rendered blocks, characters of the parsed value, the ledger's canonical snippet) and
  each `description:` (≤ 1,536), reports, and holds. The lead compares each against its family's
  measured rows in `.mochiko/memory/primitive-cost-budgets.md`; **> 25 % over the family's largest
  non-outlier row → abort to the user**. No migration enters the log before that check passes.

## 1. Sound-loop wiring — trigger fired on every item

Every item is judgment-authored content on a governing surface (plugin primitives, product-baseline
home). **Leg 1:** each producer seat is spawned once in the main tree with a plan-only brief; the lead
snapshots `git status --porcelain` before and diffs after (any change = `dirty`, FAIL); the plan comes
back verbatim; a **fresh peer of the producer's persona type** grades it per
`mochiko:review-seat-plan` (render pasted in the brief), default FAIL; the lead approves only a PASS
and resumes the same seat with the plan quoted. Re-plan bound one on a shared counter; a second
consumption goes to the user. **Watch with stop:** plan FAILs > half the seats · any second re-plan ·
any dirty tree → halt and report. **Leg 2:** §5. **Leg 3:** the user gates the bump and the commit.

**Transport:** subagents, resumed by message. Message lane fires (lead-relayed approvals) — mesh-hold ·
content-pinned approvals (the order quotes the plan it approves) · no ritual sends · fan-in confirmed
on disk. Topology lane does not fire — ownership is disjoint (§2), each seat owns its own migration
file, and the one shared projection (`.mochiko/schema-views/**`) has one writer (S5), run last.

**Tiering:** every seat brief carries the `Explore` · `model: haiku` line for locate/enumerate reads.
Seat tiers: persona defaults, except **S1 `product-engineer` deviated `sonnet` → `opus`** — reason:
the seat ports the design-director judgment the whole track rests on (persona + two skills), the
wave's highest-judgment content; disclosed in the roster line. Its plan peer runs at the same tier.
The gate grader runs `opus`.

## 2. Seats, ownership, and sequences (disjoint)

Sequence ranges assigned by the lead (migrations README): a seat never allocates its own.

| Seat | Persona | Migration | Owns |
|---|---|---|---|
| **S1 design** | `mochiko:product-engineer` (opus, deviated) | `0011-design-direction-craft-floor.yaml` | `agents/product-engineer.md` → `agents/product-designer.md` (git mv + rewrite, D3/D4) · `skills/patterns-design-direction/**` (new; `references/ios.md`, `references/android.md` ported) · `skills/patterns-craft-floor/**` (new) · `skills/authoring-prototype/SKILL.md` + its schema (step 2 baseline-first, Direction block, boundary clause R21) · `template/spec` Screens & Flows Direction block · `command/specify` rules that name the Screens & Flows content, if any · strips: `product-engineer.md` (supersession-by-ruling, rename) + `authoring-prototype.md` if content leaves · `GLOSSARY.md` scaffold (item 10 terms) |
| **S2 verification** | `mochiko:qa-engineer` | `0012-design-verification-lenses.yaml` | `skills/review-design-audit/**` (new, `review-code-minimalism` form) · `skills/testing-gap-finding/SKILL.md` + schema (critique section, D11 decline superseded for verification only) · `skills/patterns-vertical-tdd/SKILL.md` + schema (measurable `**TEST:**` example asserts, not-a-toolchain line R17) · `skills/testing-end-user/SKILL.md` + schema (computed-style execution note only; boundary untouched) · strips for each |
| **S3 baseline** | `mochiko:principal-architect` | `0013-design-baseline-home.yaml` | new `home` `product-design` at `.mochiko/product/design/` + its template (producer/checklist views; truth part · system part; a11y pointer field; shelf-seam pointer convention; single writer `product-designer`) · `home/product` subdirs gains `design` · `command/implement` rules: craft-floor load on UX-bearing cards, landing fold derives the system part (`As-built:`/`Drift:`), first-write user checkpoint (D7), critique/audit placement + depth keying (D8/D13) · `commands/implement.md` only if its scaffold text must name the new moment · `skills/review-sufficiency` clause 8 re-key (D13) · `skills/review-specifications` direction-contract checks (item 7) · `skills/patterns-code-minimalism` craft-floor cross-reference (conditional — builder's call, disclosed) · strips for each |
| **S4 setup** | `mochiko:tech-lead` | `0014-setup-product-truth-leg.yaml` | `command/setup` rules (the product-truth leg: lead asks inline, `product-designer` producer + non-author validator write; `baselines-bootstrap` third arm; design home scaffold unconditional) · `commands/setup.md` if its scaffold must name the leg · `skills/analysis-codebase/**` (SKILL.md, schema, `scripts/detect-stack.sh` design-system detection leg: UI framework · CSS system · fonts · token files · component library) · `template/codebase-analysis` if the detection output needs a section · strips for each |
| **S5 ripple + gates** | `mochiko:staff-engineer` | `0015-product-designer-rekey.yaml` | every remaining `product-engineer` rule re-key (e.g. `patterns-model-tiering.seat-default-key` floor — anchored) · `agents/staff-engineer.md` (`skills:` + `patterns-craft-floor`) · `agents/qa-engineer.md` (`skills:` + `review-design-audit`) · `plugin.json` agents list + version · `marketplace.json` · router `skills/mochiko/SKILL.md` (three new rows, persona re-key `:142`, baseline list `:81`) · `README.md:132` · `.mochiko/memory/primitive-cost-budgets.md` (three birth-seed rows, persona row re-key) · `evals/agents/product-engineer/` → `product-designer/` re-key · `evals/contract/**` pre-registration (three new skills, persona rename) · crate census tests re-key (no crate source, no crate bump) · `scripts/similar-rules-allowlist.yaml` · `.mochiko/schema-views/**` (emit) · `ARCHITECTURE.md` re-render if the store names the seat · `CHANGELOG.md` 0.114.0 · strips for `staff-engineer.md` / `qa-engineer.md` if content leaves |

Order: S1 ∥ S2 ∥ S3 ∥ S4 plan → peer grades → approve → execute (S1/S2 stop at the budget check) ·
S5 plans after S1–S4 land (it reads their final state) → peer grade → approve → execute · §5 audit ·
§6 gates · §7 landing.

**Migration authoring discipline (collision guard):** a seat drafts its migration outside the log,
validates against a scratch copy (`cp -r plugins/mochiko/migrations <scratch>/log && cp <draft>
<scratch>/log/ && mochiko-cli migrate validate --report --log-dir <scratch>/log --plugin-root
plugins/mochiko`), stamps, and only then moves the file into `plugins/mochiko/migrations/`. An
unstamped file in the log blocks every seat's replay. Cross-seat interaction is checked by S5's full
replay.

## 3. Content by seat (record items → seat)

- **S1 (items 1, 2, part of 7, part of 10).** Persona: design-director judgment (the brief wins ·
  refinement preserves, redesign replaces · visual authority is evidence · mode per surface) carried as
  identity and taste, **no workflow trace** (axis 4); `skills:` = `authoring-prototype`,
  `patterns-design-direction`; `model: sonnet` kept (the class default; the deviation is this run's
  only). `patterns-design-direction`: modes; design laws; the `shape` checklist with `onboard` /
  `clarify` / `adapt` folds; platform-routed native guidance (ios · android ported under
  `references/`; `desktop` accepted, no guidance — OQ3); output = a **Direction** block in Screens &
  Flows (mode · direction contract · declared tokens as intent · preserve / expand / replace the
  incumbent world); **no accessibility content** (D11). `patterns-craft-floor`: bans, reflexes, the
  `typeset` / `layout` / `colorize` / `animate` / `delight` folds, the register dial
  (`bolder`/`quieter`/`distill`) read from the contract, honors baseline tokens, keeps the
  code-minimalism floor line in view; loaded before any edit on a UX-bearing card, never for planning.
  Both skills: patterns-family schema pair, provenance line (upstream + SHA), verbatim passages marked,
  description MUST/SHOULD + triggers ≤ 1,536 measured, boundary clause vs `authoring-prototype` in both
  descriptions. **Named stop:** after drafting both skills (SKILL.md + draft rules), measure and report
  to the lead before stamping 0011.
- **S2 (item 4).** Critique section in `testing-gap-finding` (grades built UI against the direction
  contract, Screens & Flows, and the baseline; blocking = contract/baseline/standard-of-record
  violation, advisory = taste, carried to the landing fold; one fix round, one confirm round; depth
  keys critique only — low advisory-only no confirm round, high full D8; **no a11y probing**). The
  "Accessibility probing — declined" line superseded **for verification only**, strip names the
  record. `review-design-audit`: perf · `harden` · `optimize` · `polish` · the slop-tells checklist
  (D10's list); advisory findings block; scope the design-audit lens only; review-family schema pair;
  declared on `qa-engineer` (S5 edits the persona's `skills:` line). `patterns-vertical-tdd`: example
  asserts (contrast ≥ 4.5:1 · touch ≥ 44 px · line length · heading levels · the standard-of-record
  checks · responsive at declared viewports) + the not-a-toolchain line. `testing-end-user`: the
  computed-style execution note only. **Named stop** after drafting `review-design-audit`, as S1.
- **S3 (items 5, 7-sufficiency/specifications, part of 8).** Per the S3 row and D5/D7/D8/D11/D12/D13.
  Template-borne grammar, no authoring skill, no derived index (D5). The a11y field is a **pointer**
  to the standard of record, never a restated floor (D11). Stance-vs-value seam (D12) stated in the
  template's convention line.
- **S4 (item 6).** Per the S4 row and D6: five asks (platform validated `web`/`ios`/`android`/
  `adaptive`/`desktop`, unknown warns never defaults · brand/voice · principles · a11y pointer ·
  evidence fence), material gaps only, audience/purpose pointed at the product desk;
  `setup.interrogation-inline` honored; the write through a `product-designer` producer + non-author
  validator under the sound loop; scaffold unconditional; greenfield system part stays scaffold.
- **S5 (items 1-ripple, 8-rekey, 9).** Per the S5 row. The persona-edit advisory grid
  (`uv run evals/run.py agent grid product-designer --arms pre,post`) is **attempted**; if the kit
  cannot run under the rename in-wave, the skip and its reason are recorded in the strip entry
  (advisory only, never a gate).

## 4. Stop conditions (any seat)

Budget abort (§0) · plan-watch trip (§1) · a rejecting `migrate validate` finding a seat cannot
resolve inside its ownership · any need to write outside the seat's row (report, never write) · a
second FAIL on any gate unit (to the user) · contract suite not green after one fix round · any crate
source touched.

## 5. Audit — the gate form (leg 2)

One gate grader for the wave (D11): plain `general-purpose`, `model: opus` explicit, authored nothing.
Brief = the verbatim render of `mochiko-cli rules validation-primitive-edit` (all blocks), the unit list
with paths, the pre-pass commands. It runs `mochiko-cli migrate validate --report --plugin-root
plugins/mochiko` and the char-budget measurement itself and quotes both. Units (one verdict block + one
outcome line each): `product-designer` persona · `patterns-design-direction` pair ·
`patterns-craft-floor` pair · `authoring-prototype` pair · `review-design-audit` pair ·
`testing-gap-finding` pair · `patterns-vertical-tdd` pair · `testing-end-user` pair ·
`review-sufficiency` pair · `review-specifications` pair · `analysis-codebase` pair · setup command pair
· implement command pair · specify command pair (if touched) · `patterns-code-minimalism` pair (if
touched) · `patterns-model-tiering` pair · schema content 0011–0015 + view diff (AM-2 five) · design
home + template · router · staff-engineer + qa-engineer personas · budget ledger · pre-registration ·
CHANGELOG + manifests · strips. Splits into a second seat only if the files will not fit, and says so.
**Re-audit:** the owning seat fixes; the same grader, resumed, reads only the delta; a second FAIL
halts to the user.

## 6. Gates before the bump (item 9)

`migrate validate --report` 0 rejecting · views ≡ replay · `cargo test -p mochiko-cli` + fmt + clippy +
audit · full similarity sweep · **contract suite full run green** (SKIPPED blocks) · char-budget
pre-assert on every touched budgeted primitive · `CHANGELOG.md` names the rename · manifests synced at
0.114.0 · strips present · `LICENSE` + `NOTICE` present · provenance line on every ported skill.

## 7. Landing (items 10, 11)

`DECISIONS.md` row → record (wave BUILT) · BACKLOG: the build item → trail; the UX-prototype watch
already extended (record) — item 11 watches confirmed there · ROADMAP Production-only Next row touched ·
record Status/Landed + index entry · `GLOSSARY.md` scaffolded by S1 — **the standing governance amend
trigger fires** on it; whether to run `/mochiko:setup` amend is the user's call, surfaced at close ·
`build-log.md` closed with the `floor:` line (`plans:` segment) and the roster line · commit on the
user's word.
