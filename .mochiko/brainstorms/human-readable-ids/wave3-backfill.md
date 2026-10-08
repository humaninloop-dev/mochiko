# Wave 3 — the back-fill: one graded slug map, applied by the tool

**Status:** opened 2026-10-08. The slug map is drafted in three parallel shards, plan-only first. Apply
waits for wave 2's last migration (0048) to land and pass its gate.

Branch `human-readable-ids-build`. Scope for this session: waves 1–3 (user: "1-3"). Wave 4 (setup amend,
contract suite, bumps, `mochiko-cli` release carrying `ids`, CI step) stays with the user.

## What wave 3 does — transcribed from the record

Every requirement cites the ruling it transcribes; nothing here is new design.

1. **Live layer rewritten, history layer frozen** (D14 as changed at review and at build). The live
   layer is rewritten to the joined form:
   - `CLAUDE.md`, `DECISIONS.md`, `BACKLOG.md`, `ROADMAP.md`;
   - the governance ledger and `governance-intent.md`, the rules files;
   - every session record (cards and citations), the session index, strips, `.mochiko/decisions/`;
   - plugin prose, crate comments, and crate tests outside the replay fixture corpus and the
     template goldens.

   The history layer stays as written, and the tool's `DEFAULT_EXCLUDES` already hold most of it:
   `.mochiko/archive/`, the genesis corpus, the template goldens, eval stimuli, `CHANGELOG.md`, the
   migration log and `.mochiko/schema-views/`.
2. **One slug map, drafted then graded** (D14): keyed by family, scope and number. A producing seat
   coins; a second seat grades; the tool applies one ID at a time.
3. **Slug grammar** (D9, D13 as changed at build, D19 as changed at review, D22):
   - three lowercase ASCII words, each starting with a letter, coined from the definition's existing
     name;
   - topic words, never strength words; direction words allowed for removal and reversal rulings
     (`D1-profile-leaves-setup`);
   - an existing file-name slug is the ID's slug whatever its length (S5);
   - a dotted sub-decision takes its own slug; a clause pointer is not an ID (D22).
4. **Families in this repo:**
   - `GI` (22, defined in `.mochiko/memory/governance-intent.md`);
   - session `D` (per record, every record under `.mochiko/brainstorms/*/record.md`).
   - The twelve session-prefixed forms (`PO-D`, `TC-D`, `OO-D`, `OD-D`, `AD-D`, `SD-D`, `CS-D`, …)
     normalize to `` `<session-slug>` D<n>-<slug> `` through scoped `--alias` (D10 as changed at
     build, B2).
   - The other repo-only forms (`AM`, `OQ`, `J`, `FP`, strip census IDs) are slugged once with
     `ids literal` and are never checked (B2).
   - Product families (FEAT, FR, …) have no definition in this repo's live layer, so they are out of
     the map (D18 as changed at build).
5. **Owners** (D11 as narrowed at build, C):
   - a bare session `D<n>` in a line that links exactly one record is that session's;
   - elsewhere a cross-file cite carries its owner's slug;
   - a shorthand qualifier (`feature-map D8`) is rewritten only through a file-scoped `--alias`
     (wave 1b, Q2–Q3).
   - A file whose lines use one shorthand for two sessions is wave 3's judgment, edited by hand.
6. **The definition decides** (D16): no seat coins a slug for an ID it does not hold; the map slugs
   definitions only.
7. **Apply** (D7, D15):
   - `ids rename <owning-file> <ID> <slug>`, previewed, then `--write`, per map row;
   - `--alias` rows for the session-prefixed forms; `ids literal` for repo-only forms;
   - D15's diff-check line logged for every apply.
   - Excludes beyond the defaults: `evals/.work/`, `crates/mochiko-cli/tests/ids.rs` (its fixtures
     are test data), `.mochiko/brainstorms/*/inputs/`, `*/reports/*-raw.md`, `.mochiko/benchmarks/`.
   - *Expanded 2026-10-08 at the shard plans (W3-A §2, W3-C §1): `--exclude` takes a literal path
     prefix, no glob, so the apply passes this list exactly (the "Exclude list" below); the
     `*/reports/*-raw.md` pattern matched no file, and the one raw dump sits outside `reports/`.*
8. **Review of the applied diff** (D15's accepted risk; wave 1's N3, B3):
   - meta-text examples (a line discussing an ID as an example);
   - bold quotes of other sessions' decisions in a record (`model-tiered-seats/record.md:86`, `:92`
     take the record's own slug under rule 5; each such line is fixed by hand with its true owner);
   - single-word shorthands in linked rows.
9. **Plugin prose touched by the back-fill is a primitive edit**:
   - a strip entry per primitive (ID tokens joined per D14, a supersession by ruling);
   - the gate audit (`mochiko:validation-primitive-edit`), same ceremony as wave 2.
10. **Done check** (D18 as changed at build):
    - `mochiko-cli ids --check` with every path of the "Exclude list" below exits 0 over the live
      layer (*corrected 2026-10-08, W3-A H6: `--exclude evals/.work/` alone would read the slug
      map's own titles as bare IDs*);
    - D15's diff-check line is logged for every apply;
    - the gate grader runs it.
    - **Raised to the user at the close:** D18 also names "D8's read test has passed". Wave 0's read
      test failed and the user overrode the gate, so the clause needs the user's reading.
11. **Governance surfaces** (D17): the tool converts `governance-intent.md`, the ledger, the
    `CLAUDE.md` governance region and the rules files with the rest. The setup amend that records the
    events is wave 4's.

## Seats

- **Map producers:** three persona-less `general-purpose` seats on `model: opus`, sharded by session
  name order, plan-only first, plans graded together by one fresh generic seat on
  `mochiko:review-seat-plan`.
  - W3-A: sessions `adaptive-…` through `feature-map-layer`, plus `GI`.
  - W3-B: the sessions after `feature-map-layer` through `primitive-eval-harness`.
  - W3-C: the sessions after `primitive-eval-harness`, plus the repo-only forms, the
    session-prefixed alias rows and `.mochiko/decisions/`.
- **Map grader:** one fresh `general-purpose` seat on `model: opus` grades the merged map,
  adversarially on meaning and mirror-checklist on grammar, uniqueness and keying.
- **Apply:** one seat runs the graded map through the tool. This is mechanical execution of a ruling:
  a script, one row at a time, preview then `--write`, with the diff-check line logged.
- **Diff review:** a fresh seat reads the applied diff for item 8's classes.
- **Gate audit:** the touched plugin primitives (item 9).
- Transport: lead-relayed messaging only. Each shard seat is the single writer of its shard file
  under `inputs/`.

## Order

1. Shard plans in parallel, then one plan grade.
2. Shard builds in parallel, merged into `inputs/slug-map.tsv`.
3. The map grade.
4. Wave 2's 0048 lands and passes its gate.
5. Apply: the graded hand rows first, then the map, one row at a time.
6. Diff review.
7. Strips and gate audit.
8. The done check.
9. The close, with the user's reading of D18's read-test clause.

## Rulings on the shard plans — 2026-10-08

The three plans' questions, ruled by the lead within the record's rulings. They bind the shard
builds and are part of each seat's assigned scope at the plan grade. One question is the user's
and is still open (S11).

- **S1 — one shard file shape** (W3-A Q2, W3-B Q4, W3-C Q7). `inputs/slug-map-{a,b,c}.tsv`: UTF-8,
  no BOM, LF, a trailing newline, tab-separated, no quoting; a tab or newline in source text becomes
  one space. Header `family	owner	number	slug	definition	title	note`.
  - `family` — the prefix `ids rename` takes: `D` or `GI`.
  - `owner` — the session directory name; `-` for `GI`.
  - `number` — without the prefix, exactly as the index reads it (`001`, `11`, `2a`, `4.1`). The
    apply's ID is `GI-<number>` or `D<number>`.
  - `slug` — three words, or the existing file-name slug (D19 as changed at review).
  - `definition` — `;`-separated tree-relative `path:line`, every site the index holds for the key,
    as the tree will stand after the hand rows (S3); the first is the tool's first-read site, the
    apply's owning file. A foreign site a hand row qualifies leaves this column and goes in `note`.
  - `title` — the definition's own name, verbatim, Markdown kept, cut at 200 characters with `…`.
  - `note` — `;`-separated tags, then optional free text after ` — `. The apply reads one tag only,
    `exclude=<path prefix>` (repeatable, literal, no glob); every other tag is evidence for the
    grader and the diff review, defined in the shard's reply.
  - Sort: `family`, then `owner`, in byte order; then `number` by its integer part, then its suffix
    in byte order, bare first.
  - W3-C's `alias-c.tsv` and `literal-c.tsv` keep their planned shapes, except that `hand:` rows
    move to the hand-row file (S3).
  - *Added 2026-10-08 at W3-A's build (its open shape question; the lead took its option b): the
    Write and Edit tools drop a trailing tab, so an empty cell in any `inputs/*.tsv` carries `-`,
    and every row keeps its full field count. The apply still reads only `exclude=` from `note`.*
- **S2 — definitions the tool cannot index** (W3-A Q1, W3-C Q2).
  - The 18 glued `AR-D`/`AT-D`/`ER-D` definition cards are normalized to bare `D<n>` in their own
    records by hand rows of kind `normalize` (S3) — B2's normalization, at the definition. W3-A
    coins their 18 slugs; W3-C's three alias rows then have targets.
  - `build-vs-off-the-shelf` (6 inline `→ **D<n>**` lines) and `agent-decoupling` (4 table rows in
    `synthesis.md`, no `record.md`) stay bare and get no rows: making them indexable would change
    more than ID tokens, and D18 as changed at build already leaves an unindexed ID unreported and
    unrewritten. Booked for the backlog. The `adopt-first D4` alias row reports dead, as planned.
- **S3 — hand rows run before the apply** (W3-C Q8 option A; W3-A Q5 and H1–H2, W3-B Q3). A
  mention the tool would give the wrong owner, or an example the tool would collapse, is fixed by
  hand before any rename, so the owner's rename then joins it under the diff check. Each shard
  writes its rows to `inputs/hand-{a,b,c}.tsv`: header `site	before	after	kind	owner	reason`.
  - `site` — `path:line`; `before` — the exact text as it stands, found once on that line;
    `after` — its exact replacement.
  - `kind` — `normalize` (a glued definition token to bare `D<n>`, S2) · `qualify` (insert or
    correct the true owner's D11 qualifier: bold quotes of other sessions' decisions, foreign bare
    `D<n>`, code-span and single-word shorthands, same-number collisions) · `quote` (wrap a
    meta-text example in double quotes, D15's verbatim mask; S4).
  - `owner` — the `<session> D<n>` or `GI-NNN` the mention means; `-` for `quote`.
  - The apply seat runs the hand rows first and checks each by a preview of its owner's rename:
    `qualify` joins with the true owner's slug, `quote` leaves the site untouched, `normalize`
    gives the definition an indexed site. The diff review reads the hand-row diff as well.
  - Wave plan item 8's after-apply hand fix stays for what the diff review finds that no shard
    listed.
  - *One writer per line (added 2026-10-08 at the plan grade, the grader's cross-shard advisory).*
    A hand row is written only by the shard that owns the file: a file inside
    `.mochiko/brainstorms/<slug>/` belongs to the shard whose list holds `<slug>`;
    `.mochiko/memory/governance-intent.md` and `.mochiko/memory/governance-ledger.md` belong to
    W3-A; every other file belongs to W3-C. A shard that finds a hand row due in a file it does not
    own lists it in its build reply under "hand rows for other shards", in the hand-row shape; the
    lead relays each, quoted, to the owner, who adds it or says why not. Known now: W3-A's H4 tokens
    and W3-C's row-1 note (`human-readable-ids/record.md:901`) sit in W3-B's files. The apply seat
    stops on two rows for one `site` whose `before` spans overlap.
- **S4 — recorded evidence and meta-text** (W3-B Q2, W3-C Q6, W3-A Q3 second half). Recorded
  evidence is history (D14: eval stimuli and recorded runs) and joins the exclude list:
  `wave0-read-test.md`, the six `evals/<kit>/variants/`, the six `evals/<kit>/pass-report.md` and
  `hook-enforced-artifact-schema/wave3-census-raw.md`. A live line that shows an ID as an example
  of a different slug (W3-A H4's seven tokens and their kin) takes a `quote` hand row — the
  build-log precedent of 2026-10-08 — never a whole-file exclude, which would leave the file's real
  cites bare.
- **S5 — in-tree coins** (W3-A Q3 first half, W3-B Q2). A hand-coined joined form already in the
  live layer that meets the grammar is adopted as the slug, so the rename leaves it unchanged.
  Where two in-tree coins disagree, the one built from the definition's own words wins. The map
  grader grades adopted coins like any other.
- **S6 — repo-only series** (W3-C Q3–Q5). Each `J` and `FP` series is owned by its defining file
  and slugged once, with one `ids literal` per series over explicit paths; a file that mixes two
  series under one number takes `qualify` hand rows. The other heads (`FD`, `HF`, `F`, `FC`, `O`,
  `RI`, `DQ`, `TR`, `BD`) are out of wave 3 and booked; `R-n` gets no rows.
- **S7 — sessions with no rows** (W3-B Q1, W3-C Q10, W3-A §2): `playbook-design`,
  `vertical-graduation`, `brainstorm-command`, `command-altitude`, `author-grader-value-tiering`
  and `feature-map-granularity-and-reparenting` define no numbered decision the tool indexes.
- **S8 — clause pointers glued to the number** (W3-A Q4: `D3c`, `D10.1`, `D4b` with no
  definition). Left as written; D18 as changed at build leaves them unreported. Booked.
- **S9 — shorthand alias rows** (W3-B Q5). W3-C writes the file-scoped `--alias` rows for every
  shard, including `feature-sizing D=<file>` for W3-B's `feature-sizing-and-entry-points`; W3-A and
  W3-B tag the need in `note`.
- **S10 — setup-owned surfaces** (W3-C Q9). Noted for wave 4's amend; the back-fill converts them
  with the rest (item 11).
- **S11 — the 343 path code-span cites** (W3-C Q1). The user's (it touches D18's accepted risk).
  Ruled 2026-10-08, user: "as recommended" — C: a small crate change (wave 1c,
  `wave1-crate-ids.md` requirement 13; D11 as changed at build) so the tool reads
  `` `…/<slug>/record.md` `` as naming `<slug>`. It lands before the apply; if its code review fails
  twice, the mentions stay bare (A). Neither answer changes a map row.

### Exclude list (apply, previews and the done check)

The defaults, plus exactly these literal prefixes:

- `evals/.work/`, `crates/mochiko-cli/tests/ids.rs`, `.mochiko/benchmarks/`;
- `.mochiko/brainstorms/human-readable-ids/inputs/`,
  `.mochiko/brainstorms/author-grader-consolidation/inputs/`,
  `.mochiko/brainstorms/lead-owned-process-flexibility/inputs/`,
  `.mochiko/brainstorms/plan-run-transport-forensics/inputs/`,
  `.mochiko/brainstorms/team-method-vs-command-shape/inputs/`;
- `.mochiko/brainstorms/human-readable-ids/wave0-read-test.md`,
  `.mochiko/brainstorms/hook-enforced-artifact-schema/wave3-census-raw.md`;
- `evals/<kit>/variants/` and `evals/<kit>/pass-report.md` for each of `patterns-entity-modeling`,
  `review-brainstorm`, `review-governance-intent`, `review-plan-artifacts`,
  `review-specifications`, `validation-constitution`.
