# Charter — Module Boundaries & Non-Responsibilities Specification

Status: FOUNDATIONAL (LOCKED)  
Depends On: Canonical Naming Specification  
Purpose: Define strict boundaries for each Charter module and explicitly state what each module must never do  

---

# 1. Purpose

This document establishes **hard boundaries** between Charter modules.

It exists to:

- prevent responsibility drift  
- eliminate implicit coupling  
- preserve invariants across evolution (V1–V7+)  
- ensure each module remains independently usable and composable  

Each module is defined by:

- what it **owns**
- what it **does not own**
- what it **must never do**

---

# 2. Core Boundary Principle

## MB-01 — Negative Definition Is Required

Every module must be defined not only by what it does,  
but by what it explicitly **does not do**.

If a module begins to take on responsibilities outside its boundary,  
it is no longer the same module.

---

## MB-02 — No Authority Leakage

No module may:

- infer authority  
- create legitimacy outside the Legitimacy Engine  
- convert observation into obligation  

---

## MB-03 — No Layer Collapse

Modules must not:

- merge responsibilities  
- reinterpret artifacts outside their domain  
- assume upstream or downstream roles  

---

# 3. Module Definitions

---

## 3.1 Legitimacy Engine

### Owns

- Sessions  
- Authority evaluation  
- Candidate evaluation  
- Resolution creation  
- Legitimacy receipts  

### Does NOT Own

- Storage  
- Contexts  
- Identity  
- Signals or check-ins  
- Alignment  
- Transport  

### Must NOT

- Persist data  
- Observe runtime behavior  
- Interpret outcomes  
- Compute alignment  
- Infer intent  
- Modify identity or scope  
- Execute workflows  

---

## 3.2 Persistence Layer

### Owns

- Immutable object storage  
- Reference storage  
- Audit logs  
- Low-level append-only persistence  

### Does NOT Own

- Legitimacy  
- Commit semantics  
- Identity  
- Alignment  
- Interpretation  

### Must NOT

- Interpret stored data  
- Enforce structure beyond storage rules  
- Compute lineage meaning  
- Merge or rewrite history  
- Expose authority  

---

## 3.3 Runtime Layer

### Owns

- Context isolation  
- Session orchestration  
- Baseline review  
- Import/export coordination  
- Engine invocation  

### Does NOT Own

- Legitimacy  
- Commit structure  
- Alignment computation  
- Identity definition  
- Signal semantics  

### Must NOT

- Create legitimacy  
- Modify authority rules  
- Infer intent  
- Interpret alignment  
- Collapse context boundaries  
- Execute automatic decisions  

---

## 3.4 Charter Commit System (CCS)

### Owns

- Commit definitions  
- Commit taxonomy  
- Artifact identity rules  
- Commit lineage structure  

### Does NOT Own

- Storage  
- Alignment computation  
- Identity semantics  
- Transport  
- Legitimacy  

### Must NOT

- Store commits  
- Interpret commit meaning  
- Compute alignment  
- Create authority  
- Merge commits into canonical state  

---

## 3.5 Charter Commit Store

### Owns

- Local append-only commit storage  
- Commit retrieval and indexing  
- Local artifact availability  

### Does NOT Own

- Commit definitions  
- Alignment computation  
- Identity semantics  
- Transport  
- Legitimacy  

### Must NOT

- Interpret commit meaning  
- Modify commit content  
- Merge histories  
- Act as global source of truth  
- Compute alignment  

---

## 3.6 Charter Care Substrate (CCare)

### Owns

- Check-ins  
- Requests  
- Supportability signals  
- Silence as a state  

### Does NOT Own

- Identity  
- Versioning  
- Alignment computation  
- Telemetry ingestion (system-level concern)  
- Legitimacy  

### Must NOT

- Trigger actions  
- Enforce decisions  
- Create obligations  
- Interpret identity changes  
- Diagnose root cause  
- Optimize systems  

---

## 3.7 Charter Lineage Substrate (CLL)

### Owns

- Identity definition  
- Scope and purpose  
- Versioning (mechanical)  
- Deprecation states  
- Sunset lifecycle  

### Does NOT Own

- Behavior observation  
- Signals  
- Alignment computation  
- Legitimacy  
- Telemetry  

### Must NOT

- Observe runtime behavior  
- Interpret system health  
- Trigger action  
- Collapse identity boundaries  
- Rewrite history  
- Infer intent  

---

## 3.8 Charter Alignment Engine (CAE)

### Owns

- Derived alignment computation  
- Drift and variance detection  
- Structural propagation  
- Predictive alignment indicators  

### Does NOT Own

- Legitimacy  
- Commit storage  
- Identity definition  
- Signal creation  
- Interpretation (narrative)  

### Must NOT

- Create or modify commits  
- Create legitimacy  
- Trigger actions  
- Diagnose root cause  
- Enforce decisions  
- Convert signals into obligations  

---

## 3.9 Charter Guidance Layer (CGL)

### Owns

- Interpretation (exegesis)  
- Summaries  
- Narrative explanations  
- Drift descriptions  

### Does NOT Own

- Legitimacy  
- Alignment computation  
- Identity definition  
- Signal generation  
- Storage  

### Must NOT

- Modify any state  
- Create authority  
- Imply obligation  
- Enforce action  
- Replace human judgment  
- Present suggestions as commands  

---

## 3.10 Charter Relay System (CRS)

### Owns

- Commit transport  
- Append-only archival storage  
- Push/fetch operations  
- Timestamp preservation  

### Does NOT Own

- Legitimacy  
- Alignment  
- Identity  
- Canonical state  
- Commit interpretation  

### Must NOT

- Interpret commits  
- Merge histories  
- Reconstruct state  
- Create authority  
- Enforce synchronization  

---

# 4. Cross-Module Constraints

## MB-04 — No Upward Mutation

Higher modules must not:

- mutate lower-layer artifacts  
- redefine lower-layer semantics  

---

## MB-05 — No Downward Assumption

Lower modules must not:

- assume how higher layers will interpret data  
- encode meaning beyond their responsibility  

---

## MB-06 — Composition Without Fusion

Modules may be composed together,  
but must remain:

- logically separable  
- independently testable  
- independently replaceable  

---

# 5. Failure Modes This Prevents

This structure prevents:

- Legitimacy being influenced by alignment  
- Alignment becoming authoritative  
- Guidance becoming prescriptive  
- Relay becoming a source of truth  
- Storage becoming interpretation  
- Care becoming control  
- Lineage becoming governance  

---

# 6. Final Principle

Each module answers exactly one kind of question:

- Legitimacy Engine → “What is legitimate?”  
- Runtime → “How do we orchestrate decisions?”  
- Persistence → “What is stored?”  
- CCS → “What is a commit?”  
- Commit Store → “What do we have locally?”  
- CCare → “What is being observed relative to decisions?”  
- CLL → “Who are we over time?”  
- CAE → “What does structure + observation imply?”  
- CGL → “How can this be understood?”  
- CRS → “What can be transported or archived?”  

If a module starts answering another module’s question,  
the architecture is drifting.

This document exists to stop that from happening.