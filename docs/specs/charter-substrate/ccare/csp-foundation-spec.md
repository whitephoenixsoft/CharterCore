# Charter Signal Processing Substrate (CSP) — Foundation Specification

Status: FOUNDATIONAL  
Intent: Define high-throughput signal shaping, aggregation, and emission prior to durable artifact creation  
Scope: Signal candidate processing, pipeline definitions, clustering, aggregation, feeds, and emission policies  
Depends On: Runtime Layer, Charter Commit System (CCS) (optional durability), optional Charter Care Substrate (CCare)  
Interacts With: CRS (transport), CDS (subscriptions), CAS (indirect via CCare)  
Does NOT Define: legitimacy, alignment computation, identity, graph structure (CSG), or guidance (CGL)  

---

# 1. Purpose

The Charter Signal Processing Substrate (CSP) defines how **high-volume signal-like inputs are shaped, aggregated, and surfaced** before becoming durable artifacts.

It exists to:

- process high-frequency or noisy inputs safely  
- reduce signal noise through clustering and aggregation  
- provide configurable emission and cadence control  
- support both human-centric and automated systems  
- enable optional transformation into CCS-backed artifacts  
- act as a bridge between raw observation and structured care  

CSP is a **non-authoritative signal shaping substrate**.

---

# 2. Core Principle

> CSP shapes signals. It does not create meaning, authority, or legitimacy.

CSP:

- operates on normalized signal candidates  
- aggregates without interpreting semantics  
- preserves opposing observations independently  
- emits outputs without implying obligation  
- defers meaning to downstream systems (CCare, CAS, CGL)  

---

# 3. Architectural Position

CSP is a **Runtime-hosted substrate**.

It operates:

- before CCS durability (optional)  
- alongside CCare (but not within it)  
- independently of CAS and CSG  

CSP may be positioned:

- on **ingress** (before local adoption of external signals)  
- on **egress** (before emitting signals to CCS/CRS)  
- internally (for local-only processing and feeds)

---

# 4. Signal Candidate Model

## 4.1 Definition

CSP operates on **signal candidates**, which are normalized, minimal signal inputs.

They may originate from:

- human input  
- CCare-compatible sources  
- surveys  
- telemetry systems  
- external pipelines  

---

## 4.2 Required Shape (Conceptual)

A signal candidate must include:

- target (resolution, area, identity, or global)  
- signal_type  
- state  
- timestamp  

Optional:

- numeric confidence (e.g., 0.5 / 1.0 / 1.5)  
- annotations  
- source metadata  

---

## 4.3 Input Principle

> CSP assumes normalized inputs. It does not parse arbitrary raw data.

Raw logs, events, or unstructured data must be transformed before entering CSP.

---

# 5. Pipeline Model

CSP is configured through explicit **Pipeline Definitions**.

---

## 5.1 Pipeline Definition

A Pipeline Definition is a **named processing contract**.

It defines:

- pipeline_id  
- pipeline_direction (`ingress | egress | internal | bidirectional`)  
- intake scope  
- clustering policy  
- window policy  
- aggregation policy  
- emission policy  
- output target policy  
- feed publication policy  
- optional CCS emission rules  
- relay visibility policy (optional)  

---

## 5.2 Pipeline Instance

A Pipeline Instance is a **runtime execution** of a definition.

It maintains:

- active windows  
- transient clusters  
- rolling aggregates  
- feed state  

Pipeline instances are:

- transient  
- non-authoritative  
- not required to persist  

---

## 5.3 Pipeline Principle

> Pipelines define how signals flow, not what they mean.

---

# 6. Clustering Model

## 6.1 Purpose

Clustering groups similar signal candidates to reduce noise.

---

## 6.2 Default Identity

Clusters are defined by:

- target  
- signal_type  
- state  

---

## 6.3 Opposing States

Opposing states:

- must not cancel  
- must not merge  

Each state forms independent clusters.

---

## 6.4 Extensibility

Clustering keys may be extended by pipeline definition.

---

# 7. Window Model

CSP supports multiple windowing strategies:

- **Fixed Window** — bounded time intervals  
- **Rolling Window** — continuously moving range  
- **Manual Window** — explicit flush/close  

---

## 7.1 Window Principle

> Aggregation is scoped by windows, not by global accumulation.

---

# 8. Aggregation Model

CSP aggregates cluster data without interpretation.

Possible aggregation outputs:

- count  
- weighted confidence  
- temporal distribution  
- density  

---

## 8.1 Non-Cancellation Rule

Opposing observations remain independent.

No aggregation may:

- cancel opposing states  
- resolve contradictions  

---

# 9. Feed Model

CSP produces **transient feeds**.

---

## 9.1 Feed Types

- **Signal-Centric Feed**  
  - grouped by signal_type/state  

- **Target-Centric Feed**  
  - grouped by target  

---

## 9.2 Properties

Feeds are:

- transient  
- non-authoritative  
- non-durable by default  
- high-frequency compatible  

---

## 9.3 Subscription

Other systems (e.g., CDS, UI, operators) may subscribe to feeds.

Feeds do not:

- create obligations  
- imply urgency  

---

# 10. Emission Model

CSP may optionally emit durable artifacts via CCS.

---

## 10.1 Emission Principle

> Emission is explicit. Triggers are configurable.

---

## 10.2 Emission Modes

- explicit push (default Charter mode)  
- threshold-triggered  
- time-based cadence  
- on-demand  
- hybrid  

---

## 10.3 Emission Targets

- feed only (non-durable)  
- CCS (durable)  
- both  

---

## 10.4 Durability Rule

> Only CCS commits are durable.

All non-CCS outputs are transient.

---

# 11. CCS Integration

CSP may emit:

- CCare-compatible commits (signals, requests, etc.)  
- Host Artifact Commits (generic)  
- future CSP-specific commit families (optional)

---

## 11.1 Metadata

CSP may attach metadata such as:

- pipeline_id  
- aggregation mode  
- window boundaries  
- source counts  

---

## 11.2 Non-Graphing Rule

> CSP metadata must not create structural graph edges by default.

---

# 12. CRS Integration

CRS transports only CCS-emitted artifacts.

---

## 12.1 Relay Modes

### Direct CCare Relay
CCS → CRS

### CSP-Mediated Relay
CSP → CCS → CRS

---

## 12.2 Constraint

CRS:

- remains transport-only  
- does not interpret pipelines  
- does not process transient CSP state  

---

# 13. Ingress / Egress Placement

CSP may be placed in Runtime flows:

---

## 13.1 Ingress

CRS / external input  
→ segregated workspace  
→ CSP pipeline  
→ feed and/or CCS emission  

---

## 13.2 Egress

local candidates  
→ CSP pipeline  
→ CCS emission  
→ CRS transport  

---

## 13.3 Scope Constraint

CSP applies only to:

- signal candidates  
- care-compatible flows  

It must not intercept unrelated commit types.

---

# 14. Workspace Segregation

Relay-facing and external inputs must reside in:

- physically segregated workspaces  
- non-authoritative storage  

These may be processed by CSP before local adoption.

---

# 15. Internal vs Federated Modes

## 15.1 Internal Mode

- no CRS  
- local pipelines only  

## 15.2 Federated Mode

- CSP outputs may be relayed via CRS  
- ingestion may occur through relay  

---

# 16. Design Guarantees

CSP is:

- non-authoritative  
- non-legitimizing  
- non-interpretive  
- configurable  
- high-throughput capable  
- optionally durable  
- pipeline-driven  

---

# 17. Invariants

- CSP does not create legitimacy  
- CSP does not interpret signal meaning  
- opposing states are never collapsed  
- pipelines are explicit and named  
- feeds are non-authoritative  
- durability only exists via CCS  
- CRS only transports durable artifacts  
- CSP operational lineage is not part of structural graph  
- CSP must not process unrelated commit families  

---

# 18. Mental Model

- CCare records observation  
- CSP shapes observation flow  
- CCS preserves durable artifacts  
- CRS transports artifacts  
- CAS interprets alignment  

CSP sits between:

raw signal flow and durable structured observation

---

# 19. Final Principle

The Charter Signal Processing Substrate exists so that:

- high-volume signals can be safely handled  
- noise can be reduced without distortion  
- automation can coexist with human intent  
- systems can scale without overwhelming truth  

CSP shapes the flow of observation without deciding what that observation means.