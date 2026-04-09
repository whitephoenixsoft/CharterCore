# Charter Alignment System — Structural Detection Model (v4)

Status: FOUNDATIONAL  
Applies to: Charter Alignment System (CAS), Charter Alignment Engine (CAE)  
Depends On:  
- Structural Visibility & Relational Integrity Specification  
- Cross-Area Discoverability Specification  
- Charter Commit System (CCS)  
- Charter Structural Graph (CSG)  
- Charter Care Substrate (CCare)  
- Optional: Charter Identity Substrate (CIS)  

Does NOT define: legitimacy, structural correctness, workflow orchestration, or guidance interpretation  

---

# 1. Purpose

This document defines the **detection capabilities of the Charter Alignment System (CAS)**.

CAS exists to:

- derive alignment-related patterns from structure and signals  
- summarize system state using the **semantic lattice**  
- expose categorized, queryable views of alignment dynamics  
- surface structural–observational relationships without interpretation  

CAS does not determine truth, correctness, or action.  
It describes:

> **what is observable from declared structure (legitimate or investigative) and signals**

---

# 2. Core Principles

## 2.1 Query-Based, Non-Interruptive

> CAS produces output only when queried.

CAS:

- does not emit alerts by default  
- does not monitor continuously  
- does not generate unsolicited signals  

It is a **read-oriented analytical system**.

---

## 2.2 Deterministic and Rebuildable

All CAS outputs must be:

- deterministic  
- reproducible from inputs  
- derivable from:
  - commit artifacts (CCS)  
  - structural graph (CSG)  
  - signals (CCare)  

The Alignment State Store is:

- derived  
- rebuildable  
- non-authoritative  

---

## 2.3 Descriptive, Not Normative

> CAS describes patterns. It does not judge them.

CAS must not:

- prescribe actions  
- assign correctness  
- infer intent  
- convert observations into obligations  

---

## 2.4 Structure-Aware, Not Structure-Validating

CAS:

- consumes declared structure from CSG  
- operates over both complete and incomplete graphs  

CAS must not:

- validate graph correctness  
- enforce structural rules  
- repair missing relationships  

---

## 2.5 Visibility-Bound

> CAS operates only on visible, declared structure.

CAS cannot:

- infer missing relationships as truth  
- detect hidden dependencies directly  

It may only describe:

- patterns within visible structure  
- anomalies between structure and signals  

---

## 2.6 Projection-Aware

> CAS operates over explicit structural projections.

CAS may analyze:

- resolution-only projections  
- item-only projections  
- mixed projections  

Projection selection affects:

- propagation  
- aggregation  
- detection outcomes  

CAS must not:

- redefine structure across projections  

---

# 3. Inputs

CAS operates on the following inputs:

---

## 3.1 Structural Input (CSG)

CAS consumes structural projections containing:

- resolution nodes (legitimate structure)  
- item nodes (investigative structure)  
- structural relationships:
  - reference  
  - derivation (`derived_from`)  
  - supersession (resolution-only)  

This is the **structural field** over which CAS operates.

---

## 3.2 Observational Input (CCare)

Signals include:

- semantic states  
- confidence  
- timestamps  
- silence patterns  

These represent **observed conditions**, not authority.

---

## 3.3 Commit Artifacts (CCS)

Includes:

- resolutions  
- items (when materialized)  
- receipts  
- annotations  

Used for:

- reconstruction  
- lineage context  
- auditability  

---

## 3.4 Structural Scope Input (Optional — CIS)

CAS may operate over identity-defined scopes.

These scopes:

- define bounded regions of the structural graph  
- may overlap  
- are derived from CIS over CSG  

CAS may compute:

- identity-scoped alignment  
- boundary pressure  
- overlap tension  
- cross-identity divergence  

---

## 3.5 Bounded Contexts (Optional)

CAS may operate over:

- CDS (investigative contexts)  
- review contexts (provisional structure)  

These contexts are:

- visible  
- analyzable  
- non-authoritative  

---

## 3.6 Alignment State Store

- derived alignment states  
- cached semantic outputs  
- query-optimized structures  

Must remain:

- rebuildable  
- non-authoritative  

---

# 4. Computation Model

CAS operates in layered computation:

---

## 4.1 Numeric Layer

Computes raw metrics such as:

- drift  
- variance  
- signal density  
- recency  

This layer is:

- purely mathematical  
- node-level  
- projection-aware  

---

## 4.2 Semantic Lattice

Maps numeric outputs into **human-readable states**.

Examples:

- aligned  
- misaligned  
- unstable  
- reduced_capacity  
- recovering  

The semantic lattice:

- compresses mathematical state  
- preserves interpretability  
- does not introduce meaning beyond classification  

---

## 4.3 View Layer

Organizes semantic outputs into:

- queryable perspectives  
- structural lenses  
- domain-specific interpretations  

---

# 5. Detection Domains

CAS detects patterns across multiple domains:

---

## 5.1 Alignment Posture Detection

Derived from:

- signals  
- structure  
- propagation  

Detects:

- alignment  
- misalignment  
- uncertainty  
- reduced capacity  
- stabilization  

---

## 5.2 Volatility Detection

Describes temporal dynamics:

- stable  
- unstable  
- increasing  
- decreasing  
- oscillating  

---

## 5.3 Propagation Detection

Detects how influence moves across structure:

- limited propagation  
- delayed propagation  
- asymmetric propagation  
- containment  

Supports:

- mixed-node propagation  
- lineage-aware propagation  

---

## 5.4 Observational Tension Detection

Derived from:

- signal density  
- conflicting states  

Detects:

- tension zones  
- instability clusters  

---

## 5.5 Structural–Observational Mismatch Detection

Detects:

> divergence between structure and signals

Examples:

- stable structure with unstable signals  
- disconnected nodes with correlated signals  
- highly connected structure with no propagation  

Indicates:

- missing structure  
- hidden dependencies  
- coordination gaps  

---

## 5.6 Simulation Detection (NEW)

CAS may analyze **investigative (item-based) structure**.

Detects:

- simulated instability  
- hypothetical cascades  
- pre-legitimacy tension  

These outputs are:

- non-authoritative  
- exploratory  

---

## 5.7 Reconciliation Divergence Detection (NEW)

Detects divergence between:

- legitimate structure (resolutions)  
- investigative structure (items)  

Examples:

- simulated changes not reflected in legitimacy  
- outdated investigative assumptions  

---

## 5.8 Identity-Aware Detection (Optional)

When CIS is present, CAS may detect:

- boundary pressure  
- overlap tension  
- cross-identity misalignment  
- identity-scoped instability  

---

# 6. Semantic Output Model

CAS outputs consist of:

---

## 6.1 Semantic State

Examples:

- aligned  
- misaligned  
- reduced_capacity  

---

## 6.2 Volatility Descriptor

Examples:

- stable  
- increasing  
- oscillating  

---

## 6.3 Category / View

Examples:

- posture  
- propagation  
- tension  
- mismatch  
- simulation  

---

## 6.4 Example Output

State: reduced_capacity  
Volatility: increasing  
Category: propagation_constraint  

---

# 7. Detection Semantics

CAS outputs are:

- descriptive  
- non-causal  
- non-prescriptive  

Example:

“Observed coupling without structural linkage”

Means:

- correlation exists  
- no structural edge exists  

It does not assert causation.

---

# 8. Non-Detectable Conditions

CAS cannot detect:

- undeclared relationships as truth  
- intent or motivation  
- correctness  
- legitimacy  
- authority  

Absence of detection ≠ absence of reality.

---

# 9. Failure Interpretation

## 9.1 False Stability
Missing structure suppresses propagation.

## 9.2 Unexplained Tension
Signals conflict without structural explanation.

## 9.3 Sudden Instability
New structure reveals hidden dependencies.

## 9.4 Simulation Drift
Investigative structure diverges from legitimacy.

---

# 10. Invariants

- CAS is read-only  
- CAS is deterministic and rebuildable  
- CAS is projection-aware  
- CAS does not validate structure  
- CAS does not infer undeclared relationships  
- CAS outputs are descriptive  
- node-class distinctions must be preserved  
- Alignment State Store is non-authoritative  

---

# 11. Mental Model

CAS is:

> a field analysis system over structure and signals

It operates over:

- structure (CSG)  
- observation (CCare)  
- optional scope (CIS)  

Through:

- math  
- semantic lattice  
- views  

---

# 12. Final Principle

CAS reveals how the system behaves  
based on what has been made visible.

It operates over:

- decisions  
- investigations  
- simulations  

using:

- mathematics  
- semantic compression  
- structural awareness  

It does not decide what is true.

It makes visible:

> what the system appears to be doing  
given what has been declared and observed.