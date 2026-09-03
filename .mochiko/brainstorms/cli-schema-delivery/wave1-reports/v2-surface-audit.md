# Wave 1 — seat P2 (surface) independent code audit

**Verdict: PASS.**

Unit graded: `git diff cd5a333 -- crates/mochiko-cli`, committed as `3792104`. Grader authored
none of it. Every gate run by the grader. The render contract is character-exact, the exit codes
are exactly the four ruled, the template fixtures were independently regenerated from a rebuilt
pre-P2 binary and matched byte for byte, and the ceiling figure reproduces exactly. Three defects
survive; none breaches a ruled clause, none touches a shipped file, none crosses the bright line.

---

## 1. Gates, run by the grader

| gate | command | result |
|---|---|---|
| tests | `cargo test --all` | **161 passed / 0 failed** — cli 20 · migration 25 · render 26 · replay 46 · validate 44 |
| format | `cargo fmt --all --check` | clean, exit 0 |
| lint | `cargo clippy --all-targets -- -D warnings` | clean, exit 0 |
| audit | `cargo audit --deny warnings` | exit 0 — "Scanning Cargo.lock for vulnerabilities (31 crate dependencies)", 1239 advisories loaded, nothing reported |

Test tally matches the seat's claim exactly.

---

## 2. Findings

### Blocking for wave 3 (not for this unit)

**F1 — an explicitly empty `enforces:` mirror renders a contentless key.**
`crates/mochiko-cli/src/render.rs:236-240`

```rust
if rule.is_fail() {
    if let Some(enforces) = &rule.enforces {
        out.push_str(&format!("enforces: {}\n", enforces.join(", ")));
    }
}
```

`Some(vec![])` is a legal state — the D6 empty-with-reason mirror, legal beside a `note:` — and the
join yields the empty string, so the render emits a bare `enforces:` line carrying nothing. The
rule's `note:`, which exists precisely to say why the mirror is empty, is suppressed as maintainer
metadata under the Q4 ruling, so the delivered guidance is strictly less informative than the
source it came from.

This is not hypothetical. The shipped corpus already carries two such nodes —
`plugins/mochiko/schemas/setup.yaml:390` (`setup.fail.unclosed-trace`) and
`plugins/mochiko/schemas/setup.yaml:409` (`setup.fail.floor-category-uncovered`) — each with its
reason in a `# D6 empty-with-reason:` comment that the migration grammar carries as `note:` data.
Once P3's genesis lands, `rules setup --section setup.sec.fail-conditions` will deliver a dangling
key to the model on both.

Reproduced end-to-end on a reviewer-built log (fail node given `enforces: []` plus a `note:` by
`set-rule-field`):

```
### audit.fail.unrecorded
[class: floor · kind: fail · labels: landing]
An unrecorded verdict at .mochiko/audits/report.md.
enforces:
```

(The trailing space is present in general; here `wrap`'s `body.trim_end()` removed it because the
fail node was last in the section — so the artifact is a bare `enforces:` line in one position and
`enforces: ` with trailing whitespace in any other.)

**Fix:** filter the empty case — `if let Some(e) = &rule.enforces { if !e.is_empty() { … } }` — or,
if the reason should reach the model, render the note for this one case and take the ruling. Not
blocking for P2's wave-1 unit: the wave-1 done condition is untouched, `--section` is unbound to
any `.md` this wave, and the state that exposes it arrives with P3.

### Advisory

**F2 — an ambiguous primitive name reports the opposite of the truth.**
`crates/mochiko-cli/src/cli.rs:284-296`, message at `crates/mochiko-cli/src/render.rs:54-58`

`find_primitive` returns `None` both when neither a command nor a skill carries the name and when
**both** do; the caller maps `None` to `RenderError::UnknownPrimitive`. The seat's own doc comment
says "an overlap would make the name ambiguous rather than silently picking one, so it is reported
instead of resolved" — the code does not guess, but neither does it report; it asserts the name is
absent when the log carries it twice.

Probed with a log carrying `command/twin` and `skill/twin`. The hard set accepts that state
(`migrate validate` exits 0, 3 advisory), so it is reachable:

```
$ mochiko-cli rules twin --section preamble --log-dir <log>
error: no command or skill named 'twin' in the log — check the name against `mochiko-cli migrate status`
[exit 2]
```

A maintainer chases a naming problem that does not exist. Disjoint today across all 36 shipped
primitives, so no shipped render is affected. **Fix:** a third `RenderError` arm for `(true, true)`
naming both kinds, ~4 lines.

**F3 — `migrate validate` returns success on an empty log directory.**
`crates/mochiko-cli/src/cli.rs:321-331`

`run_validate` calls `replay::load_full` directly, bypassing the empty-log check that
`load_for_delivery` (`src/cli.rs:203-210`) applies to every other path. An existing-but-empty
directory replays to an empty state with no findings:

```
$ mochiko-cli migrate validate --log-dir <empty dir>
mochiko-cli migrate validate · 0 rejecting · 0 advisory
[exit 0]
```

`rules`, `template` and `migrate status` all exit 1 there; validate alone reports green. A CI gate
wired to `migrate validate` passes on a mis-pointed `--log-dir`. Outside the §4 contract, which
says only "replays the log and prints findings", so advisory. **Fix:** route validate through the
same empty check, or emit an advisory finding for an empty log.

**F4 — `std::env::set_var` inside a parallel test binary.**
`crates/mochiko-cli/tests/cli.rs:648,650`

The env-limb test mutates process-global state while other tests in the same binary run on other
threads. The comment argues no concurrent test reads the variable, which holds today because every
other test passes an explicit flag and `resolve_log_dir` returns before the `env::var` call — but
the guarantee is a reading of every sibling test, not a property of the code, and `set_var` becomes
`unsafe` under edition 2024. It also clears the variable unconditionally rather than restoring a
pre-existing value. **Fix:** make `resolve_log_dir` take the variable's value as a parameter and
unit-test the ordering function directly, or serialize the test behind a mutex.

**F5 — two resolution limbs are exercised only by this audit, not by the suite.**

`tests/cli.rs` covers `--log-dir` beating `--plugin-root`, and the environment limb alone. It does
not cover the plugin root beating `$MOCHIKO_MIGRATIONS`, nor the `./migrations` working-directory
last resort. Both behave correctly — the grader verified each by hand (see §4) — but the ruled
order is only half-pinned by tests.

---

## 3. Criteria, graded

**1. Head and tail lines — PASS.** `render.rs:192-199` emits exactly
`mochiko-cli rules <primitive> · section <id> · binary <v> · grammar <n> · plugin <v|unknown>` and
`mochiko-cli rules end · <primitive> · <id> · <N> rules`, blank line either side of the body.
Character-for-character against D3-as-amended and §4 on every render the grader produced. `N`
equals the rules rendered: tombstoned ids leave `section.rules` at replay, so `section.rules.len()`
is the live count (verified by tombstoning a rule and re-rendering — the count fell from 2 to 1 and
the id vanished from both the section and the preamble's section list).

**Preamble — PASS, and the pins are right corpus-wide.** Carries the identity line, resolved
`vars`, the `conditions` block (dimension · values · resolution · note), `moments` for commands
only, the `kind: fail` pin for commands and the `class: floor` pin for all, then the section list
with per-section counts; `N` is 0. The grader cross-checked the *printed* pins against
`validate::census` over all 50 shipped files by parsing them back out of the rendered text:

| kind | census rules / floors | rendered section-list rules / floor pins | |
|---|---|---|---|
| command | 321 / 110 | 321 / 110 | MATCH |
| skill | 695 / 226 | 695 / 226 | MATCH |

Fail pins summed over the 36 commands and skills: **36**, equal to the 36 `.fail.` rule ids in the
shipped corpus. No command or skill schema carries top-level `blocks`, so the render's
`schema.rules()` and the census count the same set.

**2. Section body — PASS but for F1.** `## <title>`, intent, one `### <id>` block per live rule.
Bracket line in the ruled order, kind omitted when the effective kind is `constraint`, absent
fields omitted with their separator; observed:
`[class: must · kind: duty · when: depth=high · labels: seats · pointer: mochiko:validator]` and
`[class: advisory · labels: seats]`. `${var}` substituted, `extends:` resolved to inherited text /
labels / pointer over a local `class` — and the inherited text substitutes from the *binding*
schema's vars (verified: a common-library block carrying `${audit_path}` rendered the command's
binding). Tombstones never rendered; an `anchor:` folded on by `set-rule-field` never reached a
render; a rule `note:` never rendered; an empty section rendered its note at `N = 0`.

**No second implementation.** `render.rs:213` calls `validate::resolve_extends` with a discarded
sink and `render.rs:300` calls `validate::placeholders`. Grepped `src/` for duplicated inheritance
or placeholder scanning — none; the only other `extends` mention is a test assertion.

**3. Exit codes — PASS.** 0 / 1 / 2 / 3, with 3 taking precedence. A `grammar: 99` log exits 3 on
all four subcommands, before any other finding, and the message is `skew.message` — the `Finding`
built from `ParseError::GrammarVersion`'s Display, never a copy. `INSTALL_COMMAND` has exactly one
home (`src/migration.rs:20`, used only at `src/migration.rs:85`); `cli.rs` does not mention it.
Observed: `0001-genesis.yaml: the migration log is written in grammar 99, and this binary reads
grammar 1..1. Update the binary: cargo install mochiko-cli`.

Plugin version reads `<plugin-root>/.claude-plugin/plugin.json` (`plugin 0.103.0` on a scratch
root), `unknown` with no root, and `unknown` — never a halt — on a malformed manifest. Log dir
resolves in the ruled order, all four limbs exercised by the grader:

| given | log used |
|---|---|
| `--log-dir` + `--plugin-root` | the flag |
| `--plugin-root`, `MOCHIKO_MIGRATIONS` set | the plugin root's `migrations/` |
| no flags, env set, cwd holds `./migrations` | the environment |
| no flags, no env, cwd holds `./migrations` | `migrations` |

**4. Template re-base — PASS, independently verified.** The grader rebuilt the pre-P2 binary from
`cd5a333` in a scratch workspace, ran all 8 producer and all 8 `--check` views against the shipped
schemas, stripped the trailing source line, and diffed against the committed fixtures: **16/16
byte-identical, fail=0**. The fixtures are honest, not self-certified. `producer_view` and
`check_view` bodies are unchanged from `cd5a333` apart from doc comments (`git diff` confirms).
`TEMPLATE_NAMES`, all 8 `include_str!`, `resolve`, `parse`, `Resolved`, `ResolveError` and
`--schemas-dir` are gone from `src/`; grep finds zero `include_str!`, zero `plugins` references and
zero `schemas_dir` in the crate source. An unknown template exits 2; the shelf data file is not a
template and is refused. No fallback of any kind to files under `plugins/`.

**5. `--section` required, no whole-primitive path, ceiling reproduced — PASS.** `section: String`
is non-optional; `mochiko-cli rules audit` exits 2 naming the missing argument. `render` exposes
only `preamble` and `section`; `section` with `preamble` delegates. Ceiling test rerun with
`--nocapture`:

```
measured 252 renders; largest is implement · impl.sec.tools at 15450 chars (ceiling 30000)
```

Exactly the seat's figure. 252 = 36 primitives × (preamble + 6 sections). Headroom 1.94×.

**6. Hygiene — PASS.** `Cargo.toml` gained `clap` only, `default-features = false` with
`std, derive, help, usage, error-context`. `cargo tree` shows exactly six new crates — `clap`,
`clap_builder`, `clap_derive`, `clap_lex`, `anstyle`, `heck` — and no `anstream`, `colorchoice`,
`strsim` or `is_terminal_polyfill`; `Cargo.lock` adds those six and nothing else. No `unsafe`
anywhere in `src/` or `tests/`. No network dependency, no process spawning (`std::process` appears
once, for `exit`). Every test fixture path derives from `CARGO_TARGET_TMPDIR`. `git diff --stat
cd5a333 -- plugins/` is empty and the commit touches only the crate, its fixtures, `Cargo.lock` and
the seat's own report — no `plugin.json` bump, no `CHANGELOG.md`. The old `run()` and `USAGE` are
gone from `lib.rs` (23 lines, module declarations only). All six deviations are disclosed in the
seat's report; the seventh — `views emit`, named in wave-plan §1's P2 deliverables but absent from
§6's P2 checklist row — is disclosed as deviation 6 under the lead's D-4 delta to P3. GI-019 holds:
the CLI renders guidance and validates its own log; it grades no produced artifact, sequences
nothing, spawns nothing.

**7. Skeptical read — PASS but for F1–F4.** Zero `unwrap()`, `expect(`, `panic!`, `unreachable!`,
indexing or numeric casts in any of `cli.rs`, `render.rs`, `schema.rs`, `main.rs`, `lib.rs` — no
panic site on malformed state. Output is bounded by corpus size. Rendering is linear in practice:
252 shipped renders complete in 0.09 s, `resolve_extends` scanning a 50-document state per rule.
The clap surface admits no ambiguous command: three subcommands, two global path flags parsing in
either position, `--version` and `--help` exit 0, every usage error exits 2 with a message that
names what was missing. Error text is specific and actionable everywhere except F2 — the unknown
section even lists the sections that exist, `preamble` included.

---

## 4. What the grader ran

Gates: `cargo test --all`, `cargo fmt --all --check`, `cargo clippy --all-targets -- -D warnings`,
`cargo audit --deny warnings` — outputs in §1.

End-to-end, against a log the grader built (never the seat's fixtures): a two-migration corpus with
one command carrying the six command sections, one review-family skill, both common libraries, both
registries and one template, plus a second migration tombstoning a rule and folding an `anchor:`
onto a floor rule — stamped through P1's `migration::with_hash` from a scratch crate depending on
the unit by path. Exercised: `--version`; `rules <cmd> --section preamble`; all six command
sections; a skill preamble and an `extends`-bearing skill section; `template` and `template
--check`; `migrate status`; `migrate validate` and `--report`; an unknown primitive; an unknown
section; an unknown template; a missing `--section`; `--help`; no arguments; an unknown subcommand;
`migrate` with no action; a `grammar: 99` log on all four subcommands; an empty log directory; an
absent log directory; a plugin root supplying both version and log; and all four log-dir resolution
limbs. Separately: a log with a command and a skill sharing one name (F2), and a fail node given
the explicitly empty mirror (F1). Two scratch binaries were built outside the repository — one to
stamp migration hashes, one to sum the rendered pins against `validate::census`.

An early fixture of the grader's own tripped P1's hard set (a tombstone left a section empty with
no `note:`), which is the validator behaving correctly and confirmed that an unsound log is never
rendered from: every delivery path exited 1 with the finding, none rendered a head line.

**Method note.** I read the wave plan §4/§6/§8, the record's D3-as-amended, D5, D6, D9 and D11 plus
the post-acceptance amendments, and `.claude/rules/mochiko/rust-cli.md` before opening any code,
then read `cli.rs`, `render.rs`, `schema.rs`, `main.rs`, `lib.rs`, `Cargo.toml`, `tests/cli.rs` and
`tests/render.rs` in full, and the P1 functions they lean on (`resolve_extends`, `placeholders`,
`census`, `load_full`, `effective_kind`) to confirm no second implementation. I read the seat's
cycle report last and treated every claim in it as a hypothesis to test rather than evidence: the
two claims most worth doubting — that the 16 template fixtures are byte-honest, and that the
largest shipped render is 15,450 characters — were both re-derived independently, the first by
rebuilding the pre-P2 binary from `cd5a333` and regenerating all 16 views, the second by rerunning
the measurement. The pin claim was checked the same way, by parsing the printed numbers back out of
36 rendered preambles and summing them against the validator's own census. I edited no code. The
default posture was FAIL; PASS is what the evidence above bought.

---

# Delta-confirm — fix round 1

Graded at commit `07a39b4` (range `3792104..07a39b4`), read through `git diff` and `git show`
rather than the working tree, and executed in a scratch checkout unpacked with `git archive` under
`/private/tmp` with `plugins/mochiko` symlinked in. All three advisory findings **CONFIRMED
fixed**, each with a control proving no regression on the behaviour it sits beside.

**Gates, re-run by the grader on the scratch checkout of `07a39b4`:** `cargo test --all`
**164 passed / 0 failed** (cli 22 · migration 25 · render 27 · replay 46 · validate 44 — +2 and +1
over the reviewed unit, matching the seat's claim) · `cargo fmt --all --check` exit 0 ·
`cargo clippy --all-targets -- -D warnings` exit 0. The ceiling measurement is unmoved:
`measured 252 renders; largest is implement · impl.sec.tools at 15450 chars`. No file under
`plugins/` or `migrations/` is touched by the delta.

## F1 — empty `enforces:` mirror — **CONFIRMED**

`src/render.rs:244-249` now guards with `rule.enforces.as_deref().filter(|ids| !ids.is_empty())`,
so an empty mirror emits no key at all.

**I accept the reasoning for omission over rendering the note.** The `note:` is precisely what the
Q4 ruling excludes, and its origin settles it: in the shipped corpus the reason is a
`# D6 empty-with-reason:` YAML comment that only becomes a `note:` field because comments do not
survive a typed model. Surfacing it would carve a hole in a lead-ruled boundary without a ruling,
and would contradict the seat's own `no_render_carries_an_anchor_or_a_rule_note`. Omission is also
the honest reading of §4's "`enforces: <ids>` for fail nodes" — with no ids there is nothing to
print. The one residual is that a render can no longer distinguish "empty mirror by ruling" from
"no mirror declared", and that distinction has no observer: the hard set requires `enforces` on
every `kind: fail` rule, so the second state cannot exist.

Verified corpus-wide against the fixed crate — 252 shipped renders swept:

| measure | result |
|---|---|
| dangling key lines anywhere | **0** |
| non-empty `enforces:` lines still rendered | **34** (= 36 fail nodes − the 2 empty mirrors) |
| fail pins summed | 36 |
| command census vs rendered (rules/floors) | 321/110 vs 321/110 — MATCH |
| skill census vs rendered (rules/floors) | 695/226 vs 695/226 — MATCH |

The two live cases now render clean, and a sibling with a real mirror still prints it:

```
### setup.fail.unclosed-trace
[class: floor · kind: fail · labels: evidence]
An unclosed trace from ratified intent to authored surfaces.

### setup.fail.author-graded
[class: floor · kind: fail · labels: independence]
The governance surface set never graded by anyone but its author.
enforces: setup.author-grader-default-fail, setup.stress-test-cold-seat
```

Re-probed on the grader's own log too: the same fail node printed no key with `enforces: []` and
printed `enforces: audit.boundary` with the mirror restored. The new test's closing loop — no
rendered line may be a key with an empty value — generalises past `enforces`, which is stronger
than the finding asked for.

## F2 — ambiguous primitive name — **CONFIRMED**

`find_primitive` at `src/cli.rs:290-308` now returns `Result<DocRef, RenderError>` with a
`(true, true)` arm feeding a new `RenderError::AmbiguousPrimitive`. Probed on the grader's own
two-document log:

```
$ mochiko-cli rules twin --section preamble --log-dir <log>
error: ambiguous: the log carries both a command and a skill named 'twin' — the two name sets are meant to be disjoint, so this is a defect in the log, not in the request
[exit 2]
```

It names both kinds, points at the log rather than the request, and holds on a section render as
well as the preamble. Control: a genuinely absent name still reports absence with the
`migrate status` hint, so the original message was narrowed rather than replaced.

## F3 — `migrate validate` on an empty log — **CONFIRMED**

The empty-log halt is extracted to `report_empty_log` (`src/cli.rs:209-221`) and `run_validate`
gained an `Ok(replay) if …docs.is_empty()` arm ahead of the findings path. All five paths now agree
— `migrate validate`, `migrate validate --report`, `migrate status`, `rules`, `template` — each
exit 1 naming the directory:

```
mochiko-cli: the migration log at <dir> is empty — it carries no migration file
[exit 1]
```

Controls: a sound log still validates green (`0 rejecting · 5 advisory`, exit 0), and exit 3 still
takes precedence — a `grammar: 99` log halts with the unchanged D5 message on `migrate validate`,
not with a 1. The `Ok`/`Err` arms are disjoint, so the new guard cannot shadow the skew halt, and
the probe confirms it does not.

## F4 / F5 — carried, not raised

The two remaining advisories from the audit were not in the lead's fix scope and are unchanged:
`std::env::set_var` in the parallel test binary (`tests/cli.rs`), and the two log-dir resolution
limbs covered only by this audit's hand probes. Neither blocks; both remain open for a later round.

**Delta verdict: PASS.**
