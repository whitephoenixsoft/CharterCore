# Charter Core — Query Philosophy
Document ID: ENG-QUERY-FOUNDATION  
Status: FROZEN (v1)  
Audience: Charter Core engine implementers  
Scope: Engine-level query behavior only

---

## Purpose

This document defines the **philosophical and structural constraints** governing
all queries in Charter Core.

It exists to ensure that querying Charter state:
- never alters legitimacy
- never mutates history
- never introduces semantic authority
- never weakens audit guarantees

Queries are a *lens*, not a *force*.

---

## Core Principle

> A query may observe anything, infer anything, and explain anything —  
> but it may **never cause anything to happen**.

Queries exist solely to expose **facts that already exist** in Charter Core.

---

## What a Query Is

A query is:
- a **pure function** over immutable engine state
- deterministic given identical input state
- read-only by construction
- safe to repeat indefinitely

A query:
- does not create objects
- does not emit audit events
- does not update refs
- does not alter indexes
- does not assert or repair invariants

---

## What a Query Is Not

A query is NOT:
- a validation mechanism
- an assertion
- a repair tool
- a decision mechanism
- a legitimacy check
- a semantic interpreter

If a result implies a problem, **the problem already exists**.
The query did not cause it.

---

## Legitimacy Boundary

Queries:
- MUST NOT accept resolutions
- MUST NOT evaluate authority
- MUST NOT infer consent
- MUST NOT treat silence as intent
- MUST NOT collapse disagreement

Legitimacy is created only by:
- sessions
- authority evaluation
- explicit acceptance

Queries may *describe* legitimacy.
They may never *create* it.

---

## Engine Safety Precondition

All queries MUST validate that the engine is in a **SAFE state** before execution.

If core invariants are violated:
- the query MUST fail
- no partial or degraded output is permitted

Rationale:
> Returning answers from an invalid engine state is equivalent to lying.

---

## Immutability Guarantee

All data returned by queries represents:
- immutable facts
- recorded history
- current refs (as pointers, not truth)
- derived index views (as performance aids)

Nothing returned by a query may be:
- transient runtime state
- speculative or hypothetical
- dependent on future actions

---

## Error vs Empty Semantics

Queries MUST follow strict existence rules:

- If the addressed entity does not exist → **ERROR**
- If the entity exists but has no members/results → **EMPTY SET**

This applies uniformly to:
- areas
- scopes
- sessions
- resolutions
- baselines

Ambiguity is forbidden.

---

## Ordering Semantics

Queries MUST NOT imply ordering unless explicitly requested.

Default results are:
- unordered
- set-based
- stable only by identity, not sequence

If ordering matters:
- it must be named
- it must be specified
- it must be documented

---

## Relationship to Indexes

Queries MAY use indexes for performance.

Queries MUST NOT:
- depend on index correctness for meaning
- expose index internals
- fail semantically due to missing indexes

Index loss must not alter query truth — only speed.

---

## Relationship to Assertions

Queries and assertions are strictly separated.

- Queries observe
- Assertions enforce

Assertions occur:
- during mutations
- at transaction boundaries
- during invariant validation

Queries MUST NOT:
- assert correctness
- repair inconsistencies
- auto-heal state

---

## Output Contract

All query outputs MUST be:
- machine-consumable
- explicit
- structurally typed
- semantically minimal

Human-readable interpretation is:
- a CLI concern
- a UI concern
- a V5 guidance concern

The engine reports facts, not stories.

---

## Naming & Surface Discipline

Query names SHOULD:
- align with CLI commands where practical
- reflect the fact being retrieved
- avoid semantic interpretation

Example:
- `get_active_resolutions` ✔
- `get_correct_decisions` ✘

---

## Version Discipline

This philosophy is version-stable.

Future versions may:
- add new queries
- add new projections
- add new consumers

They may NOT:
- change query purity
- introduce side effects
- blur querying with decision-making

---

## Mental Model

Objects are facts.  
Refs are pointers.  
Indexes are scratchpads.  
Queries are windows.

Nothing you see through a window
changes the building.

---

## Final Statement

If a query ever:
- changes behavior
- affects outcomes
- influences legitimacy
- or makes decisions easier than explaining them

Then the query system has violated Charter Core.