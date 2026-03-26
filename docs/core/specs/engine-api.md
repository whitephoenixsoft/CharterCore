# ENG-API — Engine Interface & Execution Boundary Specification

Status: REFACTORED (v16 – Explicit Command Surface & Candidate-Oriented Actions)  
Applies to: Engine Core (V1/V2+)  
Scope: Deterministic Engine interface, command surface, and runtime interaction boundary

Authority: Interface specification subordinate to ENG-INITIALIZATION and ENG-INTEGRITY.

Subordinate references consumed from:

- ENG-DOMAIN
- ENG-SESSION
- ENG-DECISION
- ENG-SUPERSESSION
- ENG-RECEIPT
- ENG-CANON
- ENG-ERROR
- ENG-SPECVERIFY
- ENG-INITIALIZATION
- ENG-IMPORT
- ENG-COMPILATION
- ENG-PERSISTENCE
- ENG-REVIEW-RETIRED

---

# 1. Purpose

ENG-API defines the deterministic interface between a host system and the Engine Core.

It is the authoritative specification for:

- command surface exposed by the Engine
- runtime interaction model
- mutation vs evaluation boundaries
- rehydration entry invocation
- incremental compilation invocation
- deterministic reporting guarantees

ENG-API exposes behavior defined elsewhere.  
It does not redefine it.

---

# 2. Core Principles

## ENG-API-01 — Interface, Not Authority

(All unchanged)

## ENG-API-02 — Single-Area Runtime Model

(All unchanged)

## ENG-API-03 — Deterministic Reporting

(All unchanged)

## ENG-API-04 — Identifier Ownership

(All unchanged)

## ENG-API-05 — No Implicit Legitimacy

(All unchanged)

---

# 3. Command Format

## ENG-API-06 — Standard Command Definition Format

All API commands must be defined using the following structure:

### `<command_name>`

**Purpose**  
Description of the command.

**Inputs**  
Required and optional inputs.

**Output**  
Deterministic result type (EvaluationReport, query result, runtime outcome, or exported graph).

**Mutation**  
Indicates whether the command mutates Engine state.

**High-level Preconditions**  
Minimal API-level preconditions.

**Behavioral Authority**  
Authoritative specifications governing behavior.

**Notes**  
Additional clarifications.

---

# 4. Runtime Entry

### rehydrate_engine

**Purpose**  
Rehydrate Engine runtime from a domain graph.

**Inputs**  
- domain_objects

**Output**  
- runtime_mode (NORMAL_RUNTIME | DEGRADED_READ_ONLY | INITIALIZATION_FAILURE)
- EvaluationReport

**Mutation**  
Yes (replaces runtime state)

**High-level Preconditions**  
- valid submission shape

**Behavioral Authority**  
- ENG-INITIALIZATION  
- ENG-INTEGRITY  
- ENG-SUPERSESSION  
- ENG-RECEIPT  
- ENG-CANON  
- ENG-SPECVERIFY  

---

# 5. Evaluation

### evaluate_session

**Purpose**  
Evaluate session eligibility without mutation.

**Inputs**  
- session_id

**Output**  
- EvaluationReport

**Mutation**  
No

**Behavioral Authority**  
- ENG-DECISION

**Notes**  
- includes solo-mode vote logic
- reflects candidate-level and session-level blocking

---

# 6. Session Lifecycle

### create_session

**Purpose**  
Create a new session.

**Inputs**  
- session_type (AUTHORITY | SCOPE | REGULAR)

**Output**  
- session_id  
- EvaluationReport

**Mutation**  
Yes

**Behavioral Authority**  
- ENG-SESSION  
- ENG-DECISION  
- ENG-INTEGRITY  

---

### resume_session

**Purpose**  
Advance session to next round.

**Inputs**  
- session_id

**Output**  
- EvaluationReport

**Mutation**  
Yes

**Behavioral Authority**  
- ENG-SESSION  

---

### close_session

**Purpose**  
Close session without acceptance.

**Inputs**  
- session_id

**Output**  
- receipt_id  
- EvaluationReport

**Mutation**  
Yes

**Behavioral Authority**  
- ENG-SESSION  
- ENG-RECEIPT  
- ENG-PERSISTENCE  

---

### attempt_acceptance

**Purpose**  
Attempt to accept the currently winning candidate.

**Inputs**  
- session_id

**Output**  
- resolution_id (if success)  
- receipt_id (if success)  
- EvaluationReport

**Mutation**  
Yes (only on success)

**Behavioral Authority**  
- ENG-DECISION  
- ENG-SESSION  
- ENG-SUPERSESSION  
- ENG-PERSISTENCE  
- ENG-RECEIPT  

**Notes**  
- candidate must win under authority rules  
- vacillation allowed prior to invocation  
- includes candidate-level blocking checks  
- includes session-level blocking checks  

---

# 7. Session Mutation (PRE_STANCE Only)

### add_participant

### add_candidate

**Purpose**  
Add candidate proposal.

**Inputs**  
- session_id  
- candidate_content

**Output**  
- candidate_id  
- EvaluationReport

**Mutation**  
Yes

**Behavioral Authority**  
- ENG-SESSION  

**Notes**  
Candidate content may represent:

- new proposal  
- supersede resolution(s)  
- retire resolution  
- governance change  

---

### add_constraint

### cast_vote

**Purpose**  
Record vote.

**Inputs**  
- session_id  
- participant_id  
- candidate_id  
- stance

**Output**  
- vote_id  
- EvaluationReport

**Mutation**  
Yes

**Behavioral Authority**  
- ENG-SESSION  
- ENG-DECISION  

**Notes**  
- votes may change prior to acceptance  
- one vote per participant per candidate per round  

---

# 8. Incremental Compilation

### begin_incremental_compilation  
### add_incremental_batch  
### end_incremental_compilation  

(All structured identically; behavior unchanged)

---

# 9. Administrative Operations

### set_resolution_under_review  
### restore_resolution_active  

(All structured identically; governed by ENG-REVIEW-RETIRED)

---

# 10. Queries

### list_sessions  
### list_resolutions  

### get_session_state  
### get_resolution_state  

### get_session_receipt  
### list_session_receipts  

### list_session_rounds

**Purpose**  
Return deterministic historical rounds.

**Inputs**  
- session_id

**Output**  
- ordered round snapshots

**Mutation**  
No

**Behavioral Authority**  
- ENG-RECEIPT  
- ENG-DOMAIN  

**Notes**  
- supports user reconstruction of prior session configurations  
- does not affect legitimacy  

---

# 11. Specification Identity

### get_spec_set_hash  
### verify_spec_hash  

---

# 12. Export

### export_area_dag

---

# 13. Blocking Model (Interface-Level Clarification)

## ENG-API-07 — Dual-Level Blocking Model

Blocking exists at two levels:

### Session-Level Blocking

Caused by:

- Authority invalidation → BLOCK_PERMANENT  
- Scope supersession or invalidation → BLOCK_PERMANENT  
- Scope UNDER_REVIEW → BLOCK_TEMPORARY  

These invalidate the session as a whole.

### Candidate-Level Blocking

Caused by:

- superseded target  
- retired target  
- UNDER_REVIEW target  
- unusable referenced artifacts  

These invalidate individual candidates.

ENG-API exposes both through EvaluationReport.  
It does not define their semantics.

---

# 14. Degraded Mode

(unchanged)

---

# 15. Determinism Guarantees

(unchanged)

---

# 16. Engine Invariants

- API never creates legitimacy directly  
- API never bypasses validation  
- API never mutates state on failed validation  
- API exposes both session-level and candidate-level blocking  
- API reflects authoritative evaluation without reinterpretation  
- API never merges Areas  
- API never reuses identifiers  

---

# 17. Mental Model

ENG-API is the execution surface.

It exposes:

- session orchestration  
- candidate-based decision making  
- deterministic evaluation  
- immutable historical inspection  

It does not define:

- legitimacy  
- graph semantics  
- usability semantics  
- structural validity  

Those belong to their respective specifications.