# Skill-content schema — authoring-family census (wave 2, paper exercise)

**Seat:** census-authoring · **Date:** 2026-09-01 · **Status:** delivered, awaiting wave gate
**Referent law:** `.mochiko/brainstorms/skill-content-schema/record.md` D1–D9 as amended ·
the wave-1 census (`census.md`, same directory) as structural referent · command-content-schema
D12/D15 · near-dup ADR R1–R6 (`.mochiko/decisions/2026-08-28-near-dup-convergence.md`) ·
`.claude/rules/mochiko/primitive-edits.md` skill-pair criteria block (v0.100.0).
**Corpus:** the 8 authoring-family skills under `plugins/mochiko/skills/` — `SKILL.md` whole +
every `references/*.md` whole (authoring-constitution: **9** reference files — 6 root + 3 under
`references/catalog/`; the dispatch brief said 7, corrected here) + all 8 strip files +
`DECISIONS.md` rows. STRICTLY READ-ONLY; this file is the only write.
**Measurement:** characters of the parsed value (frontmatter `description:` value; body = content
after the closing `---`), matching the canonical-snippet counts in
`.mochiko/memory/primitive-cost-budgets.md` — spot-verified against five strip-recorded figures
(AFM 15,975 · AR 4,413 · AE 7,821 · ATR 10,500 · AAS 10,884), all exact.

Member shorthand (filename stems remain the ID prefixes per the wave-1 R-b ruling; shorthand is
presentation only): AAS `authoring-architecture-store` · AC `authoring-constitution` ·
AE `authoring-epic` · AFM `authoring-feature-map` · AP `authoring-prototype` ·
AR `authoring-requirements` · ATR `authoring-technical-requirements` ·
AUS `authoring-user-stories`.

All rule IDs are **provisional** (mint-once fires at conversion, never here). Dispositions use
the D3-as-amended vocabulary: **body-stays-prose** / **moves-to-schema** / **reference-stub**
(stub default; lift only where the reference home is incidental — zero lifts found family-wide).
Every move of protected content is a supersession-transfer per D8/C4, recorded once at
conversion, protection re-homing onto the schema rule ID in the provenance sidecar.

---

## A. Protected-set reconciliation (FIRST, per D9/C4)

**Counting note.** Unlike the review family, most of this family's protection is
**DECISIONS-traceable ruled machinery rather than `KEPT:` survivor rulings**. Grep of the 8 strip
files: `KEPT` appears in 5 of 8 (AC ×7 mentions, ATR ×3, AR ×2, AP ×1, AUS ×1; AAS/AE/AFM ×0).
Distinct **live** KEPT/ruled-form survivor rulings: **7 across 5 members** — AC's [v0.28.0]
whole-remaining-body KEPT (2 elements already superseded by recorded ruling at v0.63.0, rest
live) · AR's [v0.28.0] KEPT set + [v0.23.0] ruled compact Key-Entities form ("untouchable" per
strip) · ATR's [v0.28.0] + [v0.23.0] kept-sets · AUS's [v0.23.0] T1-ruled story form
("untouchable") + [v0.28.0] KEPT set · AP's [v0.58.0] re-tag/greying kept-set (reconciled as
protected at v0.63.0). AAS, AE, and AFM are RSUF-class: birth-by-ruling or fully-re-typed bodies
whose protection is `DECISIONS.md`-traceable per rule — each such move recorded at conversion
citing its decision row, no whole-body claim (the wave-1 R-c idiom). Nothing below is silently
excluded; ended protections are listed with their ending ruling. **No disposition is "delete".**

### AAS — authoring-architecture-store (strips: 2 entries; no KEPT lines)

Born v0.81.0 by ruling (product-architecture-schema D3/D4/D7/D10/D12/D16, `DECISIONS.md`
2026-08-19); re-keyed v0.91.0 (plan-stage retirement D1/D3 + lead ruling R4, `DECISIONS.md`
2026-08-26).

| Protected unit | Lineage | Census disposition |
|---|---|---|
| Write gate — "the sign-off IS the write gate. No sign-off, no store write" (kept byte-for-byte at v0.91.0) | D10 + v0.91.0 strip "Kept deliberately" | **moves-to-schema** (`class: floor`, `kind: gate`) |
| Full-AX-table / "a missing row is an invisible row" + named readers | D4 + v0.91.0 strip | **moves-to-schema** |
| NFR one-home + `FR-XXX / SC-XXX → NFR-XXX` chain resolves | D12 + wave-lead R4 (strip) | **moves-to-schema** |
| Single-writer derived index; regenerate never hand-edit | D4 | **moves-to-schema** (floor) |
| Landing diff / fold trigger split — "Never gate the diff on whether structure was built" | D10 six-step lifecycle | **moves-to-schema** (floor) |
| Orphan rule (supersedes AT-D6-C In-flight invariant) | D10 | **moves-to-schema** |
| First-visit reconstruct-and-confirm + archive | D16 migration | **moves-to-schema** |
| Sound-loop review leg on judgment writes | patterns-sound-loop floor (pointer) | **moves-to-schema** (floor, pointer) |
| Two-arm schema citation (`mochiko-cli template architecture-store` / raw Read) | GI-020, template-schema D8 | **moves-to-schema**, both arms preserved |

### AC — authoring-constitution (strips: 12+ entries; the family's richest file)

| Protected unit | Status | Census disposition |
|---|---|---|
| [v0.28.0] KEPT: the remaining body (under-band survivor ruling) | **live, minus 2 recorded supersessions** — the enforcement-strength table and the brownfield action-mapping paragraph left at v0.63.0 by recorded ruling; survivors named: synthesis contract · surface routing · module-assembly table · both mode contracts · ceiling test · floor-accounting + mode-prerequisites blockquotes · Three-Part Rule · RFC 2119 · inventory behavioral riders | per D8/C4: the whole-body survivor ruling **re-homes onto the pair** (body + schema jointly), recorded once — the RF precedent replayed; the named survivor obligations **move-to-schema**, their narrative stays prose |
| [v0.65.0] adaptive-depth machinery (D1–D8, 12 supersession entries across SKILL.md + 5 reference files): two-row dial · breadth invariant · level-blind modules · D6 no-watcher flip ceremony · agenda test · no-pruning license | live, spread body + references | body-borne rules **move-to-schema**; reference-borne (floor cards, catalog doctrine, compliance strata) **reference-stub** |
| [v0.76.0] two-arm governance-surfaces/-intent schema pointers | live (2 body sites + 1 in INTERROGATION-AGENDA) | **moves-to-schema** (body), both arms preserved (GI-020); agenda copy rides the file (J-2) |
| D8 ownership boundary: markers-only regeneration · preserved domain-registry + output-style blocks · read-and-re-emit · superseded constitution.md | live (DECISIONS 2026-07-18 constitution-native-surfaces lineage) | **moves-to-schema** (floor-class set) |
| D4/D4.1/D4.2 waiver discipline (permanent-pending-revisit; legal-mandate unwaivable) | live, body + COMPLIANCE-MODULES + universal-floor | body rule **moves-to-schema**; reference copies **reference-stub** with dedup note |
| [v0.44.0]/[v0.28.0]/[v0.22.0] cuts | ended (recorded) | n/a — historical |

### AE — authoring-epic (strips: 6 entries; no KEPT lines)

Born v0.72.0 (multi-feature-plan-implement D1–D15, `DECISIONS.md` 2026-08-14); re-keyed v0.81.0
(product-architecture-schema D3/D10) and v0.91.0 (plan-stage retirement D1/D4).

| Protected unit | Lineage | Census disposition |
|---|---|---|
| Mint-once / resolve-by-lookup; desk the only mint door; overlap guard in full ("surfaces to the user … never a silent duplicate") | D4 (multi-feature) + plan-stage D4, kept deliberately at v0.91.0 | **moves-to-schema** (floors) |
| One open epic per feature's pending rows | D4 | **moves-to-schema** (floor) |
| Epic-run-always-fires-design-phase for the joint spine | plan-stage mechanic (b), v0.91.0 strip | **moves-to-schema** |
| One signed store delta, rendered once signed once | product-arch D3/D10, v0.81.0 strip "Kept deliberately" | **moves-to-schema** (floor) |
| Cross-member seam owner named in spine; no later-lander default | D13 | **moves-to-schema** |
| Shared-baseline single pen-holder; fold each baseline exactly once | D10 review fold C1 | **moves-to-schema** (floor) |
| Selection-scope only; delta-scope cards cannot join | D11 | **moves-to-schema** (floor) |
| Carve-out / hold reserved to the user, never the lead | D7 | **moves-to-schema** (floor, reservation) |
| Transient role; directory persists as record; map stays two-typed | D3 | **moves-to-schema** |

### AFM — authoring-feature-map (strips: 17 entries; no live KEPT-form)

The heaviest supersession lineage in the family (v0.61.0 → v0.91.0); every prior KEPT-class
survivor was itself superseded by recorded ruling at the v0.68.0 re-type (the strip's
protected-content reconciliations are explicit). Live protection = DECISIONS-traceable machinery:

| Protected unit | Lineage | Census disposition |
|---|---|---|
| The 8 hard invariants (one-home · disposition · row-level closure · map-owns-status · sticky/fold · sacred-writes/acceptance-batch/stewardship-direct · integrity-fix-on-sight · index-never-rewrite) | pm-role D2/D6/D7/D8 + feature-sizing G4 (`DECISIONS.md` 2026-08-13 / 2026-08-10) | **move-to-schema** (floor-class set, with splits) |
| Capability/work-row re-type + frame-first + story-wins | pm-role D1/D2/D5 | **moves-to-schema** |
| Four touchpoints incl. mechanic-(e) zero-gap branch (the +128 declared-overage content, ruled HOLDS) | plan-stage mechanic (e), v0.91.0 strip | **moves-to-schema** — the overage's ruling trail carried in the strip at conversion (J-6) |
| Two-arm features-index / feature-entry schema bindings | template-schema D8 (v0.76.0) | **moves-to-schema**, both arms |
| Stub discipline (unratified hypotheses · selectability-specify-only · no forced cuts · escalation-is-recommendation) | feature-sizing D2a/D12/D13 + v0.62.0 ruling | **moves-to-schema** |
| BACKLOG boundary, KM-scoped, extent-growth exception | D13 (feature-sizing) | **moves-to-schema** |

### AP — authoring-prototype (strips: 3 entries)

| Protected unit | Status | Census disposition |
|---|---|---|
| [v0.58.0] FEAT re-tag pass + coming-soon greying grammar + rejected-story invariant (R10, feature-map-layer D4) | live — the v0.63.0 entry reconciles it as protected, kept in full | **moves-to-schema** (invariants 7–8) |
| UX-D1–D9 core: two coupled artifacts · bun + file:// degrade path (no build step) · binding-flows/advisory-pixels authority split · lockstep · skeleton-first · scenario keying · manifest↔HTML agreement | live (DECISIONS 2026-08-02) | **move-to-schema** (authority split + degrade path as floors) |
| [v0.76.0] two-arm spec schema pointer | live | **moves-to-schema**, both arms |
| [v0.63.0] guardrails kept-set (8 invariants + checklist + red flags) | live | **move-to-schema** per rows above |

### AR — authoring-requirements (strips: 5 entries)

| Protected unit | Status | Census disposition |
|---|---|---|
| [v0.28.0] KEPT set: FR/SC fenced format blocks ("the only explicit homes of the line shapes") · SC rules · entity-description rules · Quality Checklist | live | format-block **shapes stay prose** (they are the artifact grammar's teaching rendering); the obligations they carry (RFC 2119 keywords, numbering, SC rules, conceptual-only entities) **move-to-schema** |
| [v0.23.0] ruled compact Key-Entities form ("untouchable") | live | rules **move-to-schema**; compact form itself stays prose |
| [v0.91.0] constraint-vs-posture rule, kept entire with its worked example | live | rule **moves-to-schema** (floor); worked example stays prose |
| Design-track carve-outs (kept entire, name re-keyed) | live | **move-to-schema** (routing) |

### ATR — authoring-technical-requirements (strips: 8 entries)

| Protected unit | Status | Census disposition |
|---|---|---|
| [v0.28.0]/[v0.23.0] kept-sets: three-artifact spine · Traceability Rules mandatory links · "'Fast' is not a requirement" floor · "constraints are facts" · three no-exceptions lines · Common Rationalizations · letter/spirit epigraph · T2 statement-line form | live (v0.64.0 + v0.91.0 reconciliations: "No prior KEPT or protected line is touched") | obligations **move-to-schema**; the T2 field forms **reference-stub** (they live in ARTIFACT-TEMPLATES.md) |
| [v0.81.0] D12 products: no-`nfrs.md` / store-row home; structural-decisions-never-D-XXX carve-out + entangled-case route | live (`Contested` D12, user-ruled absorb) | **move-to-schema** (floors) |
| [v0.91.0] D3 products: TR layer dead (no TR rules may resurface); INT/DS re-home (wave-lead R1); NFR business source (R4); design-ladder blockquote | live | **move-to-schema**; the death of TR-XXX is a conversion guard, not a rule (nothing to mint) |

### AUS — authoring-user-stories (strips: 4 entries)

| Protected unit | Status | Census disposition |
|---|---|---|
| [v0.23.0] T1-ruled story form ("untouchable" per the v0.28.0 strip): one-line scenarios, 2-3 cap with compound-story rationale | live | the caps and one-line rules **move-to-schema**; the fenced format block stays prose (teaching rendering) |
| [v0.28.0] KEPT set: format block · Independent Test bullets ("only home") · Quality Checklist · Common Rationalizations · letter/spirit epigraph | live | obligations **move-to-schema** |
| [v0.91.0] design-track carve-outs, kept entire | live | **move-to-schema** (routing) |

**Reconciliation totals:** 7 distinct live KEPT/ruled-form survivor rulings across 5 members
(plus AC's 2 already-ended-by-ruling elements inside its whole-body KEPT); 3 members (AAS, AE,
AFM) carry DECISIONS-traceable protection only — the RSUF/R-c idiom, each move recorded citing
its decision row. ~48 live protected units enumerated above, every one with a named disposition;
zero stay silent; zero deletes. AC's whole-body [v0.28.0] survivor ruling re-homes onto the pair
per the D8/C4 explicit-clause precedent (as review-feasibility's did).

---

## B. Obligation census at D12 grain

One row per independently-citable obligation (a/b splits noted). `class`: floor / must /
advisory. `kind` from the **eight-kind skill set** (`fail` and `enforces:` retired at v0.100.0,
checker-enforced — surveyed: **no authoring member asserts a run-fail predicate either**; the
corpus's MUST/STOP language is all artifact-grammar and process constraint, so the retirement
holds without re-admission). Disposition: **move** / **stub** / **prose**.

Procedure and teaching prose is NOT inventoried (D3): AC's mode walkthroughs and module tables'
narrative, AFM's vocabulary teaching, AP's structure sketch, AR/AUS's fenced format examples,
ATR's field tables (reference-side), all Common-Rationalizations "Reality" columns, and all
worked examples stay prose. Red-flag / checklist rows that mirror an inventoried rule are dedup
notes at conversion, not extra rules — only rows carrying a distinct obligation are counted.

`when:` dimensions observed live (D4 validation): AAS approved-delta-present ·
structure-built · no-ruled-content (first visit) — all surface-presence · AC mode
(greenfield/brownfield/amend, entry-derived) + synthesis-present (surface-presence) + rules-file-
in-set (surface-presence, the standing read line) · AFM KM-exists · epic-context (surface-
presence) · AP UX-bearing ruling (entry-derived) + design-system-exists (surface-presence).
All resolvable as entry-derived or surface-presence; **no new resolution kind needed.**

**Section fit is graded against the review-family six-set** (`independence` · `scope` · `inputs`
· `verdict` · `output` · `reserved`) as the skill-pair criteria block currently hard-codes it.
Headline finding, detailed per member and in J-1: **`verdict` is empty for all 8 members** (these
are producers — no clearing grammar exists), and the family's densest rule class — the produced
artifact's binding grammar and invariants (ID grammars, lifecycle enums, row forms, write gates)
— has no natural six-set home; `output` absorbs it only by stretching "report contracts" past
recognition. A family section set swapping `verdict` for an **`artifact`** section (the produced
artifact's binding grammar, invariants, and write mechanics) covers the corpus 8/8 with no
stretch. Ruling needed at the wave gate (J-1).

### AAS — authoring-architecture-store (29 rules; source: body)

| # | Obligation (trimmed) | class | kind | disp. |
|---|---|---|---|---|
| 1 | "One home. One writer." — store at `.mochiko/product/architecture/`; this skill the single writer | floor | constraint | move |
| 2a–c | three routing carve-outs: diagram craft → `patterns-system-design` · stance → `patterns-architecture-shelves` · mint routing → `/mochiko:feature` growth door ("the architecture lens proposes; the map machinery disposes") | must | routing ×3 | move |
| 3 | store layout binding (4 files; root index at repo root, never inside the store dir) | must | binding | move |
| 4 | `Scope:` line = shelf scope's durable home; "the shelf walk reads it rather than re-asking" | must | constraint | move |
| 5a | element grammar: `SPN-XXX` kinds + `AX-XXX` core fields; "the schema constrains the skeleton, never the voice" | must | constraint | move |
| 5b | two-arm schema binding (`mochiko-cli template architecture-store` / raw Read) | must | binding | move → C-A3 |
| 6 | NFR one-home on concern rows; `FR-XXX / SC-XXX → NFR-XXX` chains keep resolving | must | constraint | move |
| 7 | `Work:` holds pointers only | must | constraint | move |
| 8 | present tense, no history; rationale linked never restated | must | constraint | move |
| 9 | five lifecycle statuses; in-flight-class MUST name its FEAT-XXX; status ≠ stance axes | must | constraint | move |
| 10 | "Ruled truth is never edited in place by a delivery run … the sign-off IS the write gate. No sign-off, no store write" | floor | gate | move |
| 11 | index regenerated on **every** store write; disagreement fixed by regenerating, never editing | floor | duty | move |
| 12 | full AX table — "a missing row is an invisible row" | must | constraint | move |
| 13 | health view = index section, no separate artifact; five counts each naming rows | must | binding | move |
| 14 | readability bar — new reader places any component from the index alone | must | bound | move |
| 15 | graduation by real depth; extend beats mint; ledger keeps the row | must | constraint | move |
| 16 | landing diff fires on approved-delta-existed — "Never gate the diff on whether structure was built" (`when:` delta-present) | floor | duty | move |
| 17a | diff both directions (signed-vs-built AND built-vs-signed) | floor | duty | move |
| 17b | diff reports, each difference user-dispositioned — "never silently reconciles" | floor | reservation | move |
| 18 | fold duty on structure-built (`when:`): flip, clear keys, update As-built/Drift, regenerate | must | duty | move |
| 19 | drift probe scoped (touched + retrofit-expensive sample) — "Never all rows every visit" | must | bound | move |
| 20a | `As-built:` checked against actual code, never recollection | floor | duty | move |
| 20b | drift findings take a user disposition at the desk; probe never silently reconciles | floor | reservation | move |
| 21 | orphan rule — every in-flight-class element keys an open feature | must | constraint | move |
| 22a | first-visit bootstrap: read → reconstruct (marked derived) → confirm → archive; "Never fails, never silently invents" (`when:` no-ruled-content) | must | duty | move |
| 22b | confirmation is a real user gate; a pipeline run offers the bootstrap rather than failing | must | reservation | move |
| 23 | judgment writes take the sound-loop review leg — "Do not relabel judgment as mechanical to skip the leg" | floor | binding | move |

Floors 8 · musts 21 · advisory 0. Section fit: scope 3 · inputs 2 (19, 20a as read-duties) ·
output 3 · reserved 4 · independence 1 (23) · **artifact-class misfits 16** (grammar, lifecycle,
write gates, index duties) · verdict 0.

### AC — authoring-constitution (47 rules: 37 body + 10 reference stubs)

Body:

| # | Obligation | class | kind | disp. |
|---|---|---|---|---|
| 1 | "There is no `constitution.md`" — governance lands on the native surfaces | floor | constraint | move |
| 2 | surface set binding (region markers · `.claude/rules/mochiko/*.md` · skill pointers · ledger) | must | binding | move |
| 3 | trace summary emitted as part of the output, "not as an afterthought" | must | duty | move |
| 4 | authored from a **ratified** synthesis; synthesis owns selection, this skill owns formulation | floor | constraint | move |
| 5 | graded by an independent validator (`validation-constitution`, "never co-mounted"); loop + gates lead-owned | must | routing | move → C-A2 |
| 6 | every principle traces (GI trace; the ledger the canonical record) | floor | duty | move |
| 7 | every element realized or flagged — "never silently dropped" | floor | constraint | move |
| 8 | no unsanctioned selection; genuine problems become **flagged proposals**, ruled by the user at the acceptance gate — never silent fixes, never authored vagueness | floor | constraint | move |
| 9 | waivers authored, not skipped (D4 model; legal-mandate never waivable, D4.2) | floor | constraint | move |
| 10a–e | surface routing set: universal → region imperative line (always-on budget scarcest) · scope-bound → `paths` rules file, violation-coverage tested · rules-inject-on-Read caveat → standing new-file read line emitted (`when:` rules-file-in-set) · procedure-shaped → skill pointer, mint only session-minted · every principle → index line + ledger entry, closure validator-checked | must | routing/duty ×5 | move |
| 11 | two modes, one shared core; brownfield requires `codebase-analysis.md`; floor cards authored at the declared depth level's row | must | constraint | move |
| 12 | content-source bindings (catalog deck · ESSENTIAL-FLOOR · COMPLIANCE-MODULES · EMERGENT-CEILING) | must | binding | move (pointers) |
| 13 | two-arm governance-surfaces schema binding | must | binding | move → C-A3 |
| 14 | mode prerequisite — missing synthesis: "say so and stop" (`when:` synthesis-absent) | floor | gate | move |
| 15a | D8 ownership: regenerate only between the markers; user content untouchable | floor | constraint | move |
| 15b | preserved blocks: domain-registry + output-style switch line — read existing values, re-emit unchanged | floor | constraint | move |
| 15c | amend preserves untouched principles verbatim, bumps region semver; on-disk `constitution.md` is superseded — lead deletes, never author into it | must | constraint | move |
| 16a–c | When-NOT routing ×3 (grading → validator · eliciting → interrogation · analyzing → `analysis-codebase`) | must | routing ×3 | move |
| 17 | Three-Part Rule — every principle carries Enforcement/Testability/Rationale in its ledger entry; without all three it is incomplete | floor | constraint | move |
| 18 | enforcement MUST fit the team reality recorded in the synthesis | must | constraint | move |
| 19 | surface carries the operative constraint only; ledger the full record; shapes never restated | must | constraint | move |
| 20a–d | mandatory content inventory as four set-rules: region content set (stamp · index · universal principles floor-first, `(NON-NEGOTIABLE)`, depth-row-selected) · quality gates with **actual commands, never placeholders** · the unconditional output-style rules file every run · ledger riders + trace summary manifest | must | duty ×4 | move |
| 21 | all four Essential Floor categories accounted — principle or recorded waiver; neither = defect | floor | constraint | move |
| 22 | no CLAUDE.md-synchronization section exists | must | constraint | move |
| 23 | "Never route module content the synthesis didn't select" | floor | constraint | move |
| 24a | brownfield ceiling: codify only intentionally-good patterns ("Would I recommend this for a new project?"); unsanctioned ceiling = flagged proposal | must | constraint | move |
| 24b | `evolution-notes` attaches always in brownfield | must | constraint | move |
| 25 | roadmap stub — reference `evolution-roadmap.md` as a documented stub | advisory | constraint | move |

Reference stubs (obligation-bearing lines only; teaching content untouched):

| # | Reference obligation | class | kind | disp. |
|---|---|---|---|---|
| 26 | ESSENTIAL-FLOOR.md canonical-home header ("Edit the four categories here, nowhere else") + the four category MUST-address check sets | must | duty | stub |
| 27 | ESSENTIAL-FLOOR.md two-row dial + breadth invariant (dedup vs body 21 noted) | must | constraint | stub |
| 28 | catalog/universal-floor.md floor doctrine: dial user-declared, one-way `low`→`high`, no watcher; D5 retrofit-cost cut line | must | constraint | stub |
| 29 | catalog/universal-floor.md four FLOOR cards' two-row content sets + waiver postures (protected v0.65.0 products) | must | duty | stub |
| 30 | catalog/README.md three-source rule (floor-asserted / deck-kept / minted-traces-to-elicited-intent) + every-ruling-recorded; dropped card = recorded ruling | must | constraint | stub |
| 31 | catalog/backend-service.md arbitrated-layer frame — dealt recommend-then-arbitrate, "never asserted" | must | constraint | stub |
| 32 | COMPLIANCE-MODULES.md mechanical attachment (fact-triggered, "never from appetite"), additive-only, level-blind (D7) | must | constraint | stub |
| 33 | COMPLIANCE-MODULES.md S4 fail-safe (named elicitation · consequence-stated confirmation · brownfield cross-check confronted in the open · temporal backstop) + amend-event MAJOR | must | duty | stub |
| 34 | DOMAIN-DEPENDENCIES.md admissibility gate (both criteria; domain-relevance filters FIRST) + cite-the-trust-level + pointers-re-verified-never-snapshots | must | constraint | stub |
| 35 | DOMAIN-DEPENDENCIES.md growth gate — explicit ruling BEFORE registry entry; "the cycle checkpoint MUST NOT auto-approve while `domain_deps_added` is non-empty"; registry only in the marked block, preserved verbatim | must | gate | stub |

INTERROGATION-AGENDA.md, RFC-2119-KEYWORDS.md, EMERGENT-CEILING-PATTERNS.md: no stubs — see
J-2 (agenda) and prose/teaching dispositions.

Floors 12 · musts 34 · advisory 1. Section fit: independence 1 · scope 5 · inputs 3 ·
output 6 · reserved 4 · **artifact-class misfits 18** · verdict 0. Stubs distribute the same way.

### AE — authoring-epic (27 rules)

| # | Obligation | class | kind | disp. |
|---|---|---|---|---|
| 1 | single-sources the epic shape; consuming commands "reference this skill; they never restate it" | must | constraint | move |
| 2 | identity: `EPIC-XXX`, sequential, same family as `FEAT-XXX` | must | constraint | move |
| 3 | one epic type, two faces — manifest product view, spine tech view | must | constraint | move |
| 4 | transient role; "the directory persists as readable record — never as a living map layer"; the map stays two-typed | floor | constraint | move |
| 5a–e | When-NOT routing ×5 (grading → cluster reviewers, never self · marker/seam grammar → AFM · run mechanics → `implement.md` · transport → `patterns-transport-floor` · delta-batching parked) | must | routing ×5 | move (5a → C-A2) |
| 6 | home `.mochiko/epics/EPIC-XXX/`; no separate index — "the directory is the registry" | must | binding | move |
| 7 | `manifest.md` required fields (members linked · status enum · why-together line) | must | binding | move |
| 8 | an epic run **always fires the design phase** for the joint spine, whatever the sufficiency verdict said | floor | constraint | move |
| 9 | **one signed store delta** for the whole epic — rendered once, signed once | floor | constraint | move |
| 10 | each cross-member seam names its owner explicitly (no later-lander default) | must | constraint | move |
| 11 | ordering: shared-foundation first, then in-epic dependency order | must | constraint | move |
| 12 | per-feature design deltas stay in member dirs, linked from the manifest | must | constraint | move |
| 13 | shared baseline (2+ members) = one joint spine delta under a **single pen-holder**; the landing folds each baseline **exactly once** | floor | constraint | move |
| 14 | transport floor governs every epic shared-write surface, disclosed at run open | must | binding | move |
| 15 | mint-once; every workflow resolves `EPIC-XXX` by lookup; "re-minting does not exist" | floor | constraint | move |
| 16 | pending rows belong to at most one open epic at a time | floor | constraint | move |
| 17a | the desk is the only mint door | floor | constraint | move |
| 17b | membership overlap "surfaces to the user … never a silent duplicate" | floor | reservation | move |
| 18 | specify **proposes**, never mints | must | constraint | move |
| 19 | an implement run resolves by lookup and mints nothing | must | constraint | move |
| 20 | selection-scope only; delta-scope cards cannot join an epic | floor | constraint | move |
| 21 | close = each member's graduation batch + epic close (markers vanish · manifest stamped delivered+dated · directory stays) | must | duty | move |
| 22 | member-scoped halt disposition (carve-out / hold) **reserved to the user**, never the lead's | floor | reservation | move |

Floors 10 · musts 17. Section fit: scope 6 · inputs 0 · output 2 · reserved 3 ·
independence 1 · **artifact-class misfits 15** · verdict 0.

### AFM — authoring-feature-map (39 rules)

| # | Obligation | class | kind | disp. |
|---|---|---|---|---|
| 1 | "Violating the letter of the rules is violating the spirit of the rules." | floor | constraint | move → C-A1 |
| 2a | one surface, two row types; capability = the only thing called a feature, durable | floor | constraint | move |
| 2b | work rows transient — fold into extent at landing and vanish | floor | constraint | move |
| 2c | stories inform capabilities, never define them | must | constraint | move |
| 3 | derivation discipline lives in `mochiko:patterns-map-minimalism` — this skill authors what it rules in | must | binding | move |
| 4 | frame-first: nouns-and-verbs hypothesis, never a story list; "the story wins and the frame adjusts" | must | constraint | move |
| 5a–b | one living map, no per-spec copy; two-arm `features-index` + `feature-entry` schema bindings | must | binding ×2 | move → C-A3 |
| 6 | four touchpoints incl. the zero-gap branch (card-authoring seat performs the dependency/extent assertion) | must | constraint | move |
| 7 | map-side vs product-baseline altitude — implement's surface, untouched here | must | constraint | move |
| 8 | density per the deliverable envelope — "a record, not an essay" | must | binding | move → C-A4 |
| 9a–f | When-NOT routing ×6 (grading → RSPEC independent · discipline → map-minimalism · stories → AUS · architecture → store link, never restate · **selection is the user's ruling** — card prepared, recommendation only · defects/tooling → BACKLOG where KM exists, extent-growth exception) | must (9e floor) | routing ×5 + reservation | move (9a → C-A2) |
| 10 | invariant 1: exactly one home; extend obligations, never a second home | floor | constraint | move |
| 11 | invariant 2: complete disposition — homed or rejected with the why; no silent drops, no orphans | floor | constraint | move |
| 12 | invariant 3: dependency closure at row level; no forward dependencies | floor | constraint | move |
| 13 | invariant 4: the map owns status, one home no copies; rows carry state, not status | floor | constraint | move |
| 14a | invariant 5: delivered sticky; live rows fold and vanish; pending persists as open obligation; status never regresses | floor | constraint | move |
| 14b | `retired` terminal — entry kept, dated, never deleted; `[EPIC-XXX]` marker vanishes with the fold | floor | constraint | move |
| 15a | invariant 6: capability writes sacred (specify or user grooming ruling only); row-cutting is desk bookkeeping; stewardship direct | floor | constraint | move |
| 15b | delivery writes = one atomic batch at the acceptance landing; a rejected spec never touched the map; workspace staging | floor | constraint | move |
| 16 | invariant 7: map integrity fix-on-sight (dangling IDs · index/entry agreement · named runs · live-until-landing · the defect classes) | floor | duty | move |
| 17 | invariant 8: entries index, never rewrite — IDs cited, spec text single-sourced | floor | constraint | move |
| 18 | growth on a delivered capability rides a marked work row, never an in-place edit | must | constraint | move |
| 19 | in-flight territory obligates a read into the owning run's artifacts; contradiction escalated, never silent | must | duty | move |
| 20 | pre-acceptance derivation stays in the workspace ("unratified thought") | must | constraint | move |
| 21 | a phase row must stand alone as a working increment, never a horizontal layer | must | constraint | move |
| 22a | stubs are unratified hypotheses — never derive from stub text; a match is confirmation | must | constraint | move |
| 22b | selectability is specify-derivation-only; the desk parks and grooms, never matures | floor | constraint | move |
| 23a | a stub dependency is flagged-unverified judgment, never an asserted relation | must | constraint | move |
| 23b | dependency escalation is a recommendation for the user — never a forced cut | floor | reservation | move |
| 24 | first touch of a reconstructed entry re-verifies against the code | must | duty | move |
| 25 | every SC-XXX mapped to a verifying capability; deferred SCs + one-sided seams on Obligations lines; epic seam owner = the spine's assignment | must | constraint | move |
| 26 | acceptance batch includes the specs-index row, agreeing with the map | must | constraint | move |
| 27 | selection card contents (recommendation · deferred-SC list · per-capability completeness line · rows grouped · ordering) + parked stubs/pending rows re-surfaced | must | binding | move |
| 28 | derivation-minted stubs carry story-trace provenance | must | constraint | move |

Floors 16 · musts 23. Section fit: scope 7 · inputs 2 · output 3 · reserved 3 ·
independence 1 · **artifact-class misfits 21** · verdict 0.

### AP — authoring-prototype (20 rules)

| # | Obligation | class | kind | disp. |
|---|---|---|---|---|
| 1 | letter/spirit epigraph | floor | constraint | move → C-A1 |
| 2a | two coupled artifacts: the app + the Screens & Flows manifest; two-arm `spec` schema binding | must | binding | move → C-A3 |
| 2b | IDs per the deliverable envelope — sequential, three-digit padded, cited never re-quoted | must | binding | move → C-A4 |
| 3 | static HTML; serves with bun AND opens `file://` with no server; "No build step, no framework, no install *required* to view" | floor | constraint | move |
| 4 | authority split — manifest binding (screens, data, actions); pixels advisory | floor | constraint | move |
| 5a–d | When-NOT routing ×4 (no UX surface → waiver line, "Never manufacture screens" · stories → AUS · grading → RSPEC independent · production UI — "throwaway by design and never migrates") | must | routing ×4 | move (5c → C-A2) |
| 6 | invariant 1: skeleton first — nav frame before any story screen | must | constraint | move |
| 7 | invariant 2: lockstep, story by story — "never batched after all stories are drafted" | must | constraint | move |
| 8 | invariant 3: every `FLOW-XXX` keys to a named acceptance scenario; unkeyed flow = scope invention; P1 scenario with no flow = manifest gap | must | constraint | move |
| 9 | invariant 4: manifest ↔ HTML agreement, both directions, mechanically checkable | must | constraint | move |
| 10 | invariant 5: low-fi discipline; placeholder data with honest shape ("five rows, not one") | must | constraint | move |
| 11 | invariant 6: design system honored where one exists (`when:` design-system-present); "never invent a new visual language" | must | constraint | move |
| 12 | invariant 7: FEAT tags land as a re-tag pass at derivation; out-of-selection screens greyed coming-soon, reachable | must | duty | move |
| 13 | invariant 8: rejected stories' screens stay, greyed, marked, pointed at the recorded rejection — "never silently deleted" | must | constraint | move |
| 14 | structure: one file per screen named by SCR id; plain `<a href>`/`<form action>`, no JS state machine; navigation never dead-ends | must | constraint | move |
| 15 | stories, requirements, criteria untouched — "the prototype renders them, never edits them" | floor | constraint | move |
| 16 | scope invention surfaced as a story finding, never silently rendered | must | constraint | move |

Floors 4 · musts 16. Section fit: scope 5 · inputs 0 · output 2 · reserved 0 ·
independence 1 · **artifact-class misfits 12** · verdict 0.

### AR — authoring-requirements (20 rules; references EDGE-CASES + RFC-2119 = teaching, no stubs)

| # | Obligation | class | kind | disp. |
|---|---|---|---|---|
| 1 | technology-agnostic — WHAT and WHY, "never HOW it's implemented" | floor | constraint | move |
| 2 | "A constraint states a capability, never a posture" (+ the freezes-a-design-time-choice consequence) | floor | constraint | move |
| 3a | deliverable-envelope binding — one-line entries, ≤3-line overview, omit empty sections | must | binding | move → C-A4 |
| 3b | "Density is not a gap; a gap is missing substance" | must | constraint | move → C-A4 tail |
| 4a | PM-frame boundary — PM owns *which*, this craft owns *how well*; neither edits the other's verdicts | floor | constraint | move |
| 4b | a boundary disagreement escalates to the user | floor | reservation | move |
| 5a–f | When-NOT routing ×6 (implementation planning · architecture decisions · already-validated · API endpoints out of FRs · data model conceptual-only · stories → AUS) | must | routing ×6 | move |
| 6a | FR-XXX format with RFC 2119 keywords | must | constraint | move |
| 6b | RFC-2119 reference pointer (detailed usage) | must | binding | move (pointer) |
| 7 | FR numbering sequential, three-digit padded, no gaps | must | constraint | move |
| 8 | 3–5 edge cases from the five categories; reference pointer | must | duty | move (pointer) |
| 9a | 3–5 measurable SCs in SC-XXX form | must | duty | move |
| 9b | SC rules set (technology-agnostic · user/business-focused · measurable · outcome-oriented) | must | constraint | move |
| 10 | entities conceptual only — purpose one line, concepts not columns, no types/constraints/indexes (full model downstream) | must | constraint | move |
| 11 | validation script available (`python scripts/validate-requirements.py …`) | advisory | binding | move (pointer → `scripts/`) |

Floors 4 · musts 15 · advisory 1. Section fit: scope 6 · inputs 0 · output 1 · reserved 1 ·
independence 0 · **artifact-class misfits 12** · verdict 0.

### ATR — authoring-technical-requirements (37 rules: 31 body + 6 reference stubs)

Body:

| # | Obligation | class | kind | disp. |
|---|---|---|---|---|
| 1 | every element traces to a business source; every target measurable; every constraint accounts for its design impact | floor | constraint | move |
| 2 | envelope: the statement carries the content (no Description field); one-line entries; summary tables = the ID index | must | constraint | move → C-A4 |
| 3 | every element answers the design ladder before entering the package; stops disclosed by the design phase, "never re-derived here" | must | binding | move (pointer `patterns-plan-minimalism`) |
| 4 | letter/spirit epigraph | floor | constraint | move → C-A1 |
| 5a–e | When-NOT routing ×5 (business requirements → AR · decision technique → `patterns-technical-decisions`, project-scope decisions excluded · designing solutions · choosing technologies · slicing → `patterns-vertical-tdd`) | must | routing ×5 | move |
| 6 | layer ID prefixes + artifact homes (C/D/IP + thin INT/DS on `constraints-and-decisions.md`; NFR on store rows) | must | constraint | move |
| 7a | declare-here-author-there: `x-integration` boundary → `patterns-api-contracts` | must | routing | move |
| 7b | per-attribute sensitivity → `patterns-entity-modeling` | must | routing | move |
| 8a | "Constraints are facts, not preferences"; each D MUST reference its shaping constraints | must | constraint | move |
| 8b | C impact SHOULD reference the decisions it influences | must | constraint | move |
| 9 | no exceptions — "Not for 'well-known' constraints … Not even when the team has consensus" | floor | constraint | move |
| 10 | decision technique recorded from `patterns-technical-decisions` into the D-XXX slots — field schema owned here only | must | binding | move |
| 11a | structural decisions never become D-XXX rows — topology lives in the store's delta, whose ruling IS the record | floor | constraint | move |
| 11b | entangled case: technology decision here, shape in the delta, cross-cited by ID | must | constraint | move |
| 12 | every platform-implying constraint gets an IP-XXX | must | constraint | move |
| 13a | "There is no `nfrs.md`" — NFR rows homed on store concern rows; ids/grammar this skill's, row shape the store's | floor | constraint | move |
| 13b | a new/changed target reaches the store as a drafted delta, written at the user's sign-off | floor | reservation | move |
| 14 | "'Fast' is not a requirement" — every NFR gets a number, a measurement method, and a source; no deferrals | floor | constraint | move |
| 15 | INT-XXX thin declaration; every declaration MUST carry through to failure modes + fallback at its downstream boundary | must | constraint | move |
| 16 | DS-XXX thin declaration; sensitive data present must be governed, classified downstream | must | constraint | move |
| 17a–d | mandatory trace links ×4 (NFR → FR/SC · C ↔ D · C → impact · C/NFR → IP) | must | constraint ×4 | move |
| 18 | technology-agnostic writing + the real-infrastructure-fact exception | must | constraint | move |
| 19 | Red-Flags STOP — no exceptions for "simple" systems, "well-understood" domains, "tight timelines" | floor | constraint | move |
| 20 | ID sequences sequential, no gaps, per prefix | must | constraint | move |
| 21 | every INT/DS row present with its downstream home named | must | constraint | move |

Reference stubs (`ARTIFACT-TEMPLATES.md` 14,052 B · `TRACEABILITY-PATTERNS.md` 11,772 B —
tables and templates stay untouched):

| # | Reference obligation | class | kind | disp. |
|---|---|---|---|---|
| 22 | field-definition sets (C/D/IP/Declarations/NFR required fields + the T2 statement-line form) | must | duty | stub |
| 23 | register rule: every ID, numeric target, constraint clause a never-compress item | must | constraint | stub |
| 24 | declaration-closure rule — "a declaration with no named home is incomplete" + the constraint-vs-preference test | must | constraint | stub |
| 25 | per-link validation procedures (C-sourcing · D→C · NFR measurability · IP coverage · INT/DS closure) | must | duty | stub |
| 26 | self-checks-are-not-the-gate — cross-artifact consistency *grade* owned by `review-plan-artifacts`, "not self-asserted here" | must | routing | stub |
| 27 | layer-internal consistency rules ×3 (IDs resolve · bidirectional match · no contradictory constraints) | must | constraint | stub |

Floors 8 · musts 28 · advisory 0 (with stubs). Section fit: scope 8 · inputs 0 · output 2 ·
reserved 2 · independence 1 (26) · **artifact-class misfits 18** · verdict 0.

### AUS — authoring-user-stories (18 rules; references = teaching, no stubs)

| # | Obligation | class | kind | disp. |
|---|---|---|---|---|
| 1 | letter/spirit epigraph | floor | constraint | move → C-A1 |
| 2 | each story independently testable with measurable acceptance criteria | must | constraint | move |
| 3a | PM-frame boundary (verbatim twin of AR 4a) | floor | constraint | move |
| 3b | disagreement escalates to the user | floor | reservation | move |
| 4a–e | When-NOT routing ×5 (technical tasks · bug reports · already-story-form · architecture decisions → design track · API contracts → design track) | must | routing ×5 | move |
| 5a | exact story structure, 2–5 stories per feature | must | binding | move |
| 5b | density fields: journey ≤ 2 lines; justification + independent test one line each; **each scenario a single line** (the T1 form) | must | constraint | move |
| 6 | P1/P2/P3 priority with one-line justification; reference pointer | must | constraint | move (pointer) |
| 7a | 2–3 scenarios — happy path + key edge; "more than 3 means the story is compound" | must | bound | move |
| 7b | scenarios independently verifiable | must | constraint | move |
| 7c | concrete observable outcomes, not implementation details | must | constraint | move |
| 8 | Independent Test required, naming isolation method, data/setup, pass/fail | must | constraint | move |
| 9 | validation script available (`python scripts/validate-user-stories.py …`) | advisory | binding | move (pointer → `scripts/`) |
| 10 | STOP rule — no exceptions, "nor even if the user says 'just give me quick stories'" | floor | constraint | move |

Floors 4 · musts 13 · advisory 1. Section fit: scope 5 · inputs 0 · output 0 · reserved 1 ·
independence 0 · **artifact-class misfits 12** · verdict 0.

**Census totals:** ≈ **237 rules** at D12 grain — **221 body-borne (moves-to-schema) · 16
reference-borne (stubs: AC 10, ATR 6) · 0 lifts** (every reference obligation sits inside
surrounding procedure or field-definition structure; stub-default holds family-wide). Class mix:
**66 floor · 168 must · 3 advisory**. Projections at census grain, ±10% at conversion (the
wave-1 precedent: a/b splits and set-rule splits moved individual members by 1–3).

---

## C. Common-block candidates (R1–R6 screen, WITHIN the authoring family only)

Manual screen (no schemas exist yet; the detector cannot run pre-conversion). Bar: 3+ members,
near-identical wording, strongest-wording-wins (R2), member-specific extras keep local text with
allowlist edges (R6). **No cross-grammar sharing** (D5) and **no cross-family sharing with
`skill-review-common.yaml`** (D5 — per-family block file; a cross-family global is a graduation
candidate only). The family common file would be a new shipped primitive —
`plugins/mochiko/schemas/skill-authoring-common.yaml`.

| # | Candidate block | Members | Wording evidence | Verdict |
|---|---|---|---|---|
| C-A1 | **letter-IS-the-spirit epigraph** — "Violating the letter of the rules is violating the spirit of the rules." | AFM · AP · AUS · ATR (4) | byte-identical epigraph line in all four bodies | **CLEARS**. Cross-family note: the review side's RPA/VC "the letter IS the spirit: never skip a check…" is a different wording and was screened keep-distinct in wave 1 — evidence for the D5 cross-family graduation candidate, not a shared block; allowlist edge recorded at build |
| C-A2 | **produced-artifact-graded-independently** — "graded … by an independent reviewer/validator, never the author"; this skill never grades its own output | AFM · AP · AC · AE (4) | AFM "graded with the spec by `mochiko:review-specifications` (independent reviewer, never the author)" · AP near-identical · AC "graded by an **independent validator** … never co-mounted" · AE "graded by their cluster reviewers; … never grades its own output" | **CLEARS**. `${grader}`-style local tails per R2/R6 (grader name + graded object vary). Edge: AAS expresses the obligation as the sound-loop review leg (rule 23) — related but distinct (review-leg-before-user-ruling vs never-self-grade); kept LOCAL, allowlist edge. Cross-family note: this is the **producer mirror** of `review-common.author-grader` — evidence only (D5), drift edge to the allowlist |
| C-A3 | **two-arm template-schema binding** — "invoke `mochiko-cli template ${template}` when the binary is available; otherwise Read `plugins/mochiko/schemas/${template}.yaml` raw" | AAS · AC · AFM (×2 templates) · AP (4; +ATR's copy reference-side) | near-verbatim GI-020 two-arm form at every site; only the template name varies — the cleanest `${var}` case in either wave | **CLEARS**. `vars: template:` per binding skill; AFM binds it twice (two templates) — stub shape for a twice-bound block is a conversion detail flagged to the build seat; ATR's copy sits in ARTIFACT-TEMPLATES.md and rides its stub (like RF's C5 membership in wave 1) |
| C-A4 | **deliverable-envelope binding** — the artifact "follows the deliverable envelope in `templates/artifact-format.md`" (+ density/ID tails) | AFM · AP · AR · ATR · AUS (5) | all five bind `artifact-format.md` by path; tails vary: AR/ATR carry the verbatim "Density is not a gap; a gap is missing substance", AP the ID-grammar tail, AFM the record-not-essay tail | **CLEARS** — the block carries the envelope binding + pointer only; every density/ID tail stays local (R2/R6). Sub-evidence: "sequential, three-digit padded, no gaps" recurs in AR body · ATR body+reference · AP via envelope — single-sourced in `artifact-format.md`, so it rides this block's pointer rather than minting an `id-grammar` block that would dual-home the envelope's own text (D6 anti-dual-homing) |

**Screened and kept distinct (below bar or R5/R6 fail), recorded for the allowlist:**
- *PM-frame boundary + escalate-to-user* — AR + AUS verbatim-identical (2).
- *reserved-to-the-user rulings* — AAS sign-off/dispositions · AE carve-out/overlap · AFM
  selection · AC flagged-proposals/waivers: same reservation shape, four materially different
  wordings and reserved objects — the `reserved` section's population, not a block (R1 fails).
- *reference-never-restate* — AE ×3 · AAS · ATR: theme-level convergence, wordings vary (R1
  fails).
- *extend-beats-mint* — AAS (concern-row graduation) + AFM (via `patterns-map-minimalism`,
  the discipline's single source) (2, different objects).
- *design-track carve-out pair* — AR + AUS (2, same v0.91.0 lineage).
- *no-exceptions STOP* — ATR + AUS near-identical framing, AR none (2).

**Cross-grammar/cross-family note:** C-A1/C-A2 shapes echo command `common.yaml` and
`skill-review-common.yaml` blocks. D5 forbids both sharings; all such edges land in
`scripts/similar-rules-allowlist.yaml` at build. Two families now independently demanding an
author≠grader-shaped block is the first real data point for the D5 cross-family
`skill-common.yaml` graduation candidate — noted for the user, not actioned.

---

## D. Numeric abort check

**4 candidate blocks clear the 3+ near-identical bar** (C-A1 ×4 · C-A2 ×4 · C-A3 ×4(+1 ref) ·
C-A4 ×5). The D9-I3 abort threshold is "fewer than three" — **not tripped; the wave may proceed
to the user gate**. Stated honestly: 4 blocks is thinner than the review family's 6, and the
blocks are lighter (bindings and epigraphs rather than verdict machinery) — the drift-control
purchase here is real but smaller; the family's stronger conversion drivers are the B/C
secondary concerns (addressable IDs for ~48 protected units, checker coverage over 66 floors).
Per-member verdict: all 8 convertible; no member is obligation-starved (min 18 rules).
**Family verdict: proceed, conditional on the J-1 section-set ruling landing first.**

`kind: fail` re-check (D9/M2 symmetry): zero run-fail predicates in the corpus; the v0.100.0
retirement holds, no re-admission demanded. No new `kind:` demanded either — the eight-kind set
covers all 237 rows (`gate` ×5 · `reservation` ×10 · `bound` ×4 · `routing` ~45 · `binding` ~25 ·
`duty` ~20 · `latitude` 0 · rest `constraint`). `latitude` has no authoring carrier — legal
(kinds are grammar-wide, not per-family).

---

## F. Read-cost projection (D8/C1 · D9/I5)

Current figures: measured this tree (canonical-snippet counts). Estimates: coarse (±25%),
method — per the brief, the wave-1 **measured ×3.24** structural factor applies to the
**extracted-obligation footprint**, not the whole body: est. delivered-at-invoke =
(1 − f)·body (teaching/procedure prose stays) + 3.24·f·body (the obligation footprint's
post-conversion body-tail + schema cost at the measured wave-1 rate) + ~450 load-first block +
~1,400 family common file (est. 4 blocks × ~230 + header; read in the same first action —
all 8 members bind ≥1 stub). f = estimated obligation fraction of the body, from the §B rule
density. **Budgeted payload** (the C1 quantity, re-seeded at conversion, no +25% headroom) =
delivered − common.

| Member | Body now | Desc | Refs (exempt) | Rules | f | Est. budgeted payload | Est. delivered/invoke | vs body now |
|---|---|---|---|---|---|---|---|---|
| AAS | 10,884 | 492 | — | 29 | .55 | ~24,750 | ~26,150 | ×2.4 |
| AC | 17,886 | 481 | 67,975 (9 files) | 47 | .45 | ~36,350 | ~37,750 | ×2.1 |
| AE | 7,821 | 496 | — | 27 | .60 | ~18,800 | ~20,200 | ×2.6 |
| AFM | 15,975 | 598 | — | 39 | .55 | ~36,100 | ~37,500 | ×2.3 |
| AP | 9,066 | 493 | — | 20 | .50 | ~19,650 | ~21,050 | ×2.3 |
| AR | 4,413 | 379 | 11,083 + script | 20 | .50 | ~9,800 | ~11,200 | ×2.5 |
| ATR | 10,500 | 598 | 25,824 | 37 | .55 | ~23,900 | ~25,300 | ×2.4 |
| AUS | 5,351 | 425 | 7,813 + script | 18 | .50 | ~11,800 | ~13,200 | ×2.5 |
| **Family** | **81,896** | | ~112,700 | **237** | | **~181,150** | **~192,350** | **×2.35** |

Read plainly: **the conversion roughly doubles-and-a-third each member's per-invoke delivered
payload** (est. family ×2.35). Honesty rider: the wave-1 census estimated ×2.3 and the build
**measured ×3.24** (+41% over estimate, all validator-verified structural overhead). If the same
estimate-to-measured drift recurs, the authoring family lands nearer **×2.9–×3.3
(~240k–270k family delivered)**. The user should accept the *band*, not the point estimate; the
measured figure returns at the landing gate per the wave-1 precedent, and the read-cost
observable joins the first-live-run watch (I5).

No member carries an RF-style obligated reference read — no J-3 analogue; the `references/` mass
(largest: AC's 68k) stays exempt and on-demand.

Budget mechanics per C1: all six budgeted members re-seed to measured post-conversion payloads
(current budgets: AC 21,550 · AFM 15,413 · AP 11,123 · AR 5,127 · ATR 13,285 · AUS 6,702 — every
one lands above its cap, the re-seed path's designed case). **AFM's standing ruled overage
(+562, HOLDS at v0.91.0) dissolves into its re-seed** — the ruling trail survives in the strip
(J-6). **AAS and AE are unbudgeted at birth** (hard-cap-only); the conversion re-seed is a legal
first seeding (the ledger's third seeding path — "a ruled schema conversion") — flagged so the
ledger edit names them (J-7). The family common file is budgeted once as its own primitive.
Descriptions untouched, all ≤ 1,536 (max 598: AFM, ATR).

---

## I. Labels — `skill-labels.yaml` fit

Transfers used as-is (meaning unchanged): **binding** (heaviest — schema/two-arm/file-home
rules) · **boundary** (the When-NOT routing mass) · **user-gate** (sign-off gates, selection,
carve-out, waivers, flagged proposals) · **independence** (C-A2 cluster, sound-loop leg) ·
**floor-pointer** (pointers at `patterns-sound-loop` · `patterns-transport-floor` ·
`patterns-map-minimalism` · `patterns-plan-minimalism` · `patterns-adopt-first`-adjacent) ·
**evidence** (drift-vs-code, provenance, re-verify duties) · **reporting** (trace summary,
selection card, landing-diff reports) · **fence** (read duties: AFM in-flight-territory read,
AAS code-read, AC codebase-analysis input).

Registry-legal but unused by this family: **verdict** (no clearing grammar exists — see J-1).

New, corpus-demanded (registry-edit-first ceremony; one line each):

- **artifact-grammar** — the produced artifact's binding shape: ID grammars, lifecycle enums,
  row/field forms, required sections. (Carriers in every member; the family's densest class.)
- **single-home** — a one-home/one-writer/no-copies rule: where a thing lives once and every
  other surface links or derives. (AAS one-writer index · AFM map-owns-status/one-home ·
  ATR no-`nfrs.md` · AC ledger-canonical · AE directory-is-registry.)

Considered, rejected: `write-gate` (covered by `user-gate` + `kind: gate`) · `staging`/`landing`
(command-run vocabulary; the acceptance-batch rules carry `artifact-grammar` + `user-gate`) ·
`lifecycle` (inside `artifact-grammar`).

---

## J. Anomalies (numbered, each with a recommended disposition)

- **J-1 — the six-set does not fit a producer family (the census's headline).** `verdict` is
  empty for **all 8 members**, while ~124 of 237 rules (the artifact-grammar/write-mechanics
  class) have no natural six-set home. The skill-pair criteria block
  (`.claude/rules/mochiko/primitive-edits.md`, criterion 2) currently hard-codes "the six-set …
  all six present in every schema" — written for the pilot, binding on "every converted skill".
  *Recommendation:* the authoring rollout ruling mints the family section set —
  `independence` · `scope` · `inputs` · **`artifact`** (the produced artifact's binding grammar,
  invariants, and write mechanics) · `output` · `reserved` — and amends criterion 2 to name
  per-family section sets (each minted by that family's census, uniform within the family, empty
  markers still legal). The alternative — eight vacuous `verdict` empty-markers and an `output`
  section carrying triple its intent — satisfies the letter of the criteria block while
  defeating the D4 rationale ("minted once from the pilot census"; the pilot's set encoded the
  *grader* lifecycle). The criteria-block edit is itself a ceremonied shipped-rule edit riding
  the wave. **This ruling must land before any conversion begins.**
- **J-2 — INTERROGATION-AGENDA.md is an obligation-dense reference with no body binding.**
  Its obligations (agenda test, ten dimensions, no-pruning license, D6 flip ceremony,
  steps 0–5) bind the **setup lead's interrogation session**, not this skill's authoring
  procedure; AC's `SKILL.md` never binds it, and its consumers are cross-primitive
  (`/mochiko:setup`, and `review-governance-intent` binds it cross-directory — wave-1 J-7).
  A stub would dangle from no §B row. *Recommendation:* no stub; the file stays untouched under
  the sanctioned Single-source convention (D3/C2), its cross-directory consumers already
  schema-bound from their own sides. Named at the gate so the omission is a ruling, not a miss.
- **J-3 — two members ship `scripts/`** (`validate-requirements.py`,
  `validate-user-stories.py`) — no wave-1 precedent (no review member shipped scripts). Scripts
  stay budget-exempt; the citing rules convert as `advisory` bindings with `pointer:` into
  `scripts/`. *Recommendation:* confirm the checker's pointer-resolution accepts `scripts/`
  paths (it already accepts `references/` and cross-directory climbs); no grammar change.
- **J-4 — RFC-2119-KEYWORDS.md exists twice** (authoring-constitution 6,181 B,
  constitution-flavored; authoring-requirements 5,535 B, spec-flavored) — near-dup *files*, not
  rules; both teaching content, both stay prose. *Recommendation:* evidence note only — a
  Single-source consolidation is compression-wave territory, out of this conversion's remit;
  recorded so the detector's future file-level cousin (if any) finds it ruled.
- **J-5 — description-borne obligations.** AE/AFM/AP descriptions carry `Boundary:`/"Never
  grades its own output" clauses duplicating body rules that now gain schema IDs.
  `description:` is untouched (D3/D8 criterion 7), so this is a sanctioned dual statement.
  *Recommendation:* none — same posture as wave-1's RSUF row 29 (no rule minted for the
  description copy); noted so the audits don't read it as dual-homing.
- **J-6 — AFM's standing ruled overage dissolves at re-seed.** The +562 HOLDS ruling
  (mechanic-e zero-gap branch, byte-reconciled at v0.91.0) is protection-relevant history.
  *Recommendation:* the AFM conversion strip entry names the overage ruling explicitly so its
  trail survives the budget row's supersession (GI-006 reconstruction).
- **J-7 — two unbudgeted members gain their first budgets via conversion.** AAS and AE are
  hard-cap-only ("budgets never invented"); the third seeding path ("a ruled schema
  conversion") legally seeds them at the measured payload, no headroom. *Recommendation:* the
  ledger amendment names both as first-seeds so the "never invented" clause visibly composes
  with the third path.
- **J-8 — cross-family and cross-grammar near-dups.** C-A1 vs RPA/VC's letter-IS-the-spirit
  wording · C-A2 vs `review-common.author-grader` · AR/ATR "density is not a gap" vs RSPEC
  rule 25 / RPA's envelope line · AAS/AE/AFM reserved-to-user shapes vs
  `review-common.verdict-is-input`'s reservation posture. All evidence-only under D5;
  allowlist edges at build. Two families independently converging on author≠grader and
  envelope-density blocks is the first live signal for the D5 cross-family `skill-common.yaml`
  graduation candidate — surfaced for the user, no action proposed.
- **J-9 — the brief's reference-file count for AC was 7; the tree holds 9** (6 root + 3 under
  `references/catalog/`). All 9 censused individually in §B (stubs from 6 of them; 3 stay
  untouched — INTERROGATION-AGENDA per J-2, RFC-2119 and EMERGENT-CEILING as teaching).
  *Recommendation:* none — corrected count recorded here.

---

*End of census. Per D9-I3 this inventory returns to the user at the wave gate — with the §D
non-abort result (4 blocks, thinner than the pilot's 6, stated honestly), the §F read-cost band
(est. ×2.35, honest band to ×3.3 on the wave-1 drift precedent), the J-1 section-set ruling
(gate-blocking), and the label seed — before any conversion begins.*

---

## K. Build-corrections appendix (wave 2A landing, 2026-09-01, v0.101.0)

Corrections and final measures recorded at landing per the wave-1 §K idiom; each was disclosed
in the member's strip map at build or ruled at plan approval.

- **AAS floors 9, not the tally line's 8** — the §B table marks 9 floor rows (1 · 10 · 11 ·
  16 · 17a · 17b · 20a · 20b · 23); the "Floors 8 · musts 21" line is an arithmetic slip.
  Built 9, lead-ruled at plan approval.
- **AC 48 rules, not 47** — the `never-co-mounted` protected residue minted as an adjacent
  local rule under the wave's residue rule (a stub carries the block core only). Artifact
  section final: 20 body + 7 stubs; the census's per-section distribution under-counted
  `artifact`.
- **AFM 41, not the headline 39** — the table grain itself sums to 40; −1 rows 3+9b merged at
  D12 grain, +1 the twice-bound two-arm split (stub + local twin — `vars:` holds one template
  value per schema), +1 the 8a/8b envelope/density residue split. AFM also ships no
  `conditions:` block: the census's KM-exists/epic-context dims resolved text-borne (the RPA
  row-30 precedent), ruled at plan approval.
- **AP 21, not 20** — the 2a coupled-artifacts/two-arm split.
- **ATR 39, not the header's 37** — §B's ATR header says 31 body rows but its table
  enumerates 32; +1 the 2a/2b split; 6 reference stubs. ATR's C-A3 membership is
  reference-side, riding the ARTIFACT-TEMPLATES.md stub (the wave-1 RF/C5 mirror).
- **AUS 19, not 18** — the 5a envelope-stub/story-structure split; the census's ×5 C-A4
  membership won over the build brief's initial mis-assignment.
- **C-A1 carries no label** — §I assigned the epigraph none; the built block briefly carried
  `[artifact-grammar]`, dropped by audit ruling (V3 finding 1), with a narrow probe-covered
  checker carve: block-inherited label absence warns, never fails; a local empty `labels:`
  stays a hard finding.
- **Final measures** — family budgeted payload 140,296 · common file 1,285 · delivered-at-invoke
  **150,576** vs 81,896 pre-conversion = **×1.84, UNDER the §F ×2.35 estimate; the wave-1
  estimate-to-measured drift did not recur.** Per member: AAS 18,876 · AC 30,387 · AE 13,044 ·
  AFM 21,636 · AP 13,943 · AR 10,796 · ATR 19,946 · AUS 11,668. Family totals as built:
  244 rules · 67 floors · 16 reference stubs.
- **V2 sub-grain clause homes confirmed at landing** — map flatness lives in
  `patterns-map-minimalism` (body + checklist); work-row state + acceptance in
  `plugins/mochiko/schemas/feature-entry.yaml`; AP's token duty in its design-system rule +
  the README one-pager line.
- **Standing non-blocking observations for a future touch** — AP's `design_system`
  DECLARE-form leaves the absent pole un-gated (ruled; splitting the never-invent clause into
  an un-gated sibling is the cleanest future fix) · labels-registry citation style varies
  across members (repo-relative vs base-dir-relative; both resolve).
