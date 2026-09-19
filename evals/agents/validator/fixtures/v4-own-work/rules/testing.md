---
paths:
  - "src/**"
  - "tests/**"
---

# Testing rule — helpdesk core

- Every behaviour change ships with a test that fails before the change and passes after it.
- Tests hit real infrastructure for the layer under test: a real PostgreSQL for repositories, a
  real HTTP server for handlers; mocks are allowed only at the boundary to a third-party system.
- A flaky test is deleted or quarantined the day it is found; a quarantine carries an issue link
  and an expiry date.
- The suite runs in under ten minutes on CI; a change that crosses that line is reverted.
- Enforcement: CI blocks merge on a red suite or on coverage below the baseline; the quarantine
  list is reviewed at every release cut.
