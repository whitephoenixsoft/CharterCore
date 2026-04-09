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

CRR is a **structural lineage model**, not a workflow system.

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

> creating a new artifact in a different scope or abstraction tier based on one or more existing artifacts.

Artifacts may include:

- resolutions (legitimate structure)  
- items (investigative structure via CDS)  

---

## 3.2 Abstraction Tier (NEW)

Abstraction tier describes:

> the breadth or granularity of a resolution’s meaning and decomposition scope.

- higher abstraction → broader, more general  
- lower abstraction → narrower, more specific  

### Properties

- not related to authority  
- not related to identity  
- not inferred by CSG  
- declared or implied through artifact creation and lineage  

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

## 3.4 Legitimacy Boundary

CRR distinguishes:

### A. Pre-Legitimacy Recontextualization (CDS)

- operates on Items  
- supports simulation and restructuring  
- non-authoritative  

### B. Post-Legitimacy Recontextualization (CRR)

- produces resolution commits  
- preserves lineage from Items and/or Resolutions  
- requires Review + Session for legitimacy  

Principle:

> CRR preserves structure across the legitimacy boundary without defining it.

---

# 4. Recontextualization Types

CRR does not enforce semantics, but common structural patterns include:

---

## 4.1 Promotion (Abstraction Increase)

- narrower → broader abstraction tier  

Example:

- implementation detail → guiding principle  

---

## 4.2 Demotion (Abstraction Decrease)

- broader → narrower abstraction tier  

Example:

- principle → specific implementation  

---

## 4.3 Decomposition (NEW)

- one higher-tier resolution → multiple lower-tier resolutions  

Properties:

- preserves lineage from source to all derived artifacts  
- enables structural expansion  

---

## 4.4 Synthesis (NEW)

- multiple lower-tier artifacts → one higher-tier resolution  

Properties:

- may derive from multiple sources  
- enables consolidation of meaning  

---

## 4.5 Copy

- artifact duplicated across contexts  
- source remains active  

---

## 4.6 Move

- artifact recontextualized  
- source optionally retired  

---

# 5. Source Lifecycle Handling

## 5.1 Retain Source

- original artifact remains active  

## 5.2 Retire Source

- original artifact explicitly retired  

## 5.3 Principle

> Retirement is independent from derivation and must be explicit.

---

# 6. Multi-Artifact Derivation

A resolution may derive from:

- one or more resolutions  
- one or more Items  
- a mixture of both  

Supports:

- decomposition  
- synthesis  
- CDS-to-resolution transitions  

---

# 7. Relationship to CDS

CDS may:

- simulate recontextualization using Items  
- explore abstraction changes  
- decompose or synthesize candidate structure  

CRR:

- consumes this lineage when producing resolutions  
- does not interpret CDS semantics  

Principle:

> CDS explores structure. CRR preserves it in legitimacy.

---

# 8. Relationship to Reconciliation

Reconciliation provides:

- mapping between Items and Resolutions  
- traceability across the legitimacy boundary  

CRR depends on:

- `derives_from` relationships  
- reconciliation linkage  

Principle:

> Reconciliation connects artifacts. CRR preserves their lineage.

---

# 9. Annotation

Annotations may describe:

- rationale for abstraction change  
- reasoning for decomposition or synthesis  
- contextual decisions  

They are:

- optional  
- non-authoritative  

---

# 10. Relationship to Other Modules

## Runtime
- orchestrates workflows  

## CDS
- performs pre-legitimacy structure formation  

## Review
- evaluates and admits recontextualized artifacts  

## CCS
- records artifacts and lineage  

## CSG
- materializes structural graph  

## CIS
- binds identity independently of abstraction tier  

---

# 11. Invariants

- recontextualization creates new artifacts  
- original artifacts are never mutated  
- derivation relationships are explicit  
- derivation does not imply legitimacy  
- abstraction tier is not authority  
- abstraction tier is not identity  
- decomposition and synthesis must preserve lineage  
- source retirement is explicit  
- cross-artifact derivation is allowed  
- Items never become legitimate without Review + Session  
- annotations do not affect structure  

---

# 12. Mental Model

CRR is:

- a lineage system across abstraction and scope  
- a mechanism for decomposition and synthesis of meaning  
- a bridge between investigation and legitimacy  

It is not:

- a hierarchy engine  
- an authority model  
- an identity system  
- a workflow engine  

---

# 13. Final Principle

CRR ensures that:

- structure can evolve across abstraction tiers  
- decisions can decompose and recombine  
- investigation can inform legitimacy  
- lineage is preserved across all transformations  

Every structural change is recorded as new artifacts —  
linked explicitly to what came before —  
without rewriting history or implying authority.