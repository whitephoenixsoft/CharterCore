# ENG-STRUCTURE — Structural Graph, ACTIVE Derivation & Conflict Authority

Status: REFACTORED (v9 – Structural Graph Renaming, ON_HOLD Alignment, Informational Reference Separation)  
Applies to: Engine Core (V1/V2+)  

Authority: Foundational authority for structural graph semantics, structural ACTIVE derivation, and graph-level conflict detection.

Subordinate references consumed from:

- ENG-DOMAIN
- ENG-INTEGRITY
- ENG-USABILITY
- ENG-RECEIPT
- ENG-CANON
- ENG-SPECVERIFY
- ENG-COMPILATION
- ENG-SESSION
- ENG-DECISION

---

# 1. Purpose

ENG-STRUCTURE defines the structural graph semantics of Resolution relationships.

It is the authoritative specification for:

- structural edge meaning
- structural ACTIVE derivation
- graph acyclicity
- Area-local structural sovereignty
- graph-level conflict detection
- first-successful-accept race semantics
- governance slot participation in the structural graph
- deterministic graph reconstruction during restore
- structural interaction with incremental compilation ordering

ENG-STRUCTURE does not redefine:

- object schemas
- runtime halt conditions
- ON_HOLD / RETIRED usability semantics
- receipt structure
- canonical serialization rules
- atomic acceptance persistence boundaries
- session lifecycle state transitions

Those are defined respectively in:

- ENG-DOMAIN
- ENG-INTEGRITY
- ENG-USABILITY
- ENG-RECEIPT
- ENG-CANON
- ENG-PERSISTENCE
- ENG-SESSION

ENG-STRUCTURE is the authority for graph truth, not for all runtime consequences of that truth.

---

# 2. Single-Area Structural Doctrine

## ENG-STRUCTURE-01 — Area-Local Graph Sovereignty

The structural graph is scoped to exactly one Area.

Rules:

- structural graph evaluation must occur only within the active Area
- structural edges must not cross Area boundaries
- ACTIVE derivation must be computed strictly from Area-local structural objects
- cross-area references may exist only as informational metadata
- intra-Area informational Resolution references may exist, but they are not structural edges in the current specification set

Mixed-area structural graphs are invalid.

Runtime halt conditions for mixed-area structural graphs are enforced by ENG-INTEGRITY.  
ENG-STRUCTURE defines that such graphs are not semantically valid.

---

# 3. Graph Structure

## ENG-STRUCTURE-02 — Directed Immutable Structural Edges

Resolutions may structurally relate to zero or more prior Resolutions within the same Area through the structural field:

- superseded_by

In the current specification set, supersession is the only structural Resolution-to-Resolution edge.

Structural edges are:

- explicit
- directional
- immutable
- created only through valid acceptance as coordinated elsewhere

ENG-STRUCTURE defines the graph meaning of those edges.  
ENG-PERSISTENCE defines the atomic durability boundary within which they are committed.

---

## ENG-STRUCTURE-03 — Acyclic Requirement

The structural graph must remain acyclic.

Cycle detection must be applied whenever graph validity is evaluated, including:

- acceptance-time graph validation
- restore / rehydration reconstruction
- historical replay or compilation graph reconstruction

If a cycle exists, the graph is structurally invalid.

ENG-INTEGRITY determines halt behavior.  
ENG-STRUCTURE determines that the graph cannot be interpreted.

---

## ENG-STRUCTURE-04 — Area Consistency of Structural Edges

Every structural edge must satisfy:

- successor.area_id == predecessor.area_id

Cross-area structural edges are prohibited.

Reference classification is defined in ENG-DOMAIN.  
Halt semantics are defined in ENG-INTEGRITY.

---

# 4. Structural ACTIVE Derivation

## ENG-STRUCTURE-05 — Structural ACTIVE Is a Graph Property

A Resolution is structurally ACTIVE if and only if:

- its lifecycle state is ACTIVE
- no accepted successor exists within the same Area structural graph

This is the authoritative definition of structural ACTIVE.

ENG-STRUCTURE alone defines structural ACTIVE derivation.  
Other specifications must reference this derivation rather than redefine it.

---

## ENG-STRUCTURE-06 — Structural ACTIVE Is Distinct from Usability

Structural ACTIVE is not the same as forward usability.

ON_HOLD and RETIRED do not alter structural ACTIVE derivation.

They may suspend usability in legitimacy evaluation, but that suspension is defined in:

- ENG-USABILITY

ENG-STRUCTURE therefore distinguishes:

- structural graph truth
- runtime usability consequences

This separation must remain explicit.

---

# 5. Governance Slot Participation

## ENG-STRUCTURE-07 — Governance Objects Participate in the Graph

Authority and Scope Resolutions participate fully in the structural graph.

Their structural graph behavior is the same as other Resolutions unless otherwise constrained by authoritative state semantics elsewhere.

ENG-STRUCTURE defines:

- they may occupy graph nodes
- they may be structural predecessors or successors where permitted
- their ACTIVE status is derived by the same structural graph rules

ENG-USABILITY defines which governance states are usable or disallowed.  
ENG-INTEGRITY enforces whether a runtime may continue when governance structure becomes unsafe.

---

## ENG-STRUCTURE-08 — Governance Slot Structural Outcomes

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

## ENG-STRUCTURE-09 — Structural References Participate in Graph Meaning

A structural Resolution reference is a graph edge.

It must:

- resolve locally within the Area
- reference a valid Resolution node
- participate in deterministic graph reconstruction

Missing structural references invalidate the graph.

ENG-DOMAIN defines the field structure.  
ENG-INTEGRITY defines halt behavior.

---

## ENG-STRUCTURE-10 — Informational References Are Never Graph Edges

Cross-area or informational references:

- must not be interpreted as structural edges
- must not affect ACTIVE derivation
- must not affect governance slot graph participation
- must not affect graph conflict detection

This includes:

- cross-area informational references
- intra-Area informational Resolution references

Graph semantics apply only to structural references.

Informational references must not be used as inputs to:

- graph traversal
- ordering
- reachability analysis
- precedence evaluation
- ACTIVE derivation
- conflict detection

---

# 7. Acceptance Race Semantics

## ENG-STRUCTURE-11 — First Successful Acceptance Wins

If multiple acceptance attempts compete to supersede the same structural target set:

- the first successfully committed acceptance wins
- ENG-STRUCTURE consumes the committed structural result
- ENG-PERSISTENCE defines the atomic mechanism by which that result becomes authoritative
- later competing attempts become graph-invalid for acceptance purposes

This is the authoritative structural conflict rule.

ENG-PERSISTENCE defines how commit atomicity preserves that outcome.  
ENG-DECISION and ENG-SESSION consume the outcome for session or candidate consequence handling.

ENG-STRUCTURE does not define lifecycle transitions such as BLOCK_PERMANENT; it defines the graph truth that causes them.

---

## ENG-STRUCTURE-12 — No Graph Merge by Heuristic

The structural graph does not support automatic branch merge.

Where incompatible accepted successors would create ambiguity, deterministic failure or first-successful acceptance semantics must prevail according to the relevant execution context.

Historical replay ordering for compilation is defined in ENG-COMPILATION.  
Live acceptance ordering is governed by atomic commit timing, not timestamps.

---

# 8. Graph-Level Conflict Detection

## ENG-STRUCTURE-13 — Authoritative Conflict Conditions

A structural conflict exists when a proposed graph mutation would:

- reference a structurally non-ACTIVE target where ACTIVE is required
- introduce a cycle
- introduce cross-area structural edges
- reference missing structural predecessors
- violate graph-level governance slot structure
- conflict with already accepted structural outcomes
- rely on structurally invalid graph state

ENG-STRUCTURE is the authority for determining that these are structural conflicts.

Other specifications may define how the runtime reacts to those conflicts, but must not redefine the graph conditions themselves.

---

## ENG-STRUCTURE-14 — Evaluation and Commit-Time Recheck

Structural conflict detection must be valid both:

- during pre-acceptance evaluation
- immediately before graph mutation is committed

Acceptance orchestration belongs to ENG-DECISION.  
Atomic mutation belongs to ENG-PERSISTENCE.

ENG-STRUCTURE defines the graph truths both stages must consume.

---

# 9. Incremental Compilation Integration

## ENG-STRUCTURE-15 — Historical Replay Must Respect Graph Determinism

Historical replay and incremental compilation must reconstruct graph outcomes deterministically.

ENG-STRUCTURE requires that graph reconstruction consume:

- accepted structural edges
- structurally valid Resolution states
- canonical historical precedence as supplied by ENG-COMPILATION

ENG-STRUCTURE does not define the historical ordering algorithm itself.  
That belongs to ENG-COMPILATION.

It does define that, once such ordering is applied, the resulting structural graph must still satisfy all structural graph rules in this document.

---

## ENG-STRUCTURE-16 — Historical Ordering Does Not Change Graph Semantics

Compilation ordering may determine which historical edge wins when replaying competing legitimate artifacts.

It does not redefine:

- what a structural edge means
- what structural ACTIVE means
- what constitutes a cycle
- what constitutes a cross-area violation

Graph semantics remain invariant across runtime and compilation contexts.

---

# 10. Resolution Lifecycle Interaction

## ENG-STRUCTURE-17 — Structural Interaction with Resolution States

ENG-STRUCTURE consumes lifecycle state only where it affects graph interpretation.

Graph-relevant principles:

- ACTIVE may participate in structural ACTIVE derivation
- SUPERSEDED records that a Resolution is no longer structurally ACTIVE because a valid accepted successor exists
- ON_HOLD does not change graph structure
- RETIRED does not change graph structure

Usability consequences of ON_HOLD and RETIRED are defined only in ENG-USABILITY.

ENG-STRUCTURE must not independently redefine administrative suspension semantics.

---

# 11. Restore & Rehydration Guarantees

## ENG-STRUCTURE-18 — Deterministic Graph Reconstruction

Given identical structural objects and identical structural edges, graph reconstruction must produce identical:

- ACTIVE Resolution sets
- predecessor/successor relationships
- governance slot structural outcomes

No heuristic ordering is permitted.

Restore-time runtime safety, halt, or degraded-mode decisions are not defined here.  
Those belong to ENG-INTEGRITY.

ENG-STRUCTURE defines only the graph reconstruction truth that those runtime decisions consume.

---

### ENG-STRUCTURE-18A — Deterministic Input Closure

Deterministic structural outcomes require identical:

- structural domain objects
- structural edges
- graph-valid historical legitimacy artifacts where required
- schema versions
- rule identity inputs where applicable

ENG-STRUCTURE outcomes must not depend on storage order, timestamps, or implementation heuristics.

---

# 12. Receipt Relationship

## ENG-STRUCTURE-19 — Graph Semantics May Depend on Valid Historical Legitimacy Artifacts but Do Not Define Receipt Rules

ENG-STRUCTURE may require that accepted graph edges correspond to structurally valid legitimacy history.

However, it does not define:

- receipt schema
- canonical serialization
- content_hash rules
- rule provenance verification

Those belong to:

- ENG-RECEIPT
- ENG-CANON
- ENG-SPECVERIFY

ENG-STRUCTURE only depends on the runtime result that accepted historical graph mutations are legitimate and structurally admissible.

---

# 13. Relationship to Runtime Blocking

## ENG-STRUCTURE-20 — Graph Truth Precedes Runtime Consequence

Graph truths established by ENG-STRUCTURE may cause runtime consequences such as:

- inability to accept
- invalid governance context
- obsolete references
- candidate ineligibility
- session blocking

Those consequences are applied by:

- ENG-DECISION
- ENG-SESSION
- ENG-INTEGRITY

ENG-STRUCTURE must not redefine session lifecycle state transitions or candidate-status reporting.

---

# 14. Engine Invariants

- structural edges are immutable
- structural graph is strictly Area-local
- graph must remain acyclic
- structural ACTIVE derivation is deterministic
- governance objects participate in graph structure
- cross-area informational references are never graph edges
- intra-Area informational Resolution references are never graph edges
- informational references must not affect ACTIVE derivation or conflict detection
- first successful accepted graph mutation wins for conflicting live supersession attempts
- ON_HOLD / RETIRED do not alter structural ACTIVE
- compilation may affect historical replay precedence but not graph semantics
- structural graph rules must be consumed, not redefined, by dependent specifications

---

# 15. Mental Model

ENG-STRUCTURE defines graph truth.

It answers:

- which Resolutions structurally supersede which others
- which Resolutions are structurally ACTIVE
- whether a proposed structural mutation is graph-valid
- whether graph history is reconstructable deterministically

It does not answer:

- whether a session may resume
- whether a Resolution is usable for legitimacy despite being structurally ACTIVE
- whether the Engine must halt or degrade
- whether informational references imply graph semantics
- how receipts are serialized
- how acceptance is persisted atomically

Those belong elsewhere.

ENG-STRUCTURE is the graph layer.  
Other specifications must build on it rather than duplicate it.