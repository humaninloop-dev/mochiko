//! The six conformance checks (record D4), the first-touch amnesty, and the `Edit`-in-memory leg.
//!
//! Each check is decidable by string and count, which is what keeps the write-time gate on the
//! admitted side of the bright line: nothing here reads meaning or grades quality (GI-019).
//!
//! Every fixture is written under `CARGO_TARGET_TMPDIR`, inside `target/`.

use mochiko_cli::conform::{self, Decision};
use mochiko_cli::home::Homes;
use mochiko_cli::migration;
use mochiko_cli::replay::{self, State};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn log_dir(tag: &str) -> PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("conform-{tag}-{n}"));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("fixture log dir is creatable");
    dir
}

/// One home with three deliverables — a templated file, a whole-file-bounded one, and an
/// append-only log — plus a reports directory and its envelope.
const LOG: &str = r#"
grammar: 1
id: 0001-conform
sequence: 1
intent: One home and its templates, for the conformance matrix.
changes:
  - op: import-document
    kind: home
    name: feature
    content:
      home: feature
      title: Feature work home
      path: [".mochiko", "features", "<FEAT-ID>"]
      bounds: template
      deliverables:
        - file: spec.md
          template: demo
        - file: gates.md
          max_lines: 6
        - file: implement-log.md
          form: log
          entry_max_lines: 4
      reports:
        envelope: report-envelope
        by_type: {}
  - op: import-document
    kind: home
    name: memory
    content:
      home: memory
      title: Durable memory surfaces
      path: [".mochiko", "memory"]
      bounds: elsewhere
      bounds_cite: .mochiko/memory/knowledge-management.md
      deliverables:
        - file: knowledge-management.md
          template: demo
  - op: import-document
    kind: template
    name: demo
    content:
      template: demo
      title: Demo Artifact
      form: artifact-format.md
      register: full
      overview: A graded artifact.
      conformance:
        frontmatter:
          required: [feature, status]
          enum:
            status: [draft, accepted]
        placeholders: ["[entity]", "FEAT-XXX"]
        extra_headings: deny
      sections:
        - name: Header
          required: true
          contract: The title line. Governs no heading.
          check: Is the header present?
        - name: Intent
          heading: Intent
          required: true
          max_lines: 4
          contract: One line per ruling.
          check: Is the Intent present?
        - name: Notes
          heading: Notes
          required: false
          max_lines: 5
          contract: Optional commentary.
          check: Are the Notes present?
        - name: Examples
          heading: Examples
          required: false
          max_lines: 20
          contract: Fenced examples. Real templates carry these, which is why the fence rule matters.
          check: Are the examples fenced?
      skeleton: |
        ## Intent
  - op: import-document
    kind: template
    name: report-envelope
    content:
      template: report-envelope
      title: Report envelope
      form: report-format.md
      register: full
      overview: The envelope every report opens with.
      conformance:
        frontmatter:
          required: [report, feature]
          enum:
            report: [cycle, review]
        placeholders: ["FEAT-XXX"]
        extra_headings: deny
      sections:
        - name: Findings
          heading: Findings
          required: true
          max_lines: 4
          contract: One line per finding.
          check: Are findings one line each?
      skeleton: |
        ---
        report: cycle
        ---

        ## Findings
"#;

fn state(tag: &str) -> State {
    let dir = log_dir(tag);
    let stamped =
        migration::with_hash("0001-conform.yaml", LOG).expect("the fixture log is well-formed");
    std::fs::write(dir.join("0001-conform.yaml"), stamped).expect("fixture is writable");
    replay::load(&dir).unwrap_or_else(|findings| {
        let lines: Vec<String> = findings.iter().map(ToString::to_string).collect();
        panic!("fixture log replays:\n{}", lines.join("\n"))
    })
}

/// Grade a candidate body for one path, with no file on disk (a new file).
fn grade(state: &State, path: &str, candidate: &str) -> conform::Verdict {
    let homes = Homes::load(state);
    let resolution = homes.resolve(Path::new(path));
    conform::check(state, &resolution, candidate, None)
}

/// Grade a candidate body for one path against a baseline already on disk.
fn regrade(state: &State, path: &str, candidate: &str, baseline: &str) -> conform::Verdict {
    let homes = Homes::load(state);
    let resolution = homes.resolve(Path::new(path));
    conform::check(state, &resolution, candidate, Some(baseline))
}

const SPEC: &str = ".mochiko/features/FEAT-001/spec.md";

fn allowed(verdict: &conform::Verdict) -> bool {
    matches!(verdict.decision, Decision::Allow)
}

fn denied(verdict: &conform::Verdict) -> bool {
    matches!(verdict.decision, Decision::Deny)
}

/// A conforming body: required frontmatter with a listed enum value, both headings in order,
/// each inside its budget, no placeholder surviving.
const CONFORMING: &str = "---\nfeature: FEAT-001\nstatus: draft\n---\n\n# Demo\n\n\
                          ## Intent\n\nScope is the thing.\n\n## Notes\n\nNothing yet.\n";

// ---------------------------------------------------------------------------
// the happy path and the file set
// ---------------------------------------------------------------------------

#[test]
fn a_conforming_new_file_is_allowed_with_no_context() {
    let state = state("clean");
    let verdict = grade(&state, SPEC, CONFORMING);
    assert!(allowed(&verdict), "reason: {:?}", verdict.reason);
    assert!(verdict.context.is_none());
}

#[test]
fn an_undeclared_file_name_is_denied_and_the_reason_names_the_set_and_the_route() {
    let state = state("set");
    let verdict = grade(
        &state,
        ".mochiko/features/FEAT-001/build-order.md",
        CONFORMING,
    );
    assert!(denied(&verdict));
    let reason = verdict.reason.unwrap_or_default();
    for expected in ["build-order.md", "spec.md", "reports/"] {
        assert!(
            reason.contains(expected),
            "the reason names {expected:?}: {reason}"
        );
    }
}

#[test]
fn an_undeclared_name_on_a_file_already_on_disk_is_amnestied_and_names_the_violation() {
    // D4e as ratified at AM-3: the file set is a relaxable measure. A mis-homed file that already
    // exists stays editable, with the violation reported rather than hidden, because a gate that
    // refuses every write to it is a gate that wedges it.
    let state = state("set-amnesty");
    let verdict = regrade(
        &state,
        ".mochiko/features/FEAT-001/build-order.md",
        CONFORMING,
        "# what was already there\n",
    );
    assert!(allowed(&verdict), "reason: {:?}", verdict.reason);
    let context = verdict.context.unwrap_or_default();
    assert!(
        context.contains("build-order.md") && context.contains("not a declared deliverable"),
        "the allow names the standing violation: {context}"
    );
}

#[test]
fn a_backticked_pattern_is_a_quotation_and_not_a_surviving_placeholder() {
    // A report whose subject is the schema names patterns in its own frontmatter. Reading those as
    // surviving placeholders denies the artifact by its own subject matter — the defect this closes,
    // found on a real review report quoting `wave<n>-<slug>.md`.
    let state = state("placeholder-quoted");
    let quoted = "---\nfeature: FEAT-001\nstatus: draft\nscope: the `FEAT-XXX` pattern\n---\n\n\
                  # Demo\n\n## Intent\n\nScope is the thing.\n";
    let verdict = grade(&state, SPEC, quoted);
    assert!(
        allowed(&verdict),
        "a quoted pattern in a frontmatter value is not a placeholder: {:?}",
        verdict.reason
    );

    // The control: the same token unquoted in the same field is a survivor, and still denies.
    let bare = "---\nfeature: FEAT-001\nstatus: draft\nscope: the FEAT-XXX pattern\n---\n\n\
                # Demo\n\n## Intent\n\nScope is the thing.\n";
    let verdict = grade(&state, SPEC, bare);
    assert!(
        denied(&verdict),
        "an unquoted token in a value still denies"
    );
    assert!(verdict.reason.unwrap_or_default().contains("FEAT-XXX"));
}

#[test]
fn a_backticked_pattern_in_a_heading_is_a_quotation_and_its_bare_twin_is_not() {
    // `###` is the producer's to structure, so this exercises the heading half without also
    // tripping the undeclared-`##` check.
    let state = state("placeholder-heading");
    let quoted = "---\nfeature: FEAT-001\nstatus: draft\n---\n\n# Demo\n\n## Intent\n\n\
                  ### The `FEAT-XXX` shape\n";
    let verdict = grade(&state, SPEC, quoted);
    assert!(
        allowed(&verdict),
        "a quoted pattern in heading text is not a placeholder: {:?}",
        verdict.reason
    );

    let bare = "---\nfeature: FEAT-001\nstatus: draft\n---\n\n# Demo\n\n## Intent\n\n\
                ### The FEAT-XXX shape\n";
    assert!(
        denied(&grade(&state, SPEC, bare)),
        "an unquoted token in heading text still denies"
    );
}

#[test]
fn a_lone_backtick_cannot_hide_a_placeholder_behind_it() {
    // An unterminated run is literal text per CommonMark, so its tail is still read. Without this
    // the skip would buy a false allow: one stray backtick would blank the rest of the value.
    let state = state("placeholder-unpaired");
    let body =
        "---\nfeature: FEAT-001\nstatus: draft\nscope: a stray ` then FEAT-XXX bare\n---\n\n\
                # Demo\n\n## Intent\n\nScope is the thing.\n";
    assert!(
        denied(&grade(&state, SPEC, body)),
        "an unpaired backtick is literal text, not an opener that swallows the rest"
    );
}

#[test]
fn an_undeclared_subdir_is_denied() {
    let state = state("subdir");
    let verdict = grade(
        &state,
        ".mochiko/features/FEAT-001/prototype/index.md",
        CONFORMING,
    );
    assert!(denied(&verdict));
    assert!(verdict.reason.unwrap_or_default().contains("prototype"));
}

#[test]
fn a_path_outside_every_home_is_allowed_by_the_in_home_check() {
    let state = state("outside");
    assert!(allowed(&grade(&state, "docs/notes.md", "# anything\n")));
}

// ---------------------------------------------------------------------------
// frontmatter
// ---------------------------------------------------------------------------

#[test]
fn a_missing_required_frontmatter_field_is_denied() {
    let state = state("fm-missing");
    let body = CONFORMING.replace("status: draft\n", "");
    let verdict = grade(&state, SPEC, &body);
    assert!(denied(&verdict));
    assert!(verdict.reason.unwrap_or_default().contains("status"));
}

#[test]
fn an_unlisted_enum_value_is_denied_and_the_reason_lists_the_allowed_ones() {
    let state = state("fm-enum");
    let body = CONFORMING.replace("status: draft", "status: halfway");
    let verdict = grade(&state, SPEC, &body);
    assert!(denied(&verdict));
    let reason = verdict.reason.unwrap_or_default();
    assert!(reason.contains("halfway"), "{reason}");
    assert!(reason.contains("draft"), "{reason}");
    assert!(reason.contains("accepted"), "{reason}");
}

#[test]
fn a_body_with_no_frontmatter_at_all_is_denied_when_fields_are_required() {
    let state = state("fm-none");
    let verdict = grade(&state, SPEC, "# Demo\n\n## Intent\n\nx\n");
    assert!(denied(&verdict));
}

// ---------------------------------------------------------------------------
// headings
// ---------------------------------------------------------------------------

#[test]
fn a_missing_required_heading_is_denied() {
    let state = state("head-missing");
    let body = "---\nfeature: FEAT-001\nstatus: draft\n---\n\n# Demo\n\n## Notes\n\nx\n";
    let verdict = grade(&state, SPEC, body);
    assert!(denied(&verdict));
    assert!(verdict.reason.unwrap_or_default().contains("## Intent"));
}

#[test]
fn declared_headings_out_of_declaration_order_are_denied() {
    let state = state("head-order");
    let body = "---\nfeature: FEAT-001\nstatus: draft\n---\n\n\
                ## Notes\n\nx\n\n## Intent\n\ny\n";
    let verdict = grade(&state, SPEC, body);
    assert!(denied(&verdict));
    assert!(verdict.reason.unwrap_or_default().contains("order"));
}

#[test]
fn an_undeclared_heading_is_denied_under_the_default_posture() {
    let state = state("head-extra");
    let body = format!("{CONFORMING}\n## EPIC-002 cycles\n\nappended\n");
    let verdict = grade(&state, SPEC, &body);
    assert!(
        denied(&verdict),
        "this is the F10 drift the gate exists for"
    );
    assert!(verdict
        .reason
        .unwrap_or_default()
        .contains("## EPIC-002 cycles"));
}

#[test]
fn nested_headings_are_free_and_an_absent_optional_heading_is_allowed() {
    let state = state("head-nested");
    // `## Notes` is optional and absent; `### Any depth` is the producer's to structure. The whole
    // `## Intent` span is 3 lines, inside its budget of 4, so nothing but the heading rule is
    // under test here.
    let body = "---\nfeature: FEAT-001\nstatus: draft\n---\n\n\
                ## Intent\n### Any depth\nfine\n";
    let verdict = grade(&state, SPEC, body);
    assert!(allowed(&verdict), "reason: {:?}", verdict.reason);
}

#[test]
fn a_heading_inside_a_fenced_code_block_is_not_a_heading() {
    let state = state("head-fence");
    // Four lines from `## Intent` to EOF: inside the budget, so only the fence rule is under test.
    let fenced = "---\nfeature: FEAT-001\nstatus: draft\n---\n\n\
                  ## Intent\n```\n## Invented\n```\n";
    let verdict = grade(&state, SPEC, fenced);
    assert!(
        allowed(&verdict),
        "a fenced example must not read as an undeclared heading: {:?}",
        verdict.reason
    );

    // The control: the same heading text outside a fence *is* an undeclared heading. Without this
    // leg the test above would pass just as well on a checker that ignored headings entirely.
    let unfenced = "---\nfeature: FEAT-001\nstatus: draft\n---\n\n\
                    ## Intent\nx\n\n## Invented\n";
    let verdict = grade(&state, SPEC, unfenced);
    assert!(denied(&verdict));
    assert!(verdict.reason.unwrap_or_default().contains("## Invented"));
}

// ---------------------------------------------------------------------------
// placeholders
// ---------------------------------------------------------------------------

#[test]
fn a_placeholder_token_in_a_frontmatter_value_is_denied() {
    let state = state("ph-fm");
    let body = CONFORMING.replace("feature: FEAT-001", "feature: FEAT-XXX");
    let verdict = grade(&state, SPEC, &body);
    assert!(denied(&verdict));
    assert!(verdict.reason.unwrap_or_default().contains("FEAT-XXX"));
}

#[test]
fn a_placeholder_token_in_heading_text_is_denied() {
    let state = state("ph-head");
    // The token goes into a `###` sub-heading: inside D4c's scope, but not a `##` span, so the
    // heading rules raise nothing and the placeholder rule is the only thing that can deny.
    // Round 1's G4(b): the earlier version mutated `## Notes` into an *undeclared* heading, so the
    // heading fault fired first and the row passed on a rule it was not named for.
    let body = CONFORMING.replace("Nothing yet.", "### Notes for [entity]");
    let verdict = grade(&state, SPEC, &body);
    assert!(denied(&verdict));
    let reason = verdict.reason.unwrap_or_default();
    assert!(
        reason.contains("placeholder token `[entity]`"),
        "the deny must be the placeholder rule, not a heading rule: {reason}"
    );
}

#[test]
fn a_placeholder_token_inside_a_fenced_block_is_allowed() {
    let state = state("ph-fence");
    // Round 1's G2, the reviewer's reproduction: a shell fence carrying a `#` comment that names a
    // placeholder token. The line is neither a frontmatter value nor a heading, so it must allow —
    // and a false deny here stops honest work, where a false allow only misses drift.
    let fenced = format!(
        "{CONFORMING}\n## Examples\n\n```sh\n# FEAT-XXX is the pattern\necho '[entity]'\n```\n"
    );
    let verdict = grade(&state, SPEC, &fenced);
    assert!(
        allowed(&verdict),
        "a token inside a fenced example is not heading text: {:?}",
        verdict.reason
    );

    // The control: the same token in a real `###` heading denies, so the test above cannot pass on
    // a checker that stopped looking at headings altogether.
    let unfenced = CONFORMING.replace("Nothing yet.", "### FEAT-XXX");
    assert!(denied(&grade(&state, SPEC, &unfenced)));
}

#[test]
fn a_hash_that_is_not_a_second_or_third_level_heading_is_not_heading_text() {
    let state = state("ph-levels");
    // Plan §3.7 scopes the check to `##` and `###`. A `#` title and a `####` sub-sub-heading are
    // outside it, and `##text` with no space is not a heading at all.
    for line in ["# FEAT-XXX", "#### FEAT-XXX", "##FEAT-XXX"] {
        let body = CONFORMING.replace("Nothing yet.", line);
        assert!(
            allowed(&grade(&state, SPEC, &body)),
            "{line:?} is outside the declared placeholder scope"
        );
    }
}

#[test]
fn a_placeholder_spelling_in_body_prose_is_allowed() {
    let state = state("ph-body");
    let body = CONFORMING.replace(
        "Scope is the thing.",
        "Feature ids are written FEAT-XXX and entities as [entity] in prose.",
    );
    assert!(
        allowed(&grade(&state, SPEC, &body)),
        "honest prose names the pattern; D4c scopes the check to frontmatter and headings"
    );
}

// ---------------------------------------------------------------------------
// size
// ---------------------------------------------------------------------------

#[test]
fn a_section_one_line_over_its_budget_is_denied_and_the_reason_names_both_numbers() {
    let state = state("size-over");
    // `## Intent` budget is 4, counted from the `##` line to the next `##`.
    let body = "---\nfeature: FEAT-001\nstatus: draft\n---\n\n\
                ## Intent\na\nb\nc\nd\n\n## Notes\n\nx\n";
    let verdict = grade(&state, SPEC, body);
    assert!(denied(&verdict));
    let reason = verdict.reason.unwrap_or_default();
    assert!(reason.contains("## Intent"), "{reason}");
    assert!(reason.contains('4'), "the budget is named: {reason}");
}

#[test]
fn nested_content_counts_toward_its_sections_budget() {
    let state = state("size-nested");
    let body = "---\nfeature: FEAT-001\nstatus: draft\n---\n\n\
                ## Intent\n### deeper\na\nb\nc\n";
    assert!(
        denied(&grade(&state, SPEC, body)),
        "nesting under `###` is not an escape from the parent's budget (D4a)"
    );
}

#[test]
fn a_whole_file_bound_applies_to_a_template_less_deliverable() {
    let state = state("size-whole");
    let path = ".mochiko/features/FEAT-001/gates.md";
    assert!(allowed(&grade(&state, path, "a\nb\nc\n")));
    let verdict = grade(&state, path, "a\nb\nc\nd\ne\nf\ng\n");
    assert!(denied(&verdict));
    assert!(verdict.reason.unwrap_or_default().contains('6'));
}

#[test]
fn an_append_only_log_is_bounded_per_entry_never_per_file() {
    let state = state("size-log");
    let path = ".mochiko/features/FEAT-001/implement-log.md";
    // Six entries, each inside the per-entry bound of 4: allowed however long the file grows.
    let mut long = String::new();
    for n in 1..=6 {
        long.push_str(&format!("## 2026-09-{n:02}\n\nline\n\n"));
    }
    assert!(
        allowed(&grade(&state, path, &long)),
        "a log has no whole-file bound (D4d)"
    );
    // One entry over the bound: denied.
    let fat = "## 2026-09-01\na\nb\nc\nd\n";
    assert!(denied(&grade(&state, path, fat)));
}

#[test]
fn a_home_whose_bounds_live_elsewhere_takes_no_size_check() {
    let state = state("size-elsewhere");
    // The operating-docs case: location and set only, the constraint home cited. The `memory` home
    // is declared `bounds: elsewhere` and binds the same `demo` template whose `## Intent` budget
    // is 4, so this body is **six times over** a budget that would otherwise deny it — only the
    // exemption can make this pass. Round 1's G4(a): the earlier version of this row graded a
    // 3-line body in a `bounds: template` home, so it would have passed with the branch deleted.
    let mut over = String::from("---\nfeature: FEAT-001\nstatus: draft\n---\n\n## Intent\n");
    for n in 0..24 {
        over.push_str(&format!("line {n}\n"));
    }
    let path = ".mochiko/memory/knowledge-management.md";
    let verdict = grade(&state, path, &over);
    assert!(
        allowed(&verdict),
        "size is not checked at all under `bounds: elsewhere`: {:?}",
        verdict.reason
    );

    // The control: the identical body at a `bounds: template` home's templated file denies on size,
    // which is what proves the exemption is doing the work above rather than the body being small.
    assert!(
        denied(&grade(&state, SPEC, &over)),
        "the same body must deny where the bounds are the template's"
    );

    // And the file set still binds under the exemption — location and set only, never nothing.
    assert!(
        denied(&grade(&state, ".mochiko/memory/invented.md", "x\n")),
        "`bounds: elsewhere` exempts size, not the closed file set"
    );
}

// ---------------------------------------------------------------------------
// first-touch amnesty (D4e / C3 / V3)
// ---------------------------------------------------------------------------

/// A baseline already over the `## Intent` budget of 4.
const OVER_BASELINE: &str = "---\nfeature: FEAT-001\nstatus: draft\n---\n\n\
                             ## Intent\na\nb\nc\nd\ne\nf\n";

#[test]
fn a_new_file_over_budget_takes_the_budget_outright() {
    let state = state("amnesty-new");
    assert!(
        denied(&grade(&state, SPEC, OVER_BASELINE)),
        "a new file at a declared name has no history to be amnestied by"
    );
}

#[test]
fn a_write_that_does_not_worsen_a_standing_overage_is_allowed_with_context() {
    let state = state("amnesty-same");
    let verdict = regrade(&state, SPEC, OVER_BASELINE, OVER_BASELINE);
    assert!(allowed(&verdict), "reason: {:?}", verdict.reason);
    let context = verdict.context.unwrap_or_default();
    assert!(
        context.contains("## Intent"),
        "the standing overage is reported, not hidden: {context}"
    );
}

#[test]
fn a_write_that_worsens_a_standing_overage_is_denied() {
    let state = state("amnesty-worse");
    let worse = format!("{OVER_BASELINE}g\n");
    assert!(denied(&regrade(&state, SPEC, &worse, OVER_BASELINE)));
}

#[test]
fn a_write_that_improves_a_standing_overage_is_allowed() {
    let state = state("amnesty-better");
    let better = OVER_BASELINE.replace("e\nf\n", "");
    let verdict = regrade(&state, SPEC, &better, OVER_BASELINE);
    assert!(
        allowed(&verdict),
        "the rewrite that fixes a file must itself be allowed: {:?}",
        verdict.reason
    );
}

#[test]
fn amnesty_covers_a_standing_undeclared_heading_but_not_a_new_one() {
    let state = state("amnesty-heading");
    let baseline = format!("{CONFORMING}\n## Standing extra\n\nx\n");
    // Rewriting the file while the standing extra heading survives: allowed.
    let verdict = regrade(&state, SPEC, &baseline, &baseline);
    assert!(allowed(&verdict), "reason: {:?}", verdict.reason);
    // Adding a second undeclared heading is a new fault, not the standing one.
    let worse = format!("{baseline}\n## Another extra\n\ny\n");
    assert!(denied(&regrade(&state, SPEC, &worse, &baseline)));
}

#[test]
fn amnesty_applies_to_the_write_leg_and_the_edit_leg_alike() {
    let state = state("amnesty-both-legs");
    // The `Write` leg: `candidate` is the whole new body.
    assert!(allowed(&regrade(
        &state,
        SPEC,
        OVER_BASELINE,
        OVER_BASELINE
    )));
    // The `Edit` leg reaches the same function with the applied result as `candidate`, so the
    // two legs cannot diverge by construction — this asserts the shared contract holds.
    let applied = conform::apply_edit(OVER_BASELINE, "a\nb\n", "a\n")
        .expect("the edit applies to the baseline");
    let verdict = regrade(&state, SPEC, &applied, OVER_BASELINE);
    assert!(allowed(&verdict), "reason: {:?}", verdict.reason);
}

// ---------------------------------------------------------------------------
// the Edit-in-memory leg
// ---------------------------------------------------------------------------

#[test]
fn an_edit_is_graded_on_its_applied_result() {
    let state = state("edit-applied");
    let baseline = CONFORMING;
    // An append that adds an undeclared `##` is denied before it lands.
    let applied = conform::apply_edit(
        baseline,
        "## Notes\n\nNothing yet.\n",
        "## Notes\n\nNothing yet.\n\n## EPIC-002 cycles\n\nappended\n",
    )
    .expect("the edit applies");
    assert!(denied(&regrade(&state, SPEC, &applied, baseline)));
}

#[test]
fn an_edit_whose_old_string_is_absent_does_not_apply() {
    assert!(conform::apply_edit("a\nb\n", "nowhere", "x").is_none());
}

#[test]
fn an_edit_replaces_only_the_first_occurrence() {
    let applied = conform::apply_edit("x\nx\n", "x", "y").expect("applies");
    assert_eq!(applied, "y\nx\n");
}

// ---------------------------------------------------------------------------
// reports: names free, envelope binding
// ---------------------------------------------------------------------------

#[test]
fn any_name_under_reports_is_allowed_when_the_envelope_holds() {
    let state = state("report-ok");
    // The payload conforms to the envelope, so the file *name* is the only thing under test here.
    let body = "---\nreport: cycle\nfeature: FEAT-001\n---\n\n## Findings\n\nOne line.\n";
    assert!(allowed(&grade(
        &state,
        ".mochiko/features/FEAT-001/reports/whatever-name.md",
        body
    )));
}

#[test]
fn a_report_carrying_an_unlisted_type_is_denied() {
    let state = state("report-type");
    // Payload conforming, so only the type is wrong.
    let body = "---\nreport: invented\nfeature: FEAT-001\n---\n\n## Findings\n\nOne line.\n";
    let verdict = grade(&state, ".mochiko/features/FEAT-001/reports/x.md", body);
    assert!(denied(&verdict));
    assert!(verdict.reason.unwrap_or_default().contains("invented"));
}

#[test]
fn the_envelopes_own_headings_budgets_and_placeholders_apply_to_a_report() {
    let state = state("report-envelope-sections");
    let path = ".mochiko/features/FEAT-001/reports/round-1.md";
    let head = "---\nreport: cycle\nfeature: FEAT-001\n---\n";

    // The envelope declares `## Findings` required, budget 4, and `FEAT-XXX` as a placeholder.
    // Round 1's G10: only the frontmatter half was applied, so none of these three could deny.
    assert!(allowed(&grade(
        &state,
        path,
        &format!("{head}\n## Findings\n\nOne line.\n")
    )));

    let missing = grade(&state, path, &format!("{head}\nBody with no heading.\n"));
    assert!(denied(&missing), "the envelope's required heading binds");
    assert!(missing.reason.unwrap_or_default().contains("## Findings"));

    let over = grade(
        &state,
        path,
        &format!("{head}\n## Findings\na\nb\nc\nd\ne\n"),
    );
    assert!(denied(&over), "the envelope's section budget binds");

    let extra = grade(
        &state,
        path,
        &format!("{head}\n## Findings\n\nx\n\n## Invented\n"),
    );
    assert!(denied(&extra), "an undeclared heading in a report binds");

    let token = grade(
        &state,
        path,
        &format!("{head}\n## Findings\n\n### FEAT-XXX\n"),
    );
    assert!(denied(&token), "the envelope's placeholder tokens bind");
}

#[test]
fn a_report_deny_reason_names_the_file_and_its_home() {
    let state = state("report-reason-names");
    // Round 1's G9: the two discarded parameters now carry the home and the file into the reason,
    // so a denied seat is not left hunting for which report under which run tripped.
    let verdict = grade(
        &state,
        ".mochiko/features/FEAT-001/reports/round-7.md",
        "---\nreport: invented\nfeature: FEAT-001\n---\n\n## Findings\n\nx\n",
    );
    assert!(denied(&verdict));
    let reason = verdict.reason.unwrap_or_default();
    assert!(reason.contains("round-7.md"), "{reason}");
    assert!(reason.contains(".mochiko/features/<FEAT-ID>/"), "{reason}");
}

#[test]
fn a_report_missing_its_envelope_is_denied() {
    let state = state("report-bare");
    let verdict = grade(
        &state,
        ".mochiko/features/FEAT-001/reports/x.md",
        "# No envelope\n",
    );
    assert!(denied(&verdict));
}

// ---------------------------------------------------------------------------
// the D9 frontmatter sniff, outside every home
// ---------------------------------------------------------------------------

#[test]
fn a_report_smuggled_outside_every_home_is_denied_by_the_sniff() {
    let state = state("sniff-hit");
    let body = "---\nreport: cycle\nfeature: FEAT-001\n---\n\nBody.\n";
    let verdict = conform::sniff(&state, "docs/cycle-report.md", body);
    assert!(denied(&verdict), "a report belongs under a home's reports/");
}

#[test]
fn a_plain_markdown_file_outside_every_home_is_not_mochikos_business() {
    let state = state("sniff-miss");
    for body in [
        "# Product docs\n\nNothing to do with mochiko.\n",
        "---\ntitle: A blog post\n---\n\nBody.\n",
        "---\nreport: not-in-the-enum\n---\n\nBody.\n",
    ] {
        assert!(
            allowed(&conform::sniff(&state, "docs/page.md", body)),
            "the sniff fires only on an enumerated report type: {body}"
        );
    }
}

#[test]
fn every_deny_reason_closes_with_the_advisory_halt_sentence() {
    let state = state("halt-sentence");
    let verdict = grade(
        &state,
        ".mochiko/features/FEAT-001/build-order.md",
        CONFORMING,
    );
    let reason = verdict.reason.unwrap_or_default();
    assert!(
        reason.contains("a second deny on this path halts"),
        "D9's advisory sentence closes every reason: {reason}"
    );
}
