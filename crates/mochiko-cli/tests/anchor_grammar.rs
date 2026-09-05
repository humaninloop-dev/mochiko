//! The decision segment of a ruling anchor, pinned against the live corpus.
//!
//! `model::is_anchor` accepts `YYYY-MM-DD <session-slug>` with at most one trailing decision
//! segment. The segment's number may carry a run of lowercase letters (`D2a`) — the spelling a
//! session uses when it amends a ruling in place, and the spelling two provenance anchors in this
//! repository already use. The suffix is the whole of the widening: it is letters only, it must
//! follow at least one digit, and everything the grammar rejected before it still rejects.
//!
//! The two live anchors are asserted **by rule id**, read from the sidecar rather than restated,
//! so this test fails if the corpus stops carrying the spelling that forced the widening.

use mochiko_cli::genesis;
use mochiko_cli::model::is_anchor;
use serde_norway::Value;
use std::collections::BTreeMap;
use std::path::PathBuf;

/// A well-formed anchor with `segment` appended, so each case varies only in the segment.
fn anchor_with(segment: &str) -> String {
    format!("2026-08-10 pm-requirements-stacking {segment}")
}

/// The frozen v0.103.0 corpus, which carries its own copy of the provenance sidecar.
///
/// From wave 6 the repo-side sidecar is frozen to `.mochiko/archive/`. The copy here is the one
/// genesis was built from, so it is the honest place to read the two live anchors this test is
/// about — and it moves with the fixture rather than with an archive path.
fn frozen_corpus() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/genesis-corpus")
}

/// The sidecar's anchors, read straight from disk.
fn sidecar() -> BTreeMap<String, String> {
    let path = frozen_corpus().join(genesis::SIDECAR);
    let text = std::fs::read_to_string(&path).expect("the provenance sidecar is readable");
    let value: Value = serde_norway::from_str(&text).expect("the sidecar parses");
    let Some(Value::Mapping(anchors)) = value.get("anchors") else {
        panic!("the sidecar carries an `anchors:` mapping");
    };
    anchors
        .iter()
        .filter_map(|(k, v)| Some((k.as_str()?.to_string(), v.as_str()?.to_string())))
        .collect()
}

#[test]
fn a_lettered_decision_number_is_well_formed() {
    assert!(is_anchor(&anchor_with("D2a")), "`D2a` is a ruling pointer");
    assert!(
        is_anchor(&anchor_with("[D2a]")),
        "the bracketed spelling widens with the bare one"
    );
    assert!(
        is_anchor(&anchor_with("D12abc")),
        "the suffix is a run of letters, not a single one"
    );
}

#[test]
fn the_shapes_that_were_malformed_stay_malformed() {
    let rejected = [
        ("D", "a pointer with no number"),
        ("Da", "letters with no number in front of them"),
        ("D2 D3", "two segments where the grammar allows one"),
        ("D2a and the rest of a sentence", "trailing prose"),
        ("D2A", "an uppercase suffix"),
        ("D2-a", "a punctuated suffix"),
        ("D2a1", "a digit after the letters"),
        ("D2a3", "a digit after the letters, the grant's own case"),
        ("2a", "a number with no `D`"),
    ];
    for (segment, why) in rejected {
        assert!(
            !is_anchor(&anchor_with(segment)),
            "`{segment}` is not a decision segment: {why}"
        );
    }
}

#[test]
fn the_forms_the_corpus_already_used_are_untouched() {
    assert!(is_anchor("2026-08-10 pm-requirements-stacking"));
    assert!(is_anchor(&anchor_with("D2")));
    assert!(is_anchor(&anchor_with("D4")));
    assert!(is_anchor(&anchor_with("[D2]")));
    assert!(!is_anchor("pm-requirements-stacking D2a"));
    assert!(!is_anchor("2026-13-10 pm-requirements-stacking D2a"));
}

#[test]
fn the_two_live_anchors_that_forced_the_widening_are_accepted() {
    let anchors = sidecar();
    for id in [
        "authoring-feature-map.selectability-specify-only",
        "authoring-feature-map.story-trace-provenance",
    ] {
        let anchor = anchors
            .get(id)
            .unwrap_or_else(|| panic!("the sidecar carries an anchor for `{id}`"));
        assert!(
            anchor.contains("D2a"),
            "`{id}` is the reason this test exists; its anchor is now `{anchor}`"
        );
        assert!(is_anchor(anchor), "`{id}` carries `{anchor}`");
    }
}

#[test]
fn no_anchor_in_the_corpus_is_malformed() {
    let malformed: Vec<_> = sidecar()
        .into_iter()
        .filter(|(_, anchor)| !is_anchor(anchor))
        .collect();
    assert!(
        malformed.is_empty(),
        "the sidecar carries anchors the grammar rejects: {malformed:?}"
    );
}
