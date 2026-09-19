# Persona eval judge carries each kit's pre-registered readings (sha-pinned)

- **Status:** ruled
- **Date:** 2026-09-11
- **Evidence:** the run directories cited below are archived at the tag `eval-evidence-2026-09-19`,
  not carried in the working tree (`git fetch origin tag eval-evidence-2026-09-19`, then
  `git show eval-evidence-2026-09-19:<path>`).
- **Context:** Every persona kit pre-registers readings the generic embodiment rules do not
  carry — how the read-only wrapper's described stops read (a named stop with a default is a flag,
  a silent default is a guess, obeying a no-questions ask on a money gap is `contradicted`), which
  limb splits a near-duplicate cluster, when governance alignment counts, which plan shape on a
  given golden is `contradicted`. The calibration labeller applies them from `preregistration.md`;
  the judge saw only the claim text and the generic rules. On the `qa-engineer` baseline
  (`evals/agents/qa-engineer/runs/baseline`) that gap decided both `contradicted` pairs on the
  design-time golden q3: the labeller read them `contradicted` on the kit's q3 reading (browser
  cases must be human-checkpoint cases; `FakeTextline` must be named as declined) and the judge read
  both `reflected` — 0.917 agreement overall, 0 of 2 on the contradicted limb, ship bar (c) failed
  on an instrument defect rather than a persona finding. The same readings shape appears in every
  wave B kit. The user ruled (2026-09-11): carry the readings into the judge, as recommended.
- **Decision:**
  1. **A kit may carry `judge-readings.md`** beside `preregistration.md`: the pre-registered
     readings, copied from the preregistration's wrapper-interaction and cluster bullets — never
     restated, never widened — so judge and labeller read by one text.
  2. **The judge reads it verbatim**, as a `KIT READINGS` block between the generic rules and the
     standards, on both judge sites (`agent-prune` control reads and `agent-judge` grid reads). The
     block refines the generic rules for the named standards and goldens and never replaces them;
     where a reading names a plan shape `contradicted` or `absent`, the reading wins.
  3. **Pinned separately.** `readings_sha256` joins every run's pins, the judge-time
     `judge_models`, and the prune summary; the report header prints it (`judge readings <sha>` or
     `none (claim text alone)`). The static prompt pin `judge_prompt_sha256` is unchanged — empty
     readings render the byte-identical prompt. Grids compare only within both pins; a changed
     readings file is an instrument re-key, re-judged before any `pre`/`post` read and recorded in
     the fill log.
  4. **The labeller reads the same file** — the label sheet's instructions name it when present.
  5. **Rollout:** `qa-engineer` now (readings file cut from its preregistration, baseline re-judged
     with the labels unchanged, calibration re-run; the claim-text-judge calibration and report kept
     beside them as `*-claim-text-judge.*`). Every other kit gains the file at its next grid; until
     then it runs on claim text alone, disclosed in the report header. Kit authoring rule from here:
     a preregistration that records readings ships the file with them.
- **Rationale:** Calibration certifies the judge against the labeller's reading. A miss confined to
  exactly the pairs the kit's own readings decide is not judge noise; it is the judge reading a
  different rulebook, and it would recur on every `contradicted` pair a persona edit is meant to
  surface. Feeding the readings to the judge closes the gap at about a dollar per kit with the
  labels intact, and the separate pin keeps the re-key visible in every run record.
- **Alternatives considered:** Read ship bar (c) on agreement alone and record kit-specific
  readings as labeller-only — rejected: the next edit's `contradicted` pairs stay unreadable, which
  is the limb the bar exists for. Fold the readings into the claim text — rejected: claim ids are
  minted by source sha from the persona body, and the readings are kit-level (per golden, per
  wrapper), not persona-level.
