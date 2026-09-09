FILES-READ: `BAR.md`, `agents/accessibility-auditor.md`, `agents/analytics-engineer.md`, `agents/api-designer.md`, `agents/backend-engineer.md`, `agents/billing-analyst.md`, `agents/code-reviewer.md`, `agents/compliance-officer.md`, `agents/content-strategist.md`, `agents/data-engineer.md`, `agents/database-admin.md`, `agents/design-reviewer.md`, `agents/devops-engineer.md`, `agents/documentation-writer.md`, `agents/frontend-engineer.md`, `agents/growth-analyst.md`, `agents/incident-commander.md`, `agents/integration-engineer.md`, `agents/localization-lead.md`, `agents/ml-engineer.md`, `agents/mobile-engineer.md`, `agents/observability-engineer.md`, `agents/performance-engineer.md`, `agents/platform-engineer.md`, `agents/privacy-analyst.md`, `agents/qa-lead.md`, `agents/release-manager.md`, `agents/research-analyst.md`, `agents/security-engineer.md`, `agents/site-reliability-engineer.md`, `agents/solutions-architect.md`, `agents/support-engineer.md`, `agents/technical-writer.md`, `agents/test-automation-engineer.md`, `agents/ux-researcher.md`

---

# Grading plan — 34 persona stubs vs. `BAR.md`

## Phase 0 — Fix the scope and the ruleset before judging anything

**Do:** Confirm the population is exactly the 34 files directly under `agents/` and that `BAR.md` is the yardstick, not a graded item. List the directory (glob `agents/**/*`), count, and compare against the 34 the card claims. If the count differed, I'd stop and report the discrepancy rather than silently grade a different set — a bar that says "every file" is only meaningful against an agreed file list. It matches at 34, so I proceed.

**Read:** `BAR.md` in full, and pin down what each of its three items actually tests, because two of them are stricter than a skim suggests:
- Item 1 tests an **exact heading string**: `## Delegating Cheap Reads`. Not "a section about delegation." A section that is present but retitled fails, and I will say so in those words so the fix is obvious.
- Item 2 tests **presence of a `model:` key in frontmatter**. It does not constrain the value, so `model: opus` and any other value both pass; only absence fails.
- Item 3 is the only judgment call: no command name, no run/pipeline phase, no "the lead hands you…" sequencing anywhere in a persona body.

**Write:** nothing yet.

## Phase 1 — Read all 34 files end to end

**Do:** Read every file completely rather than pattern-matching. These stubs are ~12 lines each; a grep for `## Delegating Cheap Reads` would find the heading but would not catch a file that *lacks* the section, and a grep for `model:` would not distinguish frontmatter from body text. Full reads are cheap here and eliminate both failure modes.

**Explicitly no delegation.** The whole corpus is under 500 lines. Handing this to subagents would cost more coordination than it saves, and item 3 needs one consistent judgment applied across all 34 — split across workers, the borderline line in `backend-engineer.md` and the blatant one in `integration-engineer.md` would likely get inconsistent rulings. I grade all of it myself.

## Phase 2 — Item 1: exact delegation heading

**Do:** For each file, check for a line that is exactly `## Delegating Cheap Reads`. Record present / retitled / absent, because those are three different fixes.

**Expected from the reads:** 32 files carry the exact heading. Two fail: `localization-lead.md` has no delegation section at all (the file ends at its one-line body), and `release-manager.md` has the section but titled `## Delegating Reads` — the word "Cheap" is missing. I'd report the retitle with the actual string quoted, so it isn't mistaken for a missing section.

## Phase 3 — Item 2: model pin

**Do:** For each file, confirm a `model:` key inside the `---` fences. Check position, not just the substring — a body mentioning a model would not satisfy a frontmatter key.

**Expected:** 33 pass with `model: opus`. `support-engineer.md` fails: its frontmatter runs `name` → `description` → `color` with no `model:` line. I'd note that `color` is present, so this reads as an omission rather than a file that never had frontmatter.

## Phase 4 — Item 3: workflow traces, and the boundary I apply

This is where grading judgment actually gets spent, so I fix the rule before applying it:

- **The mandated delegation section is not evidence against itself.** All 32 sectioned files share the line "A locate, an enumeration, or a targeted quote *is handed down*." That phrasing is the shape the bar itself requires; counting it as a workflow trace would make items 1 and 3 contradict each other and fail all 34 files for a boilerplate line. I exclude it and say I'm excluding it.
- **Domain nouns are not workflow.** "release," "verification plan," "prototype," "migration," "incident" are the persona's subject matter. A persona is allowed to know its trade.
- **What fails:** a named position in a sequence, a named command or run, or an explicit handoff to another role.

**Clear fail:** `integration-engineer.md` line 9 — "You are seated in the third phase of the implement run, after design closes, and you hand your wiring report to the lead for the verification phase that follows." That is a numbered phase, a named run, a predecessor, a successor, and a lead handoff in one sentence. It is the exact thing item 3 forbids.

**Borderline — where I would stop and confirm.** Two files sit near the line, and the ruling changes the offender list, not the verdict:
1. `backend-engineer.md` line 9 — "the contract *you were handed*… without *a ruling*." No command, no phase, no named lead, but it presumes an upstream giver and an arbiter. **My default: pass.** It describes the state of an input, not the persona's slot in a pipeline; a backend engineer working alone still implements to a contract someone gave them.
2. `frontend-engineer.md` — "from an *approved* prototype" (in `description:`) and "match the prototype's structure exactly." "Approved" implies an approval step upstream. **My default: pass**, and additionally the phrase sits in frontmatter, while item 3 scopes itself to the persona **body**.

**The stop, and both branches.** I would put these two to the person who owns the bar with the lines quoted, asking a single question: does item 3 fail a persona for presupposing an upstream giver, or only for naming a phase, command, or role in a sequence? Also whether `description:` counts as "body."
- If the answer is the **strict** reading (presupposition fails, frontmatter counts): `backend-engineer.md` and `frontend-engineer.md` join the offender list, making six offending files.
- If the answer is the **narrow** reading (my default): they pass, four offending files stand, and I still list them as advisory near-misses so the owner can tighten the wording if they want.
- Either way the overall verdict is FAIL, so I do not block on this answer — I grade under the narrow default, flag the two files as raised-and-defaulted, and note that the ruling only moves the offender count.

## Phase 5 — Assemble the verdict

**Do:** Compose the report in the structure `BAR.md` asks for: per-item pass/fail, every offending file named with the verdict, and the overall verdict, which is PASS only if all three items pass.

**Expected shape:**
- Item 1 — FAIL: `localization-lead.md` (section absent), `release-manager.md` (heading is `## Delegating Reads`).
- Item 2 — FAIL: `support-engineer.md` (no `model:` key).
- Item 3 — FAIL: `integration-engineer.md` (phase-and-handoff sentence, quoted in full). Advisory, not counted: `backend-engineer.md`, `frontend-engineer.md`.
- Overall — **FAIL**. Four files carry a defect against the bar as I read it; 30 of 34 are clean.

**Write:** By default, nothing to disk — the card asks me to grade, not to produce an artifact, and `BAR.md` names no output path. I'd deliver the verdict in my reply and offer to write it to `GRADING.md` at the workspace root. If the owner wanted the file, I'd write exactly that report there and nothing else. I would **not** fix any of the four files: grading and remediating are different jobs, and editing a stub to pass a bar I am also scoring would destroy the evidence for the verdict. I'd offer the four fixes separately (add the section to `localization-lead.md`, correct the heading in `release-manager.md`, add `model:` to `support-engineer.md`, rewrite `integration-engineer.md` line 9 as a standalone competence statement) and let the owner decide.

## Phase 6 — Self-check before reporting

**Do:** Re-verify the three specific failures by re-reading those four files, since a single misread flips a named file's verdict and this report is the thing that gets acted on. Confirm the offender count reconciles: 34 total, 4 named, 30 clean, no file named under two items. Confirm no file was skipped by diffing my per-file table against the Phase 0 directory listing.

**No tests to run** — there is no code here, nothing executes, and there is no shell in this run regardless. The check is the re-read, and I'd say plainly that verification was by re-reading rather than by any automated check.

## What I'd report at the end

The overall FAIL verdict; the three items each marked pass/fail with every offending file named and the offending text quoted; the two borderline files flagged as raised-and-defaulted with the ruling I applied and which way the offender list moves under the other reading; the note that item 3's mandated boilerplate was deliberately excluded from its own test; and an offer to either write the report to a file or apply the four fixes as a separate pass.