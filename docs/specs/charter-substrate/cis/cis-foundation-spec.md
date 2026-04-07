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
- allow identity to evolve as structure is refined, promoted, demoted, or recontextualized  
- enable higher-order systems (CAS, CGL) to reason about identity without ambiguity  

CIS is a **structural identity layer**, not a semantic or interpretive system.

---

# 2. Core Principle

> Identity is declared. Boundaries are explicit. Membership is derived, never inferred beyond definition.

CIS:

- requires identity to be explicitly declared  
- requires scope boundaries to be explicitly defined  
- derives membership strictly from declared scope rules over CSG  
- may consume structural lineage such as derivation without interpreting its semantic meaning  
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

Anchors may include:

- directly declared resolutions  
- resolutions that became identity-relevant through explicit derivation lineage  

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

Traversal may consider:

- supersession edges where structurally relevant  
- derivation lineage where explicitly present in CSG  

Traversal must not:

- exceed declared bounds  
- assume completeness  
- infer scope from undeclared hierarchy  

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

Membership may evolve as:

- new structural references are admitted  
- derivation lineage introduces new valid identity-relevant paths  
- scope definitions are updated explicitly  

---

## 6.2 Shared Membership

A resolution may belong to multiple identities.

This enables:

- overlap  
- shared decisions  
- structural convergence across identities  

---

## 6.3 Ownership Principle

> Identities do not own resolutions. They bind to them.

---

# 7. Identity Relationships

## 7.1 Overlap

Shared membership.

---

## 7.2 Collaboration

Structural dependency across identities.

This may arise through:

- direct shared structure  
- cross-identity references  
- structural derivation paths that create adjacent or interacting domains  

---

## 7.3 Boundary Adjacency

Adjacent without overlap.

---

## 7.4 Relationship Principle

> Relationships are structural. Interpretation is external.

---

# 8. Identity and Structural Lineage

CIS may consume structural lineage exposed by CSG, including derivation relationships.

---

## 8.1 Derivation Awareness

When CSG exposes explicit derivation lineage, CIS may use it as part of bounded scope evaluation.

This allows identities to remain coherent when structure evolves through:

- promotion  
- demotion  
- copy  
- move  
- recontextualization across areas  

---

## 8.2 No Semantic Interpretation

CIS must not interpret derivation as:

- promotion  
- demotion  
- correctness  
- legitimacy  

CIS uses derivation only as **declared structural lineage**.

---

## 8.3 Identity Stability Across Recontextualization

Identity may remain stable, change version, or break lineage depending on:

- declared scope updates  
- not merely the existence of derivation edges  

Structural lineage informs identity evaluation.  
It does not decide it automatically.

---

# 9. Identity Materialization (CIS Store)

CIS may maintain a **materialized identity store** for performance.

---

## 9.1 Purpose

The Identity Store exists to:

- accelerate membership queries  
- provide identity-scoped views  
- support identity relationship queries  
- avoid recomputing traversal repeatedly  

---

## 9.2 Properties

The Identity Store is:

- **derived from CCS + CSG**  
- **fully rebuildable**  
- **non-authoritative**  
- **local to the system**  

---

## 9.3 Contents

May include:

- identity → resolution membership maps  
- resolution → identities index  
- overlap sets  
- collaboration edges  
- boundary adjacency mappings  
- active identity versions  
- derivation-aware membership projections  
- identity-local lineage paths  

---

## 9.4 Rebuild Principle

> The Identity Store must be fully reconstructable.

- loss is non-fatal  
- rebuild must be deterministic  

---

## 9.5 No Interpretation Rule

The Identity Store must not:

- infer scope  
- modify identity definitions  
- interpret relationships  
- compute alignment  

---

## 9.6 Separation from Other Stores

The Identity Store is not:

- Commit Store  
- Graph Store (CSG)  
- Alignment Store (CAS)  
- Runtime Persistence  

---

# 10. Structural Independence

## 10.1 CSG

Provides structure, including explicit derivation lineage.

---

## 10.2 CAS

Consumes identity and structural context.

---

## 10.3 CGL

Interprets identity, lineage, and structural evolution.

---

# 11. Invariants

- identity is explicit  
- identity is commit-based  
- versioning is explicit  
- scope is explicit  
- membership is bounded and deterministic  
- overlap is allowed  
- no inferred identity  
- derivation may inform membership only through declared structural rules  
- store is derived and rebuildable  

---

# 12. Mental Model

CIS is:

- identity overlay on a graph  
- bounded influence mapping  
- overlapping domain system  
- structurally aware of lineage without interpreting it  

---

# 13. Final Principle

CIS ensures:

- identity is explicit  
- boundaries are declared  
- overlap is preserved  
- structural evolution can be incorporated without rewriting identity history automatically  

It enables systems to understand identity  
without ever inferring or collapsing structure.