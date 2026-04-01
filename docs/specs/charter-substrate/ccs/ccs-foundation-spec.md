# Charter Commit System (CCS) — Foundation Specification

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

A commit records that something was declared, observed, packaged, or preserved.

A commit does not determine:

- whether the artifact is correct
- whether it is legitimate
- whether it is aligned
- whether it should be acted upon
- whether it is current

Meaning emerges only through interpretation by systems outside CCS.

---

# 3. The Commit Primitive

A **commit** is:

- immutable
- append-only
- independently identifiable
- structurally self-describing
- optionally related to other commits through explicit references

A commit is an **artifact envelope**, not mutable state.

---

## 3.1 Required Properties

All commits must satisfy the following:

### Immutability
Once created, a commit must never be modified.

### Append-Only Existence
Commits may be added, copied, archived, transported, or removed from a local store under explicit lifecycle rules, but the commit itself must never be altered in place.

### Stable Identity
Each commit has a globally unique `commit_id`.

### Deterministic Integrity
Each commit includes an `integrity_hash` computed over the canonical commit envelope.

### Structural Self-Description
Each commit declares its own `commit_type` and schema version explicitly.

### Explicit Relationships Only
Any relationship to another commit must be declared explicitly. No relationship may be inferred from time, storage order, co-location, or naming.

---

## 3.2 Non-Properties

A commit is not:

- a source of authority
- a unit of legitimacy by default
- a computed state
- a mutable object
- a current-state projection
- a database row
- a guarantee of relational completeness

---

# 4. Identity and Integrity

CCS distinguishes strictly between **identity** and **integrity**.

---

## 4.1 Commit Identity

`commit_id` defines commit identity.

Requirements:

- identity must be globally unique
- identity must remain stable across storage, export, relay, and import
- identity must not be derived from storage location
- identity must not be replaced by content hash

---

## 4.2 Commit Integrity

`integrity_hash` defines commit integrity.

Requirements:

- integrity hash must be deterministic
- integrity hash must validate the canonical commit envelope
- hash failure indicates corruption or mismatch, not identity change

---

## 4.3 Nested Integrity

A commit payload may contain artifacts that have their own integrity models, including hashes defined by other Charter modules.

Examples:

- runtime object hashes
- export bundle hashes
- embedded provenance hashes

These nested integrity fields:

- do not replace `commit_id`
- do not replace `integrity_hash`
- do not redefine commit identity

---

# 5. Commit Envelope

All commits share a common structural envelope.

Minimum required fields:

- `commit_id`
- `commit_type`
- `commit_schema_version`
- `created_at`
- `referenced_area_ids`
- `related_commit_ids` (optional)
- `integrity_hash`
- `payload`

Additional fields are defined by commit type.

---

## 5.1 Envelope Principle

The commit envelope is the structural boundary of CCS.

It exists to provide:

- identity
- integrity
- type declaration
- minimal structural relationship data
- transport and storage neutrality

The payload carries domain-specific meaning.

---

## 5.2 Canonical Integrity Scope

The `integrity_hash` must cover the canonical commit envelope according to the commit hashing rules defined for CCS.

Those rules must be:

- deterministic
- versioned
- stable per schema version

---

# 6. Commit Types

All semantics in CCS are carried through **commit types**.

CCS defines structural rules for commit typing, not full behavioral meaning.

---

## 6.1 Resolution Commit

Represents an accepted decision artifact.

Properties:

- may act as a legitimacy-bearing artifact when produced under the Legitimacy Engine and Runtime rules
- may be consumed by downstream systems as a decision anchor
- does not create legitimacy by existing alone

---

## 6.2 Deliberate Commit

Represents exploratory, drafting, or pre-legitimacy artifacts.

Properties:

- non-authoritative
- non-legitimizing
- preserves exploratory work without implying acceptance

---

## 6.3 Import Commit

Represents the preservation of foreign artifacts or foreign commit-derived content.

Properties:

- preserves provenance
- does not confer local legitimacy automatically
- must not silently normalize foreign meaning

---

## 6.4 Annotation Commit

Represents contextual, explanatory, or documentary material.

Properties:

- append-only
- does not modify prior commits
- may provide rationale, commentary, or explanatory attachment

---

## 6.5 Extension Principle

Future commit types must:

- extend the common envelope
- declare schema version explicitly
- preserve CCS invariants
- avoid parallel artifact mechanisms outside CCS

---

# 7. Commit Type Schema Authority

Each `commit_type` defines its own payload schema.

The schema for a commit type must specify:

- required fields
- optional fields
- canonical form for integrity purposes
- compatibility and versioning rules

CCS is responsible for structural commit validity, not full domain semantics.

---

# 8. Commit Versioning

CCS must support explicit schema versioning.

---

## 8.1 Schema Version Requirement

Every commit must declare `commit_schema_version`.

This version identifies the structural schema of the commit envelope and payload for that commit type.

---

## 8.2 Version Stability

A schema version, once published, must be immutable.

Changes to structure require:

- a new schema version
- explicit compatibility handling
- no retroactive reinterpretation of prior commits

---

## 8.3 Version Independence from Identity

Schema changes must not mutate existing `commit_id` values.

Identity remains stable even when schema families evolve.

---

# 9. Structural Relationships

Commits may optionally declare relationships to other commits.

These relationships are **structural**, not semantic, at the CCS layer.

---

## 9.1 Related Commit IDs

`related_commit_ids` may identify commits that are structurally related to the current commit.

Examples may include:

- attachment relationships
- packaging relationships
- preservation relationships
- explicit follow-on relationships

CCS itself assigns no semantic meaning to these references.

---

## 9.2 No Inference Rule

If a relationship is not explicitly declared, CCS must not infer it from:

- timestamps
- area overlap
- similar payloads
- storage order
- import source
- relay sequence

---

## 9.3 Relationship Neutrality

A structural reference does not imply:

- legitimacy
- authority
- causality
- supersession
- dependency
- alignment
- completeness

Such meanings, if they exist, belong to payload semantics or higher layers.

---

# 10. Commit Independence and Graph Shape

Commits are independently valid artifacts.

The commit graph is therefore:

- optional
- additive
- explicit
- non-authoritative at the CCS layer

Disconnected commit sets are valid.

CCS does not require a complete graph.

CCS does not guarantee relational completeness.

---

# 11. Relationship to Other Charter Modules

CCS is a shared substrate used by multiple independent modules.

---

## 11.1 Legitimacy Engine

The Legitimacy Engine may produce legitimacy-bearing artifacts packaged as commits.

The Legitimacy Engine defines legitimacy semantics. CCS only provides the artifact envelope.

---

## 11.2 Runtime Layer

The Runtime Layer may orchestrate commit creation, import, review, archival, and other workflows.

The Runtime Layer does not redefine CCS structure.

---

## 11.3 Commit Store

The Commit Store persists commits, indexes them, archives them, and manages local lifecycle operations.

The Commit Store does not define commit meaning.

---

## 11.4 Charter Care Substrate (CCare)

CCare may package check-ins, requests, silence records, and supportability artifacts inside commits.

CCS does not interpret caregiving meaning.

---

## 11.5 Charter Lineage Substrate (CLL)

CLL may package identity, scope, purpose, deprecation, and version lineage artifacts inside commits.

CCS does not interpret identity meaning.

---

## 11.6 Charter Alignment System (CAS)

CAS consumes commits and related substrate outputs to compute derived alignment state.

CCS does not compute alignment.

---

## 11.7 Charter Guidance Layer (CGL)

CGL interprets commits and derived state for descriptive explanation.

CCS does not interpret meaning.

---

## 11.8 Charter Relay System (CRS)

CRS transports commits as opaque artifacts.

CCS does not define transport policy.

---

# 12. Imports and Foreign Commits

Commits originating outside a local system are considered **foreign** until accepted or interpreted by appropriate higher layers.

Properties:

- preserved with original identity
- preserved with original integrity where applicable
- not legitimized automatically
- not normalized silently

Import handling is external to CCS.

---

# 13. No Canonical State Rule

CCS does not define canonical, active, or current state.

Any notion of:

- current authority
- active scope
- latest valid identity
- current alignment
- active area head

belongs to other modules.

CCS preserves artifacts only.

---

# 14. Invariants

The following must always hold:

- commits are immutable
- commits are append-only as artifacts
- commit identity is stable
- commit integrity is deterministic and verifiable
- identity and integrity are distinct
- all relationships are explicit
- no relationships are inferred
- commits do not interpret themselves
- commits do not create authority implicitly
- commits do not define canonical state
- commit history is never rewritten
- disconnected commit graphs are valid
- relational completeness is not assumed

Violation of these invariants invalidates CCS.

---

# 15. What CCS Does NOT Do

CCS does not:

- store commits
- orchestrate workflows
- create legitimacy
- compute alignment
- define identity truth
- define caregiving truth
- interpret meaning
- transport commits
- guarantee graph completeness
- infer missing relationships

CCS defines the artifact envelope, not the artifact lifecycle or its interpretation.

---

# 16. Mental Model

A commit is:

- a durable envelope
- a recorded artifact
- a stable identity
- a verifiable structure
- an optional node in a larger explicit graph

A commit is not:

- a decision engine
- a legitimacy engine
- a current-state store
- a semantic graph engine
- a source of authority by itself

---

# 17. Final Principle

The Charter Commit System exists so that:

- artifacts can be recorded durably
- identity remains stable
- integrity remains verifiable
- meaning may evolve without rewriting history
- independent systems may exchange artifacts without collapsing semantics

CCS preserves recorded artifacts without deciding what they mean.