# Codebase Analysis Template

This template defines the structure for brownfield codebase analysis output. It follows
the deliverable envelope ([`artifact-format.md`](artifact-format.md)) **slimmed but
legible**: findings as file-cited table rows, one line each; the judgment prose
(strengths, recommendations) stays prose — the analysis checkpoint's human reader needs it.

---

```markdown
# Codebase Analysis

> Generated: {{timestamp}}
> Mode: brownfield-setup
> Status: {{status}}

---

## Part 1: Inventory (Factual)

### Project Identity

| Aspect | Value | Source |
|--------|-------|--------|
| Name | {{project_name}} | {{source_file}} |
| Primary Language | {{language}} {{version}} | {{source_file}} |
| Framework | {{framework}} {{version}} | {{source_file}} |
| Package Manager | {{package_manager}} | {{source_file}} |
| Entry Points | {{entry_points}} | {{detection}} |

### Directory Structure

```
{{project_root}}/
├── {{dir_1}}/            # {{purpose_1}}
├── {{dir_2}}/            # {{purpose_2}}
├── {{dir_3}}/            # {{purpose_3}}
└── ...
```

### Detected Patterns

#### Architecture Pattern

| Pattern | Evidence |
|---------|----------|
| {{pattern_name}} | {{files_or_dirs_indicating_pattern}} |

#### Naming Conventions

| Element | Convention | Example |
|---------|------------|---------|
| Files | {{file_convention}} | {{example}} |
| Variables | {{var_convention}} | {{example}} |
| Functions | {{func_convention}} | {{example}} |
| Classes | {{class_convention}} | {{example}} |

#### Error Handling Pattern

| Pattern | Evidence |
|---------|----------|
| {{error_pattern}} | {{files_showing_pattern}} |

#### Test Pattern

| Aspect | Value |
|--------|-------|
| Framework | {{test_framework}} |
| Location | {{test_dir}} |
| Naming | {{test_naming_convention}} |
| Coverage Config | {{coverage_config_file}} |

### Domain Entities

| Entity | File | Field Count | Relationships |
|--------|------|-------------|---------------|
| {{entity_name}} | {{file_path}} | {{count}} | {{relationship_list}} |

### External Dependencies

| Service | Access Pattern | Config Location |
|---------|---------------|-----------------|
| {{service_name}} | {{how_accessed}} | {{config_file}} |

---

## Part 2: Assessment (Judgment)

### Strengths to Preserve

1. **{{strength_1}}**: {{why_it_matters}}
2. **{{strength_2}}**: {{why_it_matters}}
3. **{{strength_3}}**: {{why_it_matters}}

### Inconsistencies Found

| Area | Finding | Severity | Location |
|------|---------|----------|----------|
| {{area}} | {{what_inconsistent}} | low\|medium\|high | {{files}} |

### Essential Floor Status

| Category | Check | Status | Evidence (file-cited, one line) |
|----------|-------|--------|--------------------------------|
| Security | Auth at boundaries | {{present\|partial\|absent}} | {{evidence}} |
| Security | Secrets from env | {{status}} | {{evidence}} |
| Security | Input validation | {{status}} | {{evidence}} |
| Testing | Test framework configured | {{status}} | {{evidence}} |
| Testing | Test files present | {{status}} | {{count}} files found |
| Testing | CI runs tests | {{status}} | {{evidence}} |
| Error Handling | Explicit error types | {{status}} | {{evidence}} |
| Error Handling | Context preservation | {{status}} | {{evidence}} |
| Error Handling | Appropriate status codes | {{status}} | {{evidence}} |
| Observability | Structured logging | {{status}} | {{evidence}} |
| Observability | Correlation IDs | {{status}} | {{evidence}} |
| Observability | No PII in logs | {{status}} | {{evidence}} |

**Category rollup:** Security {{status}} · Testing {{status}} · Error Handling {{status}} · Observability {{status}}

### Recommended Constitution Focus

Based on this analysis, the constitution should:

1. {{recommendation_1}}
2. {{recommendation_2}}
3. {{recommendation_3}}

---

## Appendix: Detection Method

| Aspect | Method Used |
|--------|-------------|
| Tech Stack | `detect-stack.sh` script |
| Architecture | Directory pattern matching |
| Entities | ORM/framework-specific patterns |
| Conventions | File sampling and pattern inference |
```

---

## Usage Notes

1. **Status values**: `draft` (initial generation), `confirmed` (user approved at checkpoint)
2. **Essential Floor statuses**:
   - `present` - Fully implemented
   - `partial` - Some coverage, gaps exist
   - `absent` - Not implemented
3. **Severity levels**: `low` (cosmetic), `medium` (should fix), `high` (blocking)
4. **This is a reference template** - Agent produces actual content following this structure
