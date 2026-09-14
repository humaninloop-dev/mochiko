# P2 — end-state plan (plugin side + rules file), wave 6

**Status: approved 2026-09-05, all three findings ruled in scope. Execution waits on the lead's "P1 closed".** Report per family of change lands at `wave6-reports/p2-endstate.md`. No commits.

## 0. Inventory confirmed, three findings ruled in

Grep confirms 20 files under `plugins/mochiko/schemas/` and 30 `plugins/mochiko/skills/*/schema.yaml`. The only shipped `.md` files naming `plugins/mochiko/schemas/` or the absence phrase are the twelve §0 body sites plus the migrations README, which is P1's. No thirteenth site exists. The lead's rulings widen the wave beyond plan §3.2:

1. **All seven dense-five sentences are reworded**, the grep union winning over both the plan's list and the wave-5 V2 note: `analysis-codebase` (65, 69), `executing-tdd-cycle` (55, 61), `testing-end-user` (92, 98), `testing-gap-finding` (94). Four bodies. Detail in §3.
2. **Router lines 20 and 23 are reworded to the end-state truth**, as D9's doc landings cover a sentence that becomes false when no schema file ships. Detail in §2.
3. **The five other bodies saying rules live "in the schema" take the same minimal rewording**, one strip entry each, same class and same wave. Detail in §3.

## 1. Deletions (§3.1)

```
git rm -r plugins/mochiko/schemas
git rm plugins/mochiko/skills/*/schema.yaml
git rm -r .claude/skills/converting-command-to-schema
git rm scripts/check-command-schema.py scripts/check-skill-schema.py scripts/find-similar-rules.py
git rm scripts/test-check-command-schema.py scripts/test-check-skill-schema.py scripts/test-find-similar-rules.py
git mv .mochiko/provenance.yaml .mochiko/archive/provenance-frozen-2026-09-05.yaml
```

`scripts/similar-rules-allowlist.yaml` stays; untracked `scripts/__pycache__/` I remove from the working tree separately. The moved sidecar gains two header lines above its existing first line: frozen at the wave-6 landing (v0.107.0, `cli-schema-delivery` D9 wave 6), anchors now live on the log's rules and the binary enforces them, provenance queries only.

## 2. The twelve body sites (§3.2)

Each is one phrase substitution under the minimal rule, the sentence otherwise byte-identical. "Reflow" means rewrapping that paragraph to its existing column width with no other word changed. Backticks are omitted below for readability; the edits keep the originals' code spans exactly.

- `commands/specify.md:47` — `(rendered by mochiko-cli template spec, or its schema plugins/mochiko/schemas/spec.yaml Read raw when the binary is absent — the shipped schema is the first-class source of truth)` becomes `(rendered by mochiko-cli template spec)`; reflow.
- `skills/mochiko/SKILL.md:21` — `an obligated first read of the command's own plugins/mochiko/schemas/<cmd>.yaml` becomes `the command's own rules rendered at fire by mochiko-cli`.
- `skills/mochiko/SKILL.md:57` · `:74` · `:97` — drop `, or Read plugins/mochiko/schemas/<name>.yaml raw when the binary is absent`, for architecture-store · spec · tasks respectively.
- `skills/mochiko/SKILL.md:58` — `mochiko-cli template architecture-shelf-backend, or Read plugins/mochiko/schemas/architecture-shelf-backend.yaml raw` becomes `mochiko-cli doc architecture-shelf-backend`. The phantom fix; depends on P1's `doc`.
- `analysis-codebase/references/CONTEXT-GATHERING.md:9` — drop `, or plugins/mochiko/schemas/codebase-analysis.yaml read raw`; reflow.
- `validation-constitution/references/QUALITY-CHECKLIST.md:5` — drop `; when the binary is absent, Read plugins/mochiko/schemas/governance-surfaces.yaml raw`, the sentence ending at `for the mirror-checklist view.`; reflow.
- `authoring-constitution/references/INTERROGATION-AGENDA.md:8` — drop `, or plugins/mochiko/schemas/governance-intent.yaml read raw`; reflow.
- `authoring-technical-requirements/references/ARTIFACT-TEMPLATES.md:213` and `review-plan-artifacts/references/ARTIFACT-CHECKLISTS.md:103` — drop `, or Read plugins/mochiko/schemas/architecture-store.yaml raw when the binary is absent`; reflow both.
- `templates/report-format.md:78` — `plugins/mochiko/schemas/implement.yaml's impl.escalation-batching` becomes `the impl.escalation-batching`, citing the rule by id alone.

Two further router lines under ruling 2. Line 20's `and every one of the six ships as a **.md + schema pair**` becomes `and every one of the six ships as a **.md** whose rule set lives in the migration log the plugin carries`; line 23's `The schema carries the rule set in six sections` becomes `The rule set is delivered in six sections`. Line 20 states the home and line 21 the delivery, so the pair reads home-then-render rather than repeating "rendered at fire by mochiko-cli" twice. Everything after "sections —" on line 23 is untouched.

## 3. The twelve reworded "in the schema" sentences

The delivery noun moves; every section id is kept and still resolves. Across the four dense-five bodies, seven sentences: `analysis-codebase:65` "live in the schema's scope and verdict sections" becomes "are delivered by `mochiko-cli` in the scope and verdict sections"; `analysis-codebase:69`, `executing-tdd-cycle:55` and `:61`, and `testing-end-user:92` take the same substitution over `analysis-codebase.sec.output`, `executing-tdd-cycle.sec.inputs`, `executing-tdd-cycle.sec.scope` and `testing-end-user.sec.verdict`, where "are the schema's X rules" becomes "are delivered by `mochiko-cli` as the X rules"; `testing-end-user:98` "source per the schema's `testing-end-user.gate-source-binding`" becomes "source per the delivered `testing-end-user.gate-source-binding` rule"; `testing-gap-finding:94` "live in the schema; the artifact looks like this" becomes "are delivered by `mochiko-cli`; the artifact looks like this".

Across the five bodies added by ruling 3, five sentences: `authoring-prototype:68` "(binding in the schema's artifact section)" and `authoring-feature-map:68` "(bindings in the schema's artifact section)" become "(binding[s] delivered by `mochiko-cli` in the artifact section)"; `patterns-code-minimalism:45` "the stop rule and every bound on the walk live in the schema:" and `patterns-plan-minimalism:46` "the stop rule, the rung scopes, and the read duty live in the schema:" become "… are delivered by `mochiko-cli`:"; `patterns-vertical-tdd:58` "live in the schema (`patterns-vertical-tdd.sec.discipline`)." becomes "are delivered by `mochiko-cli` (`patterns-vertical-tdd.sec.discipline`)."

## 4. Strip entries

One supersession-by-ruling entry per touched primitive, stamped `[v0.107.0]`, appended newest-first to sixteen existing files: `specify.md`, `mochiko.md`, `analysis-codebase.md`, `validation-constitution.md`, `authoring-constitution.md`, `authoring-technical-requirements.md`, `review-plan-artifacts.md`, `report-format.md`, `executing-tdd-cycle.md`, `testing-end-user.md`, `testing-gap-finding.md`, plus the five added by ruling 3 — `authoring-prototype.md`, `authoring-feature-map.md`, `patterns-code-minimalism.md`, `patterns-plan-minimalism.md`, `patterns-vertical-tdd.md`. All sixteen already exist. `analysis-codebase` takes one entry covering both its reference file and its body; `mochiko.md` takes one covering all seven router lines. Each entry carries `Disposition: superseded → the CLI form`; `Tier failed: n/a — supersession by ruling (.mochiko/brainstorms/cli-schema-delivery/record.md D9 wave 6, DECISIONS.md 2026-09-05 row)`; `Content:` the pre-edit sentence verbatim; `Kept deliberately:` the surviving half; `Consumers assessed:` on the router and `report-format` only. Each file gets one wave-context comment above its first `[v0.107.0]` entry naming the pre-edit commit for `git show`.

The 50 deleted schema files take no strip entries — the §6 strips-README note is their record, per plan §7. The hook sanitizer is a pure addition and takes none.

## 5. Hook sanitizer (§3.3)

In `plugins/mochiko/hooks/scripts/dependency-halt.sh`, after `bare` is derived in the `UserPromptExpansion` branch and after an equivalent `bare=${name#mochiko:}` added to the `PreToolUse` branch, one shared guard runs before any path is built:

```sh
# A name that is not a bare identifier never reaches a path.
case "${bare:-}" in *[!A-Za-z0-9_-]* | "") exit 0 ;; esac
```

POSIX `case` only, no new dependency. Silent exit 0 by design: a traversal-shaped name is not a mochiko primitive, so the hook has nothing to say about it.

## 6. `primitive-edits.md` re-key (§3.4) and the two READMEs

Frontmatter loses the `plugins/mochiko/schemas/**` and `.mochiko/provenance.yaml` entries; six globs stand. The "Schema data files" paragraph (19–28) is replaced whole by the lead's §3.4 wording. Criterion 9 in both blocks (138–140, 274–275) becomes the `mochiko-cli migrate validate --report --plugin-root plugins/mochiko` pre-pass, noting the Python checkers retired at v0.107.0. Command criterion 10 (141–144) and skill criterion 12 (283–287) restate anchors as living on the log's rules, binary-enforced, the sidecar frozen at `.mochiko/archive/`. Criterion 11's closing co-Read clause (163–167) and skill criteria 1 (196–209) and 6 (246–255) lose their unconverted branches: the "**On a converted command/skill**" sentence becomes the only text with its prefix struck, and the raw common-file Read demand goes with the branch. Every other "on a converted …" prefix — criteria 1, 2, 3 of both blocks and skill criterion 8 — becomes unconditional the same way. Two greps close it: neither `plugins/mochiko/schemas/` nor "On a converted" survives in the file.

`.mochiko/strips/README.md` gains a paragraph after the four entry-type sections: from v0.107.0 schema content — the former `plugins/mochiko/schemas/*.yaml` and `plugins/mochiko/skills/*/schema.yaml` — is recorded by the migration log at `plugins/mochiko/migrations/`, not by strip entries; the log carries the verbatim prior content by construction, so a schema-content edit takes no entry here, and the 50 files deleted at v0.107.0 are recorded by that log and the D9 wave-6 ruling.

`README.md` takes two additions: a sentence in the "What `mochiko-cli` serves" opening paragraph that the plugin ships no schema file, the log being the only rule data it carries; and `mochiko-cli doc <name>` in the usage block under `template --check`, commented as the shelf and label-registry documents. Line 48's template-name list is unchanged.

## 7. Checks

1. **Scripted sweep, blocking.** `grep -rn "plugins/mochiko/schemas/" plugins README.md .claude` and `grep -rniE "binary is (absent|missing)" plugins README.md` must return zero rows across all file types, not only `.md`. Run after each edit batch and once at the end.
2. **No survivors.** `ls plugins/mochiko/schemas` fails; the skill `schema.yaml` glob matches nothing; `scripts/` holds `similar-rules-allowlist.yaml` alone; the sidecar exists only under `.mochiko/archive/`.
3. **Hook matrix.** Re-run all eight wave-5 rows plus the safety rows against `evals/contract/fixture/hook-input/`, adding four sanitizer rows — `mochiko:../../etc/passwd`, `mochiko:foo/bar`, `mochiko:` with an empty bare, and the `PreToolUse` twin of the first — each expecting silent exit 0. The before/after transcript diff must show only those four new rows.
4. **Render survival.** For every touched skill, `mochiko-cli rules <skill> --section <id>` still renders 7/7 sections with head and end lines. My edits never touch the log, so this is a regression check.
5. **Router row 58, sequenced last.** `mochiko-cli doc architecture-shelf-backend` must exit 0 with both wrapper lines before that row counts as done. This is the only check needing P1's `doc` command, so it runs at the end of my batch.
6. **No "in the schema" survivors.** `grep -rn "in the schema\|the schema's" plugins/mochiko/skills plugins/mochiko/templates` must return zero rows naming a rule home. A hit describing an artifact's own schema section, such as a spec or OpenAPI shape, is legitimate and is listed in the report rather than edited.
7. **Audit hand-off.** V2 grades the fourteen body sites, the twelve rewordings, the deletions, the rules-file re-key and the sanitizer; the brief cites checks 1 and 6 and the char-budget pre-assert for the nine skill bodies whose payload changed.
