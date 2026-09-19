The skill `mochiko:review-governance-intent` isn't available in this environment — it's not in my loaded skills list, and there's no matching skill file anywhere in the workspace (I checked for a `.claude/skills` directory or plugin providing it, including inside `halyard/`). Only `halyard/.mochiko/memory/` exists, which holds the synthesis and analysis artifacts, not a skill definition.

I don't want to fabricate a "devil's-advocate coherence review" process and pass it off as having run that skill — the skill likely encodes specific methodology (disposition categories, what counts as a floor row, how findings should be framed) that I'd otherwise be guessing at.

A few options:
1. You point me to where the skill actually lives (maybe it needs to be installed/enabled, or is under a different name/plugin prefix), and I retry.
2. I proceed without the skill, treating your message as direct instructions: read `counterpart-findings.md`, review `halyard/.mochiko/memory/governance-intent.md` against the codebase analysis and repo files for coherence issues, and write `review.md` as a conversational message to the lead — flagging clearly that this was done without the named skill.

Which would you like?