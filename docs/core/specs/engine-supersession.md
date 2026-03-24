# ENG-SUPERSESSION — Supersession Graph, Structural ACTIVE Derivation & Conflict Authority

Status: REFACTORED (v8 – Intra-Area Informational Reference Clarification)  
Applies to: Engine Core (V1/V2+)  

Authority: Foundational authority for supersession graph semantics, structural ACTIVE derivation, and graph-level conflict detection.

Subordinate references consumed from:

- ENG-DOMAIN
- ENG-INTEGRITY
- ENG-REVIEW-RETIRED
- ENG-RECEIPT
- ENG-CANON
- ENG-SPECVERIFY
- ENG-COMPILATION
- ENG-SESSION
- ENG-DECISION

---

# 1. Purpose

ENG-SUPERSESSION defines the graph semantics of Resolution supersession.

It is the authoritative specification for:

- supersession edge structure
- structural ACTIVE derivation
- graph acyclicity
- Area-local supersession sovereignty
- graph-level conflict detection
- first-successful-accept race semantics
- governance slot participation in structural supersession
- deterministic graph reconstruction during restore
- structural interaction with incremental compilation ordering

ENG-SUPERSESSION does not redefine:

- object schemas
- runtime halt conditions
- UNDER_REVIEW / RETIRED usability suspension semantics
- receipt structure
- canonical serialization rules
- atomic acceptance persistence boundaries
- session lifecycle state transitions

Those are defined respectively in:

- ENG-DOMAIN
- ENG-INTEGRITY
- ENG-REVIEW-RETIRED
- ENG-RECEIPT
- ENG-CANON
- ENG-PERSISTENCE
- ENG-SESSION

ENG-SUPERSESSION is the authority for graph truth, not for all runtime consequences of that truth.

---

# 2. Single-Area Supersession Doctrine

## ENG-SUPERSESSION-01 — Area-Local Graph Sovereignty

The supersession graph is scoped to exactly one Area.

Rules:

- supersession evaluation must occur only within the active Area
- structural supersession edges must not cross Area boundaries
- ACTIVE derivation must be computed strictly from Area-local structural objects
- cross-area references may exist only as informational metadata
- intra-Area informational Resolution references may exist, but they are not supersession edges

Mixed-area structural supersession is invalid.

Runtime halt conditions for mixed-area structural graphs are enforced by ENG-INTEGRITY.  
ENG-SUPERSESSION defines that such graphs are not semantically valid.

---

# 3. Graph Structure

## ENG-SUPERSESSION-02 — Directed Immutable Supersession Edges

Resolutions may supersede zero or more prior Resolutions within the same Area.

Supersession edges are:

- explicit
- directional
- immutable
- created only through valid acceptance as coordinated elsewhere

ENG-SUPERSESSION defines the graph meaning of those edges.  
ENG-PERSISTENCE defines the atomic durability boundary within which they are committed.

---

## ENG-SUPERSESSION-03 — Acyclic Requirement

The supersession graph must remain acyclic.

Cycle detection must be applied whenever graph validity is evaluated, including:

- acceptance-time graph validation
- restore / rehydration reconstruction
- historical replay or compilation graph reconstruction

If a cycle exists, the graph is structurally invalid.

ENG-INTEGRITY determines halt behavior.  
ENG-SUPERSESSION determines that the graph cannot be interpreted.

---

## ENG-SUPERSESSION-04 — Area Consistency of Edges

Every structural supersession edge must satisfy:

- successor.area_id == predecessor.area_id

Cross-area supersession edges are prohibited.

Reference resolution classification is defined in ENG-DOMAIN.  
Halt semantics are defined in ENG-INTEGRITY.

---

# 4. Structural ACTIVE Derivation

## ENG-SUPERSESSION-05 — Structural ACTIVE Is a Graph Property

A Resolution is structurally ACTIVE if and only if:

- its lifecycle state is ACTIVE
- no accepted successor exists within the same Area supersession graph

This is the authoritative definition of structural ACTIVE.

ENG-SUPERSESSION alone defines structural ACTIVE derivation.  
Other specifications must reference this derivation rather than redefine it.

---

## ENG-SUPERSESSION-06 — Structural ACTIVE Is Distinct from Usability

Structural ACTIVE is not the same as forward usability.

UNDER_REVIEW and RETIRED do not alter structural ACTIVE derivation.

They may suspend usability in legitimacy evaluation, but that suspension is defined in:

- ENG-REVIEW-RETIRED

ENG-SUPERSESSION therefore distinguishes:

- structural graph truth
- runtime usability consequences

This separation must remain explicit.

---

# 5. Governance Slot Participation

## ENG-SUPERSESSION-07 — Governance Objects Participate in the Graph

Authority and Scope Resolutions participate fully in the supersession graph.

Their structural graph behavior is the same as other Resolutions unless otherwise constrained by authoritative state semantics elsewhere.

ENG-SUPERSESSION defines:

- they may occupy graph nodes
- they may be supersession predecessors or successors where permitted
- their ACTIVE status is derived by the same structural graph rules

ENG-REVIEW-RETIRED defines which governance states are usable or disallowed.  
ENG-INTEGRITY enforces whether a runtime may continue when governance structure becomes unsafe.

---

## ENG-SUPERSESSION-08 — Governance Slot Structural Outcomes

Graph consequences include:

- a superseded Authority is not structurally ACTIVE
- a superseded Scope is not structurally ACTIVE
- a newly accepted successor becomes the structurally ACTIVE graph representative if no further accepted successor exists

Runtime consequences for sessions referencing obsolete governance artifacts are not defined here.  
Those belong to:

- ENG-DECISION
- ENG-SESSION
- ENG-INTEGRITY

---

# 6. Structural References

## ENG-SUPERSESSION-09 — Supersession References Are Structural

A supersession reference is a structural graph edge.

It must:

- resolve locally within the Area
- reference a valid Resolution node
- participate in deterministic graph reconstruction

Missing supersession references invalidate the graph.

ENG-DOMAIN defines the field structure.  
ENG-INTEGRITY defines halt behavior.

---

## ENG-SUPERSESSION-10 — Informational References Are Never Graph Edges

Cross-area or informational references:

- must not be interpreted as supersession edges
- must not affect ACTIVE derivation
- must not affect governance slot graph participation
- must not affect graph conflict detection

This includes:

- cross-area informational references
- intra-Area informational Resolution references

Graph semantics apply only to structural supersession references.

---

# 7. Acceptance Race Semantics

## ENG-SUPERSESSION-11 — First Successful Acceptance Wins

If multiple acceptance attempts compete to supersede the same structural target set:

- the first successfully committed acceptance wins
- later competing attempts become graph-invalid for acceptance purposes

This is the authoritative graph conflict rule.

ENG-PERSISTENCE defines how commit atomicity preserves that outcome.  
ENG-DECISION and ENG-SESSION consume the outcome for session blocking semantics.

ENG-SUPERSESSION does not define session lifecycle transitions such as BLOCK_PERMANENT; it defines the graph conflict truth that causes them.

---

## ENG-SUPERSESSION-12 — No Graph Merge by Heuristic

The supersession graph does not support automatic branch merge.

Where incompatible accepted successors would create ambiguity, deterministic failure or first-successful acceptance semantics must prevail according to the relevant execution context.

Historical replay ordering for compilation is defined in ENG-COMPILATION.  
Live acceptance ordering is governed by atomic commit timing, not timestamps.

---

# 8. Graph-Level Conflict Detection

## ENG-SUPERSESSION-13 — Authoritative Conflict Conditions

A supersession conflict exists when a proposed graph mutation would:

- reference a structurally non-ACTIVE target where ACTIVE is required
- introduce a cycle
- introduce cross-area supersession
- reference missing structural predecessors
- violate graph-level governance slot structure
- conflict with already accepted supersession outcomes
- rely on structurally invalid graph state

ENG-SUPERSESSION is the authority for determining that these are graph conflicts.

Other specifications may define how the runtime reacts to those conflicts, but must not redefine the graph conditions themselves.

---

## ENG-SUPERSESSION-14 — Evaluation and Commit-Time Recheck

Graph conflict detection must be valid both:

- during pre-acceptance evaluation
- immediately before graph mutation is committed

Acceptance orchestration belongs to ENG-DECISION.  
Atomic mutation belongs to ENG-PERSISTENCE.

ENG-SUPERSESSION defines the graph truths both stages must consume.

---

# 9. Incremental Compilation Integration

## ENG-SUPERSESSION-15 — Historical Replay Must Respect Graph Determinism

Historical replay and incremental compilation must reconstruct graph outcomes deterministically.

ENG-SUPERSESSION requires that graph reconstruction consume:

- accepted supersession edges
- structurally valid Resolution states
- canonical historical precedence as supplied by ENG-COMPILATION

ENG-SUPERSESSION does not define the historical ordering algorithm itself.  
That belongs to ENG-COMPILATION.

It does define that, once such ordering is applied, the resulting supersession graph must still satisfy all structural graph rules in this document.

---

## ENG-SUPERSESSION-16 — Historical Ordering Does Not Change Graph Semantics

Compilation ordering may determine which historical edge wins when replaying competing legitimate artifacts.

It does not redefine:

- what a supersession edge means
- what structural ACTIVE means
- what constitutes a cycle
- what constitutes a cross-area violation

Graph semantics remain invariant across runtime and compilation contexts.

---

# 10. Resolution Lifecycle Interaction

## ENG-SUPERSESSION-17 — Structural Interaction with Resolution States

ENG-SUPERSESSION consumes lifecycle state only where it affects graph interpretation.

Graph-relevant principles:

- ACTIVE may participate in structural ACTIVE derivation
- SUPERSEDED is graph-terminal as a predecessor outcome
- UNDER_REVIEW does not change graph structure
- RETIRED does not change graph structure

Usability consequences of UNDER_REVIEW and RETIRED are defined only in ENG-REVIEW-RETIRED.

ENG-SUPERSESSION must not independently redefine administrative suspension semantics.

---

# 11. Restore & Rehydration Guarantees

## ENG-SUPERSESSION-18 — Deterministic Graph Reconstruction

Given identical structural objects and identical supersession edges, graph reconstruction must produce identical:

- ACTIVE Resolution sets
- supersession predecessor/successor relationships
- governance slot structural outcomes

No heuristic ordering is permitted.

Restore-time runtime safety, halt, or degraded-mode decisions are not defined here.  
Those belong to ENG-INTEGRITY.

ENG-SUPERSESSION defines only the graph reconstruction truth that those runtime decisions consume.

---

# 12. Receipt Relationship

## ENG-SUPERSESSION-19 — Graph Semantics May Depend on Valid Historical Legitimacy Artifacts but Do Not Define Receipt Rules

ENG-SUPERSESSION may require that accepted graph edges correspond to structurally valid legitimacy history.

However, it does not define:

- receipt schema
- canonical serialization
- content_hash rules
- rule provenance verification

Those belong to:

- ENG-RECEIPT
- ENG-CANON
- ENG-SPECVERIFY

ENG-SUPERSESSION only depends on the runtime result that accepted historical graph mutations are legitimate and structurally admissible.

---

# 13. Relationship to Runtime Blocking

## ENG-SUPERSESSION-20 — Graph Truth Precedes Session Consequence

Graph truths established by ENG-SUPERSESSION may cause runtime consequences such as:

- inability to accept
- invalid governance context
- obsolete references
- session blocking

Those consequences are applied by:

- ENG-DECISION
- ENG-SESSION
- ENG-INTEGRITY

ENG-SUPERSESSION must not redefine session lifecycle state transitions.

---

# 14. Engine Invariants

- supersession edges immutable
- supersession graph strictly Area-local
- graph must remain acyclic
- structural ACTIVE derivation deterministic
- governance objects participate in graph structure
- cross-area informational references are never graph edges
- intra-Area informational Resolution references are never graph edges
- informational references must not affect ACTIVE derivation or conflict detection
- first successful accepted graph mutation wins for conflicting live supersession attempts
- UNDER_REVIEW / RETIRED do not alter structural ACTIVE
- compilation may affect historical replay precedence but not graph semantics
- supersession graph rules must be consumed, not redefined, by dependent specifications

---

# 15. Mental Model

ENG-SUPERSESSION defines graph truth.

It answers:

- which Resolutions structurally supersede which others
- which Resolutions are structurally ACTIVE
- whether a proposed supersession mutation is graph-valid
- whether graph history is reconstructable deterministically

It does not answer:

- whether a session may resume
- whether a Resolution is usable for legitimacy despite being structurally ACTIVE
- whether the Engine must halt or degrade
- whether informational references imply graph semantics
- how receipts are serialized
- how acceptance is persisted atomically

Those belong elsewhere.

ENG-SUPERSESSION is the graph layer.  
Other specifications must build on it rather than duplicate it.