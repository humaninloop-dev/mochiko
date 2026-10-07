---
report: review
pass: cold
stage: blind-angle-map
map_timing: end
record_contact: none
source: frame lines only (problem, destination, out-of-scope) plus free repo grounding; session artifacts and the brainstorms index fenced
angles: 48
load_bearing: 19
---

## Angle map

LB = load-bearing (a ruling on it would likely change the build). Grounding cites are repo facts checked at map time.

### A. Scope — which IDs, which surfaces

- A1 LB · ID-family list. The frame names GI and D ("IDs like"); the repo carries ~30 families (FEAT-, US-, FR-, SC-, AX-, AM-, NFR-, SCR-, FLOW-, EPIC-, ADR-, INC-, OQ-, IP-, DS-, WS-, ...). The record must list in-scope and out-of-scope families, with a reason for each exclusion.
- A2 LB · Surface reach. Mochiko-repo records only, or the plugin-shipped grammar too? Setup emits `GI-0XX` and `<!-- GI-XXX -->` markers into user CLAUDE.md and ledgers (migrations 0001, ~l.11000-11008), and the brainstorm flow mints D-cards in user repos.
- A3 · Product-side IDs the plugin mints for user products (FR/SC/FEAT/AX/US): in or out, and why the frame's problem does not apply equally to them.
- A4 · Session-prefixed decision forms (`PO-D1`, `CS-D`, `AD-D`, `ER-D`) and qualified refs (`cli-schema-delivery D11`): in scope as D IDs, or prefix forms excluded by the no-prefix-rename line.
- A5 · Sub-IDs: lettered `D1a`/`D2a` (accepted by the migration anchor grammar) and dotted `D10.1`-`D10.7`, `D4.1`. Where the slug attaches, and whether each sub-ID gets its own.
- A6 · Non-record surfaces: commit messages, PR bodies, branch names, CHANGELOG, crate doc comments (21 crate files cite GI IDs), YAML migration anchors, contract-suite fixtures. In or out, per surface.

### B. ID form — the grammar

- B1 LB · Number padding. The user's example `GI-01` conflicts with live `GI-004` and template `GI-0XX`. "Keeps its number" needs an explicit pad-width ruling.
- B2 LB · Parse boundary. In `GI-004-must-use-logging` the hyphen separates prefix, number, and slug words. Needs a stated regex: slug words never all-digit and never digit-led, and no confusion between `D2a-x-y-z` (sub-ID) and `D2-a-...` (slug).
- B3 · Word count (exactly three vs up to three), charset (lowercase ASCII, digits allowed?), stopwords, and maximum length.
- B4 · Strength in the slug. The example leads with `must-`. If the slug encodes RFC 2119 strength, a MUST-to-SHOULD amendment falsifies it.
- B5 · Relation to existing titles. Ledger headings already carry Title Case names (`### GI-004 — Primitive Audit Ratchet`). Is the slug derived from the title, or independent of it?

### C. Slug lifecycle

- C1 LB · Stability vs meaning drift. GI-020 was superseded by ruling and kept its ID. A slug that tracks meaning forces a repo-wide reference rewrite on every amendment; a frozen slug drifts into a false label. The record must pick one and state the consequence.
- C2 LB · Canonical key. Either the number alone stays authoritative (slug decorative, mismatch tolerated or flagged) or the full string is the key. This drives grep, lookup, and drift handling.
- C3 · Slug source: derived from the definition title or D-card headline, or freshly minted. Titles longer than three words (e.g. "Secrets Out of the Repo") need a compression rule.
- C4 · Minter and checker: who mints at creation (setup synthesis, brainstorm lead) and who checks slug quality. Is a grading leg needed, or does a minting rule in the owning skill suffice?
- C5 · Uniqueness. Within a family (two GIs with the same slug) and across sessions: D7 slugs can collide between records, so the slug does not make a D ID unique and qualification is still owed.
- C6 · Retired, superseded, or tombstoned IDs: does a dead ID keep its slug, and does it carry a supersession marker?

### D. Use-site policy

- D1 LB · Where the full form is required: every mention, definition sites only, first mention per document, or headings only. This sets the size of the rewrite and the readability outcome.
- D2 LB · Compound references. Ranges (`D1–D8`, `PO-D1–D7`), slashes (`D4/D10`), and lists (`GI-004, GI-005`) number ~1,650 range/slash hits in repo markdown. No slugged form is defined for any of them.
- D3 · Dense contexts: tables, HTML-comment markers (14 `<!-- GI-... -->` in CLAUDE.md, invisible when rendered), migration anchors. Exempt, or a short form allowed?
- D4 · Cross-session qualified form: `cli-schema-delivery D11` becomes `... D11-<slug>`, so the rewriter must resolve the slug from another record.
- D5 · Length budgets. CLAUDE.md is loaded every turn (token cost), skill `description` is capped at 1,536 chars, report sections are capped at 15 lines, and build-log entries at 60 lines. Slug inflation must fit these caps or be ruled exempt.

### E. Mechanical surfaces

- E1 LB · Migration-log anchors. The grammar `YYYY-MM-DD <session-slug> [D#]` is enforced by the crate (`is_anchor`; malformed-anchor errors at `validate.rs:1403`, `replay.rs:577`), and the anchor sits inside the canonical hash (`migrations/README.md:58,80`). Slugged anchors need a crate change. Rewriting old anchors means rehashing history. Leaving them bare is a sanctioned exception. The record must rule which.
- E2 LB · Shipped rule text that cites GI/D IDs lives in the hashed migration log. Rewording it means new anchored migrations, not in-place edits, under the derived-view-equals-replay gate.
- E3 · A crate change means a new `mochiko-cli` release, binary-range coordination with the plugin, and rust-cli review. The wave-4 supply-chain-control precondition history applies.
- E4 · Contract suite. Six `evals/contract/` files match literal `GI-0` / `D<n>` strings (`expected-skills.json`, `freeze_expectations.py`, fixtures). Expectations must be re-frozen and gate 6 green.
- E5 · Write-time enforcement. If the new grammar is enforced by a hook or check, that is kernel-class and needs a GI-019 ruling. If it is an advisory checker, the record should say so. (`record.md` has no template today, so its card IDs are unchecked.)
- E6 · Grep ergonomics. A bare `GI-004` still prefix-matches the slugged form. Today `D1` also matches `D10`; the slug adds a `D1-` boundary. Does any skill instruction or tool that greps exact bare IDs break?

### F. Corpus migration and record integrity

- F1 LB · Frozen surfaces: `.mochiko/archive/*` (`provenance-frozen-2026-09-05.yaml`: "never read, written, or amended"), closed records, strip history, CHANGELOG. Rewriting them touches protected content; exempting them leaves a mixed corpus forever (GI-005, GI-006).
- F2 LB · Protected content. `KEPT:` lines, DECISIONS-traceable lines, and record protected sets: is a format-only rewrite a removal/supersession needing strips plus a supersession-by-ruling, or ruled a non-semantic exemption? An unruled mass edit reads as a regression in the audit's preserved-responsibilities check.
- F3 · Rewrite mechanism: scripted one-shot or manual, an ID-to-slug registry as the single source, a dry-run diff, and an author-other-than-grader check on the rewrite.
- F4 · Ordering and scale. Every D ID in every session needs its slug before the cross-refs can be rewritten. Scale: ~17.8k `D<n>` hits and ~3k `GI-` hits in repo markdown.
- F5 · Mixed-state window: how long a half-slugged corpus is tolerated, and the wave plan.
- F6 · Git history. Provenance queries (`git log --all -- ...`) stay bare. Accepted?

### G. Governance and landing

- G1 LB · Governance event. A GI format change touches the ledger and the setup-regenerated CLAUDE.md region. Is it a `/mochiko:setup` amend or a direct edit? If the region template is not updated, the next regeneration strips the slugs.
- G2 · Each shipped-primitive edit needs a strip, an author-other-than-grader audit per unit, a `plugin.json` bump, a CHANGELOG entry, and marketplace sync (GI-004, GI-012).
- G3 · Version semantics: is a breaking ID grammar a MAJOR bump? Stated?
- G4 · Landing ritual: DECISIONS row(s), backlog item, roadmap placement, and a GLOSSARY entry for the ID grammar.
- G5 · Sound-loop leg: the rewrite of governing surfaces needs a producing seat other than the lead and a non-author review.

### H. Downstream projects

- H1 LB (conditional on A2 = plugin-wide) · Existing user projects with bare ledgers and records (kinako). What "no backward compatibility" means in practice: reject, ignore, or auto-migrate? Is there an amend path?
- H2 · The setup synthesis template already carries `GI-0XX — [Intent name]`. Is the slug minted from the user's intent name, and does the user approve it?

### I. Value, alternatives, excess

- I1 LB · Does a three-word slug cure "must look it up"? It is a lossy summary. An LLM seat may act on the slug's paraphrase instead of the clause, NON-NEGOTIABLE principles especially. What evidence or test shows the gain?
- I2 LB · Steelmen for the rejected roads must be recorded, each with why it lost:
  - (a) Titles already exist at the definition site; print the title on first mention only.
  - (b) A glossary or index lookup.
  - (c) A `mochiko-cli` lookup/explain verb.
  - (d) An inline parenthetical gloss such as `GI-004 (primitive audit)`, with no grammar change.
  - (e) Render-time expansion.
- I3 LB · Excess. Weigh ~20k occurrences, a crate change, and new migrations against a cheaper shape: prospective-only for new IDs plus definition sites, with history left bare.
- I4 · Reader asymmetry. The user benefits most; seats can grep. The token cost is paid by every seat on every load.

### J. Durability and done condition

- J1 LB · Regression guard. After landing, what stops new bare IDs: a skill rule, an advisory checker, or a hook (which needs a kernel ruling)? Without one, the corpus decays back.
- J2 LB · Done condition. It must be measurable, e.g. zero bare in-scope IDs outside a named allowlist by a named check, with a named runner.
- J3 · Rollback path if the slugs prove noisy.
