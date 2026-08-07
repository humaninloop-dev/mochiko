# Record — plugin-output-styles-delivery

**Status:** accepted (2026-08-07) · **When:** 2026-08-07 · **Session form:** lead-run `mochiko:analysis-iterative`, one question per turn · solo-cold-reviewed, 9/9 findings dispositioned (see Review).

## Topic

Deliver the caveman register as a **native Claude Code output style** shipped by the mochiko
plugin (per https://code.claude.com/docs/en/output-styles), and introduce a second style:
**caveman + BLUF** (user supplied a full BLUF style draft — US military staff-writing standard,
answer-first, plain words, `keep-coding-instructions: true`).

## Ground facts (checked 2026-08-07)

- **F1 — Plugin delivery is documented.** "Plugins can also ship output styles in an
  `output-styles/` directory." A manifest field **is** documented (amended at review, I1):
  `outputStyles` — "Custom output style files/directories (replaces default `output-styles/`)";
  replaces-default semantics, so omitting the field keeps the default directory scan.
  (`verified: code.claude.com/docs/en/output-styles` + `code.claude.com/docs/en/plugins-reference`,
  both fetched 2026-08-07; original "no manifest field documented" claim was false — claim scope
  had exceeded source scope.)
- **F2 — Frontmatter fields:** `name` · `description` · `keep-coding-instructions`
  (default `false`) · `force-for-plugin` (plugin-only: auto-applies style while plugin enabled,
  overrides the user's `outputStyle` setting; first-loaded wins across plugins). (same source)
- **F3 — Activation:** `/output-style` command was removed in v2.1.91; selection is `/config` →
  Output style, saved to `.claude/settings.local.json` (`outputStyle` field). Takes effect after
  `/clear` or new session. (same source)
- **F4 — Scope limit:** output styles apply to the **main conversation only** — subagents run
  their own system prompts. So a native style covers the lead↔user chat surface; mochiko's seat
  reports and artifacts are untouched by it. Exception (added at review, M1): a **fork**
  inherits the parent's full system prompt, styles included — grep across `plugins/mochiko/`
  found no fork-type subagent usage (all "fork" hits are metaphorical decision-forks), so the
  exception is currently vacuous here. (same source)
- **F5 — Existing home:** `plugins/mochiko/templates/output-style.md` (v1, 2026-08-01) is the
  single authoritative register home — per-surface levels (chat `full` · reports `ultra` ·
  artifacts `full`), delivered via the CLAUDE.md governance-region switch line (always-loaded)
  + Shape-5 `paths`-scoped rules file (Read-time). No native output style anywhere in the
  plugin today; no `output-styles/` directory exists.
- **F6 — plugin.json** (v0.54.0) lists `commands`, `skills`, `agents` — no output-styles entry.
- **F7 — The repo's own CLAUDE.md** carries a standing caveman block (levels off/lite/full/ultra,
  drop/never-compress lists, exemptions) — the in-repo operating instance of the same register.
  Birth-time divergence noted at review (M2): the block says **never announce** the style; the
  template says **disclose once** (first styled session names the style). The shipped styles
  follow the block's never-announce rule (ruled with I3) — `/config` selection is itself the
  disclosure, so disclose-once's job is already done by the picker.

## Decisions

### D1 — Coexistence: styles are an optional product surface; the switch line stays authoritative — `Confident`

Native output styles ship as user-selectable extras (`/config` → Output style); the
CLAUDE.md governance-region switch line remains the authoritative in-pipeline chat carrier,
and the reports/artifacts machinery is untouched. No `force-for-plugin` (would override the
user's chosen style — hostile for a public plugin); no retreat of the switch line (would make
the chat register opt-in and kill per-surface `off/lite/full/ultra` switching). Double-fire
when a user selects the style AND the switch line is live is accepted: same register, redundant
not contradictory. Ruled A over B (native-style-as-chat-carrier) and C (`force-for-plugin`).

*Amended at review (I2):* the accepted-risk note extends to **level mismatch** — a user
running the switch line at chat `lite`/`ultra` while the Caveman style (baked `full`, D4) is
selected has two levels commanded on one surface; the style side likely wins (docs: styles
trigger in-conversation adherence reminders). Accepted as-is, no winner ruled — the user
opted into the style from the picker, and either outcome is still the caveman register.

### D2 — Style files are free-standing product artifacts; no sync obligation to the register home — `Contested`

The `output-styles/` files are end-user packaging for people who pick a style from the
`/config` menu — not register carriers. Written once from the register (caveman) / the user's
draft (BLUF), then allowed to drift; `templates/output-style.md` stays canonical for the
pipeline register only, and gains no Bound-by line for the styles. Lead recommended A
(derived-copy stamp + Bound-by line + ripple-on-register-change) citing silent-divergence risk
between switch-line chat and selected-style chat; user maintained B after one pushback —
rationale: BLUF+caveman especially is a standalone artifact for outsiders, not pipeline
machinery; the sync tax is bureaucracy for a product surface. Accepted risk, on record: a
future register change does not ripple to the style files.

### D3 — Roster: two styles ship — `Caveman` and `Caveman-BLUF`; pure BLUF does not — `Confident`

The user's pasted BLUF draft merges with the caveman register into one combined style file;
BLUF-standalone never ships (no mochiko value-add over the user's own draft, and styles are
single-select so the combined file is the only way to run both at once). Ruled A over
three-style roster (B) and no-combined-file (C).

### D4 — Caveman style file: baked `full` level only, rules + yields, no ladder — `Confident`

One fixed register: `full` (drop articles, fragments OK). No `off/lite/ultra` ladder, no
per-surface table, no governance vocabulary in the file. Content: drop list ·
never-compress list · keep-user's-language · yield set (security warnings, destructive
confirmations, misorder-risk sequences, ambiguity guardrail, clarification requests) ·
"stop caveman"/"normal mode" in-session escape · written-artifacts-exempt line. Drafting
source: the repo CLAUDE.md operating block (battle-tested compact form), repo-specific
lines stripped. `keep-coding-instructions: true`. User's words: "I only want full caveman
in the config setup."

*Amended at review (I3+M2):* the content list also carries the source block's
**never-announce rule** (no "caveman mode on", no third-person tags, no plain-answer-plus-recap)
and its **response pattern line** (`[thing] [action] [reason]. [next step].` with the
not/yes example pair). Never-announce over the template's disclose-once: the `/config`
picker is itself the disclosure.

### D5 — Caveman-BLUF merge: BLUF governs structure, caveman governs diction; BLUF wins conflicts — `Confident`

From BLUF: answer-first ordering (first sentence = conclusion) · minimal supporting detail
(short why-line or 2–4 tight bullets) · jargon defined in ≤4 words · uncertainty stated with
the one resolving fact · rigor-unchanged closing note. From caveman: article-dropping,
fragments, drop list, never-compress list, keep-user's-language, yields. Stated conflict rule
in the file: where a caveman compression would blur the bottom line, BLUF wins — mirrors the
register's own plain-beats-terse tiebreak and BLUF's founding line ("an order that can be
misread is a failure"). Ruled A over wholesale-compression (B) and verbatim-append (C) —
B lost because it leaves BLUF's "one idea per sentence" contradicting caveman fragments with
no tiebreak; C lost because it ships the contradictions unresolved for the model to sort out
live (M3). `keep-coding-instructions: true` on both files.

*Amended at review (I4):* the caveman contribution to the merge **includes** the in-session
escape ("stop caveman"/"normal mode" — dropping to plain BLUF, since the style stays
selected) and the written-artifacts-exempt line — coding continues under
`keep-coding-instructions: true`, so code/comments/commits are written normally under both
styles. Sibling styles no longer diverge on escape or artifact behavior.

### D6 — Packaging + landing mechanics (bundled) — `Confident`

- **Location:** `plugins/mochiko/output-styles/caveman.md` + `caveman-bluf.md` (directory
  convention per docs; sibling to `commands/`, `skills/`).
- **Frontmatter:** `name: Caveman` · `name: Caveman BLUF`; one-line descriptions;
  `keep-coding-instructions: true` on both; no `force-for-plugin` (per D1).
- **plugin.json:** untouched — the documented optional `outputStyles` field is **omitted
  deliberately** (amended at review, I1): replaces-default semantics mean omission keeps the
  default `output-styles/` directory scan, which is all that's needed.
- **Discoverability:** one line on the router skill's user-facing surface naming the two
  styles and pointing at `/config` → Output style.
- **Landing:** pure addition — rides the decision row, no strip notes; `plugin.json` version
  bump + `CHANGELOG.md` entry + `marketplace.json` sync per release gates (GI-012); **plus the
  KM three-move ritual — DECISIONS.md row · BACKLOG stance · ROADMAP.md touch** (amended at
  review, I5); author≠grader validator pass grading both style files against this record
  **and the router-skill discoverability edit** (a shipped-primitive edit, GI-004 — amended at
  review, I6) before the bump.

## Build surface

1. Create `plugins/mochiko/output-styles/caveman.md` — baked `full`, drafted from the repo
   CLAUDE.md block per D4.
2. Create `plugins/mochiko/output-styles/caveman-bluf.md` — D5 merge (BLUF structure ·
   caveman diction · BLUF-wins conflict rule), drafted from the user's pasted BLUF text +
   the caveman file.
3. Router surface: one discoverability line.
4. Validator audit of both files against this record (author≠grader).
5. Version bump + CHANGELOG + marketplace.json sync; DECISIONS.md row; **ROADMAP.md touch**
   (I5); index update; BACKLOG untouched (built same session or item opened if deferred).

## Review

Solo cold review (devils-advocate seat, `mochiko:review-brainstorm`, frozen record read from
file, default FAIL; first spawn died on a model-name API error and was respawned with the
session-model override). Verdict: **FAIL, needs-revision — 9 findings (6 Important, 3 Minor),
9/9 dispositioned** as a user-ruled batch ("as recommended"):

| # | Finding | Disposition |
|---|---|---|
| I1 | F1/D6 "no manifest field" false — `outputStyles` documented in plugins-reference | Repaired: F1 + D6 amended; omission-keeps-default verified by lead re-read of the source (replaces-default semantics) |
| I2 | D1 silent on level mismatch (line `ultra` vs style `full`) | D1 accepted-risk note extended; no winner ruled |
| I3 | D4 omitted never-announce + pattern line from its drafting source | Both included in D4 |
| I4 | D5 merge silent on escape + artifacts-exempt | Both carry into the merge; escape drops to plain BLUF |
| I5 | Landing list missing ROADMAP touch (KM three-move ritual) | D6 + step 5 amended |
| I6 | Router edit outside audit scope (GI-004) | Audit scope extended to the router edit |
| M1 | F4 missing fork exception | F4 caveat added; grep found no fork usage — vacuous today |
| M2 | Never-announce (block) vs disclose-once (template) birth-time divergence | Never-announce ships (rides I3); F7 notes the delta |
| M3 | D5 rejected roads lacked why-lost | One line each added to D5 |

External claims re-verified live at review (F1 directory quote · F2 frontmatter table · F3
removal versions · D3 single-select premise); I1's plugins-reference premise re-read by the
lead before disposition (source re-read clause). Reviewer's clean list: confidence marks
honest, D6 no-strip-notes stance correct per `primitive-edits.md`.

## Build (2026-08-07, v0.55.0 — same session)

Plan-approved wave, all six build-surface steps executed:

- `plugins/mochiko/output-styles/caveman.md` — D4 as amended (baked `full` · drop +
  never-compress lists · never-announce + pattern line (I3) · keep-user's-language · five
  yields · escape · artifacts-exempt · `keep-coding-instructions: true`, no
  `force-for-plugin`).
- `plugins/mochiko/output-styles/caveman-bluf.md` — D5 as amended (BLUF structure rules 1–6 +
  rigor-unchanged note · caveman diction · BLUF-wins conflict rule · escape drops to plain
  BLUF (I4) · artifacts-exempt · fragments licensed only where they cannot be misread).
- Router `SKILL.md` — Output-styles discoverability section (names both, `/config` pointer,
  main-conversation-only per F4, `templates/output-style.md` named for reports/artifacts).
- plugin.json 0.54.0 → 0.55.0 (no `outputStyles` field, per D6/I1) · marketplace.json synced ·
  CHANGELOG entry · DECISIONS row annotated · trail line · ROADMAP stamp · index updated.

**Audit:** author≠grader (`mochiko:validator` subagent, session-model override — the
persona's pinned model 400s on this key; same error killed the first reviewer spawn and one
explorer this session). **PASS round 1, all three artifacts**; one non-blocking advisory
(router "one line" shipped as one wrapped sentence under its own heading — accuracy claims
justify the length; taken as-is).

## Open threads

*(none)*
