---
report: review
pass: cold
pairing: solo
lens: decision-quality + record-integrity (solo seat carries both)
map_timing: end
record: .mochiko/brainstorms/human-readable-ids/record.md (frozen 2026-10-08, D1–D20, F1–F23)
status: critical-gaps
verdict: FAIL
raised: 24
survived: 15
severity: { critical: 1, important: 8, minor: 6 }
coverage_findings: 2
verify_round_1: NOT CLEAN (1 blocking fold-introduced defect, 4 non-blocking)
verify_round_2: NOT CLEAN (0 blocking, 2 non-blocking new defects; V1–V5 closed)
---

## Failure narrative

One Critical and eight Important survivors. The Critical one is a broken load-bearing claim. D15 rules that an ID-token-only rewrite is non-semantic because "the line's content does not leave". That claim fails for verbatim quotes. D14 rewrites every session record and every strip, and those files hold at least 213 double-quoted spans carrying GI/D IDs, across 84 files. Many of them quote sources that stay bare by ruling: the migration log, the archive, git history, and the user's own words. Rewriting them falsifies the quote, and D15's own guard (the diff check) passes them, because only ID tokens change. This is silent corruption of the record layer under GI-005-record-layer-integrity (NON-NEGOTIABLE).

The Important set clusters in two places:
- The back-fill's layer boundary (D14): the replayed rule text, and the genesis fixture corpus that `cargo test` compares against.
- The mechanical tools' inputs (D4/D6/D7/D10/D12/D18): per-family scope, stale-slug detection, `C<n>` collisions, and file-name slugs.

Each one leaves a builder with a question the record does not answer. Resolution is cheap for most of them: a clause or an allowlist entry. None reopens `Contested` D3.

## Survivors

### Critical

**S1 · non-coverage · D15, D14 (and D7's diff check) · broken load-bearing claim.**
- **Scenario.** D7's tool applies D14's map to this record.
  - F9's quote `"anchor: 2026-09-03 cli-schema-delivery D9"` names a session, so under D11 the tool rewrites it to `cli-schema-delivery D9-<slug>`.
  - The migration file it quotes stays bare (D12, D14).
  - The quote is now false. D15's diff check passes it, because only an ID token changed.
  - The same happens to F4's quote of `0019-setup-agnostic-modules-out.yaml:49`, to F13's `git show dc01941` quote, and to strips quoting removed primitive text (GI-006-primitive-edit-traceability reconstruction).
- **Size.** Floor count `grep -rhoE '"[^"]{0,200}\b(GI-[0-9]{3}|D[0-9]{1,3})\b[^"]{0,200}"'` over `.mochiko/brainstorms/*/record.md` and `.mochiko/strips`: 213 spans in 84 files. Blockquotes and multi-line quotes are not counted.
- **Resolution.** Add a quoted-span clause to D14/D15:
  - Verbatim quotes keep their source's form.
  - The tool skips spans inside quotation marks, blockquotes, and code fences.
  - D18's allowlist names quoted spans.
  - Alternative: rule that only quotes of live-layer sources follow the rewrite. The user rules which.

### Important

**S2 · coverage (blind map E2, load-bearing) · D12, D14, D18.**
- **The diff.** The map folded "shipped rule text that cites IDs changes only by new migrations" into Machine-read spots. D12 rules `anchor:` lines only. D14 then freezes "the migration log (already bare by D12)". That is a wrong premise: D12 does not cover rule bodies.
- **Materiality.** Rule text seats read at every skill fire stays bare forever:
  - 140 non-anchor lines in `plugins/mochiko/migrations/*.yaml` carry `D<n>`.
  - 80 lines in `.mochiko/schema-views` do, e.g. "setup-product-agnostic D2", "withdrawn by delta D4".
  - Rules minted after landing carry the joined form, so rendered rules come out mixed.
  - D18's done check over "the live layer" hits the replayed views, and no allowlist entry names them.
  - This is the seat half of the frame's reader.
- **Resolution.** The user rules one of two:
  - (a) Rule text in replayed history stays bare, named in D14 and in D18's allowlist, with the cost stated.
  - (b) Reword migrations for the rules that cite IDs, sized first.

**S3 · non-coverage · D14, D18.**
- **Contradiction.** The map change after D4 says "Back-fill gains … the frozen fixture corpus" (from F10). D14's statement then lists "crate comments and tests" as live, and its history-layer list omits `crates/mochiko-cli/tests/fixtures/genesis-corpus/`.
- **Scenario.** That corpus is the frozen expected side of the genesis fidelity test (`crates/mochiko-cli/tests/fidelity.rs:1-14`). 21 of its files carry GI/D tokens. Rewriting them while the genesis migration stays bare turns `cargo test` red, and gate 6 blocks the bump. Calibrated eval fixtures whose labels were frozen against bare text carry the same risk.
- **Resolution.** Add the fixture corpus, and any calibrated or labelled eval stimulus, to D14's history layer and D18's allowlist.

**S4 · non-coverage · D4, D7 (F17, F18, F20).**
- **Broken dependency.** D7's accepted risk says "which families restart per artifact is in the `fact-id-families` enumeration". F20 landed with no scope or restart column, and the full report was not persisted.
- **Scenario.**
  - D4 keys identity on the number "within its owning scope (spec, record, or project)".
  - D7 rewrites mentions in the owning file plus mentions "that name that scope explicitly".
  - D11 gives an explicit qualifier to session D only.
  - So `FR-012` cited in `tasks.md`, a cycle report or `gates.md` names no spec. A rename in spec A either misses those mentions, leaving stale slugs D18 never sees (S7), or, if the builder widens scope, rewrites other specs' `FR-012`. That is the silent corruption D7's Why names.
- **Second contradiction.** D4's title says "the number is the stable key". F18 says `C-`/`D-`/`IP-`/`INT-`/`DS-XXX` numbers renumber after landing, and D7 has no number-rename path.
- **Resolution.** Add a per-family table: owning scope, restarts y/n, renumbers y/n, cross-file qualifier form. The qualifier rule for FR/SC/US/cycle citations outside their spec or tasks file is a user ruling, as D11 was.

**S5 · non-coverage · D12 vs D19.**
- **Contradiction.** D12: "File names that already carry a slug … keep it, and that slug is the ID's slug". D19: "exactly three lowercase ASCII words".
- **Evidence.**
  - Plugin-shaped files today: `FEAT-003-wallet.md` (1 word) and `FEAT-001-family-accounts.md` (2), under `evals/review-specifications/fixtures/g3-lunch-orders-ux-manifest/tally/.mochiko/features/`.
  - `scr-001-week-menu.html` (2 words, lowercase prefix).
  - F18's `AX-002-mandated` (1 word).
- **Scenario.** The builder must either rename every such file (D12 says keep) or let joined IDs carry 1–2-word slugs (D19 says no). The D6 checker and D7's file move need one answer. The `scr-` versus `SCR-` case mismatch also breaks "same slug" matching.
- **Resolution.** Rule which wins. Cheapest: D19 governs new mints, and existing file slugs are grandfathered as the ID's slug, with matching case-insensitive on the prefix.

**S6 · non-coverage · D10 vs D6, D18.**
- **Contradiction.** D10 puts cycle cards in scope (`C1-walking-skeleton`) and keeps report labels bare (`G1`/`C1`/`A1`/`F1`). That is the same lexical form, `C<n>`.
- **Scenario.**
  - A cycle report cites cycle `C1` and uses its own `C1` label.
  - The mechanical check cannot tell them apart. It either flags every report label (noise, and D18 never exits clean) or exempts every bare `C<n>` (cycles never checked).
- **Resolution.** Rename the report-label prefix, scope the check by file kind, or drop cycle cards from the joined set. The user rules.

**S7 · non-coverage · D6, D18, Untested bets · honesty about the open.**
- **Contradiction.**
  - D6's accepted risk: "Catching a stale slug under D4 needs a source for the current slug (the Rename mechanism's question)".
  - D7 answers with a rename tool, not a detector.
  - D18's check counts bare IDs only.
  - Yet Untested bets claims the slug-stays-true bet is "tested by D18's CI report showing no stale slugs over time". No ruled check sees a stale slug.
- **Scenario.** A partial apply, a hand edit, or S4's scope miss leaves "`GI-020-additive-plugin-install`" beside a definition reading `GI-020-plugin-install-model`. CI stays clean.
- **Resolution.** Extend D6's check: a joined mention whose slug differs from its in-scope definition's slug is reported. This is mechanical, and the definition site is the source D7 already relies on. Otherwise, move the bet to OQ as untested.

**S8 · non-coverage · D16.**
- **Gap.** "Graders accept a bare ID that predates the upgrade". Nothing tells a grader when an ID was minted. IDs carry no mint date, and spec or ledger dates are not upgrade-relative.
- **Second gap.** Unruled: a new artifact in a user project citing an old bare "`GI-004`". D2 demands the joined form, but no slug exists at the definition, so the citing seat would coin one. That is a second source of truth, which D7 rejected.
- **Scenario.** Kinako after upgrade: a grader either fails pre-upgrade artifacts and wedges a run, or accepts every bare ID, and enforcement is void.
- **Resolution.** Rule a test (e.g. a bare ID whose definition is bare is accepted, and its citations stay bare) and the citation form for old IDs.

**S9 · non-coverage · D8 (and D3's accepted risk).**
- **Gaps.**
  - (a) No baseline. The subject is the repo's author. If they "know" 15 of 20 bare IDs already, a pass says nothing about the slug.
  - (b) Sequencing. D8 fences only "the mass rewrite". The crate verbs, the 21-family minting-rule edits and their migrations are not ordered against it, so a fail can land after shipped primitives already mint joined forms.
  - (c) D3's accepted risk says "the bet is tested across every family at once". D8 samples GI, D and one product family.
- **Resolution.**
  - Add a bare-first arm: the user marks the 20 bare IDs, then the slugged ones, and the pass is measured on the gain.
  - Run D8 on a hand-coined map before any crate or primitive change.
  - Restate D3's risk line to match.

### Minor

**S10 · non-coverage · D4 vs D13.** D4 renames "when the meaning … changes". D13 says a topic slug survives a meaning flip (`GI-020-plugin-install-model`). The rename trigger is ambiguous: meaning or topic. **Resolution:** amend D4's statement to say topic.

**S11 · non-coverage · D5, D10 examples vs D19.** `D10-governance-envelope` (D5) and `C1-walking-skeleton` (D10) are two-word slugs, and D19 says exactly three. Examples train seats. **Resolution:** fix the examples.

**S12 · coverage (blind map A5, not load-bearing) · D19.** Only letter sub-IDs are ruled (`D2a-…`). Unruled: dotted `D10.1`–`D10.7` and `D4.1`, the latter in live rule text (rendered `authoring-constitution`), and parenthesised `D5(ii)` in `.mochiko/schema-views`. **Resolution:** a one-line grammar default.

**S13 · non-coverage · D10 confidence mark.** The repo-only extension of `Contested` D3 was "named to the user as needing their word", then "taken as confirming" from a batch "default ok". `Confident` is generous. **Resolution:** an explicit one-line confirm, or mark the extension `Assumed`.

**S14 · non-coverage · D7 Why vs OQ2 and D13 · excess.**
- D7's Why asserts "renames will be frequent, requirement IDs most of all" as fact. OQ2 says that frequency is unknown, and D13 (later) cuts renames.
- **Cheaper shape:** a throwaway apply script for D14, plus the stale-slug check (S7) guiding hand renames, with no standing kernel-class verb.
- D16's later opt-in pass may still justify the tool.
- **Resolution:** restate D7's Why on the D14 and D16 apply need, and cite OQ2.

**S15 · non-coverage · F20, F9 · fact-map integrity.**
- F20's full report is unpersisted. Its "16 rows, 21 prefixes" cannot be verified: 15 rows are visible. D10 then says "the 21 plugin prefixes (F20), session `D` cards, cycle cards", which reads as a double count. The D6 checker needs the exact list.
- The repo-only list omits durable forms present today: `J-` in strips, `FP-` in `.mochiko/memory`, `DQ-`/`RI-` in records.
- Line drifts: F9 cites `model.rs:1293`, the quote is at `:1292`. F20 cites `authoring-user-stories.yaml:151`, the quote is at `:157`.
- **Resolution:** persist the enumeration under the session's `research/`, or inline the closed list; fix the two cites.

## Card parts

All 20 cards carry Statement · Why · Rejected roads · Accepted risk · How it was decided · Changed at review (`grep -cE` = 20 each).

## Fitness (cited evidence)

- **Self-contained:** partial. F20 rests on an unpersisted scratchpad report (record l.224). See S15.
- **Decisions attackable:** yes. Each Why is concrete (e.g. D4 via F13's GI-020-plugin-install-model replay).
- **Decision trail present:** yes, as how-decided lines and ratified-run counts, l.447–891. The Review section is still owed.
- **Confidence marks honest:** mostly. D3 is `Contested` (l.480). D10 is generous (S13).
- **Rejected roads recorded:** yes, on all 20.
- **Honest about the open:** fails. Untested bets claims a stale-slug test that no ruling provides (S7, l.962–964).
- **Provenance stated:** yes, in the header (l.3–21).

## Fact-map audit (sample)

Sixteen claims checked.
- **Exact:** F3 (`0001-genesis.yaml:560`; `DECISIONS.md` 60 range lines) · F4 (`0019…:49,448,564`) · F8 · F11 (`QUALITY-CHECKLIST.md:100`) · F12 (`conform.rs:24-26`) · F13 (`dc01941` l.230; ledger l.581) · F15 (`conform.rs:9`) · F17 (`0001…:4738`) · F18 (`home.rs:258,293-294`; `implement.yaml:719`) · F19 (`is_decision_segment` rejects slug suffixes) · F21 (`validate-requirements.py:84`) · F23.
- **Approximate:** F16 (441/128 now vs 438/128) · F22 (380 now incl. this record vs 375).
- **Drifted:** F9 and F20 line cites (S15).

## Hunt ledger (ratified first)

Scenario stress plus the six classes per card. Findings by card; "clean" means hunted and nothing survived.
- **D1:** clean. The steelmen (I2 b/c/d/e) are recorded with reasons.
- **D2:** clean. Cost is stated (~5% of `CLAUDE.md`); the length caps are named.
- **D4:** S4 (renumbering families contradict the stable key) · S10.
- **D5:** S11. The list-of-four cut-off was raised and dropped as a preference.
- **D6:** S7 (deferred question never answered) · S6.
- **D8:** S9.
- **D12:** S2 · S5.
- **D13:** S10.
- **D14:** S1 · S2 · S3.
- **D15:** S1.
- **D16:** S8.
- **D17:** clean. Setup-regeneration overwrite is an accepted risk, tested by the amend run.
- **D18:** S2 · S3 · S6 · S7.
- **D7** (user's pick): S4 · S14.
- **D3** (`Contested`): not raised. The only new angle, D8's one-family sample, is filed against D8 (S9c).
- **D9:** clean.
- **D10:** S6 · S13 · S15.
- **D11:** clean.
- **D19:** S5 · S11 · S12.
- **D20:** clean. The binary-range coordination is held by the release gates.

## Coverage diff (blind map vs record)

- **Load-bearing:** 19. Answered in depth: 18. Partial: E2, which becomes S2.
- **Dropped angles** (G2, G4, G5, J3, F6): each is a reasoned ruling, accepted.
- **Non-load-bearing folds:** A5 is partial (S12). C6 (retired IDs) is unruled but absorbed by D4's supersede-in-place, so not raised.

## Builder test

Reading only the record, a builder would still ask:
1. Do quoted spans follow the rewrite? (S1)
2. Are rendered rules and schema-views slugged or allowlisted? (S2)
3. Is the genesis corpus live or frozen? (S3)
4. What is each family's owning scope and cross-file qualifier? (S4)
5. File slugs or D19: which wins? (S5)
6. How does the check tell cycle `C1` from a report-label `C1`? (S6)
7. What detects a stale slug? (S7)
8. What counts as "predates the upgrade", and how are old IDs cited? (S8)
9. When does D8 run relative to the crate and primitive work? (S9)

## Notes of note

- Tally: 24 raised, 15 survived; 4 merged into survivors; 5 dropped:
  - D20 binary range: release gates bind.
  - D5 list-of-four: preference.
  - D17 regeneration overwrite: accepted risk, tested.
  - D2 token cost: accepted risk, stated.
  - D12 bare marker vs grader string match: D4's number-keyed matching covers it.
- Pairing is solo, so there was no cross-examination. The verdict is input, and the lead owns the clearing. Findings enter the record through the lead's pen only.
- `record_contact` began only at message 2. The blind map stands unchanged at `reports/angle-map.md`.

## Verify round 1

Scope: only the changed parts and their gists — D3, D4, D5, D6, D7, D8, D10, D11, D14, D15, D16, D18 and D19; F9, F20 and F24; OQ4; Untested bets; the Review dispositions. D21 and D22 got the bounded round only (internal consistency and fitness). No fresh cold read and no coverage hunt.

**Result: NOT CLEAN.** Of the 15 survivors, 14 are closed and 1 has a fold-introduced defect. The folds introduced five defects: 1 blocking, 4 non-blocking.

### Per survivor

- **S1: closed.** D15's changed part: "a verbatim quote keeps its source's form — text inside `"…"`, a `>` blockquote or a code fence is never rewritten". D6 and D18's allowlist name these spans.
- **S2: closed** as the ruling D21, which carries V1 below.
- **S3: closed.** D14 R1 adds `genesis-corpus` and the eval stimuli to the history layer, and "eval fixtures" leaves the live list. D18's allowlist names both.
- **S4: closed.** F24 records per-family scope. I spot-checked it first-hand:
  - `tasks.yaml:10`
  - `feature-entry.yaml:67`
  - `authoring-architecture-store.yaml:109-110` ("unique store-wide")
  - `authoring-epic.yaml:77`
  - `ARTIFACT-TEMPLATES.md:226` ("the next free id in the store")

  D11's qualifier now covers the per-artifact families, D4 re-keys on a renumber, and D7 points at F24. See V3.
- **S5: closed.** D19 keeps existing file slugs, and the three-word rule binds new mints only. Prefix matching ignores case, and the D6 check accepts a short slug that matches the file name.
- **S6: closed.** D10 renames the label in `advocate-report-template.md:29` from `### C1:` to `### Q1:`; cycles keep `C`. The other `C{N}` heading in the plugin (`testing-end-user/references/REPORT-TEMPLATES.md:58`) cites a cycle gate. It is not a local label, so no collision remains.
- **S7: closed.**
  - D6 flags mention drift.
  - OQ4 holds meaning drift.
  - Untested bets now splits the bet into a mechanical half and a judgment half.
- **S8: closed.** D16 rules that the definition decides, and no seat coins a second slug.
- **S9: closed.**
  - D8 now has two arms, runs as wave 0 before any build, and passes on the gain over the bare arm.
  - D14 R2 reorders the method.
  - D3 R8 fixes the risk wording.
- **S10: fold-introduced defect V2.** The ruling is recorded, but D4's Statement (l.530, "When the meaning of the thing an ID names changes") and its title still say "meaning". R3 claims the trigger "reads … topic".
- **S11: closed.**
  - D5's example is now `D10-governance-envelope-supersessions`, and D10's is `C1-walking-skeleton-path`.
  - Every new example in the changed parts has three words: `FR-012-csv-report-export`, `D4.1-waiver-revisit-timing`.
- **S12: closed** as the ruling D22, which carries V4 below.
- **S13: closed.** The user gave an explicit "R5 yes".
- **S14: closed.** D7's Why now rests on the apply need and OQ2. The cheaper shape is recorded under Rejected roads, with its reason.
- **S15: closed.**
  - F9 now cites `:1292`, and F20 cites `:157`.
  - F20 gains a count note, and its repo-only list adds `J-1`, `HF-1`, `F-1`, `FP-1`, `FC-3`, `O-1`, `DQ-` and `RI-`.
  - D10 no longer double-counts.
  - The enumeration is still unpersisted, but the closed list is now inline.

### Fold-introduced defects

- **V1: blocking.** D21 contradicts D12 and D4, and D18 hides it.
  - D21 writes new rule text "in the joined form". But the log is immutable (D21's own Why), and D7 skips frozen surfaces.
  - So the first D4 rename of an ID cited there leaves a false slug in rules seats read every run. D4's Why says that is "worse than a bare number".
  - D12 kept anchors bare for exactly this reason: "A hashed anchor can never follow a D4 rename". Yet D21 says "as anchors do (D12)".
  - D18's allowlist exempts the whole log and `.mochiko/schema-views/`, so the drift check never sees the stale slug.
  - The user rules one of two resolutions:
    - (a) All migration-log rule text, new and old, cites IDs bare. D21 narrows to match D12.
    - (b) Keep D21 and add a duty: a rename of an ID cited in rule text ships a reword migration, and the D6 drift check reads the replayed views. The views stay allowlisted for bareness only.
- **V2: non-blocking.** D4's Statement and title contradict R3's "reads" (S10).
  - Fix: edit the Statement and title to say "topic", or reword R3 to "is amended to".
  - Same kind of leftover: D14's Statement keeps "(already bare by D12)", which its changed part supersedes through D21.
- **V3: non-blocking.** D4's renumber repair sends the mentions to D7's tool, but the tool's ruled shape (`ids rename <owning-file> <ID> <new-slug>`) changes only the slug. Separately, D15's ruling covers same-number rewrites only.
  - Fix: give D7 a re-key form (new number, slug carried over), and state whether a landing renumber rides D15's ruling.
- **V4: non-blocking.** D22's example `D4.1-waiver-revisit-timing` cites another session's decision from a shipped template with no session qualifier, against D11. Fix: qualify the example.
- **V5: non-blocking builder note.** Quoted spans wrap across lines; F9's own quote breaks after "anchor:". D15's skip must match quotes across line breaks, or the tool rewrites the tail of the quote.

### Bounded round: D21 and D22

- **D21:** all six parts are present. `Confident` on the user's ruling is honest, and the rejected road names its cost. The accepted risk names mixing but not staleness (V1).
- **D22:** all six parts are present, and the accepted risk is honest. Its two motivating forms both sit in text D21 keeps bare, so D22 governs new writes only. That is consistent, and no finding is raised.

## Verify round 2

Scope was the five fixes from verify round 1 only:
- **V1:** all seven parts of D21.
- **V2:** D4's title, Statement and V2 line; D14's Statement and V2 line.
- **V3:** the V3 lines in D7 and D15.
- **V4:** D22's Statement and Changed line.
- **V5:** the V5 line in D15.
- The verify-round-1 result paragraph in the Review section.

I also checked each fix against the other cards.

**Result: NOT CLEAN.** V1–V5 are all closed. There are no blocking defects. There are two non-blocking new defects, W1 and W2; both are one-clause text repairs, and neither needs a new user ruling.

### Per fix

- **V1: closed.** D21 now says, consistently across all seven parts, that every citation in the log stays bare, old and new.
  - Its Why gives D12's reason: "can never follow a D4 rename".
  - Option (b) is recorded under Rejected roads with its cost.
  - The Accepted risk is honest: seats meet bare citations in the rules for good.
  - It is consistent with D14 R1 and with D18's allowlist, so the drift check has nothing in the log to miss.
- **V2: closed for the named items.** D4's title and Statement now say "topic", and D14's Statement now reads "(bare by D12 and D21)". The same kind of leftover remains elsewhere in D14's Statement: see W2.
- **V3: closed.**
  - D7 gains `mochiko-cli ids rekey <owning-file> <old-ID> <new-ID>`, scoped and preview-first, and the slug travels with the entry.
  - D15 puts the renumber outside its same-number ruling.
  - The lead's reading, that renumbered entries are not yet cited in protected lines, is consistent with `.mochiko/schema-views/commands/implement.yaml:717-719`: at landing, in order, the second run renumbers its own entries before the landing flip.
- **V4: closed.** D22's example now reads `` `<session-slug>` D4.1-waiver-revisit-timing `` when cited from outside its record, per D11.
- **V5: closed.** D15's skip now matches a quoted span across line breaks.
- **Review paragraph: accurate.** It records 14 of 15 survivors closed, 5 fold-introduced defects with 1 blocking, and where V1 and V2–V5 were folded.

### New defects

- **W1: non-blocking. D21's new Statement against D1, D2 and D19.**
  - D21 says rule text in the log "cites IDs bare — the text already there and any rule a later migration writes". The plugin's minting rules are themselves rule text that later migrations write (D1).
  - Example: the governance-intent template's fixed definition line "`- **GI-001 — Type:** …`" (`.mochiko/schema-views/templates/governance-intent.yaml:63`). Under D2, that line must emit a joined definition in every user project.
  - Read literally, D21 keeps that line bare, along with the template's example citations (`US-1, US-3` · `SC-1, SC-2`, `templates/spec.yaml:220`). Seats copy examples, so they would keep minting bare IDs, which defeats D1.
  - D21's Why shows the intended scope: citations of decided IDs, which a rename can't follow.
  - **Fix:** add one clause to D21. It binds citations of decided IDs only. Minting grammar, placeholders, template definition lines and example IDs follow D1, D9 and D19.
- **W2: non-blocking. D14's Statement still contradicts its own Changed lines.** Same class as V2.
  - It still lists "eval fixtures" in the live layer, though R1 says "'eval fixtures' leaves the live list".
  - It still gives the old method order, "a second seat grades it; D8's read test runs on 20 of its citations", though R2 says D8 runs first, as wave 0, on hand-coined slugs.
  - **Fix:** carry R1 and R2 into the Statement, as V2 did for D4.
  - Not graded (out of scope): other cards amended only through their Changed lines may show the same pattern, for example D8's Statement.
