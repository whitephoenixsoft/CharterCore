# CharterCore

A governance engine for recording agreed decisions within explicit scope and authority.

It provides a structured way for individuals and teams to agree on decisions
("resolutions") within explicit scope and authority, and to preserve those
agreements over time.

Charter does not manage work, execute actions, or enforce policies.
It records legitimacy and intent.

---

## Core Concepts

### Area
A governance boundary that defines:
- Scope (what kinds of decisions are allowed)
- Authority (who may accept decisions)
- Templates (how decisions are structured)

Areas enable federated governance across teams and domains.

---

### Session
A structured, auditable conversation used to:
- Propose decisions
- Validate scope
- Validate authority
- Detect conflicts

Sessions may be:
- Active
- Blocked (due to conflicts or overreach)
- Resumed
- Closed

Sessions preserve reasoning but do not mutate decisions silently.

---

### Resolution
An immutable record of an agreed decision.

Properties:
- Accepted under explicit authority
- Bound to a scope
- Superseded (not edited) when changed
- Fully auditable

Resolutions are living documents through lineage, not mutation.

---

### Templates
Structured formats for resolutions.
Templates enforce consistency and domain-specific language,
but do not enforce semantics.

Templates belong to Areas.

---

### Authority
Explicit rules that define who may accept resolutions.
Authority is evaluated at acceptance time and is non-retroactive.

---

### Scope
Explicit boundaries that define what kinds of decisions are legitimate.
Scope violations block acceptance and must be resolved explicitly.

---

### AI Assistance (Optional)
AI may assist by:
- Facilitating sessions
- Summarizing discussions
- Drafting candidate resolutions

AI may not:
- Accept resolutions
- Change authority
- Change scope
- Execute actions

Charter must function fully without AI.

---

## What Charter Is Not

- Not a task manager
- Not a workflow engine
- Not a policy enforcement system
- Not a configuration manager
- Not a source of execution truth

Charter integrates with existing tools rather than replacing them.

---

## Design Principles

- Decisions are explicit
- Authority is visible
- Scope is enforced
- History is preserved
- No silent state changes
- Process correctness over automation

---

## MVP Scope

The initial implementation is intentionally minimal:
- Single Area
- Single-user authority
- Hard-coded rules
- Monolithic architecture

The goal is correctness and trust, not scale.

---

## Status

Early design / MVP implementation.
Expect rapid iteration and intentional constraints.

