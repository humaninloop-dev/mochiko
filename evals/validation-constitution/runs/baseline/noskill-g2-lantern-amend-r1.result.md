I couldn't invoke `mochiko:validation-constitution` — it's not in this session's available skill list (likely not installed/enabled in this workspace) — so I proceeded directly per the task's explicit instructions, verifying the producer's hand-off claims against the actual files rather than the skill's playbook.

**Verdict written to `validation-result.md`: FAIL.** Key findings:

1. **Version mislabeled** — stamped PATCH (1.0.1), but the amend bundles a depth-level flip, a module attach, and a principle removal — each independently MAJOR under the ledger's own semver policy. The synthesis itself flagged this as Critical and left the bump to the producer, who mislabeled it anyway.
2. **Layer-rules file doesn't exist** — `.claude/rules/mochiko/layers.md` is referenced from `CLAUDE.md` and the ledger but is simply missing from the directory (only `data-access.md` and `output-style.md` are present), directly contradicting the hand-off claim it's "in place with its registry block."
3. **"Nothing was removed" is false** — GI-015 was explicitly retired with no replacement.
4. **Ledger's own amendment-log row undercounts the amend** — omits the un-waive and the retirement.
5. **Trace summary is stale**, not "closing exactly as it did at 1.0.0" — it's the untouched v1.0.0 document and misstates depth level, waiver status, and two module rulings.
6. **A pinned user preference (`conversation: lite`) was silently reverted to default** during regeneration.

Full detail, evidence citations, and remediation steps are in `validation-result.md`. No graded files were modified.