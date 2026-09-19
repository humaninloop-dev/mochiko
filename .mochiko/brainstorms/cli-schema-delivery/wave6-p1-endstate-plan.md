# P1 — crate + log: the end-state plan

Pre-wave, at `62aa99d`: grammar 1, sequences 1..2, state hash
`sha256:8972891099f77b1080b243f73adc7ea5ae0c3c9479cb17414cbe1fd97cfd43fd`, 50 documents, 1016
rules; `migrate validate --report --plugin-root plugins/mochiko` gives 0 rejecting, 105 advisory,
87 pointers checked, similarity 1016/146572/0/181. `plugins/mochiko/schemas` is 272 KB.

## 1. `mochiko-cli doc <name>`

Beside `template`, taking the global `--plugin-root` for the head line's plugin version. Serves the
three non-rule, non-template documents in state — the shelf, `command-labels`, `skill-labels` — the
name set derived by kind, so a second shelf needs no code. Body is
`views::to_yaml(&document.to_value())` without the derived-view comment header, which describes a
file rather than a delivery. Head `mochiko-cli doc <name> · binary <v> · grammar <g> · plugin <p>`,
end `mochiko-cli doc end · <name>`. Bodies are 3.2 KB, 1.3 KB, 1.7 KB — no chunking, no `--section`.
Exit codes are the existing contract: 0 ok, 1 absent/empty/unsound log, 3 grammar skew, 2 unknown
name. Two new `RenderError` arms — `UnknownDocument` lists the available names as `UnknownSection`
does, `WrongCommand` names the right command for a name of another kind (`mochiko-cli template
<name>`; `mochiko-cli rules <name> --section preamble`). Common libraries get no `doc` form: the
render resolves every stub first. Tests first — each of the three renders with both lines at exit 0;
the body re-parses to its document; an unknown name exits 2 listing the three; `spec`, `specify`,
`review-feasibility` each exit 2 naming the right command.

## 2. The skill legend

`LEGEND` splits into `COMMAND_LEGEND` (unchanged, 845 bytes, 9 lines) and `SKILL_LEGEND` (673 bytes,
7 lines): the two `enforces:` lines absent, the `kind:` line carrying the eight-kind skill set.
`preamble()` picks on the existing `is_command` flag. Goldens on `review-brainstorm` and
`implement`; the size pin becomes two pins.

## 3. Views relocate

`views::view_path` re-keys on kind and loses the `plugins/mochiko/` prefix: `commands/<cmd>` ·
`skills/<skill>` · `common/<name>` · `labels/<name>` · `templates/<name>` · `shelves/<name>`. I emit
once into `.mochiko/schema-views/` and commit the 50 files; `.gitignore` untouched. CI: the `push`
and `pull_request` filters swap `plugins/mochiko/schemas/**` for `.mochiko/schema-views/**`.

`tests/views.rs`: `shipped_state()` becomes a `replay::load` of `plugins/mochiko/migrations`; the
semantic-equality test compares each emitted view against the committed file under
`.mochiko/schema-views/`, by canonical hash, still pinned at 50; the header test reads the committed
command view; the layout and `emit_to` tests assert the six new prefixes; the writer unit tests are
state-independent and stand. `tests/render.rs`: `SHIPPED_SCHEMAS_DIR` goes, `shipped_state()` and
`template_log()` come from the replay, and the two tests asserting the files exist on disk become
assertions that the log carries the eight templates and the shelf. `tests/validate.rs`'s
`shipped_documents()` likewise; its pointer pin drops from 87 to whatever the three cleared pointers
leave — measured, then pinned exactly.

**Beyond §0's crate inventory**, found by grep: `tests/fidelity.rs` reads the live tree three times
(`genesis::scan(&repo_root())`, `genesis::build(&repo_root())`, and the sidecar P2 moves);
`tests/anchor_grammar.rs` reads the same sidecar; `tests/matrix_similar.rs` builds its corpus from
`genesis::scan(&repo_root())`, and one test reads `scripts/test-find-similar-rules.py`, which P2
deletes. The sidecar and corpus reads re-point at `tests/fixtures/genesis-corpus`, which carries its
own sidecar. Fidelity's rule-by-rule comparison replays **genesis alone** into a temp log: the frozen
corpus is the pre-0002 tree, so comparing it against the full replay would grade drift the log
already accounts for. The Python-parity test and its `PYTHON_PROBES` table retire with the script.
`matrix_similar`'s two figure pins are re-measured against the post-0003 state, since rewording moves
the text those numbers score.

## 4. Migration `0003-two-arm-to-cli.yaml`

Header anchor `2026-09-03 cli-schema-delivery D9`; 23 changes. No op in it requires an anchor by the
log's own rule — reword and a `pointer:` clear lower no protection — so the anchor is authority the
migration does not strictly need, carried because the ruling is real.

**`set-var` ×6.** `tasks_schema` on `command/implement` and `command/feature` → `mochiko-cli template
tasks`. On `command/specify`: `spec_schema` → `mochiko-cli template spec`, `feature_entry_schema` →
`mochiko-cli template feature-entry`, `features_index_schema` → `mochiko-cli template
features-index`. On `command/architecture`: `store_schema` → `mochiko-cli template
architecture-store`, `shelf_schema` → `mochiko-cli doc architecture-shelf-backend`. Every reworded
text keeps the `${var}` it already carried, so no var falls out of use and each substitution reads as
a CLI invocation.

**`set-rule-field` ×3, clearing `pointer:`** where it aims at a file that will not exist:
`analysis-codebase.deliverable-two-arm-binding`, `patterns-architecture-shelves.opinions-in-data`,
`patterns-vertical-tdd.tasks-binding-two-arm`. Left standing these are three rejecting
`pointer-unresolved` findings the moment P2 deletes the directory, blocking the audit pre-pass. §0
did not list them.

**`reword-rule` ×14**, minimal: struck span first, replacement second; an empty replacement means the
sentence closes there.

- `architecture` `arch.tools-store-schema` — `` (rendered by mochiko-cli, or Read raw when the binary is absent — the shipped schema is the first-class source of truth)`` → `.`, giving "The store shape is ${store_schema}, the shelf data ${shelf_schema}. A small required core; …".
- `feature` `feat.delta-cards` — ``(rendered by `mochiko-cli template tasks`, or its schema ${tasks_schema} Read raw when the binary is absent — the shipped schema is the first-class source of truth)`` → ``(rendered by `${tasks_schema}`)``.
- `implement` `impl.cards-template` — `— rendered by mochiko-cli template tasks; when the binary is absent, its schema ${tasks_schema} Read raw is the first-class source of truth.` → `— rendered by ${tasks_schema}.`
- `setup` `setup.synthesis-artifact` — `, or its schema plugins/mochiko/schemas/governance-intent.yaml Read raw when the binary is absent — the shipped schema is the first-class source of truth` struck.
- `setup` `setup.feature-map-brownfield` — `, or their schemas plugins/mochiko/schemas/features-index.yaml and plugins/mochiko/schemas/feature-entry.yaml Read raw when the binary is absent, the shipped schemas being the first-class source of truth` struck.
- `specify` `spec.deliverable` — ``(rendered by `mochiko-cli template spec`, or its schema ${spec_schema} Read raw when the binary is absent — the shipped schema is the first-class source of truth)`` → ``(rendered by `${spec_schema}`)``.
- `specify` `spec.feature-map-craft` — ``(the feature-entry template — `mochiko-cli template feature-entry`, or its schema ${feature_entry_schema} Read raw when the binary is absent, the shipped schema being the first-class source of truth)`` → ``(the feature-entry template — `${feature_entry_schema}`)``; and ``(the features-index template — `mochiko-cli template features-index`, or its schema ${features_index_schema} Read raw when the binary is absent, likewise the first-class source)`` → ``(the features-index template — `${features_index_schema}`)``.
- `skill-authoring-common` `authoring-common.two-arm-template` — whole text → ``Invoke `mochiko-cli template ${template}`.`` Four skills extend it: `authoring-prototype`, `authoring-feature-map`, `authoring-constitution`, `authoring-architecture-store`.
- `analysis-codebase.deliverable-two-arm-binding` — `` when the binary is available; otherwise Read `plugins/mochiko/schemas/codebase-analysis.yaml` raw`` struck.
- `authoring-feature-map.feature-entry-two-arm` — `` when the binary is available; otherwise Read `plugins/mochiko/schemas/feature-entry.yaml` raw`` struck.
- `authoring-technical-requirements.nfr-store-home` — ``(`plugins/mochiko/schemas/architecture-store.yaml`)`` → ``(`mochiko-cli template architecture-store`)``. `class: floor`, anchored; the reword keeps both.
- `patterns-architecture-shelves.opinions-in-data` — ``the backend shelf ships at `plugins/mochiko/schemas/architecture-shelf-backend.yaml`, Read raw`` → ``the backend shelf is delivered by `mochiko-cli doc architecture-shelf-backend```.
- `patterns-vertical-tdd.tasks-binding-two-arm` — `` when the binary is available; otherwise Read `plugins/mochiko/schemas/tasks.yaml` raw`` struck. `class: floor`, anchored; the reword keeps both.
- `review-plan-artifacts.cycle-card-check-mirror` — `` , or Read `plugins/mochiko/schemas/tasks.yaml` when the binary is absent`` struck.

Three ids still read "two-arm" afterwards; ID continuity forbids renaming them, so they stay and the
log records why. Then `migrate stamp`, and `migrate validate` reporting 0 rejecting.

## 5. The state-wide assert

A new test in `tests/fidelity.rs`, which already replays the committed log: for every document,
serialize `document.to_value()` through the views writer and assert it carries none of
`plugins/mochiko/schemas/`, `when the binary is absent`, `when the binary is available`. The third
needle is mine — the other half of the same construct, and §2.4 names only the first two. It sweeps
rule text, vars, section intents, notes, conditions, templates, the shelf and both registries in one
pass, wider than "no rule text". The two synthetic fixtures writing `target:
plugins/mochiko/schemas/demo.yaml` are writer test data the assert never sees; I leave them.

## 6. The log README

The op table, the anchor rule and the sequence table stand. The wave-4 snapshot paragraph is
rewritten: the snapshot files are gone, the log is the only schema surface a run reads, and the
human-readable projection lives at `.mochiko/schema-views/`, regenerated by `mochiko-cli views emit
--plugin-root plugins/mochiko --out .mochiko/schema-views`, never hand-edited. The hand-mirroring
sentence goes with it.

## 7. Verification and sequencing

`cargo fmt --all --check` · `cargo clippy --all-targets -- -D warnings` · `cargo test --all` ·
`MOCHIKO_FULL_SIMILAR=1 cargo test --test matrix_similar` · `cargo audit --deny warnings`. Then
`migrate validate --report --plugin-root plugins/mochiko` at 0 rejecting, `migrate status` for the
post-wave hash recorded beside the pre-wave one above, `views emit` followed by a clean `git diff` on
`.mochiko/schema-views/`, and one `doc` render of each of the three names. Test-first throughout;
locate-and-enumerate reads dispatched to a haiku `Explore` seat, interpretive reads kept on this
tier. The Python-parity test must leave in my commit, before P2 deletes
`scripts/test-find-similar-rules.py`; the reverse order breaks CI.

## 8. Two decisions for the lead

- **The skill legend's `moments:` line.** §2.2 names two changes, so I keep it. Its wave-4 rationale
  was that a second legend was not worth maintaining, and that is gone once we maintain one. Dropping
  it gives 605 bytes, 6 lines. Say the word; otherwise §2.2 as written.
- **`COMMAND_HEADER`.** It opens "The command .md instructs a raw, full Read of this file at command
  fire" — false from wave 6, and committed into every command view. I plan to rewrite that first
  sentence to the derived-view form and keep the two semantics paragraphs, which carry the GI-006
  reconstruction value. P2's strips-README note for the deleted snapshots covers it.
