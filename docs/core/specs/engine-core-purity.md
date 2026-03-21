# ENG-CORE-PURITY — Engine Isolation, Locality, Determinism & Resource Boundary Principles

Status: REFACTORED (v6 – Constitutional Reference-Driven Model)  
Applies to: Engine Core (V1/V2+)  
Scope: Constitutional Engine guarantees

Authority: Constitutional specification governing Engine isolation, locality, determinism, and non-dependence boundaries.

Subordinate references consumed from:

- ENG-DOMAIN
- ENG-CANON
- ENG-API
- ENG-INTEGRITY
- ENG-PERSISTENCE
- ENG-SUPERSESSION
- ENG-SPECVERIFY

---

# 1. Purpose

ENG-CORE-PURITY defines the constitutional boundaries of the Engine Core.

It is the authoritative specification for:

- what the Engine may depend on
- what the Engine must never depend on
- the locality boundary of legitimacy computation
- evaluation purity
- determinism boundary conditions
- resource-envelope guarantees
- explicit non-goals of the Engine

ENG-CORE-PURITY does not define:

- object schemas
- session lifecycle behavior
- acceptance rules
- supersession graph mechanics
- receipt structure
- runtime halt policy
- persistence transaction shape

Those are defined respectively in:

- ENG-DOMAIN
- ENG-SESSION
- ENG-DECISION
- ENG-SUPERSESSION
- ENG-RECEIPT
- ENG-INTEGRITY
- ENG-PERSISTENCE

ENG-CORE-PURITY defines the constitutional limits within which those other specifications must operate.

---

# 2. Isolation Model

## ENG-CORE-01 — Storage Agnostic Engine

The Engine is storage agnostic.

The Engine:

- does not read from storage directly
- does not write to storage directly
- does not validate external persistence envelopes
- does not manage Area lifecycle outside its in-memory/runtime boundary

Persistence is the responsibility of the caller and storage adapter layer.

The Engine may compute canonical hashes where required by governing artifact specifications, but this does not make it a storage engine.

Fail if:

- Engine behavior depends on physical storage layout
- Engine traverses persistence structures directly
- Engine derives legitimacy from storage presence alone

---

## ENG-CORE-02 — Logical Input Boundary

The Engine operates only on:

- provided domain objects
- explicit structural references between those objects
- declared lifecycle state
- declared governance state
- defined rule system identity

The Engine must not:

- infer missing objects
- traverse undeclared relationships
- derive legitimacy from mere object existence
- resolve external or cross-area identifiers as structural dependencies

Fail if:

- unreferenced objects alter legitimacy
- implicit graph traversal alters outcomes
- missing facts are inferred rather than supplied

---

# 3. Structural Locality Doctrine

## ENG-CORE-03 — Area Sovereignty

Each Area is structurally sovereign.

Legitimacy within an Area must be computable only from:

- that Area’s structural domain objects
- that Area’s supersession graph
- that Area’s governance state
- the rule system bound to the evaluating Engine or preserved artifact provenance

The Engine must not:

- depend on external Areas for legitimacy computation
- traverse cross-area references as structural dependencies
- require external Areas to exist for local legitimacy evaluation

Fail if removal or absence of an external Area changes local legitimacy outcomes.

---

# 4. Identity Doctrine

## ENG-CORE-04 — Engine-Owned Identity

Engine-owned structural identifiers must:

- be Engine-generated
- be UUIDv7
- remain immutable once assigned

External systems must not override Engine identity generation for Engine-owned objects.

UUID timestamp components must not influence:

- legitimacy
- supersession precedence
- restore precedence
- evaluation ordering

Identifier form is defined in ENG-DOMAIN.  
ENG-CORE-PURITY defines the constitutional non-semantic treatment of that identity.

---

# 5. Immutability Principle

## ENG-CORE-05 — No In-Place Historical Rewrite

Historical structural artifacts must never be rewritten in place.

The Engine must preserve append-only institutional history.

Objects may evolve only through the explicit mechanisms defined in their governing specifications, such as:

- session acceptance
- supersession
- administrative usability transitions where allowed

The Engine must not rewrite prior legitimacy history as a shortcut for correction.

---

# 6. Structural Reference Boundary

## ENG-CORE-06 — Structural Edge Doctrine

Structural references may exist between domain objects.

Examples may include:

- Session → Authority Resolution
- Session → Scope Resolution
- Resolution → originating Session
- Receipt → Session
- Receipt → Resolution
- Resolution → superseded Resolution

However:

only the supersession graph determines structural ACTIVE derivation.

Other structural references may support:

- integrity validation
- provenance linkage
- audit reconstruction
- artifact correspondence

They must not independently redefine ACTIVE derivation.

ACTIVE graph meaning belongs to ENG-SUPERSESSION.

---

# 7. Cross-Area Opacity

## ENG-CORE-07 — Cross-Area References Are Informational Unless Explicitly Declared Otherwise by Schema

Cross-area references may exist as informational metadata.

The Engine must not:

- dereference them as structural dependencies
- validate their external existence for legitimacy purposes
- treat them as supersession edges
- allow them to influence legitimacy outcomes

External availability must not affect local evaluation outcomes.

Structural reference classification belongs to ENG-DOMAIN.  
ENG-CORE-PURITY establishes that external opacity is the constitutional default.

---

# 8. Determinism Doctrine

## ENG-CORE-08 — Pure Deterministic Evaluation and Mutation

Given identical:

- structural domain objects
- structural references
- session runtime state
- governance state
- rule identity context

the Engine must produce identical outcomes.

Behavior may depend only on:

- governing specifications
- canonical serialization rules where applicable
- explicit structural state
- explicit command input

Behavior must not depend on:

- storage order
- import source
- runtime environment
- system clock except for identifier generation at creation time
- external system availability
- audit ordering
- timestamp precedence unless a governing specification explicitly and constitutionally allows it

---

# 9. Evaluation Purity

## ENG-CORE-09 — Evaluation Is Side-Effect Free

Evaluation operations must be strictly non-mutating.

Evaluation must not:

- alter session state
- emit receipts
- emit audit events
- trigger lifecycle transitions
- create legitimacy artifacts

Evaluation inspects current truth.  
It does not commit new truth.

---

## ENG-CORE-10 — Evaluation Idempotence

Evaluation must be idempotent.

Given identical inputs and identical runtime state, repeated evaluation must produce identical outputs.

Acceptance and mutation commands must independently revalidate applicable invariants.

Evaluation must never act as cached or pre-authoritative legitimacy.

---

# 10. Resource Envelope Doctrine

## ENG-CORE-11 — Determinism Within a Sufficient Resource Envelope

The Engine guarantees logical determinism only within a sufficient resource envelope.

Logical determinism means identical inputs produce identical:

- reports
- state transitions
- receipts
- supersession effects
- exported structural artifacts

The Engine does not guarantee liveness under arbitrary resource exhaustion.

---

## ENG-CORE-12 — Atomic Failure Under Resource Exhaustion

If resource exhaustion occurs, the operation must fail atomically.

No partial structural mutation is permitted.

No partial legitimacy artifact emission is permitted.

Operations that must obey this principle include, where applicable:

- rehydration
- acceptance
- receipt emission
- structural commit
- canonical hash computation when part of a required artifact operation

Atomic persistence details belong to ENG-PERSISTENCE.  
Runtime consequences belong to ENG-INTEGRITY.

ENG-CORE-PURITY establishes the constitutional rule that exhaustion aborts rather than corrupts.

---

## ENG-CORE-13 — Implementation-Defined Limits

The Engine specifications do not define:

- maximum graph size
- maximum session count
- maximum receipt size
- memory ceilings
- CPU ceilings
- storage adapter performance characteristics

These are implementation-defined.

Implementations may choose limits, but those limits must not silently violate deterministic or atomic guarantees.

---

# 11. Legitimacy Boundary

## ENG-CORE-14 — Legitimacy Is Structural, Explicit, and Local

Legitimacy is created only through:

- sessions
- acceptance rules
- Area-local governance rules
- Area-local supersession rules

Legitimacy is not created by:

- evaluation
- audit events
- storage operations
- import submission alone
- timestamps
- actor identity
- cross-area references
- resource conditions
- host narration or metadata

The Engine must preserve this boundary strictly.

---

# 12. Non-Goals

## ENG-CORE-15 — Explicit Engine Non-Goals

The Engine is not:

- a storage engine
- a workflow engine
- a version control system
- a hash registry
- a permission system
- a distributed consensus protocol
- a cross-area validator
- a real-time guarantees system
- an intent inference system

The Engine is a deterministic, Area-local legitimacy compiler.

---

# 13. Constitutional Relationship to Other Specifications

## ENG-CORE-16 — Constitutional Constraint on Dependent Specifications

All other Engine specifications must conform to the principles defined here, including but not limited to:

- ENG-DOMAIN
- ENG-SESSION
- ENG-DECISION
- ENG-SUPERSESSION
- ENG-RECEIPT
- ENG-API
- ENG-INTEGRITY
- ENG-CANON
- ENG-SPECVERIFY
- ENG-PERSISTENCE

If a dependent specification introduces a behavior that violates:

- isolation
- locality
- determinism
- evaluation purity
- non-inference
- atomic failure under exhaustion

then that behavior is constitutionally invalid.

---

# 14. Engine Invariants

- Engine behavior depends only on supplied structural facts and defined rules
- Engine never derives legitimacy from storage presence
- Engine never depends on foreign Areas for local legitimacy
- Engine-owned identity is immutable and non-semantic
- Historical legitimacy is never rewritten in place
- Only supersession edges determine structural ACTIVE derivation
- Cross-area references are informational unless schema explicitly defines otherwise
- Evaluation is pure and idempotent
- Determinism is mandatory within a sufficient resource envelope
- Resource exhaustion must abort rather than partially mutate
- Legitimacy is explicit, structural, and local

---

# 15. Mental Model

Storage remembers.  
Audit records.  
API exposes.  
Initialization admits.  
Evaluation inspects.  
Acceptance commits.  
Supersession evolves.  
Integrity halts or degrades.  
Areas are sovereign.

The Engine depends only on explicit structural truth.

If truth is incomplete, it does not guess.  
If resources fail, it aborts.  
If history exists, it preserves it.  
If legitimacy is created, it is created explicitly and locally.