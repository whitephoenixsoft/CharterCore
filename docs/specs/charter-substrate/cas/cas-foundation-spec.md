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

CAS consumes declared structure and observed signals.

It may distinguish structural relationship classes, including:

- supersession
- reference
- derivation

but it must not interpret derivation as:

- promotion
- demotion
- correctness
- authority

Those meanings remain outside CAS.

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

CAS consumes a structural graph that may include:

- resolution nodes  
- supersession edges  
- reference edges  
- derivation edges  
- area boundary references  

Graph properties:

- directed  
- partially ordered (DAG for supersession)  
- potentially incomplete  
- structurally explicit  

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

Silence is valid.

---

## 4.3 Identity Input (Optional — CIS)

If present:

- identity versions  
- scope definitions  
- identity relationships  
- derivation-aware identity projections  

If absent:

- CAS operates purely structurally  

---

## 4.4 Context Input (Posture)

Context includes:

- transitions  
- experiments  
- identity version changes  
- other declared posture modifiers  

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

This layer is local and numeric only.

---

## 5.2 Semantic State Layer

Maps numeric outputs into semantic states such as:

- aligned  
- misaligned  
- unstable  
- improving  
- reduced capacity  

This layer classifies. It does not interpret intent.

---

## 5.3 Propagation Layer

Propagates across declared structural relationships, including:

- supersession edges  
- reference edges  
- derivation edges  
- area boundaries  
- identity scopes (if present)  

Computes:

- influence  
- propagation paths  
- cones  
- horizons  

### 5.3.1 Relationship Distinction

CAS may distinguish propagation behavior by relationship class.

Examples:

- supersession may affect active-state continuity  
- reference may affect dependency-like influence  
- derivation may affect lineage-aware propagation across scopes  

This distinction is structural only.

CAS must not interpret derivation as authority or hierarchy.

---

## 5.4 Dynamics Layer

Computes system-level behavior:

- tension  
- cascades  
- volatility  
- equilibrium tendencies  
- boundary pressure  
- derivation-related continuity or divergence surfaces  

This layer models alignment as a field over structure and signals.

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
- derivation-aware identity continuity surfaces  

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
- derivation lineage paths  
- cross-scope continuity surfaces  

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
- reinterpret derivation semantics beyond declared structure  

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

## Present
Observed current state

## Predictive
Emerging dynamics

Predictive includes:

- drift velocity  
- cascade likelihood  
- transition risk  
- cross-scope continuity risk  
- derivation-related divergence risk  

Predictive outputs are:

- observational  
- non-binding  

---

# 8. Numeric vs Semantic Separation

CAS maintains:

- numeric layer → computation  
- semantic layer → classification  

Neither replaces the other.

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
Interpretation lens

### Scope (optional)
Subset of graph, identity, lineage, or time

### Filters (optional)
Constraints such as:

- semantic state  
- volatility  
- time window  
- relationship class  

### Context (optional)
Posture modifiers

---

## 9.2 Query Principle

> Queries reveal state. They do not modify it.

---

# 10. View System

---

## 10.1 Canonical Views

### Posture View
- stability  
- volatility  
- contextual condition  

### Trend View
- direction  
- velocity  
- emerging shift  

### Structural View
- cones  
- influence paths  
- lineage-aware paths  

### Tension View
- concentration zones  
- boundary pressure  
- systemic strain  

### Identity View (optional)
- overlap  
- collaboration  
- boundary pressure  
- continuity across derivation paths  

### Lineage View
- derivation-aware continuity  
- recontextualization surfaces  
- cross-scope propagation  

---

## 10.2 View Properties

Views:

- operate on Alignment Store  
- are read-only  
- do not recompute base metrics  
- may expose derivation-aware projections where present  

---

## 10.3 Extensibility

Custom views may:

- combine fields  
- derive projections  
- specialize by structural relationship class  

---

# 11. Identity Interaction Model

---

## 11.1 Overlap

Shared membership.

---

## 11.2 Collaboration

Directional influence.

This may include influence that travels through:

- direct references  
- shared structure  
- derivation-aware identity continuity  

---

## 11.3 Boundary Pressure

Misalignment across scopes.

This may include pressure introduced when derived structure and identity boundaries diverge.

---

## 11.4 Identity Optionality

CAS must degrade cleanly without CIS.

---

# 12. Posture Handling

Posture is query-time only.

It may:

- adjust thresholds  
- change sensitivity  
- modify aggregation posture  

It must not:

- change underlying state  
- rewrite structure  
- alter identity definitions  

---

# 13. Structural Awareness

CAS must handle:

- incomplete graphs  
- missing signals  
- stale inputs  
- disconnected structures  
- cross-scope derivation lineage  

Outputs must remain valid under incomplete information.

---

# 14. Runtime Modes

## 14.1 Full Rebuild
- deterministic recomputation  

## 14.2 Incremental Update
- must match rebuild  

Incremental update may be triggered by:

- new signals  
- structural changes  
- derivation changes  
- identity changes  
- posture changes  

---

# 15. Boundary Rules

CAS may:

- compute  
- aggregate  
- classify  
- expose derivation-aware structural effects  

CAS must never:

- create legitimacy  
- enforce decisions  
- trigger action  
- infer authority  
- reinterpret structural lineage as normative hierarchy  

---

# 16. Design Guarantees

CAS is:

- deterministic  
- observational  
- rebuildable  
- non-authoritative  
- structurally aware  
- query-first  

---

# 17. Mental Model

- CCS → decisions  
- CSG → structure  
- CCare → observation  
- CIS → identity  

CAS computes:

> how these interact over time

including when structure evolves through derivation across scopes.

---

# 18. Final Principle

CAS does not tell systems what to do.

It makes visible:

- alignment  
- drift  
- pressure  
- emerging change  
- continuity and divergence across structural lineage  

without coercion.