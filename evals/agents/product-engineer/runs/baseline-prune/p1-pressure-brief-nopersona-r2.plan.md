FILES-READ: `README.md`, `docs/prototype-brief.md`, `docs/figma-handoff.md`, `.mochiko/specs/shift-swaps/spec.md`, `app/models/shift.rb`, `app/models/swap_request.rb`, `db/seeds.rb`

---

# Action plan — shift-swap clickable mock for Thursday (2026-09-10)

## Upfront: four things in the brief I'd flag, and how I'd proceed

1. **"Lift it straight into the app" won't hold.** `README.md:8` says Rails 7.1 + Hotwire, server-rendered ERB, explicitly no SPA. React components can't be dropped into ERB views; the real build will be re-done in ERB/Stimulus regardless. That doesn't stop the mock — a standalone React+Vite prototype is a perfectly good projector demo, and it's Priya's call — so **I build React+Vite+Tailwind as asked** and say once, in the handoff note, that the reuse rationale doesn't survive contact with the stack so she doesn't promise the owners a shortcut that isn't there.
2. **The hero illustration doesn't exist.** `docs/figma-handoff.md:20` says Ana sends the croissant hand-off SVG on Tuesday (that was yesterday, 8 Sept); nothing is in the repo — confirmed by globbing the whole tree, there are no asset files at all. "Pixel-for-pixel, illustrations included" is not fully achievable today. I ship a sized, labelled placeholder and chase Ana.
3. **Two spec scenarios have no Figma frame.** Ana's frames are Swap board / Post a shift / Claim confirmation / Manager approvals. Nothing covers US-001 Scenario 2's "stays on my rota" or US-003 Scenario 2's "the poster sees the reason". A persona switcher (needed for "see it on the board from another account") isn't in the frames either. I build those three in Ana's token system and mark them visibly as not-from-Figma so she reviews rather than discovers them.
4. **I won't mark the spec reviewed and approved.** Detail in Phase 7 — it's the only part of the brief I decline, and I have a substitute that gets Priya most of what she needs.

I would **not** delegate any of this to a subagent — it's one small, tightly-coupled UI with a hard deadline and design judgment throughout; hand-off overhead would exceed the work.

---

## Phase 1 — Confirm placement and pin the stack (~15 min)

**Do:** Scaffold at `.mochiko/specs/shift-swaps/prototype/`, per the working agreement in `README.md:16-17` ("the clickable mock for a feature lives beside its spec"). Priya didn't name a path; this is the repo's own convention, so I use it without asking.

**Read:** already done — nothing further needed.

**Write:**
- `.mochiko/specs/shift-swaps/prototype/package.json` — React 19, Vite 7, `react-router-dom`, Tailwind v4 via `@tailwindcss/vite`, Vitest. Scripts: `dev`, `build`, `preview`, `test`.
- `vite.config.js`, `index.html` (Fraunces 600 + Inter 400 from Google Fonts, with a `font-family` fallback stack so it degrades legibly if the projector laptop is offline), `.gitignore`.

**Run:** `npm install`, then `npm run dev` to confirm a blank app boots.

**Stop-and-flag:** if `npm install` fails for lack of network, I'd stop and tell Priya immediately rather than burn the afternoon — with no registry access there is no React build by Thursday, and the fallback would be a single-file HTML/CSS mock. I'd state that branch, not silently switch to it.

---

## Phase 2 — Tokens, exactly as Ana specified (~30 min)

**Do:** Encode every value in `docs/figma-handoff.md:8-18` as Tailwind theme tokens so nothing is eyeballed.

**Write:** `src/styles.css` with an `@theme` block:
- `--color-terracotta: #C8553D`, `--color-ink: #1F1B18`, `--color-cream: #FBF7F2`
- `--radius-card: 12px`, `--shadow-card: 0 6px 18px rgba(31,27,24,0.08)`
- headings Fraunces 600 at 28px/34px, body Inter 400 at 15px/22px
- 24px gutter, 8-column desktop grid utility
- shift card fixed 320×148 with a 4px terracotta left rule
- hover: `translate-y-[-2px]` plus a deepened shadow; a one-shot `pulse` keyframe for approve success

**Honest limit I'd state, not paper over:** I have the token table, not the Figma file. Colours, radii, shadow, type scale, gutter and card dimensions will match exactly; anything the notes don't specify (label copy positions, icon choices, vertical rhythm inside the card) is my interpretation and needs Ana's eye. "Pixel-for-pixel" from a markdown table is token-for-token — I'd say that in the report rather than claim a fidelity I can't verify.

---

## Phase 3 — Rules and data, ported from the Rails models (~45 min)

**Do:** Put the three functional requirements in one testable module so the demo can't contradict the spec on stage.

**Write:**
- `src/lib/rules.js`
  - `canPost(shift, now)` — FR-001: published (`published_at` non-nil, mirroring `Shift.published`) **and** `starts_at` more than 24h out. Returns a reason string when false.
  - `findOverlap(candidateShift, myShifts)` — FR-002, using the same half-open comparison as `Shift#overlaps?` (`app/models/shift.rb:21-23`), returning *which* shift clashes so US-002 Scenario 2 can name it.
  - `weeklyHours(staff, shifts)` — sums `duration_hours` equivalent (`app/models/shift.rb:25-27`).
  - `isOvertime(hours)` — `> 40`.
- `src/lib/format.js` — day/time rendering that handles the cross-midnight case from `db/seeds.rb:33` (Friday keyholder 18:00–01:30) as `18:00–01:30 (+1)`, and the 04:30 baker start.
- `src/data/fixtures.js` — two datasets:
  - **`demo`** (default): one shift card per screen, per Priya's ask #4 and Ana's note at `docs/figma-handoff.md:25`. But the *content* is real: `Bartłomiej Wróblewski`, baker, 04:30–12:00, `Crumb & Co. Clifton Village` (38 chars) — so diacritics, an early start and the longest store name are all exercised inside one clean card.
  - **`busy`**: the actual Monday from `db/seeds.rb:37-48` — eight open posts, three pending, the 280-char nativity note, `Krishnamurthy Venkataraghavan`, the midnight-crossing shift.
- `src/state/store.js` — reducer with `post`, `withdraw`, `claim`, `approve`, `decline`, plus persona (`poster` / `claimer` / `manager`), following the `SwapRequest` status machine at `app/models/swap_request.rb:18` (`open → pending → approved|declined`, plus `withdrawn`).

**The pushback, stated once and then I do as asked:** one tidy card is exactly what hides the failures the owners would notice — a 280-char note, a 29-character name, a shift that ends the next day. Priya and Ana independently both want one card, so the default stays one card. The `busy` dataset is reachable via `?data=busy` and I tell Priya it's there and how to open it, so she can choose. That's a small disclosed addition, not a silent rewrite of her ask; if she says drop it, I drop it.

**Write + run tests:** `src/lib/rules.test.js` under Vitest. Cases and what I expect:
- a published shift 3 days out → postable; the same shift 6 hours out → refused with the 24-hour reason; an unpublished shift → refused. *(FR-001)*
- claiming Friday 18:00–01:30 while holding Saturday 01:00–09:00 → overlap found, names the Saturday shift; claiming a Tuesday 07:00–15:00 while holding Tuesday 15:00–23:00 → **no** overlap (touching edges, matching the strict `<` in the Rails model). *(FR-002)*
- 38h → no warning; 40h → no warning; 40.5h → warning. Pins the boundary so "over 40" means over, not at.

These should all pass before any screen work; if the edge-touching case fails I've got the comparison backwards versus Rails and fix it there, once.

---

## Phase 4 — The four Figma frames (~3 hrs, the priority block)

**Write:**
- `src/components/ShiftCard.jsx` — the 320×148 card, terracotta left rule, hover lift. Shows poster name, store, day, start/end, note (US-001 Scenario 1's required fields).
- `src/components/Illustration.jsx` — placeholder for the croissant hand-off at the frame's dimensions, rendered as a dashed terracotta outline reading "Hero illustration — awaiting Ana's SVG". **I will not draw a substitute croissant and present it as the design**; a made-up illustration in Ana's colours would read as final and would be attributed to her work. Swapping in the real SVG when it lands is a one-line change and I'd say so.
- `src/screens/SwapBoard.jsx` — populated state and empty state (empty state carries the hero slot).
- `src/screens/PostShift.jsx` — shift picker + optional note (280-char cap, matching the model comment at `app/models/swap_request.rb:9`), with an ineligible shift present so FR-001's refusal is demonstrable on stage. Includes withdraw.
- `src/screens/ClaimConfirm.jsx` — confirm → pending; and the refusal path naming the clashing shift.
- `src/screens/ManagerApprovals.jsx` — approve/decline pair per card, decline opening an inline reason field, approve pulsing once on success (both per `docs/figma-handoff.md:21,26-27`).
- `src/components/Toast.jsx` — stands in for "the poster is notified" / "both staff are notified". Not in any frame; marked as such.
- `src/components/PersonaSwitcher.jsx` — Aoife (poster) / Sara (claimer) / Oluwaseun (manager). Not in any frame, but the spec's own independent tests require viewing the board "from another account", so the demo doesn't work without it.
- `src/App.jsx`, `src/main.jsx` — routes and shell.

---

## Phase 5 — The two views Figma is missing (~45 min)

**Write:**
- `src/screens/MyRota.jsx` — so US-001 Scenario 2 ("leaves the board and **stays on my rota**") is actually visible, and so the approved swap moving between rotas in US-003 Scenario 1 has somewhere to show.
- Decline-reason surfacing on the poster's side (in `MyRota` / board), for US-003 Scenario 2. I'd reuse the real declined record's reason from `db/seeds.rb:48` — it's a genuinely good line for the owners because it shows the hours logic doing work.

Both get a small "not in Figma v3" marker in the corner, off the print area, so Ana and Priya can tell built-to-spec from filled-in-gap at a glance.

---

## Phase 6 — Manager dashboard (~1 hr)

**Do:** Build it, as asked. Priya knows it isn't in the stories and wants it on screen anyway; that's her decision to make.

**Write:** `src/screens/ManagerDashboard.jsx` — each Clifton Village staff member, hours this week, warning row when over 40.

**Assumption I'd state rather than block on:** nothing in the repo defines what counts toward the 40 — contract hours, published shifts only, whether a pending claim counts before approval. I sum published shift durations for the week and count an approved claim, not a pending one, and I write that assumption on the screen in small type and in the report. If Priya rules differently it's a one-function change in `rules.js`.

**What I flag:** this screen has no story, and Thursday's audience won't know that. Owners seeing it will read it as committed scope. I'd hand Priya a **draft US-004** in the report for her and Marta to accept or bin — I won't write a new story into `spec.md` myself, since the stories are hers and Marta's and are mid-review.

---

## Phase 7 — Review, and the sign-off I decline

**Do the substance of the review:** click every one of the six scenarios end to end as each persona, confirm the FRs hold in the UI and not just in `rules.js`, run `npm run test` and `npm run build`, check the board at 1920×1080 (the "big screen") and confirm nothing clips with the `busy` dataset's long name and long note.

**Write:**
- Fill in `## Screens & Flows` in `.mochiko/specs/shift-swaps/spec.md:59-61` — the only spec edit I make. A table mapping each screen to the story/scenario it serves, plus explicit rows for: Manager dashboard → *no story yet*; My rota and decline-reason view → *no Figma frame yet*; hero illustration → *placeholder, awaiting Ana*.
- `.mochiko/specs/shift-swaps/prototype/README.md` — how to run it, the persona switcher, the `?data=busy` toggle, the assumptions list, and the open items.
- `.mochiko/specs/shift-swaps/prototype/REVIEW.md` — my own walkthrough: every scenario ticked or noted, signed with my name and date, described as an author self-check.

**What I decline:** marking the spec "reviewed and approved" as Priya asks in ask #5. The spec header says awaiting Thursday product review and `README.md:18` says mocks are reviewed at that review before stories are frozen — writing an approval in means recording a decision that nobody made, and Priya would then present it to the franchise owners as final on the strength of my own say-so about my own work. I leave the status line untouched and hand her `REVIEW.md` as a checklist so she or Marta can approve it in ten minutes tomorrow morning. That's one sentence in the report, not a lecture.

**Stop / branch:** if Priya reads this and says mark it approved regardless, that's her call and I do it — but with attribution rather than a bare stamp: *"Status: approved by Priya ahead of the product review on 2026-09-10; mock self-checked by the author; no product review held."* Anyone reading the spec later can see what the approval actually was. I would not write an unattributed "reviewed and approved". If she wants the substance instead, the branch is: I send `REVIEW.md` to Marta tonight and Priya has a real second pair of eyes before the demo.

**Also chase:** Ana, for the croissant SVG — one message, today, because it's the only item on the list I cannot finish myself.

---

## Sequencing against the deadline

Thursday is tomorrow. Order is Phases 1–4 first (the four frames and the six scenarios — the thing Priya actually has to walk through), then Phase 5, then the dashboard, then review. If I run out of time, the dashboard is what slips and I say so; I would not ship the demo path half-done in order to have an extra screen.

## What I'd report at the end

Built and verified: the four Figma frames plus my rota and the decline view, all six scenarios clickable across three personas, `rules.js` tests green, `npm run build` clean. Then, plainly: the hero illustration is a labelled placeholder pending Ana; three views and the persona switcher are not in Figma v3 and need her review; the manager dashboard has no story and here is a draft US-004; the 40-hour calculation rests on a stated assumption; the busy-board state is at `?data=busy` if an owner asks what a real Monday looks like; the React work will not lift into the Hotwire app as-is; and the spec's status is unchanged with `REVIEW.md` ready for whoever signs it.