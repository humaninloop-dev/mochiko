//! The similar-rule detector — an advisory report over the replayed state.
//!
//! A port of `scripts/find-similar-rules.py`, whose 48-probe matrix it inherits. It **proposes**
//! candidate clusters and nothing else: it never merges, never edits, and never gates. Combining
//! rules is judgment, which the bright line (GI-019) keeps with the skills that own it — so this
//! module reports and stops, and every finding it produces is advisory.
//!
//! # Reproducing `difflib`
//!
//! The Python scores each pair with `difflib.SequenceMatcher`, so the probes are written against
//! that algorithm's exact numbers, not against "some similarity measure". This module therefore
//! implements CPython's Ratcliff/Obershelp directly, **autojunk included**: at 200 elements or
//! more, every element occurring more than `n / 100 + 1` times is dropped from the index. That
//! heuristic is not cosmetic — measured over the real corpus during planning, it moves the ratio
//! on 962 of 2,000 long pairs, so a port without it would agree with the Python on the short
//! texts and quietly diverge on the long ones.
//!
//! Parity was measured, not assumed: over 18,577 real corpus pairs, this algorithm and
//! `difflib` agreed on every `ratio()` and every `text_sim()`, worst absolute delta 0.0.
//! `tests/matrix_similar.rs` pins that with reference vectors captured from Python and with the
//! whole-corpus figures the live detector reports.
//!
//! `isjunk` is always `None` in the detector, so `SequenceMatcher`'s junk-extension loops are
//! unreachable and are not written here.

use crate::model::{DocKind, DocRef, Rule};
use crate::replay::State;
use crate::validate::{self, Family};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

/// The combined-score threshold a pair must clear, and the detector's own default.
pub const DEFAULT_THRESHOLD: f64 = 0.60;

/// The structural bonus's ceiling. Text alone must therefore clear `threshold - BONUS_CAP`.
pub const BONUS_CAP: f64 = 0.12;

/// Below this many normalised tokens, `difflib` ratios read as grammatical frame rather than
/// similarity, so a short pair must be near-exact to pair at all.
pub const SHORT_TOKENS: usize = 6;
pub const SHORT_TEXT_SIM: f64 = 0.80;

/// The adjudicated keep-distinct pairs, maintainer-side. Never shipped with the plugin.
pub const ALLOWLIST: &str = "scripts/similar-rules-allowlist.yaml";

/// Where `difflib` starts treating popular elements as junk.
const AUTOJUNK_MIN: usize = 200;

// ---------------------------------------------------------------------------
// difflib
// ---------------------------------------------------------------------------

/// `difflib.SequenceMatcher(None, a, b).ratio()` over the two strings' characters.
pub fn ratio(a: &str, b: &str) -> f64 {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let total = a.len() + b.len();
    if total == 0 {
        return 1.0;
    }
    let index = build_index(&b);
    let mut scratch = Scratch::for_len(b.len());
    2.0 * matching_size(&a, &b, &index, &mut scratch) as f64 / total as f64
}

/// Row buffers for [`longest_match`], hoisted out of the inner loop.
///
/// CPython allocates a dict per row of `a`; over 146,572 corpus pairs that shape dominates the
/// run time here, so the rows live in two reusable arrays with the touched slots tracked for
/// clearing. The arithmetic is unchanged — the reference vectors and the corpus figures are what
/// prove that.
struct Scratch {
    prev: Vec<usize>,
    cur: Vec<usize>,
    touched_prev: Vec<usize>,
    touched_cur: Vec<usize>,
}

impl Scratch {
    fn for_len(n: usize) -> Scratch {
        Scratch {
            prev: vec![0; n + 1],
            cur: vec![0; n + 1],
            touched_prev: Vec::new(),
            touched_cur: Vec::new(),
        }
    }

    fn reset(&mut self) {
        self.prev.fill(0);
        self.cur.fill(0);
        self.touched_prev.clear();
        self.touched_cur.clear();
    }
}

/// `real_quick_ratio()`: an upper bound from the lengths alone.
pub fn real_quick_ratio(a: &str, b: &str) -> f64 {
    let (la, lb) = (a.chars().count(), b.chars().count());
    let total = la + lb;
    if total == 0 {
        return 1.0;
    }
    2.0 * la.min(lb) as f64 / total as f64
}

/// `quick_ratio()`: an upper bound from the shared character multiset.
pub fn quick_ratio(a: &str, b: &str) -> f64 {
    let mut counts: HashMap<char, isize> = HashMap::new();
    let mut la = 0usize;
    for ch in a.chars() {
        *counts.entry(ch).or_insert(0) += 1;
        la += 1;
    }
    let mut matches = 0usize;
    let mut lb = 0usize;
    for ch in b.chars() {
        lb += 1;
        if let Some(slot) = counts.get_mut(&ch) {
            if *slot > 0 {
                *slot -= 1;
                matches += 1;
            }
        }
    }
    let total = la + lb;
    if total == 0 {
        return 1.0;
    }
    2.0 * matches as f64 / total as f64
}

/// The `b2j` index: where each element of `b` occurs, minus the popular ones autojunk drops.
fn build_index(b: &[char]) -> HashMap<char, Vec<usize>> {
    let mut index: HashMap<char, Vec<usize>> = HashMap::new();
    for (i, ch) in b.iter().enumerate() {
        index.entry(*ch).or_default().push(i);
    }
    let n = b.len();
    if n >= AUTOJUNK_MIN {
        let ntest = n / 100 + 1;
        index.retain(|_, idxs| idxs.len() <= ntest);
    }
    index
}

/// `find_longest_match` over `a[alo..ahi]` and `b[blo..bhi]`.
///
/// The junk-extension loops CPython runs after this are omitted: they only fire for elements the
/// caller declared junk, and the detector never declares any. Popular elements are absent from
/// the index rather than junk, so the non-junk extension below still walks over them, exactly as
/// CPython's does.
fn longest_match(
    a: &[char],
    b: &[char],
    index: &HashMap<char, Vec<usize>>,
    window: Window,
    scratch: &mut Scratch,
) -> (usize, usize, usize) {
    let Window { alo, ahi, blo, bhi } = window;
    let (mut besti, mut bestj, mut bestsize) = (alo, blo, 0usize);
    scratch.reset();
    // Slot `j + 1` holds the run length ending at `b[j]`, so slot 0 stands in for CPython's
    // absent `j2len[j - 1]` and needs no special case.
    for (offset, ch) in a[alo..ahi].iter().enumerate() {
        let i = alo + offset;
        for &t in &scratch.touched_cur {
            scratch.cur[t] = 0;
        }
        scratch.touched_cur.clear();
        if let Some(positions) = index.get(ch) {
            for &j in positions {
                if j < blo {
                    continue;
                }
                if j >= bhi {
                    break;
                }
                let k = scratch.prev[j] + 1;
                scratch.cur[j + 1] = k;
                scratch.touched_cur.push(j + 1);
                if k > bestsize {
                    besti = i + 1 - k;
                    bestj = j + 1 - k;
                    bestsize = k;
                }
            }
        }
        std::mem::swap(&mut scratch.prev, &mut scratch.cur);
        std::mem::swap(&mut scratch.touched_prev, &mut scratch.touched_cur);
    }

    while besti > alo && bestj > blo && a[besti - 1] == b[bestj - 1] {
        besti -= 1;
        bestj -= 1;
        bestsize += 1;
    }
    while besti + bestsize < ahi
        && bestj + bestsize < bhi
        && a[besti + bestsize] == b[bestj + bestsize]
    {
        bestsize += 1;
    }
    (besti, bestj, bestsize)
}

/// The sub-ranges of `a` and `b` a match is looked for in — CPython's four loose arguments,
/// bundled so the recursion reads as one window rather than four indices.
#[derive(Clone, Copy)]
struct Window {
    alo: usize,
    ahi: usize,
    blo: usize,
    bhi: usize,
}

/// The total size of the matching blocks — the `M` of `2M / T`.
fn matching_size(
    a: &[char],
    b: &[char],
    index: &HashMap<char, Vec<usize>>,
    scratch: &mut Scratch,
) -> usize {
    let mut queue = vec![Window {
        alo: 0,
        ahi: a.len(),
        blo: 0,
        bhi: b.len(),
    }];
    let mut total = 0usize;
    while let Some(window) = queue.pop() {
        let (i, j, k) = longest_match(a, b, index, window, scratch);
        if k == 0 {
            continue;
        }
        total += k;
        let Window { alo, ahi, blo, bhi } = window;
        if alo < i && blo < j {
            queue.push(Window {
                alo,
                ahi: i,
                blo,
                bhi: j,
            });
        }
        if i + k < ahi && j + k < bhi {
            queue.push(Window {
                alo: i + k,
                ahi,
                blo: j + k,
                bhi,
            });
        }
    }
    total
}

// ---------------------------------------------------------------------------
// scoring
// ---------------------------------------------------------------------------

/// Scoring-only normalisation. The report always shows the raw text.
///
/// Lowercase; `${var}` and `/mochiko:<cmd>` collapse to markers; a citation of the schema's own
/// prefix collapses to `«self».`, so two rules that differ only by naming their own command
/// score as one wording; everything outside the kept alphabet becomes a space.
pub fn norm_for_sim(text: &str, prefix: &str) -> String {
    let lowered = text.to_lowercase();
    let collapsed = collapse_self(&collapse_commands(&collapse_vars(&lowered)), prefix);
    collapsed
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() || c.is_ascii_digit() || c == '.' || c == '«' || c == '»' {
                c
            } else {
                ' '
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// `${identifier}` → `«var»`.
fn collapse_vars(text: &str) -> String {
    let bytes: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == '$' && bytes.get(i + 1) == Some(&'{') {
            if let Some(close) = (i + 2..bytes.len()).find(|&j| bytes[j] == '}') {
                let inner: String = bytes[i + 2..close].iter().collect();
                if !inner.is_empty() && inner.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                {
                    out.push_str("«var»");
                    i = close + 1;
                    continue;
                }
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    out
}

/// `/mochiko:<slug>` → `«cmd»`.
fn collapse_commands(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let needle: Vec<char> = "/mochiko:".chars().collect();
    let mut out = String::with_capacity(text.len());
    let mut i = 0;
    while i < chars.len() {
        if chars[i..].starts_with(&needle) {
            let mut j = i + needle.len();
            let start = j;
            while j < chars.len()
                && (chars[j].is_ascii_lowercase() || chars[j].is_ascii_digit() || chars[j] == '-')
            {
                j += 1;
            }
            if j > start {
                out.push_str("«cmd»");
                i = j;
                continue;
            }
        }
        out.push(chars[i]);
        i += 1;
    }
    out
}

/// `<prefix>.`, `<prefix>.sec.` and `<prefix>.fail.` → `«self».`, on a word boundary.
fn collapse_self(text: &str, prefix: &str) -> String {
    if prefix.is_empty() {
        return text.to_string();
    }
    let chars: Vec<char> = text.chars().collect();
    let needle: Vec<char> = format!("{prefix}.").chars().collect();
    let mut out = String::with_capacity(text.len());
    let mut i = 0;
    while i < chars.len() {
        let boundary = i == 0 || !is_word(chars[i - 1]);
        if boundary && chars[i..].starts_with(&needle) {
            let mut j = i + needle.len();
            for segment in ["sec.", "fail."] {
                let seg: Vec<char> = segment.chars().collect();
                if chars[j..].starts_with(&seg) {
                    j += seg.len();
                    break;
                }
            }
            out.push_str("«self».");
            i = j;
            continue;
        }
        out.push(chars[i]);
        i += 1;
    }
    out
}

fn is_word(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// The pair's text similarity: the better of the sequence ratio and the token-sort ratio, with
/// `difflib`'s two cheap upper bounds as early-outs.
///
/// The early-out is sound for the token-sort arm too: sorting tokens keeps the character
/// multiset, which is what `quick_ratio` bounds.
pub fn text_sim(a: &str, b: &str, floor: f64) -> f64 {
    if real_quick_ratio(a, b) < floor || quick_ratio(a, b) < floor {
        return 0.0;
    }
    let r = ratio(a, b);
    r.max(ratio(&token_sort(a), &token_sort(b)))
}

fn token_sort(text: &str) -> String {
    let mut tokens: Vec<&str> = text.split_whitespace().collect();
    tokens.sort_unstable();
    tokens.join(" ")
}

// ---------------------------------------------------------------------------
// rules
// ---------------------------------------------------------------------------

/// One rule as the detector sees it: resolved text, plus the fields the structural bonus reads.
#[derive(Clone, Debug)]
pub struct ScoredRule {
    pub schema: String,
    pub prefix: String,
    pub id: String,
    /// The trailing slug of the rule's section id.
    pub section: String,
    pub kind: String,
    pub class: String,
    pub labels: BTreeSet<String>,
    pub pointer: Option<String>,
    pub extends: Option<String>,
    /// Resolved (post-`extends:`) text, as written.
    pub text: String,
    /// The same text, normalised for scoring.
    pub norm: String,
}

impl ScoredRule {
    fn tokens(&self) -> usize {
        self.norm.split_whitespace().count()
    }
}

/// The structural bonus: a shared pointer, a shared section slug, overlapping labels.
pub fn struct_bonus(x: &ScoredRule, y: &ScoredRule) -> f64 {
    let mut bonus: f64 = 0.0;
    if let (Some(a), Some(b)) = (&x.pointer, &y.pointer) {
        if !a.is_empty() && a == b {
            bonus += 0.08;
        }
    }
    if x.section == y.section {
        bonus += 0.04;
    }
    if !x.labels.is_empty() && !y.labels.is_empty() {
        let shared = x.labels.intersection(&y.labels).count() as f64;
        let union = x.labels.union(&y.labels).count() as f64;
        if union > 0.0 && shared / union >= 0.5 {
            bonus += 0.04;
        }
    }
    bonus.min(BONUS_CAP)
}

/// Every live rule of every command and skill schema, with `extends:` resolved.
///
/// Common libraries are not scanned in their own right, exactly as the Python does not scan
/// them: a block is scored through the stubs that bind it, so a promotion candidate surfaces as
/// an `EXTEND-GAP` rather than as an edge to the library.
pub fn rules(state: &State, warnings: &mut Vec<String>) -> Vec<ScoredRule> {
    let mut out = Vec::new();
    for (doc, document) in &state.docs {
        if !matches!(doc.kind, DocKind::Command | DocKind::Skill) {
            continue;
        }
        let Some(schema) = document.as_rules() else {
            continue;
        };
        let Some(prefix) = section_prefix(schema) else {
            warnings.push(format!("{doc}: no section IDs, skipped"));
            continue;
        };
        let family = (doc.kind == DocKind::Skill).then(|| Family::of(&doc.name));
        for section in &schema.sections {
            let slug = section
                .id
                .rsplit_once(".sec.")
                .map_or(section.id.as_str(), |(_, slug)| slug)
                .to_string();
            for rule in &section.rules {
                let mut ignored = Vec::new();
                let resolved = validate::resolve_extends(state, doc, rule, family, &mut ignored);
                let text = resolved.text.unwrap_or_default().trim().to_string();
                if text.is_empty() {
                    warnings.push(format!("{}: empty resolved text, skipped", rule.id));
                    continue;
                }
                out.push(ScoredRule {
                    schema: doc.name.clone(),
                    prefix: prefix.clone(),
                    id: rule.id.clone(),
                    section: slug.clone(),
                    kind: rule_kind(rule),
                    class: rule.class.clone().unwrap_or_default(),
                    labels: resolved
                        .labels
                        .map(|l| l.iter().cloned().collect())
                        .unwrap_or_default(),
                    pointer: resolved.pointer.map(str::to_string),
                    extends: rule.extends.clone(),
                    norm: norm_for_sim(&text, &prefix),
                    text,
                });
            }
        }
    }
    out
}

fn rule_kind(rule: &Rule) -> String {
    match rule.kind.as_deref() {
        Some(kind) if !kind.trim().is_empty() => kind.to_string(),
        _ => "constraint".to_string(),
    }
}

/// The prefix the schema's own section ids carry — the Python's derivation, kept so a document
/// the validator would reject still scores rather than vanishing from the report.
fn section_prefix(schema: &crate::model::RuleSchema) -> Option<String> {
    schema
        .sections
        .iter()
        .find_map(|s| s.id.split_once(".sec.").map(|(head, _)| head.to_string()))
}

// ---------------------------------------------------------------------------
// edges and clusters
// ---------------------------------------------------------------------------

/// One above-threshold pair.
#[derive(Clone, Debug)]
pub struct Edge {
    pub total: f64,
    pub sim: f64,
    pub a: usize,
    pub b: usize,
}

/// A connected group of edges, and the rules they touch.
#[derive(Clone, Debug)]
pub struct Cluster {
    pub members: Vec<ScoredRule>,
    pub edges: Vec<(f64, f64, String, String)>,
}

/// What a cluster is, in the detector's own vocabulary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tag {
    CommonCandidate,
    CrossPair,
    IntraSchema,
    ExtendGap,
}

impl Tag {
    pub fn as_str(self) -> &'static str {
        match self {
            Tag::CommonCandidate => "COMMON-CANDIDATE",
            Tag::CrossPair => "CROSS-PAIR",
            Tag::IntraSchema => "INTRA-SCHEMA",
            Tag::ExtendGap => "EXTEND-GAP",
        }
    }
}

pub fn classify(cluster: &Cluster) -> Vec<Tag> {
    let schemas: BTreeSet<&str> = cluster.members.iter().map(|m| m.schema.as_str()).collect();
    let mut tags = vec![match schemas.len() {
        0 | 1 => Tag::IntraSchema,
        2 => Tag::CrossPair,
        _ => Tag::CommonCandidate,
    }];
    let extending = cluster
        .members
        .iter()
        .filter(|m| m.extends.is_some())
        .count();
    if extending > 0 && extending < cluster.members.len() {
        tags.push(Tag::ExtendGap);
    }
    tags
}

/// An unordered pair of rule ids, the shape the allowlist suppresses.
fn pair_key(a: &str, b: &str) -> (String, String) {
    if a <= b {
        (a.to_string(), b.to_string())
    } else {
        (b.to_string(), a.to_string())
    }
}

/// One rule's normalised text, prepared once so a corpus-wide sweep does not rebuild it per pair.
struct Prepared {
    chars: Vec<char>,
    index: HashMap<char, Vec<usize>>,
    counts: HashMap<char, usize>,
    sorted: Vec<char>,
    sorted_index: HashMap<char, Vec<usize>>,
}

impl Prepared {
    fn of(norm: &str) -> Prepared {
        let chars: Vec<char> = norm.chars().collect();
        let sorted: Vec<char> = token_sort(norm).chars().collect();
        let mut counts: HashMap<char, usize> = HashMap::new();
        for ch in &chars {
            *counts.entry(*ch).or_insert(0) += 1;
        }
        Prepared {
            index: build_index(&chars),
            sorted_index: build_index(&sorted),
            counts,
            chars,
            sorted,
        }
    }
}

/// [`text_sim`] over two prepared rules. Same arithmetic, no per-pair rebuilding.
fn text_sim_prepared(x: &Prepared, y: &Prepared, floor: f64, scratch: &mut Scratch) -> f64 {
    let (la, lb) = (x.chars.len(), y.chars.len());
    let total = la + lb;
    if total == 0 {
        return 1.0;
    }
    if 2.0 * la.min(lb) as f64 / (total as f64) < floor {
        return 0.0;
    }
    let (small, large) = if x.counts.len() <= y.counts.len() {
        (&x.counts, &y.counts)
    } else {
        (&y.counts, &x.counts)
    };
    let shared: usize = small
        .iter()
        .map(|(ch, n)| (*n).min(large.get(ch).copied().unwrap_or(0)))
        .sum();
    if 2.0 * shared as f64 / (total as f64) < floor {
        return 0.0;
    }

    let r = 2.0 * matching_size(&x.chars, &y.chars, &y.index, scratch) as f64 / total as f64;
    let sorted_total = x.sorted.len() + y.sorted.len();
    let rs = if sorted_total == 0 {
        1.0
    } else {
        2.0 * matching_size(&x.sorted, &y.sorted, &y.sorted_index, scratch) as f64
            / sorted_total as f64
    };
    r.max(rs)
}

/// Score every in-kind pair, returning the above-threshold edges, how many pairs were scored,
/// and how many edges the allowlist suppressed.
pub fn score_pairs(
    rules: &[ScoredRule],
    threshold: f64,
    suppressed: &BTreeSet<(String, String)>,
) -> (Vec<Edge>, usize, usize) {
    let mut buckets: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
    for (i, rule) in rules.iter().enumerate() {
        buckets.entry(rule.kind.as_str()).or_default().push(i);
    }
    let prepared: Vec<Prepared> = rules.iter().map(|r| Prepared::of(&r.norm)).collect();
    let widest = prepared
        .iter()
        .map(|p| p.chars.len().max(p.sorted.len()))
        .max()
        .unwrap_or(0);
    let mut scratch = Scratch::for_len(widest);

    let mut edges = Vec::new();
    let mut scored = 0usize;
    let mut suppressed_hits = 0usize;
    for bucket in buckets.values() {
        for (offset, &i) in bucket.iter().enumerate() {
            for &j in &bucket[offset + 1..] {
                let (x, y) = (&rules[i], &rules[j]);
                // Both stubs over one block are already combined; there is nothing to propose.
                if x.extends.is_some() && x.extends == y.extends {
                    continue;
                }
                scored += 1;
                let sim = text_sim_prepared(
                    &prepared[i],
                    &prepared[j],
                    threshold - BONUS_CAP,
                    &mut scratch,
                );
                if sim == 0.0 {
                    continue;
                }
                if x.tokens().min(y.tokens()) < SHORT_TOKENS && sim < SHORT_TEXT_SIM {
                    continue;
                }
                let total = (sim + struct_bonus(x, y)).min(1.0);
                if total < threshold {
                    continue;
                }
                if suppressed.contains(&pair_key(&x.id, &y.id)) {
                    suppressed_hits += 1;
                    continue;
                }
                edges.push(Edge {
                    total,
                    sim,
                    a: i,
                    b: j,
                });
            }
        }
    }
    (edges, scored, suppressed_hits)
}

/// Union-find over the edges: a cluster may chain, so the report prints its weakest edge too.
pub fn cluster(edges: &[Edge], rules: &[ScoredRule]) -> Vec<Cluster> {
    let mut parent: Vec<usize> = (0..rules.len()).collect();
    fn find(parent: &mut [usize], mut a: usize) -> usize {
        while parent[a] != a {
            parent[a] = parent[parent[a]];
            a = parent[a];
        }
        a
    }
    for edge in edges {
        let (ra, rb) = (find(&mut parent, edge.a), find(&mut parent, edge.b));
        if ra != rb {
            parent[ra] = rb;
        }
    }

    let mut groups: BTreeMap<usize, Vec<&Edge>> = BTreeMap::new();
    for edge in edges {
        let root = find(&mut parent, edge.a);
        groups.entry(root).or_default().push(edge);
    }

    let mut out = Vec::new();
    for group in groups.values() {
        let mut members: BTreeMap<&str, &ScoredRule> = BTreeMap::new();
        let mut rendered = Vec::new();
        for edge in group {
            members.insert(rules[edge.a].id.as_str(), &rules[edge.a]);
            members.insert(rules[edge.b].id.as_str(), &rules[edge.b]);
            rendered.push((
                edge.total,
                edge.sim,
                rules[edge.a].id.clone(),
                rules[edge.b].id.clone(),
            ));
        }
        let mut members: Vec<ScoredRule> = members.into_values().cloned().collect();
        members.sort_by(|a, b| (&a.schema, &a.id).cmp(&(&b.schema, &b.id)));
        rendered.sort_by(|a, b| b.0.total_cmp(&a.0));
        out.push(Cluster {
            members,
            edges: rendered,
        });
    }
    out.sort_by(|a, b| {
        let schemas = |c: &Cluster| {
            c.members
                .iter()
                .map(|m| m.schema.as_str())
                .collect::<BTreeSet<_>>()
                .len()
        };
        schemas(b)
            .cmp(&schemas(a))
            .then(b.edges[0].0.total_cmp(&a.edges[0].0))
    });
    out
}

// ---------------------------------------------------------------------------
// the allowlist
// ---------------------------------------------------------------------------

/// The adjudicated keep-distinct pairs, plus every complaint about the file.
///
/// A stale entry is named rather than dropped: an id that no longer resolves means the
/// adjudication it records has lost its subject, and a quietly ignored row is a suppression
/// nobody can see.
pub fn load_allowlist(
    path: &Path,
    live_ids: &BTreeSet<String>,
) -> (BTreeSet<(String, String)>, Vec<String>) {
    let mut suppressed = BTreeSet::new();
    let mut warnings = Vec::new();
    let Ok(text) = std::fs::read_to_string(path) else {
        return (suppressed, warnings);
    };
    let Ok(value) = serde_norway::from_str::<serde_norway::Value>(&text) else {
        warnings.push(format!("{}: does not parse", path.display()));
        return (suppressed, warnings);
    };
    let Some(entries) = value.get("suppressions").and_then(|v| v.as_sequence()) else {
        warnings.push(format!(
            "{}: carries no `suppressions:` list",
            path.display()
        ));
        return (suppressed, warnings);
    };
    for (index, entry) in entries.iter().enumerate() {
        let ids = entry.get("ids").and_then(|v| v.as_sequence());
        let pair = ids.and_then(|ids| match ids.as_slice() {
            [a, b] => Some((a.as_str()?.to_string(), b.as_str()?.to_string())),
            _ => None,
        });
        let Some((a, b)) = pair else {
            warnings.push(format!(
                "allowlist suppressions[{index}]: needs `ids: [a, b]`"
            ));
            continue;
        };
        for id in [&a, &b] {
            if !live_ids.contains(id) {
                warnings.push(format!(
                    "allowlist names {id}: not a live rule ID (stale entry?)"
                ));
            }
        }
        if entry
            .get("reason")
            .and_then(|v| v.as_str())
            .is_none_or(|r| r.trim().is_empty())
        {
            warnings.push(format!("allowlist pair ({a}, {b}): no `reason` recorded"));
        }
        suppressed.insert(pair_key(&a, &b));
    }
    (suppressed, warnings)
}

// ---------------------------------------------------------------------------
// the report
// ---------------------------------------------------------------------------

/// One detector run.
#[derive(Clone, Debug)]
pub struct Report {
    pub clusters: Vec<Cluster>,
    pub scanned: usize,
    pub scored: usize,
    /// Pairs that cleared the threshold and were not suppressed — the edges the clusters are
    /// built from. Reported so a run with no allowlist can say how much it left unsuppressed.
    pub edges: usize,
    pub suppressed_hits: usize,
    /// The allowlist this run actually read, or `None` when no ancestor of the log carried one.
    pub allowlist: Option<PathBuf>,
    pub threshold: f64,
    pub warnings: Vec<String>,
}

/// Run the detector over a replayed state.
///
/// `allowlist` is read when the path exists; a run with no allowlist simply suppresses nothing,
/// which is the honest reading of "there are no adjudications here".
pub fn clusters(state: &State, threshold: f64, allowlist: Option<&Path>) -> Report {
    let mut warnings = Vec::new();
    let scored_rules = rules(state, &mut warnings);
    let live: BTreeSet<String> = scored_rules.iter().map(|r| r.id.clone()).collect();
    let suppressed = match allowlist {
        Some(path) => {
            let (set, mut complaints) = load_allowlist(path, &live);
            warnings.append(&mut complaints);
            set
        }
        None => BTreeSet::new(),
    };
    let (edges, scored, suppressed_hits) = score_pairs(&scored_rules, threshold, &suppressed);
    Report {
        scanned: scored_rules.len(),
        scored,
        edges: edges.len(),
        clusters: cluster(&edges, &scored_rules),
        suppressed_hits,
        allowlist: allowlist.map(Path::to_path_buf),
        threshold,
        warnings,
    }
}

/// The advisory report, as text. Trimmed rule texts, the detector's own classifications, and the
/// stats line the Python prints.
pub fn render_report(report: &Report) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "=== similar-rule clusters (threshold {:.2}) ===",
        report.threshold
    );
    if report.clusters.is_empty() {
        let _ = writeln!(out, "none — no pair clears the threshold");
    }
    for (i, cluster) in report.clusters.iter().enumerate() {
        let tags: Vec<&str> = classify(cluster).iter().map(|t| t.as_str()).collect();
        let kinds: BTreeSet<&str> = cluster.members.iter().map(|m| m.kind.as_str()).collect();
        let best = cluster.edges.first().map_or(0.0, |e| e.0);
        let worst = cluster.edges.last().map_or(0.0, |e| e.0);
        let edge_note = if cluster.edges.len() == 1 {
            format!("best {best:.2}")
        } else {
            format!("best {best:.2} · weakest edge {worst:.2}")
        };
        let _ = writeln!(
            out,
            "\n[{}] {} · kind: {} · {edge_note}",
            i + 1,
            tags.join(" + "),
            kinds.into_iter().collect::<Vec<_>>().join("/")
        );
        for member in &cluster.members {
            let mut marks = Vec::new();
            if member.class == "floor" {
                marks.push("⚑floor".to_string());
            }
            if let Some(extends) = &member.extends {
                marks.push(format!("extends {extends}"));
            }
            let mark = if marks.is_empty() {
                String::new()
            } else {
                format!("  [{}]", marks.join(", "))
            };
            let class = if member.class.is_empty() {
                "?"
            } else {
                member.class.as_str()
            };
            let _ = writeln!(
                out,
                "    {}  ({} · sec.{} · {class}){mark}",
                member.id, member.schema, member.section
            );
            let _ = writeln!(out, "      {}", trim(&member.text, 140));
        }
    }
    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    for cluster in &report.clusters {
        *counts.entry(classify(cluster)[0].as_str()).or_insert(0) += 1;
    }
    let breakdown = if counts.is_empty() {
        "none".to_string()
    } else {
        counts
            .iter()
            .map(|(k, v)| format!("{k} {v}"))
            .collect::<Vec<_>>()
            .join(", ")
    };
    let _ = writeln!(
        out,
        "\n=== stats ===\nrules scanned: {} · in-kind pairs scored: {} · clusters: {} ({breakdown})",
        report.scanned,
        report.scored,
        report.clusters.len()
    );
    // Always say which of the two happened. A run that resolved no allowlist suppresses
    // nothing, and reporting that silently is how adjudicated-and-closed clusters come back as
    // fresh signal with nothing to tell the reader why.
    match &report.allowlist {
        Some(_) => {
            let _ = writeln!(
                out,
                "allowlist-suppressed edges: {}",
                report.suppressed_hits
            );
        }
        None => {
            let _ = writeln!(out, "allowlist: none ({} edges unsuppressed)", report.edges);
        }
    }
    for warning in &report.warnings {
        let _ = writeln!(out, "warning: {warning}");
    }
    out
}

fn trim(text: &str, width: usize) -> String {
    let joined = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if joined.chars().count() <= width {
        return joined;
    }
    let head: String = joined.chars().take(width - 1).collect();
    format!("{head}…")
}

/// The allowlist governing a log, found by walking up from the log directory.
///
/// Resolution never consults the process working directory. The same command over the same log
/// has to produce the same report from anywhere; resolving `./scripts/…` meant a run from
/// outside the repository reported 76 adjudicated-and-closed clusters as fresh signal, with the
/// suppression line simply absent.
pub fn find_allowlist(log_dir: &Path) -> Option<PathBuf> {
    let start = log_dir
        .canonicalize()
        .unwrap_or_else(|_| log_dir.to_path_buf());
    start
        .ancestors()
        .map(|dir| dir.join(ALLOWLIST))
        .find(|candidate| candidate.is_file())
}

/// The document a scored rule came from, for callers that want to address it.
pub fn doc_of(rule: &ScoredRule, state: &State) -> Option<DocRef> {
    state
        .docs
        .keys()
        .find(|doc| {
            doc.name == rule.schema && matches!(doc.kind, DocKind::Command | DocKind::Skill)
        })
        .cloned()
}
