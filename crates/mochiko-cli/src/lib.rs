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
//! - [`schema`] — the artifact-template model and its two views
//! - [`cli`] — argument parsing, resolution order, and the exit-code contract
//! - [`genesis`] — the generator behind `migrations/0001-genesis.yaml`
//! - [`views`] — the derived views, regenerated from the replayed state
//! - [`similar`] — the advisory similar-rule detector

pub mod cli;
pub mod genesis;
pub mod migration;
pub mod model;
pub mod render;
pub mod replay;
pub mod schema;
pub mod similar;
pub mod validate;
pub mod views;
