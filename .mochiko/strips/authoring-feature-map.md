# Strip notes — `skills/authoring-feature-map/SKILL.md`

Entry formats: `strips/README.md`. Wave context: the feature-sizing & entry-points build wave
(record: `.mochiko/brainstorms/feature-sizing-and-entry-points/record.md`; `DECISIONS.md` row
2026-08-10 "Feature sizing & entry points ruled (D1–D15 as amended at review)"). The skill gains
nesting (parent/leaf, two-level cap, sticky-delivered roll-up), parent minting three ways,
`unrefined` capability stubs, and the lane-run vocabulary; the entries below record every line
the same wave superseded.

---

## [v0.62.0] Capability-stub minting no longer exclusive to /mochiko:feature — derivation may mint stubs
- **Disposition:** superseded → the rewritten "Capability stubs — parking, never a bypass" section: two seats mint `unrefined` stubs (`/mochiko:feature` idea-parking AND specify's derivation parking uncertain remainder with story-trace provenance); selectability/maturation stays specify-derivation-only, `/mochiko:feature` stewards but never matures or dispatches unratified scope; the When-to-Use stub line harmonized in the same touch
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-10 pm-requirements-stacking; record D2 as corrected at review, D2a, F-3 sustained)
- **Content:** "`/mochiko:feature` can add `proposed` entries only as **capability stubs**: name + one-breath hook, marked `unrefined`. Only specify's derivation fills extent and relations and makes an entry selectable for delivery." · When-to-Use line "Minting or grooming `unrefined` capability stubs (`/mochiko:feature` stewardship)"
- **Kept deliberately:** the anti-spec-bypass intent survives in force — the loosening extends *minting* only, never the selectability gate ("selectability stays behind specify's derivation"); "parking, never a spec-bypass", the `unrefined`-mark auditability, and the "unratified hypotheses, never extension anchors" / ignore-stub-text / match-is-confirmation rules survive re-worded
- **Consumers assessed:** `/mochiko:feature` (feature.md) remains the stewardship stub-minter — no exclusivity it relied on; specify.md binds the craft by reference (no command edit needed for minting); its selection card gained the ledger line in the same wave; `mochiko:review-specifications` grades derivation output including derivation-minted stubs

## [v0.61.0] Description frontmatter re-fit for nesting, stubs, and feature-command triggers
- **Disposition:** superseded → the rewritten `description:` (adds nesting/stub/promotion trigger phrases, the `/mochiko:feature` stewardship touchpoint, the D13-scoped boundary line; measured 1,485 chars / 1,495 bytes, under the 1,536 delivery truncation)
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-10 "Feature sizing & entry points ruled (D1–D15)"; record D2–D4, D6, D12, D13)
- **Content:** prior description named only the specify-run triggers ("during a specify run: the intent-stage map-read agenda … staging the map write that lands at spec acceptance"), carried no nesting/stub/promotion trigger phrases, and closed the boundary with "Product capabilities live on the map; defects, tooling, and process items stay in BACKLOG.md."
- **Kept deliberately:** the MUST/SHOULD grading, the stories-first framing, the three NOT-boundaries (stories · architecture · selection), and the author≠grader closing sentence survive re-worded.
- **Consumers assessed:** description is the model-invocation surface only; no primitive quotes it.

## [v0.61.0] Three-touchpoints sentence superseded by four (feature command added)
- **Disposition:** superseded → the four-touchpoint sentence: specify proposes · plan confirms · implement's landing graduates · `/mochiko:feature` stewards (stubs, promotion, retire, grooming, lane intake)
- **Tier failed:** n/a — supersession by ruling (record D5/D6 — the feature-management command; lead ruling G4 on stewardship writes)
- **Content:** "The map has three touchpoints: **specify proposes** (this skill's core work), **plan confirms and hardens** alongside architecture, and **implement's acceptance landing graduates** — status flips, delta folds, and index touches are bookkeeping edits inside that landing, never a separate close stage."
- **Kept deliberately:** all three original touchpoints and the never-a-separate-close-stage clause survive verbatim inside the extended sentence.
- **Consumers assessed:** specify/plan/implement re-keyed in parallel seats this wave; the new feature command binds this skill by reference.

## [v0.61.0] Vocabulary table: single Feature row superseded by parent/leaf rows
- **Disposition:** superseded → two rows — **Feature — parent** (capability, roll-up, never built directly) and **Feature — leaf** (deliverable, the pipeline unit; a flat entry is a leaf) — plus the mint-a-parent clause on the oversize sentence below the table
- **Tier failed:** n/a — supersession by ruling (record D2 — nested entries, leaf = pipeline unit; D3 two-level cap)
- **Content:** "| **Feature** | Product / pipeline unit | A built capability on the map; graduates through plan/implement as its own unit | **this skill** (map entry) |" · following sentence "A feature too large to land in one breath of implement is cut into vertical-slice cycles downstream — never into pseudo-features minted for pipeline convenience."
- **Kept deliberately:** the story and vertical-slice rows unchanged; the pseudo-feature ban survives verbatim, re-keyed feature→leaf; vertical-slice cycles remain the downstream cut for an oversize *at implement* — parent minting is the remedy only *at derivation*.
- **Consumers assessed:** `mochiko:patterns-vertical-tdd` (cuts one leaf now — Related line updated in the same edit) · plan/implement re-keyed to leaves in parallel seats.

## [v0.61.0] Invariant 6 "Writes land at acceptance" superseded by the split write rule
- **Disposition:** superseded → "**Delivery writes land at acceptance; stewardship writes are direct.**" — `/mochiko:feature` stewardship writes (stub minting, retroactive promotion, retire, grooming fixes) land directly outside spec acceptance; delivery-status writes (in-flight flips, graduations, delta folds) land only at acceptance landings; Write rules section gains the matching closing sentence
- **Tier failed:** n/a — supersession by ruling (lead ruling G4, citing record D6 stewardship remit + D12 stub minting)
- **Content:** "**Writes land at acceptance.** During a run, proposed entries and deltas live in the spec workspace. The map write is one atomic batch at spec acceptance. Reads happen any time; a rejected spec never touched the map."
- **Kept deliberately:** every original clause survives inside the new wording — workspace staging, the atomic acceptance batch, reads-any-time, rejected-spec-never-touched. The stewardship path is carved out, never a status flip or delta fold. Checklist line "All writes staged in the spec workspace" amended to match ("stewardship writes per invariant 6 excepted").
- **Consumers assessed:** the feature command (same wave) is the only stewardship writer; review-specifications still grades delivery writes against the workspace baseline.

## [v0.61.0] Invariant 7 R5 wording superseded — open spec OR live lane run
- **Disposition:** superseded → "every delta names its spec or lane run; every `in-flight` status or delta points at an open spec or a live lane run — live from dispatch until its acceptance landing; a delta whose lane run ended without folding is a defect, fix-on-sight." — plus the new parent-roll-up defect clause ("a parent whose status contradicts its children's roll-up is a defect")
- **Tier failed:** n/a — supersession by ruling (record D7 invariant amendment, review finding 4; D14 lane runs; D2 parent/child integrity extension)
- **Content:** "every delta names its spec; every `in-flight` status or delta points at an open spec — a closed spec still pointed at is a defect;" · agenda item 3 "**In-flight territory:** an `in-flight` or delta-carrying entry obligates a read into the owning spec's artifacts — its stories, plan, and architecture delta — so this run knows what the feature is *becoming*, not just that it is busy." (re-worded "owning spec's" → "owning work's" — lane runs also own in-flight deltas) · checklist line "Map read completed at intent; in-flight territory read into owning specs; reconstructed entries flagged for re-verify" (re-worded to "read into the owning work's artifacts (spec or lane run)", harmonized with the agenda in the same touch)
- **Kept deliberately:** the closed-spec defect clause survives verbatim; dangling-FEAT-ID, index/entry agreement, orphaned-delta, and specs-index-contradiction clauses untouched; agenda item 3's becoming-not-just-busy clause and artifact list survive verbatim.
- **Consumers assessed:** review-specifications carries the map-integrity invariants by reference (feature-map R7 — pipeline-core); the lane run's verification seat gains the boundary check in the implement cluster (parallel seat).

## [v0.61.0] Granularity guide oversize remedy superseded — split or mint a parent
- **Disposition:** superseded → "An extent that cannot be stated in ~3 lines is not one leaf — split it into two features, or mint a parent whose leaves each pass the bar." — plus the two-bars clause (one-breath polices the parent/flat name; ~3-line extent polices the leaf) and the matching red-flag/checklist rewordings
- **Tier failed:** n/a — supersession by ruling (record D1 — rounding-up happens because compositional structure is missing; D2/D4 — parent minting as the remedy)
- **Content:** "An extent that cannot be stated in ~3 lines is two features." · red flag "'This entry needs eight extent lines to be honest' — then it is two features; split" · checklist line "Every entry within the granularity guide — one-breath capability, extent ≤ ~3 lines — or split"
- **Kept deliberately:** the one-breath definition ("bigger than a story, smaller than a product area") verbatim; split remains a valid remedy alongside minting.
- **Consumers assessed:** product-manager agent applies this skill's guide at derivation — no restatement found in its persona (decoupling holds).

## [v0.61.0] Delta grammar superseded — names its spec or lane run; parent child-delta form added
- **Disposition:** superseded → "a delta on a `delivered` entry reads `extent grows by <X> — in-flight, <spec-slug or lane-run>` (on a parent carrying a late child: `new child FEAT-YYY — in-flight, <spec-slug or lane-run>`); it names its spec or lane run … A delta whose spec closed — or whose lane run ended — without folding is an integrity defect." Invariant 5 gains the matching lane-run and roll-up-yields-to-stickiness clauses; the rationalization-table row re-worded ("whose spec or lane run died").
- **Tier failed:** n/a — supersession by ruling (record D7/D14 — lane runs as first-class delta owners; D2 amended — sticky-delivered parent carries a late child as a marked delta)
- **Content:** "a delta on a `delivered` entry reads `extent grows by <X> — in-flight, <spec-slug>`; it names its spec, lives under the entry's Deltas heading, and folds into the extent lines at the owning work's acceptance landing. A delta whose spec closed without folding is an integrity defect." · invariant 5 "A later spec touching a `delivered` feature never regresses its status; the change rides as a marked delta until that work's landing folds it."
- **Kept deliberately:** the Deltas-heading home, the fold-at-acceptance timing, and stickiness itself unchanged; `retired`-is-terminal clause verbatim.
- **Consumers assessed:** `feature-entry-template.md` Deltas comment superseded in lockstep (see its strip note); the feature command authors lane deltas in this grammar.

## [v0.61.0] BACKLOG boundary line superseded — KM-scoped, extent-growth exception (D13)
- **Disposition:** superseded → "**Tracking defects, tooling, or process work** — those live in `BACKLOG.md` where KM exists (a non-KM product has no queue; lane runs accept direct requests — the stated degrade path, never silently assumed away). Extent-growth improvement ideas are the exception: they ride the map as `proposed` deltas or obligation lines — the map is the capability backlog." Rationalization row extended to match.
- **Tier failed:** n/a — supersession by ruling (record D13, review finding 12 — R15 boundary scoped to KM-adopting repos; extent-growth ideas ride the map per feature-map D9)
- **Content:** "**Tracking defects, tooling, or process work** — those live in `BACKLOG.md`; the map carries product capabilities only" · rationalization reality cell "The map states what the product does, not what needs fixing. Defects, tooling, and process live in `BACKLOG.md`."
- **Kept deliberately:** defects/tooling/process still never become entries; the feature-map D22 pseudo-feature ban untouched.
- **Consumers assessed:** the feature command's triage (D13/D14) is the runtime consumer — bound by reference in the parallel seat.
