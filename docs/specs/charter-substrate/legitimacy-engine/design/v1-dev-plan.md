# Charter Legitimacy Engine — V1 Development & Testing Milestone Plan

Status: ACTIVE  
Purpose: Define phased implementation and testing strategy for the Charter Legitimacy Engine (V1)  
Scope: Engine core (compiler + runtime), evaluation, acceptance, spec verification, and CLI dogfooding  

---

# Guiding Principle

Testing is introduced **when a layer becomes semantically stable**, not before and not only at the end.

Each milestone introduces:
- implementation scope
- corresponding test scope
- exit criteria (gate)

---

# Milestone 0 — Workspace & Crate Skeleton

## Implementation Scope
- Cargo workspace created
- `charter-legitimacy` library crate
- `charter-cli` binary crate (`charter`)
- Spec manifest stub exposed
- Minimal public API surface

## Tests
- Crate compiles
- CLI links to library
- Manifest accessible from CLI

## Gate A — Skeleton Stability
- Workspace builds cleanly
- CLI executes successfully
- No unresolved dependencies

---

# Milestone 1 — V1 Type System

## Implementation Scope
- Domain objects (ENG-DOMAIN)
- ID types
- Enums (structural + session + receipt)
- `CompiledState` (initial form)
- `EvaluationReport`
- `RuntimeMode`

## Tests (Unit Tests Begin Here)
- ReceiptBody correctness:
  - Legitimacy → always has resolution_id
  - Exploration → never has resolution_id
- CandidatePayload:
  - action_type() returns correct enum
- ID types:
  - ordering and equality stable
- CompiledState basics:
  - deterministic insertion
  - index sorting/dedup works

## Gate B — Type Stability
- All core types compile
- Illegal states are unrepresentable
- Basic deterministic behavior verified

---

# Milestone 2 — Rehydration & Compilation Core

## Implementation Scope
- `rehydrate_engine`
- Graph ingestion
- Structural index construction
- Governance index construction
- Receipt indexing
- Runtime mode selection (basic)

## Tests
- Single-area valid graph succeeds
- Missing references fail correctly
- Multi-area input rejected
- Deterministic reconstruction:
  - same input → identical compiled state
- Receipt/session linkage consistency
- Rehydration replaces prior state fully

## Gate C — Compilation Stability
- Compilation is deterministic
- Failures are fail-closed (no partial state)
- Indexes match graph truth

---

# Milestone 3 — Evaluation Core (First Pure Slice)

## Implementation Scope
- `evaluate_session`
- Candidate disposition derivation
- Session-level blocking
- `get_candidate_status`
- `list_session_candidates`

## Tests (Heavy Focus)
- No candidates → REJECTED
- Invalid candidate → INVALID
- ON_HOLD target → BLOCK_TEMPORARY
- RETIRED target → BLOCK_PERMANENT
- Superseded target → BLOCK_PERMANENT
- Deterministic evaluation ordering
- Evaluation is:
  - side-effect free
  - idempotent

## Gate D — Evaluation Stability
- Evaluation is pure
- Candidate status fully derived
- Reports are deterministic and complete

---

# Milestone 4 — Session Mutation Commands

## Implementation Scope
- Create session
- Add/remove participant
- Add/remove candidate
- Add/remove constraint
- Cast vote / clear vote
- Annotation updates
- Freeze boundary enforcement
- Resume behavior

## Tests
- PRE_STANCE mutations allowed
- First vote freezes round
- Frozen mutations rejected
- Vote replacement (vacillation)
- Resume:
  - resets round state
  - increments round index
- Terminal sessions reject mutation

## Gate E — Command Stability
- All commands enforce invariants
- No mutation leaks across phases
- Round lifecycle is deterministic

---

# Milestone 5 — Acceptance Orchestration

## Implementation Scope
- Winning candidate determination
- Acceptance eligibility rules
- Mutation planning (no persistence yet)
- Receipt construction

## Tests
- One eligible winner → SUCCESS
- No eligible candidates → REJECTED
- Multiple winners → REJECTED (deterministic)
- LEGITIMACY receipt:
  - always has resolution_id
- EXPLORATION receipt:
  - always null resolution_id
- No partial mutation on failure

## Gate F — Acceptance Stability
- Acceptance is atomic (conceptually)
- Receipt structure correct and complete
- No illegitimate state transitions

---

# Milestone 6 — Canonicalization & SpecVerify

## Implementation Scope
- Canonical JSON encoding (ENG-CANON)
- Deterministic hashing
- Build-time spec embedding
- Spec manifest generation
- `spec_set_hash`
- Provenance validation

## Tests (Golden Tests Introduced)
- Field ordering deterministic
- Arrays preserve order
- Explicit null preserved
- Identical inputs → identical bytes
- Spec change → hash change
- Manifest ordering stable
- Spec hash reproducibility

## Gate G — Provenance Stability
- Canonicalization deterministic
- Spec identity immutable per build
- Receipt hashing reproducible

---

# Milestone 7 — CLI Dogfooding

## Implementation Scope
- CLI commands mapped to engine API
- Rehydrate-per-command workflow
- Graph export/import
- Basic UX flows

## Tests (Integration Focus)
- CLI commands invoke engine correctly
- End-to-end governance flow works:
  - create session
  - add candidate
  - vote
  - accept
- Output stable enough for users

## Gate H — Usability Stability
- CLI usable for real workflows
- No engine invariant violations via CLI
- End-to-end flows deterministic

---

# Test Strategy Summary

## Early Phases
- Compile tests
- Basic unit tests

## Middle Phases
- Heavy unit tests
- Focused integration tests

## Late Phases
- Acceptance tests
- Golden tests (hashing, canon, spec)
- CLI integration tests

---

# Directory Suggestion

tests/
- types.rs
- rehydrate.rs
- evaluate.rs
- commands.rs
- acceptance.rs
- canon.rs
- specverify.rs
- cli_integration.rs

---

# Final Principle

The engine must always remain:

- deterministic
- explicit
- non-inferential
- append-only (historically)
- reproducible from artifacts alone

Tests are not validation of behavior only —  
they are enforcement of these invariants.
