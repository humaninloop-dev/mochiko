# Wave 6 — the end state (lead-drafted referent)

**Ruling home:** `record.md` D2 (provenance folded into the log; sidecar frozen), D6 (the Python
scripts retire after the matrix port — ported at wave 1), D8 as amended (the no-Read assert goes
run-wide from wave 6), D9 wave 6 ("the shipped snapshot files are deleted from the plugin, the
derived views relocate repo-side to `.mochiko/schema-views/`, human-readable, never shipped — the
plugin ships the migration log only, and 'no fallback files' is literally true; Python retirement;
provenance sidecar frozen; the two-arm text migrated to CLI-only across its 32 sites, router row 58's
phantom template fixed in the same migration; converter skills superseded; doc landings"), D10.6
(`primitive-edits.md` and the delivery watches re-keyed), governance v3.0.2 (the transition clause's
expiry and the `primitive-edits.md` re-key are a **pre-authorized PATCH**, ledger amendment policy),
and the wave-5 follow-ups. **Wave open (2026-09-05), user-ruled:** lands as **0.107.0**; **no halt
paragraph trim** (the patterns overage stays accepted). **Floor:** sound loop tripped; transport
floor: disjoint file sets per seat; P1 first (the log and the crate move under everyone else).

**Done condition (fixed):** no schema file ships in the plugin (`plugins/mochiko/schemas/` gone, no
`skills/*/schema.yaml`); every delivery the deleted files served has a CLI form — rules (done),
templates (`mochiko-cli template`), shelf and registry documents (`mochiko-cli doc`, new); the 32
two-arm sites and every remaining `plugins/mochiko/schemas/` reference in shipped text point at the
CLI; the derived views live at `.mochiko/schema-views/` with the CI view ≡ replay check retargeted;
the three Python checkers and their tests are gone; the provenance sidecar is frozen to the archive;
the contract suite's no-Read assert is run-wide and green on a full run; every audit PASS; the
governance PATCH v3.0.3 strikes the transition clause and re-keys `primitive-edits.md`; the wave
lands as `plugin.json` 0.107.0 with the landing ritual complete — and the six-wave build item closes.

---

## 0. Inventory (lead, 2026-09-05, at `62aa99d`)

- **Shipped snapshot files:** 20 under `plugins/mochiko/schemas/` (6 command schemas · `common` ·
  `command-labels` · `skill-review-common` · `skill-authoring-common` · `skill-labels` · 7
  templates · `architecture-store` (a template — `mochiko-cli template architecture-store` renders
  it today) · `architecture-shelf-backend` (a shelf document — no CLI form yet)) and 30
  `skills/*/schema.yaml`. 272 KB. Nothing converted reads them; the log carries every document.
- **Two-arm and schema-path sites in shipped text:** in **rule texts** (log content — a migration):
  `impl` (the `${tasks_schema}` sentence), `spec` ×3, `setup` ×2, `arch` ×1,
  `authoring-common.two-arm-template`, `analysis-codebase`, `authoring-feature-map`,
  `authoring-technical-requirements`, `patterns-architecture-shelves`, `patterns-vertical-tdd`,
  `review-plan-artifacts`; in **`vars:`** (a migration): `tasks_schema` (implement, feature),
  `spec_schema` · `feature_entry_schema` · `features_index_schema` (specify), `store_schema` ·
  `shelf_schema` (architecture). In **bodies** (primitive edits with strips): `commands/specify.md`
  Goal (line 48); the router `skills/mochiko/SKILL.md` lines 21 · 57 · 58 (the phantom
  `template architecture-shelf-backend`) · 74 · 97; `skills/analysis-codebase/references/CONTEXT-GATHERING.md:10`;
  `skills/validation-constitution/references/QUALITY-CHECKLIST.md:6`;
  `skills/authoring-constitution/references/INTERROGATION-AGENDA.md:9`;
  `skills/authoring-technical-requirements/references/ARTIFACT-TEMPLATES.md:214`;
  `skills/review-plan-artifacts/references/ARTIFACT-CHECKLISTS.md:104`;
  `templates/report-format.md:78` (a file citation of `impl.escalation-batching`). Header comments in
  the snapshot files vanish with the files.
- **Crate surfaces bound to the schemas directory:** `src/views.rs:69` (view path mapping),
  `tests/views.rs` (semantic-equality and layout tests against the shipped files), `tests/render.rs:19`
  (`shipped_state` from the schemas dir), `tests/validate.rs:956` (corpus read), a comment in
  `tests/matrix_similar.rs:950`; `src/genesis.rs` reads a `--root` (the frozen fixture) and is
  untouched.
- **Python:** `scripts/check-command-schema.py` · `check-skill-schema.py` · `find-similar-rules.py`
  and their three tests retire; `scripts/similar-rules-allowlist.yaml` stays (the crate's detector
  reads it). The repo skill `.claude/skills/converting-command-to-schema/` is superseded.
- **Sidecar:** `.mochiko/provenance.yaml` (57 KB) — anchors live on the log's rules since genesis.
- **Governance mentions of the transition clause:** `CLAUDE.md` lines 74 and 119; ledger lines 52,
  384, 407; `rust-cli.md` line 25; `primitive-edits.md` `paths` (`schemas/**`, the sidecar), the
  "Schema data files" paragraph, criteria 9 and 10 in both blocks, criterion 11's `common.yaml`
  co-Read, skill criteria 1 and 6's family-common mentions.
- **Wave-5 follow-ups folded in:** a skill variant of the legend; three dense-five bodies saying
  rules live "in the schema"; a name sanitizer in the hook; both D13 checkers retire (above).

## 1. Scope and ownership

| seat | owns | delivers |
|---|---|---|
| **P1 — crate + log** | `crates/mochiko-cli/**`, `plugins/mochiko/migrations/**` (migration `0003`, README), `.mochiko/schema-views/**` (new, emitted), `.github/workflows/ci.yml` (filter: `.mochiko/schema-views/**`; drop `plugins/mochiko/schemas/**`) | `mochiko-cli doc <name>` (§2.1); the skill legend variant (§2.2); views relocated and every test re-pointed (§2.3); migration `0003-two-arm-to-cli.yaml` (§2.4) |
| **P2 — plugin side + rules file** | the 50 snapshot files (deleted), the 12 body sites, the three dense-five bodies, `plugins/mochiko/hooks/scripts/dependency-halt.sh` (sanitizer), the strip files touched, `.mochiko/strips/README.md`, `.claude/rules/mochiko/primitive-edits.md` (the wave-6 re-key, lead wording in §3.4), `.mochiko/provenance.yaml` → archive, `.claude/skills/converting-command-to-schema/` (deleted), `README.md`, `scripts/` (the six Python files deleted) | the end-state plugin tree; the body edits with strips; the rules-file re-key; the retirements |
| **P3 — contract suite** | `evals/contract/**` | the run-wide no-Read assert (§4.1); the `deliverables` host case (§4.2); the full run (§4.3) |

P1 first — its migration changes rendered rule texts and its `doc` command is what the router
rows and the shelf rule cite; P2 and P3 write after P1 closes (P2's deletions must not precede the
views relocation, or the semantic-equality test loses its comparand). Nothing else moves: commands'
and skills' Rules sections, `hooks.json`, the templates directory.

## 2. P1

### 2.1 `mochiko-cli doc <name>`
Renders a non-rule document from the replayed state — the `Shelf` kind (`architecture-shelf-backend`)
and the two label registries (`command-labels`, `skill-labels`) — as YAML through the views writer,
wrapped like every delivery: head line `mochiko-cli doc <name> · binary <v> · grammar <g> · plugin <p>`,
the document, end line `mochiko-cli doc end · <name>`. Exit 2 with the available names on an unknown
name; templates stay under `template` (a `doc` request for a template names the right command in
its error). Tests: each shelf and registry renders with both lines; unknown name; a template name
redirects.

### 2.2 The skill legend
Skill preambles print a skill variant of the legend: the `kind:` line lists the kinds a skill
schema may carry (no `fail`), and the `enforces:` lines are absent. Command preambles unchanged.
Goldens on `review-brainstorm` and `implement`; the legend size pin becomes two pins.

### 2.3 Views relocate
`views emit` writes under `.mochiko/schema-views/` with a layout by kind — `commands/<cmd>.yaml` ·
`common/<name>.yaml` (`common`, `skill-review-common`, `skill-authoring-common`) · `labels/<name>.yaml` ·
`skills/<skill>.yaml` · `templates/<name>.yaml` · `shelves/<name>.yaml` — the mapping in
`src/views.rs`. P1 emits the views once and commits them; `tests/views.rs`'s semantic-equality test
now compares the emitted view of every document against the **committed view file** (drift = a
failing test; the CI gate 5 "view ≡ replay" keeps its meaning), the layout test asserts the new
paths, and the header test compares against the committed command view. `tests/render.rs`'s
`shipped_state` and `tests/validate.rs`'s corpus read come from the log replay (`replay::load` of
`plugins/mochiko/migrations`), never a directory of files. `.gitignore` untouched (the views are
committed, human-readable, never shipped — they live outside `plugins/`).

### 2.4 Migration `0003-two-arm-to-cli.yaml`
Anchor `2026-09-03 cli-schema-delivery D9` (some reworded rules are floors or anchored — the
header anchor is required). Changes: `set-var` ×6 — `tasks_schema` → `mochiko-cli template tasks`
(implement and feature), `spec_schema` → `mochiko-cli template spec`, `feature_entry_schema` →
`mochiko-cli template feature-entry`, `features_index_schema` → `mochiko-cli template
features-index`, `store_schema` → `mochiko-cli template architecture-store`, `shelf_schema` →
`mochiko-cli doc architecture-shelf-backend`; `reword-rule` on every rule whose text carries a
two-arm phrase or a `plugins/mochiko/schemas/` path (§0's list — P1 finds the rule ids by grepping
the replayed state, not the snapshot files), with the **minimal rewording rule**: the phrase
"…, or Read `<path>` raw when the binary is absent …" (and its variants) becomes "…, delivered by
`mochiko-cli template <name>`" (or `doc` for the shelf), the sentence otherwise byte-identical; no
rule text may still name `plugins/mochiko/schemas/` or "when the binary is absent" after replay
(a test asserts it over the whole state). `migrate stamp`; `migrate validate` 0 rejecting; the
README's op table unchanged; its snapshot paragraph (wave 4) rewritten: the snapshot files are
gone, the views are the human-readable projection at `.mochiko/schema-views/`, regenerated by
`views emit --out .mochiko/schema-views`.

## 3. P2

### 3.1 Deletions
`git rm` the 20 files under `plugins/mochiko/schemas/` (the directory goes) and the 30
`skills/*/schema.yaml`; `git rm -r .claude/skills/converting-command-to-schema/`; `git rm` the six
Python files under `scripts/` (keep `similar-rules-allowlist.yaml`); `git mv .mochiko/provenance.yaml
.mochiko/archive/provenance-frozen-2026-09-05.yaml` with a two-line header comment (frozen at the
wave-6 landing; anchors live on the log's rules; provenance queries only).

### 3.2 Body sites (strips per primitive, `[v0.107.0]`)
The twelve §0 body sites: each two-arm or schema-path phrase becomes the CLI form
(`mochiko-cli template <name>`, `mochiko-cli doc architecture-shelf-backend`), the sentence
otherwise unchanged; router row 58's phantom `template architecture-shelf-backend` becomes `doc`;
router line 21's "obligated first read of the command's own `plugins/mochiko/schemas/<cmd>.yaml`"
becomes "the command's own rules rendered at fire by `mochiko-cli`"; `templates/report-format.md:78`
cites `impl.escalation-batching` by id alone. The three dense-five bodies (`analysis-codebase`,
`testing-end-user`, `testing-gap-finding` — V2 named them; P2 confirms by grep) reword "in the
schema" to "delivered by `mochiko-cli`". One strip entry per touched primitive (supersession by
ruling, D9 wave 6), Content verbatim.

### 3.3 Hook sanitizer
`dependency-halt.sh`: after `bare` is derived, reject anything outside `[A-Za-z0-9_-]` with a silent
exit 0 (a traversal-shaped name never reaches a path). Host matrix as at the wave-5 rework; V2 delta.

### 3.4 `primitive-edits.md` — the wave-6 re-key (pre-authorized PATCH v3.0.3; lead wording, P2 applies)
- `paths`: drop `plugins/mochiko/schemas/**` and `.mochiko/provenance.yaml`.
- The "Schema data files" paragraph becomes: *Schema content — every command's and skill's rules,
  the family common blocks, the label registries, the artifact templates, the shelf data — lives in
  the migration log at `plugins/mochiko/migrations/` and is delivered at fire by `mochiko-cli`
  (`cli-schema-delivery` D1–D3, D9 wave 6). No schema file ships. Editing schema content is a new
  migration file under the log (grammar in the log's README), validated by `mochiko-cli migrate
  validate`; the migration carries its ruling anchor where the hard set demands one, and the
  verbatim prior content is in the log by construction, so schema-content edits take no strip
  entry. The human-readable projection is `.mochiko/schema-views/`, regenerated never hand-edited.*
- Criterion 9 (both blocks): the deterministic pre-pass is `mochiko-cli migrate validate --report
  --plugin-root plugins/mochiko` cited in the audit brief (the Python checkers retired at v0.107.0).
- Criterion 10 (command block): anchors live on the log's rules (a supersession or tombstone of
  protected content carries its anchor in the migration; the binary enforces it); the sidecar is
  frozen at `.mochiko/archive/`.
- Criterion 11's closing co-Read clause and skill criteria 1 and 6's family-common mentions: the
  render resolves every stub — no raw common-file Read exists to demand; the converted clauses
  become the only form (the unconverted branches are struck — no unconverted primitive remains).
- The v3.0.1/v3.0.2 "on a converted command/skill" prefixes become unconditional (all are converted).

### 3.5 `.mochiko/strips/README.md` and `README.md`
Strips README: a note that schema content (the former `schemas/*.yaml` and `skills/*/schema.yaml`)
is recorded by the migration log, not by strip entries, from v0.107.0. README: the sentence that
the plugin ships no schema file, and `mochiko-cli doc` in the usage block.

## 4. P3
- **4.1 Run-wide no-Read:** the assert now fails on any Read or shell read of a `.yaml` under any
  `schemas/` directory or any `schema.yaml` anywhere in the run, every case, not per primitive; and
  a host check that no rendered rule of any primitive contains "when the binary is absent" or
  `plugins/mochiko/schemas/`.
- **4.2 `deliverables` host case:** every template name renders through `template` and `--check`,
  every shelf and registry through `doc`, each with head and end lines, exit 0; the fixture
  probe-plugin unchanged.
- **4.3 Full run:** host cases · the two fixture cases · the brainstorm mechanism cases · delivery ×1
  and absence ×1 per primitive (36 + 36) · preload — ≈ 80 sessions, once, after P2 closes. The
  read-back stays recorded. Report per primitive: criterion (1); per family: unchanged from wave 5
  (the two-arm rewords move a few rule texts by a few bytes — measure, do not assume).

## 5. Governance PATCH v3.0.3 (lead, at landing, pre-authorized)
Transition clause struck: `CLAUDE.md` lines 74 and 119 (the clause sentence and the pointer's
parenthetical); ledger GI-020 clause (line 384: replaced by "expired at v3.0.3 / plugin v0.107.0 —
no schema file ships; the contract suite's run-wide no-Read assert holds it"), Testability row
(line 407 stays true and is now live), amendment-log row 3.0.3; `rust-cli.md` line 25; the region
stamp v3.0.3; `primitive-edits.md` per §3.4 (P2). The three delivery watches (`BACKLOG.md` lines
50 · 83 and the template-schema CLI watch) move to the trail as superseded by D10.6.

## 6. Landing (lead)
`plugin.json` 0.107.0 · `marketplace.json` · `CHANGELOG.md` · `DECISIONS.md` row · `BACKLOG.md`
(the six-wave build item → DONE → the trail; the follow-ups that survive listed once) · `ROADMAP.md`
(the Template-schema CLI Next row closes) · record "Wave 6 — end state landed" + the Landed section ·
index · plugin size before/after · kinako note. Audits: V1 (crate + log + views), V2 (plugin edits,
deletions, the rules-file re-key, the sanitizer), V3 (suite; independent subset re-run).

## 7. Standing assumptions (lead; say the word to change any)
- The command is named `doc`; templates stay under `template`.
- The views layout is by kind under `.mochiko/schema-views/`, committed, outside `plugins/`.
- The 50 deletions take one strips-README note, not fifty entries: the log is their record (D2).
- The converter skill is deleted, not marked; git history and the DECISIONS row keep it.
- Version 0.107.0 and no halt trim, as ruled.
