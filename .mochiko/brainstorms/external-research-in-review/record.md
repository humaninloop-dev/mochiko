# External research in validation & adversarial review — session record

**Status:** accepted (2026-08-04) — solo cold review 6/6 dispositioned, verify CLEAN
**Opened:** 2026-08-04
**Topic:** In setup, plan, and brainstorm — how is research outside repo scope (web, official docs, registries, standards) used to validate claims or feed adversarial review; and should it be.
**Sibling:** `adversarial-review-generality` (same day — that session ruled on angle coverage; this one on fact substrate).

---

## Ground facts (Explore-mapped, 2026-08-04)

**Headline: exactly one mandated external-research touchpoint pipeline-wide** — setup's domain-dependency seed arbitration (`DOMAIN-DEPENDENCIES.md:38-39`): trust sources "re-verified live at seed time, never copied as snapshots"; unlisted ecosystems get "live research constrained to filling hierarchy levels… never free-form judgment"; level citation exists so a hallucinated source is challengeable. Zero occurrences of WebSearch/WebFetch/context7 anywhere in `plugins/mochiko/` or `.claude/rules/mochiko/`.

Every review seat is repo-grounded by construction:

- **setup / review-governance-intent:** reality facts checked "against the analysis or the files"; user-declared facts routed to the user; no web route for either fork (`SKILL.md:79-84`).
- **plan / review-feasibility:** strictly cross-artifact; impossibility verdicts ("physics" calls on NFRs) rest on model memory — no license to consult vendor docs/benchmarks (`FEASIBILITY-LENS.md:26,62`).
- **plan / review-plan-artifacts:** repo mechanical checks only (its ground-truth line is "A `failed` count here is ground truth", `SKILL.md:92`; the grep phrasing belongs to `validator.md:102` — citation fixed at review F5).
- **brainstorm / fact-checker seat:** maps and checks claims *about files*; outside-world claims have no defined checking substrate (seat design: `fact-checker-seat/record.md` — "the only live defense against false premises," all cited catches file facts).
- **brainstorm / review-brainstorm:** "verify against the files yourself… an unverifiable claim is a finding, not a benefit of the doubt" (`SKILL.md:33`) — a false external premise the files can't contradict survives as merely "unverifiable."
- **validator evidence hierarchy** (`validator.md:102`): grep > Read > inference — web absent from the hierarchy entirely.

**Unverified-external-claim surfaces (accepted from model memory today):**
1. Plan technology decisions — alternatives' capabilities/maturity/ecosystem scored from judgment (`patterns-technical-decisions` + `EVALUATION-MATRIX.md`); worked example itself asserts "well-maintained, audited library" from memory (`patterns-technical-decisions/references/DECISION-RECORD.md:71` — citation fixed at review F5).
2. Feasibility physics/impossibility verdicts.
3. Fact-profile regulatory consequences — compliance modules map from the library's own summaries, never current regulation text.
4. Shipped tool tables (catalog) — static content, no currency check.
5. Brainstorm outside-repo claims — no substrate (see above).
6. Domain-registry growth at implement — human ruling but no trust-signal re-verification at add time (next amend run re-validates).

**Doctrine posture:** no ban on web research anywhere — the pipeline is silent, not prohibitive; external research is structurally out of every review loop because every fact-authority definition resolves to disk.

**Prior art:** the "live-verified" convention originates in `domain-dependency-allowlist` (2026-07-21, F8 review fold). "Official docs" in decision history = platform docs consulted while designing mochiko (meta-practice, never codified into pipeline primitives). User's own standing practice (memory): design premises verified against official docs, quotable text.

---

## Decisions

**ER-D1 — Scope: one pipeline-wide doctrine, producer and reviewer sites together.** `Confident`
User ruled (Q1: option C, "yes both"): external claims get a defined checking substrate at both producer sites (plan's technology decisions and the like) and review seats (feasibility, brainstorm pair, fact-checker) — not per-surface conventions. Rationale: the map shows one missing primitive everywhere (no fact substrate for outside-world claims); the domain-allowlist discipline (live-verify at use time, cite the source, constrained scope — never free-form) already exists and survived a review cycle; extend it rather than mint siblings. Scope-bloat risk accepted with the mitigation: rule the doctrine once, stage carriers per surface (precedent: security-depth and ops-observability sessions).

**ER-D2 — Trigger: load-bearing-claim rule over a named-class floor.** `Confident` (user adopted recommendation) *(amended by ER-D3: the trigger survives intact but its execution site moved — verification is a review-seat duty, not an authoring-time obligation; producer side keeps only the disclosure line)*
External verification fires when a claim is load-bearing — a decision, verdict, or gate rests on it. Under that judgment rule, a **non-exhaustive floor** of claim classes always fires: version/capability claims · security-posture claims · regulatory content · benchmark/limit numbers. Floor-not-ceiling by design — direct application of the sibling session's AR-D1 lesson (closed taxonomies read as ceilings). Every externally-fed claim carries a disclosure: `verified: <source>` or `memory-asserted` — the disclosure line keeps under-calls of "load-bearing" visible and auditable. **An undisclosed external claim is itself a finding** (added at review F6): the reviewer live-checks floor classes regardless, but the missing line is reported — omission stays visible, the audit signal can't be silently lost. Reviewer-demand-only (option C) rejected: plausible false premises are exactly the ones nobody suspects enough to challenge (the fact-checker blind spot restated).

**ER-D3 — Architecture: verify-at-review, pure.** `Confident` (user-initiated reversal — "this is where the adversarial review is better to do this"; ruled at a no-recommendation fork, both architectures steelmanned)
The adversarial review seat owns external verification: it hunts the artifact's external premises and live-checks the load-bearing + floor-class ones cold (ER-D2's trigger relocates into the review skills). Producers do not verify — they **disclose**: every externally-sourced claim carries `memory-asserted` (or `verified: <source>` if they happened to check); the disclosure line is the producer's whole obligation. Wins: author≠grader at the fact layer by construction (checker never the claimant — motivated reading structurally out); verification concentrates where claims are already under attack; sibling session (`adversarial-review-generality`) hardened exactly this seat. Accepted risks, recorded honestly: (1) a false premise can shape the whole artifact before review catches it — rework cost larger than catch-at-authoring; (2) a waived or skipped review ships unverified claims — the waiver now waives fact-checking too, which belongs on the waiver's stated cost. Rejected: producer-floor hybrid (option B) — splits the duty across seats, reintroducing self-verification exactly on the highest-stakes classes.

**ER-D4 — Mechanics: reviewer checks inline, with the source re-read clause.** `Contested` (user tilted A against the lead's B recommendation; contrast dealt A-vs-C on request; user composed A + re-read from the contrast's push-back)
Review seats run external checks themselves (WebSearch/WebFetch mid-review) — no dispatch protocol, one mind holds claim + context + severity; the adversarial stance mostly points checking the right way (hunting disconfirming sources against producer claims). The structural seam — a finding's own premise sourced by the finding's author — is closed by the **re-read clause**: a finding whose premise is an external claim must cite its fetched source (quotable text, not a summary), and the counterpart reviewer (pair) or the lead (solo) re-reads that source before the finding survives. One cold read between cherry-pick and kill-verdict. Rejected: B (disposable cold checker per claim — dispatch cost + relay loss on every check; the re-read clause buys the independence where it matters at a fraction of the cost) · C (fact-checker web jurisdiction — doctrine forks by stage, seat is conditional in v8, two mechanisms to maintain). Noted for the record: this is the library's first deliberate stance-over-structure call at a fact layer — the re-read clause is the structure kept.
*Fact-checker residual duty (ruled at review F3):* none beyond existing file jurisdiction — reviewers own external claims; the seat keeps mapping/checking file facts, and its map may **flag** external premises as `memory-asserted` for the reviewers' hunt list, but never fetches (flag-don't-fetch — a pre-built hunt list at zero jurisdiction cost).
Pair-review seam (from the contrast, absorbed): two reviewers fetching independently on the same claim resolve source conflicts at cross-exam with the re-read clause applying to both sources — the surviving finding cites the source that survived the counterpart's read. *(Amended at review F1: this resolution is delegated from `CROSS-EXAM.md` by an explicit external-claim carve-out — CROSS-EXAM remains the single pair-protocol home and points at EXTERNAL-CLAIMS.md for this dispute class; without the carve-out, its "route to the fact authority" rule dead-ends, since the fact-checker holds no web jurisdiction. CROSS-EXAM.md touch = shipped-primitive ceremony, priced into the build item.)*

**ER-D5 — No-review paths: the hole is accepted and priced into the waiver.** `Confident` (user adopted recommendation)
Waiving a review now waives external verification with it — the waiver's stated cost names both ("un-reviewed record, externally-unverified claims"). Bare sessions ride the user's own premise-checking practice. No residual gate check (option B rejected: rebuilds verify-at-authoring through the back door on every waived run, against ER-D3). Flagged for the fallback, not ruled: regulatory/compliance claims are the class where user-side checking is weakest — if a shipped wrong regulatory mapping ever materializes, the surgical fallback is option C (mandatory live check on the setup fact-profile's compliance mapping only), recorded here as the named re-open condition.

**ER-D6 — Carrier shape: one shared reference file + thin per-skill pointers.** `Contested` (user ruled B against the lead's A recommendation and maintained after one push-back; push-back itself conceded a half-error — B's shape is the surviving CROSS-EXAM.md pattern, not the purged doctrine-template class)
Mint `EXTERNAL-CLAIMS.md` as a `references/` file under one review skill (home: `review-brainstorm/references/`, per the CROSS-EXAM.md precedent — single-sourced, cited by binders), carrying the whole doctrine: ER-D2 trigger + floor classes · ER-D4 inline-check mechanics + re-read clause · pair source-conflict resolution · disclosure-line grammar. Consumer skills (review-feasibility, review-specifications, review-brainstorm, review-governance-intent, **review-plan-artifacts** — added at review F2) bind it with thin pointers in their own voice; producer surfaces (patterns-technical-decisions, artifact-format) carry only the disclosure-line grammar pointer. *Per-consumer rationale (recorded at review F2):* review-specifications binds because specs carry regulatory/product-legal claims (a floor class; the sibling session's benchmark seeds exactly those shapes there) — the map's sweep missing it was a sweep gap, not an exclusion ruling. review-plan-artifacts binds **thin**: mirror-checklist form gets no judgment duty — one mechanical check row only (floor-class claims in graded artifacts carry a disclosure line; undisclosed = issue); deep verification stays with review-feasibility's adversarial half of the same package. Accepted costs, stated: widest-consumed references/ file in the library (**8 touch surfaces post-fold** — 5 review skills + 2 producer surfaces + CROSS-EXAM.md's carve-out pointer; verify-pass correction of the pre-fold "~6", vs CROSS-EXAM's 2) — edit-time all-consumer guard applies on every touch; cross-skill read dependency in review seats. Won: one authoritative amendment home, no five-way floor-class drift (A's named risk).

---

## Review (solo cold review, 2026-08-04 — devils-advocate, review-brainstorm, spawned solo)

**Tally: 8 formed → 6 survived reviewer's own kill pass → dispositions below (walked one-by-one with the user).** Verdict at review: needs-revision — substrate verified sound (14 load-bearing claims checked, all VERIFIED, two with citation blur), nothing critical. Contested decisions (ER-D4/D6) not re-litigated; F1 attacks a seam neither ruling saw.

| # | Sev | Finding (compressed) | Disposition |
|---|---|---|---|
| F1 | Important | ER-D4's pair source-conflict seam + ER-D6's homing of pair material in EXTERNAL-CLAIMS.md collide with CROSS-EXAM.md's single-source charter and its "checked, never argued — one route per fact" rule; the fact-authority route dead-ends for external claims (fact-checker has no web jurisdiction post-ER-D4) | **User ruled A:** CROSS-EXAM.md gains an external-claim carve-out — external-claim disputes resolve per EXTERNAL-CLAIMS.md (mutual source re-read), delegated by pointer so CROSS-EXAM stays the pair-protocol home. Shipped-primitive touch (strip note + author≠grader audit) priced into the build item. ER-D4 amended at source (D6 needed no amendment — its content already consistent with the carve-out; ledger corrected at verify). |
| F2 | Important | ER-D6 consumer list unreconciled with the surface map — review-specifications bound without a mapped surface; review-plan-artifacts mapped (benchmark floor class in constraints/NFRs) but unbound | **User confirmed fold:** review-specifications stays (regulatory/product-legal floor class lives in specs; sweep gap, not exclusion); review-plan-artifacts enters thin — one mechanical disclosure-line check row, judgment verification staying with feasibility. Rationale lines recorded in ER-D6. |
| F3 | Important | Verify-at-review leaves never-reviewed surfaces structurally unreachable (catalog tool tables); fact-checker named in ER-D1 scope but got no post-ER-D4 residual ruling | **User confirmed fold:** open thread 2 upgraded to a coverage-or-exclusion build obligation (silent unhandled surface = build FAIL); tool tables pre-ruled excluded (library-maintenance concern, primitive-edit ceremony); fact-checker residual ruled in ER-D4 — file jurisdiction unchanged, flag-don't-fetch (`memory-asserted` flags feed the reviewers' hunt list). |
| F4 | Minor | ER-D2/ER-D5 `Confident` marks not auditable — trail said only "recommendation adopted" | **User ruled: confirm.** Marks stand; both trail lines annotated "user-confirmed at review F4" (pattern from sibling session's F6). |
| F5 | Minor | Two citation blurs — grep-ground-truth line misattributed to review-plan-artifacts; audited-library example misattributed to EVALUATION-MATRIX.md | **Folded (user-confirmed):** both fixed at source with erratum notes; semantics held in both. |
| F6 | Minor | Missing disclosure line has no ruled consequence — omission silently loses the audit signal | **Folded (user-confirmed):** ER-D2 gains the clause — an undisclosed external claim is itself a finding; reviewer live-checks floor classes regardless. |

**All 6 dispositioned (6/6). Post-fold status: every finding repaired at source or ruled; verdict path needs-revision → folds landed.**

**Verify pass (same reviewer, fresh read): CLEAN on substance — all 6 folds landed at source, no fold-introduced contradiction, no silent narrowing (F3 landed stronger than raised).** Three non-blocking ledger defects repaired same round: F1 disposition over-claimed "/D6" as an amendment site (D6 needed none) · ER-D6's accepted-cost consumer count corrected ~6 → 8 post-fold touch surfaces · status header advanced.

## Open threads

1. **Validator evidence hierarchy placement** — `validator.md:102` ranks `grep > Read > inference`; fetched quotable text needs a rung in that ladder (build-session line, flagged not ruled).
2. **Coverage-or-exclusion obligation (upgraded at review F3)** — the build session must give each of the six mapped surfaces either a bound carrier or a recorded exclusion line; a surface silently unhandled is a build FAIL. Pre-ruled here: catalog tool tables = recorded **exclusion** (static library content — currency is a library-maintenance concern riding the primitive-edit ceremony, not run-time review); registry growth keeps its deferred next-amend-run control, noted as deferred-not-absent.
3. **ER-D5's named re-open condition** — a shipped wrong regulatory mapping re-opens for the option-C surgical carve-out.

## Session trail

- Q1 (anchor surface): user ruled C — both producer and reviewer sites, one doctrine (ER-D1).
- Q2 (trigger): recommendation adopted — load-bearing rule over named-class floor (ER-D2). *Mark user-confirmed at review F4.*
- Q3 (execution, first deal): user reframed mid-fork — "adversarial review is better to do this"; lead re-dealt as architecture fork, recommendation-free (ER-D3: verify-at-review pure; ER-D2 amended at source).
- Q4 (mechanics): lead recommended B (disposable checkers); user tilted A, asked A-vs-C contrast; user composed **A + re-read clause** from the contrast's push-back (ER-D4 `Contested`).
- Q5 (no-review paths): recommendation adopted — hole priced into waiver, option-C fallback named (ER-D5). *Mark user-confirmed at review F4.*
- Q6 (carriers): lead recommended A (per-skill inline); user ruled B, lead pushed back once (conceding the half-error: B = surviving CROSS-EXAM pattern, not purged doctrine-template class), user maintained (ER-D6 `Contested`).
- Close: user ordered close sequence; sized **solo** for a 6-decision single-thread record, offered with the close nudge and not objected to.
