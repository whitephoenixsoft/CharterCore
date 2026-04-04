# Charter Identity Substrate (CIS) — Foundation Specification (Revised)

Status: FOUNDATIONAL  
Intent: Define identity, scope, and bounded influence over structural graphs  
Scope: Identity declaration, scope definition, versioning, lineage, and identity relationships  
Depends On: Charter Commit System (CCS), Charter Graph Substrate (CSG)  
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

> Identity is declared. Boundaries are explicit. Nothing is inferred.

CIS:

- requires identity to be explicitly declared  
- requires scope boundaries to be explicitly defined  
- does not infer identity membership from unrestricted connectivity  
- preserves overlap and incompleteness without correction  

---

# 3. Identity Model

## 3.1 Identity as a Commit

An identity is defined by a **CCS commit type: Identity Commit**.

Identity commits are:

- immutable  
- append-only  
- non-legitimizing  
- declarative (not authoritative)  

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

- anchors or boundaries adjusted  
- identity remains the same bounded concept  

Result:
- new version  
- same identity_id  

---

### C. Scope Redefinition (Lineage Break)

- identity-defining boundary changes materially  
- scope represents a different “thing”  

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

The active scope is the scope of the latest identity version.

---

## 5.2 Scope Components

A scope definition consists of:

### 5.2.1 Anchor Resolutions

- resolution_ids where identity attaches to the graph  
- define structural entry points  

---

### 5.2.2 Boundary Stops

Boundary stops define where traversal must stop.

Two types:

- **Hard Stop**  
  - node is excluded  
  - traversal must not continue beyond  

- **Soft Stop**  
  - node may be included  
  - traversal must not continue beyond  

---

### 5.2.3 Explicit Inclusions (Optional)

- resolution_ids explicitly included regardless of traversal  

---

### 5.2.4 Explicit Exclusions (Optional)

- resolution_ids explicitly excluded  

---

## 5.3 Scope Evaluation

Membership is determined by a **bounded traversal**:

- starting from anchors  
- constrained by boundary stops  
- modified by inclusion/exclusion rules  

Traversal must not:

- expand beyond declared boundaries  
- assume full graph connectivity  

---

## 5.4 Scope Principle

> Anchors define attachment. Boundaries define limits.

---

## 5.5 Pre-Structural Identity

An identity may be declared without anchors.

In this state:

- it has no structural membership  
- it becomes active once anchors are defined  

---

# 6. Identity Membership

## 6.1 Membership Definition

A resolution belongs to an identity if:

- it is reachable through bounded traversal from anchors  
- it is not excluded by boundary rules  
- or it is explicitly included  

---

## 6.2 Shared Membership

A resolution may belong to multiple identities.

This enables:

- overlap  
- shared decisions  
- convergence  

---

## 6.3 Ownership Principle

> Identities do not own resolutions. They bind to them.

---

# 7. Identity Relationships

CIS preserves structural relationships between identities.

---

## 7.1 Overlap

Occurs when:

> two identities include the same resolution(s)

Properties:

- symmetric  
- structural  
- non-directional  

---

## 7.2 Collaboration

Occurs when:

> one identity’s structure references or depends on another’s structure

Properties:

- directional  
- structural  

---

## 7.3 Boundary Adjacency

Occurs when:

> identities meet at declared boundaries without overlap

Properties:

- structural  
- non-overlapping  
- indicates potential interaction surface  

---

## 7.4 Relationship Principle

> CIS preserves relationships. It does not interpret them.

---

# 8. Structural Independence

## 8.1 Separation from CSG

- CSG defines graph structure  
- CIS defines identity over that structure  

CIS must not:

- modify graph  
- infer edges  

---

## 8.2 Separation from CAS

- CAS computes alignment  
- CIS defines boundaries  

CIS must not:

- compute alignment  
- interpret tension  

---

## 8.3 Separation from CGL

- CGL interprets identity  
- CIS remains structural  

---

# 9. Invariants

The following must always hold:

- identity is explicitly declared  
- identity is commit-based and immutable  
- identity versioning is explicit  
- scope definition is explicit  
- anchors and boundaries govern scope  
- membership is bounded and deterministic  
- identity membership is not inferred from unrestricted connectivity  
- resolutions may belong to multiple identities  
- identity relationships are preserved structurally  
- CIS does not interpret meaning  

Violation of these breaks CIS correctness.

---

# 10. Mental Model

CIS is:

- a system of declared identities  
- a mapping of bounded influence over a graph  
- a model of overlapping and interacting domains  

It is not:

- a hierarchy  
- a control system  
- an alignment engine  
- an interpretation layer  

---

# 11. Final Principle

CIS ensures that:

- identity is explicit  
- boundaries are declared  
- overlap is preserved  
- collaboration is visible  

It enables systems to understand:

- where identity begins  
- where it ends  
- how identities relate  

without ever:

- guessing  
- inferring  
- or collapsing structure into assumption