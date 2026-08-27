# Command-schema ontology — decision record

**Topic:** the command content schemas' node ontology — today every content block is one
`rule` shape (`{id, labels, class, text, pointer?}`, command-content-schema D6/D14) — plus
first-class conditional execution (run-shape branches now living in prose) and dependencies
between rules (sequencing and cross-references now living in prose adverbials and unchecked
parenthetical citations). Driver: the user, reading the shipped schemas — "everything is a
rule? we need to improve ontology. Also, i want to look at conditional execution and
dependencies in between."

**Status:** accepted (2026-08-27)
**Opened:** 2026-08-27
**Lead:** session lead (brainstorm charter, run inline in-conversation)

**Prior-session relations:** amends, if ruled, the `command-content-schema` grammar
(D6 rule-block grammar · D14 nested sections · D15 referential closure · D16 runtime-only
schemas) — all six pairs converted and shipped; HEAD at v0.97.0 after the scaffold wave
*(version corrected at review, M1)*. That session's D2 stage-1/stage-2 split and its
unfired absorption trigger are framed against this session by D11 *(review fold, I9)*. Composes with the
`command-plan-only-eval` instrument (built 2026-08-27; its baseline grid ran this
session — F7 as corrected, C1/B1) and with the `extends: common.*` prototype in the
working tree (Option A of the rule-similarity dive, 2026-08-27 — ruled here as D8). The
`command-md-scaffold-standardization` D4/D5 six-set (one day old) constrains any section
restructuring.

---

## Ground facts

All facts verified this session by whole-file reads of all six command schemas
(`plugins/mochiko/schemas/{implement,feature,specify,architecture,setup,brainstorm}.yaml`),
the checker (`scripts/check-command-schema.py`), and the cited records.

- **F1 — grammar state.** One node shape for all content: rule blocks
  `{id, labels, class, text, pointer?}` nested in the six-set sections (D6 as amended by
  D14/D15/D16). Corpus at v0.97.0 *(corrected at review, M1)*: 320 rules — impl 104 ·
  spec 51 · feat 49 · arch 47 · setup 40 · brainstorm 29 (command-content-schema record, rollout outcome; feat/setup
  counts independently re-counted this session, exact). Working tree adds the
  then-unruled `extends:` prototype on the implement pair + `common.yaml` (6 shared
  blocks; ruled downstream as D8 — tense aligned at verify close, cosmetic residual).

- **F2 — three axes tangled.** The corpus discriminates content three ways, none of them a
  kind field: `class` carries bindingness (floor/must/advisory), `labels` carry topic
  clusters that do double duty as weak types (`user-gate`, `fail-condition`, `binding`
  labels ≈ latent kinds), and the six-set sections mix axes — `boundaries` is class-keyed,
  `tools` mostly binding-keyed, `fail-conditions` kind-keyed, `reserved` holder-keyed,
  `roles`/`ways-of-working` topic-keyed.

- **F3 — kind census (full-corpus walk).** The walk censused seven recurring buckets
  (gate/reservation counted as one). *(Corrected at review, I10 — the "no rule resisted
  classification, no eighth kind emerged" claim over-reached:)* ~8 latitude-grant rules
  (the five `*.staffing-latitude` rules, `spec.lead-latitude`, `impl.design-seats-staffing`,
  and candidates the I8 inventory classifies) are grants of judgment, neither obligation
  nor prohibition — a residue the census under-reported, resolved by D1's `latitude` kind;
  and the gate-vs-reservation split D1 adopted was not itself census-validated — it is
  validated in the I8 inventory before conversion. The buckets:
  - **constraint** — standing obligation/prohibition; the largest class everywhere.
  - **duty** — an action the lead always performs. The desk DM sets are the clean case:
    arch carries 9 `dm-*` duties at `class: floor` ("Surface health before the ask",
    "Close the visit with a verdict"…), feat 8, impl 4–6 run-side analogues, setup's
    scaffolding obligations. The latitude rules themselves draw the duty/latitude
    boundary (`feat.staffing-latitude`: "The bare minimum that must always happen is
    carried as the Delivery Manager rules … everything beyond it is your per-visit
    judgment") — duties are the ruled complement of latitude, not a collision with it
    (charter-ritual-balance's "bare minimum" made addressable).
  - **gate / reservation** — blocking checkpoint vs standing decision ownership. Run
    commands carry individual gates (`impl.gate-design-checkpoint` …); desks carry
    aggregate reservation rows (`feat.user-reserved`, `arch.user-reserved-rulings`,
    `spec.reserved-to-user` — single blocks bundling many holdings, in tension with the
    D12 one-obligation grain).
  - **binding** — what lives where, shaped by what. The render-degrade formula
    ("rendered by `mochiko-cli template X`, or its schema Read raw when the binary is
    absent — the shipped schema is the first-class source of truth") sits in **7 rules
    across five command schemas, 8 occurrences** *(counts corrected at review, C2 — the
    open's "~8+ sites" counted occurrences; the review's own 6/4 missed
    `impl.cards-template`)* — and **zero of the seven share a whole text**, so
    whole-field inheritance cannot lift the shared clause (the fact that struck this
    candidate from D8).
  - **bound** — numeric budget + redeclaration point + exhaustion route. Implement-only
    (`impl.attempt-per-grade`, `impl.gap-rework-bound`, `impl.no-progress-stop`).
  - **routing** — event/demand → destination with defaults. Desk-heavy: /mochiko:feature
    is a routing desk (`feat.stable-ground-triage` is a literal three-branch decision
    table: delivered → delta lane · in-flight → file to owning run · no owner → product
    lane); escalation routings appear in all six.
  - **fail** — end-state predicate, already segregated by `<cmd>.fail.*` segment and
    label. Desk pattern: exactly 1 (per-visit done condition); run pattern: 4–15.

- **F4 — conditionality measured.** *(Re-measured at review, M4 — the open's "117
  conditional-marker lines / 43 implement hits" were line-grep figures with an unstated
  marker list, superseded by these per-rule counts:)* 74 of 320 rules carry a guard
  marker, 78 a moment marker, 27 a branch set, 176 carry none; only **16 rules open with
  a guard** — the remainder embed conditions mid-sentence as exceptions and carve-outs,
  which D3 routes to stays-prose. Branch dimensions:
  scope (selection | delta | epic | lane — implement), mode (greenfield | brownfield |
  amend — setup), depth (low | high — implement only; `arch.no-depth-dial-coupling` is a
  floor forbidding the desk to read it), UX-bearing (specify), surface presence (KM file ·
  feature map · store ruled-content · governance region · codebase-analysis · the binary),
  multi-seat composition (transport-floor trigger, all six). Resolution classes observed:
  entry-derived (scope) · surface-presence (checked when relevant) · **moment-resolved**
  (UX-bearing is ruled at the intent stage and conditions the rules gated on it) ·
  **user-ruled**
  (`setup.user-mode-ruling`: ambiguous mode is the user's ruling) · standing trigger
  (multi-seat). A conditions grammar must carry the resolution point, not just the values.

- **F5 — dependency state.** Two forms, split by conversion era. Implement and brainstorm
  (converted before D15): **zero** in-text rule-ID citations — sequencing rides prose
  moment adverbials only. The four D15-era conversions carry **19 parenthetical ID
  citations** (feat 7 · setup 6 · arch 4 · spec 2) *(corrected at review, M2)*, e.g.
  `(spec.confirm-frame-post-stories)` as a sequencing anchor — a reference idiom that
  emerged organically under D15's deixis ban; the population D5's scan covers — every
  `<cmd>.*` ID token in any rule text — is **29** at HEAD (impl 2 · feat 9 · setup 7 ·
  arch 6 · spec 5), including five `spec.md` file-path lookalikes the scan must exclude
  (M3). All 19 resolve today (verified), but the
  D13 checker never checks them (deixis lint + section-ID + sidecar-anchor resolution
  only) — a tombstoned target would dangle silently. The moment vocabulary the adverbials
  reference (run-open · entry · design checkpoint · card confirm · cycle checkpoint ·
  final validation · landing · acceptance · visit-open · close) has no first-class
  existence anywhere.

- **F6 — coverage holes found by hand** (the defect class a declared-conditions grammar
  would catch deterministically):
  1. implement's landing rules cover selection/epic/delta scope
     (`impl.landing-selection` / `-epic` / `-delta`) — **lane scope has no landing
     rule**.
  2. implement's Not-done pin counts 15 "any one standing", but two members are
     scope-conditional and mutually exclusive (`impl.fail.gap-finding-missing`
     selection/epic-only · `impl.fail.skip-unstated` delta/lane-only) — the pin is
     honest, the set's conditional activation is undeclared.
  3. setup's feature-map obligations and `setup.fail.no-feature-map` branch on
     greenfield/brownfield — **amend mode is unaddressed** in both.

- **F7 — instrument and in-flight state.** The plan-only eval (command-plan-only-eval
  D1–D11, built 2026-08-27, commits f420695/99d219e) is the measurement instrument for
  exactly this class of edit; its D8 plan-observable-vs-contingency partition is
  hand-authored today and would be largely derivable from kinds (fail/routing/exhaustion
  routes = contingency); its D6 changed-text fourth bucket absorbs a mass reword.
  *(Corrected at review, C1 — the open-state "its pre-edit baseline grid has not yet
  run" was already false at the record's final write:)* the grid RAN this session —
  `evals/commands/implement/runs/optionA-grid/` holds a completed 18-run pre/post grid
  (`old_ref: HEAD`, 3 scenarios × 3 replicates × 2 arms, $13.0973, runs pinned plugin
  v0.97.0). Its pre arm is the reusable HEAD baseline; its post arm is the six-block
  `extends:` prototype; `evals/commands/run.py` was extended (uncommitted) with
  `extends:` resolution to support it — and its differ resolves `extends:` **before**
  diffing, so that grid's four-bucket diff reads the prototype as unchanged (a stated
  instrument limit, I1). The prototype itself sits uncommitted on the implement pair.

- **F8 — hygiene find (minor).** `setup.baselines-bootstrap` opens its shipped text with
  an inline `` `Assumed` (feature-sizing record, open thread 4 …) `` confidence mark and
  record citation — provenance in a shipped schema, the class D16 moved to the
  `.mochiko/provenance.yaml` sidecar.

## Lead's opening position (pre-questioning, revisable)

Carried in from the pre-session analysis, amended by the walk: promote `kind:` as a closed
field (sections stay the D4/D5 six-set; kind crosscuts them) · declare per-schema
`conditions:` with resolution points and a `when:` field, condition single-homed ·
declare per-schema `moments:` with an `at:` field · link fail mirrors via `enforces:`.
**Amended by the walk:** (a) a `duty` kind is now recommended — the earlier "don't mint
step/duty" position is overturned by F3's evidence that duties are the ruled complement of
latitude, not a collision; (b) the dependency recommendation sharpens from "defer
entirely" to "check what exists": the 18 live citations (F5) deserve checker resolution
now, a structured `requires:` field still waits for a consumer.

## Decisions

### D1 — Kind set adopted: nine kinds, `constraint` the omitted default — ship `Confident` · efficacy `Assumed (n=0)` *(mark split at review, I2; ninth kind at I10)*

**Statement:** every rule block gains an optional `kind:` field from a closed set of
eight: `constraint` (the default — an absent `kind:` reads `constraint`) · `duty` (an
action the lead always performs — the always-happens floor made addressable) · `gate`
(a blocking checkpoint) · `reservation` (standing decision ownership without a scheduled
checkpoint) · `binding` (what lives where, shaped by what) · `bound` (numeric budget with
redeclaration point and exhaustion route) · `routing` (event/demand → destination with
defaults) · `fail` (end-state predicate; the `<cmd>.fail.*` segment). Orthogonality
preserved: `class` keeps bindingness (floor/must/advisory), `labels` keep topics, the
six-set sections stay per scaffold-standardization D4/D5 — kind crosscuts sections. This
amends command-content-schema D6.

**Rationale:** the F3 walk census — the corpus classifies into these kinds *(census
corrected at I10: the latitude residue and the un-validated gate/reservation split are
named there — the original "no residue, no eighth candidate" claim retracted, verify
nit 1)*; duties are the ruled complement of lead latitude
(the latitude rules draw the boundary themselves), so the earlier don't-mint-duty
position was overturned on evidence; desk aggregate-reservation rows get a kind of their
own so the D12 grain review has a name for them. User ruled the recommended option at
the Q1 fork.

*(Amended at review, I10 — user-ruled in the I/M batch:)* a ninth kind **`latitude`** is
minted — a grant of per-run/per-visit judgment, neither obligation nor prohibition; the
census had silently defaulted ~8 such rules to `constraint` (membership per F3 as
corrected, finalized in the I8 inventory), and the eval's hand partition already carries
`latitude` as a reason class. The gate-vs-reservation split — the one census bucket D1
subdivided — is validated against the corpus in the I8 inventory before any conversion.

### D2 — Kind ships as a tag; per-kind fields graduate benefit-keyed — ship `Confident` · efficacy `Assumed (n=0)` *(mark split at review, I2)*

**Statement:** this wave ships `kind:` alone; `text` stays the sole content carrier. The
kind-specific structured fields surveyed at the fork (gate `holder:`, bound
`value:/redeclare:/exhausted:`, routing structured branches, binding render-degrade
structure) are pre-authorized to graduate later, each keyed on a demonstrated consumer —
a checker assert actually written, or a plan-eval rubric use — landing as an ordinary
build citing this record (the staged idiom of command-content-schema D2/D4). Universal
fields (`when:`, `at:`, `enforces:`) are separate decisions, not covered here.

**Rationale:** plan-minimalism — the tag alone already serves the named consumers (eval
D8 partition derivation, kind-keyed audit criteria, checker per-kind asserts); fields
without a written consumer would be structure on spec. Smallest churn over 320 rules.
User ruled the recommended option at the Q2 fork. *(Review fold, I5: the `at:` field
joins this graduation set — see D4 as amended.)*

### D3 — Conditions: per-schema declared dimensions + `when:` field, single-homed — ship `Confident` · efficacy `Assumed (n=0)` *(mark split at review, I2)*

**Statement:** each command schema gains a top-level `conditions:` block (sibling of
`vars:`) declaring every run-shape branch dimension it uses: the dimension name, its
closed value set (or `presence` for surface-existence flags), and its **resolution
point** — one of entry-derived · surface-presence · moment-resolved (names the moment) ·
user-ruled · standing-trigger. Rules gain an optional `when:` field: a conjunction of
`dimension: value` / presence terms, declared vocabulary only — no boolean algebra, no
negation beyond a declared value, no free-form strings. **Single-homing:** rule-level
activation conditions live in `when:` alone and leave the `text`; intra-rule branches
(several arms inside one obligation, e.g. `feat.stable-ground-triage`) stay prose — the
field carries whether the rule binds, never its internal logic. Checker duties: every
`when:` term resolves against the declared block; unused declared dimensions flagged;
a per-dimension coverage report (advisory, warning-class) enumerating which rules
activate per value — making the F6 hole class (lane landing, amend-mode feature-map,
conditionally-active fail pins) deterministic findings instead of hand-audit luck.

**Rationale:** the F4 evidence — dimensions and resolution classes are enumerable and
small; free-form values would give the checker nothing to resolve (the D8-registry
lesson applied to conditions); declare-only would defer the entire activation-filtering
and coverage benefit. Runtime gain: a lead can deterministically shed inactive rules for
the run's declared shape. User ruled the recommended option at the Q3 fork.

*(Amended at review, C4 — user-ruled: floors are never shed:)* the shedding rationale
applies to non-`floor` rules only. A `class: floor` rule is **always read and always
delivered** whatever its `when:` — `when:` on a floor gates the obligation's
application, never its delivery. Any shed set is re-evaluated when the run's shape
changes: a seat added mid-run re-activates every `when:`-gated rule its addition
touches (the live case: the three `transport-floor` rules whose guard is seat count).
The checker makes no coverage claim over floors. Without this clause, D3's rationale
reproduced the exact retreat trigger command-content-schema D2 names — "floors missed
on the schema side."

*(Amended at review, I6:)* the moment-resolved resolution class is defined as "resolved
at the named moment; rules gated on it are inapplicable until it resolves." The word
"later" and any order-verification claim are dropped: D4's moment list is unordered, so
precedence between moments is not checkable and is not claimed — the checker verifies
naming only.

*(Amended at build, inventory anomaly J-1 — user-ruled:)* single-homing gains a second
disposition. **MOVE** (the default): the condition leaves the text, a strip records the
clause. **DECLARE**: where the condition rides the rule's subject noun and extraction
would create deixis or falsify the text (six rules at conversion: `impl.lane-never-widens`
· `impl.epic-shared-baseline-single-pen` · `spec.map-obligated-read` ·
`spec.missing-map-surfaced` · `spec.governance-region-absent` ·
`spec.whole-feature-prototype`), `when:` is added and the text stays unchanged — a pure
addition, no strip owed. The `when:` term is data; the subject noun is not a second home
but the obligation's own referent.

### D4 — Moments: per-schema declared anchors, unordered; `at:` deferred to graduation — ship `Confident` · efficacy `Assumed (n=0)` *(mark split at review, I2; `at:` deferral at I5)*

**Statement:** each command schema gains a top-level `moments:` block (sibling of
`vars:`/`conditions:`) declaring the run's named anchor points — name plus one
navigation line each (run commands: run-open · entry · design checkpoint · card confirm ·
cycle checkpoint · final validation · landing · acceptance as implement uses them; desks:
visit-open · close; each schema declares only the moments it uses). Rules gain an
optional `at:` field naming one declared moment. The declared list is **unordered** —
relative sequence stays the `.md` protocol's narrative and the lead's latitude; `at:`
anchors an obligation to a moment, it never schedules moments against each other. D3's
moment-resolved conditions name their resolution moment from this vocabulary. Checker
duties: every `at:` and every moment-resolved condition resolves against the declared
block; unused declared moments flagged; a per-moment listing (advisory) of anchored
obligations.

**Rationale:** F5 — the moment vocabulary already carries most real sequencing
information as prose adverbials with no addressable existence; declaring it is
near-zero grammar cost and gives duties, landings, and D3 their anchor. Ordering was
declined deliberately: an ordered moment list is the workflow-engine direction —
it would brush the lead's ruled sequencing latitude and add power with no consumer.
User ruled the recommended option at the Q4 fork.

*(Amended at review, I5 — user-ruled in the I/M batch:)* the `moments:` block ships —
its demonstrated consumer is D3's moment-resolved conditions, a dependency inside this
same wave. The **`at:` field does not ship**: its only listed consumer was an advisory
per-moment listing, which fails D2's own demonstrated-consumer bar stated in the same
record. `at:` joins the D2 graduation set (keyed on a written checker assert or a
plan-eval rubric use); until it graduates, duties anchor to moments in prose, and every
`at:` reference elsewhere in this record reads as deferred with it.

### D5 — Dependencies: in-text ID citations become checked references; no `requires:` field — ship `Confident` · efficacy `Assumed (n=0)` *(mark split at review, I2)*

**Statement:** the citation idiom the D15-era conversions grew — a parenthetical rule-ID
token `(cmd.slug)` inside rule text as a sequencing or definition anchor — is ratified as
the dependency form, and the D13 checker gains its resolution check: every `<cmd>.*` ID
token appearing in any rule text must resolve to a live node in the same schema, or to a
tombstone (flagged as superseded-reference, error-class like a dangling section pointer).
The 19 existing citations *(count corrected at review, M2)* become checked references —
the scan covers the full 29-token population per F5 as corrected, excluding file-suffix
tokens (`.md`/`.yaml`) so the five live `spec.md` path mentions never false-positive
*(M3)*; implement and brainstorm may adopt
the idiom at their next touch, not as an owed sweep. A structured `requires:` field is
**named as a graduation candidate, not minted** — keyed on a real consumer arriving
(e.g. the plan-only eval growing an ordering axis), landing as an ordinary build citing
this record.

**Rationale:** F5 — the idiom already exists organically (19 sites, per F5 as
corrected) and carries the
dependency information the corpus actually needs; checking it closes the silent-dangle
defect class for one small deterministic check. A `requires:` graph today would be
structure with zero consumers and the strongest pull toward workflow-engine posture the
session surveyed. User ruled the recommended option at the Q5 fork.

### D6 — Fail nodes carry `enforces:` mirror links — ship `Confident` · efficacy `Assumed (n=0)` *(mark split at review, I2)*

**Statement:** every `kind: fail` node gains `enforces:` — a list of one or more local
rule IDs naming the gate/floor/duty it is the end-state contrapositive of. An empty
`enforces:` is legal only with a one-line reason (the weak-mirror case: the obligation
lives in a pointer skill, e.g. `setup.fail.unclosed-trace`), so absence is always a
statement, never an omission. Checker duties: every listed ID resolves (tombstoned
target = error, per D5's semantics); an advisory reverse-coverage report listing
`class: floor` rules and gates no fail node enforces. The pair audit's FAIL-survival
criterion gains the structural handle. Unlike the D2-deferred per-kind fields, this
field ships now because its consumer already exists — the audit's FAIL-survival and
floor-must-survive checks.

**Rationale:** F3/F5 — the mirror mapping was demonstrated across all six commands by
hand this session; making it data costs one list field on 36 fail nodes and closes the
"which floors have no fail mirror" question deterministically. User ruled the
recommended option at the Q6 fork.

*(Amended at review, I3 — the D16 test applied:)* command-content-schema D16 — "a
command schema carries only what the run consumes" — is the standing bar this field
must pass, and the record now argues the carve instead of assuming it: `enforces:` has a
run consumption — at Not-done reporting, a standing fail condition cites the obligation
it enforces, so the link is acted on in-run, unlike the D16-evicted `ruling:` anchors.
The reverse-coverage report remains checker-side.

*(Amended at review, I7:)* the reverse-coverage report is **input to the user-deferred
Desk FAIL-set widening pass** (BACKLOG 2026-08-27: "separate ruled pass … never a
silent widening") and is never audit pressure — the pair audit's FAIL-survival handle
covers the **existing** fail sets only. The review's measurement (74 `class: floor`
rules corpus-wide with no fail mirror: arch 21 · impl 19 · feat 12 · setup 12 · spec 7 ·
brainstorm 3) is that deferred pass's evidence, not this wave's work.

### D7 — Sequencing: reuse the completed HEAD baseline; one combined amendment wave; per-edit measurement — `Confident` *(amended at review, C1/I1/M1)*

**Statement:** order of operations: **(1)** the plan-only eval's pre-edit baseline grid
runs against git HEAD (the shipped v0.96.0 pairs — the working tree's uncommitted
prototype is excluded from the pre-arm by construction); **(2)** one combined grammar
amendment wave lands D1–D6 **and** the `extends:` question — which this session
therefore rules as its own D8 — as a single conversion + audit cycle over the six pairs;
**(3)** the post-edit arm runs within the same grid (eval D6 three-part diff, the
changed-text fourth bucket absorbing the mass reword). The rule-similarity dive's
"not yet ruled" note on `common.yaml` is discharged by D8 here, not by a separate
session.

**Rationale:** two separate grammar waves would mean two conversions, two audit cycles,
and an eval diff spanning two edits; the within-grid pre/post diff is the instrument's
main design point and this amendment is its first real customer. User ruled the
recommended option at the Q7 fork.

*(Amended at review, C1/I1/M1 — user-ruled: repair, no re-run:)* step (1) was already
discharged when this record was first written — the optionA grid's pre arm
(`old_ref: HEAD`, the **v0.97.0** pairs; the "v0.96.0" parentheticals were wrong) is
the reusable baseline. Attribution is decomposed per edit: the existing post arm
measures the `extends:` prototype at delivery level only (the four-bucket differ
resolves `extends:` before diffing and reads it as unchanged — a stated instrument
limit); the ontology edit owes its **own** post arm after the wave, diffed against the
same pre arm. The one-build-wave ruling stands for conversion and audit economy; the
measurement is per-edit, so a post-arm regression is attributable.

### D8 — `extends: common.<slug>` adopted; command-content-schema D3 amended, narrowly — ship `Confident` · efficacy `Assumed (n=0)` *(mark split at review, I2; narrowed at C2; precedence at C3)*

**Statement:** the rule-similarity dive's Option A semantics enter the grammar: a rule
block may carry `extends: common.<slug>`, inheriting every field (text, class, labels,
pointer, and the new D1–D6 fields) from the named block in
`plugins/mochiko/schemas/common.yaml`; any locally declared field replaces the inherited
one; `${var}` placeholders in resolved text substitute from the **binding** schema's
`vars:`; the stub's `<cmd>.*` ID stays the citable ID; `common.yaml` becomes a shipped
primitive under full ceremony, its block IDs minted once under D11 lifecycle. The
command `.md`'s load instruction extends to a raw full Read of `common.yaml` in the same
first action (as the working-tree prototype already words it for implement).

**This is a recorded supersession-by-ruling of command-content-schema D3** (`Contested`,
2026-08-26: no shared rule library, duplication accepted and visible) — **amended, not
reversed**: per-command rules remain the default and the label vocabulary remains the
cross-command link; extraction into `common.yaml` is legal **only** for text that is
exact-duplicate boilerplate across three or more command schemas. ~~The build admits the
render-degrade formula ("rendered by `mochiko-cli template X`, or its schema Read raw
when the binary is absent — the shipped schema is the first-class source of truth") as a
common block — the walk's strongest candidate (~8+ sites, five schemas)~~ *(struck at
review, C2 — see the amendment below; kept struck-through because it was an imperative
to the build, verify nit 6)*. The build **re-screens the prototype's current six blocks
against the same 3+ exact-duplicate bar**, keeping only those that pass; the rest revert
to per-command rules.

**Rationale:** the dive's driver (multi-site drift on identical boilerplate) is real and
the walk quantified it; D3's original value — every command readable from its own file —
survives under the narrow bar because a common block is boilerplate by definition, never
command-specific judgment. User ruled the recommended option at the Q8 fork, with the
supersession stakes stated plainly first.

*(Amended at review, C2 — user-ruled: the narrowed form re-affirmed eyes-open:)* the
render-degrade admission is **struck**: the formula sits in 7 rules across five schemas
with zero identical whole texts (F3 as corrected), and `extends:` inherits whole
fields — it cannot lift a shared sub-clause; extraction would force multi-way rule
splits (D11 children, D12 grain) this wave does not take. The re-screen result stands as
review evidence to confirm at build: `register` (5 commands) · `no-git-mutations` (5) ·
`acceptance-plain-text` (3) · `transport-floor` (3) pass the 3+-exact-duplicate bar;
`author-grader-default-fail` (2) and `model-tiering` (2) fail and revert to per-command
rules. The quantified read-path arithmetic is adopted into the cost line (net positive
chars per fire); the surviving benefit is single-point drift-protection on genuinely
identical floor/boilerplate text, and the D3 supersession is re-affirmed on that
narrowed basis — the user's ruling, made knowing the price was misstated at Q8.

*(Amended at review, C3 — precedence clause:)* `extends:` inherits `text`, `labels`,
and `pointer` **only**. `class:` and every absence-meaningful field (`kind`, `when`,
`enforces`, and `at:` if it graduates) are always local — a stub declares `class:`
explicitly and the checker asserts it; the D1/D3/D6 absence defaults apply
post-resolution. Side effect: a floor's class is always readable from its own file,
closing the D15 single-file-readability exposure the review flagged. Build note (verify
nit 7): after single-homing, `common.transport-floor` carries guard-stripped text only —
each binding stub re-declares `class: floor` and the seat-count `when:` locally, so the
shared block carries less than it appears to; the I8 inventory lists these stubs
explicitly.

### D9 — Holes declared in the wave; hole content ruled at the wave's user gate; F8 fixed mechanically — ship `Confident` · efficacy `Assumed (n=0)` *(mark split at review, I2)*

**Statement:** the conversion wave **declares** conditions honestly wherever the corpus
already branches — including the two conditionally active Not-done members
(`impl.fail.gap-finding-missing` gains `when: {scope: [selection, epic]}`,
`impl.fail.skip-unstated` gains `when: {scope: [delta, lane]}`), making the pin's
conditional semantics data. The two **content** holes — what a product-lane run's
landing is, and what amend mode owes the feature map (`setup.feature-map-*` +
`setup.fail.no-feature-map`) — are drafted by the build as proposed rule texts and ruled
at the wave's existing user gate, never builder-decided; a ruling may also be "no rule
owed" with the reason recorded. The F8 provenance leak is fixed mechanically in the
wave: `setup.baselines-bootstrap`'s inline `Assumed` mark and record citation move to
`.mochiko/provenance.yaml`, strip recorded.

**Rationale:** declaring is transcription of what the corpus already means — safe in the
wave; inventing landing semantics or amend-mode obligations is content the user's pen
owns; booking everything separately would baseline the eval on a known-holey corpus
twice. User ruled the recommended option at the Q9 fork.

*(Amended at build, inventory anomalies J-7/J-10 — user-ruled:)* two corrections from
the I8 inventory. **Scope (J-7):** the amend-mode hole covers **six** setup rules, not
two — the feature-map pair and fail node plus `setup.store-scaffold-unconditional`,
`setup.architecture-scope-handoff`, and `setup.map-never-overwrite` (each phrased as
exhaustive over two of the three declared mode values); the gate rules one coherent
amend-mode disposition across all six. **F8 shape (J-10):** the fix is citation-only,
not the D9 statement's sidecar move — the record CITATION "(feature-sizing record, open
thread 4 …)" leaves both setup sites (`setup.baselines-bootstrap` and the `setup.md`
mirror), its provenance carried verbatim by the strip entry; the `Assumed` word STAYS as
prose in both — it is a runtime-relevant hedge, and `arch.tools-brownfield-reconstruction`
references it by name (a full move would dangle that reference, and the sidecar's grammar
has no confidence-mark field and no list-valued keys). Sidecar untouched.

### D10 — Governance envelope: no new kernel admission; GI-019/GI-020 argued — `Confident` *(review fold, I5; ruled in the I/M batch)*

**Statement:** the amended grammar needs no new kernel-class admission: every new
surface (`kind:`, `conditions:`/`when:`, `moments:`, `enforces:`, `extends:`,
`common.yaml`) is data the model interprets at command fire — nothing executable gates
progress, dispatches, or sequences agents, so GI-019 is untouched. The two clauses that
keep the schema on the declaration side of the sequencer line are deliberate: the
moment list is unordered (D4) and floors are never shed (D3 as amended). GI-020 holds:
the raw Read stays the first-class path; `common.yaml` is one more data file in the same
directory, read whole in the same first action; the D13 checker stays an advisory
exit-code signal. A future `mochiko-cli` view over command schemas still takes its own
ruling (command-content-schema D9, unchanged).

**Rationale:** command-content-schema gave its wave an explicit governance-envelope
decision (its D9); the review found this record had none while adding the fields that
most resemble sequencing scaffolding. Argued here rather than assumed safe because it
is data.

### D11 — Framing: grammar-axis work, stage-orthogonal to the D2 absorption split — `Confident` *(coverage-survivor reopen, I9 — ruled inline by the user)*

**Statement:** this session amends the **rule grammar** — what a rule block can say
about itself; command-content-schema D2's stage split governs **content location** —
which prose lives in the `.md` vs the schema. The amendment moves no narrative into the
schema and thins no `.md`, so it neither advances nor retards D2's stage 2; the
benefit-keyed absorption trigger stays unfired and untouched, and D2's retreat branch
survives unchanged (a retreat to all-`.md` would carry the kind/when/enforces
information back as prose under the namespace-level tombstone of command-content-schema
D11). One directional note (verify observation): build item 7 adds a one-line `when:`
interpretation clause to each `.md` — a thickening, not a thinning; named so the claim
stays exact. Noted for the trigger's own ledger: the completed optionA grid is
delivery-probe evidence of the D10 first-live-run-watch class, to be read into that
watch when the wave lands — read through plan-as-proxy's own `Assumed` mark
(command-plan-only-eval D10).

**Rationale:** the sibling scaffold session was reviewer-corrected for the same missing
framing one day earlier (its D7); the reviewer's I9 caught the repeat here. Ruled
inline at the coverage-survivor gate; per the reopen rule this decision gets one
bounded verify round, no fresh cold read.

## Session trail

- Session opened from an in-conversation deep read (user: "everything is a rule? we need
  to improve ontology … conditional execution and dependencies in between"), followed by
  the user-directed six-schema walk; ground facts F1–F8 recorded at open.
- **Q1 — kind set** (seven-marked-kinds-plus-default recommended / six-no-duty / minimal
  three): user ruled as recommended. → D1.
- **Q2 — per-kind structure** (tag-only recommended / tag + gate holder / full fields):
  user ruled as recommended. → D2.
- **Q3 — conditions grammar** (declared block + `when:` recommended / free-form `when:` /
  declare-only): user ruled as recommended. → D3.
- **Q4 — moments grammar** (declared block + `at:` recommended / no field / moments with
  ordering): user ruled as recommended. → D4.
- **Q5 — dependencies** (check in-text citations only, recommended / mint `requires:` now /
  do nothing): user ruled as recommended. → D5.
- **Q6 — fail mirrors** (mint `enforces:` recommended / defer with the D2 fields): user
  ruled as recommended. → D6.
- **Q7 — sequencing** (baseline-then-one-combined-wave recommended / baseline-then-two-waves
  / no baseline gate): user ruled as recommended. → D7; the extends ruling becomes this
  session's D8.
- **Q8 — extends** (adopt with narrow D3 amendment, recommended / adopt six-blocks-as-is /
  reject): the D3 supersession stakes stated plainly before the fork (Contested ruling,
  protected content, leaves only by recorded supersession); user ruled as recommended. → D8.
- **Q9 — holes & leak** (declare-in-wave + rule-content-at-gate recommended / book all
  separately / fix all inline): user ruled as recommended. → D9. The original agenda's
  rollout question (per-command vs one wave) was absorbed by D7's one-wave ruling — no
  separate fork needed.
- **Sizing gate:** lead recommended solo cold review (blind-map two-message dispatch;
  pair and waiver offered with their trade-offs); user ruled "as recommended" — solo.
- **Dispatch:** message 1 sent — topic statement + goal line only, fence declared on the
  session directory and its index entry; awaiting the Phase 0 angle map before the record
  path is sent.
- **Phase 0 map returned:** 46 angles. **Disclosed fence leak:** the reviewer's orienting
  `head` of `.mochiko/brainstorms/index.md` rendered this session's top entry before it
  could stop — the entry as written at open (agenda scopes visible, "decisions pending";
  no rulings). Reviewer marked every leak-touched angle `[LEAK]` and self-instructed a
  coverage-finding discount on the five leaked scopes; session directory unread, record
  unread. Leak class matches the `command-plan-only-eval` precedent (disclosed,
  negligible-candidate); its acceptance rides the survivor-disposition gate. Map hunt
  priorities: stage-1.5/absorption-trigger collision · `observable.yaml` reconciliation ·
  cheaper-shape lint extension · guard-extraction arithmetic (16 vs 74) · `enforces:` vs
  D16 · eval-baseline perturbation · `extends:`/D3 sequencing on a dirty tree. Record
  path sent in message 2 after this entry; the record is frozen for the read.
- **Disposition rulings (post-review):** C1–C4 individually ruled, all repairs as
  recommended (C2 through the narrow-D8 door); I9 ruled inline → D11; the I/M batch
  "as recommended"; the leak accepted negligible. All folds executed into the record;
  one bounded verify round dispatched per the reopen rule — covering the D11 reopen and
  every fold, no fresh cold read. *(Bullet re-ordered here at verify, nit 9.)*
- **Verify round 1 — NOT CLEAN, lead-repaired same round:** 18/18 folds confirmed
  landed, D11 graded sound and fit, the lead's count correction confirmed against the
  reviewer's own figures (C2's count limb withdrawn; 7 rules · 5 schemas · 8
  occurrences, the 7th a paraphrase — strengthening C2's substance limb). 2 blocking
  (B1 the stale "baseline grid not yet run" clause in Prior-session relations · B2
  build item 11's DECISIONS.md rows missing D10/D11) + 9 nits — all 11 repaired same
  round, plus the reviewer's two D11 observations (the `Assumed` plan-as-proxy hedge
  where the grid evidence is offered; item 7's one-line `.md` thickening named).
  Reviewer pre-cleared: B1+B2 repaired reads CLEAN, no second verify round needed.
- **Verify close — CLEAN (reviewer delta check):** the reviewer re-checked its twelve
  named sites against the repaired record — both blockers cleared, all nine nits
  confirmed repaired, one cosmetic residual (F1's "unruled" tense) noted at lead's
  discretion and aligned here. Verdict CLEAN; nothing further owed from the seat.
- **Build open (2026-08-27, post-acceptance):** the conversion wave opened on the user's
  "convert". Seat P0 authored the I8 inventory
  (`conversion-inventory.md` — kind census constraint 137 · binding 42 · routing 39 ·
  fail 36 · duty 26 · reservation 17 · gate 10 · latitude 8 · bound 5; movable population
  37 = 31 MOVE + 6 DECLARE; extends stubs 16, C2's counts confirmed with non-obvious
  member sets — implement binds `register` alone; 14 anomalies). Anomaly routing: four
  user-ruled at a pre-conversion batch (J-1 DECLARE admitted → D3 amendment · J-10
  citation-only F8 → D9 amendment · J-7 amend-mode gate widened to six → D9 amendment ·
  J-12 moments stay all-six per D4, asymmetry noted on the D4 graduation review); ten
  lead-routed as applications of ruled policy (J-2 `baseline` presence-dimension in,
  `review_pairing` prose · J-3 `gaps` prose, graduation-noted · J-4 latitude membership
  stands, validators re-check · J-5 two-limb bound discriminator, five bounds · J-6
  recorded, no action · J-8 `enforces: [spec.stress-test-one-pass]`, no third
  empty-reason class · J-9 no text normalization, backtick near-miss noted · J-11 scan
  pinned all-prefix, section-IDs legal, bare form covered · J-13 header normalization
  rides the canonical header · J-14 repaired by C3).
- **D9 hole gate (2026-08-27, build wave — user-ruled):** both drafts adopted as
  recommended. **Lane landing:** `impl.landing-lane` minted as drafted — `class: must`,
  `when: {scope: [lane]}`, the landing adds no map work (a product lane owns no
  capability entry; the lane directory persists as record); gives
  `impl.landing-verifier-folds`' lane check its referent; implement 104 → 105 rules, no
  fail node, pin stays 15, pure addition (no strip); `impl.landing-verifier-folds`
  deliberately untouched. **Amend mode, adopted whole (eight sites):** the coherent
  posture — an amend run owns the governance surface set and nothing else; product
  surfaces belong to the establishment paths and their owning desks. Feature-map pair
  unchanged (mode-narrowed) · `setup.fail.no-feature-map` gains the third arm ("on an
  amend, a missing map neither surfaced nor offered" — amend owes surfacing, never
  scaffolding, the `spec.missing-map-surfaced` posture) · `setup.store-scaffold-
  unconditional` "on every path", create-if-absent explicit · `setup.architecture-scope-
  handoff` write-if-absent, never re-declaring an existing `Scope:` line ·
  `setup.map-never-overwrite` (floor) restated instances-not-extent ·
  `setup.store-ruled-content-never-here` same treatment · setup.md's Goal-step "both
  paths" line aligned. `mode: amend` reaching no feature-map rule becomes a recorded
  expectation, not a hole.
- **Build trail (2026-08-27, the conversion wave):** producers P0 (inventory) + P1/P23
  (six-pair conversion; the first P1/P2/P3 seats died mid-work on a session-limit reset
  and were respawned as continuations against the partial tree) + P4 (+P4b) infra/ceremony
  + P5 (+P5b) checker. Deterministic pre-pass: all six pairs checker-PASS 0 findings;
  negative matrix 79/79 at audit. Four fresh author≠grader validators: V1 implement PASS ·
  brainstorm PASS (4 Minor, observations/referent notes) · V2 feature PASS · architecture
  PASS (2 Minor, both against the inventory referent) · V3 specify PASS · setup PASS
  (3 Minor — M1 diction in a setup floor, fixed same round with an honest same-version
  strip note; M2 referent; M3 the recorded amend expectation) · V4 common.yaml PASS ·
  command-labels PASS · primitive-edits PASS · SKILL.md FAIL (1 Major: the
  `D6 empty-with-reason:` literal unnamed) · checker+tests FAIL (1 Major: 18 asserts
  without failing fixtures, census supplied) → fix round (P4b + P5b + P23) → V4
  delta-confirm **both CONFIRMED-PASS, zero residual**; the fix round also fixtured the
  pre-existing asserts (matrix 79 → 133, universal-coverage docstring now literally
  true). Inventory-referent corrections owed at landing: the section-D
  "every moment referenced" preamble claim (V1 M1 / V2 M1 / V3 M2) and C.7's
  non-exhaustive expected-warning table (V2 M2).
- **Partition watch measured (2026-08-27, build item 10):** the kind-derived partition
  vs the hand-authored `observable.yaml` (105 rules): kind-only
  ({fail, routing, bound, latitude} = out) agrees **72%** (76/105); kind **plus
  `when:`-gated = out** agrees **82%** (86/105, observable 59 vs the hand's 58 — near-
  identical size). Divergences decompose into three named classes, none noise:
  (a) nine hand-`contingency` rules that are **event-triggered** (deviation-gate,
  adopt-first, constraint-challenge, escalations…) — exactly the `on:` trigger class
  this session deliberately did not mint; (b) routing/bound rules the hand judges
  plan-observable because a forward plan can *state* the policy (escalation cadence,
  attempt bounds at run-open) — kind alone cannot carry "statable in a plan";
  (c) `when:`-gated rules whose conditions the fixtures force TRUE (transport-floor,
  landing-selection) — closable only when fixture condition-assignments become data,
  D3's own promise. Verdict for the watch: the derivation is a strong first
  approximation, not a replacement; the hand partition stays authoritative for the
  eval, and the residual classes are the graduation evidence for `on:` (if ever) and
  for condition-keyed fixtures. `impl.landing-lane` classified `conditional`
  out-of-instrument (no scenario plants lane scope; rubric re-validated
  58 observable / 47 out / 105).
- **Eval arms measured (2026-08-27/28, build item 10 as amended + the user-directed
  second grid):** judges upgraded to opus by the user's ruling mid-run. **Implement**
  (pre arm reused from optionA per D7; post + a noise-guard r4 pair; 24 plans, grid
  $18.31): no stable regression attributable to the edit — every flagged
  unchanged-bucket regression is replicate-flaky in its own arm; coverage flat
  (pre 151 / post 149 of 174 pass^4 cells); the preregistered noise guard fired at r3
  AND r4 (s2 flaky share ~21% breaches the 20% bound) — the instrument is
  noise-dominated at this k, evidence booked to the eval session's noise falsifier
  (instrument finding, not an edit finding); pairwise 4 post / 3 pre / 5
  position-biased — no signal; `impl.landing-lane` out-of-instrument by design
  (`conditional`, no lane scenario). **Setup** (user-picked second command; fresh
  substrate — 3 mode-keyed fixtures, rubric 32 observable / 8 out; 18 runs, $10.56):
  directionally positive where this wave changed content — brownfield pre 26 → post 30,
  amend pre 22 → post 24, **zero regressions in both**; s1-greenfield's two flagged
  regressions both flaky-class; the ten observable changed-text rules land against
  their new texts. Cross-cutting: the `Read,Grep,Glob` fence held in all 42 runs
  (setup attempted one denied Bash in every run, both arms — diff-neutral;
  brownfield-analysis instinct); eval-infra find: the name-resolution assert
  false-positives on the three governance comment-marker names in every setup run
  (marker allowlist candidate, booked to the eval watch). Judge passes opus (billed
  outside the grids' totals).
- **Wave landed (2026-08-28, v0.98.0):** gates 4/5/6 executed — CHANGELOG 0.98.0 entry ·
  plugin.json + marketplace.json 0.98.0 · cargo test 12/12; landing whole — build item →
  trail · ROADMAP/DECISIONS rows carry built status · index updated · the two audit-owed
  inventory-referent corrections applied (section-D preamble struck with the unused-moment
  facts; C.7 marked non-exhaustive with the two discovered correct-absence warnings).
  Final build acceptance given by the user 2026-08-28 — the wave closes accepted, with
  the eval results explained and on record.
- **Acceptance (2026-08-27):** the user accepted the record after the CLEAN verify close.
  Landing executed whole: `DECISIONS.md` row (D1–D11, the D3 supersession named) · index
  entry to accepted · ROADMAP Next-row merge · BACKLOG "Command-schema ontology wave"
  section (conversion-wave build item + graduation candidates).

**Sizing:** solo cold review, user-ruled "as recommended" at the named gate.
**Dispatch:** blind two-message per the charter; Phase 0 map (46 angles) returned before
the record path was sent. **Disclosed fence leak:** the reviewer's orienting `head` of
the brainstorms index rendered this session's open-state entry (agenda visible, rulings
not); reviewer self-applied a coverage-finding discount on the five leaked scopes;
impact statement: negligible — its coverage findings are all non-leak-derived.
**Verdict:** `critical-gaps` — 4 Critical · 10 Important · 4 Minor survived the
reviewer's own cross-examination; 9 further angles raised and killed by the reviewer
(kill list in the review output: cheaper-shape ruled at D5 · sequencing ruled at D7 ·
observable.yaml reconciled at F7/item 10 · falsifier present · rejected roads present ·
GI-020 disclosure present · provenance anchors survive rewords · cargo-test claim
accurate · F8 verified exact).
**Lead verification of load-bearing review claims (pre-disposition):** C1 grid exists
(`evals/commands/implement/runs/optionA-grid/summary.json`: `old_ref: HEAD`, arms
pre+post, replicates 3, $13.0973) · M1 HEAD is v0.97.0 · I7 BACKLOG "Desk FAIL-set
widening … separate ruled pass" confirmed · I4 checker's bidirectional
fail-label/segment check confirmed (`check-command-schema.py:311–316`) · C2 substance
confirmed (zero identical whole-text render-degrade blocks; whole-field inheritance
cannot lift sub-clauses) with one count correction to the review itself: the formula
sits in **7 rules across 5 schemas, 8 occurrences** (reviewer's 6/4 missed
`impl.cards-template`); F3's "~8+ sites, five schemas" counted occurrences — both
figures repaired at disposition.
**Dispositions (2026-08-27, user-ruled):** C1–C4 individually — C1 repair as
recommended (F7 restated, D7/item 0 reworded, no re-run) · C2 the narrow-D8 door
(4 blocks; render-degrade struck; arithmetic adopted; the D3 supersession re-affirmed
eyes-open) · C3 the precedence clause · C4 floors never shed. I9 ruled **inline** at
the coverage-survivor gate → D11. I1–I8, I10, M1–M5 one batch "as recommended" — every
fold traceable to its finding in the amended decisions, ground facts, build surface,
and cost line. **Fence leak accepted as disclosed-negligible** (the plan-only-eval
precedent), the user's ruling at the same gate.

## Cost line

Priced and accepted eyes-open. Standing read-path growth: `conditions:` + `moments:`
blocks per schema, one to three field lines on many of the 320 rules, and — for commands
binding common blocks — a second data file (`common.yaml`) in the first-action Read
(D8's own price). One-time wave cost: six-pair conversion (~2k schema lines touched),
six author≠grader audits, checker growth (kind/conditions/moments/citation/enforces/
extends checks + advisory coverage reports), conversion-skill update, and two eval grid
arms (the pilot grid's headless runs ×2, priced in the eval session's own cost line).
The compensating assets: deterministic coverage over run-shape branches (the F6 hole
class), a derivable eval partition, kind-keyed audit criteria, checked references, and
deterministic rule-activation filtering at run time (non-floor rules only, C4).

*(Quantified at review, C2/M5:)* the `common.yaml` read is ~2.5k chars on every fire of
a binding command, against 52–434 chars of removed duplication per command — **net
positive chars on the read path**; the surviving purchase is single-point
drift-protection, not size. The wave's eval price is measured, not estimated: the
optionA grid's 18 runs cost $13.0973 unjudged (sunk — pre arm + extends post arm); the
remaining spend is the ontology post arm, 9 runs ≈ $6.55 at the measured ~$0.73/run,
plus judge passes *(arithmetic corrected at verify, nit 5 — the earlier ~$26 priced a
pre arm D7-as-amended reuses)*. Named loss (verify nit 8): this is the first wave to
abandon the predecessors' provable byte-identical text invariance (v0.95.0 "IDs/texts
unchanged" · v0.97.0 "byte-identical, proven three ways") — the I8 inventory is the
compensator.

## Build surface (cold-buildable, one wave per D7)

0. **Eval baseline (D7 as amended): discharged.** The optionA grid's HEAD pre arm
   (v0.97.0 pairs) is the baseline — no re-run. The wave's remaining measurement
   obligation is the ontology post arm (item 10).
1. **Grammar surface:** amend the six schemas' header grammar comments to the D1–D8
   grammar (kind set · `conditions:` · `moments:` · `when:` · `enforces:` · `extends:`;
   `at:` deferred per D4 as amended — not documented in the headers); update
   `.claude/skills/converting-command-to-schema/SKILL.md` to author the new form.
2. **`common.yaml`:** confirm the C2 re-screen at build (expected: `register` ·
   `no-git-mutations` · `acceptance-plain-text` · `transport-floor` pass;
   `author-grader-default-fail` · `model-tiering` revert to per-command rules); **no
   render-degrade block** (struck, C2); every stub declares `class:` locally per the C3
   precedence clause; ship as a primitive; the "not yet ruled" header note is discharged
   citing this record.
3. **Six-pair conversion:** `kind:` on every rule (`constraint` default omitted;
   `latitude` per D1 as amended) · per-schema `conditions:`/`moments:` blocks · `when:`
   with condition text single-homed out of `text` (`at:` deferred, I5) — **preceded by
   the I8 exhaustive clause inventory** (the Appendix-A idiom): every rule losing a
   clause to `when:` enumerated with its reword, the movable population sized (the
   review brackets it 16–74), each move a reword-ID-kept backed by a strip entry, and
   the gate/reservation + latitude membership classified; the inventory is the audit
   referent · `enforces:` on all fail nodes (empty-with-reason where the mirror is
   skill-owned) · `extends:` stubs where the surviving common blocks bind · the D9
   declarations (the two conditional Not-done members gain `when:`) · the F8 mark moved
   to `.mochiko/provenance.yaml`.
4. **Fail addressing re-key:** `kind: fail` replaces the `fail-condition` label as the
   operative selector — registry edit retiring the label, the six `.md` Not-done lines
   re-worded to "the N rules of `kind: fail`", counts re-pinned; the checker asserts the
   count match on the new key **and keeps the bidirectional `.fail.`-segment ↔
   `kind: fail` cross-check** — a like-for-like replacement of the shipped label check
   (I4), with `kind: fail` never defaulted on a `.fail.*` ID. (The count-pin guard
   survives, re-keyed.)
5. **Hole content drafts (D9):** proposed rule texts for the lane landing and the
   amend-mode feature map, presented at the user gate; "no rule owed" is a legal ruling,
   recorded.
6. **Checker (D13) extensions:** kind vocabulary (incl. `latitude`) · `when:`
   resolution against the declared `conditions:` block, and D3 resolution-point naming
   against `moments:` · unused declared dimensions/moments flagged · per-dimension
   coverage report (advisory; no coverage claim over floors, C4) · in-text ID-citation
   resolution over the 29-token population with file-suffix exclusion (D5/M3;
   tombstoned target = error) · `enforces:` resolution + reverse-coverage report
   (advisory; marked as input to the deferred Desk FAIL-set widening pass, I7) ·
   `extends:` target resolution, orphan common blocks, and the C3 local-`class:` assert ·
   the re-keyed Not-done count check and the I4 segment cross-check. Negative tests per
   check, per house checker practice.
7. **`.md` updates:** the six Rules sections name the new grammar in one breath (load
   `common.yaml` in the same first action where the schema binds it; `when:`
   interpretation line); Adaptive Goal Protocol untouched beyond the Not-done re-key.
8. **Strips + supersessions:** the D3 supersession entry citing D8 here; per-schema
   strips with verbatim text for every clause leaving `text` (single-homing moves,
   boilerplate → common); the F8 move.
9. **Audit re-key:** `.claude/rules/mochiko/primitive-edits.md` criteria gain kind/
   conditions/moments/enforces/extends conformance and the re-keyed FAIL-survival
   (criterion 3 re-keys from the label set to `kind: fail`); author≠grader audits per
   pair, checker pre-pass cited (D10-wave idiom: producers + fresh validators).
10. **Post-edit eval arm (D7 as amended):** the ontology edit's own post arm against
    the existing pre arm, D6 three-part diff reported (the optionA post arm attributes
    the `extends:` half, delivery-level only — differ blindness stated, I1); **watch:**
    the hand-authored D8 partition vs a kind-derived partition (fail/routing/exhaustion
    = contingency) — agreement measured, divergences named.
11. **Gates + landing:** `CHANGELOG.md` · `marketplace.json` sync · `cargo test` (binary
    untouched — command schemas are outside the template set) · DECISIONS.md rows
    (D1–D11, the D3 supersession named) · BACKLOG watches (graduation candidates from
    D2/D5, the eval watch) · ROADMAP touch · index updated at acceptance.

## Open questions

- None blocking. Named graduation candidates (not open questions): per-kind structured
  fields (D2), `at:` (D4 as amended, I5), `requires:` (D5) — each keyed on a
  demonstrated consumer. Standing watch: kind-derived vs hand-authored eval partition
  agreement (build item 10).
