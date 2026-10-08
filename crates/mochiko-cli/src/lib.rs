//! `mochiko-cli` — the mochiko plugin's rules and template delivery.
//!
//! Truth is the ordered migration log; the projection is replayed in memory at every invocation
//! (record D1). The binary reads that log and prints from it: one schema section at a time for a
//! command or skill's rules (D3 as amended), and the producer or checklist view of an artifact
//! template. It never gates pipeline progress, dispatches agents, or grades an artifact — the
//! standing bright line (GI-019).
//!
//! - [`migration`] — the file grammar, the change ops, the body hash, the version contract
//! - [`model`] — the typed document model and its canonical encoding
//! - [`replay`] — log loading and the replay engine
//! - [`validate`] — the hard set and the advisory reports
//! - [`render`] — the delivery render
//! - [`home`] — the artifact-home model, its path tokens, and path resolution
//! - [`conform`] — the six mechanical conformance checks and the first-touch amnesty
//! - [`hook`] — the PreToolUse payload and the decision JSON; shell targets come from [`shell`]
//! - [`shell`] — the shell write-position parse: which paths a Bash or PowerShell command writes
//! - [`schema`] — the artifact-template model and its two views
//! - [`cli`] — argument parsing, resolution order, and the exit-code contract
//! - [`genesis`] — the generator behind `plugins/mochiko/migrations/0001-genesis.yaml`
//! - [`views`] — the derived views, regenerated from the replayed state
//! - [`similar`] — the advisory similar-rule detector
//! - [`ids`] — the ID family table, the in-text ID scanner, and the advisory `ids --check`
//! - [`rename`] — `ids rename`, `ids rekey` and `ids literal`: preview-first rewrites behind a diff check

pub mod cli;
pub mod conform;
pub mod genesis;
pub mod home;
pub mod hook;
pub mod ids;
pub mod migration;
pub mod model;
pub mod rename;
pub mod render;
pub mod replay;
pub mod schema;
pub mod shell;
pub mod similar;
pub mod validate;
pub mod views;
