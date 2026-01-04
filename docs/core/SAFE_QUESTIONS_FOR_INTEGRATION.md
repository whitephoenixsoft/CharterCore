# What Is a “Safe Question”?

*This is meant for systems integrating with Charter Core.*

A safe question is a query that:
- Does not require semantic inference
- Does not require interpreting intent
- Does not expand authority
- Does not retroactively reinterpret history

Charter Core guarantees that answers to safe questions are:
- Deterministic
- Auditable
- Stable over time

---
# Canonical Safe Questions

### Legitimacy & Status

- Does an active resolution exist for X?
- What Area does this resolution belong to?
- Under which Authority was this resolution accepted?
- What Scope was active at the time of acceptance?
- What is the current lifecycle state of this resolution?
- Has this resolution been superseded or retired?

#### Authority & Governance

- What is the active Authority for this Area?
- What decision rule does this Authority enforce?
- Was the Authority rule satisfied at acceptance time?
- What Authority was active when this decision was made?

### Sessions & History

- Which session accepted this resolution?
- Who were the participants in that session?
- Was the session blocked or paused at any point?
- What Authority and Scope were active during the session?

### Cross-Area Awareness

- Does this resolution reference other Areas?
- Which external resolutions reference this Area?
- What decisions outside this Area claim relevance here?

*(Note: visibility only — no authority implied)*

### Change & Evolution

- What resolution superseded this one?
- What resolution did this supersede?
- When did governance rules change for this Area?

---
# Explicitly Unsafe Questions (By Design)

Charter Core **will not** answer:
- Does this decision violate intent?
- Should this decision apply to another Area?
- Are these scopes “the same”?
- Who should have authority?
- Is this decision “correct”?
- Does this policy conflict with that one semantically?

These questions require:
- Human judgment
- Domain knowledge
- Or facilitation layers (AI, policy engines, workflows)

They are intentionally out of scope.

---
## Why This Matters 

> Charter Core answers questions of legitimacy, not questions of wisdom.

Downstream systems may:
- Act on answers
- Enforce gates
- Trigger workflows
- Ask humans or AI for interpretation

But Charter Core itself remains neutral, deterministic, and auditable.