I ran the full checklist independently rather than the requested spot-check, and the result is **FAIL**, not a conditional pass. Result written to `validation-result.md`.

Six concrete issues, sourced to file/line:

1. **Version mismatch** — `CLAUDE.md` says "Ratified v1.0.0," the ledger says "Version: 0.9.0 (must match the region stamp)" and doesn't.
2. **An unauthorized waiver** — the ledger has a waiver on the observability logging check that isn't in the ratified synthesis at all (synthesis says "Waivers: None"), has no GI-ID, a non-real revisit trigger ("later"), and directly contradicts the NON-NEGOTIABLE principle text and the ledger's own enforcement claim for the same GI-ID. This is the substantive one — it means the set currently asserts both that PII redaction in logs is built and that it isn't.
3. **`[SECURITY_COMMAND]` placeholder** still in the quality gates table, despite the real commands (`gitleaks`, `pnpm audit`) being available and used elsewhere in the same file.
4. **`GI-XXX` placeholder trace stamp** on the knowledge-management line.
5. **Under-scoped rules file** — `api-errors.md` only covers `apps/api/src/**`, but the ratified synthesis explicitly scopes that principle over the portal's route handlers too; they're missing from the glob entirely.
6. **Missing standing new-file read line** in the governance region.

I didn't apply any fixes myself — patching the files and then counting it as passing would mean grading my own edit, which defeats the purpose of having an independent validator, and it would bury the waiver discrepancy instead of surfacing it for someone to actually rule on. That decision, plus everything the producer's "I checked it myself" claim got wrong, is documented in the result file for the lead.