# Charter Structural Graph (CSG) — Foundation Specification (Revised vNext)

Status: FOUNDATIONAL  
Intent: Define the generic structural DAG substrate derived from admitted artifacts  
Scope: Graph construction, node/edge definitions, structural projections, and graph materialization  
Does NOT Define: identity (CIS), alignment (CAS/CAE), guidance (CGL), legitimacy, or storage of commits  

---

# 1. Purpose

The Charter Structural Graph (CSG) defines how admitted artifacts are transformed into a **deterministic structural graph**.

It exists to:

- materialize explicit structural relationships between artifacts  
- preserve full historical lineage without mutation  
- support both legitimate and investigative structures  
- provide a stable foundation for higher-order systems (CIS, CAS, CGL)  
- enable structural queries without introducing interpretation  

CSG is a **generic structural DAG substrate**.

---

# 2. Core Principle

> CSG represents declared structure. It does not infer missing structure.

CSG:

- uses only admitted artifacts  
- uses only explicitly declared relationships  
- never invents or infers edges  
- preserves incompleteness as truth  

---

# 3. Graph Model

## 3.1 Node Definition

CSG nodes are derived from **admitted artifacts**.

Node classes include:

- **Resolution Nodes** (from resolution commits)  
- **Item Nodes** (from CDS artifact materialization or equivalent admission)

Each node:

- represents a single immutable artifact  
- has a unique identifier  
- carries a declared node class  

### 3.1.1 Item Node Admission (NEW)

Item nodes may be admitted into CSG only when:

- explicitly materialized through Runtime processes  
- in a stable state (e.g., LOCKED, APPLIED, or SETTLED)  

In-progress Items must not be admitted.

Principle:

> CSG contains stable investigative structure, not active thought.

---

## 3.2 Node Class Principle

> Node class must remain explicit and must not be inferred.

CSG must preserve:

- distinction between resolution and item nodes  
- the origin of each node  
- the non-authoritative nature of item nodes  

CSG does not assign meaning to node classes.

---

## 3.3 Non-Node Artifacts

The following are NOT graph nodes:

- Areas  
- Annotations  
- Receipts (Review, Exploration, Deliberate, etc.)  
- Signals (CCare)  
- Identity artifacts (CIS)  

---

## 3.4 Boundary Model (Areas)

Areas are:

- physical/runtime boundaries  
- partial graph holders  

In CSG:

> Areas are **not nodes**, but **addressable boundaries**.

---

# 4. Edge Model

CSG defines a minimal, explicit edge set.

Edges may connect:

- resolution → resolution  
- item → item  
- item → resolution  
- resolution → item  

All edges must be explicitly declared.

---

## 4.1 Supersession Edge

Represents replacement over time.

- Type: resolution → resolution  
- Meaning: “this resolution supersedes another”

Properties:

- directional  
- acyclic  
- preserves lineage  

Supersession applies only to resolution nodes.

Item nodes must not participate in supersession relationships.

Principle:

> Supersession represents legitimate replacement, not investigative evolution.

---

## 4.2 Reference Edge

Represents declared relevance or dependency.

- Type: node → node  
- Meaning: “this node references another node”

Properties:

- directional  
- non-destructive  
- may form arbitrary graph structures  

---

## 4.3 Derivation Edge

Represents structural lineage across scopes or contexts.

- Type: node → node  
- Meaning: “this node is derived from another node”

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

- source remains valid unless separately retired  
- derivation preserves lineage across contexts  
- no semantic interpretation is performed in CSG  

---

## 4.4 Area Reference (Boundary Reference)

- Type: node → area  
- Meaning: awareness of another boundary  

---

## 4.5 Edge Principles

- all edges must originate from explicit declarations  
- no inferred edges  
- missing relationships remain missing  

## 4.6 Cross-Class Edge Constraint (NEW)

Edges between node classes (item ↔ resolution) are allowed.

However:

- such edges must not imply legitimacy transfer  
- must not imply authority equivalence  
- must not collapse node-class distinction  

These edges exist to:

- support simulation  
- preserve investigative lineage  
- enable reconciliation workflows  

Principle:

> Cross-class edges connect structure without transferring authority.

---

# 5. Graph Construction

## 5.1 Input

CSG operates only on:

- admitted artifacts (via CCS and Runtime processes)

---

## 5.2 Determinism

Graph construction must be:

- deterministic  
- reproducible  
- independent of runtime state  

---

## 5.3 Full Graph

The full graph is:

- append-only  
- immutable in history  
- inclusive of all nodes and edges ever admitted  

---

# 6. Graph Materialization (CSG Store)

## 6.1 Purpose

The Graph Store exists to:

- accelerate structural queries  
- provide adjacency lookup  
- support projection efficiently  

---

## 6.2 Properties

The Graph Store is:

- derived  
- fully rebuildable  
- non-authoritative  

---

## 6.3 Contents

- node index (with node class)  
- adjacency:
  - supersession  
  - reference  
  - derivation  
  - boundary references  
- active/inactive flags  
- reachability  

---

## 6.4 Rebuild Principle

The Graph Store must be:

- fully reconstructable  
- deterministic  

---

## 6.5 No Semantic Interpretation

CSG must not:

- interpret node class meaning  
- interpret derivation semantics  
- infer hierarchy  
- compute identity  
- compute alignment  

---

# 7. Structural Projections (NEW)

CSG provides **explicit structural projections**.

---

## 7.1 Projection Definition

A projection is a:

> filtered view of the graph based on node and edge constraints

Projections do not mutate the graph.

---

## 7.2 Canonical Projections

### A. Resolution Projection

- includes resolution nodes only  
- excludes item nodes  

Used for:

- legitimacy-aware structure  
- downstream systems requiring authoritative context  

---

### B. Item Projection

- includes item nodes only  

Used for:

- investigation  
- simulation  
- exploratory analysis  

---

### C. Mixed Projection

- includes both node classes  

Used for:

- simulation with real structure  
- reconciliation-aware analysis  

### D. Lineage Projection (NEW)

- includes nodes connected through derivation relationships  
- may span both resolution and item nodes  
- emphasizes structural lineage over node class  

Used for:

- reconciliation workflows  
- simulation continuity  
- investigative tracing  

Principle:

> Lineage projections reveal how structure evolves across contexts.

---

## 7.3 Projection Principle

> Projections expose structure without redefining it.

---

# 8. Active Graph Projection

## 8.1 Active Node Definition

A node is active if:

- not superseded (if applicable)  
- not retired (if applicable)  

---

## 8.2 Projection Principle

> Activity is a projection, not a mutation.

---

# 9. Structural Incompleteness

CSG must preserve incomplete structure.

Valid cases include:

- disconnected subgraphs  
- mixed node-class graphs  
- sparse relationships  

---

# 10. Structural Queries

CSG must support:

- predecessors / successors  
- supersession chains  
- derivation lineage  
- reachability  
- dependency neighborhoods  
- projection-aware queries  

---

# 11. Relationship to Other Modules

## 11.1 CIS

Consumes projections of CSG.

---

## 11.2 CAS

Operates over CSG projections.

---

## 11.3 CGL

Interprets structure across projections.

---

## 11.4 Runtime

Controls admission.

---

# 12. Invariants

- nodes are immutable  
- node class is explicit  
- edges are explicit  
- no inferred structure  
- graph is append-only  
- projections do not mutate history  
- graph store is derived and rebuildable  

---

# 13. Mental Model

CSG is:

- a generic structural DAG  
- a shared substrate for legitimate and investigative structure  
- a lineage-preserving system  

---

# 14. Final Principle

CSG ensures that:

- structure is explicit  
- lineage is preserved  
- multiple forms of structure may coexist  
- meaning is never imposed  

It provides a single structural foundation  
for both decision and investigation  
without collapsing the distinction between them.