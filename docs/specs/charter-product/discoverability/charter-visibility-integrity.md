# Charter — Structural Visibility & Relational Integrity Specification

Status: FOUNDATIONAL  
Applies to: Runtime Layer, Charter Alignment System (CAS), Charter Care Substrate (CCare), Charter Lineage Substrate (CLL)  
Depends On: Cross-Area Discoverability Specification  
Does NOT define: alignment computation, detectability algorithms, or guidance interpretation  

---

# 1. Purpose

This document defines the **structural requirements for representing relationships across Areas and Resolutions** within Charter.

It establishes:

- what must be visible to form relationships  
- what makes relationships structurally valid  
- how cross-area relationships are represented  
- what constitutes structural completeness  
- what failures occur when relationships are missing or misrepresented  

This document defines **correctness of structure**, not how structure is discovered or analyzed.

---

# 2. Core Principles

## 2.1 Visibility Precedes Relationship

> A relationship cannot exist structurally unless its target is visible.

All references must resolve to:

- a locally available artifact  
- or an artifact present in a valid review context  

---

## 2.2 Relationships Must Be Explicit at the Artifact Level

> All structural relationships must originate from explicit artifact-level references.

Relationships are expressed through:

- Resolution-level references  

No relationship may be:

- inferred  
- assumed  
- derived implicitly  

---

## 2.3 Structure Reflects Declared Reality

Charter does not infer real-world relationships.

> The system reflects only what is explicitly represented in artifacts.

---

## 2.4 Missing Structure Is a Deficiency

> Failure to represent real relationships is a structural error.

This includes:

- omitted dependencies  
- hidden coordination  
- undeclared coupling  

This is not neutral incompleteness.  
It is **inaccurate representation of reality**.

---

# 3. Structural Visibility

## 3.1 Requirement for Referencing

A reference may only be formed if:

- the target Area or Resolution is discoverable  
- and locally available or present in review context  

References to unknown or unresolved targets are invalid.

---

## 3.2 Dependency on Discoverability

Structural visibility depends on:

- Cross-Area Discoverability  
- explicit acquisition of foreign graphs  

The system does not assume global visibility.

---

# 4. Reference Model

## 4.1 Reference Targets

Relationships may be expressed through references to:

- **Areas** (boundary-level, contextual)  
- **Resolutions** (artifact-level, structural)  

---

## 4.2 Resolution-Level References (Primary Mechanism)

Resolution references define:

- decision dependencies  
- alignment anchors  
- cross-area structural linkage  

These are the **primary source of structural truth**.

---

## 4.3 Area-Level References (Contextual Only)

Area references:

- may exist as contextual or descriptive constructs  
- may be used during review or workflow  

However:

> Area-level relationships are NOT authoritative structural primitives.

They must not:

- replace resolution-level references  
- define structural truth independently  

---

## 4.4 No Implicit Relationships

The system must not:

- infer area relationships from context alone  
- infer resolution dependencies from area proximity  

All structural relationships must originate from explicit references.

---

# 5. Runtime-Derived Area Relationship Graph

## 5.1 Definition

The Runtime MUST derive an **active area relationship graph**.

This graph represents:

> the current set of structurally relevant cross-area relationships.

---

## 5.2 Source of Derivation

The area relationship graph is derived from:

- active, reviewed Resolution references  
- structurally integrated artifacts  

It must NOT be derived from:

- provisional references  
- unreviewed imports  
- isolated foreign graphs  

---

## 5.3 Active vs Historical Structure

Two distinct structural views exist:

### A. Active Structure (Runtime-Derived)

- derived from active resolutions  
- represents current relationships  
- used for:
  - federation posture  
  - current CAS topology  
  - coordination boundaries  

---

### B. Historical Structure (Full Graph)

- includes all commits and supersession history  
- used for:
  - lineage analysis  
  - alignment computation (CAE)  
  - historical reasoning  

---

## 5.4 Purpose of Separation

> Active structure defines current topology.  
> Historical structure preserves full truth.

The system must not conflate these.

---

# 6. Relational Integrity

Relational integrity defines whether declared relationships are structurally valid.

---

## 6.1 Validity Requirements

All relationships must be:

- explicitly declared (via resolutions)  
- correctly typed  
- resolvable  
- non-ambiguous  

---

## 6.2 Boundary vs Artifact Integrity

- Resolution references define **true structural relationships**  
- Area relationships are **derived projections**, not primary truth  

Misuse includes:

- relying on area relationships without resolution anchors  
- assuming structure from context  

---

## 6.3 Independence of Layers

- Resolution graph = source of truth  
- Area graph = runtime projection  

Both must remain conceptually distinct.

---

# 7. Relational Completeness

## 7.1 Definition

> A structure is complete when all real dependencies are explicitly represented in resolution references.

---

## 7.2 Incomplete Structures

A structure is incomplete when:

- an Area depends on another Area but no resolution reference exists  
- decisions rely on external context without referencing it  
- coordination is real but not modeled  

---

## 7.3 Nature of Incompleteness

Incomplete structure results in:

- hidden dependencies  
- false independence  
- inaccurate system representation  

This is a **structural deficiency**, not a neutral state.

---

# 8. Structural Failure Modes

## 8.1 Invisible Dependency

**Condition:**

- Real dependency exists  
- No resolution reference exists  

**Effect:**

- Runtime area graph omits the relationship  
- CAS treats areas as independent  

---

## 8.2 False Independence

**Condition:**

- Areas are coupled in reality  
- No structural references exist  

**Effect:**

- CAS reports stability  
- coordination is unmanaged  

---

## 8.3 Vague Alignment

**Condition:**

- weak or indirect references only  
- no clear resolution anchors  

**Effect:**

- alignment appears present  
- lacks operational meaning  

---

## 8.4 Fragmented Graph

**Condition:**

- partial or inconsistent referencing  

**Effect:**

- incomplete propagation  
- inconsistent structural interpretation  

---

## 8.5 Delayed Structural Shock

**Condition:**

- missing relationships later introduced  

**Effect:**

- sudden CAS changes  
- apparent instability  

Cause:

> previously hidden structure becomes visible.

---

# 9. Interaction with CAS and CCare

## 9.1 CAS Behavior

CAS:

- consumes both:
  - active area relationship graph  
  - full historical graph  

CAS does not:

- infer missing relationships  

---

## 9.2 False Alignment Condition

If structure is incomplete:

- CAS may show alignment  
- because missing edges suppress tension  

---

## 9.3 CCare Signals

When structure is incomplete:

- check-ins may show:
  - tension  
  - inconsistency  
  - unexplained drift  

This reflects:

> influence from unmodeled relationships.

---

## 9.4 CAS vs Reality Divergence

Condition:

- CAS = stable  
- CCare = stressed  

Indicates:

> structural incompleteness.

---

# 10. Invariants

- Relationships must originate from explicit resolution references  
- Targets must be visible before referencing  
- Runtime must derive the active area relationship graph  
- Derived area graph must reflect only reviewed, active structure  
- Historical and active structures must remain distinct  
- No implicit relationships may exist  
- Missing relationships are structural deficiencies  
- CAS must not infer undeclared structure  

---

# 11. Mental Model

- Resolutions define relationships  
- Runtime derives current topology  
- Area graph is a projection, not truth  
- Full graph preserves history  
- Missing edges = missing truth  
- CAS reflects visible structure  
- CCare reveals hidden pressure  

---

# 12. Final Principle

Charter requires structural honesty at the artifact level.

Relationships must be:

- explicitly declared  
- visible  
- and structurally complete  

The Runtime may project structure,  
but only **explicit references define truth**.

If relationships are not declared:

- they do not exist structurally  
- alignment becomes misleading  
- and reality diverges from representation