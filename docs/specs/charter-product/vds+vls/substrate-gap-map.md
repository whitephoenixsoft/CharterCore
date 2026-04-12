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

### Gaps

- There is no shared model for transition phases, rollout states, or identity evolution timing  

---

### Direction

- Value Lineage Systems define identity lifecycle, version transitions, and sunset behavior  
- Value-Directed Systems observe behavior during transitions and emit signals based on observed impact  

---

## 3.8 Persistence and Store Model Clarity

**Needed By:** Runtime, CLI, All Substrates  
**Priority:** IMPORTANT  

### Gaps

- The distinction between authoritative stores, derived stores, and reference stores is not fully formalized  

---

### Direction

Each storage surface must explicitly define:

- ownership  
- mutability  
- authority level  

---

## 3.9 Simulation and Replay Support

**Needed By:** VDS, VLS, software team workflows  
**Priority:** IMPORTANT  

### Gaps

- There is no mechanism to replay telemetry or feed data against simulated structures in the Deliberate System  
- There is no standardized way to project historical signal data onto simulated Charter Structure Graph configurations  
- There is no comparison model between live structures and simulated structures  

---

### Direction

- Simulation must allow structural experimentation using Deliberate System items and Charter Structure Graph projections  
- Replay capability must allow historical or sampled feed data to be evaluated against simulated structures  
- Replay must remain non-legitimizing  
- Systems must support comparison between:
  - current structure  
  - simulated structure  
  - resulting alignment and signal outcomes  

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