# Wave 1 — extension unit 1b (family-2 checks) independent code audit

**PASS**

Unit frozen at `e66d76e`; `git diff cb9939d HEAD -- crates/mochiko-cli` is exactly the unit. The
reviewer authored none of it and defaults to FAIL. No blocking findings survive. Nine advisory.

Blocking: 0 · Advisory: 9.

The unit does what it claims. The three things I most expected to break did not: Python parity on
the citation scanner is exact including its backtracking behaviour, pointer resolution is a
line-for-line port of `check_pointer`, and the thirteen new advisories reproduce one-for-one when
the Python checkers are run against the same tree. The probe ledger is no longer a transcription
that can drift — `accounted_for` is a bidirectional partition test, and it passes.

---

## What I verified independently, not from the report

**Python parity, read against the scripts.**

- *Citation scanner.* `citations` + `dotted_tail` (`src/validate.rs:514`, `:560`) accept the same
  language as `cite_re` (`scripts/check-command-schema.py:253`),
  `\b({alt})((?:\.[a-z0-9]+(?:-[a-z0-9]+)*)+)\b`, including the part that is easy to get wrong.
  Python's trailing `\b` makes the engine backtrack: in `demo.a.b_c` the longest tail ends before
  `_`, which is not a boundary, so the reported token is the shorter `demo.a`. `dotted_tail`
  collects candidate stops and walks them in reverse to the first that closes on a non-word byte.
  Same token, by the same rule. The left boundary is `is_word_byte` against ASCII `\w`, matching
  `\b` for the ASCII text the corpus holds.
- *Exclusions.* File suffixes are the last dotted segment against `CITATION_SUFFIXES`, matching
  `rest.rsplit(".", 1)[-1]`. Section tokens are skipped and left to the section limb, matching
  `SEC_ID_RE.match(tok)` in `cite_tokens`.
- *Tombstoned is rejecting.* The Python appends to `findings`, not `warnings`
  (`check-command-schema.py:864`, `:881`); `Code::CiteUnresolved` is rejecting. Correct, and it is
  the severity the check table claims.
- *Foreign is advisory, once per document, sorted.* Python `warnings.append(...)` over a sorted
  set; Rust emits one `Finding::doc` from a `BTreeSet`. Same shape.
- *Pointer resolution.* `validate_pointers` (`src/validate.rs:752`) is a faithful port of
  `check_pointer` (`scripts/check-skill-schema.py:451`): path-shaped means carrying `/` or ending
  `.md`; the base is the skill's own directory; absolute is its own finding; the three failure
  shapes and their order are identical; the counter increments on every path-shaped pointer
  including the absolute one; `mochiko:<skill>` is a name and skipped. Python's `plugin_root` is
  `skill_dir.parent.parent`, which is the same directory the Rust receives as `root`.
- *Residual severities.* Every one is `findings` in the Python, and rejecting in the Rust: the
  retired registry label (`:607`), flat top-level `rules:` (`:620`, skill `:634`), a moment with
  no navigation line (`:291`), the library with no blocks (`:411` / skill `:374`), and either
  registry with no labels mapping (`:605` / skill `:618`). The last three collapse into one
  `document-empty`, which the report states.
- *`names_retired_selector`* is `fail-condition(?!s)` exactly (`:167`), applied to section
  title/intent as a warning (`:667`) and to rule text as a warning (`:834`).

**The corpus delta, reproduced by running the Python.** Both checkers report 0 findings on the
shipped tree. Their warnings match the thirteen new advisories one for one:

- 9 zero-member labels, and the same nine — `attempt-economy` in architecture, feature, setup,
  specify; `binding` in implement; `scope-entry` in brainstorm; `stewardship` in brainstorm,
  implement, specify.
- 4 inherited label absences, and the same four — `letter-is-spirit` in authoring-feature-map,
  authoring-prototype, authoring-technical-requirements, authoring-user-stories.
- The skill sweep reports zero zero-member labels, and so does the Rust: all nine of its
  `zero-member-label` lines are `command/`.
- Summing the Python's per-skill `pointers N` stats gives **87** across 30 skills, the same figure
  the Rust reports as checked.

The CLI's own output is 105 findings, 105 distinct, all advisory, and the histogram closes:
39 condition-coverage · 39 budget · 9 zero-member-label · 8 unused-moment · 6 enforces-coverage ·
4 labels-inherited. 105 − 9 − 4 = 92, the figure before this unit. No duplicate finding, which
matters because `resolve_extends` now runs several times per rule and only the first pass is given
a live findings sink.

**The pointer pass, probed rather than read.** Against an empty scratch root every one of the 87
pointers fails, exit 1, and the findings print **without** `--report` — so the report's claim that
a rejecting pointer finding is never gated behind the flag holds. Against a root carrying
`references/CONTEXT-GATHERING.md` at the top but not under the skill directory, the middle branch
fires with its own message: "resolves only from the plugin root". Against the real tree, 0.

**The CLI contract, both ways.**

```
--plugin-root plugins/mochiko --report   pointer resolution: 87 checked against plugins/mochiko
--report (no root)                       pointer resolution: skipped (no --plugin-root; pointers
                                         are unchecked, not clean)
bare migrate validate                    one line only
```

Nothing passes silently: the skipped line is unconditional under `--report`, and the counted line
carries the number rather than an absence of findings.

**The ledger moves are machine-enforced, not asserted.** `accounted_for`
(`tests/matrix/mod.rs:338`) is bidirectional — every Python probe claimed exactly once across
{ported, genesis-side, not-applicable, outside}, and no claim that is not a Python probe. It
passes. The command file declares `PYTHON_PROBES: [&str; 134]`, and the ledger arrays count 8 /
14 / 18, which forces ported = 94; the skill file's claims total 114 distinct against its asserted
114. The seven stale C3 rows are gone from both `OUTSIDE_THE_HARD_SET` arrays (4 command, 3 skill)
and re-claimed as ported, with no new code — I confirmed the guard they re-claim was already in
`check_class_and_kind`. The probes assert the node, not merely the code: `Expect` carries
`RejectOn`/`AdvisoryOn` with an id, plus `CleanAbsent` for the Python matrix's `absent` field, and
`Expect::Reject` bare is used **zero** times in either matrix.

**Coverage.** `every_rejecting_code_is_raised_by_some_probe` now asserts set equality over all
**46** rejecting codes in both directions, with 25 state-level mutations, 13 real migration logs
and a rooted pointer pass. All nine new rejecting codes and all seven new advisory codes carry
multiple dedicated tests with positive controls beside them (`a_local_text_that_actually_differs_
is_no_pointless_override`, `every_block_bound_by_some_stub_makes_no_orphan_claim`,
`the_live_plural_section_slug_is_never_read_as_the_retired_selector`, and others).

**The `extra` map.** `an_unknown_rule_key_is_preserved_through_the_round_trip` decodes a rule
carrying `ruling:`, asserts the key survives in `extra`, and compares canonical hashes of the
original and the re-emitted value. The round trip was lossy before and is not now.
`migrations/` and `plugins/` are byte-untouched in the unit diff, so the genesis hash cannot have
moved, and `tests/fidelity.rs` stays green.

**Hygiene.** `Cargo.toml` and `Cargo.lock` are untouched by this unit. No `unsafe` in the crate.
No network or subprocess in the new code. No cwd-dependent lookup added — the pointer root arrives
as an argument, and the corpus pointer test resolves through `CARGO_MANIFEST_DIR`; the fixture
tree is written under `CARGO_TARGET_TMPDIR`. No file under `plugins/` changed.
`replay::load` keeps its meaning: `validate_pointers` is a separate rooted pass with no caller
inside `replay`, pinned by `the_state_only_validator_makes_no_pointer_claim`. GI-019 holds — the
new checks are structural facts about the store's own data, and the one filesystem check asks
whether a reference resolves, never what the pointed-to file says.

---

## Advisory

Ranked. None blocks.

**A1 — the report's command probe counts are off by one and two.**
`tests/matrix_command.rs` has exactly three `Probe::extra` call sites (`:633`, `:667`, `:900`),
none inside a loop, so the command extras are **3**, not the 4 in the report's §3 table, and the
file holds **97** probes (94 ported + 3 extra), not the 98 in §9. The skill figures are right: 89
ported + 2 extra = 91. The 95 `p.push` sites resolve to 97 probes because the C3 row at `:783`
pushes three from one site. This does not touch the 81 accounting, which is machine-enforced and
correct — extras carry `python: None` and sit outside the partition. Fix: correct the two cells.

**A2 — the advisory code set has no coverage guard.**
`every_rejecting_code_is_raised_by_some_probe` covers `Code::REJECTING` only. The single
`Code::ADVISORY` assertion (`tests/validate.rs:439`) runs one way — every advisory finding raised
has a declared code — so a fifteenth advisory code added with no probe would fail nothing. This is
the same species of gap the previous round's A2 closed for the rejecting set, and the fix is the
same shape: collect the advisory codes the probe corpus raises and assert set equality.

**A3 — pointer resolution has no containment guard.**
`src/validate.rs:786` uses `Path::exists()`, which follows `../` climbs and symlinks with no check
that the result stays under the plugin root. Probed: a pointer of `../../../../CLAUDE.md` from a
skill directory resolves to the repository's own `CLAUDE.md` and passes clean. This is **faithful
to the Python** (`(skill_dir / p).exists()` with no guard either), so it is inherited parity rather
than a regression, and the Q4 ruling was parity. Naming it because the check reads as "this
pointer resolves" while it means "something exists at that path, wherever that is". Fix when the
lead wants it: canonicalise both and assert the target starts with the canonical root.

**A4 — the shipped-pointer test pins a floor, not the number.**
`every_shipped_pointer_resolves_from_its_own_skill_directory` asserts `report.checked > 50`. The
corpus census elsewhere pins exact figures (50 documents, 321 rules, 226 floors), and 87 is a
figure the report leans on twice. A silent drop to 51 would pass. Fix: `assert_eq!(report.checked,
87)`.

**A5 — the retired-selector lint runs on skill schemas, which the Python never lints.**
`check-skill-schema.py` contains no occurrence of `fail-condition` at all; the lint is command-side
only (`check-command-schema.py:667`, `:834`). `names_retired_selector` is applied in
`check_sections` and `check_text` for every rule-bearing kind. A benign superset — advisory
severity, zero hits on the corpus — but the check table cites the command checker's 2b/5 without
saying the Rust widened the scope. Fix: state it, or scope it to commands.

**A6 — the sigil lint is broader than `SKELETON_SIGIL_RE`.**
`src/validate.rs:2160` tests for `{{` followed anywhere by `}}`; the Python is
`\{\{[^}]*\}\}` (`:137`), which forbids a `}` between. `{{a}b}}` fires in Rust and not in Python.
Advisory severity, zero hits on the corpus. Fix: require no `}` between the braces, or note the
divergence.

**A7 — the record's unit-1b paragraph will be stale at landing in a second place.**
`record.md:1119` calls family 3 "7 per-skill sweep-mode claims"; this unit re-claims four of them
into the ported set on the argument that a whole-state validator *is* the sweep, leaving three.
P1b's open items flag the stale `92 advisory` figure for the lead but not this one. The same
paragraph's "each at the severity its Python carried" also needs the one lead-ruled divergence
noted (a common block carrying `class:` warns in Python, rejects in Rust, and has since P1). Fix:
fold both into the landing edit alongside 92 → 105.

**A8 — `Rule::extra` still drops a non-string key.**
`src/model.rs:717` collects with `filter_map(|(key, value)| key.as_str().map(...))`, so a mapping
key that is not a string is discarded and the round trip stays lossy for it. No shipped rule
carries one and YAML rule keys are strings by convention, so this is a corner rather than a hole —
but the fix that made the round trip honest for unknown string keys did not finish the job. Fix:
keep the raw key, or make a non-string key a decode error.

**A9 — an ASCII-only word boundary where the Python's is Unicode-aware.**
`is_word_byte` (`src/validate.rs:497`) is ASCII; Python's `\b` on a `str` pattern uses Unicode
`\w`. A citation immediately preceded by a non-ASCII letter matches in Rust and not in Python.
Negligible for the English rule text the corpus holds; recorded so the parity claim is exact
rather than approximate.

**Observation, not a finding.** `resolve_extends` now runs three to four times per rule — once in
the main loop, once or twice through `labels_carried`, and again in `validate_pointers` — and each
call scans `state.docs` for the library. That is O(rules × documents), the one super-linear path
this unit adds. It measures fine: the whole `migrate validate` pass is 0.06s of user time over
1,016 rules, and the validate suite runs in 0.23s. Worth remembering rather than fixing.

---

## Criterion-by-criterion

| # | criterion | verdict |
|---|---|---|
| 1 | every table check exists at the stated severity with a single-mutation test; Python parity where claimed | **pass** — 16 codes present at the claimed severities, all verified against the two scripts; citation scanner, pointer base and its three failure shapes, and the eight residuals all match; every one of the 46 rejecting codes is machine-proven to have a probe, and each new advisory code has dedicated tests with positive controls. Caveats at A5, A6, A9 |
| 2 | the ledger moves are honest; the 81 sums; the seven stale C3 rows corrected | **pass** — `accounted_for` is a bidirectional partition and passes; ledger arrays are 8/14/18 and 7/9/9, forcing ported 94 and 89, so the 81 resolves as 54 + 12 + 3 + 11 + 1; the seven C3 rows are removed from both `OUTSIDE_THE_HARD_SET` arrays and re-claimed with no new code; probes assert the id via `RejectOn`/`AdvisoryOn` and bare `Expect::Reject` is unused. Report bookkeeping error at A1 |
| 3 | the `extra` map is lossless; the round trip pins it; genesis hash unchanged | **pass** — decoded, re-emitted last in document order, canonical-hash equality asserted for a rule carrying `ruling:`; `migrations/` untouched in the diff so the genesis file is byte-identical, and `tests/fidelity.rs` stays green. Corner at A8 |
| 4 | corpus stays 0 rejecting; the 13 new advisories match the Python one-for-one | **pass** — reproduced by running both checkers: 0 findings each, the same nine zero-member labels on the same documents and the same four inherited absences on the same rules, and 87 pointers on both sides. 105 findings, 105 distinct, all advisory |
| 5 | hygiene | **pass** — no dependency change, no `unsafe`, no cwd-dependent lookup, no shipped file changed, fixtures under `CARGO_TARGET_TMPDIR`, `replay::load` unchanged and pinned by a test, GI-019 intact |
| 6 | what a skeptical senior would refuse | **A1–A9.** No quadratic scan that will not scale (see the observation). The citation scanner does not match inside a word — that is the whole point of `dotted_tail`'s backtracking, and it reproduces the Python's own token choice — and cannot match inside a `${var}`, whose names admit no dot. No panic path: every slice index is either a checked char boundary or the end of an ASCII run. No dead code. The pointer check does follow climbs out of the root, which is A3 |

---

## Gate outputs

Run by the reviewer at `e66d76e`, working tree clean.

```
$ cargo test --all                    296 passed; 0 failed, across 11 binaries
   anchor_grammar 5 · cli 26 · fidelity 10 · matrix_command 2 · matrix_similar 48
   matrix_skill 3 · migration 25 · render 27 · replay 46 · validate 94 · views 10
   exit 0

$ cargo fmt --all --check             exit 0 (no output)
$ cargo clippy --all-targets -- -D warnings
                                      exit 0 (re-run after touching lib.rs, not cached)
$ cargo audit --deny warnings         exit 0 — 31 crate dependencies, no advisories

$ cargo run -q -- migrate validate --log-dir migrations --plugin-root plugins/mochiko --report
   pointer resolution: 87 checked against plugins/mochiko
   mochiko-cli migrate validate · 0 rejecting · 105 advisory          exit 0

$ cargo run -q -- migrate validate --log-dir migrations --report
   pointer resolution: skipped (no --plugin-root; pointers are unchecked, not clean)
   mochiko-cli migrate validate · 0 rejecting · 105 advisory          exit 0

$ uv run scripts/check-command-schema.py --all      0 findings
$ uv run scripts/check-skill-schema.py              0 findings, 0 warnings per skill
```

---

## Method

I read P1b's report, the record's unit-1b paragraph, and both Python checkers before opening the
Rust, then read the whole 995-line `validate.rs` diff plus the `model.rs`, `cli.rs` and matrix
deltas. Parity claims were checked against the scripts line by line rather than taken from the
table — that is where A5, A6 and A9 came from, and where the citation backtracking and the pointer
branches were confirmed. The corpus claims were reproduced rather than accepted: I ran both Python
checkers and matched their warnings against the CLI's output by label, document and rule, and
summed the Python's per-skill pointer stats to 87 independently. The pointer pass was probed
against three scratch plugin roots — empty, root-only, and the real tree — to exercise all three
failure shapes and to confirm rejecting pointer findings print without `--report` and set exit 1.
The ledger arithmetic was recomputed from the arrays rather than read off the report, which is how
A1 surfaced. Nothing in the repository was edited; the scratch roots live in the session
scratchpad. Every gate above I executed myself.

---

## Delta-confirm — unit 1b advisory round

**PASS.** Five of nine closed, four named-not-changed as ruled. Nothing regressed.

Graded via `git diff e66d76e HEAD -- crates/mochiko-cli`. The crate, `migrations/` and `plugins/`
are unmodified in the working tree, so the graded surface is exactly HEAD. Each of the five fixes
was checked twice: read in the source, then reverted in a scratch copy of the crate to confirm the
named test goes red. A fix whose test does not fail without it is not a fix.

| item | verdict | evidence |
|---|---|---|
| **A2** advisory coverage guard | **CONFIRMED** | `every_advisory_code_is_raised_by_some_probe` (`tests/validate.rs:1697`) asserts set equality against `Code::ADVISORY` in both directions — `missing` and `unexpected` — over all 14 codes, mirroring the rejecting guard. It bites, and it isolates: deleting the `condition-coverage` mutation fails with `these advisory codes are declared but no probe raises them: ["condition-coverage"]`, and deleting the `unused-condition` mutation fails naming that code alone. So the two mutations are genuinely independent, which is what the lead asked |
| **A2** the two condition codes | **CONFIRMED** | Each has its own mutation with the distinction stated in a comment: `unused-condition` moves *both* users of the `map` dimension to `seats`, so the dimension itself goes unnamed; `condition-coverage` moves one `when:` from `present` to `absent`, leaving the dimension in use and one declared value uncovered. P1b's §13 records that the first draft conflated them and the guard caught it on its first run — the failure mode the guard exists for, found by the guard |
| **A1** report counts | **CONFIRMED** | §3 now reads 3 command extras and 5 total; §9 reads 97 probes for `matrix_command.rs`, 98 for `tests/validate.rs`, 300 total. All four figures match the code: 94 ported + 3 extra = 97, and the suite reports 300 |
| **A6** sigil scanner | **CONFIRMED** | `has_skeleton_sigil` (`src/validate.rs:1005`) scans to the first `}` and requires `}}` there, which is `\{\{[^}]*\}\}` exactly, and continues past a failed candidate. Two tests, not one: `a_brace_inside_the_sigil_is_not_a_skeleton_sigil` and `a_well_formed_sigil_after_a_malformed_one_still_fires` — the second guards the fix against over-correcting into an early return. Restoring the substring test fails the first and leaves the second green, which is the right discrimination |
| **A8** non-string rule key | **CONFIRMED** | `decode_rule` (`src/model.rs:717`) returns `DecodeError` rather than skipping, and `a_non_string_rule_key_is_a_decode_error_rather_than_a_silent_drop` asserts the message names the shape refused. Replacing the `return Err` with a `continue` fails it |
| **A4** pointer count pinned | **CONFIRMED** | `assert_eq!(report.checked, 87)` replaces `> 50` at `tests/validate.rs:2485`, with the reason in a comment. The corpus still reports 87 checked |
| **A5** retired-selector superset | **NOTED, documented** | `names_retired_selector` carries a doc comment citing the audit item, stating that `check-skill-schema.py` contains no occurrence of the word and that the Rust applies the lint corpus-wide deliberately, because the label is retired vocabulary rather than a fact about one grammar and the lint cannot block. Zero hits either way. The claim is now visible where a reader of the code will meet it |
| **A3** pointer containment | **NAMED, not changed** | §11 item 5 records the `../../../../CLAUDE.md` probe, states it is inherited parity under the Q4 ruling, and names the fix as two canonicalisations and a prefix assert |
| **A7** record sentences | **NAMED, lead's at landing** | §11 item 3 now names all three: `92 advisory` → 105, family 3 "7" → 3, and the severity-parity sentence needing the lead-ruled `class:` divergence noted. The seat's earlier open item had only the first |
| **A9** ASCII word boundary | **NAMED, not changed** | §11 item 6, with the reason it is negligible for the corpus and why it is recorded anyway |
| **`resolve_extends` re-scan** | **NAMED, not changed** | §11 item 7, carried as the observation it was raised as, with the measured figure |

**No regression.** The corpus is unchanged where it must be: 0 rejecting, 105 advisory, 105
distinct, 87 pointers checked. This mattered for two of the fixes — A8 changed the decoder and A6
narrowed a lint — and neither moved a shipped figure. `Cargo.toml` and `Cargo.lock` are untouched
this round.

The operating docs and the session record are modified in the working tree, which is the lead's
landing work, including the A7 corrections. Outside the crate and outside this grade.

### Gate outputs

```
$ cargo test --all                    300 passed; 0 failed, across 11 binaries
   anchor_grammar 5 · cli 26 · fidelity 10 · matrix_command 2 · matrix_similar 48
   matrix_skill 3 · migration 25 · render 27 · replay 46 · validate 98 · views 10
   exit 0        (validate 94 → 98; total 296 → 300)

$ cargo fmt --all --check             exit 0 (no output)
$ cargo clippy --all-targets -- -D warnings
                                      exit 0 (re-run after touching lib.rs, not cached)
$ cargo audit --deny warnings         exit 0 — 31 crate dependencies, no advisories

$ cargo run -q -- migrate validate --log-dir migrations --plugin-root plugins/mochiko --report
   pointer resolution: 87 checked against plugins/mochiko
   mochiko-cli migrate validate · 0 rejecting · 105 advisory          exit 0
```

**Final verdict: PASS.**
