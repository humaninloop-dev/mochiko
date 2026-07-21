# Domain-Dependency Allowlist — seeding and growth

The domain layer under a layered architecture MAY import approved third-party libraries — an
empty or undiscussed registry degrades to "standard library only" in practice, a stricter rule
than anyone ratified. This file is the single source for how the allowlist is **seeded** during
the interrogation's arbitration stage and how it **grows** at implement time. Design record:
`.mochiko/brainstorms/domain-dependency-allowlist/record.md` (D1–D5 + review folds).

## When this fires

Whenever the session keeps **or mints** a layered-architecture principle — i.e., whenever the
`layer-rules` module is adopted (the interrogation's layered-architecture beat records the
module ruling either way). The seed arbitration below runs as part of deck arbitration.

## Qualification criteria (both required — the admissibility gate)

1. **Domain-relevance** — domain modeling without I/O: value objects, validation, equality,
   precise arithmetic, immutable collections. A library that performs I/O, talks to a platform,
   or renders UI is inadmissible regardless of how trusted it is.
2. **Ubiquity** — effectively an ecosystem standard (>80% adoption in comparable projects).

**Compose in this order: domain-relevance filters the candidate pool FIRST; the trust hierarchy
below ranks and grounds only what passes.** Trust and admissibility are orthogonal axes — an
ecosystem's most-trusted libraries are mostly I/O/UI (Flutter Favorites is dominated by them),
while its best domain libraries may carry no curation badge at all.

## Trust-signal hierarchy (ranks the admissible)

Walk top-down; **cite the level each seed rests on** — the citation is what makes a weak or
hallucinated source challengeable at arbitration:

1. **Official curation** — ecosystem-owner curated lists (e.g. Flutter Favorites).
2. **Semi-official stewardship** — maintained by the ecosystem's core team or foundation
   (e.g. `golang.org/x/*`, .NET Foundation projects, PyPA-maintained packaging tools).
3. **Credible community curation** — recognized community-maintained guides (e.g. blessed.rs).
4. **Quantitative proxies** — sustained adoption stats, OpenSSF Scorecard, verified
   publishers — with the qualification criteria doing the real work.

Starter table of known instances — **entries are pointers, re-verified live at seed time, never
copied as snapshots** (curated lists change; a stale copy admits the de-listed):

| Ecosystem | Trust source | Level |
|-----------|--------------|-------|
| Flutter / Dart | Flutter Favorites · pub.dev verified publishers + pub points | 1 · 4 |
| Go | `golang.org/x/*` | 2 |
| .NET | .NET Foundation projects | 2 |
| Python | PyPA (packaging tools only); otherwise level 4 | 2 / 4 |
| Rust | blessed.rs | 3 |
| JS/TS, JVM, Ruby, others | no known curation — level 4 + criteria | 4 |

An ecosystem not in the table gets live research **constrained to filling hierarchy levels**
(find its level-1/2/3 sources or conclude none exist) — never free-form judgment.

## Seed arbitration (setup, at deck arbitration)

Recommend a small seed set (typically 3–6 libraries) for the detected stack: filter by
domain-relevance, rank by trust level, cite each seed's level and justification. The user
**keeps / drops / tightens** — recommend-then-arbitrate: they sort supplied content, they are
not asked to generate it. Every ruling lands in the synthesis's Domain-dependency seeds table.
A non-empty day-one registry is the point: an empty list reads as prohibition.

## Growth (implement time — the add-process authored into the policy)

Tier-keyed gate, with a visibility floor at every tier:

- **`poc` / `internal`** — self-serve: the implementer MAY add a dependency that meets both
  criteria directly to the registry block, recording justification + signal level; the addition
  MUST be disclosed in the cycle report (`domain_deps_added`) — visible and contestable at the
  cycle checkpoint, non-blocking.
- **`production` / `regulated`** — gated: a proposed addition surfaces to the human as an
  explicit ruling BEFORE entering the registry; the cycle checkpoint MUST NOT auto-approve
  while `domain_deps_added` is non-empty.

Additions never require a setup re-run; the next amend run re-validates the block's rows.

## Registry surfaces (what authoring produces)

- **The list** lives ONLY in a marked block inside the domain-layer rules file —
  `<!-- mochiko:domain-registry:begin -->` / `<!-- mochiko:domain-registry:end -->` —
  delivered natively to any agent editing domain code and **preserved verbatim** across
  setup/amend regenerations (the one carve-out from rules-files-regenerated-whole; see the
  ownership boundary in the authoring skill). Row schema:
  `| Dependency | Justification | Signal level | Added (by/when) | Gate |`.
- **The policy** (these criteria, the hierarchy, the add-process, the tier gate) is
  setup-owned: recorded in the governance ledger's Domain-dependency policy section and
  regenerated into the rules file's preamble. No copy of the list exists outside the block —
  nothing can drift.
