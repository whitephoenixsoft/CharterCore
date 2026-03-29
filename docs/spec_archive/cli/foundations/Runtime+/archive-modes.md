# Charter — Archival Modes (Conceptual Extension)

Status: FOUNDATIONAL (Conceptual Extension)  
Extends: Charter — Archival Model  
Applies to: CLI Runtime, Archival Workflows  
Does NOT apply to: Engine legitimacy semantics  

---

## 1. Purpose

This document defines **archival modes**, which determine how artifact sets are selected and packaged during archival.

Archival modes control:

- dependency inclusion
- completeness guarantees
- portability characteristics
- reconstruction capability

They do not affect:

- legitimacy
- identity
- provenance
- interpretation

---

## 2. Core Principle

> Archival modes control completeness, not meaning.

All modes:

- preserve identity
- preserve hashes
- preserve lineage
- preserve provenance

They differ only in **how much context is included**.

---

## 3. Mode Overview

Charter defines three conceptual archival modes:

1. **MINIMAL**
2. **CLOSURE**
3. **NARRATIVE**

Each mode represents a different tradeoff between:

- size
- independence
- interpretability

---

## 4. MINIMAL Mode

### Definition

MINIMAL mode archives only the explicitly selected artifacts.

No additional dependency expansion is performed.

---

### Properties

- smallest possible archive
- fastest to produce
- may contain unresolved references
- not self-sufficient

---

### Behavior

- includes only selected objects
- preserves all internal references as-is
- does not resolve or include dependencies
- external references remain external

---

### Use Cases

- quick offloading of data
- temporary storage reduction
- relay-based redundancy
- intermediate archival steps

---

### Risks

- incomplete interpretability
- requires external context to understand
- fragile if dependencies are lost

---

## 5. CLOSURE Mode

### Definition

CLOSURE mode archives a **dependency-complete subgraph**.

All required artifacts for interpretation are included.

---

### Properties

- self-contained
- fully reconstructible
- stable over time
- larger than minimal

---

### Behavior

- recursively includes referenced artifacts
- ensures no required references are missing
- produces an independent archival unit

---

### Closure Scope

Closure MUST include:

- referenced resolutions
- referenced sessions
- referenced receipts
- authority and scope lineage required for interpretation

Closure MUST NOT:

- include unrelated artifacts
- expand beyond required dependency graph

---

### Use Cases

- long-term archival
- legal or institutional recordkeeping
- independent transfer between systems
- cold storage

---

### Guarantees

A CLOSURE archive can be:

- restored independently
- audited without external dependencies
- verified deterministically

---

## 6. NARRATIVE Mode

### Definition

NARRATIVE mode extends closure by preserving **contextual understanding**.

It includes artifacts that explain *how and why* decisions evolved.

---

### Properties

- maximizes interpretability
- preserves exploration history
- preserves discarded paths
- largest archive size

---

### Behavior

Includes everything in CLOSURE mode, plus:

- related deliberates
- breakouts
- synthesis artifacts
- baseline review artifacts
- rejected and abandoned proposals
- relevant annotations

Selection may be:

- lineage-driven
- time-bound
- user-scoped

---

### Use Cases

- institutional memory
- research reproducibility
- post-mortem analysis
- knowledge preservation

---

### Tradeoffs

- increased size
- more complex selection logic
- may include non-essential artifacts

---

## 7. Mode Comparison

| Property            | MINIMAL | CLOSURE | NARRATIVE |
|--------------------|--------|---------|-----------|
| Size               | Small  | Medium  | Large     |
| Self-contained     | No     | Yes     | Yes       |
| Dependency-safe    | No     | Yes     | Yes       |
| Explains decisions | No     | Partial | Yes       |
| Includes workflow  | No     | Minimal | Extensive |

---

## 8. Mode Selection Principles

Mode selection must be:

- explicit
- visible to the user
- auditable

The system must never:

- silently upgrade or downgrade modes
- infer completeness guarantees
- assume intent

---

## 9. Default Recommendations

For most systems:

- MINIMAL → temporary or relay usage  
- CLOSURE → default archival mode  
- NARRATIVE → optional, high-value archival  

---

## 10. Interaction with CCE

- CLOSURE and NARRATIVE modes SHOULD produce valid CCE exports  
- MINIMAL mode MAY produce partial exports  
- All modes must preserve:
  - spec_set_hash  
  - engine_version  
  - object hashes  

---

## 11. Interaction with Relay

Relay treats all modes uniformly:

- no interpretation of completeness  
- no validation of closure  
- no differentiation between modes  

Mode semantics exist only at the CLI/runtime layer.

---

## 12. Invariants

- Mode must not affect legitimacy  
- Mode must not alter identity  
- Mode must not alter provenance  
- Mode must not rewrite history  
- Mode must not infer missing data  

---

## 13. Mental Model

- MINIMAL → “take exactly what I selected”  
- CLOSURE → “take everything needed to understand it”  
- NARRATIVE → “take everything needed to explain it”  

---

## 14. Final Principle

Archival completeness is a choice.

Meaning is not.