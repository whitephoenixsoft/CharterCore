# Charter Resolution Recontextualization (CRR) — Foundation Specification (Revised vNext)

Status: FOUNDATIONAL  
Intent: Define how structural artifacts are recontextualized across scopes and abstraction tiers without rewriting history  
Scope: Promotion, demotion, decomposition, synthesis, copy, move, and derivation relationships across resolutions and pre-legitimacy artifacts  
Depends On: Charter Commit System (CCS), Charter Structural Graph (CSG), Charter Deliberate Substrate (CDS)  
Does NOT Define: legitimacy, workflow orchestration, identity (CIS), alignment (CAS), or guidance (CGL)  

---

# 1. Purpose

The Charter Resolution Recontextualization (CRR) defines how structural artifacts may be recontextualized across:

- scopes (areas, contexts)  
- abstraction tiers (breadth vs granularity of meaning)  

It exists to:

- enable late discovery of correct abstraction and scope  
- support structural evolution over time  
- preserve full lineage across investigation and legitimacy  
- allow decomposition and synthesis of decisions  
- connect investigative structure (CDS) with legitimate structure  

CRR is a **structural lineage and transformation specification model**, not a workflow system.

---

# 2. Core Principle

> Recontextualization creates new artifacts. It never mutates existing ones.

All changes in:

- scope  
- abstraction tier  
- structural interpretation  

must be expressed as:

- new artifacts (typically resolutions)  
- explicit lineage relationships  

---

# 3. Core Concepts

## 3.1 Recontextualization

Recontextualization is the act of:

> defining how a new artifact should be created in a different scope or abstraction tier based on one or more existing artifacts.

Artifacts may include:

- resolutions (legitimate structure)  
- items (investigative structure via CDS)  

---

## 3.2 Abstraction Tier

Abstraction tier describes:

> the breadth or granularity of a resolution’s meaning and decomposition scope.

- higher abstraction → broader, more general  
- lower abstraction → narrower, more specific  

### Properties

- not related to authority  
- not related to identity  
- not inferred by CSG  
- expressed only through artifact creation and lineage  

### Principle

> Abstraction tier reflects how meaning is structured, not who controls it.

---

## 3.3 Derivation Relationship

CRR relies on:

### `derives_from`

An artifact may declare lineage from one or more prior artifacts.

Properties:

- explicit  
- directional  
- non-destructive  
- cross-area allowed  
- cross-artifact-class allowed (item ↔ resolution)  

---

## 3.4 Recontextualization Plans (NEW — CRITICAL)

CRR does not create artifacts directly.

CRR produces:

> **recontextualization plans**

A plan defines:

- candidate artifacts to be created  
- their intended abstraction tier  
- their scope or area  
- their `derives_from` lineage relationships  
- their structural transformation type (promotion, decomposition, etc.)

These plans:

- are non-authoritative  
- are not commits  
- do not create legitimacy  
- must be evaluated through Review  

Principle:

> CRR defines how structure should change.  
> Review and Sessions decide whether it becomes real.

---

## 3.5 Legitimacy Boundary

CRR operates across the legitimacy boundary but does not cross it directly.

### A. Pre-Legitimacy (CDS)

- operates on Items  
- supports simulation and restructuring  
- non-authoritative  

### B. CRR Plans

- may originate from CDS or existing resolutions  
- define candidate transformations  

### C. Post-Legitimacy (via Review + Session)

- plans become Review Items  
- accepted items proceed to Sessions  
- Sessions produce resolution commits  

Principle:

> CRR preserves structure across the legitimacy boundary without executing it.

---

# 4. Recontextualization Types

CRR defines structural transformation patterns, including:

---

## 4.1 Promotion (Abstraction Increase)

- narrower → broader abstraction tier  

---

## 4.2 Demotion (Abstraction Decrease)

- broader → narrower abstraction tier  

---

## 4.3 Decomposition

- one higher-tier artifact → multiple lower-tier artifacts  

---

## 4.4 Synthesis

- multiple artifacts → one higher-tier artifact  

---

## 4.5 Copy

- artifact duplicated across contexts  

---

## 4.6 Move

- artifact recontextualized with optional source retirement  

---

# 5. Source Lifecycle Handling

Unchanged.

---

# 6. Multi-Artifact Derivation

Unchanged.

---

# 7. Relationship to CDS

CDS may:

- simulate recontextualization using Items  
- explore abstraction changes  
- generate candidate structure  

CRR:

- formalizes these into recontextualization plans  
- preserves lineage for potential legitimacy  

Principle:

> CDS explores structure. CRR formalizes it. Review decides it.

---

# 8. Relationship to Reconciliation

Unchanged but implicitly strengthened.

---

# 9. Annotation

Unchanged.

---

# 10. Relationship to Other Modules

## Runtime
- orchestrates execution of CRR plans through Review  

## Review
- evaluates CRR plans as Review Items  

## Legitimacy Engine
- produces resulting resolutions via Sessions  

(others unchanged)

---

# 11. Invariants

- recontextualization creates new artifacts  
- original artifacts are never mutated  
- derivation relationships are explicit  
- abstraction tier is not authority  
- abstraction tier is not identity  
- decomposition and synthesis must preserve lineage  
- source retirement is explicit  
- cross-artifact derivation is allowed  
- annotations do not affect structure  

**Critical Legitimacy Constraints:**

- CRR MUST NOT produce committed resolutions  
- CRR MUST NOT bypass Review  
- CRR MUST NOT bypass Sessions  
- All CRR plans MUST pass through Review + Session before becoming durable artifacts  

---

# 12. Mental Model

CRR is:

- a lineage system across abstraction and scope  
- a transformation specification layer  
- a plan generator for structural evolution  
- a bridge between investigation and legitimacy  

It is not:

- a workflow engine  
- a legitimacy creator  
- an authority system  

---

# 13. Final Principle

CRR ensures that:

- structure can evolve across abstraction tiers  
- decisions can decompose and recombine  
- investigation can inform legitimacy  
- lineage is preserved across all transformations  

All change is first expressed as a plan,  
then evaluated,  
then decided,  
and only then recorded as legitimate structure.