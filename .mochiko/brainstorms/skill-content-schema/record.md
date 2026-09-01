# Skill-content schema — decision record

**Topic:** extend the command content-schema architecture to skills — a common schema for
similar-ontology skill families with `extends:`-style inheritance, and a ruled logical
boundary between what lives in a skill's `SKILL.md` and what moves to schema. Driver ask
(user, 2026-09-01): "have some common schema for similar ontology skills and then extend
them. Also some logical boundary of what should be in .md vs what should be schema" — walked
aspect-by-aspect against the command-schema precedent.

**Status:** accepted (2026-09-01)
**Opened:** 2026-09-01
**Lead:** session lead (brainstorm charter, run inline in-conversation)

**Prior-session relations:** builds on the command-side lineage — `command-content-schema`
D1–D16 (pair form, D2 rules-move/narrative-stays split, D12 grain, D16 runtime-only),
`command-schema-ontology` D1–D11 (nine-kind grammar, `conditions:`/`when:`, `moments:`,
`enforces:`, D8 `extends: common.*`), the near-dup convergence ADR R1–R6
(`.mochiko/decisions/2026-08-28-near-dup-convergence.md` — near-identical 3+ bar,
strongest-wording-wins, allowlist), and the schema-header runtime-kernel ADR
(`.mochiko/decisions/2026-08-28-schema-header-runtime-kernel.md`). Composes with
`skill-compression-tooling` (accepted — per-skill degradation eval) and
`primitive-eval-harness` (open). Any grammar ruled here for skills amends none of the
command rulings; skills would be a new grammar family under the same governance envelope.

---

## Ground facts

Verified this session by whole-file reads of the command pair exemplars
(`brainstorm.yaml` + `brainstorm.md`, `common.yaml`, `command-labels.yaml`), the three
canonical records, both 2026-08-28 ADRs, `converting-command-to-schema/SKILL.md`,
`primitive-edits.md` criterion set, and the detector's header.

- **F1 — command precedent state.** Six `.md` + schema pairs at v0.99.0 (header-kernel trim
  at HEAD, ships with next bump): ~321 rules, three orthogonal axes (`class` bindingness ·
  nine-kind `kind:` · registry labels), declared `conditions:`/`when:` (floors never shed),
  unordered `moments:`, `enforces:` fail mirrors, `vars:`/`${var}`, mint-once dotted-slug
  IDs, provenance sidecar. `common.yaml`: 9 blocks, 24 stubs; stub inherits
  `text`/`labels`/`pointer` only, `class`/`kind`/`when`/`enforces` always local.
- **F2 — skill inventory.** 38 shipped skills under `plugins/mochiko/skills/`, natural
  families: `patterns-*` 13 · `authoring-*` 8 · `review-*` 7 + `validation-*` 1 ·
  `testing-*` 3 · `analysis-*` 2 · singletons `mochiko` (router) ·
  `grooming-operating-docs` · `brownfield-integration` · `executing-tdd-cycle`. The review
  family shares near-identical machinery in prose (default-FAIL posture, author≠grader,
  blind dispatch, verdict grammars); the minimalism trio + sound-loop self-describe as
  siblings.
- **F3 — extraction-bar lineage (commands).** D3 `Contested` (no shared library; labels the
  only link) → ontology D8 (exact duplicate across 3+) → near-dup R1–R6 (near-identical
  3+, strongest-wording-wins, command-specific members keep local text, keep-distinct edges
  allowlisted in `scripts/similar-rules-allowlist.yaml`).
- **F4 — read-path arithmetic (measured, commands).** `common.yaml` adds ~2.5k chars to
  every binding command's fire vs 52–434 chars of removed duplication — net positive chars;
  the purchase is single-point drift protection, never size. Skills are model-invoked
  mid-run inside other agents' contexts, so any shared-file read multiplies across seats —
  the arithmetic bites harder than for commands.
- **F5 — skill delivery mechanics.** A skill's `SKILL.md` body is the delivered payload at
  invoke; `description:` (≤1,536-char delivery cap) carries the MUST/SHOULD trigger
  encoding and is the discovery surface; classification user-invoked / model-invoked
  (five-axes #1); persona carries judgment, skill carries procedure (five-axes #4);
  producer↔validator pairing (five-axes #5). Skills have no `$ARGUMENTS` protocol and no
  Not-done count-pin; several ship `references/` files (budget-exempt) and templates.
- **F6 — evidence state.** Model-interpreted schema delivery is `Assumed` at n=0 live runs
  even command-side; all instrument evidence is plan-only eval proxy (pre/post grids, no
  attributable regressions at v0.98.0/v0.99.0). The command D10 first-live-run watch is
  still open. Skills currently have their own instrument: the skill-compression LLM-judge
  degradation eval; a plan-only analogue for skills does not exist.

## Problem — why this session (ranked driver set, user-ruled at Q1)

**Primary: drift control across families.** Skill families (review/validation graders, the
minimalism siblings, authoring producers) restate shared machinery — default-FAIL posture,
author≠grader independence, blind dispatch, ladder framing — in per-skill prose that drifts
at every touch. A common schema single-homes the shared text; family members extend it.
This is the near-dup convergence win (R1–R6) applied to skills, and the user's own phrasing
of the ask ("common schema for similar ontology skills and then extend them").

**Secondary, named concerns:** deterministic checkability (IDs, registry vocabulary, a
checker over skill obligations — command driver 1) and addressability for ceremony (strips,
audits, compression waves citing `skill.*` IDs instead of verbatim-quote hunting — command
driver 2). Read-cost structure was offered and not taken as a driver.

## Decisions

### D1 — Pilot corpus: the review family; further families by their own ruling — `Confident`

**Statement:** the pilot converts the review family — the 8 grader skills
(`review-brainstorm` · `review-code-minimalism` · `review-feasibility` ·
`review-governance-intent` · `review-plan-artifacts` · `review-specifications` ·
`review-sufficiency` · `validation-constitution`) — to the skill schema form in one wave.
Every further family (patterns, authoring, testing, analysis, singletons) converts by its
own recorded ruling (the command D10 door-open idiom). Neither a single-skill pilot nor an
all-38 wave.

**Rationale:** drift-control is the primary driver (Q1) and the review family carries the
densest shared machinery — default-FAIL posture (where a clearing verdict exists; the
composition is corrected in the amendment below), author≠grader independence, blind
dispatch, verdict grammars — so one wave exercises both family inheritance and per-skill
schemas; a single skill leaves the primary driver unexercised, and 38 audits with n=0
delivery evidence (F6) is unpriceable. User ruled option A at the Q2 fork.

*(Amended at review, I3/M7 — user-ruled in the I/M batch.)* "The 8 grader skills" is
corrected: the family is the 8 skills of the grading lane, of which **six issue clearing
verdicts** — `review-code-minimalism` is advisory-only ("never a cycle-failing gate") and
`review-specifications` produces gap-finding input, never a PASS/FAIL verdict; the
verdict-vocabulary heterogeneity (FAIL · `feasible` · `ready` · advisory) is flagged to
the census as prime allowlist territory under strongest-wording-wins. The `mochiko`
router is **out of scope for the foreseeable** — a ~41k-char user-invoked index,
deliberately unbudgeted, where an obligations-only conversion is near-meaningless.

### D2 — Schema home: in-directory, beside `SKILL.md` — `Confident`

**Statement:** a converted skill's schema lives at
`plugins/mochiko/skills/<name>/schema.yaml`, inside the skill's own directory — not in the
central `plugins/mochiko/schemas/` command home, not embedded in `SKILL.md`. The
family-common file's home is decided at the extends aspect, separately.

**Rationale:** skill invocation injects the skill's base directory, so an in-dir relative
Read is robust from the plugin cache — repo-relative central paths are not; the skill
directory stays the self-contained shipping unit (`references/` already sets the multi-file
precedent); an embedded YAML block would forfeit the change-regime separation the schema
exists for. User ruled the recommended option at the Q3 fork. *(Reordered to mint
position at review, M4.)*

### D3 — Boundary: obligations-only schema; procedure stays prose in `SKILL.md` — `Confident`

**Statement:** the skill schema carries **obligation-shaped content only** — floors, fences
("never grades own output"), boundary lines (what the skill consumes but never redefines),
output/verdict contracts, independence rules. `SKILL.md` keeps: the `description:`
frontmatter (discovery surface, delivery-cap-bound — never moves), the overview/identity
voice, and the **entire sequenced procedure as prose**. ~~`references/` files are
untouched.~~ *(superseded by the C2 amendment below — the three-home boundary; struck at
verify, R2)*. No ordered step/phase grammar is minted; declared phase anchors (a `moments:` analogue) are
a named graduation candidate, keyed on a demonstrated consumer — not shipped.

**Rationale:** the drifting machinery the primary driver targets is obligation-shaped
(default-FAIL, blind dispatch, severity/verdict grammars) — obligations-only serves it
fully; ordered procedure nodes are exactly the workflow-engine direction the command
ontology deliberately declined (unordered `moments:`, ontology D4), and doubling the
structural bet at n=0 delivery evidence (F6) prices badly. Command D2 replayed
conservatively. User ruled the recommended option at the Q4 fork.

*(Amended at review, C2 — user-ruled: the three-home boundary.)* The boundary covers
`references/` explicitly — the pilot's largest obligation home (reference mass up to 9×
the body it rides). **Stub is the default:** an obligation-shaped line in a `references/`
file gains a schema stub rule — ID, `class:`, `kind:`, `pointer:` naming the reference
file as the procedure home; referenced, never restated, so the obligation is addressable
and checker-visible while the file stays intact. **Lift by census judgment:** an
obligation lifts fully into the schema only where its reference home is incidental (a
standalone floor line, no surrounding procedure) — decided per obligation at the D9
census, never ad hoc at conversion. **Non-obligation reference content** (lenses, worked
examples, deep procedure) stays untouched; its drift control is the existing "Single
source" + "Consumed by:" convention, hereby named the sanctioned mechanism for
reference-borne shared content. The D6 anti-dual-homing floor is preserved — a stub
points, never duplicates.

### D4 — Grammar posture: inherit the command grammar as baseline, named skill deltas — `Confident`

**Statement:** skill schemas reuse the command rule grammar wholesale — three orthogonal
axes (`class` floor/must/advisory · the nine-kind closed `kind:` set, `constraint` the
omitted default · registry labels), `conditions:` + `when:` with floors-never-shed,
`vars:`/`${var}`, mint-once dotted-slug IDs with tombstones, one-obligation grain
(content-schema D12), referential closure (D15), runtime-only content with the provenance
sidecar (D16). Named skill deltas: **no `moments:` block** (procedure is prose, D3) · the
**section set is NOT the command six-set** — minted once from the pilot census of the 8
graders, uniform across the family, a section with no rules carrying its explicit empty
marker · **new kinds only by census evidence**, never invented at conversion · labels come
from a **sibling registry `plugins/mochiko/schemas/skill-labels.yaml`**, seeded from the
overlapping command vocabulary (`independence` · `evidence` · `user-gate` ·
`floor-pointer` transfer cleanly), same registry-edit-first ceremony — the command
registry's scope statement stays honest.

**Rationale:** the obligations-only boundary (D3) leaves exactly the content class the
command grammar was built for; every field was already priced by the command lineage, and
live `when:` cases exist in the family today (review-sufficiency's ten-clause/three-clause
scope collapse, review-feasibility's store-delta pass); a minimal fresh grammar would
forfeit the B/C secondary drivers day one, and a skill-native redesign is speculative
until a census shows the nine kinds failing. Checker reuse comes near-free. User ruled
the recommended option at the Q5 fork, sibling-registry lean included.

### D5 — Common form: explicit stubs + per-family common block file; command extends semantics wholesale — `Confident`

**Statement:** the shared surface is a **flat block library per family** —
`plugins/mochiko/schemas/skill-review-common.yaml` for the pilot — never a whole-section
base schema. Member schemas bind via explicit `extends: review-common.<slug>` stubs under
the command semantics unchanged: inheritance covers `text`/`labels`/`pointer` only;
`class`/`kind`/`when`/`enforces` always local; the stub's `<skill>.*` ID stays the citable
ID; `${var}` substitutes from the binding schema's `vars:`. Extraction bar = the near-dup
convergence rulings adopted wholesale (R1–R6: 3+ members near-identical,
strongest-wording-wins, command/skill-specific members keep local text, keep-distinct edges
recorded in the allowlist); the similar-rule detector extends over skill schemas. A
cross-family global `skill-common.yaml` is a **graduation candidate**, keyed on a real
3+-family block emerging — not shipped. **No cross-grammar sharing:** a block texted
identically in command `common.yaml` (e.g. `author-grader-default-fail`) stays a separate
skill-side block, the drift edge noted in the allowlist.

**Rationale:** implicit whole-section inheritance reopens at section scale the
single-file-readability exposure C3 closed, and mints merge semantics the command lineage
never priced; explicit stubs keep every member's bindings readable from its own file and
reuse proven machinery end-to-end (checker asserts, detector, allowlist, ceremony). The
per-family file keeps the F4 read multiplier down — a review skill reads only review
blocks. User ruled the recommended option at the Q6 fork, cross-grammar observation
included.

*(Amended at review, C3 — coverage survivor, user-routed **rule inline**.)* **Rejected
road — shared-`references/` single-source (recorded).** The pilot family already ships a
working shared-single-source mechanism: a shared `references/*.md` with a "Single source"
header, a "Consumed by:" consumer list, and an edit-time all-consumers guard
(`CROSS-EXAM.md` serves two skills today; `EXTERNAL-CLAIMS.md` names nine consumers). It
serves the primary drift-control driver at zero new grammar, registry, checker surface,
or ceremony — and is rejected as the family mechanism on the named secondary concerns
the user ranked real at Q1: no IDs (nothing for strips/audits to cite), no `class: floor`
semantics (non-waivability stays folklore), no machine surface (checker and detector
blind), no per-member override path (a variant consumer is back to prose drift). Its best
part is adopted rather than lost: the Single-source convention is the sanctioned drift
control for non-obligation reference content (D3 as amended, C2). Noted, not decided: the
road stays a live candidate at the **patterns-family door** — that family's content is
judgment-prose more than gradeable obligation, and its own rollout ruling should weigh
this road explicitly.

*(Amended at review, I7 — user-ruled in the I/M batch: the cross-grammar clause argued.)*
The no-cross-grammar-sharing clause stands on recorded argument, not assertion: the two
grammars ship on different delivery paths (command fire vs skill invoke) with different
`${var}` scopes and different label registries; a cross-grammar block would couple two
ceremony domains through one shared file both planes must read, to save 52–434 chars of
duplicated class text — the allowlist edge is the cheaper drift control at that price.

### D6 — Delivery guard: load-first block + floor-count pin; first-live-run watch — `Confident`

**Statement:** each converted skill's `SKILL.md` body gains a "Rules — load the schema
first" section: the obligated first action is a raw, whole Read of the skill's
`schema.yaml` (base-dir-relative per D2) and, where any stub binds,
`skill-review-common.yaml` in the same first action; the schema's section IDs are
enumerated in the block; the reading grammar (`when:` interpretation, floors always
delivered, stub inheritance limits) is carried there. One pinned line — "the N rules of
`class: floor`" — is checker-asserted against the schema; a stale pin is an out-of-sync
halt-and-surface, the command C2 count-guard re-keyed to floors (skills have no Not-done
set; floors are the content whose silent loss hurts most). Floors are never duplicated
inline — dual-homing is the drift the session exists to kill. Converted skills join a
**first-live-run watch** in `BACKLOG.md` (delivery probes: schema read? whole? before the
first procedural step?), extending the command D10 watch idiom — one item, never a
parallel per-skill item.

**Rationale:** `SKILL.md` is guaranteed delivered at invoke while the schema is an
instructed-not-forced Read — the exact C2 exposure the command wave priced; the pin gives
the deterministic tripwire, and the watch carries the n=0 delivery honesty (F6). User
ruled the recommended option at the Q7 fork.

*(Amended at review, I1 — user-ruled in the I/M batch: the pin's claim corrected, a
read-back added.)* The pin is a **desync guard**, not a delivery tripwire — it catches
authoring-time `.md`↔schema drift via the checker; a model that skips the schema read
never evaluates it. Delivery gains its own mechanism: the load-first block obligates
**stating the floor count back before the first procedural step**, so a skipped or
partial read produces a visible blank instead of silence. The residual delivery exposure
is carried, named, by the first-live-run watch alone (n=0).

### D7 — Advisory tooling: checker extension, detector extension, compression-eval as the fidelity instrument — ship `Confident` · efficacy `Assumed (n=0)`

**Statement:** three pieces, all advisory (GI-019 carve-out), negative-tested per house
practice. **Checker:** the skill grammar family joins the deterministic checker (extend
`scripts/check-command-schema.py` or a sibling `check-skill-schema.py` — the build's
judgment): `kind: skill` discriminator, in-dir path sweep (D2), section-ID grammar, ID
uniqueness/format/tombstones, registry-label resolution against `skill-labels.yaml`,
`${var}` binding, `when:` resolution against declared `conditions:`, `extends:` target
resolution + local-`class:` assert, and the D6 floor-count pin grep. **Detector:**
`scripts/find-similar-rules.py` extends over skill schemas (its layer-1 "similar-items"
framing already anticipates this); the allowlist grows skill edges. **Instrument:** the
skill-compression eval harness is reused for conversion fidelity — LLM-judge before/after
degradation, each converted skill graded against its frozen pre-conversion referent. No
skill plan-only eval is built — no demonstrated consumer.

**Rationale:** the checker and detector are the B/C secondary drivers made deterministic;
skills, unlike commands, already own a fit measurement instrument — building a second one
would be structure on spec. Efficacy marks stay `Assumed` until a caught defect or a
measured degradation exists. User ruled the recommended option at the Q8 fork.

*(Amended at review, I2/M3/M5 — user-ruled in the I/M batch.)* **Instrument claim
narrowed (I2):** the harness grades a **judged sample of the wave** — two members
minimum — never all eight; its blocking preconditions are real and priced in D9
(per-skill goldens ×3 authored by a non-author seat · a settled probe run · a
user-ratified pre-registered ship bar · four-arm × three-replicate grids, metered
spend), and a conversion's arm shape (pre-conversion baseline vs converted pair, staged
as full skill-directory copies) is defined at pre-registration. **Detector discovery
(M3):** `find-similar-rules.py` gains the in-dir sweep —
`plugins/mochiko/skills/*/schema.yaml` (D2) — beside its flat-glob command home.
**Scope (M5):** D7 binds this conversion only; the open `primitive-eval-harness`
session keeps its own instrument question, unforeclosed.

### D8 — Governance envelope: no new kernel admission; shipped primitives under full ceremony; sibling audit criteria block — `Confident`

**Statement:** the skill schema family needs **no new kernel-class admission** — schemas
are data the model interprets at skill invoke; the checker and detector stay advisory
exit-code signals (GI-019 untouched, the advisory carve-out). GI-020 holds: `schema.yaml`
files ride the plugin as data in the skill directories, the raw Read is the first-class
path, no binary, install no heavier. `plugins/mochiko/skills/*/schema.yaml`,
`plugins/mochiko/schemas/skill-labels.yaml`, and
`plugins/mochiko/schemas/skill-review-common.yaml` are shipped primitives under the full
strip + author≠grader ceremony. `.claude/rules/mochiko/primitive-edits.md` gains a
**skill-pair criteria block** — graded unit `SKILL.md` + `schema.yaml`: load-first block
present · floor-count pin matching · section enumeration set-wise · floor-survival
(`class: floor` leaves only by recorded supersession) · ID continuity with tombstones ·
`extends:` conformance (local `class:`, R1–R6 bar) · `description:` untouched and under
the 1,536-char cap. A **sibling** block beside the command block, never a fork of it.

**Rationale:** the command lineage argued this envelope twice (content-schema D9, ontology
D10); every clause transfers because the delivery mechanism is the same — data plus model
interpretation. Proposed whole; user adopted as stated at the Q9 wrap.

*(Amended at review, C1 — user-ruled: the budget clause.)* The char-budget gate is re-keyed
so conversion cannot defeat it: **a converted skill's budgeted quantity is its
delivered-at-invoke payload** — `SKILL.md` body plus its own `schema.yaml` chars, one
number, parsed-value measurement. At conversion each member's budget **re-seeds to the
measured post-conversion total**, the audit grading the delta against the pre-conversion
body figure as structural overhead only (IDs, keys, grammar); content growth takes the
normal argued-overage path, named in the brief — no headroom minting, no forced
compression smuggled into the conversion. The family common file is budgeted once as its
own primitive, never per-binding-skill. `.mochiko/memory/primitive-cost-budgets.md`'s
exemption line is amended in the wave: the exemption keys on "never auto-loaded" — a
schema whose read is obligated at invoke is budgeted; `references/` and `scripts/` stay
exempt. *(R1 verify repair, lead-executed within this ruling:)* **two** ledger edits ride
the wave, not one — the exemption line above, **and** the seeding-paths clause gains
"a ruled schema conversion" as a third seeding path (a conversion re-seed is neither a
benchmark-measured winner nor a ruled editorial cut; unadmitted, every re-seed would fail
its own grader). The re-seeded figure carries **no +25% headroom** — a deliberate,
stated departure from the ledger's universal headroom rule: the conversion is a
relocation, never a measured winner.

*(Amended at review, C4 — user-ruled: the protected-content move's ceremony class.)*
Relocating a `KEPT:`-protected or `DECISIONS.md`-traceable line into the pair's own
schema is a **recorded supersession-by-ruling entry citing this session's decision** —
the protection **transfers** to the schema rule, the rule ID inheriting protected status
in the provenance sidecar; never a silent move, never read as deletion. **Explicit
review-feasibility clause:** its whole-body v0.26.0 survivor ruling is superseded by
this ruling into pair form — the survivor protection re-homes onto the pair (body +
schema jointly), recorded once, so no per-line ambiguity survives. GI-005/GI-006 hold:
every protected line traceable through the strip ledger to its new home, reconstructible
in both directions.

*(Amended at review, I6/M6 — user-ruled in the I/M batch.)* **The grading seat is named
(I6):** `mochiko:validator` grades the skill pair — `SKILL.md` + `schema.yaml` — against
the skill-pair criteria block, exactly as it grades a command pair; the "matching
`validation-*`/`review-*` skill" routing never applies to a converted pair (no
validator-for-skills exists, and the pilot's own members never grade themselves).
**GI-012 (M6):** gate 5's `marketplace.json` sync is unaffected — schema files ride the
plugin as data; the schema-data/binary consistency gate does not extend, skill schemas
sitting **explicitly outside the Rust crate's template set** (the content-schema D9
posture), `cargo test` untouched by the wave.

### D9 — Rollout: census-first review-family build; further families by own ruling; repo-level converter skill — census-first + door-open `Confident` · converter-skill + eval-gate limbs `Assumed` *(mark split at review, I8)*

**Statement:** the review family converts in one wave (D1). **Build step 0 is the census
inventory** of the 8 graders: obligation census at D12 grain → the section set minted
(D4), kind membership validated against the nine-kind set, common-block candidates
screened against the R1–R6 bar — the inventory is the audit referent (the ontology I8
idiom). The conversion procedure lands as a repo-level skill
`.claude/skills/converting-skill-to-schema/` (sibling of `converting-command-to-schema`,
never shipped). Each further family — patterns, authoring, testing, analysis, singletons —
converts by its own recorded ruling. Evidence honesty: model-interpreted delivery for
skills is n=0; the D6 first-live-run watch and the D7 compression-eval fidelity gate carry
the honesty, and stage-2 style extensions (phase anchors, global common layer, new kinds)
graduate benefit-keyed only.

**Rationale:** census-first prevents inventing sections/kinds at conversion time (the
command wave's costliest lesson — inventory before extraction); per-family door-open
rollout is the ruled house idiom (content-schema D10). Proposed whole; user adopted as
stated at the Q9 wrap.

*(Amended at review, C4 — user-ruled: protected-set reconciliation first.)* The census's
**literal first step** is protected-set reconciliation (the `compressing-skills` R2
idiom): enumerate every `KEPT:` survivor ruling and every `DECISIONS.md`-traceable line
across the 8 members — 19 `KEPT:` lines across 7 members at review count — **before any
obligation is drafted into a schema**; each protected line gets a named census
disposition: stays-in-body / moves-to-schema / stub-covered (D3 as amended). The
ceremony class of the move is ruled in D8's amendment.

*(Amended at review, I2/I3/I4/I5/M2 — user-ruled in the I/M batch.)* **Census timing
(I3):** the census runs **pre-wave as a paper exercise**, before any build commitment
beyond D1's scope ruling; if it yields materially fewer common blocks than the drift
driver assumes — fewer than three clearing the 3+ bar — the wave returns to the user
with the evidence before any conversion begins. **Census question (M2):** whether any
member asserts a fail predicate — if none, `kind: fail` leaves the skill-side set by
census evidence (kind retirement, symmetric with D4's admission path). **Eval gate
priced (I2):** the judged sample per D7 as amended; spend estimated at census, stated at
the wave gate. **Read cost (I5):** the census states each member's expected per-invoke
read cost (body + schema + bound common blocks); the user accepts it eyes-open at the
wave gate or sets a bound; it joins the first-live-run watch as an observable.
**Rollback (I4):** retreat is **reserved to the user**, its named triggers: the
first-live-run watch showing schema-carried delivery underperforming the prose baseline,
or the eval sample degrading past the pre-registered bar. A retreat retires each pair by
namespace-level tombstone (the content-schema D11 idiom); GI-006 holds — the conversion
is reconstructible in both directions from strips + `DECISIONS.md` + version stamps.

## Session trail

- Session opened 2026-09-01 on the user's `/mochiko:brainstorm` fire ("based on above. i
  want to go through each aspects mentioned") following an in-conversation context sweep of
  the command-schema architecture. Schema pair + `common.yaml` read whole in-session before
  open; ground facts F1–F6 recorded at open. Index entry added (status: open).
- Agenda (walk order, from the context sweep): driver → scope/family selection → pair form
  for skills → rule grammar/ontology → common + extends → `.md`-vs-schema boundary →
  checker/instrument → rollout + governance envelope.
- **Q1 — driver** (A drift-control recommended / B checkability / C addressability / D
  read-cost): user ruled **A primary, B and C named secondary concerns**; D not taken.
  → Problem section.
- **Q2 — pilot corpus** (A review-family recommended / B single-skill / C common-blocks-only
  / D all-38): user ruled **A**. → D1.
- **Q3 — schema home** (A in-dir recommended / B central `schemas/skills/` / C embedded):
  user ruled **as recommended** — in-dir. → D2.
- Lead re-ordered the agenda after Q3: the `.md`-vs-schema boundary is walked BEFORE the
  rule grammar — for skills the boundary decides whether the grammar must express ordered
  procedure at all (sequencing latitude, the walk's sharpest fork).
- **Q4 — boundary** (A obligations-only recommended / B procedure moves as ordered nodes /
  C unordered phase anchors): user ruled **as recommended** — obligations-only, procedure
  stays prose; phase anchors named a graduation candidate. → D3.
- **Q5 — grammar posture** (A inherit-with-deltas recommended, sibling label registry lean
  flagged / B fresh minimal / C skill-native redesign): user ruled **as recommended**. → D4.
- **Q6 — common form** (A per-family flat block file + explicit stubs recommended / B
  whole-section family base / C no common file): user ruled **as recommended**. → D5.
- **Q7 — delivery guard** (A load-first block + floor-count pin recommended / B no pin /
  C inline floor duplication): user ruled **as recommended**; first-live-run watch rides
  the ruling. → D6.
- **Q8 — advisory tooling** (A checker + detector + compression-eval-reuse recommended /
  B plus a skill plan-only eval / C checker only): user ruled **as recommended**. → D7.
- **Q9 — wrap pair, proposed whole** (governance envelope + census-first rollout): user
  adopted **as stated**. → D8, D9. Agenda complete — every walked aspect ruled.
- **Sizing gate:** lead recommended solo cold review (pair and waiver offered with their
  trade-offs, the pair's lens-split coverage evidence stated); user ruled **as
  recommended** — solo.
- **Dispatch:** blind two-message per the charter — message 1 topic statement + goal line
  only, fence declared on the session directory AND on `.mochiko/brainstorms/index.md`
  whole (the ontology session's disclosed index-head leak, prevented structurally this
  time); record path withheld until the Phase 0 angle map returns.
- **Phase 0 map returned:** 38 angles, fence honored (session directory unread, index
  never opened) — grounded in non-session repo reads. Reviewer's own top-five hunt
  priorities: compression-fence collision (skill-compression D5 topology fence vs
  conversion-as-relocation) · budget-by-relocation (schema files exempt from
  primitive-cost budgets) · `references/` as the cheaper already-shipping shared-source
  shape · order-bearing procedure vs unordered rule grammar · delivery-guard mechanism +
  installed-plugin path resolution. Record path sent in message 2 after this entry; the
  record is frozen for the read.

## Review + disposition trail

**Sizing:** solo cold review, user-ruled "as recommended" at the named gate.
**Dispatch:** blind two-message per the charter — message 1 topic + goal only; fence held
on the session directory AND on `.mochiko/brainstorms/index.md` whole (never opened —
the ontology session's index-head leak prevented structurally). 38-angle Phase 0 map
returned before the record path was sent. One disclosed non-substantive drift: the record
grew 302 → 309 lines mid-review (a Session-trail append logging the map's arrival);
Decisions and Open questions byte-identical — disclosed by the reviewer, not raised.
**Verdict:** `critical-gaps` — 4 Critical · 9 Important · 7 Minor survived the reviewer's
own cross-examination; 13 further angles raised and killed (kill list in the review
output — headline kills: ordered-procedure destruction answered by D3 · diamond
inheritance answered by D5 · installed-path resolution answered by D2, called the
record's strongest decision · the compression D5 topology fence binds the compression
pass, not a ceremonied landing). Ground facts F1–F6 all reviewer-verified exact.
**Survivor headlines:** C1 the conversion silently defeats the char-budget gate (schema
data files budget-exempt as "never auto-loaded" — falsified by D6's obligated read) ·
C2 `references/` is an unruled third obligation home (floor-bearing reference files
outside IDs/checker/ceremony) · C3 coverage — the shared-`references/` single-source
mechanism already shipping in the pilot family was never on the Q6 fork · C4 nineteen
`KEPT:` protected lines across 7 of 8 pilot members move with no supersession path
(GI-005). I1–I9 (pin honesty · eval-gate pricing · unmeasured cohesion + census timing ·
no rollback · unpriced read multiplier · unnamed grader · unargued cross-grammar clause ·
unforked D9 marks · Open-questions contradiction) · M1–M7.
**Lead verification of load-bearing review claims (pre-disposition):** budget exemption
line verified verbatim (`primitive-cost-budgets.md:24`) · `KEPT:` counts verified
(review-plan-artifacts 7 · validation-constitution 4 · review-feasibility whole-body
v0.26.0 survivor ruling real) · `CROSS-EXAM.md` "Single source" header line 3 ·
detector's flat-glob discovery (`find-similar-rules.py:133–135`) — all exact.
**Dispositions (2026-09-01, user-ruled):** C1–C4 individually — C1 the budget re-key
(delivered-at-invoke payload budgeted; ledger exemption amended; family common budgeted
once as its own primitive) · C2 the three-home boundary (stub default, lift by census
judgment, Single-source convention sanctioned) · C3 routed **rule inline** at the
coverage gate — the shared-`references/` road recorded and rejected on the B/C secondary
concerns, a live-candidate note left at the patterns-family door · C4 protected-set
reconciliation as the census's first step + the supersession-transfer ceremony,
review-feasibility's whole-body ruling re-homed explicitly. I1–I9 + M1–M7 one batch "as
recommended" — every fold traceable to its finding in the amended decisions (I1→D6 ·
I2/M3/M5→D7 · I3/M7→D1 · I4/I5/M2→D9 · I6/M6→D8 · I7→D5 · I8→D9 mark split · I9→Open
questions · M4→D2 reordered to mint position). M1 (the command path-resolution exposure)
executes at landing as a line on the command D10 first-live-run watch in `BACKLOG.md`.
One bounded verify round over the folds dispatched to the reviewer per the standing
offer.
**Verify round 1 — NOT CLEAN, lead-repaired same round:** 14/14 folds CONFIRMED, none
missing, none contradicting its finding (several graded stronger than asked — C4's
protection-transfer, I6's self-grading guard, I3's numeric census abort, C3's
override-path argument). 2 blocking, both fold-introduced: R1 the C1 ledger edit
under-specified (the seeding-paths clause also crossed — repaired: "a ruled schema
conversion" named as a third seeding path, the no-+25%-headroom departure stated) ·
R2 D3's statement still carried "`references/` files are untouched" against its own C2
amendment (repaired: struck with a supersession marker). 2 nits repaired: N1
owed-at-build enumeration added to Open questions · N2 D1's whole-family "default-FAIL
posture" phrasing qualified beside its own correction. Reviewer self-correction logged:
the review's headline tally was 33 raised (20 survivors + 13 killed), not 32 — the
record's own figures were already consistent. Fitness re-grade: all seven items
checkable, the three `ready`-blockers repaired. Delta-check of R1/R2 requested.
**Verify close — CLEAN (reviewer delta-check):** all four repairs confirmed in place
(R1 both limbs, the third seeding path named against the ledger's real clause; R2 struck
with no dangling dependents; N1/N2 landed); repair-introduced surface: none — three
interactions checked coherent (F5's references-exempt line vs C1 · C2 stubs budgeted
while their reference targets stay exempt, "the right way round" · the struck D3 clause
dependent-free). One non-finding observation logged for the build seats: zero headroom
means a converted skill's first post-conversion character takes the argued-overage path —
the ruled choice, compression-regime R11 precedent. Reviewer status recommendation:
**`ready`**; nothing further owed from the seat.
**Acceptance (2026-09-01):** the user accepted the record after the CLEAN delta-check.
Landing executed whole: `DECISIONS.md` row (D1–D9 as amended) · index entry to accepted ·
ROADMAP Next-row merge · BACKLOG "Skill-content schema build" section (census-first build
item + first-live-run watch) · the M1 path probe added to the command D10 watch · KM close
ritual run (three-part landing; status agreement across record, index, decisions).
- **Build open (2026-09-01, post-acceptance):** the wave opened on the user's "lets
  build". Step 0 per D9 as amended: seat P0 spawned to author the census
  (`census.md`, this directory) — protected-set reconciliation first, obligation census
  at D12 grain, common-block candidates vs R1–R6, numeric abort check, `kind: fail`
  question, per-member read-cost figures, section-set + label-seed proposals, anomalies
  for user ruling. Plan-first per the sound-loop floor; census returns to the user before
  any conversion.
- **Census delivered (2026-09-01, P0 — plan approved with rulings R-a provisional IDs ·
  R-b filename-stem prefixes · R-c protected-set operable bound):** `census.md`, 595
  lines, lead spot-verified (C5 members · C1/C2 wordings · KEPT mention counts — exact).
  Headlines: ~200 rules at D12 grain (181 move · 19 reference-stubs · 0 lifts) ·
  protected-set reconciled FIRST — 9 distinct survivor rulings (3 ended by recorded
  supersession), ~34 live protected units all dispositioned, zero deletes, RF whole-body
  re-home per D8/C4 · **abort check NOT tripped — 6 common blocks clear the 3+ bar**
  (C1 evidence-floor ×7 · C2 default-FAIL ×6 · C3 author≠grader ×8 · C4 verdict-is-input
  ×5 · C5 its-command-states-them ×3 · C6 never-excess ×5) · `kind: fail`: zero run-fail
  predicates — retirement executes by the D9/M2 census-evidence path (8-kind skill set,
  `enforces:` leaves) · section set minted: independence · scope · inputs · verdict ·
  output · reserved (8/8 coverage; command six-set tested and rejected) · labels: 6
  transfers + 3 new (verdict · boundary · fence) · read cost: family **×2.3**
  delivered-at-invoke (~37k → ~84k; RF real read ~25k incl. its obligated lens, J-3) ·
  eval sample RB+RF, 24 grid runs + ~48 judge calls est. · anomalies J-1..J-7, each with
  a recommended disposition. Gate presented to the user.
- **Wave gate ruled (2026-09-01, user: "as recommended, proceed"):** read cost ×2.3
  accepted eyes-open, no bound — joins the first-live-run watch as the I5 observable ·
  eval sample RB+RF approved (pre-registration + ship bar still take their own user
  ratification per the harness) · J-1..J-7 one batch as recommended · `kind: fail`
  retirement + six-section set + 6+3 label seed execute as census-minted
  (pre-authorized, D9/M2 + D4). Conversion wave opens: five plan-first producer seats,
  strictly disjoint ownership — P1 shared files (`skill-labels.yaml` +
  `skill-review-common.yaml`) · P2 pairs RB/RCM/RF/RGI + their strips · P3 pairs
  RPA/RSPEC/RSUF/VC + their strips · P4 tooling (checker + detector extensions, negative
  tests) · P5 ceremony (`primitive-edits.md` skill-pair block · budgets-ledger two-part
  amendment · provenance transfers · post-conversion re-seeds). Fresh author≠grader
  validators follow; gates + landing lead-owned.
- **Plan-approval round (2026-09-01, all five seats):** P4 approved — sibling
  `check-skill-schema.py` (command matrices green by construction), detector skill glob;
  discriminator pinned `kind: skill-common`. P5 approved both phases — provenance kind
  renames `primitive-provenance` in phase B; stamps v0.100.0; phase A DELIVERED and
  lead-verified (criteria block + both ledger sentences verbatim). P1 approved — six
  block texts + nine labels as proposed; block-4 dual label [verdict, user-gate] ruled;
  DELIVERED and lead-verified (1,050 + 1,627 chars). P3 approved — five flags ruled:
  manifest-present reassigned to RSPEC (census correction, landing annotation) · RPA
  fresh author-grader stub as recorded deviation · RPA row-13 D12 split (+1) · VC mints
  24 (census arithmetic) · dotted slugs with census-row→ID maps in strips. P2 approved
  WITH two corrections: numeric census-number IDs rejected — dotted slugs per the
  wave-wide ruling; RB's default-fail stays a LOCAL rule (protected v0.83.0 tail would
  not survive stub inheritance) with the keep-distinct edge queued for the allowlist —
  C2 membership ×6 → ×5 (landing annotation). Missing-row mints (RB/RF C5, RCM C3/C4)
  approved as recorded deviations; a/b limb splits approved; floor pins as planned:
  RB 9 · RCM 3 · RF 9 · RGI 15 · RPA 11 · RSPEC 8 · RSUF 8 · VC 13.
- **Producers complete (2026-09-01):** all five seats delivered, every deliverable
  lead-verified against the artifacts. P4: sibling `check-skill-schema.py` 16 checks /
  84-probe matrix (+ the lead-approved in-wave provenance-anchor addendum, check 16) ·
  detector in-dir extension 42/42 · command matrix untouched 133/133 — all three re-run
  green by the lead's own hand. P2/P3: all 8 pairs built; full-sweep pre-pass **PASS, 0
  findings, 0 warnings**; all 6 common blocks bound; descriptions byte-identical
  (diff-verified); pins as built RB 9 · RCM 3 · RF 9 · **RGI 16** (4c
  `ratification-user-owned` local floor, approved) · RPA 11 · RSPEC 8 · RSUF 8 ·
  **VC 14** (`binary-verdict` split, approved). Recorded-deviation classes: a/b limb
  splits · fresh mints for §C members with no §B row · RB `never-default-ready` LOCAL
  (C2 ×5) · RSUF evidence-floor LOCAL · RPA row-30 both-arms-in-text (no `when:`) ·
  RSPEC row 18 unconditional. P5 phase B: provenance kind →
  `primitive-provenance`, 120 skill entries (all anchors resolving through check 16;
  original protecting rulings anchored, one recorded fallback) · 9 ledger rows re-seeded
  [v0.100.0], third seeding path, no headroom. **Measured payloads (flag for the landing
  gate):** family budgeted payload 106,686 chars vs census est. ~68.6k — real
  delivered-at-invoke multiplier ≈ **×3.2 vs the ×2.3 the user accepted eyes-open**; the
  audits grade structural-overhead-vs-content-growth per member, and the measured figure
  returns to the user at landing. Census-correction + allowlist queues held for landing.
  Three fresh author≠grader validators dispatched over the quiesced tree: V1 (P2's four
  pairs) · V2 (P3's four pairs + P1's shared files) · V3 (P4 tooling + P5 ceremony).
- **Audit round 1 (2026-09-01):** schema content clean everywhere — census fidelity
  reconciles rule-for-rule across all 8 pairs, no obligation missing, none invented,
  descriptions byte-identical, pins exact, no unargued content growth (V1's RB
  reconstruction: +517 grammatical expansion, judged structural; V2: RSPEC 0.88× ·
  RSUF 0.81× · VC 0.91× · RPA 1.20× census-ruled stubs). FAILs cluster in ceremony:
  **V2** RPA/RSPEC/VC FAIL on one cause — P3's strip insertions each overwrote the
  file's prior `## [vX]` heading (GI-005 record corruption, lead-verified via git
  diff) + F4–F7 minors; RSUF + shared files PASS. **V1** RB/RCM FAIL — allowlist
  edges asserted in strips but recorded nowhere (criterion 6) + RCM's deleted
  `## Inputs` left two items unstripped and the diff-location mechanic homeless;
  RF/RGI PASS; 8 further minors (provenance anchor gaps, guard double-homing,
  budget-accounting absence). **V3** unit A PASS (matrices 84/84 · 42/42 · 133/133
  re-run by the validator AND the lead) · unit B FAIL on one blocking wave
  regression — the provenance kind rename broke the live command sweep
  (`check-command-schema.py:969` hard-codes the old kind; fixture-green, tree-red;
  lead-reproduced) + W-1 allowlist/adjudication owed. **Fix rounds dispatched:** P3
  (heading restores + F4–F7 + accounting lines) · P2 (RCM Inputs strip + mechanic
  restore, RB-2/RB-3/RCM-3; RCM-4 ruled wave-wide — no `conditions:` = the `when:`
  grammar sentence legally omitted) · P4 (both-kinds acceptance + the
  lead-found same-block-extends skip gap: detector cluster [1] fires three stubs of
  ONE block) · P5 (four provenance anchors). **Lead-side executed:** allowlist seeded
  with 25 skill-side edges + reasons; deliberately unsuppressed: the evidence-floor
  triple (P4's structural fix) and the external-claims-binding cross-pair family —
  the one genuine adjudication, presented at the landing gate with the measured
  ×3.2 payload figure (V2/V3 both confirm 36,975 → 119,702 delivered-at-invoke).
- **Fix rounds + delta re-grades (2026-09-01):** P3 restored the three strip headings
  byte-exact (git-diff zero deletions — lead- and V2-verified) + F4–F7 + accounting
  lines · P2 restored RCM's diff-location mechanic + the missing Inputs strip entry,
  RB-2 clean MOVE, RB's +517 argued as grammatical expansion, RCM-3's dropped limb
  carried into `dispute-at-checkpoint-only` · P4 fixed the command-checker kind
  acceptance and took A-2 + two minors (matrices 134/134 · 86/86 · 44/44) — and
  correctly HELD against the lead's wrong structural-skip request, proving with an edge
  dump that the same-block skip already works and the live edges run through LOCAL rules
  (the EXTEND-GAP surface by design; quieting rides R6) · P5 four provenance anchors +
  final re-seeds (RSUF 14,950 · VC 13,285 after V2's R1). Allowlist completed in three
  passes (lead 25 → P4 9 → lead 7 + header) — **96 edges, detector 0 clusters**. Delta
  re-grades: V1 RB PASS · RCM PASS · V2 RPA/RSPEC/VC PASS, RSUF holds (F5's
  restore-over-amend endorsed on three named grounds). **All nine graded units PASS.**
- **Landing gate ruled (2026-09-01, user: "as recommended, proceed"):** the measured
  **×3.24** accepted eyes-open — structural overhead validator-verified, RB's +517 the
  only content growth, argued; the read-cost observable rides the watch with the
  user-reserved rollback triggers · the external-claims family ruled **keep-distinct** —
  member-specific object class and pointer path, net-reduction fails after local
  pointers; six allowlist rows; extraction reopens on convergence.
- **Wave landed (2026-09-01, v0.100.0):** gates 4/5/6 executed — CHANGELOG 0.100.0 ·
  plugin.json + marketplace.json 0.100.0 · cargo test 12/12 (crate untouched; skill
  schemas outside its template set per D8/M6); the schema-header runtime-kernel trim ADR
  ships with this bump as its own delivery note promised. Census §K build-corrections
  appendix applied (the ontology-wave idiom). Landing ritual whole: DECISIONS row →
  ruled + built · BACKLOG build item → trail, the first-live-run watch re-scoped to
  measured figures · ROADMAP Next row → BUILT · index updated. Detector residue zero;
  library = six command pairs + eight skill pairs, two grammars, one ceremony.

## Wave-2 family-door rulings (2026-09-01)

Three family censuses ran as parallel paper exercises (in-repo, this directory:
`census-authoring.md` · `census-patterns.md` · `census-small-families.md`), each mirroring
the wave-1 census idiom with the protected-set reconciliation as the literal first step
(D9/C4). Per D9-I3 the abort evidence returned to the user before any conversion; the user
ruled all four gate questions "as recommended" (`Confident`):

- **Wave shape — three sequential waves**, authoring → patterns → small families. Each wave
  carries its own build, author≠grader audits, version bump, and landing ritual, preserving
  D9 rollback granularity; a single 22-pair landing was declined for conflating rollback.
- **Authoring (8 members, all convert).** Abort not tripped: 4 blocks clear the 3+ bar
  (letter-IS-the-spirit epigraph ×4 · produced-artifact-graded-independently ×4 · two-arm
  template-schema binding ×4 · deliverable-envelope binding ×5) → new
  `plugins/mochiko/schemas/skill-authoring-common.yaml`. The family section set mints
  **`artifact`** (the produced artifact's binding grammar, invariants, and write mechanics)
  in place of `verdict` — census J-1: `verdict` is empty for all 8 producers and ~124
  artifact-grammar rules had no six-set home — and `primitive-edits.md` criterion 2 is
  amended to per-family section sets (each minted by that family's census, uniform within
  the family, explicit empty markers legal); the amendment rides the wave as a ceremonied
  shipped-rule edit. Labels minted: `artifact-grammar`, `single-home`.
  INTERROGATION-AGENDA.md stays un-stubbed by ruling (J-2: its obligations bind the setup
  session, not this skill; a stub would dangle from no §B row). Payload accepted as a band,
  not a point: est. ×2.35 family delivered, honest band ×2.9–×3.3 on the wave-1
  estimate-to-measured drift precedent; the measured figure returns at the landing gate.
- **Patterns (9 of 13 convert).** The shared-`references/` road is **rejected at its
  door** — the D5/C3 live-candidate note is discharged. §ROAD evidence: the judgment-prose
  premise holds for only 4 of 13 members (the 2026-08 floor-skill births changed the
  family's character), and the road's F4 arithmetic fails by an order of magnitude
  (~26–52k added family-wide to consolidate under 400 chars); the per-member Single-source
  convention stays the sanctioned mechanism for the teachers' reference content. The abort
  evidence is borderline (strict R1 reading: 2 blocks — tripped; thin-core reading: exactly
  3) and the ruling proceeds **on the secondary drivers** (citable IDs, floor semantics,
  checker visibility) for the nine discipline carriers: adopt-first · architecture-shelves
  · code-minimalism · map-minimalism · model-tiering · plan-minimalism · sound-loop ·
  transport-floor · vertical-tdd. Section set minted: `trigger · scope · discipline ·
  inputs · disclosure · reserved` (13/13 coverage-tested, census §B). **No common file**
  — the 2–3 thin candidates fail R5 net-reduction. Labels minted: `trigger`, `ladder`.
  The four teachers (api-contracts · entity-modeling · technical-decisions · system-design)
  stay prose; revisit on evidence.
- **Small families (dense five convert).** Abort TRIPPED — zero common blocks in any
  micro-family; the ruling proceeds eyes-open on the B/C drivers for the five
  obligation-dense members: testing-gap-finding · executing-tdd-cycle · testing-end-user ·
  brownfield-integration · analysis-codebase. The review six-set is reused with explicit
  empty markers; no common file. analysis-iterative · grooming-operating-docs ·
  testing-governance-injection stay prose with their floors protected as today.

Cross-cutting census findings carried into the waves: ~30 pre-listed allowlist edges
(letter-IS-spirit, strict-order, rationalization-STOP, author≠grader mirrors, envelope
density) · two families independently converging on author≠grader and envelope-density
shapes is the first live signal for the D5 cross-family `skill-common.yaml` graduation
candidate (evidence only, not actioned) · analysis-codebase's dangling "indicators below"
pointer takes a one-line ruled repair riding its wave, never a silent fix.

### Wave 2A — authoring family BUILT (2026-09-01, v0.101.0)

- **Build:** 4 plan-approved producers under disjoint ownership (shared surfaces · 3+3+2
  pairs, the pairs-C seat doubling as single-writer closer for provenance/allowlist/ledger);
  one mid-wave session-limit crash, all four seats resumed against lead-verified tree state.
  Delivered: 8 in-dir pairs (244 rules · 67 floors · 16 reference stubs · floor pins
  9/12/10/16/4/4/8/4 with read-backs) · `plugins/mochiko/schemas/skill-authoring-common.yaml`
  (4 blocks C-A1–C-A4) · labels registry +2 (`artifact-grammar`, `single-home`) · checker
  per-family section sets (prefix-derived; the authoring set swaps `verdict` for `artifact`)
  + detector authoring-library resolution · `primitive-edits.md` criteria 1/2/6 amended ·
  provenance sidecar 389 anchors · allowlist 96→157 with detector at 0 clusters · ledger 6
  re-seeds + AAS/AE first-seeds via the third seeding path, no headroom, common file budgeted
  once (1,285).
- **Audits (3 fresh author≠grader validators, 10 units):** round 1 — 8 PASS, AC FAIL (four
  protected "Shape N" template-reference tokens dropped from rule texts; restored verbatim-
  in-substance, strip artifact-count corrected 18→20 with row-by-row enumeration), AUS major
  (3 floors without sidecar anchors; added, plus 2 fallback anchors on Overview-born floors).
  V3's label finding ruled: the epigraph block's `[artifact-grammar]` dropped per census §I
  with a narrow probe-covered checker carve (inherited-absence warns, local-empty fails);
  V2's crossing-state BLOCKING on that same drop overruled with the ruling trail and accepted
  on V2's own re-verification. Fix-round payload drift re-measured same round (the wave-1
  V2/R1 lesson applied). Delta re-grades: **ALL TEN UNITS PASS.**
- **Measured at landing:** family delivered-at-invoke **150,576** vs 81,896 pre-conversion =
  **×1.84 — under the census ×2.35 estimate; the wave-1 estimate-to-measured drift did not
  recur.** Matrices 105/105 · 134/134 · 48/48; sweeps 16/16 skill pairs · 6/6 command pairs.
- Landing ritual: census §K build-corrections appendix applied · DECISIONS row → wave 2A
  built · BACKLOG first-live-run watch extended to the authoring family · ROADMAP Next
  touched · index updated · CHANGELOG 0.101.0 · marketplace synced · cargo test (gate 6).
  Remaining waves per the door rulings: 2B patterns (9 carriers) · 2C small families (dense
  five), each census-first with its own landing.

### Wave 2B — patterns family (9 carriers) BUILT (2026-09-01, v0.102.0)

- **Build:** 4 plan-approved producers, disjoint ownership (shared surfaces · 3+4+2 pairs,
  the pairs-C seat again single-writer closer). Delivered: 9 in-dir pairs (124 rules ·
  46 floors · 3 reference stubs · pins 7/5/3/3/2/4/6/11/5 with read-backs) · the minted
  patterns section set `trigger · scope · discipline · inputs · disclosure · reserved` ·
  **no common file** (§ROAD held; zero `extends:` family-wide, checker-enforced) · labels
  +2 (`trigger`, `ladder`; registry 13) · checker library-less-family mechanics (matrix
  105→114) · criteria 1/2 + Rulings block amended · provenance 118 new anchors (507 total,
  every floor anchored, six deliberate must-absences named) · allowlist 157→181 distinct
  rows, detector 0 clusters over 912 rules · ledger 2 re-seeds (VT's +294 HOLDS absorbed)
  + 7 first-seeds, no headroom. Two strip files born (AS, TF — census J-P2).
- **Class-mix rulings at plan approval (row grain beats tally, four times):** AF 7 floors ·
  CM 3 · TF 11 (version-floor overruled up — the table, §D, and the lead's own brief all
  under-counted) · VT 5 (two-arm promoted, §D's explicit naming). All in §K.
- **Audits (3 fresh author≠grader validators, 11 units):** round 1 — 9 pairs PASS with
  1 major (AS display-for-override obligation lost at census row grain — restored) +
  11 minors; W3 unit B FAIL on three trail-accuracy majors (stale quiesced-tree claim ·
  one-of-six deliberate absences named · duplicate allowlist row) — all repaired; one VT
  strip heading overwrite (the GI-005 class) caught by a SIBLING SEAT mid-wave, restored
  byte-exact, verified thrice. Delta re-grades: **ALL ELEVEN UNITS PASS.**
- **Measured at landing:** family delivered-at-invoke **95,858** vs 50,379 = **×1.90**,
  +5.1% over the ×1.81 estimate, inside the ±25% band. Matrices 114/114 · 134/134 · 48/48;
  sweeps 25/25 skill pairs · 6/6 command pairs.
- **Cross-family graduation evidence:** the 4-member read-before-claim convergence
  (AFM · MM · RB · RSPEC) recorded as the strongest D5 `skill-common.yaml` candidate signal
  yet — evidence only, suppressed on D5 grounds.
- Landing ritual: §K appendix · DECISIONS row → 2B built · BACKLOG watch extended ·
  ROADMAP · index · CHANGELOG 0.102.0 · marketplace synced · cargo (gate 6). Remaining:
  wave 2C small families (dense five). The four teachers (api-contracts · entity-modeling ·
  technical-decisions · system-design) stay prose by ruling; revisit on evidence.

### Wave 2C — small families, dense five, BUILT (2026-09-01, v0.103.0) — THE ARC CLOSES

- **Build:** 3 plan-approved seats (2+3 pairs · a shared/closer seat). Delivered: 5 in-dir
  pairs (104 rules · 35 floors · 11 reference stubs · pins 7/9/10/6/3 with read-backs) under
  the REUSED review six-set with explicit empty markers (no set minted — the door ruling) ·
  **no common file and zero `extends:`** (abort was TRIPPED; the checker cannot enforce
  zero-extends for fall-through stems, so the bar is procedural — held and audit-verified) ·
  criteria criterion-2 reuse sentence + census citation · provenance 90 new anchors (597
  total; all 35 floors anchored; four named deliberate must-absences) · allowlist 181→214
  distinct rows, detector 0 clusters over 1,016 rules — **the no-allowlist run confirmed
  zero within-micro-family 3+ clusters: the census's zero-blocks finding held at landed
  grain** · ledger 4 re-seeds + TGF first-seed, no headroom. J2-8's KEPT dual-homing twins
  shipped as ruled (stubs point at their own reference homes; reconciliation paragraphs;
  2026-08-01 anchors); J2-9's dangling-pointer repair executed as its own disclosed entry.
- **Class-mix rulings:** TEU 7 floors · TGF 9 (row grain, applications six and seven) ·
  BI 6 via the [v0.49.0]-keep-set row-7 promotion (protected-set naming beats the cell —
  the 2B VT precedent); TEU 23 / TGF 31 rules via census-marked a/b splits.
- **Audits (X1 · X2 · X3→X3b, 7 units):** all 7 PASS — zero blocking across the whole wave,
  a first for the arc. One validator seat went silent twice and was respawned fresh (X3b).
  Six citation-class minors fixed in one micro-batch. X3b's standout: it REFUTED a sibling
  validator's queued ledger "correction" by proving the "was X/Y" notation is a prior-row
  citation (the same-wave AC row's 104-char gap as proof) — the fix was cancelled before it
  corrupted a four-wave convention. Author≠grader catching the grader: the loop works in
  both directions.
- **Measured at landing:** family delivered **81,799** vs 46,809 = **×1.75**, +4.2% over the
  ×1.68 estimate, in band. Matrices 114/114 · 134/134 · 48/48; sweeps 30/30 skill pairs ·
  6/6 command pairs.
- **THE WAVE-2 CONVERSION ARC IS COMPLETE:** 30 converted pairs of 38 skill directories —
  8 review (v0.100.0) · 8 authoring (v0.101.0) · 9 patterns carriers (v0.102.0) · 5 dense
  five (v0.103.0); 7 stay prose by ruling (analysis-iterative · grooming-operating-docs ·
  testing-governance-injection · the four patterns teachers); the `mochiko` router out of
  scope (D1). Four grammars' worth of family section sets (review · authoring · patterns ·
  review-reused), two family common files, one label registry at 13, 597 provenance anchors,
  214 allowlist rows at zero detector residue. Measured multipliers by family: review ×3.24 ·
  authoring ×1.84 · patterns ×1.90 · dense five ×1.75.
- Landing ritual: §K appendix · DECISIONS row → 2C built + arc closed · BACKLOG watch
  extended to all four families · ROADMAP · index · CHANGELOG 0.103.0 · marketplace synced ·
  cargo (gate 6). Standing after the arc: the first-live-run watch (all four families'
  read-cost observables) · the D7 compression-eval judged sample (own ratification) · the
  D5 cross-family `skill-common.yaml` graduation candidate (evidence accumulating: the
  author≠grader/envelope convergence from 2A + the 4-member read-before-claim convergence
  from 2B) · the prose seven revisit on evidence.

## Open questions

- None blocking. Named graduation candidates (not open questions): phase anchors (D3) ·
  cross-family global `skill-common.yaml` (D5 — first live signal 2026-09-01: the
  authoring and small-families censuses independently converge on author≠grader and
  envelope-density shapes; evidence only) · kind admission/retirement by census
  evidence (D4/D9). The shared-`references/` road at the patterns-family door was
  discharged 2026-09-01 — rejected by ruling (see Wave-2 family-door rulings). Standing
  watches owed at build: the first-live-run delivery watch with the I5 read-cost
  observable (D6/D9) · the M1 command path-resolution note on the command D10
  watch (`BACKLOG.md`).
- Owed at build, enumerated once (each durable in its decision — N1): the two-part ledger
  amendment incl. the third seeding path (D8/C1) · the `primitive-edits.md` skill-pair
  criteria block with the budget and grader clauses (D8) · the detector in-dir sweep
  (D7/M3) · the census's protected-set first step (D9/C4).
