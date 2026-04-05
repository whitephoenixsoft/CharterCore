# Charter Alignment System (CAS) — Foundation Specification (Revised)

Status: FOUNDATIONAL  
Applies to: Charter Alignment System (CAS), Alignment Engine, Alignment State Store, Query Layer  
Depends On: Charter Commit System (CCS), Commit Store, Charter Care Substrate (CCare), Charter Structural Graph (CSG), optional Charter Identity Substrate (CIS)  
Does NOT define: legitimacy, workflow orchestration, storage backends, UI rendering  

---

# 1. Purpose

The Charter Alignment System (CAS) computes and exposes the relationship between:

- declared decisions (resolutions)
- observed conditions (signals)
- structural relationships (graph)
- optional identity constructs

CAS exists to make **alignment dynamics legible** without:

- creating legitimacy  
- enforcing action  
- inferring authority  

CAS is:

- deterministic  
- observational  
- rebuildable  
- query-driven  

---

# 2. Core Principle

> CAS computes alignment.  
> It does not decide, enforce, or interpret intent beyond structure and signals.

---

# 3. Position in Architecture

CAS operates strictly above:

- CCS + Commit Store (artifact truth)
- CCare (observations)
- CSG (structure)
- CIS (optional identity layer)

CAS produces:

- derived alignment state  
- queryable views  
- descriptive outputs  

CAS does not:

- modify upstream artifacts  
- participate in legitimacy  
- alter identity  

---

# 4. Inputs

## 4.1 Structural Input (CSG)

- resolution nodes  
- supersession edges  
- reference edges  
- area boundary references  

Graph is:

- directed  
- partially ordered (DAG for supersession)  
- potentially incomplete  

---

## 4.2 Observational Input (CCare)

Signals include:

- semantic state  
- confidence  
- timestamp  

Signals are:

- descriptive  
- non-authoritative  
- optional  

---

## 4.3 Identity Input (Optional — CIS)

If present:

- identity versions  
- scope definitions  
- identity relationships  

If absent:

- CAS operates purely structurally  

---

## 4.4 Context Input (Posture)

Context includes:

- transitions  
- experiments  
- identity version changes  

Context modifies interpretation, not truth.

---

# 5. Internal Computation Model

CAS operates in deterministic layers.

---

## 5.1 Local Metrics Layer

Per-resolution numeric computation:

- drift  
- variance  
- signal density  
- recency  

---

## 5.2 Semantic State Layer

Maps numeric → semantic states:

- aligned  
- misaligned  
- unstable  
- improving  
- reduced capacity  

---

## 5.3 Propagation Layer

Propagates across:

- supersession edges  
- reference edges  
- area boundaries  
- identity scopes (if present)  

Computes:

- influence  
- propagation paths  
- cones and horizons  

---

## 5.4 Dynamics Layer

Computes system-level behavior:

- tension  
- cascades  
- volatility  
- equilibrium tendencies  

---

## 5.5 Materialization Layer

Produces derived, queryable state.

---

# 6. Alignment State Store (CAS Store)

CAS may maintain a **materialized alignment store**.

---

## 6.1 Purpose

The Alignment State Store exists to:

- accelerate queries  
- persist derived alignment state  
- avoid recomputation  
- support multiple query views  

---

## 6.2 Properties

The Alignment State Store is:

- **derived from CCS + CCare + CSG (+ CIS)**  
- **fully rebuildable**  
- **non-authoritative**  
- **local to the system**  

---

## 6.3 Contents

### Resolution-Level

- drift  
- variance  
- signal density  
- semantic state  
- confidence  

---

### Area-Level

- aggregated drift  
- variance  
- capacity pressure  
- tension indicators  

---

### Identity-Level (if present)

- aggregated drift  
- boundary pressure  
- overlap tension  
- transition volatility  

---

### Global-Level

- system drift  
- variance  
- active tension regions  

---

### Structural Derivatives

- cones  
- propagation paths  
- influence maps  
- cascade indicators  

---

## 6.4 Rebuild Principle

> The Alignment Store must be fully reconstructable.

- loss is non-fatal  
- rebuild must be deterministic  
- incremental updates must match full rebuild  

---

## 6.5 No Interpretation Rule

The Alignment Store must not:

- infer intent  
- create authority  
- enforce decisions  
- reinterpret identity  

---

## 6.6 Separation from Other Stores

The Alignment Store is not:

- Commit Store  
- Graph Store (CSG)  
- Identity Store (CIS)  
- Runtime Persistence  

---

# 7. Present vs Predictive Outputs

CAS separates:

### Present
Observed current state

### Predictive
Emerging dynamics

Predictive includes:

- drift velocity  
- cascade likelihood  
- transition risk  

---

# 8. Numeric vs Semantic Separation

CAS maintains:

- numeric layer → computation  
- semantic layer → classification  

---

# 9. Query Model

CAS is query-first.

---

## 9.1 Query Components

### Target
- resolution  
- area  
- identity  
- global  

### View
interpretation lens  

### Scope (optional)

### Filters (optional)

### Context (optional)

---

## 9.2 Query Principle

> Queries reveal state. They do not modify it.

---

# 10. View System

---

## 10.1 Canonical Views

### Posture View
- stability / volatility  

### Trend View
- direction / velocity  

### Structural View
- cones  
- influence paths  

### Tension View
- concentration zones  

### Identity View (optional)
- overlap  
- collaboration  
- boundary pressure  

---

## 10.2 View Properties

Views:

- operate on Alignment Store  
- are read-only  
- do not recompute base metrics  

---

## 10.3 Extensibility

Custom views may:

- combine fields  
- derive projections  

---

# 11. Identity Interaction Model

---

## 11.1 Overlap
Shared membership

## 11.2 Collaboration
Directional influence

## 11.3 Boundary Pressure
Misalignment across scopes

---

## 11.4 Identity Optionality

CAS must degrade cleanly without CIS.

---

# 12. Posture Handling

Posture is query-time only.

It may:

- adjust thresholds  
- change sensitivity  

It must not:

- change underlying state  

---

# 13. Structural Awareness

CAS must handle:

- incomplete graphs  
- missing signals  
- stale inputs  

Outputs must remain valid.

---

# 14. Runtime Modes

## Full Rebuild
- deterministic recomputation  

## Incremental Update
- must match rebuild  

---

# 15. Boundary Rules

CAS may:

- compute  
- aggregate  
- classify  

CAS must never:

- create legitimacy  
- enforce decisions  
- trigger action  

---

# 16. Design Guarantees

CAS is:

- deterministic  
- observational  
- rebuildable  
- non-authoritative  

---

# 17. Mental Model

- CCS → decisions  
- CSG → structure  
- CCare → observation  
- CIS → identity  

CAS computes:

> how these interact over time

---

# 18. Final Principle

CAS does not tell systems what to do.

It makes visible:

- alignment  
- drift  
- pressure  
- emerging change  

without coercion.