# Wave 1 — the crate: `mochiko-cli ids`

**Status:** closed 2026-10-08. Wave 0 ran (a cold seat, FAIL on clause 1; the user overrode the
gate and restated the bet, D8/D13 as changed at build). Build questions B1–B3 were ruled. P1 built
the tool; the code review passed at re-review 5 after four user-granted fix rounds (`build-log.md`).
Wave 1b (`rekey` by joined ID, `--alias` path scoping, and `rename`'s shared-number refusal) closed
2026-10-08: the review passed after one fix round (`build-log.md`). Wave 1c (a record path as a
qualifier, requirement 13) closed 2026-10-08: the review passed first time (`build-log.md`).

Branch `human-readable-ids-build` off `main` @ `bf410cc` (user: "ok"). Scope for this session: waves
1–3; wave 4 (setup amend, contract suite, bumps, tag) stays with the user (user: "1-3").

## What wave 1 builds — transcribed from the record

A new `ids` subcommand family in `crates/mochiko-cli`. Every requirement below cites the ruling it
transcribes; nothing here is new design.

1. **Grammar** (D9, D19, D22): the joined form is `<ID>-<w1>-<w2>-<w3>` — the ID at its family's
   padding (three digits for `-XXX` families; unpadded `US-<n>`, session `D<n>`, cycle `C<n>`), a
   sub-ID letter or dotted part attached to the number (`D2a-…`, `D4.1-…`), then exactly three
   lowercase ASCII words joined by hyphens, each starting with a letter. An existing file-name slug of
   any length is the ID's slug (D19 as changed at review, S5); prefix matching ignores case (`scr-001`
   is `SCR-001`).
2. **Families and scopes** (D3, D10, F20, F24): project-wide — `GI`, `FEAT`, `EPIC`, `AX`, `SPN`,
   `NFR`, `GAP`; per artifact — `FR`, `SC`, `US`, `SCR`, `FLOW` (per spec), `C`/`D`/`IP`/`INT`/`DS`
   (per product file), `BR` (per `data-model.md`), session `D` (per record), cycles `C<n>` (per
   `tasks.md`). Local labels stay bare and are never flagged: `T3.2`, `C3-gate-2`, run folders, report
   `G1`/`Q1`/`A1`/`F1`, survivors `S1`, a record's `F`/`Q`/`S`/`V` (D10).
3. **Definition sites** (D4 as amended, D16, S7): each family's definition is found where its minting
   rule puts it (F20's quoted rules) — e.g. the `governance-intent.md` lines (`- **GI-NNN — …**`,
   `| GI-NNN |`; the minting rule's "unique forever within this file"), `### D<n> — …` cards in a
   record, `**FR-NNN**:` lines in a spec, `# FEAT-NNN — …` entry titles. *(Corrected 2026-10-08 at
   P1's plan grade, F3: the example first named the ledger's `### GI-NNN — …` heading — the lead's
   transcription error; the ledger heads 11 of 22 GI IDs, `governance-intent.md` defines all 22.)* The number within its owning
   scope is the key (D4); the slug at the definition is the source of truth (D7).
4. **Qualifiers** (D11 as extended at review, S4): a per-artifact ID cited outside its owning file
   carries its owner's name in front (`` `lunch-orders` FR-012-csv-report-export ``,
   `` `cli-schema-delivery` D11-widened-kernel-admission ``). See build question B1.
5. **`mochiko-cli ids --check [<path>…]`** (D6, D18): advisory, exit code only, never a gate; reports
   (a) bare in-scope IDs — except a bare citation of a bare definition (D16 as changed at review, S8);
   (b) mention drift — a joined mention whose slug differs from its in-scope definition's (S7).
   Allowlist: ranges and lists longer than three (D5) · local labels (D10) · machine-read spots —
   `anchor:` lines, `<!-- GI-… -->` markers, number-only paths (D12) · verbatim quotes — `"…"`, `>`
   blockquotes, code fences, matched across line breaks (D15, S1, V5) · the history layer (D14, D21):
   `.mochiko/archive/`, the crate's replay fixture corpus, eval stimuli, released `CHANGELOG.md`
   entries, the migration log and `.mochiko/schema-views/`. Defaults plus repeatable exclusions so a
   user project and this repo both fit.
6. **`mochiko-cli ids rename <owning-file> <ID> <new-slug>`** (D7, D12, D15): rewrites every mention
   of one ID in its scope to the new joined form — bare to joined included (the back-fill, D14) —
   skipping the allowlist above; moves a file whose name carries the slug (D12). Previews by default
   (prints the diff); writes only with `--write`. Before writing, its diff check confirms every
   changed span is an ID token, and any other change blocks the write (D15).
7. **`mochiko-cli ids rekey <owning-file> <old-ID> <new-ID>`** (D4 as changed at review, V3): the same
   for a landing renumber — the slug travels with the entry.
8. **Bright line** (`CLAUDE.md:72`; `.claude/rules/mochiko/rust-cli.md`): the tool holds no judgment —
   it never coins a slug (the seat supplies it), never grades meaning, never dispatches agents; `--check`
   never blocks. D7 is this tool's recorded kernel-class admission; the ledger trace lands at D17's
   amend (wave 4).

*Requirement 5 narrowed 2026-10-08 (D18 as changed at build; user: "as recommded"): the check reports
only mentions it can tie to a definition — an ID with no definition in the indexed tree, or a
per-artifact or session mention with no resolvable owner, is never reported and never rewritten.*
*Requirement 5's history layer widened 2026-10-08 (D14 as changed at build; user: "as recommded"):
the template goldens `crates/mochiko-cli/tests/fixtures/template/` join the default excludes —
renders of the log, compared byte for byte.*

## Seats (sound loop, `mochiko:patterns-sound-loop` — floor tripped: crate code is a governing surface)

- **P1 — producer:** `mochiko:staff-engineer` (persona default tier), TDD. Plan-only dispatch first,
  read-only, then stops.
- **Plan grade:** a fresh `mochiko:staff-engineer` peer on `mochiko:review-seat-plan`.
- **Code review:** a fresh seat that wrote none of the code — `mochiko:tech-lead` on the diff, plus the
  four crate layers run first-hand: `cargo test --all`, `cargo fmt --all --check`,
  `cargo clippy --all-targets -- -D warnings`, `cargo audit --deny warnings`.
- Transport: lead-relayed messaging only; P1 is the single writer of `crates/mochiko-cli/**` this wave.

9. **Line-named owner** (D11 as changed at build, B1): a bare session `D<n>` in a row or entry that
   names exactly one session — a link to its record, or a leading `` `slug` `` qualifier — belongs to
   that session; check and rename resolve it so. An explicit qualifier is owed only where the line
   names none or several. *(Narrowed 2026-10-08, user: "yes go with C": only a record link names
   the owner; every link and qualifier still counts toward "several".)*
10. **Normalization of session-prefixed decisions** (D10 as changed at build, B2): wave 3's back-fill
    rewrites "`PO-D1`" (and the eleven other prefixes) as `` `production-only-focus` D1-<slug> ``; the
    rename path must accept such an alias form as the mention it rewrites, and the diff check counts
    that qualifier change as an ID-token change. Other repo-only forms (`AM`, `OQ`, `J`, `FP`, census
    IDs) are slugged once and are not checked — the check never flags them.
11. **Family table in crate code, plus a drift test** (D6 as changed at build, B3): the table carries
    F24's facts; a crate test replays the log and fails if a family's minting rule shows a form other
    than the table's. No log grammar change.
12. **Direction words** (D13 as changed at build): the grammar accepts direction words in a slug
    (`D1-profile-leaves-setup`; a direction word counts toward D19's three) and still never strength
    words; the check never judges word choice
    — that stays the graders' (the bright line).
13. **A record path as a qualifier — wave 1c** (D11 as changed at build, 2026-10-08; user: "as
    recommended"): a mention's own code-span qualifier that is the path of a session's `record.md`
    (`` `.mochiko/brainstorms/<slug>/record.md` D3 ``) names `<slug>`, as `` `<slug>` `` would — for
    rename and for the check. The rename rewrites the ID token only; the path stays as written, and
    the diff check holds. Shapes the tool already leaves bare (ranges, lists of four or more, masked
    spans) stay bare. Census: W3-C found 343 such mentions in 86 files (`scratchpad/w3c-shorthand.tsv`).
    Fallback if the code review fails twice: the mentions stay bare (D18 as narrowed).
    *P1's plan questions, ruled by the lead within the ruling's own words (2026-10-08):* Q1 — a path
    qualifier names its mention's owner only; it does not make the line name one owner, so a bare
    `D<n>` beside it stays as D11 as narrowed leaves it (`line_sessions` unchanged; C2's lesson).
    Q2 — only the tree-root path `.mochiko/brainstorms/<slug>/record.md` names a session; a bare
    `` `record.md` `` or `` `<slug>/record.md` `` stays bare (D18 as narrowed).

## Build questions for the user — ruled 2026-10-08

B1 → A (the line names the owner) · B2 → B (normalize session-prefixed decisions; slug the other
repo-only forms once, unchecked) · B3 → A (crate code plus a drift test). User: "as recommded" each
time. Recorded on D11, D10 and D6 ("Changed at build").

### The questions as put

- **B1 — implicit qualifier.** `DECISIONS.md`, the brainstorms index, `BACKLOG.md` and `ROADMAP.md`
  carry bare `D<n>` on 550 lines (163 · 221 · 150 · 16), mostly inside a row or entry that names one
  session by a link to its record or a leading `` `slug` `` qualifier. Read literally, D11 makes each
  carry its session slug again.
- **B2 — repo-only forms (D10's extension).** Twelve session-prefixed decision forms (`PO-D` 38,
  `TC-D` 21, `OO-D` 17, `OD-D` 17, `AD-D` 15, `SD-D` 14, `CS-D` 13, …) plus `AM-n`, `OQ-n`, `J-n`,
  `FP-n` and the strip census IDs: no plugin rule mints them, so the crate has no definition site for
  them unless this repo declares one.
- **B3 — where the family table lives.** In crate code (the F24 facts as a table; no log grammar
  change, so no grammar bump) or as data in the migration log (the log is truth for schema content,
  but a new node kind means a grammar bump).
