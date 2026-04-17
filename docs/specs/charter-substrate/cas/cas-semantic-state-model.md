# Charter — Semantic State Model

Status: FOUNDATIONAL (DRAFT)  
Applies to: CAS, VDS, VLS, CLI, and all human-facing interpretation layers  
Depends On: CCare Signal Model, CIS Identity Model, CSG Structure Model, Determinism, Non-Interpretation Principle  
Does NOT define: computation formulas, enforcement logic, decision rules, or UI rendering  

---

# 1. Purpose

This document defines the **Semantic State Model** for Charter.

It exists to:

- provide human-readable interpretations of system condition  
- unify how system state is expressed across substrates  
- enable consistent interpretation across different scopes of the graph  
- bridge computational outputs (CAS) and human understanding  

Semantic States are the primary way Charter communicates system condition to humans.

---

# 2. Core Principle

> Semantic States describe system condition. They do not prescribe action.

Semantic States must remain:

- descriptive  
- non-authoritative  
- non-interpreting  
- deterministic given inputs  

---

# 3. Definition

A Semantic State is:

> a derived, human-readable interpretation of system condition for a defined projection scope.

It is computed from:

- signals  
- structural relationships  
- identity boundaries  
- observed system behavior  
- temporal patterns  

---

# 4. Signal Foundation

Semantic States are derived from signals.

---

## 4.1 Canonical Signal Set

The system defines the following signal types:

- alignment  
- misalignment  
- uncertainty  
- reduced_capacity  
- intentional_pause  
- need_reassessment  

---

## 4.2 Signal Properties

Signals must:

- be explicit  
- be typed  
- carry provenance  
- include confidence  

---

## 4.3 Confidence Levels

Each signal includes a confidence level:

- low  
- medium  
- high  

Confidence represents:

> the consistency, stability, and reliability of observed conditions.

---

# 5. Derived Conditions

Semantic States may derive higher-level conditions from signal patterns.

---

## 5.1 Derived Uncertainty

Uncertainty is derived when:

- signals conflict  
- structure is incomplete  
- interpretation cannot stabilize  

Definition:

> Uncertainty is the inability to resolve a stable interpretation of system condition given available signals and structure.

---

## 5.2 Derived Capacity

Capacity is not directly measured.

It is inferred.

Definition:

> Capacity is the system’s ability to convert intent into coherent action.

Capacity decreases when:

- conflict increases  
- coordination overhead increases  
- uncertainty increases  
- signal volatility increases  
- dependency flow is constrained  

---

# 6. Temporal Degradation

Semantic States must account for time-based system behavior.

---

## 6.1 Flow Degradation

Flow Degradation occurs when:

- dependency traversal slows or stalls  
- blockers persist  
- coordination delays increase  

Key indicators include:

- blocker duration  
- blocker density  
- failure to restore flow  

---

## 6.2 Persistent Reduced Capacity

Reduced capacity becomes structurally significant when:

- it persists over time  
- recovery does not occur  
- flow remains degraded  

Persistent reduced capacity is a primary precursor to:

- need_reassessment  

---

## 6.3 Feedback Degradation (Stale Alignment)

A system may remain structurally aligned while losing validation over time.

Definition:

> Feedback Degradation occurs when alignment is not reinforced through activity, signals, or observation.

Characteristics:

- lack of new decisions  
- absence of signals  
- absence of feedback loops  
- stable but untested structure  

---

## 6.4 Confidence Decay

In the absence of reinforcement:

> Confidence in alignment must decrease over time.

This ensures that:

- silence is not mistaken for stability  
- unvalidated systems are not over-trusted  

---

# 7. Projection Scopes

Semantic States are always computed relative to a projection scope.

---

## 7.1 Scope Types

### Node Scope
- single resolution or item  
- most granular  

---

### Local Graph Scope (Area or Subgraph)
- bounded operational region  
- typically corresponds to an Area  

---

### Identity Scope

- defined by CIS boundary  
- may span multiple Areas  
- represents meaningful ownership or responsibility  

Examples:

- teams  
- departments  
- leadership domains  

---

### Global Scope

- entire Charter Structure Graph  

---

## 7.2 Principle

> Projection scopes are different views over the same structure.

They are not hierarchical layers.

---

# 8. Identity-Centric Interpretation

Identity Scope is the primary human-facing aggregation.

---

## 8.1 Properties

- defined by structural boundaries (CIS)  
- may overlap with other identities  
- may cross Area boundaries  

---

## 8.2 Rationale

Humans reason about:

- teams  
- organizations  
- ownership  

not:

- graph partitions  

---

## 8.3 Constraint

Semantic States must not infer identity boundaries.

They must use explicitly defined CIS identities.

---

# 9. Semantic State Structure

A Semantic State consists of:

---

## 9.1 Core State

One or more dominant conditions:

- alignment  
- misalignment  
- reduced_capacity  
- uncertainty  
- intentional_pause  
- need_reassessment  

---

## 9.2 Modifiers

Modifiers provide additional context.

---

### Trend

Indicates directional change over time:

- improving  
- declining  
- stable  

---

### Volatility

Indicates stability of signals:

- low  
- medium  
- high  

---

### Confidence

Represents reliability of the state:

- low  
- medium  
- high  

---

## 9.3 Interpretation Principle

> Core State represents structural condition.  
> Modifiers represent temporal and stability characteristics.

---

## 9.4 Composition Example

Example Semantic State:

- Core: reduced_capacity  
- Trend: declining  
- Volatility: high  
- Confidence: high  

---

# 10. Aggregation Model (Conceptual)

Semantic States are derived through:

1. signal collection  
2. aggregation by scope  
3. conflict resolution  
4. dominant condition identification  
5. temporal evaluation  
6. modifier derivation  

This process must remain:

- deterministic  
- explicit  
- non-interpreting  

---

# 11. Constraints

Semantic States must not:

- assign blame  
- prescribe actions  
- reinterpret legitimacy  
- override structural truth  
- infer missing relationships  

---

# 12. Relationship to CAS

CAS is responsible for:

- computing signals  
- aggregating structure  
- evaluating temporal behavior  
- deriving Semantic States  

CAS does not:

- define meaning beyond structure  
- prescribe responses  

---

# 13. Relationship to Other Substrates

---

## CCare

- provides signal inputs  

---

## CIS

- defines identity boundaries for aggregation  

---

## CSG

- provides structural relationships  

---

## CDS

- provides observation items (e.g., blockers, delays)  

---

## VDS

- originates signals and threshold-triggered observations  

---

## VLS

- consumes Semantic States for lineage and visualization  

---

## CLI

- presents Semantic States to users  

---

# 14. Determinism

Given identical:

- signals  
- structure  
- identity boundaries  
- temporal inputs  

Semantic States must resolve to identical outputs.

---

# 15. Mental Model

Semantic States answer:

- what is happening  
- how stable the system is  
- whether the system can operate effectively  
- whether conditions are improving or degrading  

They do not answer:

- what should be done  
- who is at fault  

---

# 16. Final Principle

Semantic States make complex systems understandable without distorting their truth.

They translate structure, signals, and time into:

- clear  
- consistent  
- human-readable system conditions