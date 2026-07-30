# Strip notes — `commands/specify.md`

Entry formats: `strips/README.md`. Wave context: the specify cluster wave (the first of
the five one-shot-command waves; BACKLOG item 7 of the pattern-codification build). The
wave also ran the **D2 conversion assessment** and the **S8 home-revision checkpoint**
(shape v1 → v2 — see the REGISTRY `command-shape.md` row). **Stale as a standing claim:**
the shape is now **v5** (2026-07-30) — see the v0.35.0 section below.

---

# v0.35.0 — the goal-shape rebuild wave (CS-D10 step 4)

**Wave context:** command goal-shape rebuild, **step 4 of 4** — the five-command wave after the
audit-PASSed plan pilot (design: `.mochiko/brainstorms/command-succinctness-strip/record.md`,
CS-D3/D4/D5 + D8 + D10; `DECISIONS.md` 2026-07-30 rows). Authored against **shape v5** with the
obligated `loop-discipline` read **retained** — the drop is deferred to a named live-run trigger
(pilot-checkpoint ADR `.mochiko/decisions/2026-07-30-goal-shape-pilot-checkpoint.md`), so a v5
command that omits it is non-conformant, not early. specify declares the **in-loop critique
branch**, so it must not reference `sized-end-stage-review.md` — check 1's negative direction; it
does not (see the phrasing entry below, which removed the one near-miss).

**Baseline provenance — read this before auditing the ledger.** The working tree held a partial
rewrite by a since-stopped seat executing a superseded instruction. **This ledger is derived from
`HEAD` (the authoritative 146-line baseline), not from that draft**, and every one of HEAD's 146
lines was walked clause by clause against the delivered file. The audit found **one genuine
fidelity gap** the inherited draft had introduced and this wave restored: HEAD listed the enriched
description as an explicit producer-**brief** field, which the draft had compressed to "carry the
enriched description forward in-session" with no destination named. The delivered Enrichment
invariant now reads "forward in-session **into the producer's brief**". Mechanical backstop for the
same class: every backticked token in HEAD was diffed against the delivered file, and each of the 11
absences is an accounted relocation, a namespace-prefix convention the pilot already set
(`mochiko:requirements-analyst` → the table's bare `requirements-analyst`), or deleted loop
arithmetic (`round = 1`, `round += 1`).

**Measured: 1,273 → 1,100 words (−13.6%), 9,390 → 7,901 B (−15.9%)** — `wc`-measured after the
re-derivation fix round landed, per the pilot's standing habit. Against the pre-wave measured floor
of 991 w: **+109 w (+11.0%)** — over, not under, which is the safe side of CS-D8 (landing materially
*under* a floor row would signal dropped content). The overage is accounted, measured not estimated:
the P4 not-done states are new v5 content the floor row's arithmetic did not carry (**44 w**), the
Recovery table's `accepted` row absorbed the whole Finalize paragraph rather than dropping it
(**26 w**), the `KEPT:` survivor's evidence pointer is newly bound (**8 w**), the restored
brief-destination clause above (**6 w**), and the residual **25 w** is the four gate lines'
three-part `evidence · rules · decides` scaffolding, which the flow prose carried implicitly in
sentence form.

**Run-level, stated honestly because specify is the first command where it does not flatter the
rebuild:** the file drops 1,489 B, while the v5 shared read floor **adds 3,225 B** per run — read
set 29,611 → 32,836 B (`command-shape.md` 12,502 → 16,735 = +4,233; `agent-dispatch.md` 5,183 →
4,175 = −1,008; `loop-discipline/SKILL.md` 11,926, unchanged; `sized-end-stage-review.md` is *not*
in specify's read set). So a specify-only session is net **+1,736 B** against v4. Two readings, both
true: (a) charged per run, the goal-shape rebuild is
net-negative on an already-twice-stripped light command — specify was cut at v0.13.0 and again at
v0.31.0, so there was little narration left to delete; (b) charged once across the surface, the
shared floor is a surface-wide investment that plan's −19,749 B alone repays roughly six times
over. Not a reason to cut protected content (CS-D8 forbids it), but the wave ceremony should have
the datapoint: **the anatomy pays for itself on heavy commands and is byte-neutral-to-negative on
light ones.** Conformance, not a percentage, is CS-D2′'s success criterion — that is what this file
is delivered against.

**Correction to a pilot figure:** plan's v0.34.0 note records the shared read-floor delta as
**+2,895 B**. That was correct when measured at v0.33.0 (shape 16,405 B); the pilot's own commit
then grew the shape home by 330 B with the deferred-read transition note. The live figure is
**+3,225 B**. Same stale-headline cause the pilot's standing habit names — a figure measured before
a later edit landed — this time across two notes rather than within one.

> **Correction to this entry's own replacement figure, ratification round (auditor-caught).** This
> note first stated **+3,223 B**, from a baseline of 29,613. Wrong by 2 B, and the cause is worth
> recording because it is a *different* failure from the stale headline above: I anchored the v4
> baseline to `e30533e` (the commit where the *shape* went v4) instead of to `70f4efd`, the tree
> immediately before the v5 revision. `agent-dispatch.md` kept evolving between them — `21cb75e`
> trimmed it 5,185 → 5,183 — so the shape's own version stamp is not a safe proxy for the read
> set's state. The auditor's **29,611 → 32,836 = +3,225** is authoritative, and it reconciles the
> whole chain: plan's +2,895 is exactly 32,506 − 29,611, i.e. the pilot used this baseline
> correctly. **Lesson for the ceremony:** anchor a shared-read-floor baseline to the revision
> commit's parent, never to the version stamp of one file in the set.

Block sizes against the grader's ceilings (terms as the grader counts them — **G=4** gate lines,
S=2 seat rows, A=3 artifacts, R=7 resume rows): preamble 90/130 · Goal 89/150 · Seats & checks
147/190 · Constraints 441/540 (82%) · **Bindings 145/156 (93%)** · Recovery 120/158. Tightest is
Bindings, and the `+30` KM/index term is what makes it fit — without that term specify's Bindings
floor would sit at 145/126, a FAIL. Second live datapoint for confirming the term at the ceremony.

## [v0.35.0] The flow body, the seat bullets, and the Contract section retired into the five-block anatomy
- **Disposition:** superseded → the goal-shaped anatomy. `Team-form parameters`→ the preamble's
  probe line + shape Layer 2 (see the next entry) · `Session constraints`→ **Bindings** (workspace
  + deliverable + IDs) and the **Bounds** line (kill-switch) · `The seats`→ the **Seats & checks**
  table + G2 (producer clarifications) + the Enrichment/Loop invariants · `The flow`'s Triage→**G1**
  · Enrichment→ the **Enrichment** invariant · Spec loop→ the **Goal**, G2's routing classes, the
  **Bounds** line and the **Escalation** gate · Acceptance→**G3** · Finalize→ the Recovery table's
  `accepted` row + **Report hygiene** + the KM binding. The `Contract` section's four clauses →
  **Goal** (done-condition + not-done states), the **Seats & checks** table and its validation-model
  line (producer↔validator), **Constraints** (bounds + the four gate lines).
- **Tier failed:** n/a — supersession by ruling (**CS-D3** condition-first documents · **CS-D4** the
  connective procedure is deleted and what survives is *restructured* · **CS-D5** the five-block
  anatomy and the Contract-as-document inversion).
- **Content:** five `## `-level sections of ordered procedure and appendix (`Recovery` is the sixth
  and survives, restructured). Not reproduced verbatim — every *rule* inside them is resolved
  individually in the CS-D8 ledger below, and the deleted remainder is connective narration
  (`initialize round = 1`, "loop to produce", "then apply the bounds", the round-arithmetic sentence,
  and the lead's job description restated per section). Recoverable in full at
  `git show 7898d86:plugins/mochiko/commands/specify.md`.
- **Kept deliberately:** every gate, bound, routing class, trigger, ordering rule and artifact
  binding — the ledger below resolves each one.

## [v0.35.0] The `Team-form parameters` section retired — three lines, three different fates
- **Disposition:** split.
  - "Hard-require `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` per the shape" → **relocated** to
    `command-shape.md` Layer 2 (Hard requirement), which the command Reads. The user-facing
    declaration survives in `description:` — which is also what makes the file grep-detectable as
    team-form for check 1.
  - "Transport mechanics + the addressability check: `templates/agent-dispatch.md` (Seat transport)"
    → **superseded**: at v5 Seat transport was absorbed *into* `command-shape.md` Layer 2 (CS-D6),
    so this pointer named a section that no longer exists in the file it points at. A stale
    cross-reference, retired rather than re-aimed — the shape home is already an obligated read, so
    re-aiming it would restate a read the preamble mandates.
  - "The no-fallback bet is the same `Contested` dogfood-pilot ruling as the other team-form
    commands" → **relocated** to `command-shape.md` Layer 2, which carries it as "**no fallback
    transport**, a dogfood-pilot bet marked `Contested`, to be revisited when mochiko distributes
    beyond the author's machines" (read this run to confirm the home holds the content).
    DECISIONS-traceable (`2026-07-04` brainstorm-v2 row, hard-require teams / no fallback /
    `Contested`) → superseded by a home that states it, not dropped.
- **Tier failed:** 1 (altitude) for the two relocations; n/a for the stale pointer.

## [v0.35.0] The `What you own (not the seats)` footer deleted
- **Disposition:** deleted.
- **Tier failed:** 1 — a declared duplicate. Every clause now sits in a block: input triage → G1 ·
  the enrichment call → the Enrichment invariant · the loop and its counter → the Bounds line ·
  gap routing → G2's three classes · the verdict against the default-FAIL done-condition → the Goal
  and the validation-model line · the human gates → the four gate lines · "never letting producer
  and critic collapse into one seat" → the Seats table, where check 7 grades it mechanically.
- **Kept deliberately:** nothing was unique to it. The closing "Full rules: `mochiko:loop-discipline`"
  survives as the preamble's obligated read.

## [v0.35.0] Three check-8 marker restatements removed from the Contract and Recovery
- **Disposition:** superseded → their homes in `command-shape.md`. At v5 each is a *floor-FAIL
  marker* the grader greps for, so carrying them would fail the file mechanically regardless of
  prose quality:
  - "the critic's status is **input, never the gate**" (Contract done-condition, clause 2) → home:
    Layer 2 Clearing. **The rule survives, the phrasing does not:** the Goal's not-done states carry
    "the critic's status taken as the gate without your read".
  - "disjoint agents, disjoint skills, **structurally separated**" (Contract, Producer↔validator) →
    home: Layer 2 Independence by structure, which states both phrasings. The table now *shows* what
    that sentence asserted; check 7 grades it.
  - "a critic **respawn is cold by design**" (Recovery preamble) → home: Layer 2 Independence by
    structure. The Recovery block keeps the workflow-specific half — a respawned producer re-reads
    `spec.md` + the gap list.
- **Tier failed:** 1 (altitude) — each names one homed rule.

## [v0.35.0] "no sized end-stage review here" → "unsized by design"
- **Disposition:** rewritten in place on the validation-model line.
- **Tier failed:** n/a — a hazard fix, not a strip. The clause's *content* is unchanged and its
  word count barely moves; what changed is that the v4 phrasing wrapped as "no sized / end-stage
  review", which normalizes to a near-miss of check 1's negative direction for specify's branch
  (`sized-end-stage-review` must be absent). A literal grep of the path token never matched it, but
  a fuzzy one does — and pointing an auditor at a hit that is really a declaration of *absence* is
  the same false-positive class the pilot retired its `shape-exception` marker over. P6 requires the
  branch to be **named**, not the other branch to be denied, so "unsized by design" carries the
  declaration with no token to trip on.

## [v0.35.0] CS-D8 survivor re-grade ledger — every protected line resolved

CS-D8 (extended by user ruling U4) protects two sets: `KEPT:`/Tier-2-evidenced lines, **and** every
line traceable to a `DECISIONS.md` row. specify carries **one live `KEPT:` entry** (the v0.13.0
enrichment-boundary survivor, re-graded in full below) plus the *Kept deliberately* field of the
v0.31.0 supersession and the DECISIONS row trace. Grepped before any cut. **All 15 rows survive,
zero dropped: 14 translated into the file's own blocks, 1 resolved by relocation** — the
no-fallback-transport bet, whose home (`command-shape.md` Layer 2) is an obligated read of this
command and was Read this run to confirm it holds the content.

Per the pilot's step-4 instruction, the **compressed-evidence clauses were grepped rather than
trusted** — the pilot lost content inside gate lines that still read as complete. Both of specify's
are intact and verified line by line: G1 keeps the `@`-reference recovery's **named cause** *and*
its **two-option prompt** (re-enter, or proceed and enrich from scratch), and G2 keeps **all three**
routing classes (preference ruled here · knowledge → a native `Explore` pass, never to the user ·
scope escalates).

**The `KEPT:` re-grade** (the v0.13.0 entry below — the lead-inline enrichment boundary).
**Verdict: translated, not superseded. The failure still has a path in the goal-shaped file.**

The supersession this entry could have taken would rest on a structural-prevention claim, and
check 14 verifies those against the anatomy rather than the author's say-so. Tested, it does not
hold:

- The **Seats & checks** table's one hard structural rule is that no row grades its own output
  (check 7). A producer seat that *enriched* the input and then authored from it would still grade
  nothing — check 7 passes on that arrangement. The coupling this survivor guards against is
  author-from-self-shaped-input, which is not the producer↔validator collapse the table detects.
- The anatomy constrains which **class** of content each block carries, not which **actor** owns a
  given responsibility. Adding "conditions sparse input" to the producer row's produces cell, and
  deleting the Constraints invariant, would be a shape-conformant file.

**Partial prevention does exist and is worth recording:** at v4 the seats and the enrichment step
were adjacent procedural prose (`## The seats` / `## The flow`) at the same altitude, so ownership
was ambiguous by layout; at v5 they sit in different blocks with different owners, so the
*placement* is legible at a glance. But legible placement is not prevention — and the specific
pressure this survivor exists to resist is a future altitude pass reading the rationale clause as
strippable prose, which the anatomy does nothing to stop.

**Resolved:** translated into Constraints as the **Enrichment** invariant with the rationale clause
intact ("it conditions the input, and it neither authors nor grades — handing it to the producer
would have that seat author from input it shaped itself"), now additionally carrying the
**evidence pointer P9 mandates** and the v4 file never had. Boundary provenance is unchanged: the
specify port's rehome ruling (`.mochiko/transform/specify/reconcile.md`).

| protected line | source | resolved |
|---|---|---|
| Lead-inline enrichment boundary — it conditions the input, neither authors nor grades | v0.13.0 `KEPT:` (Tier-2) | **Enrichment** invariant, rationale intact + evidence pointer newly bound — full re-grade above |
| Every verdict stays the lead's; **no devolved branch** (specify has no deterministic-CLI verification, so shape D3's branch cannot apply — declared, not left implicit) | v0.31.0 *Kept deliberately* | **Loop invariants**: "No devolved branch — the critique is a judgment grade, never all-deterministic-CLI, so no gate is skipped and every verdict is yours" + the validation-model line |
| The **`@`-reference recovery** — empty `$ARGUMENTS` has a *named cause* (the `@`-reference drop bug) and a two-option prompt | `command-altitude` DECISIONS row (its retrofit-regression warning names this recovery among the hard-won fixes verbosity encodes); the class the pilot dropped and had restored under audit | **G1** decides-clause, both halves present. Grepped, not assumed — the pilot's named failure mode |
| All three gap-**routing classes** — preference ruled at the gate · knowledge → a native `Explore` pass, never to the user · scope escalates | record D5 fold (a) graded exemplar; `loop-discipline` gap routing | **G2**, all three named; the scope class lands on the **Escalation** gate's evidence. The other class the pilot dropped |
| In-loop mesh — producer hands work to the verifying seat directly; the lead is the exception handler | Team-method D1 (`Contested`) + Layer-2 mesh rewrite row | Seat table's **peer edges** column, both rows |
| Cold arrival is a property of the **stage**, not of the traffic | Team-method D2 | Critic row's spawn cell: "cold at first critique, standing after" |
| Devolved clean-cycle verdicts — and specify's **declared absence** of that branch | Team-method D3 | Declared, see the no-devolved-branch row above |
| Hard-require agent teams, **no fallback transport** (`Contested` dogfood-pilot bet) | brainstorm-v2 row (2026-07-04) | `description:` declaration + `command-shape.md` Layer 2 — see the Team-form-parameters entry |
| The `review-*` family boundary — a reviewer produces **lead-adjudicated input**, never the authoritative grade | setup-adversarial-review row (the `validation-*`/`review-*` split) | Validation-model line, once (deduped to a single site at v0.13.0 and still single) |
| Governance region is a **prerequisite, surfaced never auto-resolved**; `paths`-scoped rules do not fire for from-scratch authoring, so the producer gets a one-line obligated read | constitution-native-surfaces + governance-injection-probe rows | **G1** (surface + the two exits) and Bindings' **Governance brief** (the obligated-read line) |
| A knowledge gap routes to a native `Explore` pass — the cheap-explorer avenue, never the user | model-tiered-seats row | **G2** and Bindings' **Fact route** |
| KM landing ritual + invariants under fix-on-sight, naming the **project copy** `.mochiko/memory/knowledge-management.md` | OD-D6 (subtractive landing) + the CS step-1 adjudication making the KM reference mandatory in KM-carrying commands | Bindings' **KM landing**; check 1's KM member greps the project path, and it is the project path |
| New domain terms minted into `GLOSSARY.md` | OD-D10 (glossary joins core, `Contested`) | Bindings' KM landing |
| Round reports cleaned by default; **never offer to delete the deliverable** | current body | **Report hygiene** invariant |
| Uncertainty rides the spec template's **Assumptions / Open Questions**, not confidence marks (the shape's producer-authored branch) | current body (P11) | Bindings' **Uncertainty carrier** |

**Additions, logged rather than folded silently** (pure additions ride the decision row per the
Job-4 rule; these are within-command precision, not doctrine): the Goal's artifact set now names
"both round reports written" (P3 previously named only `spec.md`), and the flow's escalation clause
is promoted to a named **Escalation** gate line in the three-part form, which is what makes
specify's gate set countable at **G=4**.

## [v0.31.0] Lead-relayed gap lists superseded by the in-loop mesh (shape v4 conforming edit)
- **Disposition:** superseded → `templates/command-shape.md` v4 (Layer 2 — "Independence by structure" + "In-loop mesh"). Rewritten in place: the critic is still cold-spawned at first critique, and the producer↔critic peer edge is declared on the roster.
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/team-method-vs-command-shape/record.md` **D1**, scoped by **D2**), not a minimalism strip. Permanent no-contact was the falsified claim; cold *arrival* survives as a property of the stage.
- **Content (superseded, verbatim):**
  - producer seat: "Round > 1 is a message to the same seat carrying the critic's gap list verbatim"
  - critic seat: "spawned **cold at first critique**, never in contact with the producer"
  - Contract, Producer ↔ validator: "(critic cold-spawned, gap lists lead-routed, no producer↔critic contact)"
- **Kept deliberately (not superseded):** every verdict stays the lead's — specify has no deterministic-CLI verification, so **D3's devolved branch cannot apply here**; the Contract now declares that absence rather than leaving it implicit.

## [v0.13.0] Conversion note (D2/S4 — one-shot → team-form, 2026-07-19)

- **Command-specific rationale (user-ratified):** specify's producer spans up to 3
  revision rounds plus clarification feedback — the same longer-horizon context-retention
  bet `/mochiko:setup`'s authoring loop was ruled team-form on (standing analyst seat;
  gap lists lead-routed verbatim). The critic maps to setup's validator seat: cold at
  first critique, same-seat messages after, no producer contact — independence stays
  structural. Transport rides the v3 fix (`agent-dispatch.md` Seat transport +
  addressability probe).
- **Steelman recorded:** zero successful team-form runs observed at conversion time (two
  setup defect runs; the kinako acceptance test pending); brainstorm v2 measured standing
  seats as more expensive than dispatches. Ruled team-form anyway per D2's declared
  default + S4 (no prior dogfood evidence required; checkpoint below).
- **Confirm-or-revert checkpoint:** the first post-conversion dogfood run (the open
  "Dogfood `/mochiko:specify`" BACKLOG item) confirms the conversion or reverts it to
  one-shot Layer-1 form.

## [v0.13.0] Per-run contract fill (`workflow-contract.md` → `contract.md`)
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, Contract section — the authoring-time-fill rule); the per-workflow values survive as the command's Contract section
- **Tier failed:** 1 (altitude — the shape retired per-run fills whose values are constant at authoring time)
- **Content:** "Fill `templates/workflow-contract.md` → `.mochiko/specs/<feature>/contract.md` with the values below, then confirm it against `mochiko:loop-discipline`. The filled artifact is the inspectable proof — not this command body."

## [v0.13.0] Sound-loop paragraph + four-requirement enumeration
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, One lead) + the `mochiko:loop-discipline` reference
- **Tier failed:** 1
- **Content:** "This is a mochiko sound loop: invoke `mochiko:loop-discipline` and honor all four requirements (default-FAIL done-condition, independent validation, bounded iteration, named human gates), and brief each dispatch per `agent-dispatch`. Those rules are not restated here…" — the requirement list restated `loop-discipline`'s own enumeration.

## [v0.13.0] Verdict-ownership triplication
- **Disposition:** deduped to once (the Contract's Producer ↔ validator clause; the `review-*` family boundary also lives in `review-specifications`' description + REGISTRY)
- **Tier failed:** 1
- **Content:** stated three times pre-wave — L8 ("The critic *recommends* a status; **you own the clearing verdict** — its status is input, never the gate"), L19 (Team clause "it produces lead-adjudicated input, never the authoritative grade"), L67 (footer "the advocate recommends a status *from* the spec, you Read the artifacts and decide").

## [v0.13.0] HIL done-condition comparison blockquote
- **Disposition:** deleted (user-ratified)
- **Tier failed:** 2 (no behavior produced — historical provenance; preserved in ROADMAP's Decision Trail + `.mochiko/transform/specify/`)
- **Content:** "> Why this done-condition differs from HIL's: HIL exited on the State-Analyst's *autonomous* verdict with no human acceptance — it could self-declare done on pass 1, violating `loop-discipline` req. 1. The advocate's three-way status survives as input to your verdict, plus the new G3 acceptance gate."

## [v0.13.0] Feature-numbering-script aside
- **Disposition:** deleted
- **Tier failed:** 2 (HIL-history note; provenance in `.mochiko/transform/specify/`)
- **Content:** "(No feature-numbering script — workspace-as-state replaces it.)"

## [v0.13.0] Spec-grammar enumeration in the produce brief
- **Disposition:** relocated → the grammar's single sources: `mochiko:authoring-requirements` + `mochiko:authoring-user-stories` + `templates/spec-template.md` (user-ratified; the brief keeps "the template to fill per those skills — no placeholder tokens", and the goal line names the deliverable's parts once)
- **Tier failed:** 1
- **Content:** "(prioritized P1/P2/P3 user stories with Given-When-Then, FR-XXX requirements, measurable SC-XXX, edge cases; technology-agnostic; no placeholder tokens)"

## [v0.13.0] Footer ground rules + one-shot transport line
- **Disposition:** kernel-free/git relocated → `templates/command-shape.md` (Layer 1, Ground rules — homed at shape v2, this wave); the Task-tool line superseded by the team-form conversion (transport now per shape Layer 2 + `agent-dispatch.md` Seat transport)
- **Tier failed:** 1
- **Content:** "Stay kernel-free; brief agents per `agent-dispatch`; always dispatch via the Task tool (never inline agent behavior); do not modify git or push."

## [v0.13.0] Recovery memory-model parenthetical
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, Recovery — "never a context `phase` field")
- **Tier failed:** 1
- **Content:** "Resume from workspace evidence (there is no context-file `phase`)"

## [v0.13.0] KEPT: "Lead-inline because enrich-or-not is loop-entry triage: it conditions the input, it neither authors nor grades."
- **Tier-2 evidence:** guards the lead-inline vs seat boundary — without it the natural reading is to hand enrichment to the producer seat, coupling input conditioning into authoring (the producer would then author from input it shaped itself). Boundary provenance: the specify port's rehome ruling (`.mochiko/transform/specify/reconcile.md` — enrichment landed on the lead, not the analyst).
