I tried to invoke `mochiko:validation-constitution`, but it isn't in this session's available-skills list, and calling it directly returned `Unknown skill`. I also checked the workspace (`.claude/`, `.mochiko/`) and there's no local plugin/skill definition for it either — the mochiko artifacts (ledger, intent, trace summary) are here, but the skill that defines the validator's constitution (grading rubric, required checks, etc.) isn't installed in this environment.

I don't want to fabricate what that constitution says and then grade a compliance-heavy production launch (GDPR/PCI-DSS/a11y) against invented rules — that's exactly the kind of guess that could produce a wrong sign-off on real stakes.

A couple of ways to unblock:
1. If the mochiko plugin/skill is supposed to be installed, you may need to enable it (e.g., via a marketplace/plugin add) before I can load its actual instructions.
2. Alternatively, I can proceed with a general, best-practice independent validation of the handoff (cross-checking CLAUDE.md governance region, the rules files, the ledger, the ratified intent, the trace summary, and `docs/engineering-norms.md` for conflicts) using standard governance-review judgment rather than the mochiko-specific constitution, and clearly label the result as such.

How would you like me to proceed?