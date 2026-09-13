I've hit a hard blocker: the skill's own reference files it points me to (`references/ARTIFACT-CHECKLISTS.md`, `references/ISSUE-TEMPLATES.md`, and the `mochiko:advocate-report-template` skill) live outside this session's allowed working directory (`/private/var/folders/.../skilleval-xk8u23r5/ws`), in the plugin's install path. Every Read attempt there is being auto-denied by the sandbox — it's not a one-time approval prompt, it's a boundary I can't cross with any tool available to me (Read, Bash, find all refuse the same path).

This isn't a minor inconvenience I can work around: the skill's rules are explicit that these are pointers I must dereference, never restate from memory — "there is no fallback" for the rules-delivery channel, and the same principle applies here: I should not fabricate the mirror checklists, severity/verdict-mapping table, or report template structure from guesswork and then present that as a rules-governed review. Doing so would silently violate `review-plan-artifacts.letter-is-spirit` and `review-plan-artifacts.evidence-floor` while looking legitimate.

I don't want to guess my way past this and hand you a `review.md` that claims a grammar I never actually read. A couple of ways to unblock, your call:

1. If you can make the plugin's `skills/review-plan-artifacts/references/` directory (and the `advocate-report-template` skill) readable within this session's working directory (e.g., a symlink or copy into `ws/`), I can proceed immediately.
2. If those checklists are meant to be fetched via a different in-sandbox mechanism (e.g., a `mochiko-cli` command rather than direct file Read), point me at it.
3. Otherwise, I can flag this as a broken skill dependency and stop here rather than produce a review.

How would you like me to proceed?