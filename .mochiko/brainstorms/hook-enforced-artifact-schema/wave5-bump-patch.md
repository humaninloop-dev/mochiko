# The bump PATCH — pre-authorized governance edits applied with the 0.109.0 bump (lead draft)

*Re-keyed 2026-09-20: this PATCH is **v3.1.2**. The v3.1.1 row was taken on 2026-09-20 by the
`validator` retirement (`author-grader-consolidation` wave 3, user-ruled), so every "v3.1.1" below,
the ledger's GI-019 clause-iv pointer, and the BACKLOG line now read v3.1.2; the `Ratified:` line
and the ledger `**Version:**` field move 3.1.1 → 3.1.2 when this lands. Content unchanged.*

**Authority:** the AM-3 ledger row (v3.1.0, 2026-09-14) pre-authorizes two PATCH amendments
"recorded in the log row": (a) activation of GI-019's dormant conformance-testability limb at the
`plugin.json` bump that ships the hooks (the v2.0.1 / v3.0.1 / v3.0.3 idiom); (b) the strike of the
wave-4 precondition clause once the bump has landed under it. This draft adds (c), a factual
correction the AM-3 verify pass could not have known: the amnesty paragraph's description of the
pre-fix gap. Nothing here lands before the bump; the bump does not land before the crate publish
with all four controls and the full contract run (gate 6). Semver: **PATCH — v3.1.0 → v3.1.1.**

## (a) GI-019 Testability — activate the conformance limb

The AM-3 fix round split Testability into two tiers with the conformance limb *dormant until the
wave-4 hook ship*. At the bump: mark the limb **active**, citing the contract cases that test it
(`gate-input` 31 rows · `reminder-input` 6 · `if-placement` 2 · `gate-live` · `reminder-spawn`
— `evals/contract/run.py`) and the crate matrices (`crates/mochiko-cli/tests/{home,conform,hook}.rs`).

## (b) GI-012 — strike the wave-4 precondition clause

Once the bump has landed under it: strike the sentence on the region's gates line and the two
ledger paragraphs ("From AM-3 … those two owed controls gate the wave-4 hook ship too …"),
replacing them with one line: *"Wave-4 hook ship landed at v0.109.0 (<date>) after the first
publish (`mochiko-cli-v<x>`, four controls in place); precondition discharged — PATCH v3.1.1."*
The standing first-publish sentence (the two controls gate the first publish under the GI-002
trigger) stays, marked discharged with the tag.

## (c) Amnesty paragraph — correct the pre-fix description on evidence

Current text (AM-3): "**Known gap, measured at the verify pass:** an `Edit` to an existing
undeclared file name rides amnesty with a bare allow and no `additionalContext`, so a mis-homed
…". Measured at wave 4 against the 2026-09-13 binary (seat + non-author reviewer, both limbs):
**a `Write` and an `Edit` over an existing undeclared name each returned `permissionDecision:
deny` at exit 4** — no allow existed. Replace the sentence with: *"Known gap at AM-3, corrected at
wave 4 (v0.109.0): the pre-fix binary denied both a `Write` and an `Edit` over an existing
undeclared name at exit 4 (the AM-3 verify pass's 'bare allow' reading did not reproduce); the
file-set limb of first-touch amnesty was built at wave 4 as ratified — an existing undeclared name
is editable with the name in `additionalContext`, a new undeclared name still denies, path is
never relaxed."*

## (d) Stated limits — one ledger line, pointer only (GI-017)

Under the GI-019 admission bullet, one pointer line: *"Stated limits of the gate (record § Build
trail): repo-root writes uncaught; three shell shapes evade the write-operator scan (`cd &&`,
variable-held path, relative write from inside a home); `PowerShell` routing unverifiable on
macOS; `disableAllHooks` projects keep the procedural ceremony only."* — no restatement of the
limits themselves.

## (e) Status flips already transcribed at the wave-3 close (2026-09-15) — carried into the log row

The user's ratification made two surfaces stale; both were flipped as transcription on the day
(KM status-agreement invariant, fix on sight): the `DECISIONS.md` 2026-09-13 row ("ratified
2026-09-15 with the C1 amendment") and the ledger GI-019 clause (iv) condition + its trace clause
("DISCHARGED 2026-09-15"). The v3.1.1 row names both so the ledger's own history closes.

## Landing mechanics

Applied by the lead at the bump commit as transcription under the pre-authorization; `CLAUDE.md`
governance region `Ratified:` line → `v3.1.1 · <date> (PATCH)`; ledger `**Version:**` → 3.1.1;
amendment-log row appended citing this file; `mochiko:validation-constitution` grades the edited
region + ledger (author≠grader) before the bump commit closes.
