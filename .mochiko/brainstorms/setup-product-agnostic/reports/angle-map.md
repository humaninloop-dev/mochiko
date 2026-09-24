---
record_contact: none
report: review
phase: blind-angle-map
reviewer: cold-reviewer (solo, lenses decision-quality + record-integrity, pass cold)
built_from: topic + goal line + free repo grounding only
angle_count: 23
load_bearing_count: 17
class_count: 7
classes:
  A: cut line — what "product-agnostic" means
  B: inventory of setup's product-shaped duties
  C: change mechanics — how a product change stops being a governance event
  D: supersession, migration, landing
  E: leak prevention and enforceability
  F: diagnosis and rejected roads
  G: downstream consumers and the scope fence
angles:
  - id: A1
    class: A
    load_bearing: true
    angle: >-
      A stated sorting test that classifies any obligation as product-agnostic or product-shaped,
      with mixed cases resolved. Example of a mixed case: "auth enforced at every boundary" is
      agnostic, but kinako GI-004 enumerates five named boundaries including the harness capture
      crossing SPN-029. Without a test the cut is re-argued card by card at every amend.
    grounding: kinako .claude/rules/mochiko/engine-port.md; kinako DECISIONS.md 2026-09-03 and 2026-09-19 rows (SPN-037 proposed as a sixth GI-004 boundary)
  - id: A2
    class: A
    load_bearing: true
    angle: >-
      Principle versus instance. May an agnostic principle point at volatile product data (store
      AX rows, spine SPN ids, rules-file project-structure trees)? Is a pointer to a volatile row
      still coupling? Mochiko's own GI-017 already says governance points at constraint homes and
      never restates them; the record should say whether the cut is new doctrine or enforcement
      of that existing restate-ban.
    grounding: mochiko CLAUDE.md GI-017; kinako DECISIONS.md 2026-09-05 row (engine-port.md project structure re-enumerated against the built tree)
  - id: A3
    class: A
    load_bearing: true
    angle: >-
      Product-agnostic is not product-blind. Setup still needs some stable product facts: project
      type or platform selects shelves and shapes each floor category's expression; the fact
      profile (data classes, jurisdictions, contracts) triggers compliance modules. The record
      should rule which product facts stay as inputs and why they are not "product guidelines".
    grounding: INTERROGATION-AGENDA.md dimensions 2-3; catalog/README.md "Expression"; COMPLIANCE-MODULES.md trigger table
  - id: A4
    class: A
    load_bearing: false
    angle: >-
      The agenda's identity (dimension 1), risk surface (dimension 4) and values (dimension 9)
      dimensions are product-flavored inputs. Which survive, reframed or cut.
    grounding: INTERROGATION-AGENDA.md ten-dimension table
  - id: A5
    class: A
    load_bearing: true
    angle: >-
      Deployment is kept by the user's ask, yet deployment content is also volatile product
      detail. Kinako's release gate names the brew tap formula and the plugin tag channel; RUNBOOK
      was promoted to asserted under OO-D4. Where the deployment standard ends and product
      distribution detail begins must be ruled, or the same drift returns through dimension 8.
    grounding: kinako CLAUDE.md Quality gates release-gate line (GI-033); mochiko DECISIONS.md OO-D1 and OO-D4 rows
  - id: B1
    class: B
    load_bearing: true
    angle: >-
      A complete inventory of setup's product-shaped obligations with a keep or cut ruling on
      each: setup.product-truth-leg, setup.design-truth-write, setup.design-scaffold-unconditional,
      setup.baselines-bootstrap, setup.feature-map-brownfield and setup.feature-map-greenfield,
      setup.user-map-confirmation, setup.store-scaffold-unconditional,
      setup.architecture-scope-handoff, setup.next-step, plus analysis-codebase's design-system
      detection. A partial inventory yields a partial cut.
    grounding: mochiko-cli rules setup, sections setup.sec.roles, setup.sec.reserved, setup.sec.tools
  - id: B2
    class: B
    load_bearing: true
    angle: >-
      Floor-class and fail rules touching product: setup.fail.no-feature-map,
      setup.map-never-overwrite, setup.store-ruled-content-never-here. Removing a floor or a
      kind:fail rule needs a recorded supersession, a changed kind:fail pin count (six today), and
      rewritten Goal done-condition text in setup.md, which names the feature map and product
      baselines at close.
    grounding: mochiko-cli rules setup preamble pins and setup.sec.fail-conditions; plugins/mochiko/commands/setup.md Adaptive Goal Protocol step 2
  - id: B3
    class: B
    load_bearing: false
    angle: >-
      Scaffold versus content. An empty store stub or an empty FEATURES.md index is structure,
      not product guidance. The record should say whether scaffolds stay while elicited product
      truth goes.
    grounding: setup.store-scaffold-unconditional; setup.feature-map-greenfield
  - id: C1
    class: C
    load_bearing: true
    angle: >-
      An explicit list of what remains a governance event after the cut: module attach or detach,
      depth flip, un-waive, and the open question of a type or platform change. Scenario: the
      product pivots from desktop to web, the shelves dealt change and the coding standards
      change with them. Is that a governance event or not?
    grounding: INTERROGATION-AGENDA.md "Depth per mode" (amend); catalog/README.md shelf model
  - id: C2
    class: C
    load_bearing: true
    angle: >-
      Compliance tension. COMPLIANCE-MODULES.md's temporal backstop makes a product change that
      introduces a regulated data class re-open the fact profile as a governance event, and
      legal-mandate obligations are unwaivable. If product change stops being a governance event
      wholesale, a feature that starts storing health or card data attaches no module silently,
      the S4 under-scoping failure. The record must carve this out or show why it cannot occur.
    grounding: COMPLIANCE-MODULES.md "The fact-validation fail-safe" item 4 and "Amend events"
  - id: C3
    class: C
    load_bearing: true
    angle: >-
      Version-stamp coupling. Kinako v1.2.1 was a PATCH owed because product-structure text landed
      at epic acceptance with the stamp unmoved. After the cut, what bumps the governance semver,
      and does any product-driven edit still touch the region or rules files?
    grounding: kinako DECISIONS.md 2026-09-05 row (v1.2.1 stamp owed for R-G and R-H)
  - id: C4
    class: C
    load_bearing: false
    angle: >-
      Path-scoped rules files carry repo-layout globs in their paths frontmatter (kinako:
      src-tauri/**, crates/**, components/**). The globs drift with the code tree even when the
      rule content is agnostic.
    grounding: kinako .claude/rules/mochiko/engine-port.md frontmatter; kinako CLAUDE.md path-scoped rules line
  - id: D1
    class: D
    load_bearing: true
    angle: >-
      The supersession chain. The cut reverses recent rulings: impeccable-design-integration D6
      (setup's product-truth leg, ruled 2026-09-19), D5 and D7 (design baseline writer paths), the
      product-architecture-schema scope handoff, the feature-map-layer setup obligations, and
      PO-D3 "setup elicits facts". Protected content leaves only by recorded ruling. The landing
      needs strips, migration-log entries, the validation-primitive-edit audit, and a plugin
      version class.
    grounding: mochiko DECISIONS.md 2026-09-19 impeccable row and PO-D3 row; CLAUDE.md "Editing a shipped primitive is itself a landing"
  - id: D2
    class: D
    load_bearing: true
    angle: >-
      Already-ratified governance in consumer projects. Kinako carries product-shaped GIs:
      GI-037 "Kinako proposes; the leader rules", GI-036 on prose, the corpus rules GI-011, GI-012
      and GI-031, engine-port GI-008 and GI-016, and the UI rules GI-028 to GI-030. What happens
      at their next amend? The record needs a legacy-migration clause like the retired-tier one.
      They cannot vanish silently, because trace closure and durables-never-deleted still bind.
    grounding: kinako CLAUDE.md governance region; kinako governance-ledger.md principle index; setup.durables-never-deleted; setup.fail.unclosed-trace
  - id: D3
    class: D
    load_bearing: false
    angle: >-
      Self-application. Mochiko's own governance carries product-shaped principles: GI-019 on
      kernel-class tooling, GI-020 on clone-only install, GI-022 declining a feature map. Does the
      cut apply to mochiko's own in-repo governance?
    grounding: mochiko CLAUDE.md Governance Principles
  - id: D4
    class: D
    load_bearing: false
    angle: >-
      Consistency with the production-only rulings PO-D1 to PO-D7 and adaptive-depth D1 to D8.
      "Product-agnostic" must not read as widening the target to libraries or CLIs, and must not
      read as dropping the production floor.
    grounding: CLAUDE.md "What this is"; catalog/README.md shelf model note
  - id: E1
    class: E
    load_bearing: true
    angle: >-
      Re-entry doors. Product content can come back through dimension 9 minting (kinako GI-036
      and GI-037 were minted from values), through arbitrated architecture-opinion cards with the
      layer-rules domain-registry carve-out that names product crates, and through
      domain-dependency seeds. The record needs a mint-admission test, a grader criterion in
      validation-constitution or review-governance-intent, or both. Otherwise the cut decays.
    grounding: INTERROGATION-AGENDA.md steps 2-4 after the dimensions; setup.carve-outs-preserved; kinako governance-intent.md amendment log 1.2.0
  - id: E2
    class: E
    load_bearing: true
    angle: >-
      Enforceability tension. The validator demands testable pass/fail with real commands, and
      the agenda forbids authoring vagueness. Stripping instance specifics can leave principles
      untestable, or testability can pull product specifics back in. The record should show that
      agnostic principles stay enforceable.
    grounding: ESSENTIAL-FLOOR.md Enforcement and Testability blocks; INTERROGATION-AGENDA.md dimension 9 note; validation-constitution QUALITY-CHECKLIST.md
  - id: F1
    class: F
    load_bearing: true
    angle: >-
      Root cause. Is out-of-sync governance caused by product content living in governance, or by
      the cost of the amend ceremony (pair review, MAJOR bumps, interrogation slices) making
      amends lag behind the product? Kinako's B30 amend ran eleven cards with a 30-finding cold
      review. The two causes call for different fixes, and the record should ground its diagnosis
      in observed incidents.
    grounding: kinako governance-intent.md amendment log 1.2.0; kinako DECISIONS.md 2026-09-05 v1.2.1 row
  - id: F2
    class: F
    load_bearing: true
    angle: >-
      Steelmans of the rejected roads, each dismissed with a reason. First, keep the product
      content but point at the architecture store rows instead of restating them. Second, a
      lightweight product-delta amend class. Third, the status quo plus a drift probe at landing
      folds, which the architecture store already runs for itself.
    grounding: authoring-architecture-store skill description (landing folds, drift probes); GI-017
  - id: F3
    class: F
    load_bearing: false
    angle: >-
      Excess check. Is the chosen cut heavier than the pain needs? A narrower cut, removing only
      instance enumerations and the product-truth leg, might answer it.
    grounding: hunt class excess machinery
  - id: G1
    class: G
    load_bearing: true
    angle: >-
      Downstream readers of what setup writes: review-feasibility FEASIBILITY-LENS,
      review-plan-artifacts ARTIFACT-CHECKLISTS, testing-governance-injection,
      patterns-technical-decisions DECISION-RECORD, and the design-baseline consumers. The design
      baseline template requires the truth headings, and implement's critique and craft-floor
      lenses read them. Dropping setup's writes leaves those consumers reading absent content.
      Even with relocation out of scope, the break must be named and held as a seam.
    grounding: grep of governance consumers across plugins/mochiko; mochiko-cli template design-baseline conformance
  - id: G2
    class: G
    load_bearing: true
    angle: >-
      The scope fence needs an owner. "Where product obligations live" is out of scope by the
      user's ruling, so the record must name an interim state (for example, design.md's truth
      part unwritten until X) and an owned open thread in BACKLOG. Without them, the relocation
      is an unowned decision.
    grounding: goal line scope clause; review-brainstorm critical-gaps criterion (unowned decision)
---

## Notes of note

- The fence held. No file under `.mochiko/brainstorms/setup-product-agnostic/` was read or
  listed. `.mochiko/brainstorms/index.md` was not opened.
- One directory listing of `.mochiko/brainstorms/` showed sibling slug names, including the
  name `index.md`. I filtered out this session's own directory name. No contents were read.
- Grounding covered the setup command and all seven rule sections, the authoring-constitution
  references (agenda, floor, compliance modules, catalog), the governance-intent and
  design-baseline templates, mochiko's DECISIONS, ROADMAP and BACKLOG, and kinako's governance
  region, rules files, intent amendment log, ledger outline and DECISIONS.
- Model-tiering deviation: every read was a known-path targeted read whose value was
  interpretive, namely classifying content as product-shaped or not. All reads stayed on the
  seat tier and no haiku explorer was spawned.
- Load-bearing means that a gap here would plausibly or likely have changed a ruling.
