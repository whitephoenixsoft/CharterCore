# Charter Commit System (CCS) — Foundation Specification (Revised vNext)

Status: FOUNDATIONAL  
Intent: Define the commit as Charter’s core immutable artifact envelope  
Scope: Commit identity, integrity, structure, invariants, and cross-module boundaries  
Does NOT Define: storage implementation, runtime orchestration, legitimacy semantics, alignment computation, guidance behavior, or transport behavior  

---

# 1. Purpose

The Charter Commit System (CCS) defines the **commit** as the fundamental, durable unit of recorded artifact within the Charter platform.

It exists to:

- provide a stable, immutable envelope for recorded artifacts  
- preserve declared or observed semantic truth without rewriting history  
- support independent storage, transport, and interpretation  
- provide a common structural substrate for multiple Charter modules and downstream systems  

CCS is a **protocol and structural layer**.

It is not:

- a storage system  
- a workflow engine  
- a legitimacy engine  
- a state machine  
- an interpretation layer  

---

# 2. Core Principle

> Commits preserve recorded artifacts. They do not interpret them.

A commit records that something was:

- declared  
- observed  
- packaged  
- preserved  
- derived  
- reconciled  

A commit does not determine:

- correctness  
- legitimacy  
- alignment  
- authority  
- currentness  

Meaning emerges only through interpretation by systems outside CCS.

---

# 3. The Commit Primitive

A **commit** is:

- immutable  
- append-only  
- independently identifiable  
- structurally self-describing  
- optionally related to other artifacts through explicit references  

A commit is an **artifact envelope**, not mutable state.

---

## 3.1 Required Properties

### Immutability
Once created, a commit must never be modified.

### Append-Only Existence
Commits may be added, copied, archived, transported, or removed from a local store under explicit lifecycle rules, but the commit itself must never be altered.

### Stable Identity
Each commit has a globally unique `commit_id`.

### Deterministic Integrity
Each commit includes an `integrity_hash`.

### Structural Self-Description
Each commit declares its own `commit_type` and schema version.

### Explicit References Only
All relationships must be explicitly declared.

---

## 3.2 Non-Properties

A commit is not:

- a source of authority  
- a unit of legitimacy by default  
- a computed state  
- a mutable object  
- a current-state projection  
- a guarantee of relational completeness  

---

## 3.3 Node-Class Neutrality

CCS does not define or interpret node classes.

However:

- commits may represent different artifact classes (e.g., resolutions, items)  
- CCS must preserve all references required for downstream systems (e.g., CSG) to construct node-class-aware graphs  

> CCS preserves structure. It does not classify it.

---

# 4. Identity and Integrity

## 4.1 Commit Identity

`commit_id`:

- globally unique  
- stable across systems  
- independent of storage  

---

## 4.2 Commit Integrity

`integrity_hash`:

- deterministic  
- validates full envelope  
- signals corruption if invalid  

---

## 4.3 Nested Integrity

Payloads may include additional integrity mechanisms.

These:

- do not replace commit identity  
- do not redefine CCS guarantees  

---

# 5. Commit Envelope

Minimum fields:

- `commit_id`  
- `commit_type`  
- `commit_schema_version`  
- `created_at`  
- `integrity_hash`  
- `payload`  

Optional:

- `references`  

---

## 5.1 Envelope Principle

The envelope provides:

- identity  
- integrity  
- structural neutrality  

All semantics reside in payload or external systems.

---

## 5.2 Timestamp Neutrality

`created_at` is informational only.

It must not imply:

- causality  
- ordering semantics  
- authority  

---

# 6. Commit Types

## 6.1 Core Commit Type Families

- Resolution Commit  
- Identity Commit  
- Signal Commit  
- Request Commit  
- Annotation Commit  
- Import Commit  

### Receipts
- Review Receipt Commit  
- Deliberate Receipt Commit  
- Reconciliation Receipt Commit  
- Legitimacy Receipt Commit  

### Exploratory / Pre-Legitimacy
- Deliberate Commit  

### Generic / Extension
- Host Artifact Commit (generic, non-Charter-specific)  

---

## 6.2 Resolution Commit

Represents a decision artifact.

- may carry legitimacy when produced via sessions  
- may include structural references:
  - `supersedes`  
  - `derives_from`  

---

## 6.3 Identity Commit

Represents identity declaration.

- non-legitimizing  
- consumed by CIS  

---

## 6.4 Signal Commit

Represents observation.

- non-authoritative  
- consumed by CCare and CAS  

---

## 6.5 Request Commit

Represents non-coercive request.

---

## 6.6 Annotation Commit

Provides context or explanation.

---

## 6.7 Import Commit

Preserves foreign artifacts.

---

## 6.8 Deliberate Commit

Represents exploratory or investigative artifacts.

- non-legitimizing  
- may represent Items or drafts  

---

## 6.9 Review Receipt Commit

Represents closure of a review process.

- non-legitimizing  
- captures evaluation outcomes  

---

## 6.10 Deliberate Receipt Commit

Represents closure of a deliberate instance.

- non-legitimizing  
- captures:
  - item sets  
  - applied items  
  - settled items  

---

## 6.11 Reconciliation Receipt Commit

Represents synchronization between:

- Items (CDS)  
- Resolutions (legitimacy layer)  

Properties:

- non-legitimizing  
- records mapping:
  - item ↔ resolution  
- may reference `derived_from` relationships  

> Reconciliation records alignment between thinking and decision without creating authority.

---

## 6.12 Legitimacy Receipt Commit

Represents session closure.

- **only receipt type that records legitimacy**  
- captures:
  - authority  
  - scope  
  - participant set  
  - stances  
  - outcome  

---

## 6.13 Host Artifact Commit

Represents generic, non-Charter-specific data.

- used for external systems  
- prevents misuse of CCare or CDS commits  
- still follows CCS invariants  

---

## 6.14 Extension Principle

New commit types must:

- preserve envelope structure  
- declare schema version  
- avoid bypassing CCS  

---

# 7. Commit Type Schema Authority

Each commit type defines:

- required fields  
- optional fields  
- canonical form  

CCS validates structure, not meaning.

---

# 8. Commit Versioning

Every commit declares `commit_schema_version`.

- immutable per version  
- no retroactive reinterpretation  

---

# 9. Structural References

## 9.1 Reference Block

Each reference includes:

- `ref_type`  
- `target_kind`  
- `target_id`  

---

## 9.2 Target Kinds

- `commit`  
- `area`  
- `bundle`  
- `export`  

---

## 9.3 Structural Reference Families

### Generic
- `references`  
- `annotates`  

### Packaging
- `contains`  
- `preserves`  
- `originates_from`  
- `produced_by_review`  

### Structural Lineage
- `supersedes`  
- `derives_from`  

### Boundary
- `references_area`  

---

## 9.4 Derivation Reference

`derives_from` defines lineage.

Properties:

- explicit  
- directional  
- non-destructive  
- cross-area allowed  
- **cross-artifact-class allowed (e.g., item ↔ resolution)**  

It does not imply:

- authority  
- correctness  
- supersession  

---

## 9.5 Relationship Admission Rule

Structural relationships admitted via review must be encoded explicitly as references in resulting commits.

---

## 9.6 No Inference Rule

No relationships may be inferred from:

- timestamps  
- storage order  
- similarity  

---

# 10. Commit Independence and Graph Shape

- commits are independently valid  
- graph is optional and explicit  
- incomplete graphs are valid  

CCS does not guarantee:

- reachability  
- completeness  

---

# 11. Relationship to Other Modules

## 11.1 Runtime Layer

Runtime may orchestrate:

- creation  
- review  
- reconciliation  
- projection (resolution ↔ item)  
- import/export  

---

## 11.2 CSG

Constructs graph from references.

---

## 11.3 CDS

Produces exploratory artifacts and Items.

---

## 11.4 CAS

Computes alignment over structure and signals.

---

## 11.5 CIS

Defines identity over structure.

---

## 11.6 CCare

Provides observational signals.

---

## 11.7 CSP

May emit CCS-compatible commits.

- does not require new commit types  
- may shape signals prior to emission  

---

## 11.8 CRS

Transports commits as opaque artifacts.

---

# 12. Imports and Foreign Commits

Foreign commits:

- remain non-legitimate  
- retain identity  
- require explicit review  

---

# 13. No Canonical State Rule

CCS does not define:

- current truth  
- active state  
- authority  

---

# 14. Invariants

- commits are immutable  
- append-only  
- identity is stable  
- integrity is verifiable  
- references are explicit  
- no inference  
- no authority implied  
- no canonical state  
- history is never rewritten  

---

# 15. What CCS Does NOT Do

CCS does not:

- create legitimacy  
- compute alignment  
- interpret meaning  
- enforce workflows  
- define identity truth  

---

# 16. Mental Model

A commit is:

- a durable envelope  
- a recorded artifact  
- a structural unit  

It is not:

- authority  
- decision logic  
- current state  

---

# 17. Final Principle

The Charter Commit System exists so that:

- artifacts can be recorded durably  
- identity remains stable  
- integrity remains verifiable  
- structure can evolve without rewriting history  
- multiple systems can interoperate without collapsing meaning  

CCS preserves recorded artifacts  
without deciding what they mean.