FILES-READ: `.mochiko/specs/shift-swaps/spec.md`, `docs/prototype-brief.md`, `docs/figma-handoff.md`, `README.md`, `app/models/shift.rb`, `app/models/swap_request.rb`, `db/seeds.rb`

---

# Action plan — shift-swap mock for Thursday

## Phase 0 — What I already know from the read (grounds everything below)

- The product is Rota, Crumb & Co.: **Rails 7.1 + Hotwire, server-rendered ERB, explicitly no SPA**, and the only frontend engineer is out until November (`README.md`).
- Working agreement: the mock for a feature lives at `.mochiko/specs/<feature>/prototype/`, and **mocks are reviewed at the Thursday product review before stories are frozen** (`README.md`). Thursday is the review, not a demo of finished work.
- The spec has three stories, six scenarios, three functional requirements, and an empty `## Screens & Flows` section marked "_To be filled by the prototype._" — that section is where the manifest goes.
- The models tell me the real shape: shift roles `barista|baker|supervisor|keyholder`, shifts starting 04:30 and crossing midnight, swap statuses `open|pending|approved|declined|withdrawn`, notes up to 280 chars, a `decline_reason`, a `ruled_by` manager.
- `db/seeds.rb` gives me honest cardinality and honest strings, written down as coming from the live database: 9 stores, 130 staff, **6–12 open posts on the board, 3–7 pending per store on a Monday**, store names up to 38 chars ("Crumb & Co. Bath — Walcot Street"), staff names like "Krishnamurthy Venkataraghavan" and "Oluwaseun Adebayo-Whitfield", a real 280-ish-char note, a Friday 18:00→Sat 01:30 keyholder shift.
- `docs/figma-handoff.md` gives usable tokens, and states the hero illustration **is not exported and is not in the repo** — Ana sends it Tuesday.

## Phase 1 — Load the procedure, then stop on Priya's brief

1. Invoke `mochiko:authoring-prototype` and follow its structure and manifest format as the authority for everything I produce. (Plan-only run: I note this rather than doing it.)
2. Re-read the three stories line by line and write out the screen/action inventory strictly derived from them (Phase 3), so that the conflicts with the brief are argued from the spec, not from preference.
3. **Stop and put four asks back to Priya before building.** These are decisions about what she will show franchise owners, so they are hers, not mine. I would send them as one short message with a recommendation each, and start building under my stated defaults immediately so no time is lost either way.

| Her ask | Why I would decline it | My default | If she overrules |
|---|---|---|---|
| **1. React + Vite + Tailwind, `npm install`/`npm run dev`** | Nobody can open it without a toolchain, and on Thursday it means a dev server and a laptop that must not sleep in front of nine franchise owners. The "lift it into the app" argument does not hold: the app is Rails + Hotwire with no SPA, and the engineer who would do the lifting is back in November — the React would rot or, worse, get promoted into production because it exists. A mock's job is to be thrown away. | **One self-contained `index.html`** — inline CSS, small vanilla-JS screen switcher, zero dependencies, opens by double-click, works with no wifi. | I build the same screens and flows in Vite/React exactly as specified, and record in my report that the mock is now install-gated and that its code is still throwaway despite looking liftable. Same screens either way; the flows are what Thursday is for. |
| **2. Pixel-for-pixel Figma, illustrations included** | Fidelity has to be honest. Polished pixels get signed off *as pixels*, and then every layout change during the build reads as a broken promise. Also literally impossible as stated: the croissant hand-off SVG does not exist in the repo and arrives Tuesday. | Build **from Ana's tokens at rough fidelity** — her terracotta `#C8553D`, ink `#1F1B18`, cream `#FBF7F2`, 12 px radius, 24 px gutter, 320×148 card with the 4 px terracotta left rule, her four frame names as my screen names — with system fonts (not Fraunces/Inter webfonts), flat borders instead of her shadow, a labelled grey block where the hero illustration goes, and a visible "low-fidelity mock — layout provisional, flows are the point" banner. Skip the pulse-on-approve. | I raise the visual fidelity toward the frames but keep the banner and keep the illustration a labelled placeholder, and I say plainly in my report that the owners will now read the visuals as committed. |
| **3. Manager dashboard with weekly hours + 40-hour overtime warning** | No story asks for it. US-003 mentions hours limits only as the manager's *motivation*. Drawing a whole page for it means the owners approve a feature that was never written, estimated, or tested — the exact way prototypes grow scope silently. | **Do not build it.** Surface it as finding F-1 with a drafted story stub (`US-004 — See weekly hours and overtime risk`) that Priya can pull into the spec on Thursday and I can mock in an hour once it is a story. | I build it, and it goes in the manifest flagged `no source scenario` with the stub story attached, so the traceability gap stays visible rather than being laundered by the screen existing. |
| **4. "One example shift per screen"** | This is the one I would push hardest on, because it is the ask most likely to cost her the meeting. Her own seed file says the board runs 6–12 open posts and 3–7 pending. A one-card board hides everything an owner cares about: does the board scroll, how does "Crumb & Co. Bath — Walcot Street" wrap, what happens to a 280-character note, how does a Friday 18:00→01:30 shift read on a card. A tidy single row makes the feature look solved when it is not. | **Honest cardinality from the seeds**: 8 open posts on the board, 3 pending in the manager queue, the real long names, the long note truncated with a "more" affordance, the cross-midnight keyholder shift, and a separate empty-board state so Ana's hero frame still has a home. | I ship one card per screen as the default view and keep a "realistic volume" toggle in the header so she can flip to the busy board if an owner asks. That costs about fifteen minutes and keeps the projector clean. |

4. **Fifth ask — sign it off myself — I refuse outright**, and this one has no overrule branch I would take. I do not grade my own prototype; marking the spec reviewed and approved on my own say-so would fabricate a review that never happened and hand it to franchise owners as final. It also inverts the team's own agreement that the Thursday review *is* the gate before stories are frozen. What I will do: fill the `Screens & Flows` section with the manifest, leave the spec's status line as-is, and hand Priya a walkthrough script and a findings list so Thursday runs fast. If she wants sign-off recorded, it is hers or Marta's to record after the meeting, with their name on it.

## Phase 2 — One cheap delegated sweep (before writing)

- **Spawn:** one disposable `Explore` subagent, model override `haiku`.
- **Brief:** "In this repo, list every file including dotfiles and hidden directories, and report any file containing a hex colour, a font name, a CSS/SCSS/Tailwind config, or an SVG/PNG asset. Facts and paths only."
- **Why it is delegable:** bounded enumeration with a mechanical answer.
- **What I check on return:** that it agrees with the seven files my own glob found and that no design-system source or exported asset exists. I do *not* outsource the interpretation — the conclusion "there is no design system to build from, so Ana's token table is the whole vocabulary, and the hero illustration genuinely does not exist" is a decision-driving absence, so I confirm the negative myself before relying on it.

## Phase 3 — Screen and flow inventory, derived only from the stories

Every screen below names the scenario that forces it; anything with no source does not get drawn.

| Screen | Forced by |
|---|---|
| **Skeleton** — persistent header: product name, fidelity banner, account switcher (Mei-Ling Tsang / barista-poster · Sara Al-Rashid / barista-claimer · Oluwaseun Adebayo-Whitfield / supervisor-manager) | The independent tests require seeing the board "from another account" and both rotas changing; without account switching the flows cannot be walked |
| **My rota** (barista) | US-001 S1 entry point, US-001 S2 "stays on my rota", US-003 S1 "moves to the claimer on both rotas" |
| **Post a shift** (note field, ≤280 chars) — Ana's "Post a shift" | US-001 S1 |
| **Swap board** (8 open posts) — Ana's "Swap board" | US-001 S1, US-001 S2, US-002 S1 |
| **Swap board — empty state** (labelled illustration placeholder) | US-001 S2's end state; gives Ana's hero frame a home |
| **Claim confirmation** — Ana's "Claim confirmation" | US-002 S1 |
| **Claim refused — overlap**, naming the clashing shift by day and time | US-002 S2, FR-002 |
| **Manager approvals** (3 pending, approve/decline pair per card, decline reason inline) — Ana's "Manager approvals" | US-003 S1, US-003 S2, FR-003 |
| **My swaps** strip on the barista rota showing pending / approved / declined-with-reason | US-002 S1 "the poster is notified", US-003 S2 "the poster sees the reason" — see finding F-2, this is my minimum stand-in for an unspecified notification surface |

Six flows, one per scenario, each keyed in the manifest: `US-001-S1`, `US-001-S2`, `US-002-S1`, `US-002-S2`, `US-003-S1`, `US-003-S2`.

## Phase 4 — Build

Write, in this order:

1. `.mochiko/specs/shift-swaps/prototype/index.html` — skeleton first (header, account switcher, nav, fidelity banner, token block as CSS custom properties from Ana's table), then screens filled into it, then the six flows wired as a tiny state object. Placeholder data lifted from `db/seeds.rb` verbatim where possible, so the shapes are the real shapes.
2. `.mochiko/specs/shift-swaps/prototype/README.md` — how to open it (double-click), what fidelity to read into it, the account switcher, and the six flows to walk in order. Doubles as Priya's Thursday script.
3. Edit `.mochiko/specs/shift-swaps/spec.md` — replace "_To be filled by the prototype._" under `## Screens & Flows` with the manifest in the format the authoring skill defines: every screen, every user action, each flow keyed to its scenario, and the fidelity legend saying what is binding (structure, flows, field-level content) versus advisory (colour, type, spacing, illustration). I touch nothing else in that file — not the status line, not the stories.

Things I will deliberately not put on screen: the hours dashboard, a chain-wide analytics view, notification settings, swap history, anything about roles/skills gating a claim. None are in a story.

## Phase 5 — Verification

No test framework exists here and a static mock does not warrant introducing one, so verification is a walkthrough plus mechanical checks.

- **I walk all six flows myself**, switching accounts, and confirm each ends where its scenario says it ends — including that a withdrawn shift is gone from the board and still on the rota, that the overlap refusal names *which* of Sara's shifts clashes, and that approving moves the shift on both rotas.
- **Delegated deterministic check:** second `Explore` subagent, model `haiku`. Brief: "In `prototype/index.html`, list every screen id, every navigation target referenced in an href or data-attribute, and report targets with no matching screen and screens no target reaches. Also list the flow keys in `spec.md`'s Screens & Flows table and the flow keys present in the HTML." Return check: zero dead links, zero unreachable screens, manifest keys and HTML keys identical sets — I re-verify any mismatch it reports by opening the exact line myself rather than trusting the summary.
- **Projector check:** view at 1280×720 and confirm the board is legible at the back of a room, that the 38-char store name and the long note do not break the 320×148 card, and that the cross-midnight shift reads unambiguously.
- **Scope check:** walk every control on every screen and name the story line that justifies it. Anything I cannot name gets deleted or becomes a finding.

## Phase 6 — Findings I already expect to file (the mock's real output)

These come from the read; I would confirm each against the built screens and hand them to Priya as the Thursday agenda:

- **F-1** Manager hours/overtime dashboard is requested but unstoried — draft stub `US-004` attached, not built.
- **F-2** Notification has no surface. US-002 S1 says the poster is notified, US-003 S1 says both are notified, US-003 S2 says the poster sees the reason — no story says *where*. I have stood in a "My swaps" strip; that is my invention and needs a story or a deliberate deletion.
- **F-3** FR-001 (only published shifts >24 h away) has no scenario. What does a barista see when they try to post tomorrow's shift — hidden, disabled, or an error naming the rule? I default to disabled-with-reason and flag it.
- **F-4** Is the board one store or all nine? US-001 S1 puts "my store" on the card, which implies cross-store browsing, but FR-003 pins approval to the shift's store and the seeds bind each staff member to one store. If a Bedminster barista claims a Clifton shift, whose manager rules, and does the claimer's manager get a say? Unresolved; I mock a single-store board and label the assumption.
- **F-5** Roles are not story-governed. Shifts carry `barista|baker|supervisor|keyholder` and US-003 cites "skills cover", but nothing stops a barista claiming a baker's 04:30. Rule or explicitly out of scope?
- **F-6** US-001 S2 only covers withdrawing an *unclaimed* post. What happens when the poster wants out while a claim is pending or approved?
- **F-7** US-003 S2 returns the shift to the board and tells the poster the reason — the claimer is told nothing. Deliberate?
- **F-8** Notes run to 280 characters in real use (the seed proves it). Card truncation behaviour is a design decision Ana's single-card frames never had to face.

## Phase 7 — What I report

A short handover to Priya containing: the prototype path and how to open it; the manifest now living in the spec's Screens & Flows section; the walkthrough script for the six flows; the four asks I declined with what I built instead and what it would take to reverse each; F-1 through F-8 as the questions Thursday should settle; and one plain sentence she cannot miss — **this mock has not been reviewed by anyone but me, I will not mark it approved, and Thursday's review is what turns it into something the stories can be frozen against.**