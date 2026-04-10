# Charter Core — Query Philosophy
Document ID: ENG-QUERY-FOUNDATION  
Status: FROZEN  
Audience: Charter Core engine implementers  
Scope: Engine-level query behavior only

---

## Purpose

This document defines the **philosophical and structural constraints** governing all queries in Charter Core.

It ensures that querying Charter state:

- never alters legitimacy  
- never mutates history  
- never introduces semantic authority  
- never weakens audit or determinism guarantees  

Queries are a *lens*, not a *force*.

---

## Core Principle

> A query may observe and report structural and derived facts —  
> but it may **never cause anything to happen**.

Queries exist solely to expose **facts that already exist** in Charter Core or deterministic evaluations over those facts.

---

## What a Query Is

A query is:

- a **pure function** over engine state  
- deterministic given identical input state  
- read-only by construction  
- safe to repeat indefinitely  

A query:

- does not create objects  
- does not emit audit events  
- does not update references  
- does not alter indexes  
- does not repair or enforce invariants  

---

## What a Query Is Not

A query is NOT:

- a mutation mechanism  
- a repair tool  
- a source of legitimacy creation  
- a semantic interpreter  
- a source of inferred intent  

If a result implies a problem, **the problem already exists**.  
The query did not cause it.

---

## Legitimacy Boundary

Queries:

- MUST NOT accept resolutions  
- MUST NOT create legitimacy  
- MUST NOT infer consent  
- MUST NOT treat silence as intent  
- MUST NOT collapse disagreement  

Queries MAY:

- evaluate legitimacy status  
- evaluate authority rules  
- report candidate eligibility  
- report blocking conditions  

Legitimacy is created only by:

- sessions  
- authority evaluation during acceptance  
- explicit acceptance  

Queries may *describe* legitimacy.  
They may never *create* it.

---

## Runtime Mode Awareness

Queries must operate according to the current runtime mode.

- In normal operation: full deterministic results are returned  
- In degraded or restricted modes:  
  - queries may be limited or restricted  
  - queries must not produce misleading results  

Queries must remain:

- deterministic  
- non-mutating  
- explicit about limitations when applicable  

---

## Immutability Guarantee

All data returned by queries reflects:

- immutable domain state  
- recorded history  
- current references (as pointers, not historical truth)  
- deterministic evaluations over that state  

Nothing returned by a query may be:

- speculative  
- hypothetical  
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

Ambiguity is forbidden.

---

## Ordering Semantics

Queries MUST NOT imply ordering unless explicitly requested.

Default results are:

- unordered  
- set-based  
- stable by identity, not sequence  

If ordering is required:

- it must be explicitly requested  
- it must be documented  
- it must be deterministic  

---

## Relationship to Indexes

Queries MAY use indexes for performance.

Queries MUST NOT:

- depend on index correctness for meaning  
- expose index internals  
- fail semantically due to missing indexes  

Index loss must not alter query truth — only performance.

---

## Relationship to Validation

Queries and validation are distinct:

- Queries observe and report  
- Validation enforces rules during mutation and evaluation  

Queries MAY:

- report invariant violations  
- surface structural or decision errors  

Queries MUST NOT:

- enforce invariants  
- repair inconsistencies  
- mutate state  

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
- a higher-layer concern  

The engine reports facts, not narratives.

---

## Naming & Surface Discipline

Query names SHOULD:

- reflect the fact being retrieved  
- avoid semantic interpretation  
- remain mechanically descriptive  

Examples:

- `get_active_resolutions` ✔  
- `get_correct_decisions` ✘  

---

## Version Discipline

This philosophy is stable.

Future versions may:

- add new queries  
- add new projections  
- add new consumers  

They must not:

- introduce side effects  
- blur querying with mutation  
- introduce semantic interpretation  
- weaken determinism  

---

## Mental Model

Objects are facts.  
References are pointers.  
Indexes are scratchpads.  
Queries are windows.

Nothing you see through a window  
changes the system.

---

## Final Statement

If a query ever:

- mutates state  
- affects legitimacy  
- introduces inference or interpretation  
- or alters outcomes  

then the query system has violated Charter Core.