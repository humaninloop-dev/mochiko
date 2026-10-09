# Build log — `human-readable-ids`

Append-only. One `##` entry per event, 60 lines at most.

## 2026-10-08 — wave 0 opened: the two-arm read test

- Record accepted 2026-10-08; the close ritual ran before any build step (decisions row, backlog
  section, roadmap fold, index entry). The user committed it on `main` (`bf410cc`, "Add
  human-readable ID decision record") and asked for the build: "i commited, implement the changes".
- D8 (as changed at review, S9) puts the read test first, as wave 0, before anything is built; a fail
  stops the build. So the build opens with the test, not with a primitive or crate edit.
- Plan and sheet: `wave0-read-test.md`. Sampler: a read-only script in the lead's session scratchpad,
  seed `20261008`; pool 16 GI · 85 cross-session D · 56 FEAT; 8 GI + 9 D + 3 FEAT per arm, disjoint.
- Arm 2's 20 slugs were coined by the lead (disclosed in the sheet), under D13 and D19.
- No branch yet: wave 0 writes only session artifacts. A branch off `main` is put to the user before
  wave 1 touches a plugin primitive or the crate.
- Nothing in `plugins/mochiko/`, `crates/` or the operating docs has been touched by the build.

## 2026-10-08 — wave 0 run as a cold-seat test: FAIL, build stopped

- User, with arm 1 on screen: "i am okay for you to implement without testing involving me". Put to
  the user with the build plan: replacement (a) cold seat / (b) skip · branch · scope; user: "a, ok,
  1-3". Recorded on D8 as changed at build.
- Branch `human-readable-ids-build` created off `main` @ `bf410cc` (user: "ok"). Nothing committed.
- `wave1-crate-ids.md` written (plan only); it raises build questions B1–B3. No seat dispatched for
  wave 1.
- Reader `cold-reader` (general-purpose, opus): 40 IDs, one shuffled list, no tools (attested); the
  project `CLAUDE.md` loaded at spawn and names 8 bare IDs — disclosed by the seat.
- Scorer `read-test-scorer` (general-purpose, opus), rules fixed before the answers: arm 1 knew 0 ·
  arm 2 knew 7, misled 0, guesses right 12, inverted 1 (item 38). Clause 1 (≥15) fails; clauses 2 and
  3 pass. Detail: `wave0-read-test.md` § Marks.
- Per D8: the build stops before anything is built and returns to the user.
- floor: clear (wave 0 wrote only session artifacts; no governing surface touched). Seats that ran:
  `cold-reader` (subject) and `read-test-scorer` (score); the lead built the sheet and coined the slugs.

## 2026-10-08 — the failed gate overridden by the user; D13 amended

- Put to the user: A proceed with a restated bet plus direction words for removal rulings · B rule
  slugs and a re-test · C stop; case against A first ("redefining success after a failed gate is what
  the gate exists to stop"); lead's lean A. User: "as recommded".
- Recorded on the record's frame card (bet line), D8 and D13 ("Changed at build") and Untested bets.
  The original bet stays recorded as failed.
- Next: build questions B1–B3 (`wave1-crate-ids.md`), one at a time, before P1's plan dispatch.

## 2026-10-08 — wave 1 opened: B1–B3 ruled, P1 dispatched plan-only

- B1 → A (the line names the owner) · B2 → B (normalize session-prefixed decisions; other repo-only
  forms slugged once, unchecked) · B3 → A (family table in crate code plus a drift test). User: "as
  recommded" each time. Recorded on D11, D10 and D6 ("Changed at build"); requirements 9–12 added
  to `wave1-crate-ids.md`.
- Tree snapshot taken before the dispatch (HEAD `bf410cc`; only this session's files modified or
  untracked), for the plan-only dirty check.
- P1 `p1-crate` (`mochiko:staff-engineer`, persona default tier, no `model:` override) dispatched
  plan-only and read-only, writing nothing in the repo; its plan returns in its reply (copy in the
  lead's scratchpad). The brief carried requirements 1–12, the record cards to read, the rust-cli
  bright line and gates, the frozen fixture corpus, the no-bump fence, and the routing line.
- Next: dirty check, then a fresh `mochiko:staff-engineer` peer grades the plan on
  `mochiko:review-seat-plan`; only a PASS is approved.

## 2026-10-08 — wave 2 planned while P1 plans; B4 ruled

- `wave2-plugin-minting.md` written (plan only, no seat dispatched): the IDs section, minting
  carriers per family from F20, graders, the `Q<n>` rename, scripts, ripple, tests and kits,
  ceremony; three persona-less producers (`W2-schema`, `W2-prose`, `W2-tests`) and one gate grader.
- B4 put to the user: where the joined-ID writing rule lives — A one copy in
  `templates/artifact-format.md` · B a new `templates/id-grammar.md` · C three inline copies; case
  against A first; lead's lean A. User: "as recommded". Recorded in the wave-2 plan.
- Ratified run B1–B4: four in a row — the next fork goes in another form.
- Open for the wave-3 close, not yet put to the user: D18's done check names "D8's read test has
  passed"; it failed and the user overrode it, so that clause cannot be met as written.

## 2026-10-08 — scope re-confirmed; P1's plan returned clean; plan grade dispatched

- The user asked whether the complexity came from backward compatibility. The lead's answer: very
  little — the drivers are D3 (every family), D2 (every mention), D14 (back-fill), D4 (slug follows
  the topic, so a rename tool and a drift check) and the primitive-edit ceremony; D16 and the
  frozen history layer are the small compatibility share. Cuts offered: GI and session D only (the
  lead's pick, if narrowing) · new IDs only · slugs frozen at mint. User: "continue as decided". No
  ruling changed.
- P1 returned its plan (about 1,150 source lines, about 70 tests, write set all under
  `crates/mochiko-cli/`), with six stops (S1–S6, each with a default) and hand-offs H1–H6. Tree
  diffed against the snapshot: no change by the seat (only the lead's own wave-2 plan file is new);
  HEAD unchanged. Disclosed: the seat also saved its plan in the lead's scratchpad, outside the repo
  (`review-seat-plan.plan-verbatim` keeps the plan in the grader's brief only).
- The grade was held while the scope question was open, then dispatched: `p1-plan-grader`, a fresh
  `mochiko:staff-engineer` (persona default tier), the `review-seat-plan` render and the plan
  verbatim in its brief, default FAIL, every tree claim checked first-hand.

## 2026-10-08 — P1's plan graded FAIL (items 1, 3, 5); the wave's one re-plan round spent

- `PLAN GRADE: p1-crate · FAIL · 1,3,5`. Seven blocking findings: F1 legacy plain `<slug> D<n>`
  cites (126 in records) resolved to the citing session — a rename would rewrite them wrongly and
  pass the diff check · F2 the walk enters nested checkouts and gitignored scratch (`evals/.work/`,
  178,057 `.md` files) · F3 the GI definition site (the ledger heads 11 of 22 GI IDs) · F4 a cycle's
  owner is an uppercase `FEAT-NNN` qualifier · F5 the four-word direction-slug example against
  D19's three · F6 no checked apply path for B2's other repo-only forms · F7 attempt bound unstated.
  Item 2, 4 and 6 passed; every tree claim checked first-hand.
- Lead rulings sent with the verdict, verbatim, in the re-plan order (readings of made rulings, no
  new ruling): R1 plain names qualify only when they equal a session directory; others unresolved,
  reported, never rewritten; `--alias` takes any literal prefix · R2 stop at nested `.git`,
  default-exclude `.claude/worktrees/` and `.mochiko/runs/`, `evals/.work/` by `--exclude` · R3 GI
  defined in `governance-intent.md` · R4 FEAT/EPIC-owned artifacts qualify by the joined FEAT/EPIC
  ID · R5 a direction word counts toward D19's three · R6 a seat-scoped literal-token mode for the
  other repo-only forms · R7 bounds stated · R8 advisories adopted, C4-model names a fixed exception.
- Lead's own errors found by the grade, corrected: `wave1-crate-ids.md` requirement 3 named the
  ledger as GI's definition site (F3); D13's build note used a four-word example (F5) — that record
  repair goes to the user.
- P1 resumed for its revision (plan-only); the same grader re-grades. A second FAIL goes to the user.

## 2026-10-08 — P1's revision returned; in-round rulings; re-grade sent; D13 repaired

- P1's revision 1 absorbed R1–R8, then — same round, before any grade — four more lead rulings: S2
  NFR's source is `nfr-one-home` (R8's `element-grammar` was the lead's error) · S7 the C4 diagram
  terms, `level` included, a fixed exception · S8 `.claude/settings.local.json` default-excluded,
  never read (it holds a live token) · R9 an unresolved bare `C<n>` is a D10 local label, never
  reported or rewritten, no folder inference — retiring the ~1,515 bare `C<n>` labels P1 counted as
  a block on D18's clean run. Accepted risk named: a real cycle cited bare outside its `tasks.md`,
  unqualified, goes unreported.
- Dirty check after each return: no change by the seat; HEAD `bf410cc`. Revised plan (321 lines,
  sha256 prefix `386a440d00eed833`) sent verbatim to the same grader, `p1-plan-grader`, with the
  in-round rulings named as part of P1's scope.
- D13 record repair, user: "ok of d13" — the build-note example now reads `D1-profile-leaves-setup`
  (a direction word counts toward D19's three); `wave1-crate-ids.md` requirement 12 likewise.

## 2026-10-08 — P1's plan re-graded PASS; approved; P1 building

- `PLAN GRADE: p1-crate · PASS · —` on revision 1 (sha256 prefix `386a440d00eed833`); F1–F7 each
  closed, tree claims checked first-hand by the grader. P1 reported its scratchpad copy reformatted
  on disk by something else between its reads, content unchanged; the grade read the brief's copy.
- Approved by the lead; P1 resumed to build inside its write set (`crates/mochiko-cli/`), TDD. Tree
  snapshot taken before the build (session files only).
- Grader advisories routed: A2 recorded captures (`.mochiko/brainstorms/*/inputs/`,
  `*/reports/*-raw.md`, `.mochiko/benchmarks/`) join wave 3's `--exclude` list, no crate change ·
  A6 H7 done · A7 the counted tests carry the first plan's names · A1 (~1,700 product-family tokens
  with no definition in this repo — foreign, fixture and example IDs), A4 (prose words such as
  `user-ruled` before `D<n>`) and A5 (R9 narrows D18) go to the user as one question: what the check
  does with IDs it cannot tie to a definition · A3 (an ambiguous shorthand, `feature-map`, matches two
  sessions; a project-wide `--alias` would mis-own) — a later crate increment, planned and graded on
  its own, never folded into this build.

## 2026-10-08 — D18 narrowed by the user; supersession sent to P1 mid-build

- Put to the user in the changed form (case against each option, weak lean A): what the check does
  with IDs it cannot tie to a definition — A silent · B report, and count a recorded leftover
  (~2,000) as done. User: "as recommded". Recorded on D18 ("Changed at build") and on
  `wave1-crate-ids.md` requirement 5.
- Content-pinned supersession sent to P1: it quotes and voids revision 1's item 5(a) clause on
  unresolved bare mentions, item 3's no-definition clause, and item 9's reporting effect; it names
  what is unchanged and the tests that flip. A narrowing (less reported), no new feature; folded
  into the running build as a transcription of the user's ruling. A3's alias path scoping stays a
  separate later increment.

## 2026-10-08 — wave 2 opened in parallel: W2-schema dispatched plan-only

- `wave2-plugin-minting.md` gains "Hand-offs from wave 1" (H1 the drift test's `shows` strings, H2
  definition lines keep the ID first, H3 the check's narrowed reach) and an ordering line: no wave-2
  migration lands while wave 1 builds or is under review — the crate's drift test replays the
  working tree's log; wave-2 seats may plan in parallel, read-only.
- W2-schema (`general-purpose`, `model: opus`, the 2026-10-06 precedent for persona-less migration
  authors) dispatched plan-only and read-only; its brief carries the wave plan, the record cards to
  read, the log grammar, B4's IDs-section pointer, its write set (migrations from `0046`, views
  re-emit, allowlist rows), what is not its own, the bounds and the routing line.
- Tree snapshot outside `crates/` taken before the dispatch (P1 is writing `crates/mochiko-cli/`
  concurrently: disjoint surfaces).

## 2026-10-08 — P1 built wave 1; supersession found unapplied; pre-review amendment

- P1's cycle report: 14 tasks, `ids.rs` · `rename.rs` · `tests/ids.rs` created, four crate files
  changed; seat-reported four layers exit 0 (664 tests, 83 in `tests/ids.rs`); no worker; ~2,320
  source lines against ~1,300 planned, disclosed. Tree: writes only under `crates/mochiko-cli/`.
- Lead read-back of `tests/ids.rs`: the D18 supersession is not in the build — `:979-982` and `:1016`
  assert the voided behavior (undefined and unresolved bare mentions reported). Fan-in miss: the
  order was sent after approval while the seat was building; whether it arrived is asked.
- Pre-review amendment order (not the review fix round): the supersession re-issued, quoted · N1
  `…`/`...` ranges stay bare (D5) · N2 a hyphen run is a slug only at exactly three words or the
  file-name slug, else a local label (`D8-as-amended`) · N5 rename refuses `--write` when a moved
  file's old name appears outside scope. N3 (meta-text examples like "a bare `D7` is ambiguous"
  outside quotes would be rewritten) goes to wave 3's map and review, no crate change.

## 2026-10-08 — amendment built; tech-lead code review dispatched

- The supersession did reach P1 — after its first cycle report; applied and reported, the report
  crossing the re-issue (no double work: item 1 already in the tree). A status pull drew the rest.
- N1, N2, N5 built test-first: 9 new tests (ids 92, total 673), red lines quoted by the seat;
  one existing fixture changed under N2 (`GI-001-secrets-out-of-repo`, four words, now
  "`GI-001-secrets-outside-repo`"). Seat-reported four layers exit 0. Smoke `649 bare · 42 drift`
  (was 10,148 · 88 before D18's narrowing). Six judgment calls (a)–(f) disclosed; the reviewer rules.
- Write set held: `crates/mochiko-cli/` only (ids 1,336 · rename 872 · tests 2,111 · cli +262 ·
  conform ±16 · home, lib small). No dependency, no bump.
- Code review dispatched: `w1-code-review`, a fresh `mochiko:tech-lead` (persona default tier),
  default FAIL, the four layers and the real-tree check run first-hand, no repo writes (write
  experiments on a scratch copy without the settings file). P1 holds. One fix round; a second
  failed review goes to the user.

## 2026-10-08 — W2-schema's plan returned; lead rulings Q1–Q3 before its grade

- W2-schema returned a 712-line plan: four migrations `0046`–`0049` (ten template replaces under
  D21; thirteen minting-rule rewords under D1; the landing renumber through `ids rekey` under D4;
  a `review-common.joined-ids` common block plus five grader stubs under D6), 26 views re-emitted,
  every drift-test `shows` string kept, the joined cycle heading named `### - [ ] C<n>-<slug> —
  <title>`, hand-offs P1–P9 (W2-prose), T1–T5 (W2-tests), C1 and L1–L3. Dirty check outside
  `crates/`: no change by the seat. The seat too reports its scratchpad copy reformatted on disk by
  something else after writing (content unchanged) — the same oddity P1 saw.
- Lead rulings sent before the grade (same round): Q1 D21's "placeholders" reach only forms a seat
  copies (13 rewords; the literal ~55 more rejected, on D21's own purpose clause) · Q2 the two desk
  graders (`arch.seat-tech-lead-grader`, `feat.author-grader`) gain the check — D6 binds every
  minting producer's grader · Q3 `0046`–`0050` to W2-schema, `0051` held for the gate fix round · C1
  the crate's legacy `Cycle <n>` reader stays (D16).
- Amended plan returned (809 lines, ~55 KB; Q2 adds two command must rules to 0049). Dirty check
  outside `crates/`: clean. Transport note, disclosed: the return message was cut short in transit,
  so the lead froze the seat's own saved text read-only (`w2-schema-plan.r0.frozen.md`, sha256
  prefix `8d66547d53e3ed5c`) and the grader reads it there, sha verified — the 2026-10-06 precedent.
- Plan grade dispatched: `w2-schema-plan-grader`, fresh `general-purpose`, `model: opus` (the
  producer's tier), the `review-seat-plan` render pasted, default FAIL.

## 2026-10-08 — wave-1 code review FAIL; the one fix round opened

- `CODE REVIEW: wave-1 mochiko-cli ids · FAIL`. Layers run first-hand by the reviewer: test exit 0
  (673; ids 92), fmt 0, clippy 0 (fresh target dir), audit 0; real tree `649 bare · 43 drift`.
  Bright line PASS. No repo write by the reviewer (scratch copies only).
- Blocking: C1 two planned moves to one name — `fs::rename` overwrote a file (reproduced) · C2 a
  mid-line qualifier names the line for B1 — 50 of 51 index.md resolutions wrong-owner · C3 a
  hard-wrapped qualifier is dropped (8+ tokens) · H1 the write drops the file mode (a hook script
  would turn non-executable) · H2 a prose line opening with an ID becomes a definition (31 of 611)
  · H3 slugged path tokens rewritten with no move (dead links) · H4 SCR's prototype file-name slug
  not honored (S5) · H5 `--alias D` / `--alias ""` turn rename into a bare-number rewrite. Medium
  M1–M4 (diff check blind to N2 labels; partial write reported as nothing; literal ignores N2;
  alias vs lists of four). Low L1, L3–L5.
- Lead rulings for the fix round: all C/H/M and L1, L3, L5 fixed as the reviewer wrote; (f)
  reversed (refusal inside `rename::write`); (a) narrowed; L4 `/` after an ID or alias is a pair
  separator; A4 `.env`/`.env.*` default-excluded; A2 four dedupes; A3 to wave 3's excludes; A5
  baseline rerun. Same reviewer re-reviews; a second FAIL goes to the user.

## 2026-10-08 — W2-schema's plan graded FAIL (item 1); wave 2's one re-plan round spent

- `PLAN GRADE: w2-schema · FAIL · 1` (frozen sha `8d66547d53e3ed5c` verified by the grader): F1
  C-family cites qualified by spec only — the product baseline's owner is missed · F2 cycle owners
  leave out lane runs · F3 feature-entry's deferred-SC slot unqualified · F4 EPIC has no
  definition line anywhere. Items 2–6 pass; every tree claim checked, all true.
- Lead rulings sent with the verdict: Q4 the product baseline's qualifier is `` `product` `` · F2
  the run key (joined FEAT/EPIC ID, or `` `lane-<slug>` ``) · F3 and F4 accepted, F4 in scope (D2, D3)
  · A5 an instance key in a status example is a mention, joined; kind lists stay; run folders bare
  · A6 rekey must run where the run's entry alone holds the number, else a stop — no new floor step
  invented. Revision goes to a new file (`w2-schema-plan.r1.md`); the frozen r0 stays.
- Crate side of A6 sent to P1 as fix-round addendum item 9: cycle definitions from every
  `tasks.md` (epic and lane homes), owners per item 4. Voids nothing in the fix-round order.
- The wave's shared re-plan round is now spent: any further wave-2 plan FAIL goes to the user.

## 2026-10-08 — wave-1 fix round built; re-review dispatched

- P1's fix round: every review finding fixed test-first, a red line quoted per finding; ids tests
  92 → 114, total 695; seat-reported four layers exit 0; real tree `621 bare · 45 drift` (the new
  D18 baseline); the D3 probe preview no longer touches index.md:647 or delta-files record.md:39.
  Three existing tests changed, each forced by a ruling (C2, C3, H4), disclosed.
- Fan-in misses, both pulled: fix-round addendum item 9 (epic and lane cycles) was not in the first
  fix report; built after a pull. Its lane-owner call (P1 mapped `<slug>` to `lane-<slug>`) was
  ruled the plain directory name — lane dirs are already `lane-<slug>` (implement.yaml:463, :611);
  the ruling needed a re-issue before it landed. Verified in the tree by the lead.
- P1's walk disclosure: a preview without `--exclude evals/.work/` was still walking 240,353 files
  after ~5 CPU-minutes and was killed — wave 3's runs carry that exclude.
- Re-review sent to the same seat, `w1-code-review`, with the fix-round orders, P1's claims to
  verify, and named regression hunts (C3 soft-break reach, H3 relative paths, M1 against file-name
  slugs, H5 against the real alias prefixes). A second FAIL goes to the user.

## 2026-10-08 — W2-schema revision 1 returned with an S5 stop; Q5 ruled; re-grade sent

- Revision 1 (800 lines) fixes F1–F4 and folds Q4/A5; dirty check outside `crates/`: clean. It
  holds 0048 on S5: at the landing renumber the tree holds two entries with the old number, and
  `ids rekey` keys on the number alone — the seat proved it with a scratch-tree preview on wave 1's
  binary (both runs' entries and citations moved; the diff check would pass it). The only tree
  where the run's entry stands alone comes before the other run's landing — a new floor step, not
  drafted. Options: (a) rekey takes the entry's joined old ID · (b) no 0048, a D4 ruling · (c) a
  new step.
- Q5 ruled (a), the lead's call: a tool mechanism inside D4/D7, no floor step, no ruling changed.
  The crate side becomes a wave-1b increment with A3's `--alias` path scoping — planned and graded
  on its own after wave 1's re-review; 0048 is written only after it lands.
- r1 frozen (`w2-schema-plan.r1.frozen.md`, sha256 prefix `8ccbda8af049e578`, the return message cut
  short again) and sent to the same grader. A FAIL now goes to the user.

## 2026-10-08 — W2-schema approved; W2-prose and W2-tests dispatched plan-only

- `PLAN GRADE: w2-schema · PASS · none` on r1: F1–F4 closed, every `shows` string re-checked,
  H2 lines checked against wave 1's new definition leaders, 0048 graded as drafted under (a).
  Grader disclosure: in its r0 grade it ran two read-only git commands against the brief's "no
  git"; nothing mutated.
- Approved by the lead with feedback R2 (desk-rule wording fits EPIC too), R3 (0048 "every mention
  of that entry"), R4 (stale labels), R1 (lane and epic cycles already in wave 1's tree). Build
  gated on S1 (wave 1's re-review); 0048 on wave 1b.
- W2-prose and W2-tests (`general-purpose`, `model: opus`) dispatched plan-only and read-only,
  in parallel; each brief names W2-schema's frozen r1 hand-offs (P1–P11, T1–T5) as binding, the
  other seats' write sets, the ordering, and that a plan FAIL now goes to the user (the wave's
  shared re-plan round is spent). Tree snapshot taken before both dispatches.

## 2026-10-08 — wave-1 re-review FAIL (C2); the second failed review goes to the user

- `CODE REVIEW: wave-1 mochiko-cli ids · FAIL · C2`. Layers first-hand: test 0 (695; ids 114),
  fmt 0, clippy 0 (fresh target), audit 0; real tree `621 bare · 45 drift`. No repo write.
- Closed and verified by the reviewer: C1, C3, H1–H4, M1–M4, L1, L3–L5, (f), A4, A2. Partly: H5
  (`--alias "(D"` / `"the D"` still rewrite; N3) and (a) (moves not narrowed; N2).
- Still open, blocking: C2 — the fix keys the line owner on the first session-D token wherever it
  sits, so index.md "Landed" lines still go to the superseder (index.md:39 still wrong) and
  index.md:58, safely unresolved before, is now mis-owned; 42 of 43 line-resolved index tokens
  wrong, 340 repo-wide owned by lines naming another session. Reviewer's fix: name set = every
  record link and qualifier; own only on exactly one session, named by a record link or a head
  qualifier; red tests on the :39 and :58 shapes and the original R1 fixture.
- Medium N1 (leaf-name owners collide across `**/tasks.md`) · N2 (moves of label-named files) · N3
  (alias guard) · N4 (the quote mask covers HTML attributes). Low L1–L3.
- Bound reached: put to the user — A one more fix round with the reviewer's fixes · B re-staff ·
  C narrow B1 to record links only; cases against each; lead's lean A.

## 2026-10-08 — user grants fix round 2 with B1 narrowed (C)

- The user asked what C changes. The lead read index.md:39 and :58 — both name the other session by
  a mid-line qualifier and link no record, so under C they go unresolved (safe) — and counted, by a
  rough script, ~7 bare `D<n>` in 6 lines that only a leading qualifier would own. Lean moved A → C,
  said so. User: "yes go with C".
- Recorded on D11 ("Narrowed at build") and `wave1-crate-ids.md` requirement 9: only a record link
  names a line's owner; every link and qualifier still counts toward "names several".
- Fix round 2 sent to P1 (user-granted; a further failed review returns to the user): C2 under C
  with red tests on the real index shapes and the original R1 fixture, plus N1 (three cycle homes
  only), N2 (moves need a slug-shaped name), N3 (alias guard), N4 (quote mask in `.md` only), L1–L3;
  three real-tree previews to quote.

## 2026-10-08 — W2-tests' plan returned; lead rulings; plan grade dispatched

- W2-tests returned a 560-line plan: 4 crate test files plus ~14 regenerated goldens, a new
  `tests/ids_definition_lines.rs` (T5), 13 eval-kit files, no contract-suite edit; it found what
  the T-list missed (`tests/validate.rs`'s second census copy; the pointer count 83 → 88 under the
  five stubs; `check-rubric` rows for the desk-grader rules). Probes in the scratchpad only: the
  golden regeneration reproduced all 24 goldens byte for byte; the wave-1 binary read all 11
  planned joined definition forms. Dirty check outside `crates/`: clean.
- Lead rulings: Q-T1, Q-T2, Q-T3 leans stand (Q-T2's pre-existing kit drift booked for the
  backlog at landing — `check-rubric implement` red with 5 uncovered ids,
  `review-specifications/rules.json` missing `sf-direction-checks`, stale `evals/plan/README.md`
  figures); W2-tests' crate test changes take a non-author code review by the wave-1 tech-lead
  seat, not the primitive-edit gate grader.
- Plan frozen (`w2-tests-plan.r0.frozen.md`, sha256 prefix `bf947a29802b2417`, return cut short)
  and graded by `w2-tests-plan-grader`, fresh `general-purpose`, `model: opus`. A FAIL goes to the
  user (the wave's re-plan round is spent).

## 2026-10-08 — wave-1 fix round 2 built; N4 narrowed by the lead

- P1's fix round 2: C2 under the narrowed B1 (red on the real index shapes, two existing tests
  changed under the ruling, the original R1 fixture restored), N1 three cycle homes only, N2 moves
  need a slug-shaped name, N3 alias guard (the twelve real prefixes and three shorthands pass;
  `D`, `(D`, `the D` refused), N4, L1–L3. ids 114 → 121, total 702; seat-reported layers exit 0.
- Real-tree previews, read-only: D2 no longer touches index.md:39; D6 no longer index.md:58; D3
  neither index.md:647 nor delta-files record.md:39.
- Risk raised by the seat: N4 as ruled (quote mask in `.md` only) un-masked Rust string literals,
  so the D3 rename would rewrite `tests/fidelity.rs:584/:589` — expected provenance matched against
  the bare log — and turn `cargo test` red; drift rose to 86 (+41 all in `tests/ids.rs`). Ruled
  (ii), the lead's own N4 narrowed: quotes unmasked only in `.html`/`.htm`. Sent to P1 in the same
  round, test-first, with the D3 preview and the count to re-run.
- (ii) built test-first (ids 122, total 703; layers exit 0, seat-reported): the D3 preview now 5
  files, not `tests/fidelity.rs`; real tree back to `621 bare · 45 drift` with `evals/.work/` the
  only exclude. Re-review 2 sent to `w1-code-review` with the user's C ruling, the round's order,
  N4 as narrowed, and named regression hunts; a FAIL returns to the user.

## 2026-10-08 — W2-prose's plan returned; plan grade dispatched

- W2-prose returned its plan (707 lines; the message cut short in transit). The lead froze the
  seat's saved text read-only as `w2-prose-plan.r0.frozen.md` (sha256 prefix `f0c63a406b75c40d`).
  `git status --porcelain` unchanged from the pre-planning snapshot: planned clean. The seat
  disclosed one read-only `git status` against its brief's "no git".
- Plan in gist: a `## IDs` section in `templates/artifact-format.md` (12 rules, ~50 lines, format
  v3 → v4); ID-token changes in 29 other prose and script files; 22 strip entries (one new strip
  file, `evolution-notes-module.md`); one unreleased `CHANGELOG.md` entry; no ripple.
- Three questions for the lead before build, not yet ruled: Q-A (owners on example IDs in
  reference prose), Q-B (correct `DECISION-RECORD.md`'s `D1`… and two skills' `US#1` to their
  family forms while joining), Q-C (how to measure the `skill-review-common.yaml` budget row).
- Graded by `w2-prose-plan-grader`, fresh `general-purpose`, `model: opus`, render pasted. A FAIL
  goes to the user (the wave's re-plan round is spent).

## 2026-10-08 — wave-1 re-review 2 FAIL (F1); W2-tests plan FAIL (F1); both go to the user

- `w1-code-review`, round 3: `CODE REVIEW: wave-1 mochiko-cli ids · FAIL · an unowned bare
  file-name link is rewritten to another spec's renamed screen (rename.rs:461-464 with :563-564)`.
  Four layers exit 0 first-hand (703 tests); real tree `621 bare · 45 drift`; round-2 items C2
  (ruling C), N1–N4, L1, L2 verified. F1 (High): two specs share `scr-001-week-menu.html`; a root
  file's unqualified cite of the other spec's screen is rewritten to the renamed file, exit 0, diff
  check passing — a dead pointer and a wrong-owner rewrite (D18 as narrowed; GI-005-record-layer-integrity). Fix named:
  resolve the written path from the citing file's directory or the tree root, as the slash branch
  does; a bare name passes only beside the moved file. A1 (Low): `stale_spots` (:709-745) matches
  by name alone, same fix. A2 (Info): project-wide moves span the tree (D12; confirm).
- `w2-tests-plan-grader`: `PLAN GRADE: w2-tests · FAIL · 5 (with 1, D18 in scope)`. Regenerated
  template goldens (`crates/mochiko-cli/tests/fixtures/template/`) are live to the check — not in
  `DEFAULT_EXCLUDES` (`src/ids.rs:1050`) — so after 0046 they report "GI-001-project-surface-type"
  as drift against this repo's bare "`GI-001`" (probed: 2 drift), and wave 3 would rewrite their
  decided-ID cites ("`setup-product-agnostic D3`"), breaking byte-equality. Fix named: a post-T1 check
  task with expected output, plus a stop routing the goldens' exclusion. Advisories A1–A7. All
  other tree claims verified first-hand.
- Bounds: wave 1 has spent its fix rounds (round 2 was the user's grant); wave 2's re-plan round
  is spent. Both go to the user.

## 2026-10-08 — user grants wave-1 fix round 3 and a W2-tests re-plan; goldens join history

- User: "as recommded" to both. (1) A: fix round 3 for P1 — F1 and A1 by path resolution, test
  first; the same reviewer reads only the fix. (2) A: one more W2-tests re-plan round, and the
  template goldens `crates/mochiko-cli/tests/fixtures/template/` join the history layer — recorded
  on D14 ("Changed at build") and `wave1-crate-ids.md` requirement 5. The exclude lands in P1's
  `DEFAULT_EXCLUDES` (item G1 of fix round 3); W2-tests adds a post-T1 check task, a stop on P1's
  exclude, and A1–A7. A2 (project-wide moves span the tree) confirmed as D12, no change.
- Orders sent content-pinned to `p1-crate` (F1 verbatim, the fix, the exact exclude line) and to
  `w2-tests` (the verdict verbatim, the ruling, the advisories; A5 ruled by the lead: README and
  footer figures in scope only where a wave-2 op moves them).
- Transport miss, the lead's: the re-plan order named A6 without its text; W2-tests asked, the
  lead sent it verbatim. The seat saved r1 before A6 arrived, then added its A6 answer (keep the g1
  alternation, D14) in place; the lead froze only after that second idle.
- W2-tests r1 frozen read-only as `w2-tests-plan.r1.frozen.md` (486 lines, sha256 prefix
  `f498ec740c7509af`); tree status unchanged. Sent to the same grader, resumed, with the ruling and
  the note that P1 is writing `src/**` meanwhile; a FAIL goes to the user.

## 2026-10-08 — wave-1 fix round 3 built; re-review 3 sent

- P1 (seat-reported): F1, A1, G1 each red first, then green; `src/ids.rs`, `src/rename.rs` and
  `tests/ids.rs` only; no worker. F1: `written_path`/`resolves_to`, with a strict `links_move` on the
  unclaimed link arm and the bare-name fallback kept for claimed tokens. A1: `stale_spots` resolves
  each occurrence. G1: the goldens' exclude line, `[&str; 9]`. Two existing tests changed and
  disclosed: H3's spot name (rename.rs:489) and the N5 fixture's path (`notes.md:2:11` → `:2:39`).
  706 tests (ids 125); four layers exit 0; the r15 scenario rebuilt in scratch: cite untouched.
- P1's real-tree run showed 46 drift; the +1 was the lead's own build-log line carrying a joined
  "GI-001" example. Fixed on sight by quoting it (D15); the lead's re-run: `621 bare · 45 drift`.
- Re-review 3 sent to `w1-code-review`, scoped to the fix, with both test changes named for
  scrutiny and named regression hunts (`./`/`../`, anchors, encoded or bracketed targets, absolute
  paths); a FAIL goes to the user.

## 2026-10-08 — W2-tests r1 PASS; approved, build held

- `PLAN GRADE: w2-tests · PASS · none` on r1 (sha256 prefix `f498ec740c7509af`). F1 and A1–A7
  closed first-hand (the exclude probed on the current debug binary: a drifting golden gives
  `0 bare · 0 drift`); no regression against r0's verified set. Advisories B1–B5.
- The lead approved, holding the build for W2-schema's 0046, 0047 and 0049. Approval notes, content
  pinned: B1, S1 no longer waits for 0048 (it rides wave 1b; A3's re-derive list governs when it
  lands); B2, check `impl.baseline-diff-review`'s `why` against 0048's text; B3, `rekey.md`'s form
  keeps its bare `D<n>` notes and the stated reason is corrected; B5, re-grep the exclude at build.
- plans: W2-schema PASS(1) · W2-tests PASS(1, user-granted) · W2-prose pending.

## 2026-10-08 — wave-1 re-review 3 FAIL (F2); user grants fix round 4

- `CODE REVIEW: wave-1 mochiko-cli ids · FAIL · A1's stale_spots narrowing lets --write leave dead
  file names silently, against N5 (rename.rs:759-761)`. Layers exit 0 first-hand (706 tests); real
  tree `621 bare · 45 drift`; F1, G1, the H3 spot-name fix and the rewrite-side hunts (`./`, `../`,
  anchors, queries, `<…>`, leading `/`) verified. F2 (High), owned by the reviewer as its own
  round-3 advisory's error: with A1, an unresolvable occurrence of the old name is dropped, so
  `--write` exits 0 leaving dead names (scratch r17w: a bare unique cite, a blob URL, an absolute
  path, a backslash path, the root shorthand, an encoded segment, a path out of the tree). The N5
  fixture change removed the case N5 proves (fails open). B1 (Low): a longer-extension name.
- Put to the user: A, invert A1 (drop only where the path resolves to another existing file) and
  restore N5; B, revert A1. User: "as recommded" (A). Order sent to `p1-crate`, content pinned,
  voiding round 3's A1 rule; B1 folded in; the F1-shape block named as accepted cost.

## 2026-10-08 — W2-prose plan FAIL (items 1, 3, 5); goes to the user

- `PLAN GRADE: w2-prose · FAIL · 1: rule 7 B1 narrowing, rule 10 D15 span rule, rule 11 vs rule
  12, two missed carriers (EXAMPLES.md story headings, analyst-report US2); 3: H-L6 false; 5:
  DR-XXX decided, not asked`. Frozen text verified (sha256 prefix `f0c63a406b75c40d`).
- F1: § IDs rule 7 transcribes B1 before the user's narrowing (C) — a leading `slug` no longer
  owns a line. F2: `authoring-user-stories/references/EXAMPLES.md:10/:28/:45` story headings and
  `templates/analyst-report-template.md:17` `US2` missed (greps too narrow). F3: rule 10 states
  D15 per quote, not per span, and Q-A's default rests on that reading. F4: rule 11 wider than
  D16 and against rule 12/D7. F5: H-L6 false since N1 (three cycle homes). F6:
  `DECISION-RECORD.md:14` `DR-XXX` kept by the seat's own call; belongs in Q-B.
- Passing first-hand: write set disjoint; rung claims; 22 sampled spot lists; hand-offs P1–P11
  and R2; P6; script reds; budgets reproduced exactly; strips; ripple. Advisories A1–A9.
- Bound: wave 2's re-plan round is spent (and W2-tests' extra round was the user's grant).
- Put to the user: A, one more re-plan round, with the lead's answers to Q-A–Q-C; B, narrow the
  scope. User: "as recommded" (A). Lead rulings sent with the order: Q-A default stands; its
  sub-question (F3) ruled — example IDs inside quote spans that quote no source are joined by this
  edit, as the seat's own illustrative text under the gate, each span listed with its class; Q-B
  A widened to `DR-XXX` and `US2`; Q-C default stands, figure ruled at the gate. Findings F1–F6 and
  advisories A1–A8 sent verbatim with their fixes.

## 2026-10-08 — wave-1 fix round 4 built; re-review 4 sent

- P1 (seat-reported): `src/rename.rs` and `tests/ids.rs` only; no worker. F2 inverted — an
  occurrence is dropped only where its written path resolves to a real file that is not a move
  source; unresolvable ones stay stale. N5 fixture restored (`notes.md:2:11`). Three T15 tests red
  first (a unique bare root cite; the reviewer's seven shapes; `.html.bak`). 709 tests (ids 128);
  layers exit 0; real tree `621 bare · 45 drift`; r17w `--write` refused, exit 2, 7 spots, tree
  unchanged; r15's FEATURES cite now blocks (the accepted cost).
- Boundary P1 disclosed: an existing `.bak` file resolves to another real file, so it is dropped.
  Lead ruling: the drop stands — the link names a real file that did not move, so it is not dead;
  put to the reviewer to test.
- Re-review 4 sent to `w1-code-review`, scoped to the round, with hunts named (two candidates,
  symlinked dirs, case-insensitive paths, masked spans, two moves sharing an old name); a FAIL goes
  to the user.

## 2026-10-08 — wave-1 re-review 4 FAIL (F3); put to the user

- `CODE REVIEW: wave-1 mochiko-cli ids · FAIL · F2's "elsewhere" test compares path spelling, not
  file identity: a symlinked-dir or case-variant link to the moved file is dropped and --write
  leaves it dead (rename.rs:769-774)`. Layers exit 0 first-hand (709 tests); real tree unchanged;
  r17 and r15 shapes refuse as ruled; N5 restored; the `.bak` ruling holds once F3 is fixed.
  Advisories: A1 (an unclaimed link rewritten though another candidate exists), A2 (duplicate
  stale spots inflate the count), A3 (the old-name search is case-sensitive, since round 2).
- The last two FAILs sit in the same noise-cutting exception (round 3's A1, a Low advisory). Put
  to the user: A, patch it again with canonical file identity; B, the lead's lean — remove the
  exception: any old name the rename does not rewrite is stale and blocks, matched ignoring case,
  and an ambiguous link is listed, never rewritten.
- User: "as recommded" (B). Fix round 5 ordered to `p1-crate`, content pinned. It voids round 4's
  "elsewhere" drop and the lead's `.bak` ruling, and keeps F1, G1, H3 and N5. Rules: (1) every
  un-rewritten occurrence of a moved file's old name is stale, with no resolution-based drop;
  (2) the stale scan ignores ASCII case and the rewrite side stays exact; (3) an unclaimed link is
  rewritten only when the move source is its sole existing candidate; (4) stale spots are deduped.
  Red first for F3's r21 links, an upper-case name, the r19 two-candidate link, and a shared old
  name. A real-tree preview checks that the case-insensitive scan adds no noise for wave 3.

## 2026-10-08 — W2-prose r1 returned; re-grade sent

- W2-prose returned "plan final": r1, 1,040 lines, frozen as `w2-prose-plan.r1.frozen.md`
  (sha256 prefix `6042a9edd2e6d57a`); tree status unchanged. F1–F6 and A1–A8 addressed; the write
  set is 32 plugin files and 23 strip entries; a new §2.12 lists the joined quote spans. The seat
  flags three calls of its own (rule 10's third bullet, "the ID alone", stop S6) and hands A4 to
  the lead: the cycle-report owner `` `user-auth` ``, the example's own `feature:` key. Provisional
  lead reading, put to the grader: a spec-slug owner (D11).
- Sent to the same grader, resumed, with the rulings on Q-A (and its sub-question), Q-B and Q-C;
  a FAIL goes to the user.

## 2026-10-08 — wave-1 fix round 5 built (option B); re-review 5 sent

- P1 (seat-reported):
  - only `src/rename.rs` and `tests/ids.rs` touched; no worker;
  - the stale drop removed; the stale scan case-folded with offsets kept; spots deduped;
    ambiguous unclaimed links not rewritten;
  - T16 tests red first: the symlink, the case variant, an upper-case name, the two-candidate
    link, a shared old name;
  - 715 tests (ids 134); layers exit 0; real tree `621 bare · 45 drift`, line-for-line unchanged;
  - r21 refused with 2 spots; r15 still blocks;
  - real-tree previews for two session D's: 0 moves, 0 stale. No walked file outside eval stimuli
    carries an `<ID>-<slug>` name, so wave 3 cannot be blocked by the scan.
- Disclosed: the existing-`.bak` test was written after the code (its first premise was wrong).
  P1 proved it bites by restoring round 4's clause for one run. One round-3 test was inverted
  under the ruling, red first.
- Lead ruling on P1's rule-3 edge: a link whose every existing candidate is a move source is
  rewritten (both moves take the same new name). Put to the reviewer.
- Re-review 5 sent to `w1-code-review`, scoped to the round. A FAIL goes to the user.

## 2026-10-08 — W2-prose r1 re-grade FAIL (items 1, 5); goes to the user

- `PLAN GRADE: w2-prose · FAIL · 1: A4 owner user-auth invents a cycle-owner form, so write the
  run key as rule 7 allows; 5: rule 10's third bullet decides an unruled point, so strike it`.
  r0's F1–F6 and A1–A8 closed first-hand; no regression; 34 snapshotted files unchanged.
- G1: the lead's provisional reading (a spec-slug owner) was wrong. A cycle's owner is its run
  key: a joined FEAT or EPIC ID, or `lane-<slug>`. `feature: user-auth` is a stale feature-id
  value (the envelope's contract is `feature: <feature-id>`). Fix: `` `FEAT-XXX-<slug>` ``. The
  stale `feature:` field enters scope only on a ruling.
- G2: rule 10's third bullet (seat-written text inside a quote span joins IDs) widens the lead's
  sub-ruling into a standing rule. It carries an unweighed cost: the tool skips quote spans, so
  such IDs never follow a rename. Fix: strike it.
- Advisories B1–B5 (B1: rule 12's "every mention follows" excludes quote spans).

## 2026-10-08 — wave 1 closed: re-review 5 PASS; W2-schema built; wave 1b planned

- `CODE REVIEW: wave-1 mochiko-cli ids · PASS · none`, from `w1-code-review` at re-review 5:
  - layers exit 0 first-hand (715 tests); real-tree findings byte-identical to round 3;
  - red-first independently re-proved against round-4 code;
  - fresh r19, r20, r21 and r22 copies refuse as ruled;
  - non-ASCII, case-folded boundaries and the H3 dedupe hold;
  - the rule-3 edge (every existing candidate is a move source) leaves no dead link.
- Advisories: A1 (Info, a trailing `_` is not a "longer" boundary; fails safe), booked for the
  backlog. A2 (option B's refusal blocks on other specs' own links to a same-named screen) goes to
  W2-prose: the rename prose tells a seat to exclude the foreign spec files, then run `ids --check`
  on them.
- Wave 1 seats: P1 (`mochiko:staff-engineer`, default tier) produced; `p1-plan-grader` graded the
  plan; `w1-code-review` (`mochiko:tech-lead`, default tier) reviewed over five rounds. The user
  granted rounds 2–5 (C, 1A, A, B).
- S1 go sent to `w2-schema`: build 0046, 0047 and 0049 per r1, with 0048 held for wave 1b. Tree
  snapshot taken before (`tree-before-w2schema-build.txt`).
- `p1-crate` re-issued the pen for wave 1b, plan-only: Q5 (a), `ids rekey` takes the joined old ID;
  A3, `--alias` path scoping. Its build opens after W2-schema's stamps.

## 2026-10-08 — wave 1b plan returned; Q1–Q3 ruled; plan grade dispatched

- P1's plan (338 lines), frozen as `p1b-plan.r0.frozen.md` (sha256 prefix `8dd3e89d6aa6c371`).
  The crate is unchanged around the planning; the tree changes seen are W2-schema's build
  (migrations 0046, 0047, 0049 and 28 views).
- Shape in gist:
  - `rekey`'s old ID may be joined, picking the one holder whose definition carries that slug;
    untied mentions are listed, never rewritten;
  - refusals: a bare number with two holders, an unmatched joined ID, two definitions sharing the
    joined ID;
  - a D15 limb ties a changed token to the chosen holder's slug;
  - `--alias LITERAL[=PATH]`, with the scope carried into the diff check per file;
  - 14 red tests; write set `src/rename.rs`, `src/ids.rs`, `src/cli.rs`, `tests/ids.rs`.
- Q1–Q3 ruled by the lead, tool mechanisms inside D4, D7, D15 and D18, as P1 leaned:
  - Q1: an untied mention does not block `--write`; it is listed, exit 0;
  - Q2: a shorthand alias requires a path scope, while glued prefixes may stay tree-wide; two
    wave-1 tests change under it, disclosed;
  - Q3: path grain, with mixed-meaning files left to wave 3's judgment.
- Graded by `p1b-plan-grader`, a fresh `mochiko:staff-engineer` (default tier), render pasted, with
  named silent-corruption hunts.

## 2026-10-08 — W2-schema built 0046, 0047, 0049; W2-tests' build opened

- W2-schema (seat-reported):
  - the three migrations stamped, with 0048 held for wave 1b; validate is unchanged at
    `0 rejecting · 113 advisory`, 0 clusters, 185 suppressed, with no new advisory;
  - exactly the 28 expected views changed; every changed line maps to the plan's rewords and mints;
  - 14 of 14 drift `shows` strings survive;
  - built in a scratch log copy, then copied in after each file's `check` returned `allow`; no
    cargo, no allowlist row.
- Departures from r1's text, disclosed, for the gate audit:
  - the governance-intent ripple line keeps "(deterministic …)";
  - `spec`'s `FEAT-YYY` derived-features row joins `US-2`;
  - `review-common.joined-ids` carries `anchor: … D6` (the 0008 precedent).
- Lead first-hand checks:
  - `migrate status`: `1..49 (46 migrations)`, state `sha256:f1d8ce9c…`, 87 documents, 1175 rules;
  - `migrate validate`: 0 rejecting, 113 advisory;
  - a fresh `views emit` to scratch diffs clean against `.mochiko/schema-views/`;
  - `git status` shows only the three migrations and the views beyond the earlier set.
- W2-tests' build opened, per r1 with B1–B5. The departures were named, with every red test to be
  reported. P1's 1b build is held until W2-tests reports, so `cargo test` stays single-writer.

## 2026-10-08 — wave 1b plan FAIL (F1–F4); re-plan round sent

- `PLAN GRADE: p1-wave1b · FAIL · 1, 3`:
  - F1: holder identity is per line, so three shipped shapes (a D-family index row plus heading,
    an AX catalog heading plus graduated file, a record summary plus card) read as several
    holders; 0048's own case would be refused.
  - F2: a shorthand naming a real session passes and re-owns.
  - F3: a directory `=PATH` re-creates A3's mis-own.
  - F4: the D15 limb's slug set spans every holder; moves are unchecked.
  - Probes ran in scratch only; the repo is untouched. Advisories A1–A8.
- Wave 1b's one re-plan round, sent content-pinned. Lead rulings:
  - a holder is the group of sites carrying one slug;
  - a bare site beside slugged ones is never a second holder;
  - the identical-slug refusal fires only on two same-kind sites in one file;
  - fixes F2–F4 as named.
- A7 folded into scope by the lead: `ids rename` refuses a number with two or more holders. The
  bare-holder dead end is booked for the backlog. A further FAIL goes to the user.
- r1 returned "plan final":
  - 428 lines, frozen as `p1b-plan.r1.frozen.md` (sha256 prefix `c3915d273f4f129b`), with crate
    source hashes recorded;
  - holders are grouped by equal scanned slug, with site kinds from the index's `Leaders` set plus
    a file-name kind;
  - F2–F4, A1–A8 and A7's rename refusal are planned;
  - 20 reds and 3 guards; about 950–1,100 lines.
- Q4 ruled by the lead as P1 leaned: "the number is shared" means two or more holders, or one file
  holding a bare site of a holder's kind. This closes a bare legacy entry silently renumbered
  beside a joined one (GI-005-record-layer-integrity). It never makes a bare site a holder. Sent to the same grader.

## 2026-10-08 — wave 1b r1 FAIL (R1, R2); goes to the user

- `PLAN GRADE: p1-wave1b · FAIL · 1, 5 (R1) · 4 (R2)`.
  - R1: the same-kind refusal's reach is stated three ways. Read narrowly, a bare old-ID rekey on
    the identical-slug landing shape renumbers both entries silently. Read strictly, 14
    session-D keys with amendment or reversal cards (e.g. `cli-schema-delivery` D3-rules-delivery-binding, :400 and
    :1037) can never be re-slugged after wave 3.
  - R2: task 9's red is green on arrival; the three-word other-holder token and a bare token,
    which the limb must block, pass today.
- Closed first-hand: r0's F1–F3, A1–A8, and F4's set and move limb. The grader emulated the
  grouping and the kinds in scratch: the three shapes separate. Q4 fires falsely nowhere (0 of 621
  repo keys, 0 of 160 eval trees, 0 in shipped templates). A7 refuses none of wave 3's first
  renames.
- Advisories: B1, the AX guard follows the store template (no `concerns.md` table row). B2, a bare
  legacy entry of different kinds can still tie (no such shape today). B3, wave 3's first rename
  of `model-tiered-seats` D1-usage-accounting-unit also joins two bold quotes of other sessions' D1 (:86, :92), which
  goes to wave 3's per-apply review.
- Put to the user with the W2-prose decision: A, one more round with the refusal on rekey only,
  never rename (the lead's lean); B, one more round, refusal everywhere.

## 2026-10-08 — user grants wave 1b and W2-prose one more plan round each

- User: "as recommded" to both.
- Wave 1b (A): the same-kind refusal applies to `rekey` (bare or joined old ID) and never to
  `rename`. Accepted cost: a rekey of an amendment-card decision is refused. A7's shared-number
  rename refusal stands. Reds: a bare and a joined identical-slug landing rekey. Guards: rename of
  an amendment-card decision passes, its rekey refuses. R2 aimed at the three-word and bare tokens;
  B1 and B2 folded in.
- W2-prose (A):
  - G1 owner `` `FEAT-XXX-<slug>` `` (the lead's provisional reading voided);
  - G2 rule 10's third bullet struck;
  - B1, B2 and B4 folded in;
  - wave 1's closing A2 refusal guidance added to the § IDs rename line;
  - stale `feature: user-auth` left untouched and booked for the backlog.
- Booked for the backlog at landing (running list):
  - Q-T2's pre-existing kit drift;
  - wave 1's A1, the trailing-`_` boundary;
  - wave 1b's bare-holder dead end;
  - the stale `feature: user-auth` examples (`CYCLE-REPORT-FORMAT`, `REPORT-TEMPLATES`).
- W2-prose r2 returned "plan final":
  - 1,143 lines, frozen as `w2-prose-plan.r2.frozen.md` (sha256 prefix `88ba06164c9eeef9`);
  - G1, G2, B1, B2, B4 and the A2 refusal sentence applied;
  - S3 re-read against the landed log (`1..49`), with 0 of the 34 snapshotted files changed;
  - plugin prose, strips and `CHANGELOG.md` unchanged in the tree.
- Sent to the same grader, scoped to the touched spots plus a regression check.
- Wave 1b r2 returned "plan final":
  - 457 lines, frozen as `p1b-plan.r2.frozen.md` (sha256 prefix `3fa5cdc8a41e41fc`), with crate
    hashes unchanged;
  - R1's one reach is stated once, with the reds and the rename guard as ordered; R2 is re-aimed;
  - B1's AX guard follows the store template; a new red, the AX catalog as owning file, closes
    today's first-read refusal;
  - B2 named; 22 reds, 5 guards.
- P1's observation, booked: on the AX shape, the derived `ARCHITECTURE.md` link text lists stale
  and blocks the write (option B). This repo has no AX store. Sent to the same grader.

## 2026-10-08 — W2-prose r2 PASS; approved, build held for W2-tests' gates

- `PLAN GRADE: w2-prose · PASS · none` on r2 (sha256 prefix `88ba06164c9eeef9`).
  - G1, G2, B1, B2 and B4 closed first-hand. The A2 sentence was checked against `stale_spots`,
    `walk`, `check()` and today's `--help`.
  - S3's ten views were read, with no gap. The three W2-schema departures are as stated.
  - No regression against r1.
- Approved by the lead. The build is held until W2-tests' gates finish, since its host-only
  contract cases read `plugins/mochiko/`. Approval notes:
  - C1: the refusal sentence adds that a spot meaning the moved file is fixed by hand;
  - C2: stale plan text read as "no standing line beyond rule 12's";
  - C3: name A7 in the gate brief if 1b lands first;
  - C4: the stepped replay attributes the grown renders.
- plans: W2-schema PASS(1) · W2-tests PASS(1, user-granted) · W2-prose PASS(2, user-granted).

## 2026-10-08 — wave 1b r2 PASS; approved, build held for W2-tests' report

- `PLAN GRADE: p1-wave1b · PASS · none` on r2 (sha256 prefix `3fa5cdc8a41e41fc`), checked
  first-hand by the grader:
  - R1 and R2 closed, and B1's probe reproduced;
  - the AX-catalog red is inside the holder rule, not new scope;
  - task 17: the real-tree rename preview of `cli-schema-delivery` D3-rules-delivery-binding exits 0 with both cards
    rewritten; 0 of 621 numbers are shared;
  - no regression.
- Approved by the lead, with notes:
  - C1: Q4's holder means a slugged holder; an all-bare group never marks a number shared;
  - C2, ruled: rekey's bare owning check widens to any of the holder's sites, while rename keeps
    today's first-read check; the asymmetry is booked;
  - C3: the joined variant's reason assert;
  - C4 and C5: residual risks named in the report (disjoint-kind identical slugs; GI multi-cell
    tables).
- The build opens on the lead's word after W2-tests reports. W2-tests' crate test review will
  read a frozen copy, so P1's build stays single-writer in the main tree.
- Booked (running list): the rekey/rename owning asymmetry; the AX `ARCHITECTURE.md` link-text
  stale.
- plans (wave 1b): P1 PASS(2; the second round user-granted).

## 2026-10-08 — W2-prose build opened; W2-tests' report pulled

- W2-tests' interim (seat-reported): the four layers green — `cargo test --all` exit 0, no red
  test left — and its 32 changed paths `0 bare · 0 drift`. Three late gates (the opt-in
  similarity test, the release build, `contract --host-only`) ran past 04:24 with no report. At
  15:31 local no cargo or eval process was running, so the full report was pulled.
- W2-prose's build opened: the gates reading `plugins/mochiko/` are done, and its write set is
  disjoint from W2-tests' and P1's. The lead corrected the seat's C3 restatement (A7 refuses two
  or more holders, not files). P1's 1b build waits for W2-tests' full report.

## 2026-10-08 — W2-tests built; review sent on a frozen copy; wave 1b build opened

- W2-tests' report (seat-reported; evidence in `scratchpad/w2t-build/`):
  - T1–T5 each red first, then green: 14 goldens regenerated, census 362/813/1175, pointers 88,
    and the matrix figures; T5's new `tests/ids_definition_lines.rs` (11 bare rows exact, a "GI-020"
    negative control, two red proofs);
  - command kits and five review kits updated (65 insertions, 0 deletions);
  - four layers exit 0, with no red test left; `MOCHIKO_FULL_SIMILAR=1` 48 passed; release build
    exit 0; `contract --host-only` 7/7 (filtered, not a gate run);
  - task 1a and the task 10 write set `0 bare · 0 drift`; the control under `other/` 2 drift;
  - W2-schema's departures move nothing unpredicted.
- Disclosed: T5's FEAT row is held by its file name, so it proves line-name agreement. Implement's
  check-rubric red predates the wave (Q-T2, booked). B2 waits for 0048.
- Frozen review copy taken at `scratchpad/w2t-review-tree/` (no `.git`, `target` or
  `evals/.work`), with plugin prose still clean. Crate test review sent to `w1-code-review` there.
  Kit changes go to the gate audit.
- P1's 1b build opened in the main tree, with W2-tests' T5 named as a test that must stay green.
  W2-prose builds plugin prose in parallel; all three write sets are disjoint.

## 2026-10-08 — W2-tests' crate tests PASS; W2-prose built; 1b S4 ruled

- `CODE REVIEW: wave-2 W2-tests crate tests · PASS · none`, on the frozen copy:
  - 716 tests, four layers exit 0, full similarity 48, release build exit 0;
  - figures traced by replaying the HEAD log against the new one, with an op tally of 0046
    (10 template replaces, no rules), 0047 (14 text-only rewords) and 0049 (8 mints);
  - all 24 goldens re-rendered byte-identical; task 1a reproduced; no weakened assert;
  - T5 meets the hand-off intent: mutation probes reproduce the seat's red 2, and FEAT is held by
    its file name.
  - Advisories: A1 (Low), T5's FEAT comment overclaims — add a bare-named FEAT row or narrow the
    comment, routed to W2-tests' 0048 round; A2, P1 keeps `ids`' public API (relayed); A3, 0048's
    re-derive list.
  - A layout note: one existing test needs the target inside its tree.
- Wave 1b, S4 (P1): `two_moves_onto_one_name_are_refused_and_nothing_is_lost`'s FEAT half is now
  refused by A7 (two holders) before the collision check. Ruled: change only its reason assert, and
  keep the fixture and the nothing-lost asserts; L3's test still pins "both move to". P1's read-only
  task 17 previews match the pre-1b binary for an unshared bare rekey and for the
  `cli-schema-delivery` D3-rules-delivery-binding and `human-readable-ids` D3-slugged-id-families renames. A file-scoped alias touches only
  its file; a pathless shorthand is refused.
- W2-prose built (seat-reported; the report was cut short in transit and the rest is pulled):
  - 32 plugin files, 22 strip entries plus one new strip file, and one unreleased `CHANGELOG.md`
    entry;
  - § IDs is 6,825 characters against ~5,000 planned;
  - `GI-031` replaces the plan's "`GI-020`" example under its collision guard;
  - one NFR slug changed to avoid a slug clash.
  - Q1: `validate-requirements.py` counts only the first FR in a bulleted list (pre-existing; the
    SKILL template's own form). Ruled by the lead: fix it test-first in this build (a one-token
    lookahead), with a strip line, disclosed to the gate as a scope addition.

## 2026-10-08 — wave 1b built; review sent; W2-prose's Q1 fix landed

- P1's wave 1b build (seat-reported; evidence `scratchpad/b1-*`):
  - 22 reds, each red for its stated reason and then green, plus 5 guards;
  - four existing tests changed and named: Q2 ×2, S4's A7 reason assert, and a mechanical
    `shared: false`;
  - `ids` 161 tests; T5 green and its API unchanged;
  - four layers exit 0; real tree `621 bare · 45 drift`, unchanged;
  - task 17 previews byte-identical to the pre-1b binary (an unshared bare rekey, the
    `cli-schema-delivery` D3-rules-delivery-binding and `human-readable-ids` D3-slugged-id-families renames); a file-scoped alias touches only
    its file;
  - about 1,290 lines added against 1,000–1,170 planned.
- Residual risks named: B2, C4, C5, the C2 asymmetry, and the AX link text.
- Review sent to `w1-code-review`, diffed against the frozen pre-1b copy
  (`scratchpad/w2t-review-tree/`). The crate is quiet in the main tree. One review fix round; a
  second FAIL goes to the user.
- W2-prose's Q1 fix (seat-reported): a red bulleted fixture gave `fr 1`, green gives 2. Every
  script fixture was re-run with no regression; a real bulleted spec goes from 1 to 6 FR. A strip
  line was added. One hook deny on a `cp` into a declared home was followed by Write/Edit, the
  named route. Its report was cut short twice in transit and was pulled to a scratch file.

## 2026-10-08 — W2-prose report read; wave-2 gate audit dispatched

- W2-prose's full report (`scratchpad/w2p-build-report.md`, 393 lines):
  - § IDs written as 13 bullets; the size is argued, with about 300 characters of trim candidates
    named;
  - the joined example slug map has no slug on two IDs;
  - a stepped-replay budget table puts the render growth on W2-schema (+571/+570/+568/+568/+564
    on the five review skills via 0049's common block) and the seat's own body growth at +96/+35/
    +84/+84/±1;
  - `ids --check` over the 32 files `0 bare · 0 drift`.
- Departure, disclosed, a process concern: the 23 strip writes went through a Python script, so
  the write-time conformance hook never saw them. The seat's `mochiko-cli check` dry-runs and
  re-checks allowed all 23. Put to the gate as a first-hand pre-pass.
- One fence span joined outside §2.12's list (`testing-end-user/references/TASK-PARSING.md:77-82`).
- Gate audit dispatched: `w2-gate-audit`, one plain `general-purpose` seat on `model: opus` (the
  producers' tier).
  - The seat renders `validation-primitive-edit` first-hand and diffs it against the lead's copy
    (`vpe-render-1-49.txt`, identical to the pre-wave render).
  - Units: schema content S1–S3 (0046, 0047, 0049), skill pairs K1–K4, prose P1–P23, strips and
    `CHANGELOG.md`, plus a separate non-gate read of the eval kit changes.
  - Ten departures and flags named; the report lands in `reports/w2-gate-audit.md`.

## 2026-10-08 — wave 1b review FAIL (F1); fix round sent

- `CODE REVIEW: wave-1b mochiko-cli ids · FAIL · one entry whose definition sites drifted reads
  as two holders: A7 then refuses the drift fix, and its own advice (a joined rekey) splits the
  entry with exit 0`.
  - Layers exit 0 first-hand (743 tests); red-first proved on pre-1b plus a scaffold (25 fail as
    stated).
  - 41 of 42 real-tree previews are byte-identical; the one difference is the ruled R1 refusal.
  - A full rename sweep over every session D and GI-001–022: 618 identical outputs.
  - Alias scope clean.
- F1 (High), scratch x4 and x5: one entry whose definition sites carry different slugs groups as
  two holders. A7 refuses the drift fix and advises a joined rekey, which splits the entry with
  exit 0 and a clean check — a wrong pointer (GI-005-record-layer-integrity) no ruling accepted.
- B2 is broader than booked (x2): a joined old ID also renumbers a legacy bold bare site.
  Advisories: A1 (path normalisation), A2 (list another holder's kept mentions).
- Fix round ordered (wave 1b's one), with the lead's rulings:
  - a shared number is provably two entries only on same-kind sites in one file, two entry files,
    or Q4; otherwise it is ambiguous and the joined rekey refuses with drift-or-landing advice;
  - no "act as one entry" on an ambiguous number (fail-safe; wave 3 unaffected);
  - B2 closed by a refusal on a bare site of a kind the chosen holder lacks;
  - A1 and A2 folded in.
  A second FAIL goes to the user.

## 2026-10-08 — wave-2 gate audit PASS, 30 of 30; post-gate work resumed

- `w2-gate-audit` (plain `general-purpose`, `model: opus`), report
  `reports/w2-gate-audit.md` (`report: review`, the hook allowed it):
  - contract rendered first-hand, byte-identical to the lead's; floor pin 11;
  - pre-pass first-hand: 0 rejecting · 113 advisory; views equal the replay; all 23 strips allow;
    budgets reproduced;
  - every overage held under D6/D21; eval kits PASS.
- Outcome lines (unit · seat · tier · files · rounds · blocking), all `w2-gate-audit · opus ·
  1 rounds · 0 blocking`: S1 0046 (11 files) · S2 0047 (11) · S3 0049 (9) · K1
  authoring-requirements (4) · K2 authoring-user-stories (4) · K3 patterns-vertical-tdd (4) · K4
  testing-gap-finding (2) · P1–P23 prose primitives (2 each). The verbatim lines are in the report.
- Advisories:
  - P2 `report-format.md` rule 5 examples lack owners (the most material);
  - P1 the quotes bullet omits the `.html` unmask;
  - S1 `governance-intent.yaml:160`'s check reads `GI-0XX`;
  - S2 `impl.baseline-entry-grammar` leaves the proposed value implicit.
- The lead's process miss, named by the grader: the brief paraphrased `one-seat-per-wave`, which
  `brief-carries-unit` does not allow. The grade ran against the render only, so the verdict
  stands.
- H-L1 confirmed: the installed `mochiko-cli` 0.3.0 has no `ids`, so a release carrying it is a
  wave-4 precondition.
- Lead rulings:
  - Q-C: the common row is set to 1,917;
  - P1 and P2 fixed now (one clause each, strip lines added), with the same gate seat re-reading
    only those two;
  - S1's check reword folded into W2-schema's 0048 round;
  - S2 booked.
- W2-prose resumed for the 18 ledger rows, the CHANGELOG budget sentence and the P1/P2 fixes,
  with Write/Edit only.

## 2026-10-08 — wave 1b fix round built; re-review sent

- P1 (seat-reported):
  - `Holders::provably_two()` — (i) same-kind sites of two holders in one file, (ii) two entry
    files, (iii) Q4 — so a joined rekey on an ambiguous number refuses with the drift advice; both
    shared-number refusals name both readings;
  - B2 closed for rekey; A1 `scope_path()`; A2 `kept` lines;
  - 7 tests (reds x4, x5, x2, both readings, A1, A2, plus a two-AX-headings guard); one test
    corrected before code after a wrong-reason red; one mechanical `kept: vec![]`;
  - 168 `ids` tests; four layers exit 0; real tree unchanged; task 17 previews byte-identical
    apart from the ruled R1 refusal.
- New residual P1 raised: x2 under `ids rename` merges a legacy bare bold entry into the joined
  holder's slug. This is unchanged from wave 1, and no shipped template writes the shape; it cannot
  be told apart from a half-joined single entry. The lead accepts it as a residual and books it.
  Put to the reviewer to challenge.
- Re-review sent to `w1-code-review`, scoped to the fix. A FAIL goes to the user.

## 2026-10-08 — W2-prose post-gate resume done; P1/P2 gate re-read sent

- W2-prose (seat-reported; `scratchpad/w2p-resume-report.md`, diff `w2p-resume.diff`):
  - 17 skill ledger rows plus the common row (1,917, method named: characters of the view, python
    `len`; 1,274 at `bf410cc`), with render growth split per migration by stepped replays;
  - `authoring-constitution`'s old overage is retired (re-measured inside);
  - the CHANGELOG budget sentence;
  - P2: rule 5's examples carry owners; P1: the quotes bullet notes the HTML exception;
  - every write dry-run with `check` and written with Edit, byte-identical to its dry run.
- Lead: the seat's three ledger choices stand; one CHANGELOG clause added for Q1's FR-count
  change. P1/P2 re-read sent to `w2-gate-audit`, resumed, with its round-2 section appended to the
  report.

## 2026-10-08 — wave 1b closed: fix-round re-review PASS; 0048 go

- `CODE REVIEW: wave-1b mochiko-cli ids · PASS · none`, checked first-hand:
  - 750 tests, T5 green, layers and release build exit 0; real tree byte-identical;
  - red-first re-proved on pre-fix 1b (6 fail as stated);
  - x2–x9 re-run: drift refused, landings move one holder with `kept` lines, unshared unchanged;
  - 41 of 42 previews byte-identical (the ruled R1 refusal is the difference); the 618-rename sweep
    identical;
  - `provably_two` checked against every shipped shape; A1 widens no scope.
  - Residual 1 (the x2 rename merge) agreed, not blocking, and booked.
- Advisories booked for the backlog:
  - A1: the joined-ambiguous refusal should name both readings;
  - A2: the B2 advice should name both readings.
- The reviewer's observation (the ledger file changed) is W2-prose's ordered post-gate write.
- Wave 1b seats: P1 produced; `p1b-plan-grader` (`mochiko:staff-engineer`) graded the plan, three
  rounds, the third user-granted; `w1-code-review` reviewed, two rounds.
- 0048 go sent to `w2-schema`: S2 re-reads 1b's `--help`, and 0048's text must match what the tool
  does. The gate's S1 advisory (`governance-intent` check `GI-0XX`) is folded in, its op shape the
  seat's call, disclosed.

## 2026-10-08 — wave 3 opened in parallel: slug map shards dispatched plan-only

- `wave3-backfill.md` written, transcribing D7, D9–D11, D13–D19 and D22 plus the wave 1/1b rulings.
  Order: shard plans, then shard builds, the map grade, 0048's gate, apply, diff review, strips
  and gate, the done check, and the close (D18's read-test clause goes to the user).
- The slug map is sharded three ways by session name order, balanced on a definition census
  (384 heading cards and 240 bold or table lines across 74 records; 5 session directories have no
  `record.md`):
  - W3-A: 30 sessions plus GI-001–022;
  - W3-B: 22 sessions, this session's record among them;
  - W3-C: 27 sessions plus the repo-only forms, the session-prefixed alias rows, shorthand
    qualifiers and `.mochiko/decisions/`.
  Lists are in `scratchpad/w3-shard-{A,B,C}.txt`.
- Seats `w3-map-a`, `-b` and `-c` (persona-less `general-purpose`, `model: opus`) were dispatched
  plan-only. Each enumerates as the tool indexes, writes only its `inputs/slug-map-<x>.tsv` at
  build, and flags hazards for review. One fresh generic seat grades all three plans; one re-plan
  round, then the user.

## 2026-10-08 — P1/P2 re-read PASS; 0048 and 0050 landed; audits and re-derive sent

- Gate round 2: P1 and P2 PASS again with 0 blocking. The HTML clause matches `quotes_masked`; the
  owners match the IDs section. Reversing the resume hunks restores round 1 exactly. Outcome lines:
  `audit: P1 templates/artifact-format.md · w2-gate-audit · opus · 2 files · 2 rounds · 0
  blocking` and the same for P2 `templates/report-format.md`.
- CHANGELOG: the Q1 clause was added (check allow, Edit, byte-identical).
- W2-schema (seat-reported):
  - S2 re-read 1b's `--help` and probed a scratch landing: a joined rekey moves one holder, the
    other is `kept`, bare is untied, and a bare old ID is refused;
  - **0048** rewords the floor rule `impl.baseline-diff-review` (anchor "`human-readable-ids D4`";
    id, class and anchors kept) to name the joined old ID, with untied mentions settled by hand;
  - **0050**, its own migration under D21, is a `replace-document` of `template/governance-intent`
    changing one token (`GI-0XX` → `GI-0XX-<slug>`);
  - two views changed; validate unchanged; 14 of 14 `shows` strings hold.
  - The seat asked about a refusal path in the floor rule. The lead declined: a new step, and the
    tool's refusal text names its own next step.
- Lead first-hand: `migrate status` `1..50 (48 migrations)`, state `sha256:c7e23ab0…`, 1,175
  rules; `0 rejecting · 113 advisory`; a fresh `views emit` diffs clean.
- Sent:
  - S4 (0048) and S5 (0050) to `w2-gate-audit` as new units, round 3;
  - W2-tests' re-derive round (sequence 48 and 50, B2's `why`, the 0050 goldens, review advisory
    A1 on T5's FEAT row);
  - W2-prose: the CHANGELOG names 0048 and 0050.

## 2026-10-08 — wave-2 gate round 3: S4 and S5 PASS; the wave's primitives all pass

- `audit: S4 0048-landing-rekey · w2-gate-audit · opus · 2 files · 1 rounds · 0 blocking` ·
  `audit: S5 0050-governance-intent-check-joined · w2-gate-audit · opus · 2 files · 1 rounds · 0
  blocking`. The report is now `round: 3`, units 32 of 32, 32 PASS.
  - Pre-pass first-hand: `1..50`, `c7e23ab0…`, `0 rejecting · 113 advisory`; views equal the
    replay; 383 floor and fail rules survive.
  - S4's text agrees with `ids rekey --help` and the crate's untied listing, so the lead's
    no-refusal-path ruling holds.
  - S5 closes round 1's S1 advisory.
- Advisory, a crate follow-up booked: two refusals name no next step — R1 (`rename.rs:268-273`, same
  slug twice in one file) and the number-only path refusal (`:321-324`). Both exit 2 and nothing
  lands wrong. Booked with wave 1b's A1 and A2 as one item: `ids` refusal texts name their next step
  and both readings.

## 2026-10-08 — wave 3: all three shard plans in; questions ruled; one plan grade dispatched

- Plans returned, plan-only, no repo writes reported: W3-A 444 lines (224 rows: 22 `GI` plus 202
  session `D` in 21 of 30 sessions; its harness and an independent regex census agree 217 of 217),
  W3-B 478 lines (195 rows), W3-C 521 lines (202 rows, plus alias and literal tables). Frozen
  read-only in the scratchpad: `w3-plan-a.r0.frozen.md` `59a30728b9b7496f`,
  `-b` `a60bc926eb99700d`, `-c` `55e4e20f58e94086` (sha256 prefixes).
- The lead ruled the plans' questions within the record's rulings: S1–S10 in `wave3-backfill.md`
  ("Rulings on the shard plans"). In short: one shard-file shape; the 18 glued `AR-D`/`AT-D`/`ER-D`
  definitions normalized by hand under B2; `build-vs-off-the-shelf` and `agent-decoupling` left
  bare (booked); hand rows (`normalize` · `qualify` · `quote`) run before the apply, one file per
  shard; recorded evidence excluded as history; in-tree coins adopted; one `ids literal` per `J`/`FP`
  series; other heads and `R-n` out (booked); clause pointers left as written (booked).
- Wave plan corrected: `--exclude` takes a literal prefix, no glob, so item 7's patterns are now an
  explicit "Exclude list" (`*/reports/*-raw.md` matched no file); item 10's done check takes the same
  list (W3-A H6: with `evals/.work/` alone it reads the map's own titles).
- S11, the 343 path code-span cites W3-C found (the tool reads `` `…/<slug>/record.md` `` as an
  unresolvable qualifier): put to the user — A, leave bare, or C, a small crate change run in
  parallel with the map build. Open. Neither answer changes a map row.
- Lead's miss, disclosed: the first append of the rulings went through a shell heredoc and the write
  hook denied it; re-done with Edit.
- Plan grade dispatched: `w3-plan-grader`, fresh `general-purpose`, `model: opus` (the producers'
  tier), the skill named and its working-tree render delivered by path (`scratchpad/rsp-render.txt`,
  7 blocks, read in full first) rather than pasted — the path route `review-seat-plan.two-way-delivery`
  allows, recorded here; all three plans by frozen path with sha check (the 2026-10-06 transport
  precedent); scope = `wave3-backfill.md` items and rulings; default FAIL.

## 2026-10-08 — wave-3 plan grade: A and C PASS, B FAIL on item 5; A and C building

- `PLAN GRADE: w3-map-a · PASS · none` · `PLAN GRADE: w3-map-b · FAIL · 5: no attempt bound; add
  "Attempt bound: one re-plan round on a plan FAIL; past it, the user."` ·
  `PLAN GRADE: w3-map-c · PASS · none`. The grader re-ran each shard's `ids --check` count (224,
  195 bare plus 18 drift, 202), C's 12-prefix census (773) and A's H1 lines first-hand; all hold.
- B sent the one-line fix, quoted; its re-plan round is spent; the same grader re-grades.
- Advisories taken as binding build notes: A's coins found only in `wave0-read-test.md` (now
  history, S4) are fresh coins, tagged so; GI-004-primitive-audit-ratchet and GI-005-record-layer-integrity carry their second own sites
  (`governance-intent.md:56`, `:57`); C takes `definition` from the index's own sites, not a copy of
  `strip_leaders`.
- Cross-shard advisory ruled: one writer per hand-row line, by file owner (S3's new sub-bullet in
  `wave3-backfill.md`); every one of the 79 session directories sits in exactly one shard list.
  Cross-file needs are relayed through the lead.
- A and C approved and resumed to build (write sets: `inputs/slug-map-a.tsv`, `hand-a.tsv`;
  `inputs/slug-map-c.tsv`, `alias-c.tsv`, `literal-c.tsv`, `hand-c.tsv`). B waits for its re-grade.
- B's revision: the lead's diff of r0 and r1 shows the one line only; frozen as
  `w3-plan-b.r1.frozen.md` (`b86faba96531aa2d`). `PLAN GRADE: w3-map-b · PASS · none`, re-graded by
  the same grader, which re-ran `195 bare · 18 drift` and the crate hashes. B approved and resumed
  to build (`inputs/slug-map-b.tsv`, `hand-b.tsv`); it owns the `quote` rows for W3-A's H4 tokens
  and W3-C's `record.md:901` note, all in `human-readable-ids/`.
- Pen note for the apply: `build-log.md` is live layer and holds a hand row (`:186`); the lead
  holds its own build-log writes while the apply seat holds the pen, and appends after.
- Plans line so far: `plans: w3-map-a:PASS(0) · w3-map-b:PASS(1) · w3-map-c:PASS(0)`.

## 2026-10-08 — wave-2 close in progress: W2-tests re-derive done; re-review sent; CHANGELOG pulled

- W2-tests and W2-prose had both gone idle with no deliverable to the lead; both were pulled
  (fan-in confirmation).
- W2-tests (seat-reported): four layers exit 0, similarity 48 passed; `fidelity.rs` takes
  sequences 48 and 50, census unchanged 362/813/1175; `validate.rs` and `matrix_similar.rs` comments
  only; one golden moved, `governance-intent.check.txt` by one line (`GI-0XX` to `GI-0XX-<slug>`), the
  other 23 byte-identical; T5 gains a 12th row (`FEAT-005.md`, advisory A1), red-probed. B2's `why`
  holds, so no kit edit. The seat's whole-tree run found 24 findings in `evals/.work/`, already in
  wave 3's Exclude list.
- Lead first-hand: against the frozen pre-copy `w2t-review-tree/`, exactly those five test-side
  files differ (plus P1's reviewed `tests/ids.rs`); no `evals/` file differs. Frozen after-copy
  taken at `scratchpad/w2t-rederive-tree/` (no `.git`, `target`, `evals/.work`, or the local
  settings file). Re-review sent to `w1-code-review` on the two frozen trees, layers first-hand.
- W2-prose is re-reading `migrate validate --report` at 1..50 before it quotes the figure, then
  applies its CHANGELOG draft naming 0048 and 0050; the lead gave the go.

## 2026-10-08 — S11 ruled C: wave 1c opened, P1 plan-only

- The user ruled S11, the 343 path code-span cites: "as recommended" (the lead's lean C). Recorded
  on `record.md` D11 ("Changed at build"), `wave1-crate-ids.md` requirement 13, and S11 in
  `wave3-backfill.md`. Fallback if 1c's code review fails twice: A, the mentions stay bare.
- P1 (`p1-crate`, `mochiko:staff-engineer`, default tier) dispatched plan-only. The build is fenced
  out of the main tree's `crates/` while the shard seats use `target/debug/mochiko-cli` and the
  `ids.rs`/`rename.rs` hashes: it runs in a scratch copy (`scratchpad/w1c-tree/`), is reviewed there
  frozen, and lands later by a lead-ordered, sha-checked copy, before the apply.
- Loop: a fresh `mochiko:staff-engineer` peer grades the plan; `w1-code-review` reviews the code with
  the four layers first-hand.

## 2026-10-08 — W2-tests re-derive PASS; commit 1 made

- `CODE REVIEW: wave-2 W2-tests re-derive · PASS · none`, on the frozen trees: test 0 (750
  passed), fmt 0, clippy 0, audit 0; release build 0; similarity 48 passed. Log ops traced: 0048 one
  `reword-rule` (class and kind kept, still a floor), 0050 one `replace-document` changing one
  check line; validate figures unmoved (pointers 88, scanned 1175, clusters 0, suppressed 185); all
  24 goldens re-render byte-identical; T5's new row held by its line under a mutation probe; B2's
  `why` holds. `src/**` and `tests/ids.rs` byte-equal to the wave-1b code passed.
- Commit 1 (user: "commit strategically at the right time"), `d331f1d` on `human-readable-ids-build`,
  not pushed: the `ids` tool — `src/{ids,rename,cli,conform,home,lib}.rs`, `tests/ids.rs` — plus
  `wave0-read-test.md` and `wave1-crate-ids.md`. Proven first on HEAD plus those files alone in a
  scratch tree: 749 tests pass, fmt 0, clippy 0 (`Cargo.toml`/`Cargo.lock` unchanged, so audit is
  HEAD's). The archive of `d331f1d` diffs clean against that tree.
- Disclosed: the first scratch run put `CARGO_TARGET_DIR` outside the tree, and
  `matrix_similar::a_log_whose_own_tree_carries_no_allowlist_keeps_walking_up` failed — the test walks
  up from the target dir for the repository's allowlist. Re-run with the target inside the tree: green.
- Commit 2 (wave 2) waits for W2-prose's CHANGELOG line.

## 2026-10-08 — wave 1c: P1's plan in; Q1 and Q2 ruled at its lean; plan grade sent

- P1's plan (232 lines; frozen `scratchpad/p1c-plan.r0.frozen.md`, `d576a960d3eceb46`): one change
  in `resolve` (`ids.rs:1348`–`:1361`), a new `record_session(qualifier)` reading the tree-root
  record path as its slug for session `D` only; rename, rekey, the diff check, the alias guard and
  `line_sessions` untouched. Seven tests written red or as guards first. Whole-tree before/after
  comparison of `--check` and three rename previews. Built in `scratchpad/w1c-tree/`, target in the
  scratchpad. Its census (by the mention's own qualifier): 388 code-span record paths in 111 files,
  359 canonical; the exact figure comes from the build's whole-tree run.
- Main tree untouched by planning: the three crate hashes are unchanged (no `dirty`).
- Q1 and Q2 ruled by the lead at the plan's lean, inside the user's ruling text (recorded under
  requirement 13): a path qualifier names its mention's owner only, never the line's; only the
  tree-root path counts.
- Plan grade: `p1c-plan-grader`, a fresh `mochiko:staff-engineer` (persona default tier),
  `review-seat-plan` render by path (`scratchpad/rsp-render.txt`), plan by frozen path with sha
  check, default FAIL.

## 2026-10-08 — W3-A built: 242 map rows, 89 hand rows; empty-cell shape ruled

- W3-A (seat-reported): `inputs/slug-map-a.tsv` 242 rows (220 `D`, 22 `GI`; 18 of them the
  normalized `AR`/`AT`/`ER` cards, previewed after normalize) and `inputs/hand-a.tsv` 89 rows (18
  `normalize`, 71 `qualify`). Lint clean (grammar, banned words, uniqueness, sort, index join
  242/242). Previews: 224/224 exit 0 on the real tree; a scratch simulation with all hand rows
  applied, 25/25, no foreign leak. 12 rows for other shards' files.
- Lead first-hand: field counts in `slug-map-a.tsv` are 147 × 6 and 96 × 7 — the Write tool drops a
  trailing tab. Ruled W3-A's option b: an empty cell carries `-` (S1, added); W3-A rewrites.
- Relayed verbatim (S3, one writer per line): 7 `quote` rows in `human-readable-ids/` and the
  `pm-role-and-feature-derivation` :86 residual to W3-B; 5 `qualify` rows in `skill-content-schema`
  and `teammate-message-races`, plus W3-A's alias-need list, to W3-C.
- Pen note: two relayed rows sit in this session's `record.md` (:886, :894), shifted ten lines by
  the D11 entry of this morning. The lead holds further `record.md` edits until the hand rows are
  applied; the apply seat stops on any `before` not found on its line.
- Residuals for the diff review, no rows (W3-A): `primitive-edits.md` D7 at AGC :172 (a strip path,
  outside 1c's tree-root rule); "its D9" at `brainstorm-v2-revision/record.md` :23, :54.

## 2026-10-08 — wave 1c plan FAIL (items 1, 5); P1's one re-plan round opened

- `PLAN GRADE: p1-crate · FAIL · 1: the whole-tree comparison cannot detect the change (today every
  session definition is bare, so `--check` shows no difference; three sampled renames miss the rest)
  — fix: rename previews for every session-D number in every record with both binaries, hunk sets
  diffed, groups of 2–3 inheriting the path allowed · 5: the "main tree untouched" check reads the
  whole `git status` while declared writers move it, with no stop named — fix: scope it to
  `crates/` plus the hashes and the binary's mtime, name the stop, close by reporting the two files'
  shas for the code review`. Items 2, 3, 4, 6 pass; every code claim checked first-hand holds.
- Sent to P1 verbatim with the advisories (four layers at task 0; a positive control in T7) and a
  lead note: keep `CARGO_TARGET_DIR` inside the copy's tree. The same grader re-grades.
- r1 (265 lines, frozen `p1c-plan.r1.frozen.md`, `158078329935a7c3`): `PLAN GRADE: p1-crate · PASS ·
  items 1-6 hold`. The sweep previews all 599 (record, number) pairs (67 of 74 records) with both
  binaries; S7 is scoped to `crates/`. One advisory taken as a binding build note: the token-only
  rule compares against the token as written (`<number>` or `<number>-<old-slug>`), else the 28
  drift mentions the base binary already rewrites stop task 5; inheritance crosses one soft break.
  The grader re-checked 599, 0 joined headings and 4.08 s a preview first-hand.
- P1 approved and resumed to build in `scratchpad/w1c-tree/`. Plans line for 1c:
  `p1-crate:PASS(1)`.
- Wave 3 status: W3-A's map rewritten to 243 × 7 (lead re-counted: 147 `-` notes); hashes
  `slug-map-a.tsv` `24ee83526f215883`, `hand-a.tsv` `4dc7d52df5ab69cb`. B and C building.

## 2026-10-08 — W3-B built: 195 map rows, 135 hand rows

- W3-B (seat-reported): `inputs/slug-map-b.tsv` 195 rows, `inputs/hand-b.tsv` 135 rows (56
  `qualify`, 79 `quote` — 40 meta examples, W3-A's 7 relayed tokens, W3-C's `record.md:911` note, 17
  D12 anchors, 14 D21 rule-text quotes). Lint clean; previews 195/195 exit 0 on the live tree and on a
  scratch tree with the hand rows applied, 0 refusals, 46 rows differing exactly as tagged. No hand
  rows for other shards. Residuals left bare and flagged for the diff review: `idi:29` D15 (owner
  unknown), `mts:87` D6a–d, `prfd:172`'s list of four; `peh:194` read as
  `schema-based-template-guidance` D11-kernel-position-softened (the map grader to confirm).
- Lead first-hand: 196 × 7 and 136 × 6, hashes `f471fadc679942a0` and `ab2d27d8bdbda405`; kinds
  56/79. Hand-row sites in this session's lead files: `build-log.md` :186, :388, :389, :786, :967 and
  37 in `record.md` (:44 to :1201); none in the `wave*.md` files. The lead appends to the build log
  only at its end and holds `record.md` edits until the apply.
- W3-B's alias needs (`feature-map`, `feature-sizing`, `adaptive-depth`, `map-layer`, `angle-1`, with
  two do-not-alias meta lines) relayed verbatim to W3-C.
- W3-B on the relayed rows: all 7 of W3-A's `quote` rows were already covered by its own rows (same
  edit, a longer `before`), so adding them would quote each token twice; none added, files unchanged.
  `pm-role-and-feature-derivation/record.md:86` "charter (D10)" and "(D8)" are the record's own; no
  row. W3-A's reason text is not swapped in (the lead's call: the covering rows carry their own).

## 2026-10-08 — wave 2 closed; commit 2 made

- W2-prose (after a second pull; it disclosed it had held the edit past its validate run): the
  CHANGELOG's unreleased entry names 0046–0050, the log as "sequences 1..50", with 0048's and 0050's
  effects. Validate re-read first-hand by the seat: `0 rejecting · 113 advisory`; `check` allow;
  written byte-identical to its draft.
- Lead first-hand before committing: `crates/`, `plugins/`, `evals/`, the views, the strips and the
  cost-budget ledger equal the frozen tree the code review passed; only the CHANGELOG lines above
  differ. A fresh `views emit` equals `.mochiko/schema-views/`. CHANGELOG `check` allow.
- Wave 2 closed: the gate audit 32 of 32 PASS; W2-tests' crate tests PASS twice; CHANGELOG done.
- Commit 2, `7e0dafb`, not pushed: 128 files — migrations 0046–0050, views, plugin prose, strips,
  the ledger, eval kits, crate tests and goldens, the CHANGELOG, `reports/w2-gate-audit.md`, and the
  session files (record, this log, the wave-2 and wave-3 plans). `inputs/` stays out (wave 3's).
  A clean-archive test run of `7e0dafb` is going in the background.
- Clean-archive run of `7e0dafb` (target inside the extracted tree): test 0 (750 passed, 0
  failed), fmt 0, clippy 0.

## 2026-10-08 — wave 1c built green in the copy; code review sent

- P1 (seat-reported; report `scratchpad/p1c-build-report.md`, `b59e0bf188c29f89`): `src/ids.rs`
  +19 −2, `tests/ids.rs` +219, in `scratchpad/w1c-tree/`; four layers 0 at task 0 and at the close
  (`tests/ids.rs` 175 passed). Reds as planned; T2's plain-text-path case asserts `Owned`, the B1
  line rule's existing behaviour (disclosed deviation). Sweep: 599 pairs per binary, `--check`
  byte-identical, 0 refusals, 0 violations, **320 new rewrites** (272 direct, 48 inherited) in 75
  files; 87 canonical path qualifiers unreached, each accounted for (52 ranges, 27 lists, 4 fenced,
  2 undefined `D6a`, 2 `<slug>`). A checker bug (a doubled `D` prefix, 6,755 false violations) was
  fixed and disclosed before the clean run.
- Stop S7 fired at the close: main `crates/` status went from 18 entries to none — the lead's
  commit 2. Cleared as status-only after the lead's first-hand check: main `crates/` against the copy
  differs only in P1's two files; the three base hashes are unchanged; the copy holds no local
  settings file.
- Code review sent to `w1-code-review` on the frozen copy, four layers first-hand, a sweep sample
  with both binaries; one fix round on a FAIL, then the fallback.

## 2026-10-08 — wave 1c code review PASS; landing ordered. W3-C built; cross-shard relays

- `CODE REVIEW: wave-1c record-path qualifier · PASS · findings: none blocking`. First-hand: test 0
  (757 passed), fmt 0, clippy 0, audit 0; red-first confirmed (base source with the new tests: 169
  pass, 6 fail, each for its stated reason); its own 599-pair sweep matches P1's exactly (320 new,
  272 direct, 48 inherited, 75 files, 0 violations); `--check` byte-identical (621 bare · 59 drift);
  a hunt fixture of 20-odd path variants and a `--write` probe on a copy behave as ruled. Its note
  (both binaries, not 1c): a rename rewrites `inputs/hand-b.tsv` unless excluded — `inputs/` is on
  the Exclude list.
- Landing ordered to P1 (single writer of main `crates/mochiko-cli/` for the step): hash-checked copy
  of the two files, the four layers and `cargo build` in the main tree, `diff -rq` against the copy.
  The lead commits after.
- W3-C (seat-reported): `slug-map-c.tsv` 202 rows, `alias-c.tsv` 98 (12 glued, 86 file-scoped),
  `literal-c.tsv` 52, `hand-c.tsv` 79 (75 `qualify`, 4 `quote`); lint 0; previews all exit 0 (202
  renames, 179 other-owner probes, 49 literal calls); a scratch tree with every shard's hand rows
  applied, 399 runs exit 0. W3-A's 5 relayed rows were already held in `hand-c.tsv`. Alias needs
  all joined except "adaptive-depth D1–D8", a range (bare by rule). Lead first-hand: field counts
  full in all eight `inputs/` files.
- Relayed (file and sha, verbatim): 17 rows to W3-A (`w3c-hand-for-a.tsv`, `d2a6591354bf3f5b`) and 45
  to W3-B (`w3c-hand-for-b.tsv`, `38c21169b01ad7ff`), plus two unjudged pointers each.
- Flags for the map grade and the apply: titles differ by shard (A and C cut confidence marks, B keeps
  them; evidence only, not ruled); W3-C's four `quote` rows edit crate doc comments
  (`src/ids.rs` :374, :504, :568; `rename.rs:1278`), and 1c adds two lines after `ids.rs:463`, so
  :504 and :568 become :506 and :570 at landing (W3-C to refresh); crate comments the renames touch
  take `cargo test` and the rust-cli review at the apply; `DECISIONS.md:78`'s qualify row leaves
  that row's own seven bare `D<n>` unresolved (no wrong owner).
- Pen note: `wave1-crate-ids.md:83` holds a relayed `quote` row; the lead edits that file only within
  existing lines until the apply.

## 2026-10-08 — wave 1c landed and closed; commit 3

- P1 landed 1c (seat-reported, evidence `scratchpad/land-*.txt`): base hashes confirmed, the two
  files copied and hash-verified; main tree test 0 (757 passed), fmt 0, clippy 0, audit 0, build 0;
  `target/debug/mochiko-cli` rebuilt (sha256 prefix `6020cbb8c33d65e1`); `diff -rq` against the
  reviewed copy empty. Lead first-hand: `src/ids.rs` `27ce157f8ccc7f0e`, `tests/ids.rs`
  `ecedd910fb2a9cea`; `git status` shows only those two crate files.
- Wave 1c closed (`wave1-crate-ids.md` status, same line). Plans line for 1c: `p1-crate:PASS(1)`;
  seats: P1 produced / `p1c-plan-grader` graded the plan / `w1-code-review` reviewed the code.
- Commit 3: the two crate files, `wave1-crate-ids.md` and this log; not pushed.

## 2026-10-08 — cross-shard folds; a wrong-owner class found and swept

- W3-C refreshed its two `ids.rs` sites for 1c (:504 to :506, :568 to :570); lead first-hand:
  `hand-c.tsv` `cfb2e72245c129e5`, 80 × 6.
- W3-A folded 16 of W3-C's 17 relayed rows (the 14 `FP` rows, OMS D4 at AGC :165, the field-review
  D2 at delta :248), declined `architect-role-pushback-and-abstraction/record.md:62` (the line itself
  rules that bare D7 there is the record's own; its sibling uses already carry a row), judged both
  pointers with a row each, and added two rows at `constitution-native-surfaces/record.md:263`.
  `hand-a.tsv` 109 rows, `c0e2efde7908a1de`.
- Found by W3-A, ruled by the lead: inside a record, a mention cited through a SHORT path
  (`` `<other-slug>/record.md` decision D8 ``) is not a qualifier (Q2), so the tool gives the bare
  `D<n>` to the record it sits in — a wrong owner, confirmed by real previews. Q2 never meant such a
  mention goes without a row: S3's `qualify` class covers it. W3-A writes rows for
  `constitution-native-surfaces` :55, :84, :261, :267; W3-B and W3-C sweep their own records for the
  same class.
- W3-A wrote six rows there (`hand-a.tsv` 115 rows: 97 `qualify`, 18 `normalize`; lead first-hand
  `c9a104743bde90ae`, 116 × 6). Flag for the map grader and diff review: at :263 and :267 a code span
  around the number is dropped so the qualifier attaches, and inside :267's paste-ready code span the
  qualifier is the plain kebab name — non-token text changes made by hand, outside the tool's diff
  check. W3-A's simulation: the nine tokens resolve to `setup-constitution-flexibility`, nothing else
  on those lines moves.
- W3-C's sweep: 13 candidate lines in its 26 records, 2 with a wrong owner —
  `verbosity-caveman-ops-separation/record.md:88` (D1–D6 of `workflow-token-reduction`, cited by a
  short path) and `:92` (its D3). 7 `qualify` rows added; `hand-c.tsv` 86 rows,
  `7dc0ec040c45210e`; the 1c binary on a scratch tree joins them to the true owner. The stale
  `hand-joins`/`hand-leaves` tags on 12 keys in `slug-map-c.tsv` are being refreshed on the lead's
  word, then C's files freeze.
- W3-C froze: `slug-map-c.tsv` `4de57742270f601f` (12 note cells only), `alias-c.tsv`
  `9ca41e17cf6cf6b7`, `literal-c.tsv` `59ed7308a230e2a7`, `hand-c.tsv` `7dc0ec040c45210e` (lead
  re-hashed all four).
- W3-B folded all 45 of W3-C's rows after its own reading (the 14 "prior" rows checked against
  `hook-enforced-artifact-schema`'s decisions) and added 9 of its own: two "prior" cites W3-C
  missed, five `orchestrator-model-selection` cites the tool would give to that record's own numbers,
  and two `quote` rows masking blind-map labels at `human-readable-ids/record.md:321`, :326 (the lead
  confirmed `quote` as the kind: D10 keeps labels bare, D15's mask is the only way). `hand-b.tsv` 189
  rows (98 `qualify`, 91 `quote`), `342de46f4a73ed0e`; `slug-map-b.tsv` note cells on 22 rows,
  `185dd96dcb25741b`. Lead re-counted both.
- W3-B's residual risk, for the map grade and the diff review: implicit foreign cites (no qualifier,
  no other session named on the line) were read token by token only in its `orchestrator-model-selection`
  record and for prior-style cites across its 22 records; own-number joins in records that follow an
  earlier session want a reading. Its short-path sweep result was not in the reply; pulled.
- W3-B's sweep (pulled): 8 candidate lines, no short-path row owed; it found and fixed a defect in its
  own rows — its `primitive-eval-harness-v2` qualifiers at `primitive-eval-harness/record.md:3`
  carried through the `·` separator to the record's own D2 and D5 (a real preview showed the wrong
  join); two `qualify` rows stop the carry. `hand-b.tsv` 191 rows (100 `qualify`, 91 `quote`).

## 2026-10-08 — the map frozen; three map graders dispatched

- Frozen read-only in `scratchpad/map-grade/inputs-r0/` (hashes in `inputs-r0.sha256`):
  `slug-map-a` `24ee83526f215883` (242), `slug-map-b` `b984d7a51225ce8e` (195), `slug-map-c`
  `4de57742270f601f` (202); `hand-a` `c9a104743bde90ae` (115), `hand-b` `7505e4a9e0846365` (191),
  `hand-c` `7dc0ec040c45210e` (86); `alias-c` `9ca41e17cf6cf6b7` (98); `literal-c`
  `59ed7308a230e2a7` (52). Field counts full in all eight. The producers hold.
- Lead's deviation, disclosed: the wave plan names one map grader; three are dispatched, one per
  shard, for throughput (639 map rows, 392 hand rows, 150 alias and literal rows). Each is a fresh
  `general-purpose` seat on `model: opus` (the producers' tier), wrote nothing it grades, and reads a
  shared brief (`scratchpad/map-grade/brief.md`) that points at the bar — wave plan items and
  S1–S11, the record's D9–D22 cards, the tool's requirements — without restating it. Modes per the
  wave plan: mirror checklist on shape, grammar, uniqueness, keying and hand-row mechanics;
  adversarial on meaning, owners, missed rows and qualifier carries. `map-grader-c` also runs the
  cross-shard checks. Findings go back to the owning shard for one fix round; the same grader
  re-grades; a second FAIL goes to the user.

## 2026-10-08 — map grade C: FAIL (34 blocking, all missed rows); cross-shard PASS

- `MAP GRADE: C · FAIL · 44 findings (34 blocking)` · `CROSS-SHARD: PASS` (639 keys, no duplicate;
  71 owners each in one shard; 392 hand rows all in owning-shard files, each `before` once, no
  overlaps; every alias owner has map rows except the two dead `adopt-first` rows). Findings:
  `scratchpad/map-grade/findings-C.md` (`d6afe09b4b2d1da8`).
- `slug-map-c` mirror PASS and slugs on meaning; the 86 existing hand rows' owners right. Every
  blocking finding is one class: 34 sites, 38 tokens in shard-C files where a foreign bare `D<n>`
  joins the wrong owner with no hand row — the missed-row class the brief named. The grader left
  verified candidate rows (`map-grader-c/fix-rows.tsv`); W3-C judges them as evidence, not as its own.
- Lead's rulings on the grader's choices: `DECISIONS.md:85` takes `:78`'s trade (the line then names
  two sessions; its own `D<n>` stay bare, no wrong owner); `:78` col 601 qualified explicitly;
  `setup-operating-docs-scaffolding/record.md:103` quotes the labels D8 and D9 with "D6"; `literal-c`
  adds `a3r-prose.md` to AM-2 and `p2-plan.md` to J-1/J-6, OQ-4's title made verbatim; the prose-loss,
  "D1 to D8" and residual-bare advisories accepted as is.
- W3-C's one fix round opened. Relay pending for W3-A: `p2-plan.md:574`, `:576` J-7 (mixed series)
  owe two hand rows.

## 2026-10-08 — map grade B: FAIL (11 blocking); a new class: anchor cites

- `MAP GRADE: B · FAIL · 23 findings (11 blocking)`. The harness refused the grader's write of its
  findings file; the lead transcribed its returned list verbatim to `scratchpad/map-grade/findings-B.md`.
  Mirror clean; all 100 qualify owners and 91 quote rows hold. Blocking: 9 missed `qualify` rows
  (foreign `D<n>` joined to the record's own) and 3 unmasked verbatims in
  `orchestrator-model-selection/reports/p1-report.md` (an anchor split over :31–32; :75's old rule text).
- Lead's rulings on B's advisories: hand rows for `ponytail-concepts-integration/record.md:166` (so the
  result does not hang on apply order), the D4.1/D4.2 cites, the two GI ranges written in words (quoted
  so they stay bare, D5-compound-reference-forms) and `human-readable-ids/record.md:551`; the two ambiguous "D7" cites are W3-B's
  judgment, recorded either way.
- New class, from B's grader: an anchor cited in prose (`YYYY-MM-DD <session> D<n>`, bare by D12) is
  joined by a rename. Lead's narrow grep (single-line code span, outside the excluded layers): 50 such
  cites, 6 already masked, 44 not — 37 in shard-A files (`author-grader-consolidation` 27,
  `cli-schema-delivery` 8, `brainstorm-target-state` 2), 7 in shard-C files (`setup-product-agnostic`
  3, `evals/plan` 2, strips 2); list `scratchpad/anchor-uncovered.tsv`. Each shard sweeps its own files
  wider (split lines, no code span): B within its fix round, C as an add-on to its round, A with its
  grade's round.

## 2026-10-08 — map grade A: FAIL (37 blocking, all missed rows); anchors ruled for all shards

- `MAP GRADE: A · FAIL · 48 findings (37 blocking)`; findings `scratchpad/map-grade/findings-A.md`
  (`1a39a7e7f44f65fb`). Mirror clean; the grader replicated the definition-site read and ran all 639
  previews on a tree with every shard's hand rows (9,505 hunks). All 37 blocking findings are missed
  `qualify` rows, 17 of them in `command-schema-ontology` and 8 in `cli-schema-delivery`. The flagged
  points held. Candidate rows left as evidence (`map-grader-a/fix-a.tsv`, 40).
- Lead's rulings on A's advisories: add the six optional rows; re-coin `architecture-design-primitive`
  D5-plan-run-artifact (`always-` is a strength word) and `brainstorm-command-rewrite` D8-strip-ledger-check (`-confirmed` is a verdict
  word); prefer an in-span plain-kebab qualifier at `constitution-native-surfaces` :263 and :267, so
  the code span stays.
- Anchor cites ruled for all shards: a migration anchor quoted in prose is a verbatim machine string,
  bare by D12, and takes a `quote` row. The grader counts 101 tree-wide (49 in A's files), wider than
  the lead's grep of 50; each shard sweeps wider.
- W3-A's one fix round opened, carrying the `p2-plan.md` J-7 relay. All three shards are now in their
  fix rounds. The pattern across all three grades: 82 blocking missed rows, every one a foreign bare
  `D<n>` the rename would give to the record it sits in — the producers' line test missed implicit
  cites; the graders' preview reading caught them.
- Anchor list from shard A's grader, built from previews: `scratchpad/map-grade/map-grader-a/anchor-sites.txt`
  (`be0860ce8757b920`) — 101 same-line sites plus 7 split across a soft break. By ownership: A 49 + 4,
  B 0 + 1 (already blocking at `p1-report.md:32`), C 52 + 2. 21 of C's sit in comments of five crate
  test files and `evals/contract/run.py` (none in a string literal), so the apply's crate-comment
  edits grow: `cargo test` and the rust-cli review at the apply, as item H10 already books. Relayed
  to each shard as its fix round's anchor scope.

## 2026-10-08 — W3-B's fix round done; re-grade sent

- W3-B (seat-reported): all 11 blocking findings fixed with 12 rows; two more `production-only-focus`
  D4-waivers-reach-everything rows of its own (`ops-observability-hardening/record.md:1251`, `:1271`); the advisory rows as
  ruled; the two ambiguous D7 cites read as the record's own (no row). Anchor sweep: 21 candidates, 16
  already masked, 4 inside quotes, 1 (the split anchor) covered by a blocking-fix row. Carries
  checked; 195 previews on a tree with all its rows applied, 0 errors. Disclosed: it wrote with Edit,
  which trimmed the separator tab on 7 new lines; caught by `cmp`, fixed, re-checked.
- Lead first-hand: `hand-b.tsv` `bb045fe9420d37c8`, 212 × 6, +20 rows on r0, none changed;
  `slug-map-b.tsv` `6b0157a6c0591026`, 196 × 7. Frozen in `scratchpad/map-grade/inputs-r1-b/`.
  Re-grade sent to `map-grader-b`.
- W3-C's fix round (seat-reported): all 34 blocking findings fixed with 41 rows, each owner judged
  (it agreed with all 38 of the grader's candidates); the ruled `literal-c` edits; 30 note cells
  recomputed. `hand-c.tsv` 127 rows `3073ece60ad6da6d`, `slug-map-c.tsv` `8e3be7c3e6faca88`,
  `literal-c.tsv` `37e33c8e56a3ff41`. Disclosed: the three ruled literal path additions do nothing —
  the cited lines sit inside code fences the tool masks (`a3r-prose.md` fence from :22, `p2-plan.md`
  from :270; the lead confirmed both first-hand). For the same reason the `p2-plan.md` J-7 relay to
  W3-A is voided by a supersession naming it.
- W3-C's reply did not cover the anchor add-on (its 54 sites); pulled.
- `MAP GRADE: B · PASS · 1 finding (0 blocking)` on the r1 freeze. The grader re-ran all 195 B-key
  previews and 11 foreign-owner previews on a tree with every shard's hand rows: every difference sits
  at a fixed site; the ER-D alias case no longer hangs on order; its own anchor hunt in B's files
  found 21, all masked. Advisory `ops-observability-hardening/record.md:1261` ("D4/PO-D4", no wrong
  join) accepted as written by the lead — no reopening of a passed file for a cosmetic row.
  Shard B is done: `slug-map-b` `6b0157a6c0591026`, `hand-b` `bb045fe9420d37c8`.
- W3-A's fix round crossed the lead's supersession in transit: it added the two voided `p2-plan.md` J-7
  rows; the supersession tells it to remove them. A's freeze waits for that.
- W3-A's fix round (seat-reported): the 37 blocking findings fixed with 40 rows (owners judged; it
  agreed with all the grader's candidates); six optional rows plus one of the same class; the two
  slugs re-coined (`plan-run-artifact`, `strip-ledger-check`); :263 in-span, :267's "§4 `D8`" quoted;
  57 `quote` rows over its 53 anchor sites (its own sweep matched the grader's list exactly); the J-7
  rows removed on the supersession. Simulation with all shards' rows: 242 previews exit 0, hunks equal
  its predictions in its files.
- Lead first-hand: `hand-a.tsv` `be0e72ebced9cdbf`, 220 × 6, no `p2-plan.md` row, 104 added and 3
  changed on r0; `slug-map-a.tsv` `e3ea2a9a9dea6903`, three rows changed as reported. Frozen in
  `scratchpad/map-grade/inputs-r1-a/`; re-grade sent to `map-grader-a`.
- `MAP GRADE: A · PASS · 0 findings (0 blocking)` (record `scratchpad/map-grade/findings-A-r1.md`,
  `b4eb45c221a3dc39`). All 639 previews on a tree with hand-a r1, hand-b r1 and hand-c r0; every one of
  the 79 changes in A files explained; the 37 fixes byte-identical to the rows it verified; anchors
  masked with no join moved. Shard A is done.
- Its two notes: `team-lead-strategic-compaction/record.md:252` is a false positive in its anchor list
  (a date then a real cite, already a `qualify` row) — relayed to W3-C, whose anchor share becomes
  51 + 2; and A's anchors cited in 9 of C's files, plus A's alias-dependent joins, wait on C's r1 (the
  final cross-check runs on all three r1 files).
- W3-C's anchor add-on (seat-reported): 71 `quote` rows — 53 same-line and 9 soft-break pairs —
  covering its 53 listed sites plus 8 its wider sweep found (soft breaks in crate-test comments, an
  `observable.yaml` comment); TLSC:252 left unquoted as corrected. Its verification: anchor-touch
  previews 0 of 94, 399 rename previews 0 nonzero, 57 hunk sites lost all on anchor lines.
- Lead first-hand: `hand-c.tsv` `a9ea747c8c1d5f72`, 199 × 6, +112 on r0 (41 fix, 71 anchor), none
  changed; `slug-map-c` `6272ebc0913257d9`; `literal-c` `37e33c8e56a3ff41`; `alias-c` unchanged.
  Frozen in `scratchpad/map-grade/inputs-r1-c/`; re-grade sent to `map-grader-c`, with the
  cross-shard checks re-run on all three r1 files.

## 2026-10-08 — map re-grade C: FAIL on one overlap; put to the user

- `MAP GRADE: C · FAIL · 1 finding (1 blocking)` · `CROSS-SHARD: FAIL` on the same single overlap,
  otherwise PASS (639 keys; 628 hand rows, every site in its owning shard's file, every `before`
  once; alias owners all mapped but the two dead). Findings `scratchpad/map-grade/findings-C-r1.md`
  (`618e330f24431d06`). The blocker: hand-c rows 95 and 96 at `teammate-message-races/record.md:26`
  overlap on "shape" — from the grader's own round-1 candidates, copied verbatim; fix: row 96's
  `before` shortened to `; D1 `Contested``, result byte-identical. Everything else holds: 202 C
  previews exit 0; all 74 new quote rows mask anchors or labels only; 121 alias previews exit 0.
- Shard C's second FAIL: put to the user (A: one more one-row fix round; B: stop C and skip that
  line's rows). Ruled, user: "as recommded" — A. W3-C changes row 96 only; the same grader re-checks
  that row.

## 2026-10-09 — C's row 96 fixed; re-check sent to map-grader-c

- W3-C: "Row 96 fixed. hand-c.tsv sha256 c046811c185cd831 (was a9ea747c8c1d5f72). Only row 96
  changed." Read first-hand: 199 lines of 6 fields; the diff against the r1 copy is row 96 alone
  (`before` now `; D1 `Contested``, `after` `; `command-architecture-realignment` D1-choreography-leaves-commands `Contested``);
  row 95's `+ D6 v8 shape` is a disjoint span on :26. Frozen as `scratchpad/map-grade/inputs-r2-c/`.
- Order to map-grader-c: re-check row 96 only (once on :26, disjoint from every A/B/C row there,
  the `qualify` kind and owner, the diff touches row 96 only); its r1 verdict on every other row
  stands. Map-grader-a's bounded re-check of A's dependencies on C r1 is still out.
- Apply brief updated: `hand-c` hash `c046811c185cd831`, HEAD `0b8982a`, and a deviation noted —
  no merged `inputs/slug-map.tsv`; the apply seat reads the three shard maps directly, a then b then
  c.
- `RE-CHECK C: PASS · 0 findings` (map-grader-c). Hash `c046811c185cd831` matches; the diff
  against r1 is row 96 alone; `before` occurs once on :26; spans on :26 are rows 94 [165,186),
  95 [187,200), 96 [200,216) — disjoint, no A/B rows there; the result is byte-identical to r1's
  post-tree line; the owner is `command-architecture-realignment` D1-choreography-leaves-commands (its record :72). Shard C's
  map closes. Waiting on map-grader-a's re-check of A against C r1.

## 2026-10-09 — map closed on all three shards; apply dispatched

- `RE-CHECK A: PASS` (map-grader-a), on hand-a r1 + hand-b r1 + hand-c r1: all 242 A-key previews
  with alias rows exit 0; no A-key rename touches a quoted anchor (13 vanished hunks, all inside C's
  new quote rows); `alias-c` r1 is byte-identical to r0; 25 A-key hunk changes between C r0 and r1,
  none an alias-form change, each traced to a new C row (11 new joins spot-checked, owners right).
  Its point 3 — row 96 must still put the qualifier in front of D1 — holds in C r2 (`; `command-
  architecture-realignment` D1 `Contested``).
- Fan-in: A PASS r1 + re-check PASS · B PASS r1 (one advisory accepted as is, ops :1261) · C PASS
  at r2 (row 96). The map is closed: inputs as listed in `scratchpad/apply/brief.md`.
- Apply brief finalized (sha256 `3dd7d636a191d5a4`): the binary is copied to `scratchpad/apply/bin/`
  and pinned by hash (`6020cbb8c33d65e1`), since step 4's cargo runs rebuild `target/debug/`; each
  rename previews, then writes, and checks S3 before the write — every `qualify`/`normalize` hand row
  owned by that key must show its join on its site line, or the seat stops; after the apply, every
  `quote` row's `after` text must stand verbatim.
- Apply seat `w3-apply`: persona-less `general-purpose`, `model: opus` (mechanical execution of a
  graded ruling — no plan leg, wave plan "Seats"). It is the single writer of the whole working tree
  until it returns; the lead holds this log and every other file.

## 2026-10-09 — D18-build-done-check's read-test clause ruled; the apply moved to a release binary

- The clause in D18-build-done-check that D8-read-test-bet's read test "has passed" was put to the
  user with the case against first. User: "as recommended" — lean A: the clause is met by the
  user's override, and the test stays recorded as failed. Rejected B (a fresh cold read test on the
  final slugs, its pass rule set after the first result was seen) and C (strike the clause). Written
  to the record's D18-build-done-check card, changed at build, after the apply returned the pen.
- Apply progress, step 1: 628 hand rows over 118 files; every dry run `allow`; line counts
  unchanged. Step 2 measured ~20 s per row (~10–11 s per `ids` call on the debug binary): ~3.5 h.
- Supersession sent at row ~12, voiding the brief's binary pin (`scratchpad/apply/bin/`, sha256
  `6020cbb8c33d65e1`) for every row not yet started: a release build of `git archive 0b8982a`
  (`Cargo.toml`, `Cargo.lock`, `crates/mochiko-cli`) in `scratchpad/relbuild/`, sha256
  `b82706463b0344b2`, copied to `scratchpad/apply/bin-release/`. Equivalence on map-grader-c's
  `post-r1` tree: 9 rename previews (3 per shard map) and `ids --check` (639 bare · 12 drift)
  byte-identical to the debug build, exit codes included; 0.45 s vs 4 s per call
  (`scratchpad/relprobe/`). The time was the tool, not the tree.
- Seat: first release row 026 (`slug-map-a.tsv:27`), hash verified, switch logged; rows 001–025
  stay as written on debug. Disclosed: its first stop ended the wrapper shell, not the runner, so
  019–025 ran on debug as normal; one read-only debug preview of row 026 was orphaned by the real
  stop — unlogged, uncaptured, no tree effect; row 026 then re-ran in full on release, logged.

## 2026-10-09 — the apply stopped at row 091 on S3; three hand rows ruled to expect no hunk

- Stop at row 091 (`slug-map-a.tsv:92`, `cli-schema-delivery` D3-rules-delivery-binding): hand row
  `hand-b.tsv:40` (qualify; site `hook-enforcement-field-review/record.md:34`) showed no hunk on
  its site line. Nothing was written for the row.
- Lead forecast before ruling: every remaining rename previewed on a scratch copy of the stopped
  tree (`scratchpad/s3sim/`, ~1 min, reusing the seat's own call builder and check). Three S3
  failures in all: `hand-b.tsv:40`; `hand-b.tsv:46` (same record :165, "D2/R2b", which the tool
  reads as a path); `hand-a.tsv:200` (`delta-files-vs-direct-baseline-edits/record.md:555`,
  "D2/OQ1", also a path — its own reason already says no rename touches it).
- Probe (`scratchpad/s3sim/wrongjoin.sh`): no plausible owner's rename puts a hunk on any of the
  three lines. Each mention stays bare behind its true owner's qualifier; no wrong owner. Line :34
  is odd — a qualified slash pair joins on the same record's :163 — so a line-start code-span
  qualifier may not be read. Booked for the backlog (crate) and the diff review.
- Lead ruling (mechanical, within S3 and D18-build-done-check as narrowed): those three rows expect
  no hunk, a hunk there is a stop, and every other row keeps the check. Supersession sent naming the
  voided line; the seat logged it before row 091. The three sites go to the diff review.

## 2026-10-09 — the apply returned: the done check is clean; reviews dispatched

- Stall, disclosed: the seat's runner finished step 2 at 08:06 and step 4 at 08:23 local, and
  both times the seat was not woken to go on. The lead read the logs first-hand and sent an open
  order each time (go on with step 3; write the return), each pinned to the brief and voiding
  nothing.
- Return (`w3-apply`), checked against the logs first-hand: step 1, 628 hand rows (normalize 18 ·
  qualify 377 · quote 233) over 118 files; step 2, 639 renames (1,278 calls, preview then write),
  every call exit 0, every write "ID tokens only", 0 moves; step 3, 48 literals (4 rows with path
  `-` skipped), all exit 0; refusals 0. The three ruled rows showed no hunk (rows 091, 259, 270);
  no other S3 failure.
- Step 4, read first-hand in `scratchpad/apply/out/step4/`: `mochiko-cli ids --check · 0 bare ·
  0 drift` (exit 0, full Exclude list, release binary) · `cargo test --all` exit 0 (757 passed, 0
  failed) · `cargo fmt --all --check` 0 · `cargo clippy --all-targets -- -D warnings` 0 · `check`
  over 403 changed files, none not `allow` · 233 quote rows, none no longer verbatim. `git diff
  --stat 0b8982a`: 403 files, +7477 −7249 (the +228 gap is this log's pre-apply change). Inputs
  re-hashed, all 8 unchanged. Apply log sha256 `1d5903ef1969adbb`.
- Departures (seat): the Explore/haiku locate route returned nothing twice, so the seat grepped
  the crate source itself; step 1 ran every dry run before any write.
- Flagged by the seat for the diff review: (1) four added lines carry a doubled qualifier, an alias
  row having hit an already-qualified mention (the lead's grep agrees: 4 lines); (2) 53 hand rows
  whose owner no map row carries — 51 FP/J literal-series rows, plus `hand-a.tsv:77` and
  `hand-b.tsv:47` — had only step 1's mechanical checks, no join check; 52 of the 53 `after` texts
  still stand, and on `hand-b.tsv:47` a later rename joined the D9 inside it.
- Lead writes after the return: the D18-build-done-check card's changed-at-build paragraph and this
  log's last three entries, this one included. The lead now holds the whole tree; it is frozen for the reviews.
- Dispatched in parallel on the frozen tree: the diff review (persona-less, `model: opus`, brief
  `scratchpad/post-apply/diff-review-brief.md`, the wave plan's item 8 classes plus residuals and
  the two flags) and the crate review (`mochiko:tech-lead`, its default tier, brief
  `scratchpad/post-apply/rust-review-brief.md`). Both are read-only. Strips, the `CHANGELOG.md`
  line and the gate audit wait on their verdicts.

## 2026-10-09 — both reviews FAIL; one fix round plus the strip entries; the done check holds

- `CRATE REVIEW: FAIL · 4 findings (2 blocking)` (findings sha256 `e8416d8c43d88b15`). Four layers
  green (757 passed; `cargo audit` run at the workspace root — the brief's `crates/` was wrong, the
  lockfile is at the root). Blocking: `cli.rs:157`, the `ids rename` help text, gave a joined ID as
  the "bare" example; `.github/workflows/ci.yml:80`, a joined ID inside the secret scan's `echo`.
- `DIFF REVIEW: FAIL · 96 findings (56 blocking)` (findings sha256 `0640783fc593d351`). Swept 403
  files, 11,335 joins. D owners: 6,687 joins, 0 mismatches mechanically, ~1,260 suspects read by
  hand, 2 wrong (meta-text in this log, :1322 and :1337). The big class: 49 GI mentions that are
  not mochiko's register entries took mochiko's slugs — kinako's GIs, mochiko-app's "GI-002", eval
  fixtures' GIs, template slots. A GI rename is project-wide, and no map grader caught a foreign GI.
  Plus 4 meta-text examples, 4 joins inside a four-backtick fence (a tool defect:
  `conform.rs` `fenced_lines` ignores fence length), 7 doubled qualifiers (the lead's grep found 4).
- Lead dispositions (`scratchpad/post-apply/fix-brief.md`, sha256 `c39460c1688fc76a`): every
  blocking fix as the reviewer wrote it (foreign and example GIs quote-masked, `"GI-004"`, the W3-C
  form; fenced and nested-quote text restored bare); 20 advisory fixes (the doubled qualifiers, one
  quote closing mid-token, two meta-text masks in this log, 12 missed shorthand joins with each slug
  checked against the graded map). Not applied, with reasons: `ci.yml:80` ruled in scope (no code
  reads the message; a revert leaves a bare ID); the 100-column comment width (a reflow is churn no
  gate checks); lists of four PO IDs expanded by alias rows (each owner right); 33 joins inside six
  records' "pasted verbatim" fact-checker sections (inside D15-protected-line-rewrites's letter —
  only `"…"`, `>` and fences are masked); the booked residuals the reviewer passed.
- Exclude list amended (wave plan, done check only): `.mochiko/strips/governance-surfaces-template.md`
  and `.mochiko/strips/validation-constitution.md` — 5 restored verbatim lines the tool misreads.
  Without them: `5 bare · 0 drift`. The crate fix is booked; it matters before wave 4's release.
- Fix round (`w3-apply`, foreground only), checked first-hand: 77 edits over 30 files, 0 skipped,
  every dry run `allow`, line counts unchanged; nine `[v0.118.0]` strip entries for the eight plugin
  units (three new strip files: `validation-primitive-edit`, `artifact-gate`, `dependency-halt`),
  superseded lines placed by script inside a fenced `text` block. `mochiko-cli ids --check · 0 bare ·
  0 drift` with the amended list; cargo test/fmt/clippy/audit all 0; 37 changed files all `allow`.
  Fix log sha256 `aa8bda68586e8aaa`.
- Next, in parallel on the frozen tree: each reviewer verifies its own findings' fixes once (changed
  lines only), and the gate grader runs the eight-unit audit and the done check.

## 2026-10-09 — fix verifies PASS; gate 8 PASS at round 2; the done check is clean

- `FIX VERIFY: PASS · 0 problems` (diff review): all 77 edits exact on their lines; the five
  restores byte-equal to `0b8982a`; no stray edit; every advisory not applied untouched.
- `CRATE VERIFY: PASS · 0 problems`: the two help-text lines as written; `ids rename --help` shows
  the bare quoted example; four layers green (757 passed). Its note that 26 `evals/` markdown files
  sat outside the crate brief is covered by the diff review, which swept every changed file. My
  crate brief had said "comments only" where its own bar was "no string literal a test or the
  binary reads" — the source of the `ci.yml:80` finding, ruled in scope.
- Gate (`w3-gate`, persona-less, `model: opus`, report `reports/w3-gate-audit.md`): round 1, 7 PASS ·
  1 FAIL. U2, the mochiko router: "(realignment D5)" at `SKILL.md:71` was a missed one-word
  shorthand, and its strip entry wrongly called it kept by ruling. Rulings of the grader's own,
  disclosed: `validation-primitive-edit`'s description grows 730 to 754 against a 730 budget and
  `patterns-model-tiering`'s standing overage moves to +7,517 — both HOLD under the record's D2;
  the budget rows update at landing.
- U2 fix (`w3-apply`): a hand qualify, then `ids rename` of `command-architecture-realignment`
  D5-transport-neutral-harness. Its first preview also hit four lines of the new gate report (meta
  text about this very command), so the seat stopped. Lead ruling: exclude the report from that one
  call, the report being the grader's own; the write then touched `SKILL.md:71` only ("1 files · 1
  spans · ID tokens only"). The strip entry was rewritten to claim :71; the lead edited its heading.
- The gate report itself broke the done check (8 bare: example and cited tokens in its own prose).
  At its re-audit the grader quote-masked its example tokens and joined its cites.
- Round 2: U2 PASS; `8 PASS · 0 blocking`. Done check by the grader, report included: `mochiko-cli
  ids --check · 0 bare · 0 drift`, exit 0, the amended Exclude list, the release binary. Lead re-run:
  the same; `migrate validate` 0 rejecting · 113 advisory. Report sha256 `3839acbfc5aab087`; fix log
  `aed2c2bb87a9f5ac`.
- Wave 3's work is done. Commit 4 carries the back-fill, `inputs/`, the strips and the reports.
  Landing items held for wave 4: the budget rows, one U1 wording advisory (accepted, not fixed), and
  the backlog items from this wave (the `fenced_lines` fence-length defect before the
  `mochiko-cli` release, nested quotes, foreign GIs under a project-wide rename, the Explore route,
  the teammate wake-up).

## 2026-10-09 — fence-length fix in the crate before wave 4's release (wave 3b), ruled

- Put to the user after commit `4b2291e`: fix the `fenced_lines` defect in the crate before wave 4
  publishes `mochiko-cli` with `ids`, or ship it and book it. Lean: fix first — the defect rewrites
  text inside a four-backtick fence in any consumer's repo. User: "as recommended".
- Scope: `crates/mochiko-cli/src/conform.rs` `fenced_lines` only — a closing fence must match the
  opener's character and be at least its length, with nothing after it but whitespace; a backtick
  opener's info string holds no backtick. The one classifier serves both callers (`check`'s heading
  scan and the `ids` mask), so the conformance hook's reading of fences changes too. Nested quotes
  stay booked, not in scope.
- Crate unit ceremony (`.claude/rules/mochiko/rust-cli.md`): a `mochiko:staff-engineer` seat plans
  first, read-only; a fresh staff-engineer peer grades the plan (`mochiko:review-seat-plan`); the
  seat builds red-green; the non-author crate review (`mochiko:tech-lead`) and the four layers
  follow. After it lands: the governance-surfaces-template strip leaves the Exclude list and the
  done check re-runs; the validation-constitution strip stays excluded (nested quotes).

## 2026-10-09 — wave 3b built and reviewed; the template strip leaves the Exclude list

- Seat `w3b-fence` (`mochiko:staff-engineer`, default tier): plan-only first, `scratchpad/fence/plan.md`
  (sha256 `5e97dea249612159`), no repo write. Plan grade (`w3b-plan-grade`, a fresh staff-engineer
  peer, `mochiko:review-seat-plan`): PASS · 0 blocking · 5 advisory. The grader ran every red leg
  at HEAD itself. Its A1 caught that the `check` sweep could never flip allow/deny: each file is
  graded against itself, and standing faults pass. The sweep now compares fault sets, plus a
  fresh-write mode. The lead approved and folded A1, A2, A4 and A5.
- Build: `fenced_lines` keeps the opener's character and run length; a closer must match both, with
  only whitespace after; a backtick opener whose info string holds a backtick does not open; an
  unclosed block runs to the end. A private `fence_run` helper; no interface change. 8 new tests,
  each red at HEAD with its predicted value (7 planned, plus the unclosed-block test from review A1).
- Sweeps: `ids --check` on HEAD 4 bare (the template strip's fenced lines), on the fix 0. `check`
  over 494 tracked `.mochiko/` files: 0 differences against the on-disk baseline and 0 as fresh
  writes. Disclosed: an out-of-repo `CARGO_TARGET_DIR` turns one existing `matrix_similar` test red,
  since it assumes the target dir sits in the repo. The test layer ran on the default dir; booked.
- Crate review (`w3-crate-review`, `mochiko:tech-lead`, non-author): `CRATE REVIEW 3b: PASS · 2
  findings (0 blocking)` (findings sha256 `e3e593ef8ef7397a`). 17 edge cases probed (tabs, CRLF,
  tildes in backtick blocks, longer closers, unclosed blocks), all green. A1 folded: the unclosed-
  block test. A2: the `matrix_similar` assumption, booked.
- Four layers: `cargo test --all` 765 passed, 0 failed; fmt, clippy and audit 0.
- Lead, first-hand on a release build of the current tree (`scratchpad/fence/target-lead`, sha256
  `3601ba4976b7b411`): `mochiko-cli ids --check · 0 bare · 0 drift` with the Exclude list now
  minus the template strip. Without the validation-constitution strip: `1 bare` (its nested-quote
  line, :68). `migrate validate` 0 rejecting · 113 advisory. The wave plan's Exclude list is updated.

## 2026-10-09 — the plugin contract suite is green on `4d8bdb3` (gate 6 evidence for wave 4)

- First run, after the user's `sbx login`: 7/7 host cases passed, sandbox cases SKIPPED (exit 3) —
  not green. The suite's probe said "run `sbx login`", but the real error inside the sandbox was
  "Failed to authenticate: OAuth session expired and could not be refreshed": the sandbox's
  stored Anthropic credential, a separate sign-in from Docker's. The user refreshed it.
- Second run, the full suite (`python3 evals/contract/run.py`), no filter: `contract suite: 97/97
  cases passed, 97 ran, 327 measurement(s) recorded and not asserted`, exit 0. The tree is HEAD
  `4d8bdb3`, clean; the plugin is the working tree's `plugins/mochiko/` (0.117.0 in `plugin.json`,
  unbumped). `target/release/mochiko-cli` was rebuilt from the same source first (sha256
  `3601ba4976b7b411`, the wave-3b release build).
- This is the deterministic-set evidence GI-012-release-gates-module gate 6 asks for at the bump.
  It holds while `plugins/mochiko/` and the crate stay as committed here; any wave-4 change to
  either re-runs the suite.
