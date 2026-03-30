# Charter Commit Store — Foundational Specification

Status: FOUNDATIONAL  
Intent: Define how commits are stored, retrieved, and lifecycle-managed locally  
Scope: Commit persistence, indexing, archival, purge boundaries  
Does NOT Define: commit structure (CCS), legitimacy, runtime workflows, alignment, guidance, or transport semantics  

---

# 1. Purpose

The Charter Commit Store defines how commits are:

- stored locally  
- retrieved and indexed  
- preserved over time  
- archived and optionally purged  

It exists to provide a **durable, append-only storage substrate** for commit artifacts defined by the Charter Commit System (CCS).

The Commit Store manages **where commits live**, not **what they mean**.

---

# 2. Core Principle

> The Commit Store preserves commits. It does not interpret them.

The Commit Store:

- stores commit artifacts exactly as defined by CCS  
- does not assign meaning, authority, or correctness  
- does not compute state or alignment  

It is a **storage and lifecycle system**, not a semantic system.

---

# 3. Separation of Concerns

## 3.1 Relationship to CCS

- CCS defines **what a commit is**  
- Commit Store defines **how commits are stored and retained**  

The Commit Store must treat all commits as:

- opaque  
- immutable  
- append-only artifacts  

---

## 3.2 Relationship to Runtime Layer

- Runtime produces commits  
- Runtime may query commits  
- Runtime may initiate archival or purge workflows  

The Commit Store must not:

- orchestrate workflows  
- enforce runtime rules  
- modify commit contents  

---

## 3.3 Separation from Runtime Object Store

The Commit Store is **distinct from the Runtime object store**.

| Runtime Object Store | Commit Store |
|---------------------|--------------|
| Mutable working state | Immutable commit artifacts |
| Supports workflows | Stores outcomes and records |
| May be pruned or reorganized | Append-only (with controlled purge) |
| Not portable by default | Portable via commits |

The two systems must never be conflated.

---

# 4. Storage Model

## 4.1 Append-Only Storage

The Commit Store is append-only by default:

- new commits may be added  
- existing commits must not be modified  
- commit identity and integrity must be preserved  

---

## 4.2 Identity and Integrity

For every stored commit:

- commit_id must remain unchanged  
- integrity_hash must remain verifiable  
- referenced relationships must remain intact  

The store must not:

- rewrite commits  
- reassign identities  
- alter hashes  

---

## 4.3 Indexing

The Commit Store may provide indexing for:

- commit_id  
- commit_type  
- referenced_area_id  
- timestamps  
- relationships  

Indexes:

- are secondary structures  
- may be rebuilt  
- must not affect commit integrity  

---

# 5. Retrieval Model

The Commit Store must support:

- direct retrieval by commit_id  
- traversal via commit relationships  
- filtered queries (type, area, time)  

All retrieval must return:

- original, unmodified commit artifacts  

---

# 6. Lifecycle Model

The Commit Store defines **storage lifecycle**, not semantic lifecycle.

---

## 6.1 Default State

All commits are:

- active  
- locally available  
- queryable  

No implicit aging or expiration exists.

---

## 6.2 Archival

Archival moves commits into a **different storage posture** without altering them.

### Properties:

- commits remain immutable  
- identity and hashes preserved  
- relationships preserved  
- may be relocated to:
  - cold storage  
  - export bundles  
  - relay endpoints  

### Archival does NOT:

- delete commits  
- compress meaning  
- rewrite history  

---

## 6.3 Archival Modes

Archival may follow modes such as:

- MINIMAL  
- CLOSURE  
- NARRATIVE  

These modes determine **completeness**, not meaning.

---

## 6.4 Purge (Explicit and Controlled)

Purge is the **physical removal of commits from the local store**.

### Critical Distinction:

> Purge affects local availability, not historical truth.

---

### Purge Requirements

Purge must be:

- explicit  
- user-initiated  
- auditable  
- reversible only via re-import  

---

### Purge Workflow (Conceptual)

1. Selection  
   - commits identified via filters or explicit selection  

2. Review  
   - dependencies evaluated  
   - impact understood  

3. Archival (recommended)  
   - commits exported before removal  

4. Purge Execution  
   - commits removed from local store  

---

### Purge Constraints

The system must prevent or warn against:

- breaking required dependency chains (unless explicitly allowed)  
- accidental removal of critical lineage without awareness  

---

### Important Boundary

Purge does NOT:

- invalidate commits globally  
- erase commits from relay or other systems  
- alter CCS invariants  

---

# 7. Import and Rehydration

The Commit Store must support:

- importing commits from external sources  
- restoring archived commits  
- rehydrating purged data  

Imported commits:

- retain original identity  
- remain immutable  
- are treated as local copies of external truth  

---

# 8. Relationship to Relay (CRS)

The Commit Store may:

- push commits to relay  
- fetch commits from relay  

Relay:

- acts as external archival/transport  
- does not affect local storage rules  

The Commit Store must not:

- assume relay completeness  
- depend on relay for correctness  

---

# 9. Relationship to Alignment and Guidance

The Commit Store provides input to:

- Charter Alignment System (CAS)  
- Charter Guidance Layer (CGL)  

It does not:

- compute alignment  
- interpret meaning  
- generate insights  

---

# 10. Invariants

The following must always hold:

- commits are stored immutably  
- commit identity is preserved  
- commit integrity is verifiable  
- storage does not alter meaning  
- archival does not alter meaning  
- purge does not imply deletion of truth globally  
- commit relationships remain intact unless explicitly broken  

Violation of these invariants breaks the Commit Store contract.

---

# 11. What the Commit Store Does NOT Do

The Commit Store does not:

- define commit structure (CCS responsibility)  
- create legitimacy (Legitimacy Engine responsibility)  
- orchestrate workflows (Runtime responsibility)  
- compute alignment (CAS responsibility)  
- interpret commits (CGL responsibility)  
- transport commits (CRS responsibility)  

---

# 12. Mental Model

The Commit Store is:

- a durable ledger of artifacts  
- a local memory of recorded truth  
- a storage boundary with lifecycle controls  

It is not:

- a database of current state  
- a decision engine  
- an interpretation layer  

---

# 13. Final Principle

The Commit Store ensures that:

- commits are preserved faithfully  
- storage can evolve without rewriting history  
- data can be reduced locally without destroying truth globally  

It manages **where truth lives**,  
without ever deciding **what truth means**.