# Charter Alignment System (CAS) — Foundation Specification (Revised v4)

Status: FOUNDATIONAL  
Applies to: Charter Alignment System (CAS), Alignment Engine, Alignment State Store, Query Layer  
Depends On: Charter Commit System (CCS), Commit Store, Charter Care Substrate (CCare), Charter Structural Graph (CSG), optional Charter Identity Substrate (CIS), optional Charter Signal Processing Substrate (CSP)  
Does NOT define: legitimacy, workflow orchestration, storage backends, UI rendering  

---

# 1. Purpose

The Charter Alignment System (CAS) computes and exposes the relationship between:

- declared or admitted structural artifacts  
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

> CAS computes alignment over declared structure and observed signals.  
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

CAS may analyze mixed legitimate and investigative structure, but must never collapse their distinction.

---

# 3. Position in Architecture

CAS operates strictly above:

- CCS + Commit Store (artifact truth)  
- CCare (observations)  
- CSG (structure)  
- CIS (optional identity layer)  
- CSP (optional upstream shaping of signal flow)  

CAS produces:

- derived alignment state  
- queryable views  
- descriptive outputs  

CAS does not:

- modify upstream artifacts  
- participate in legitimacy  
- alter identity  
- redefine graph structure  

---

# 4. Inputs

## 4.1 Structural Input (CSG)

CAS consumes structural projections from CSG.

CSG may include:

- resolution nodes  
- item nodes  
- supersession edges  
- reference edges  
- derivation edges  
- cross-class edges  
- area boundary references  

Graph properties:

- directed  
- structurally explicit  
- potentially incomplete  
- projection-aware  

CAS must preserve node-class distinctions supplied by CSG.

Item-based analysis applies only to **admitted stable item nodes** present in CSG, not mutable CDS workspace state.

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
- identity projections over CSG  

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

## 4.5 Signal Shaping Input (Optional — CSP)

If CSP is present, CAS may consume CCare artifacts that were:

- directly emitted  
- clustered before durable emission  
- aggregated under explicit pipeline policy  

CAS does not consume transient CSP feeds as authoritative inputs unless explicitly materialized into durable artifacts through accepted runtime policy.

CSP affects signal cadence and shape, not CAS semantics.

---

# 5. Structural Scope Model

CAS operates over **explicit structural projections**.

These may include:

- resolution-only projection  
- item-only projection  
- mixed projection  
- lineage projection  

Projection choice affects what structure CAS analyzes, but does not alter CAS semantics.

---

## 5.1 Projection Principle

> CAS analyzes the selected projection of declared structure. It does not redefine the graph.

---

## 5.2 Node-Class Principle

CAS must preserve distinction between:

- legitimate structural nodes (e.g., resolutions)  
- investigative or exploratory structural nodes (e.g., items)  

Mixed analysis is allowed, but node-class distinctions must remain visible in outputs and views.

Cross-class edges may participate in analysis, but must not imply legitimacy transfer or authority equivalence.

---

# 6. Internal Computation Model

CAS operates in deterministic layers.

---

## 6.1 Local Metrics Layer

Per-node numeric computation may include:

- drift  
- variance  
- signal density  
- recency  

This layer is local and numeric only.

Node class does not change the mathematical role of the layer, but may affect projection selection and interpretation.

---

## 6.2 Semantic State Layer

Maps numeric outputs into semantic states such as:

- aligned  
- misaligned  
- unstable  
- improving  
- reduced capacity  

This layer classifies. It does not interpret intent or legitimacy.

---

## 6.3 Propagation Layer

Propagates across declared structural relationships, including:

- supersession edges  
- reference edges  
- derivation edges  
- cross-class edges  
- area boundaries  
- identity scopes (if present)  

Computes:

- influence  
- propagation paths  
- cones  
- horizons  

### 6.3.1 Relationship Distinction

CAS may distinguish propagation behavior by relationship class.

Examples:

- supersession may affect active-state continuity for resolution nodes only  
- reference may affect dependency-like influence  
- derivation may affect lineage-aware propagation across scopes  
- cross-class edges may affect simulation continuity across legitimate and investigative structure  

This distinction is structural only.

CAS must not interpret derivation as authority or hierarchy.

### 6.3.2 Mixed-Projection Propagation

When item and resolution nodes coexist in the selected projection:

- propagation may traverse both node classes  
- node class must remain visible in results  
- mixed traversal must not imply equivalence of legitimacy  

### 6.3.3 Supersession Constraint

Supersession applies only to resolution nodes.

Item nodes must not participate in supersession semantics.

---

## 6.4 Dynamics Layer

Computes system-level behavior:

- tension  
- cascades  
- volatility  
- equilibrium tendencies  
- boundary pressure  
- derivation-related continuity or divergence surfaces  

This layer models alignment as a field over structure and signals.

Mixed-projection and lineage-projection analysis may be used for:

- simulation  
- exploratory investigation  
- pre-legitimacy structural reasoning  
- reconciliation-aware analysis  

Such outputs remain non-authoritative.

---

## 6.5 Materialization Layer

Produces derived, queryable state.

---

# 7. Alignment State Store (CAS Store)

CAS may maintain a **materialized alignment store**.

---

## 7.1 Purpose

The Alignment State Store exists to:

- accelerate queries  
- persist derived alignment state  
- avoid recomputation  
- support multiple query views  
- preserve projection-aware results  

---

## 7.2 Properties

The Alignment State Store is:

- **derived from CCS + CCare + CSG (+ CIS, optional CSP-shaped CCare inputs)**  
- **fully rebuildable**  
- **non-authoritative**  
- **local to the system**  

---

## 7.3 Contents

### Node-Level

Node-level state may exist for:

- resolution nodes  
- item nodes  

Fields may include:

- drift  
- variance  
- signal density  
- semantic state  
- confidence  
- node class  

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
- projection-aware neighborhoods  
- lineage-aware projections  

---

## 7.4 Rebuild Principle

> The Alignment Store must be fully reconstructable.

- loss is non-fatal  
- rebuild must be deterministic  
- incremental updates must match full rebuild  

---

## 7.5 No Interpretation Rule

The Alignment Store must not:

- infer intent  
- create authority  
- enforce decisions  
- reinterpret identity  
- reinterpret derivation semantics beyond declared structure  
- collapse node-class distinctions  

---

## 7.6 Separation from Other Stores

The Alignment Store is not:

- Commit Store  
- Graph Store (CSG)  
- Identity Store (CIS)  
- Runtime Persistence  
- CDS Workspace  

---

# 8. Present vs Predictive Outputs

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
- projection-aware  

---

# 9. Numeric vs Semantic Separation

CAS maintains:

- numeric layer → computation  
- semantic layer → classification  

Neither replaces the other.

---

# 10. Query Model

CAS is query-first.

---

## 10.1 Query Components

### Target
- resolution  
- item  
- area  
- identity  
- global  

### Projection
Defines structural slice, such as:

- resolution-only  
- item-only  
- mixed  
- lineage  

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
- node class  

### Context (optional)
Posture modifiers

---

## 10.2 Query Principle

> Queries reveal state. They do not modify it.

Projection and target selection do not redefine the graph. They select analyzable structure.

---

# 11. View System

---

## 11.1 Canonical Views

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
- projection-aware paths  

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

### Simulation View
- exploratory propagation  
- mixed-node tension  
- item-to-resolution continuity  
- delta-sensitive structural effects  

Simulation outputs remain non-legitimate and non-authoritative.

---

## 11.2 View Properties

Views:

- operate on Alignment Store  
- are read-only  
- do not recompute base metrics  
- may expose derivation-aware projections where present  
- must preserve node-class distinctions where relevant  

---

## 11.3 Extensibility

Custom views may:

- combine fields  
- derive projections  
- specialize by structural relationship class  
- specialize by node class or projection type  

---

# 12. Identity Interaction Model

---

## 12.1 Overlap

Shared membership.

---

## 12.2 Collaboration

Directional influence.

This may include influence that travels through:

- direct references  
- shared structure  
- derivation-aware identity continuity  

---

## 12.3 Boundary Pressure

Misalignment across scopes.

This may include pressure introduced when derived structure and identity boundaries diverge.

---

## 12.4 Identity Optionality

CAS must degrade cleanly without CIS.

If CIS is present, identity scopes may be applied over whichever structural projection is explicitly selected.

---

# 13. Posture Handling

Posture is query-time only.

It may:

- adjust thresholds  
- change sensitivity  
- modify aggregation posture  

It must not:

- change underlying state  
- rewrite structure  
- alter identity definitions  
- collapse projection boundaries  

---

# 14. Structural Awareness

CAS must handle:

- incomplete graphs  
- missing signals  
- stale inputs  
- disconnected structures  
- cross-scope derivation lineage  
- mixed node-class graphs  
- cross-class structural continuity  

Outputs must remain valid under incomplete information.

---

# 15. Runtime Modes

## 15.1 Full Rebuild
- deterministic recomputation  

## 15.2 Incremental Update
- must match rebuild  

Incremental update may be triggered by:

- new signals  
- structural changes  
- derivation changes  
- identity changes  
- posture changes  
- admitted item or resolution graph changes  

---

# 16. Boundary Rules

CAS may:

- compute  
- aggregate  
- classify  
- expose derivation-aware structural effects  
- analyze mixed legitimate and investigative projections  

CAS must never:

- create legitimacy  
- enforce decisions  
- trigger action  
- infer authority  
- reinterpret structural lineage as normative hierarchy  
- imply that exploratory nodes are equivalent to legitimate nodes  
- treat cross-class linkage as legitimacy transfer  

---

# 17. Design Guarantees

CAS is:

- deterministic  
- observational  
- rebuildable  
- non-authoritative  
- structurally aware  
- projection-aware  
- query-first  

---

# 18. Mental Model

- CCS → durable artifacts  
- CSG → explicit structure  
- CCare → observation  
- CIS → identity  
- CSP → optional shaping of signal flow  

CAS computes:

> how these interact over time

across selected structural projections, including when structure mixes legitimate and investigative nodes.

---

# 19. Final Principle

CAS does not tell systems what to do.

It makes visible:

- alignment  
- drift  
- pressure  
- emerging change  
- continuity and divergence across structural lineage  
- exploratory and legitimate dynamics across shared structure  

without coercion.