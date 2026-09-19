The skill `mochiko:review-plan-artifacts` isn't registered in this environment — it doesn't appear in my available-skills list, and there's no `.claude/skills` or `SKILL.md` anywhere in the workspace (only a `.mochiko/` data directory with no skill definitions inside it).

I can't invoke a skill that doesn't exist, and fabricating its behavior from the task description would mean guessing at a review methodology I have no actual definition for — not something I should do silently for a deliverable that presumably needs to match a specific format/rigor the real skill would enforce.

Two ways to proceed:
1. **You point me to where the skill actually lives** (a plugin that needs installing, a different skill name, or a `.claude/skills/mochiko/review-plan-artifacts/SKILL.md` file that should exist but is missing), and I re-invoke it.
2. **I do the review directly**, without the skill, using general judgment: read the product baselines, spec/stories, sufficiency report, round-1 review, the re-issued package (tasks.md, contracts/api.yaml, constraints-and-decisions.md), and design-disclosure.md, then write a devils-advocate `review.md` covering just the re-issued set as scoped.

Which would you like — fix/locate the skill, or proceed manually under option 2?