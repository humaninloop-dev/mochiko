# Strip notes — `templates/feature-entry-template.md`

Entry formats: `strips/README.md`. Wave context: the feature-sizing & entry-points build wave
(record: `.mochiko/brainstorms/feature-sizing-and-entry-points/record.md`; `DECISIONS.md` row
2026-08-10). The template gains the Parent/Children sections, the `unrefined` stub form, and
the feature-command proposed origin; the entries below record the superseded comment text.

---

<!-- Lineage note: from v0.76.0 this primitive IS `plugins/mochiko/schemas/feature-entry.yaml` —
the template retired into it (entry below) and this file continues as the schema's strip home,
one file per primitive, one continuous history. Wave context for [v0.81.0]: the
product-architecture-schema Stage-1 build wave. Ruling:
`.mochiko/brainstorms/product-architecture-schema/record.md` (D3 · D4) → `DECISIONS.md`
2026-08-19 product-architecture row. -->

## [v0.81.0] Architecture section re-pointed from `ARCHITECTURE.md` components to store elements (D3/D4)

- **Disposition:** superseded → the realizing **store elements, named by id** — `SPN-XXX` spine
  elements of any kind (container | boundary | flow) and/or `AX-XXX` concern rows at
  `.mochiko/product/architecture/`. Ids, not kind nouns: pointing at "components" would have
  silently narrowed the spine to containers and dropped boundaries and flows.
- **Tier failed:** n/a — supersession by ruling (record D3 — one store, the per-feature
  architecture artifact dies — and D4, which makes repo-root `ARCHITECTURE.md` a derived index
  rather than a component register; `DECISIONS.md` 2026-08-19 row).
- **Content:** verbatim, three sites. (1) Contract: "Which ARCHITECTURE.md components realize
  this capability — pointers only, navigable both directions; never restate the component view.
  (Domain -> components mapping is dormant: it lands only at the soft cap, per
  mochiko:patterns-map-minimalism.)" (2) The `check` string: "Does Architecture point at the
  realizing ARCHITECTURE.md components (pointers only, no restated component view)?" (3) The
  skeleton line: "- {{component_name}} (`ARCHITECTURE.md` § {{section}})".
- **Kept deliberately:** three load-bearing properties survive intact — the **pointers-only**
  rule, the **navigable-both-directions** property (the store's `Work:` field is the return leg,
  per the D13 routing this wave lands), and the **dormant domain mapping** with its soft-cap
  trigger and its `mochiko:patterns-map-minimalism` pointer. The section stays `required: false`.
  Pointing at the store rather than its derived index is what makes the back-link durable: under
  D4 the index is regenerated on every store write, so a pointer into it would have been a
  pointer at a rendered projection.
- **Consumers assessed:** `mochiko:authoring-feature-map` owns the entry's authoring and is P4's
  this wave; `features-index.yaml`'s peer-of lines re-worded in the same wave (own strip entry);
  `mochiko-cli template feature-entry` renders from this data file and needs no code change.

## [v0.76.0] Template retired — superseded by schema-based template guidance (D1/D3/D8)
- **Disposition:** superseded → plugins/mochiko/schemas/feature-entry.yaml + mochiko-cli template feature-entry
- **Tier failed:** n/a — supersession by ruling (schema-based-template-guidance D1/D3/D8; record `.mochiko/brainstorms/schema-based-template-guidance/record.md`; `DECISIONS.md` "Template-schema CLI ruled")
- **Content (superseded template, full verbatim below):**

````markdown
<!-- Form: templates/artifact-format.md (the deliverable envelope). One file per capability:
     .mochiko/features/FEAT-XXX-<slug>.md. Authored and amended per
     mochiko:authoring-feature-map — the skill carries the derivation method, the work-row
     and fold rules, and the write timing; this template carries only the shape. An entry is
     a durable capability plus a block of transient work rows (its delivery increments);
     there is NO parent/leaf nesting — capabilities are flat, grouped by domains only at the
     soft cap (mochiko:patterns-map-minimalism). Status is owned HERE (and mirrored one-line
     in FEATURES.md): story files derive theirs by following the FEAT-ID. Omit empty sections
     per artifact-format.md rule 6. Register: `full` per artifact-format.md rule 11. -->

# FEAT-{{XXX}} — {{capability_name}}

> Status: {{status}}  <!-- proposed | in-flight | delivered | retired -->
> {{status_line_detail}}
<!-- Status-dependent detail, one line:
     in-flight  → since {{date}} · live work rows in run(s): `.mochiko/specs/{{spec-slug}}/`
                  (or the owning lane run, for feature-command growth-door work)
     delivered  → since {{date}} · sticky — live rows may still be visible below
     retired    → {{date}} · ruling: {{pointer}}  (or, on a merge: merged-into FEAT-{{YYY}})
     proposed   → surfaced by {{spec-slug}} ({{date}}) — or, for a capability stub,
                  minted by /mochiko:feature ({{date}}) · marked `unrefined`
     Reconstructed entries add: `reconstructed-from-code ({{date}}) — first touching
     spec re-verifies extent` — cleared at that spec's acceptance.
     `unrefined` stubs carry name + one-breath hook only — no Extent, no work rows;
     only specify's derivation fills them and makes the entry selectable. -->

## Capability

{{capability_statement}}
<!-- 1–3 lines, the system's own language — what the product does, standing on its own
     without reference to any story or spec. More than ~3 lines of extent below usually
     means this is two capabilities, or its bulk is undelivered work rows. -->

## Extent

<!-- What is in, and what is notably NOT in — one line each. This is where stories sharpen
     the capability over time, and where a delivered work row folds in at its landing. -->

- {{in_scope_line}}
- Not: {{out_of_scope_line}}

## Work rows

<!-- Transient delivery increments under this capability — one line each, newest first.
     Each row carries a state and its run pointer; a `live` row folds into Extent above at
     its acceptance landing and leaves this list, a `pending` row persists as open
     obligation (deferred work never silently disappears). Grammar:
       - `pending` — {{increment}} · acceptance: {{criteria}} · cut by {{spec-slug or lane-run}}
       - `live`    — {{increment}} · acceptance: {{criteria}} · in {{spec-slug or lane-run}}
     Story-shaped increments are fine here; a story-shaped *capability* is not. -->

- `live` — {{increment}} · acceptance: {{criteria}} · in {{spec_or_lane_run}}

## Relations

<!-- One line per relation to another FEAT-ID: depends-on / extends / composes-with.
     A work row may depend on another capability's row — note it here; row-level
     dependencies order the two capability-batch runs. -->

- depends-on: {{FEAT-ID}} — {{one_line_why}}

## Architecture

<!-- Which ARCHITECTURE.md components realize this capability — pointers only, navigable
     both directions; never restate the component view. (Domain → components mapping is
     dormant: it lands only at the soft cap, per mochiko:patterns-map-minimalism.) -->

- {{component_name}} (`ARCHITECTURE.md` § {{section}})

## Story trace

<!-- Accumulating provenance: which specs/stories informed this capability — IDs and
     pointers only, newest first. -->

- {{spec-slug}}: US-{{n}}, US-{{n}}

## Obligations

<!-- Deferred completeness pointers riding this entry until the work builds — one line each,
     distinct from work rows (which are cut delivery increments): deferred SC-XXX (travels
     here when its covering capability went unselected) · deferred seams ("when built,
     verify seam against FEAT-XXX") · cross-cutting extend obligations from stories homed
     elsewhere. -->

- {{obligation_line}}
````
- **Kept deliberately:** Every line of guidance preserved — lifted into `plugins/mochiko/schemas/feature-entry.yaml` (skeleton / contract / overview / register / density) and rendered by `mochiko-cli template feature-entry`; the `.yaml` ships in the plugin as the raw-Read first-class degraded path (D8, GI-020, no install regression). Net-new per-section `check` lines were authored under D7 (disclosed, not lifted). Nothing dropped.
- **Consumers assessed:** `commands/specify.md` (re-pointed by P4) · `commands/setup.md` (re-pointed by P4) · `skills/authoring-feature-map/SKILL.md` (re-pointed by P5). V2 fidelity PASS 2026-08-16 (schema graded 8/8 at the M3 gate).

## [v0.68.0] Re-type: capability entry + transient work rows; parent/leaf nesting removed (wave context)

Wave context: the PM-role & feature-derivation build wave (record:
`.mochiko/brainstorms/pm-role-and-feature-derivation/record.md`; `DECISIONS.md` row 2026-08-13).
The entry re-types to a durable **capability** plus a **Work rows** block (pending/live delivery
increments that fold into Extent at landing and vanish); the **Parent** and **Children** sections
and all parent/leaf nesting die, and the **Deltas** section re-types into Work rows (per the
record's D6 exhaustive per-clause inventory). Superseded clauses that were feature-sizing survivors
(`DECISIONS.md` 2026-08-10; the v0.61.0 entries below) are named as such — a ruling, not a silent
drop. Pure `feature`→`capability` vocabulary swaps that preserve a comment verbatim-in-meaning ride
the decision row.

## [v0.68.0] Parent and Children sections removed — nesting dies
- **Disposition:** superseded → deleted; capabilities are flat (grouping is domains at the soft cap, `mochiko:patterns-map-minimalism`), and parent status roll-up is replaced by the work-row fold
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-13 pm-role-and-feature-derivation; record D6 — two-level nesting superseded, roll-up → fold)
- **Content (verbatim — both sections removed, newlines shown as ·):**
  - Parent section: `## Parent` · `<!-- Leaf under a parent only: one line pointing up. A flat entry omits this section. -->` · `- [FEAT-{{PPP}}]({{FEAT-PPP-slug}}.md)`
  - Children section: `## Children` · `<!-- Parent entries only: one line per leaf child. The parent is navigation + status roll-up, never built directly — plan/implement key to leaves. Roll-up: in-flight when any child is; delivered when all children were delivered at the time delivered was earned. Delivered is sticky: a later in-flight child rides as a Deltas line below, never regresses this status. -->` · `- [FEAT-{{CCC}}]({{FEAT-CCC-slug}}.md) — {{child_status}}`
- **Protected-content reconciliation:** both sections were feature-sizing D2/D3 additions (the v0.61.0 header-comment entry, "Kept deliberately") — superseded now by pm-role D6. The sticky-delivered rule the Children comment carried survives on the capability entry (Status detail + the skill's invariant 5); the late-child-as-Deltas-line mechanism re-types onto pending/live work rows.
- **Kept deliberately:** nothing from these two sections remains in the template; their behavior re-homes (stickiness → capability Status; roll-up → the fold).
- **Consumers assessed:** `authoring-feature-map` (removed the nesting invariants same wave — its strip) · `features-index-template.md` (dropped the nested-row shape same wave).

## [v0.68.0] Deltas section superseded → Work rows (pending/live)
- **Disposition:** superseded → the **Work rows** section — pending/live delivery increments with acceptance criteria and a run pointer; a `live` row folds into Extent at its landing and leaves the list, a `pending` row persists as open obligation
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-13 pm-role-and-feature-derivation; record D2 work rows + fold, D6 — delta-carry re-types onto rows)
- **Content (verbatim, newlines shown as ·):** `## Deltas` · `<!-- Marked changes riding a delivered entry — status never regresses. Grammar: 'extent grows by {{X}} — in-flight, {{spec-slug or lane-run}}'; on a delivered parent carrying a late child: 'new child FEAT-{{YYY}} — in-flight, {{spec-slug or lane-run}}'. Each delta names its spec or lane run; it folds at that work's acceptance landing, then leaves this list. -->` · `- extent grows by {{X}} — in-flight, {{spec_or_lane_run}}`
- **Protected-content reconciliation:** the Deltas grammar was the v0.61.0 delta-grammar ruling (feature-sizing D7/D14, "Kept deliberately") — superseded now (delta → work row, D2/D6). The fold-at-acceptance-then-leave lifecycle and the names-its-run obligation survive on the Work rows shape.
- **Kept deliberately:** "status never regresses", the fold-then-leave lifecycle, and names-its-spec-or-lane-run — all survive re-typed onto work rows; the parent-late-child delta form dies with nesting.
- **Consumers assessed:** `authoring-feature-map` delta grammar superseded in lockstep (its strip) · the feature command authors work rows in this grammar (parallel seat).

## [v0.68.0] Header comment nesting clause superseded
- **Disposition:** superseded → the re-typed header comment (skill carries "the work-row and fold rules"; an entry is a durable capability + transient work-row block; NO parent/leaf nesting — flat, domains at the soft cap)
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-13 pm-role-and-feature-derivation; record D2/D6)
- **Content (verbatim — superseded clauses, newlines collapsed):**
  - "the skill carries the derivation method, nesting and roll-up rules, the delta grammar, and the write timing"
  - "Entries nest two levels max: parent (capability, roll-up) and leaf (deliverable, the pipeline unit); a flat entry is a leaf. Use Parent on a leaf under a parent, Children on a parent — never both."
- **Protected-content reconciliation:** the nesting sentence was a feature-sizing D2/D3 addition (v0.61.0 header entry, "Kept deliberately") — superseded now by pm-role D6.
- **Kept deliberately:** the Form/artifact-format framing, the one-file-per-entry home, status-owned-HERE, omit-empty-sections, and the register line survive verbatim; "derivation method … write timing" survives re-worded as "derivation method, the work-row and fold rules, and the write timing".
- **Consumers assessed:** `authoring-feature-map` fills this shape (re-typed same wave).

## [v0.68.0] Status/Capability/Extent parent clauses superseded; merged-into pointer added (D12)
- **Disposition:** superseded → the re-typed comments — Status detail gains "live work rows in run(s)", "sticky — live rows may still be visible", and the **merged-into FEAT-YYY** retire form (D12 merge mechanics); Capability/Extent lose their parent clauses; stub form re-keyed "no Extent, no work rows"
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-13 pm-role-and-feature-derivation; record D12 merge, D2 rows + fold, D6 — no parents)
- **Content (verbatim — superseded clauses, newlines collapsed):**
  - Status detail: "delivered → since {{date}}" · "retired → {{date}} · ruling: {{pointer}}" · "in-flight → since {{date}} · owning spec: `.mochiko/specs/{{spec-slug}}/` (or owning lane run, for feature-command delta work)" · stub line "`unrefined` stubs carry name + one-breath hook only — no Extent, no Relations; only specify's derivation fills them and makes the entry selectable."
  - Capability comment: "More than ~3 lines of extent below usually means this leaf is two features — or a parent waiting to be minted."
  - Extent comment: "This is where stories sharpen the feature over time. Leaves and flat entries only — a parent's extent is its children."
- **Protected-content reconciliation:** the "or a parent waiting to be minted" and "a parent's extent is its children" clauses were feature-sizing D2 survivors (v0.61.0 header entry's "pure additions") — superseded now (no parents). The stub form's `no Relations` becomes `no work rows`, matching the new entry shape.
- **Kept deliberately:** status-owned-HERE, the reconstructed-from-code mark, the `unrefined`-stub name+hook-only rule and its selectability-specify-only clause, the in-flight owning-run pointer — all survive re-worded; the retire `ruling: {{pointer}}` form survives, extended with the merge alternative.
- **Consumers assessed:** `authoring-feature-map` (Status/Extent/stub rules by reference) · `features-index-template.md` (mirrors status one-line, `unrefined` mark).

## [v0.61.0] Deltas comment superseded — a delta names its spec OR lane run; parent child-delta form
- **Disposition:** superseded → the rewritten Deltas comment: grammar `extent grows by {{X}} — in-flight, {{spec-slug or lane-run}}`, the parent late-child form `new child FEAT-{{YYY}} — in-flight, {{spec-slug or lane-run}}`, "Each delta names its spec or lane run; it folds at that work's acceptance landing"; placeholder line re-keyed to `{{spec_or_lane_run}}`
- **Tier failed:** n/a — supersession by ruling (record D7 invariant amendment + D14 — lane runs own deltas; D2 amended — sticky-delivered parent carries a late child as a marked delta)
- **Content:** "Marked changes riding a delivered entry — status never regresses. Grammar: 'extent grows by {{X}} — in-flight, {{spec-slug}}'. Each delta names its spec; it folds into Extent at that work's acceptance landing, then leaves this list." · placeholder "- extent grows by {{X}} — in-flight, {{spec-slug}}"
- **Kept deliberately:** "status never regresses", the Deltas-list home, and the fold-then-leave lifecycle verbatim.
- **Consumers assessed:** `authoring-feature-map` SKILL.md delta grammar superseded in lockstep (its strip note); the feature command authors lane deltas against this shape.

## [v0.61.0] Header comment superseded — nesting shape and stub form added
- **Disposition:** superseded → the extended header comment (skill carries "nesting and roll-up rules"; "Entries nest two levels max: parent … and leaf …; a flat entry is a leaf. Use Parent on a leaf under a parent, Children on a parent — never both.") plus the extended status-detail comment: in-flight gains the owning-lane-run alternative, proposed gains the feature-command stub origin ("minted by /mochiko:feature ({{date}}) · marked `unrefined`") and the stub-form rule ("name + one-breath hook only — no Extent, no Relations; only specify's derivation fills them and makes the entry selectable")
- **Tier failed:** n/a — supersession by ruling (record D2/D3 nesting; D12 stubs + rider; D6 lane; lead ruling G2 layout)
- **Content:** header comment listed only "the derivation method, the delta grammar, and the write timing" as the skill's carry; status comment read "in-flight → since {{date}} · owning spec: `.mochiko/specs/{{spec-slug}}/`" and "proposed → surfaced by {{spec-slug}} ({{date}})" with no stub or lane-run alternative.
- **Kept deliberately:** status-owned-HERE rule, reconstructed-from-code mark text, omit-empty-sections and register lines — verbatim. Pure additions alongside (no strip owed): the Parent and Children sections with their roll-up comment; the Capability comment's "or a parent waiting to be minted" clause; the Extent comment's leaves-only clause.
- **Consumers assessed:** `authoring-feature-map` (fills this shape — updated same wave) · `features-index-template.md` (mirrors status one-line).
