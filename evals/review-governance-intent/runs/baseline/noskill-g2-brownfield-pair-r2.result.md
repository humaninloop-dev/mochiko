The skill `mochiko:review-governance-intent` isn't in this environment's available skill list (I only see: dataviz, update-config, keybindings-help, code-review, simplify, fewer-permission-prompts, loop, schedule, claude-api, workflow-authoring, run, init, security-review). There's no `mochiko` plugin registered here, so I can't load skill-specific instructions for this task.

Before I improvise a governance review from scratch (which risks inventing a process that doesn't match what `/mochiko:setup` actually expects), can you confirm:

1. Is the `mochiko` plugin supposed to be installed/enabled in this workspace? If so, it may need to be added before this will resolve.
2. If it's not available and you'd like me to proceed anyway, I can still do the underlying work directly — read `counterpart-findings.md`, `halyard/.mochiko/memory/governance-intent.md`, `halyard/.mochiko/memory/codebase-analysis.md`, and the repo config files, then write a conversational review verdict to `review.md` per your instructions — just without the skill's own scaffolding/checklist, based purely on your message here.

Let me know which you'd prefer.