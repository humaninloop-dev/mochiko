# evals/contract/ — the plugin contract suite (maintainer-side, never shipped)

Provenance: `.mochiko/brainstorms/cli-schema-delivery/record.md` **D8 as amended**. This is the
layer the crate's own tests can never be: `cargo test` proves the store is sound, and proves
nothing at all about whether a real Claude Code session, loading a real plugin, actually receives
the rules before the model acts. That claim — "the plugin doesn't fail" — is what the user asked
for, and it lives here.

## What one case is

One headless `claude -p` run inside the Docker AI sandbox `claude-mochiko`, loading the fixture
plugin under `fixture/probe-plugin/`, with `mochiko-cli` placed on the sandbox `PATH` the way a
user would install it (D4). The run's event stream is then asserted against D8's deterministic
set:

| assertion | what it catches |
|---|---|
| the `!` line executed | preprocessing did not run, or the grant was missing and the line was denied |
| the version-triple line present | the render never reached the model |
| the closing end line present | an oversized render truncated, keeping only its head line (wave-0 probe (e)) |
| no schema file Read | the model fell back to a file — the posture no-fallback exists to rule out |
| absence halts | a missing binary degraded silently instead of halting |
| skew halts | an out-of-range grammar was read best-effort instead of halting |

## The two cases that run at wave 1

Both are failure paths, because a success path needs a converted primitive and none exists yet
(the pilot is wave 3).

- **absence** — the binary is off the sandbox `PATH`. Nothing can be delivered; the run must halt
  and must not read a schema file.
- **skew** — the log declares `grammar: 99`. The binary halts with the D5 message naming the
  install command, rather than reading what it can.

The per-primitive cases, and the behavioural read-back metric with its pre-registered bar, arrive
at wave 3.

## Running it

```
python3 evals/contract/run.py           # run the cases
python3 evals/contract/run.py --list    # print the case list and exit
```

Exit codes are deliberately three-valued, because a suite that silently does nothing is worse
than one that fails:

| exit | meaning |
|---|---|
| 0 | every declared case ran and passed |
| 1 | a case ran and an assertion failed |
| 3 | **SKIPPED** — the suite could not run, with the reason printed |

The case list prints on every path, so "0 cases ran" is visible rather than inferred.

## Prerequisites, and why a skip is the honest answer

The runner checks, in order: `sbx` on `PATH` · the sandbox reachable · `claude` runnable inside
it · **the sandbox authenticated for headless runs** · the fixture present · `cargo` available in
the sandbox to build the binary there. Any failing rung exits 3.

The authentication rung matters most. An unauthenticated sandbox still starts a session and then
returns an error result, which a careless suite would read as a failed assertion — a red result
that says nothing about the plugin. **`sbx login` is the user's own action**; the suite never
attempts it.

The binary is built **inside** the sandbox rather than copied in: the sandbox is Linux and the
maintainer's host is macOS, so a host build is the wrong architecture. Before the crates.io
publish (wave 2) that is the only way to get a binary there; after it, `cargo install
mochiko-cli` inside the sandbox is the D4 shape and this step is replaced by it.

## Gate split, and the cost

Per D8 as amended: the **full contract suite is a maintainer-side gate at `plugin.json` bumps**,
run in the sandbox at no metered cost. **GitHub CI keeps the four crate layers** (`cargo test` ·
`fmt` · `clippy` · `audit`) and runs no headless sessions — the original D8's "API key in CI
secrets" clause is withdrawn. The sandbox is Linux and the host is macOS; together they are the
two OS rows, and there is no CI matrix.

## Caveat carried on record

The kinako record marks sandbox subscription auth a `Contested` ruling sustained against adverse
Terms-of-Service evidence: automated headless use of a consumer subscription may sit outside what
it permits. The user adopted D8's amendment with that on record. It is repeated here because this
is the file a future maintainer reads before running the suite.

## What is not here

The suite never edits the plugin, never dispatches an agent, and never grades a primitive's
content — it asserts delivery mechanics and nothing else (GI-019). Nothing in this directory
ships with the plugin (GI-020).
