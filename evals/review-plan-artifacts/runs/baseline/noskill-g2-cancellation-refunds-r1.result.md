The `mochiko:review-plan-artifacts` skill isn't available in this environment — it's not in the list of skills I have access to (I only see things like `code-review`, `security-review`, `simplify`, etc., no `mochiko` namespace at all).

Before I improvise a substitute, a few options:

1. **You may have the wrong skill name.** If Mochiko skills are supposed to be installed here, they aren't registered — possibly a plugin/marketplace isn't loaded in this session.
2. I can still do the review manually: read `COVER-NOTE.md`, the product baselines, the FEAT-034 design package, and `src/refunds/service.ts`, then write `review.md` as a devil's-advocate critique — following the spirit of the task even without the packaged skill instructions.
3. You could point me to where the skill is defined (e.g., a `.claude/skills/` directory or plugin) if it's supposed to exist locally, and I'll check for it directly.

Want me to proceed with option 2 (manual review, same deliverable), or do you want to first sort out why the skill isn't loading?