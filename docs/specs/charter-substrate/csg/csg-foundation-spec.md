# Charter Structural Graph (CSG) — Foundation Specification (Revised)

Status: FOUNDATIONAL  
Intent: Define the structural graph model derived from admitted commits  
Scope: Graph construction, node/edge definitions, structural projections, and graph materialization  
Does NOT Define: identity (CIS), alignment (CAS/CAE), guidance (CGL), legitimacy, or storage of commits  

---

# 1. Purpose

The Charter Structural Graph (CSG) defines how admitted commits are transformed into a **deterministic structural graph**.

It exists to:

- materialize structural relationships between resolutions  
- preserve full historical lineage without mutation  
- support structural evolution across scopes (via derivation)  
- provide a stable foundation for higher-order systems (CIS, CAS, CGL)  
- enable structural queries without introducing interpretation  

CSG is a **pure structural layer**.

---

# 2. Core Principle

> CSG represents declared structure. It does not infer missing structure.

CSG:

- uses only admitted local commits  
- uses only explicitly declared relationships  
- never invents or infers edges  
- preserves incompleteness as truth  

---

# 3. Graph Model

## 3.1 Node Definition

CSG nodes are derived from:

- **Resolution commits only**

Each node represents:

- a single accepted resolution artifact  
- immutable, uniquely identifiable  

---

## 3.2 Non-Node Artifacts

The following are NOT graph nodes:

- Areas  
- Annotations  
- Receipts (Review, Exploration, etc.)  
- Signals (CCare)  
- Identity artifacts (CIS)  

---

## 3.3 Boundary Model (Areas)

Areas are:

- physical/runtime boundaries  
- partial graph holders  

In CSG:

> Areas are **not nodes**, but **addressable boundaries**.

---

# 4. Edge Model

CSG defines a minimal, explicit edge set.

---

## 4.1 Supersession Edge

Represents replacement over time.

- Type: node → node  
- Meaning: “this resolution supersedes another”  

Properties:

- directional  
- acyclic  
- preserves lineage  

---

## 4.2 Resolution Reference Edge

Represents dependency or relevance.

- Type: node → node  
- Meaning: “this resolution references another resolution”  

Properties:

- directional  
- non-destructive  
- arbitrary graph structures allowed  

---

## 4.3 Derivation Edge (NEW)

Represents structural lineage across scopes.

- Type: node → node  
- Meaning: “this resolution is derived from another resolution”  

Properties:

- directional  
- non-destructive  
- cross-area allowed  
- many-to-one and one-to-many allowed  
- does not imply supersession  
- does not imply dependency  

---

### 4.3.1 Derivation Principle

> Derivation captures recontextualization without replacement.

- source resolution remains valid unless separately retired  
- derivation preserves lineage across scope changes  
- no semantic interpretation (promotion/demotion) is performed in CSG  

---

## 4.4 Area Reference (Boundary Reference)

- Type: node → area  
- Meaning: awareness of another boundary  

---

## 4.5 Edge Principles

- All edges must originate from explicit commit declarations  
- No inferred edges  
- Missing relationships remain missing  

---

# 5. Graph Construction

## 5.1 Input

- admitted commits only  

---

## 5.2 Determinism

- fully deterministic  
- reproducible  

---

## 5.3 Full Graph

- append-only  
- historical  

---

# 6. Graph Materialization (CSG Store)

## 6.1 Purpose

- adjacency  
- lookup  
- projections  

---

## 6.2 Properties

- derived  
- rebuildable  
- non-authoritative  

---

## 6.3 Contents

- node index  
- adjacency:
  - supersession  
  - reference  
  - **derivation (NEW)**  
  - boundary references  
- active flags  
- reachability  

---

## 6.4 Rebuild Principle

- fully reconstructable  

---

## 6.5 No Semantic Interpretation

CSG must not:

- interpret derivation as promotion or demotion  
- infer hierarchy  
- compute identity  

---

# 7. Active Graph Projection

## 7.1 Active Node

- not superseded  
- not retired  

---

## 7.2 Supersession

- excludes from active  

---

## 7.3 Retirement

- excludes from active  

---

## 7.4 Projection Principle

- projection only  

---

# 8. Structural Incompleteness

- sparse graphs valid  
- disconnected valid  

---

# 9. Structural Queries

Must support:

- predecessors / successors  
- supersession chains  
- **derivation lineage (NEW)**  
- reachability  
- neighborhoods  

---

# 10. Relationship to Other Modules

## 10.1 CIS

Consumes structure including derivation.

---

## 10.2 CAS

May use derivation for propagation modeling.

---

## 10.3 CGL

Explains derivation history.

---

## 10.4 Runtime

Controls admission.

---

# 11. Invariants

- explicit edges only  
- derivation is non-destructive  
- no inferred hierarchy  
- append-only graph  
- store is rebuildable  

---

# 12. Mental Model

CSG is:

- structural truth  
- lineage-preserving graph  
- scope-agnostic  

---

# 13. Final Principle

CSG ensures that:

- structure evolves without mutation  
- lineage is preserved across scopes  
- meaning can shift without rewriting history  

Derivation enables structure to grow —  
without assuming what that growth means.