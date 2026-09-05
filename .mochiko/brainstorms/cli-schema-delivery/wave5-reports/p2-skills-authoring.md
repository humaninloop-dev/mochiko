# Wave 5 — P2 family report: authoring (16 of 30)

All eight authoring members are re-pointed to `mochiko-cli` delivery, with sixteen strip entries
recorded. Every check green. Pre-edit ref for every strip Content field: `7d098b9`.

Members: `authoring-architecture-store` · `authoring-constitution` · `authoring-epic` ·
`authoring-feature-map` · `authoring-prototype` · `authoring-requirements` ·
`authoring-technical-requirements` · `authoring-user-stories`.

## Checks

**Diff scope.** Asserted by script against `7d098b9`: for all eight, everything outside the Rules
section is byte-identical, and the frontmatter delta is exactly the one line
`allowed-tools: Bash(mochiko-cli *)`. `name` and `description` untouched.

**Delivery.** 7/7 blocks render for every member, each with its version-triple head line and its
end line. Each preamble's `class: floor` pin equals the count of ids on its `floors:` line:
9 · 12 · 10 · 16 · 4 · 4 · 8 · 4. The six ids are the family set with `artifact` in the `verdict`
slot, in the preamble's printed order.

**D13 checker.** The full-library sweep now reports 32 findings across the 16 converted skills —
one missing-pin and one missing-heading each, both conversion-expected and superseded by this
wave's ruling. Zero findings of any other class library-wide.

**Strip Content.** Machine-compared against `git show 7d098b9:` for all eight: entry one's Content
is the verbatim old Rules section, entry two's the verbatim pin sentences.

## Budget — the D10.6 re-keyed payload

Body chars plus the seven rendered blocks' chars, hook lines excluded.

| skill | body | render | payload | old budget (body + schema) |
|---|---|---|---|---|
| authoring-architecture-store | 5,391 | 14,342 | 19,733 | 18,876 |
| authoring-constitution | 7,695 | 21,919 | 29,614 | 30,387 |
| authoring-epic | 3,129 | 10,933 | 14,062 | 13,044 |
| authoring-feature-map | 5,936 | 16,387 | 22,323 | 21,636 |
| authoring-prototype | 4,601 | 10,379 | 14,980 | 13,943 |
| authoring-requirements | 3,439 | 8,934 | 12,373 | 10,796 |
| authoring-technical-requirements | 4,038 | 16,737 | 20,775 | 19,946 |
| authoring-user-stories | 4,529 | 8,915 | 13,444 | 11,668 |

Family total 147,304 against the record's F3 authoring baseline of 150,576 — **2.2 % under**, so
criterion (2) holds for this family at the aggregate. These are the re-seed values for the
`[v0.106.0]` ledger block, which lands once all four families are measured.

## Member-specific content

`authoring-requirements` keeps its pointer wording as a sentence after the read-back: "A `pointer:`
here may bind you to a script's content as well as a file's or skill's procedure — referenced,
never restated." The halt paragraph's generic wording names a file's or skill's procedure only,
and this skill's pointers reach a script, so dropping it would have narrowed a live obligation.

Three members (`authoring-feature-map` · `authoring-prototype` · `authoring-requirements`) carried
per-section glosses, two of them including "empty by design" markers. Both are covered by the
render: its `sections` line prints a title per section, and an empty section renders a `note:`
giving the reason. The "this body carries identity and teaching only" clause in those three is
dropped by the approved ruling and named in each strip entry rather than removed silently.
