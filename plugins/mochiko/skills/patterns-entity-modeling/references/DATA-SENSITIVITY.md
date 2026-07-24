# Data Sensitivity Classification

Reference documentation for classifying the sensitivity of entity attributes and specifying their handling requirements in `data-model.md`. The four-level taxonomy and the decision tree live in the SKILL; this reference holds the per-aspect field definitions, the handling-by-level matrix, the compact Sensitivity Details row format, and compliance-mapping examples.

**Density (deliverable envelope, `templates/artifact-format.md`):** the handling-by-level matrix is stated **once per `data-model.md`** — the self-containment floor; each Confidential+ attribute then carries **one row** recording only its specifics (retention, access) and deviations from its level default, never a repeated per-attribute aspect table.

## Classification Levels

| Level | Definition | Examples |
|-------|------------|----------|
| **Public** | Freely shareable, no access controls needed | Product names, public profile info |
| **Internal** | Organization-internal, basic access controls | Transaction IDs, internal status codes |
| **Confidential** | Sensitive, role-based access required | Email addresses, billing addresses, standard PII |
| **Restricted** | Highly sensitive, strict access and audit | Passwords, SSNs, payment card numbers, credentials |

**PII and classification:** PII is expressed *through* these levels, not as a separate marker. Standard PII (email, phone, postal address) → **Confidential**. Highly sensitive PII (SSN, credentials, payment cards, health data) → **Restricted**.

## Sensitivity Annotation — Field Definitions

Every Confidential+ attribute's handling covers these aspects. **Encryption, audit, and masking are carried by the level default** (the matrix below, stated once per document); **retention and access control are per-attribute specifics** (they vary by data semantics); anything departing from the level default is a named **deviation**:

| Aspect | Carried by | Format | Rules |
|--------|-----------|--------|-------|
| Classification | Attribute table + Details row | Public / Internal / Confidential / Restricted | Per attribute |
| Encryption at Rest / in Transit | Level default | Per the handling matrix | Deviation named in the row only if it departs |
| Audit | Level default | Per the handling matrix | Deviation named in the row only if it departs |
| Masking | Level default | Per the handling matrix | Attribute-specific display formats named in the row |
| Retention Period | Details row (per attribute) | Duration + expiry action | How long kept, what happens after |
| Access Control | Details row (per attribute) | Role-based description | Who can read, write, delete |
| Compliance | Details row (per attribute) | Regulation + requirement | GDPR, HIPAA, PCI-DSS, SOC2, NIST mappings; cite DS-XXX where realized |

## Handling Requirements by Classification Level

| Aspect | Public | Internal | Confidential | Restricted |
|--------|--------|----------|-------------|------------|
| Encryption at rest | Not required | Recommended | Required | Required (strong) |
| Encryption in transit | TLS recommended | TLS required | TLS required | TLS required + additional |
| Access control | None | Basic auth | Role-based | Role-based + MFA |
| Audit logging | Not required | Not required | Required | Required (real-time) |
| Retention limits | None | Organization policy | Defined period | Minimum necessary |
| Masking in logs | Not required | Not required | Required | Required (no logging preferred) |
| Breach notification | Not required | Internal review | Required (regulatory timeline) | Immediate (regulatory timeline) |

## Classification Decision Tree

```
Is the data publicly available or intended for public sharing?
├── Yes → PUBLIC
└── No
    ├── Is it internal operational data with no PII?
    │   ├── Yes → INTERNAL
    │   └── No
    │       ├── Is it PII, financial, or business-sensitive?
    │       │   ├── Yes → Is it highly sensitive (credentials, SSN, payment cards)?
    │       │   │   ├── Yes → RESTRICTED
    │       │   │   └── No → CONFIDENTIAL
    │       │   └── No → INTERNAL
    │       └── When in doubt → CONFIDENTIAL (classify up, not down)
```

## Sensitivity Details Row Format

Each entity with Confidential+ attributes carries one **Sensitivity Details** table — one row per attribute, recording specifics + deviations only (the level default carries the rest):

```markdown
### Sensitivity Details

| Attribute | Level | Retention | Access | Deviations | Compliance |
|-----------|-------|-----------|--------|------------|------------|
| email | Confidential | Delete ≤ 30d after account closure | Users read own; admins read all | Log masking: j***@example.com | GDPR Art. 6, Art. 17 |
| password_hash | Restricted | Until account deletion; purge on delete | System-only; no user/admin read | — | NIST 800-63 |
```

`Deviations` is `—` when the attribute's handling is exactly its level default; a named departure (a weaker or stronger aspect, an attribute-specific masking format) is spelled out. An attribute needing more room than a row can carry (rare) may add a one-line footnote under the table — never a per-attribute aspect table.

## Data Sensitivity Summary

Roll every Confidential+ attribute up into the summary table at the top of `data-model.md`:

```markdown
## Data Sensitivity Summary

| Entity | Attribute | Classification | Compliance |
|--------|-----------|---------------|------------|
| User | email | Confidential | GDPR Art. 6 |
| User | password_hash | Restricted | NIST 800-63 |
| Payment | card_number | Restricted | PCI-DSS |
| Order | transaction_id | Internal | - |
| Product | name | Public | - |
```

## Compliance Mapping Examples

| Regulation | Applies To | Typical Requirement |
|------------|-----------|---------------------|
| **GDPR** | EU personal data (PII) | Lawful basis (Art. 6); right to erasure within 30 days (Art. 17); data minimization |
| **HIPAA** | Protected health information | Encryption, access audit, breach notification |
| **PCI-DSS** | Payment card data | No storage of full PAN/CVV; tokenization; restricted access |
| **NIST 800-63** | Authentication credentials | Salted + hashed storage; no plaintext; no reversible encryption for passwords |
| **SOC 2** | Customer data (trust criteria) | Access control, monitoring, audit logging |

## Traceability to Analysis

A per-attribute classification often realizes a **DS-XXX** data-sensitivity requirement declared during analysis (in `authoring-technical-requirements`). When it does, reference the DS-XXX id in the attribute's compliance/description cell so the design annotation traces back to the analysis declaration. This skill owns the *classification and handling*; the analysis skill owns the *declaration of which data is sensitive*.

## Validation Checklist

- [ ] Every attribute carries exactly one classification (Public / Internal / Confidential / Restricted)
- [ ] PII expressed through classification (Confidential or Restricted), not a parallel marker
- [ ] Handling-by-level defaults stated once per document
- [ ] Every Confidential+ attribute has a Sensitivity Details row
- [ ] Retention period and expiry action defined per Confidential+ attribute
- [ ] Access control documented per Confidential+ attribute
- [ ] Deviations from the level default named per attribute (encryption / audit / masking departures never silent)
- [ ] Attribute-specific masking formats named where data appears in logs or UIs
- [ ] Compliance mappings recorded where a regulation applies (DS-XXX cited where realized)
- [ ] Data Sensitivity Summary table reflects all Confidential+ attributes
- [ ] Ambiguous data classified up (Confidential), not down
