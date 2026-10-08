//! The six conformance checks (record D4), the first-touch amnesty, and the `Edit`-in-memory leg.
//!
//! Each check is decidable by string and count, which is what keeps the write-time gate on the
//! admitted side of the bright line: nothing here reads meaning or grades quality (GI-019-kernel-tooling-admission).
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
    grade_routed(state, path, candidate, None)
}

/// [`grade`], with the run folder a raw-output home would name.
fn grade_routed(
    state: &State,
    path: &str,
    candidate: &str,
    run_folder: Option<&str>,
) -> conform::Verdict {
    let homes = Homes::load(state);
    let resolution = homes.resolve(Path::new(path));
    conform::check(state, &resolution, candidate, None, run_folder)
}

/// Grade a candidate body for one path against a baseline already on disk.
fn regrade(state: &State, path: &str, candidate: &str, baseline: &str) -> conform::Verdict {
    let homes = Homes::load(state);
    let resolution = homes.resolve(Path::new(path));
    conform::check(state, &resolution, candidate, Some(baseline), None)
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
    // D4e as ratified at AM-3-conformance-gate-admission: the file set is a relaxable measure. A mis-homed file that already
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

// ---------------------------------------------------------------------------
// the closed world's two routes (field review D3)
// ---------------------------------------------------------------------------

/// The run folder a raw-output home names, as the hook would pass it.
const RUN_FOLDER: &str = "/repo/.mochiko/runs/<run-id>/";

fn reason_of(verdict: conform::Verdict) -> String {
    assert!(denied(&verdict), "expected a deny");
    verdict.reason.unwrap_or_default()
}

#[test]
fn an_undeclared_subdir_reason_names_both_routes_and_no_longer_sends_the_seat_upstream() {
    let state = state("routes-subdir");
    let reason = reason_of(grade_routed(
        &state,
        ".mochiko/features/FEAT-001/evidence/console.log",
        "raw\n",
        Some(RUN_FOLDER),
    ));
    // The opener the contract suite keys on stays (`evals/contract/run.py:4311`).
    assert!(reason.contains("not a declared sub-directory"), "{reason}");
    // Route 1: the resolved home is the nearest one, and it opens a reports directory.
    assert!(reason.contains(".mochiko/features/<FEAT-ID>/"), "{reason}");
    assert!(
        reason.contains(".mochiko/features/<FEAT-ID>/reports/"),
        "{reason}"
    );
    // Route 2: the run folder by absolute path, stated as ephemeral.
    assert!(reason.contains(RUN_FOLDER), "{reason}");
    assert!(reason.contains("ephemeral"), "{reason}");
    // D3's last sentence: the sentence the lead booked (F12) is gone.
    assert!(!reason.contains("takes a migration"), "{reason}");
}

#[test]
fn an_undeclared_file_reason_names_both_routes_and_keeps_its_opener() {
    let state = state("routes-file");
    let reason = reason_of(grade_routed(
        &state,
        ".mochiko/features/FEAT-001/console.log",
        "raw\n",
        Some(RUN_FOLDER),
    ));
    // The opener the contract suite keys on (`evals/contract/run.py:4314`, `:4357`, `:4848`).
    assert!(reason.contains("not a declared deliverable"), "{reason}");
    for expected in [
        "spec.md",
        ".mochiko/features/<FEAT-ID>/reports/",
        RUN_FOLDER,
        "ephemeral",
    ] {
        assert!(reason.contains(expected), "names {expected:?}: {reason}");
    }
}

#[test]
fn a_log_that_declares_no_raw_output_home_names_no_run_folder() {
    // Today's log declares none: the route is omitted, never invented.
    let state = state("routes-no-run-folder");
    let reason = reason_of(grade(
        &state,
        ".mochiko/features/FEAT-001/evidence/console.log",
        "raw\n",
    ));
    assert!(reason.contains(".mochiko/features/<FEAT-ID>/"), "{reason}");
    assert!(!reason.contains("run folder"), "{reason}");
    assert!(!reason.contains("ephemeral"), "{reason}");
}

#[test]
fn a_path_under_mochiko_in_no_home_is_refused_and_names_both_routes() {
    let state = state("closed-world");
    let homes = Homes::load(&state);
    let reason = reason_of(conform::closed_world(
        &homes,
        Path::new(".mochiko/evidence/console.log"),
        Some(RUN_FOLDER),
    ));
    assert!(reason.contains("a write here is refused"), "{reason}");
    // No declared home shares more than `.mochiko/` with this path, so route 1 says so and points
    // at the render that lists every home.
    assert!(
        reason.contains("No declared home shares this path's prefix"),
        "{reason}"
    );
    assert!(
        reason.contains("mochiko-cli home .mochiko/evidence/console.log"),
        "{reason}"
    );
    assert!(reason.contains(RUN_FOLDER), "{reason}");
    assert!(reason.contains("ephemeral"), "{reason}");
}

#[test]
fn a_closed_world_refusal_names_the_home_sharing_the_longest_prefix() {
    // The fixture log carries no `.mochiko/features/` index home, so `notanid/` resolves to no
    // home at all; the feature home shares two segments with it and is the nearest.
    let state = state("closed-world-nearest");
    let homes = Homes::load(&state);
    let reason = reason_of(conform::closed_world(
        &homes,
        Path::new(".mochiko/features/notanid/x.md"),
        Some(RUN_FOLDER),
    ));
    assert!(
        reason.contains("the nearest home, `.mochiko/features/<FEAT-ID>/`"),
        "{reason}"
    );
    assert!(
        reason.contains(".mochiko/features/<FEAT-ID>/reports/"),
        "{reason}"
    );
}

// ---------------------------------------------------------------------------
// the per-entry budget kind (field review D2, OQ1; grammar 2)
// ---------------------------------------------------------------------------

/// A product-baseline home whose stores are bounded per entry: a `###`-entry store with a bound
/// on its sections' own text and two exempt marker fields, a `##`-entry store, and a `###`-entry
/// store bound to a template whose shape rules still run.
const ENTRIES_LOG: &str = r####"
grammar: 2
id: 0001-entries
sequence: 1
intent: Entry-bounded stores, for the per-entry budget kind.
changes:
  - op: import-document
    kind: home
    name: product
    content:
      home: product
      title: Product baselines
      path: [".mochiko", "product"]
      bounds: whole-file
      deliverables:
        - file: data-model.md
          form: entries
          entry_heading: "###"
          entry_max_lines: 4
          section_max_lines: 3
          entry_exempt_fields: [Lifecycle, Raised]
        - file: decisions.md
          form: entries
          entry_heading: "##"
          entry_max_lines: 4
        - file: spine.md
          form: entries
          entry_heading: "###"
          entry_max_lines: 4
          template: store-shape
  - op: import-document
    kind: template
    name: store-shape
    content:
      template: store-shape
      title: A store with a shape
      form: artifact-format.md
      register: full
      overview: A store whose frontmatter and headings are declared.
      conformance:
        frontmatter:
          required: [store]
        extra_headings: deny
      sections:
        - name: Entities
          heading: Entities
          required: true
          max_lines: 2
          contract: One entry per entity.
          check: Is every entity an entry?
      skeleton: |
        ## Entities
"####;

fn entries_state(tag: &str) -> State {
    let dir = log_dir(tag);
    let stamped = migration::with_hash("0001-entries.yaml", ENTRIES_LOG)
        .expect("the entries fixture log is well-formed");
    std::fs::write(dir.join("0001-entries.yaml"), stamped).expect("fixture is writable");
    replay::load(&dir).unwrap_or_else(|findings| {
        let lines: Vec<String> = findings.iter().map(ToString::to_string).collect();
        panic!("entries fixture log replays:\n{}", lines.join("\n"))
    })
}

const DATA_MODEL: &str = ".mochiko/product/data-model.md";

#[test]
fn a_third_level_entry_is_bounded_on_its_own_and_ends_at_the_next_second_level_heading() {
    let state = entries_state("entries-level-3");
    let fits = "# Data model\n\n## Entities\nintro\n\n### User\na\nb\nc\n### Order\na\n\
                ## Relations\nx\n## More\n### Line\nb\nc\nd\n";
    assert!(
        allowed(&grade(&state, DATA_MODEL, fits)),
        "{:?}",
        grade(&state, DATA_MODEL, fits).reason
    );

    // `### User` stops at `## Relations`: were Relations' lines counted into it, it would be 6.
    let boundary = "## Entities\n### User\na\nb\nc\n## Relations\nx\ny\n";
    assert!(allowed(&grade(&state, DATA_MODEL, boundary)));

    let over = "## Entities\n### User\na\nb\nc\nd\n";
    let reason = grade(&state, DATA_MODEL, over).reason.unwrap_or_default();
    assert!(
        reason.contains("`### User`") && reason.contains("5 lines") && reason.contains("of 4"),
        "{reason}"
    );
}

#[test]
fn an_entry_store_carries_no_whole_file_bound() {
    let state = entries_state("entries-no-file-cap");
    let mut body = String::from("## Entities\n");
    for n in 0..60 {
        body.push_str(&format!("### Entity {n}\nfield\n\n"));
    }
    assert!(
        allowed(&grade(&state, DATA_MODEL, &body)),
        "D2: a fold that adds entries conforms"
    );
}

#[test]
fn a_heading_inside_a_fence_does_not_end_an_entry() {
    let state = entries_state("entries-fence");
    // Counted whole, `### User` is 5 lines; read as a boundary, the fenced line would split it
    // into two entries of 2 and 3, both inside the bound of 4.
    let body = "## Entities\n### User\n```\n### not a heading\nline\n```\n";
    let reason = grade(&state, DATA_MODEL, body).reason.unwrap_or_default();
    assert!(reason.contains("`### User` is 5 lines"), "{reason}");
}

#[test]
fn exempt_marker_lines_are_not_counted_and_other_fields_are() {
    let state = entries_state("entries-exempt");
    // Both spellings of an exempt field: a bare `**Name:**` line and a list item (the list-item
    // form is the plan's disclosed interpretation of seam 4).
    let exempt = "## Entities\n### User\n**Lifecycle:** proposed (FEAT-001-run5)\n\
                  - **Raised:** C3\na\nb\nc\n";
    assert!(
        allowed(&grade(&state, DATA_MODEL, exempt)),
        "{:?}",
        grade(&state, DATA_MODEL, exempt).reason
    );
    let counted = "## Entities\n### User\n**Weighed:** one line\na\nb\nc\n";
    assert!(
        denied(&grade(&state, DATA_MODEL, counted)),
        "`Weighed` is not in this store's exempt list"
    );
}

#[test]
fn a_sections_own_text_outside_its_entries_takes_the_section_bound() {
    let state = entries_state("entries-section");
    let over = "## Entities\nl1\nl2\nl3\n### User\na\n";
    let reason = grade(&state, DATA_MODEL, over).reason.unwrap_or_default();
    assert!(
        reason.contains("`## Entities`")
            && reason.contains("outside its entries")
            && reason.contains("4 lines"),
        "{reason}"
    );
    assert!(allowed(&grade(
        &state,
        DATA_MODEL,
        "## Entities\nl1\nl2\n### User\na\n"
    )));
}

#[test]
fn a_second_level_entry_counts_its_nested_headings() {
    let state = entries_state("entries-level-2");
    let path = ".mochiko/product/decisions.md";
    assert!(allowed(&grade(
        &state,
        path,
        "# D\n\n## D-001\na\n## D-002\nb\n"
    )));
    let reason = grade(&state, path, "## D-001\na\nb\n### sub\nc\n")
        .reason
        .unwrap_or_default();
    assert!(reason.contains("`## D-001` is 5 lines"), "{reason}");
}

#[test]
fn the_amnesty_applies_per_entry_and_a_renamed_entry_is_a_new_one() {
    let state = entries_state("entries-amnesty");
    let baseline = "## Entities\n### User\na\nb\nc\nd\ne\n";
    // Non-growing: the over-budget entry untouched, a new entry added — allowed, overage named.
    let added = format!("{baseline}### Order\na\n");
    let verdict = regrade(&state, DATA_MODEL, &added, baseline);
    assert!(allowed(&verdict), "{:?}", verdict.reason);
    assert!(verdict.context.unwrap_or_default().contains("### User"));
    // Growing the over-budget entry: denied (V9).
    let grown = "## Entities\n### User\na\nb\nc\nd\ne\nf\n";
    assert!(denied(&regrade(&state, DATA_MODEL, grown, baseline)));
    // Renamed: a new key, so the amnesty does not carry, and the reason says so.
    let renamed = "## Entities\n### Account\na\nb\nc\nd\ne\n";
    let reason = regrade(&state, DATA_MODEL, renamed, baseline)
        .reason
        .unwrap_or_default();
    assert!(reason.contains("renamed"), "{reason}");
}

#[test]
fn an_entry_store_bound_to_a_template_keeps_its_shape_rules_and_drops_its_section_budgets() {
    // Plan Q6, as ruled: the template's frontmatter, heading and placeholder checks run; its
    // per-section `max_lines` (2 here) do not — the entry budgets replace them.
    let state = entries_state("entries-template");
    let path = ".mochiko/product/spine.md";
    let long = "---\nstore: spine\n---\n\n## Entities\n### A\na\n### B\nb\n### C\nc\n";
    assert!(
        allowed(&grade(&state, path, long)),
        "{:?}",
        grade(&state, path, long).reason
    );
    let shapeless = "## Entities\n### A\na\n";
    let reason = grade(&state, path, shapeless).reason.unwrap_or_default();
    assert!(
        reason.contains("`store`"),
        "the frontmatter rule still runs: {reason}"
    );
    let extra = "---\nstore: spine\n---\n\n## Entities\n### A\na\n## Invented\nx\n";
    assert!(
        denied(&grade(&state, path, extra)),
        "the heading rule still runs"
    );
}

// ---------------------------------------------------------------------------
// joint wave 3: a store's preamble (census row P1)
// ---------------------------------------------------------------------------

#[test]
fn a_stores_text_above_its_first_heading_is_bounded_as_one_entry() {
    // Row P1 (b), ratified: the text above a store's first heading was counted by nothing, so a
    // store could grow there without limit. It is bounded at the entry budget, counted from the
    // file's first line as a span of its own.
    let state = entries_state("preamble-bound");
    let decisions = ".mochiko/product/decisions.md";
    let fits = "# D\nl1\nl2\nl3\n## D-001\na\n";
    assert!(
        allowed(&grade(&state, decisions, fits)),
        "{:?}",
        grade(&state, decisions, fits).reason
    );
    let over = "# D\nl1\nl2\nl3\nl4\n## D-001\na\n";
    let reason = grade(&state, decisions, over).reason.unwrap_or_default();
    assert!(
        reason.contains("above the first `##` heading")
            && reason.contains("5 lines")
            && reason.contains("of 4"),
        "{reason}"
    );
    // A stray `###` above a `##` store's first entry is no boundary: it counts into the preamble.
    let stray = "# D\n### stray\nl1\nl2\nl3\n## D-001\na\n";
    assert!(denied(&grade(&state, decisions, stray)));
    // A `###` store's preamble ends at its first `##` or `###`, and its reason says so.
    let store = "# M\nl1\nl2\nl3\nl4\n## Entities\n### User\na\n";
    let reason = grade(&state, DATA_MODEL, store).reason.unwrap_or_default();
    assert!(
        reason.contains("above the first `##` or `###` heading") && reason.contains("5 lines"),
        "{reason}"
    );
    // A file with no heading at all is all preamble.
    assert!(denied(&grade(&state, decisions, "l1\nl2\nl3\nl4\nl5\n")));
}

#[test]
fn a_standing_preamble_overage_is_amnestied_and_a_growing_one_is_denied() {
    let state = entries_state("preamble-amnesty");
    let path = ".mochiko/product/decisions.md";
    let baseline = "# D\nl1\nl2\nl3\nl4\nl5\n## D-001\na\n";
    let verdict = regrade(&state, path, baseline, baseline);
    assert!(allowed(&verdict), "{:?}", verdict.reason);
    let context = verdict.context.unwrap_or_default();
    assert!(
        context.contains("above the first `##` heading"),
        "the standing preamble is named, not hidden: {context}"
    );
    let grown = "# D\nl1\nl2\nl3\nl4\nl5\nl6\n## D-001\na\n";
    assert!(denied(&regrade(&state, path, grown, baseline)));
}

// ---------------------------------------------------------------------------
// joint wave 3: a heading a file repeats (RA3, census row X1)
// ---------------------------------------------------------------------------

/// A report whose `## Findings` repeats, one section per size, each counted from its heading.
fn repeated_findings(sizes: &[usize]) -> String {
    let mut body = String::from("---\nreport: cycle\nfeature: FEAT-001\n---\n\n");
    for size in sizes {
        body.push_str("## Findings\n");
        for n in 1..*size {
            body.push_str(&format!("f{n}\n"));
        }
    }
    body
}

#[test]
fn a_file_repeating_a_heading_keeps_its_amnesty_per_rank() {
    // kinako's driver-fix-report.md repeats `## Notes of note` at 29, 26, 52 and 62 lines, and its
    // unchanged rewrite was denied: every section was compared with the first of its name. Sections
    // sharing a heading are told apart by rank, largest first (Q3, ruled).
    let state = state("amnesty-rank");
    let path = ".mochiko/features/FEAT-001/reports/fix.md";
    let baseline = repeated_findings(&[6, 8]);
    let unchanged = regrade(&state, path, &baseline, &baseline);
    assert!(allowed(&unchanged), "{:?}", unchanged.reason);
    // The larger section grown past its standing size: denied, and the reason names it.
    let grown = regrade(&state, path, &repeated_findings(&[6, 9]), &baseline);
    assert!(denied(&grown));
    let reason = grown.reason.unwrap_or_default();
    assert!(reason.contains("`## Findings` is 9 lines"), "{reason}");
    // A section dropped: allowed.
    assert!(allowed(&regrade(
        &state,
        path,
        &repeated_findings(&[8]),
        &baseline
    )));
    // A third over-budget section under the standing heading is a new fault: denied, by its size.
    let added = regrade(&state, path, &repeated_findings(&[6, 8, 6]), &baseline);
    assert!(denied(&added));
    let reason = added.reason.unwrap_or_default();
    assert!(reason.contains("`## Findings` is 6 lines"), "{reason}");
    // The ruled residual: a swap within one heading lifts no rank above its standing fault.
    assert!(allowed(&regrade(
        &state,
        path,
        &repeated_findings(&[8, 3]),
        &baseline
    )));
}

// ---------------------------------------------------------------------------
// joint wave 3: a home that declares nothing (census row H2)
// ---------------------------------------------------------------------------

/// A capability's `contracts/` home after delta D4 withdrew it: declared, with an empty set.
const EMPTY_HOME_LOG: &str = r#"
grammar: 1
id: 0001-empty
sequence: 1
intent: A withdrawn home, kept with an empty file set.
changes:
  - op: import-document
    kind: home
    name: feature-contracts
    content:
      home: feature-contracts
      title: Interface contracts under a capability — withdrawn by delta D4, see product-contracts
      path: [".mochiko", "features", "<FEAT-ID>", "contracts"]
      bounds: elsewhere
      bounds_cite: the contract's own interface
      deliverables: []
"#;

fn empty_home_state(tag: &str) -> State {
    let dir = log_dir(tag);
    let stamped = migration::with_hash("0001-empty.yaml", EMPTY_HOME_LOG)
        .expect("the empty-home fixture log is well-formed");
    std::fs::write(dir.join("0001-empty.yaml"), stamped).expect("fixture is writable");
    replay::load(&dir).unwrap_or_else(|findings| {
        let lines: Vec<String> = findings.iter().map(ToString::to_string).collect();
        panic!("empty-home fixture log replays:\n{}", lines.join("\n"))
    })
}

#[test]
fn a_home_that_declares_nothing_is_never_named_as_the_nearest_home() {
    // A new file under a withdrawn home was told to write "a declared deliverable of the nearest
    // home" — that same home, which declares none. The route names the home's title instead.
    let state = empty_home_state("empty-home");
    let verdict = grade(
        &state,
        ".mochiko/features/FEAT-001/contracts/new-contract.md",
        "# x\n",
    );
    assert!(denied(&verdict));
    let reason = verdict.reason.unwrap_or_default();
    assert!(
        !reason.contains("the nearest home, `.mochiko/features/<FEAT-ID>/contracts/`"),
        "{reason}"
    );
    assert!(
        reason.contains("declares none") && reason.contains("see product-contracts"),
        "{reason}"
    );
}
