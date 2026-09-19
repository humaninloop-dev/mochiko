---
report: review
topic: impeccable-design-integration
phase: cold-read-complete
pass: cold
lens: decision-quality
pairing: solo
reviewer: cold-reviewer
record_read: ".mochiko/brainstorms/impeccable-design-integration/record.md"
grading_baseline: "working tree plugins/mochiko/ at plugin 0.109.0"
recommended_status: critical-gaps
counts:
  critical: 2
  important: 10
  minor: 10
  survived: 22
  angles_mapped: 54
  dropped_as_ruled_moot_or_commentary: 8
blind_map_reference: "the angles: block below — written and returned to the lead before any record contact"
classes:
  - id: C1
    name: Non-negotiable conformance
    scope: GI-019 kernel-class, GI-020 clone-only and one-dependency, skills-and-agents-primary, the five skill-library axes
  - id: C2
    name: Artifact homes and the two trees
    scope: where PRODUCT.md and DESIGN.md live, home minting for the artifact gate, .impeccable versus .mochiko, derived versus authored design truth
  - id: C3
    name: Pipeline fire mechanics
    scope: the trigger condition, specify versus implement placement, write authority over product code, depth and breadth behavior
  - id: C4
    name: Collision with prior mochiko rulings
    scope: authoring-prototype, the throwaway-prototype ruling, the declined a11y probe, the a11y compliance module, the frontend shelf gap, the Later design track
  - id: C5
    name: Adoption-mechanism integrity
    scope: per-part mechanism, the null option, self-applied adopt-first and plan-minimalism ladders, part coupling, upstream drift, Apache-2.0 attribution
  - id: C6
    name: Operational and security surface
    scope: live mode, install side effects, unsupported environments, absence behavior, network calls, injection surface
  - id: C7
    name: Cost, evidence, and landing discipline
    scope: token budget, efficacy confidence, provenance, the landing ritual, release gates, reversibility, wave sequencing
angles:
  - id: A01
    class: C1
    q: "Is the 61-rule detector engine classified against GI-019 explicitly, and if admitted does the record cite the existing advisory-post-hoc-checker carve-out rather than minting a fresh kernel admission?"
  - id: A02
    class: C1
    q: "Does the record forbid the detector from gating pipeline progress: no deny-before-write on design judgment, exit code 2 never halting a cycle or a command?"
  - id: A03
    class: C1
    q: "Does the record hold the bright line that the 61 encoded thresholds are data served to a model, never judgment a binary holds, and argue the distinction rather than assert it?"
  - id: A04
    class: C1
    q: "Does adoption add a second required binary or runtime (the Impeccable engine, Node or npx), against GI-020 naming mochiko-cli as the one required dependency?"
  - id: A05
    class: C1
    q: "Does the record preserve clone-only install: no download-on-first-run into a home-directory bin path, no npx installer step, no submodule-class burden?"
  - id: A06
    class: C1
    q: "Are Impeccable's 24 commands reconciled with mochiko's deliberately small six-command surface, or does the record import a command explosion it never prices?"
  - id: A07
    class: C1
    q: "Is every adopted command or skill converted to the mochiko .md-plus-schema pair form served from the migration log, with no schema file shipped in the plugin?"
  - id: A08
    class: C1
    q: "Does each adopted model-invoked skill get a graded MUST/SHOULD description with exact trigger phrases, measured against the 1,536-char delivery truncation rather than estimated?"
  - id: A09
    class: C1
    q: "Does the adopted surface enter through the one user-invoked router, with user-invoked never calling user-invoked (axes 1 and 2)?"
  - id: A10
    class: C1
    q: "Are adopted agents rewritten persona-only, carrying no trace of any workflow (the keystone decoupling test), with skills declared (axis 4)?"
  - id: A11
    class: C1
    q: "Does every adopted reviewable artifact get a structurally independent validator (axis 5), or does Impeccable's own finish-reviewer agent end up grading work its own cluster produced?"
  - id: A12
    class: C2
    q: "Where do PRODUCT.md and DESIGN.md live? Impeccable writes them at project root; does the record place them under .mochiko/product/ instead, and give a reason?"
  - id: A13
    class: C2
    q: "If any design artifact lands under .mochiko/, is a home document minted in the migration log with its file set, ordered headings and budgets, or will the artifact-gate PreToolUse hook deny the write?"
  - id: A14
    class: C2
    q: "Does the record resolve the two competing artifact trees, Impeccable's ungoverned .impeccable/ working directory against mochiko's hook-gated .mochiko/, or leave both standing?"
  - id: A15
    class: C2
    q: "Is design truth derived or authored? ARCHITECTURE.md is a derived single-writer index over a store; does design truth get the same store-plus-index treatment, or a hand-maintained file with no re-render path?"
  - id: A16
    class: C2
    q: "Does PRODUCT.md duplicate what the capability map and the architecture store already hold (audience, purpose, operating context, constraints), and is that overlap adjudicated rather than accepted?"
  - id: A17
    class: C2
    q: "Are Impeccable's tracked-versus-gitignored file splits ruled on, given GI-003 pins .claude/settings.local.json gitignored and bars credentials from records?"
  - id: A18
    class: C3
    q: "What exactly triggers the design workflow? Does it reuse specify's existing UX-bearing intent ruling and its prototype-waived-at-intent line, or mint a second, competing trigger?"
  - id: A19
    class: C3
    q: "Which stage owns which part: what fires in specify as design-time truth versus in implement as build-time craft, and is any part claimed by both?"
  - id: A20
    class: C3
    q: "Do Impeccable's source-editing commands get write authority over product code, and if so how does that survive implement's rule that code is written only inside a TDD cycle card behind its TEST gate?"
  - id: A21
    class: C3
    q: "Is a missing design pass a clause in the entry sufficiency check, so it scopes the in-run design phase, or does the design workflow fire outside the gap list and escape the binding verdict?"
  - id: A22
    class: C3
    q: "How does the design step behave at low depth versus high depth, and does the breadth invariant (every row walked at both) survive a 24-command surface?"
  - id: A23
    class: C3
    q: "For a non-UX feature, is the design workflow provably inert, and which seat or clause asserts that rather than relying on the model noticing?"
  - id: A24
    class: C3
    q: "Where does a design finding land: a cycle card, a TEST assert, a spec gap, or advisory-only? The record must name one destination, not leave it to the seat."
  - id: A25
    class: C4
    q: "authoring-prototype already authors the clickable low-fi prototype and the Screens and Flows manifest. Is Impeccable's shape command the same job, and does the record rule one subordinate, merged, or dead with strips recorded?"
  - id: A26
    class: C4
    q: "The prototype is ruled throwaway by design and never migrates. Impeccable's DESIGN.md is durable and derived from shipped code. Is the durable-versus-throwaway boundary restated cleanly, or blurred by adoption?"
  - id: A27
    class: C4
    q: "testing-gap-finding records accessibility probing as declined, with the a11y floor staying a build-time standard. Impeccable's audit does a11y. Does the record supersede that ruling explicitly, or silently contradict it?"
  - id: A28
    class: C4
    q: "The a11y WCAG compliance module already exists as a seed keyed to accessibility statutes. Is Impeccable's a11y surface routed through that module, or does it open a parallel path that bypasses the module machinery?"
  - id: A29
    class: C4
    q: "patterns-code-minimalism's floor line already names accessibility as unsacrificeable at build time. Is the new design surface reconciled with that rung, or does it duplicate an obligation that already has a home?"
  - id: A30
    class: C4
    q: "The architecture shelves name frontend, mobile and desktop as Tier-I Stage-2 gaps. Does the record fill the frontend shelf with Impeccable's opinions through the existing shelf mechanism, or mint a structure parallel to it?"
  - id: A31
    class: C4
    q: "ROADMAP's Later list carries a non-committed design track with a ui-designer seat. Does the record acknowledge it is pulling that item forward, and is the ROADMAP touch part of the landing?"
  - id: A32
    class: C4
    q: "Impeccable's four visitor modes are a new taxonomy. Does adding them fire the standing GLOSSARY.md amend trigger, and do they collide with mochiko's existing capability frames, shelf stances and depth levels?"
  - id: A33
    class: C5
    q: "Is an adoption mechanism named for EACH part (vendor the text, depend on the plugin, coexist and hand off), or does one blanket verdict cover parts with different costs?"
  - id: A34
    class: C5
    q: "Is the null option steelmanned: both plugins installed side by side, mochiko adopting nothing and documenting only the hand-off? Impeccable already ships as its own Claude Code plugin."
  - id: A35
    class: C5
    q: "Does the record run mochiko's own patterns-adopt-first ladder on this decision, naming Impeccable as the real shelf candidate so any re-derived mochiko design guidance must win in writing against it?"
  - id: A36
    class: C5
    q: "Does the record disclose plan-minimalism rung claims per adopted element (required, simpler shape, already exists, minimum now, builder's room), or assert necessity without the ladder?"
  - id: A37
    class: C5
    q: "Are the parts independently adoptable? Four detector rules read DESIGN.md, so detector and DESIGN.md are coupled. Does the record test each part's dependencies before naming its mechanism?"
  - id: A38
    class: C5
    q: "Impeccable is actively developed against a pinned engine version. If text is vendored, who refreshes it, on what trigger, and what detects rot between refreshes?"
  - id: A39
    class: C5
    q: "Impeccable is Apache-2.0 and mochiko carries no LICENSE and no NOTICE file. Does the record name the attribution obligation and where the notice lands BEFORE any text is copied?"
  - id: A40
    class: C5
    q: "Does the record distinguish copying Impeccable's text from re-deriving the same judgment in mochiko's own voice and form, and price the second honestly instead of calling it adoption?"
  - id: A41
    class: C5
    q: "What stays out, and is each exclusion given a reason rather than appearing as a bare list?"
  - id: A42
    class: C6
    q: "Live mode runs a localhost HTTP server, injects JavaScript into the page, and executes a package.json script with the user's permissions. Is it ruled in or out, and on security grounds rather than convenience?"
  - id: A43
    class: C6
    q: "Impeccable's installer writes hook manifests into .claude/settings.local.json and can run hooks independent of model-tool approval. Is that install side effect ruled out explicitly?"
  - id: A44
    class: C6
    q: "mochiko declares shell-disabled environments and PowerShell-only Windows unsupported. Does the adopted surface widen that unsupported set, and is the declaration updated in the same move?"
  - id: A45
    class: C6
    q: "Absence behavior: if an adopted design step needs the Impeccable engine and it is missing, does the run halt loudly (mochiko's stated posture) or degrade (forbidden)? Silently skipping must be ruled out by name."
  - id: A46
    class: C6
    q: "Does anything adopted make a network call at run time (engine download, font index, URL scanning)? mochiko runs local; is a network dependency introduced without being named?"
  - id: A47
    class: C6
    q: "Is the prompt-injection surface considered, with third-party reference text becoming instructions inside mochiko's seats? Another harness gates project-local skills for exactly this reason."
  - id: A48
    class: C7
    q: "Token cost: the cost gate budgets every skill and agent with full ledger coverage. Is the adopted surface budgeted and its ledger rows seeded, or is the budget discipline silently broken?"
  - id: A49
    class: C7
    q: "Efficacy: is there any measurement that Impeccable guidance improves mochiko-produced UI, or is confidence honestly marked assumed at n equals zero with a named falsifier?"
  - id: A50
    class: C7
    q: "Provenance under GI-006: is every adopted primitive reconstructible from strips plus the migration log plus DECISIONS.md, with third-party origin traceable rather than laundered?"
  - id: A51
    class: C7
    q: "Is the landing ritual named as one move: the DECISIONS.md rows, the BACKLOG item, the ROADMAP Now or Next touch, and statuses agreeing across the brainstorms index and the record?"
  - id: A52
    class: C7
    q: "Does the record name the release gates a shipping bump must clear: audits PASS, strips recorded, CHANGELOG entry, marketplace sync, derived view equal to replay, cargo test and the contract suite green?"
  - id: A53
    class: C7
    q: "What is the revert path if adoption fails, is it a revert-the-wave shape like recent builds, and is rollback user-reserved?"
  - id: A54
    class: C7
    q: "Is the work waved with an abort condition at wave zero, as every recent mochiko build has been, or does it land as one undifferentiated change?"
record_strengths:
  - "Evidence honesty section states n=0 on every ruling and names three falsifiers; confidence marks are per-decision and the Assumed limbs (D6 desktop, D8 blocking line) are flagged rather than buried."
  - "D9's exclusion set gives a reason per excluded verb, which is exactly what goal-line (d) asked for."
  - "The adopt-first ladder is genuinely self-applied at D2: the shelf candidate is named and rejected in writing on the driver, not on capability."
  - "GI-019 is engaged precisely: the record distinguishes the advisory exit-code carve-out from the PreToolUse write-block rather than blurring them."
  - "D7 and OQ4 keep the throwaway-prototype ruling clean: the landing fold reads the built UI, never the prototype."
  - "The streak flag at Q6 and the Q7 no-recommendation steelman test are real ownership discipline, rare in a record."
findings:
  - id: R01
    severity: Critical
    attacks: "D3 rationale, D6, D8(iii), D10; build items 4 and 6"
    claim: "The record's accessibility placement rule is false against the working tree, collides with a floor-class rule, contradicts a recorded decline, and bypasses a recorded destination."
    evidence: "D3 rationale and D8 both state accessibility lives on the audit lens only. Working tree says otherwise in four places. (1) migrations/0001-genesis.yaml:6219-6230, rule patterns-code-minimalism.lazy-not-negligent, class: floor — no rung ever sacrifices a floor obligation or accessibility, named explicitly because the floor carries no a11y card yet, pending the frontend shelf. That rule runs at build-time decomposition on staff-engineer, the same seat D8 loads craft-floor onto, and D3 rules accessibility out of craft-floor. (2) migrations/0001-genesis.yaml:7565-7570, review-code-minimalism.floor-line-check, class: must — an accessibility-driven cut is a finding on the qa lens already. (3) skills/testing-gap-finding/SKILL.md:50-52, under the heading When NOT to Use — Accessibility probing, declined; the a11y floor stays a build-time standard. Build item 4 gives that same skill the dispatch line to a lens whose audit half is a11y. (4) skills/authoring-constitution/references/COMPLIANCE-MODULES.md:21 — an a11y (WCAG) module, stratum legal-mandate, already seeded and unwaivable per the strata rule at line 26. D6 mints a fifth a11y home, an accessibility floor in the design baseline, with no precedence rule against any of these."
    scenario: "The build lands the audit lens on qa-engineer at final validation and re-keys testing-gap-finding. The declined-probe line is either silently overwritten — which the author-not-grader audit's preserved-responsibilities check is specified to read as a regression — or it survives and the repo ships two contradictory a11y rulings in adjacent skills. Separately, a legally-mandated obligation acquires a second home in a product baseline with no stated precedence against the compliance module, and the floor rule's own recorded destination, the frontend shelf, is bypassed without a word."
    disposition: "rule inline — a D-level ruling on a11y precedence across the five homes, plus an explicit supersession entry for the testing-gap-finding decline. Lead's pen: the audit-lens-only placement was lead-recommended inside the D8 bundle and never put to the user on its own, so repairing it does not reopen a user ruling."
  - id: R02
    severity: Critical
    attacks: "D2, the Constraints-carried-in licensing bullet, build item 0"
    claim: "The port's licensing posture rests on a premise the record never states as a claim and therefore never tests, and the target plugin declares a different licence."
    evidence: "plugins/mochiko/.claude-plugin/plugin.json declares license: MIT. There is no LICENSE file at the repository root (verified absent) and no NOTICE file (build item 0 mints one). The record's constraint reads: Apache 2.0 on Impeccable — porting prose is licensed; attribution owed (NOTICE-style) on any verbatim carry. Build item 0 reads: the port is a rewrite to mochiko's split, never a verbatim carry. Those two lines together carry the whole licensing position, and the load-bearing step between them — that a rewrite discharges the obligations — is never written down, so it is never argued or checked."
    scenario: "A plugin distributed through a marketplace under a declared MIT licence ships content derived from an Apache-2.0 work, on an untested premise about what a rewrite discharges, in a repository that states no licence terms to a recipient at all. If the premise is wrong the remedy is not a NOTICE line, it is a licence decision about the shipped artifact, and it is far cheaper before the wave than after it."
    disposition: "rule inline AND escalate to the user. State the premise explicitly in D2 so it can be graded; add a root LICENSE file reconciling the declared MIT with the ported content; put the Apache-2.0-into-MIT question to the user before build. This reviewer is flagging an unverified load-bearing claim, not offering a legal conclusion — the disposition is the user's, not the lead's."
  - id: R03
    severity: Important
    attacks: "D1, D5, D6"
    claim: "The record's own ground fact F11 names three pipeline moments where UI gets made; D1 rules on two and drops the third silently, and the dropped one overlaps D5 and D6 directly."
    evidence: "F11 reads: (i) specify's UX-prototype stage; (ii) implement's cycle execution; (iii) /mochiko:architecture desk — frontend shelf not yet authored. D1 states it bites at both places UI gets made (F11). F10 additionally carries Stage 2 — frontend/mobile/desktop shelf authoring and Shelf builds — the translation tables (Tier I), frontend first, as open items, cited as ground fact and then never ruled on. The overlap is concrete: D6's brownfield detection leg targets UI framework, CSS system, fonts, token files, component library — the same rows a frontend architecture shelf would deal as stances. And per R01's evidence the a11y floor card's recorded destination is that shelf."
    scenario: "The build ships a design baseline that answers the styling-system, component-library and token questions, and the Tier-I frontend shelf later deals the same rows as architecture stances. Two stores answer one question with no precedence rule, which is the exact failure the architecture store's three-strata precedence rule exists to prevent."
    disposition: "rule inline — one decision stating the relationship between .mochiko/product/design/ and the unauthored frontend shelf: same thing, subordinate, or orthogonal with a named seam."
  - id: R04
    severity: Important
    attacks: "D5, D7; build item 5"
    claim: "The design store is minted with no authoring skill, no single-writer rule, and no ruling on whether it gets a derived index — the first product store without any of the three."
    evidence: "The architecture store's precedent, verified: authoring-architecture-store/SKILL.md:3 names the derived repo-root ARCHITECTURE.md index (single writer); its store-layout table line 57 marks the index regenerated, not hand-written; its landing-duties table line 78 requires index regenerated on a fold. .claude/rules/mochiko/operating-docs.md states ARCHITECTURE.md is DERIVED and NEVER hand-edit it. The record's three new skills are patterns-design-direction, patterns-craft-floor and review-design-finish — none is an authoring skill for the store. D5 names product-designer as author of record, D6 has setup writing the design-truth part and D7 has implement's landing fold writing the system part: two writers on one store, no single-writer rule, and no statement on whether a derived index exists or is declined."
    scenario: "Setup writes the truth part and the landing fold writes the system part with no shared grammar. The store's two writers drift in section shape; the artifact-gate hook enforces the home's headings but nothing enforces the lifecycle statuses, the As-built and Drift grammar, or the orphan rule the architecture store needed a whole skill to carry."
    disposition: "rule inline — name the store's writer and grammar owner, and rule the derived-index question explicitly either way."
  - id: R05
    severity: Important
    attacks: "build items 6, 7, 8; the Cost line"
    claim: "The build surface omits the migration-log work for every edited primitive, which post-wave-6 is where their rules actually live."
    evidence: "ROADMAP records wave 5 (all 30 skills CLI-served) and wave 6 as the end state: no schema file ships, the plugin carries the migration log only. The record's build items 2, 3 and 4 correctly say Schema pair for the three NEW skills. Items 6 and 7 edit setup, analysis-codebase, authoring-prototype and review-specifications, and item 4 edits testing-gap-finding, patterns-vertical-tdd and testing-end-user — seven primitives whose rule content is migration-log content — and no item names a migration. Item 8's gate list names CHANGELOG, plugin.json, marketplace.json, contract suite and cargo test but not the log. The release gate derived view equal to replay under the released binary range depends on exactly this work."
    scenario: "A builder works the record cold, edits seven SKILL.md files, and finds at the gate that the rules those primitives actually deliver are unchanged because they are replayed from the log. The wave either stalls at gate 6 or ships primitives whose prose and served rules disagree."
    disposition: "repair — add a migration item to the build surface naming the log edits for the seven edited primitives, and add the log to item 8's gate list."
  - id: R06
    severity: Important
    attacks: "D5, D7; the Cost line"
    claim: "review-sufficiency is given new reading obligations by two decisions and appears in neither the build surface nor the Cost line, and the design clause is never defined as new or existing."
    evidence: "D5: the sufficiency check at implement entry reads it as it reads the other baselines. D7: the sufficiency check's design clause reads the contract. Verified: skills/review-sufficiency/SKILL.md:3 describes a ten-clause check over the spec, the architecture store, and the product baselines; the clause list at lines 45-48 runs testable criteria, contract exposure, data exposure, structural trigger, NFR targets, commodity exposure, dependency order, UX trace, delivered-feature exposure, in-flight exposure. There is a UX trace clause but no design clause. The Cost line enumerates roughly seven edited primitives and review-sufficiency is not among them."
    scenario: "The build either adds an eleventh clause to a check whose ten-clause shape is named in its own description and its router row, or silently widens the UX-trace clause. Either way an unlisted primitive takes a graded edit with no strip planned and no audit seat assigned."
    disposition: "repair — add review-sufficiency to the edited set and rule whether the design clause is an eleventh clause or a re-keyed UX-trace clause."
  - id: R07
    severity: Important
    attacks: "D4; build items 1 and 8"
    claim: "The rename ripple list is incomplete in three verified places, and one of them must be regenerated rather than edited."
    evidence: "D4 lists: the persona file, its plugin.json entry, the router row, specify's dispatch brief, the migration-log references, the strips file, and the wave-B eval kit. A whole-repo grep for product-engineer additionally returns: ARCHITECTURE.md:195, :203 and :212; README.md:132; .mochiko/memory/primitive-cost-budgets.md:309. ARCHITECTURE.md is the derived index (R04's evidence) — build item 8 hedges ARCHITECTURE.md if the store carries the seats, and the grep proves it does, so the hedge is false and the re-key must go through the store and a re-render, never a hand edit. primitive-cost-budgets.md is the ledger GI-004's cost gate reads. Separately, F14 states no command file names product-engineer directly and the grep confirms zero hits under plugins/mochiko/commands/, so D4's specify's dispatch brief names a touch point that does not exist as a string. Router line skills/mochiko/SKILL.md:81 also enumerates the product baselines and would need the design store added."
    scenario: "The wave lands, ARCHITECTURE.md is hand-edited or left stale against the store, README ships the retired name to users, and the cost ledger row is orphaned so the budget gate reads a seat that no longer exists."
    disposition: "repair — complete the ripple list, mark ARCHITECTURE.md as re-render-not-edit, and drop or correct the non-existent dispatch-brief touch point."
  - id: R08
    severity: Important
    attacks: "D3, D9; build items 2, 3, 4"
    claim: "Three new skills are asserted without the simplest-execution ladder being run, and a cheaper shape exists for at least two of them."
    evidence: "patterns-plan-minimalism is mochiko's design-time ladder — required, simpler shape, already exists, minimum now, builder's room, stop at the first failing rung, disclosed rung-wise. The record discloses no rung claim for any of the three skills, the home, the template or the two new legs. D2's adopt-first disclosure is present and good; the plan-minimalism sibling is absent. Cheaper shapes the record does not test: patterns-craft-floor against a section added to patterns-code-minimalism, which already runs on staff-engineer at build-time decomposition and already carries the floor line R01 cites; review-design-finish's audit half against a section added to testing-gap-finding, which already owns the final-validation pass the lens is to ride, and its critique half against review-specifications' existing prototype walk."
    scenario: "Three new skills land where one new skill plus two sections would do. Each new skill costs a schema pair, a budget-ledger row against a ledger the skill-content-schema build recorded as having no headroom, a router row, a strips file and an audit seat — paid on a wave the record itself sizes at roughly three times the QA gap-finding build with n=0 evidence behind it."
    disposition: "repair — run the ladder and disclose a rung claim per new primitive, or record why the ladder does not bind a brainstorm build surface."
  - id: R09
    severity: Important
    attacks: "D9; the build surface port list"
    claim: "D9 claims to dispose of the other verbs with folds or reasoned exclusions, and three of the 24 have neither."
    evidence: "Enumerating the 24 against D9: craft, overdrive, live, generate, the hook, doctor, pin and comp-first are excluded with reasons; shape goes to D3, init to D6, critique and audit to D8, and eleven more fold at D9. Unaccounted: polish, document, extract. polish is worse than unaccounted — it is in the build surface's port list (skill/reference/{...,polish,...}.md, 25 files, matching the record's own roughly 25 upstream files) with no decision naming which of the three skills receives it. document and extract are absent from both the port list and the exclusion set, and they are the closest upstream analogue to D7's landing-fold derivation, so their silence is the one place the record should say the fold replaces them."
    scenario: "A builder reads the port list cold, opens polish.md, and has no ruling telling them whether it is craft-floor content, finish-lens content or redundant with both."
    disposition: "repair — one line each for polish, document and extract: fold target or exclusion reason."
  - id: R10
    severity: Important
    attacks: "D7 and D8 jointly"
    claim: "Advisory taste findings become binding baseline truth at landing, and neither decision addresses the path."
    evidence: "D8 rules the finding split: a violation of the direction contract, the design baseline, or the accessibility floor blocks; a taste finding advises. D7 rules that at landing the fold writes shipped into the baseline. D7's rationale defends this with: the finish lens grades the built UI against that contract before landing, so a mess reaches the baseline only if the lens passes it. Under D8's own split the lens does pass a mess whose only defect is taste, because taste never blocks."
    scenario: "Feature one ships a palette and type scale the critique lens flags as taste findings, advisory, unfixed. The landing fold writes them into .mochiko/product/design/ as the design system. Feature two's craft-floor is now obliged to honor them, and D8 makes a departure from the baseline a blocking finding. A one-off advisory has become a binding standard with no user ruling anywhere on the path."
    disposition: "rule inline — either a user checkpoint on the first landing fold that writes the system part, or a rule that unresolved advisory findings are carried into the fold as Drift rather than as baseline."
  - id: R11
    severity: Important
    attacks: "D3, D8(i); build item 3"
    claim: "craft-floor is classified as procedure and never argued to be procedure, and its own description reads as judgment."
    evidence: "D3: procedure lands as model-invoked skills ... a craft-floor skill loaded by staff-engineer ... (bans + reflexes, procedure not judgment). The parenthesis is the entire argument. F3 describes the upstream source as the quality floor, the absolute bans, and the reflexes no detector catches, and build item 3 adds the register dial (bolder / quieter / distill) read from the contract. Axis 4 is that persona carries judgment and skill carries procedure, with a persona containing no trace of any workflow. staff-engineer's persona carries TDD discipline and no design judgment. The record's own falsifier (a) names D3's placement, not the content, as the first suspect if the first live run shows no lift."
    scenario: "A design-taste obligation is delivered as a checklist to a seat with no design judgment to apply it with. The seat satisfies the bans literally and produces compliant but lifeless UI — precisely the outcome falsifier (a) is written to catch, discovered at n=1 after the wave has shipped."
    disposition: "fold — argue the procedure classification explicitly in D3, or move the judgment half to the direction contract that product-designer authors and leave staff-engineer only the literal bans."
  - id: R12
    severity: Important
    attacks: "D1, D8, D9; the whole build surface"
    claim: "The production floor's depth level is never mentioned, so the record never says how a three-skill design surface behaves at low depth."
    evidence: "CLAUDE.md's target is one production floor with a user-declared low/high depth level, breadth invariant at both, one-way ratchet (production-only PO-D1-D7 as amended by adaptive-depth D1-D8). Governance records depth: high for mochiko itself at GI-021. The record contains no depth ruling: not in D1's scope, not in D8's three bites, not in the build surface, not in the watches."
    scenario: "A low-depth project runs a UX-bearing feature and gets the full three-bite design surface, or gets none of it, depending on how a seat reads the absence. The breadth invariant says every row is walked at both levels, which constrains what a low-depth design step may skip, and nothing in the record tells the builder which parts are depth-conditional."
    disposition: "rule inline — one clause naming what fires at low depth and what the breadth invariant requires of it."
  - id: R13
    severity: Minor
    attacks: "D5 and D7"
    claim: "D5's edited-only-through-baseline-delta rule contradicts D7's landing-fold write as written."
    evidence: "D5: at build time it is edited only through the graded baseline-delta.md path every product baseline already carries. D7: At landing the fold reconciles declared against shipped, writes shipped into the baseline. Landing is build time. The architecture store carries both a graded-delta path and a landing fold, so the intent is almost certainly consistent; the word only is what breaks."
    disposition: "repair — name the landing fold as the second graded path in D5, or drop only."
  - id: R14
    severity: Minor
    attacks: "D2 and D10"
    claim: "D2 accepts upstream drift as a maintenance watch; D10 removes tracking and no drift watch appears in the watch list."
    evidence: "D2 rationale: at the cost of drift from upstream (accepted: drift is a maintenance watch, not a dependency). D10: Mochiko does not track Impeccable releases ... The revisit trigger for the ported content is mochiko's own evidence. Build item 10's watches are the critique blocking line, the desktop gap, budget fit, the first landing-fold derivation and the setup leg. No drift watch."
    disposition: "fold — reword D2's parenthesis to match D10, since D10 is the later and deliberate ruling."
  - id: R15
    severity: Minor
    attacks: "D8, D9, D10; the Question trail"
    claim: "The record's own streak discipline lapsed after the test it set for itself."
    evidence: "The Q6 streak flag notes Q3, Q5 and Q6 accepted the recommendation in sequence and states Q7 posed as steelmans with no recommendation to test ownership. Q7 passed. Q8, Q9 and Q10 then returned as recommended, as recommended and as proposed — three more unbroken, un-flagged. Those three carry the largest surface in the record: implement firing, fourteen verbs, and the 61 rules. D8 additionally carries an Assumed limb and a named falsifier."
    disposition: "defer to the lead — one more ownership test before build, best spent on D8's blocking-versus-advisory split, which is the Assumed limb with a falsifier already written against it."
  - id: R16
    severity: Minor
    attacks: "the build surface and the Cost line"
    claim: "No abort condition at the census and no revert path for the rename."
    evidence: "Build item 0 is a census of upstream file sizes against the budget ledger — a natural abort point — with no abort condition stated. Recent comparable builds carry one: the CLI wave-0 PATH-visibility abort, the skill-content-schema census-first rollout with numeric abort and user-reserved rollback. D4's rename is the wave's one non-additive act, rippling through plugin.json, the router, the log, strips, the eval kit and the files at R07; reverting it after ship is a second supersession, not a git revert."
    disposition: "repair — state a census abort threshold and a revert posture for D4."
  - id: R17
    severity: Minor
    attacks: "D10; build item 4"
    claim: "The measurable-rule asserts assume a browser driver in the consumer's repository, never priced."
    evidence: "D10: testing-end-user executes them through Playwright computed styles at the FLOW gate. GI-020 names mochiko-cli as the one required dependency. These asserts run in the consumer's product repo, so the expectation lands on the consumer, not on mochiko's install. F9 shows Playwright already used for the devils-advocate prototype walk, so the expectation is not new, but extending it to computed-style asserts at every UX-bearing FLOW gate widens it."
    disposition: "fold — one line saying the asserts are examples in a grammar, not a required toolchain, and that a project without a driver states its own equivalent."
  - id: R18
    severity: Minor
    attacks: "build items 2, 3, 4; the Constraints-carried-in axis-3 bullet"
    claim: "The 1,536-character description cap is carried as a constraint and never scheduled as a build step."
    evidence: "Constraints carried in names #3 model-invoked description grammar (less than or equal to 1,536 chars). Build item 0 censuses upstream size against the budget ledger, which is the token-cost gate, not the description cap. No build item says measure the descriptions. Axis 3's own wording is that delivery truncates at 1,536 chars — measure first."
    disposition: "repair — add the measurement to item 0 or to each skill's item."
  - id: R19
    severity: Minor
    attacks: "D3, D5, D6"
    claim: "The wave mints a new vocabulary and the standing GLOSSARY.md amend trigger is never checked."
    evidence: "New terms across D3, D5, D6, D7 and D9: mode in the Persuade / Operate / Read / Experience sense, direction contract, design truth, design system as a store part, evidence fence, craft floor, register dial. CLAUDE.md's governance operations list standing amend triggers: public-product transition, GLOSSARY.md content. The record's landing item 9 covers DECISIONS, BACKLOG, ROADMAP and the index, not the amend trigger."
    disposition: "fold — one line either firing the trigger or recording that it does not fire."
  - id: R20
    severity: Minor
    attacks: "D2 rejected roads"
    claim: "The do-nothing road is not among the rejected roads, and the record holds the fact that makes it live."
    evidence: "D2's rejected roads are compose whole and hybrid port plus optional detector. Coexist-and-hand-off — mochiko adopts nothing, both plugins stay installed, the user runs Impeccable between mochiko runs — is neither, and is genuinely new to the ruling. F8 records: Plugin is installed in this environment (impeccable:impeccable skill + the four impeccable:* agents visible in the session's agent list). Against that, the Cost line is roughly three times the QA gap-finding build and Evidence honesty states n=0 on every ruling."
    disposition: "defer to the user — D2 is Contested and this disposition is the user's, not the lead's. The user's driver (incorporate part of impeccable) most likely forecloses the road; the ask is one line in D2 recording that it was considered and foreclosed by the driver, not a reopen."
  - id: R21
    severity: Minor
    attacks: "D3, build item 2 against F9 and F14"
    claim: "Two skills on one seat write the same spec section with no boundary line between them."
    evidence: "Build item 2 gives patterns-design-direction the output = a Direction block in the spec's Screens & Flows. skills/authoring-prototype/SKILL.md:3 declares its own boundary as the static HTML app under .mochiko/specs/<feature>/prototype/ and the spec's Screens & Flows section (SCR-XXX, FLOW-XXX). Both are declared on the same seat, so there is no write race; what is missing is the obligations-only boundary line the skill-content-schema family requires between neighbouring skills."
    disposition: "repair at build — a boundary clause in each description, which the audit will want anyway."
  - id: R22
    severity: Minor
    attacks: "D4; build item 8"
    claim: "The rename is a consumer-visible breaking change and the record names neither a semver level nor an alias posture."
    evidence: "plugins/mochiko/.claude-plugin/plugin.json enumerates agents by path and declares version 0.109.0; a published marketplace entry exists at .claude-plugin/marketplace.json. Build item 8 says plugin.json bump without a level. Anything a user has saved that addresses mochiko:product-engineer breaks silently."
    disposition: "fold — name the semver level and rule whether a deprecation alias is owed."
dropped_candidates:
  - "Impeccable's .impeccable/ working tree, its installer writing to .claude/settings.local.json, and its runtime network calls — all mooted by D2's zero-dependency ruling, which is a ruling and not a gap."
  - "Prompt-injection surface from ported third-party prose — closed by the port itself plus the author-not-grader audit; nothing left to resolve, so commentary rather than a finding."
  - "Write authority of Impeccable's source-editing verbs over product code — D9's rationale answers it directly (every ad-hoc write would become a mini-pipeline under the sound loop)."
  - "Whether the design workflow is inert on a non-UX feature — D8's SCR/FLOW-trace condition and the never for planning-only work clause answer it."
  - "Token budget discipline — build item 0 engages the GI-004 cost gate explicitly; only the description-cap half survives, as R18."
  - "Part-independence of the detector from DESIGN.md — mooted once D10 declines the detector."
  - "Upstream drift ownership — D10 rules it cleanly; only the D2 wording residue survives, as R14."
  - "Adopt-first self-application — genuinely discharged in D2's rationale; no finding."
version_note: "mochiko-cli reports plugin 0.109.0 for the working tree and the skill render in this session reported plugin 0.108.0 from the installed cache. Recorded as a fact, not a finding: no ruling in the record depends on which copy is read, and all findings above were graded against the working tree."
verify:
  pass: verify
  date: 2026-09-19
  scope: "reopen-born verify bound — fold landing per finding, internal consistency of D11-D14 against D3-D8, the three counts, record fitness. No fresh cold read, no new blind map, no new hunt classes, no second reopen."
  status: NOT CLEAN
  landed: 22
  partial: 0
  missing: 0
  fold_introduced_residuals: 6
  residual_severity: {important: 2, minor: 4}
  fold_landing:
    - "R01 LANDED — D11 minted with the standard of record, a five-point precedence list, and the verification-only supersession; D3 gains the placement paragraph (line 77), D5 the pointer (93), D6 the pointer (103), D8 the standard-of-record wording in both the audit lens and the finding split (123); F16 added; build item 4 carries the supersession strip. The claim I broke — accessibility lives on the audit lens only — is gone from both places it appeared."
    - "R02 LANDED — D14 minted; D2 carries the licensing premise as a stated, gradeable claim (63); F17 added; item 0 owns LICENSE + NOTICE and gate 9 checks presence. Confidence honestly split: Confident on the actions, Assumed on the legal premise, with falsifier (d)."
    - "R03 LANDED — D12 minted with a stance-versus-value seam and a conflict precedence; F11 amended to route (iii) to D12; D1 amended to say the third moment is not a feature moment."
    - "R04 LANDED, all three limbs — D5 names product-designer as single writer, rules the grammar template-borne with no authoring skill, and declines the derived index with a reason (two files, nothing to index)."
    - "R05 LANDED — build item 8 minted, listing nine primitives and the views regeneration; gate 9 gains derived view equal to replay."
    - "R06 LANDED — D13 re-keys clause 8; verified against the working tree that clause 8 is indeed the UX-trace clause (review-sufficiency/SKILL.md:45-48, counting the list in order), so the ten-clause shape does survive; item 7 and the cost line both carry it."
    - "R07 LANDED — D4 carries the full ripple including the ARCHITECTURE.md re-render-never-hand-edit clause, README.md:132, the cost-ledger row and router :81/:142; F18 added; the non-existent dispatch-brief touch point is dropped and F14 reworded to dispatch rides the router row."
    - "R08 LANDED — the ladder is disclosed per new primitive with rungs named and two rung-2 kills taken; review-design-finish is dead and referenced only as a rejected road, no dangling mint."
    - "R09 LANDED — polish folds to the audit lens; document and extract are excluded with a reason that names D6 and D7 as doing their job. The allocation claim is true; its tally sentence is not (see V4)."
    - "R10 LANDED — D7 carries the first-write user checkpoint, road Y survives as post-checkpoint behavior, confidence records the ownership test, and falsifier (b) is widened to advisories piling into Drift unread. D13 keeps the checkpoint alive at low depth."
    - "R11 LANDED — D3 gains a real argument, not an assertion: every taste choice is made upstream in the contract, the seat decides nothing, and where it would decide it stops. Cross-referenced to falsifier (a)."
    - "R12 LANDED — D13 minted; presence of each bite invariant at both depths, only finish rigor depth-keyed, one-way ratchet stated."
    - "R13 LANDED — D5 now reads through the graded paths, naming both baseline-delta.md and the landing fold; the word only no longer excludes D7."
    - "R14 LANDED — D2's parenthesis reworded to drift accepted and not watched, matching D10; OQ2 notes the alignment."
    - "R15 LANDED as deferred — the ownership test was spent and Q10 carries the second-streak note. The claim about what it tested is slightly off (see V5)."
    - "R16 LANDED — item 0 abort at plus 25 percent of the family band; D4 carries the revert posture as a second supersession by ruling, user-reserved."
    - "R17 LANDED — D10 carries the not-a-toolchain line with the project-states-its-own-equivalent clause and the GI-020 reassurance; item 4 repeats it."
    - "R18 LANDED — item 0 measures each description against 1,536; items 2 and 3 repeat it; the constraints bullet records where it is measured."
    - "R19 LANDED — item 10 fires the trigger and scaffolds GLOSSARY.md, listing the minted terms. Under-scoped as a landing bullet (see V2 note and the disposition below)."
    - "R20 LANDED — D2's rejected roads carry coexist-and-hand-off, foreclosed by the driver, recorded so the road is visible and explicitly not reopened. Exactly the shape asked for."
    - "R21 LANDED — item 2 carries the boundary clause against authoring-prototype in both descriptions, and authoring-prototype is in item 8's migration list so the second description edit has a home."
    - "R22 LANDED — D4 names MINOR, no deprecation alias, with the router-dispatch reason and the CHANGELOG naming."
  counts_checked:
    - "Skills two not three — CONSISTENT. D3 line 73 says new skills: two; the cost line says 2 new skills plus 2 lens sections; the ladder lists two skills and two sections; build items 2 and 3 mint skills, item 4 mints sections. No survivor of the killed third anywhere but the rejected-road lines."
    - "Primitives nine — CONSISTENT. The cost line and build item 8 name the same nine in the same order. Item 8 marks patterns-code-minimalism conditional (if its floor line gains the cross-reference) while the cost line counts it flat; harmless, noted not raised."
    - "Verbs 24 all allocated — TRUE but MIS-TALLIED. See V4. Every one of the 24 does have a home; the sentence counting them does not add up, and two compensating errors hide it."
  fitness: "PASS — all fourteen decisions carry statement, rationale and a confidence mark. D11-D14 each carry all three. Assumed limbs are marked at D6 (desktop), D8 (blocking line), D14 (legal premise) and each has a falsifier in Evidence honesty."
  residuals:
    - id: V1
      severity: Important
      fold: "R08 — the kill of review-design-finish relocated the audit lens"
      line: "D3 line 73 (an audit lens section inside testing-end-user), D8 line 123 (iii), build item 4 line 196"
      claim: "The audit lens is placed inside a skill whose declared boundary excludes what the lens does."
      evidence: "testing-end-user/SKILL.md:3 — MUST be invoked when executing a **TEST:** verification task ... parsing its Setup/Action/Assert fields, running actions and capturing evidence, evaluating asserts; Consumes the **TEST:** grammar owned by mochiko:patterns-vertical-tdd. Its When NOT to Use list (lines 43-49) names Static code analysis tasks, Documentation review tasks, and Tasks without clear pass/fail criteria. The skill executes declared tasks; it originates no findings. The audit lens originates findings with no declared TEST case behind them (slop tells, polish and baseline alignment, perf, responsive) and carries a blocking-versus-advisory verdict split. That is review work in an executor skill. My R08 named testing-gap-finding as the target for this half; testing-end-user is the lead's substitution, so the mismatch is fold-introduced. Symptom already visible in the record: D13's low-depth clause reads the audit lens verifies the accessibility standard of record and the FLOW gate's asserts only, conflating the audit lens (D8 iii) with the FLOW gate (D8 ii) because both now sit in one skill."
      cheaper_shape: "The library already has the pattern: review-code-minimalism/SKILL.md:3 — the code-minimalism lens run by the verification seat ... Findings are ADVISORY ... Scope is the minimalism lens ONLY. A design-audit lens in that shape is a sibling of an existing family, not a new one, and keeps the executor skill clean."
      disposition: "repair — either move the audit lens to a lens-shaped home on the qa seat in review-code-minimalism's form, or record an explicit boundary widening of testing-end-user as a strip with its When NOT to Use list amended. Either way, fix the D8(ii)/(iii) blur in D13."
    - id: V2
      severity: Important
      fold: "R04 — the single-writer clause"
      line: "D5 line 93 (the setup leg and the landing fold both write through that seat), D6 line 103, build item 6 line 198"
      claim: "The single-writer rule collides with a class: must duty on setup and fits neither arm of setup's existing baseline-bootstrap rule."
      evidence: "migrations/0001-genesis.yaml:2050 — setup.interrogation-inline, class: must, kind: duty: Run the interrogation yourself, inline — the agenda's dimensions worked adaptively via mochiko:analysis-iterative. D6's leg is an interrogation (five asks), so its asking is bound inline to the lead, while D5 now binds its write to the product-designer seat; the record never says how the two meet, and setup dispatches no such seat today. Separately migrations/0001-genesis.yaml:2197 — setup.baselines-bootstrap: brownfield bootstraps the product baselines from the delivered code, greenfield seeds those baselines at the first implement run's design phase instead. An interrogation-sourced write at setup is a third pattern neither arm covers. Related gap: migrations/0001-genesis.yaml:2207 — setup.store-scaffold-unconditional scaffolds the architecture store on every path; build item 5 mints the design home but names no equivalent scaffold, so the design store's birth moment is unstated."
      disposition: "repair — rule how the design-truth write reaches the baseline at setup: inline authoring with a seat review, a dispatched producer setup does not seat today, or riding the baselines-bootstrap pattern. And say whether .mochiko/product/design/ scaffolds unconditionally like its sibling."
    - id: V3
      severity: Minor
      fold: "R05 — inserting build item 8 renumbered the gate list"
      line: "D14 line 173 — The build's item 0 owns (i)-(ii) and item 8's gate list checks their presence."
      claim: "Stale pointer: the gate list is item 9, not item 8. Item 8 is the migration-log work R05 inserted."
      evidence: "Build surface items: 0 pre-wave, 1 rename, 2 design-direction, 3 craft-floor, 4 lens sections, 5 baseline home, 6 setup leg, 7 specify and implement-entry re-key, 8 migration-log work, 9 ripple and gates (line 201, which is where LICENSE + NOTICE present sits), 10 landing, 11 watches. The Review table's R05 row already says gate 9 correctly, so the record disagrees with itself."
      disposition: "repair — change D14's pointer to item 9."
    - id: V4
      severity: Minor
      fold: "R09 — the completeness sentence added to D9"
      line: "D9 line 133 — eleven + `polish` → this decision's folds, eight excluded."
      claim: "The allocation is complete and correct; the sentence counting it is wrong twice, and the errors compensate so the total still reaches 24."
      evidence: "Folded verbs, distinct: onboard, clarify, adapt (design-direction); typeset, layout, colorize, animate, delight (craft-floor); bolder, quieter, distill (the dial); harden, optimize, adapt, polish (audit lens) — fourteen distinct, with adapt folded in two places. Excluded entries in D9's list number ten, but only six are among the 24: craft, overdrive, live, generate, document, extract. The other four — comp-first build path, the edit-time hook, doctor, pin — are not commands in the 24; F7 itself says doctor and hooks are utility, kept out of the design menu. Correct tally: fourteen folded, six excluded, four to D3/D6/D8, total 24."
      disposition: "repair — restate the sentence as fourteen folded (adapt twice), six excluded, four allocated to D3/D6/D8."
    - id: V5
      severity: Minor
      fold: "R15 — the second-streak note added to Q10"
      line: "Q10 line 231 — the review's ownership test (Q11-A5) was spent on D8's `Assumed` limb, as the reviewer recommended."
      claim: "The record slightly overstates what its own ownership test tested."
      evidence: "Q11-A5 as posed (line 232) was R10 x R15, roads X (user checkpoint) / Y (Drift: only) — the fate of advisory findings at the landing fold, which is D7's checkpoint. D8's Assumed limb is whether a taste finding should block at all, and it was not itself re-put to the user; D8's confidence line still carries it as Assumed with falsifier (b) untouched. The test was real and on the adjacent question, which is defensible — the sentence claiming it was spent on D8's limb is what is inaccurate."
      disposition: "fold — reword Q10's note to name what A5 actually tested, or put D8's split to the user before the wave."
    - id: V6
      severity: Minor
      fold: "R02 — D14's attribution clause"
      line: "D14 line 173 (ii) — the Apache 2.0 license text or a pointer to it."
      claim: "The clause offers an alternative that weakens the obligation its own rationale relies on."
      evidence: "D14's rationale (line 175) rests the posture on the Apache license and notices retained for the derived material. Retaining a license and pointing at one are different acts, and the clause lets the build choose the weaker without a reason. This is an internal inconsistency between a clause and its rationale, not a legal opinion."
      disposition: "repair — drop the or a pointer to it alternative and include the license text, which is the cheaper and safer default."
  delta_check:
    date: 2026-09-19
    scope: "the six repairs only — no re-read of the rest of the record, no findings outside what a repair introduced."
    status: NOT CLEAN
    residuals_landed: 6
    residuals_partial: 0
    residuals_missing: 0
    repair_introduced: 5
    repair_severity: {important: 1, minor: 4}
    landing:
      - "V1 LANDED, and landed harder than asked — the audit lens is split by FORM, not relocated whole: pass/fail legs (the accessibility standard of record, the measurable asserts, responsive at declared viewports) become `**TEST:**` cases qa-authored at design time and executor-run, which is testing-end-user's own declared form; finding-originating legs become review-design-audit in the review-code-minimalism form on qa-engineer. The ladder was genuinely re-run rather than rubber-stamped: build surface line 189 discloses rung 2 failing for BOTH cheaper shapes, naming testing-end-user/SKILL.md:49 and review-code-minimalism's minimalism-lens-ONLY scope line as the two reasons. Skills 2 to 3, disclosed in D3 line 73, the cost line, and the Review table's R08 row. D13 correctly re-keyed to depth-condition the critique lens only, since both audit halves are now depth-invariant by form. Item 4 gives testing-end-user only the computed-style note, boundary untouched."
      - "V2 LANDED — D6 line 103 splits asking from writing: the lead interrogates inline so setup.interrogation-inline is honored by name, and the write is dispatched to a product-designer producer plus a non-author validator under the sound loop, which keeps D5's single-writer line true at setup. setup.baselines-bootstrap gains an explicit third arm (truth part interrogation-sourced on every path, system part code-seeded brownfield and scaffold greenfield), and the design home scaffolds unconditionally on the setup.store-scaffold-unconditional pattern, so the store's birth moment is now stated. Items 5 and 6 carry both."
      - "V3 LANDED — D14 line 173 now reads item 9's gate list, which is correct: gates are item 9 (line 202)."
      - "V4 LANDED and arithmetically correct — D9 line 133 now reads 4 + 14 + 6 = 24 with the fourteen folds enumerated (adapt counted once), the six excluded commands named, and the four non-command exclusions (comp-first, the edit-time hook, doctor, pin) separated out with the F7 reason. I re-checked each of the three groups against F7's own enumeration; all three are right."
      - "V5 LANDED — Q10 line 232 now says the test was spent on R10's roads, the fate of advisories at the landing fold, adjacent to D8's split which stays Assumed with falsifier (b) untouched. Accurate."
      - "V6 LANDED — D14 line 173 now reads the Apache 2.0 license text retained in the repository, with the pointer alternative struck and the reason given inline."
    repair_introduced_residuals:
      - id: W1
        severity: Important
        repair: "V1 — the audit-lens split"
        line: "D8 Rationale line 125 and D8 Rejected line 127"
        claim: "D8's rationale and rejected-road lines were not carried through the repair and now contradict D8's own repaired statement."
        evidence: "Line 125 still reads: the lenses are sections on skills those seats already load, no new seat, no new gate, no new skill (R08). Line 123, repaired in the same decision, mints review-design-audit as a new skill on qa-engineer, and D3 line 73 says new skills: three. Line 127 still rejects a third new skill review-design-finish on the reason that two sections on two existing skills do the same work — the reason V1 disproved, since a section inside the executor breaks its boundary, which is why a third skill landed. A decision whose rationale denies what its statement mints, and whose rejected road is refused for a reason the record has since overturned, will read to an auditor as either a stale edit or an unrecorded reversal."
        disposition: "repair — two sentences: strike no new skill from line 125 and say the audit split costs one review-family sibling; re-key line 127 to reject review-design-finish on its actual surviving reason (a single finish skill spanning two seats and two forms) rather than on the two-sections claim."
      - id: W2
        severity: Minor
        repair: "V1 — skills 2 to 3"
        line: "build item 0 line 193 and build item 8 line 201"
        claim: "Two build items still enumerate two new skills after the count went to three."
        evidence: "Line 193: census the upstream files above and the rewritten drafts of the two new skills against the skill budget ledger. Line 201: carrying: the two new skills' schema pairs. D3 line 73, the cost line at 208 and the ladder at 183-191 all say three. Item 4 does name review-design-audit's own schema pair and description measurement, so the work has a home and nothing is lost — the two enumerations simply under-count, and item 0 is the abort gate, so its census would skip the third skill's budget."
        disposition: "repair — the three new skills in both lines."
      - id: W3
        severity: Minor
        repair: "V2 — the setup write path, against R13's earlier generalisation of D5"
        line: "D5 line 93 against D6 line 103 and item 5 line 198"
        claim: "D5's path enumeration is now closed against the two setup writes D6 mandates."
        evidence: "D5 line 93 reads: Baseline edits travel only through the graded paths: baseline-delta.md at build time and the landing fold at feature close (R13) — unqualified, two paths. V2's repair adds two more writes at setup: the design-truth part written by a dispatched product-designer producer with a non-author validator and user ratification (D6 line 103), and the unconditional scaffold of the design home (D6 line 103, item 5 line 198). Both are legitimately graded — the first carries the full sound loop — so this is an incomplete enumeration rather than a conflict of substance, but as written D5 forbids what D6 requires and a builder reading D5 alone would reject the setup write."
        disposition: "repair — name setup's graded write as the third path, and the scaffold as a creation rather than an edit, in D5's enumeration."
      - id: W4
        severity: Minor
        repair: "V1 — D13's depth re-key"
        line: "D13 line 165"
        claim: "D13 cites the audit lens's pass/fail legs to D8 (ii); D8 places the whole audit lens under (iii)."
        evidence: "Line 165 reads: the audit lens's pass/fail legs (`**TEST:**`-shaped, D8 ii). D8 line 123 puts the audit lens, in two forms on qa-engineer, inside (iii) Final validation; (ii) is the FLOW-XXX cycle gate. The legs take (ii)'s FORM and sit at (iii)'s MOMENT, which is coherent, but the bare cite reads as a location and points at the wrong bite."
        disposition: "repair — in the form of D8 (ii), under D8 (iii)."
      - id: W5
        severity: Minor
        repair: "V5 — the Q10 correction"
        line: "Review table R15 row, line 261"
        claim: "The disposition table still carries the claim V5 corrected, so the record now says two different things about its own ownership test."
        evidence: "Line 261 reads: ownership test spent on D8's limb (Q11-A5); noted in Q10. Line 232, repaired by V5, reads: spent on R10's roads — the fate of advisories at the landing fold, D7's checkpoint — adjacent to D8's Assumed split. The table row is the stale half."
        disposition: "repair — re-key the R15 row to match Q10."
    delta_note: "All five are repair-introduced and in bound; none reopens a user ruling, and none touches a decision's substance. W1 is the only one that misstates a ruling rather than a count or a pointer. The V1 repair is better than the fix I proposed — I suggested relocating the lens to a review skill; splitting it by form keeps the pass/fail half on the deterministic layer where it belongs and confines the new skill to the findings that genuinely need one. The V2 repair likewise answers a question I raised without ruling on it: the asking-versus-writing split is the shape that honors setup.interrogation-inline and D5's single writer at once."
  verify_note: "All six residuals are fold-introduced and in bound. Nothing here reopens a user ruling: V1 and V2 are build-surface repairs under D3/D5/D6/D8 as ruled, V3-V6 are wording or pointer fixes. The two Criticals that produced the critical-gaps verdict are discharged — D11 and D14 both answer the finding as raised, and D14 in particular is more honest than I asked for, marking the legal premise Assumed with a falsifier rather than asserting it."
---

## Failure narrative

Delta-check NOT CLEAN — all six residuals V1-V6 landed, five repair-introduced issues
remain (one Important, four Minor). None reopens a user ruling; none touches a decision's
substance. The two original Criticals stay discharged.

W1 is the only one that misstates a ruling rather than a count or a pointer. D8's statement
(line 123) now mints `review-design-audit`, but D8's own rationale (line 125) still reads
"no new seat, no new gate, no new skill", and its rejected-road line (127) still refuses a
third skill because "two sections on two existing skills do the same work" — the reason V1
disproved. A decision that denies in its rationale what it mints in its statement reads as
either a stale edit or an unrecorded reversal. Two sentences fix it.

The four Minors are carry-through misses: build items 0 and 8 still say "the two new
skills" (W2); D5's path list is closed against the two setup writes D6 now mandates (W3);
D13 cites the audit lens's pass/fail legs to D8 (ii) when the lens sits at (iii) (W4); and
the Review table's R15 row still carries the claim V5 corrected in Q10 (W5).

## Notes of note

Round history in one line: 22 survivors of 54 angles, all folded and all landed; six
fold-introduced residuals, all repaired; five carry-through issues from those repairs.
Both V1 and V2 were repaired better than I proposed — splitting the audit lens by form
keeps the pass/fail half on the deterministic layer instead of moving it wholesale, and the
asking-versus-writing split at setup honors two rules I could only show were in tension.

The V1 ladder was genuinely re-run, not rubber-stamped: build surface line 189 discloses
rung 2 failing for both cheaper shapes and names why for each. That is the disclosure R08
asked for, working as intended on a decision it had already changed once.

Bound honored: the six repairs only, no re-read of the rest of the record, nothing raised
outside what a repair introduced. Every cited line was re-checked in place this round. The
clearing verdict remains the lead's; my status is input.
