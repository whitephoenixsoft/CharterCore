# Charter Structural Graph (CSG) — Foundation Specification

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

# 6. Graph Materialization (CSG Store)

CSG may maintain a **materialized graph store** for performance and query efficiency.

---

## 6.1 Purpose

The Graph Store exists to:

- accelerate structural queries  
- provide adjacency lookup  
- support active graph projection efficiently  
- avoid recomputation of graph relationships  

---

## 6.2 Properties

The Graph Store is:

- **derived from admitted commits**  
- **fully rebuildable**  
- **non-authoritative**  
- **local to the system**  

---

## 6.3 Contents

The Graph Store may include:

- node index (resolution → node)  
- adjacency structures:
  - supersession edges  
  - reference edges  
  - boundary references  
- active/inactive node flags  
- reachability caches  
- structural lookup maps  

---

## 6.4 Rebuild Principle

> The Graph Store must be fully reconstructable from the Commit Store.

- loss of the graph store is non-fatal  
- rebuild must produce identical results  
- no data may exist in the graph store that cannot be derived  

---

## 6.5 No Semantic Interpretation

The Graph Store must not:

- infer missing edges  
- interpret relationships  
- apply identity semantics  
- compute alignment  

It is strictly structural.

---

## 6.6 Separation from Other Stores

The Graph Store is not:

- the Commit Store  
- a Runtime Persistence store  
- an Identity store (CIS)  
- an Alignment store (CAS)  

It is a **derived structural cache** only.

---

# 7. Active Graph Projection

CSG provides a derived **active graph view**.

---

## 7.1 Active Node Definition

A resolution node is **active** if:

- it is not superseded  
- it is not retired  

---

## 7.2 Superseded Nodes

A node is superseded if:

- another node declares a supersession edge pointing to it  

Superseded nodes:

- remain in the full graph  
- are excluded from the active projection  

---

## 7.3 Retired Nodes

Retirement is:

> an explicit structural state declared by a commit

Retired nodes:

- remain in the full graph  
- are excluded from the active projection  

---

## 7.4 Projection Principle

> The active graph is a projection, not a mutation.

---

# 8. Structural Incompleteness

CSG must preserve incomplete structure.

---

## 8.1 Sparse Graphs

Valid cases include:

- disconnected subgraphs  
- nodes with no references  
- isolated areas  

---

## 8.2 Declared Upstream Boundary Linkage

An Area may be structurally connected via:

- higher-order resolutions that reference its boundary  

This creates:

> declared upstream boundary linkage

---

## 8.3 No Compensation by Inference

CSG must not:

- infer missing dependencies  
- create edges from hierarchy  
- assume completeness  

---

# 9. Structural Queries

CSG must support:

- predecessors / successors  
- supersession chains  
- reachability  
- dependency neighborhoods  
- active vs historical projections  

All queries must operate on explicit structure only.

---

# 10. Relationship to Other Modules

## 10.1 CIS (Identity)

Consumes CSG structure.

---

## 10.2 CAS / CAE (Alignment)

Consumes CSG + CCare.

---

## 10.3 CGL (Guidance)

Interprets outputs.

---

## 10.4 Runtime

Controls admission.

---

# 11. Invariants

- only admitted commits used  
- nodes are resolution-only  
- edges are explicit  
- no inferred structure  
- graph is append-only  
- projections do not mutate history  
- graph store is derived and rebuildable  

---

# 12. Mental Model

CSG is:

- a deterministic structural map  
- a historical DAG  
- a boundary-aware graph  

---

# 13. Final Principle

CSG ensures:

- structure is explicit  
- history is preserved  
- incompleteness is respected  

It provides a foundation for higher systems  
without introducing interpretation or assumption.