# Charter — Canonical Naming Specification

Status: FOUNDATIONAL (LOCKED)  
Applies to: All Charter layers, specifications, libraries, and documentation  
Purpose: Establish stable, non-ambiguous naming for Charter architecture independent of versioning  

---

# 1. Purpose

This document defines the canonical naming system for Charter.

It exists to:

- eliminate ambiguity across evolving versions (V1–V7+)
- prevent architectural drift caused by renaming
- decouple version history from system structure
- ensure consistent terminology across:
  - specifications
  - codebases
  - CLI
  - documentation
  - future integrations

---

# 2. Core Rule

## CN-01 — Architecture Names Are Stable

Architecture names describe what the system is.  
They MUST remain stable across versions.

---

## CN-02 — Version Numbers Are Historical

Version numbers (V1, V2, …) describe when capabilities were introduced, not what they are.

They MUST NOT be used as architectural identifiers.

---

## CN-03 — No Cross-Substitution

Architecture names and version numbers MUST NOT be used interchangeably.

Example (invalid):

- “V6 handles commits”

Example (valid):

- “The Commit System handles commits (introduced in V6)”

---

# 3. Canonical Architecture Layers

The following names are authoritative and permanent.

---

## 3.1 Legitimacy Engine

Canonical Name: Legitimacy Engine  

Responsibility:
- Sessions
- Authority evaluation
- Candidate evaluation
- Resolution creation
- Legitimacy receipts

Key Property:
- Only component that creates legitimacy

Notes:
- Derived from V1/V2 concepts
- Fully deterministic
- Storage-agnostic

---

## 3.2 Persistence Layer

Canonical Name: Persistence Layer  

Responsibility:
- Object store (immutable)
- Ref store
- Audit store
- Commit storage (when enabled)
- Hashing and integrity

Key Property:
- Append-only truth ledger

---

## 3.3 Runtime Layer

Canonical Name: Runtime Layer (CLI Runtime)

Responsibility:
- Context isolation
- Session orchestration
- Baseline review
- Import/export coordination
- Engine invocation

Key Property:
- Orchestrates workflow but does not create legitimacy

---

## 3.4 Charter Commit System (CCS)

Canonical Name: Charter Commit System (CCS)

Responsibility:
- Append-only commit artifacts
- Commit taxonomy (Resolution, Receipt, Review, Exploration, etc.)
- Artifact identity (UUID-based)
- Commit-level lineage

Key Property:
- Stores immutable artifacts independent of canonical state

Notes:
- Previously referred to as “V6”
- Must never be called by version number

---

## 3.5 Charter Alignment System (CAS)

Canonical Name: Charter Alignment System (CAS)

Responsibility:
- VLS (structural graph)
- VDS (signals / check-ins)
- Alignment Engine (derived computation)

Key Property:
- Observational and predictive only
- Does not create legitimacy

---

## 3.6 Charter Guidance Layer (CGL)

Canonical Name: Charter Guidance Layer (CGL)

Responsibility:
- AI-assisted interpretation
- Summaries
- Drift detection
- Reflection support

Key Property:
- Read-only, non-legitimizing

---

## 3.7 Charter Relay System (CRS)

Canonical Name: Charter Relay System (CRS)

Responsibility:
- Transport of commit artifacts
- Append-only archival endpoints
- Push/fetch operations

Key Property:
- Opaque transport layer
- No interpretation, no legitimacy

---

# 4. Cross-Layer Relationships

## CN-04 — Directionality

Dependencies flow upward:

Legitimacy Engine  
↑  
Persistence Layer  
↑  
Runtime Layer  
↑  
CCS / CAS / CGL / CRS  

Lower layers must never depend on higher layers.

---

## CN-05 — Separation of Concerns

- Legitimacy Engine → creates legitimacy  
- Runtime → orchestrates workflows  
- CCS → stores artifacts  
- CAS → computes alignment  
- CGL → interprets  
- CRS → transports  

No layer may assume another layer’s responsibility.

---

# 5. Mapping Legacy Version Terms

For historical reference only:

| Legacy Version | Canonical Layer |
|----------------|----------------|
| V1–V2 | Legitimacy Engine |
| V1–V4 | Runtime Layer |
| V5 (AI) | Charter Guidance Layer (CGL) |
| V6 (commits) | Charter Commit System (CCS) |
| V6+ (alignment) | Charter Alignment System (CAS) |
| V7 | Charter Relay System (CRS) |

This table is informational only.  
Future documents MUST use canonical names.

---

# 6. Naming Rules

## CN-06 — Use Canonical Names in All Specs

All new documents MUST use:

- Legitimacy Engine  
- Runtime Layer  
- Persistence Layer  
- CCS  
- CAS  
- CGL  
- CRS  

---

## CN-07 — Avoid Abbreviations in First Use

First reference in any document must use full name:

Example:

“Charter Commit System (CCS)”

Subsequent references may use abbreviation.

---

## CN-08 — Do Not Reintroduce Version Labels

Prohibited patterns:

- “V6 commits”
- “V5 alignment”
- “V7 relay layer”

Allowed pattern:

- “CCS (introduced in V6)”

---

# 7. Mental Model

Charter is a layered system composed of:

- a legitimacy kernel
- a storage substrate
- a workflow orchestrator
- multiple non-authoritative observational and transport systems

Each layer is:

- independent
- replaceable
- auditable

Version numbers describe evolution.  
Canonical names describe structure.

---

# 8. Final Principle

If naming becomes ambiguous, architecture will drift.

This document exists to ensure:

- clarity across time
- consistency across implementations
- survivability across forks

Canonical names are part of Charter’s long-term integrity.