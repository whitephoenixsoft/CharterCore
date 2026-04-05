# Charter Identity Substrate (CIS) — Foundation Specification (Revised)

Status: FOUNDATIONAL  
Intent: Define identity, scope, and bounded influence over structural graphs  
Scope: Identity declaration, scope definition, versioning, lineage, identity relationships, and identity materialization  
Depends On: Charter Commit System (CCS), Charter Structural Graph (CSG)  
Does NOT Define: graph structure (CSG), alignment (CAS), guidance (CGL), or legitimacy  

---

# 1. Purpose

The Charter Identity Substrate (CIS) defines how **identities are declared, bounded, and evolved** over a shared structural graph.

It exists to:

- represent identity as an explicit, versioned construct  
- define bounded areas of influence over graph structure  
- support overlapping and collaborating identities  
- preserve identity continuity and lineage over time  
- enable higher-order systems (CAS, CGL) to reason about identity without ambiguity  

CIS is a **structural identity layer**, not a semantic or interpretive system.

---

# 2. Core Principle

> Identity is declared. Boundaries are explicit. Membership is derived, never inferred beyond definition.

CIS:

- requires identity to be explicitly declared  
- requires scope boundaries to be explicitly defined  
- derives membership strictly from declared scope rules  
- preserves overlap and incompleteness without correction  

---

# 3. Identity Model

## 3.1 Identity as a Commit

An identity is defined by a **CCS commit type: Identity Commit**.

Identity commits are:

- immutable  
- append-only  
- non-legitimizing  
- declarative  

Each identity commit represents a **versioned state of an identity**.

---

## 3.2 Identity Properties

Each identity version (commit) contains:

- identity_id  
- version_id (commit_id)  
- optional predecessor_identity_id  
- scope_definition  
- purpose_reference  
- lifecycle_state  

---

## 3.3 Identity Lifecycle

Identity evolves through commits:

- purpose updates  
- scope updates  
- lifecycle transitions  

All changes are:

- explicit  
- append-only  
- auditable  

---

# 4. Identity Versioning Model

## 4.1 Version as Identity State

Each identity commit represents a **complete snapshot** of:

- scope  
- purpose  
- lifecycle  

The **active identity state** is the latest version.

---

## 4.2 Types of Change

### A. Purpose Change
- scope unchanged  
- identity unchanged  

Result:
- new version  
- same identity_id  

---

### B. Scope Refinement
- boundaries adjusted  
- identity remains the same bounded concept  

Result:
- new version  
- same identity_id  

---

### C. Scope Redefinition (Lineage Break)
- identity-defining boundary changes materially  

Result:
- new identity_id  
- predecessor reference recorded  

---

## 4.3 Versioning Principle

> Identity persists only while its defining boundary remains the same bounded thing.

---

# 5. Scope Model

## 5.1 Scope Definition

Each identity version defines exactly one **scope definition**.

---

## 5.2 Scope Components

### 5.2.1 Anchor Resolutions

- structural attachment points in CSG  

---

### 5.2.2 Boundary Stops

- define traversal limits  

Types:

- Hard Stop → exclude + stop traversal  
- Soft Stop → include + stop traversal  

---

### 5.2.3 Explicit Inclusions (Optional)

- force inclusion  

---

### 5.2.4 Explicit Exclusions (Optional)

- force exclusion  

---

## 5.3 Scope Evaluation

Membership is determined by **bounded traversal over CSG**:

- start from anchors  
- follow structural edges  
- respect boundary stops  
- apply inclusion/exclusion rules  

Traversal must not:

- exceed declared bounds  
- assume completeness  

---

## 5.4 Scope Principle

> Anchors define attachment. Boundaries define limits.

---

## 5.5 Pre-Structural Identity

An identity may exist without anchors.

In this state:

- it has no structural membership  
- it becomes structurally active once anchors exist  

---

# 6. Identity Membership

## 6.1 Membership Definition

A resolution belongs to an identity if:

- it is reachable via bounded traversal  
- not excluded  
- or explicitly included  

---

## 6.2 Shared Membership

A resolution may belong to multiple identities.

---

## 6.3 Ownership Principle

> Identities do not own resolutions. They bind to them.

---

# 7. Identity Relationships

---

## 7.1 Overlap

Shared membership.

---

## 7.2 Collaboration

Structural dependency across identities.

---

## 7.3 Boundary Adjacency

Adjacent without overlap.

---

## 7.4 Relationship Principle

> Relationships are structural. Interpretation is external.

---

# 8. Identity Materialization (CIS Store)

CIS may maintain a **materialized identity store** for performance.

---

## 8.1 Purpose

The Identity Store exists to:

- accelerate membership queries  
- provide identity-scoped views  
- support identity relationship queries  
- avoid recomputing traversal repeatedly  

---

## 8.2 Properties

The Identity Store is:

- **derived from CCS + CSG**  
- **fully rebuildable**  
- **non-authoritative**  
- **local to the system**  

---

## 8.3 Contents

May include:

- identity → resolution membership maps  
- resolution → identities index  
- overlap sets  
- collaboration edges  
- boundary adjacency mappings  
- active identity versions  

---

## 8.4 Rebuild Principle

> The Identity Store must be fully reconstructable.

- loss is non-fatal  
- rebuild must be deterministic  

---

## 8.5 No Interpretation Rule

The Identity Store must not:

- infer scope  
- modify identity definitions  
- interpret relationships  
- compute alignment  

---

## 8.6 Separation from Other Stores

The Identity Store is not:

- Commit Store  
- Graph Store (CSG)  
- Alignment Store (CAS)  
- Runtime Persistence  

---

# 9. Structural Independence

## 9.1 CSG

Provides structure.

---

## 9.2 CAS

Consumes identity.

---

## 9.3 CGL

Interprets identity.

---

# 10. Invariants

- identity is explicit  
- identity is commit-based  
- versioning is explicit  
- scope is explicit  
- membership is bounded and deterministic  
- overlap is allowed  
- no inferred identity  
- store is derived and rebuildable  

---

# 11. Mental Model

CIS is:

- identity overlay on a graph  
- bounded influence mapping  
- overlapping domain system  

---

# 12. Final Principle

CIS ensures:

- identity is explicit  
- boundaries are declared  
- overlap is preserved  

It enables systems to understand identity  
without ever inferring or collapsing structure.