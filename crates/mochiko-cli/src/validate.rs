//! Findings, and the hard set the store's own data must satisfy.
//!
//! Two severities, and the split is a ruling rather than a preference. A [`Severity::Reject`]
//! finding is a structural fact about the store's data — an id that does not resolve, a label
//! outside its registry, a floor rule leaving without a ruling — and the log may never enter a
//! state carrying one, so the replay is valid by construction. A [`Severity::Advisory`] finding
//! is heuristic or coverage-shaped; it prints and exits 0, because a false positive there must
//! never block an edit.
//!
//! Nothing here grades a primitive's judgment content. The checks are structural validity on data
//! this tool owns, which is what keeps the widened kernel-class admission inside the bright line.

use crate::model::{
    canonical_depth, is_anchor, is_dotted_id, is_slug, norm_value, Class, Condition, DocKind,
    DocRef, Document, LabelRegistry, Ordered, Resolution, Rule, RuleKind, RuleSchema, Values,
    MAX_CANONICAL_DEPTH,
};
use crate::replay::State;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

// ---------------------------------------------------------------------------
// findings
// ---------------------------------------------------------------------------

/// Whether a finding stops the log or merely reports.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Severity {
    /// Rejects: exit 1, and the state is never rendered from.
    Reject,
    /// Reports: exit 0.
    Advisory,
}

/// Every finding code the binary can raise. The code is the stable handle a test asserts on, so
/// message wording can improve without breaking the suite.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Code {
    // grammar and log level
    GrammarParse,
    GrammarHeader,
    GrammarVersion,
    SequenceCollision,
    SequenceMismatch,
    HashMismatch,
    OpUnknown,
    OpMalformed,
    OpInapplicable,
    LogFileName,
    // document level
    KindDiscriminator,
    SectionSet,
    IdFormat,
    IdPrefix,
    IdDuplicate,
    MintOnce,
    TombstoneIntegrity,
    LabelUnknown,
    VarUnbound,
    ExtendsUnresolved,
    ExtendsCrossFamily,
    ExtendsClassLocal,
    WhenUndeclared,
    WhenValue,
    ConditionDeclaration,
    MomentUndeclared,
    EnforcesUnresolved,
    EnforcesRequired,
    EnforcesMisplaced,
    FailSegment,
    SkillGrammar,
    ClassUnknown,
    RuleKindUnknown,
    TextMissing,
    ProtectedExit,
    AnchorFormat,
    DepthExceeded,
    // advisory
    Deixis,
    UnusedVar,
    UnusedCondition,
    UnusedMoment,
    EnforcesCoverage,
    ConditionCoverage,
    Budget,
}

impl Code {
    pub fn as_str(self) -> &'static str {
        match self {
            Code::GrammarParse => "grammar-parse",
            Code::GrammarHeader => "grammar-header",
            Code::GrammarVersion => "grammar-version",
            Code::SequenceCollision => "sequence-collision",
            Code::SequenceMismatch => "sequence-mismatch",
            Code::HashMismatch => "hash-mismatch",
            Code::OpUnknown => "op-unknown",
            Code::OpMalformed => "op-malformed",
            Code::OpInapplicable => "op-inapplicable",
            Code::LogFileName => "log-file-name",
            Code::KindDiscriminator => "kind-discriminator",
            Code::SectionSet => "section-set",
            Code::IdFormat => "id-format",
            Code::IdPrefix => "id-prefix",
            Code::IdDuplicate => "id-duplicate",
            Code::MintOnce => "mint-once",
            Code::TombstoneIntegrity => "tombstone-integrity",
            Code::LabelUnknown => "label-unknown",
            Code::VarUnbound => "var-unbound",
            Code::ExtendsUnresolved => "extends-unresolved",
            Code::ExtendsCrossFamily => "extends-cross-family",
            Code::ExtendsClassLocal => "extends-class-local",
            Code::WhenUndeclared => "when-undeclared",
            Code::WhenValue => "when-value",
            Code::ConditionDeclaration => "condition-declaration",
            Code::MomentUndeclared => "moment-undeclared",
            Code::EnforcesUnresolved => "enforces-unresolved",
            Code::EnforcesRequired => "enforces-required",
            Code::EnforcesMisplaced => "enforces-misplaced",
            Code::FailSegment => "fail-segment",
            Code::SkillGrammar => "skill-grammar",
            Code::ClassUnknown => "class-unknown",
            Code::RuleKindUnknown => "rule-kind-unknown",
            Code::TextMissing => "text-missing",
            Code::ProtectedExit => "protected-exit",
            Code::AnchorFormat => "anchor-format",
            Code::DepthExceeded => "depth-exceeded",
            Code::Deixis => "deixis",
            Code::UnusedVar => "unused-var",
            Code::UnusedCondition => "unused-condition",
            Code::UnusedMoment => "unused-moment",
            Code::EnforcesCoverage => "enforces-coverage",
            Code::ConditionCoverage => "condition-coverage",
            Code::Budget => "budget",
        }
    }

    /// The severity this code always carries. Severity is a property of the check, never of the
    /// occurrence, so a code can never be advisory in one place and blocking in another.
    pub fn severity(self) -> Severity {
        match self {
            Code::Deixis
            | Code::UnusedVar
            | Code::UnusedCondition
            | Code::UnusedMoment
            | Code::EnforcesCoverage
            | Code::ConditionCoverage
            | Code::Budget => Severity::Advisory,
            _ => Severity::Reject,
        }
    }

    /// The rejecting codes, so a test can assert every one has a probe.
    pub const REJECTING: [Code; 37] = [
        Code::GrammarParse,
        Code::GrammarHeader,
        Code::GrammarVersion,
        Code::SequenceCollision,
        Code::SequenceMismatch,
        Code::HashMismatch,
        Code::OpUnknown,
        Code::OpMalformed,
        Code::OpInapplicable,
        Code::LogFileName,
        Code::KindDiscriminator,
        Code::SectionSet,
        Code::IdFormat,
        Code::IdPrefix,
        Code::IdDuplicate,
        Code::MintOnce,
        Code::TombstoneIntegrity,
        Code::LabelUnknown,
        Code::VarUnbound,
        Code::ExtendsUnresolved,
        Code::ExtendsCrossFamily,
        Code::ExtendsClassLocal,
        Code::WhenUndeclared,
        Code::WhenValue,
        Code::ConditionDeclaration,
        Code::MomentUndeclared,
        Code::EnforcesUnresolved,
        Code::EnforcesRequired,
        Code::EnforcesMisplaced,
        Code::FailSegment,
        Code::SkillGrammar,
        Code::ClassUnknown,
        Code::RuleKindUnknown,
        Code::TextMissing,
        Code::ProtectedExit,
        Code::AnchorFormat,
        Code::DepthExceeded,
    ];

    /// The advisory codes.
    pub const ADVISORY: [Code; 7] = [
        Code::Deixis,
        Code::UnusedVar,
        Code::UnusedCondition,
        Code::UnusedMoment,
        Code::EnforcesCoverage,
        Code::ConditionCoverage,
        Code::Budget,
    ];
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// One finding: what went wrong, where, and how loud.
#[derive(Clone, Debug, PartialEq)]
pub struct Finding {
    pub code: Code,
    /// The document the finding is about, where it is about one.
    pub doc: Option<DocRef>,
    /// The rule or section id, where the finding names one.
    pub id: Option<String>,
    pub message: String,
    pub severity: Severity,
}

impl Finding {
    pub fn new(
        code: Code,
        doc: Option<DocRef>,
        id: Option<String>,
        message: impl Into<String>,
    ) -> Finding {
        Finding {
            code,
            doc,
            id,
            message: message.into(),
            severity: code.severity(),
        }
    }

    /// A finding about a whole document.
    pub fn doc(code: Code, doc: &DocRef, message: impl Into<String>) -> Finding {
        Finding::new(code, Some(doc.clone()), None, message)
    }

    /// A finding about one node inside a document.
    pub fn node(code: Code, doc: &DocRef, id: &str, message: impl Into<String>) -> Finding {
        Finding::new(code, Some(doc.clone()), Some(id.to_string()), message)
    }

    /// A finding about the log rather than about state.
    pub fn log(code: Code, message: impl Into<String>) -> Finding {
        Finding::new(code, None, None, message)
    }

    pub fn is_rejecting(&self) -> bool {
        self.severity == Severity::Reject
    }
}

impl fmt::Display for Finding {
    /// `code · schema · id · message`, with `-` for a column the finding does not fill.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let doc = self
            .doc
            .as_ref()
            .map(DocRef::to_string)
            .unwrap_or_else(|| "-".into());
        let id = self.id.clone().unwrap_or_else(|| "-".into());
        write!(f, "{} · {} · {} · {}", self.code, doc, id, self.message)
    }
}

// ---------------------------------------------------------------------------
// family and prefix derivation
// ---------------------------------------------------------------------------

/// The canonical six sections every command schema carries.
pub const COMMAND_SECTIONS: [&str; 6] = [
    "roles",
    "reserved",
    "tools",
    "ways-of-working",
    "boundaries",
    "fail-conditions",
];

/// A skill's grammar family, derived from its directory-name stem exactly as the shipped checker
/// derives it: an `authoring-` or `patterns-` prefix names its family, and everything else falls
/// through to the review set, which the small families reuse by ruling.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Family {
    Review,
    Authoring,
    Patterns,
}

impl Family {
    pub fn of(stem: &str) -> Family {
        if stem.starts_with("authoring-") {
            Family::Authoring
        } else if stem.starts_with("patterns-") {
            Family::Patterns
        } else {
            Family::Review
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Family::Review => "review",
            Family::Authoring => "authoring",
            Family::Patterns => "patterns",
        }
    }

    /// The family's six section slugs, each minted once by that family's census ruling.
    pub fn sections(self) -> [&'static str; 6] {
        match self {
            Family::Review => [
                "independence",
                "scope",
                "inputs",
                "verdict",
                "output",
                "reserved",
            ],
            Family::Authoring => [
                "independence",
                "scope",
                "inputs",
                "artifact",
                "output",
                "reserved",
            ],
            Family::Patterns => [
                "trigger",
                "scope",
                "discipline",
                "inputs",
                "disclosure",
                "reserved",
            ],
        }
    }

    /// The block-id prefix of the family's common library, and `None` for a family that ships
    /// none. The patterns family ships no library by ruling, so any `extends:` there is a finding.
    pub fn common_prefix(self) -> Option<&'static str> {
        match self {
            Family::Review => Some("review-common"),
            Family::Authoring => Some("authoring-common"),
            Family::Patterns => None,
        }
    }
}

/// A common library's block-id prefix, derived from the document's own name: a skill-side library
/// drops the leading `skill-` (`skill-review-common` blocks are `review-common.<slug>`), and the
/// command-side library is `common`.
pub fn common_prefix_of(doc: &DocRef) -> String {
    match doc.kind {
        DocKind::CommandCommon => "common".to_string(),
        _ => doc
            .name
            .strip_prefix("skill-")
            .unwrap_or(&doc.name)
            .to_string(),
    }
}

/// A command schema's rule-id prefix, read off its own section ids.
///
/// The grammar carries no `prefix:` field — the prefixes were frozen by ruling and the section
/// ids are the source — so a schema whose sections disagree cannot be checked set-wise and says
/// so rather than guessing.
pub fn derive_prefix(schema: &RuleSchema) -> Result<String, Vec<String>> {
    let found: BTreeSet<String> = schema
        .sections
        .iter()
        .filter(|s| s.id.contains(".sec."))
        .filter_map(|s| s.id.split('.').next().map(str::to_string))
        .collect();
    match found.len() {
        1 => Ok(found.into_iter().next().unwrap_or_default()),
        _ => Err(found.into_iter().collect()),
    }
}

// ---------------------------------------------------------------------------
// the deixis marker list (advisory)
// ---------------------------------------------------------------------------

/// The curated deixis markers, ported from the shipped checker's expression exactly: references
/// that dangle once a rule is quoted on its own. "this schema" and "the run" are legal
/// self-reference and stay off the list deliberately.
const DEIXIS_MARKERS: [&str; 10] = [
    "these rules",
    "this section",
    "the section above",
    "the section below",
    "as stated above",
    "as stated earlier",
    "see above",
    "see below",
    "aforementioned",
    // `there is no <X> section` is matched separately, by `deixis_marker`, because its middle
    // token is a wildcard.
    "there is no",
];

/// The first deixis marker in `text`, matched on word boundaries.
///
/// A substring test would fire on "this sectional", which is why the boundaries are explicit.
fn deixis_marker(text: &str) -> Option<String> {
    let lowered = text.to_ascii_lowercase();
    for marker in DEIXIS_MARKERS {
        let mut from = 0;
        while let Some(at) = lowered[from..].find(marker) {
            let start = from + at;
            let end = start + marker.len();
            let before_ok = start == 0 || !is_word_byte(lowered.as_bytes()[start - 1]);
            let after_ok = end == lowered.len() || !is_word_byte(lowered.as_bytes()[end]);
            if before_ok && after_ok {
                if marker == "there is no" {
                    // `there is no <X> section` — the wildcard form; a bare "there is no" is not
                    // deictic on its own.
                    let rest = lowered[end..].trim_start();
                    let mut words = rest.split_whitespace();
                    if words.next().is_some() && words.next() == Some("section") {
                        return Some("there is no … section".to_string());
                    }
                } else {
                    return Some(marker.to_string());
                }
            }
            from = end;
        }
    }
    None
}

fn is_word_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

// ---------------------------------------------------------------------------
// the validator
// ---------------------------------------------------------------------------

/// Grade a replayed state against the hard set, plus the advisory reports.
///
/// The findings come back in document order, rejecting and advisory interleaved; callers split on
/// [`Finding::is_rejecting`].
pub fn validate(state: &State) -> Vec<Finding> {
    let mut findings = Vec::new();
    for (doc, document) in &state.docs {
        match document {
            Document::Rules(schema) => validate_rule_schema(state, doc, schema, &mut findings),
            Document::Labels(registry) => validate_registry(doc, registry, &mut findings),
            Document::Opaque(value) => {
                // Templates and shelf data carry no grammar, so the only thing to check is that
                // they can be encoded at all: past the canonical encoder's depth bound a document
                // would hash to a marker rather than to its content.
                let depth = canonical_depth(value);
                if depth > MAX_CANONICAL_DEPTH {
                    findings.push(Finding::doc(
                        Code::DepthExceeded,
                        doc,
                        format!(
                            "nests at least {depth} levels deep, past the canonical encoder's \
                             bound of {MAX_CANONICAL_DEPTH} — it cannot be hashed as itself"
                        ),
                    ));
                }
            }
        }
    }
    findings
}

/// The live label vocabulary a document draws from. `None` when no registry of the matching kind
/// is in state, in which case the label check is skipped rather than failing every rule at once.
fn registry_labels(state: &State, doc: &DocRef) -> Option<BTreeSet<String>> {
    let kind = doc.kind.registry()?;
    let registry = state
        .docs
        .iter()
        .find(|(key, _)| key.kind == kind)
        .and_then(|(_, document)| document.as_labels())?;
    Some(
        registry
            .labels
            .iter()
            .map(|(name, _)| name.clone())
            .collect(),
    )
}

fn validate_registry(doc: &DocRef, registry: &LabelRegistry, findings: &mut Vec<Finding>) {
    if registry.declared_kind.as_deref() != Some(doc.kind.as_str()) {
        findings.push(Finding::doc(
            Code::KindDiscriminator,
            doc,
            format!(
                "`kind: {}` missing — the document declares {:?}",
                doc.kind,
                registry.declared_kind.as_deref().unwrap_or("")
            ),
        ));
    }
    for (label, meaning) in &registry.labels {
        if meaning.trim().is_empty() {
            findings.push(Finding::node(
                Code::TextMissing,
                doc,
                label,
                "a registry label carries no meaning",
            ));
        }
    }
}

/// A rule's fields after `extends:` resolution — the three inherited fields and nothing else.
///
/// Public so the render path resolves inheritance through this implementation rather than a
/// second copy of it: a renderer that resolved `extends:` differently from the validator would
/// show guidance the hard set never graded.
pub struct ResolvedRule<'a> {
    pub text: Option<&'a str>,
    pub labels: Option<&'a [String]>,
    pub pointer: Option<&'a str>,
}

fn validate_rule_schema(
    state: &State,
    doc: &DocRef,
    schema: &RuleSchema,
    findings: &mut Vec<Finding>,
) {
    let is_skill_side = matches!(doc.kind, DocKind::Skill | DocKind::SkillCommon);
    let family = (doc.kind == DocKind::Skill).then(|| Family::of(&doc.name));

    check_discriminators(doc, schema, findings);
    let prefix = check_sections(doc, schema, family, findings);
    check_conditions(doc, schema, findings);

    let registry = registry_labels(state, doc);
    let mut live_ids: BTreeSet<String> = BTreeSet::new();
    let mut duplicates: BTreeSet<String> = BTreeSet::new();
    for section in &schema.sections {
        if !live_ids.insert(section.id.clone()) {
            duplicates.insert(section.id.clone());
        }
    }
    for rule in schema.rules() {
        if !live_ids.insert(rule.id.clone()) {
            duplicates.insert(rule.id.clone());
        }
    }
    for id in &duplicates {
        findings.push(Finding::node(
            Code::IdDuplicate,
            doc,
            id,
            "two live nodes share this id — an id is minted once",
        ));
    }

    for tombstone in &schema.tombstones {
        if live_ids.contains(&tombstone.id) {
            findings.push(Finding::node(
                Code::TombstoneIntegrity,
                doc,
                &tombstone.id,
                "both live and tombstoned — an id is minted once",
            ));
        }
        if tombstone.disposition.trim().is_empty() {
            findings.push(Finding::node(
                Code::TombstoneIntegrity,
                doc,
                &tombstone.id,
                "tombstone carries no disposition",
            ));
        }
    }

    let mut used_vars: BTreeSet<String> = BTreeSet::new();
    let mut activated: BTreeSet<(String, String)> = BTreeSet::new();
    let mut fail_targets: BTreeSet<String> = BTreeSet::new();
    let mut floors_and_gates: Vec<String> = Vec::new();
    let mut char_budget = 0usize;
    let mut all_text: Vec<String> = Vec::new();

    for rule in schema.rules() {
        let resolved = resolve_extends(state, doc, rule, family, findings);
        check_ids(doc, rule, prefix.as_deref(), is_skill_side, findings);
        check_class_and_kind(doc, rule, is_skill_side, findings);
        check_labels(doc, rule, &resolved, registry.as_ref(), findings);
        check_when(doc, schema, rule, &mut activated, findings);
        check_enforces(
            doc,
            schema,
            rule,
            &live_ids,
            is_skill_side,
            &mut fail_targets,
            findings,
        );
        check_text(doc, schema, rule, &resolved, &mut used_vars, findings);

        if let Some(anchor) = &rule.anchor {
            if !is_anchor(anchor) {
                findings.push(Finding::node(
                    Code::AnchorFormat,
                    doc,
                    &rule.id,
                    format!(
                        "anchor {anchor:?} is malformed — want 'YYYY-MM-DD <session-slug> [D#]'"
                    ),
                ));
            }
        }
        if matches!(rule.class_of(), Some(Class::Floor)) || rule.effective_kind() == RuleKind::Gate
        {
            floors_and_gates.push(rule.id.clone());
        }
        char_budget += resolved.text.map(str::len).unwrap_or(0);
        if let Some(text) = resolved.text {
            all_text.push(text.to_string());
        }
    }

    // --- advisory reports ---
    for (name, _) in &schema.vars {
        if !used_vars.contains(name) {
            findings.push(Finding::node(
                Code::UnusedVar,
                doc,
                name,
                "declared in `vars:` but named by no rule text",
            ));
        }
    }
    for (name, condition) in &schema.conditions {
        let used: BTreeSet<&String> = activated
            .iter()
            .filter(|(dim, _)| dim == name)
            .map(|(_, token)| token)
            .collect();
        if used.is_empty() {
            findings.push(Finding::node(
                Code::UnusedCondition,
                doc,
                name,
                "declared in `conditions:` but no rule's `when:` names it",
            ));
            continue;
        }
        for token in condition.tokens() {
            if !used.contains(&token) {
                findings.push(Finding::node(
                    Code::ConditionCoverage,
                    doc,
                    name,
                    format!("value {token:?} is declared but named by no rule's `when:`"),
                ));
            }
        }
    }
    // Declared moments. A moment is used by a moment-resolved condition or by a prose mention,
    // and by nothing else. The prose half is a bare substring test, exactly as the shipped
    // checker does it: a moment whose name is a common word reads as used on any incidental
    // mention, so the check under-reports unused moments and never invents one.
    let moment_resolved: BTreeSet<&str> = schema
        .conditions
        .iter()
        .filter_map(|(_, condition)| match condition.resolution_of() {
            Some(Resolution::MomentResolved(moment)) => Some(moment),
            _ => None,
        })
        .map(|moment| {
            schema
                .moments
                .iter()
                .map(|(name, _)| name.as_str())
                .find(|name| *name == moment)
                .unwrap_or("")
        })
        .collect();
    for (name, _) in &schema.moments {
        if moment_resolved.contains(name.as_str()) {
            continue;
        }
        if all_text.iter().any(|text| text.contains(name.as_str())) {
            continue;
        }
        findings.push(Finding::node(
            Code::UnusedMoment,
            doc,
            name,
            "declared but named by no moment-resolved condition and mentioned in no rule text",
        ));
    }

    if !fail_targets.is_empty() {
        let uncovered: Vec<&String> = floors_and_gates
            .iter()
            .filter(|id| !fail_targets.contains(*id))
            .collect();
        if !uncovered.is_empty() {
            findings.push(Finding::doc(
                Code::EnforcesCoverage,
                doc,
                format!(
                    "{} floor/gate rules no fail node enforces: {}",
                    uncovered.len(),
                    uncovered
                        .iter()
                        .map(|s| s.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            ));
        }
    }
    findings.push(Finding::doc(
        Code::Budget,
        doc,
        format!(
            "{} rules · {} resolved characters of rule text",
            schema.rules().count(),
            char_budget
        ),
    ));
}

fn check_discriminators(doc: &DocRef, schema: &RuleSchema, findings: &mut Vec<Finding>) {
    if schema.declared_kind.as_deref() != Some(doc.kind.as_str()) {
        findings.push(Finding::doc(
            Code::KindDiscriminator,
            doc,
            format!(
                "`kind: {}` missing — the document declares {:?}",
                doc.kind,
                schema.declared_kind.as_deref().unwrap_or("")
            ),
        ));
    }
    if matches!(doc.kind, DocKind::Command | DocKind::Skill) {
        match schema.declared_name.as_deref() {
            Some(name) if name == doc.name => {}
            other => findings.push(Finding::doc(
                Code::KindDiscriminator,
                doc,
                format!(
                    "the document names itself {:?} but is filed as {:?} — the name is one name",
                    other.unwrap_or(""),
                    doc.name
                ),
            )),
        }
    }
    if matches!(doc.kind, DocKind::Skill | DocKind::SkillCommon) && !schema.moments.is_empty() {
        findings.push(Finding::doc(
            Code::SkillGrammar,
            doc,
            "carries a `moments:` block — moments are command grammar",
        ));
    }
}

/// The section-set check, and the prefix a command schema's ids must lead with.
fn check_sections(
    doc: &DocRef,
    schema: &RuleSchema,
    family: Option<Family>,
    findings: &mut Vec<Finding>,
) -> Option<String> {
    if !doc.kind.is_rule_bearing() {
        return None;
    }
    // Common libraries carry blocks at the top level and no sections at all.
    if matches!(doc.kind, DocKind::CommandCommon | DocKind::SkillCommon) {
        if !schema.sections.is_empty() {
            findings.push(Finding::doc(
                Code::SectionSet,
                doc,
                "a common library carries blocks at the top level, never sections",
            ));
        }
        return Some(common_prefix_of(doc));
    }

    let (prefix, slugs): (String, Vec<&str>) = match doc.kind {
        DocKind::Skill => {
            let family = family.unwrap_or(Family::Review);
            (doc.name.clone(), family.sections().to_vec())
        }
        _ => match derive_prefix(schema) {
            Ok(prefix) => (prefix, COMMAND_SECTIONS.to_vec()),
            Err(found) => {
                findings.push(Finding::doc(
                    Code::IdPrefix,
                    doc,
                    if found.is_empty() {
                        "no well-formed section ids — the rule prefix cannot be derived".to_string()
                    } else {
                        format!(
                            "section ids disagree on the rule prefix ({}) — one schema carries one prefix",
                            found.join(", ")
                        )
                    },
                ));
                return None;
            }
        },
    };

    let expected: BTreeSet<String> = slugs
        .iter()
        .map(|slug| format!("{prefix}.sec.{slug}"))
        .collect();
    let live: BTreeSet<String> = schema.sections.iter().map(|s| s.id.clone()).collect();
    let family_name = family.map(Family::as_str).unwrap_or("command");
    for missing in expected.difference(&live) {
        findings.push(Finding::doc(
            Code::SectionSet,
            doc,
            format!(
                "canonical section {missing} absent — every {family_name}-family schema carries \
                 all six, empty ones explicitly"
            ),
        ));
    }
    for extra in live.difference(&expected) {
        findings.push(Finding::node(
            Code::SectionSet,
            doc,
            extra,
            format!(
                "not one of the six canonical {family_name}-family sections ({})",
                slugs.join(" · ")
            ),
        ));
    }
    for section in &schema.sections {
        // The set check above catches a malformed id as an "extra" whenever the prefix derives;
        // this reports it as what it is, and still reports it when the prefix does not derive.
        let well_formed = section
            .id
            .strip_prefix(&format!("{prefix}.sec."))
            .is_some_and(is_slug);
        if !well_formed {
            findings.push(Finding::node(
                Code::IdFormat,
                doc,
                &section.id,
                format!("section id fails the dotted-slug format — want `{prefix}.sec.<slug>`"),
            ));
        }
        if section.title.trim().is_empty() || section.intent.trim().is_empty() {
            findings.push(Finding::node(
                Code::TextMissing,
                doc,
                &section.id,
                "a section carries both a `title:` and an `intent:`",
            ));
        }
        if section.rules.is_empty() && section.note.as_deref().unwrap_or("").trim().is_empty() {
            findings.push(Finding::node(
                Code::TextMissing,
                doc,
                &section.id,
                "empty with no `note:` — a deliberately empty section names the emptiness deliberate",
            ));
        }
    }
    Some(prefix)
}

fn check_conditions(doc: &DocRef, schema: &RuleSchema, findings: &mut Vec<Finding>) {
    let moments: BTreeSet<&str> = schema.moments.iter().map(|(n, _)| n.as_str()).collect();
    for (name, condition) in &schema.conditions {
        if condition.value_kind() == Values::Malformed {
            findings.push(Finding::node(
                Code::ConditionDeclaration,
                doc,
                name,
                "`values:` must be a non-empty list of closed values, or the word `presence`",
            ));
        }
        match condition.resolution_of() {
            None => findings.push(Finding::node(
                Code::ConditionDeclaration,
                doc,
                name,
                "`resolution:` missing — every dimension declares where it resolves",
            )),
            Some(Resolution::Other(token)) => findings.push(Finding::node(
                Code::ConditionDeclaration,
                doc,
                name,
                format!(
                    "resolution {token:?} is not one of entry-derived · surface-presence · \
                     user-ruled · standing-trigger · moment-resolved(<moment>)"
                ),
            )),
            Some(Resolution::MomentResolved(moment)) => {
                if matches!(doc.kind, DocKind::Skill | DocKind::SkillCommon) {
                    findings.push(Finding::node(
                        Code::SkillGrammar,
                        doc,
                        name,
                        "resolves at a moment — skills declare no `moments:`",
                    ));
                } else if !moments.contains(moment.as_str()) {
                    findings.push(Finding::node(
                        Code::MomentUndeclared,
                        doc,
                        name,
                        format!(
                            "resolution names moment {moment:?}, which `moments:` does not declare"
                        ),
                    ));
                }
            }
            Some(_) => {}
        }
    }
}

/// A rule id is its document's prefix, then one slug — or, command-side only, `fail.<slug>`.
///
/// The prefix limb is enforced for commands as well as skills. The shipped command checker
/// derives a prefix from the section ids but never asserts that rule ids lead with it; every
/// shipped command schema already conforms, so binding it here closes the gap rather than
/// widening the corpus's obligations.
fn check_ids(
    doc: &DocRef,
    rule: &Rule,
    prefix: Option<&str>,
    is_skill_side: bool,
    findings: &mut Vec<Finding>,
) {
    let Some(prefix) = prefix else {
        // The prefix could not be derived; the id still has to be a dotted slug.
        if !is_dotted_id(&rule.id) {
            findings.push(Finding::node(
                Code::IdFormat,
                doc,
                &rule.id,
                "id fails the dotted-slug format — lowercase kebab segments, two or more of them",
            ));
        }
        return;
    };
    let Some(rest) = rule.id.strip_prefix(&format!("{prefix}.")) else {
        findings.push(Finding::node(
            Code::IdPrefix,
            doc,
            &rule.id,
            format!("id does not lead with this document's prefix {prefix:?}"),
        ));
        return;
    };
    let well_formed =
        is_slug(rest) || (!is_skill_side && rest.strip_prefix("fail.").is_some_and(is_slug));
    if !well_formed {
        findings.push(Finding::node(
            Code::IdFormat,
            doc,
            &rule.id,
            format!(
                "id fails the dotted-slug format — want `{prefix}.<slug>`{}",
                if is_skill_side {
                    ""
                } else {
                    " or `<prefix>.fail.<slug>`"
                }
            ),
        ));
    }
}

fn check_class_and_kind(
    doc: &DocRef,
    rule: &Rule,
    is_skill_side: bool,
    findings: &mut Vec<Finding>,
) {
    // A common-library block declares no class of its own: every stub that binds it declares one.
    let is_block = matches!(doc.kind, DocKind::CommandCommon | DocKind::SkillCommon);
    match rule.class_of() {
        None if is_block => {}
        None => findings.push(Finding::node(
            Code::ClassUnknown,
            doc,
            &rule.id,
            "`class:` missing — want floor|must|advisory",
        )),
        Some(Class::Other(token)) => findings.push(Finding::node(
            Code::ClassUnknown,
            doc,
            &rule.id,
            format!("`class: {token}` is not one of floor|must|advisory"),
        )),
        Some(_) if is_block => findings.push(Finding::node(
            Code::ExtendsClassLocal,
            doc,
            &rule.id,
            "a common block carries `class:` — class is always local to the binding stub",
        )),
        Some(_) => {}
    }

    if is_block {
        for (name, present) in [
            ("kind", rule.kind.is_some()),
            ("when", !rule.when.is_empty()),
            ("enforces", rule.enforces.is_some()),
        ] {
            if present {
                findings.push(Finding::node(
                    Code::ExtendsClassLocal,
                    doc,
                    &rule.id,
                    format!(
                        "a common block carries `{name}:` — an absence-meaningful field is never \
                         inherited and is always local to the stub"
                    ),
                ));
            }
        }
    }

    let legal = if is_skill_side {
        RuleKind::SKILL_KINDS.to_vec()
    } else {
        RuleKind::COMMAND_KINDS.to_vec()
    };
    if let Some(token) = &rule.kind {
        if !legal.contains(&token.as_str()) {
            let code = if is_skill_side && token == "fail" {
                Code::SkillGrammar
            } else {
                Code::RuleKindUnknown
            };
            findings.push(Finding::node(
                code,
                doc,
                &rule.id,
                format!("`kind: {token}` is not one of {}", legal.join(" · ")),
            ));
        }
    }
}

fn check_labels(
    doc: &DocRef,
    rule: &Rule,
    resolved: &ResolvedRule<'_>,
    registry: Option<&BTreeSet<String>>,
    findings: &mut Vec<Finding>,
) {
    let Some(registry) = registry else { return };
    let Some(labels) = resolved.labels else {
        // A stub inheriting from a block that carries no labels resolves label-less by design.
        return;
    };
    for label in labels {
        if !registry.contains(label) {
            findings.push(Finding::node(
                Code::LabelUnknown,
                doc,
                &rule.id,
                format!("label {label:?} is not in this family's registry"),
            ));
        }
    }
}

fn check_when(
    doc: &DocRef,
    schema: &RuleSchema,
    rule: &Rule,
    activated: &mut BTreeSet<(String, String)>,
    findings: &mut Vec<Finding>,
) {
    for (dim, value) in &rule.when {
        let Some(condition) = lookup(&schema.conditions, dim) else {
            findings.push(Finding::node(
                Code::WhenUndeclared,
                doc,
                &rule.id,
                format!("`when:` names dimension {dim:?}, which `conditions:` does not declare"),
            ));
            continue;
        };
        let presence = condition.is_presence();
        let legal = condition.tokens();
        for raw in value.values() {
            let token = norm_value(raw, presence);
            if legal.contains(&token) {
                activated.insert((dim.clone(), token));
            } else {
                let mut shown: Vec<&String> = legal.iter().collect();
                shown.sort();
                findings.push(Finding::node(
                    Code::WhenValue,
                    doc,
                    &rule.id,
                    format!(
                        "`when: {{{dim}: {token}}}` is not a declared value of {dim:?} ({})",
                        shown
                            .iter()
                            .map(|s| s.as_str())
                            .collect::<Vec<_>>()
                            .join(" · ")
                    ),
                ));
            }
        }
    }
}

fn lookup<'a>(items: &'a Ordered<Condition>, key: &str) -> Option<&'a Condition> {
    items.iter().find(|(k, _)| k == key).map(|(_, v)| v)
}

fn check_enforces(
    doc: &DocRef,
    schema: &RuleSchema,
    rule: &Rule,
    live_ids: &BTreeSet<String>,
    is_skill_side: bool,
    fail_targets: &mut BTreeSet<String>,
    findings: &mut Vec<Finding>,
) {
    let is_fail = rule.is_fail();
    let fail_segment = rule.id.contains(".fail.");

    if is_skill_side {
        if rule.enforces.is_some() {
            findings.push(Finding::node(
                Code::SkillGrammar,
                doc,
                &rule.id,
                "carries `enforces:` — the field left the skill grammar with `kind: fail`",
            ));
        }
        return;
    }

    // The keying is bidirectional and explicit: `constraint` is the only defaulted kind, so a
    // `.fail.` rule that omits `kind:` would silently read as a constraint.
    if fail_segment && !is_fail {
        findings.push(Finding::node(
            Code::FailSegment,
            doc,
            &rule.id,
            match &rule.kind {
                Some(kind) => format!("under the .fail. segment but carries `kind: {kind}`"),
                None => {
                    "under the .fail. segment with no explicit `kind: fail` — the fail kind is \
                         never defaulted"
                        .to_string()
                }
            },
        ));
    } else if is_fail && !fail_segment {
        findings.push(Finding::node(
            Code::FailSegment,
            doc,
            &rule.id,
            "`kind: fail` outside the .fail. segment",
        ));
    }

    match (&rule.enforces, is_fail) {
        (None, true) => findings.push(Finding::node(
            Code::EnforcesRequired,
            doc,
            &rule.id,
            "a `kind: fail` node carries no `enforces:`",
        )),
        (Some(_), false) => findings.push(Finding::node(
            Code::EnforcesMisplaced,
            doc,
            &rule.id,
            "carries `enforces:` on a node that is not `kind: fail` — the field is a fail node's \
             mirror link and nothing else's",
        )),
        (Some(targets), true) if targets.is_empty() => {
            if rule.note.as_deref().unwrap_or("").trim().is_empty() {
                findings.push(Finding::node(
                    Code::EnforcesRequired,
                    doc,
                    &rule.id,
                    "`enforces: []` with no `note:` — an empty mirror is legal only with a stated \
                     reason, so absence is a statement rather than an omission",
                ));
            }
        }
        (Some(targets), true) => {
            for target in targets {
                if schema.is_tombstoned(target) {
                    findings.push(Finding::node(
                        Code::EnforcesUnresolved,
                        doc,
                        &rule.id,
                        format!("`enforces: {target}` names a tombstoned rule"),
                    ));
                } else if target.contains(".sec.") {
                    findings.push(Finding::node(
                        Code::EnforcesUnresolved,
                        doc,
                        &rule.id,
                        format!(
                            "`enforces: {target}` names a section — the mirror link points at the \
                             rule the fail node is the contrapositive of"
                        ),
                    ));
                } else if !live_ids.contains(target) {
                    findings.push(Finding::node(
                        Code::EnforcesUnresolved,
                        doc,
                        &rule.id,
                        format!("`enforces: {target}` resolves to no rule in this document"),
                    ));
                } else {
                    fail_targets.insert(target.clone());
                }
            }
        }
        (None, false) => {}
    }
}

/// Resolve a rule's `extends:` against its family's common library, reporting every way the
/// binding can be wrong. Inheritance covers text, labels and pointer, and nothing else — class,
/// kind, `when:` and `enforces:` are always local, so their absence stays meaningful.
///
/// Public for the render path (D-2). Pass an empty `findings` sink to resolve without reporting.
pub fn resolve_extends<'a>(
    state: &'a State,
    doc: &DocRef,
    rule: &'a Rule,
    family: Option<Family>,
    findings: &mut Vec<Finding>,
) -> ResolvedRule<'a> {
    let mut resolved = ResolvedRule {
        text: rule.text.as_deref(),
        labels: rule.labels.as_deref(),
        pointer: rule.pointer.as_deref(),
    };
    let Some(target) = rule.extends.as_deref() else {
        return resolved;
    };

    // Which library may this document bind? A command schema binds `common.`; a skill schema
    // binds only its own family's, and the patterns family ships none at all.
    let wanted_prefix = match doc.kind {
        DocKind::Command => Some("common"),
        DocKind::Skill => match family.unwrap_or(Family::Review).common_prefix() {
            Some(prefix) => Some(prefix),
            None => {
                findings.push(Finding::node(
                    Code::ExtendsCrossFamily,
                    doc,
                    &rule.id,
                    format!(
                        "`extends: {target}` — the patterns family ships no common library; \
                         every patterns rule carries local text"
                    ),
                ));
                return resolved;
            }
        },
        _ => {
            findings.push(Finding::node(
                Code::ExtendsCrossFamily,
                doc,
                &rule.id,
                format!("`extends: {target}` — a common library never binds another"),
            ));
            return resolved;
        }
    };

    let Some(wanted_prefix) = wanted_prefix else {
        return resolved;
    };
    let matches_prefix = target
        .split_once('.')
        .is_some_and(|(head, slug)| head == wanted_prefix && is_slug(slug));
    if !matches_prefix {
        findings.push(Finding::node(
            Code::ExtendsCrossFamily,
            doc,
            &rule.id,
            format!(
                "`extends: {target}` — want `{wanted_prefix}.<slug>`; sharing across families or \
                 across grammars is forbidden"
            ),
        ));
        return resolved;
    }

    let library_kind = if doc.kind == DocKind::Command {
        DocKind::CommandCommon
    } else {
        DocKind::SkillCommon
    };
    let block = state
        .docs
        .iter()
        .filter(|(key, _)| key.kind == library_kind && common_prefix_of(key) == wanted_prefix)
        .filter_map(|(_, document)| document.as_rules())
        .find_map(|library| library.blocks.iter().find(|b| b.id == target));

    let Some(block) = block else {
        findings.push(Finding::node(
            Code::ExtendsUnresolved,
            doc,
            &rule.id,
            format!("`extends: {target}` names no block in the {wanted_prefix} library"),
        ));
        return resolved;
    };

    if rule.class.is_none() {
        findings.push(Finding::node(
            Code::ExtendsClassLocal,
            doc,
            &rule.id,
            "an `extends:` stub declares no local `class:` — class is never inherited, so a \
             floor's bindingness stays readable from its own file",
        ));
    }

    // Inheritance covers text, labels and pointer, and nothing else.
    if resolved.text.is_none() {
        resolved.text = block.text.as_deref();
    }
    if resolved.labels.is_none() {
        resolved.labels = block.labels.as_deref();
    }
    if resolved.pointer.is_none() {
        resolved.pointer = block.pointer.as_deref();
    }
    resolved
}

fn check_text(
    doc: &DocRef,
    schema: &RuleSchema,
    rule: &Rule,
    resolved: &ResolvedRule<'_>,
    used_vars: &mut BTreeSet<String>,
    findings: &mut Vec<Finding>,
) {
    let Some(text) = resolved.text.filter(|t| !t.trim().is_empty()) else {
        findings.push(Finding::node(
            Code::TextMissing,
            doc,
            &rule.id,
            "`text:` missing or empty, and no `extends:` supplies one",
        ));
        return;
    };

    // `${var}` closure, checked on resolved text: an inherited text substitutes from the BINDING
    // document's vars, never from the library's. A common library therefore declares no vars and
    // is never checked for closure itself — its blocks are closed by every schema that binds
    // them, and each of those bindings is checked here in its own right.
    let is_library = matches!(doc.kind, DocKind::CommandCommon | DocKind::SkillCommon);
    let mut named_here: BTreeSet<String> = BTreeSet::new();
    for name in placeholders(text) {
        used_vars.insert(name.clone());
        // One finding per placeholder per rule, however often that rule's text repeats it.
        if !named_here.insert(name.clone()) || is_library {
            continue;
        }
        if !schema.vars.iter().any(|(v, _)| *v == name) {
            findings.push(Finding::node(
                Code::VarUnbound,
                doc,
                &rule.id,
                format!("orphan placeholder ${{{name}}} — unbound in `vars:`"),
            ));
        }
    }

    if let Some(marker) = deixis_marker(text) {
        findings.push(Finding::node(
            Code::Deixis,
            doc,
            &rule.id,
            format!("deictic reference {marker:?} — the referent lives outside the block"),
        ));
    }
}

/// Every `${name}` in a string, in order of appearance.
///
/// Public for the render path (D-2), which substitutes the same placeholders the hard set
/// checks for closure — one scanner, so the two can never disagree about what a placeholder is.
pub fn placeholders(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let bytes = text.as_bytes();
    let mut i = 0;
    while i + 1 < bytes.len() {
        if bytes[i] == b'$' && bytes[i + 1] == b'{' {
            if let Some(end) = text[i + 2..].find('}') {
                let name = &text[i + 2..i + 2 + end];
                if !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                    out.push(name.to_string());
                }
                i += 2 + end + 1;
                continue;
            }
        }
        i += 1;
    }
    out
}

/// Per-document rule and floor counts, for the corpus pins and the budget report.
pub fn census(state: &State) -> BTreeMap<DocKind, (usize, usize)> {
    let mut out: BTreeMap<DocKind, (usize, usize)> = BTreeMap::new();
    for (doc, document) in &state.docs {
        let Some(schema) = document.as_rules() else {
            continue;
        };
        let entry = out.entry(doc.kind).or_default();
        for rule in schema.sections.iter().flat_map(|s| s.rules.iter()) {
            entry.0 += 1;
            if rule.is_floor() {
                entry.1 += 1;
            }
        }
    }
    out
}
