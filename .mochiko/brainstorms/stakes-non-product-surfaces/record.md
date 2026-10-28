# Stakes key for non-product surfaces — decision record

Status: **accepted** 2026-10-28 (user's word: "accept") · **BUILT 2026-10-28 at v0.124.0** (branch `adaptive-rigor-build`, three waves, migrations `0105`–`0107`; `build-log.md`, gate audits `reports/audit-w1.md`, `audit-w2.md`, `audit-w3.md`, all PASS; OQ2 and OQ3 decided at build; OQ1 and bets A–C read on the user's own first runs) · landed: `DECISIONS.md` row 2026-10-28 (rows 2026-10-27 and 2026-09-19 annotated), `BACKLOG.md` OQ2 item to the trail + new item *Planning-output stakes build* (gated on DF3), `ROADMAP.md` Next touched, index entry updated · DF3 withdrawn 2026-10-28 by the user's word, build opened on `adaptive-rigor-build` · frame hardened 2026-10-28 (user: "confirmed" — every line, size standard) · ruled 3 (D1–D3) · defaults DF1–DF6 confirmed · deciding finished 2026-10-28 — map empty · accept screen confirmed · cold review round 1 landed (0C/4I/10M), every survivor dispositioned by the user's word · verify pass pending

## Topic (user's words)

2026-10-28, after the v0.123.0 adaptive-rigor build: "in the last session, and version we made
changes to implement, can we rollout to other workflows too?" — then, after the lead's answer
that the rollout waits on an unruled stakes key: "what does that mean, use simple language with
example" — then `/mochiko:brainstorm` "based on context above".

## Orientation

Session index read 2026-10-28 (`.mochiko/brainstorms/index.md`); newest entry
`implement-lead-adaptive-rigor` (accepted 2026-10-27, built 2026-10-28 at v0.123.0). This session
is that record's OQ2, handed to a later session by the user's word.

Facts in hand (quoted; readings marked apart):

- F1 — `BACKLOG.md:31-35`: "**Stakes key for non-product surfaces — later session** … what
  "stakes" means for a plugin-primitive edit, a governance surface or a brainstorm record, and
  whether the gate jobs (`author-grader-consolidation` D2, D3, D7; GI-004) may ever thin under
  it; until ruled, non-product surfaces have no band and those reviews run in their fixed forms."
- F2 — `plugins/mochiko/migrations/0102-sound-loop-stakes.yaml:37-46`
  (`patterns-sound-loop.stakes-key`): "The stakes key is the harm class of the run's product
  work: the worst kind of thing that breaks if the run is wrong, read at run-open from one cited
  line, from a closed list — money · data (loss or corruption) · security (or trust) ·
  availability · wrong-answer (shown) · cosmetic."
- F3 — same file, lines 81-83 (`patterns-sound-loop.kind-keyed-library-wide`): "where the
  reviewed object is not product work — a plugin primitive, a governance surface, a brainstorm
  record — no harm class applies, there is no band, and the review runs in its fixed form."
- F4 — same file, lines 116-117 (`patterns-sound-loop.review-form-invariants`): "The gate jobs
  keep their fixed forms: the primitive-edit audit and setup's governance validation."
- F5 — same file, lines 55-62 (`patterns-sound-loop.stakes-band`): "Money, data, security: no
  step outside the floor thins, whatever any probe says. Availability, wrong-answer, cosmetic: any
  `class: must` verification step the command keys on `stakes` may thin, and only the four
  non-leg steps … may be skipped, stated".
- F6 — `.mochiko/brainstorms/author-grader-consolidation/record.md:216-218`
  (D2-gate-input-grading): "**Gate** — a binary PASS/FAIL the lead cannot ship past — applies
  only where the artifact leaves the repo with no human ruling behind the grade".
- F7 — `.mochiko/brainstorms/implement-lead-adaptive-rigor/record.md:197-198` (frame change 2):
  "implement's floors are re-keyed in this session; another command's floors are re-keyed on its
  next touch, the shared-skill change reaching it by pointer until then."
- F8 — same record, lines 209-210 (frame change 3, cold review S1): "the harm classes have no
  meaning for a spec, a governance file or a record." *(Corrected for the spec by
  D2-borrow-product-label — S5.)*

Facts added after the frame hardened (2026-10-28; renders by `mochiko-cli rules <command> --section <id> --plugin-root plugins/mochiko`, binary 0.4.0, plugin 0.123.0, cited by rule id):

- F9 — specify's reviews: `spec.plan-approval` (plan-QA per producing seat) · `spec.blind-journey-list`
  ("one fresh `general-purpose` teammate spawned with `model: opus` … returns the journeys it
  sees") · `spec.stress-test-one-pass` ("The stress-test grades spec + stories + capability/row
  derivation + map delta in one pass").
- F10 — feature's reviews: plan-QA (`feat.author-grader`) · `feat.card-blind-list` ("Once per visit
  that cuts more than one card or a growth row … a visit that cuts a single bug or improvement
  car[d]" is carved out) · `devils-advocate` "grades the card against the visit file for sameness
  of meaning" · `feat.sound-loop-floor`.
- F11 — architecture's reviews: plan-QA (`arch.author-grader-separation`) · `arch.focus-blind-list`
  · `arch.seat-tech-lead-grader` ("tech-lead is the independent grader of the architect's judgment
  writes, before the user's ratification") · `arch.sound-loop-floor`.
- F12 — setup's reviews: plan-QA (`setup.plan-approval-producers`) · `setup.stress-test-cold-seat`
  + `setup.blind-map-dispatch` (blind cold stress-test of governance intent) ·
  `setup.validate-seat-form` ("The validate step's grader is a plain fresh seat that authored no
  surface, running `mochiko:validation-constitution` with its floors unchanged") — the gate job.
- F13 — brainstorm's reviews: `brainstorm.blind-map-dispatch` + `brainstorm.record-review-independence`
  (blind map, then cold end read) · `brainstorm.blind-second-list` · `brainstorm.plan-approval-producers`.
- F14 — the primitive-edit audit's form: `.claude/rules/mochiko/primitive-edits.md` "**Check** — one
  binary gate audit over one unit, run by **the gate grader**".
- F15 — unknown: no measured cost per review step exists in the repo the lead has read; a fork
  that leans on "which review is heavy" carries it as open.

Reading (lead's, not a fact): F10's single-card carve-out shows feature already scales one step by
size, the axis `patterns-sound-loop.stakes-key` forbids for the product key ("never a count of
files, cards, rows, gaps or users").

Reading (lead's, not a fact): F1 names three surfaces (primitive, governance, record); F8 adds the
spec. The feature map and the architecture store are named nowhere in F1–F8 as product or
non-product — the record's scope line is silent on them.

## Frame card

*(shown 2026-10-28; hardened at the user's first reply, "confirmed" — every guessed line
confirmed as written, size ruled standard; no frame change since — heading refreshed at review,
S14)*

- **Problem** *(lead's guess, confirmed)* — the leads of `/mochiko:specify`, `/mochiko:feature`,
  `/mochiko:architecture`, `/mochiko:setup` and `/mochiko:brainstorm`, and the maintainer landing
  a plugin-primitive edit, cannot match review depth to how costly a mistake would be: every
  review runs its fixed form whatever the change, because the six harm classes describe broken
  product code only (F3, F8).
- **Destination** *(lead's guess, confirmed)* — a ruled answer, per non-product surface, to "how
  bad could a wrong one be?" (a key, or a ruled "no key here"), plus a ruling on whether the two
  gate jobs may ever run lighter — enough that each command can be re-keyed on its next touch (F7).
- **Must not break** *(lead's guess, confirmed)* — `patterns-sound-loop.never-thins` (user gates,
  no output cleared by its author, the trail, one non-author read) · the gate jobs' default-FAIL
  posture and tamper-proof clause (`author-grader-consolidation` D3-gate-default-fail) unless this
  session rules otherwise · implement's shipped product key and band (F2, F5) · raising rigor stays
  free, lowering stays the user's ruling.
- **Betting on** *(lead's guess, confirmed)* — (A) the cost of a wrong spec, map, store,
  governance file, record or primitive can be read at run-open from one cited line, as product
  harm is — if wrong, the key becomes ceremony; (B) these surfaces' reviews are heavy enough that
  thinning pays — if wrong, there is nothing to gain and the answer is "no key"; (C) a planning
  surface can borrow the harm class of the product work it will drive — if wrong, it needs its own
  list.
- **Out of scope** *(lead's guess, confirmed)* — the build that re-keys each command (a later
  build item) · the product-run key and band themselves (ruled, `implement-lead-adaptive-rigor`
  D3-harm-band-smoke, D5-band-step-latitude) · the code-standards low/high `depth` dial · the Rust
  CLI grammar (a `when:` key fits the existing form).
- **Size** *(lead's proposal, ruled standard)* — standard. Fog at ruling: one key or one per
  surface · borrow versus own list · whether gate jobs thin · which surfaces count as product work.

## Decision map

*(drawn 2026-10-28 with Q1; never put to the user for approval; redrawn after D1 and again when
the blind map landed; final state below — refreshed at review, S14)*

- **Ruled** — *Which outputs get a lever, and how far* (D1-planning-quick-read; blind map B1, B3,
  A6, H1, C3, B2, B4) · *Label shape* (D2-borrow-product-label; blind map A4, A6, C1, C2, H2, H3) ·
  *Who picks the label* (D3-lead-proposes-label; blind map F2, G1).
- **Defaults, confirmed** — DF1 *The two locked reviews keep their fixed forms* (blind map E1, E2,
  B5) · DF2 *The surfaces the six missed* (blind map D1) · DF3 *The build waits for a trial read of
  the product lever* (blind map A1) · DF4 *What else a low label lowers* (blind map C3, E3) · DF5
  *Where the label is read* (blind map F1, F3) · DF6 *No-cite default* (blind map C4).
- **Ruled out** — *Brainstorm's kept conditions* (D1: records get no lever).
- **Askable / in fog** — none. *(map empty 2026-10-28)*
- **Out of scope** — per the frame card.

**Blind map fold (2026-10-28).** `reports/angle-map.md` landed: 58 angles, 18 load-bearing, nine
classes, `record_contact: none`, one incidental exposure disclosed (this slug's index When/About
lines). Folded, each marked on the ruled item or default it bears on: A1, A4, A6, B1, B3, C1, C2,
C3, D1, E1, E2, E3, F1, F2, F3, G1, H1, H3; B2 (the shipped rules a ruling supersedes or keeps) and
B4 (kinship with the rejected surface-value tiering road) carried to D1 as duties; B5 (GI-004's
amend route) rides DF1. Dropped for the end read: every other angle (A2's cost point is F15
already; A3, A5, E4, G2–G7, H4 and classes I and the rest of B, C, D, E, F as written in the
map; C4 and H2, first dropped, were taken up at the defaults batch and Q2 re-put — DF6 and D2 —
and are listed folded above, V3). Verified by the lead before folding: B1 against `patterns-sound-loop.never-thins`; B3
against `author-grader-consolidation` record lines 330–337 (D8-input-job-conditions: "the
completed-hunt rule … the blind angle map where a skill carries one, the evidence floor,
never-fix-what-you-grade, and the cold/blind dispatch floors all stay"); A6 against
`patterns-sound-loop.exemptions-exactly-three` ("fix-on-sight integrity repairs (typos, status
agreement, dead pointers)"); A4 against the `implement` preamble's `stakes` condition ("each read
from its own spec lines").

## Session trail

- Q1 (2026-10-28) — *Which outputs get a lever.* Put with the F9–F15 inventory; unblocks label
  shape and brainstorm's kept conditions. *(Options restored at review, S14.)* A — the three
  planning outputs only (spec, feature map, architecture store), governance, records and plugin
  files kept full · B — all six outputs, the gate jobs ruled apart · C — none, the cheaper shape;
  the rejected road — each command decides at its next touch (five meanings of "light"). Lead's
  pick A. User (2026-10-28): "as recommended" — A ratified (streak 1). D1's first form: the lever
  on the three planning outputs, inside the existing floors. Consequence: *Brainstorm's kept
  conditions* leaves the fog, ruled out by D1; *The two locked reviews* both review surfaces D1
  keeps fixed — carried to the defaults batch as DF1 rather than its own turn (lead's call, the
  user may pull it out).
- Q2 (2026-10-28) — *Label shape*, for the three outputs D1 names. Blind second list dispatched
  first (`brainstorm.blind-second-list`: a fork costly to undo), its seat given the frame and the
  question only (fresh `general-purpose`, `model: opus`). Its roads (`blind list`): (1) inherit
  the product class · (2) max over touched FEAT/AX rows · (3) a new closed list of planning failure
  modes · (4) reversibility / retrofit cost · (5) change-kind table per surface · (6)
  catch-distance — thin where a later independent gate re-reads · (7) fixed per surface, no read ·
  (8) two axes, product class × reversibility or catch-distance. Its deciding question: "is a
  planning artifact's stakes the harm of the product it steers (inherit), or the harm of the
  artifact's own failure mode". Lead's list: borrow · own list · split by surface. Put to the user
  as A borrow (folding road 2) · B own list wording/scope/boundary (road 3; road 5 a likely
  reading of it) · C split; roads 4, 6, 8 named and not carried, with reasons (4 hard to cite from
  one line; 6 compounds — each stage assumes the next catches; 8 two reads, two rulings); road 7 is
  ruled out by D1. Fact for road 6: `mochiko:review-sufficiency` description — "over the spec, the
  architecture store, and the product baselines". Lead's pick B. Unanswered when the blind map
  landed; paused — the map's B1, B3, A6, H1 bear on D1, which Q2 hangs on.
- Q3 (2026-10-28) — *D1 back with the user* (`brainstorm.blind-map-fold`: a ruling already made
  that a late angle bears on goes back to the user). Options: A keep D1 inside the existing
  floors (lighten only plan-QA depth, the blind list, a changed-part read) · B go further —
  supersede part of D8-input-job-conditions and `patterns-sound-loop.never-thins` so a low-stakes
  change skips the big review · C step back to no lever (H1). Lead's pick A. User asked for it in
  simple language (re-put plainly), then asked "why not B?" (lead's case against B: no smoke
  analog for documents; the author's lead would propose skipping its own review; two prior rulings
  undone; no trial evidence; a middle path "A now, revisit B on a trigger" offered). User
  (2026-10-28): "lets do B" — against the lead's pick: **Contested**. Streak reset.
  Lead's error surfaced in the same turn: the lead's plain-language example for B ("no reviewer
  reads it. Only the plan gets checked") contradicts the hardened frame's must-not-break line
  ("one non-author read" never thins). Clarifying question put: B1 — skip the big review, keep one
  quick non-author read of the change · B2 — no non-author read (a frame change, the user's word
  needed). Lead's pick B1. User (2026-10-28): "as recommended" — B1 (streak 1). D1 re-carded.
- Q2 re-put (2026-10-28) — *Label shape*, now keying whether the full review is skipped. Lead's
  pick changed from B (own list) to A (borrow), on blind map A6 (the own list's low end is mostly
  typos, already exempt) and A4/H2 (a spec's harm class is already read from its spec lines at
  implement; borrow adds no new list). User: "as recommded" — A (streak 2). D2 recorded.
- Q4 (2026-10-28) — *Who picks the label* (blind map F2, G1). Unblocks the defaults batch.
  User: "as recommded" — A (streak 3, flagged). D3 recorded.
- Defaults batch put 2026-10-28, case-against-first per the streak rule. User: "confirmed" —
  DF1–DF6. **Deciding finished 2026-10-28** — map empty: nothing askable, no fog, no open frame
  line, defaults confirmed, the blind map landed and every angle folded or listed as dropped.

## Decisions

### D1-planning-quick-read — Only the three planning outputs get a stakes lever, and at low stakes it skips their full review while one quick non-author read stays

*(Joined ID re-slugged at review from `D1-planning-outputs-skip-full-review` to three words — S13.)*

**Statement.** The stakes lever reaches the outputs of three commands' own runs — specify's spec
and its map delta, the feature desk's map writes, the architecture desk's store writes — and
nothing else: governance surfaces, brainstorm records and plugin primitives get no lever, every
review of them running its fixed form, as `patterns-sound-loop.kind-keyed-library-wide` already
says (F3). *(Boundary narrowed at review, S9.)* A store write made inside `/mochiko:implement`'s
design phase follows implement's own run stakes, as today; product baselines and epics stay
outside the lever (DF2). On the three outputs, a change the stakes label reads as low skips the
full review — the completed open hunt of the full stress-test or grade (`spec.stress-test-one-pass`,
`devils-advocate`'s card grade, `arch.seat-tech-lead-grader`) — and takes one quick read of the
change by a seat that did not author it, before the user rules it. *(S10: "the blind angle map"
struck from that list — none of the three reviews carries one.)* The quick read keeps
`patterns-sound-loop.review-form-invariants` (1)–(4): never the author, the surface itself,
starting at FAIL, at or above the producer's tier.
*(Added at review, S4.)* The quick read keeps each review's hard checks and drops its open hunt:
in specify, a fresh non-author seat still runs the prototype walk on a UX-bearing feature
(`spec.stress-test-prototype-walk`) and the spec–stories–derivation–map-delta agreement check of
`spec.stress-test-one-pass`, and skips the open gap hunt (missing requirements, ambiguities,
edge cases); at low stakes `spec.fail.blocking-gap` and `spec.fail.screens-flows` are enforced by
the quick read as far as its hard checks reach, the unrun hunt disclosed *(reworded at verify,
S4 residual)*. "The change" is what the run wrote: a new spec is read whole. The
same shape holds for the map card grade and the store grade — each keeps its hard checks
(`feat.grader-checks-joined-ids`, `arch.grader-checks-joined-ids`, sameness of meaning against
the visit file) and drops its open hunt. *(Rewritten at review, S2/S3.)* Each rule or ruling
D1 moves or keeps, by name — the build lands exactly this list:
1. `author-grader-consolidation` D8-input-job-conditions — **bent in part**: the completed-hunt
   rule only, for the spec stress-test only, at low stakes only; the headline verdict word, the
   evidence floor, never-fix-what-you-grade and the cold dispatch floors stay. The quick read
   reports the hunt as "open hunt not run", never "no gaps". (D8's input-job list names specs;
   it does not name the map card grade or the store grade, so nothing of D8 bends there — S10.)
2. `patterns-sound-loop.never-thins` — **kept whole**: the quick read keeps every record-guarding
   check (prototype drift, the derivation and map-delta agreement, joined IDs, sameness of
   meaning); the skipped open hunt finds gaps, it does not guard the record (lead's reading,
   ruled by the user at R2).
3. `patterns-sound-loop.kind-keyed-library-wide` — **rewritten**: the key and band bind
   product-run reviews and the reviews of the spec, the feature map and the architecture store.
4. `patterns-sound-loop.stakes-key` — **rewritten**: a planning output's class is read from the
   product work it steers (D2-borrow-product-label).
5. `patterns-sound-loop.stakes-band` — **rewritten**, a planning clause: no smoke precondition;
   the full grade may become the quick read; the three commands' blind lists join the skippable
   steps (DF4).
6. `patterns-sound-loop.review-form-invariants` — **widened**: invariants (1)–(4) bind the quick
   read; its form is fixed by D1, not the lead's free pick.
7. `spec.fail.blocking-gap`, `spec.fail.screens-flows` — **kept**; their `enforces:` lists gain
   the quick read, which enforces them as far as its hard checks reach, the unrun hunt disclosed.
8. `implement-lead-adaptive-rigor` frame change 3 and D8-home-sound-loop's scope clause ("The key
   and the band bind product-run reviews only") — **superseded in part** for the three outputs.
9. `author-grader-consolidation` D1-universal-grader-independence as amended 2026-10-27
   (`DECISIONS.md` row 33: "the grade's grain and form are the lead's inside the stakes band; the
   surface-value rejection stands for grades") — **kept**: a grader always reads; D1 widens only
   where the band reaches.
10. `mochiko:review-specifications` — **gains** the quick-read form (a new rule, no supersession).
11. `/mochiko:feature`'s card grade (`devils-advocate` against the visit file) and
    `arch.seat-tech-lead-grader` — **gain** the quick-read form, each keeping its hard checks
    (new rules, no supersession) *(added at verify, V1)*.

*(Revert route, added at review — S12; widened at verify — V2.)* If the user's trials show a
low-label change whose skipped full review let a defect through, the revert is one migration:
items 3–7 return to their prior text — the planning clause leaves
`patterns-sound-loop.kind-keyed-library-wide`, `stakes-key`, `stakes-band` and
`review-form-invariants`, and the two specify fail rules' `enforces:` lists drop the quick read —
the three reviews return to their fixed forms, and the quick-read rules (items 10 and 11) retire.

**Why.** The user's call, against the lead's pick: the saving worth having is the full review,
and staying inside D8-input-job-conditions leaves only the plan check, the blind list and a
changed-part read to lighten (blind map B1, B3, A6). The frame's must-not-break line — one
non-author read never thins — holds: the full review goes, the outside read stays.

**Rejected roads.** A — keep the lever inside the existing floors (the lead's pick at Q3): saves
little once typos are already exempt. C — no lever on non-product outputs (blind map H1): rejected
at Q1 and again at Q3. B2 — no non-author read at low stakes: breaks the hardened frame's
must-not-break line. All six outputs (Q1's B): reaches the likely-heaviest reviews at the price of
thinning the record layer's own checks. Each command deciding at its next touch (Q1's rejected
road): five meanings of "light".

**Accepted risk.** (1) No smoke analog: a document has nothing to run, so the label is the only
thing standing between a low-stakes change and a skipped full review (blind map C3) — a wrong
label lets a meaning-changing edit through on one quick read. (2) The quick read of the change
alone can miss a clash with the rest of the output. (3) No trial evidence for the product lever
yet (blind map A1). (4) *(Rewritten at review, S1.)* Kin to the surface-value tiering road
`author-grader-consolidation` D1-universal-grader-independence rejected (blind map B4): D2's label
does read the product area, as implement's does. The ground D1 stands on is the grader's form —
the 2026-10-27 amendment lets "the grade's grain and form" thin inside the stakes band while "the
surface-value rejection stands for grades", and the quick read is a grade with a non-author
reader; were the quick read ever to lose its reader, D1 would cross into the rejected road.

**How it was decided.** Q1, 2026-10-28: A ratified ("as recommended") — first form, planning
outputs only, inside the floors. Q3, 2026-10-28, after the blind map's B1/B3/A6/H1 bore on it:
options A (keep inside the floors, lead's pick) · B (go further) · C (no lever); re-put in simple
language on request; "why not B?" answered with the lead's case; user: "lets do B" —
**Contested**. The lead's plain example for B contradicted the frame's must-not-break line; B1/B2
clarification put, lead's pick B1; user: "as recommended" — B1. Provenance `user's words` (B),
`lead's guess, ratified` (B1). Confidence: Contested.

**Changed at review.** S4 (Important, 2026-10-28): the quick read defined — hard checks kept,
open hunt dropped; specify's two floor fail conditions enforced by it at low stakes. Put as R1:
A keep the hard checks · B a plain look, the two floors rewritten to full-review-only · C a new
spec always full, the quick read for edits only (the cheaper shape). User: "as recommded" — A
(streak 4). The extension to the map card grade and the store grade is the lead's reading of A,
shown to the user at R2. S1, S2, S3 (Important, 2026-10-28): the supersession list named in full
(ten items); accepted risk 4 rewritten on the grader-form ground, consolidation D1 kept; the
never-thins narrowing withdrawn — the quick read keeps every record-guarding check, so the frame's
must-not-break line holds and no frame change is stamped. Put as R2 case-against-first; the user
asked "think harder and recommded"; the lead re-read row 33 and never-thins and changed two leans
(consolidation D1 bent → kept; never-thins narrowed + stamp → kept whole, no stamp), the case
against shown (a strict reader may hold the hunt guards the record). User: "as recommded" —
option (1) (streak 5, the recommendation asked for). R3 (2026-10-28, user: "confirmed"): S9 the
boundary narrowed to the three commands' own outputs, implement's design-phase store writes on
implement's stakes; S10 the blind angle map struck from the skipped list and D8's reach stated;
S12 the revert route added; S13 re-slugged; S14 the rejected roads restated without "as before".
Verify (2026-10-28, `reports/verify.md`, user: "confirm"): V1 item 11 gives the map and store
grades their quick-read home; V2 the revert covers items 3–7, 10 and 11; S4 residual — the
specify fail rules enforced as far as the hard checks reach, the unrun hunt disclosed.

### D2-borrow-product-label — The three planning outputs borrow implement's harm classes; no new list

**Statement.** A planning output's stakes label is the product key's own closed list (F2), read
from the product work the output steers: a spec takes the harm class of the feature it specifies,
read from its own spec lines as implement's sufficiency seat already reads it (blind map A4) —
*(source named at review, S6)* at specify's intent confirm, before any spec line exists, the
citable line is the confirmed intent synthesis, which "lands verbatim as the spec's Intent
section" (`spec.intent-synthesis-governs`); a feature-map or architecture-store change takes the
highest class among the capabilities or store elements it touches. *(Added at review, S7.)* Where
one review grades more than one output — specify's one-pass stress-test grades the spec and its
map delta together — the higher of their labels governs. Money, data, security: the full review
runs. Availability, wrong-answer, cosmetic: the full review may be skipped under
D1-planning-quick-read. Where no line can be cited, the class reads as money, data or security, as
for product work. *(Added at review, S5.)* This corrects F8's premise for the spec — "the harm
classes have no meaning for a spec" — that `implement-lead-adaptive-rigor` frame change 3 rested
on: a spec carries the harm class of the product it steers, as implement's own `stakes` condition
already reads it.

**Why.** It reuses a list implement already reads and adds no second vocabulary beside it. The
own-list road's low end — rewording with meaning unchanged — is mostly typos, which
`patterns-sound-loop.exemptions-exactly-three` already exempts (blind map A6), so it would rarely
come out low and D1's saving would not arrive.

**Rejected roads.** An own list of change kinds — wording · scope · boundary (blind list road 3,
the lead's first pick). No label, the lead proposing quick or full each run (blind map H3). Two
labels at once (blind list road 8). Reversibility (road 4) and catch-distance (road 6), named at
Q2 and not carried.

**Accepted risk.** The label judges the product area, not the size of the change: a new story on a
cosmetic feature takes the quick read, a one-word fix to the refund spec takes the full review.
Map and store changes that span features will mostly read high and rarely lighten (blind list
road 1's risk). Taking the maximum over touched rows must stay a harm read, never a row count
(`patterns-sound-loop.stakes-key`: "never a count of files, cards, rows, gaps or users").

**How it was decided.** Q2, put 2026-10-28 with the blind second list's differences shown; paused
for D1's re-ruling; re-put with the lead's pick changed from B to A and the reason given. User:
"as recommded". Provenance `lead's guess, ratified` (road 1 also `blind list`). Confidence:
Confident. Streak 2.

**Changed at review.** R3 (2026-10-28, user: "confirmed"): S5 F8's corrected premise stated; S6
the intent synthesis named as the citable source at specify's intent confirm; S7 the higher label
governs a review that grades two outputs.

### D3-lead-proposes-label — The lead proposes the label with its cited line, the user rules it, and the quick reader may raise it

*(Joined ID re-slugged at review from `D3-lead-proposes-reader-raises` to three words — S13.)*

**Statement.** In specify, feature and architecture — no entry grading seat — the lead proposes the
D2-borrow-product-label label with its one cited line, per `patterns-sound-loop.stakes-key` ("the
lead where none exists"); the user rules it. The seat taking the quick read under
D1-planning-quick-read may raise the label when the change touches a money, data or security
surface; a raise voids the skip and the full review runs
(`patterns-sound-loop.raise-free-lower-ruled`). *(Reworded at review, S11.)* No seat lowers it; a
lowering is the user's checkpoint ruling. *(Added at review, S6.)* A raise restores the full review
only: a blind list already skipped under DF4 is not re-run, its moment passed — the full review's
open hunt covers missing journeys.

**Why.** It reuses the shipped rule and adds one independent check where the author's side
proposes (blind map F2): the reader, who never authored the change, sees the change itself.

**Rejected roads.** A fresh seat proposes the label at entry — independent, one more seat every
run. The user picks with no proposal — cheapest, but the user then reads the spec to find the
line the lead would cite.

**Accepted risk.** A rubber-stamped proposal leaves the quick reader as the only check, and a
reader of the changed part alone may miss that the change reaches money elsewhere (blind map G1).

**How it was decided.** Q4, 2026-10-28, A/B/C with the case against A. User: "as recommded".
Provenance `lead's guess, ratified`. Confidence: Confident. **Streak 3** — flagged to the user;
the defaults batch put case-against-first (`analysis-iterative.ratified-streak`).

**Changed at review.** R3 (2026-10-28, user: "confirmed"): S11 the lowering reworded to match
`patterns-sound-loop.raise-free-lower-ruled`; S6 a raise does not re-run a skipped blind list;
S13 re-slugged.

## Defaults batch

*Confirmed on one screen 2026-10-28 (user: "confirmed"), put case-against-first after the streak
flag. Provenance on each: `lead's guess, ratified`; blind-map angles named.*

- **DF1 — The two locked reviews keep their fixed forms.** The primitive-edit audit and setup's
  governance validation do not thin; follows from D1-planning-quick-read (governance and
  primitives get no lever) and avoids a GI-004 amend (blind map B5). Case against shown: setup's
  validate step has a user acceptance after it (blind map E2). Lean firm.
- **DF2 — The surfaces the six missed get no lever.** Product baselines, epics, `gates.md`,
  operating docs, crate code and migrations keep their fixed reviews (blind map D1). Case against
  shown: baselines and epics lead to code too. Lean weak — widening later is cheaper than
  narrowing.
- **DF3 — The build waits for one real trial of the product lever.** No command is re-keyed under
  D1–D3 until the user has run implement's v0.123.0 lever on a real project and judged it fine
  (blind map A1; `implement-lead-adaptive-rigor` D9-evidence-users-trials, the user's own trials).
  Case against shown: delays the ask; the levers differ. Lean weak. **Withdrawn 2026-10-28 by the
  user's word** after acceptance — offered A keep the gate · B drop it and build now · C build now,
  hold the bump for the trial (lead's pick C); user: "happy for you to build on the branch. no need
  to wait" — the build opens at once on `adaptive-rigor-build`, the trial no longer a precondition
  (bet B's test and OQ1 move to the user's first runs after the build).
- **DF4 — What else a low label lowers.** At low stakes the blind list may be skipped, stated in
  the run's report; the plan check lightens and never vanishes — as the product band treats the
  same steps (`patterns-sound-loop.stakes-band`). Case against shown: the blind list is cheap.
  Lean firm.
- **DF5 — Where the label is read.** On each command's existing confirmation point — specify's
  intent confirm, a desk visit's goal line — never a new gate; a visit with nothing to lighten
  states no line (blind map F1, F3). Case against shown: the build picks the point per command.
  Lean firm.
- **DF6 — No-cite default.** Where no line can be cited, the class reads money, data or security
  and the full review runs (blind map C4). Case against shown: map and store will mostly read
  full. Lean firm.

## Open questions

- OQ1 — *(unknown, F15)* — what the full review and the quick read each cost per run in specify,
  feature and architecture. No measure exists; the first trial runs under the build can read it
  (bet B's test).
- OQ2 — *(build-time, DF5)* — which existing confirmation point hosts the label in each of the
  three commands. Left to the build's plan, graded at its audit.
- OQ3 — *(build-time, added at review — S8)* — where specify records a lowering line:
  `patterns-sound-loop.trail-of-lowerings` (floor) records each lowering "in the command's final
  report file", and specify's rendered rules name none (feature and architecture have the visit
  file). Left to the build's plan beside OQ2, graded at its audit.

## Review

**Accept screen confirmed 2026-10-28** ("confirmed"): D1–D3 by name with gists, DF1–DF6, the out-of-scope
line, OQ1–OQ2, and bets A–C with their tests — A: count how often the first trial runs cite a real line
against the no-cite default; B: measure a full review against a quick read in the first trial run
(closes OQ1); C: count how often map and store changes read low, D2 revisited for them if almost never.
**Record frozen 2026-10-28** for the cold end read; the review seat (the blind-map `mochiko:devils-advocate`
seat on `mochiko:review-brainstorm`) resumed with this file's path. No writer touches the record until
its verdict lands (`patterns-transport-floor.quiesce-before-cold-grade`).

**Cold end read landed 2026-10-28** — `reports/cold-review.md` (round 1, `record_contact: read`):
recommended status needs-revision · 16 raised, 14 survived · Critical 0 · Important 4 (S1–S4) ·
Minor 10 (S5–S14) · 2 withdrawn on the seat's own checks · coverage: B2, B4 taken thin (S2, S1);
dropped D4, D5, F4, G7 unanswered (S9, S7, S8, S12). Fan-in confirmed: one deliverable expected,
one landed. The lead verified S4's two floors against the specify render before routing
(`spec.fail.blocking-gap` "enforces: spec.stress-test-one-pass"; `spec.fail.screens-flows`
"enforces: spec.prototype-craft, spec.stress-test-prototype-walk").

Routing (`brainstorm.non-coverage-survivors`, `brainstorm.coverage-survivor-routing`): S4 its own
turn (a real choice, costly) · S1 + S2 + S3 one turn (the supersession list and the frame stamp,
each the user's word) · S5, S6, S10, S11, S13, S14 the lead's repair batch · S7, S8, S9, S12 the
coverage candidates, each offered explore now · rule inline · defer, with the lead's inline text.

- R1 (2026-10-28) — S4 put: what the quick read checks. User: "as recommded" — A. D1 amended.
  Streak 4: R2 put case-against-first.
- R2 (2026-10-28) — S1 + S2 + S3: D1's full supersession list, `author-grader-consolidation`
  D1-universal-grader-independence kept or bent, and the frame stamp on the must-not-break line.
  User: "think harder and recommded" → the lead's firm re-recommendation; user: "as recommded" —
  (1). D1 amended; no frame change.
- R3 (2026-10-28) — the repair batch (S5, S6a, S10, S11, S13, S14) and the small rulings (S6b, S7,
  S8, S9, S12 — the four coverage candidates ruled inline) on one screen. User: "confirmed".
  Landed: D1 (S9, S10, S12, S13, S14) · D2 (S5, S6a, S7) · D3 (S6b, S11, S13) · OQ3 (S8) · the
  frame card and decision map labels and angle ids (S14) · joined citations across the record
  (S13; F1's quote left verbatim).

Every survivor S1–S14 dispositioned by the user's word. Next: the review seat's one bounded verify
pass over the changed cards and their gists (`brainstorm.fix-one-card-verify-once`).

**Verify landed 2026-10-28** — `reports/verify.md` (round 2): recommended needs-revision · fixed 12
(S1, S2, S3 by the R2 ruling, S5–S11, S13, S14) · partly fixed 2 (S4, S12) · verify-born V1
(Important — the quick read's map and store home missing from the "exactly this list"), V2 (Minor
— the revert left items 4, 6, 7 rewritten), V3 (Minor — C4, H2 listed both folded and dropped).
The lead's four one-line repairs put in one named batch; user (2026-10-28): "confirm". Landed on
D1 (V1, V2, S4 residual) and the fold paragraph (V3). One bounded re-verify of these lines next;
a second failed verify goes to the user.

**Re-verify landed 2026-10-28** — `reports/re-verify.md` (round 3): 4 checked · 4 fixed (S4
residual, V1, V2, V3) · 0 not fixed · 0 broke something · 0 new · recommended status ready. Verify
bound closed. After-review screen shown to the user.

**Accepted 2026-10-28** — user's word: "accept". Close ritual run (`.mochiko/memory/knowledge-management.md`
landing ritual): `DECISIONS.md` row appended, the 2026-10-27 and 2026-09-19 rows annotated for the
partial supersessions · `BACKLOG.md` *Stakes key for non-product surfaces — later session* moved to
the trail, *Planning-output stakes build* opened in its place · `ROADMAP.md` Next (Floor builds)
touched · index entry updated. Next-step offer (`brainstorm.next-step-offer`): none by default — the
build waits on DF3.
