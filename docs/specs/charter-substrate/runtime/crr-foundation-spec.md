# Charter Resolution Recontextualization (CRR) — Foundation Specification

Status: FOUNDATIONAL  
Intent: Define how resolutions are recontextualized across scopes without rewriting history  
Scope: Promotion, demotion, copy, move, and derivation relationships between resolutions  
Depends On: Charter Commit System (CCS), Charter Structural Graph (CSG)  
Does NOT Define: legitimacy, workflow orchestration, identity (CIS), alignment (CAS), or guidance (CGL)  

---

# 1. Purpose

The Charter Resolution Recontextualization (CRR) defines how resolutions may be:

- promoted to higher-order scopes  
- demoted to lower-order scopes  
- copied across areas  
- moved across areas  

without mutating or rewriting existing artifacts.

It exists to:

- enable late discovery of correct scope  
- support structural evolution over time  
- preserve full historical lineage  
- allow flexible reorganization of decision hierarchies  

CRR is a **structural lineage model**, not a workflow system.

---

# 2. Core Principle

> Recontextualization creates new resolutions. It never mutates existing ones.

All changes in scope or context must be expressed as:

- new resolution artifacts  
- explicit lineage relationships  

---

# 3. Core Concept

## 3.1 Recontextualization

Recontextualization is the act of:

> creating a new resolution in a different scope based on one or more existing resolutions.

This includes:

- promotion (lower → higher scope)  
- demotion (higher → lower scope)  
- lateral transfer (area-to-area)  

---

## 3.2 Derivation Relationship

CRR defines a canonical structural relationship:

### `derives_from`

A resolution may declare that it derives from one or more prior resolutions.

Properties:

- explicit  
- directional  
- non-destructive  
- may cross areas  

---

# 4. Recontextualization Types (Derived Semantics)

CRR does not enforce semantic interpretation, but common patterns include:

---

## 4.1 Promotion

- lower-scope resolution → higher-scope resolution  

Example:

- design decision → mission principle  

---

## 4.2 Demotion

- higher-scope resolution → lower-scope resolution  

Example:

- mission principle → team-level implementation  

---

## 4.3 Copy

- resolution duplicated across contexts  
- source remains active  

---

## 4.4 Move

- resolution recontextualized  
- source is retired  

---

# 5. Source Lifecycle Handling

Recontextualization may optionally affect the source resolution.

---

## 5.1 Retain Source

- original resolution remains active  
- both source and derived resolution coexist  

---

## 5.2 Retire Source

- original resolution is explicitly retired  
- derived resolution becomes the active representation  

---

## 5.3 Principle

> Retirement is independent from derivation and must be explicit.

---

# 6. Multi-Resolution Derivation

A resolution may derive from multiple sources.

This supports:

- consolidation (many → one)  
- synthesis of ideas  
- structural merging  

---

# 7. Annotation

Annotations are supported for all recontextualization actions.

---

## 7.1 Annotation Scope

Annotations may be attached to:

- derivation relationships  
- new resolutions  
- source resolutions (if updated via workflow context)  

---

## 7.2 Annotation Properties

Annotations are:

- optional  
- non-authoritative  
- descriptive  

They may include:

- rationale for recontextualization  
- explanation of scope change  
- notes on consolidation or decomposition  
- contextual reasoning  

---

## 7.3 Principle

> Annotation preserves intent without affecting structural correctness.

---

# 8. Relationship to Other Modules

---

## 8.1 Runtime Layer

- orchestrates promotion, demotion, copy, and move workflows  
- determines whether to retain or retire source  

---

## 8.2 Deliberate (CDS)

- may reshape or synthesize resolutions prior to recontextualization  

---

## 8.3 Review Workflow

- evaluates and admits recontextualized resolutions  
- ensures authority boundaries are respected  

---

## 8.4 CCS

- records new resolution commits  
- preserves derivation references  

---

## 8.5 CSG

- materializes derivation edges as part of the structural graph  

---

## 8.6 CIS

- may rebind identity and scope based on new structure  

---

## 9. Invariants

The following must always hold:

- recontextualization creates new resolution artifacts  
- original resolutions are never mutated  
- derivation relationships are explicit  
- derivation does not imply legitimacy  
- source retirement is explicit and separate  
- multiple derivation sources are allowed  
- cross-area derivation is allowed  
- annotations do not alter structural meaning  

Violation of these breaks CRR correctness.

---

# 10. Mental Model

CRR is:

- a system of structural lineage  
- a mechanism for evolving decision hierarchy  
- a record of how meaning shifts across scope  

It is not:

- a workflow engine  
- a legitimacy system  
- an interpretation layer  

---

# 11. Final Principle

The Charter Resolution Recontextualization model ensures that:

- decisions can move across scopes  
- structure can evolve over time  
- meaning can be refined without rewriting history  

Every change in context is preserved as new structure —  
never as mutation of the past.