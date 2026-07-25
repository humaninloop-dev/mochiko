# Operating-Docs Maintenance — Decision Record

**Status:** **accepted 2026-07-25** — full-pair review, 29/29 survivors dispositioned (R1–R7 user-adopted in-session), verify pass CLEAN (round 1 + bounded delta + two-spot check), clearing verdict `ready`, user-accepted
**Opened:** 2026-07-25
**Provenance:** record penned by the session lead (`/mochiko:brainstorm`, team-form); rulings by the user (Deepesh); the Fact-checker map and errata are the fact-checker seat's verbatim text. *(added per review RI-18)*
**Topic:** Rethinking how mochiko projects actively maintain their living operating documents — governance, backlog, roadmap, architecture/ADR records, and whatever else a production project needs. Trigger observations (user's opening statement): this repo's `BACKLOG.md` has grown into something unwieldy; `ROADMAP.md` is not fit for purpose; there is no architecture/ADR record layer at all; the brainstorm-session layer (`.mochiko/brainstorms/` + `index.md`) has stayed comparatively easy to navigate and maintain.

**Prior art:** `setup-operating-docs-scaffolding` (2026-07-17) — scaffolded this layer as the knowledge-management constitution module. This session re-examines how the layer *stays fit* in active use, and what document kinds are missing.

---

## Fact-checker map

*Scope note: three requested inputs need caveats up front. (a) `plugins/mochiko/templates/constitution-template.md` **does not exist** — retired 2026-07-18 per `REGISTRY.md:154`; its successor `governance-surfaces-template.md` was read in its place. (b) **No local kinako clone exists** — `/Users/deepeshadmin/Documents/GitHub/kinako` is absent, and the parent dir contains only `CLAUDE.md` and `mochiko`; §1–§6 below carry no kinako filesystem evidence. (c) The brainstorms layer was skimmed for structure per the brief, not read record-by-record.*

### 1. Each top-level doc's actual shape and load

Sizes at HEAD (`7920ccb`, 2026-07-25):

| Doc | Lines | Bytes | Last touched |
|---|---|---|---|
| `BACKLOG.md` | 557 | 74,796 | 2026-07-25 (`7920ccb`) |
| `ROADMAP.md` | 247 | 104,791 | 2026-07-25 (`7920ccb`) |
| `REGISTRY.md` | 202 | 55,709 | 2026-07-24 (`c1e0ec9`) |
| `CLAUDE.md` | 71 | 5,542 | 2026-07-21 (`78cc925`) |
| `.mochiko/brainstorms/index.md` | 161 | 29,565 | 2026-07-25 (`7920ccb`) |

**BACKLOG.md** — 23 `##` sections (`BACKLOG.md:11,31,76,141,152,207,298,354,372,390,399,410,421,430,443,456,469,499,513,521,531,541,549`). **121 top-level checkbox items: 72 open `[ ]`, 49 done `[x]`** — done items are retained in place, not archived. Section naming is provenance-keyed ("…follow-ups (from the `/mochiko:transform-cluster setup` run, 2026-06-27)"), so sections accrete per build event and are never merged.

Oldest open items: the four in `## Open design decisions` (`BACKLOG.md:15,17,21,25`) — "Prose vs. gate allocation", "Claude-Code portability", "Intensity modes", "Command orchestration substrate". Their stated provenance (`BACKLOG.md:13`) is `agent-skills-research/synthesis/my-framework.md`, a path that **no longer exists** (submodule removed 2026-07-21 per `CLAUDE.md:19-21`; `ls` confirms absent). Next-oldest open: 2026-06-27 setup-port items (`BACKLOG.md:376,378`) and specify-port items (`:396,397`) — 28 days old. `BACKLOG.md:378` cites `human-in-loop/plugins/humaninloop/templates/codebase-inventory-schema.json`, also now-absent. Dangling removed-submodule citations: 2 in `BACKLOG.md`, 3 in `ROADMAP.md`, 5 in `CLAUDE.md`, 0 in `REGISTRY.md`.

Grooming state: **one grooming pass is visible in git and it was reversed within the same day.** `BACKLOG.md` went 92,376 B (`d956fe0`, 2026-07-24) → **59,321 B** (`8463c1b`, 2026-07-25) = −35.8%, at nearly flat line count (398→400) — i.e. long delivered-item entries were compressed in place, not deleted. Over the next six commits (all 2026-07-25) it regained to 74,796 B, **+26.1% above the groomed floor in one day**. Individual `[x]` items do show the compressed one-line form (`BACKLOG.md:179,380,394,408`), so the compression convention is real; the growth outpaced it.

**ROADMAP.md** — its own preamble (`ROADMAP.md:1-3`) calls it "Mochiko Roadmap — v2 (Re-baselined) / *Rewritten: 2026-06-27*". Actual section inventory and byte load:

| Section | Lines | Bytes | Share |
|---|---|---|---|
| Front matter (What changed / Thesis / sound-loop doctrine) | 1–51 | 4,330 | 4.1% |
| **Key Decisions** | 52–90 | **58,954** | **56.3%** |
| Skill-library conventions (five axes) | 92–102 | 2,102 | 2.0% |
| **Decision Trail** | 104–225 | **36,672** | **35.0%** |
| Open Questions + Recommended Next Steps | 226–248 | 2,731 | 2.6% |

**Key Decisions holds 34 rows** and is **91.3% of the file by bytes together with Decision Trail**. Zero of the document's sections are roadmap-forward: "Recommended Next Steps" (`ROADMAP.md:240-245`) lists four items — author the doctrine primitives, build `setup`, build `specify`, "dogfood then generalize" — **all four completed** (per `REGISTRY.md:40,41` and the Decision Trail entries at `ROADMAP.md:194,202`); they have never been updated. "Open Questions (live)" lists 5 items of which 3 are struck-through-resolved in place (`:232,234,236`), leaving 2 genuinely open.

Two concrete defects in the Key Decisions table:
- **The table is split in two by a blank line at `ROADMAP.md:77`.** Rows 56–76 (21 rows) form table 1 with its header at `:54-55`; rows 78–90 (13 rows) form a **second table with no header row** — in any markdown renderer the "Constitution dissolution" row (`:78`) becomes that table's header.
- Row size is unbounded. Longest rows: `:88` **11,424 chars** (skill-succinctness), `:83` 4,123, `:85` 3,441, `:86` 3,227, `:84` 3,210, `:90` 2,993. A single table cell at `:88` is larger than all of `CLAUDE.md` (5,542 B) twice over.

**Decision Trail has stopped tracking Key Decisions.** 16 `###` entries; latest is "wave 2" dated 2026-07-24 (`ROADMAP.md:218`). Key Decisions rows carry 3 dated 2026-07-25, 1 at 2026-07-24, 2 at 2026-07-23, 1 at 2026-07-21, 1 at 2026-07-19, and 6 at 2026-07-18 — against Decision Trail's one 2026-07-18 entry, one 07-23, one 07-24 and nothing after. **Roughly 10 of the most recent 12 decision rows have no Decision Trail entry.** The two structures are documented as complementary but only one is being fed.

**REGISTRY.md** — tracks HIL→mochiko migration inventory by primitive type: net-new primitives (10 rows, 6 marked `[R]` retired), Workflows/Commands (9), Agents (12), Skills (~30 across 8 cluster sub-tables), Templates (26), Catalogs (2), Scripts (4). Status key `[ ]/[x]/[~]/[-]` (`REGISTRY.md:5`).

**It is measurably no longer being updated.** Byte-flat at 55,709 since `c1e0ec9` (2026-07-24); before that 55,144 (07-24) and 54,729 (07-21) — **+1.8% over the whole 18-commit window**. Highest plugin version stamp anywhere in the file is **v0.23.0** (`grep -oE 'v0\.[0-9]+\.[0-9]+'`: v0.22.0 ×1, v0.23.0 ×1, nothing above). Meanwhile the skill-succinctness pass shipped **v0.24.0 through v0.28.0**, editing 27 skills across a pilot and four waves (`ROADMAP.md:88`, `BACKLOG.md:106-131`). `grep -c 'succinctness' REGISTRY.md` = **0**. The convention the file itself established for strip provenance — "**Stripped** 2026-07-19 (specify wave, v0.13.0)" style stamps, present in 6 rows (`REGISTRY.md:56,60,61,62,63,103`) — was applied by the five *command* strip waves and **not once** by the five *skill-succinctness* waves. 28 of 51 files in `.mochiko/strips/` carry a `v0.24–v0.28` stamp; REGISTRY carries none of them.

**CLAUDE.md** — imposes these obligations on the other docs:
- `CLAUDE.md:7-9` — read-pointers to all three (purpose statements only, no maintenance duty).
- `CLAUDE.md:47-50` — new-workflow procedure: "Check `BACKLOG.md`…", "Identify… in `REGISTRY.md`…", step 4 "**Update `REGISTRY.md` and resolve any related `BACKLOG.md` items**".
- `CLAUDE.md:54` — "record the decision in `ROADMAP.md` under Key Decisions and close the backlog item. Don't let structural decisions live only in conversation context."
- `CLAUDE.md:58` — session artifacts belong in `.mochiko/brainstorms/<slug>/`, "never at the repo top level"; top level reserved for the four operating docs; "A session's *ruling* still lands in `ROADMAP.md` Key Decisions with a pointer to the session record".
- `CLAUDE.md:60` — the index contract: read-before-open, entry-on-open, status-update-on-close, "a defect — fix it on sight."

Every obligation is **additive** (append a decision, add an entry, add a REGISTRY row, close an item). **No line in `CLAUDE.md` obliges anyone to prune, compress, archive, re-section, or re-verify any of the three docs.** There is no size bound, no staleness check, and no fitness definition for `ROADMAP.md`, `BACKLOG.md`, or `REGISTRY.md`.

### 2. Maintenance carriers

**Enforced carrier — exactly one, and it covers only the brainstorms index.** `plugins/mochiko/commands/brainstorm.md:20` carries the index contract as executable command steps: read the index before opening; add the entry on open with status `open`; update at acceptance/supersession "naming where the outcome landed (a ROADMAP row, or an explicit no-graduation)"; and "**At open and close, run the module's invariants … under fix-on-sight**". It gates on `.mochiko/brainstorms/index.md` existing **or** a CLAUDE.md governance-region KM pointer, with an explicit silent-skip branch when neither is present.

**No carrier at all for `BACKLOG.md`, `ROADMAP.md`, or `REGISTRY.md` content.** Exhaustive grep of `plugins/` for `BACKLOG|ROADMAP|REGISTRY` returns 23 hits, all of which are one of four non-carrier kinds:
- Module *definition* text (`templates/constitution-modules/knowledge-management.md:12,27,28,30,31,32,52,57,72,75,76,81,84` — 13 of the 23) — describes roles and circulation, does not execute.
- Scaffolding-only (`commands/setup.md:177` — creates the files at G5; no update or groom duty).
- Doc pointers (`skills/loop-discipline/SKILL.md:138`; `commands/slice.md:40`; `templates/governance-surfaces-template.md:58`).
- Routing of *findings into* BACKLOG (`skills/testing-governance-injection/SKILL.md:28,36,87` — writes to it, never grooms it).

**`REGISTRY.md` has no mention in any command or skill** — the only obligation to update it lives in `CLAUDE.md:50`, prose with no executable carrier. This matches the observed v0.24–v0.28 gap in §1.

**Content quality is explicitly exempted by design, not by omission.** `knowledge-management.md:56-58`: "Content **quality** of `BACKLOG.md`/`ROADMAP.md` is explicitly exempt from mechanical enforcement — that boundary is declared, not implied."

**Two of the KM module's three declared carriers do not currently fire.** `knowledge-management.md:36-46` names three: (1) command bookkeeping, (2) CLAUDE.md sync rows, (3) setup/amend re-audit.
- Carrier 2 is **stub-backed** — `REGISTRY.md:80` shows `[ ] syncing-claude-md` unported; `knowledge-management.md:44` admits "(stub-backed until the `syncing-claude-md` cluster ports)"; `BACKLOG.md:511` tracks it open.
- Carrier 3 is **not implemented in the command.** `knowledge-management.md:45-46` claims "the constitution validator re-checks the invariants whenever `/mochiko:setup` runs." Grep of `commands/setup.md` for `invariant|re-audit|fix-on-sight` returns **no match**. On the validator side, `validation-constitution/references/QUALITY-CHECKLIST.md:60` says only "`knowledge-management` fragment (if selected…)" — it delegates to the module's embedded fragment, whose item 7 (`knowledge-management.md:87`) does specify a repo-level re-audit, but no setup-command step invokes it. So the chain is: module asserts the carrier → checklist points at the fragment → fragment describes the check → **no command step runs it**.
- Only carrier 1 (brainstorm's index bookkeeping) is live end-to-end.

**Mochiko's own repo cannot use carriers 2 or 3 regardless.** `grep 'mochiko:governance' CLAUDE.md` → none; `.mochiko/memory/` does not exist; `.claude/rules/` is empty. Mochiko has never run `/mochiko:setup` on itself — it holds the KM bundle by hand-authorship. Carrier 1 still fires here via the index-exists branch.

### 3. The brainstorms layer's contract

Structural properties, as facts:

- **Per-session directories.** 22 session dirs under `.mochiko/brainstorms/` plus `index.md`. Contract stated at `CLAUDE.md:58` and `knowledge-management.md:25` ("never at repo root").
- **Newest-first index, one entry per session.** `index.md:3`. Entry count = **22**, session dir count = **22**, and the coherence check passes: a per-dir loop for `^## \`<dir>\`` in the index found **zero missing entries**. The invariant is currently clean.
- **Status field, five values in use** across the 22 entries: `accepted` ×15, `acted` ×4, `ruled` ×1, `open` ×1, and one entry whose status text wraps to a second line (the multi-line entry at `index.md:23-26`) — the value is present, only the one-line grep shape differs.
- **Enforced open/close invariants.** `index.md:5`: "opening a session adds an entry at the top (status: open); acceptance, supersession, or abandonment updates it — status, review state, and where the outcome landed. A session directory without an index entry, or an entry whose status contradicts its record, is a defect to fix on sight." Mechanically specified at `knowledge-management.md:48-55` with pass/fail conditions and a vacuous-at-zero-sessions clause; executed at `commands/brainstorm.md:20`.
- **Entries mutate in place; records are append-only-by-convention.** Each index entry carries five fixed fields (When / Status / Artifacts / About / Landed) rewritten as status changes — the entry is a mutable summary, not a log. Records themselves are described as "as-you-go" audit trails (`commands/brainstorm.md` session-parameters section).
- **Two artifact generations coexist.** `index.md:7`: `record.md` is canonical; `synthesis.md` is the deliverable for pre-v2 sessions and, from v2.2 on, an on-request digest stamped *derived — record canonical*. On disk: 17 dirs have `record.md`, 5 have `synthesis.md` only (`agent-decoupling`, `brainstorm-command`, `command-altitude`, `playbook-design`, `vertical-graduation`), 2 have both (`setup-constitution-flexibility`, `setup-operating-docs-scaffolding`), 1 has an `inputs/` subdir (`team-method-vs-command-shape`), 1 carries a `run-costs.md` (`model-tiered-seats`).
- **Review state is a first-class index field**, and un-reviewed records are labeled as such — e.g. `index.md:18` "record **un-reviewed** (bare session…)".
- **Landing is named per entry**, closing the graduation loop back to ROADMAP — e.g. `index.md:21` "**Landed:** ROADMAP Key Decisions (team-method row, 2026-07-25); BACKLOG …". The open session's entry reads "*(session open)*" (`index.md:16`).
- Load: 29,565 B / 161 lines for 22 entries ≈ 1,344 B per entry average, though entries vary (6-line entries at `:45-50` vs 22-line at `:23-44`).

### 4. The ADR gap

Decision-record machinery that exists today, at two disjoint scopes:

**Feature scope — `D-XXX` records inside `constraints-and-decisions.md`.** The technique lives in `plugins/mochiko/skills/patterns-technical-decisions/` (SKILL.md 93 lines + `references/DECISION-RECORD.md` 171 + `references/EVALUATION-MATRIX.md` 126). `SKILL.md:57` — "Record decisions in ADR format for future maintainers — Status, Context, Decision, Rationale…, Alternatives Considered". `SKILL.md:63` and `references/DECISION-RECORD.md:42` both state the destination: "The decision records this skill produces are written into the **`constraints-and-decisions.md`** artifact". The ownership split is explicit at `SKILL.md:69-72`: the **artifact** (the `D-XXX` field schema, `C-XXX`↔`D-XXX`/`IP-XXX` traceability) belongs to `authoring-technical-requirements`; the **technique** belongs to `patterns-technical-decisions`. Confirmed from the other side at `authoring-technical-requirements/references/ARTIFACT-TEMPLATES.md:113` and `references/TRACEABILITY-PATTERNS.md:90`. **Scope is per-feature**: the artifact lives under `.mochiko/specs/<feature>/`, so a D-XXX record is reachable only through the feature that produced it.

**Framework scope — ROADMAP Key Decisions rows.** 34 rows (§1), mochiko's own decisions only, each with Decision/Choice/Confidence/Rationale columns and a pointer to a `.mochiko/brainstorms/<slug>/record.md`.

**No project-level architecture-decision store exists anywhere.** Confirmed three ways:
1. The KM module scaffolds exactly four artifacts (`knowledge-management.md:23-28`): `.mochiko/brainstorms/<slug>/`, `.mochiko/brainstorms/index.md`, `BACKLOG.md`, `ROADMAP.md`. **No ADR/decisions directory.**
2. The other three constitution modules are `evolution-notes.md` (1,953 B), `layer-rules.md` (3,468 B), `release-gates.md` (1,913 B) — none is a decision store.
3. The governance ledger (`governance-surfaces-template.md`, Shape 3, `:89-147`) holds Waivers / Amendment policy / Exception registry / Principles Three-Part metadata keyed by GI-ID / Amendment log. It records **governance principles and their amendments**, not architecture decisions.

Every remaining `ADR` mention under `plugins/` is either the feature-scope machinery above, a routing exclusion (`authoring-requirements/SKILL.md:29` — "Technical architecture decisions - Use ADRs or technical design documents", pointing outward at no mochiko artifact), or **brownfield read-only detection of ADRs the project already had**: `analysis-codebase/SKILL.md:40` ("Check README, CLAUDE.md, CODEOWNERS, ADRs"), `references/CONTEXT-GATHERING.md:44` (`ls -la docs/architecture* docs/adr/* ADR/*`), `:163` ("**ADRs**: [Count] found in [location]"). Mochiko can *detect* an existing ADR store; it scaffolds and maintains none.

### 5. What the knowledge-management module scaffolds for user projects

Module file: `plugins/mochiko/templates/constitution-modules/knowledge-management.md` (89 lines, 6,430 B).

**Doc set — exactly four, adopted or declined whole** (`:23-28`; the whole-bundle rule at `:4-9`: "Offered DEFAULT-ON in every mode — the user must actively decline — and adopted or declined WHOLE: a fixed four-part bundle, **no inner menu, no layout parameterization** (layout mirrors mochiko's own dogfooded shape). A project for which any part feels heavy declines the module, not a fragment"). Elicited at interrogation dimension #7 (`skills/authoring-constitution/references/INTERROGATION-AGENDA.md:27`); scaffolded at setup G5 on a never-overwrite floor (`commands/setup.md:177`, `knowledge-management.md:68`); a recorded decline is durable (`:9`).

**Invariants (`:48-58`)** — index↔session-dir bijection; index status matches the record's Status line; all four bundle artifacts exist; every accepted entry names its landing (a ROADMAP row or an explicit no-graduation). Plus two declared boundaries: vacuously satisfied at zero sessions, and content quality of BACKLOG/ROADMAP exempt.

**Against the three user-observed problems:**

- **Backlog bloat — the module is silent, and deliberately so.** `BACKLOG.md`'s entire specification is one table cell (`:27`): "Open threads: design decisions awaiting a ruling, scoped-but-unbuilt work. Not an issue tracker." No size guidance, no item lifecycle, no archive rule, no pruning obligation, no done-item disposition. The content-quality exemption at `:56-58` affirmatively places bloat outside enforcement. Nothing in the module or any command distinguishes a 121-item backlog from a 5-item one.
- **Roadmap unfitness — the module acknowledges one failure mode and only one.** `:31-32`: "An index whose rulings **cannot graduate** is a dead ledger" — i.e. it guards *inflow* (rulings must be able to reach ROADMAP), and the fourth invariant enforces that every accepted session names a landing. It is **silent on outflow and on fitness**: no statement of what a decision row should contain, no size bound (mochiko's own row at `ROADMAP.md:88` is 11,424 chars), no supersession/retirement mechanics for rows, no separation of decision-log from forward-plan. The module's own role definition (`:28`) is "**The durable decision log**: accepted session rulings graduate here as decision rows" — a decision log, not a roadmap. Mochiko's actual `ROADMAP.md` matches that role at 91.3% by bytes (§1) while still carrying a stale forward-plan section (`ROADMAP.md:240-245`) the module never asked for.
- **ADR absence — the module is entirely silent.** The word does not appear in the module. Its four-part bundle contains no decision-record artifact at any scope other than the ROADMAP decision log, which is scoped to *session rulings*, not architecture decisions. There is no acknowledgement of the gap and no pointer to another module or skill that fills it. `:12-15` does one adjacent piece of housekeeping — disambiguating the module's `ROADMAP.md` from `.mochiko/memory/evolution-roadmap.md` (the brownfield improvement plan, produced by the still-unported roadmap cluster, `REGISTRY.md:117,171`) — but that clarifies naming, not the absent artifact class.

Other silences worth recording: no per-doc ownership, no review cadence, no staleness or age signal on any artifact, and no obligation on `REGISTRY.md`-class inventory docs (the module never contemplates one).

### 6. Git evidence of growth

18 commits, 2026-07-21 (`78cc925`) → 2026-07-25 (`7920ccb`). Bytes, not lines — lines materially undercount both files (e.g. `bbcd67b`→`f7e9517`: ROADMAP lines went 247→246 while bytes went 100,522→101,797):

| Date | Rev | BACKLOG B | ROADMAP B | REGISTRY B |
|---|---|---|---|---|
| 07-21 | `78cc925` | 77,468 | 69,047 | 54,729 |
| 07-24 | `572f49a` | 77,468 | 75,180 | 54,729 |
| 07-24 | `a263d78` | 81,528 | 79,448 | 55,144 |
| 07-24 | `c1e0ec9` | 82,710 | 85,894 | **55,709** |
| 07-24 | `ff6e171` | 84,522 | 89,122 | 55,709 |
| 07-24 | `d956fe0` | **92,376** ← peak | 89,122 | 55,709 |
| 07-25 | `8463c1b` | **59,321** ← groomed | 89,122 | 55,709 |
| 07-25 | `ff5275a` | 66,428 | 89,122 | 55,709 |
| 07-25 | `e47f256` | 68,991 | 92,355 | 55,709 |
| 07-25 | `a41d13a` | 68,523 | 93,693 | 55,709 |
| 07-25 | `8a4ba24` | 68,868 | 95,150 | 55,709 |
| 07-25 | `6626d32` | 69,206 | 96,629 | 55,709 |
| 07-25 | `bbcd67b` | 70,924 | 100,522 | 55,709 |
| 07-25 | `f7e9517` | 70,904 | 101,797 | 55,709 |
| 07-25 | `7920ccb` | **74,796** | **104,791** | 55,709 |

- **`ROADMAP.md`: +51.8% in 4 days (69,047 → 104,791 B), monotonically non-decreasing — it never shrank once across all 18 commits.** Line count over the same window: 226 → 247 (+9.3%), so lines report one-fifth of the actual growth.
- **`BACKLOG.md`: one grooming event, reversed same-day.** 92,376 → 59,321 B at `8463c1b` (−35.8%, at flat line count 398→400 — in-place compression of long entries, not deletion), then 59,321 → 74,796 B over the next 6 commits (**+26.1% recovery inside one calendar day**). Net across the window: 77,468 → 74,796 B (−3.4%) — i.e. four days of accretion roughly cancelled by the single groom, with the post-groom slope steeper than the pre-groom slope.
- **`REGISTRY.md`: +1.8% total (54,729 → 55,709 B), and byte-identical since `c1e0ec9` (2026-07-24)** — 13 consecutive commits with zero change, including the five skill-succinctness version bumps v0.24.0–v0.28.0 that edited 27 skills the registry has rows for. Commits touching `REGISTRY.md` cluster entirely in the 2026-07-19→07-24 window (`REGISTRY.md` log: 07-24 ×2, 07-20 ×1, 07-19 ×6, plus older).
- Direction of the last three commits: BACKLOG +3,892 B, ROADMAP +4,269 B, REGISTRY +0 B.

---

## Problem framing (from questioning)

- **Q1 — the job these docs fail at:** the user reaches for ROADMAP.md / BACKLOG.md to maintain a *high-level view* — what needs to be worked on, and the future direction. Both times the reach failed the same way: "a very verbose document with awkward structure." The failure is at *read* time; the docs are written to dutifully but don't serve the view job when read.

---

## Decisions

### D1 — Root-cause diagnosis: BACKLOG is fixable in kind; ROADMAP is the wrong kind

**Statement:** BACKLOG.md's failure is (A) a shape defect — the scannable view and the archival resume-context are interleaved in one file — **plus** (B) a maintenance defect — write obligations exist but no grooming obligation anywhere. Both are fixable while keeping "a backlog" as the document kind. ROADMAP.md's failure is (C) — it is the wrong document *kind* for the "future direction" job; restructuring or grooming it would not make it fit. It needs reconception, not repair.

**Rationale:** The read-time job (Q1) is a high-level view; the brainstorms layer — the one healthy layer — has both structural view/archive separation and session-boundary invariants, supporting A+B for backlog-shaped content. The user rules ROADMAP's defect is categorical, not structural.

**Confidence:** Backlog half `Confident` (aligned with lead recommendation, ruled crisply). Roadmap half `Confident` — sharpened by the Q3 reframe, which the user confirmed from their roadmap experience: ROADMAP.md as it exists is a **de-facto decision archive wearing a roadmap's name** (Key Decisions rows + Decision Trail = an unlabeled proto-ADR store). The genuine future-direction document was never crowded out — it was never built. Consequences carried forward: (1) the "missing ADR layer" half-exists and is squatting in ROADMAP.md; (2) the direction document is a green-field design, not a repair.

### D2 — The direction document: four pieces, one screen

**Statement:** The future-direction document (working name DIRECTION.md; naming ruled later) is green-field and holds exactly four pieces: (1) **Thesis** — 2–3 lines on what the project is becoming and the core bet, subject to a single-home ruling vs CLAUDE.md's overview; (2) **Now / Next / Later** — three short lists, one line per item, each linking to its backlog item or session record; *Later* is explicitly non-committed; (3) **Standing bets & revisit conditions** — the few strategic `Contested`/provisional bets with the condition that would reopen each; (4) **Nothing else** — decision rows, trails, and rationale prose live in the decision archive, not here. Piece 2 is the heart; piece 4 is the discipline that keeps it one screen.

**Rationale:** The read-job (Q1) is a one-glance view of current work + direction. Now/Next/Later is the standard shape for exactly that job. The fact-checker map independently confirms the need: ROADMAP's one structurally forward-looking section was never revised — three of its four Next-Steps items silently completed, the fourth still live with a deleted destination (map §1 as corrected by errata E1; rationale restated per verify V2). Known risk, accepted: horizon lists rot fastest — piece 2 depends on the maintenance-carrier design (ruled later) more than any other piece.

**Confidence:** `Confident` (user adopted the recommended shape; unelaborated adoption — streak watch at 1).

**Review folds:** *Later* items are exempt from piece 2's link rule until promoted to *Next* — a novel Later line need not mint a BACKLOG item (RI-11). One-screen is enforced mechanically, not aspirationally: horizon caps live in D6's invariant list (R4, user-adopted). ROADMAP's live Next-Steps item 4 — convention extraction, per errata E1 — routes into DIRECTION *Next* at migration (RI-3).

### D3 — Decisions live as a thin index over records; this is the ADR layer

**Statement:** Decisions evicted from ROADMAP live under the index-over-records pattern: a thin decisions index (one line per decision — date, title, status incl. supersession, pointer). The fat rationale lives where it already lives — the session record. A decision with no session record (e.g. a build-time wave ruling) gets a small per-decision ADR file in a `decisions/` directory, indexed the same way. This single design absorbs both the ROADMAP eviction and the missing ADR layer, for mochiko itself and for scaffolded production projects alike.

**Rationale:** Index-over-records is the one pattern with a proven health record in this repo (map §3). Option B — the disciplined table — is what Key Decisions already implicitly was, and it outgrew its intended pointer discipline to an 11,424-char cell (map §1) because a table cell is the only home for rationale when no record exists. Option C — folding into the brainstorms index — fails for exactly those record-less decisions.

**Confidence:** `Confident` (user adopted the recommended option A; unelaborated adoption — streak watch at 2).

**Review folds:** Granularity contract stated (DQ-4): one index row per *ruled decision* — a session may yield several rows (this one yields 12); the prior unit was ~1.5 rows/session. The decisions index is **inside the groom pass's scope**; at groom, a superseded row compresses to a one-line supersession entry — **one line per superseded decision, status preserved**, so R2's status-agreement invariant keeps its per-decision anchor (DQ-4; verify V5) — retirement without deletion. Placement (lead-formulated, `Assumed`, RI-5 residue): the index is a living read surface → top-level `DECISIONS.md`; per-decision records under `.mochiko/decisions/`. Schema ownership (DQ-9): the **module template** owns the project-scope decision-record schema as **authoring source** — the scaffolded project-pinned copy is the runtime source (verify V1); `patterns-technical-decisions` keeps the technique, and the boundary sentences in both it and `authoring-technical-requirements` get the corresponding build-time edits.

### D4 — ROADMAP.md retires; DIRECTION.md replaces it; the decision layer is a standalone sibling

**Statement:** The roadmap document kind is retired — no file named ROADMAP.md survives. The direction document (D2's four-piece, one-screen doc) is **DIRECTION.md**, a different document kind, not a restructured roadmap. The decision/ADR layer (D3) is its **own standalone artifact set** — thin decisions index + records — a sibling of DIRECTION.md, never a section inside it or any other doc. Build-time dispersal of the existing ROADMAP.md: thesis prose → DIRECTION.md piece 1 (subject to D2's single-home ruling vs CLAUDE.md); Key Decisions rows + Decision Trail entries → the decisions layer (compressed to index rows where a session record already carries the rationale; per-decision records where none does); dead sections (completed "Recommended Next Steps", resolved open questions) dropped; the skill-library-conventions section rehomes at build time (likely CLAUDE.md — deferred). Incoming pointers (CLAUDE.md obligations, KM module text, 22 index entries' "Landed: ROADMAP…" lines) are a named migration surface, not silently broken.

**Rationale:** User-originated ruling ("is roadmap actually direction.md, and the adr or decision records should be its own thing") — the explicit form of what D1's reframe + D2 + D3 already implied. Separation is the structural fix for the squatting failure mode: the decision log grew inside ROADMAP because rulings were routed into a doc with no other home for them (map §1, §5).

**Confidence:** `Confident` (user-originated; aligned with the ruled architecture).

**Review folds:** Dispersal is a **triage, not a wholesale drop** (RI-3/E1): the four Next-Steps items are triaged at migration — three complete (drop), item 4 live (→ DIRECTION *Next*, its deleted `PLAYBOOK.md` destination noted); reviewer dissent on whether the original instruction was self-limiting is recorded — the triage step moots it. The migration surface **adds `plugins/mochiko/commands/brainstorm.md:20` and the module invariant's hardcoded target at `knowledge-management.md:52-53`** (per errata E4) — the executable pointers to "a ROADMAP row" — both retargeted to a decisions-index row (RI-4; verify V7). ROADMAP.md archives at a **stable path** so historical index `Landed:` pointers resolve unrewritten (DQ-13; per E5, 18 of 21 landed entries name ROADMAP — this entry's earlier "22 index entries" figure corrected, RI-17). ROADMAP's two genuinely-open questions (Claude-Code portability, intensity modes) **already exist as BACKLOG open items** (`BACKLOG.md:17`, `:21`) — at migration they are reconciled in place (provenance stamped per D5's terminal state), never duplicated (RI-13; verify V6). CLAUDE.md:58's top-level reservation is restated at migration as the explicit new living set (RI-5).

### D5 — BACKLOG.md: open items only; done work moves to an append-only trail

**Statement:** BACKLOG.md holds **only open items** — one bounded entry each (title, date, provenance pointer, and resume-cold context, preserving the earlier grooming ruling), in theme-keyed sections that merge on groom (never provenance-keyed-forever). On completion an item is **not deleted**: it compresses to the established one-line DONE + pointer form and **moves to an append-only trail file** — the trail is never thrown away. *Lead-formulated placement, unconfirmed:* the trail lives at `.mochiko/backlog-archive.md` — top level stays reserved for living read surfaces; history belongs under `.mochiko/` beside the brainstorm records and strip notes.

**Rationale:** User ruled A (open-items-only read surface) with an explicit trail-preservation requirement ("don't want to completely remove when a work is done, need to save the trail"). The map grounds both halves: 49 of 121 items are done-items bloating the read surface (§1), while the compression convention itself is real and worth keeping — as the trail's entry format, not as a backlog resident.

**Confidence:** `Confident` on the ruling; trail placement `Assumed` (lead's fill — correct at will).

**Review folds:** BACKLOG's read-job stated (DQ-1): the **complete open-set detail store** — scannably complete, never curated; DIRECTION is the sole *curated* scan surface, and the "everything open, at a glance" reach lands on BACKLOG's theme-keyed open list, not on DIRECTION. Rejected roads recorded (RI-10): **B** compress-in-place + carrier — rejected because done-rows still occupy the read surface and the done-surface now exists in the trail (verify V9); **C** a second top-level done-archive doc — rejected as duplicating the decisions layer and git. The trail's read-job named (RI-6/DQ-14): **resume-cold on a reopened item + provenance lookup** — it passes D10's admission rule legitimately; no archive exemption needed. Dead provenance gets a terminal state (RI-12): `provenance: unrecoverable (<what it was>, removed <date>)` satisfies both this decision's pointer requirement and D6's dead-pointer scan; applied at migration to `BACKLOG.md:13` and `:378`.

### D6 — Maintenance carriers: invariants + subtractive landing as the floor

**Statement:** Four carriers, ranked: (1) **extended invariants at command boundaries** — no `[x]` items in BACKLOG, every decisions-index pointer resolves, every DIRECTION *Now* item points at live work, dead-pointer scan; fix-on-sight; (2) **subtractive landing ritual** — closing work means: append the decision row + move the closed item to the trail + touch DIRECTION Now/Next in the same moment; (3) **named groom procedure** (not a command until evidence demands one) — theme-section merges, Next/Later re-ranks, ~2×-groomed-baseline byte alarm *(alarm replaced — see R7 fold)*; (4) **minimal staleness signals** — dates on Now/Next items and standing bets; last-groomed stamps on DIRECTION and BACKLOG. 1+2 are the enforced floor; 3+4 stay minimal.

**Rationale:** The map's core finding: every existing obligation is additive and the only carrier that held is command-boundary invariants (§2); the one groom without a carrier was overrun in a day (§6). Carrier 2 attacks the additive-only asymmetry at its root.

**Confidence:** `Confident` (user: "sounds good").

**Review folds (incl. user rulings R1/R2/R4/R7):** **(R2, closes Critical DQ-3)** the landing ritual covers **supersession** as well as closing — one landing move updates *both* indexes — and the invariant upgrades from "pointer resolves" to "**statuses agree**" across brainstorms index ↔ decisions index ↔ record Status lines. **(R4)** One-screen enforced: horizon caps **Now ≤5 / Next ≤7 / Later ≤10** join the invariant list. **(R7, fact-settled by E6)** The ~2× whole-file byte alarm is **replaced**: the observed regrowth was new items (+12 open; 72 open — the window's highest), not re-expansion, and no baseline in history reaches 2× — the alarm becomes a **per-open-item size bound plus an open-item-count watch**, with baseline figures living on the last-groomed stamp line (DQ-12's value-home). **(R1, closes Critical DQ-2 with D7)** Omission-class drift is caught **only** at boundary invariants — named: brainstorm open/close (live today), setup/amend, and specify/plan/implement landings where those commands run. **(DQ-6)** 6.3 clarified: the groom is a *skill*, model-invokable — it attaches to already-firing boundaries (brainstorm close; setup/amend), never to user initiative. **(V10)** The groom's tier stated: it is *invoked by fix-on-sight* whenever a cap or bound trips — a tier-1 trigger — and stays minimal otherwise. Rejected roads recorded (RI-10/DQ-6): a dedicated groom *command* (user-invoked — the reliance §6 shows failing) and hook/CI enforcement (kernel-free rule). Honesty note (RI-8): the streak-watch counter opened at D2 reached **3 at this decision** and was not flagged at the time — a process miss, recorded; D7–D12 were engaged answers (a rider added, the architecture reshaped at D4, a recommendation overridden at D10), which reset the streak.

### D7 — Enforcement surfaces: command steps primary, paths-scoped rules for ad-hoc edits, CLAUDE.md pointers only — scaffolded by setup

**Statement:** Three-surface enforcement split: (1) **command steps** carry D6's invariants and the subtractive landing as executable steps (the proven surface); (2) **a `paths`-scoped `.claude/rules/` file** over the operating docs injects the shape contract for ad-hoc edits outside any command — the drift channel that killed REGISTRY *(clause withdrawn — see R1 fold)*; its delivery is probe-verified via `testing-governance-injection`, never assumed; (3) **CLAUDE.md** holds pointers and the groom-procedure text only — never again a sole carrier. No hooks or scripts — kernel-free stands. **Setup integration (user rider):** these surfaces are part of the `/mochiko:setup` process — the KM module scaffolds the rules file and CLAUDE.md content alongside the doc set, so production projects get the enforcement, not just the documents.

**Rationale:** Direct evidence split: CLAUDE.md-prose-as-sole-carrier measurably failed (REGISTRY, map §1–2); executable command steps measurably held (index, map §2–3). Rules cover the one channel command steps can't reach.

**Confidence:** `Confident` (user: "yes, I will adopt"; setup-integration rider user-originated).

**Review folds (user rulings R1/R5):** **(R1, closes Critical DQ-2 with D6)** Rationale corrected: `paths`-scoped rules raise **touch-time edit quality**; they are structurally blind to omission — REGISTRY died of *no edit* (13 byte-identical commits), a failure mode only boundary invariants catch; the earlier "the drift channel that killed REGISTRY" clause is withdrawn. Mochiko's own compliance is **manual** until more commands run in this repo, and the module dogfood **gates on the D7.2 injection probe**. **(R5, closes DQ-8′)** Module invariants are **project-pinned at scaffold time**; a plugin upgrade arrives as an *amend offer* through the existing ledger/version-bump machinery — never silent enforcement of unratified governance.

### D8 — REGISTRY.md retires to the archive

**Statement:** REGISTRY.md's job — HIL→mochiko migration tracking — is done ("dead job", user). It moves to the archive under `.mochiko/` per the D5 trail principle (archive, never delete); CLAUDE.md's REGISTRY obligations are dropped in the migration. The plugin directory itself is the living primitive inventory *(clause withdrawn — see R3 fold)*. REGISTRY-class inventory docs are project-specific artifacts, not part of the scaffolded module.

**Rationale:** *(added per RI-9)* the migration-tracking read-job ended with the migration; retention is the D5 trail principle, not utility.

**Confidence:** `Confident`.

**Review folds (user ruling R3):** the "plugin directory itself is the living primitive inventory" clause is **withdrawn** (RI-1/DQ-7): `ls` distinguishes present from absent but cannot express *deliberately retired* vs *wanted-but-unbuilt* — the status information being archived. At migration the 13 open `[ ]` rows are dispositioned: still-wanted rows → BACKLOG open items (D5's "scoped-but-unbuilt work" is their exact description); the rest stamped **abandoned** in the archive; `[ ] authoring-roadmap` → **abandoned** (its document kind retires with D4). **(V3)** The archive's read-job named: a **provenance query** — what was ported, retired, or abandoned, carrying the status the filesystem cannot express — so it passes D10's admission rule by naming, not assertion. **(V11, lead-formulated `Assumed`)** Stable archive paths: `.mochiko/archive/REGISTRY.md` and, symmetrically for D4, `.mochiko/archive/ROADMAP.md` — so migrated BACKLOG items' provenance pointers and historical index `Landed:` pointers resolve. **(RS6)** Two archive conventions coexist (`.mochiko/backlog-archive.md` for the trail, `.mochiko/archive/` for retired docs) — both `Assumed`; the build settles one home, lead's preference `.mochiko/archive/` for both.

### D9 — Scope: this is the redesigned knowledge-management module, scaffolded for every plugin-using project

**Statement:** The ruled architecture is not repo housekeeping — it is the **redesign of the knowledge-management constitution module**, scaffolded by `/mochiko:setup` on mochiko projects that use the plugin. The module's bundle becomes: `brainstorms/<slug>/` + `index.md` · open-only `BACKLOG.md` · the append-only trail · `DIRECTION.md` · the decisions layer (index + records) — plus the D7 enforcement surfaces (rules file, CLAUDE.md content, command steps). Mochiko's own repo is the first migration and dogfood. (Whole-bundle adopt/decline and never-overwrite scaffolding presumed carried forward from the current module unless build reveals a conflict. *Bundle-rule presumption superseded — see R6 fold.*)

**Rationale:** *(added per RI-9)* the observed defects are module-design defects (map §5's silences), not repo quirks; fixing them only here would fork doctrine from product.

**Confidence:** `Confident` (user-originated, twice stated — Q7 rider and Q9).

**Review folds (user ruling R6):** **Known risk, accepted** (RI-2): the redesign's evidence base is **n=1 and hand-authored** — this repo never ran setup, has no `.mochiko/specs/`, and its §2 carrier evidence does not generalize to scaffolded projects; the module dogfood is the test. **(R6, closes DQ-10)** The bundle rule is re-ruled deliberately: **core adopted/declined whole; electives per-doc opt-in at setup** — explicitly superseding the module's ratified "no inner menu, no layout parameterization" clause, not reinterpreting it. **(V4)** The core enumerated once, authoritatively — **brainstorms layer (`brainstorms/<slug>/` + `index.md`) · open-only `BACKLOG.md` + the append-only trail · `DIRECTION.md` · the decisions layer (index + records) · `ARCHITECTURE.md` · `GLOSSARY.md`** — adopted or declined whole; **electives: `CHANGELOG.md`, `RUNBOOK.md`**, per-doc opt-in. D10's additions are inside the core; this decision's earlier five-part list is superseded by this enumeration — **plus the D7 enforcement surfaces (rules file, CLAUDE.md content, command steps): core, not separately declinable (RS2)** — documents never scaffold without their carriers.

### D10 — Coverage: admission rule + the widened core bundle

**Statement:** A doc enters the module only if it names a **read-job, a writer moment, and a carrier** (the REGISTRY lesson — no carrier, no scaffold). Under that rule: **ARCHITECTURE.md joins the core bundle** — the living system view (components, boundaries, data flow); ADRs record changes, this records the resulting system; carrier: plan/implement landings on structural change. **GLOSSARY.md joins the core bundle** — user override of the elective recommendation, held after challenge: domain language has always been a recurring pain in their projects; carrier: spec landing when new terms mint. **CHANGELOG.md and RUNBOOK.md stay elective**, elicited at setup by project type (releases / deployed service). Risk register dropped — BACKLOG's job. README/CONTRIBUTING out of mochiko's remit.

**Rationale:** *(added per RI-9)* membership needs a gate or the doc set regrows unboundedly — the admission rule is the anti-REGISTRY test stated positively.

**Confidence:** Architecture-core `Confident`; glossary-core `Contested` (deliberate user preference sustained after the dead-doc pushback); changelog-elective `Assumed` — inferred from the user defending only glossary after the pushback on both; correct if changelog was meant to stay core. *(Marks added per RI-9:)* the **admission rule itself** `Confident` (rationale above); RUNBOOK-elective, risk-register-drop, README-exclusion each `Confident` (ruled with the table).

**Review folds:** the trail and the REGISTRY archive pass the admission rule via their named read-jobs (D5 fold — trail; D8 fold — REGISTRY archive) — no archive-class exemption needed (RI-6/DQ-14). **(DQ-15)** D12's watch trigger moves off the elective CHANGELOG to an always-present surface: the groom pass carries the expansion-heavy-surface check.

### D11 — Primitive structure: two new skills, zero new commands, zero new agents

**Statement:** The build rides existing primitives wherever one exists: setup G5 + a rewritten KM module template carry scaffolding; the invariant list and subtractive landing are **single-sourced in the module template** and referenced by every command's landing step (brainstorm's proven pattern); `patterns-technical-decisions` extends to the project-scope `decisions/` destination; GLOSSARY needs no skill (term format in the module template + a specify-landing step); grading stays on `validator` via the module's invariant fragment in `validation-constitution` — including fixing the map §2 finding that setup never actually invokes it. Two new skills only: **`grooming-operating-docs`** (the D6.3 groom procedure; CLAUDE.md carries a pointer to it, not the text — amends D7's wording) and **`authoring-architecture`** (living system view, on `principal-architect`, fired at plan/implement landings on structural change). No new commands; no new agents.

**Rationale:** Single-sourcing and riding proven carriers is the library's own doctrine; every new primitive is future maintenance surface. Acceptance was by-exception — the user contested only the agents row (→ D12) and let the rest stand.

**Confidence:** `Confident`.

**Review folds:** **(RI-7)** Reference-presence is graded, not assumed: `validation-command-shape`'s deterministic grep floor gains the module-reference check, enumerated over the five carrying commands (brainstorm, specify, plan, implement, setup) — the declare-but-never-wire failure map §2 documents gets a grader. **(DQ-9)** The two skill boundary-sentence edits ride the build items (see D3 fold). **(V1, blocking — resolved)** The R5 × D11 collision reconciled: the module template is the **authoring source**; setup scaffolds a **project-pinned copy**; command landing steps reference the *project copy*; `validation-command-shape`'s grep floor asserts the project-copy reference string; plugin upgrades reach pinned projects only as R5 amend offers. This decision's "single-sourced in the module template" reads as authoring-time single-sourcing, never runtime resolution. **(RS1)** The project copy's path named (lead-formulated, `Assumed`): `.mochiko/memory/knowledge-management.md` — the grep floor asserts *that* reference string; `brainstorm.md:20`'s invariant-source reference joins the build-edit list (retargeted from the plugin template to the project copy) alongside its ROADMAP retarget; and "brainstorm's proven pattern" refers to the *step-exists-at-the-boundary* pattern, not its plugin-relative path resolution, which the V1 order now forbids.

### D12 — Report-writer agent: prior ruling held

**Statement:** No report-writer / scribe agent. The user's proposal for a cheap cross-workflow report-writer was checked against `model-tiered-seats` (2026-07-24): D3 closed the scribe as a non-avenue (its target surface doesn't exist; mechanical writes are turn-shaped, staying on the lead's pen) and D5 defers all seat-tiering to a dedicated brainstorm gated on cheap-tier *reliability* evidence. **User ruled: stick.** One fix lands as a build item: the re-open condition ("a future workflow grows a genuinely expansion-heavy document surface") currently has no named trigger owner anywhere — it gains a **named watch item** riding this module's dogfood, with CHANGELOG (the one genuinely derivable-from-artifacts doc, elective) as the surface most likely to trigger it.

**Rationale:** The fact-check quoted both prior rulings verbatim; the new doc set does not meet the recorded re-open condition today — trail/index/decision-row writes are exactly D3's lead-pen writes; ARCHITECTURE/DIRECTION are judgment work.

**Confidence:** `Confident` (user-ruled with the conflict laid out plainly).

**Review fold (DQ-15):** the watch item's trigger surface attaches to the **groom pass** (always present), not the elective CHANGELOG — an elective decline must not orphan the watch.

---

## Review

**Sizing:** user ruled **full pair** (decision-quality + record-integrity lenses). Counts formed independently: reviewer-dq 16 (3C/8I/5M), reviewer-ri 19. Cross-examination one-shot exchange run per protocol. reviewer-ri's map sample audit (~20 claims) surfaced challenges mid-exchange; routed to the fact-checker. Errata below is the fact-checker's verbatim response — it supersedes the challenged map text above.

### Fact-checker errata (verbatim — supersedes challenged map entries)

*Verified against the repo at HEAD `7920ccb`. Challenges 1, 2, 3 and the line-count half of 5 are **right** — corrections below supersede the map text. Challenge 4 is **wrong** — the disputed text is in both files. Challenge 5's first half asks about a count the map never asserted; the facts are given anyway. Item 6 is new and answered.*

#### E1 — §1's "Recommended Next Steps" claim: challenge upheld, correction is three-of-four

**The challenge is right on both points.** ROADMAP.md's item 4 (`:245`) is live, and it is live on all three of its sub-clauses. Verbatim: "4. **Dogfood, then generalize.** After `setup`+`specify` run for real, extract the crystallized conventions into the playbook and CLAUDE.md, and re-evaluate the deferred-kernel ledger entry."

- *Dogfood setup+specify for real* — both still open: `BACKLOG.md:397` "- [ ] **Dogfood `/mochiko:specify` for real (behavioral validation).**" and `BACKLOG.md:453` "- [ ] **Dogfood `/mochiko:setup` v3 for real (supersedes the v2 dogfood check).**"
- *Extract crystallized conventions into the playbook and CLAUDE.md* — not done, and the destination is partly gone. `CLAUDE.md:62` is still headed "## Skill-library conventions (evolving)"; `:64` still reads "These **will be extracted** from real workflows as they're built"; `:71` still closes "Document conventions here as they crystallize from `setup` and `specify`." A concrete unpropagated delta the challenge did not cite: **`CLAUDE.md:66-69` lists four axes; `ROADMAP.md:96-100` lists five** — axis 5, "**Producer↔validator pairing**", was promoted in the 2026-06-27 rewrite and never reached `CLAUDE.md`. And "the playbook" no longer exists: `ROADMAP.md:80` records "the `PLAYBOOK.md` doctrine doc — **deleted** once the migration landed"; `ls PLAYBOOK.md` → No such file.
- *Re-evaluate the deferred-kernel ledger entry* — `ROADMAP.md:57` still reads "| Kernel / code | **Deferred — code-free until dogfooding** |", and `BACKLOG.md:426` still carries "- [ ] **The parallelism deferral is now a live `deliberate-shortcut-ledger` candidate.**"

**Corrections to §1.** Replace "all four completed" with: **three of four are complete but unmarked (items 1–3); item 4 remains live on all three of its sub-clauses, with one of its two named destinations (`PLAYBOOK.md`) deleted.** And **"Zero of the document's sections are roadmap-forward" is withdrawn** — it overreached. The accurate statement: Recommended Next Steps is a structurally forward-looking section that has never been revised, so it now mixes three silently-completed items with one still-live item whose target no longer exists. That is a weaker claim than the map made, and it is the one the evidence supports.

#### E2 — §1's dangling-citation counts: challenge upheld; corrected counts and the rule

**The challenge is right.** The map's figures came from an inconsistent rule — I counted bare-name occurrences of `agent-skills-research` alongside path-prefixed occurrences of `human-in-loop/`, which inflated two of the four numbers.

**Counting rule now applied:** a dangling citation is a reference that **names a filesystem path inside a removed submodule** — something a reader could attempt to open and would fail. Excluded: the project name used in prose; the removal disclosure itself; and the `git submodule add` restore commands, whose trailing tokens are target-directory arguments, not citations.

| Doc | Map said | Correct | Path-shaped citations |
|---|---|---|---|
| `BACKLOG.md` | 2 | **2** ✓ | `:13` `agent-skills-research/synthesis/my-framework.md` · `:378` `human-in-loop/plugins/humaninloop/templates/codebase-inventory-schema.json` |
| `ROADMAP.md` | 3 | **2** | `:5` `agent-skills-research/synthesis/my-framework.md` · `:29` `[human-in-loop](human-in-loop/)` |
| `CLAUDE.md` | 5 | **3** | `:5` `[human-in-loop](human-in-loop/)` · `:24` `human-in-loop/plugins/humaninloop/` · `:25` `agent-skills-research/synthesis/my-framework.md` |
| `REGISTRY.md` | 0 | **0** ✓ | none — `:3` "Track every primitive from `human-in-loop`" is a bare project name |

Excluded and why: `ROADMAP.md:88` mentions `agent-skills-research` only to record that a pointer to it **was repaired** ("Loop-discipline's dead pointer to the removed `agent-skills-research` submodule repaired.") — counting it as dangling inverted its meaning. `CLAUDE.md:13` is the removal disclosure; `:18-19` are the restore commands, as the challenge states. `ROADMAP.md:35,56,59,94,129` and `CLAUDE.md:39` are prose project-name uses.

**One fact that strengthens the challenge past its own claim:** of the 7 corrected citations, the **3 in `CLAUDE.md` are explicitly guarded** — `CLAUDE.md:15-16` warns "the paths below will not exist until the submodules are re-added", and `:24-25` sit directly under it. The **4 unguarded** ones are `BACKLOG.md:13`, `BACKLOG.md:378`, `ROADMAP.md:5`, `ROADMAP.md:29`. So the count that matters for a reader hitting a dead path without warning is **4, not 10** as the map's arithmetic implied.

#### E3 — §6's "last three commits" deltas: challenge upheld exactly

**The challenge is right, including its replacement figures.** The three-commit span is `6626d32` → `7920ccb` (i.e. from the parent of `bbcd67b`):

| | 6626d32 | HEAD 7920ccb | true 3-commit delta | map's figure | what the map actually measured |
|---|---|---|---|---|---|
| BACKLOG | 69,206 | 74,796 | **+5,590** | +3,892 | `f7e9517`→HEAD = **one** commit |
| ROADMAP | 96,629 | 104,791 | **+8,162** | +4,269 | `bbcd67b`→HEAD = **two** commits |

Replace §6's final bullet with: **"Direction of the last three commits (`6626d32`→`7920ccb`): BACKLOG +5,590 B, ROADMAP +8,162 B, REGISTRY +0 B."** The correction moves the finding in the same direction the map argued — growth is faster than reported, not slower.

#### E4 — §5's attribution of the "no-graduation" parenthetical: challenge is incorrect

**Refuted. The text appears in both files, and the map's attribution was right.** `grep -rn 'no-graduation'` over `plugins/` returns two hits:

- `plugins/mochiko/templates/constitution-modules/knowledge-management.md:52-53` (wrapped across two lines, which is likely why a single-line grep missed it): "every index entry marked accepted names where its outcome landed (a `ROADMAP.md` row, or an explicit no-graduation)."
- `plugins/mochiko/commands/brainstorm.md:20`: "naming where the outcome landed (a ROADMAP row, or an explicit no-graduation)."

The two differ only in `` `ROADMAP.md` row`` vs `ROADMAP row`. The map cited the module at `:48-58` for the invariant (correct — `:52-53` is inside that range) and cited `brainstorm.md:20` separately in §2 for the command-carrier wording (also correct). **No correction needed.** A third, non-identical restatement of the same invariant exists at `knowledge-management.md:87` in the validator checklist fragment ("accepted entries name their landing").

#### E5 — Small counts: one correction, one clarification

- **`knowledge-management.md` is 88 lines, not 89** — challenge upheld; `wc -l` = 88, `wc -c` = 6,430, final line `-->`. §5's parenthetical should read "(88 lines, 6,430 B)". Every line citation the map made against this file (`:4-9`, `:23-28`, `:25`, `:27`, `:28`, `:31-32`, `:36-46`, `:44`, `:45-46`, `:48-58`, `:56-58`, `:68`, `:87`) was verified and remains correct — the error was the total only.
- **The "Landed: ROADMAP…" count: the map never asserted 22 or 20.** §3 said only that landing is named per entry and quoted `index.md:21` as an example. The facts: **22** `- **Landed:**` lines, one per entry, matching the 22 entries. **18** mention ROADMAP. **21** name some landing; the 22nd (`:15`) is the open session, reading "*(session open)*". The four not mentioning ROADMAP are `:15` (open session), `:115`, `:151`, `:157` — each of the latter three names a different landing (a plugin version, `PLAYBOOK.md` and the transformer cluster, agent personas and convention sections). So no entry lacks a landing, and 18/21 landed entries graduated to ROADMAP.

#### E6 — NEW: composition of the +15,475 B BACKLOG regrowth (`8463c1b` → `7920ccb`)

**The regrowth is new content, not groom-compressed entries re-expanding.** Diffstat: **186 insertions, 29 deletions** — a 6.4:1 add-to-delete ratio.

| | 8463c1b (groomed floor) | 7920ccb (HEAD) | Δ |
|---|---|---|---|
| bytes | 59,321 | 74,796 | +15,475 |
| lines | 400 | 557 | +157 |
| open `[ ]` | 60 | 72 | **+12** |
| done `[x]` | 43 | 49 | **+6** |
| `##` sections | 20 | 23 | **+3** |

The three sections are net-new, none of them a restoration: "Team-method-vs-command-shape build items (… 2026-07-25)", "Skill-succinctness strip pass (… 2026-07-25)", and "Kinako mvp-thin-loop validation pass (2026-07-24)". *(Note the third: its header is dated 07-24 but it entered the file after the 07-25 groom — header dates are event dates, not landing dates.)*

**The groom itself deleted nothing.** `d956fe0` → `8463c1b`: 84 insertions / 82 deletions, sections 20 → 20 (only one header shortened), open 61 → 60, done 42 → 43 — a single item flipped open-to-done. So it cut 33,055 bytes purely by compressing prose *inside* existing entries while leaving the item inventory intact. Nothing was archived or dropped, which is why no re-expansion was available to happen.

**On the ~2× byte alarm: the observed history does not support it.** HEAD against each candidate baseline: **1.26×** the groomed floor (59,321), **0.81×** the pre-groom peak (92,376), **0.97×** the window start on 2026-07-21 (77,468). Nothing in the 18-commit window reaches 2× of any of these. If the record carries a ~2× figure it needs its baseline named or the claim dropped.

**What the numbers do support**, stated at the strength the evidence bears: the groom compressed prose without reducing the item inventory, and within one day the file regained 26% by adding 12 open items, 6 done items and 3 sections. The open-item count is now **72 — its highest in the observed window** (it was 60 at the groomed floor and 61 pre-groom). That is an item-accretion finding, not a byte-bloat finding, and it holds regardless of which byte baseline is chosen.

### Combined tally and verdict (lead's merge)

Raised: 35 (dq 16, ri 19) → survived cross-exam: 33 (dq 15: 2C/10I/3M · ri 18: 11I/6M + 1 substrate correction attached) → **merged survivors: 29 (2 Critical / 19 Important / 8 Minor)** after lead-merged duplicates (DQ-7→RI-1, DQ-14→RI-6, DQ-13→RI-4; DQ-16 withdrawn into RI-9/RI-18). Fallen findings retrievable from the reviewers on ask.

Reviewer recommendations: dq `critical-gaps` (D7.2's rationale defect + two re-decisions), ri `needs-revision`. Both Criticals were re-ruled **in-session** by the user (R1, R2 below) and folded. **Lead's clearing verdict: ready, pending the verify pass.**

**User rulings (batch R1–R7, all adopted):** R1 omission-drift caught only at named boundary invariants, mochiko compliance manual, dogfood gates on injection probe, D7.2 rationale corrected (closes DQ-2). R2 supersession joins the landing ritual; status-agreement invariant (closes DQ-3). R3 REGISTRY's 13 open rows dispositioned at migration; `authoring-roadmap` abandoned (closes RI-1/DQ-7). R4 horizon caps Now ≤5 / Next ≤7 / Later ≤10 (closes DQ-5). R5 invariants project-pinned; upgrades as amend offers (closes DQ-8′). R6 core-whole + elective per-doc opt-in, superseding no-inner-menu (closes DQ-10). R7 per-open-item bound + item-count watch replaces the ~2× byte alarm (closes DQ-11, on E6).

### Survivor dispositions (29/29)

| # | Survivor | Disposition | Where folded |
|---|---|---|---|
| C1 | DQ-2 enforcement blind to omission; carriers unexercisable in dogfood | user-ruled (R1) | D6 + D7 folds |
| C2 | DQ-3 two mutable status heads, no supersession ritual | user-ruled (R2) | D6 fold |
| 1 | RI-1+DQ-7 REGISTRY rows undisposed; "living inventory" false | user-ruled (R3) | D8 fold |
| 2 | RI-2 n=1 hand-authored evidence base uncaveated | resolved | D9 fold (accepted-risk line) |
| 3 | RI-3 D4 names a live section dead | resolved (dq dissent recorded; triage moots it) | D4 fold |
| 4 | RI-4+DQ-13 brainstorm.md:20 omitted; historical pointers | resolved | D4 fold (migration surface + stable archive path) |
| 5 | RI-5 top-level reservation silently redefined | resolved | D4 fold; placement residue in D3 fold (`Assumed`) |
| 6 | RI-6+DQ-14 trail fails admission rule | resolved | D5 + D10 folds (read-job named) |
| 7 | RI-7 reference-presence ungraded | resolved | D11 fold (validation-command-shape grep floor) |
| 8 | RI-8 streak counter abandoned | resolved | D6 fold (honesty note) |
| 9 | RI-9 (absorbs DQ-16) unmarked rulings, missing rationales | resolved | D8/D9/D10 rationale fields + marks |
| 10 | RI-10 rejected roads unrecorded on D5/D6 | resolved | D5 + D6 folds |
| 11 | RI-12 provenance-pointer × dead-pointer-scan vise | resolved | D5 fold (`provenance: unrecoverable` terminal state) |
| 12 | DQ-1 BACKLOG read-job unstated | resolved | D5 fold |
| 13 | DQ-4 decisions-index granularity/retirement unstated | resolved | D3 fold |
| 14 | DQ-5 one-screen carried by non-enforcing tier | user-ruled (R4) | D2 + D6 folds |
| 15 | DQ-6 zero rejected roads on D6; groom-boundary reframe | resolved (steelman withdrawal recorded) | D6 fold |
| 16 | DQ-8′ governance mutates on plugin upgrade | user-ruled (R5) | D7 fold |
| 17 | DQ-9 project-scope schema unowned | resolved | D3 + D11 folds |
| 18 | DQ-10 whole-bundle rule vs elective tier | user-ruled (R6) | D9 fold |
| 19 | DQ-11 groom-overrun reading untested | fact-settled (E6) + user-ruled (R7) | D6 fold |
| m1 | RI-11 Later link-rule accretion | resolved | D2 fold |
| m2 | RI-13 open questions' destination | resolved | D4 fold |
| m3 | RI-14 dangling-citation counts | resolved by errata E2 | (map superseded in place) |
| m4 | RI-15 "last three commits" bases | resolved by errata E3 | (map superseded in place) |
| m5 | RI-17 count slips (20/88/247) | resolved | D4 fold + errata E5 + Verify-pass V8 note (247) |
| m6 | RI-18 no provenance line | resolved | header provenance line |
| m7 | DQ-12 byte-alarm baseline homeless | resolved | D6 fold (last-groomed stamp carries values) |
| m8 | DQ-15 watch rides elective surface | resolved | D10 + D12 folds |

### Verify pass

**Round 1 (reviewer-ri): NOT CLEAN.** All 29 dispositions present and substantively responsive; twelve defects introduced by the folds themselves — **V1 blocking** (R5 project-pinning × D11 template-single-sourcing collision, incl. an undefined grep-floor target), V2–V5 Important (D2 rationale still citing the E1-withdrawn premise; REGISTRY archive's read-job asserted not named; core bundle enumerated twice differently under R6; D3's supersession compression breaking R2's per-decision anchor), V6–V11 Minor, V12 observation (four Statements retaining withdrawn clauses unmarked). The pass also recorded ri's own RI-16 as corrected by errata E4, and re-verified E3/E5/E6 figures. Tally arithmetic confirmed: 33 surviving − 3 duplicate merges − 1 attached substrate correction = 29.

**Repairs applied (lead's pen):** V1 → resolution order in D11 fold + D3 schema clause (template = authoring source; setup scaffolds the project-pinned copy commands reference; grep floor asserts the project-copy string; upgrades only as R5 amend offers). V2 → D2 rationale restated in E1's terms. V3 → REGISTRY archive read-job named (provenance query) in D8 fold. V4 → core enumerated once in D9 fold (six artifact groups + the D7 enforcement surfaces, 2 electives). V5 → one supersession line *per decision*, status preserved (D3 fold). V6 → RI-13 items reconciled in place, not duplicated. V7 → "only" struck; `knowledge-management.md:52-53` added to the migration surface. V8 → the 247/248 line-count slip acknowledged here as the map's known line-figure imprecision (bytes exact — E-family pattern); no design value rests on it. V9 → "decisions layer" → "trail" in D5's option-B rejection. V10 → groom tier clause (fix-on-sight-invoked on cap/bound trips, else minimal). V11 → stable archive paths named (`.mochiko/archive/ROADMAP.md`, `.mochiko/archive/REGISTRY.md`, `Assumed`). V12 → inline markers added to the four withdrawn/superseded Statement clauses (D6.3, D7.2, D8, D9).

**Round 2 (reviewer-ri, bounded delta-confirm on V1–V12): unblocked — 12/12 repairs landed, none reversing a ruling.** Eight confirmed clean; residue RS1–RS2 Important (V1's repair left the project-copy path and `brainstorm.md:20`'s invariant-source retarget unnamed; V4's authoritative core enumeration omitted the D7 enforcement surfaces), RS3–RS6 Minor bookkeeping. Reviewer's verdict: fold RS1/RS2, sweep RS3–RS6, then clean by the record-integrity lens — no third verify round warranted.

**RS repairs applied (lead's pen):** RS1 → project-copy path named `.mochiko/memory/knowledge-management.md` (`Assumed`); `brainstorm.md:20` invariant-source retarget added to build edits; "proven pattern" scoped to step-existence (D11 fold). RS2 → D7 enforcement surfaces named core, not separately declinable (D9 fold). RS3 → D10 cross-reference fixed. RS4 → core count corrected in the round-1 summary. RS5 → m5 cell completed. RS6 → dual archive conventions flagged, one-home preference recorded (D8 fold). **Spot-check on RS1/RS2 folds: both confirmed as specified** (RS3 visible in scope and correct; RS4–RS6 taken on the lead's word, outside the named spots). One non-blocking note recorded: the project-pinned copy is a fourth scaffolded artifact not listed in RS2's illustrative three-surface parenthetical — the chain (R5 scaffold-time pinning × RS2 core surfaces) reaches it; the list is not closed.

**Final verify verdict: CLEAN.** All four fitness items that blocked `ready` on the cold read are satisfied (rationales, marks, rejected roads, provenance); no repair across the three rounds reversed a ruling or left a surviving contradiction. **Lead's clearing verdict: `ready`.** Combined review bound honored: one cold read per reviewer + one-shot cross-exam + verify (round 1 + bounded delta + two-spot check).
