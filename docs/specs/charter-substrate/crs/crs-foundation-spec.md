# Charter Relay System (CRS) — Foundation Specification (Revised)

Status: FOUNDATIONAL  
Intent: Define append-only, opaque transport and foreign archival of commit artifacts  
Scope: Relay namespaces, transport operations, storage neutrality, federation-safe exchange, and transport boundaries  
Does NOT Define: legitimacy, review workflows, alignment, identity semantics, commit meaning, signal processing, or storage implementation details beyond relay behavior  

---

# 1. Purpose

The Charter Relay System (CRS) defines how commit artifacts are:

- transported between systems  
- stored in foreign, append-only archives  
- retrieved without implying trust or integration  

CRS exists to:

- enable safe exchange of artifacts across boundaries  
- support federation without shared state  
- preserve artifacts without interpretation  
- maintain strict separation between transport and trust  

CRS is a **transport and foreign archival substrate**.

It is not:

- a synchronization system  
- a source of truth  
- a legitimacy system  
- a state reconstruction engine  
- a trust mechanism  

---

# 2. Core Principle

> CRS moves and stores artifacts without assigning meaning or trust.

Artifacts in CRS are:

- opaque  
- append-only  
- non-authoritative  
- non-integrated  

Presence in a relay does not imply:

- correctness  
- legitimacy  
- alignment  
- authorship  
- completeness  

CRS operates only on **durable CCS artifacts**.

CRS does not:

- process transient data  
- interpret signal flows  
- perform aggregation or shaping  

> Signal shaping, aggregation, or emission control must occur outside CRS.

---

# 3. Relay Model

## 3.1 Relay as Foreign Archive

A relay is a collection of **foreign artifact namespaces**.

Each relay:

- accepts commit artifacts  
- stores them append-only  
- exposes retrieval interfaces  
- does not interpret or validate meaning  

> A relay is a permanent foreign archive.

---

## 3.2 Pure Relay Mode

A host may operate CRS in **pure relay mode**:

- accepts commits  
- stores them append-only  
- exposes fetch/list operations  
- performs no review  
- performs no interpretation  

In this mode:

- all artifacts are foreign  
- no local trust is established  

---

## 3.3 No Canonical State

CRS does not define:

- current state  
- latest version  
- active artifacts  
- canonical truth  

It only preserves recorded artifacts.

---

# 4. Relay Namespace Model

## 4.1 Relay Namespaces

A **relay namespace** is a logical partition of the relay archive.

Each namespace:

- stores a set of commit artifacts  
- is append-only  
- is isolated from other namespaces  

---

## 4.2 Namespace Identity

A namespace is identified by:

(namespace_id, relay_endpoint)

Properties:

- namespace identity is scoped to a relay  
- namespace IDs are not globally unique  
- identical namespace IDs may exist across relays  

---

## 4.3 Namespace Purpose

Namespaces provide:

- storage segregation  
- multi-user isolation  
- multi-system partitioning  

They do not provide:

- trust boundaries  
- authority boundaries  
- identity semantics  

---

## 4.4 Foreign Archive Workspace

Each namespace corresponds to a:

> **foreign archive workspace**

Artifacts within a namespace:

- remain foreign  
- are not locally trusted  
- require explicit review for admission into trusted systems  

---

## 4.5 Relay-Facing Workspace Segregation

Relay namespaces correspond to **physically segregated foreign workspaces**.

These workspaces:

- are non-authoritative  
- are isolated from local runtime state  
- are visible to relay transport operations  
- may serve as staging areas for ingestion or export  

Artifacts in relay-facing workspaces:

- remain foreign  
- must not be treated as locally trusted  
- may be processed externally before local adoption  

> Segregation ensures that foreign artifacts do not implicitly merge with local truth.

---

# 5. Transport Operations

CRS defines explicit transport operations.

---

## 5.1 Push

Push transfers commits into a relay namespace.

Properties:

- append-only  
- idempotent  
- no overwrite  
- no merge  

If a commit already exists in the namespace:

- push is a no-op  

---

## 5.2 Fetch

Fetch retrieves commits from a relay namespace.

Fetch does not:

- imply completeness  
- imply freshness  
- imply correctness  

Fetched artifacts remain foreign.

---

## 5.3 Fetch Modes (Minimal Set)

CRS supports:

- fetch by commit_id(s)  
- fetch by cursor (opaque, implementation-defined)  
- fetch full namespace (optional, may be partial in practice)  

Ordering must not imply semantic meaning.

---

## 5.4 No Implicit Synchronization

CRS does not:

- auto-sync  
- auto-update  
- maintain consistency  

All transport is explicit.

---

## 5.5 Transport Boundary

CRS operates strictly at the boundary of **durable artifact transport**.

CRS:

- transports only CCS-compliant artifacts  
- does not accept or process transient data  
- does not participate in signal shaping, aggregation, or emission control  

> Any preprocessing or shaping of artifacts prior to transport must occur outside CRS.

---

# 6. Artifact Handling

## 6.1 Opaque Artifacts

CRS treats all commits as opaque.

It does not interpret:

- commit type  
- payload  
- relationships  
- semantics  

---

## 6.2 Commit Compatibility

CRS transports any valid CCS commit:

- Resolution commits  
- Identity commits  
- Signal commits  
- Request commits  
- Annotation commits  
- Import commits  
- Review / Exploration receipts  
- Deliberate commits  
- Host artifact commits (generic, non-Charter specific)  

All are treated uniformly.

---

## 6.3 Deduplication

Deduplication is:

- per namespace  
- based on commit_id  

Properties:

- duplicate commits are ignored  
- same commit may exist in multiple namespaces  

---

## 6.4 Partiality

CRS makes no guarantees that:

- a namespace is complete  
- all related commits are present  
- a graph can be reconstructed  

Absence does not imply non-existence.

---

# 7. Relationship to Local Systems

## 7.1 Separation from Local Commit Store

CRS is distinct from the Charter Commit Store.

- Commit Store → local trusted artifacts  
- CRS → foreign archived artifacts  

CRS must not:

- modify local commit stores  
- imply trust equivalence  

---

## 7.2 No Required Local Commit Store

CRS may be used without a local Commit Store.

A host system may:

- store artifacts in its own database  
- treat CRS as external archive only  

---

## 7.3 Admission Boundary

Artifacts from CRS:

- remain foreign after fetch  
- must pass through explicit review before becoming trusted  

CRS does not perform admission.

---

## 7.4 Interaction with Signal Processing (CSP)

CRS may operate alongside the Charter Signal Processing Substrate (CSP), but remains strictly independent.

---

### 7.4.1 Direct Relay Path

CCare-compatible artifacts are transported directly:

CCS → CRS

Properties:

- no preprocessing  
- direct transport  
- suitable for low-volume or intentional emission  

---

### 7.4.2 CSP-Mediated Relay Path

Signal candidates or artifacts may be processed before transport:

Signal candidates / foreign artifacts  
→ CSP pipeline  
→ CCS durable emission  
→ CRS transport  

Properties:

- aggregation and noise reduction may occur externally  
- emission cadence may be controlled externally  
- only resulting CCS artifacts are transported  

---

### 7.4.3 Constraint

CRS must not:

- interpret CSP pipelines  
- depend on pipeline configuration  
- process transient pipeline state  

> CRS interacts only with durable outputs, never with pipeline execution.

---

# 8. Federation Model

## 8.1 Federation as Transport

Federation is:

- exchange of artifacts via CRS  
- storage in foreign namespaces  

It is not:

- shared state  
- implicit synchronization  
- trust propagation  

---

## 8.2 Cross-System Exchange

Systems may:

- push to each other’s namespaces  
- fetch from multiple relays  
- store multiple foreign archives  

All artifacts remain foreign until reviewed.

---

## 8.3 No Namespace Relationships

CRS does not define:

- relationships between namespaces  
- mirroring semantics  
- aggregation rules  

These are external concerns.

---

## 8.4 Ingress and Egress Flow Model

CRS participates in both inbound and outbound artifact movement.

---

### 8.4.1 Ingress Flow (Inbound)

CRS fetch  
→ relay-facing workspace (foreign)  
→ optional external processing (e.g., CSP)  
→ optional local admission via review  

Properties:

- fetched artifacts remain foreign  
- no automatic integration occurs  
- preprocessing is external to CRS  

---

### 8.4.2 Egress Flow (Outbound)

Local durable artifacts  
→ optional external processing (e.g., CSP)  
→ CRS push  

Properties:

- CRS only transports final CCS artifacts  
- preprocessing must complete before push  
- CRS does not control emission timing  

---

### 8.4.3 Principle

> CRS defines transport boundaries, not processing pipelines.

---

# 9. Metadata Neutrality

## 9.1 Allowed Metadata

CRS may expose:

- storage timestamps  
- namespace size/count  
- pagination cursors  

---

## 9.2 Forbidden Semantics

CRS must not expose metadata implying:

- “latest”  
- “active”  
- “canonical”  
- “authoritative”  

---

## 9.3 Ordering Neutrality

Ordering:

- may exist for storage  
- must not imply meaning  

---

# 10. Integrity and Authenticity

## 10.1 Integrity Preservation

CRS must preserve:

- commit_id  
- integrity_hash  

without modification.

---

## 10.2 No Authenticity Guarantees

CRS does not guarantee:

- authorship  
- origin  
- trustworthiness  
- legitimacy  

Consumers must verify externally.

---

# 11. Large Payloads and Bundles

CRS:

- does not define payload size limits  
- does not define chunking or streaming  

Bundles and large artifacts:

- are transported as opaque commits or external packages  
- remain outside CRS semantics  

---

# 12. Invariants

The following must always hold:

- all artifacts are append-only  
- no artifact is modified in place  
- all artifacts remain opaque  
- transport is explicit  
- deduplication is idempotent  
- namespaces are isolated  
- no trust is implied by storage  
- no interpretation occurs within CRS  
- no canonical state is defined  
- no relationships are inferred  
- partial data is valid  
- history is never rewritten  
- CRS operates only on durable CCS artifacts  
- CRS does not process transient or pipeline-managed data  
- CRS does not interpret or depend on CSP pipelines  

---

# 13. Mental Model

CRS is:

- a distributed append-only archive  
- a transport layer for opaque artifacts  
- a system for exchanging foreign truth  

CRS is not:

- a database of truth  
- a synchronization engine  
- a governance system  
- a semantic graph  
- a trust system  
- a signal processing system  

---

# 14. Final Principle

CRS ensures that:

- artifacts can move safely between systems  
- storage never implies trust  
- exchange never implies agreement  
- history remains intact across boundaries  

It enables systems to share information  
without ever requiring them to believe it.