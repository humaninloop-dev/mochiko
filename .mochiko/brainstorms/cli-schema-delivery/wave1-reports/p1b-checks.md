# Wave 1 — extension unit 1b (family-2 checks) cycle report

**Seat:** P1b — the family-2 checks the user ruled into wave 1 on 2026-09-04, plus the eight
shipped-checker residuals the lead ruled in at plan approval (Q2).
**Plan:** returned and approved with rulings Q1–Q6. Substrate: the crate at wave-1 acceptance.
**Shipped files:** no file under `plugins/` changed. Nothing committed.

**Gates at close (advisory round):** `cargo test --all --no-fail-fast` **300 passed, 0 failed**
across eleven test binaries · `cargo fmt --all --check` clean · `cargo clippy --all-targets -- -D warnings` clean ·
`cargo audit --deny warnings` clean. `migrate validate --log-dir migrations --plugin-root
plugins/mochiko --report` reports **0 rejecting · 105 advisory**, with 87 pointers checked.

---

## 1. The check table, as built

Sixteen new finding codes: nine rejecting (`Code::REJECTING` 37 → 46) and seven advisory
(`Code::ADVISORY` 7 → 14). Severity follows the Python in every case but one, disclosed at §4.

| Python check | Py severity | Rust code | Severity | State-level semantics |
|---|---|---|---|---|
| 12 · in-text ID citation | finding | `cite-unresolved` | reject | One scanner over resolved text, two limbs. Section tokens (`<stem>.sec.<slug>`) resolve against the document's own sections; every other `<prefix>.<segs>` token resolves against its live ids. A tombstoned target is a superseded reference, not a dangle (Q1). File-suffix tokens (`.md`, `.yaml`) are paths. A common library is skipped — its text is scanned at each binding stub, where the resolution set is real |
| 12 · foreign prefix/stem | warning | `cite-foreign` | advisory | Named once per document, sorted. Command scan set is the six frozen prefixes plus every loaded command's derived prefix; skill scan set is every skill name in state |
| 11 · pointer resolution (J-7) | finding | `pointer-unresolved` | reject | Skill-side only (Q4). Path-shaped = carries `/` or ends `.md`; `mochiko:<skill>` is a name. Base is `<root>/skills/<name>/`. Three messages: absolute · resolves only from the plugin root · resolves to no file |
| 6 · inline `ruling:` (D16) | finding | `superseded-field` | reject | Reported off the preserved `extra` map (§2) |
| — · any other unmodelled key | — | `unknown-field` | reject | The rest of `extra`; new, per Q5 |
| 2b · flat top-level `rules:` (D14) | finding | `flat-rules` | reject | A `Command` or `Skill` document with non-empty `blocks`. The decoder files top-level `rules:` there for every rule-bearing kind, so before this the flat grammar decoded silently |
| load_common · C3 absence-meaningful | finding (`class`: warning) | `extends-class-local` | reject | **Already ported.** Seven ledger rows said otherwise and were stale |
| 2 · retired label in registry | finding | `retired-label` | reject | A registry carrying `fail-condition` in `labels:` |
| 2b/5 · retired selector in prose | warning | `retired-selector` | advisory | `fail-condition` not followed by `s`, in a section title or intent and in resolved rule text. The live `fail-conditions` slug is excluded by the grammar, not filtered after |
| resolve_extends · pointless override | warning | `pointless-override` | advisory | Local `text` whitespace-collapse-equal to the block's. Re-wrapping is not an override |
| 14 · orphan block | warning | `orphan-block` | advisory | Cross-document pass: every `extends:` target bound anywhere in state, then per library the blocks bound by none. Guarded — no claim over a library whose family has no member in state |
| end · zero-member label | warning | `zero-member-label` | advisory | Command: per document, matching the Python's per-pair scope. Skill: once on the registry, over every skill at once, matching its sweep scope |
| 5 · `{{…}}` sigil | warning | `skeleton-sigil` | advisory | In resolved rule text |
| 7 · label-less rule | finding + carve | `labels-missing` | reject | Resolved labels absent or empty. Never on a library block: the census assigned some posture blocks none |
| 7 · inherited label absence | warning | `labels-inherited` | advisory | Skill-side, `labels:` absent locally and the bound block carries none. A **local** `labels: []` stays a finding whatever the block says |
| 10/11 · moment with no line | finding | `moment-declaration` | reject | A declared moment whose navigation line is empty |
| various · empty document | finding | `document-empty` | reject | A library with no blocks, or either registry with no `labels:` mapping — three Python findings, one code |

Two further residuals landed on existing codes rather than new ones: a `when:` term naming a
dimension with an empty value list reports `when-value`, and a node tombstoned twice reports
`tombstone-integrity`. A section with no `id:` was already rejected as `id-format` and is filed
with `Probe::porting`, which records the Python name beside the Rust one.

**The sweep is not a mode, it is the shape.** Four probes the wave-1 report filed as family 3
(unportable sweep claims) do have referents: a whole-state validator *is* the sweep the Python
performs at the end of a run, so "a block bound by no stub in any swept skill" and "a label no
swept schema carries" are exactly what a cross-document pass answers. What has no referent is the
*single-skill* run — three probes, still outside, named as such.

## 2. The model delta, as amended (Q5)

`Rule` gained one field: `pub extra: Ordered<Value>`, every key the grammar does not model, in
document order. `RULE_KEYS` names the eleven it does. `Rule::to_value` writes `extra` back last.

The lead's amendment is the better shape, and the reason is worth stating: a dedicated `ruling`
field would have made the model carry a spelling a ruling retired. The map instead records that
the document said *something* the grammar has no home for, and the validator names it — the D16
message for `ruling`, `unknown-field` for anything else. Both are pinned.

The round trip was **quietly lossy before this** and is not now: an unknown key decoded to
nothing and re-encoded as nothing, so a document could lose content and still hash equal to
itself. `an_unknown_rule_key_is_preserved_through_the_round_trip` asserts the canonical hash of a
rule carrying `ruling:` survives decode and re-encode. No shipped rule carries an unmodelled key,
so `canonical_hash` over the corpus and `migrations/0001-genesis.yaml` are byte-unchanged —
`tests/fidelity.rs` regenerates the log byte-identically and still passes.

## 3. The 81-probe accounting, summed

Every figure below is read off the two matrices' own set-algebra tests, which assert each Python
probe is claimed by exactly one ledger.

| | command | skill | total |
|---|---|---|---|
| Python probes | 134 | 114 | 248 |
| ported (was 66 / 63) | **94** | **89** | 183 |
| beyond the Python matrix (`Probe::extra`) | 3 | 2 | 5 |
| genesis-side | 8 | 7 | 15 |
| not applicable under D6 | 14 | 9 | 23 |
| outside the hard set (was 46 / 35) | **18** | **9** | 27 |

**The 81 named at wave-1 acceptance, all accounted:**

| disposition | count | detail |
|---|---|---|
| re-claimed into the ported set | **54** | 47 family-2 (22 command · 25 skill) + 7 residuals (6 command · 1 skill) |
| family 1 — the decoder rejects it before a finding exists | 12 | 11 command · 1 skill. A decode error is not a `Finding`, so a probe asserting one has nowhere to land |
| family 3 — no referent in a whole-state validator | 3 | all skill, all about a *single-skill* run |
| family 4 — `.md` pin grammar and report wording, dead under D6 | 11 | 6 command · 5 skill |
| named residual, unported | **1** | below |

54 + 12 + 3 + 11 + 1 = 81.

## 4. The eight residuals (Q2): seven ported, one named

| residual | outcome |
|---|---|
| `when:` naming a dimension with an empty value list | ported, `when-value`, rejecting |
| the same node tombstoned twice | ported, `tombstone-integrity`, rejecting |
| a moment declared with no navigation line | ported, `moment-declaration`, rejecting |
| the library carrying no `rules:` list | ported, `document-empty`, rejecting |
| the registry carrying no `labels:` mapping (command) | ported, `document-empty`, rejecting |
| the registry carrying no `labels:` mapping (skill) | ported, `document-empty`, rejecting |
| a section missing its `id` | ported under `id-format` via `Probe::porting` — already rejected, by the limb that reaches it first |
| **a section missing its `rules` key** | **not ported.** Reason below |

**Why the last one stayed out.** The Python separates three states — `rules:` absent, `rules:`
null, and `rules: []` — and the model reads all three as an empty section. Telling them apart
needs a `Section` field recording whether the key was written, and `model.rs` was granted for the
`extra` map alone. It is also the thinnest of the eight: an empty section is already a finding
unless it carries a `note:`, whichever way it was spelled. Taken as the lead's escape clause
invited rather than forced. The ledger row now carries this whole argument.

**One severity divergence, kept and disclosed (Q3).** The Python *warns* on a common block
carrying `class:`; the Rust rejects it, and has since P1. Kept rejecting — it is shipped
behaviour the corpus already satisfies, and weakening a live check to match a retiring script is
the wrong direction. The probe is filed with `Probe::porting` and the divergence is stated in the
matrix.

## 5. How pointer resolution receives a root, and what it says when it has none

`validate(state)` is unchanged and remains the state-only hard set, so `replay::load` still means
"deliverable" and nothing runs twice. Pointer resolution is a separate `validate_pointers(state,
root) -> PointerReport`, called only by the maintainer gate.

The boundary is the point, not an accident of plumbing. Every other check is a fact about the
store's own data, true wherever the log is read. This one is a fact about the tree beside it, and
the replay has no tree.

`PointerReport` carries `checked` alongside `findings`, because *no findings over zero pointers*
is not the claim *no findings over every pointer*. Under `--report`, `migrate validate` prints one
of two lines, always:

```
pointer resolution: 87 checked against plugins/mochiko
pointer resolution: skipped (no --plugin-root; pointers are unchecked, not clean)
```

It is gated on `--report` because a bare `migrate validate` prints exactly one line, which P2's
`migrate_validate_exits_0_on_a_sound_log_and_reports_its_tally` asserts. A *rejecting* pointer
finding prints either way, as every rejecting finding does.

The `cli.rs` delta is the one argument the plan named plus those two lines.

## 6. The red/green/refactor trail

Five cycles, each opened on a failing test. The coverage guard
`every_rejecting_code_is_raised_by_some_probe` stayed red across all five — it names each
unraised code — and was the last thing to go green, which is the shape it was built for.

| cycle | red | green |
|---|---|---|
| 1 — the `extra` map | three tests would not compile: no `Rule::from_value`, no `extra`, no codes | the model field, `RULE_KEYS`, `check_extra_fields` |
| 2 — citations | 12 tests failing on an absent `cite-unresolved` | the two scanners, `check_citations`, the prefix sets |
| 3 — labels, flat rules, selector, sigil | 11 failing | the label carve, `flat-rules`, `names_retired_selector`, the sigil limb |
| 4 — the library and the residuals | 14 failing | `cross_document`, `zero_member_labels`, `collapse_whitespace`, the six residual limbs |
| 5 — pointers | 10 failing on an absent `validate_pointers` | the rooted pass, `PointerReport`, the CLI wiring |

Three findings worth more than the tests that passed.

**The C3 guard was already there.** Seven ledger rows across the two matrices said the
absence-meaningful-field guard was not ported. It is — `check_class_and_kind` has carried it since
P1, with a test. Seven probes were re-claimed with **zero new code**. Worth stating as a method
point: the plan checked the source rather than trusting the ledger, and the ledger was wrong.

**Deriving the citation scan set from state alone was too narrow.** With one command in state
there is no sibling prefix, so `spec.gate-acceptance` in a `demo` rule was not scanned at all and
the foreign-citation probe could not land. The six frozen command prefixes are now unioned in, as
the Python does it, for the reason the Python does it: a citation of `spec.*` is a foreign
reference whether or not `specify` happens to be loaded. Skill stems stay state-derived, there
being no frozen list.

**The matrix probes were falsified before being trusted.** Twenty-eight command and twenty-six
skill probes passed on the first run, which is the kind of result worth doubting. Two expectations
were deliberately inverted and both failed with the right message — `expected clean, got
pointer-unresolved on demo-grader.read-report` and `expected no rejecting finding, got
labels-missing on demo-grader.carve-out` — then restored. The rows discriminate.

## 7. The shipped corpus: 0 rejecting, and the advisory delta named

Both Python checkers report **0 findings** on the shipped tree today; that was measured before any
code was written, so no rejecting check added here could push the corpus off zero. It did not.

| | before | after |
|---|---|---|
| rejecting | 0 | **0** |
| advisory | 92 | **105** |

The thirteen new advisories are the Python's own warnings, one for one — same labels, same
documents, same rules:

- **9 × `zero-member-label`** — `attempt-economy` in architecture, feature, setup and specify ·
  `scope-entry` and `stewardship` in brainstorm · `binding` and `stewardship` in implement ·
  `stewardship` in specify.
- **4 × `labels-inherited`** — `<skill>.letter-is-spirit` in authoring-feature-map,
  authoring-prototype, authoring-technical-requirements and authoring-user-stories, each
  inheriting from `authoring-common.letter-is-spirit`, which carries no labels.

Every other new advisory fires zero times on the corpus: no foreign citations, no retired
selector in data (it survives only in a YAML comment, which the model drops), no pointless
overrides, no orphan blocks, no sigils. **87 of the 87 path-shaped shipped pointers resolve**
base-directory-relative, asserted against the real tree by
`every_shipped_pointer_resolves_from_its_own_skill_directory`.

The record's wave-1 section still says `92 advisory`; per Q6 that figure is the lead's to update
at landing.

## 8. Pre-code ladder disclosures

| rung | not built | why |
|---|---|---|
| exist at all | a regex crate | P1's Q4 ruling holds. Both scanners are hand-written, including the backtracking a word-boundary needs (`dotted_tail` walks candidate stops, because `demo.a.b_c` must yield `demo.a`) |
| exist at all | a C3 guard | already implemented and tested; 7 probes re-claimed with no code |
| exist at all | a `--json` surface, per-check opt-outs | outside the ruled scope |
| in codebase | a second `extends:` resolver, placeholder scanner, family map | reused `resolve_extends`, `placeholders`, `Family::of`, `common_prefix_of`, `derive_prefix`, `registry_labels`, `Finding`/`Code` |
| in codebase | a `norm_text` dependency | `collapse_whitespace` is one line over `split_whitespace` |
| simpler shape | threading a root through `replay` | a filesystem fact must not become a property of the log; a separate rooted pass says so structurally |
| simpler shape | two codes for the empty-document family | one `document-empty` covers three Python findings |
| minimum now | two cross-document passes | one pass serves orphan blocks and zero-member labels |
| stdlib | `walkdir` | `Path::exists` is the whole pointer check |
| one line | a new dependency | `Cargo.toml` untouched |

## 9. Test tally

| suite | tests | state |
|---|---|---|
| `tests/validate.rs` | 98 | pass — was 44 |
| `tests/matrix_command.rs` | 2 (97 probes) | pass — was 69 |
| `tests/matrix_skill.rs` | 3 (91 probes) | pass — was 64 |
| `tests/matrix_similar.rs` | 48 | pass, untouched |
| `tests/replay.rs` · `migration.rs` · `render.rs` | 98 | pass, untouched |
| `tests/cli.rs` | 26 | pass |
| `tests/fidelity.rs` · `views.rs` · `anchor_grammar.rs` | 25 | pass |
| **total** | **300** | **300 pass, 0 failed** |

Fifty-four tests added; 246 at wave-1 acceptance. (Fifty at the unit's close; four more in the
advisory round, §13.)

## 10. Files touched

Exactly the granted pen, verified by `git status`:

- `crates/mochiko-cli/src/validate.rs` — 16 codes, the two scanners, `check_citations`,
  `check_extra_fields`, `cross_document`, `zero_member_labels`, `validate_pointers`,
  `find_block`, `collapse_whitespace`, `names_retired_selector`, and limbs in
  `check_discriminators`, `check_sections`, `check_labels`, `check_when`, `check_text`,
  `validate_registry`, `resolve_extends`.
- `crates/mochiko-cli/src/model.rs` — the `extra` map, `RULE_KEYS`, `Rule::from_value`, and the
  `to_value` write-back. Nothing else.
- `crates/mochiko-cli/src/cli.rs` — one argument into `run_validate`, the pointer pass, two
  `--report` lines.
- `crates/mochiko-cli/tests/validate.rs` — 50 tests, and 9 rows plus a rooted pass into the
  coverage guard.
- `crates/mochiko-cli/tests/matrix_command.rs`, `tests/matrix_skill.rs` — 54 ledger moves, the
  probes behind them, and the stale reasons corrected.
- `crates/mochiko-cli/tests/matrix/mod.rs` — `Fixture.root`, `use_pointer_root`, `drop_skill`,
  and `findings()` appending the rooted pass.

## 11. Open items

1. **One residual unported** — a section missing its `rules` key (§4), needing a `Section` field
   outside this unit's pen. The lead rules whether it is worth a delta before wave 6.
2. **Command-side pointers are resolved by nothing** — 23 path-shaped `pointer:` values in
   `architecture` (7), `implement` (14) and `common.yaml` (2). No checker has ever resolved them,
   Python included. Parity was the Q4 ruling; the gap is named for the wave landing.
3. **The record's unit-1b paragraph is stale in three places, all the lead's at landing** — the
   `92 advisory` figure is now 105 (§7); family 3 is described as "7 per-skill sweep-mode claims"
   and is now 3, four having been re-claimed on the argument that a whole-state validator is the
   sweep; and "each at the severity its Python carried" needs the one lead-ruled divergence noted
   (a common block carrying `class:` warns in Python and rejects here, and has since P1). Audit
   A7, raised there because the seat's own open items had named only the first.
4. **The two matrices still carry transcribed `PYTHON_PROBES` arrays** — P3's open item 6, and the
   54 moves did not change it. `matrix_similar.rs` re-derives its names from the script; the same
   treatment stays cheap here.
5. **Pointer resolution follows climbs out of the plugin root** (audit A3). `Path::exists` resolves
   `../../../../CLAUDE.md` from a skill directory to the repository's own file, and it passes
   clean. This is inherited parity — the Python's `(skill_dir / p).exists()` has no guard either,
   and Q4 ruled parity — so it is named rather than changed, per the lead. The check reads as
   "this pointer resolves" and means "something exists at that path, wherever that is". A
   containment guard is two canonicalisations and a prefix assert whenever the lead wants it.
6. **The citation scanner's word boundary is ASCII where the Python's is Unicode** (audit A9).
   `is_word_byte` classifies ASCII only; Python's `\b` over a `str` pattern uses Unicode `\w`, so
   a citation immediately preceded by a non-ASCII letter matches here and not there. Negligible
   for the English rule text the corpus holds, and recorded so the parity claim reads as exact
   rather than approximate.
7. **`resolve_extends` runs three to four times per rule**, each call scanning `state.docs` for
   the library — the one super-linear path this unit adds, raised as an observation rather than a
   finding by the audit. Measured fine: `migrate validate` is 0.06 s of user time over 1,016
   rules. Worth remembering rather than fixing.

## 12. Suggested commit

```
Port the family-2 checks and the shipped-checker residuals

Wave 1 extension unit 1b of the CLI schema-delivery build. The validator
gains sixteen finding codes: in-text rule-ID citation resolution over both
grammars, skill-side pointer file resolution, the inline `ruling:` and flat
top-level `rules:` guards, the retired fail-condition selector, the orphan
block and pointless override warnings, the zero-member label warning at each
grammar's own scope, the label-less rule check with its inherited-absence
carve, and the {{...}} sigil.

54 of the 81 probes wave 1 named as exercising checks the hard set did not
carry are re-claimed into the ported ledgers. The remaining 27 are named with
their reason: 12 shape errors the decoder rejects before a finding exists, 3
single-skill-run claims a whole-state validator has no referent for, 11 .md
pin and report-wording probes dead under D6, and one section-shape residual
whose port needs a model field outside this unit's pen.

Rule gains a preserved `extra` map of unknown keys, decoded and re-emitted,
which both fixes a quietly lossy round trip and gives the D16 guard a field
to name. Pointer resolution reads the installed tree rather than the store, so
it is a separate rooted pass that migrate validate calls with --plugin-root
and reports as skipped when it has none — never as clean.

The shipped corpus stays at 0 rejecting. Advisory goes 92 to 105: nine
zero-member labels and four inherited label absences, each matching a warning
the Python checkers already print. All 87 shipped pointers resolve.

No new dependency. No file under plugins/ changed.
```

## 13. Advisory round

Against `v1b-checks-audit.md` (PASS: 0 blocking, 9 advisory), unit frozen at `e66d76e`. The two
items the lead directed, plus three of the remaining seven that were genuine one-line fixes. Each
went red first.

### A2 — the advisory code set gets the guard the rejecting set has (directed)

`every_advisory_code_is_raised_by_some_probe` now asserts set equality over all **14** advisory
codes in both directions, mirroring the rejecting guard. The pre-existing assertion ran one way
only — every advisory finding raised carries a declared code — so a fifteenth code with no probe
behind it would have failed nothing.

It earned its place twice while being written. The first run named `unused-condition`: the
mutation meant to raise it moved only one of the `map` dimension's two users, so the dimension was
still in use. The second run then named `condition-coverage`, which the broken mutation had been
raising by accident. Both now have a mutation of their own, and the pair pins the distinction the
two codes exist for — **a dimension no rule names** against **a dimension in use whose declared
value no rule names**.

### A1 — the report's command probe counts, corrected (directed)

The grader's recount is right. `tests/matrix_command.rs` holds **three** `Probe::extra` call sites,
not four, and **97** probes, not 98. My count came from `grep -c 'p.push('`, which reads 95 and
undercounts by two because the C3 row pushes three probes from one site inside a loop; I then
carried a second arithmetic slip on top of it. Corrected in §3 and §9. **The 81-probe accounting
is untouched** — extras carry `python: None` and sit outside the partition, which the matrix's own
set-algebra test enforces rather than asserts.

Counting by hand is how P3's ledger went wrong at wave 1, and it is how this went wrong here. The
figures the accounting rests on are machine-checked; the two I typed by eye were not.

### A6 — the sigil scanner now matches the shipped one (taken)

`has_skeleton_sigil` replaces a substring test for `{{` followed anywhere by `}}`. The shipped
`\{\{[^}]*\}\}` forbids a `}` inside the body, so `{{a}b}}` fired here and not in Python. The scan
continues past a failed candidate rather than stopping, so a well-formed sigil later in the same
text still fires — pinned by both cases.

### A8 — a non-string rule key is now a decode error (taken)

`extra` is keyed by `String`, so a non-string mapping key could not be carried through it and was
being dropped — leaving, for that corner, exactly the lossy round trip the map exists to close.
The decoder now refuses the document instead of quietly thinning it. No shipped rule carries one,
and `tests/fidelity.rs` stays green.

### A4 — the shipped-pointer count is pinned exactly (taken)

`assert_eq!(report.checked, 87)` replaces `> 50`. A figure this report leans on twice should fail
when it moves, and the corpus census elsewhere pins exact numbers.

### A5 — the retired-selector lint is a deliberate superset, now said so (documented)

The grader is right that `check-skill-schema.py` contains no occurrence of `fail-condition`: the
lint is command-side only in the Python, and it is applied here to every rule-bearing document.
Kept, and documented at the definition. The label is retired vocabulary across the corpus rather
than a fact about one grammar, and the lint is advisory — a superset can inform and cannot block.
Zero hits on the shipped tree either way. The widening is now stated where a reader meets it,
which is what the finding asked for.

### Named, not taken

A3 (pointer climbs escaping the plugin root — Python parity, and the lead's instruction was to
name it), A7 (three stale sentences in the record's unit-1b paragraph, the lead's at landing), A9
(ASCII versus Unicode word boundary) and the audit's performance observation are all carried into
§11 as items 3, 5, 6 and 7.

### Gates

`cargo test --all --no-fail-fast` **300 passed, 0 failed** · `cargo fmt --all --check` clean ·
`cargo clippy --all-targets -- -D warnings` clean. `migrate validate --log-dir migrations
--plugin-root plugins/mochiko --report` still reports **0 rejecting · 105 advisory**. Four tests
added: the advisory guard, two sigil cases, and the non-string key. Nothing committed; no file
under `plugins/` changed.
