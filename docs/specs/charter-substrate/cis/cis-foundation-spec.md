# Charter Identity Substrate (CIS) — Foundation Specification

Status: FOUNDATIONAL  
Intent: Define identity, scope, and bounded influence over structural graphs  
Scope: Identity declaration, scope definition, versioning, lineage, and identity relationships  
Does NOT Define: graph structure (CSG), alignment (CAS/CAE), guidance (CGL), or legitimacy  

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
- does not infer identity membership from connectivity  
- preserves overlap and incompleteness without correction  

---

# 3. Identity Model

## 3.1 Identity as a Commit

An identity is defined by an **identity commit**.

Each identity commit:

- is immutable  
- uniquely identifiable  
- represents a declared identity state  

---

## 3.2 Identity Properties

An identity has:

- identity_id  
- optional predecessor_identity_id (for lineage)  
- current scope_definition  
- current purpose reference  
- lifecycle state (active, deprecated, sunset, etc.)  

---

## 3.3 Identity Lifecycle

Identity evolves through commits:

- updates to purpose  
- updates to scope definition  
- lifecycle transitions  

All changes are:

- append-only  
- explicit  
- auditable  

---

# 4. Identity Lineage

## 4.1 Version Continuity

An identity remains in the same lineage when:

- purpose changes  
- lower-order scope refinements occur  

These produce new identity commits with the same identity_id.

---

## 4.2 Lineage Break

A new identity lineage is created when:

> the identity-defining scope changes materially

This produces:

- a new identity_id  
- an explicit predecessor reference  

---

## 4.3 Lineage Principle

> Identity continuity exists only while its defining boundary remains the same bounded thing.

---

# 5. Scope Model

## 5.1 Scope Definition

Each identity has exactly one **active scope definition**.

A scope definition is:

> a declared boundary specification over the structural graph

---

## 5.2 Scope Components

A scope definition consists of:

### 5.2.1 Anchor Resolutions

- resolution_ids where the identity attaches to the graph  
- define structural entry points  

---

### 5.2.2 Boundary Stops

- resolution_ids or rules where identity membership must stop  
- define limits of influence  

---

### 5.2.3 Optional Explicit Inclusions

- resolution_ids explicitly included regardless of connectivity  

---

### 5.2.4 Optional Explicit Exclusions

- resolution_ids explicitly excluded  

---

## 5.3 Scope Principle

> Anchors define attachment. Boundaries define limits.

A valid scope must:

- include at least one anchor  
- define sufficient boundary constraints to prevent implicit expansion  

---

## 5.4 No Connectivity Inference

CIS must not:

- assume identity includes all reachable nodes from anchors  
- infer membership from graph connectivity alone  

Membership is determined only by the declared scope definition.

---

# 6. Identity Membership

## 6.1 Membership Definition

A resolution belongs to an identity if:

- it is included by the scope definition  
- and not excluded by boundary constraints  

---

## 6.2 Shared Membership

A resolution may belong to multiple identities.

This enables:

- shared decisions  
- convergence across identities  
- structural overlap  

---

## 6.3 Ownership Principle

> Identities do not own resolutions. They bind to them.

---

# 7. Identity Relationships

CIS preserves structural relationships between identities.

---

## 7.1 Overlap

Overlap occurs when:

> two identities include the same resolution(s)

Properties:

- symmetric  
- structural  
- non-directional  

---

## 7.2 Collaboration

Collaboration occurs when:

> one identity’s structure references or depends on another’s structure

Properties:

- directional  
- structural  
- may imply dependency or influence  

---

## 7.3 Relationship Principle

> CIS preserves relationship types. It does not collapse them.

Interpretation of these relationships is external (CAS/CGL).

---

# 8. Structural Independence

## 8.1 Separation from CSG

- CSG defines graph structure  
- CIS defines identity over that structure  

CIS must not:

- modify graph structure  
- infer missing edges  

---

## 8.2 Separation from CAS

- CAS computes alignment and tension  
- CIS defines identity boundaries  

CIS must not:

- compute alignment  
- interpret tension  

---

## 8.3 Separation from CGL

- CGL interprets identity and alignment  
- CIS remains structural  

---

# 9. Invariants

The following must always hold:

- identity is explicitly declared  
- identity is commit-based and immutable  
- identity lineage is explicit  
- scope definition is explicit  
- anchors and boundaries are both required  
- identity membership is never inferred from connectivity  
- resolutions may belong to multiple identities  
- identity relationships are preserved structurally  
- CIS does not interpret meaning  

Violation of these breaks CIS correctness.

---

# 10. Mental Model

CIS is:

- a system of declared identities  
- a mapping of bounded influence over a graph  
- a model of overlapping and collaborating domains  

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

- who is responsible for what  
- where influence begins and ends  
- how identities interact  

without ever:

- guessing  
- inferring  
- or collapsing structure into assumption