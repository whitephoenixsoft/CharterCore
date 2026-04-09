# Charter — Structural Visibility & Relational Integrity Specification (vNext)

Status: FOUNDATIONAL  
Applies to: Runtime Layer, Charter Structural Graph (CSG), Charter Alignment System (CAS), Charter Deliberate Substrate (CDS), Charter Care Substrate (CCare)  
Depends On: Cross-Area Discoverability Specification  
Does NOT define: alignment computation, detectability algorithms, or guidance interpretation  

---

# 1. Purpose

This document defines the **structural requirements for representing relationships across all Charter artifacts**.

It establishes:

- what must be visible to form relationships  
- what makes relationships structurally valid  
- how relationships exist across Areas and artifacts  
- how structure behaves across legitimacy and investigation  
- what constitutes structural completeness  
- what failures occur when relationships are missing or misrepresented  

This document defines **correctness of structure**, not how structure is discovered, interpreted, or evaluated.

---

# 2. Core Principles

## 2.1 Visibility Precedes Relationship

> A relationship cannot exist structurally unless its target is visible.

All references must resolve to:

- a locally available artifact  
- or an artifact present in a valid bounded context (e.g., review or CDS)

---

## 2.2 Relationships Must Be Explicit at the Artifact Level

> All structural relationships must originate from explicit artifact-level references.

Relationships are expressed through:

- node-level references (CSG)

Nodes may include:

- Resolution nodes  
- Item nodes  

No relationship may be:

- inferred  
- assumed  
- derived implicitly  

---

## 2.3 Structure Reflects Declared Reality

> Charter represents only what is explicitly declared.

The system:

- does not infer relationships  
- does not correct missing structure  
- does not assume completeness  

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

A reference may only be formed if the target is:

- discoverable  
- and locally available or present in a bounded context  

---

## 3.2 Bounded Contexts

References may exist in different contexts:

### A. Durable Context (Legitimacy / CSG)
- must reference locally available artifacts  
- must be fully resolvable  

### B. Review Context
- may reference foreign or provisional artifacts  
- references are temporary and non-durable  

### C. CDS Context (Investigation / Simulation)
- may reference:
  - resolutions  
  - items  
  - incomplete or evolving structures  

These references remain:

- non-legitimate  
- bounded to CDS  

---

## 3.3 Discoverability Dependency

Structural visibility depends on:

- discoverability  
- acquisition of foreign graphs  

The system does not assume global visibility.

---

# 4. Structural Model (Node-Level)

## 4.1 Node-Based Relationships

All structural relationships exist between **nodes in CSG**.

Node classes include:

- Resolution nodes (legitimate artifacts)  
- Item nodes (investigative artifacts)  

---

## 4.2 Node-Class Principle

> Node class must remain explicit and must not be collapsed.

- Items are not resolutions  
- Investigative structure is not legitimate structure  
- Mixed graphs must preserve distinction  

---

## 4.3 Relationship Types

Structural relationships may include:

- reference  
- derivation (`derived_from`)  
- supersession (resolution-only)  

All relationships:

- must be explicit  
- must be declared at artifact level  
- must not be inferred  

---

# 5. Structural Projections

## 5.1 Projection Model

CSG supports multiple projections:

- Resolution-only  
- Item-only  
- Mixed  

---

## 5.2 Projection Principle

> Structural visibility and integrity are evaluated within a projection.

- projections do not change structure  
- they expose subsets of structure  

---

## 5.3 Usage

- Resolution projection → legitimacy-aware structure  
- Item projection → investigative/simulation structure  
- Mixed projection → reconciliation and hybrid analysis  

---

# 6. Dual Structure Model

Charter supports two coexisting structural domains:

---

## 6.1 Legitimate Structure

- resolution nodes only  
- admitted through review + session  
- durable  

---

## 6.2 Investigative Structure

- item nodes  
- created within CDS  
- may include simulated relationships  
- non-legitimate  

---

## 6.3 Principle

> Both domains must follow the same structural rules, but differ in authority.

---

# 7. Reconciliation-Aware Structure

## 7.1 Bidirectional Traceability

Structure may flow between domains:

### A. Resolution → Item
- via reconciliation  
- item has `derived_from = resolution_id`

### B. Item → Resolution
- via review + session  
- resolution may include `derived_from = item_id`

---

## 7.2 Structural Preservation

Reconciliation must preserve:

- lineage  
- structural relationships (as context)

But must not:

- create legitimacy  
- mutate source artifacts  

---

## 7.3 Principle

> Structure may be mirrored across domains without collapsing them.

---

# 8. Area and Boundary Model

## 8.1 Areas

Areas are:

- boundaries  
- containers of partial graphs  

They are not nodes.

---

## 8.2 Area Relationships

Area relationships are:

- **derived projections**, not primary structure  

They must be derived from:

- node-level relationships (CSG)  

---

## 8.3 Principle

> Node-level structure is truth. Area-level structure is projection.

---

# 9. Relational Integrity

## 9.1 Validity Requirements

All relationships must be:

- explicit  
- correctly typed  
- resolvable within context  
- non-ambiguous  

---

## 9.2 Node-Level Integrity

- relationships must originate from nodes  
- node class must remain explicit  
- cross-node-class relationships are allowed  

---

## 9.3 No Implicit Relationships

The system must not:

- infer dependencies  
- infer hierarchy  
- infer causality  

---

# 10. Relational Completeness

## 10.1 Definition

> A structure is complete when all real dependencies are explicitly represented.

---

## 10.2 Incompleteness

Occurs when:

- dependencies exist but are not declared  
- relationships are hidden or omitted  

---

## 10.3 Effects

- hidden dependencies  
- false independence  
- inaccurate system representation  

---

# 11. Structural Failure Modes

## 11.1 Invisible Dependency

- real dependency exists  
- no explicit relationship  

Effect:

- structure appears independent  

---

## 11.2 False Stability

- missing edges suppress propagation  

Effect:

- CAS shows stability incorrectly  

---

## 11.3 Fragmented Graph

- partial relationships  

Effect:

- incomplete propagation  

---

## 11.4 Simulation-Reality Divergence

- investigative structure differs from legitimate structure  

Effect:

- unresolved tension between domains  

---

## 11.5 Delayed Structural Shock

- missing structure later introduced  

Effect:

- sudden CAS shifts  

---

# 12. Interaction with CAS and CCare

## 12.1 CAS Behavior

CAS:

- consumes structural projections  
- does not infer missing structure  

---

## 12.2 Projection Awareness

CAS may operate on:

- resolution-only  
- item-only  
- mixed projections  

---

## 12.3 False Alignment Condition

If structure is incomplete:

- CAS may show alignment  
- due to missing relationships  

---

## 12.4 CCare Signals

When structure is incomplete:

- signals may show tension  
- unexplained drift  

---

## 12.5 Insight

> CAS reflects visible structure.  
> CCare may reveal pressure from missing structure.

---

# 13. Invariants

- relationships must be explicit  
- relationships must originate from nodes  
- node class must remain explicit  
- projections must not alter structure  
- area relationships must be derived, not primary  
- reconciliation must preserve lineage  
- no implicit relationships may exist  
- missing relationships are structural deficiencies  
- CAS must not infer undeclared structure  

---

# 14. Mental Model

- structure exists at the node level  
- nodes may be legitimate or investigative  
- projections expose different views  
- reconciliation connects domains  
- areas are boundaries, not structure  
- missing edges = missing truth  
- CAS reflects visible structure  
- CCare reveals hidden pressure  

---

# 15. Final Principle

Charter requires structural honesty across **all forms of structure**:

- decisions  
- investigations  
- simulations  

Relationships must be:

- explicit  
- visible  
- and complete  

The system does not infer reality —  
it only reflects what is declared.

If structure is missing:

- alignment becomes misleading  
- investigation diverges from decision  
- and reality is only partially represented