# Strip notes — the plugin manifests (`plugins/mochiko/.claude-plugin/plugin.json` + `.claude-plugin/marketplace.json`)

Entry formats: `strips/README.md`. First entry for the manifest pair (new file, created v0.76.0 —
the manifests had no prior recorded removal because they carry metadata, not primitive prose;
this is the first identity phrase retired by ruling). One file covers both manifests: they are
edited as a pair and the superseded phrase lived identically in both.

## [v0.76.0] "Kernel-free" identity phrase superseded — the manifests now lead "Skills-first"

- **Disposition:** superseded → the D11-amended identity. The manifest description/keyword identity
  phrase re-worded to the "skills-first" frame that CLAUDE.md's core-bet sentence and
  `## Non-negotiable constraints` now carry ("engineering discipline lives first in the quality of
  the skill library"; "Skills and agents are the primary quality surface"). Only the leading
  identity phrase moved; each manifest's remaining wording (and the two pre-existing manifest
  divergences — see *Kept deliberately*) is untouched.
- **Tier failed:** n/a — supersession by ruling (`schema-based-template-guidance` D11, the
  no-kernel-non-negotiable softening — `.mochiko/brainstorms/schema-based-template-guidance/record.md`;
  `DECISIONS.md` 2026-08-16 governance v2.0.0 row + template-schema row; **user-ruled rider,
  2026-08-16, DM session** — the stale "kernel-free" tagline flagged at the P6 landing, user
  ruled it moves this wave).
- **Content (superseded text, verbatim):**

  `plugins/mochiko/.claude-plugin/plugin.json`
  - description:

    ```
    Kernel-free agent-skill framework: self-contained command supervisors built from native agent teams and skills
    ```

    replaced by:

    ```
    Skills-first agent framework: self-contained command supervisors built from native agent teams and skills
    ```

  - `keywords` entry:

    ```
    "kernel-free"
    ```

    replaced by:

    ```
    "skills-first"
    ```

  `.claude-plugin/marketplace.json`
  - `metadata.description`:

    ```
    Kernel-free agent-skill framework — sound-loop workflows built from native agent teams and skills
    ```

    replaced by:

    ```
    Skills-first agent framework — sound-loop workflows built from native agent teams and skills
    ```

  - `plugins[0].description`:

    ```
    Kernel-free agent-skill framework: sound-loop workflows built from native agent teams and skills
    ```

    replaced by:

    ```
    Skills-first agent framework: sound-loop workflows built from native agent teams and skills
    ```

  The ruling's ground: at governance v2.0.0 (AM-1) the no-kernel non-negotiable was softened —
  kernel-class tooling is admissible by recorded ruling (GI-019), and the first admitted instance,
  the template-schema Rust CLI, landed at v0.76.0. "Kernel-free" as the plugin's leading identity
  claim was then factually stale: the plugin ships a Rust crate. "Skills-first" states the surviving
  bet (skills and agents are the primary quality surface) without the now-false absolute. The AM-1
  identity rewording had reached the ROADMAP thesis + CLAUDE.md prose but not the manifests; this
  entry closes that gap.
- **Kept deliberately:** the two pre-existing manifest divergences the landing auditor called INFO
  are **left as-is by ruling** — (1) `plugin.json` says "self-contained command supervisors" while
  both `marketplace.json` descriptions say "sound-loop workflows"; (2) `marketplace.json`
  `metadata.description` uses an em-dash separator while `plugin.json` and `marketplace.json`
  `plugins[0].description` use a colon. Only the "Kernel-free agent-skill framework" → "Skills-first
  agent framework" phrase moved; convergence of the divergent tails was explicitly out of scope for
  this rider. Version (0.76.0), owner/author blocks, `agent-skills`/`workflow`/`human-in-loop`
  keywords, and all structural fields untouched.
- **Consumers assessed:** the manifests are read by the Claude Code plugin loader / marketplace,
  not quoted by any shipped primitive — `grep` across `plugins/mochiko/` finds no skill, command,
  agent, or template that quotes the manifest description or the `kernel-free` keyword. The full
  manifest description strings live only in these two files. The shorter identity phrase
  "kernel-free agent-skill framework" also appears in the governance synthesis
  `.mochiko/memory/governance-intent.md:16` (the frozen Identity line) — but that copy was
  **already superseded in place at AM-1**: its line-17 annotation (2026-08-16) re-states the
  identity as "markdown-first primitive library with kernel-class tooling admissible by recorded
  ruling" (GI-019), so this rider leaves it untouched — and in `.mochiko/archive/ROADMAP.md:117`
  (frozen archive, never edited). The ROADMAP thesis and CLAUDE.md carry the "skills-first /
  kernel-class by ruling" identity in their own words (reworded at AM-1), independent of the
  manifest phrasing.
