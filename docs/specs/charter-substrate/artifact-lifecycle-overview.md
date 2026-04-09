# Charter — Artifact Lifecycle Overview (Revised vNext)

Status: FOUNDATIONAL (Canonical Overview)  
Applies to: All Charter Modules  
Purpose: Provide a unified, end-to-end view of how artifacts are created, transformed, preserved, interpreted, exchanged, and optionally removed  

---

# 1. Purpose

This document describes the **full lifecycle of artifacts in Charter** across:

- investigation (CDS)  
- legitimacy (Runtime + Engine)  
- structure (CSG)  
- identity (CIS)  
- alignment (CAS)  
- exchange (Federation)  

It answers:

- Where does structure originate?  
- How does investigation become legitimacy?  
- How is structure preserved and evolved?  
- How is meaning derived without authority?  
- How does structure move across systems?  
- How is local state safely removed?  

This document is **descriptive**, not prescriptive.

It defines how modules **connect without collapsing boundaries**.

---

# 2. Core Principle

> Artifacts move forward. Authority is created only once. Meaning is always derived.

Each stage:

- transforms **representation**, not truth  
- preserves **identity and lineage**  
- does not inherit authority from other layers  

---

# 3. Lifecycle Overview (High-Level)

Charter operates as **interconnected flows**, not a single pipeline.

---

## 3.1 Primary Structural Flow (Legitimacy Path)

Runtime / CDS  
↓  
Review Layer (Foreign Integration)  
↓  
Legitimacy Engine (Session)  
↓  
CCS (Commit Formation)  
↓  
Commit Store (Preservation)  
↓  
CSG (Structural Graph)  
↓  
CIS (Identity Overlay, optional)  
↓  
CAS (Alignment Computation)  
↓  
CGL (Interpretation)  

---

## 3.2 Investigative Flow (CDS Loop)

Signals / Structure  
↓  
CDS (Deliberate: Items, Observations, Simulation)  
↓  
Synthesis (LOCKED Items)  
↓  
Review Layer (integration path)  

---

## 3.3 Reconciliation Flow (Bidirectional)

Legitimacy → CDS  
CDS → Legitimacy (via Review + Session)  

---

## 3.4 Federation Flow

External Systems  
↓  
Discover → Query → Acquire  
↓  
Isolated Foreign Graph  
↓  
Review Layer  
↓  
Local Integration  

and:

Local Artifacts  
↓  
Emission Policy  
↓  
CRS Transport  

---

## 3.5 Archival & Purge Flow

Runtime / CDS State  
↓  
Archival Packaging  
↓  
CCS (Archive Commits)  
↓  
Commit Store  
↓  
Runtime Purge (optional)  

---

# 4. Stage 1 — Runtime & Deliberate (CDS)

**Modules:** Runtime Layer + CDS  
**Storage:** Runtime Store + CDS Workspace  

---

## 4.1 What Exists Here

- sessions  
- deliberate instances (Epics)  
- Items (questions, hypotheses, proposals)  
- observations (signals, evidence)  
- review workspaces  
- workflow state  

---

## 4.2 Characteristics

- mutable  
- exploratory  
- non-authoritative  
- simulation-capable  

---

## 4.3 Key Property

> This is where thinking happens.

It may:

- reference existing structure  
- simulate new structure  
- operate without legitimacy  

---

# 5. Stage 2 — Review Layer (Integration Boundary)

**Module:** Runtime (Review Layer)

---

## 5.1 Review Types

### Foreign Integration Review

- CDS → legitimacy  
- foreign → local  
- candidate structure → admitted structure  

### Reconciliation Review

- legitimacy → CDS  
- CDS state ←→ legitimacy state  

---

## 5.2 What Happens

- candidate artifacts are evaluated  
- provisional structure is explored  
- relationships are surfaced and refined  

---

## 5.3 Key Property

> Review prepares structure. It does not create legitimacy.

---

# 6. Stage 3 — Legitimacy Creation

**Module:** Legitimacy Engine  

---

## 6.1 What Happens

- sessions evaluate accepted candidates  
- authority rules are applied  
- decisions are accepted or rejected  

---

## 6.2 Output

- resolutions  
- legitimacy receipts  

---

## 6.3 Key Property

> This is the only place legitimacy is created.

---

# 7. Stage 4 — Artifact Formation (CCS)

**Module:** CCS  

---

## 7.1 What Happens

All outputs become **immutable commit artifacts**:

- resolution commits  
- identity commits  
- signal commits  
- receipt commits  
- deliberate artifacts (optional packaging)  

---

## 7.2 Responsibilities

- assign identity (`commit_id`)  
- enforce structure  
- preserve explicit references (`derives_from`, etc.)  

---

## 7.3 Key Property

> CCS defines structure, not meaning.

---

# 8. Stage 5 — Preservation (Commit Store)

**Module:** Commit Store  

---

## 8.1 What Happens

- commits stored immutably  
- append-only history preserved  

---

## 8.2 Characteristics

- immutable  
- append-only  
- identity-preserving  
- lineage-preserving  

---

## 8.3 Key Property

> This is the durable record of all artifacts.

---

# 9. Stage 6 — Structural Materialization (CSG)

**Module:** Charter Structural Graph (CSG)  

---

## 9.1 What Happens

- commits are transformed into graph structure  
- nodes:
  - resolutions  
  - items (if admitted)  
- edges:
  - supersession  
  - reference  
  - derivation  

---

## 9.2 Projections

- resolution-only  
- item-only  
- mixed  

---

## 9.3 Key Property

> Structure is explicit, projection-aware, and non-interpreting.

---

# 10. Stage 7 — Identity Overlay (CIS, Optional)

**Module:** CIS  

---

## 10.1 What Happens

- identities define scope over structure  
- membership derived via bounded traversal  

---

## 10.2 Key Property

> Identity is declared and bounded, not inferred.

---

# 11. Stage 8 — Observational Signals (CCare + CSP)

**Modules:** CCare (+ optional CSP)

---

## 11.1 What Happens

- signals are recorded:
  - state  
  - confidence  
  - time  

- CSP may:
  - shape  
  - filter  
  - aggregate signals  

---

## 11.2 Key Property

> Signals describe reality. They do not enforce it.

---

# 12. Stage 9 — Alignment Computation (CAS)

**Module:** CAS  

---

## 12.1 What Happens

- computes alignment over:
  - structure (CSG projections)  
  - signals (CCare/CSP)  
  - identity (optional CIS)  

---

## 12.2 Outputs

- semantic states  
- propagation patterns  
- tension and drift  
- predictive dynamics  

---

## 12.3 Key Property

> Alignment is computed, not declared.

---

# 13. Stage 10 — Interpretation (CGL)

**Module:** CGL  

---

## 13.1 What Happens

- transforms outputs into human-readable explanations  

---

## 13.2 Outputs

- summaries  
- explanations  
- narratives  

---

## 13.3 Key Property

> Interpretation adds clarity, not authority.

## 13.4 — Charter Query Language (CQL)

**Module:** Cross-Substrate Query Layer  

---

## 13.4.1 Purpose

CQL provides a **unified query interface across Charter substrates**.

It allows systems and users to:

- query structural data (CSG)  
- query identity-scoped views (CIS)  
- query investigative state (CDS)  
- query alignment state (CAS)  
- compose cross-substrate views  

---

## 13.4.2 Position in Lifecycle

CQL is not a transformation stage.

It operates:

> across preserved, structural, and derived state

It sits between:

- stored / computed artifacts  
and  
- interpretation (CGL) or external consumption  

---

## 13.4.3 Capabilities

CQL supports:

- projection selection (resolution, item, mixed)  
- structural traversal (graph queries)  
- identity scoping (CIS overlays)  
- alignment queries (CAS views)  
- cross-substrate composition  

---

## 13.4.4 Key Property

> CQL reveals structure and state. It does not modify them.

It is:

- read-only  
- deterministic  
- non-authoritative  

---

## 13.4.5 Relationship to CGL

- CQL → structured query results  
- CGL → human-readable interpretation  

CQL answers:

> “What is the state?”

CGL answers:

> “What does it mean?”

---

# 14. Stage 11 — Federation & Transport

**Modules:** Runtime + CRS  

---

## 14.1 Ingress

- discover → query → acquire  
- isolate foreign structure  
- review before integration  

---

## 14.2 Egress

- select artifacts  
- emit via policy  
- transport via CRS  

---

## 14.3 Key Property

> Structure moves. Authority does not.

---

# 15. Archival Lifecycle

---

## 15.1 Process

1. select runtime/CDS artifacts  
2. review and package  
3. convert to commits  
4. store immutably  

---

## 15.2 Outcome

- preserved artifacts  
- runtime state becomes purge-eligible  

---

# 16. Purge Lifecycle

---

## 16.1 Scope

- runtime-only  
- CDS workspace-only  

---

## 16.2 Guarantees

- no loss of preserved artifacts  
- no impact on legitimacy  
- no mutation of commit history  

---

# 17. Invariants Across Lifecycle

- legitimacy is created only by the Legitimacy Engine  
- review never creates legitimacy  
- CCS defines structure, not meaning  
- commit store is immutable and append-only  
- CSG never infers structure  
- CIS never infers identity  
- CAS never infers authority  
- CGL never enforces action  
- federation never implies synchronization  
- archival precedes purge  
- history is never rewritten  

---

# 18. Mental Model

Charter is a layered system:

- CDS → thinking and simulation  
- Review → boundary and preparation  
- Engine → legitimacy creation  
- CCS → artifact formation  
- Store → preservation  
- CSG → structure  
- CIS → identity  
- CCare/CSP → observation  
- CAS → alignment  
- CGL → explanation  
- Federation → exchange  

---

# 19. Final Principle

Charter separates:

- thinking  
- deciding  
- recording  
- structuring  
- observing  
- analyzing  
- explaining  
- exchanging  

So that:

- nothing is conflated  
- nothing is assumed  
- nothing is lost  

> Everything becomes visible, traceable, and reconstructable  
without ever collapsing authority, structure, or meaning.