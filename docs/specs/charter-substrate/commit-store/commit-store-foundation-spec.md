# Charter Commit Store — Foundational Specification

Status: FOUNDATIONAL  
Intent: Define how commits are stored, retrieved, and lifecycle-managed locally  
Scope: Commit persistence, indexing, archival, and purge boundaries  
Does NOT Define: commit structure (CCS), legitimacy, runtime workflows, alignment, guidance, or transport semantics  

---

# 1. Purpose

The Charter Commit Store defines how commits are:

- stored locally  
- retrieved and indexed  
- preserved over time  
- archived and optionally purged  

It exists to provide a **durable storage substrate** for commit artifacts defined by the Charter Commit System (CCS).

The Commit Store manages **where commits are available**, not **what they mean**.

---

# 2. Core Principle

> The Commit Store preserves commits. It does not interpret them.

The Commit Store:

- stores commit artifacts exactly as defined by CCS  
- treats all commits as opaque structures  
- does not assign meaning, authority, or correctness  
- does not compute state, alignment, or relationships  

It is a **storage and lifecycle system**, not a semantic system.

---

# 3. Logical Model vs Physical Storage

## 3.1 Logical Append-Only Model

The Commit Store is **logically append-only**:

- commits, once recorded, are never modified  
- commit identity and integrity are permanent  
- history is never rewritten  

This defines the **truth model** of the system.

---

## 3.2 Physical Storage Behavior

Physical storage MAY:

- archive commits to different storage tiers  
- remove commits via explicit purge  

However:

> Physical removal does not alter the historical validity of commits as artifacts.

Absence in the local store does not imply non-existence.

---

# 4. Separation of Concerns

## 4.1 Relationship to CCS

- CCS defines **what a commit is**  
- Commit Store defines **how commits are stored and retained**  

The Commit Store must treat all commits as:

- opaque  
- immutable  
- append-only artifacts (logically)  

---

## 4.2 Relationship to Runtime Layer

- Runtime produces commits  
- Runtime queries commits  
- Runtime initiates archival and purge workflows  

The Commit Store must not:

- orchestrate workflows  
- enforce runtime rules  
- modify commit contents  

---

## 4.3 Separation from Runtime Object Store

The Commit Store is **distinct from the Runtime object store**.

| Runtime Object Store | Commit Store |
|---------------------|--------------|
| Mutable working state | Immutable commit artifacts |
| Supports workflows | Stores recorded artifacts |
| May be reorganized | Logically append-only |
| Not portable by default | Portable via commits |

These systems must never be conflated.

---

# 5. Non-Interpretation Guarantee

The Commit Store must not:

- inspect commit payload semantics  
- branch logic based on commit type  
- infer relationships between commits  
- compute derived or canonical state  

> The Commit Store must not define or expose “current”, “latest”, or “authoritative” state.

All interpretation belongs to higher layers (Runtime, CAS, CGL).

---

# 6. Storage Model

## 6.1 Identity and Integrity

For every stored commit:

- commit_id must remain unchanged  
- integrity_hash must remain verifiable  
- commit contents must remain byte-consistent  

The store must not:

- rewrite commits  
- reassign identities  
- alter hashes  

---

## 6.2 Opaque Storage

Commits must be stored as opaque artifacts.

The store may index metadata fields but must not:

- depend on internal semantics  
- alter storage behavior based on meaning  

---

## 6.3 No Ordering Guarantee

The Commit Store must not guarantee:

- global ordering of commits  
- causal ordering between commits  

Ordering may exist via:

- timestamps  
- external interpretation  

But is not enforced by storage.

---

# 7. Indexing

The Commit Store may provide indexes for:

- commit_id  
- commit_type  
- referenced_area_id  
- timestamps  

Indexes:

- are secondary and rebuildable  
- must not affect commit integrity  
- must not introduce inferred relationships  

> Indexes accelerate access. They do not create meaning.

---

# 8. Retrieval Model

The Commit Store must support:

- direct retrieval by commit_id  
- filtered queries (type, area, time)  

Retrieval must:

- return original, unmodified commit artifacts  

Retrieval must not:

- merge commits  
- synthesize views  
- infer relationships  
- transform commit contents  

---

# 9. Presence Model

The Commit Store operates over **local availability**, not global completeness.

Each commit is in one of three states:

- **Present** — stored locally and queryable  
- **Archived** — stored in an alternate location (local or external)  
- **Absent** — not present in the local store  

> The Commit Store does not guarantee that all related commits are present locally.

---

# 10. Lifecycle Model

The Commit Store defines **storage lifecycle only**, not semantic lifecycle.

---

## 10.1 Default State

All commits are:

- present  
- locally available  
- queryable  

No implicit aging or expiration exists.

---

## 10.2 Archival

Archival changes storage posture without altering commits.

### Properties:

- commits remain immutable  
- identity and hashes preserved  
- relationships preserved  
- storage location may change  

Archival may move commits to:

- cold storage  
- export bundles  
- relay systems  

---

### Archival Invariants

Archival must not:

- mutate commits  
- replace commits  
- create new identities for existing commits  

Archival may produce **new commits representing archival artifacts**,  
but must not alter original commits.

---

### Archival Modes

Archival may use modes such as:

- MINIMAL  
- CLOSURE  
- NARRATIVE  

These define completeness, not meaning.

---

## 10.3 Purge (Explicit and Controlled)

Purge is the **physical removal of commits from the local store**.

> Purge affects local availability, not historical existence.

---

### Purge Requirements

Purge must be:

- explicit  
- user-initiated  
- auditable  

Restoration requires re-import or retrieval from archive/relay.

---

### Purge Responsibilities

The Commit Store:

- executes removal  
- reports impact  
- preserves auditability  

The Commit Store must not:

- enforce semantic importance  
- prevent purge based on meaning  
- infer which commits “should” be retained  

Higher layers (Runtime, CAS, user) are responsible for decisions.

---

# 11. Import and Rehydration

The Commit Store must support:

- importing commits from external sources  
- restoring archived commits  
- rehydrating purged data  

Imported commits:

- retain original identity  
- retain integrity  
- preserve provenance  

They are treated as **locally stored artifacts with external origin**,  
not as transformed or reinterpreted data.

---

# 12. Relationship to Relay (CRS)

The Commit Store may:

- push commits to relay  
- fetch commits from relay  

Relay:

- acts as transport and external archival  
- does not affect local storage semantics  

The Commit Store must not:

- assume relay completeness  
- depend on relay for correctness  

---

# 13. Relationship to Alignment and Guidance

The Commit Store provides input to:

- Charter Alignment System (CAS)  
- Charter Guidance Layer (CGL)  

It does not:

- compute alignment  
- interpret meaning  
- generate insights  

---

# 14. Retention Policy Boundary

The Commit Store does not:

- enforce retention policies  
- automatically archive data  
- automatically purge data  

All lifecycle transitions are:

- explicit  
- externally initiated  

---

# 15. Invariants

The following must always hold:

- commits are immutable  
- commit identity is preserved  
- commit integrity is verifiable  
- storage does not alter meaning  
- archival does not alter meaning  
- purge does not imply global deletion  
- commit interpretation never occurs at this layer  
- local completeness is not guaranteed  

Violation of these invariants breaks the Commit Store contract.

---

# 16. What the Commit Store Does NOT Do

The Commit Store does not:

- define commit structure (CCS responsibility)  
- create legitimacy (Legitimacy Engine responsibility)  
- orchestrate workflows (Runtime responsibility)  
- compute alignment (CAS responsibility)  
- interpret commits (CGL responsibility)  
- transport commits (CRS responsibility)  
- define canonical state  

---

# 17. Mental Model

The Commit Store is:

- a local ledger of artifacts  
- a storage boundary for recorded truth  
- a system of availability and retention  

It is not:

- a source of meaning  
- a database of current state  
- a decision engine  
- a graph interpreter  

---

# 18. Final Principle

The Commit Store ensures that:

- commits are preserved faithfully  
- storage can evolve without rewriting history  
- data can be reduced locally without destroying truth globally  

It manages **where artifacts are available**,  
without ever deciding **what those artifacts mean**.