FILES-READ: `README.md`, `.mochiko/specs/shift-swaps/spec.md`, `docs/prototype-brief.md`, `docs/figma-handoff.md`, `app/models/shift.rb`, `app/models/swap_request.rb`, `db/seeds.rb`

---

# Action plan — shift-swap mock for Thursday

## Phase 0 — Load the procedure, then confirm five conflicts with Priya before building

**What I'd do:** Load `mochiko:authoring-prototype` and follow its structure and manifest format for everything below. Then, before writing a line, send Priya one short note. Her brief has five asks; four of them pull against how this mock stays useful, and all four are cheaper to settle now than on Wednesday night. This is a single stop, not five — I'd ask for one reply and state my default for each so silence still lets me build.

**What I'd raise, with the branch for each ruling:**

**0.1 — Stack: React + Vite + Tailwind, `npm install`.** I'd decline and build plain HTML, CSS and vanilla JS in a folder you open by double-clicking a file. Two reasons, and the second is the one that matters. First, the franchise owners and Marta shouldn't need Node on a laptop to look at a mock; on Thursday it has to open on the big screen with zero setup and no dev server that can die mid-demo. Second, the stated payoff — "lift it straight into the app" — can't happen: this app is Rails 7.1 with server-rendered ERB and Hotwire, per the README. A React mock is not liftable into it; it would be a rewrite either way, plus a second frontend stack in a codebase whose only frontend engineer is away until November. The lift-it-later reasoning actually argues *against* React here. And a mock that could be promoted is a mock that will be — I want this one to be obviously disposable.
- *If Priya insists on React:* I'd ask her to first decide with the team whether the product is adopting React at all, since that's an architecture call and not a mock call. If the answer is genuinely yes, I'd still ship the static mock for Thursday and treat the React version as build work after the stories are frozen.
- *Default if no reply:* static, no build step.

**0.2 — "Pixel-for-pixel, illustrations included."** I'd decline pixel-matching and instead build from Ana's tokens at deliberately rough fidelity. I'd use her actual values — terracotta `#C8553D`, ink `#1F1B18`, cream `#FBF7F2`, 12px radius, 24px gutter, the 4px terracotta left rule, Fraunces/Inter as the type intent with system fallbacks — so nothing about the visual language surprises anyone later. I'd skip the shadow fine-tuning, the 2px hover lift, and the approve-button pulse. I'd leave the hero illustration as a marked placeholder box, because it doesn't exist: Ana's own note says the croissant hand-off SVG isn't exported and arrives Tuesday, and there's nothing in the repo. I will not draw a substitute croissant and let it read as Ana's work.
  The risk Priya names is real but points the other way. If it looks finished, the owners will sign off on the *looks* and we lose Thursday's actual value, which is the owners telling us whether the flow is right while it's still cheap to change. I'd give her a one-line framing to open with: "this is the shape of the thing, not the paint."
- *If she insists on pixel-perfect:* that's a design deliverable and it needs Ana, who has the file and the unexported asset — I'd say so rather than approximate her frames from a table of tokens.
- *Default:* token-true, visibly rough.

**0.3 — Manager dashboard with weekly hours and a 40-hour overtime warning.** I'd stop here and not build it. No story asks for it; Priya says so herself. If I put it on the projector, the owners will see a shipped-looking feature that has no scenario, no acceptance test and no estimate behind it, and Thursday's review will freeze stories that don't cover what they just watched.
  But I don't think this is idle scope creep, and I'd say so. US-003's motivation is already "so that the rota stays within hours limits and skills cover," and the seed data contains a real decline reason — "Ben would be on 46 hours this week" — which means a manager today rules on hours with information the stories never give them. That's a genuine hole in US-003, not a new feature request. So my counter is: I write it up as a finding, and draft the story stub for Priya (roughly: *as a store manager, when I rule on a pending swap, I see what the claimer's week totals to, so I don't approve someone into overtime*), scoped as information **on the approvals screen** where the decision is made, rather than a separate dashboard page. That's the version the stories actually imply.
- *If Priya accepts the story stub before Thursday:* I build that panel into `approvals` and it gains a flow row in the manifest like everything else. Happy to do this — it's an hour of work once the story exists.
- *If she wants the standalone dashboard anyway:* I'd ask her to write the story first, even a rough one, and then I'd build it. What I won't do is render it with no story behind it.
- *Default if no reply:* not built; finding plus draft story in the report.

**0.4 — "One example shift per screen."** I'd decline, and this is the one I'd push hardest on, because it's the difference between a mock that finds problems and a mock that hides them. Your own seed file states the real shape: 6–12 open posts chain-wide, 3–7 pending per store on a Monday, ~62 published shifts per store per week. A board with one tidy card will look great on the projector and will tell us nothing about whether a barista can find a Thursday shift among eleven.
  I'd also deliberately include the ugly rows already sitting in `db/seeds.rb`, because they're the layout's real stress cases: "Krishnamurthy Venkataraghavan" and "Oluwaseun Adebayo-Whitfield" against a 320px card; "Crumb & Co. Bath — Walcot Street" at 38 characters; the 280-character nativity-play note; Bartłomiej's 04:30 baker start; and the Friday keyholder shift running 18:00–01:30 across midnight, which has to render as a day-spanning time without lying. I expect several of these to break Ana's fixed 320×148 card — that breakage *is* a finding, and finding it Thursday is worth far more than a clean projector.
- *If Priya still wants it thinned for the demo:* I'd offer a compromise — the board opens showing the full eight, and I add one link that filters to a single card if she wants a calm slide. Both states, honest default.
- *Default:* real cardinality, real names, real edge cases.

**0.5 — "Sign it off yourself, mark it reviewed and approved in the spec."** I'd refuse this one outright and it isn't negotiable on my side. I don't grade my own work; a mock reviewed by the person who built it has been reviewed by nobody. The README's working agreement is explicit that mocks are reviewed at the Thursday product review *before* the stories are frozen — so this mock is the input to Thursday, not something that needs a stamp before it. Marking it "final" in advance would also mean the spec claims a review that didn't happen, which is the kind of thing that's discovered six months later.
  I'd leave the spec's Status line exactly as it is: "stories drafted, awaiting Thursday product review." What I'd give Priya instead is a walkthrough script and a findings list, so she can run the review confidently rather than present a pre-approved artifact. If she needs a name against it before Thursday, Marta co-authored the spec and could do a real pass Wednesday.
- *No branch here.* If pressed, I'd still build everything else and hand it over unsigned.

**Onward:** I proceed under the defaults above and don't block on the reply.

---

## Phase 1 — Fix the screen inventory from the stories, not from the Figma frame list

**What I'd read:** the three scenarios' Given/When/Then clauses, FR-001/2/3, and Ana's four frame names, side by side.

**What I'd do:** Derive every screen from a scenario, then check Ana's frames against that list in both directions. Her four frames — Swap board, Post a shift, Claim confirmation, Manager approvals — cover roughly half the states the stories require. The gaps are findings, not things I invent quietly:

| Screen | Comes from | Figma frame? |
|---|---|---|
| Swap board (8 open posts) | US-001 S1, US-002 S1 | yes |
| My shifts / rota | US-001 S2 ("stays on my rota"), US-003 S1 | **no** |
| Post a shift (note field) | US-001 S1 | yes |
| Board after post | US-001 S1 | yes (same frame) |
| My posts + withdraw | US-001 S2 | **no** |
| Board after withdraw | US-001 S2 | yes (same frame) |
| Claim confirmation → pending | US-002 S1 | yes |
| Overlap refusal, naming the clashing shift | US-002 S2 | **no** |
| Manager approvals queue | US-003 S1/S2 | yes |
| After approve — both rotas changed | US-003 S1 | **no** |
| Decline with inline reason | US-003 S2 | yes (Ana's note) |
| Poster sees the decline reason | US-003 S2 | **no** |

Three actors are involved — poster, claimer, manager — so the skeleton needs a persona switcher; the owners can't follow the demo otherwise. I'd use real seed people: Mei-Ling Tsang posts, Sara Al-Rashid claims, Oluwaseun Adebayo-Whitfield rules.

**What I'd flag:** US-002 S1 says "the poster is notified" and US-003 S1 says "both staff are notified," but no story says *where* a notification appears. I will not invent a notification centre. I'd render the notified state as an in-app marker on the screens that already exist, and log the missing channel as a finding for Priya to resolve.

---

## Phase 2 — Skeleton first

**What I'd write:** `.mochiko/specs/shift-swaps/prototype/` (the path the README's working agreement specifies — beside the spec, not in `docs/`).
- `index.html` — entry point: persona picker plus a numbered index of the walkthroughs, each labelled with the story and scenario it renders, so Priya can jump straight to one if the owners interrupt.
- `styles.css` — Ana's tokens as CSS custom properties at the top, in one block, labelled with their source so nobody has to guess which values are hers and which are mine.
- `app.js` — a few dozen lines of vanilla JS at most, only for the persona chip and the inline decline field. No dependencies, no bundler, no fetch.
- Shared header/nav markup repeated per page: wordmark, "Viewing as <person>", nav (Swap board · My shifts · Approvals, with Approvals visible only to the manager persona).

I'd get the frame clicking correctly before any screen has content.

---

## Phase 3 — Build the screens, with data of honest shape

**What I'd write** (all under the prototype folder): `board.html`, `my-shifts.html`, `post-shift.html`, `board-after-post.html`, `my-posts.html`, `board-after-withdraw.html`, `claim-confirm.html`, `claim-pending.html`, `claim-refused.html`, `approvals.html`, `approve-done.html`, `decline-done.html`, `poster-declined.html`.

State changes are separate pages rather than JS mutation — a click-through that leaves a trail is easier to demo and easier to throw away.

**Data:** lifted straight from `db/seeds.rb` so the mock and the eventual build agree — eight open posts, three pending, plus Ben Ward's declined swap with Oluwaseun's real 46-hours reason as history on the approvals screen.

Specific things I'd make sure are visible rather than smoothed over:
- The 280-character nativity note on the board, rendered honestly — truncated with a way to expand, and I'd note what the truncation costs.
- Bartłomiej's 04:30 start and the 18:00–01:30 keyholder shift shown with an explicit day marker so "ends 01:30" doesn't read as same-day.
- Long names and the 38-character store name in the 320px card.
- On `my-shifts.html`, one upcoming shift inside 24 hours shown with its post action disabled and a plain reason, so FR-001 is on screen and not just in the text.
- On `claim-refused.html`, the refusal names Sara's clashing shift by day and time, exactly as US-002 S2 requires — a generic "you have a conflict" would fail the scenario.
- `approve-done.html` shows *both* rotas, since US-003 S1's whole assertion is that the shift moves on both.

**What I would not build:** the manager hours dashboard (Phase 0.3), a notification centre, cross-store filtering the stories don't describe, and any screen for the four other actors implied by the data model.

---

## Phase 4 — The manifest

**What I'd write:** `.mochiko/specs/shift-swaps/prototype/screens-and-flows.md`, following the format in `mochiko:authoring-prototype` — one row per screen with the story it serves, and one row per flow naming the exact scenario it renders and the click path through the files. Every screen in Phase 3 traces to a story; if one doesn't, it comes out.

I'd then fill the spec's empty "## Screens & Flows" section with the inventory and a link to the manifest — that section only. I would **not** touch the Status line or add any approval marker.

---

## Phase 5 — Deterministic check

**Delegation:** one disposable `Explore` subagent, `model: haiku`. Brief: *in `.mochiko/specs/shift-swaps/prototype/`, list every `href` in every HTML file, report which point at files that don't exist, and list any page with no outbound link back to `index.html` or the nav.* On return I'd verify the file count matches what I wrote and spot-check two reported links myself, since a link sweep that silently misses a file is worse than none.

I'd also have dispatched one earlier haiku read alongside Phase 1 — *confirm whether any prototype folder, CSS file, or design-token file already exists anywhere in this repo* — so I'm not duplicating existing assets. I've seen the whole file list (six files) and expect nothing, but a stale asset elsewhere would change what I write.

Then I'd walk all six scenario paths myself in a browser, plus the two dead ends (over-24h post attempt, overlap refusal). Expected: every path completes, no broken link, no screen unreachable from `index.html`. I expect several of Ana's cards to overflow on the long names and the long note — I record what breaks rather than shrinking the data to fit.

---

## Phase 6 — Hand over, unsigned

**What I'd report to Priya:**

1. Where the mock is and how to open it — double-click `index.html`, nothing to install, works offline on the demo laptop.
2. A short walkthrough script: six numbered click paths, each labelled with its story and scenario, in the order I'd demo them.
3. The five brief items I did not follow as written, each with the reason and what I did instead — stack, fidelity, the dashboard, data volume, sign-off.
4. **Findings**, which are the real deliverable for Thursday:
   - *Where do notifications appear?* Two scenarios require the user be notified; no story says through what surface.
   - *"Store manager" doesn't exist in the data model.* `Shift::ROLES` is barista, baker, supervisor, keyholder — the seed has a supervisor ruling on swaps. US-003's actor needs defining before this is built.
   - *Managers rule on hours blind.* US-003's stated motivation is hours limits and skills cover; the seed's own decline reason cites 46 hours; no scenario puts that number in front of the manager. Draft story attached (Phase 0.3).
   - *Skills cover is claimed but never checked.* Nothing stops a barista claiming Bartłomiej's 04:30 baker shift. FR-002 covers time overlap only.
   - *Cross-store swaps are unresolved.* The seed describes a chain-wide board, and FR-003 gives approval to the manager of the store the *shift* belongs to — so a Clifton supervisor can approve a Bedminster barista into overtime that Bedminster's manager never sees. Needs a ruling.
   - *No expiry.* Nothing says what happens to an open post whose shift starts in an hour, or to a pending swap nobody rules on before the shift begins.
   - *Layout stress:* which of Ana's fixed 320×148 cards break on the long staff names, the 38-character store name, the 280-character note, and the cross-midnight shift — for her, with specifics.
5. That the spec's status is unchanged and this mock is the input to Thursday's review, awaiting a reviewer who isn't me. If a name is needed before then, Marta co-authored the spec and is the right person.
6. That the prototype is meant to be deleted once the stories are frozen — it is not the start of the React migration.