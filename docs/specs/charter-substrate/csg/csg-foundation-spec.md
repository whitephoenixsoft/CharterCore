# Charter Structural Graph (CSG) — Foundation Specification

Status: FOUNDATIONAL  
Intent: Define the structural graph model derived from admitted commits  
Scope: Graph construction, node/edge definitions, and structural projections  
Does NOT Define: identity (CIS), alignment (CAS/CAE), guidance (CGL), or legitimacy  

---

# 1. Purpose

The Charter Structural Graph (CSG) defines how admitted commits are transformed into a **deterministic structural graph**.

It exists to:

- materialize structural relationships between resolutions  
- preserve full historical lineage without mutation  
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

These may be referenced but are not part of the structural graph.

---

## 3.3 Boundary Model (Areas)

Areas are:

- physical/runtime boundaries  
- partial graph holders  

In CSG:

> Areas are **not nodes**, but **addressable boundaries**.

They may be referenced by nodes but are not traversable graph vertices.

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
- acyclic (must not form loops)  
- preserves historical lineage  

---

## 4.2 Resolution Reference Edge

Represents explicit dependency or relevance.

- Type: node → node  
- Meaning: “this resolution references another resolution”

Properties:

- directional  
- non-destructive  
- may form arbitrary graph structures  

---

## 4.3 Area Reference (Boundary Reference)

Represents structural awareness of another boundary.

- Type: node → area boundary  
- Meaning: “this resolution depends on or relates to another Area”

Properties:

- not a node-to-node edge  
- weaker than resolution reference  
- does not imply knowledge of internal structure  

---

## 4.4 Edge Principles

- All edges must originate from explicit commit declarations  
- No inferred or implicit edges are allowed  
- Missing relationships must remain missing  

---

# 5. Graph Construction

## 5.1 Input

CSG operates only on:

- **admitted commits within the local Commit Store**

Unreviewed or foreign commits are excluded.

---

## 5.2 Determinism

Graph construction must be:

- deterministic  
- reproducible from the same commit set  
- independent of runtime state  

---

## 5.3 Full Graph

The full graph is:

- append-only  
- immutable in history  
- inclusive of all nodes and edges ever admitted  

---

# 6. Active Graph Projection

CSG provides a derived **active graph view**.

---

## 6.1 Active Node Definition

A resolution node is **active** if:

- it is not superseded  
- it is not retired  

---

## 6.2 Superseded Nodes

A node is superseded if:

- another node declares a supersession edge pointing to it  

Superseded nodes:

- remain in the full graph  
- are excluded from the active projection  

---

## 6.3 Retired Nodes

Retirement is:

> an explicit structural state declared by a commit

Retired nodes:

- remain in the full graph  
- are excluded from the active projection  
- are not replaced by another node necessarily  

---

## 6.4 Projection Principle

> The active graph is a projection, not a mutation.

- full graph = historical truth  
- active graph = current structural view  

No history is rewritten or removed.

---

# 7. Structural Incompleteness

CSG must preserve incomplete structure.

---

## 7.1 Sparse Graphs

Valid cases include:

- nodes with no references  
- disconnected subgraphs  
- areas with no outgoing edges  

These are not errors.

---

## 7.2 Declared Upstream Boundary Linkage

An Area may be structurally connected via:

- higher-order resolutions that reference its boundary  

This creates:

> declared upstream boundary linkage

CSG includes such linkage only when explicitly declared.

---

## 7.3 No Compensation by Inference

CSG must not:

- infer missing dependencies  
- create edges based on hierarchy or expectation  
- assume completeness  

---

# 8. Structural Queries

CSG must support structural queries such as:

- predecessors / successors  
- supersession chains  
- reachable nodes  
- dependency neighborhoods  
- active vs historical views  

All queries must:

- operate on explicit structure  
- avoid interpretation  

---

# 9. Relationship to Other Modules

---

## 9.1 CIS (Identity)

- consumes CSG structure  
- defines identity, scope, and versioning  
- CSG does not represent identity  

---

## 9.2 CAS / CAE (Alignment)

- consumes CSG structure + CCare signals  
- computes alignment dynamics  
- CSG does not compute alignment  

---

## 9.3 CGL (Guidance)

- interprets outputs from CSG + CAE  
- produces explanations  
- CSG does not interpret  

---

## 9.4 Runtime

- controls admission of commits via review  
- CSG only uses admitted commits  

---

# 10. Invariants

The following must always hold:

- CSG uses only admitted commits  
- Nodes are resolution-only  
- Areas are not graph nodes  
- Edges are explicit and declared  
- No inferred relationships exist  
- Full graph is append-only  
- Active graph is a projection only  
- Supersession preserves history  
- Retirement is explicit  
- Structural incompleteness is preserved  

Violation of these breaks CSG correctness.

---

# 11. Mental Model

CSG is:

- a deterministic structural map  
- a historical DAG of resolutions  
- a boundary-aware graph  

It is not:

- an identity system  
- an alignment engine  
- a decision system  
- an interpretation layer  

---

# 12. Final Principle

CSG ensures that:

- structure is explicit  
- history is preserved  
- incompleteness is respected  

It provides the foundation upon which:

- identity can be defined  
- alignment can be computed  
- meaning can be interpreted  

without ever distorting the underlying structure.