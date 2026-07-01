# Data Sensitivity Classification

Reference documentation for classifying the sensitivity of entity attributes and specifying their handling requirements in `data-model.md`. The four-level taxonomy and the decision tree live in the SKILL; this reference holds the per-aspect field definitions, the handling-by-level matrix, the Sensitivity Details block format, and compliance-mapping examples.

## Classification Levels

| Level | Definition | Examples |
|-------|------------|----------|
| **Public** | Freely shareable, no access controls needed | Product names, public profile info |
| **Internal** | Organization-internal, basic access controls | Transaction IDs, internal status codes |
| **Confidential** | Sensitive, role-based access required | Email addresses, billing addresses, standard PII |
| **Restricted** | Highly sensitive, strict access and audit | Passwords, SSNs, payment card numbers, credentials |

**PII and classification:** PII is expressed *through* these levels, not as a separate marker. Standard PII (email, phone, postal address) → **Confidential**. Highly sensitive PII (SSN, credentials, payment cards, health data) → **Restricted**.

## Sensitivity Annotation — Field Definitions

For each attribute classified **Confidential** or **Restricted**, document a Sensitivity Details block with these aspects:

| Field | Required | Format | Rules |
|-------|----------|--------|-------|
| Classification | Yes | Public / Internal / Confidential / Restricted | Per attribute |
| Encryption at Rest | Yes (Confidential+) | Required / Not required + standard | Mandatory for Confidential and Restricted |
| Encryption in Transit | Yes (Confidential+) | Required / Not required + standard | Mandatory for all classifications |
| Retention Period | Yes (Confidential+) | Duration + expiry action | How long kept, what happens after |
| Access Control | Yes (Confidential+) | Role-based description | Who can read, write, delete |
| Audit | Yes (Confidential+) | Yes / No | Whether access is logged |
| Masking | No | Display format | How data appears in UIs and logs |
| Compliance | No | Regulation + requirement | GDPR, HIPAA, PCI-DSS, SOC2, NIST mappings |

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

## Sensitivity Details Block Format

Place one block per Confidential+ attribute inside its entity section:

```markdown
### Sensitivity Details (Confidential+ attributes)

#### email (Confidential)

| Aspect | Requirement |
|--------|-------------|
| Encryption at Rest | Required — AES-256 |
| Encryption in Transit | Required — TLS 1.3+ |
| Retention Period | Delete within 30 days of account closure |
| Access Control | Authenticated users read own; admins read all |
| Audit | All access logged with timestamp and user |
| Masking | Displayed masked in logs: j***@example.com |
| Compliance | GDPR Art. 6 (consent via signup), Art. 17 (deletion within 30 days) |

#### password_hash (Restricted)

| Aspect | Requirement |
|--------|-------------|
| Encryption at Rest | Required — AES-256 |
| Encryption in Transit | Required — TLS 1.3+ |
| Retention Period | Retain until account deletion; purge on delete |
| Access Control | System-only; no user or admin read access |
| Audit | All access logged; real-time alerts on anomalous access |
| Masking | Never displayed; never logged |
| Compliance | NIST 800-63 (credential storage) |
```

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
- [ ] Every Confidential+ attribute has a Sensitivity Details block
- [ ] Encryption at rest specified for all Confidential+ attributes
- [ ] Encryption in transit specified for all attributes that need it
- [ ] Retention period and expiry action defined for Confidential+ attributes
- [ ] Access control documented for Confidential+ attributes
- [ ] Audit logging indicated for Confidential+ attributes
- [ ] Masking specified where data appears in logs or UIs
- [ ] Compliance mappings recorded where a regulation applies
- [ ] Data Sensitivity Summary table reflects all Confidential+ attributes
- [ ] Ambiguous data classified up (Confidential), not down
