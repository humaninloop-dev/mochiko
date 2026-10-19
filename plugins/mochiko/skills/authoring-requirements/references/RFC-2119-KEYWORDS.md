# RFC 2119 Keywords in Requirements

The keywords carry their RFC 2119 meanings. This file holds only the conventions this skill adds.

- **Choosing the level.** MUST when the feature does not work without it. SHOULD when most users
  would expect it but a valid exception exists. MAY when either choice is acceptable.
- **Security, data-integrity and compliance requirements are MUST or MUST NOT, never SHOULD.**
  "System SHOULD encrypt sensitive data" is the named mistake; "System MUST encrypt sensitive data
  at rest and in transit" is the fix.
- **No MUST for qualities nobody can test.** "System MUST be user-friendly" says nothing; state the
  observable behavior at the right level ("Error messages SHOULD suggest a corrective action").
- **State WHAT, never HOW.** "System MUST use bcrypt" or "API MUST use JSON" name an
  implementation; write "System MUST use industry-standard password hashing" and "API MUST use a
  structured, parseable response format".
- **Conditional requirements combine levels.** An optional capability can carry an absolute
  constraint once it exists: "System MAY offer social login; if offered, it MUST NOT share user
  data with third parties without explicit consent."
