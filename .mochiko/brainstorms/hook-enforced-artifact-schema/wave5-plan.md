# Wave 5 — the violator pass (plan; awaiting lead approval)

**Author:** QA seat · **Date:** 2026-09-15 · **Ruling home:** record D11 wave 5 as amended (V5:
pass → upgrade → dogfood) · D10 watch · wave-3 R1/R4 and the seat's calls · AM-3 review C10 (this
repo's pass before the local wave-4 install) · the pending C1 table amendment.
**Nothing is moved, split, or committed until the lead opens it. I run no git mutation.**

## 1. The inventory — script and counts

`…/scratchpad/wave5/inventory.py` (111 lines; not under `evals/`). One synthetic `Write` per
existing file under `.mochiko/**` plus the root docs, fed to `mochiko-cli check --hook-json -`:

```python
{"hook_event_name":"PreToolUse","tool_name":"Write","cwd":empty,"tool_input":{"file_path":rel,"content":content}}
```

`empty` is a fresh temp directory, so the candidate has no baseline and D4e's amnesty never
fires; against the real repo root almost every row would allow and the inventory would measure
nothing. The measure is read off the binary's own sentences: "not a declared sub-directory" is
path, "not a declared deliverable" is set, the bound and budget sentences are size, the rest
shape. Both runs resolve against this tree's `0005` log — what will gate kinako after its
upgrade, not the 0.103.0/0.108.0 copies installed today, neither of which carries it. No deny
fell on a root operating doc in either repo, which is R6 holding.

| repo | files | allow | deny | path | set | shape | size |
|---|---|---|---|---|---|---|---|
| mochiko | 474 | 331 | **143** | 69 | 14 | 2 | 58 |
| kinako | 499 | 139 | **360** | 202 | 12 | 32 | 114 |

## 2. What the pass must fix, and what amnesty already covers

**Path is the only non-relaxable measure.** `Resolution::UndeclaredSubdir` denies unconditionally;
every other resolution routes through `settle`, which excuses a fault whose key already stands in
the baseline. So the 271 path violators are wedged, every future write to them denying forever,
while the 216 set, shape and size violators stay editable the day the gate goes live with the
overage named in `additionalContext`. The **mandatory** scope is the path half.

**A move is a fresh write at the target path**, carrying no baseline, so it must conform in full:
pre-gate nothing checks it and the file lands as-is, post-gate the move must bring frontmatter,
headings and budgets with it. That is why V5 put the pass first. **Amnesty is also per fault
key** — an existing over-budget file stays editable, but a *new* over-budget section in it is a
new key and denies, so kinako's largest deliverables are frozen in shape, not tolerated.

## 3. Move list — the re-home half (does not wait on the table ruling)

Mochiko 83 files, kinako 214 — 297 in all.

| repo | from | n | to | ruling |
|---|---|---|---|---|
| mochiko | `<session>/wave{1..6}-reports/` | 64 | `<session>/reports/` | R4 |
| mochiko | `<session>/wave0-fixtures/` | 5 | `<session>/research/wave0-fixtures/` | R4; nesting verified allowed |
| mochiko | `<session>/{run-costs,census*,conversion-inventory,implement-rewrite}.md` | 10 | `<session>/research/` | set violators |
| mochiko | `<session>/{review,review-lens-a,review-lens-b}.md` | 4 | `<session>/reports/` | `reviews/` → `reports/` |
| kinako | `features/FEAT-00{1,2}/reports/evidence/**` | 161 | **no route exists** — §7 | unruled |
| kinako | `{EPIC-001,EPIC-002,FEAT-006}/reviews/` | 27 | sibling `reports/` | `reviews/` → `reports/` |
| kinako | `features/B53/` | 9 | owning feature's `reports/` | R1 |
| kinako | `features/B61/` | 1 | same | **R1 names only `B53`** |
| kinako | `epics/EPIC-001/landing/` | 4 | unruled | `landing/` forbidden |
| kinako | `brainstorms/sandbox-runner-and-credential-ux/review-*.md` | 9 | `reports/` | set violators |
| kinako | `features/desk/2026-09-09-sandbox-runner/review.md` | 1 | desk `reports/` | set violator |
| kinako | `{decisions,features}/.gitkeep` | 2 | delete | undeclared name, no content |

**Pointer fixes.** Old-path mentions in records, indexes, reports, DECISIONS and BACKLOG: mochiko
**204** across 64 files (`wave5-` and `wave3-reports/` 39 each, `wave4-` 35, `wave1-` 31,
`wave6-` 30, `wave2-` 20, `wave0-fixtures/` 10); kinako **143** (`/landing/` 57, `/reviews/` 51,
`reports/evidence/` 21, `features/B53` 14). Each is a `sed` plus a re-read of every touched index.
Separately, 9 of 11 sampled wave reports carry no `report:` field; moved as-is they land
permanently amnestied and invisible to the D9 sniff, and adding it is free only in this window.

## 4. Size split — the half that waits on the C1 ruling

Under the pending numbers (record/synthesis unbounded · pipeline deliverables and wave files 300 ·
entry-class 150), mochiko's size half nearly evaporates and kinako's does not:

| repo | size denies today | survive the amendment | what survives |
|---|---|---|---|
| mochiko | 58 | **6** | 4 wave files 350–1,135 · `build-log.md` 469 · one report section 20/15 |
| kinako | 114 | **102** | 67 report-section · 33 whole-file 300–4,744 · 2 log entries |

A wave file splits by wave into `research/`; a report over budget re-sections under
`extra_headings: allow`; a 4,744-line deliverable is a rewrite, not a split. **My recommendation:
the pass rewrites none of them** — amnesty holds every one, the cost is a corpus that never
conforms, and that trade is the user's. `build-log.md` fits none of the three proposed rows,
being neither record nor wave file, and needs naming in the ruling.

## 5. Order of operations

**mochiko** — branch; moves, pointer fixes and frontmatter as one commit per class; re-run the
inventory expecting `path 0 · set 0`; lead commits. Proposed messages: *"Wave 5: re-home brainstorm
wave reports under reports/"*, *"… session working files under research/"*, *"… repoint the 204
moved-path references"*.

**kinako** — same shape, after the mochiko pass and the §7 rulings. Its plugin resolves through a
**project-scoped 0.103.0** pin, not the user-scoped 0.108.0, so the mochiko upgrade does not gate
it; both entries verified in `installed_plugins.json`. The check before its pass opens is that the
pin still reads pre-gate. **The upgrade** is V5 step 2 and not mine — it follows the AM-3 wave-4
precondition, so the sequence is mochiko pass → bump → install → kinako pass → upgrade → dogfood.

## 6. D10 watch instrumentation for the dogfood run

Five figures, all from the transcript and the tree, none needing a hook change: **denies by kind**
(each deny's own sentence, the §1 classifier reused); **section sizes before/after** and
**per-home total volume** (per-`##` spans and `wc -l` by home at open and close, I7's displacement
watch); **reports-per-run** (files created under any `reports/`); **injections per run**
(`hook_additional_context` attachments in the run's sidechain transcripts, the wave-4 method).
Re-key trigger is D10's: a deny rate still high on the second run is the table's fault.

## 7. What needs the user

1. **The 161 kinako evidence files** (154 `.txt`, 3 `.jsonl`, 2 `.json`, 2 `.md`). Verified:
   `reports/` rejects a nested directory and requires `report:` frontmatter a `.txt` cannot carry,
   and the feature home declares only `contracts/` — **no legal location exists inside
   `.mochiko/`**. Declare an `evidence/` subdir (a migration and a release), move the tree out of
   `.mochiko/` (allowed today, my recommendation), or delete it. Blocks 161 of 214 kinako moves.
2. **`B61`** — R1 forbade `B53`-class ids by name; `B61` is the same class, unnamed.
3. **`epics/EPIC-001/landing/`** — forbidden, no target ruled.
4. **The C1 amendment**, including where `build-log.md` sits, and whether the pass rewrites
   kinako's 102 survivors or leaves them to amnesty.
5. **kinako is the user's consumer repo.** Every move there is theirs to authorize or run.
