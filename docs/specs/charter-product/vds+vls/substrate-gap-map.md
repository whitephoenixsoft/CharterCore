# Charter — Substrate Gap Map

Status: WORKING (ARCHITECTURAL TRACKING)  
Purpose: Identify and track gaps exposed by VDS and VLS integration  
Scope: Substrate-level capabilities required for system coherence  

---

# 1. Overview

This document tracks known gaps in Charter substrates as exposed by:

- Value-Directed Systems (VDS)  
- Value Lineage Systems (VLS)  

These gaps are not failures.

They are pressure signals indicating where substrates must evolve to support:

- system-level interpretation (VDS)  
- structural lineage (VLS)  

---

# 2. Gap Categories

Gaps are categorized as:

- REQUIRED — must exist for system correctness  
- IMPORTANT — strongly improves usability and clarity  
- OPTIONAL — can be deferred or handled at host level  

---

# 3. Gap Map

## 3.1 CCare — Signal Model Expansion

**Needed By:** VDS  
**Priority:** REQUIRED  

### Gaps

- Threshold and bounds model is not defined as a first-class structural concept  
- Signal lifecycle is not defined, including ephemeral versus persisted signals  
- Signal aggregation and grouping behavior is not defined  
- Silence semantics are not defined as an explicit outcome  
- No standardized structure exists for signal payloads such as check-ins or requests  
- No formal linkage exists between signal semantics and triggering conditions  

---

### Direction

- Signals are originated by Value-Directed Systems  
- Signals are passed through Charter Signal Pipeline pipelines for distribution and feeds  
- Signals must remain non-authoritative, non-binding, and descriptive  

- Signal types must be constrained to a defined set, including:
  - alignment  
  - misalignment  
  - reduced capacity  

- Signal payloads must be capable of carrying:
  - related resolutions or items  
  - triggering condition summaries  
  - time window information  
  - source references  
  - references to threshold or relationship definitions  

---

### Principle

Signal semantics define how thresholds are interpreted.

---

## 3.2 Decision to Measurement Bridge

**Needed By:** VDS  
**Priority:** REQUIRED  

### Current Gap

There is no explicit mechanism that connects:

- decisions defined in Charter  
- measurable telemetry originating from external systems  

---

### Direction

#### CDS as Measurement and Investigation Interface

- Value-Directed Systems use the Deliberate System  
- Observations are represented as Deliberate System items using an observation subtype  
- Value-Directed Systems may create observation items when signal conditions indicate investigation is needed  
- Observation items may be investigated, expanded, or marked as settled when no further action is required  
- Deliberate System artifacts may be federated across systems  

---

#### Relationship-Based Binding

Measurement must be defined through explicit structural relationships:

- decision to observable  
- observable to metric source or feed binding  
- observable to threshold or trigger relationship  

Thresholds must:

- exist as explicit structural artifacts  
- be represented as relationships or resolutions  
- be defined in terms of signal semantics rather than only numeric comparison  

---

#### Observable Model

An observable represents:

- a defined aspect of behavior that is being measured  
- a concept independent of any specific telemetry source  
- a structure that may be bound to one or more feeds  

---

### Properties

- implicit mapping is not allowed  
- all measurement definitions must be explicit  
- telemetry is considered secondary and may be discarded  
- decisions, observables, and thresholds are durable structural artifacts  
- measurement does not create legitimacy  

---

## 3.3 CAS — Alignment Computation Role

**Needed By:** VDS, VLS  
**Priority:** IMPORTANT  

### Gaps

- The role of CAS execution is not clearly defined as local, remote, or hybrid  
- The authority level of CAS outputs is not clearly defined  
- The federation model for CAS outputs is not defined  
- There is no defined durable artifact model for CAS summary outputs that are shared across systems  

---

### Direction

- CAS outputs must remain reproducible, explainable, and non-authoritative  
- CAS computation is derived and may be ephemeral when used locally  

---

### Federated CAS Output Model

When CAS outputs are shared across systems, preserved for comparison, or used for structural visibility, they must be emitted as non-authoritative commit artifacts.

---

### CAS Commit Properties

CAS-derived commit artifacts must:

- be explicitly identified as derived artifacts  
- include provenance  
- include rule identity  
- include scope and context  
- include references to input data where possible  
- include timestamps  

- be immutable, auditable, and reproducible relative to inputs  

---

### Constraints

CAS-derived commits must not:

- create legitimacy  
- modify identity structures  
- override local structural truth  
- be treated as authoritative state  

---

## 3.4 CIS — Identity Boundary and Version Standardization

**Needed By:** VLS  
**Priority:** REQUIRED  

### Current State

- Identity is defined as a bounded region of nodes within the Charter Structure Graph  
- Identity is determined by explicitly defining the boundary of connected nodes  
- Connected nodes may extend beyond the identity, but boundary definition determines inclusion  
- Each identity version is already represented as a commit  
- A version field exists but is not standardized  

---

### Gaps

- The structure and encoding of identity boundaries are not formally standardized  
- There is no explicit standard for how boundary limits are defined when nodes connect to external structures  
- Node roles within an identity are not formally defined, including:
  - nodes that define identity  
  - nodes included within the boundary  
  - nodes that are connected but excluded  

- The version field is not standardized or derived through a deterministic process  
- Structural transitions such as split, merge, promotion, and demotion are not formally encoded at the identity boundary level  

---

### Direction

- Identity must remain a structural definition based on a bounded region within the Charter Structure Graph  
- Identity must remain explicit, bounded, and versioned through commits  

- CIS must define:
  - a deterministic representation of identity boundaries  
  - rules for inclusion and exclusion of connected nodes  
  - standardized roles for nodes within an identity boundary  
  - deterministic version derivation rules  

---

### Constraint

CIS must not define lifecycle meaning, deployment timing, or behavioral interpretation.

---

## 3.5 CQL — Canonical Query Interface and JSON Intermediate Language

**Needed By:** All systems  
**Priority:** REQUIRED  

### Current Gap

- There is no formally defined canonical query contract for machine interaction  
- There is no standard JSON representation for queries across all domains  
- The existing DSL representation is human-oriented and not suitable for system-level interaction  

---

### Direction

- The Charter Query Language must define a canonical JSON Intermediate Language for all queries  

- All systems, including:
  - Runtime  
  - Value-Directed Systems  
  - Value Lineage Systems  
  - CLI  
  must interact with CQL through the JSON Intermediate Language  

- The DSL is a human-facing syntax layer only and must compile deterministically into the JSON Intermediate Language  

---

### Query Scope Requirements

The JSON Intermediate Language must support querying:

- live runtime state  
- persisted commit artifacts  
- derived Value-Directed System signals  
- Deliberate System items, including observation subtype  
- Value Lineage System lineage structures  
- CAS-derived commit artifacts  
- federated data  

---

### Properties

- all queries must be deterministic  
- domain boundaries must be explicit  
- no implicit interpretation is allowed  
- JSON Intermediate Language is the authoritative query contract  

---

## 3.6 Federation Model — Commit Transport, Push and Fetch Semantics

**Needed By:** Charter, VDS, VLS, CAS, CRS, CSP  
**Priority:** IMPORTANT  

### Current State

- Durable data exchange across systems is commit-based  
- All durable cross-system flow moves through Charter Commit System commit artifacts  
- Charter Relay System transports commits and does not interpret them  
- Hosts experience federation as push and fetch operations over an Area commit store  
- Charter Signal Pipeline may shape outbound monitoring-related flow before relay when configured  
- Local runtime stores remain local and mutable, but durable closures emit Charter Commit System commits  

---

### Remaining Gaps

- Commit taxonomy is not fully standardized across all durable artifact types, including:
  - legitimacy outputs  
  - reconciliation closures  
  - deliberate closures  
  - signal artifacts  
  - CAS summary artifacts  
  - CIS identity version artifacts  

- Push policy is not fully defined, including:
  - which commit classes are eligible for outbound transport by default  
  - which commit classes require explicit host or pipeline configuration  
  - whether push operates per Area, per commit class, or per filtered selection  

- Fetch policy is not fully defined, including:
  - default fetch scope  
  - filtering by commit type, area, source, or provenance  
  - interaction between fetched commits and feeds  

- Charter Relay System transport semantics are not fully defined, including:
  - retention  
  - indexing  
  - deduplication  
  - replay behavior  
  - routing metadata  

- Charter Signal Pipeline shaping semantics for outbound commit flow are not fully defined, including:
  - aggregation  
  - summarization  
  - cadence control  
  - threshold-gated transport behavior  

---

### Direction

Federation must preserve:

- autonomy  
- provenance  
- non-coercion  
- explicitness of commit type and origin  

---

### Clarification

- Charter propagates structural changes through reconciliation and commit transport  
- Value-Directed Systems originate signals, and Charter Signal Pipeline may shape their outbound commit flow before relay  
- Value Lineage Systems propagate identity and lineage structures as commit-backed durable artifacts  
- CAS propagates derived summary commits  
- Charter Relay System transports commits only and does not interpret legitimacy, identity, or alignment meaning  

Each system maintains distinct federation semantics, but all durable exchange uses Charter Commit System commit artifacts.

---

## 3.7 Deployment and Transition Model

**Needed By:** VDS, VLS  
**Priority:** IMPORTANT  

### Current State

- Systems may undergo periods where behavior is intentionally expected to deviate from normal conditions  
- These periods may originate externally to the affected teams or identities  
- Value-Directed Systems already support a signal type representing intentional pause  
- There is currently no standardized mechanism to declare external conditions that should influence this signal posture across systems  

---

### Gaps

- There is no standardized commit model for declaring external disturbances that affect system behavior across a defined scope  
- There is no explicit distinction between:
  - internally generated intentional pause conditions  
  - externally declared disturbance conditions  

- There is no defined mechanism for:
  - scoping disturbances to specific Areas, identities, or structural regions  
  - propagating disturbance effects across abstraction tiers within the Charter Structure Graph  

- There is no standardized definition of time windows associated with disturbances, including:
  - execution window during which disruption occurs  
  - observation window during which heightened monitoring or altered interpretation is expected  

- There is no defined integration between disturbance declarations and Value-Directed System signal interpretation behavior  

---

### Direction

- External disturbances must be declared as durable commit artifacts through the Charter Commit System  

- These commits represent externally originated conditions that may impact system behavior and productivity  

- Value-Directed Systems must interpret these commits as external sources of intentional pause conditions  

- Internal intentional pause conditions remain distinct and originate from within the monitored system or team  

---

### Structural Scope and Propagation

- Disturbance commits must explicitly define structural scope, which may include:
  - Areas  
  - identities  
  - specific nodes or regions within the Charter Structure Graph  

- Disturbance effects must propagate through structural relationships across abstraction tiers  

- Higher-level disturbance declarations may affect all downstream resolutions, identities, or teams within the defined boundary  

- Propagation must follow explicit structural containment rules and must not rely on implicit inference  

---

### Time Window Model

Disturbance commits must define time-based context, including:

- execution window during which the disturbance is active  
- observation window during which systems should apply heightened monitoring or adjusted interpretation  

These windows are defined by the host and are non-legitimizing  

---

### VDS Integration

- Value-Directed Systems must:
  - interpret disturbance commits as external context  
  - map disturbance context to intentional pause signal behavior  
  - adjust signal interpretation, aggregation, and threshold behavior accordingly  

- CAS may continue to measure intentional pause signals without modification to its core model  

---

### VLS Integration

- Value Lineage Systems may use disturbance commits to:
  - contextualize identity behavior over time  
  - explain anomalies in alignment or performance  
  - correlate structural changes with periods of disruption  

---

### Constraints

Disturbance commits must:

- not create legitimacy  
- not modify structural relationships  
- not override identity definitions  
- not enforce behavior directly  

They provide contextual input for interpretation only  

---

### Principle

External disturbance is declared explicitly.

Interpretation of that disturbance is performed by Value-Directed Systems through existing signal semantics.

---

## 3.8 Persistence and Store Model Clarity

**Needed By:** Runtime, CLI, All Substrates, VDS, VLS  
**Priority:** IMPORTANT  

### Current State

- Charter defines multiple logical storage surfaces across substrates, including:
  - commit store (Charter Commit System artifacts)  
  - reference store (active pointers and mutable references)  
  - metadata store  
  - audit store  
  - review and deliberate workspace stores  
  - identity and derived data stores  

- These stores are conceptually defined but often assumed to be implemented using object storage patterns  

- Runtime may operate entirely in memory or with persistence enabled  

- Durable artifacts are emitted through the Charter Commit System regardless of storage implementation  

---

### Gaps

- There is no explicit separation between:
  - logical substrate-defined stores  
  - physical storage implementations  

- There is no standardized persistence interface that allows hosts to provide alternative storage backends  

- There is no formal adapter model for integrating:
  - relational databases  
  - document stores  
  - key-value systems  
  - graph databases  
  - or hybrid storage systems  

- There is no defined mechanism for hosts to:
  - centrally manage or pool data across multiple substrates  
  - override default storage implementations  
  - unify access to runtime, commit, and derived data  

- The role of default object stores and commit stores is not clearly defined as reference implementations rather than required architecture  

- There is no explicit contract defining how persistence must preserve:
  - determinism  
  - provenance  
  - rule identity  
  - commit integrity  

---

### Direction

- All substrate storage must be defined in terms of **logical storage contracts**, not physical storage implementations  

- Physical storage must be **pluggable at the host boundary** through persistence adapters  

- Hosts must be able to supply storage implementations for all logical stores, including:
  - commit store  
  - reference store  
  - metadata store  
  - audit store  
  - workspace stores  
  - identity and derived data stores  

- Default object store and commit store implementations must be treated as **reference implementations only**, not required components  

---

### Persistence Model

- Runtime may operate:
  - fully in memory  
  - with partial persistence  
  - with full persistence  

- When persistence is enabled:
  - all state changes must be written to defined logical stores  
  - durable artifacts must be emitted through the Charter Commit System  

- When persistence is not enabled:
  - runtime behavior must still remain deterministic within the execution context  

---

### CCS and Storage Independence

- The Charter Commit System defines the **canonical protocol and structure for durable artifacts**  

- Commit artifacts must remain:
  - transportable  
  - verifiable  
  - immutable  

regardless of how they are physically stored  

- The commit store is a logical construct that may be implemented using any storage backend  

---

### Host Data Control and Pooling

- Hosts must be able to:
  - aggregate data across multiple substrate domains  
  - maintain centralized or distributed storage models  
  - expose unified access patterns across:
    - runtime state  
    - commit artifacts  
    - derived data  
    - external telemetry  

- Hosts may choose to:
  - bypass default storage implementations entirely  
  - integrate Charter data into existing data platforms  

---

### VDS and VLS Considerations

- Value-Directed Systems and Value Lineage Systems may require additional storage for:
  - telemetry data  
  - signal histories  
  - derived summaries  
  - lineage materializations  
  - replay datasets  

- These storage needs are external to core Charter substrate persistence and must be host-defined  

- Substrates must not impose storage requirements on these systems beyond commit-based integration  

---

### Constraints

Persistence implementations must:

- preserve determinism of query and execution  
- preserve provenance and rule identity  
- preserve commit integrity and immutability  
- not introduce implicit interpretation or mutation  
- maintain explicit boundaries between logical store domains  

---

### Principle

Persistence is a host-controlled concern.

Substrates define what must be stored.

Hosts define how and where it is stored.

---

## 3.9 Simulation and Replay Support

**Needed By:** VDS, VLS, software team workflows  
**Priority:** IMPORTANT  

### Current State

- Simulation is conceptually supported through:
  - Deliberate System items  
  - Charter Structure Graph projections  
  - relationship-driven structure  

- Value-Directed Systems produce observation items representing meaningful conditions that require investigation  

- Software systems may maintain event histories or telemetry streams external to Charter  

- There is no formalized mechanism for replaying observation conditions against simulated structures  

---

### Gaps

- There is no standardized model for applying observation items to simulated Charter Structure Graph configurations  

- There is no defined mechanism for:
  - replaying historical observation conditions  
  - evaluating their structural impact across a simulated hierarchy  
  - capturing resulting alignment effects through CAS  

- There is no formal structure defining replay-capable observation items, including:
  - condition representation  
  - structural target  
  - threshold or relationship linkage  
  - time window or persistence characteristics  

- There is no defined process for combining:
  - observation items from Value-Directed Systems  
  - event history from software systems  

into a unified investigation and simulation workflow  

- There is no standardized method for comparing:
  - current system behavior  
  - simulated structure behavior  
  - alternative threshold configurations  

---

### Direction

Simulation and replay must operate as **structural and semantic replay**, not raw telemetry replay.

---

### Semantic Replay Model

Replay operates by:

- taking observation items representing meaningful system conditions  
- applying them to a simulated structure derived from Deliberate System items and Charter Structure Graph projections  
- using threshold relationships and structural bindings to determine how signals would be emitted  
- allowing CAS to calculate resulting alignment effects and cascade behavior across the structure  

Replay is therefore:

- not a reconstruction of all raw telemetry  
- but a reapplication of meaningful, curated conditions  

---

### Observation-Driven Replay

Observation items serve as the primary replay unit.

Replay-capable observation items must include:

- the condition being represented  
- the structural target or scope  
- linkage to threshold or relationship definitions  
- signal semantic classification  
- time window or persistence characteristics where applicable  

These items represent:

- conditions that were significant enough to be surfaced  
- missing or problematic scenarios in the system  
- opportunities for structural or behavioral improvement  

---

### Integration with Event History

- Software systems may provide event history or telemetry streams  

- Event history may be used to:
  - enrich observation items  
  - validate observed conditions  
  - provide additional context for investigation  

- Full telemetry replay is not required for the substrate model  

- Observation-driven replay remains the primary mechanism for simulation  

---

### Simulation Execution Model

Simulation is executed by:

1. constructing a simulated structure using Deliberate System items  
2. projecting that structure into the Charter Structure Graph  
3. applying observation items to the simulated structure  
4. resolving threshold and relationship bindings  
5. emitting simulated signals locally within the deliberate context  
6. invoking CAS to calculate alignment and cascade effects across the graph  

---

### CAS Role

- CAS calculates how applied observation conditions affect alignment across the structure  

- CAS must support:
  - propagation of effects across structural relationships  
  - evaluation of cascade behavior  
  - comparison between different simulation configurations  

- CAS outputs remain derived and non-authoritative  

---

### Software Team Workflow

Simulation supports real-world problem solving by enabling teams to:

- identify missing or unhandled scenarios through observation items  
- determine whether issues originate from:
  - software behavior  
  - threshold definitions  
  - structural relationships  

- adjust thresholds to reflect realistic system behavior  
- modify software to eliminate undesirable conditions  
- evaluate how changes would affect alignment across the system  

- use disturbance windows when deploying changes that intentionally alter behavior  

---

### Comparative Simulation

Systems must support comparison between:

- current structure and behavior  
- simulated structure configurations  
- alternative threshold definitions  

Comparison must allow teams to evaluate:

- alignment impact  
- signal frequency and distribution  
- cascade effects across dependent structures  

---

### Constraints

Simulation and replay must:

- remain non-legitimizing  
- not modify committed structure or identity  
- operate within isolated deliberate contexts  
- preserve explicit structure and relationships  
- avoid implicit inference or interpretation  

---

### Principle

Replay is the reapplication of meaningful observation conditions to simulated structure in order to understand structural consequences and improve system behavior.

---
# 4. Non-Gaps (Confirmed Stable)

The following areas are considered stable:

- Runtime orchestration model  
- Reconciliation workflow  
- Session and legitimacy boundaries  
- Commit and Charter Commit System model  
- Non-interpretation principle  
- Rule identity model  

---

# 5. Strategy

Gaps should not be solved in isolation.

Instead, Value-Directed Systems and Value Lineage Systems should drive substrate evolution through real usage pressure.

---

# 6. Final Principle

Substrate gaps are not failures.

They are signals that the system is being used correctly.