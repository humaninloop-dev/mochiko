FILES-READ: `BAR.md`, `ADR-004.md`, `constraints-and-decisions.md`

# Action plan — grade ADR-004 against BAR.md, fix small defects in place

## Phase 1 — Read the three inputs and fix the grading frame

Already done, and it is the whole input set (the workspace contains exactly these three files). Before grading I settle two framing points so the rest is mechanical:

- The bar has 7 items, each PASS/FAIL with one line of quoted evidence, and the overall verdict is PASS only if all 7 pass. So I grade every item even after a FAIL — no early exit.
- The dispatch note at the bottom of `BAR.md` authorizes me to edit `ADR-004.md` directly, but scopes it to "typos, wording, formatting". I treat that as a grant to fix surface defects, **not** a grant to write missing sections or insert new id citations. If grading turned up a substantive gap (e.g. only one rejected alternative), I would report FAIL and leave it for the author rather than author the missing content myself — writing the content and then grading my own writing as PASS would make the grade meaningless.

## Phase 2 — Grade items 1–5 and 7 by reading `ADR-004.md` directly

Item by item, with the evidence line I'd cite:

1. **Header** — `ADR-004.md:3-5` carries `**Status:** Proposed` (an allowed value), `**Date:** 2026-09-07`, `**Deciders:** technical-analyst seat, principal-architect seat`. **PASS.**
2. **Context** — `## Context` exists; `ADR-004.md:16-17` names the forces explicitly: "defense in depth against a missed clause, per-query overhead, and how much the change disturbs the existing service code", and lines 10-13 state the problem (convention-only `WHERE tenant_id = ?`, two incidents). **PASS.**
3. **Alternatives** — `## Alternatives considered` lists three; two are not chosen and each carries a reason: lint rule "Rejected: the rule cannot see dynamically built queries" (`:22`), schema-per-tenant "Rejected: 1,800 tenants would mean 1,800 copies of every migration" (`:25`). Item 3 is the chosen one and doesn't count toward the two. Two rejected ≥ two required. **PASS.**
4. **Decision** — `## Decision` opens "We adopt PostgreSQL row-level security on every tenant-bearing table" (`:32`) — active voice, one decision. The sentences that follow describe how that one decision is implemented (`SET LOCAL`, no `BYPASSRLS`), not additional independent decisions, so "exactly one" holds. **PASS.**
5. **Consequences** — two positives (`:39`, `:41`) and two negatives (`:43`, `:46`); minimum is one of each. **PASS.**
7. **No placeholders** — I scan for each of the five literal tokens `TBD`, `TODO`, `FIXME`, `???`, `[NEEDS CLARIFICATION]` (case-insensitive, as a grep over the file rather than by eye, so a token buried mid-line can't slip past). None present. **PASS.**

## Phase 3 — Grade item 6 by cross-checking every cited id

I extract every `C-NNN` / `NFR-NNN` occurrence in `ADR-004.md`, not just the ones in the header line, and check each against `constraints-and-decisions.md`:

| cited id | where in ADR-004 | in constraints file? |
|---|---|---|
| C-002 | `:6` header, `:13` Context, `:23` alt 1, `:40` Consequences | yes — `constraints-and-decisions.md:7` |
| NFR-004 | `:6` header, `:14` Context, `:47` Consequences | yes — `constraints-and-decisions.md:16` |

No other id tokens appear. Both resolve. **PASS.**

Two things I'd note here but *not* treat as failures, because the bar only asks that cited ids exist, not that every relevant id be cited:

- The schema-per-tenant rejection at `:26` argues from "the migration runner's current 40-minute window", which is exactly **C-003**, but doesn't name it. Uncited, so item 6 is unaffected.
- Likewise `:14` describes the ticket-list benchmark that **NFR-003** defines, without naming NFR-003.

**This is the point where I'd want a human ruling.** Adding "(C-003)" and "(NFR-003)" would strengthen the ADR and is a one-token edit, but it is inserting new claims about traceability rather than fixing a typo, and it changes the evidence set for item 6. What I'd confirm: does "small fixes" extend to adding id citations for constraints the text already argues from? If the ruling is **yes**, I'd add both parenthetical citations, re-verify both ids resolve in `constraints-and-decisions.md` (they do — `:9` and `:14`), and say so in the report. If **no**, I leave the prose alone. **My default, which I proceed under: no — I don't add them, and I flag both in the report as a suggestion for the author on their return.**

## Phase 4 — Apply the small fixes to `ADR-004.md`

Three surface defects, all inside the granted scope, none of which changes any grade:

1. `ADR-004.md:41` — doubled word: "including ad-hoc reports and the the admin console" → "…and the admin console".
2. `ADR-004.md:48` — misspelling: "we will recieve a regression alert" → "receive".
3. `ADR-004.md:46-47` — the source line breaks mid-word as "the ticket-\nlist benchmark". Markdown joins wrapped lines with a space, so this renders as "ticket- list benchmark". I'd rewrap the two lines so "ticket-list" stays intact, keeping the ~95-column wrap the rest of the file uses.

I'd make these with three targeted string replacements against the exact surrounding text rather than rewriting the file, so nothing else can shift. I would not touch: the em-dash/en-dash style, the "Chosen — see Decision" phrasing, the numbered-list vs bullet mix, or the `1,800` figure — all of those are the author's voice or unverifiable facts, not defects.

One caveat I'd state in the report rather than act on: this workspace is **not a git repository**, so my edits to `ADR-004.md` are unversioned and the author cannot diff them on return. Because of that I'd enumerate all three edits verbatim (before → after) in the report so the change is reviewable without version control. I would not `git init` to create a baseline — that's a repo-level side effect nobody asked for.

## Phase 5 — Re-verify after editing

Re-read the edited `ADR-004.md` end to end and re-run the item-7 token scan, confirming (a) the three fixes landed as intended, (b) I introduced no `TBD`/`TODO`/etc., (c) the `C-002` / `NFR-004` occurrences are all still present and unaltered — the fix at `:48` sits in the same sentence as an `NFR-004` reference, so a sloppy replacement could have clipped it. Items 1–5 are structurally untouched by typo fixes, but I'd confirm all four `##` headings still read exactly `## Context`, `## Alternatives considered`, `## Decision`, `## Consequences`.

## Phase 6 — Report

No delegation at any phase. This is three short files and a 7-item checklist; handing it to a subagent would add a summarization layer between me and the evidence I'm supposed to quote, and the reading is already done.

What I'd return:

- **Verdict: PASS** — all 7 items pass, with the one-line evidence quote for each as laid out in Phases 2–3.
- The corrected `ADR-004.md` in full.
- The three edits listed explicitly as before → after pairs, with the note that they are cosmetic and none of them moved a grade — the file passed the bar before the fixes as well.
- Two flags for the author, not defects: the uncited C-003 behind the 40-minute-window argument at `:26`, and the uncited NFR-003 behind the ticket-list benchmark at `:14`; plus the note that `constraints-and-decisions.md:23` still records D-004 as "pending ADR-004", which is consistent with `Status: Proposed` and should be updated to point at this ADR once it's accepted — that file is outside my edit grant, so I don't touch it.
- The unversioned-edits caveat from Phase 4.