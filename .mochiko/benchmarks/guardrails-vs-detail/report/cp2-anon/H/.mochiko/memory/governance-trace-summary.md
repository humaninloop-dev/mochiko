# Trace summary — Ledgerline governance surface set v1.0.0

One row per principle-bearing GI element → primary enforceable home + companions.

| GI-ID | Principle | Source | Primary home | Companions present |
|-------|-----------|--------|--------------|--------------------|
| GI-003 | Security by default | floor-asserted: FLOOR-SEC | CLAUDE.md region line | index ✓ · ledger ✓ |
| GI-014 | No raw cardholder data | minted | CLAUDE.md region line | index ✓ · ledger ✓ |
| GI-004 | Testing discipline | floor-asserted: FLOOR-TEST | CLAUDE.md region line + quality gates | index ✓ · ledger ✓ |
| GI-005 | Error handling | floor-asserted: FLOOR-ERR | CLAUDE.md region line | index ✓ · ledger ✓ |
| GI-006 | Observability | floor-asserted: FLOOR-OBS | CLAUDE.md region line | index ✓ · ledger ✓ |
| GI-030 | Email deliverability | minted (reopen-born S6) | CLAUDE.md region line | index ✓ · ledger ✓ |
| GI-011 | Tenant isolation | minted | rules/mochiko/tenant-isolation.md | index ✓ · ledger ✓ |
| GI-012 | Invoice/payment integrity | minted | rules/mochiko/payment-integrity.md | index ✓ · ledger ✓ |
| GI-013 | Money uses Decimal | minted | rules/mochiko/payment-integrity.md | index ✓ · ledger ✓ |
| GI-026 | Stripe webhook trust | minted (reopen-born S1) | rules/mochiko/payment-integrity.md | index ✓ · ledger ✓ |
| GI-029 | Invoice/payment audit trail | minted (reopen-born S5) | rules/mochiko/payment-integrity.md | index ✓ · ledger ✓ |
| GI-007 | Hexagonal layering | deck-kept: BE-HEX | rules/mochiko/layers.md | index ✓ · ledger ✓ |
| GI-008 | Code quality (SRP) | deck-kept: BE-SRP | rules/mochiko/code-quality.md | index ✓ · ledger ✓ |
| GI-009 | Dependency discipline | deck-kept: BE-DEP | rules/mochiko/dependencies.md | index ✓ · ledger ✓ |
| GI-010 | Accessibility WCAG 2.1 AA | module: a11y-wcag (legal-mandate) | rules/mochiko/accessibility.md | index ✓ · ledger ✓ |
| GI-028 | Backup + tested restore | minted (reopen-born S4) | ledger release-gates section (region pointer) | index ✓ (release-gates line) · ledger ✓ |

**Module realizations (non-principle GI elements):**
- GI-015 knowledge-management → region pointer + index line; ledger section; scaffolding at finalize.
- GI-016 release-gates → region summary line + pointer; ledger Release gates section.
- GI-017 layer-rules → rules/mochiko/layers.md (import rules + domain-registry block); ledger Domain-dependency policy.
- GI-018 pydantic → domain-registry block in layers.md. (GI-019 attrs — dropped ruling, not a surface element.)

**Fact profile:** GI-001 → ledger Governance Floor header + region stamp.
**Type/identity:** GI-002 → region Technology stack.
**Exclusions (non-principle, no surface):** GI-020 (SOC 2), GI-021 (ops maturity), GI-022 (data
retention — deferred open question, routed to BACKLOG), GI-023 (out-of-scope features), GI-024
(multi-user), GI-025 (SSO/scale), GI-027 (CCPA/CPRA not-triggered + revisit). Recorded in the
synthesis; not realized as principles by design.

Flagged proposals: none.
Waivers: none.
