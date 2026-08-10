# Strip notes — `templates/feature-entry-template.md`

Entry formats: `strips/README.md`. Wave context: the feature-sizing & entry-points build wave
(record: `.mochiko/brainstorms/feature-sizing-and-entry-points/record.md`; `DECISIONS.md` row
2026-08-10). The template gains the Parent/Children sections, the `unrefined` stub form, and
the feature-command proposed origin; the entries below record the superseded comment text.

---

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
