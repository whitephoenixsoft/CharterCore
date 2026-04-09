# Charter Signal Processing Substrate (CSP) — Foundation Specification (Revised v2)

Status: FOUNDATIONAL  
Intent: Define high-throughput signal shaping, aggregation, publication, and optional durable emission prior to durable artifact creation  
Scope: Signal candidate processing, pipeline definitions, clustering, aggregation, feeds, subscriptions, and emission policies  
Depends On: Runtime Layer, Charter Commit System (CCS) (optional durability), optional Charter Care Substrate (CCare)  
Interacts With: CRS (transport), CDS (subscriptions), CAS (indirect via CCare), CIS (optional scoped feeds)  
Does NOT Define: legitimacy, alignment computation, identity semantics, graph structure (CSG), or guidance (CGL)  

---

# 1. Purpose

The Charter Signal Processing Substrate (CSP) defines how **high-volume signal-like inputs are shaped, aggregated, and surfaced** before becoming durable artifacts.

It exists to:

- process high-frequency or noisy inputs safely  
- reduce signal noise through clustering and aggregation  
- provide configurable emission and cadence control  
- support both human-centric and automated systems  
- publish named feeds for ongoing monitoring  
- support identity-, area-, target-, and signal-scoped publication  
- enable optional transformation into CCS-backed artifacts  
- act as a bridge between raw observation and structured care  

CSP is a **non-authoritative signal shaping substrate**.

---

# 2. Core Principle

> CSP shapes signal flow. It does not create meaning, authority, or legitimacy.

CSP:

- operates on normalized signal candidates  
- aggregates without interpreting semantics  
- preserves opposing observations independently  
- emits outputs without implying obligation  
- publishes feeds without implying urgency  
- defers meaning to downstream systems (CCare, CAS, CGL)  

---

# 3. Architectural Position

CSP is a **Runtime-hosted substrate**.

It operates:

- before CCS durability (optional)  
- alongside CCare (but not within it)  
- independently of CAS and CSG  
- before CRS transport when configured for egress shaping  
- after foreign acquisition when configured for ingress shaping  

CSP may be positioned:

- on **ingress** (before local adoption of external signal-like artifacts)  
- on **egress** (before emitting signal-derived artifacts to CCS/CRS)  
- **internally** (for local-only processing and feeds)  
- **bidirectionally** across both ingress and egress flows  

CSP does not:

- create legitimacy  
- evaluate alignment  
- create CDS Items automatically  
- transport artifacts directly  
- interpret graph semantics  

---

# 4. Signal Candidate Model

## 4.1 Definition

CSP operates on **signal candidates**, which are normalized, minimal signal inputs.

Signal candidates may originate from:

- human input  
- CCare-compatible sources  
- surveys  
- telemetry systems  
- external pipelines  
- imported or federated care-compatible artifacts  

---

## 4.2 Required Shape

A signal candidate must include:

- `target`  
- `signal_type`  
- `state`  
- `timestamp`  

`target` may reference:

- resolution  
- area  
- identity  
- global  

---

## 4.3 Optional Shape

A signal candidate may include:

- `confidence` (numeric)  
- `annotations`  
- `source_class`  
- `evidence_refs`  
- source-specific metadata  

Canonical confidence values are:

- low = `0.5`  
- medium = `1.0`  
- high = `1.5`  

Hosts may provide enum or numeric input, but CSP normalization must preserve or map into this numeric model.

---

## 4.4 Input Principle

> CSP assumes normalized inputs. It does not parse arbitrary raw data.

CSP must not accept directly:

- raw logs  
- raw telemetry streams  
- arbitrary events  
- unanchored observations  

Normalization happens outside CSP.

---

## 4.5 Care Compatibility Principle

> If an input does not express an alignment-relevant observation about a target, it does not enter CSP.

CSP is generic in processing behavior, not in raw semantic ingestion.

---

# 5. Pipeline Model

CSP is configured through explicit **Pipeline Definitions**.

---

## 5.1 Pipeline Definition

A Pipeline Definition is a **named processing contract**.

It defines:

- `pipeline_id`  
- `pipeline_direction` (`ingress | egress | internal | bidirectional`)  
- intake scope  
- clustering policy  
- window policy  
- aggregation policy  
- emission policy  
- output target policy  
- feed publication policy  
- optional CCS emission rules  
- relay visibility policy (optional)  
- optional scoping filters  
- optional identity, area, target-set, and signal filters  

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

## 5.4 Processing Profiles

Pipelines may be configured from conceptual processing profiles such as:

- pass-through  
- low-cadence  
- team-windowed  
- high-output  
- real-time  

Profiles are templates. Pipeline Definitions are the explicit configured units.

---

# 6. Clustering Model

## 6.1 Purpose

Clustering groups similar signal candidates to reduce noise.

---

## 6.2 Default Identity

Clusters are defined by:

- `target`  
- `signal_type`  
- `state`  

This is the canonical default clustering key.

---

## 6.3 Opposing States

Opposing states:

- must not cancel  
- must not merge  
- must not resolve contradiction  

Each state forms independent clusters.

Interpretation of competing states is deferred to CAS or other downstream systems.

---

## 6.4 Extensibility

Clustering keys may be extended by pipeline definition.

Additional dimensions may include:

- source_class  
- confidence band  
- scoped publication partition  

Annotations and evidence references do not define cluster identity by default.

---

# 7. Window Model

CSP supports multiple windowing strategies:

- **Fixed Window** — bounded intervals  
- **Rolling Window** — continuously moving range  
- **Manual Window** — explicit flush/close  

---

## 7.1 Window Principle

> Aggregation is scoped by windows, not by global accumulation.

---

## 7.2 Window Policy

Window policy is pipeline-defined and may vary by use case, including:

- retrospectives  
- surveys  
- operational monitoring  
- telemetry systems  
- federated imports  

---

# 8. Aggregation Model

CSP aggregates cluster data without interpretation.

Possible aggregation outputs include:

- count  
- weighted confidence  
- temporal distribution  
- density  
- source-class summary  
- confidence summary  

---

## 8.1 Non-Cancellation Rule

Opposing observations remain independent.

No aggregation may:

- cancel opposing states  
- resolve contradiction  
- infer correctness  
- imply authority  

---

## 8.2 Confidence Aggregation

Confidence aggregation must preserve the numeric model.

Valid outputs may include:

- weighted totals  
- mean confidence  
- confidence distribution summaries  

CSP does not redefine the meaning of confidence.

---

# 9. Feed Model

CSP produces **named, transient feeds**.

A Feed is a:

> named, transient publication surface produced by a CSP pipeline

Feeds expose processed signal outputs for consumption by:

- CDS  
- operators  
- UI or CLI surfaces  
- external systems  

Feeds are:

- non-authoritative  
- transient  
- subscribable  
- pipeline-defined  

---

## 9.1 Feed Identity

Each feed must have:

- `feed_id`  
- `pipeline_id`  

A feed cannot exist without a pipeline.

Optional feed metadata may include:

- annotations  
- aggregation mode  
- windowing metadata  
- scoped publication metadata  

---

## 9.2 Feed Types

CSP defines two canonical feed types:

### A. Signal-Centric Feed

Grouped by:

- signal_type  
- state  

Used for:

- pattern detection  
- clustering visibility  
- system-wide signal monitoring  

### B. Target-Centric Feed

Grouped by:

- target  

Used for:

- monitoring specific resolutions, areas, identities, or target sets  
- care-driven investigation  
- CDS subscriptions  

---

## 9.3 Feed Scope

A feed may be scoped to:

### A. Pipeline Scope
- all signal candidates processed by the pipeline  

### B. Explicit Target Scope
- resolution_ids  
- area_ids  
- identity_ids  
- mixed target sets  

Target-scoped feeds are the primary mode for care deliberates and large organizations.

---

## 9.4 Feed Filters

Feed publication may be filtered by pipeline definition using:

- identity scope  
- area scope  
- explicit target set  
- signal type  
- signal state  
- source class  

Subscriptions may narrow feed scope further without redefining the feed.

---

## 9.5 Feed Behavior

Feeds:

- update according to pipeline cadence  
- reflect clustered or aggregated publication state  
- do not guarantee completeness  
- do not imply stability  
- may be consumed as snapshots or ongoing streams  

---

## 9.6 Subscription

Other systems may subscribe to feeds.

Subscriptions:

- are non-authoritative  
- do not create obligations  
- do not imply urgency  
- do not mutate downstream state automatically  

CDS may consume feed outputs for ongoing monitoring, but feed subscription does not create Items automatically.

---

## 9.7 Feed Principle

> Feeds expose shaped observation flow without assigning meaning or triggering action.

---

# 10. Emission Model

CSP may optionally emit durable artifacts via CCS.

---

## 10.1 Emission Principle

> Emission is explicit. Triggers are configurable.

---

## 10.2 Emission Modes

Supported emission modes include:

- explicit push (default Charter mode)  
- threshold-triggered  
- time-based cadence  
- on-demand  
- hybrid  

Default Charter mode is **explicit push** to preserve low-noise, intentional history.

---

## 10.3 Emission Targets

A pipeline may emit to:

- feed only (non-durable)  
- CCS (durable)  
- both  

---

## 10.4 Durability Rule

> Only CCS commits are durable.

All non-CCS outputs are transient and outside Charter durability guarantees.

---

## 10.5 Automation Boundary

CSP is the primary transition point between:

- human-first Charter workflows  
- automation-compatible signal processing  

Automation may change cadence and emission behavior, but must not change meaning.

---

# 11. CCS Integration

CSP may emit:

- CCare-compatible commits (signals, requests, supportability artifacts, suggestions where defined)  
- Host Artifact Commits (generic, non-Charter-native artifacts)  
- future CSP-specific durable artifact families (optional)

---

## 11.1 Metadata

CSP may attach metadata such as:

- `pipeline_id`  
- aggregation mode  
- window boundaries  
- source counts  
- source-class summary  
- confidence summary  
- processing profile  

---

## 11.2 Non-Graphing Rule

> CSP metadata must not create structural graph edges by default.

CSP operational lineage, processing lineage, and pipeline provenance remain:

- metadata  
- or members of segregated CSP-specific durable families  

They do not participate in CSG unless a future spec explicitly promotes them.

---

## 11.3 Segregation Principle

Only references intended to participate in shared Charter structural semantics may enter the shared structural graph.

CSP operational references remain segregated.

---

# 12. CRS Integration

CRS transports only CCS-emitted artifacts.

---

## 12.1 Relay Modes

### A. Direct CCare Relay

CCare-compatible CCS artifacts  
→ CRS

Used when:

- volume is low  
- no shaping is needed  
- direct durability is sufficient  

### B. CSP-Mediated Relay

Signal candidates / imported care-compatible artifacts  
→ CSP pipeline  
→ optional CCS durable emission  
→ CRS

Used when:

- aggregation is needed  
- cadence control is needed  
- ingress/egress shaping is needed  
- noise reduction matters  

---

## 12.2 Constraint

CRS:

- remains transport-only  
- does not interpret pipelines  
- does not process transient CSP state  
- does not depend on pipeline semantics  

Runtime composes CSP and CRS. CRS does not own CSP behavior.

---

# 13. Ingress / Egress Placement

CSP may be placed in Runtime flows.

---

## 13.1 Ingress

External or foreign care-compatible input  
→ segregated workspace  
→ CSP pipeline  
→ feed and/or CCS emission  

Use cases include:

- relay-fetched care artifacts  
- imported survey-derived signals  
- external monitoring inputs  

---

## 13.2 Egress

Local signal candidates or care-compatible artifacts  
→ CSP pipeline  
→ optional CCS emission  
→ CRS transport  

---

## 13.3 Internal

Local candidates  
→ CSP pipeline  
→ feed and/or CCS emission  

No relay required.

This supports centralized internal systems and server-terminal deployments.

---

## 13.4 Scope Constraint

CSP applies only to:

- signal candidates  
- care-compatible flows  
- care-derived publication and emission  

It must not intercept unrelated commit types.

---

# 14. Workspace Segregation

Relay-facing and external inputs must reside in:

- physically segregated workspaces  
- non-authoritative storage  

These workspaces:

- are visible to relay/governed ingestion paths  
- are isolated from local trusted runtime state  
- may be processed by CSP before local adoption  

Segregation prevents foreign or noisy inputs from implicitly becoming local truth.

---

# 15. Internal vs Federated Modes

## 15.1 Internal Mode

- no CRS required  
- local pipelines only  
- feeds and optional local CCS emission supported  

## 15.2 Federated Mode

- CSP outputs may be relayed via CRS  
- ingress may occur through relay-facing workspaces  
- both direct and CSP-mediated care relay paths are valid  

---

# 16. Relationship to CDS

CSP does not create CDS Items automatically.

CSP may:

- publish feeds for CDS subscription  
- publish target-centric feeds for investigative monitoring  
- surface grouped observations for human or Runtime consumption  

Runtime or humans decide how feed outputs become investigative inputs.

---

# 17. Relationship to CIS

Identity-scoped publication is supported through feed and pipeline filters.

CSP does not define identity semantics, but may scope publication using identity references supplied by upstream normalization or configured pipeline scope.

---

# 18. Relationship to CAS

CSP does not compute alignment.

CSP shapes signal flow before durable CCare input reaches CAS.

CAS may consume CCare artifacts that were:

- directly emitted  
- CSP-shaped and emitted  
- aggregated under explicit pipeline policy  

---

# 19. Design Guarantees

CSP is:

- non-authoritative  
- non-legitimizing  
- non-interpretive  
- configurable  
- high-throughput capable  
- optionally durable  
- pipeline-driven  
- feed-oriented  
- compatible with both human and automated systems  

---

# 20. Invariants

- CSP does not create legitimacy  
- CSP does not interpret signal meaning  
- opposing states are never collapsed  
- pipelines are explicit and named  
- feeds are named, transient, and non-authoritative  
- feeds do not create CDS Items automatically  
- durability only exists via CCS  
- CRS only transports durable artifacts  
- CSP operational lineage is not part of the shared structural graph  
- CSP must not process unrelated commit families  
- normalization happens outside CSP  
- feed scope and publication policy must be explicit  

---

# 21. Mental Model

- CCare records observation  
- CSP shapes observation flow  
- feeds expose shaped observation for monitoring  
- CCS preserves durable artifacts  
- CRS transports durable artifacts  
- CAS interprets alignment  

CSP sits between:

raw or high-volume observation flow and durable structured care-compatible artifacts

---

# 22. Final Principle

The Charter Signal Processing Substrate exists so that:

- high-volume signals can be safely handled  
- noise can be reduced without distortion  
- automation can coexist with human intent  
- named monitoring feeds can support ongoing investigation  
- systems can scale without overwhelming truth  

CSP shapes the flow of observation  
without ever deciding what that observation means.