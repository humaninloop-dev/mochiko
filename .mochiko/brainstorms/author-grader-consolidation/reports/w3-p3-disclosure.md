---
report: disclosure
round: 1
seat: P3 (suite · crate fixtures · eval kit)
wave: wave 3 — the validator persona retirement
unit: crates/mochiko-cli/tests/{fidelity,validate}.rs · evals/agents/validator/preregistration.md · evals/README.md
status: complete — four crate layers green; write set as planned, nothing outside it
measured_against: post-0010 tree — sequences 1..10 · state sha256:d03d5dee… · 75 documents · 1,082 rules
figures_moved: fidelity.rs sequence list 1..9 to 1..10 (source — migrate status)
figures_held: docs 75 · command rules 329 · skill rules 753 · total 1,082 · skill floors 257 · command floors 117 · fail nodes 36 · sidecar 597 · tiering pin 8 · pointers 84 · views 75/75/75 · floor indexes 38 · sweep 1082/161376/0/170 · family 329/12607/0/56
probe: transient raw-sweep test removed before the final runs — matrix_similar.rs byte-identical to HEAD
unchanged_as_planned: evals/contract/run.py · evals/contract/README.md · evals/plan/agents.py
gates: cargo test 472 passed 0 failed · full sweep 48 passed · fmt exit 0 · clippy exit 0
---

## Notes of note

One figure moved: the sequence list, 1..9 to 1..10. Three id-preserving rewords mint and retire
nothing, so every count, floor pin, sidecar figure, pointer count and similarity number held,
each confirmed by running its test rather than skipped. The dispatch expected the state hash and
byte totals to move; no crate test pins either — the state-hash assertion is a prefix check, and
the byte-identity test builds from the frozen genesis corpus.

Q1 closed by my own raw sweep, not the pinned pass alone: against an empty allowlist the corpus
still reports 77 clusters over 170 edges and the command family 29 over 56, the arithmetic the
comments already record. No edge shifted, so both zeros stay suppressed and no allowlist row was
needed. Q2 closed by the landed file: three rewords, no anchor op, the sidecar anchor rides.

Disclosed call: `validate.rs`'s census comment gained a 0010 paragraph though its figures held.
