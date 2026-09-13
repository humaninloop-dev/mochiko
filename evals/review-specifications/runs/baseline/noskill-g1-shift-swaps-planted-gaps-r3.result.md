The skill `mochiko:review-specifications` isn't available in this session — it's not in my list of installed skills, and both `Skill({skill: "mochiko:review-specifications"})` and the unscoped name failed with "Unknown skill." I also don't see any plugin/skill definition files under the working directory (`kestrel/.mochiko/` only contains the spec content itself, not a skill definition).

Since I don't have the skill's actual methodology (its specific rubric, required report sections, severity taxonomy, etc.), I don't want to guess at it and produce a report that looks authoritative but follows an invented process.

Two ways I can proceed — let me know which you'd prefer:

1. **Manual review**: I read `kestrel/CLAUDE.md`, `README.md`, `docs/`, `FEATURES.md`/`.mochiko/features/`, `.mochiko/specs/index.md`, and the `shift-swaps` spec/stories/derivation myself, and write `review.md` as a stress-test critique using my own judgment (consistency with decisions/venue facts, gaps in stories, derivation correctness, etc.).
2. You point me to where the `mochiko` plugin/skill should be installed (e.g., a marketplace or local plugin dir) so it loads correctly, and I retry with the real skill.