# Relationship Patterns

How `data-model.md` documents relationships. Cardinality and delete semantics are standard; this
file fixes the documentation formats.

## Relationship entry

Document each cross-entity relationship under `## Relationships` with this table. Use the rows that
apply to its cardinality:

```markdown
### User -> Tasks (1:N)

| Aspect | Value |
|--------|-------|
| **Type** | One-to-Many |
| **From** | User (one) |
| **To** | Task (many) |
| **Foreign Key** | task.userId |
| **Required** | Task requires User |
| **On Delete** | Cascade (delete tasks) |
```

- **1:1** puts the foreign key on the optional side and marks it unique (`profile.userId (unique)`).
- **N:M** replaces the foreign-key rows with `**Join Entity**` and `**Additional Fields**`.
- **Self-referential** relationships say so in the heading (`Category -> Subcategories (1:N Self)`)
  and use the `**Entity**` row in place of `From` / `To`.
- **On Delete** takes one of Cascade, Set Null or Restrict, and is never left blank.

## Join entity

Every N:M relationship has a join entity, written as a full entity entry:

```markdown
### Entity: ProjectMember

**Lifecycle:** proposed (<key>)

> Join entity for User-Project relationship.

#### Attributes

| Attribute | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| id | UUID | Yes | auto | Unique identifier |
| userId | Reference(User) | Yes | - | Member user |
| projectId | Reference(Project) | Yes | - | Project |
| role | Enum[owner,admin,member,viewer] | Yes | member | Member role |
| joinedAt | Timestamp | Yes | auto | When user joined |

#### Constraints

- Unique: (userId, projectId) - user can join project once
```

## Relationships inside an entity

Each entity lists its own relationships:

```markdown
### Relationships

| Relationship | Type | Target | Description |
|--------------|------|--------|-------------|
| owner | N:1 | User | Task owner |
| comments | 1:N | Comment | Task comments |
| assignees | N:M | User | Assigned users (via TaskAssignment) |
```

## Diagram notation

```
User ──1:N──▶ Task (owns)
User ◀──N:M──▶ Project (via ProjectMember)
Task ──N:1──▶ Project (belongs to)
Category ──1:N──▶ Category (parent-child)
```

| Symbol | Meaning |
|--------|---------|
| `──1:N──▶` | One-to-many (arrow points to many) |
| `──N:1──▶` | Many-to-one (arrow points to one) |
| `◀──N:M──▶` | Many-to-many (bidirectional) |
| `──1:1──▶` | One-to-one |
| `(via Entity)` | Join entity for N:M |
| `(verb)` | Relationship description |

## Validation checklist

- [ ] Every Reference type has relationship documented
- [ ] Cardinality is explicit (1:1, 1:N, N:M)
- [ ] Foreign key location is specified
- [ ] Delete behavior is documented
- [ ] N:M relationships have join entity defined
- [ ] Join entities have necessary additional fields
- [ ] Self-referential relationships are clearly marked
- [ ] Diagram matches relationship documentation
