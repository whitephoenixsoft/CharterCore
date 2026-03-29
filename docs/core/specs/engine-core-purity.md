# ENG-CORE-PURITY — Engine Isolation, Locality, Determinism & Resource Boundary Principles

Status: REFACTORED (v7 – Structural/Usability Alignment & Terminology Finalization)  
Applies to: Engine Core (V1/V2+)  
Scope: Constitutional Engine guarantees  

Authority: Constitutional specification governing Engine isolation, locality, determinism, and non-dependence boundaries.

Subordinate references consumed from:

- ENG-DOMAIN
- ENG-CANON
- ENG-API
- ENG-INTEGRITY
- ENG-PERSISTENCE
- ENG-STRUCTURE
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
- structural graph mechanics  
- receipt structure  
- runtime halt policy  
- persistence transaction shape  
- usability semantics  

Those are defined respectively in:

- ENG-DOMAIN  
- ENG-SESSION  
- ENG-DECISION  
- ENG-STRUCTURE  
- ENG-RECEIPT  
- ENG-INTEGRITY  
- ENG-PERSISTENCE  
- ENG-USABILITY  

ENG-CORE-PURITY defines the constitutional limits within which all other specifications must operate.

---

# 2. Isolation Model

## ENG-CORE-01 — Storage-Agnostic Engine

The Engine is storage agnostic.

The Engine:

- does not read from storage directly  
- does not write to storage directly  
- does not validate external persistence envelopes  
- does not manage Area lifecycle outside its runtime boundary  

Persistence is the responsibility of the caller and storage adapter layer.

The Engine may compute canonical hashes where required by governing specifications, but this does not make it a storage engine.

Fail if:

- Engine behavior depends on physical storage layout  
- Engine traverses persistence structures directly  
- Engine derives legitimacy from storage presence  

---

## ENG-CORE-02 — Logical Input Boundary

The Engine operates only on:

- provided domain objects  
- explicit structural references  
- declared lifecycle state  
- declared governance state  
- declared rule identity  

The Engine must not:

- infer missing objects  
- traverse undeclared relationships  
- derive legitimacy from mere object existence  
- resolve external identifiers as structural dependencies  

Fail if:

- unreferenced objects alter outcomes  
- implicit traversal alters legitimacy  
- missing facts are inferred  

---

# 3. Structural Locality Doctrine

## ENG-CORE-03 — Area Sovereignty

Each Area is structurally sovereign.

Legitimacy within an Area must be computable only from:

- that Area’s domain objects  
- that Area’s structural graph (ENG-STRUCTURE)  
- that Area’s governance state  
- the rule identity of the evaluating Engine or artifact provenance  

The Engine must not:

- depend on external Areas  
- traverse cross-area references as structural inputs  
- require external Areas for local legitimacy  

Fail if removal of external data changes local outcomes.

---

# 4. Identity Doctrine

## ENG-CORE-04 — Engine-Owned Identity

Engine-owned identifiers must:

- be Engine-generated  
- be UUIDv7  
- remain immutable  

Identifier timestamp components must not influence:

- legitimacy  
- structural precedence  
- evaluation ordering  

Identifier semantics belong to ENG-DOMAIN.  
ENG-CORE-PURITY enforces their non-semantic nature.

---

# 5. Immutability Principle

## ENG-CORE-05 — No In-Place Historical Rewrite

Historical artifacts must never be rewritten in place.

History evolves only through:

- session acceptance  
- structural graph updates (via candidate actions)  
- usability transitions where explicitly allowed  

The Engine must not rewrite prior legitimacy history.

---

# 6. Structural Reference Boundary

## ENG-CORE-06 — Structural Graph Authority

Structural references may exist between objects, including:

- Session → Authority Resolution  
- Session → Scope Resolution  
- Resolution → Session  
- Receipt → Session  
- Receipt → Resolution  

However:

only the structural graph defined by ENG-STRUCTURE determines:

- supersession relationships  
- ACTIVE derivation  
- structural precedence  

All other references:

- support validation  
- support provenance  
- support traceability  

They must not influence structural truth.

---

# 7. Informational vs Structural References

## ENG-CORE-07 — Informational References Are Non-Semantic

References classified as informational:

- must not affect legitimacy  
- must not affect structural graph  
- must not affect evaluation outcomes  

Cross-area informational references:

- must not be dereferenced  
- must not be validated as dependencies  
- must not influence outcomes  

The Engine must treat them as opaque metadata.

Reference classification belongs to ENG-DOMAIN.  
Structural meaning belongs to ENG-STRUCTURE.

---

# 8. Determinism Doctrine

## ENG-CORE-08 — Pure Deterministic Behavior

Given identical:

- domain objects  
- structural graph  
- session state  
- governance state  
- rule identity  

the Engine must produce identical results.

The Engine must not depend on:

- storage order  
- import order  
- runtime environment  
- system clock (except ID generation at creation)  
- external availability  
- audit ordering  

Determinism is absolute within the resource envelope.

---

# 9. Evaluation Purity

## ENG-CORE-09 — Evaluation Is Side-Effect Free

Evaluation must not:

- mutate state  
- emit receipts  
- emit audit  
- create artifacts  
- change lifecycle  

Evaluation inspects truth only.

---

## ENG-CORE-10 — Evaluation Idempotence

Evaluation must be idempotent.

Repeated evaluation with identical inputs must yield identical outputs.

Evaluation must not act as cached authority.

---

# 10. Resource Envelope Doctrine

## ENG-CORE-11 — Determinism Within Resource Bounds

Determinism is guaranteed only within sufficient resources.

The Engine does not guarantee:

- liveness under exhaustion  

---

## ENG-CORE-12 — Atomic Failure Under Exhaustion

If resources are insufficient:

- operation must abort atomically  
- no partial mutation allowed  
- no partial artifact emission allowed  

Applies to:

- initialization  
- evaluation where required  
- acceptance  
- receipt emission  
- structural commit  

---

## ENG-CORE-13 — Implementation-Defined Limits

The specification does not define:

- size limits  
- memory limits  
- performance guarantees  

Implementations must not violate determinism or atomicity.

---

# 11. Legitimacy Boundary

## ENG-CORE-14 — Legitimacy Is Explicit and Local

Legitimacy is created only through:

- sessions  
- accepted candidate outcomes  
- structural rules (ENG-STRUCTURE)  
- governance rules  

Legitimacy is not created by:

- evaluation  
- storage  
- import  
- timestamps  
- metadata  
- cross-area references  

---

# 12. Non-Goals

## ENG-CORE-15 — Explicit Non-Goals

The Engine is not:

- a storage engine  
- a workflow system  
- a version control system  
- a permission system  
- a consensus protocol  
- a cross-area validator  
- an inference system  

The Engine is a deterministic legitimacy compiler.

---

# 13. Constitutional Constraint

## ENG-CORE-16 — All Specs Must Conform

All specifications must comply with:

- isolation  
- locality  
- determinism  
- evaluation purity  
- non-inference  
- atomic failure  

Violations are constitutionally invalid.

---

# 14. Engine Invariants

- Engine depends only on explicit structural inputs  
- Engine never infers missing data  
- Engine never depends on foreign Areas  
- identity is immutable and non-semantic  
- history is append-only  
- structure defines truth  
- informational references are non-semantic  
- evaluation is pure and idempotent  
- determinism is mandatory  
- resource failure aborts safely  
- legitimacy is explicit and local  

---

# 15. Mental Model

The Engine is a closed system.

It:

- accepts explicit structure  
- evaluates deterministically  
- commits atomically  
- preserves history  

It does not:

- guess  
- infer  
- reach outside its Area  
- repair invalid structure  

If truth is incomplete, it stops.  
If resources fail, it aborts.  
If legitimacy is created, it is explicit and permanent.