//! `mochiko-cli ids`: the family table, the scanner, `--check`, and the rewrite commands
//! (`human-readable-ids` record D6, D7, D15 and their review and build changes).
//!
//! Every fixture tree is written under `CARGO_TARGET_TMPDIR` with its own `.git`, so it is its own
//! tree and never this repository's. The drift test is the one test that reads this repository: it
//! replays the shipped migration log.

use mochiko_cli::ids::{self, Form, Owner, Resolved, Scope};
use mochiko_cli::model::{DocRef, Document};
use mochiko_cli::replay;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

/// A scratch git tree: a directory holding its own `.git` (pattern of `tests/hook.rs:22`).
fn tree(tag: &str) -> PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("ids-{tag}-{n}"));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join(".git")).expect("scratch tree is creatable");
    dir
}

/// Write `body` at tree-relative `rel`, creating its directories.
fn put(root: &Path, rel: &str, body: &str) {
    let path = root.join(rel);
    std::fs::create_dir_all(path.parent().expect("a parent")).expect("dirs");
    std::fs::write(path, body).expect("fixture is writable");
}

fn read(root: &Path, rel: &str) -> String {
    std::fs::read_to_string(root.join(rel)).unwrap_or_else(|e| panic!("{rel}: {e}"))
}

/// The index over a whole scratch tree.
fn index(root: &Path) -> ids::Index {
    ids::Index::build(root, &ids::walk(root, &[], &[]))
}

fn name(owner: &str) -> Owner {
    Owner::Name(owner.to_string())
}

/// What each token of `text`, read as the file `rel`, resolves to, keyed by the token's text.
fn resolutions(root: &Path, rel: &str, text: &str) -> Vec<(String, Resolved)> {
    ids::resolve_file(&index(root), Path::new(rel), text)
        .into_iter()
        .map(|(token, resolved)| (text[token.start..token.end].to_string(), resolved))
        .collect()
}

/// The repository root, anchored at the crate directory.
fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fam(name: &str) -> &'static ids::Family {
    ids::family(name).unwrap_or_else(|| panic!("the table carries no `{name}` row"))
}

// ---------------------------------------------------------------------------
// T1 — the family table and the ID parse
// ---------------------------------------------------------------------------

#[test]
fn the_table_carries_every_f24_family_once() {
    let names: Vec<&str> = ids::FAMILIES.iter().map(|f| f.name).collect();
    let mut expected = vec![
        "GI",
        "FEAT",
        "EPIC",
        "AX",
        "SPN",
        "NFR",
        "GAP",
        "FR",
        "SC",
        "US",
        "SCR",
        "FLOW",
        "C",
        "D",
        "IP",
        "INT",
        "DS",
        "BR",
        "session D",
        "cycle C",
    ];
    let mut sorted = names.clone();
    sorted.sort();
    expected.sort();
    assert_eq!(
        sorted, expected,
        "the table's rows are F24's twenty families, once each"
    );
    for name in ["GI", "FEAT", "EPIC", "AX", "SPN", "NFR", "GAP"] {
        assert_eq!(
            fam(name).scope,
            Scope::Project,
            "{name} is project-wide (F24)"
        );
    }
    for name in [
        "FR",
        "SC",
        "US",
        "SCR",
        "FLOW",
        "C",
        "D",
        "IP",
        "INT",
        "DS",
        "BR",
        "session D",
        "cycle C",
    ] {
        assert_eq!(
            fam(name).scope,
            Scope::Artifact,
            "{name} restarts per artifact (F24)"
        );
    }
}

#[test]
fn padding_follows_each_family_minting_rule() {
    for name in ["US", "session D", "cycle C"] {
        assert_eq!(fam(name).form, Form::Unpadded, "{name} is unpadded (D9)");
    }
    for family in ids::FAMILIES {
        if !["US", "session D", "cycle C"].contains(&family.name) {
            assert_eq!(
                family.form,
                Form::Padded,
                "{} is three-digit (D9)",
                family.name
            );
        }
    }
}

#[test]
fn a_padded_core_parses_to_its_family_and_number() {
    let id = ids::parse_id("FR-012").expect("FR-012 parses");
    assert_eq!(id.family.name, "FR");
    assert_eq!(id.number, "012");
    let id = ids::parse_id("GI-0191").expect("four digits is at least three");
    assert_eq!((id.family.name, id.number.as_str()), ("GI", "0191"));
}

#[test]
fn unpadded_cores_and_sub_ids_parse() {
    let cases = [
        ("D7", "session D", "7"),
        ("D2a", "session D", "2a"),
        ("D4.1", "session D", "4.1"),
        ("C1", "cycle C", "1"),
        ("US-12", "US", "12"),
    ];
    for (text, family, number) in cases {
        let id = ids::parse_id(text).unwrap_or_else(|| panic!("{text} parses"));
        assert_eq!(
            (id.family.name, id.number.as_str()),
            (family, number),
            "{text}"
        );
    }
}

#[test]
fn the_hyphen_tells_product_decisions_from_session_decisions() {
    assert_eq!(ids::parse_id("D-007").map(|id| id.family.name), Some("D"));
    assert_eq!(
        ids::parse_id("D7").map(|id| id.family.name),
        Some("session D")
    );
    assert_eq!(ids::parse_id("C-001").map(|id| id.family.name), Some("C"));
    assert_eq!(
        ids::parse_id("C1").map(|id| id.family.name),
        Some("cycle C")
    );
    assert_eq!(ids::parse_id("DS-003").map(|id| id.family.name), Some("DS"));
}

#[test]
fn a_core_off_its_family_padding_or_with_a_tail_does_not_parse() {
    for text in [
        "GI-01",
        "C-1",
        "FR-12",
        "FR-012-csv",
        "PO-D1",
        "XFR-001",
        "D",
        "gi-004",
        "",
    ] {
        assert!(ids::parse_id(text).is_none(), "{text:?} is not one ID core");
    }
}

// ---------------------------------------------------------------------------
// T2 — the drift test: the table against the replayed log (D6 as changed at build, B3)
// ---------------------------------------------------------------------------

/// The text a family's minting source carries in the replayed log.
fn minting_text(state: &replay::State, mint: &ids::Mint) -> String {
    let doc = DocRef::new(mint.kind, mint.doc);
    let document = state
        .docs
        .get(&doc)
        .unwrap_or_else(|| panic!("the log carries no `{doc}`"));
    match (mint.rule, document) {
        (Some(rule), Document::Rules(schema)) => schema
            .find_rule(rule)
            .unwrap_or_else(|| panic!("`{doc}` carries no rule `{rule}`"))
            .text
            .clone()
            .unwrap_or_default(),
        (None, Document::Opaque(value)) => mochiko_cli::views::to_yaml(value),
        (rule, _) => panic!("`{doc}` is not the document kind {rule:?} expects"),
    }
}

/// The form a `shows` string writes for `prefix`: the first token of the prefix, read as digits,
/// `XXX`/`NNN` or `<n>`; or, where the rule spells the padding in words, "three-digit padded".
fn form_shown(shows: &str, prefix: &str) -> Option<Form> {
    let mut search = 0;
    while let Some(found) = shows[search..].find(prefix) {
        let at = search + found;
        search = at + prefix.len();
        let before = shows[..at].chars().next_back();
        if before.is_some_and(|c| c.is_ascii_alphanumeric() || c == '-') {
            continue;
        }
        let rest = &shows[at + prefix.len()..];
        if rest.starts_with("<n>") {
            return Some(Form::Unpadded);
        }
        let digits = rest.chars().take_while(|c| c.is_ascii_digit()).count();
        let marks = rest.chars().take_while(|c| *c == 'X' || *c == 'N').count();
        if digits >= 3 || marks >= 3 {
            return Some(Form::Padded);
        }
        if digits >= 1 {
            return Some(Form::Unpadded);
        }
    }
    shows.contains("three-digit padded").then_some(Form::Padded)
}

#[test]
fn every_minting_rule_still_shows_its_family_form_in_the_replayed_log() {
    let log = repo_root().join("plugins/mochiko/migrations");
    let replay = replay::load_full(&log).expect("the shipped log replays");
    let mut failures = Vec::new();
    let covered = ids::FAMILIES.iter().filter(|f| f.mint.is_some()).count();
    assert_eq!(
        covered, 18,
        "eighteen of the twenty families mint in the log"
    );
    for family in ids::FAMILIES {
        let Some(mint) = &family.mint else { continue };
        let text = minting_text(&replay.state, mint);
        if !text.contains(mint.shows) {
            failures.push(format!(
                "{}: the minting text no longer shows {:?}",
                family.name, mint.shows
            ));
        }
        if form_shown(mint.shows, family.prefix) != Some(family.form) {
            failures.push(format!(
                "{}: {:?} does not show the table's form {:?}",
                family.name, mint.shows, family.form
            ));
        }
    }
    assert!(failures.is_empty(), "drift:\n{}", failures.join("\n"));
}

#[test]
fn only_gap_and_br_have_no_log_minting_source() {
    let mut uncovered: Vec<&str> = ids::FAMILIES
        .iter()
        .filter(|f| f.mint.is_none())
        .map(|f| f.name)
        .collect();
    uncovered.sort();
    assert_eq!(
        uncovered,
        vec!["BR", "GAP"],
        "S2: GAP and BR mint in plugin prose only"
    );
}

// ---------------------------------------------------------------------------
// T3 — the scanner (D19, D5, D10, D12, S3, S4, S7)
// ---------------------------------------------------------------------------

/// `(text of the token, family name, slug, skip)` for every token in `text`.
fn tokens(text: &str) -> Vec<(String, &'static str, Option<String>, Option<ids::Skip>)> {
    ids::scan(text)
        .into_iter()
        .map(|t| {
            (
                text[t.start..t.end].to_string(),
                t.family.name,
                t.slug,
                t.skip,
            )
        })
        .collect()
}

fn token_texts(text: &str) -> Vec<String> {
    tokens(text).into_iter().map(|t| t.0).collect()
}

#[test]
fn bare_and_joined_tokens_are_read_with_their_slug() {
    let found = tokens("see GI-004 and GI-019-kernel-tooling-admission.");
    assert_eq!(found.len(), 2, "{found:?}");
    assert_eq!(found[0], ("GI-004".into(), "GI", None, None));
    assert_eq!(
        found[1],
        (
            "GI-019-kernel-tooling-admission".into(),
            "GI",
            Some("kernel-tooling-admission".into()),
            None
        )
    );
}

#[test]
fn a_slug_word_must_start_with_a_letter_or_the_token_is_a_local_label() {
    for text in ["D2-2fa-login", "C3-gate-2", "D1-7"] {
        let found = tokens(text);
        assert_eq!(found.len(), 1, "{text}: {found:?}");
        assert_eq!(
            found[0].3,
            Some(ids::Skip::Label),
            "{text} is a local label (D10, D19)"
        );
    }
}

#[test]
fn a_core_glued_to_a_word_is_not_a_token() {
    for text in ["PO-D1", "AD-D3", "XFR-001", "FR-001x2", "_D4", "v2D4"] {
        assert!(token_texts(text).is_empty(), "{text} carries no ID token");
    }
}

#[test]
fn a_run_folder_name_is_a_local_label() {
    let found = tokens(".mochiko/runs/FEAT-001-run2/log.md and FEAT-001-run2");
    assert!(
        found.iter().all(|t| t.3 == Some(ids::Skip::Label)),
        "{found:?}"
    );
    assert_eq!(found.len(), 2);
}

#[test]
fn both_ends_of_a_range_are_skipped() {
    let found = tokens("D1–D7 and FR-001–FR-004 and D3–5");
    let skips: Vec<_> = found.iter().map(|t| (t.0.as_str(), t.3)).collect();
    assert_eq!(
        skips,
        vec![
            ("D1", Some(ids::Skip::Range)),
            ("D7", Some(ids::Skip::Range)),
            ("FR-001", Some(ids::Skip::Range)),
            ("FR-004", Some(ids::Skip::Range)),
            ("D3", Some(ids::Skip::Range)),
        ]
    );
    let found = tokens("D1-D7");
    assert_eq!(
        found.len(),
        1,
        "the far end is glued to a hyphen: {found:?}"
    );
    assert_eq!(found[0].3, Some(ids::Skip::Range));
}

#[test]
fn an_ellipsis_joins_a_range_like_an_en_dash() {
    let found = tokens("C-001…C-004 and SC-001...SC-006");
    assert_eq!(found.len(), 4, "{found:?}");
    assert!(
        found.iter().all(|t| t.3 == Some(ids::Skip::Range)),
        "N1 as ruled: {found:?}"
    );
}

#[test]
fn a_list_of_four_or_more_stays_bare_and_three_is_still_a_list_of_mentions() {
    let four = tokens("D1, D2, D3 and D4");
    assert_eq!(four.len(), 4);
    assert!(
        four.iter().all(|t| t.3 == Some(ids::Skip::List)),
        "{four:?}"
    );
    let three = tokens("D1, D2 and D3");
    assert!(three.iter().all(|t| t.3.is_none()), "{three:?}");
    let slashes = tokens("GI-004 / GI-005 / GI-006 / GI-007");
    assert!(
        slashes.iter().all(|t| t.3 == Some(ids::Skip::List)),
        "{slashes:?}"
    );
}

#[test]
fn a_number_only_path_segment_is_skipped_and_a_slugged_file_name_is_a_mention() {
    let found = tokens("`.mochiko/features/FEAT-001/tasks.md`, `stories/US-12.md`");
    assert_eq!(found.len(), 2, "{found:?}");
    assert!(
        found.iter().all(|t| t.3 == Some(ids::Skip::Path)),
        "{found:?}"
    );

    let scanned = ids::scan("[AX](concerns/AX-001-tenancy.md)");
    assert_eq!(scanned.len(), 1);
    assert!(scanned[0].path && scanned[0].skip.is_none());
    assert_eq!(scanned[0].slug.as_deref(), Some("tenancy"));
}

#[test]
fn a_lower_case_prefix_is_read_in_a_file_name_only() {
    let scanned = ids::scan("prototype/scr-001-week-menu.html");
    assert_eq!(scanned.len(), 1, "S5: scr-001 is SCR-001 in a file name");
    assert_eq!(scanned[0].family.name, "SCR");
    assert_eq!(scanned[0].slug.as_deref(), Some("week-menu"));
    assert!(token_texts("scr-001 shows the menu; d3 draws it; c1 is a CSS class").is_empty());
}

#[test]
fn a_slash_pair_is_two_mentions_and_the_qualifier_covers_both() {
    let scanned = ids::scan("per `cli-schema-delivery` D4/D10.");
    assert_eq!(scanned.len(), 2, "{scanned:?}");
    for token in &scanned {
        assert!(!token.path && token.skip.is_none(), "{token:?}");
        assert_eq!(
            token.qualifier,
            Some(ids::Qualifier {
                text: "cli-schema-delivery".into(),
                code: true
            }),
            "S4: the qualifier covers its slash pair"
        );
    }
}

#[test]
fn a_plain_kebab_name_in_front_is_captured_and_a_plain_word_is_not() {
    let scanned = ids::scan("as adaptive-depth D7 rules; see record D4.");
    assert_eq!(scanned.len(), 2);
    assert_eq!(
        scanned[0].qualifier,
        Some(ids::Qualifier {
            text: "adaptive-depth".into(),
            code: false
        })
    );
    assert_eq!(
        scanned[1].qualifier, None,
        "`record` is one word, not a name"
    );
}

#[test]
fn a_feat_id_in_a_code_span_qualifies_a_cycle_and_is_itself_a_mention() {
    let scanned = ids::scan("`FEAT-001-family-accounts` C3-walking-skeleton-path");
    let names: Vec<&str> = scanned.iter().map(|t| t.family.name).collect();
    assert_eq!(names, vec!["FEAT", "cycle C"]);
    assert_eq!(
        scanned[1].qualifier.as_ref().map(|q| q.text.as_str()),
        Some("FEAT-001-family-accounts")
    );
}

#[test]
fn c4_model_level_names_are_labels_and_other_c4_is_a_cycle_token() {
    for text in [
        "C4-container",
        "C4 container level",
        "at C4 **container** level",
        "C4-level-3",
        "C4 level-3",
        "C4-model",
        "C4 context",
        "C4 component",
        "C4 code",
    ] {
        let found = tokens(text);
        assert_eq!(found.len(), 1, "{text}: {found:?}");
        assert_eq!(
            found[0].3,
            Some(ids::Skip::Label),
            "{text} is a C4 model term (S7)"
        );
    }
    let found = tokens("C4 stub");
    assert_eq!(found.len(), 1);
    assert_eq!((found[0].1, found[0].3), ("cycle C", None));
}

#[test]
fn sub_ids_ride_the_number_in_text() {
    let scanned = ids::scan("D2a and D4.1-waiver-revisit-timing.");
    let numbers: Vec<&str> = scanned.iter().map(|t| t.number.as_str()).collect();
    assert_eq!(numbers, vec!["2a", "4.1"]);
    assert_eq!(scanned[1].slug.as_deref(), Some("waiver-revisit-timing"));
}

#[test]
fn a_sentence_full_stop_is_not_a_file_extension() {
    let scanned = ids::scan("see FR-001.\nThen FR-002.Next");
    assert!(!scanned[0].path, "a full stop then a line break is prose");
    assert!(
        scanned[1].path,
        "a full stop then a letter reads as an extension"
    );
}

#[test]
fn nothing_is_scanned_inside_a_masked_span() {
    let text = "GI-001\n```\nGI-002\n```\n> GI-003\n\"GI-004\"\n<!-- GI-005 -->\nanchor: 2026-01-01 x D6\nGI-007\n";
    assert_eq!(token_texts(text), vec!["GI-001", "GI-007"]);
}

// ---------------------------------------------------------------------------
// T4 — the masks: verbatim quotes (D15), machine-read spots (D12)
// ---------------------------------------------------------------------------

/// The masked text of `text`, as a string of the masked characters.
fn masked_text(text: &str) -> String {
    let mask = ids::masked(text);
    text.char_indices()
        .filter(|(at, _)| mask[*at])
        .map(|(_, c)| c)
        .collect()
}

#[test]
fn a_fenced_block_is_masked_from_fence_to_fence() {
    let text = "a\n```rust\nGI-001\n```\nb\n";
    assert_eq!(masked_text(text), "```rust\nGI-001\n```\n");
}

#[test]
fn a_blockquote_line_is_masked() {
    assert_eq!(masked_text("a\n> quoted GI-001\nb"), "> quoted GI-001\n");
}

#[test]
fn a_double_quoted_span_is_masked_across_a_line_break() {
    let text = "it said \"the anchor:\nD9 stays\" then GI-001";
    assert_eq!(masked_text(text), "\"the anchor:\nD9 stays\"");
}

#[test]
fn an_unterminated_quote_masks_only_to_its_paragraph_end() {
    let text = "5\" screen GI-001\nstill here\n\nGI-002 \"x\" GI-003";
    assert_eq!(masked_text(text), "\" screen GI-001\nstill here\n\"x\"");
}

#[test]
fn an_escaped_quote_is_not_a_delimiter() {
    let text = "a \"b \\\" c\" d";
    assert_eq!(masked_text(text), "\"b \\\" c\"");
}

#[test]
fn a_gi_marker_comment_is_masked_and_another_comment_is_not() {
    let text = "x <!-- GI-001 · GI-021 --> y <!-- FR-001 note -->";
    assert_eq!(masked_text(text), "<!-- GI-001 · GI-021 -->");
}

#[test]
fn an_anchor_line_is_masked() {
    let text = "a\n  anchor: 2026-09-03 cli-schema-delivery D9\n- anchor: x D1\nb\n";
    assert_eq!(
        masked_text(text),
        "  anchor: 2026-09-03 cli-schema-delivery D9\n- anchor: x D1\n"
    );
}

// ---------------------------------------------------------------------------
// T5 — the definition index, the walk, and the resolver (R1–R4, D11 as changed at build, B1)
// ---------------------------------------------------------------------------

const INTENT: &str = ".mochiko/memory/governance-intent.md";

#[test]
fn gi_definitions_are_read_through_chained_leaders_in_the_intent_file() {
    let root = tree("gi-leaders");
    put(
        &root,
        INTENT,
        "- **GI-001 — Facts:** x\n| GI-008 | y |\n- **GI-007:** z\n- **GI-002-repo-type-shelf — Type:** w\n",
    );
    let index = index(&root);
    let gi = fam("GI");
    for number in ["001", "007", "008"] {
        let def = index
            .definition(gi, None, number)
            .unwrap_or_else(|| panic!("GI-{number}"));
        assert_eq!(def.slug, None, "GI-{number} is a bare definition");
        assert_eq!(def.path, PathBuf::from(INTENT));
    }
    assert_eq!(
        index
            .definition(gi, None, "002")
            .and_then(|d| d.slug.clone()),
        Some("repo-type-shelf".to_string())
    );
}

#[test]
fn a_ledger_heading_is_a_mention_and_the_first_definition_line_wins() {
    let root = tree("gi-first");
    put(
        &root,
        INTENT,
        "| GI-004 | FLOOR-TEST |\n- **GI-004-other-slug-here (FLOOR-TEST) — x**\n",
    );
    put(
        &root,
        ".mochiko/memory/governance-ledger.md",
        "### GI-004-ledger-slug-here — y\n### GI-005 — z\n",
    );
    let index = index(&root);
    let def = index.definition(fam("GI"), None, "004").expect("GI-004");
    assert_eq!(
        (def.slug.as_deref(), def.start),
        (None, Some(2)),
        "the table row comes first"
    );
    assert!(
        index.definition(fam("GI"), None, "005").is_none(),
        "R3: the ledger defines nothing"
    );
}

#[test]
fn a_file_name_slug_is_the_definition_slug() {
    let root = tree("feat-file");
    put(
        &root,
        ".mochiko/features/FEAT-003-wallet.md",
        "# FEAT-003 — Wallet\n",
    );
    let def = index(&root)
        .definition(fam("FEAT"), None, "003")
        .cloned()
        .expect("FEAT-003");
    assert_eq!((def.slug.as_deref(), def.start), (Some("wallet"), None));
}

#[test]
fn a_definition_line_whose_hyphen_run_is_no_slug_defines_nothing() {
    let root = tree("n2-defs");
    put(&root, INTENT, "- **GI-005-class — x**\n");
    put(
        &root,
        ".mochiko/specs/lunch-orders/spec.md",
        "| SCR-001-week-menu | Menu |\n",
    );
    put(
        &root,
        ".mochiko/specs/lunch-orders/prototype/scr-001-week-menu.html",
        "x\n",
    );
    put(
        &root,
        ".mochiko/specs/other-spec/spec.md",
        "| SCR-001-week-menu | Cart |\n",
    );
    let index = index(&root);
    assert!(
        index.definition(fam("GI"), None, "005").is_none(),
        "N2: a local label defines nothing"
    );
    assert_eq!(
        index
            .definition(fam("SCR"), Some(&name("lunch-orders")), "001")
            .and_then(|d| d.slug.clone()),
        Some("week-menu".to_string()),
        "S5: the screen file under its own spec carries the slug"
    );
    assert!(
        index
            .definition(fam("SCR"), Some(&name("other-spec")), "001")
            .is_none(),
        "another spec's screen file is not this spec's slug"
    );
}

#[test]
fn per_spec_definitions_are_owned_by_the_spec_directory_stories_included() {
    let root = tree("spec-owner");
    put(
        &root,
        ".mochiko/specs/lunch-orders/spec.md",
        "- **FR-012-csv-report-export** The system MUST export.\n| SCR-001 | Menu |\n",
    );
    put(
        &root,
        ".mochiko/specs/lunch-orders/stories/US-3.md",
        "### US-3 — Order lunch\n",
    );
    let index = index(&root);
    let owner = name("lunch-orders");
    assert_eq!(
        index
            .definition(fam("FR"), Some(&owner), "012")
            .and_then(|d| d.slug.clone()),
        Some("csv-report-export".into())
    );
    assert!(index.definition(fam("SCR"), Some(&owner), "001").is_some());
    assert!(
        index.definition(fam("US"), Some(&owner), "3").is_some(),
        "a story file belongs to its spec, not to `stories`"
    );
}

#[test]
fn product_file_nfr_and_cycle_definitions_are_found_at_their_sites() {
    let root = tree("sites");
    put(
        &root,
        ".mochiko/product/constraints-and-decisions.md",
        "### C-001 — Budget\n",
    );
    put(
        &root,
        ".mochiko/product/architecture/concerns.md",
        "## AX-001 Tenancy\n- **Targets**: NFR-014 (from SC-007) — p95 < 200ms\n",
    );
    put(
        &root,
        ".mochiko/features/FEAT-001/tasks.md",
        "### - [ ] Cycle 1: Walking skeleton\n",
    );
    let index = index(&root);
    assert!(index
        .definition(fam("C"), Some(&name("product")), "001")
        .is_some());
    assert!(index.definition(fam("AX"), None, "001").is_some());
    assert!(
        index.definition(fam("NFR"), None, "014").is_some(),
        "R8: the Targets leader"
    );
    let feat = Owner::Id {
        prefix: "FEAT-",
        number: "001".into(),
    };
    let cycle = index
        .definition(fam("cycle C"), Some(&feat), "1")
        .expect("cycle 1");
    assert_eq!(
        cycle.slug, None,
        "`Cycle 1` is a bare definition of C1 (H2)"
    );
}

#[test]
fn a_qualifier_resolves_a_per_spec_id_and_an_unqualified_one_elsewhere_is_unresolved() {
    let root = tree("qualify");
    put(
        &root,
        ".mochiko/specs/lunch-orders/spec.md",
        "- **FR-012** x\n",
    );
    let found = resolutions(
        &root,
        "notes.md",
        "`lunch-orders` FR-012; then FR-012; then `other-spec` FR-012",
    );
    assert_eq!(
        found,
        vec![
            ("FR-012".into(), Resolved::Owned(name("lunch-orders"))),
            ("FR-012".into(), Resolved::Unresolved),
            ("FR-012".into(), Resolved::Unresolved),
        ],
        "an owner the index does not hold resolves nothing"
    );
    let inside = resolutions(&root, ".mochiko/specs/lunch-orders/spec.md", "see FR-012");
    assert_eq!(
        inside,
        vec![("FR-012".into(), Resolved::Owned(name("lunch-orders")))]
    );
}

#[test]
fn a_cycle_resolves_in_its_own_tasks_file_or_behind_a_feat_qualifier_only() {
    let root = tree("cycles");
    put(
        &root,
        ".mochiko/features/FEAT-001/tasks.md",
        "### - [ ] Cycle 3: x\n",
    );
    let feat = Resolved::Owned(Owner::Id {
        prefix: "FEAT-",
        number: "001".into(),
    });
    let own = resolutions(&root, ".mochiko/features/FEAT-001/tasks.md", "after C3");
    assert_eq!(own, vec![("C3".into(), feat.clone())]);
    let qualified = resolutions(&root, "notes.md", "`FEAT-001-x-y-z` C3; later C3");
    assert_eq!(
        qualified,
        vec![
            ("FEAT-001-x-y-z".into(), Resolved::Project),
            ("C3".into(), feat),
            ("C3".into(), Resolved::Unresolved),
        ]
    );
}

#[test]
fn a_name_before_a_session_decision_resolves_only_to_an_existing_session() {
    let root = tree("r1");
    put(
        &root,
        ".mochiko/brainstorms/x-session/record.md",
        "### D7 — x\n",
    );
    put(
        &root,
        ".mochiko/brainstorms/adaptive-depth/record.md",
        "### D7 — y\n",
    );
    let rel = ".mochiko/brainstorms/x-session/record.md";
    let found = resolutions(
        &root,
        rel,
        "D7 here; adaptive-depth D7; producer-plan D6; `field-review` D2",
    );
    assert_eq!(
        found,
        vec![
            ("D7".into(), Resolved::Owned(name("x-session"))),
            ("D7".into(), Resolved::Owned(name("adaptive-depth"))),
            ("D6".into(), Resolved::Unresolved),
            ("D2".into(), Resolved::Unresolved),
        ],
        "R1: a non-session name in front leaves the mention unresolved, even in a record"
    );
}

#[test]
fn a_line_naming_exactly_one_session_owns_its_bare_decisions() {
    let root = tree("b1");
    put(
        &root,
        ".mochiko/brainstorms/foo-bar/record.md",
        "### D3 — x\n",
    );
    put(
        &root,
        ".mochiko/brainstorms/baz-qux/record.md",
        "### D3 — y\n",
    );
    let one = "| 2026-10-08 | ruled D3 | [record](.mochiko/brainstorms/foo-bar/record.md) |";
    assert_eq!(
        resolutions(&root, "DECISIONS.md", one),
        vec![("D3".into(), Resolved::Owned(name("foo-bar")))]
    );
    let lead = "- `foo-bar` D1 widened, D3 kept";
    assert_eq!(
        resolutions(&root, "BACKLOG.md", lead)[1],
        ("D3".into(), Resolved::Unresolved),
        "B1 as narrowed (user ruling C): a qualifier never owns the line"
    );
    let two = "D3 per [a](foo-bar/record.md) and [b](baz-qux/record.md)";
    assert_eq!(
        resolutions(&root, "index.md", two),
        vec![("D3".into(), Resolved::Unresolved)]
    );
    let shorthand = "D3 per [a](foo-bar/record.md), see producer-plan D6";
    assert_eq!(
        resolutions(&root, "index.md", shorthand)[0],
        ("D3".into(), Resolved::Unresolved),
        "(e) R1: an unresolved name counts toward B1's exactly-one test"
    );
}

#[test]
fn the_walk_skips_nested_trees_history_and_scratch_and_takes_exclusions() {
    let root = tree("walk");
    for rel in [
        "CLAUDE.md",
        "docs/a.md",
        "nested/.git/HEAD",
        "nested/b.md",
        ".claude/worktrees/w/c.md",
        ".claude/settings.local.json",
        ".mochiko/runs/FEAT-001-run1/d.md",
        ".mochiko/archive/e.md",
        ".mochiko/schema-views/f.yaml",
        "plugins/mochiko/migrations/0001-genesis.yaml",
        "crates/mochiko-cli/tests/fixtures/genesis-corpus/g.md",
        "CHANGELOG.md",
        "evals/x/fixtures/h.md",
        "evals/x/runs/i.md",
        "evals/contract/fixture/j.md",
        "evals/x/README.md",
        "evals/.work/k.md",
        "target/l.md",
        "node_modules/m.md",
    ] {
        put(&root, rel, "x");
    }
    let mut files: Vec<String> = ids::walk(&root, &[], &["evals/.work/".into()])
        .into_iter()
        .map(|p| p.to_string_lossy().into_owned())
        .collect();
    files.sort();
    assert_eq!(files, vec!["CLAUDE.md", "docs/a.md", "evals/x/README.md"]);
    let under: Vec<PathBuf> = ids::walk(&root, &[PathBuf::from("docs")], &[]);
    assert_eq!(under, vec![PathBuf::from("docs/a.md")]);
}

// ---------------------------------------------------------------------------
// T6 — `ids --check` (D6, D16, D18, R2, R9, S5, S8)
// ---------------------------------------------------------------------------

/// `(path, line, kind, token)` per finding of a whole-tree check.
fn findings(
    root: &Path,
    paths: &[&str],
    excludes: &[&str],
) -> Vec<(String, usize, ids::Kind, String)> {
    let paths: Vec<PathBuf> = paths.iter().map(PathBuf::from).collect();
    let excludes: Vec<String> = excludes.iter().map(|e| e.to_string()).collect();
    ids::check(root, &paths, &excludes)
        .into_iter()
        .map(|f| {
            (
                f.path.to_string_lossy().into_owned(),
                f.line,
                f.kind,
                f.token,
            )
        })
        .collect()
}

/// A tree whose intent file defines GI-001 joined and GI-002 bare.
fn gi_tree(tag: &str) -> PathBuf {
    let root = tree(tag);
    put(
        &root,
        INTENT,
        "- **GI-001-repo-secret-hygiene — x**\n- **GI-002 — y**\n",
    );
    root
}

#[test]
fn a_tree_citing_every_definition_by_its_slug_is_clean() {
    let root = gi_tree("clean");
    put(
        &root,
        "CLAUDE.md",
        "per GI-001-repo-secret-hygiene and GI-002\n",
    );
    assert_eq!(findings(&root, &["CLAUDE.md"], &[]), vec![]);
}

#[test]
fn a_bare_mention_of_a_joined_definition_is_bare() {
    let root = gi_tree("bare");
    put(&root, "CLAUDE.md", "x\nper GI-001 here\n");
    assert_eq!(
        findings(&root, &["CLAUDE.md"], &[]),
        vec![("CLAUDE.md".into(), 2, ids::Kind::Bare, "GI-001".into())]
    );
}

#[test]
fn a_bare_definition_is_reported_once_and_its_bare_citations_are_not() {
    let root = gi_tree("bare-def");
    put(&root, "CLAUDE.md", "per GI-002\n");
    assert_eq!(
        findings(&root, &[], &[]),
        vec![(INTENT.into(), 2, ids::Kind::Bare, "GI-002".into())],
        "D16 as changed at review: the definition decides"
    );
}

#[test]
fn a_mention_with_another_slug_drifts_and_names_the_definition_slug() {
    let root = gi_tree("drift");
    put(
        &root,
        "CLAUDE.md",
        "GI-001-secrets-outside-repo and GI-002-type-of-repo\n",
    );
    let found = ids::check(&root, &[PathBuf::from("CLAUDE.md")], &[]);
    let got: Vec<_> = found
        .iter()
        .map(|f| (f.kind, f.token.as_str(), f.definition.clone()))
        .collect();
    assert_eq!(
        got,
        vec![
            (
                ids::Kind::Drift,
                "GI-001-secrets-outside-repo",
                Some(Some("repo-secret-hygiene".into()))
            ),
            (ids::Kind::Drift, "GI-002-type-of-repo", Some(None)),
        ],
        "a joined citation of a bare definition is drift too (D16 S8)"
    );
}

#[test]
fn an_id_with_no_definition_anywhere_is_never_reported() {
    let root = tree("no-def");
    put(
        &root,
        "notes.md",
        "GAP-001 and GAP-002-floor-test-gap\nkinako FEAT-002 ships\n",
    );
    assert_eq!(findings(&root, &[], &[]), vec![], "D18 as changed at build");
}

#[test]
fn an_unresolved_bare_cycle_is_a_local_label_and_a_qualified_one_is_checked() {
    let root = tree("r9");
    put(
        &root,
        ".mochiko/brainstorms/foo-bar/record.md",
        "### D1 — x\nreview C3 stands\n",
    );
    put(
        &root,
        ".mochiko/features/FEAT-001/reports/r.md",
        "### C3: question\n",
    );
    put(
        &root,
        ".mochiko/features/FEAT-001/tasks.md",
        "### - [ ] C3-walking-skeleton-path: x\n",
    );
    put(&root, "notes.md", "`FEAT-001-x-y-z` C3\n");
    assert_eq!(
        findings(&root, &[], &[]),
        vec![
            (
                ".mochiko/brainstorms/foo-bar/record.md".into(),
                1,
                ids::Kind::Bare,
                "D1".into()
            ),
            ("notes.md".into(), 1, ids::Kind::Bare, "C3".into()),
        ],
        "R9: only the FEAT-qualified C3 is a cycle mention"
    );
}

#[test]
fn an_unresolved_name_before_a_decision_is_not_reported() {
    let root = tree("r1-check");
    put(&root, "notes.md", "as adaptive-depth D7 says\n");
    assert_eq!(findings(&root, &[], &[]), vec![], "D18 as changed at build");
}

#[test]
fn a_joined_unqualified_per_spec_mention_outside_its_spec_is_not_reported() {
    let root = tree("s5");
    put(
        &root,
        ".mochiko/specs/lunch-orders/spec.md",
        "- **FR-012-csv-report-export** x\n",
    );
    put(&root, "tasks-notes.md", "FR-012-csv-report-export\n");
    assert_eq!(
        findings(&root, &["tasks-notes.md"], &[]),
        vec![],
        "S5, accepted risk"
    );
}

#[test]
fn repo_only_forms_and_the_allowlist_are_never_flagged() {
    let root = gi_tree("allow");
    put(
        &root,
        "CLAUDE.md",
        "PO-D1 AM-5 OQ-2 J-1 RGI-7\nD1–D7 and GI-001, GI-002, GI-003, GI-004\n\"GI-001\" <!-- GI-001 -->\n> GI-001\n```\nGI-001\n```\nanchor: 2026-01-01 x D9\n.mochiko/features/FEAT-001/tasks.md C3-gate-2 C4-container\n",
    );
    assert_eq!(findings(&root, &["CLAUDE.md"], &[]), vec![]);
}

#[test]
fn nested_trees_scratch_and_the_settings_file_are_never_read() {
    let root = tree("r2");
    put(&root, INTENT, "- **GI-001-repo-secret-hygiene — x**\n");
    put(&root, "nested/.git/HEAD", "x");
    put(&root, "nested/a.md", "GI-001\n");
    put(&root, ".claude/worktrees/w/b.md", "GI-001\n");
    put(&root, ".mochiko/runs/FEAT-001-run1/c.md", "GI-001\n");
    put(
        &root,
        ".claude/settings.local.json",
        "{\"token\": \"GI-001 sk-planted-secret\"}\n",
    );
    put(&root, "evals/.work/d.md", "GI-001\n");
    put(&root, "e.md", "GI-001\n");
    assert_eq!(
        findings(&root, &[], &["evals/.work/"]),
        vec![("e.md".into(), 1, ids::Kind::Bare, "GI-001".into())]
    );
}

#[test]
fn a_finding_carries_its_column_in_characters() {
    let root = tree("col");
    put(&root, INTENT, "- **GI-001-repo-secret-hygiene — x**\n");
    put(&root, "a.md", "— é GI-001\n");
    let found = ids::check(&root, &[], &[]);
    assert_eq!((found[0].line, found[0].col), (1, 5));
    assert_eq!(found[0].to_string(), "a.md:1:5 · bare · GI-001");
}

#[test]
fn a_hyphen_run_that_is_no_slug_makes_a_local_label_and_three_words_still_drift() {
    let root = tree("n2-check");
    put(&root, INTENT, "- **GI-005-repo-type-shelf — x**\n");
    put(
        &root,
        ".mochiko/brainstorms/foo-bar/record.md",
        "### D8-profile-leaves-setup — x\n### D3-depth-stays-fixed — y\nD8-as-amended holds; D3-first wins\n",
    );
    put(
        &root,
        ".mochiko/features/FEAT-003-wallet.md",
        "# FEAT-003-wallet — Wallet\n",
    );
    put(
        &root,
        "notes.md",
        "GI-005-class rules\nGI-005-repo-type-other\nFEAT-003-wallet ships; FEAT-003 too\n",
    );
    assert_eq!(
        findings(&root, &[], &[]),
        vec![
            (
                "notes.md".into(),
                2,
                ids::Kind::Drift,
                "GI-005-repo-type-other".into()
            ),
            ("notes.md".into(), 3, ids::Kind::Bare, "FEAT-003".into()),
        ],
        "N2 as ruled: only three words or the file name's slug is a slug"
    );
}

// ---------------------------------------------------------------------------
// T7 — `ids rename` (D7, D12, D14, D15, R1, R4, R5, B2)
// ---------------------------------------------------------------------------

use mochiko_cli::rename;

fn rename_plan(root: &Path, owning: &str, id: &str, slug: &str, aliases: &[&str]) -> rename::Plan {
    let aliases: Vec<String> = aliases.iter().map(|a| a.to_string()).collect();
    rename::plan_rename(root, Path::new(owning), id, slug, &aliases, &[])
        .unwrap_or_else(|e| panic!("rename {id} is refused: {}", e.0))
}

fn apply(root: &Path, plan: &rename::Plan) -> (usize, usize) {
    rename::write(root, plan).unwrap_or_else(|e| panic!("the write is blocked: {e}"))
}

#[test]
fn a_rename_previews_and_writes_nothing_until_written() {
    let root = gi_tree("preview");
    put(&root, "CLAUDE.md", "per GI-001 here\n");
    let plan = rename_plan(&root, INTENT, "GI-001", "secret-hygiene-rules", &[]);
    let shown = rename::preview(&plan);
    assert!(
        shown.contains("CLAUDE.md:1\n-per GI-001 here\n+per GI-001-secret-hygiene-rules here\n"),
        "{shown}"
    );
    assert_eq!(
        read(&root, "CLAUDE.md"),
        "per GI-001 here\n",
        "a preview writes nothing"
    );
    let (files, spans) = apply(&root, &plan);
    assert_eq!(
        (files, spans),
        (2, 2),
        "the definition line and the mention"
    );
    assert_eq!(
        read(&root, "CLAUDE.md"),
        "per GI-001-secret-hygiene-rules here\n"
    );
    assert_eq!(
        read(&root, INTENT),
        "- **GI-001-secret-hygiene-rules — x**\n- **GI-002 — y**\n"
    );
}

#[test]
fn a_rename_skips_the_allowlist_and_ranges_and_long_lists() {
    let root = gi_tree("allow-rw");
    let body = "\"GI-001\" <!-- GI-001 -->\n> GI-001\n```\nGI-001\n```\nanchor: 2026-01-01 x D9\nGI-001–GI-004 and GI-001, GI-002, GI-003, GI-004\n\nand GI-001/GI-002\n";
    put(&root, "CLAUDE.md", body);
    let plan = rename_plan(&root, INTENT, "GI-001", "repo-secret-hygiene", &[]);
    let edit = plan
        .edits
        .iter()
        .find(|e| e.path == Path::new("CLAUDE.md"))
        .expect("CLAUDE.md edit");
    assert_eq!(
        edit.new,
        body.replace("and GI-001/GI-002", "and GI-001-repo-secret-hygiene/GI-002"),
        "only the slash pair's GI-001 is a mention to rewrite"
    );
}

#[test]
fn a_per_spec_rename_stays_inside_its_spec_and_its_qualified_cites() {
    let root = tree("spec-rw");
    put(
        &root,
        ".mochiko/specs/lunch-orders/spec.md",
        "- **FR-001** x\nsee FR-001\n",
    );
    put(
        &root,
        ".mochiko/specs/other-spec/spec.md",
        "- **FR-001** y\n",
    );
    put(
        &root,
        ".mochiko/features/FEAT-001/tasks.md",
        "`lunch-orders` FR-001; then FR-001; then `other-spec` FR-001\n",
    );
    let plan = rename_plan(
        &root,
        ".mochiko/specs/lunch-orders/spec.md",
        "FR-001",
        "csv-report-export",
        &[],
    );
    apply(&root, &plan);
    assert_eq!(
        read(&root, ".mochiko/specs/lunch-orders/spec.md"),
        "- **FR-001-csv-report-export** x\nsee FR-001-csv-report-export\n"
    );
    assert_eq!(
        read(&root, ".mochiko/specs/other-spec/spec.md"),
        "- **FR-001** y\n"
    );
    assert_eq!(
        read(&root, ".mochiko/features/FEAT-001/tasks.md"),
        "`lunch-orders` FR-001-csv-report-export; then FR-001; then `other-spec` FR-001\n",
        "D7: never the bare number project-wide"
    );
}

#[test]
fn a_session_rename_reaches_line_named_rows_and_leaves_foreign_names_alone() {
    let root = tree("r1-rw");
    let record = ".mochiko/brainstorms/x-session/record.md";
    put(
        &root,
        record,
        "### D7 — Topic\nD7 here; adaptive-depth D7; x-session D7\n",
    );
    put(
        &root,
        "DECISIONS.md",
        "| ruled D7 | [record](.mochiko/brainstorms/x-session/record.md) |\nx-session D7 and `x-session` D7\n",
    );
    let plan = rename_plan(&root, record, "D7", "frame-hardens-early", &[]);
    apply(&root, &plan);
    assert_eq!(
        read(&root, record),
        "### D7-frame-hardens-early — Topic\nD7-frame-hardens-early here; adaptive-depth D7; x-session D7-frame-hardens-early\n",
        "R1: `adaptive-depth` names no session here, so its D7 is untouched; a plain name stays plain"
    );
    assert_eq!(
        read(&root, "DECISIONS.md"),
        "| ruled D7-frame-hardens-early | [record](.mochiko/brainstorms/x-session/record.md) |\nx-session D7-frame-hardens-early and `x-session` D7-frame-hardens-early\n"
    );
}

#[test]
fn a_rename_moves_the_file_carrying_the_slug_and_rewrites_its_links() {
    let root = tree("move");
    put(
        &root,
        ".mochiko/features/FEAT-003-wallet.md",
        "# FEAT-003 — Wallet\n",
    );
    put(
        &root,
        "FEATURES.md",
        "| [FEAT-003](.mochiko/features/FEAT-003-wallet.md) | Wallet |\n",
    );
    let plan = rename_plan(
        &root,
        ".mochiko/features/FEAT-003-wallet.md",
        "FEAT-003",
        "digital-wallet-accounts",
        &[],
    );
    assert!(rename::preview(&plan).contains(
        "move .mochiko/features/FEAT-003-wallet.md to .mochiko/features/FEAT-003-digital-wallet-accounts.md"
    ));
    apply(&root, &plan);
    assert!(!root.join(".mochiko/features/FEAT-003-wallet.md").exists());
    assert_eq!(
        read(
            &root,
            ".mochiko/features/FEAT-003-digital-wallet-accounts.md"
        ),
        "# FEAT-003-digital-wallet-accounts — Wallet\n"
    );
    assert_eq!(
        read(&root, "FEATURES.md"),
        "| [FEAT-003-digital-wallet-accounts](.mochiko/features/FEAT-003-digital-wallet-accounts.md) | Wallet |\n"
    );
}

#[test]
fn the_existing_file_name_slug_is_accepted_whatever_its_length() {
    let root = tree("short-slug");
    put(
        &root,
        ".mochiko/features/FEAT-003-wallet.md",
        "# FEAT-003 — Wallet\n",
    );
    put(&root, "FEATURES.md", "FEAT-003\n");
    let plan = rename_plan(
        &root,
        ".mochiko/features/FEAT-003-wallet.md",
        "FEAT-003",
        "wallet",
        &[],
    );
    assert!(plan.moves.is_empty());
    apply(&root, &plan);
    assert_eq!(
        read(&root, "FEATURES.md"),
        "FEAT-003-wallet\n",
        "S5: the back-fill keeps the file's slug"
    );
}

#[test]
fn a_rename_rewrites_the_file_name_slug_and_never_a_local_label() {
    let root = tree("n2-rw");
    put(
        &root,
        ".mochiko/features/FEAT-003-wallet.md",
        "# FEAT-003 — Wallet\n",
    );
    put(
        &root,
        "notes.md",
        "FEAT-003-wallet ships; FEAT-003-wallet-card stays\n",
    );
    let plan = rename_plan(
        &root,
        ".mochiko/features/FEAT-003-wallet.md",
        "FEAT-003",
        "digital-wallet-accounts",
        &[],
    );
    apply(&root, &plan);
    assert_eq!(
        read(&root, "notes.md"),
        "FEAT-003-digital-wallet-accounts ships; FEAT-003-wallet-card stays\n",
        "N2 as ruled: `wallet` is the file's slug; `wallet-card` makes a label"
    );
}

#[test]
fn an_alias_rewrites_session_prefixed_and_shorthand_forms_to_the_qualified_form() {
    let root = tree("alias");
    let record = ".mochiko/brainstorms/production-only-focus/record.md";
    put(&root, record, "### D1 — Floor\n### D6 — Plan\n");
    put(
        &root,
        "CLAUDE.md",
        "PO-D1 rules; PO-D1–D7 stays; producer-plan D6 too\n",
    );
    let plan = rename_plan(&root, record, "D1", "one-production-floor", &["PO-D"]);
    apply(&root, &plan);
    assert_eq!(
        read(&root, "CLAUDE.md"),
        "`production-only-focus` D1-one-production-floor rules; PO-D1–D7 stays; producer-plan D6 too\n"
    );
    let plan = rename_plan(
        &root,
        record,
        "D6",
        "producer-plan-seats",
        &["producer-plan D=CLAUDE.md"],
    );
    apply(&root, &plan);
    assert!(
        read(&root, "CLAUDE.md")
            .ends_with("; `production-only-focus` D6-producer-plan-seats too\n"),
        "R1: a shorthand name normalizes through --alias"
    );
}

#[test]
fn a_feat_rename_rewrites_the_feat_id_qualifying_a_cycle() {
    let root = tree("feat-qual");
    put(
        &root,
        ".mochiko/features/FEAT-001-old.md",
        "# FEAT-001 — x\n",
    );
    put(&root, "notes.md", "`FEAT-001-old` C3\n");
    let plan = rename_plan(
        &root,
        ".mochiko/features/FEAT-001-old.md",
        "FEAT-001",
        "family-account-sharing",
        &[],
    );
    apply(&root, &plan);
    assert_eq!(
        read(&root, "notes.md"),
        "`FEAT-001-family-account-sharing` C3\n"
    );
}

#[test]
fn a_rename_is_refused_before_anything_is_planned() {
    let root = tree("refuse");
    put(&root, INTENT, "- **GI-001 — x**\n");
    put(&root, "notes.md", "GAP-001\n");
    put(
        &root,
        ".mochiko/specs/lunch-orders/spec.md",
        "| SCR-001 | Menu |\n",
    );
    put(
        &root,
        ".mochiko/specs/lunch-orders/prototype/scr-001-week-menu.html",
        "a\n",
    );
    put(
        &root,
        ".mochiko/specs/lunch-orders/prototype/scr-001-weekly-menu-grid.html",
        "b\n",
    );
    let refused = |owning: &str, id: &str, slug: &str, why: &str| {
        let reason = rename::plan_rename(&root, Path::new(owning), id, slug, &[], &[])
            .err()
            .unwrap_or_else(|| panic!("{id} {slug} plans"))
            .0;
        assert!(reason.contains(why), "{id} {slug}: {reason}");
    };
    refused(
        "notes.md",
        "GAP-001",
        "floor-test-gap",
        "holds no definition",
    ); // R3
    refused(INTENT, "GI-1", "a-b-c", "is not an ID");
    refused(
        INTENT,
        "GI-001",
        "repo-secret-hygiene-rules",
        "is not a slug",
    ); // R5: four words
    refused(INTENT, "GI-001", "secret-hygiene", "is not a slug"); // two words
    refused(INTENT, "GI-001", "Repo-secret-hygiene", "is not a slug");
    refused(INTENT, "GI-001", "repo-2fa-rules", "is not a slug");
    refused(
        "notes.md",
        "GI-001",
        "repo-secret-hygiene",
        "holds no definition",
    );
    refused(
        ".mochiko/specs/lunch-orders/spec.md",
        "SCR-001",
        "weekly-menu-grid",
        "disagree",
    ); // H4: two screen files, two slugs
    let excluded =
        [".mochiko/specs/lunch-orders/prototype/scr-001-weekly-menu-grid.html".to_string()];
    let reason = rename::plan_rename(
        &root,
        Path::new(".mochiko/specs/lunch-orders/spec.md"),
        "SCR-001",
        "weekly-menu-grid",
        &[],
        &excluded,
    )
    .err()
    .map(|r| r.0)
    .unwrap_or_default();
    assert!(
        reason.contains("already exists"),
        "a target outside the walk still blocks: {reason}"
    );
}

#[test]
fn a_rename_never_writes_into_nested_trees_scratch_or_settings() {
    let root = gi_tree("r2-rw");
    put(&root, "nested/.git/HEAD", "x");
    put(&root, "nested/a.md", "GI-001\n");
    put(&root, ".claude/settings.local.json", "GI-001\n");
    put(&root, ".claude/worktrees/w/b.md", "GI-001\n");
    let plan = rename_plan(&root, INTENT, "GI-001", "repo-secret-hygiene", &[]);
    assert_eq!(
        plan.edits.len(),
        0,
        "the definition already carries the slug"
    );
    let plan = rename_plan(&root, INTENT, "GI-001", "secret-hygiene-rules", &[]);
    let paths: Vec<_> = plan.edits.iter().map(|e| e.path.clone()).collect();
    assert_eq!(paths, vec![PathBuf::from(INTENT)]);
    assert!(
        !rename::preview(&plan).contains("settings"),
        "S8: never printed"
    );
}

#[test]
fn a_number_only_path_keeps_its_number_through_a_rename() {
    let root = tree("path-rw");
    put(
        &root,
        ".mochiko/features/FEAT-001-old.md",
        "# FEAT-001 — x\n",
    );
    put(
        &root,
        "notes.md",
        "see .mochiko/features/FEAT-001/tasks.md for FEAT-001\n",
    );
    let plan = rename_plan(
        &root,
        ".mochiko/features/FEAT-001-old.md",
        "FEAT-001",
        "family-account-sharing",
        &[],
    );
    apply(&root, &plan);
    assert_eq!(
        read(&root, "notes.md"),
        "see .mochiko/features/FEAT-001/tasks.md for FEAT-001-family-account-sharing\n",
        "D12"
    );
}

#[test]
fn a_lower_case_screen_file_moves_with_its_case_and_its_link_follows() {
    let root = tree("scr");
    let spec = ".mochiko/specs/lunch-orders/spec.md";
    put(
        &root,
        spec,
        "| SCR-001 | Menu |\n[menu](prototype/scr-001-week-menu.html)\n",
    );
    put(
        &root,
        ".mochiko/specs/lunch-orders/prototype/scr-001-week-menu.html",
        "<html></html>\n",
    );
    let plan = rename_plan(&root, spec, "SCR-001", "weekly-menu-grid", &[]);
    apply(&root, &plan);
    assert!(root
        .join(".mochiko/specs/lunch-orders/prototype/scr-001-weekly-menu-grid.html")
        .exists());
    assert_eq!(
        read(&root, spec),
        "| SCR-001-weekly-menu-grid | Menu |\n[menu](prototype/scr-001-weekly-menu-grid.html)\n"
    );
}

#[test]
fn a_move_lists_every_spot_its_scope_leaves_naming_the_old_file() {
    let root = tree("stale");
    let spec = ".mochiko/specs/lunch-orders/spec.md";
    put(
        &root,
        spec,
        "| SCR-001 | Menu |\n[menu](prototype/scr-001-week-menu.html)\n",
    );
    put(
        &root,
        ".mochiko/specs/lunch-orders/prototype/scr-001-week-menu.html",
        "<html></html>\n",
    );
    put(
        &root,
        ".mochiko/specs/other-spec/spec.md",
        "| SCR-001 | Cart |\n[menu](../lunch-orders/prototype/scr-001-week-menu.html)\n",
    );
    put(
        &root,
        "notes.md",
        "```\nprototype/scr-001-week-menu.html\n```\n",
    );
    let plan = rename_plan(&root, spec, "SCR-001", "weekly-menu-grid", &[]);
    let spots: Vec<String> = plan.stale.iter().map(ToString::to_string).collect();
    assert_eq!(
        spots,
        vec![
            ".mochiko/specs/other-spec/spec.md:2:34 · scr-001-week-menu.html".to_string(),
            "notes.md:2:11 · scr-001-week-menu.html".to_string(),
        ],
        "N5 as ruled: a cross-owner link and a quote the rewrite leaves"
    );
    assert!(
        rename::preview(&plan).contains(
            "stale link .mochiko/specs/other-spec/spec.md:2:34 · scr-001-week-menu.html\n"
        ),
        "the preview lists them too"
    );
}

// ---------------------------------------------------------------------------
// T8 — the D15 diff check
// ---------------------------------------------------------------------------

fn gi_target(slug: &str, aliases: &[&str], owner: Option<&str>) -> rename::Target {
    rename::Target::Rename {
        family: fam("GI"),
        number: "001".into(),
        slug: slug.into(),
        aliases: aliases.iter().map(|a| a.to_string()).collect(),
        owner: owner.map(str::to_string),
        slugs: Vec::new(),
    }
}

#[test]
fn a_token_only_change_passes_and_counts_its_spans() {
    let target = gi_target("repo-secret-hygiene", &[], None);
    let old = "GI-001 and GI-001-old-slug-here and GI-002";
    let new = "GI-001-repo-secret-hygiene and GI-001-repo-secret-hygiene and GI-002";
    assert_eq!(rename::diff_check(old, new, &target), Ok(2));
}

#[test]
fn any_change_outside_a_target_token_blocks() {
    let target = gi_target("repo-secret-hygiene", &[], None);
    let old = "GI-001 is the rule";
    for new in [
        "GI-001-repo-secret-hygiene is a rule",
        "GI-001-repo-secret-hygiene is the rule.",
        "GI-002-repo-secret-hygiene is the rule",
        "GI-001-repo-secret-hygienes is the rule",
        "\"GI-001-repo-secret-hygiene\" is the rule",
    ] {
        assert!(
            rename::diff_check(old, new, &target).is_err(),
            "{new:?} must block"
        );
    }
}

#[test]
fn a_rewrite_inside_a_masked_span_blocks() {
    let target = gi_target("repo-secret-hygiene", &[], None);
    let old = "> GI-001 quoted";
    let new = "> GI-001-repo-secret-hygiene quoted";
    assert!(
        rename::diff_check(old, new, &target).is_err(),
        "D15: a quote keeps its source's form"
    );
}

#[test]
fn an_alias_rewrite_carries_its_qualifier_and_a_stray_qualifier_blocks() {
    let target = rename::Target::Rename {
        family: fam("session D"),
        number: "1".into(),
        slug: "one-production-floor".into(),
        aliases: vec!["PO-D".into()],
        owner: Some("production-only-focus".into()),
        slugs: Vec::new(),
    };
    let old = "PO-D1 rules and D1 too";
    let new = "`production-only-focus` D1-one-production-floor rules and D1 too";
    assert_eq!(rename::diff_check(old, new, &target), Ok(1));
    let stray = "`production-only-focus` D1-one-production-floor rules and `x-y` D1 too";
    assert!(rename::diff_check(old, stray, &target).is_err());
}

// ---------------------------------------------------------------------------
// T9 — `ids rekey` (D4 as changed at review, D7 V3)
// ---------------------------------------------------------------------------

#[test]
fn a_rekey_moves_the_number_and_keeps_each_mention_form_and_slug() {
    let root = tree("rekey");
    let file = ".mochiko/specs/lunch-orders/constraints-and-decisions.md";
    put(
        &root,
        file,
        "### C-003-budget-ceiling-rule — x\nsee C-003 and C-003-budget-ceiling-rule\n",
    );
    put(
        &root,
        "notes.md",
        "`lunch-orders` C-003-budget-ceiling-rule\n",
    );
    let plan =
        rename::plan_rekey(&root, Path::new(file), "C-003", "C-005", &[]).expect("rekey plans");
    apply(&root, &plan);
    assert_eq!(
        read(&root, file),
        "### C-005-budget-ceiling-rule — x\nsee C-005 and C-005-budget-ceiling-rule\n"
    );
    assert_eq!(
        read(&root, "notes.md"),
        "`lunch-orders` C-005-budget-ceiling-rule\n"
    );
}

#[test]
fn a_rekey_onto_an_existing_definition_is_refused() {
    let root = tree("rekey-taken");
    let file = ".mochiko/product/constraints-and-decisions.md";
    put(&root, file, "### C-003 — x\n### C-005 — y\n");
    let reason = rename::plan_rekey(&root, Path::new(file), "C-003", "C-005", &[]).err();
    assert!(reason.is_some_and(|r| r.0.contains("already defined")));
}

#[test]
fn a_rekey_that_would_orphan_a_number_only_path_is_refused() {
    let root = tree("rekey-path");
    put(&root, ".mochiko/features/FEAT-001.md", "# FEAT-001 — x\n");
    put(
        &root,
        "notes.md",
        "see .mochiko/features/FEAT-001/tasks.md\n",
    );
    let reason = rename::plan_rekey(
        &root,
        Path::new(".mochiko/features/FEAT-001.md"),
        "FEAT-001",
        "FEAT-002",
        &[],
    )
    .err();
    assert!(
        reason.is_some_and(|r| r.0.contains("number-only path")),
        "R2 guard"
    );
}

#[test]
fn a_rekey_across_families_is_refused() {
    let root = tree("rekey-family");
    let file = ".mochiko/product/constraints-and-decisions.md";
    put(&root, file, "### C-003 — x\n");
    let reason = rename::plan_rekey(&root, Path::new(file), "C-003", "D-003", &[]).err();
    assert!(reason.is_some_and(|r| r.0.contains("different families")));
}

#[test]
fn a_rekey_never_rewrites_a_local_label() {
    let root = tree("n2-rekey");
    put(&root, INTENT, "- **GI-005-repo-type-shelf — x**\n");
    put(&root, "notes.md", "GI-005-class and GI-005\n");
    let plan =
        rename::plan_rekey(&root, Path::new(INTENT), "GI-005", "GI-006", &[]).expect("rekey plans");
    apply(&root, &plan);
    assert_eq!(read(&root, "notes.md"), "GI-005-class and GI-006\n", "N2");
}

#[test]
fn a_rekey_move_lists_the_spots_naming_the_old_file_too() {
    let root = tree("stale-rekey");
    put(
        &root,
        ".mochiko/features/FEAT-003-wallet.md",
        "# FEAT-003 — Wallet\n",
    );
    put(
        &root,
        "notes.md",
        "> see .mochiko/features/FEAT-003-wallet.md\n",
    );
    let plan = rename::plan_rekey(
        &root,
        Path::new(".mochiko/features/FEAT-003-wallet.md"),
        "FEAT-003",
        "FEAT-004",
        &[],
    )
    .expect("rekey plans");
    let spots: Vec<String> = plan.stale.iter().map(ToString::to_string).collect();
    assert_eq!(
        spots,
        vec!["notes.md:1:25 · FEAT-003-wallet.md".to_string()]
    );
}

// ---------------------------------------------------------------------------
// T10 — `ids literal` (R6: B2's other repo-only forms)
// ---------------------------------------------------------------------------

fn literal_plan(
    root: &Path,
    token: &str,
    slug: &str,
    paths: &[&str],
) -> Result<rename::Plan, rename::Refused> {
    let paths: Vec<PathBuf> = paths.iter().map(PathBuf::from).collect();
    rename::plan_literal(root, token, slug, &paths, &[])
}

#[test]
fn a_literal_is_refused_for_a_table_family_a_malformed_token_or_a_bad_slug() {
    let root = tree("lit-refuse");
    put(&root, "a.md", "AM-5\n");
    let reason = |token: &str, slug: &str, paths: &[&str]| {
        literal_plan(&root, token, slug, paths)
            .err()
            .map(|r| r.0)
            .unwrap_or_default()
    };
    assert!(reason("GI-004", "a-b-c", &["a.md"]).contains("use `ids rename`"));
    assert!(reason("am-5", "a-b-c", &["a.md"]).contains("not a repo-only token"));
    assert!(reason("AM5", "a-b-c", &["a.md"]).contains("not a repo-only token"));
    assert!(reason("AM-5", "a-b", &["a.md"]).contains("is not a slug"));
    assert!(reason("AM-5", "a-b-c", &[]).contains("never rewrites project-wide"));
}

#[test]
fn a_literal_rewrites_inside_the_given_paths_only() {
    let root = tree("lit-scope");
    put(&root, "docs/a.md", "AM-5 and AM-50 and XAM-5\n");
    put(&root, "other.md", "AM-5\n");
    let plan = literal_plan(&root, "AM-5", "hook-ship-exception", &["docs"]).expect("plans");
    apply(&root, &plan);
    assert_eq!(
        read(&root, "docs/a.md"),
        "AM-5-hook-ship-exception and AM-50 and XAM-5\n"
    );
    assert_eq!(read(&root, "other.md"), "AM-5\n");
}

#[test]
fn a_literal_reslugs_a_joined_token() {
    let root = tree("lit-reslug");
    put(&root, "a.md", "AM-5-old-slug-words\n");
    apply(
        &root,
        &literal_plan(&root, "AM-5", "hook-ship-exception", &["a.md"]).expect("plans"),
    );
    assert_eq!(read(&root, "a.md"), "AM-5-hook-ship-exception\n");
}

#[test]
fn a_literal_skips_long_lists_ranges_and_quotes() {
    let root = tree("lit-allow");
    let body = "AM-1, AM-2, AM-3, AM-5\nAM-1–AM-5\n\"AM-5\"\n";
    put(&root, "a.md", body);
    let plan = literal_plan(&root, "AM-5", "hook-ship-exception", &["a.md"]).expect("plans");
    assert!(plan.edits.is_empty(), "{:?}", plan.edits);
}

#[test]
fn an_ellipsis_range_stays_bare_through_alias_and_literal_rewrites() {
    let root = tree("ellipsis-rw");
    let record = ".mochiko/brainstorms/production-only-focus/record.md";
    put(&root, record, "### D1 — Floor\n");
    put(
        &root,
        "CLAUDE.md",
        "PO-D1…D7 and PO-D1...D7 stay; AM-1…AM-5 and AM-5...AM-9 stay\n",
    );
    let plan = rename_plan(&root, record, "D1", "one-production-floor", &["PO-D"]);
    assert!(
        plan.edits.iter().all(|e| e.path != Path::new("CLAUDE.md")),
        "N1: {:?}",
        plan.edits
    );
    let plan = literal_plan(&root, "AM-5", "hook-ship-exception", &["CLAUDE.md"]).expect("plans");
    assert!(plan.edits.is_empty(), "N1: {:?}", plan.edits);
}

#[test]
fn a_literal_write_passes_its_diff_check_and_check_never_flags_the_form() {
    let root = tree("lit-check");
    put(&root, "a.md", "per AM-5 and OQ-2\n");
    let (files, spans) = apply(
        &root,
        &literal_plan(&root, "AM-5", "hook-ship-exception", &["a.md"]).expect("plans"),
    );
    assert_eq!((files, spans), (1, 1));
    assert_eq!(findings(&root, &[], &[]), vec![]);
}

#[test]
fn a_literal_diff_check_blocks_any_other_change() {
    let target = rename::Target::Literal {
        token: "AM-5".into(),
        slug: "hook-ship-exception".into(),
    };
    assert_eq!(
        rename::diff_check("AM-5 x", "AM-5-hook-ship-exception x", &target),
        Ok(1)
    );
    assert!(rename::diff_check("AM-5 x", "AM-5-hook-ship-exception y", &target).is_err());
}

// ---------------------------------------------------------------------------
// T11 — the command surface (exit codes: 0 ok · 1 findings or a blocked write · 2 usage)
// ---------------------------------------------------------------------------

/// Run `mochiko-cli <args>` in-process; `(exit code, stdout, stderr)`.
fn run(args: &[&str]) -> (i32, String, String) {
    let args: Vec<String> = args.iter().map(|a| a.to_string()).collect();
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let code = mochiko_cli::cli::dispatch_io(&args, &mut std::io::empty(), &mut out, &mut err);
    (
        code,
        String::from_utf8_lossy(&out).into_owned(),
        String::from_utf8_lossy(&err).into_owned(),
    )
}

fn arg(root: &Path, rel: &str) -> String {
    root.join(rel).to_string_lossy().into_owned()
}

#[test]
fn check_exits_one_with_its_findings_and_zero_when_clean() {
    let root = gi_tree("cli-check");
    put(&root, "CLAUDE.md", "per GI-001\n");
    put(&root, "scratch/a.md", "GAP-001\n");
    let (code, out, _) = run(&["ids", "--check", &arg(&root, "CLAUDE.md")]);
    assert_eq!(code, 1, "{out}");
    assert_eq!(
        out,
        "CLAUDE.md:1:5 · bare · GI-001\nmochiko-cli ids --check · 1 bare · 0 drift\n"
    );
    put(&root, "CLAUDE.md", "per GI-001-repo-secret-hygiene\n");
    let (code, out, _) = run(&["ids", "--check", &arg(&root, "CLAUDE.md")]);
    assert_eq!(
        (code, out.as_str()),
        (0, "mochiko-cli ids --check · 0 bare · 0 drift\n")
    );
    let (code, _, _) = run(&["ids", "--check", &arg(&root, "."), "--exclude", "scratch/"]);
    assert_eq!(code, 1, "GI-002's bare definition is still reported");
}

#[test]
fn rename_previews_by_default_and_writes_with_write() {
    let root = gi_tree("cli-rename");
    put(&root, "CLAUDE.md", "per GI-002\n");
    let owning = arg(&root, INTENT);
    let (code, out, err) = run(&["ids", "rename", &owning, "GI-002", "repo-type-shelf"]);
    assert_eq!(code, 0, "{err}");
    assert!(
        out.contains("CLAUDE.md:1\n-per GI-002\n+per GI-002-repo-type-shelf\n"),
        "{out}"
    );
    assert!(
        out.ends_with("nothing written; --write applies it\n"),
        "{out}"
    );
    assert_eq!(read(&root, "CLAUDE.md"), "per GI-002\n");
    let (code, out, err) = run(&[
        "ids",
        "rename",
        &owning,
        "GI-002",
        "repo-type-shelf",
        "--write",
    ]);
    assert_eq!(code, 0, "{err}");
    assert!(
        out.contains("diff check: 2 files · 2 spans · ID tokens only\n"),
        "{out}"
    );
    assert_eq!(read(&root, "CLAUDE.md"), "per GI-002-repo-type-shelf\n");
}

#[test]
fn rekey_and_literal_run_through_the_command_surface() {
    let root = tree("cli-rekey");
    let file = ".mochiko/product/constraints-and-decisions.md";
    put(&root, file, "### C-003 — x\n");
    put(&root, "notes.md", "AM-5\n");
    let (code, _, err) = run(&[
        "ids",
        "rekey",
        &arg(&root, file),
        "C-003",
        "C-004",
        "--write",
    ]);
    assert_eq!(code, 0, "{err}");
    assert_eq!(read(&root, file), "### C-004 — x\n");
    let (code, _, err) = run(&[
        "ids",
        "literal",
        "AM-5",
        "hook-ship-exception",
        &arg(&root, "notes.md"),
        "--write",
    ]);
    assert_eq!(code, 0, "{err}");
    assert_eq!(read(&root, "notes.md"), "AM-5-hook-ship-exception\n");
}

#[test]
fn usage_errors_exit_two() {
    let root = gi_tree("cli-usage");
    put(&root, "notes.md", "GAP-001\n");
    for args in [
        vec!["ids".to_string()],
        vec!["ids".into(), arg(&root, "notes.md")],
        vec![
            "ids".into(),
            "rename".into(),
            arg(&root, "notes.md"),
            "GAP-001".into(),
            "a-b-c".into(),
        ],
        vec![
            "ids".into(),
            "literal".into(),
            "AM-5".into(),
            "a-b-c".into(),
        ],
        vec!["ids".into(), "--check".into(), "/".into()],
    ] {
        let args: Vec<&str> = args.iter().map(String::as_str).collect();
        let (code, _, err) = run(&args);
        assert_eq!(code, 2, "{args:?}: {err}");
    }
}

#[test]
fn a_write_that_would_leave_a_stale_link_exits_two_and_writes_nothing() {
    let root = tree("cli-stale");
    let file = ".mochiko/features/FEAT-003-wallet.md";
    put(&root, file, "# FEAT-003 — Wallet\n");
    put(
        &root,
        "notes.md",
        "> see .mochiko/features/FEAT-003-wallet.md\n",
    );
    let owning = arg(&root, file);
    let rename = [
        "ids",
        "rename",
        &owning,
        "FEAT-003",
        "digital-wallet-accounts",
    ];
    let (code, out, err) = run(&rename);
    assert_eq!(code, 0, "a preview: {err}");
    assert!(
        out.contains("stale link notes.md:1:25 · FEAT-003-wallet.md\n"),
        "{out}"
    );
    let (code, _, err) = run(&[&rename[..], &["--write"]].concat());
    assert_eq!(code, 2, "N5 as ruled: {err}");
    assert!(err.contains("notes.md:1:25 · FEAT-003-wallet.md"), "{err}");
    assert!(
        err.contains("--exclude <file>"),
        "L5: the way out for a verbatim quote: {err}"
    );
    assert_eq!(
        read(&root, file),
        "# FEAT-003 — Wallet\n",
        "nothing written"
    );
    assert!(!root
        .join(".mochiko/features/FEAT-003-digital-wallet-accounts.md")
        .exists());
}

#[test]
fn an_alias_in_its_own_code_span_takes_the_span_and_one_inside_a_longer_span_is_left() {
    let root = tree("alias-code");
    let record = ".mochiko/brainstorms/production-only-focus/record.md";
    put(&root, record, "### D1 — Floor\n");
    put(
        &root,
        "CLAUDE.md",
        "forms (`PO-D1` here) and `PO-D1 or PO-D2` stay\n",
    );
    let plan = rename_plan(&root, record, "D1", "one-production-floor", &["PO-D"]);
    apply(&root, &plan);
    assert_eq!(
        read(&root, "CLAUDE.md"),
        "forms (`production-only-focus` D1-one-production-floor here) and `PO-D1 or PO-D2` stay\n",
        "never a code span nested in a code span"
    );
}

#[test]
fn a_hyphen_that_starts_no_slug_word_makes_the_token_a_label() {
    for text in [
        "D1-<slug>",
        "GI-019-{slug}",
        "FR-001-[name]",
        "D4-",
        "GI-004-two-_x",
    ] {
        let found = tokens(text);
        assert_eq!(found.len(), 1, "{text}: {found:?}");
        assert_eq!(
            found[0].3,
            Some(ids::Skip::Label),
            "{text}: a placeholder tail (D19)"
        );
    }
}

// ---------------------------------------------------------------------------
// T12 — the code review fix round (C1–C3, H1–H5, M1–M4, L1, L3–L5, rulings 2, 3, 5)
// ---------------------------------------------------------------------------

#[test]
fn two_moves_onto_one_name_are_refused_and_nothing_is_lost() {
    // C1, the reviewer's repro: two screen files of one ID.
    let root = tree("c1");
    let spec = ".mochiko/specs/lunch-orders/spec.md";
    put(&root, spec, "| SCR-001 | Menu |\n");
    let desktop = ".mochiko/specs/lunch-orders/prototype/scr-001-week-menu.html";
    let mobile = ".mochiko/specs/lunch-orders/prototype/scr-001-week-menu-mobile.html";
    put(&root, desktop, "DESKTOP\n");
    put(&root, mobile, "MOBILE\n");
    let owning = arg(&root, spec);
    let (code, _, err) = run(&[
        "ids",
        "rename",
        &owning,
        "SCR-001",
        "weekly-menu-grid",
        "--write",
    ]);
    assert_eq!(code, 2, "{err}");
    assert_eq!(read(&root, desktop), "DESKTOP\n");
    assert_eq!(read(&root, mobile), "MOBILE\n");
    // Two entry files of one FEAT would both move to the new name.
    let root = tree("c1-feat");
    put(
        &root,
        ".mochiko/features/FEAT-003-cards.md",
        "# FEAT-003 — Cards\n",
    );
    put(
        &root,
        ".mochiko/features/FEAT-003-wallet.md",
        "# FEAT-003 — Wallet\n",
    );
    let reason = rename::plan_rename(
        &root,
        Path::new(".mochiko/features/FEAT-003-cards.md"),
        "FEAT-003",
        "digital-wallet-accounts",
        &[],
        &[],
    )
    .err()
    .map(|r| r.0)
    .unwrap_or_default();
    // Two holders of one number: A7 refuses the rename before any move is planned.
    assert!(
        reason.contains("is held by")
            && reason.contains("FEAT-003-cards")
            && reason.contains("FEAT-003-wallet"),
        "C1: {reason}"
    );
}

#[test]
fn a_move_target_that_appears_after_planning_blocks_the_whole_write() {
    let root = tree("c1-late");
    let file = ".mochiko/features/FEAT-003-wallet.md";
    put(&root, file, "# FEAT-003 — Wallet\n");
    let plan = rename_plan(&root, file, "FEAT-003", "digital-wallet-accounts", &[]);
    let to = ".mochiko/features/FEAT-003-digital-wallet-accounts.md";
    put(&root, to, "LATE\n");
    assert!(
        rename::write(&root, &plan).is_err(),
        "C1: the move would overwrite"
    );
    assert_eq!(read(&root, to), "LATE\n");
    assert_eq!(
        read(&root, file),
        "# FEAT-003 — Wallet\n",
        "nothing written"
    );
}

#[test]
fn a_qualifier_never_names_a_line_for_its_bare_decisions() {
    let root = tree("c2");
    put(
        &root,
        ".mochiko/brainstorms/hook-field-review/record.md",
        "### D2 — x\n",
    );
    put(
        &root,
        ".mochiko/brainstorms/delta-files/record.md",
        "### D2 — y\n### D3 — z\n",
    );
    let got = resolutions(
        &root,
        ".mochiko/brainstorms/index.md",
        "- D2 and D10 landed; superseded by `delta-files` D3\n- `delta-files` D3 rules; D2 too\n",
    );
    assert_eq!(
        got,
        vec![
            ("D2".into(), Resolved::Unresolved),
            ("D10".into(), Resolved::Unresolved),
            ("D3".into(), Resolved::Owned(name("delta-files"))),
            ("D3".into(), Resolved::Owned(name("delta-files"))),
            ("D2".into(), Resolved::Unresolved),
        ],
        "C2, B1 as narrowed (user ruling C): leading or mid-line, a qualifier owns no line"
    );
}

#[test]
fn a_qualifier_at_a_soft_line_break_still_qualifies_its_compound() {
    let root = tree("c3");
    put(
        &root,
        ".mochiko/brainstorms/product-architecture-schema/record.md",
        "### D3 — a\n### D10 — b\n",
    );
    let record = ".mochiko/brainstorms/delta-files/record.md";
    put(&root, record, "### D3 — c\n");
    let got = resolutions(
        &root,
        record,
        "as ruled by `product-architecture-schema`\n  D3/D10 hold\n\n`product-architecture-schema`\n\nD3 is ours\n",
    );
    assert_eq!(
        got,
        vec![
            (
                "D3".into(),
                Resolved::Owned(name("product-architecture-schema"))
            ),
            (
                "D10".into(),
                Resolved::Owned(name("product-architecture-schema"))
            ),
            ("D3".into(), Resolved::Owned(name("delta-files"))),
        ],
        "C3: a soft break is the one space; a blank line ends the paragraph"
    );
    let scanned = ids::scan("per `foo-bar` D1,\nD2 holds");
    assert_eq!(
        scanned[1].qualifier.as_ref().map(|q| q.text.as_str()),
        Some("foo-bar"),
        "C3: a compound joins across a soft break"
    );
}

#[cfg(unix)]
#[test]
fn a_rewrite_keeps_the_file_mode() {
    use std::os::unix::fs::PermissionsExt;
    let root = gi_tree("h1");
    let script = "hooks/scripts/halt.sh";
    put(&root, script, "#!/bin/sh\n# per GI-001\n");
    let path = root.join(script);
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).expect("chmod");
    apply(
        &root,
        &rename_plan(&root, INTENT, "GI-001", "secret-hygiene-rules", &[]),
    );
    assert_eq!(
        read(&root, script),
        "#!/bin/sh\n# per GI-001-secret-hygiene-rules\n"
    );
    let mode = std::fs::metadata(&path)
        .expect("metadata")
        .permissions()
        .mode()
        & 0o777;
    assert_eq!(mode, 0o755, "H1");
}

#[test]
fn a_prose_line_that_starts_with_an_id_defines_nothing() {
    let root = tree("h2");
    let record = ".mochiko/brainstorms/foo-bar/record.md";
    put(
        &root,
        record,
        "Some prose that wraps and then\nD10-governance-envelope-supersessions`) x\n- D1's asymmetry holds\n### D1 — a\n### D10 — b\n",
    );
    put(
        &root,
        INTENT,
        "GI-005 is waived here\n- GI-006 plain list\n- **GI-005 — x**\n| GI-006 | y |\n",
    );
    let index = index(&root);
    let at = |rel: &str, def: Option<&ids::Definition>| -> String {
        let def = def.expect("a definition");
        let text = read(&root, rel);
        let start = def.start.expect("a line definition");
        text[start..].lines().next().unwrap_or_default().to_string()
    };
    let owner = name("foo-bar");
    assert_eq!(
        at(
            record,
            index.definition(fam("session D"), Some(&owner), "1")
        ),
        "D1 — a"
    );
    assert_eq!(
        at(
            record,
            index.definition(fam("session D"), Some(&owner), "10")
        ),
        "D10 — b",
        "H2: a wrapped prose line is no definition"
    );
    assert_eq!(
        at(INTENT, index.definition(fam("GI"), None, "005")),
        "GI-005 — x**"
    );
    assert_eq!(
        at(INTENT, index.definition(fam("GI"), None, "006")),
        "GI-006 | y |"
    );
}

#[test]
fn a_path_token_is_rewritten_only_when_it_names_a_planned_move() {
    let root = tree("h3");
    let file = ".mochiko/features/FEAT-003-wallet.md";
    put(&root, file, "# FEAT-003 — Wallet\n");
    put(
        &root,
        "notes.md",
        "[w](.mochiko/features/FEAT-003-wallet.md) and [t](.mochiko/features/FEAT-003-wallet/tasks.md)\n",
    );
    put(&root, "docs/a.md", "[w](../elsewhere/FEAT-003-wallet.md)\n");
    let plan = rename_plan(&root, file, "FEAT-003", "digital-wallet-accounts", &[]);
    let notes = plan
        .edits
        .iter()
        .find(|e| e.path == Path::new("notes.md"))
        .expect("a notes edit");
    assert_eq!(
        notes.new,
        "[w](.mochiko/features/FEAT-003-digital-wallet-accounts.md) and [t](.mochiko/features/FEAT-003-wallet/tasks.md)\n",
        "H3: the unmoved directory keeps its name"
    );
    assert!(plan.edits.iter().all(|e| e.path != Path::new("docs/a.md")));
    let spots: Vec<String> = plan.stale.iter().map(ToString::to_string).collect();
    assert_eq!(
        spots,
        vec![
            "docs/a.md:1:18 · FEAT-003-wallet.md".to_string(),
            "notes.md:1:69 · FEAT-003-wallet".to_string(),
        ]
    );
}

#[test]
fn a_screen_files_slug_is_its_bare_lines_slug_and_a_rename_keeps_it() {
    let root = tree("h4");
    let spec = ".mochiko/specs/lunch-orders/spec.md";
    put(
        &root,
        spec,
        "| SCR-001 | Menu |\n[menu](prototype/scr-001-week-menu.html)\n",
    );
    put(
        &root,
        ".mochiko/specs/lunch-orders/prototype/scr-001-week-menu.html",
        "x\n",
    );
    put(&root, "notes.md", "`lunch-orders` SCR-001\n");
    assert_eq!(
        findings(&root, &[], &[]),
        vec![("notes.md".into(), 1, ids::Kind::Bare, "SCR-001".into())],
        "H4: S5 keeps the screen file's slug"
    );
    let plan = rename_plan(&root, spec, "SCR-001", "week-menu", &[]);
    assert!(plan.moves.is_empty());
    apply(&root, &plan);
    assert_eq!(
        read(&root, spec),
        "| SCR-001-week-menu | Menu |\n[menu](prototype/scr-001-week-menu.html)\n"
    );
    assert_eq!(
        read(&root, "notes.md"),
        "`lunch-orders` SCR-001-week-menu\n"
    );
    put(
        &root,
        ".mochiko/specs/lunch-orders/prototype/scr-001-week-menu-mobile.html",
        "y\n",
    );
    put(&root, spec, "| SCR-001 | Menu |\n");
    let reason = rename::plan_rename(
        &root,
        Path::new(spec),
        "SCR-001",
        "weekly-menu-grid",
        &[],
        &[],
    )
    .err()
    .map(|r| r.0)
    .unwrap_or_default();
    assert!(reason.contains("disagree"), "H4: {reason}");
}

#[test]
fn an_alias_that_is_empty_or_scans_as_an_id_is_refused() {
    let root = tree("h5");
    let record = ".mochiko/brainstorms/foo-bar/record.md";
    put(&root, record, "### D1 — x\n");
    for alias in ["", "  ", "D", " D"] {
        let reason = rename::plan_rename(
            &root,
            Path::new(record),
            "D1",
            "foo-one-slug",
            &[alias.to_string()],
            &[],
        )
        .err()
        .map(|r| r.0)
        .unwrap_or_default();
        assert!(reason.contains("alias"), "H5 {alias:?}: {reason}");
    }
}

#[test]
fn the_diff_check_blocks_a_rewritten_local_label() {
    let rename = rename::Target::Rename {
        family: fam("GI"),
        number: "005".into(),
        slug: "repo-type-shelf".into(),
        aliases: vec![],
        owner: None,
        slugs: vec![],
    };
    assert!(
        rename::diff_check(
            "GI-005-class rules",
            "GI-005-repo-type-shelf rules",
            &rename
        )
        .is_err(),
        "M1"
    );
    assert_eq!(
        rename::diff_check("GI-005 rules", "GI-005-repo-type-shelf rules", &rename),
        Ok(1)
    );
    let kept = rename::Target::Rename {
        family: fam("FEAT"),
        number: "003".into(),
        slug: "digital-wallet-accounts".into(),
        aliases: vec![],
        owner: None,
        slugs: vec!["wallet".into()],
    };
    assert_eq!(
        rename::diff_check(
            "FEAT-003-wallet x",
            "FEAT-003-digital-wallet-accounts x",
            &kept
        ),
        Ok(1),
        "a held file-name slug is the ID's"
    );
    let rekey = rename::Target::Rekey {
        family: fam("GI"),
        from: "005".into(),
        to: "006".into(),
        slugs: vec![],
        shared: false,
    };
    assert!(
        rename::diff_check("GI-005-class", "GI-006-class", &rekey).is_err(),
        "M1, rekey"
    );
}

#[test]
fn a_write_that_fails_partway_says_what_it_wrote() {
    let root = gi_tree("m2");
    put(&root, "zz/b.md", "GI-001\n");
    std::fs::create_dir_all(root.join("zz/.b.md.ids-tmp")).expect("a directory in the way");
    let (code, _, err) = run(&[
        "ids",
        "rename",
        &arg(&root, INTENT),
        "GI-001",
        "secret-hygiene-rules",
        "--write",
    ]);
    assert_eq!(code, 1, "{err}");
    assert!(
        err.contains("partially written: .mochiko/memory/governance-intent.md"),
        "M2: {err}"
    );
    assert!(err.contains("zz/b.md"), "{err}");
}

#[test]
fn a_literal_never_reslugs_a_run_that_is_no_slug() {
    let root = tree("m3");
    put(&root, "a.md", "No J-3-class defect; J-3 stands\n");
    let plan = literal_plan(&root, "J-3", "census-small-family", &["a.md"]).expect("plans");
    apply(&root, &plan);
    assert_eq!(
        read(&root, "a.md"),
        "No J-3-class defect; J-3-census-small-family stands\n",
        "M3"
    );
}

#[test]
fn an_alias_heading_a_list_of_four_stays_bare_and_the_write_goes_through() {
    let root = tree("m4");
    let record = ".mochiko/brainstorms/audit-design/record.md";
    put(&root, record, "### D1 — x\n");
    put(&root, "CLAUDE.md", "AD-D1/D2/D5/D8 hold; AD-D1 rules\n");
    let plan = rename_plan(&root, record, "D1", "audit-first-pass", &["AD-D"]);
    apply(&root, &plan);
    assert_eq!(
        read(&root, "CLAUDE.md"),
        "AD-D1/D2/D5/D8 hold; `audit-design` D1-audit-first-pass rules\n",
        "M4"
    );
}

#[test]
fn the_tree_root_as_the_owning_file_exits_two() {
    let root = gi_tree("l1");
    let tree_arg = root.to_string_lossy().into_owned();
    for args in [
        ["ids", "rename", tree_arg.as_str(), "GI-001", "a-b-c"],
        ["ids", "rekey", tree_arg.as_str(), "GI-001", "GI-003"],
    ] {
        let (code, _, err) = run(&args);
        assert_eq!(code, 2, "L1 {args:?}: {err}");
    }
}

#[cfg(unix)]
#[test]
fn a_symlink_is_never_followed() {
    let root = tree("l3");
    put(&root, INTENT, "- **GI-001-repo-secret-hygiene — x**\n");
    let outside = tree("l3-outside");
    put(&outside, "secret.md", "GI-001 sk-planted-secret\n");
    std::os::unix::fs::symlink(outside.join("secret.md"), root.join("link.md")).expect("link");
    std::os::unix::fs::symlink(&root, root.join("loop")).expect("cycle");
    assert_eq!(findings(&root, &[], &[]), vec![], "L3");
}

#[test]
fn a_slash_after_a_glued_alias_core_is_a_pair_separator() {
    let found = tokens("AD-D1/D2 and PO-D2/D3");
    let skips: Vec<_> = found.iter().map(|t| (t.0.as_str(), t.3)).collect();
    assert_eq!(skips, vec![("D2", None), ("D3", None)], "L4");
}

#[test]
fn an_alias_pair_normalizes_and_its_second_member_inherits_the_session() {
    let root = tree("l4");
    let record = ".mochiko/brainstorms/audit-design/record.md";
    put(&root, record, "### D1 — x\n### D2 — y\n");
    put(&root, "CLAUDE.md", "AD-D1/D2 hold\n");
    apply(
        &root,
        &rename_plan(&root, record, "D2", "second-audit-pass", &["AD-D"]),
    );
    assert_eq!(
        read(&root, "CLAUDE.md"),
        "AD-D1/D2-second-audit-pass hold\n",
        "L4: the member is the alias's session's"
    );
    apply(
        &root,
        &rename_plan(&root, record, "D1", "audit-first-pass", &["AD-D"]),
    );
    assert_eq!(
        read(&root, "CLAUDE.md"),
        "`audit-design` D1-audit-first-pass/D2-second-audit-pass hold\n"
    );
}

#[test]
fn the_write_itself_refuses_a_plan_with_stale_links() {
    let root = tree("stale-write");
    let file = ".mochiko/features/FEAT-003-wallet.md";
    put(&root, file, "# FEAT-003 — Wallet\n");
    put(
        &root,
        "notes.md",
        "> see .mochiko/features/FEAT-003-wallet.md\n",
    );
    let plan = rename_plan(&root, file, "FEAT-003", "digital-wallet-accounts", &[]);
    assert!(
        matches!(rename::write(&root, &plan), Err(rename::WriteError::Stale(spots)) if spots.len() == 1),
        "ruling 2: the one write path refuses"
    );
    assert_eq!(read(&root, file), "# FEAT-003 — Wallet\n");
}

#[test]
fn a_project_wide_file_slug_comes_from_the_familys_own_files_only() {
    let root = tree("r3-slugs");
    put(
        &root,
        ".mochiko/features/FEAT-003.md",
        "# FEAT-003 — Wallet\n",
    );
    put(&root, "sketches/FEAT-003-wallet.png", "x");
    put(&root, "notes.md", "FEAT-003-wallet ships\n");
    assert_eq!(
        findings(&root, &["notes.md"], &[]),
        vec![],
        "ruling 3: a stray file's name is no slug source"
    );
}

#[test]
fn env_files_are_never_read_or_printed() {
    let root = gi_tree("env");
    put(&root, ".env", "TOKEN=GI-001 sk-planted-secret\n");
    put(&root, "apps/web/.env.local", "GI-001 sk-planted-secret\n");
    let found = findings(&root, &[], &[]);
    assert!(found.iter().all(|f| !f.0.contains(".env")), "{found:?}");
    let plan = rename_plan(&root, INTENT, "GI-001", "secret-hygiene-rules", &[]);
    assert!(
        !rename::preview(&plan).contains("sk-planted-secret"),
        "ruling 5: never printed"
    );
}

#[test]
fn a_lane_tasks_file_defines_its_cycles_under_the_lane_run_key() {
    // Item 9 (fix-round addendum): every `tasks.md` defines cycles; a lane directory is already
    // named `lane-<slug>`, and its owner is that name.
    let root = tree("lane-cycles");
    put(
        &root,
        ".mochiko/product/lane-x-y/tasks.md",
        "### - [ ] C2-walking-skeleton-path: x\n",
    );
    put(
        &root,
        "notes.md",
        "`lane-x-y` C2\n`lane-x-y` C2-wrong-slug-here\n`no-such-lane` C2\n",
    );
    assert_eq!(
        findings(&root, &["notes.md"], &[]),
        vec![
            ("notes.md".into(), 1, ids::Kind::Bare, "C2".into()),
            (
                "notes.md".into(),
                2,
                ids::Kind::Drift,
                "C2-wrong-slug-here".into()
            ),
        ],
        "an unheld name stays unresolved (R9)"
    );
}

#[test]
fn an_epic_tasks_file_defines_its_cycles_under_the_epic_id() {
    let root = tree("epic-cycles");
    put(
        &root,
        ".mochiko/epics/EPIC-002/tasks.md",
        "### - [ ] C1-walking-skeleton-path: x\n",
    );
    put(&root, "notes.md", "`EPIC-002-checkout-flow-rework` C1\n");
    assert_eq!(
        findings(&root, &["notes.md"], &[]),
        vec![("notes.md".into(), 1, ids::Kind::Bare, "C1".into())],
        "item 9: R4's EPIC owner"
    );
}

// ---------------------------------------------------------------------------
// T13 — the second review fix round (B1 as narrowed, N1–N4, L1–L3)
// ---------------------------------------------------------------------------

#[test]
fn only_a_record_link_owns_a_lines_bare_decisions() {
    let root = tree("b1-narrow");
    for session in ["foo-bar", "other-session", "third-session", "other"] {
        put(
            &root,
            &format!(".mochiko/brainstorms/{session}/record.md"),
            "### D1 — x\n",
        );
    }
    let of = |rel: &str, line: &str, token: &str| -> Vec<Resolved> {
        resolutions(&root, rel, line)
            .into_iter()
            .filter(|(text, _)| text == token)
            .map(|(_, resolved)| resolved)
            .collect()
    };
    let index_md = ".mochiko/brainstorms/index.md";
    // (a) index.md:39's shape: a mid-line qualified pair, a later bare D2, no record link.
    let a = "- **Landed:** row · **superseded in part** by `other-session` D1/D6a — OQ1 answered; D2's budgets stand ([record](../decisions/x.md) R1)\n";
    assert_eq!(of(index_md, a, "D2"), vec![Resolved::Unresolved], "(a)");
    assert_eq!(
        of(index_md, a, "D6a"),
        vec![Resolved::Owned(name("other-session"))],
        "S4 inheritance is unchanged"
    );
    // (b) index.md:58's shape: two different mid-line qualifiers and a bare D6.
    let b = "- **Landed:** row · **superseded in part** by `other-session` D1 — D6's path dies · **and again** by `third-session` D1/D5 — D6's leg struck\n";
    assert_eq!(
        of(index_md, b, "D6"),
        vec![Resolved::Unresolved, Resolved::Unresolved],
        "(b)"
    );
    // (c) a DECISIONS.md row linking its own record and naming no other session.
    let c = "| 2026-09-24 | ruled (D1–D7): D2 and D4 stand | [record](.mochiko/brainstorms/foo-bar/record.md) |\n";
    assert_eq!(
        of("DECISIONS.md", c, "D2"),
        vec![Resolved::Owned(name("foo-bar"))],
        "(c)"
    );
    assert_eq!(
        of("DECISIONS.md", c, "D4"),
        vec![Resolved::Owned(name("foo-bar"))]
    );
    // (d) the same row naming another session mid-line.
    let d = "| 2026-09-24 | ruled (D1–D7): D2 and D4 stand; `other` D3 kept | [record](.mochiko/brainstorms/foo-bar/record.md) |\n";
    assert_eq!(
        of("DECISIONS.md", d, "D2"),
        vec![Resolved::Unresolved],
        "(d)"
    );
    assert_eq!(
        of("DECISIONS.md", d, "D3"),
        vec![Resolved::Owned(name("other"))],
        "a mention's own qualifier is unchanged"
    );
}

#[test]
fn cycles_are_defined_only_in_the_three_cycle_homes() {
    let root = tree("n1-homes");
    put(
        &root,
        "docs/x/tasks.md",
        "### - [ ] C1-alpha-beta-gamma: a\n",
    );
    put(
        &root,
        "notes/x/tasks.md",
        "### - [ ] C1-delta-epsilon-zeta: b\nsee C1-delta-epsilon-zeta\n",
    );
    put(&root, ".mochiko/strips/tasks.md", "### - [ ] Cycle 1: c\n");
    let index = index(&root);
    for owner in ["x", "strips"] {
        assert!(
            index
                .definition(fam("cycle C"), Some(&name(owner)), "1")
                .is_none(),
            "N1: `{owner}` is no cycle home"
        );
    }
    assert_eq!(
        findings(&root, &[], &[]),
        vec![],
        "two same-named directories outside the homes share nothing"
    );
}

#[test]
fn a_file_whose_name_carries_a_label_never_moves() {
    let root = tree("n2-move");
    let file = ".mochiko/features/FEAT-003-wallet.md";
    put(&root, file, "# FEAT-003 — Wallet\n");
    put(&root, "docs/FEAT-003-wallet-notes.md", "notes\n");
    let plan = rename_plan(&root, file, "FEAT-003", "digital-wallet-accounts", &[]);
    let moved: Vec<PathBuf> = plan.moves.iter().map(|(from, _)| from.clone()).collect();
    assert_eq!(
        moved,
        vec![PathBuf::from(file)],
        "N2: `wallet-notes` is a label"
    );
}

#[test]
fn an_alias_is_refused_when_its_form_ends_in_an_unqualified_id() {
    let root = tree("n3-alias");
    let record = ".mochiko/brainstorms/foo-bar/record.md";
    put(&root, record, "### D1 — x\n");
    put(&root, "CLAUDE.md", "x\n");
    let plan = |alias: &str| {
        rename::plan_rename(
            &root,
            Path::new(record),
            "D1",
            "foo-one-slug",
            &[alias.to_string()],
            &[],
        )
    };
    for prefix in [
        "PO", "CS", "AD", "ER", "TC", "AR", "OO", "AT", "SD", "OD", "PT", "UX",
    ] {
        let alias = format!("{prefix}-D");
        assert!(plan(&alias).is_ok(), "{alias} is a real prefix");
    }
    for alias in ["producer-plan D", "feature-map D", "adaptive-depth D"] {
        // Q2 (wave 1b): a shorthand name passes with the file it means this ID in, never bare.
        assert!(
            plan(&format!("{alias}=CLAUDE.md")).is_ok(),
            "{alias} is a shorthand name"
        );
        assert!(plan(alias).is_err(), "{alias} needs a file");
    }
    for alias in ["D", "(D", "the D"] {
        let reason = plan(alias).err().map(|r| r.0).unwrap_or_default();
        assert!(reason.contains("alias"), "N3 {alias:?}: {reason}");
    }
}

#[test]
fn an_html_href_is_a_link_not_a_quote_and_follows_its_screen() {
    let root = tree("n4-href");
    let spec = ".mochiko/specs/lunch-orders/spec.md";
    put(&root, spec, "| SCR-001 | Menu |\n");
    put(
        &root,
        ".mochiko/specs/lunch-orders/prototype/scr-001-week-menu.html",
        "x\n",
    );
    let index_html = ".mochiko/specs/lunch-orders/prototype/index.html";
    put(
        &root,
        index_html,
        "<a href=\"scr-001-week-menu.html\">Menu</a>\n",
    );
    let plan = rename_plan(&root, spec, "SCR-001", "weekly-menu-grid", &[]);
    assert!(plan.stale.is_empty(), "N4: {:?}", plan.stale);
    apply(&root, &plan);
    assert_eq!(
        read(&root, index_html),
        "<a href=\"scr-001-weekly-menu-grid.html\">Menu</a>\n"
    );
    assert!(root
        .join(".mochiko/specs/lunch-orders/prototype/scr-001-weekly-menu-grid.html")
        .exists());
}

#[test]
fn a_heading_card_beats_an_earlier_bold_line_for_a_session_decision() {
    let root = tree("l2-card");
    let record = ".mochiko/brainstorms/foo-bar/record.md";
    put(
        &root,
        record,
        "- **D1 frame — a summary line**\n### D1 — Frame\n",
    );
    let def = index(&root)
        .definition(fam("session D"), Some(&name("foo-bar")), "1")
        .cloned()
        .expect("D1");
    let text = read(&root, record);
    assert!(
        text[def.start.expect("a line definition")..].starts_with("D1 — Frame"),
        "L2: {:?}",
        def.start
    );
}

#[test]
fn two_moves_whose_targets_differ_only_in_case_are_refused() {
    let root = tree("l3-case");
    let file = ".mochiko/features/FEAT-003-cards-and-more.md";
    put(&root, file, "# FEAT-003 — Cards\n");
    put(
        &root,
        ".mochiko/features/feat-003-wallet-and-more.md",
        "x\n",
    );
    let reason = rename::plan_rename(
        &root,
        Path::new(file),
        "FEAT-003",
        "digital-wallet-accounts",
        &[],
        &[],
    )
    .err()
    .map(|r| r.0)
    .unwrap_or_default();
    assert!(reason.contains("both move to"), "L3: {reason}");
}

#[test]
fn a_quoted_string_in_code_is_kept_and_only_html_lifts_the_quote_mask() {
    // N4 as ruled (ii): a quoted string in code is an expectation, often the bare log verbatim.
    let root = tree("n4-code");
    let record = ".mochiko/brainstorms/foo-bar/record.md";
    put(&root, record, "### D3 — x\n");
    put(
        &root,
        "tests/a.rs",
        "// per foo-bar D3\nlet anchor = \"2026-09-24 foo-bar D3\";\n",
    );
    let plan = rename_plan(&root, record, "D3", "pinned-base-review", &[]);
    let edit = plan
        .edits
        .iter()
        .find(|e| e.path == Path::new("tests/a.rs"))
        .expect("the comment is a mention");
    assert_eq!(
        edit.new,
        "// per foo-bar D3-pinned-base-review\nlet anchor = \"2026-09-24 foo-bar D3\";\n"
    );
    for (rel, masked) in [
        ("a.md", true),
        ("a.rs", true),
        ("a.py", true),
        ("a.html", false),
        ("a.htm", false),
    ] {
        assert_eq!(ids::quotes_masked(Path::new(rel)), masked, "{rel}");
    }
}

// ---------------------------------------------------------------------------
// T14 — the third review fix round (F1, A1, G1)
// ---------------------------------------------------------------------------

/// Two specs whose prototypes each hold a `scr-001-week-menu.html`.
fn two_spec_tree(tag: &str) -> PathBuf {
    let root = tree(tag);
    for (spec, title) in [("lunch-orders", "Menu"), ("other-spec", "Cart")] {
        put(
            &root,
            &format!(".mochiko/specs/{spec}/spec.md"),
            &format!("| SCR-001 | {title} |\n"),
        );
        put(
            &root,
            &format!(".mochiko/specs/{spec}/prototype/scr-001-week-menu.html"),
            "x\n",
        );
    }
    root
}

#[test]
fn an_unqualified_cite_of_another_specs_same_named_screen_is_not_rewritten() {
    let root = two_spec_tree("f1");
    let cite = "The cart screen: `scr-001-week-menu.html`\n";
    put(&root, "FEATURES.md", cite);
    let spec = ".mochiko/specs/lunch-orders/spec.md";
    let plan = rename_plan(&root, spec, "SCR-001", "weekly-menu-grid", &[]);
    assert!(
        plan.edits
            .iter()
            .all(|e| e.path != Path::new("FEATURES.md")),
        "F1: a bare name at the root sits next to no moved file"
    );
    assert_eq!(
        plan.moves,
        vec![(
            PathBuf::from(".mochiko/specs/lunch-orders/prototype/scr-001-week-menu.html"),
            PathBuf::from(".mochiko/specs/lunch-orders/prototype/scr-001-weekly-menu-grid.html"),
        )]
    );
}

#[test]
fn a_link_to_a_same_named_file_in_another_directory_is_stale() {
    // Option B at round 5: no path resolution decides a drop; `--exclude` is the way out.
    let root = two_spec_tree("a1");
    put(
        &root,
        ".mochiko/specs/other-spec/spec.md",
        "| SCR-001 | Cart |\n[cart](prototype/scr-001-week-menu.html)\n",
    );
    put(
        &root,
        "docs/screens.md",
        "[cart](../.mochiko/specs/other-spec/prototype/scr-001-week-menu.html)\n",
    );
    let spec = ".mochiko/specs/lunch-orders/spec.md";
    let plan = rename_plan(&root, spec, "SCR-001", "weekly-menu-grid", &[]);
    let spots: Vec<String> = plan.stale.iter().map(ToString::to_string).collect();
    assert_eq!(
        spots,
        vec![
            ".mochiko/specs/other-spec/spec.md:2:18 · scr-001-week-menu.html".to_string(),
            "docs/screens.md:1:47 · scr-001-week-menu.html".to_string(),
        ]
    );
}

#[test]
fn a_template_golden_is_neither_reported_nor_rewritten() {
    let root = gi_tree("g1");
    let golden = "crates/mochiko-cli/tests/fixtures/template/governance.md";
    let body = "per GI-001-old-secret-words and GI-001\n";
    put(&root, golden, body);
    let held: Vec<_> = findings(&root, &[], &[])
        .into_iter()
        .filter(|f| f.0 == golden)
        .collect();
    assert_eq!(held, vec![], "G1: D14 as changed at build");
    let plan = rename_plan(&root, INTENT, "GI-001", "secret-hygiene-rules", &[]);
    assert!(plan.edits.iter().all(|e| e.path != Path::new(golden)));
    apply(&root, &plan);
    assert_eq!(read(&root, golden), body);
}

// ---------------------------------------------------------------------------
// T15 — the fourth review fix round (F2, B1)
// ---------------------------------------------------------------------------

/// One spec whose prototype holds the only `scr-001-menu.html` (the reviewer's r17).
fn one_screen_tree(tag: &str) -> PathBuf {
    let root = tree(tag);
    put(
        &root,
        ".mochiko/specs/lunch-orders/spec.md",
        "| SCR-001 | Menu |\n",
    );
    put(
        &root,
        ".mochiko/specs/lunch-orders/prototype/scr-001-menu.html",
        "a\n",
    );
    root
}

fn stale_of(root: &Path) -> Vec<String> {
    let spec = ".mochiko/specs/lunch-orders/spec.md";
    let plan = rename_plan(root, spec, "SCR-001", "weekly-menu-grid", &[]);
    plan.stale.iter().map(ToString::to_string).collect()
}

#[test]
fn a_root_bare_cite_of_the_only_file_with_that_name_is_stale_and_blocks_the_write() {
    let root = one_screen_tree("f2-bare");
    put(
        &root,
        "README.md",
        "R1 The lunch-orders screen is `scr-001-menu.html` today.\n",
    );
    assert_eq!(
        stale_of(&root),
        vec!["README.md:1:32 · scr-001-menu.html".to_string()],
        "F2: a spot that resolves to no other file stays stale"
    );
    let spec = ".mochiko/specs/lunch-orders/spec.md";
    let plan = rename_plan(&root, spec, "SCR-001", "weekly-menu-grid", &[]);
    assert!(matches!(
        rename::write(&root, &plan),
        Err(rename::WriteError::Stale(_))
    ));
    assert!(root
        .join(".mochiko/specs/lunch-orders/prototype/scr-001-menu.html")
        .exists());
    assert_eq!(read(&root, spec), "| SCR-001 | Menu |\n");
}

#[test]
fn every_unresolvable_spelling_of_the_old_name_is_stale() {
    let root = one_screen_tree("f2-shapes");
    put(
        &root,
        "README.md",
        "R1 The lunch-orders screen is `scr-001-menu.html` today.\n\
         R6 https://github.com/o/r/blob/main/.mochiko/specs/lunch-orders/prototype/scr-001-menu.html\n\
         R7 /Users/me/repo/.mochiko/specs/lunch-orders/prototype/scr-001-menu.html\n\
         R8 .mochiko\\specs\\lunch-orders\\prototype\\scr-001-menu.html\n\
         R9 prototype/scr-001-menu.html\n\
         R10 .mochiko/specs/lunch%2Dorders/prototype/scr-001-menu.html\n",
    );
    put(
        &root,
        "docs/x.md",
        "X3 [m](../../outside/scr-001-menu.html)\n",
    );
    assert_eq!(
        stale_of(&root),
        vec![
            "README.md:1:32 · scr-001-menu.html".to_string(),
            "README.md:2:75 · scr-001-menu.html".to_string(),
            "README.md:3:57 · scr-001-menu.html".to_string(),
            "README.md:4:42 · scr-001-menu.html".to_string(),
            "README.md:5:14 · scr-001-menu.html".to_string(),
            "README.md:6:45 · scr-001-menu.html".to_string(),
            "docs/x.md:1:22 · scr-001-menu.html".to_string(),
        ],
        "F2: R1, R6–R10 and X3"
    );
}

#[test]
fn an_old_name_run_on_into_a_longer_extension_stays_stale() {
    let root = one_screen_tree("b1-bak");
    put(
        &root,
        "README.md",
        "B1 a copy: .mochiko/specs/lunch-orders/prototype/scr-001-menu.html.bak\n",
    );
    assert_eq!(
        stale_of(&root),
        vec!["README.md:1:50 · scr-001-menu.html".to_string()],
        "B1"
    );
}

// ---------------------------------------------------------------------------
// T16 — the fifth review fix round (F3; option B: every old name the rename leaves is stale)
// ---------------------------------------------------------------------------

/// `ids rename … SCR-001 weekly-menu-grid --write` on a one-screen tree: its exit code and stderr.
fn write_one_screen(root: &Path) -> (i32, String) {
    let owning = arg(root, ".mochiko/specs/lunch-orders/spec.md");
    let (code, _, err) = run(&[
        "ids",
        "rename",
        &owning,
        "SCR-001",
        "weekly-menu-grid",
        "--write",
    ]);
    (code, err)
}

#[cfg(unix)]
#[test]
fn a_link_through_a_symlinked_directory_is_stale_and_blocks_the_write() {
    let root = one_screen_tree("f3-link");
    std::os::unix::fs::symlink(
        ".mochiko/specs/lunch-orders/prototype",
        root.join("screens"),
    )
    .expect("a symlink");
    put(
        &root,
        "README.md",
        "L1 via symlink [m](screens/scr-001-menu.html)\n",
    );
    assert_eq!(
        stale_of(&root),
        vec!["README.md:1:28 · scr-001-menu.html".to_string()],
        "F3"
    );
    let (code, err) = write_one_screen(&root);
    assert_eq!(code, 2, "{err}");
    assert!(root
        .join(".mochiko/specs/lunch-orders/prototype/scr-001-menu.html")
        .exists());
}

#[test]
fn a_case_variant_path_to_the_moved_file_is_stale_and_blocks_the_write() {
    let root = one_screen_tree("f3-case");
    put(
        &root,
        "README.md",
        "L2 case dir [m](.mochiko/specs/Lunch-Orders/prototype/scr-001-menu.html)\n",
    );
    assert_eq!(
        stale_of(&root),
        vec!["README.md:1:55 · scr-001-menu.html".to_string()],
        "F3: stale on any filesystem"
    );
    let (code, err) = write_one_screen(&root);
    assert_eq!(code, 2, "{err}");
}

#[test]
fn an_upper_case_spelling_of_the_old_name_is_stale() {
    let root = one_screen_tree("a3-case");
    put(
        &root,
        "README.md",
        "L3 case name .mochiko/specs/lunch-orders/prototype/SCR-001-menu.html here\n",
    );
    assert_eq!(
        stale_of(&root),
        vec!["README.md:1:52 · SCR-001-menu.html".to_string()],
        "A3: the scan ignores ASCII case"
    );
}

#[test]
fn a_link_that_resolves_to_the_move_and_to_another_file_is_not_rewritten_and_is_stale() {
    let root = one_screen_tree("a1-two");
    put(
        &root,
        "docs/.mochiko/specs/lunch-orders/prototype/scr-001-menu.html",
        "docs decoy\n",
    );
    let body = "X1 two candidates .mochiko/specs/lunch-orders/prototype/scr-001-menu.html here\n";
    put(&root, "docs/x.md", body);
    let spec = ".mochiko/specs/lunch-orders/spec.md";
    let plan = rename_plan(&root, spec, "SCR-001", "weekly-menu-grid", &[]);
    assert!(
        plan.edits.iter().all(|e| e.path != Path::new("docs/x.md")),
        "A1 at round 5: an ambiguous link is never rewritten"
    );
    assert_eq!(
        stale_of(&root),
        vec!["docs/x.md:1:57 · scr-001-menu.html".to_string()]
    );
}

#[test]
fn an_existing_longer_extension_file_is_stale_too() {
    // Outside the spec, so the copy stays put (one inside it carries the ID and moves too).
    let root = one_screen_tree("b1-held");
    put(&root, "docs/scr-001-menu.html.bak", "old\n");
    put(
        &root,
        "README.md",
        "B1 a copy: docs/scr-001-menu.html.bak\n",
    );
    assert_eq!(
        stale_of(&root),
        vec!["README.md:1:17 · scr-001-menu.html".to_string()]
    );
}

#[test]
fn two_move_sources_sharing_an_old_name_list_each_spot_once() {
    let root = tree("a2-dedupe");
    let file = ".mochiko/features/FEAT-003-wallet.md";
    put(&root, file, "# FEAT-003 — Wallet\n");
    put(&root, "docs/FEAT-003-wallet.md", "x\n");
    put(&root, "notes.md", "```\nFEAT-003-wallet.md\n```\n");
    let plan = rename_plan(&root, file, "FEAT-003", "digital-wallet-accounts", &[]);
    assert_eq!(plan.moves.len(), 2, "{:?}", plan.moves);
    let spots: Vec<String> = plan.stale.iter().map(ToString::to_string).collect();
    assert_eq!(
        spots,
        vec!["notes.md:2:1 · FEAT-003-wallet.md".to_string()],
        "A2"
    );
    let owning = arg(&root, file);
    let (code, _, err) = run(&[
        "ids",
        "rename",
        &owning,
        "FEAT-003",
        "digital-wallet-accounts",
        "--write",
    ]);
    assert_eq!(code, 2, "{err}");
    assert!(
        err.contains("nothing written: 1 spots"),
        "A2: a true count: {err}"
    );
}

// ---------------------------------------------------------------------------
// T17 — wave 1b: `ids rekey` by joined old ID (Q5 a), `ids rename` on a shared number (A7),
// `--alias` path scope (A3)
// ---------------------------------------------------------------------------

const BASELINE: &str = ".mochiko/product/constraints-and-decisions.md";

fn rekey_plan(root: &Path, owning: &str, old: &str, new: &str) -> rename::Plan {
    rename::plan_rekey(root, Path::new(owning), old, new, &[])
        .unwrap_or_else(|e| panic!("rekey {old} is refused: {}", e.0))
}

fn rekey_refusal(root: &Path, owning: &str, old: &str, new: &str) -> String {
    match rename::plan_rekey(root, Path::new(owning), old, new, &[]) {
        Ok(plan) => panic!(
            "rekey {old} plans {} edits, not a refusal",
            plan.edits.len()
        ),
        Err(refused) => refused.0,
    }
}

/// A product baseline in the D-family template's shape: one index row and one heading per entry.
fn baseline(entries: &[(&str, &str)]) -> String {
    let mut text = String::from(
        "### Decision Summary  *(the ID index)*\n\n| ID | Decision | Choice |\n|----|----------|--------|\n",
    );
    for (id, title) in entries {
        text.push_str(&format!("| {id} | {title} | x |\n"));
    }
    for (id, title) in entries {
        text.push_str(&format!("\n### {id}: {title}\n\n**Context** (x)\n"));
    }
    text
}

#[test]
fn a_bare_rekey_of_one_template_shaped_entry_keeps_todays_behaviour() {
    for id in ["D-012", "D-012-audit-log-retention"] {
        let root = tree("1b-template");
        put(&root, BASELINE, &baseline(&[(id, "Audit log retention")]));
        put(&root, "docs/a.md", &format!("per `product` {id}\n"));
        let plan = rekey_plan(&root, BASELINE, "D-012", "D-013");
        apply(&root, &plan);
        let moved = id.replace("D-012", "D-013");
        assert_eq!(
            read(&root, BASELINE),
            baseline(&[(&moved, "Audit log retention")]),
            "{id}"
        );
        assert_eq!(read(&root, "docs/a.md"), format!("per `product` {moved}\n"));
    }
}

/// The architecture store's shape (`architecture-store.yaml`): headings only in `concerns.md`, a
/// graduated file with its own heading, and the derived root `ARCHITECTURE.md` citing the row.
fn ax_store(tag: &str) -> PathBuf {
    let root = tree(tag);
    put(
        &root,
        ".mochiko/product/architecture/concerns.md",
        "# Concern Ledger\n\n## AX-001-tenant-data-isolation Tenancy\n\n- **Stance**: decided\n",
    );
    put(
        &root,
        ".mochiko/product/architecture/concerns/AX-001-tenant-data-isolation.md",
        "# AX-001-tenant-data-isolation Tenancy\n",
    );
    put(
        &root,
        "ARCHITECTURE.md",
        "# Architecture\n\n| AX | Concern | Detail |\n|----|---------|--------|\n| AX-001-tenant-data-isolation | Tenancy | [concerns/AX-001-tenant-data-isolation.md](.mochiko/product/architecture/concerns/AX-001-tenant-data-isolation.md) |\n",
    );
    root
}

/// What an AX-001 to AX-002 rekey on the store shape plans, whatever the owning file.
fn assert_ax_plan(plan: &rename::Plan) {
    let edited: Vec<String> = plan
        .edits
        .iter()
        .map(|e| e.path.to_string_lossy().into_owned())
        .collect();
    assert_eq!(
        edited,
        vec![
            ".mochiko/product/architecture/concerns/AX-001-tenant-data-isolation.md",
            ".mochiko/product/architecture/concerns.md",
            "ARCHITECTURE.md",
        ],
        "walk order"
    );
    let architecture = plan
        .edits
        .iter()
        .find(|e| e.path == Path::new("ARCHITECTURE.md"))
        .expect("the row");
    assert!(architecture.new.contains(
        "| AX-002-tenant-data-isolation | Tenancy | [concerns/AX-001-tenant-data-isolation.md](.mochiko/product/architecture/concerns/AX-002-tenant-data-isolation.md) |"
    ));
    assert_eq!(
        plan.moves,
        vec![(
            PathBuf::from(".mochiko/product/architecture/concerns/AX-001-tenant-data-isolation.md"),
            PathBuf::from(".mochiko/product/architecture/concerns/AX-002-tenant-data-isolation.md"),
        )]
    );
    let spots: Vec<String> = plan.stale.iter().map(ToString::to_string).collect();
    assert_eq!(
        spots,
        vec!["ARCHITECTURE.md:5:54 · AX-001-tenant-data-isolation.md".to_string()],
        "the link text names no file from the root: wave 1's option B"
    );
}

#[test]
fn the_graduated_ax_concern_rekeys() {
    let root = ax_store("1b-ax");
    let graduated = ".mochiko/product/architecture/concerns/AX-001-tenant-data-isolation.md";
    assert_ax_plan(&rekey_plan(&root, graduated, "AX-001", "AX-002"));
}

#[test]
fn the_l2_record_rekeys() {
    let root = tree("1b-l2");
    let record = ".mochiko/brainstorms/foo-bar/record.md";
    put(
        &root,
        record,
        "- **D1 frame — a summary line**\n### D1 — Frame\n",
    );
    apply(&root, &rekey_plan(&root, record, "D1", "D2"));
    assert_eq!(
        read(&root, record),
        "- **D2 frame — a summary line**\n### D2 — Frame\n"
    );
}

/// A session decision with an amendment card (`cli-schema-delivery` D3, cards :400 and :1037).
fn amendment_cards(id: &str) -> String {
    format!(
        "### {id} — Delivery binding\n\nbody\n\n### {id} — amended (probe (e)): render chunking\n"
    )
}

#[test]
fn a_rename_of_an_amendment_card_decision_rewrites_both_cards() {
    // R1: the same-kind refusal is rekey's only; renaming a duplicate loses nothing.
    let root = tree("1b-cards-rename");
    let record = ".mochiko/brainstorms/foo-bar/record.md";
    put(&root, record, &amendment_cards("D3"));
    apply(
        &root,
        &rename_plan(&root, record, "D3", "delivery-binding-rule", &[]),
    );
    assert_eq!(
        read(&root, record),
        amendment_cards("D3-delivery-binding-rule")
    );
    apply(
        &root,
        &rename_plan(&root, record, "D3", "render-binding-cards", &[]),
    );
    assert_eq!(
        read(&root, record),
        amendment_cards("D3-render-binding-cards")
    );
}

/// W2-schema's landing case (0048): the landed entry and the in-flight one share `D-012`.
fn landing(tag: &str) -> PathBuf {
    let root = tree(tag);
    put(
        &root,
        BASELINE,
        &baseline(&[
            ("D-012-audit-log-retention", "Audit log retention"),
            ("D-012-csv-export-format", "CSV export format"),
        ]),
    );
    put(
        &root,
        "docs/a.md",
        "per `product` D-012-audit-log-retention\n",
    );
    put(
        &root,
        "docs/b.md",
        "per `product` D-012-csv-export-format\n",
    );
    root
}

#[test]
fn a_joined_old_id_rekeys_only_its_template_shaped_entry() {
    let root = landing("1b-landing");
    let plan = rekey_plan(&root, BASELINE, "D-012-csv-export-format", "D-013");
    assert!(plan.untied.is_empty(), "{:?}", plan.untied);
    apply(&root, &plan);
    assert_eq!(
        read(&root, BASELINE),
        baseline(&[
            ("D-012-audit-log-retention", "Audit log retention"),
            ("D-013-csv-export-format", "CSV export format"),
        ])
    );
    assert_eq!(
        read(&root, "docs/a.md"),
        "per `product` D-012-audit-log-retention\n"
    );
    assert_eq!(
        read(&root, "docs/b.md"),
        "per `product` D-013-csv-export-format\n"
    );
}

#[test]
fn a_joined_old_id_with_one_holder_rekeys_its_bare_mentions_too() {
    let root = tree("1b-one-holder");
    put(
        &root,
        BASELINE,
        &baseline(&[("D-012-audit-log-retention", "Audit log retention")]),
    );
    put(
        &root,
        "docs/a.md",
        "per `product` D-012 and `product` D-012-audit-log-retention\n",
    );
    apply(
        &root,
        &rekey_plan(&root, BASELINE, "D-012-audit-log-retention", "D-013"),
    );
    assert_eq!(
        read(&root, BASELINE),
        baseline(&[("D-013-audit-log-retention", "Audit log retention")])
    );
    assert_eq!(
        read(&root, "docs/a.md"),
        "per `product` D-013 and `product` D-013-audit-log-retention\n"
    );
    let root = ax_store("1b-ax-joined");
    let graduated = ".mochiko/product/architecture/concerns/AX-001-tenant-data-isolation.md";
    assert_ax_plan(&rekey_plan(
        &root,
        graduated,
        "AX-001-tenant-data-isolation",
        "AX-002",
    ));
}

#[test]
fn the_ax_catalog_is_an_owning_file_too() {
    // C2: rekey takes any of the holder's sites as its owning file; rename keeps the first one.
    let root = ax_store("1b-ax-catalog");
    let catalog = ".mochiko/product/architecture/concerns.md";
    assert_ax_plan(&rekey_plan(&root, catalog, "AX-001", "AX-002"));
}

#[test]
fn a_near_prefix_slug_is_another_holder() {
    let root = tree("1b-near");
    put(
        &root,
        BASELINE,
        &baseline(&[
            ("D-012-csv-export-format", "CSV export format"),
            ("D-012-csv-export-formats", "CSV export formats"),
        ]),
    );
    put(
        &root,
        "docs/a.md",
        "per `product` D-012-csv-export-formats\n",
    );
    apply(
        &root,
        &rekey_plan(&root, BASELINE, "D-012-csv-export-format", "D-013"),
    );
    assert_eq!(
        read(&root, BASELINE),
        baseline(&[
            ("D-013-csv-export-format", "CSV export format"),
            ("D-012-csv-export-formats", "CSV export formats"),
        ]),
        "A4: the equal scanned slug, never a prefix"
    );
    assert_eq!(
        read(&root, "docs/a.md"),
        "per `product` D-012-csv-export-formats\n"
    );
}

#[test]
fn a_bare_old_id_on_a_shared_number_is_refused() {
    let root = landing("1b-bare-shared");
    let reason = rekey_refusal(&root, BASELINE, "D-012", "D-013");
    assert!(
        reason.contains("D-012-audit-log-retention")
            && reason.contains("D-012-csv-export-format")
            && reason.contains("joined"),
        "{reason}"
    );
}

#[test]
fn the_identical_slug_landing_shape_is_refused_by_rekey() {
    // R1: two entries share the slug, so no slug can tell them apart.
    let root = tree("1b-same-slug");
    put(
        &root,
        BASELINE,
        &baseline(&[
            ("D-012-csv-export-format", "CSV export format"),
            ("D-012-csv-export-format", "CSV report layout"),
        ]),
    );
    for old in ["D-012", "D-012-csv-export-format"] {
        let reason = rekey_refusal(&root, BASELINE, old, "D-013");
        assert!(
            reason.contains(&format!("{BASELINE}:5")) && reason.contains(&format!("{BASELINE}:6")),
            "{old}: {reason}"
        );
    }
}

#[test]
fn a_rekey_of_an_amendment_card_decision_is_refused() {
    let record = ".mochiko/brainstorms/foo-bar/record.md";
    for (cards, olds) in [
        ("D3", vec!["D3"]),
        (
            "D3-delivery-binding-rule",
            vec!["D3", "D3-delivery-binding-rule"],
        ),
    ] {
        let root = tree("1b-cards-rekey");
        put(&root, record, &amendment_cards(cards));
        for old in olds {
            let reason = rekey_refusal(&root, record, old, "D4");
            assert!(
                reason.contains(&format!("{record}:1")) && reason.contains(&format!("{record}:5")),
                "C3 {cards} by {old}: {reason}"
            );
        }
    }
}

#[test]
fn a_joined_old_id_no_holder_carries_is_refused() {
    let root = landing("1b-no-holder");
    let reason = rekey_refusal(&root, BASELINE, "D-012-pdf-export-format", "D-013");
    assert!(
        reason.contains("holds no definition of `D-012-pdf-export-format`"),
        "{reason}"
    );
}

#[test]
fn a_shared_numbers_bare_mention_is_untied_and_listed() {
    let root = landing("1b-untied");
    put(&root, "docs/c.md", "see `product` D-012 here\n");
    let plan = rekey_plan(&root, BASELINE, "D-012-csv-export-format", "D-013");
    assert!(plan.edits.iter().all(|e| e.path != Path::new("docs/c.md")));
    let spots: Vec<String> = plan.untied.iter().map(ToString::to_string).collect();
    assert_eq!(spots, vec!["docs/c.md:1:15 · D-012".to_string()], "D18, Q1");
    assert!(plan.stale.is_empty());
}

#[test]
fn a_bare_entry_beside_the_chosen_one_is_listed_not_renumbered() {
    // Q4: a bare site of a holder's kind in the holder's file marks the number shared.
    let root = tree("1b-legacy");
    let entries = [
        ("D-012", "Audit log retention"),
        ("D-012-csv-export-format", "CSV export format"),
    ];
    put(&root, BASELINE, &baseline(&entries));
    let plan = rekey_plan(&root, BASELINE, "D-012-csv-export-format", "D-013");
    let spots: Vec<String> = plan.untied.iter().map(ToString::to_string).collect();
    assert_eq!(
        spots,
        vec![
            format!("{BASELINE}:5:3 · D-012"),
            format!("{BASELINE}:8:5 · D-012")
        ]
    );
    let reason = rekey_refusal(&root, BASELINE, "D-012", "D-014");
    assert!(reason.contains("D-012-csv-export-format"), "{reason}");
    apply(&root, &plan);
    assert_eq!(
        read(&root, BASELINE),
        baseline(&[
            ("D-012", "Audit log retention"),
            ("D-013-csv-export-format", "CSV export format"),
        ])
    );
}

#[test]
fn a_joined_rekey_moves_only_its_own_file() {
    let root = tree("1b-feat-moves");
    let csv = ".mochiko/features/FEAT-012-csv-export-format.md";
    let audit = ".mochiko/features/FEAT-012-audit-log-retention.md";
    put(&root, csv, "# FEAT-012-csv-export-format — CSV\n");
    put(&root, audit, "# FEAT-012-audit-log-retention — Audit\n");
    put(
        &root,
        "notes.md",
        "FEAT-012-csv-export-format and FEAT-012-audit-log-retention\n",
    );
    let plan = rekey_plan(&root, csv, "FEAT-012-csv-export-format", "FEAT-013");
    assert_eq!(
        plan.moves,
        vec![(
            PathBuf::from(csv),
            PathBuf::from(".mochiko/features/FEAT-013-csv-export-format.md"),
        )]
    );
    apply(&root, &plan);
    assert_eq!(
        read(&root, ".mochiko/features/FEAT-013-csv-export-format.md"),
        "# FEAT-013-csv-export-format — CSV\n"
    );
    assert_eq!(
        read(&root, audit),
        "# FEAT-012-audit-log-retention — Audit\n"
    );
    assert_eq!(
        read(&root, "notes.md"),
        "FEAT-013-csv-export-format and FEAT-012-audit-log-retention\n"
    );
}

/// A shared number's rekey target, keyed to the chosen holder's slugs only (F4).
fn shared_target(family: &str, slugs: &[&str]) -> rename::Target {
    rename::Target::Rekey {
        family: fam(family),
        from: "012".into(),
        to: "013".into(),
        slugs: slugs.iter().map(|s| s.to_string()).collect(),
        shared: true,
    }
}

#[test]
fn the_limb_blocks_another_holders_three_word_token_and_a_bare_token() {
    // R2: a planner bug that renumbers the other holder, or an untied bare mention, cannot pass.
    let target = shared_target("D", &["csv-export-format"]);
    assert_eq!(
        rename::diff_check(
            "per `product` D-012-csv-export-format\n",
            "per `product` D-013-csv-export-format\n",
            &target
        ),
        Ok(1),
        "the chosen holder's own token"
    );
    for (old, new) in [
        (
            "per `product` D-012-audit-log-retention\n",
            "per `product` D-013-audit-log-retention\n",
        ),
        ("per `product` D-012\n", "per `product` D-013\n"),
    ] {
        assert!(
            rename::diff_check(old, new, &target).is_err(),
            "{old:?} must not renumber on a shared number"
        );
    }
}

#[test]
fn the_narrowed_slug_set_blocks_another_holders_short_file_slug() {
    // F4: the set is the chosen holder's slugs only, so the other's file slug is no target token.
    let target = shared_target("FEAT", &["csv-export-format"]);
    for slug in ["wallet", "audit-log"] {
        let old = format!("see FEAT-012-{slug}\n");
        let new = format!("see FEAT-013-{slug}\n");
        assert!(rename::diff_check(&old, &new, &target).is_err(), "{slug}");
    }
}

#[test]
fn a_shared_rekey_write_refuses_a_move_of_the_other_holders_file() {
    // F4: moves get their own check; a planner bug that moves the other holder's file blocks.
    let root = tree("1b-move-limb");
    let audit = ".mochiko/features/FEAT-012-audit-log-retention.md";
    put(&root, audit, "# FEAT-012-audit-log-retention — Audit\n");
    let plan = rename::Plan {
        edits: vec![],
        moves: vec![(
            PathBuf::from(audit),
            PathBuf::from(".mochiko/features/FEAT-013-audit-log-retention.md"),
        )],
        stale: vec![],
        untied: vec![],
        kept: vec![],
        target: shared_target("FEAT", &["csv-export-format"]),
    };
    let result = rename::write(&root, &plan);
    assert!(
        matches!(result, Err(rename::WriteError::Blocked { .. })),
        "{result:?}"
    );
    assert!(root.join(audit).exists(), "nothing moved");
}

#[test]
fn the_cli_lists_untied_mentions_once_and_still_writes() {
    // Q1: listed, never a bar to the write; A6: printed once, in the preview before the write.
    let root = landing("1b-cli-untied");
    put(&root, "docs/c.md", "see `product` D-012 here\n");
    let owning = arg(&root, BASELINE);
    let rekey = ["ids", "rekey", &owning, "D-012-csv-export-format", "D-013"];
    let line = "untied mention docs/c.md:1:15 · D-012\n";
    let (code, out, err) = run(&rekey);
    assert_eq!(code, 0, "{err}");
    assert_eq!(out.matches(line).count(), 1, "{out}");
    assert!(
        out.ends_with("· 1 untied · nothing written; --write applies it\n"),
        "{out}"
    );
    let (code, out, err) = run(&[&rekey[..], &["--write"]].concat());
    assert_eq!(code, 0, "{err}");
    assert_eq!(out.matches(line).count(), 1, "{out}");
    assert!(out.ends_with("· 1 untied · written\n"), "{out}");
    assert_eq!(read(&root, "docs/c.md"), "see `product` D-012 here\n");
}

#[test]
fn a_rename_on_a_shared_number_is_refused() {
    // A7: a rename on a doubly-held number would give both entries one slug; resolve the landing
    // with a joined rekey first. Never the same-kind rule (R1).
    let root = landing("1b-a7");
    let refusal = |root: &Path| {
        rename::plan_rename(
            root,
            Path::new(BASELINE),
            "D-012",
            "csv-export-rules",
            &[],
            &[],
        )
        .err()
        .map(|r| r.0)
        .unwrap_or_default()
    };
    let reason = refusal(&root);
    assert!(
        reason.contains("D-012-audit-log-retention")
            && reason.contains("D-012-csv-export-format")
            && reason.contains("ids rekey"),
        "{reason}"
    );
    let root = tree("1b-a7-legacy");
    put(
        &root,
        BASELINE,
        &baseline(&[
            ("D-012", "Audit log retention"),
            ("D-012-csv-export-format", "CSV export format"),
        ]),
    );
    let reason = refusal(&root);
    assert!(
        reason.contains("D-012 (bare)") && reason.contains("D-012-csv-export-format"),
        "Q4: {reason}"
    );
}

/// Two sessions the shorthand `feature-map` could mean, each defining D10 (A3).
fn feature_map_sessions(tag: &str) -> PathBuf {
    let root = tree(tag);
    put(
        &root,
        ".mochiko/brainstorms/feature-map-layer/record.md",
        "### D10 — Layer\n### D11 — Split\n",
    );
    put(
        &root,
        ".mochiko/brainstorms/feature-map-granularity-and-reparenting/record.md",
        "### D10 — Grain\n",
    );
    root
}

const LAYER: &str = ".mochiko/brainstorms/feature-map-layer/record.md";

fn alias_refusal(root: &Path, record: &str, id: &str, aliases: &[&str]) -> String {
    let aliases: Vec<String> = aliases.iter().map(|a| a.to_string()).collect();
    match rename::plan_rename(
        root,
        Path::new(record),
        id,
        "layer-split-rule",
        &aliases,
        &[],
    ) {
        Ok(_) => panic!("{aliases:?} plans, not a refusal"),
        Err(refused) => refused.0,
    }
}

#[test]
fn a_scoped_alias_rewrites_only_in_its_file() {
    let root = feature_map_sessions("a3-scope");
    put(&root, "a.md", "per feature-map D10 here\n");
    put(&root, "b.md", "per feature-map D10 there\n");
    put(
        &root,
        "c.md",
        "per `feature-map-layer` D10 and feature-map D10/D11\n",
    );
    let plan = rename_plan(
        &root,
        LAYER,
        "D10",
        "layer-split-rule",
        &["feature-map D=a.md"],
    );
    let c = plan
        .edits
        .iter()
        .find(|e| e.path == Path::new("c.md"))
        .expect("c.md's qualified mention");
    assert!(
        rename::diff_check(&c.old, &c.new, &plan.target).is_ok(),
        "A5: the public check reads the target as for a file in no scope"
    );
    apply(&root, &plan);
    assert_eq!(
        read(&root, "a.md"),
        "per `feature-map-layer` D10-layer-split-rule here\n"
    );
    assert_eq!(read(&root, "b.md"), "per feature-map D10 there\n");
    assert_eq!(
        read(&root, "c.md"),
        "per `feature-map-layer` D10-layer-split-rule and feature-map D10/D11\n",
        "an out-of-scope form in an edited file stays, and the write's check passes"
    );
}

#[test]
fn a_shorthand_alias_without_a_path_is_refused() {
    let root = feature_map_sessions("a3-q2");
    put(&root, "a.md", "per feature-map D10 here\n");
    let reason = alias_refusal(&root, LAYER, "D10", &["feature-map D"]);
    assert!(
        reason.contains("feature-map D") && reason.contains("shorthand"),
        "Q2: {reason}"
    );
}

#[test]
fn a_shorthand_naming_a_held_session_is_refused() {
    // F2, the reviewer's P3: a real session's name would re-own that session's own mentions.
    let root = feature_map_sessions("a3-f2");
    put(
        &root,
        "a.md",
        "per feature-map-granularity-and-reparenting D10\n",
    );
    let reason = alias_refusal(
        &root,
        LAYER,
        "D10",
        &["feature-map-granularity-and-reparenting D=a.md"],
    );
    assert!(
        reason.contains("names an owner the tree holds"),
        "F2: {reason}"
    );
    // The same for a per-artifact owner: a spec the tree holds.
    put(
        &root,
        ".mochiko/specs/lunch-orders/spec.md",
        "**FR-001**: x\n",
    );
    put(
        &root,
        ".mochiko/specs/other-spec/spec.md",
        "**FR-001**: y\n",
    );
    let aliases = vec!["lunch-orders FR-=a.md".to_string()];
    let reason = rename::plan_rename(
        &root,
        Path::new(".mochiko/specs/other-spec/spec.md"),
        "FR-001",
        "other-spec-rule",
        &aliases,
        &[],
    )
    .err()
    .map(|r| r.0)
    .unwrap_or_default();
    assert!(
        reason.contains("names an owner the tree holds"),
        "F2: {reason}"
    );
}

#[test]
fn a_shorthand_path_must_name_a_file() {
    // F3: a directory would cover both sessions' records again.
    let root = feature_map_sessions("a3-f3");
    put(&root, "a.md", "per feature-map D10 here\n");
    let reason = alias_refusal(
        &root,
        LAYER,
        "D10",
        &["feature-map D=.mochiko/brainstorms/"],
    );
    assert!(reason.contains("one walked file"), "F3: {reason}");
    let aliases = vec!["FM-D=.mochiko/brainstorms/".to_string()];
    assert!(
        rename::plan_rename(
            &root,
            Path::new(LAYER),
            "D10",
            "layer-split-rule",
            &aliases,
            &[]
        )
        .is_ok(),
        "a glued prefix may keep a directory scope"
    );
}

#[test]
fn a_scope_path_with_no_walked_file_is_refused() {
    let root = feature_map_sessions("a3-nofile");
    put(&root, "CHANGELOG.md", "FM-D10\n");
    for value in ["FM-D=missing.md", "FM-D=CHANGELOG.md"] {
        let reason = alias_refusal(&root, LAYER, "D10", &[value]);
        assert!(
            reason.contains(value) && reason.contains("no walked file"),
            "{value}: {reason}"
        );
    }
}

#[test]
fn one_literal_given_tree_wide_and_scoped_is_refused() {
    let root = feature_map_sessions("a3-both");
    put(&root, "a.md", "FM-D10\n");
    let reason = alias_refusal(&root, LAYER, "D10", &["FM-D", "FM-D=a.md"]);
    assert!(reason.contains("both tree-wide and scoped"), "{reason}");
}

#[test]
fn the_cli_takes_an_alias_with_its_file() {
    let root = feature_map_sessions("a3-cli");
    put(&root, "a.md", "per feature-map D10 here\n");
    put(&root, "b.md", "per feature-map D10 there\n");
    let owning = arg(&root, LAYER);
    let (code, out, err) = run(&[
        "ids",
        "rename",
        &owning,
        "D10",
        "layer-split-rule",
        "--alias",
        "feature-map D=a.md",
    ]);
    assert_eq!(code, 0, "{err}");
    assert!(out.contains("a.md:1\n"), "{out}");
    assert!(!out.contains("b.md:1\n"), "{out}");
}

// ---------------------------------------------------------------------------
// T17 fix round — F1: a shared number that nothing proves two entries is ambiguous; B2, A1, A2
// ---------------------------------------------------------------------------

const CATALOG: &str = ".mochiko/product/architecture/concerns.md";
const DRIFTED: &str = ".mochiko/product/architecture/concerns/AX-001-tenant-isolation-rules.md";

/// The reviewer's x4: one AX concern whose catalog heading and graduated file drifted apart.
fn drifted_ax(tag: &str) -> PathBuf {
    let root = tree(tag);
    put(
        &root,
        CATALOG,
        "# Concern Ledger\n\n## AX-001-tenant-data-isolation Tenancy\n\n- **Stance**: decided\n",
    );
    put(&root, DRIFTED, "# AX-001-tenant-isolation-rules Tenancy\n");
    put(
        &root,
        "ARCHITECTURE.md",
        "| AX-001-tenant-data-isolation | Tenancy | [g](.mochiko/product/architecture/concerns/AX-001-tenant-isolation-rules.md) |\n",
    );
    root
}

/// Run a `--write` that must be refused, and confirm every file of the tree is as it was.
fn refused_write(root: &Path, args: &[&str], files: &[&str]) -> String {
    let before: Vec<String> = files.iter().map(|f| read(root, f)).collect();
    let (code, _, err) = run(&[args, &["--write"]].concat());
    assert_eq!(code, 2, "{args:?}: {err}");
    let after: Vec<String> = files.iter().map(|f| read(root, f)).collect();
    assert_eq!(before, after, "{args:?}: nothing written");
    err
}

#[test]
fn a_joined_rekey_of_a_drifted_ax_concern_is_refused() {
    let root = drifted_ax("1b-x4");
    let files = [CATALOG, DRIFTED, "ARCHITECTURE.md"];
    for (owning, old) in [
        (CATALOG, "AX-001-tenant-data-isolation"),
        (DRIFTED, "AX-001-tenant-isolation-rules"),
    ] {
        let err = refused_write(
            &root,
            &["ids", "rekey", &arg(&root, owning), old, "AX-002"],
            &files,
        );
        assert!(
            err.contains(
                "these may be one entry whose slugs drifted: give its sites one slug by hand first"
            ) && err.contains(&format!("{CATALOG}:3"))
                && err.contains(&format!("{DRIFTED}:1")),
            "F1 x4 {old}: {err}"
        );
    }
}

#[test]
fn a_joined_rekey_of_a_drifted_d_entry_is_refused() {
    // The reviewer's x5: one entry's row and heading carry near-prefix slugs.
    let root = tree("1b-x5");
    put(
        &root,
        BASELINE,
        "### Decision Summary\n\n| ID | Decision | Choice |\n|----|----------|--------|\n| D-012-csv-export-format | CSV export | x |\n\n### D-012-csv-export-formats: CSV export\n\n**Context** (x)\n",
    );
    put(
        &root,
        "docs/a.md",
        "per `product` D-012-csv-export-format here, and bare `product` D-012 there\n",
    );
    for old in ["D-012-csv-export-format", "D-012-csv-export-formats"] {
        let err = refused_write(
            &root,
            &["ids", "rekey", &arg(&root, BASELINE), old, "D-013"],
            &[BASELINE, "docs/a.md"],
        );
        assert!(
            err.contains("drifted") && err.contains(&format!("{BASELINE}:5")),
            "F1 x5 {old}: {err}"
        );
    }
}

#[test]
fn a_shared_numbers_refusals_name_both_readings() {
    // F1 item 3: a landing goes to a joined rekey; drift goes to a hand alignment, then rename.
    let root = drifted_ax("1b-x4-readings");
    // The rename's owning file is the first-read definition, the graduated file (C2).
    let rename = rename::plan_rename(
        &root,
        Path::new(DRIFTED),
        "AX-001",
        "tenant-data-isolation",
        &[],
        &[],
    )
    .err()
    .map(|r| r.0)
    .unwrap_or_default();
    let rekey = rekey_refusal(&root, CATALOG, "AX-001", "AX-002");
    for reason in [rename, rekey] {
        assert!(
            reason.contains("AX-001-tenant-data-isolation")
                && reason.contains("AX-001-tenant-isolation-rules")
                && reason.contains("a landing")
                && reason.contains("joined old ID")
                && reason.contains("drifted")
                && reason.contains("by hand"),
            "{reason}"
        );
    }
}

#[test]
fn two_ax_catalog_headings_stay_two_entries() {
    // F1 item 5: two headings of one kind in one file prove two entries; the joined rekey runs.
    let root = tree("1b-ax-two");
    put(
        &root,
        CATALOG,
        "# Concern Ledger\n\n## AX-001-tenant-data-isolation Tenancy\n\n## AX-001-audit-log-retention Audit\n",
    );
    apply(
        &root,
        &rekey_plan(&root, CATALOG, "AX-001-audit-log-retention", "AX-002"),
    );
    assert_eq!(
        read(&root, CATALOG),
        "# Concern Ledger\n\n## AX-001-tenant-data-isolation Tenancy\n\n## AX-002-audit-log-retention Audit\n"
    );
}

#[test]
fn a_bare_entry_of_another_kind_beside_the_chosen_one_is_refused() {
    // B2 as broadened (the reviewer's x2): a legacy bold line beside a joined row and heading.
    let root = tree("1b-x2");
    put(
        &root,
        BASELINE,
        "### Decision Summary\n\n| ID | Decision | Choice |\n|----|----------|--------|\n| D-012-csv-export-format | CSV export | x |\n\n- **D-012:** Legacy audit log retention, decided long ago\n\n### D-012-csv-export-format: CSV export\n\n**Context** (x)\n",
    );
    put(
        &root,
        "docs/a.md",
        "per `product` D-012-csv-export-format and the legacy `product` D-012 retention\n",
    );
    let err = refused_write(
        &root,
        &[
            "ids",
            "rekey",
            &arg(&root, BASELINE),
            "D-012-csv-export-format",
            "D-013",
        ],
        &[BASELINE, "docs/a.md"],
    );
    assert!(err.contains(&format!("{BASELINE}:7")), "B2: {err}");
}

#[test]
fn a_shorthand_path_is_read_as_a_scope_path_is() {
    // A1: `./` in front and `/` behind are trimmed, as `under` trims a scope.
    for value in ["feature-map D=./a.md", "feature-map D=a.md/"] {
        let root = feature_map_sessions("a3-a1");
        put(&root, "a.md", "per feature-map D10 here\n");
        put(&root, "b.md", "per feature-map D10 there\n");
        let aliases = vec![value.to_string()];
        let plan = rename::plan_rename(
            &root,
            Path::new(LAYER),
            "D10",
            "layer-split-rule",
            &aliases,
            &[],
        )
        .unwrap_or_else(|e| panic!("{value}: {}", e.0));
        apply(&root, &plan);
        assert_eq!(
            read(&root, "a.md"),
            "per `feature-map-layer` D10-layer-split-rule here\n",
            "{value}"
        );
        assert_eq!(
            read(&root, "b.md"),
            "per feature-map D10 there\n",
            "{value}"
        );
    }
}

#[test]
fn another_holders_mention_is_listed_as_kept() {
    // A2: a shared rekey skips the other holder's mentions; the preview lists each once.
    let root = landing("1b-kept");
    let rekey = [
        "ids",
        "rekey",
        &arg(&root, BASELINE),
        "D-012-csv-export-format",
        "D-013",
    ];
    let (code, out, err) = run(&rekey);
    assert_eq!(code, 0, "{err}");
    let line = "kept docs/a.md:1:15 · D-012-audit-log-retention · another holder's\n";
    assert_eq!(out.matches(line).count(), 1, "{out}");
}

// ---------------------------------------------------------------------------
// T18 — wave 1c: a session's record path as a mention's own qualifier (D11 as changed at build,
// item 13)
// ---------------------------------------------------------------------------

const FOO_BAR: &str = ".mochiko/brainstorms/foo-bar/record.md";

/// Two sessions: `foo-bar` defining D3, D6 and D7 bare, and `baz-qux` defining D3.
fn path_sessions(tag: &str) -> PathBuf {
    let root = tree(tag);
    put(
        &root,
        FOO_BAR,
        "### D3 — Delivery binding\n### D6 — y\n### D7 — z\n",
    );
    put(
        &root,
        ".mochiko/brainstorms/baz-qux/record.md",
        "### D3 — w\n",
    );
    root
}

#[test]
fn a_record_path_in_code_names_its_session() {
    let root = path_sessions("1c-path");
    let text = format!("per `{FOO_BAR}` D3, D6 and `{FOO_BAR}`\n  D7 here\n");
    assert_eq!(
        resolutions(&root, ".mochiko/strips/x.md", &text),
        vec![
            ("D3".into(), Resolved::Owned(name("foo-bar"))),
            ("D6".into(), Resolved::Owned(name("foo-bar"))),
            ("D7".into(), Resolved::Owned(name("foo-bar"))),
        ],
        "item 13: the path names its session, for a compound's member (S4) and across a soft break (C3)"
    );
}

#[test]
fn a_path_names_a_session_only_as_its_record() {
    let root = path_sessions("1c-boundary");
    for text in [
        "per `.mochiko/brainstorms/no-such/record.md` D3\n",
        "per `.mochiko/brainstorms/foo-bar/synthesis.md` D3\n",
        "per `.mochiko/brainstorms/foo-bar/reports/record.md` D3\n",
        "per `.mochiko/brainstorms/<slug>/record.md` D3\n",
        "per `record.md` D3\n",
        "per `foo-bar/record.md` D3\n",
    ] {
        assert_eq!(
            resolutions(&root, ".mochiko/strips/x.md", text),
            vec![("D3".into(), Resolved::Unresolved)],
            "Q2: {text}"
        );
    }
    // Another family behind a record path keeps today's reading.
    put(&root, ".mochiko/specs/foo-bar/spec.md", "**FR-001**: x\n");
    assert_eq!(
        resolutions(
            &root,
            ".mochiko/strips/x.md",
            &format!("per `{FOO_BAR}` FR-001\n")
        ),
        vec![("FR-001".into(), Resolved::Unresolved)]
    );
    // A path in plain text is no qualifier; the line reads it as a record link, as before (B1).
    assert_eq!(
        resolutions(
            &root,
            ".mochiko/strips/x.md",
            &format!("per {FOO_BAR} D3\n")
        ),
        vec![("D3".into(), Resolved::Owned(name("foo-bar")))]
    );
}

#[test]
fn the_check_reports_a_path_qualified_mention() {
    let root = tree("1c-check");
    put(
        &root,
        FOO_BAR,
        "### D3-delivery-binding-rule — Delivery binding\n",
    );
    let strip = ".mochiko/strips/x.md";
    put(
        &root,
        strip,
        &format!("per `{FOO_BAR}` D3 and `{FOO_BAR}` D3-render-chunking-rule\n"),
    );
    assert_eq!(
        findings(&root, &[strip], &[]),
        vec![
            (strip.into(), 1, ids::Kind::Bare, "D3".into()),
            (
                strip.into(),
                1,
                ids::Kind::Drift,
                "D3-render-chunking-rule".into()
            ),
        ]
    );
}

#[test]
fn a_rename_rewrites_a_path_qualified_token_and_keeps_the_path() {
    let root = path_sessions("1c-rename");
    let strip = ".mochiko/strips/x.md";
    put(&root, strip, &format!("per `{FOO_BAR}` D3 as amended\n"));
    let plan = rename_plan(&root, FOO_BAR, "D3", "delivery-binding-rule", &[]);
    let edit = plan
        .edits
        .iter()
        .find(|e| e.path == Path::new(strip))
        .expect("the strip's path-qualified mention");
    assert_eq!(
        rename::diff_check(&edit.old, &edit.new, &plan.target),
        Ok(1)
    );
    let (code, out, err) = run(&[
        "ids",
        "rename",
        &arg(&root, FOO_BAR),
        "D3",
        "delivery-binding-rule",
        "--write",
    ]);
    assert_eq!(code, 0, "{err}");
    assert!(out.contains("ID tokens only"), "{out}");
    assert_eq!(
        read(&root, strip),
        format!("per `{FOO_BAR}` D3-delivery-binding-rule as amended\n"),
        "the token is rewritten and the path stays as written"
    );
}

#[test]
fn a_rekey_follows_a_path_qualified_mention() {
    // As `foo-bar` would: the path names the owner for every rewrite that keys on it.
    let root = path_sessions("1c-rekey");
    let strip = ".mochiko/strips/x.md";
    put(&root, strip, &format!("per `{FOO_BAR}` D3 here\n"));
    let plan = rekey_plan(&root, FOO_BAR, "D3", "D4");
    let edit = plan
        .edits
        .iter()
        .find(|e| e.path == Path::new(strip))
        .expect("the strip's path-qualified mention");
    assert_eq!(edit.new, format!("per `{FOO_BAR}` D4 here\n"));
}

#[test]
fn a_path_qualifier_still_counts_toward_names_several() {
    let root = path_sessions("1c-several");
    let of = |line: &str, token: &str| -> Vec<Resolved> {
        resolutions(&root, "DECISIONS.md", line)
            .into_iter()
            .filter(|(text, _)| text == token)
            .map(|(_, resolved)| resolved)
            .collect()
    };
    // (a) a path cite of one session and a link to another: the line names several.
    let a = format!("| D5 per `{FOO_BAR}` D3 and [b](.mochiko/brainstorms/baz-qux/record.md) |\n");
    assert_eq!(of(&a, "D5"), vec![Resolved::Unresolved], "(a)");
    assert_eq!(
        of(&a, "D3"),
        vec![Resolved::Owned(name("foo-bar"))],
        "(a) the path's own mention"
    );
    // (b) Q1 as ruled: a path qualifier never owns its line's bare decisions.
    let b = format!("| D5 per `{FOO_BAR}` D3 |\n");
    assert_eq!(of(&b, "D5"), vec![Resolved::Unresolved], "(b) Q1");
    // (c) the record link and the slug qualifier read as before.
    let c = "| ruled D5 | [record](.mochiko/brainstorms/foo-bar/record.md) |\n";
    assert_eq!(
        of(c, "D5"),
        vec![Resolved::Owned(name("foo-bar"))],
        "(c) B1"
    );
    assert_eq!(
        of("per `foo-bar` D3\n", "D3"),
        vec![Resolved::Owned(name("foo-bar"))],
        "(c) D11"
    );
}

#[test]
fn ranges_lists_and_masked_path_cites_stay_bare() {
    // D5 and D15: the shapes the tool leaves bare stay bare behind a path; one plain cite is the
    // control the rename must reach.
    let root = tree("1c-bare");
    put(
        &root,
        FOO_BAR,
        "### D3-delivery-binding-rule — a\n### D4 — b\n### D5 — c\n### D6 — d\n### D7 — e\n",
    );
    let strip = ".mochiko/strips/x.md";
    let bare = format!(
        "ruled `{FOO_BAR}` D1–D7\n\nruled `{FOO_BAR}` D3 · D4 · D5 · D6\n\n> per `{FOO_BAR}` D3\n\n```\nper `{FOO_BAR}` D3\n```\n\nquoted \"per `{FOO_BAR}` D3 here\"\n"
    );
    put(&root, strip, &format!("{bare}\nper `{FOO_BAR}` D3 here\n"));
    let control = bare.lines().count() + 2;
    assert_eq!(
        findings(&root, &[strip], &[]),
        vec![(strip.into(), control, ids::Kind::Bare, "D3".into())],
        "only the control is reported"
    );
    let plan = rename_plan(&root, FOO_BAR, "D3", "render-binding-cards", &[]);
    let edit = plan
        .edits
        .iter()
        .find(|e| e.path == Path::new(strip))
        .expect("the control line");
    assert_eq!(
        edit.new,
        format!("{bare}\nper `{FOO_BAR}` D3-render-binding-cards here\n")
    );
}
