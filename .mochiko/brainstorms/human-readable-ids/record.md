# Human-readable IDs — decision record

**Status:** **accepted 2026-10-08** (user: "accept", after the post-review screen) — D1–D22 (D1–D20
before review, D21–D22 born at review; accept screen confirmed "screen ok" before the freeze) ·
**Landed:** `DECISIONS.md` row 2026-10-08 · `BACKLOG.md` *Human-readable IDs build* section (the build
item + the opt-in pass for user projects, OQ3) · `ROADMAP.md` folded into the Next row *Template-schema
CLI build* (caps held: Now 5 · Next 7) · the brainstorms index entry · **built 2026-10-09 at v0.118.0** (`build-log.md`; trail; `GLOSSARY.md` *joined ID*) · **Opened:** 2026-10-08
· **Lead:** session lead (inline questioning via `mochiko:analysis-iterative`) · **Review:** solo cold
review — blind map (48 angles, 19 load-bearing), cold read `critical-gaps` (15 survivors, 15/15
dispositioned), verify round 1 NOT CLEAN (1 blocking, user-ruled), verify round 2 NOT CLEAN with
nothing blocking (3 text repairs applied, closed seat-unverified by the user's word, disclosed) ·
**Size:** `standard` (user-ruled
2026-10-08, "rest ok") · **Seats:** `blind-reviewer` (`mochiko:devils-advocate` on
`mochiko:review-brainstorm`, persona default tier, no deviation; spawned at frame hardening with the
problem, destination and out-of-scope lines only; writes `reports/angle-map.md`, later
`reports/review.md`) · `blind-second-list` (fresh general-purpose seat, `opus`, read-only; the frame
and the two costliest questions, never the lead's options) ·
**Transport (`mochiko:patterns-transport-floor`, read back: 11 floors —
message-lane-trigger · topology-lane-trigger · neither-lane-waivable · composition-steer ·
single-writer-per-surface · mesh-hold · content-pinned-supersession · quiesce-before-cold-grade ·
no-ritual-sends · fan-in-confirmation · version-floor):** lead-relayed messaging only, so the message
lane binds; one writer per file — the record and the index are the lead's, the reports are the
reviewer's — so no shared write surface · **Tiering (`mochiko:patterns-model-tiering`, read back: 8
floors — class-key-session-tier · seat-default-key · override-is-the-pin · seat-deviation-bounds ·
persona-less-grader-pin · worker-return-is-a-claim · brief-obligation · worker-seat-set-reserved):**
fact reads so far were direct tool calls (the ladder's lowest rung); both seat briefs carry the
routing line

## Topic

Driver ask (user, 2026-10-08, at command entry): "i want to ensure in mochiko , the labels for
things like goverance , 'GI' of decisions like 'D1' becomes human readable. So retain GI-01 but
then after that add 3 word slug sperated by hypen , for example GI-01-must-use-logging"

## Frame card

Shown to the user 2026-10-08, before any deciding. Each line marked *user* (the user's words) or
*guess* (the lead's). **Hardened 2026-10-08** — line 3 corrected by the user and the lead's
rewording shown; the user confirmed it and every other line with "rest ok". From here a line
changes only by the user's word, stamped here.

- **Problem** — *guess* (the complaint is the user's, the "who" is the lead's): a reader of
  mochiko's records and governance — the user, and any seat reading a record or the ledger
  cold — meets bare IDs like "`GI-004`" or "`D7`" and must look each one up to know what it means.
- **Destination** — *user* (format) + *guess* (reach): each such ID keeps its number and gains a
  three-word kebab slug after it — the user's example `GI-01-must-use-logging`.
- **Must not break** — ~~*guess*: an existing ID's number never changes, so every citation
  already written still resolves; mint-once and ID continuity hold; the CLI-rendered rules and the
  contract suite stay green.~~ **Corrected by the user, 2026-10-08:** "i am okay with breaking
  changes and dont need backward compatibilty". Lead's rewording, confirmed by the user ("rest ok"): no compatibility owed — existing IDs and every citation of them may be rewritten
  wholesale, and old forms need not resolve; only the project's own release gates (contract suite,
  `cargo test`) still hold for whatever ships.
- **Betting on** — *guess*: three words carry enough meaning to skip the lookup, and stay true
  as the thing they name evolves. **Changed by the user's word, 2026-10-08, at build** (D8-read-test-bet's wave-0
  FAIL, overridden on the lead's lean A, "as recommded"): three topic words tell the reader what an ID
  is about; reading the rule for what it demands stays the reader's job.
- **Out of scope** — *guess*: rule IDs (already dotted slugs, e.g. `brainstorm.frame-card`);
  renaming the prefixes themselves (`GI`, `D`).
- **Proposed size** — *guess*: `standard` — several real forks (which ID families, slug at every
  citation or only where defined, back-fill old IDs or new only, the plugin's minting for
  consumer projects or this repo only, enforcement).

## Facts

Quoted source lines with path and line number; the lead's reading follows each, marked.

- **F1 — Definitions already carry a name.** `.mochiko/memory/governance-ledger.md:92`:
  "### GI-003 — Secrets Out of the Repo · home: CLAUDE.md region line" ·
  `.mochiko/brainstorms/brainstorm-target-state/record.md:214`: "### D1 — Open: the lead orients
  lightly before its first message — `Confident`". *Reading:* where an ID is defined, a human name
  already sits beside it; the lookup pain is at the citation sites.
- **F2 — Citation sites are bare.** `CLAUDE.md:113`: "… (NON-NEGOTIABLE) <!-- GI-003 -->" ·
  `CLAUDE.md:72`: "… (softened per the `schema-based-template-guidance` D11 ruling; governance trace
  GI-019 · …". *Reading:* these are the lines the problem describes.
- **F3 — D numbers are scoped per session, and cited in ranges.** `plugins/mochiko/migrations/0001-genesis.yaml:560`:
  "decisions in one `D1…` namespace." · `DECISIONS.md:23`: "| 2026-09-24 | Delta files vs direct
  baseline edits ruled (D1–D7 as review-amended): …". *Reading:* a bare "`D7`" is ambiguous across
  sessions (cross-session citations already prefix the session slug, F2's `schema-based-template-guidance`
  D11-kernel-position-softened); 60 lines of `DECISIONS.md` carry a range like `D1–D7` (count by `grep -cE 'D[0-9]+–D?[0-9]+'`).
- **F4 — The plugin mints GI IDs in every user project.** `plugins/mochiko/migrations/0019-setup-agnostic-modules-out.yaml:49`:
  "GI-ID rule: sequential GI-001, GI-002, … — unique forever within this file; never reuse a" ·
  `:448`: "- [Imperative universal principle] <!-- GI-XXX -->". *Reading:* a change to the GI form is
  a change to shipped primitives (migration + strips + audit + version bump), not only a docs sweep.
- **F5 — The plugin mints many more numbered families.** Token counts of `PREFIX-XXX`/`PREFIX-NNN`
  forms under `plugins/mochiko/` (method: `grep -rhoE '\b[A-Z]{1,5}-(X{2,4}|N{2,4}|[0-9]{3})\b'`):
  FEAT 150 · AX 86 · D 82 · GI 79 · C 75 · IP 71 · NFR 69 · FR 69 · SC 50 · SPN 46 · SCR 45 · FLOW 41
  · DS 38 · EPIC 36 · INT 23. *Reading:* `D-XXX` is a second, separate "D" family (product design
  decisions), distinct from the session `D1…` cards.
- **F6 — Volume of existing citations.** Token counts (method: `grep -rhoE '\bGI-[0-9]{1,4}\b'` and
  `'\bD[0-9]{1,3}\b'`): GI — `.mochiko/` 1,946 · `evals/` 13,145 · `DECISIONS.md` 65 · `CLAUDE.md`
  26 · `crates/` 40 · `plugins/mochiko/` 41; bare D — `.mochiko/` 17,262 · `evals/` 5,098 ·
  `DECISIONS.md` 849 · `crates/` 564 · `plugins/mochiko/` 681. *Reading:* a back-fill is a mass
  rewrite; `evals/` holds most GI tokens.
- **F7 — The CLI does not parse GI IDs.** Every GI hit under `crates/mochiko-cli/src` found by
  `grep -rnE 'GI-'` is a doc comment, e.g. `crates/mochiko-cli/src/lib.rs:7`: "//! standing bright
  line (GI-019)." *Reading:* method-scoped — no GI parser was found by that grep; the crate only
  cites GI. **Corrected 2026-10-08:** the first wording said "these IDs" and covered D too; that was
  wrong — see F9, surfaced by the blind second-list seat and verified first-hand by the lead.
- **F9 — The CLI does parse session D numbers, in migration anchors.** `crates/mochiko-cli/src/replay.rs:577`:
  "\"anchor {anchor:?} is malformed — want 'YYYY-MM-DD <session-slug> [D#]'\"" ·
  `crates/mochiko-cli/src/model.rs:1292` (cite corrected at review, R7 — was `:1293`): "/// `D<n>` or
  `[D<n>]` — the human-readable decision pointer, never resolved here." · live example, `plugins/mochiko/migrations/*.yaml`: "anchor:
  2026-09-03 cli-schema-delivery D9"; 43 migration files carry `anchor:` (`grep -lE 'anchor:'`).
  *Reading:* a slugged D in an anchor is rejected at replay today; slugging D there is a crate
  change, not only a prose change.
- **F10 — The blind second list.** Fresh seat, frame and the two questions only, 2026-10-08. Reach
  roads: do nothing + a lookup verb · this repo's docs only · GI and D end to end, minters included
  (its lean) · same but forward-only · every governance/record family (GI, D, AM, PO-D, OQ) · every
  numbered family incl. product IDs. Placement roads: slug at every single-ID mention, ranges and
  lists bare (its lean) · mint site only · other files only · first mention per document · prose
  only, machine-read spots bare. It also flagged the padding question (`GI-01` vs "`GI-004`") and the
  frozen-corpus edge (`.mochiko/archive/`, `crates/mochiko-cli/tests/fixtures/genesis-corpus/`).
  *Reading:* its reach roads 4–6 split into the lead's Back-fill and Which-families items; its
  placement roads 3 and 5 are new to the lead's list.
- **F11 — Setup regenerates the governance surfaces, but keeps untouched GI IDs verbatim.** Fact
  seat `fact-setup-amend` (fresh, `opus`, read-only), 2026-10-08; the two decisive rules re-rendered
  by the lead (`mochiko-cli rules --section authoring-constitution.sec.artifact
  authoring-constitution`): `authoring-constitution.markers-only-regeneration` — "Rules files and the
  ledger are setup-owned and regenerated whole, except the two preserved blocks." ·
  `authoring-constitution.amend-preserves-verbatim` — "Amend preserves untouched principles verbatim
  (their GI-IDs are stable) — …". The three-digit form is stated in one rule only
  (`plugins/mochiko/migrations/0019-setup-agnostic-modules-out.yaml:49`, F4); elsewhere it is implied
  by placeholders `GI-XXX` / `GI-0XX` and literal examples (seat's report; ledger heading shape
  `### GI-XXX — [Principle Name] · home: …` at `0019-setup-agnostic-modules-out.yaml:564`), plus
  shipped prose `plugins/mochiko/skills/validation-constitution/references/QUALITY-CHECKLIST.md:100`
  "Trace stamps are real IDs (e.g., \"GI-007\", NOT \"GI-XXX\")" (seat's quote). No mechanical
  GI-form check exists: the seat's dry-run `mochiko-cli check` of this repo's
  `governance-intent.md` with every `GI-0NN` rewritten to `GI-0NN-alpha-beta-gamma` gave the same
  verdict as the unmodified file (seat's report, not re-run by the lead). *Reading:* changing the
  setup template alone would not slug an existing project's GI IDs — an amend keeps untouched IDs
  verbatim; the slug needs its own rule. The trace match is a grader seat's string match, so the
  joined form must be matched consistently on both sides.
- **F12 — No path migrates an existing project to a new ID grammar.** Fact seat, 2026-10-08:
  amend opens only on a closed set of six governance events and a plugin upgrade is not one
  (setup template Shape 1, seat's quote: "the governance events are closed at six: the depth flip ·
  a waiver added, lifted or re-grounded · a stack, toolchain or layout change · an engineering
  principle minted, dropped or redefined · an engineering module attached or detached · the
  deliberate exclusions changed; …"); the only legacy path is a closed list of retired shapes that
  an amend run supersedes in place, and the GI form is not on it; `mochiko-cli migrate` works on the
  plugin's migration log only. The artifact gate tolerates stale shapes —
  `crates/mochiko-cli/src/conform.rs:24-26` (lead-verified): "a fault the baseline already had and
  the candidate does not worsen is **allowed**". *Reading:* an existing project such as kinako keeps
  bare IDs until something new rewrites them; nothing in today's plugin would.
- **F13 — A name beside a number already drifts, and today it is renamed in place.**
  `git show dc01941:.mochiko/memory/governance-ledger.md` (2026-08-16): "### GI-020 — Additive Plugin
  Install · home: CLAUDE.md `## Non-negotiable constraints` (prose)" · today,
  `.mochiko/memory/governance-ledger.md:581`: "### GI-020 — Clone-Only Install with a Required
  `mochiko-cli` Dependency · home: …" · `CLAUDE.md:74`: "(governance trace GI-020,
  superseded-by-ruling v3.0.0 — `cli-schema-delivery` D4/D10)". *Reading:* the number held through a
  supersession that reversed the principle's meaning (additive install, then a required binary);
  the title was renamed in place. A slug frozen at mint would read "`GI-020-additive-plugin-install`" —
  now false.
- **F14 — Compound references are common, and most ranges are wide.** Counts over every `*.md` in
  the repo (method: `grep -rhoE` with the patterns shown): ranges `[A-Z]{1,5}-?[0-9]+–…` 1,710;
  slash pairs `[A-Z]{1,5}-?[0-9]+/…` 2,962 (rough — catches some non-ID pairs); session-D ranges
  by width: ≤3 IDs 77 · 4–7 IDs 337 · ≥8 IDs 393, widest 23 (`D1–D23`, 8 hits). Examples:
  `DECISIONS.md:23` "(D1–D7 as review-amended)" · `CLAUDE.md:74` "`cli-schema-delivery` D4/D10".
  *Reading:* expanding every range would put seven-plus joined IDs in most of them.
- **F15 — What enforcement is admissible, and what the gate checks today.** `CLAUDE.md:72`:
  "advisory post-hoc checkers used as optional exit-code signals are not kernel-class" — and the same
  line: kernel-class tooling "is admitted only where a recorded ruling justifies it, and even then
  never gates pipeline progress, never dispatches or sequences agents, and never holds judgment that
  skills own". `crates/mochiko-cli/src/conform.rs:9`: "//! # The six checks, in the order a deny
  reports them (record D4)" — path · file set · frontmatter · headings · placeholders · size
  (`conform.rs:11-18`); none reads ID form (F11). *Reading:* a rules-only road and an advisory
  checker need no ruling; a write-time deny on bare IDs is a seventh gate check and needs a recorded
  kernel-class ruling (precedent: the conformance-gate admission `hook-enforced-artifact-schema` D1-conformance-deny-channels/D7-governance-routing-supersession,
  `CLAUDE.md:72`). Whether a slug is apt is judgment the skills own; whether an ID is bare is mechanical.
- **F16 — The size of one rename.** `GI-020-plugin-install-model` (the F13 case) is mentioned 438 times across 128 files,
  100 of them outside `.mochiko/archive/` and `evals/` (method: `grep -rhoE '\bGI-020\b'` /
  `grep -rlE`, excluding `.git` and `target`). *Reading:* under D4-slug-follows-topic, that supersession would have
  been a ~100-file rewrite.
- **F17 — Some families restart their numbers per artifact.** `plugins/mochiko/migrations/0001-genesis.yaml:4738`:
  "FR numbering is sequential, three-digit padded, no gaps (FR-001, FR-002…);" · two eval-fixture
  specs both open at `FR-001`
  (`evals/agents/requirements-analyst/fixtures/r1-vague-request-discovery/specs/expense-submission/spec.md`,
  `…/r2-vague-criteria-rewrite/specs/ticket-attachments/spec.md`, by `grep -oE '\bFR-[0-9]+'`);
  session `D1…` restarts per record (F3). *Reading:* `FR-001` and "`D7`" are unique only inside their
  owning spec or record, so "the number is the key" holds only within that scope — a rename keyed on
  the bare number project-wide would rewrite every spec's `FR-001`. Which other families restart per
  artifact is in the `fact-id-families` enumeration, not yet landed.
- **F18 — The joined form already exists, in file paths.** `crates/mochiko-cli/src/home.rs:293-294`
  (lead-verified): "/// `<PREFIX>-<digits>` with at least three digits, optionally `-<slug>`. `AX`
  ids always carry /// the trailing slug; `FEAT` and `EPIC` ids may stand alone." Live examples in
  repo markdown: `AX-002-mandated`, `AX-099-probe` (`grep -rhoE '\bAX-[0-9]{3}-[a-z]+…'`). Migration
  log: 739 `anchor:` lines in all (`grep -rhcE '^\s+anchor:'`, summed) — the 43 header anchors plus
  the per-change anchors inside supersede operations, all inside the hash. *Reading:* `AX` file names
  are a precedent for the joined form; but a slug in a path means a D4-slug-follows-topic rename moves a file. Added
  from `fact-id-families` (seat's quote, `crates/mochiko-cli/src/home.rs:258`): `"<n>" =>
  segment.chars().all(|c| c.is_ascii_digit())` — a story file `stories/US-12-export-csv-report.md`
  would be refused today. Also (seat's quote, `.mochiko/schema-views/commands/implement.yaml:719`):
  "the second run to land renumbers its own entries" — `C-`/`D-`/`IP-`/`INT-`/`DS-XXX` numbers already
  change after landing, so their slugs travel with a renumber.
- **F22 — Where the existing GI and D mentions sit, and the protected-content rule.** Counts of
  `\bGI-[0-9]{3}\b|\bD[0-9]{1,3}[a-z]?\b` by `grep -rhoE`: `.mochiko/archive/` 615 · `.mochiko/` outside
  the archive and the replayed views 18,838 · `.mochiko/schema-views/` 468; `evals/` GI hits are mostly
  recorded run captures (F19). Decision-card headings `^### D<n> ` across the 74 session records: 375
  (older records use other layouts, so the D count is a floor). `CLAUDE.md:90`: "Protected content — a
  record's protected set, a `KEPT:` line, or a `DECISIONS.md`-traceable line — leaves **only** as a
  recorded supersession-by-ruling; a silent deletion is what the audit's preserved-responsibilities
  check reads as a regression." *Reading:* the archive is small; the bulk is the live record layer;
  slugging needs at least ~375 decision slugs coined by judgment; a format-only rewrite of protected
  lines is not plainly a "leave", so it needs its own ruling. All-family counts (a second, slower
  run over the in-scope prefixes plus bare `D<n>`): repo-wide 102,449 · `.mochiko/archive/` 621 ·
  `CHANGELOG.md` 388 · migrations 832 · `.mochiko/schema-views/` 551; its eval-runs figure is
  discarded — the `find … -o …` expression double-counted (it exceeded the repo-wide total).
- **F23 — Kernel-class admissions have gone through a setup amend.**
  `.mochiko/brainstorms/hook-enforced-artifact-schema/record.md:463`: "### D7 — Governance routing:
  narrow supersession of `cli-schema-delivery` D7 and a GI-019 ledger amend, through a `/mochiko:setup`
  amend run before the hooks ship" · `CLAUDE.md:72`: "(… governance trace GI-019 · widened admission:
  `cli-schema-delivery` D11 · conformance-gate admission: `hook-enforced-artifact-schema` D1/D7)" ·
  `.mochiko/schema-views/templates/governance-surfaces.yaml:64`: "Amend via `/mochiko:setup` — the
  governance events are closed at six: …" (the list includes "an engineering principle minted, dropped
  or redefined", F12). *Reading:* both prior widenings of GI-019-kernel-tooling-admission were recorded on the ledger through an
  amend run; a third (D7-scoped-rename-command's tool) fits "a principle redefined".
- **F24 — Each family's numbering scope** (added at review, S4; read off F20's quoted minting rules).
  *Project-wide:* `GI` ("unique forever within this file", one intent file per project) · `FEAT`,
  `EPIC` ("the same id family as `FEAT-XXX`") · `AX`, `SPN` ("unique store-wide") · `NFR` ("the next
  free id in the store") · `GAP` (ledger). *Per artifact:* `FR`, `SC`, `US`, `SCR`, `FLOW` (per spec;
  `FR` restarts, F17) · `C`/`D`/`IP`/`INT`/`DS` (per product file — a spec's or the product baseline's
  `constraints-and-decisions.md`; renumbered after landing, F18) · `BR` (per `data-model.md`) · session
  `D` (per record) · cycles `C<n>` (per `tasks.md`). Cross-file citers of per-artifact IDs:
  `.mochiko/schema-views/templates/tasks.yaml:10`: "Cite spec/design content by ID (US-#, FR-#, SC-#,
  C-#)"; SC is also cited from feature-entry obligations and NFR sources (F20). A feature's work rows
  are cut by more than one spec over time — `.mochiko/schema-views/templates/feature-entry.yaml:67`:
  "`pending` — {{increment}} · acceptance: {{criteria}} · cut by {{spec-slug or lane-run}}". *Reading:*
  a `tasks.md` citing `FR-012` does not by itself say which spec's `FR-012`.
- **F19 — The second blind list.** Fresh seat `blind-second-list-2`, frame + rulings D1–D4 + the
  three questions only, 2026-10-08. Its facts, first-hand: `is_decision_segment`
  (`crates/mochiko-cli/src/model.rs:1298`) rejects "`D2-audit-before-bump`"; no migration hash is
  pinned outside the log; `<!-- GI-… -->` markers are read only by skills and template goldens;
  `evals/` GI hits are mostly recorded `runs/` captures. Back-fill roads: new mints only · rewrite
  everything incl. archive and run captures · rewrite the live layer, freeze the history layer (its
  lean) · at touch time · a reviewed slug map keyed (family, session, number) applied by a throwaway
  script (its lean's method). Machine-read roads: exempt every machine spot (its lean) · forward-only
  with a grammar bump · rewrite and re-stamp every migration · bare keys with the slug rendered from a
  registry · split by kind (hashed log bare, live markers joined, directories number-only).
  Existing-project roads: accept both forms, number as key (its lean, with an opt-in pass deferred) ·
  upgrade at touch time · an opt-in back-fill pass · a forced migration · a CLI apply of an approved
  slug map. *Reading:* its machine-spots lean is a carve-out from D2-joined-form-placement — D2-joined-form-placement left machine spots to this
  separate decision, so no ruling is challenged; its "split by kind" road is new to the lead's list.
- **F20 — The complete family list.** Fact seat `fact-id-families` (fresh, `opus`, read-only),
  2026-10-08, over the replayed views `.mochiko/schema-views/` (`V/`) and `plugins/mochiko/` prose;
  full report kept in the lead's session scratchpad (`id-family-enumeration.md`, not persisted —
  every row below cites a repo path). Spot-checked by the lead:
  `V/skills/authoring-technical-requirements.yaml:268` (rule
  `authoring-technical-requirements.sequential-ids`) and `V/templates/tasks.yaml:74` "### - [ ]
  Cycle 1: Walking skeleton — [thinnest end-to-end path]".
  - *Durable, plugin-minted* (16 rows, 21 prefixes, seat's quotes; `AX` and `SPN` share one entry
    below, so 15 entries show — count note added at review, R7): `FR-XXX`
    (`V/skills/authoring-requirements.yaml:119` "FR numbering is sequential, three-digit padded, no
    gaps") · `SC-XXX` (`:137`; padding mixed — `SC-1` at `V/templates/spec.yaml:220` vs `SC-007` at
    `V/templates/architecture-store.yaml:201`) · `US-<n>` unpadded
    (`V/skills/authoring-user-stories.yaml:157`, rule id at `:151` — cite corrected at review, R7:
    "`stories/US-<n>.md`") · `FEAT-XXX`
    (`V/skills/authoring-feature-map.yaml:126` "`.mochiko/features/FEAT-XXX-<slug>.md`") ·
    `EPIC-XXX` (`V/skills/authoring-epic.yaml:74`) · `AX-XXX` and `SPN-XXX`
    (`V/skills/authoring-architecture-store.yaml:105`) · `C-`, `D-`, `IP-`, `INT-`, `DS-XXX`
    (`V/skills/authoring-technical-requirements.yaml:268`) · `NFR-XXX` (`…/ARTIFACT-TEMPLATES.md:226`)
    · `SCR-XXX`, `FLOW-XXX` (`V/skills/authoring-prototype.yaml:83`) · `GI-XXX`
    (`V/templates/governance-intent.yaml:35`) · `GAP-XXX`
    (`P/templates/constitution-modules/evolution-notes.md:20`) · `BR-XXX` (reference example only,
    `P/skills/patterns-entity-modeling/references/VALIDATION-RULES.md:108`) · brainstorm `D1…`
    (`V/commands/brainstorm.yaml:182`) · cycle cards `Cycle N` / `C<n>` (`V/templates/tasks.yaml:74`,
    `:89` "**Depends on:** C1"; cited by `**Raised:** <cycle>`, `V/commands/implement.yaml:697`) ·
    `wave<n>-<slug>.md` (already slugged).
  - *Session-internal, plugin-minted*: build tasks `T{cycle}.{n}` ("IDs local to this report",
    `P/skills/executing-tdd-cycle/references/CYCLE-REPORT-FORMAT.md:46`) · gates `C<n>-gate[-k]` ·
    run folders `<owner>-run<n>` (deleted at acceptance) · report labels `G<n>`, `C<n>`, `A<n>`,
    `F<n>` · intent-review survivors `S<n>` · counters.
  - *Repo-only, not minted by any plugin rule*: session-prefixed decisions ("`PO-D1`" `CLAUDE.md:42`,
    "`OD-D1`, `AD-D1`, `PT-D1`, …") · `AM-5` (`CLAUDE.md:109`) · `OQ-2` · `J-1`, `HF-1`, `F-1`, `FP-1`,
    `FC-3`, `O-1` (seat's list) · `DQ-`, `RI-` (in the lead's opening prefix count, `grep -rhoE
    '\b[A-Z]{1,5}-[0-9]{1,4}\b'` over `*.md`/`*.yaml`/`*.rs`: DQ 122, RI 136; added at review, R7) · census IDs in strips (`RB-4`, `RGI-7`, `RF-7`, `RCM-4`, `R-012`).
  - *Already slugged in file names*: FEAT entry files, AX graduated files (crate-required), SCR
    screen files `scr-001-<slug>.html`, `wave<n>-<slug>.md`, migrations `NNNN-<slug>.yaml`.
  - *Names at the definition site*: present for US, FEAT, AX, SPN, C/D/IP/INT/DS, NFR, SCR/FLOW, GI
    (partial), brainstorm D, cycles; absent for FR ("one line each", `V/templates/spec.yaml:61`), SC,
    EPIC, GAP, BR.
  *Reading:* `C1` (a cycle) and `C-001` (a constraint) differ only by the hyphen; `FR`, `SC`, `EPIC`,
  `GAP` and `BR` have no name today, so their slug is coined from the line's text.
- **F21 — Plugin scripts parse FR/SC/US strictly.**
  `plugins/mochiko/skills/authoring-requirements/scripts/validate-requirements.py:84` (lead-verified):
  "if not re.match(rf'^{prefix}-\d{{3}}$', req['id'], re.IGNORECASE):" · seat's quotes:
  `plugins/mochiko/skills/review-plan-artifacts/scripts/check-artifacts.py:151`
  `r'\bFR-[A-Z0-9]+-?[0-9]*\b|\bFR-[0-9]+\b'` ·
  `plugins/mochiko/skills/authoring-user-stories/scripts/validate-user-stories.py:31` (story heading
  pattern). *Reading:* the joined form fails `validate-requirements.py` today; these scripts are
  shipped primitives and change with the build. In the crate, only `FEAT`, `EPIC`, `AX` (paths) and
  the anchor `D<n>` are parsed (F9, F18); no other family has a crate grammar (seat, by grep over
  `crates/mochiko-cli/src`).
- **F8 — Archives are frozen.** `.claude/rules/mochiko/operating-docs.md:21`: "`.mochiko/archive/**`
  and the trail are append-only/frozen — never edit archived content." *Reading:* a back-fill meets
  this invariant; the user's "no compatibility owed" does not by itself lift it.

## Decision map

Drawn 2026-10-08 after the frame hardened. Each later change is shown in the conversation and
noted here.

**Askable now**
- **Reach** — the plugin's ID grammar for every user project, or this repo's own records only (F4).
- **Placement** — where the slug appears: at definitions (already named, F1), at every citation
  (F2), or citations only (F1+F2).
- **Number form** — keep today's padding ("`GI-003`") or change it to the user's example (`GI-01`).
  Default proposed: keep today's padding.

**In fog**
- **Which families** — `GI` and session `D` only, the governance/record families too (AM, PO-D,
  OQ), or every family the plugin mints incl. product IDs (F5, F10) — hangs on Reach. *[blind map
  A1 LB; A3, A4 folded]*
- **Joined-form grammar** — *[blind map B2 LB]* the hyphen is shared by number and slug: a stated
  pattern (slug words never digit-led), sub-IDs `D2a` vs a slug `D2-a…`, word count, charset, grep
  boundary (`D1-` no longer matches "`D10`") — hangs on Number form. *[A5, B3, E6 folded]*
- **Slug life** — coined once and frozen, or renamed when the meaning drifts; whether the number
  alone stays the key and the slug is a label; slug source (from the existing title?), strength
  words like `must-` in the slug, retired IDs — hangs on Placement. *[blind map C1, C2 LB; B4, B5,
  C3, C6, H2 folded]*
- **Compound references** — what ranges `D1–D7`, slashes "`D4/D10`", lists and cross-session cites
  `` `slug` D11 `` become (~1,650 range/slash hits, blind map's count); D slugs can collide across
  sessions, so the session qualifier stays owed (F3) — hangs on Placement. *[blind map "D2 LB; C5, D4"
  folded]*
- **Machine-read spots** — migration `anchor:` fields sit inside the canonical hash: change the
  crate, rehash history, or leave them bare by a ruled exception; shipped rule text that cites IDs
  changes only by new migrations; `<!-- GI-… -->` markers; crate code — hangs on Reach and
  Placement. *[from F9/F10; blind map E1, E2 LB; "D3, E3" folded]*
- **Back-fill** — rewrite existing IDs, or new IDs only (forward-only); archives, closed records
  and the replay fixture corpus frozen; `KEPT:` and `DECISIONS.md`-traceable lines: a format-only
  rewrite as supersession-by-ruling with strips, or a ruled non-semantic exemption; contract-suite
  expectations re-frozen; rewrite mechanism, order and the mixed-state window — hangs on Reach,
  Which families, Placement. *[blind map F1, F2, I3 LB; A6, E4, F3, F4, F5 folded]*
- **Governance route** — *[blind map G1 LB, new]* a GI form change as a `/mochiko:setup` amend or a
  direct edit; the region template must change or the next regeneration strips the slugs; version
  semantics of a breaking ID grammar — hangs on Reach. *[G3 folded]*
- **Existing user projects** — *[blind map H1 LB, new]* if Reach is plugin-wide: projects like
  kinako with bare ledgers and records — reject, ignore, or migrate the bare form — hangs on Reach.
- **Enforcement** — a CLI check, a review criterion, or nothing (F7; kernel-class only by ruling);
  who mints a slug and who checks its quality — hangs on Reach and Placement. *[blind map J1 LB; C4,
  E5 folded]*
- **Done check** — *[blind map J2 LB, new]* a measurable end: zero bare in-scope IDs outside a named
  allowlist, by a named check, with a named runner — hangs on Enforcement and Back-fill.
- **Bet test** — *[blind map I1 LB, new]* the frame's bet (three words carry enough meaning) is
  untested; a seat may act on the slug's paraphrase instead of the rule, NON-NEGOTIABLE lines
  especially; how the bet is tested — hangs on Placement. *[I4 folded: every seat pays the token
  cost on every load]*

**Out of scope** — rule IDs; prefix renames.

**Steelmen carried to the forks** *[blind map I2 LB]* — (a) titles already at the definition site,
printed on first mention only: Placement; (b) a glossary and (c) a `mochiko-cli` lookup verb: Reach
option C; (d) an inline gloss with no grammar change, e.g. "`GI-004 (primitive audit)`": outside the
user's destination line (the joined form is the user's words) — shown to the user once, kept only by
a frame change; (e) render-time expansion: with (c).

**Blind-map angles dropped, for the end read** — G2 and G4 (the primitive-edit ceremony and the
landing ritual bind the build already; no decision to make) · G5 (the sound-loop floor binds the
rewrite already) · J3 (rollback: no compatibility owed, so rollback is a git revert) · F6 (git
history is immutable; provenance queries stay bare by necessity, nothing to rule).

*Map change 2026-10-08:* Machine-read spots added; Back-fill gains the blind list's forward-only road
and the frozen fixture corpus; Which families gains the governance/record-family set (AM, PO-D, OQ).
*Map change 2026-10-08, blind map landed (48 angles, 19 load-bearing; fan-in 2/2 — map and second
list both arrived):* Joined-form grammar, Governance route, Existing user projects, Done check and
Bet test added; every load-bearing angle folded above, marked; five non-load-bearing angles dropped,
listed. No ruling yet made, so none goes back to the user.
*Map change 2026-10-08, after D1-slug-form-reach:* Reach decided (D1-slug-form-reach). Which families graduates to askable now.
Governance route and Existing user projects graduate to askable once their facts are in hand (how a
setup amend regenerates the region; whether setup has a migration path for an existing ledger).
*Map change 2026-10-08, after D2-joined-form-placement:* Placement decided (D2-joined-form-placement). Slug life, Compound references,
Machine-read spots, Enforcement and Bet test graduate to askable now (each hung only on Reach and
Placement). Back-fill waits on Which families; Done check on Enforcement and Back-fill. Fact seat
dispatched for Governance route and Existing user projects. Number form stays a default that
Joined-form grammar hangs on, so it is confirmed before that question.
*Map change 2026-10-08, facts landed (F11, F12; fan-in 1/1):* Governance route and Existing user
projects graduate to askable now.
*Map change 2026-10-08, after D3-slugged-id-families:* Which families decided (D3-slugged-id-families, `Contested`). Back-fill graduates to
askable now. New default, shown by name: **Family boundary** — durable IDs (cited outside the
artifact that minted them) take the slug; session-internal working labels (`F`, `Q`, `S`, `V` in a
record or review) do not — Back-fill hangs on it, so it is confirmed before that question. Fact seat
`fact-id-families` dispatched for the complete family list (blind map A1).
*Map change 2026-10-08, after D4-slug-follows-topic:* Slug life decided (D4-slug-follows-topic). Blind second list `blind-second-list-2`
dispatched on the three forks costly to undo — Back-fill, Machine-read spots, Existing user
projects — so each is put only after its list lands. Compound references put next.
*Map change 2026-10-08, after D5-compound-reference-forms:* Compound references decided (D5-compound-reference-forms). New default, shown by name:
**Cross-session qualifier** — a citation of another session's decision keeps the session slug in
front (`` `cli-schema-delivery` D11-widened-kernel-admission ``), since D numbers and slugs can repeat
across sessions (blind map C5); the lead first stated it to the user as following from D2-joined-form-placement without a
question — corrected here to a default for the wrap-up batch. New item, askable after Enforcement:
**Rename mechanism** — who and what rewrites every mention when a slug is renamed (D4-slug-follows-topic's accepted
risk). Enforcement put next.
*Map change 2026-10-08, after D6-minting-check-enforcement:* Enforcement decided (D6-minting-check-enforcement). Rename mechanism graduates to askable
now and is put next, in another form (case against first, a weak lean) after three ratified rulings
in a row. Done check still waits on Back-fill.
*Map change 2026-10-08, after D7-scoped-rename-command:* Rename mechanism decided (D7-scoped-rename-command); D4-slug-follows-topic's statement amended by the
user's word (scoped identity, F17). Governance route now also carries the GI-019-kernel-tooling-admission admission route for
D7-scoped-rename-command's tool, and is put after Existing user projects — both turn on the same question (what rewrites an
existing ledger). Bet test put next while the second blind list is out.
*Map change 2026-10-08, after D8-read-test-bet:* Bet test decided (D8-read-test-bet). Nothing else askable until the second
blind list lands, so the three standing defaults (Number form · Family boundary · Cross-session
qualifier) are put for confirmation now — Joined-form grammar hangs on the first, Back-fill on the
second.
*Map change 2026-10-08, second blind list landed (F19; fan-in 1/1):* Machine-read spots and Existing
user projects graduate to askable now; Back-fill waits only on the Family boundary default. Machine-
read spots gains "directory and file names" (F18).
*Map change 2026-10-08, family list landed (F20, F21; fan-in 1/1):* two defaults restated before the
user answered them, to fit the list — **Number form:** each family keeps the padding its minting rule
states (three digits for the `-XXX` families; unpadded `US-<n>`, session `D<n>`, cycle `C<n>`); `SC`'s
mixed examples settle on three digits, which `validate-requirements.py:84` already requires.
**Family boundary:** the test is durable — cited outside the file that minted it — gets the slug;
local working labels stay bare (`T3.2`, `C3-gate-2`, run folders, report `G1`/`C1`/`A1`/`F1`, `S1`,
this record's `F`/`Q`/`S`/`V`); in scope: the 21 plugin prefixes, session `D` cards, cycle cards, and
— an extension of D3-slugged-id-families's "plugin mints" wording, so it needs the user's word — this repo's own durable
forms ("`PO-D1`", `AM-5`, `OQ-2`, census IDs where cited outside their strip); file names already
slugged stay as they are.
*Map change 2026-10-08, defaults confirmed ("default ok"):* Number form (D9-number-padding-form), Family boundary (D10-durable-id-boundary,
extending D3-slugged-id-families) and Cross-session qualifier (D11-cross-session-qualifier) decided. Joined-form grammar and Back-fill graduate
to askable now. Machine-read spots put next.
*Map change 2026-10-08, after D12-machine-read-spots:* Machine-read spots decided (D12-machine-read-spots). Joined-form grammar put next,
in another form (case against first, weak lean), its one real fork being what a slug says; its
mechanical parts shown as defaults — pattern `<ID>-<w1>-<w2>-<w3>`, exactly three lowercase ASCII
words each starting with a letter (the user's "3 word slug"), a sub-ID letter attached to the
number (`D2a-…`), the slug coined from the definition's name or text.
*Map change 2026-10-08, after D13-topic-word-slugs:* Joined-form grammar decided (D13-topic-word-slugs; its mechanical defaults ride to
the wrap-up batch). New item, hanging on Back-fill: **Protected lines** — whether a format-only ID
rewrite of a `KEPT:` or `DECISIONS.md`-traceable line is a supersession needing strips, or a ruled
non-semantic change (F22, blind map F2). Back-fill put next, in the changed form.
*Map change 2026-10-08, after D14-live-layer-backfill:* Back-fill decided (D14-live-layer-backfill). Protected lines and Done check graduate
to askable now. Protected lines put next.
*Map change 2026-10-08, after D15-protected-line-rewrites:* Protected lines decided (D15-protected-line-rewrites). Existing user projects put next;
then Governance route (D7-scoped-rename-command's admission and D15-protected-line-rewrites's ruling both route through it); then Done check.
*Map change 2026-10-08, after D16-existing-user-projects:* Existing user projects decided (D16-existing-user-projects). Governance route put next.
New default, shown by name: **Version bump** — the build ships as an ordinary minor `plugin.json`
bump under 0.x semantics (blind map G3), with the breaking change named in `CHANGELOG.md`.
*Map change 2026-10-08, after D17-governance-amend-route:* Governance route decided (D17-governance-amend-route). Done check put next — the last
askable item; after it, only the defaults batch (grammar mechanics, Version bump) remains.
*Map change 2026-10-08, after D18-build-done-check:* Done check decided (D18-build-done-check). The map is empty — nothing askable,
nothing in fog, no open frame line, the blind map landed and every angle folded or listed as dropped.
The defaults batch goes to the user on the accept screen.

## Decisions

### D1-slug-form-reach — Reach: the plugin's minting rules and this repo both carry the slugged form — `Confident`

**Statement.** The slugged ID form is a change to the plugin's own ID grammar — the rules that mint
IDs in every user project (setup's GI markers and ledger entries, brainstorm's decision cards) — and
this repo's own records follow it. Which families, and what happens to existing IDs, are separate
decisions.

**Why.** This repo's records are written by the plugin's own commands: a repo-only rewrite is undone
on the next run — the next `/mochiko:setup` amend writes `<!-- GI-XXX -->` bare again (F4,
`plugins/mochiko/migrations/0019-setup-agnostic-modules-out.yaml:448`) and the next brainstorm here
mints a bare "`D1`" (F3, `plugins/mochiko/migrations/0001-genesis.yaml:560`). A lookup leaves the
lookup in place, which is the problem the frame names.

**Rejected roads.** *This repo only* — drifts back to bare on the first run of the plugin's own
minting rules. *No ID change, add a lookup* (a `mochiko-cli` verb printing "`GI-004 — Primitive
Audit Ratchet`"; also the blind map's glossary and render-time-expansion steelmen, I2 b/c/e) — keeps
the lookup the frame wants gone. *Inline gloss with no grammar change* ("`GI-004 (primitive audit)`",
I2 d) — outside the user's destination line; shown to the user once, not taken up.

**Accepted risk.** The costliest road: several shipped-primitive landings (migration, strips, audits,
version bump); a crate change once session D numbers reach migration `anchor:` lines, which sit
inside the canonical hash (F9; `plugins/mochiko/migrations/README.md:58`: "`hash` | the canonical
hash of `{id, sequence, anchor, changes}`"); every user project's IDs change form, and existing
projects such as kinako carry bare IDs until the Existing-user-projects decision says otherwise.

**How it was decided.** Q1 (Reach), put with options A plugin and this repo · B this repo only · C
no ID change plus a lookup, after the blind second list (F10) — which leaned the same road — and the
blind map's added costs were shown; user: "as recommeded" — the lead's recommendation, ratified
(ratified run: 1).

**Changed at review.** —

### D2-joined-form-placement — Placement: the joined form at every single-ID mention — `Confident`

**Statement.** Wherever a single in-scope ID is cited — prose, tables, headings, the definition
site — it is written in the joined form, number then slug (`GI-019-kernel-tooling-admission`).
Compound references (ranges, slashes, lists) and machine-read spots (migration `anchor:` lines,
`<!-- GI-… -->` markers) are separate decisions.

**Why.** The definition sites already carry a name (F1); the lookup pain is at citations (F2). Every
partial rule — first mention only, other files only, definition only — leaves some reader at a bare
ID, which is the frame's problem. The joined form starts with the number, so a search for "`GI-019`"
still finds every mention.

**Rejected roads.** *First mention per document* — a later mention, or a search hit landing on it,
shows no name; moving a paragraph moves the first mention. *Other files only* — mixed forms and a
"same file?" judgment at every write. *Definition site only* — close to today (F1); the reader still
looks up at the citation. (The blind second list, F10, carried the same four plus "prose only,
machine-read spots bare", held as its own decision.)

**Accepted risk.** Longer lines: `CLAUDE.md` carries ~40 single-ID mentions (26 GI + 14 D tokens by
the F6 method); at ~25 characters each that is ~1,000 characters on an 18,425-character file
(`wc -c CLAUDE.md`), ~5%, paid by every seat on every turn. A renamed slug means a repo-wide rewrite.
Length caps press harder — skill `description` 1,536 characters, report sections 15 lines (blind map
D5-compound-reference-forms).

**How it was decided.** Q2 (Placement), put with a replay of `CLAUDE.md:72` in all four forms; user:
"yes A" — the lead's recommendation, ratified (ratified run: 2).

**Changed at review.** —

### D3-slugged-id-families — Which families: every numbered family the plugin mints, product IDs included — `Contested`

**Statement.** The joined form applies to every numbered ID family the plugin mints — governance
(`GI`), session decision cards (`D1…`), and the product families (`FEAT`, `US`, `FR`, `SC`, `NFR`,
`AX`, `EPIC`, `SCR`, `FLOW`, `DS`, `IP`, `SPN`, `INT`, `C-XXX`, `D-XXX`, and any other the plugin
mints). The complete family list, with each family's minting rule, is a fact being enumerated
(seat `fact-id-families`); where a durable/session-internal boundary falls is a separate default.
*(Extended by the user's word at D10-durable-id-boundary: this repo's own durable forms that no plugin rule mints —
"`PO-D1`", `AM-5`, `OQ-2` — are in scope too; the family list is F20.)*

**Why.** The user's pick. The frame names "things like" GI and D; product IDs are where users meet
most IDs (F5: FEAT 150, FR 69 in the plugin's own rules vs GI 79), and a partial set leaves one
project mixed — "`D7-frame-hardens-early`" in records beside a bare `D-007` in its constraints file.

**Rejected roads.** *GI and session D only* (the lead's recommendation) — test the bet on the two
named families first, widen by a later ruling; rejected by the user. *Every family that records a
ruling* (`D-XXX`, `C-XXX`, `IP-XXX`, `AM`) — the middle road; rejected by the user.

**Accepted risk.** The widest blast radius: the spec, feature-map, architecture-store and epic
grammars all change, and their skills with them. Requirement and story wording changes most often
(the blind second list's reading, F10 — not measured), so a three-word slug on `FR`/`US`/`SC` goes
stale first; Slug life carries the weight of that. The change rolls out across every family at once,
not on two first; the bet itself is tested on a sample (D8-read-test-bet).

**How it was decided.** Q3 (Which families), options A GI and session D only (lead's pick) · B every
ruling family · C every numbered family; user: "every numbered family" — against the lead's
recommendation, so `Contested` (ratified run reset to 0).

**Changed at review.** R8 (S9c), round 1 repair batch (user: "batch ok"): the accepted-risk line said
the bet "is tested across every family at once"; D8-read-test-bet samples three families — now "the change rolls out
across every family at once … the bet itself is tested on a sample (D8)". Not reopened: the reviewer
did not raise D3-slugged-id-families itself.

### D4-slug-follows-topic — Slug life: the slug follows the topic; the number in its scope is the key — `Confident`

**Statement.** When the topic of the thing an ID names changes, its slug is renamed and every
mention rewritten to the new joined form. The number within its owning scope (spec, record, or
project) is the identity: tools, searches and cross-checks match on the number in its scope
("`GI-020`" project-wide; `FR-012` inside its spec; "`D7`" inside its record), never on the full joined
string. *(Amended 2026-10-08 by the user's word — was "The number alone is the identity"; F17 showed
`FR` and session `D` numbers restart per artifact.)* Surfaces that stay
frozen keep the slug they were written with (which surfaces stay frozen is Back-fill's call).

**Why.** Today's titles already track meaning while the number holds (F13: `GI-020-plugin-install-model` went from
"Additive Plugin Install" to "Clone-Only Install with a Required `mochiko-cli` Dependency" in place).
A frozen slug turns false on the first supersession, and a false label is worse than a bare number.

**Rejected roads.** *Frozen at mint* (cheapest) — "`GI-020-additive-plugin-install`" would stand
today, false. *Frozen, a meaning change mints a new ID* ("`GI-020`" retired, `GI-024-clone-only-install`
minted) — contradicts the project's supersede-in-place practice and multiplies IDs; old citations
keep pointing at the retired one.

**Accepted risk.** Every rename is a rewrite of every mention, in this repo and in user projects —
rewording a requirement touches every spec, task list and report that cites it. A frozen surface
carries a stale slug, so any grep or check keyed on the full string misses it.

**How it was decided.** Q4 (Slug life), put with the `GI-020-plugin-install-model` replay (F13); user: "as recommded" —
the lead's recommendation, ratified (ratified run: 1). Statement amended at Q7 (the lead proposed
the scoped-identity wording after F17; user: "yes", answering the rename and the wording fix together).

**Changed at review.** S4, round 1: the key is not stable through a landing renumber — `C-`/`D-`/`IP-`/
`INT-`/`DS-XXX` numbers change when a second run lands (F18). Added, user-ruled ("as recommded", the
repair put with S4): a landing renumber re-keys the entry; its slug travels with it, and the landing
run rewrites the entry's mentions with D7-scoped-rename-command's tool. **R3 (S10)**, repair batch (user: "batch ok"): the
rename trigger reads "when the topic of the thing an ID names changes" — D13-topic-word-slugs makes slugs topic words,
so a change of strength or wording inside one topic renames nothing. **V2**, verify round 1 repair
(user: "as recommded"): R3 had not reached the title and Statement; both now say "topic".

### D5-compound-reference-forms — Compound references: ranges stay bare; slash pairs and lists of up to three are expanded — `Confident`

**Statement.** A range (`D1–D7`, `PO-D1–D7`) stays bare. A slash pair or a list of up to three IDs
("`D4/D10`", "`GI-004, GI-005`") is written as joined IDs ("`D4-route-through-amend /
D10-governance-envelope-supersessions`"). A list longer than three is treated as a range-like summary and
stays bare.

**Why.** A pair or a short list carries real citations and reads like single mentions (D2-joined-form-placement); most
ranges are wide — 730 of 807 session-D ranges span four or more IDs (F14) — and expanded they would
swamp the line.

**Rejected roads.** *All compound forms bare* (cheapest) — leaves ~2,962 slash pairs (F14, rough)
as bare as today. *Expand everything* — `D1–D23` becomes 23 joined IDs.

**Accepted risk.** A range still leaves the reader at bare numbers — the frame's problem, kept on
purpose (`DECISIONS.md` alone carries 60 range rows, F3). "Up to three" is a judgment at each write.

**How it was decided.** Q5 (Compound references), put with a replay of `DECISIONS.md:23` and
`CLAUDE.md:74` in three forms; user: "as recommeded" — the lead's recommendation, ratified (ratified
run: 2).

**Changed at review.** R4 (S11), round 1 repair batch (user: "batch ok"): the example slug
`D10-governance-envelope` (two words, against D19-slug-grammar-mechanics) now reads "`D10-governance-envelope-supersessions`".

### D6-minting-check-enforcement — Enforcement: minting rules plus an advisory `mochiko-cli` check — `Confident`

**Statement.** The skills that mint IDs write the joined form, and their paired graders check it —
the producing seat coins the slug (judgment the skills own) and the grader judges whether it fits.
On top, `mochiko-cli` gains an advisory check that lists bare in-scope IDs (ranges and lists longer
than three exempt, per D5-compound-reference-forms) and reports through its exit code only; it never gates a write or a
pipeline step.

**Why.** Bareness is mechanical and graders miss it; slug aptness is judgment and stays with the
skills. An advisory checker used as an optional exit-code signal is not kernel-class (F15,
`CLAUDE.md:72`), so it needs no admission ruling. Without some check the corpus decays back to bare
(blind map J1).

**Rejected roads.** *Rules only* — nothing mechanical catches a bare ID a grader skims past. *A
write gate* (a seventh artifact-gate check denying any write that adds a bare ID) — strongest, but
kernel-class: it would need a recorded admission ruling, and a deny on a judgment-adjacent form
risks wedging seats mid-run.

**Accepted risk.** An advisory signal can be ignored, so drift is slowed, not stopped. D3-slugged-id-families makes the
check learn every family's pattern. Catching a stale slug under D4-slug-follows-topic needs a source for the current
slug (the Rename mechanism's question).

**How it was decided.** Q6 (Enforcement), options A rules only · B rules plus an advisory check · C
rules plus a write gate; user: "as recommended" — the lead's recommendation, ratified (ratified run:
3 — the next fork goes in another form, per `brainstorm.ratified-yes`).

**Changed at review.** S7, round 1: stale-slug detection was deferred to the Rename mechanism, which
delivered a tool, not a detector (D7-scoped-rename-command). Added, user-ruled ("as recommded", lead's lean A): **the check
also flags mention drift** — a joined mention whose slug differs from its in-scope definition's —
reusing the ID-to-definition resolution D7-scoped-rename-command's tool needs (scope per F24 and D11-cross-session-qualifier). Meaning drift at the
definition stays judgment, recorded as OQ4. Rejected: no detection (a missed mention stays wrong
forever, unseen). Accepted risk: more crate code in an advisory tool.

**Changed at build.** 2026-10-08, build question B3 (`wave1-crate-ids.md`): where the family table
lives. Ruled, user ("as recommded", lead's lean A): **in crate code** — a table built from F24's facts
(prefix, padding, scope, definition site); the migration log's grammar does not change, so no grammar
bump and no lockstep release — **plus a drift test**: a crate test replays the log and fails if a
family's minting rule shows a form other than the table's. Rejected: the families as data in the log
(a new node kind, grammar 2 → 3, binary range, hooks and plugin moving together, for a table that
changes rarely). Accepted risk: a new family means a crate release.

### D7-scoped-rename-command — Rename mechanism: a scoped, preview-first `mochiko-cli` rename command, admitted as kernel-class here — `Confident`

**Statement.** `mochiko-cli` gains a rename command — shape `mochiko-cli ids rename <owning-file>
<ID> <new-slug>` — that rewrites every mention of one ID to its new joined form, keyed on the number
within its owning scope (D4-slug-follows-topic as amended): mentions inside the owning file and its scope, plus mentions
elsewhere that name that scope explicitly (a cross-session cite carrying the session slug); never the
bare number project-wide. It previews by default — prints the diff — and writes only with `--write`.
The seat supplies the new slug; the tool holds no judgment. Frozen surfaces are skipped (which ones:
Back-fill). This record carries its kernel-class admission: it never gates pipeline progress, never
dispatches or sequences agents, and holds no judgment a skill owns (the bright line, `CLAUDE.md:72`).

**Why.** A scoped apply is needed by D14-live-layer-backfill's back-fill, by D16-existing-user-projects's later opt-in pass, and by every D4-slug-follows-topic
rename after them — how often renames come is unknown (OQ2), and D13-topic-word-slugs's topic slugs cut them; one rename
can touch ~100 files (F16), mechanical work a tool does exactly and a seat does unreliably. The
scope condition exists because `FR-001` opens every spec and "`D1`" every record (F17): a project-wide
rename keyed on the bare number would corrupt silently, which is worse than a missed mention.

**Rejected roads.** *The seat rewrites by hand, guided by the D6-minting-check-enforcement checker* (the lead's weak lean,
put after three ratified rulings in a row) — no write tool and no second source, but slow and
miss-prone at ~100 files per rename. *A per-project registry file mapping number to slug* — a second
source of truth beside the definition site, which already holds the name. *A throwaway apply script
for the back-fill plus D6-minting-check-enforcement's mention-drift check* (the reviewer's cheaper shape, S14) — the apply recurs
at every rename in every project after the back-fill, and its scope resolution is the code the check
needs anyway.

**Accepted risk.** A new kernel-class write tool, admitted by this ruling: its admission still needs
its governance route (the GI-019-kernel-tooling-admission ledger trace — precedent: `hook-enforced-artifact-schema` D7-governance-routing-supersession routed
its admission through a `/mochiko:setup` amend), carried by the Governance route decision. The scope
resolution per family must be right — which families restart per artifact is in the
`fact-id-families` enumeration. A crate change means a `mochiko-cli` release and the rust-cli review.

**How it was decided.** Q7 (Rename mechanism), put in the changed form (case against the lead's pick
first, weak lean A); the user leaned B and asked the lead's view; the lead agreed with B on two
conditions (scoped, preview-first) after finding F17; user: "yes" — taken as yes to B with both
conditions and to the D4-slug-follows-topic wording fix, said in one reply to the two asks together. The user's own
pick (ratified run reset to 0).

**Changed at review.** S4, round 1: the per-family scope the accepted-risk line deferred to the
enumeration is now F24; the tool resolves scope from F24 and D11-cross-session-qualifier's qualifier as extended at review.
**R6 (S14)**, round 1 repair batch (user: "batch ok"): the Why no longer asserts frequent renames as
fact — it rests on the apply need (D14-live-layer-backfill, D16-existing-user-projects, D4-slug-follows-topic) with frequency unknown (OQ2); the reviewer's cheaper
shape is recorded under Rejected roads. **V3**, verify round 1 repair (user: "as recommded"): D4-slug-follows-topic's
landing renumber changes the number, which `ids rename` cannot do; the tool gains a re-key form,
`mochiko-cli ids rekey <owning-file> <old-ID> <new-ID>`, scoped and preview-first like rename, used by
the landing run; the slug travels with the entry.

### D8-read-test-bet — Bet test: a two-arm read test, run as wave 0 before any build — `Confident`

**Statement.** Before anything is built, the user reads two arms of real citations from this repo — a
mix of `GI`, session `D` and one product family: arm 1, 20 random bare citations; arm 2, 20 other
random citations with hand-coined slugs (no tool needed) — and marks each: knew it without a lookup ·
looked it up anyway · misled. Pass: arm 2 at least 15 "knew" and at least 5 more than arm 1, none
"misled". A fail stops the build before anything ships and returns to the user.

**Why.** It tests the frame's own problem — the reader's lookup — at about 15 minutes of the user's
time, and it sits in front of the costly part (~20k rewrites, F6).

**Rejected roads.** *Untested, watched* (cheapest) — the costly rewrite runs before the bet is
checked. *A seat eval* (seats answering from a slugged vs bare `CLAUDE.md`) — the only road that
tests the blind map's I1 risk directly, but a new eval kit and runs.

**Accepted risk.** The marks are subjective and n = 20. The blind map's I1 risk — a seat acting on
the slug's paraphrase instead of the rule, NON-NEGOTIABLE lines especially — stays untested and is
carried as an open question.

**How it was decided.** Q8 (Bet test), put with a mock of the marking sheet; user: "as recommded" —
the lead's recommendation, ratified (ratified run: 1).

**Changed at review.** S9 (a, b), round 1: with no bare baseline, "15 of 20 knew" cannot show the slug
helped — and the reader wrote the IDs; and the test fenced only the mass rewrite, after the crate, the
rules for every family and the templates had shipped. Changed, user-ruled ("as recommded", lead's lean
A): **two arms, run as wave 0 before any build** — arm 1, 20 random bare citations; arm 2, 20 other
random citations with hand-coined slugs (no tool needed); the user marks each knew / looked up anyway
/ misled. Pass: arm 2 at least 15 "knew" and at least 5 more than arm 1, none "misled". A fail stops the
build before anything ships. Rejected: keeping it as ruled (a fail lands after the build has shipped);
moving it earlier without a baseline. Accepted risk: about 30 minutes of the user's time; the reader is
still the IDs' author, which the baseline narrows but does not remove. **W3**, verify round 2 repair
(lead-found, same class as the reviewer's W2; user: "as recommded"): S9's change carried into the title
and Statement — they had kept the one-arm test before the mass rewrite.

**Changed at build.** 2026-10-08, wave 0, with arm 1 put to the user: the user waived their own
marking — "i am okay for you to implement without testing involving me". The user-marked test does
not run; the user chose a cold-seat replacement ("a, ok, 1-3"): a fresh reader seat guesses from the
IDs alone, a second fresh seat scores against the true titles, D8-read-test-bet's pass rule unchanged. Result
(`wave0-read-test.md`): **FAIL on clause 1** — the slugged arm 7 "knew" against a bar of 15; gain 7;
none misled; the slug carried the right topic on 19 of 20 joined IDs against 0 of 20 bare; one topic
slug on a removal ruling read inverted (item 38). The build stopped and returned to the user.
**The user overrode the failed gate**, on the lead's lean A ("as recommded"): the build proceeds; the
bet is restated from "skip the lookup" to "tell the reader what an ID is about" — the part the test
showed working (19 of 20 against 0 of 20) — and D13-topic-word-slugs gains direction words for removal rulings. The
original bet stands recorded as failed by a cold reader, not as passed.

### D9-number-padding-form — Number form: each family keeps the padding its minting rule states — `Confident`

**Statement.** The number part of the joined form keeps today's padding per family: three digits for
the `-XXX` families (`GI-004-…`, `FR-012-…`, `C-001-…`); unpadded for `US-<n>`, session `D<n>` and
cycle `C<n>`. `SC`'s mixed examples settle on three digits.

**Why.** "Keeps its number" (frame, destination line); the user's `GI-01` read as an illustration of
the format. `SC` already has a three-digit validator (`validate-requirements.py:84`, F21) while its
template examples show `SC-1` (F20).

**Rejected roads.** Two-digit padding per the user's example — renumbers every existing ID for no
reader gain.

**Accepted risk.** `SC`'s template examples change.

**How it was decided.** A default, first shown 2026-10-08 after the frame hardened, restated after
F20 landed; confirmed in a batch before Joined-form grammar; user: "default ok".

**Changed at review.** —

### D10-durable-id-boundary — Family boundary: durable IDs take the slug, local working labels stay bare — `Confident`

**Statement.** The test is durability: an ID cited outside the file that minted it takes the joined
form. In scope: F20's 21 durable plugin forms — which include session `D` cards and cycle cards
(`C1-walking-skeleton-path`) — and this repo's own durable forms that no plugin rule mints ("`PO-D1`",
`AM-5`, `OQ-2`, `J-1`, `FP-1`, census IDs where cited outside their strip). Bare: build tasks `T3.2`, gates `C3-gate-2`, run folders, report
labels `G1`/`C1`/`A1`/`F1`, survivors `S1`, a record's `F`/`Q`/`S`/`V`. File names that already
carry a slug keep it.

**Why.** The frame's problem is the cold reader meeting an ID away from where it is defined; a label
read only inside its own file sits beside its definition.

**Rejected roads.** Slug every numbered label — this record's `F17` would read
`F17-numbers-restart-per-artifact` at every one of its citations, inside the same file.

**Accepted risk.** "Cited outside the file" is a judgment at mint time for borderline labels (a
report `G3` later cited in a spec).

**How it was decided.** A default, shown after D3-slugged-id-families, restated after F20; the repo-only extension was
named to the user as needing their word because it reaches past D3-slugged-id-families's "plugin mints" wording; user:
"default ok" — taken as confirming the extension too.

**Changed at review.** Round 1 repair batch (user: "batch ok, R5 yes"): **R4 (S11)** — the example
`C1-walking-skeleton` (two words) now reads `C1-walking-skeleton-path`; **R7 (S15)** — the in-scope
line no longer counts session `D` cards and cycles twice (they are among F20's 21 forms), and the
repo-only examples add `J-1`, `FP-1`; **R5 (S13)** — the repo-only extension, first taken from a batch
"default ok", is now confirmed by the user's explicit word ("R5 yes"), so `Confident` stands.
S6, round 1: cycle cards (in scope, `C1-walking-skeleton-path`) and report
clarification labels (bare; `plugins/mochiko/templates/advocate-report-template.md:29`: "### C1:
{{question_title}}") share the form `C<n>`, so no mechanical check can tell them apart. Added,
user-ruled ("as recommded", lead's lean A): **the report clarification label is renamed `C<n>` →
`Q<n>`** (`### Q1: {{question_title}}`), free in shipped primitives outside examples; cycles keep `C`.
Rejected: telling them apart by location (ambiguous for a cycle cited inside a report); dropping cycles
from the slug set (narrows D3-slugged-id-families). Accepted risk: a shipped-template edit, with its strip and audit, and
a label seats already know changes.

**Changed at build.** 2026-10-08, build question B2 (`wave1-crate-ids.md`): no plugin rule mints this
repo's own forms, so the crate has no definition site for them — session-prefixed decisions ("`PO-D1`",
"`TC-D4`, `OO-D3`" and nine more prefixes; 422 live mentions), `AM-n` (276), `J-n` (56), `FP-n` (39),
`OQ-n` (24), strip census IDs (~130). Ruled, user ("as recommded", lead's weak lean B): **session-
prefixed decisions are normalized** at back-fill to the D11-cross-session-qualifier form — "`PO-D1`" becomes
`` `production-only-focus` D1-<slug> `` — so the crate needs no code for them, and D15-protected-line-rewrites's diff check
accepts that qualifier change as an ID-token change; **the other repo-only forms are slugged once** from
D14-live-layer-backfill's map and then sit on D18-build-done-check's allowlist, unchecked. Named to the user before the ruling: the
normalization changes how ~420 mentions look, not only adds a slug — close to the frame's out-of-scope
"renaming the prefixes (`GI`, `D`)", which does not list these shorthands. Rejected: a declared
repo file the crate reads (a new hand-kept home for forms no plugin rule mints); slugging every
repo-only form once with nothing checked. Accepted risk: `AM`/`OQ`/`J`/`FP`/census slugs can drift
unseen.

### D11-cross-session-qualifier — Cross-session qualifier: a cite of another session's decision keeps the session slug — `Confident`

**Statement.** A citation of a decision from another session carries that session's slug in front:
`` `cli-schema-delivery` D11-widened-kernel-admission ``.

**Why.** Session `D` numbers restart per record (F3, F17), and slugs can repeat across sessions (blind
map C5), so neither the number nor the slug alone identifies the decision; D7-scoped-rename-command's scoped rename needs
the scope named.

**Rejected roads.** Dropping the qualifier once the slug is present — "`D7-frame-hardens-early`" could
exist in two records.

**Accepted risk.** Longer cross-session citations.

**How it was decided.** The lead first stated it to the user as following from D2-joined-form-placement without a
question (corrected, map change after D5-compound-reference-forms); shown as a default; user: "default ok".

**Changed at review.** S4, round 1: the qualifier extends from session `D` to every family whose number
repeats per artifact (F24: `FR`, `SC`, `US`, `SCR`, `FLOW`, `C`/`D`/`IP`/`INT`/`DS`, `BR`, cycles) —
cited outside its owning file, the ID carries its owner's name in front (`` `lunch-orders`
FR-012-csv-report-export `` in a `tasks.md`); project-wide families (`GI`, `FEAT`, `EPIC`, `AX`, `SPN`,
`NFR`, `GAP`) need none. User-ruled ("as recommded", lead's lean A). Rejected: inferring the owner from
the citing file's folder — wrong as soon as a feature has two specs (`feature-entry.yaml:67`), and a
wrong guess silently rewrites another spec. Accepted risk: longer citations in `tasks.md`, cycle
reports and feature entries.

**Changed at build.** 2026-10-08, build question B1 (`wave1-crate-ids.md`): `DECISIONS.md`, the
brainstorms index, `BACKLOG.md` and `ROADMAP.md` carry bare `D<n>` on 550 lines, nearly all inside a
row or entry that already names one session. Added, user-ruled ("as recommded", lead's lean A): **the
line names the owner** — a bare session `D<n>` in a row or entry that names exactly one session (a
link to its record, or a leading `` `slug` `` qualifier) belongs to that session, and the tool and
the check resolve it so; an explicit qualifier is owed only where the line names none or several.
Rejected: the literal qualifier on every mention (rows grow by about a third, repeating a name the
row shows). Accepted risk: a second rule beside D11-cross-session-qualifier's, applied line by line.
**Narrowed at build**, 2026-10-08, after wave 1's code re-review failed twice on it (C2: a qualifier
mid-line, on the brainstorms index's "Landed" lines, made the tool give a session's bare `D<n>` to the
superseding session — `index.md:39` and `:58` reproduced; the diff check cannot see an owner error).
Put to the user with the case against each road (A another fix round on the leading-qualifier rule ·
B re-staff · C narrow); the lead's lean moved from A to C once a count showed C's cost. Ruled, user:
"yes go with C" — **only a link to its record names a line's owner**; a leading `` `slug` `` qualifier
no longer does. Every link and qualifier on the line still counts toward "names several", which leaves
the line's bare `D<n>` unresolved. A mention's own qualifier is untouched. Accepted risk: about 7 bare
`D<n>` in 6 lines (the lead's rough script, not the tool) that only a leading qualifier would have
owned stay bare after back-fill.
**Changed at build**, 2026-10-08, at wave 3's shard plans (W3-C Q1): 343 mentions in 86 files, most
of them strips, carry their own qualifier as a code-span path to the owner's record
(`` `.mochiko/brainstorms/<slug>/record.md` D3 ``). The tool read that path as a qualifier naming no
session, so neither rename nor check reached them. Put to the user with the case against each road
(A leave them bare, under D18-build-done-check as narrowed · C a small crate change, built while the map is drafted);
user: "as recommended" (the lead's lean C). Added: **a mention's code-span qualifier that is the path
of a session's `record.md` names that session**, as its slug would; a rename rewrites the ID token
and leaves the path as written. Built as wave 1c under the sound loop; if its code review fails
twice, the fallback is A. Rejected: A — the 343 cites never take a slug; hand-editing them — 343
edits outside the diff check. Accepted risk: one more qualifier shape the tool reads.

### D12-machine-read-spots — Machine-read spots: every spot a program reads keeps the bare number — `Confident`

**Statement.** Migration `anchor:` lines, `<!-- GI-… -->` markers, and number-only paths
(`.mochiko/features/FEAT-001/`, `.mochiko/epics/EPIC-002/`, `stories/US-12.md`) keep the bare number
— a carve-out from D2-joined-form-placement. File names that already carry a slug (`FEAT-001-<slug>.md`, `AX-002-<slug>.md`,
`scr-001-<slug>.html`) keep it, and that slug is the ID's slug, so a D7-scoped-rename-command rename moves the file. No
crate grammar change for anchors. Parsers that read prose citations — the D6-minting-check-enforcement checker, the plugin's
scripts (F21) — accept the joined form wherever prose carries it.

**Why.** A hashed anchor can never follow a D4-slug-follows-topic rename (F9; `plugins/mochiko/migrations/README.md:80`:
"The hash covers the `anchor:`, which is the evidence that protected content left by ruling"); a
slugged path moves on every rename; a marker sits beside the principle's own text. Under D4-slug-follows-topic the number
in its scope is the key, and the anchor already names the session.

**Rejected roads.** *Split by kind* (hashed history and paths bare, live markers joined; the second
blind list's road, F19) — the markers sit beside the full principle text, so the slug adds little and
churns at every rename. *Joined everywhere going forward* (crate accepts `D9-slug` in new anchors,
slugged directories and story files) — a grammar bump, and every rename moves paths. *Rewrite and
re-stamp every migration* — voids the hash's evidence; not offered.

**Accepted risk.** A maintainer reading a migration still meets "`cli-schema-delivery D9`" bare and
looks it up — the frame's problem, kept for that reader. A seat reading raw `CLAUDE.md` sees a bare
marker (beside the principle's text).

**How it was decided.** Q9 (Machine-read spots), put with a three-way mock (anchor · marker · story
file); user: "as recommded" — the lead's recommendation, ratified (ratified run: 3, counting the
defaults batch as one — the next fork goes in another form).

**Changed at review.** —

### D13-topic-word-slugs — Joined-form grammar: the slug names the topic, never the rule — `Confident`

**Statement.** A slug names what the ID is about, not what it demands: no strength or verdict words
(`must`, `should`, `never`, `may`). `GI-003-repo-secret-hygiene`, not "`GI-003-must-gitignore-secrets`".
The mechanical grammar (pattern, word rules, sub-ID letters, slug source) rides as a default for the
wrap-up batch (map change after D12-machine-read-spots).

**Why.** A topic slug cannot be acted on as a rule, so the reader still opens the line for the
obligation — it narrows D8-read-test-bet's open risk (a seat acting on the slug's paraphrase, blind map I1,
NON-NEGOTIABLE lines most). A topic outlives a change of strength or direction, so it renames less
under D4-slug-follows-topic (`GI-020-plugin-install-model` survives the v3.0.0 flip that made
"`GI-020-additive-plugin-install`" false, F13).

**Rejected roads.** *Rule slugs* — the user's own example style (`GI-01-must-use-logging`); more
meaning at a glance, but an actionable paraphrase and more renames. *No rule, the minting seat
picks* — mixed styles, nothing for the grader to check.

**Accepted risk.** A topic slug saves less of the lookup than a rule slug — the D8-read-test-bet read test measures
this; `repo-secret-hygiene` does not say what is required. The user's own example now reads
differently (`GI-01-logging-…`).

**How it was decided.** Q10 (Joined-form grammar), put in the changed form — case against the lead's
pick first, a weak lean, a mock of three real GI IDs both ways; user: "as recommded" — the lead's weak
lean, ratified (ratified run: 4; forks keep the changed form).

**Changed at review.** —

**Changed at build.** 2026-10-08, after wave 0's FAIL (D8-read-test-bet): a topic slug on a removal ruling read
inverted — "`` `setup-product-agnostic` D1-setup-fact-profile ``" taken as "setup captures the fact
profile" when the ruling is that it leaves setup (`wave0-read-test.md`, item 38). Added, user-ruled
("as recommded", lead's lean A): **a ruling that removes, retires, moves out or declines something
carries its direction word** — "`D1-profile-leaves-setup`", `GI-022-no-feature-map` — still never a
strength word (`must`, `should`, `never`, `may`). Rejected: rule slugs throughout, re-tested (B — trades
the risk D13-topic-word-slugs guards against for a better score); stopping the build (C). *Repaired 2026-10-08 (P1's
plan grade, F5; user: "ok of d13"): the first example, `D1-fact-profile-leaves-setup`, had four words
against D19-slug-grammar-mechanics's three — a direction word counts toward the three.*

### D14-live-layer-backfill — Back-fill: rewrite the live layer, freeze the history layer, from one graded slug map — `Confident`

**Statement.** Existing IDs and their citations in this repo's live layer are rewritten to the joined
form: `CLAUDE.md`, `DECISIONS.md` / `BACKLOG.md` / `ROADMAP.md`, the governance ledger and
`governance-intent.md`, the rules files, every session record (cards and citations), strips,
`.mochiko/decisions/`, plugin prose, crate comments and crate tests outside the replay fixture corpus.
The history layer stays as written: `.mochiko/archive/`, the crate's replay fixture corpus
(`crates/mochiko-cli/tests/fixtures/genesis-corpus/`), eval stimuli (fixture inputs and recorded runs),
released `CHANGELOG.md` entries, and the migration log with its replayed views (bare by D12-machine-read-spots and D21-log-rule-citations).
Eval expectations that assert the plugin's minted form change with the plugin build, not with the
back-fill. Method, in order: D8-read-test-bet's two-arm read test runs first, as wave 0 before any build, on
hand-coined slugs; then a producing seat drafts one slug map keyed by family, scope and number; a
second seat grades it; D7-scoped-rename-command's rename tool applies the map one ID at a time.

**Why.** The frame's problem lives in the live layer — ~19k GI and D mentions there against 615 in the
archive (F22); the archive is frozen by rule (F8) and run captures are recorded evidence (F19). One
graded map keeps coining (judgment, the seats') apart from applying (mechanical, D7-scoped-rename-command's tool).

**Rejected roads.** *New IDs only* (cheapest) — leaves the existing records, where the problem lives,
untouched; two forms forever. *Rewrite everything, history too* — breaks the archive-frozen rule and
alters recorded evidence. *At touch time* — never finishes. (The second blind list, F19, carried the
same four and leaned the same road and method.)

**Accepted risk.** The largest one-shot job: at least 375 decision slugs (F22, a floor) plus every
other family, ~19k mentions, and the strip-plus-audit ritual for every plugin primitive edited. A
reader of the history layer still meets bare IDs. The repo is mixed while the build runs.

**How it was decided.** Q11 (Back-fill), put in the changed form — case against the lead's lean first,
a layer diagram; user: "as recommended" — ratified (ratified run: 5).

**Changed at review.** Round 1, repair batch (user: "batch ok"): **R1 (S3)** — the history layer adds
the crate's replay fixture corpus `crates/mochiko-cli/tests/fixtures/genesis-corpus/` (21 files carry
IDs; `crates/mochiko-cli/tests/fidelity.rs:48` reads it as the frozen expected side, so rewriting it
turns `cargo test` red) and eval stimuli (fixture inputs and recorded runs); eval expectations that
assert the plugin's minted form change with the plugin build, not with the back-fill; "eval fixtures"
leaves the live list. With D21-log-rule-citations, the history layer's "migration log" covers its rule text and the
replayed views `.mochiko/schema-views/`. **R2 (S9)** — the method's order changes: D8-read-test-bet's read test runs
first, as wave 0 before any build, on hand-coined slugs; the slug map is drafted and graded after it.
**V2**, verify round 1 repair: the Statement's "(already bare by D12)" now reads "(bare by D12 and
D21)". **W2**, verify round 2 repair (user: "as recommded"): R1 and R2 carried into the Statement
itself — the history layer's full list, and D8-read-test-bet first in the method order.

**Changed at build.** 2026-10-08, W2-tests' plan grade (user: "as recommded"): the history layer adds
the crate's template goldens `crates/mochiko-cli/tests/fixtures/template/`. They are renders of the
migration log, compared byte for byte by crate tests, so their text follows the bare log (D21-log-rule-citations):
rewriting them turns `cargo test` red, and checking them reports the log's minted examples as drift
against this repo's own definitions. The exclusion lands in the check's default list with wave 1's
fix round 3.

### D15-protected-line-rewrites — Protected lines: an ID-token-only rewrite is ruled non-semantic, behind a mandatory diff check — `Confident`

**Statement.** A rewrite that changes only ID tokens — same number, a slug added or renamed — is not
protected content leaving (`CLAUDE.md:90`). This record is the ruling that covers it for `KEPT:`
lines, `DECISIONS.md`-traceable lines and records' protected sets, so no per-line supersession entry
is written. Condition: D7-scoped-rename-command's tool checks every apply — any changed span that is not an ID token blocks
that apply — and a grader confirms the check ran. Shipped plugin primitives the back-fill edits still
take the normal per-primitive audit (GI-004-primitive-audit-ratchet, untouched).

**Why.** The line's content does not leave; only its pointer gains a name. Per-line strip entries
would record nothing a reader needs and bury the real supersessions.

**Rejected roads.** *A supersession per protected line* — hundreds of strip entries. *Protected lines
stay bare* — bare IDs in the most important lines.

**Accepted risk.** GI-005-record-layer-integrity is NON-NEGOTIABLE (`CLAUDE.md:115`: "The record layer MUST NOT silently
corrupt"), and a bulk rewrite under one ruling is the shape the preserved-responsibilities check
exists to catch; the diff check is the only guard against a tool bug riding this ruling. This ruling
also needs its governance route alongside D7-scoped-rename-command's admission (Governance route).

**How it was decided.** Q12 (Protected lines), put in the changed form — case against the lead's lean
first, a before/after of one `KEPT:` line; user: "as recommded" — ratified, with the lean's condition
(ratified run: 6).

**Changed at review.** S1 (Critical), round 1: the ruling's premise — "the line's content does not
leave" — fails inside verbatim quotes, whose source may stay bare (migration log, archive, git output,
the user's words; 213 quoted spans in records and strips). Added, user-ruled ("as recommded", lead's
lean A): **a verbatim quote keeps its source's form** — text inside `"…"`, a `>` blockquote or a code
fence is never rewritten; D7-scoped-rename-command's tool skips those spans, and the D6-minting-check-enforcement check and D18-build-done-check's allowlist name them.
Rejected: a quote follows its source (a per-quote judgment in a tool that holds none); a supersession
per rewritten quote. Accepted risk: a reader meets bare IDs inside quotes even where the quoted source
is now joined. **V5**, verify round 1 builder note (user: "as recommded"): quotes wrap across lines in
the markdown source (F9's own quote breaks after "anchor:"), so the tool's skip matches a quoted span
across line breaks. **V3**, verify round 1: a landing renumber (D4-slug-follows-topic) changes the number, so it falls
outside this ruling, which covers same-number rewrites only; it rewrites only the landing run's own
new entries (the lead's reading — such entries are not yet cited as protected lines).

### D16-existing-user-projects — Existing user projects: both forms accepted; an opt-in pass waits on D14-live-layer-backfill's proof — `Confident`

**Statement.** On upgrade, a project that already holds bare IDs keeps them; every new mint takes the
joined form. Graders accept a bare ID that predates the upgrade, and the D6-minting-check-enforcement check lists it as bare
(advisory). An opt-in back-fill pass for user projects — D14-live-layer-backfill's method packaged (a slug map drafted by
a seat, graded, applied by D7-scoped-rename-command's tool) — goes to the backlog, triggered once D14-live-layer-backfill's run on this repo
has proven the method.

**Why.** Nothing migrates a project today and an upgrade opens no amend (F11, F12); proving the
method here before shipping it as a primitive avoids building a producer-grader step on an untested
method. The second blind list leaned the same (F19).

**Rejected roads.** *Ship the opt-in pass now* — a new shipped step with its own drafting seat and
grader before the method is proven once. *Upgrade at touch time* — mixed forever. *Forced migration
on upgrade* — blocks every command until a pass runs; contradicts the upgrade-opens-no-amend posture;
not offered.

**Accepted risk.** Projects such as kinako stay mixed until the pass ships, and the D6-minting-check-enforcement check keeps
listing their old bare IDs — advisory noise until then.

**How it was decided.** Q13 (Existing user projects), put with no firm pick after six ratified
rulings in a row (named to the user) — a weak lean, cases against both A and B, a mock of kinako
after upgrade; user: "as recommded" — the weak lean, ratified (ratified run: 7).

**Changed at review.** S8, round 1: "a bare ID that predates the upgrade" is untestable — an ID carries
no mint date — and how new work cites an old bare ID was unruled. Added, user-ruled ("as recommded",
lead's lean A): **the definition decides** — a bare definition makes bare citations of it acceptable;
once the definition is joined, citations must be joined; new work cites an old bare ID bare and never
coins a slug for it (D7-scoped-rename-command and D14-live-layer-backfill keep one slug source). The D6-minting-check-enforcement check applies this through the same
ID-to-definition resolution S7 added. Rejected: dating IDs from git history (shallow clones, moves and
rebases break it); the citing seat coining a slug (a second slug source). Accepted risk: in user
projects, new work citing old IDs stays bare until the opt-in pass (OQ3), so the mix spreads.

### D17-governance-amend-route — Governance route: the tool converts every live file, then one setup amend records the events — `Confident`

**Statement.** D7-scoped-rename-command's tool converts the setup-owned files — `governance-intent.md`, the ledger, the
`CLAUDE.md` governance region, the rules files — with the rest of the live layer, under D15-protected-line-rewrites, from
D14-live-layer-backfill's one slug map. Then one `/mochiko:setup` amend run records the two governance events on the
ledger — D7-scoped-rename-command's tool as a kernel-class admission under GI-019-kernel-tooling-admission, D15-protected-line-rewrites's ruling under GI-005-record-layer-integrity — and bumps the
region's semver. The plugin's setup template and rules mint the joined form (D1-slug-form-reach), so the amend's
regeneration changes no ID form.

**Why.** One slug source (D14-live-layer-backfill's graded map) for every file; governance events go through setup as both
earlier GI-019-kernel-tooling-admission widenings did (F23), and "a principle redefined" opens an amend (F12).

**Rejected roads.** *The amend converts the setup-owned files itself* — setup would coin its own GI
slugs beside D14-live-layer-backfill's map: two sources for the same slugs. *No amend, hand-added ledger notes* —
cheapest; breaks the F23 precedent for kernel-class admissions.

**Accepted risk.** A tool edits files setup owns (F11: "regenerated whole"); if the tool's output and
setup's template disagree, the next amend silently undoes the tool's work — the amend run here is the
first test that they agree.

**How it was decided.** Q14 (Governance route), put with a weak lean and the cases against A and B;
user: "as recommded" — the weak lean, ratified (ratified run: 8).

**Changed at review.** —

**Changed at build.** 2026-10-09, at wave 4. The user asked whether the lead could make the amend's
edits instead of a setup run. Put with the case against first (the F23 precedent broken; the
regeneration test this card relied on deferred) and a weak lean to the hand edit, on condition that
wave 4's CI step lands; user: "Hand edit by lead". **The two events and the version bump are
recorded by hand by the lead**: the ledger, the `CLAUDE.md` version line and GI-019-kernel-tooling-admission pointers,
`.claude/rules/mochiko/rust-cli.md`'s admitted-by sentence and bright line (added at review: that
touch-time rule still read "three recorded rulings" and a tool that only renders, replays and
validates), and an entry in `governance-intent.md`, as AM-6-id-tool-admission (v3.3.0). A fresh
non-author seat reviews them, and the user ratifies the reviewed text. Rejected: the `/mochiko:setup` amend in a new session on the working-tree plugin
(this card's road), which costs a session, setup's questions and its review round. Accepted risk:
this card's own — setup's template and the back-fill may disagree. That is now first tested at the
next setup amend, where the `ids --check` CI step reports any ID the regeneration unjoins, as a
report, never a block. The trace summary is not regenerated: the v3.1.1 lead-PATCH precedent
carried no manifest.

### D18-build-done-check — Done check: one clean run of the D6-minting-check-enforcement check closes the build; CI reports drift afterwards — `Confident`

**Statement.** The build is done when `mochiko-cli ids --check` exits clean over the live layer —
zero bare in-scope IDs, and no joined mention whose slug differs from its definition's (D6-minting-check-enforcement as changed
at review), outside the named allowlist: ranges and lists longer than three (D5-compound-reference-forms), local labels (D10-durable-id-boundary),
machine-read spots (D12-machine-read-spots), the history layer incl. the migration log's rule text and the replayed views
(D14-live-layer-backfill, D21-log-rule-citations), verbatim quotes (D15-protected-line-rewrites), and bare citations of a bare definition (D16-existing-user-projects) — and D8-read-test-bet's read test
has passed and D15-protected-line-rewrites's diff check is logged for every apply. The build's gate grader runs it. After landing, the same
check runs on every pull request in CI, reporting only, never blocking.

**Why.** A measurable end with a named check and a named runner (blind map J2); drift stays visible
after landing without turning D6-minting-check-enforcement's advisory tool into a gate.

**Rejected roads.** *A one-time check only* — nothing watches after landing. *A release gate* (the
check in the contract suite's deterministic set, a bare ID blocking the next `plugin.json` bump) —
turns D6-minting-check-enforcement's advisory tool into a blocker, against D6-minting-check-enforcement's ruling.

**Accepted risk.** A CI report can be ignored like any advisory signal (D6-minting-check-enforcement's accepted risk, unchanged).

**How it was decided.** Q15 (Done check), put with a weak lean and the cases against A and C, a mock
of the check's output; user: "as recommded" — the weak lean, ratified (ratified run: 9).

**Changed at review.** S1, round 1: the allowlist gains verbatim quotes — `"…"`, `>` blockquotes, code
fences (ruled at D15-protected-line-rewrites). R1 (S3) and D21-log-rule-citations, round 1: the allowlist also names the crate's replay fixture
corpus, eval stimuli, the migration log's rule text and `.mochiko/schema-views/`; and, per S7 (D6-minting-check-enforcement), the
check also reports mention drift. W3-class carry, verify round 2 (user: "as recommded"): the Statement
now lists the full allowlist itself, adding D16-existing-user-projects's bare-definition rule.

**Changed at build.** 2026-10-08, at P1's plan re-grade (advisories A1, A4, A5): in this repo about
2,000 citations can never be tied to a definition — ~1,700 product-family IDs with no definition in
the live tree (another project's, `kinako FEAT-002`; eval-fixture IDs; examples), plain words before a
decision number (`user-ruled D4`), and the bare `C<n>` labels the lead had ruled local (R9). D16-existing-user-projects
forbids coining a slug for them, so "zero bare in-scope IDs" could never be met. Ruled, user ("as
recommded", the lead's weak lean A, put with the case against first): **the check reports only
mentions it can tie to a definition** — an ID with no definition in the indexed tree, or a
per-artifact or session mention with no resolvable owner, is never reported and never rewritten; the
done check is clean over the rest. Rejected: B, report them and count a recorded leftover as done (a
CI report that always shows ~2,000 findings gets ignored). Accepted risk: a real bare citation whose
definition the tool cannot find goes unreported; the D11-cross-session-qualifier shorthand session cites (`adaptive-depth
D7`) leave the check's view and are found by wave 3's slug map instead.
2026-10-09, at wave 3's apply, the read-test clause put to the user: D8-read-test-bet's read test
failed on a cold reader and the user overrode the gate (D8-read-test-bet, changed at build). Ruled,
user ("as recommended", the lead's lean A, put with the case against first): **the clause is met by
the user's override** — the done check reads "D8-read-test-bet's read test has passed" as "D8-read-test-bet's
gate is cleared", and the test stays recorded as failed. Rejected: B, a new cold read test on the
final slugs (a seat run and a pass rule set after the first result was seen); C, striking the clause
(hides that the test was ever part of the done check). Accepted risk: "passed" now reads as
"cleared", which this entry states plainly.

### D19-slug-grammar-mechanics — Grammar mechanics: `<ID>-<w1>-<w2>-<w3>`, three lowercase words — `Confident`

**Statement.** The joined form is the ID (its padding per D9-number-padding-form, a sub-ID letter attached to the number,
`D2a-…`) followed by exactly three lowercase ASCII words joined by hyphens, each word starting with a
letter. The slug is coined from the definition's existing name, or — where there is none (`FR`, `SC`,
`EPIC`, `GAP`, `BR`, F20) — from the line's text, under D13-topic-word-slugs's topic rule.

**Why.** The user's "3 word slug sperated by hypen"; a word that starts with a letter keeps the
boundary between number and slug unambiguous (blind map B2) — `C3-gate-2` (a local gate label, D10-durable-id-boundary)
fails the pattern and is never read as a cycle's joined form.

**Rejected roads.** "Up to three" words — variable shape for the D6-minting-check-enforcement check to parse; digits allowed to
lead a word — `D2-2fa-…` blurs the number boundary.

**Accepted risk.** Some names will not compress to three words cleanly ("Clone-Only Install with a
Required `mochiko-cli` Dependency").

**How it was decided.** A default, shown on the map after D12-machine-read-spots and in the defaults batch on the accept
screen; user: "screen ok".

**Changed at review.** S5, round 1: D12-machine-read-spots makes an existing file-name slug the ID's slug, and real ones are
one or two words (`FEAT-003-wallet.md`, `FEAT-001-family-accounts.md`, `scr-001-week-menu.html`, under
`evals/review-specifications/fixtures/g3-lunch-orders-ux-manifest/`). Added, user-ruled ("as
recommded", lead's lean A): **existing file-name slugs are kept** — a file's current slug stays the
ID's slug whatever its length; the three-word rule binds new mints only; prefix matching ignores case
(`scr-001` is `SCR-001`); the D6-minting-check-enforcement check accepts a short slug that matches the ID's file name. Rejected:
re-slugging the files (moves linked files, mostly eval fixtures S3 freezes); a separate file handle
beside a prose slug (two slugs per ID, against D7-scoped-rename-command). Accepted risk: a permanent exception to the
three-word rule.

### D20-minor-version-bump — Version bump: an ordinary minor bump, the break named in the changelog — `Confident`

**Statement.** The build ships as an ordinary minor `plugin.json` bump under 0.x semantics, with the
breaking ID-form change named in `CHANGELOG.md`; the governance region's own semver is bumped by D17-governance-amend-route's
amend run.

**Why.** The plugin is pre-1.0 (`0.117.0`); the frame owes no compatibility (blind map G3 asked that
the version semantics be stated).

**Rejected roads.** A 1.0 or major-style bump for this change alone.

**Accepted risk.** A consumer reading only the version number will not see the break; the changelog
carries it.

**How it was decided.** A default, shown on the map after D16-existing-user-projects and in the defaults batch on the accept
screen; user: "screen ok".

**Changed at review.** —

### D21-log-rule-citations — Rule text in the migration log: every citation stays bare, old and new — `Confident`

**Statement.** Rule text in the migration log cites IDs bare — the text already there and any rule a
later migration writes or supersedes — as anchors do (D12-machine-read-spots). D14-live-layer-backfill's history layer and D18-build-done-check's allowlist
name the migration log and the replayed views (`.mochiko/schema-views/`). It binds citations of decided
IDs only: minting grammar, placeholders, template definition lines and example IDs in rule text follow
D1-slug-form-reach, D9-number-padding-form and D19-slug-grammar-mechanics — the joined form — so seats copying a template mint joined IDs. *(Amended at verify
round 1, V1 — first ruled: new rule text written joined; scope clause added at verify round 2, W1.)*

**Why.** The log is hashed history; changing a rule's text goes only through a new migration (blind
map E2). A joined citation written into it can never follow a D4-slug-follows-topic rename — D7-scoped-rename-command's tool skips frozen
surfaces — so it would turn false, the case D4-slug-follows-topic calls worse than a bare number and the reason D12-machine-read-spots keeps
anchors bare. Rewording every citing rule now would be one large migration plus an audit of every
primitive it touches, for a change of form alone.

**Rejected roads.** One migration rewording all 140 citing lines (lead's count: non-anchor `D<n>` lines
in `plugins/mochiko/migrations/*.yaml`) through the log's normal supersession path — rendered rules
fully joined, at that cost. *New rule text joined, with a reword migration shipped at every rename of
an ID cited in rule text and the drift check reading the views* (V1's option b) — an extra migration
and audit per rename.

**Accepted risk.** Seats reading rendered rules every run meet bare citations there for good (the
reviewer counts 80 citing lines in the views), e.g. "never tiered down (model-tiered-seats D5;" — the
frame's problem, kept for that reader.

**How it was decided.** Born from coverage survivor S2 (blind map E2) at review round 1; the user ruled
the path "rule inline" and the lead's proposed ruling in one reply ("as recommded"). Amended at verify
round 1 (V1, blocking): the user ruled option (a) on the lead's weak lean ("as recommded").

**Changed at review.** V1 (blocking), verify round 1: the first ruling wrote new rule text joined into
the immutable log; the first D4-slug-follows-topic rename of an ID cited there would leave a false slug in rules seats
read every run, hidden by D18-build-done-check's allowlist, and it contradicted D12-machine-read-spots's own reason while citing D12-machine-read-spots.
Amended to (a): all log rule text cites bare, old and new. **W1**, verify round 2 repair (user: "as
recommded"): read literally, (a) would keep the minting templates bare too ("`- **GI-001 — Type:** …`",
`.mochiko/schema-views/templates/governance-intent.yaml:63`; example IDs `SC-1, SC-2`,
`.mochiko/schema-views/templates/spec.yaml:220`), defeating D1-slug-form-reach and D2-joined-form-placement; the scope clause limits D21-log-rule-citations to
citations of decided IDs, as its Why intended.

### D22-sub-id-forms — Sub-ID forms: a dotted sub-decision takes its own slug; a clause pointer is not an ID — `Confident`

**Statement.** A dotted sub-ID that is its own decision takes its own slug after the full ID
(`` `<session-slug>` D4.1-waiver-revisit-timing `` when cited from outside its record, per D11-cross-session-qualifier), as a
lettered one does (D19-slug-grammar-mechanics, `D2a-…`). A bracketed clause pointer
such as `(ii)` is not an ID: cite the parent in the joined form, then the clause
(`` `delta-files-vs-direct-baseline-edits` D5-<slug> (ii) ``).

**Why.** Real forms in live text were unruled (blind map A5): "Waivers are permanent pending the D4.1
revisit" in the governance-intent template, and "`delta-files-vs-direct-baseline-edits D5(ii)`" in the
replayed rules.

**Rejected roads.** A slug on every clause pointer — a clause is a part of one decision's text, not a
minted thing.

**Accepted risk.** Telling a sub-decision from a clause pointer is a judgment at write time.

**How it was decided.** Born from coverage survivor S12 at review round 1; the user ruled the path "rule
inline" and the lead's proposed ruling in one reply ("as recommded").

**Changed at review.** V4, verify round 1 repair (user: "as recommded"): the example cited another
session's sub-decision without the session qualifier D11-cross-session-qualifier requires; now qualified.

## Defaults batch

Put to the user on the accept screen, 2026-10-08, and confirmed ("screen ok") — now D19-slug-grammar-mechanics (grammar
mechanics) and D20-minor-version-bump (version bump). D9-number-padding-form, D10-durable-id-boundary and D11-cross-session-qualifier were confirmed earlier as their own batch
("default ok").

## Out of scope

From the hardened frame: rule IDs (already dotted slugs, e.g. `brainstorm.frame-card`); renaming the
prefixes themselves (`GI`, `D`, `FR`, …).

## Open questions

- **OQ1 — A seat acting on the slug instead of the rule** (blind map I1). D13-topic-word-slugs's topic slugs narrow it;
  D8-read-test-bet's read test does not measure it. Untested. *(Deferred by D8-read-test-bet's own accepted risk.)*
- **OQ2 — How often requirement and story wording actually changes.** D3-slugged-id-families's accepted risk rests on the
  second blind seat's reading, not a measurement (F10); it sets how often D7-scoped-rename-command's rename runs in user
  projects. Unknown.
- **OQ3 — The opt-in back-fill pass for user projects** (D16-existing-user-projects) — a backlog item, triggered once D14-live-layer-backfill's
  run has proven the method.
- **OQ4 — Meaning drift at the definition** (added at review, S7). A definition whose slug no longer
  fits its topic is invisible to any tool; it rests on the grader at each amendment (D4-slug-follows-topic's trigger).
  Untested.

## Untested bets

- **Three topic words are enough to skip the lookup** (frame, betting line; D13-topic-word-slugs makes them topic
  words) — tested by D8-read-test-bet's two-arm read test, run as wave 0 before any build (as changed at review,
  S9): the slugged arm at least 15 of 20 "knew it" and at least 5 more than the bare arm, none
  "misled". **Tested 2026-10-08 by a cold seat: FAILED** (7 of 20 knew); the user overrode the gate
  and restated the bet — *three topic words tell the reader what an ID is about* — on the test's own
  evidence: right topic on 19 of 20 joined IDs against 0 of 20 bare, none misled (D8-read-test-bet as changed at
  build). The restated bet's untested part: a reader who knows the topic opens the rule rather than
  guessing its verdict (OQ1).
- **The slug stays true as meaning evolves** (frame, betting line) — held by D4-slug-follows-topic's rename-on-change and
  D7-scoped-rename-command's tool. *Reworded at review (S7):* its mechanical half — every mention agrees with its definition
  — is tested by D6-minting-check-enforcement's check as extended at review, run in D18-build-done-check's CI report; D17-governance-amend-route's amend run is the first
  check that tool output and setup template agree. Its judgment half — a definition's slug still fits
  its topic — is untested (OQ4).
- **The tool changes only ID tokens** (D15-protected-line-rewrites) — tested on every apply by D15-protected-line-rewrites's mandatory diff check.

## For the landing

Not decided here, recorded so the landing does not lose them: a `DECISIONS.md` row; `BACKLOG.md` items
for the build and for OQ3's pass; a `ROADMAP.md` touch; a `GLOSSARY.md` entry for the joined-form
grammar (blind map G4).

## Review

**Round 1 — cold read, 2026-10-08.** Seat `blind-reviewer` (`mochiko:devils-advocate` on
`mochiko:review-brainstorm`, persona default tier, solo pairing, both lenses); blind map drawn first
from the frame's problem, destination and out-of-scope lines only (`record_contact: none` attested),
then resumed with the frozen record. Report: `reports/review.md`. Status `critical-gaps`, verdict
FAIL (input to the lead and the user, not a clearing). 24 raised, 15 survived — 1 Critical, 8
Important, 6 Minor; 2 coverage. Coverage: 18 of 19 load-bearing angles answered in depth, E2 partial;
the five dropped angles accepted. The lead re-checked S1's count (213 quoted spans, by
`grep -rhoE '"[^"]{0,200}\b(GI-[0-9]{3}|D[0-9]{1,3}[a-z]?)\b[^"]{0,200}"'` over records and strips),
S2's (140 non-anchor `D<n>` lines in migrations), S3's (21 corpus files carry IDs; the corpus is
`crates/mochiko-cli/tests/fidelity.rs:48`'s frozen side) and S5's files (all three exist, under
`evals/review-specifications/fixtures/g3-lunch-orders-ux-manifest/`) first-hand.

**Routing of the 15 survivors** (`brainstorm.non-coverage-survivors`, `brainstorm.coverage-survivor-routing`):
- *Own turn — a real choice, costly, or challenging a user ruling:* S1 (Critical; D15-protected-line-rewrites, D14-live-layer-backfill) · S4 (D4-slug-follows-topic,
  D7-scoped-rename-command scope per family) · S5 (D12-machine-read-spots vs D19-slug-grammar-mechanics, existing file slugs) · S6 (D10-durable-id-boundary cycle `C1` vs report `C1`) ·
  S7 (D6-minting-check-enforcement/D18-build-done-check stale-slug detection) · S8 (D16-existing-user-projects legacy test and citation form) · S9 (D8-read-test-bet's design).
- *Coverage — the user rules the path (explore now · rule inline · defer):* S2 (blind map E2:
  replayed rule text in the migration log) · S12 (blind map A5: dotted and parenthesised sub-IDs).
- *Lead's repairs, one named batch for the user's word:* S3 (D14-live-layer-backfill's history list adds the fixture
  corpus) · S10 (D4-slug-follows-topic "meaning" vs D13-topic-word-slugs "topic") · S11 (two-word example slugs) · S13 (D10-durable-id-boundary's extension:
  explicit confirm or `Assumed`) · S14 (D7-scoped-rename-command's Why restated against OQ2) · S15 (F20 persisted, cites
  fixed).

Dispositions follow, one per survivor, as they are ruled.

- **S1** (Critical, D15-protected-line-rewrites) — accepted; user-ruled 2026-10-08 ("as recommded", lead's lean A: quotes keep
  their source form). Folded into D15-protected-line-rewrites's and D18-build-done-check's changed-at-review parts.
- **S4** (Important, D4-slug-follows-topic, D7-scoped-rename-command, F17, F18, F20) — accepted; per-family scope recorded as F24 (lead's
  repair); the cross-file qualifier user-ruled ("as recommded", A: qualify like D11-cross-session-qualifier) and folded into
  D11-cross-session-qualifier; the D4-slug-follows-topic renumber repair user-confirmed in the same reply; D7-scoped-rename-command gains a pointer line.
- **S5** (Important, D12-machine-read-spots vs D19-slug-grammar-mechanics) — accepted; user-ruled ("as recommded", A: existing file slugs kept).
  Folded into D19-slug-grammar-mechanics.
- **S6** (Important, D10-durable-id-boundary vs D6-minting-check-enforcement/D18-build-done-check) — accepted; user-ruled ("as recommded", A: report clarification
  label renamed to `Q<n>`). Folded into D10-durable-id-boundary.
- **S7** (Important, D6-minting-check-enforcement, D18-build-done-check, Untested bets) — accepted; user-ruled ("as recommded", A: the check flags
  mention drift; meaning drift → OQ4). Folded into D6-minting-check-enforcement; the bet line reworded; OQ4 added.
- **S8** (Important, D16-existing-user-projects) — accepted; user-ruled ("as recommded", A: the definition decides). Folded
  into D16-existing-user-projects.
- **S9** (Important, D8-read-test-bet, D3-slugged-id-families) — (a) and (b) accepted; user-ruled ("as recommded", A: a bare arm, run as
  wave 0 before any build). Folded into D8-read-test-bet. (c) — D3-slugged-id-families's accepted-risk wording — rides the repair batch.
- **S2** (Important, coverage, blind map E2) — the user ruled the path "rule inline" with the lead's
  proposed ruling ("as recommded"); born as D21-log-rule-citations.
- **S12** (Minor, coverage, blind map A5) — the user ruled the path "rule inline" with the lead's
  proposed ruling ("as recommded"); born as D22-sub-id-forms.
- **Repair batch** (user: "batch ok, R5 yes") — **S3** as R1 (D14-live-layer-backfill, D18-build-done-check) · **S9c** as R8 (D3-slugged-id-families) · **S10**
  as R3 (D4-slug-follows-topic) · **S11** as R4 (D5-compound-reference-forms, D10-durable-id-boundary) · **S13** as R5 (D10-durable-id-boundary, confirmed explicitly) · **S14** as R6
  (D7-scoped-rename-command) · **S15** as R7 (F9, F20, D10-durable-id-boundary); plus R2, the S9 knock-on to D14-live-layer-backfill's method order. All 15
  survivors dispositioned; none listed as changing nothing.

**Verify round 1** — dispatched 2026-10-08 to the same seat: the changed cards and their gists only
(D3, D4, D5, D6, D7, D8, D10, D11, D14, D15, D16, D18, D19, the born D21-log-rule-citations and D22-sub-id-forms under their bounded
round, F9, F20, F24, Open questions, Untested bets).
Result (`reports/review.md` § Verify round 1): **NOT CLEAN** — 14 of 15 survivors closed, S10 carried a
fold-introduced defect; 5 fold-introduced defects, 1 blocking. **V1** (blocking, D21-log-rule-citations vs D12-machine-read-spots/D4-slug-follows-topic) —
user-ruled (a) on the lead's weak lean ("as recommded"): all log rule text cites bare; folded into D21-log-rule-citations.
**V2** (D4-slug-follows-topic title and Statement still "meaning"; D14-live-layer-backfill "already bare by D12") · **V3** (renumber needs a
re-key form in D7-scoped-rename-command; outside D15-protected-line-rewrites) · **V4** (D22-sub-id-forms example unqualified) · **V5** (multi-line quotes, D15-protected-line-rewrites) —
lead's repairs, confirmed by the user in the same reply; folded into D4, D14, D7, D15 and D22.

**Verify round 2** — the five fixes only, dispatched to the same seat 2026-10-08 (the lead's proposal,
confirmed by the user in the same reply; a second failed verify goes to the user).
Result (`reports/review.md` § Verify round 2): **NOT CLEAN, nothing blocking** — V1–V5 closed (V3's
reading checked by the seat against `implement.yaml:717-719`); two new non-blocking defects, each a
one-clause text repair: **W1** (D21-log-rule-citations's "bare, old and new" read literally also binds minting templates)
and **W2** (D14-live-layer-backfill's Statement still carried pre-R1/R2 text). The lead found **W3** of the same class
(D8-read-test-bet's title and Statement kept the one-arm test) and carried D18-build-done-check's allowlist into its Statement. Put to
the user as a second failed verify (`brainstorm.fix-one-card-verify-once`): options A — apply and
close seat-unverified, disclosed — or B — a third verify; user: "as recommded" (A). W1, W2, W3 and the
D18-build-done-check carry applied to D21, D14, D8 and D18. **Closed seat-unverified, disclosed:** the W1–W3 repairs
were not read by the review seat; each carries an existing ruling's text into a Statement and decides
nothing new.
