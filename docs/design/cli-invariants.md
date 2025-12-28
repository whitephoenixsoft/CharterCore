# CLI Invariants — Naming, Identity, Audit, and Ergonomics

These are CLI-layer invariants only.
Violations do not imply engine bugs, but CLI correctness failures.

## Identity & Naming

### CLI-INV-01: Engine Identity Is Canonical

Every Area, Session, Resolution, and Scope has a canonical **engine ID***.

Engine IDs are:
- Content-addressed hashes when possible
- UUIDs otherwise
Rules:
- CLI-visible labels NEVER replace engine IDs
- All engine operations ultimately resolve to engine IDs
#### Fail if
- Any CLI operation relies on a label as authoritative identity

### CLI-INV-02: Labels Are Ergonomic, Not Semantic

Labels exist only for human usability.

Rules:
- Labels do not encode meaning
- Labels do not affect legitimacy
- Labels do not affect authority or scope
#### Fail if
- CLI logic interprets label structure or naming patterns

### CLI-INV-03: Labels Are Area-Scoped by Default

User-assigned labels (e.g. R-1, Auth-Core, Scope-Infra) must be unique **within an Area**.

Rules:
- CLI must never assume global label uniqueness
- Cross-area operations require explicit qualification
#### Fail if
- A label collision across Areas causes ambiguity or hidden coupling

#### CLI-INV-04: Area Context Is Required for Unqualified Labels

If a command references a label without an Area qualifier:
- The CLI must have an active Area context, OR
- The command must fail with a disambiguation prompt

#### Pass examples
```Bash
charter area use platform
charter resolution show R-2
```
#### Fail example
```Bash
charter resolution show R-2
error: label ambiguous without area context
```

### CLI-INV-05: Global Disambiguation Is Always Explicit

When listing or searching across Areas, labels MUST be rendered as:

`<area>/<label>`

#### Example
```
platform/R-3
finance/R-1
security/R-7
```
#### Fail if
- Bare labels appear in a global view

### CLI-INV-06: Global Label Uniqueness Is Optional and Explicit

The CLI MAY support an optional configuration for globally unique labels.

Rules:
- Must be opt-in
- Must be reversible
- Must be documented as increasing merge/import friction
#### Fail if
- Global uniqueness is silently enforced

---

## Audit Invariants

### CLI-AUD-01: Audits Are Read-Only

No audit command may mutate state.

### CLI-AUD-02: Audits Do Not Infer Meaning

Audit commands must not:
- Judge correctness
- Flag violations
- Infer scope breaches

They may only report **recorded facts**.

### CLI-AUD-03: Audits Are Deterministic

Same input → same output.

No timestamps, randomness, or reordering unless explicitly requested.

### CLI-AUD-04: Grep-Friendliness Is a First-Class Constraint

- Audit output must obey:
    - One event per line
    - Stable field order
- Explicit keywords:
    - AREA
    - SESSION
    - RESOLUTION
    - AUTH
    - SCOPE
    - REVIEW 
- Never wrap lines
- Engine IDs always included

This is **non-negotiable**.

---
## Ergonomics & Authority Awareness

### CLI-ERG-01: Authority-Aware Command Collapsing

When the active Authority rule makes individual voting redundant (e.g. SOLO):
- The CLI MAY collapse vote + accept into a single user action
- The engine must still record:
    - Vote(s)
    - Acceptance
    - Authority context
#### Fail if
- Mechanical history is skipped or elided

### CLI-ERG-02: Explicitness Beats Convenience

The CLI may reduce keystrokes, but must never:
- Skip legitimacy checks
- Infer intent
- Apply defaults that create decisions
#### Fail if
- A resolution can be accepted without a conscious user action

---
## Context & Safety

### CLI-CTX-01: Context Switching Is Explicit

Area switches, session pauses, or review mode entry must be explicit commands.
#### Fail if
- The CLI silently changes context
### CLI-CTX-02: Import and Review Are Separate Mental Modes

The CLI must visually and structurally distinguish:
- Normal decision-making
- Import review / consolidation
#### Fail if
- Imported resolutions feel indistinguishable from locally created ones

