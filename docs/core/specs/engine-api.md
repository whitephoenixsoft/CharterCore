# ENG-API — Minimal Engine Interface Specification

Status: FROZEN (v14 – Rehydration, Incremental Compilation & Spec Verification Alignment)  
Applies to: Engine Core (V1/V2+)  
Scope: Deterministic, storage-agnostic engine interface

Authority: Subordinate to ENG-DOMAIN, ENG-ERROR, ENG-CANON, ENG-SESSION, ENG-DECISION, ENG-SUPERSESSION, ENG-RECEIPT, ENG-INTEGRITY, ENG-SPECVERIFY

---

# 1. Purpose

ENG-API defines the minimal deterministic interface between a host and the Engine Core.

The Engine:

- Operates on canonical domain objects only  
- Loads exactly one Area at a time  
- Generates all UUIDv7 identifiers internally  
- Emits structured EvaluationReports per command  
- Emits audit events append-only  
- Emits immutable session receipts at closure  
- Enforces governance slot invariants  
- Enforces participant epoch semantics  
- Never manages storage, indexing, or external orchestration  
- Does not evaluate foreign DAGs

Legitimacy is created only via explicit session acceptance.  
Receipts formalize closure but do not create legitimacy.  
Bootstrap occurs via standard AUTHORITY sessions; no special bootstrap APIs exist.

---

# 2. Global Runtime Rules

## ENG-API-00 — Single-Area Runtime Model

- Engine instance operates on exactly one Area at a time  
- May not host multiple Areas simultaneously  
- May not evaluate legitimacy across Areas  
- May not retain legitimacy state across rehydration  

Area switching is host-managed via `rehydrate_engine`.

---

## ENG-API-01 — Deterministic Command Reporting

- Every mutating call emits exactly one EvaluationReport (ENG-ERROR)  
- Report serialization follows ENG-CANON  
- Read-only calls produce deterministic snapshots  

The Engine must not:

- Throw semantic exceptions  
- Return booleans for legitimacy  
- Encode meaning in narrative strings  

Clients must:

- Consume ordered `errors` list  
- Treat `primary_error_code` as optional convenience only  
- Not assume first-error-wins semantics

---

## ENG-API-02 — UUID Ownership

All identifiers are UUIDv7 generated internally:

- session_id  
- participant_id  
- candidate_id  
- resolution_id  
- evaluation_id  
- vote_id  
- receipt_id  

Callers may reference but not generate IDs.  
IDs are valid only in the current Area.

---

## ENG-API-03 — No Implicit Legitimacy

No API may:

- Create legitimacy outside acceptance  
- Supersede resolutions directly  
- Mutate immutable objects  
- Auto-close sessions  
- Bypass governance preconditions  
- Evaluate un-rehydrated graphs  
- Reuse participant_id values

Supersession occurs only via acceptance.  
Receipts never create legitimacy.

---

# 3. Engine Rehydration

## ENG-API-04 — rehydrate_engine

Load a complete domain graph for exactly one Area.

Inputs:

- sessions  
- resolutions  
- receipts  
- annotations

Behavior:

- Replaces prior Area state  
- Executes full structural validation (ENG-INTEGRITY)  
- Accumulates all violations  
- Does not short-circuit  
- Validates receipt integrity and canonical serialization  
- Derives ACTIVE resolution sets

Outcome:

- No violations → Engine enters normal runtime  
- Structural violations → initialization fails with deterministic errors  
- Non-fatal integrity issues → degraded read-only mode

No API may evaluate domain graphs without successful rehydration.

---

# 4. Read-Only Evaluation

## ENG-API-05 — evaluate_session

Inputs:

- session_id

Properties:

- Pure, idempotent, deterministic  
- Non-mutating  
- Executes full validation  
- Accumulates all violations

Must not:

- Mutate session state  
- Emit receipts  
- Emit audit events  
- Insert implicit votes  
- Trigger lifecycle transitions

Returns:

- EvaluationReport with ordered `errors`  
- Optional `primary_error_code`  
- Deterministic outcome

Repeated calls on identical state produce identical results.  
Evaluation is simulation only; acceptance is transactional.

---

# 5. Session Lifecycle Commands

All lifecycle mutation commands:

- Enforce phase invariants  
- Enforce governance invariants  
- Enforce participant epoch invariants  
- Execute full validation  
- Accumulate violations deterministically  
- Derive outcome after full validation  
- Must not mutate state on invariant failure

---

## ENG-API-06 — create_session

Inputs:

- session_type (AUTHORITY | SCOPE | REGULAR)  
- participants (initial display_name set, optional)  
- annotations (optional)

Behavior:

- Generates session_id  
- Generates participant_id for each display_name  
- Enforces display_name uniqueness  
- Enforces governance prerequisites per session type

---

## ENG-API-07 — pause_session

Transitions:

ACTIVE → PAUSED

---

## ENG-API-08 — resume_session

Transitions:

PAUSED → PRE_STANCE  
BLOCK_TEMPORARY → PRE_STANCE

Behavior:

- Terminates all participation epochs  
- Clears participant, candidate, constraint, and vote sets  
- Increments round_index  
- Participants re-added with new participant_id values

---

## ENG-API-09 — close_session

Transitions:

ACTIVE | PAUSED | BLOCK_PERMANENT → CLOSED

Emits EXPLORATION receipt.

---

## ENG-API-10 — add_participant

Allowed only in PRE_STANCE.  
Generates new participant_id.  
display_name unique per round.

---

## ENG-API-11 — remove_participant

Allowed only in PRE_STANCE.  
Terminates epoch permanently.

---

## ENG-API-12 — add_candidate

Allowed only in PRE_STANCE.  
Generates candidate_id unique per round.

---

## ENG-API-13 — remove_candidate

Allowed only in PRE_STANCE.  
Prohibited after VOTING begins.

---

## ENG-API-14 — record_vote

Allowed only in VOTING phase.

Rules:

- participant_id and candidate_id must belong to current round  
- One vote per participant per candidate

---

## ENG-API-15 — attempt_acceptance

Inputs:

- session_id

Behavior:

- Executes full deterministic validation  
- Accumulates all violations  
- Does not short-circuit

On success:

- Freeze participant, candidate, constraint, and vote snapshots  
- Create Resolution  
- Transition session → ACCEPTED  
- Emit LEGITIMACY receipt  
- Apply supersession

On failure:

- No mutation  
- Ordered errors returned

Acceptance is atomic.

---

# 5a. Incremental Compilation APIs

## ENG-API-23 — begin_incremental_compilation

Inputs:

- area_id (required)  
- historical_sessions (optional, list)  
- historical_resolutions (optional, list)  
- historical_receipts (optional, list)

Behavior:

- Pauses runtime  
- Initializes incremental DAG batch  
- Historical objects required for legitimacy; missing objects → validation error

Outcome:

- Engine ready for incremental batch  
- Runtime suspended

---

## ENG-API-24 — add_incremental_batch

Inputs:

- sessions, resolutions, receipts, annotations (optional)

Behavior:

- Validates historical consistency, legitimacy, schema compliance  
- Deterministic  
- No commit until `end_incremental_compilation`  
- Mixed-area or incomplete references → StructuralIntegrityFailure

---

## ENG-API-25 — end_incremental_compilation

Behavior:

- Full structural validation (ENG-INTEGRITY)  
- Derives ACTIVE sets  
- Applies supersession  
- Commits batch atomically  
- Resumes runtime  
- Any failure aborts batch; no partial commit

---

# 5b. Administrative / Under Review APIs

## ENG-API-26 — set_resolution_under_review

Inputs:

- resolution_id (required)  
- admin_actor (optional)

Behavior:

- Transition ACTIVE → UNDER_REVIEW  
- Only Engine-administered  
- Supersession and legitimacy paused  
- Emits EvaluationReport and audit

---

## ENG-API-27 — restore_resolution_active

Inputs:

- resolution_id (required)  
- admin_actor (optional)

Behavior:

- Transition UNDER_REVIEW → ACTIVE  
- Resumes normal governance and supersession  
- Emits EvaluationReport and audit

---

# 6. Read-Only Queries

## ENG-API-16 — list_sessions  
## ENG-API-17 — list_resolutions  
## ENG-API-18 — get_session_state  
## ENG-API-19 — get_resolution_state  
## ENG-API-20 — get_session_receipt  
## ENG-API-21 — list_session_receipts  

Receipt queries return:

- content_hash  
- hash_algorithm  
- spec_set_hash  
- engine_version

---

# 6a. Spec Verification APIs

## ENG-API-28 — get_spec_set_hash

- Returns current Engine spec_set_hash  
- Read-only, deterministic

---

## ENG-API-29 — verify_spec_hash

Inputs:

- incoming_spec_set_hash (required)  
- allow_legacy (optional, default=false)

Outputs:

- verification_result (MATCH | LEGACY_MATCH | SPEC_SET_UNKNOWN)  
- current_spec_set_hash  
- supported_legacy_spec_set_hashes (optional)

Behavior:

- Compares incoming_spec_set_hash to current and historical sets (if allow_legacy)  
- Deterministic, read-only  
- Does not mutate state  
- SPEC_SET_UNKNOWN → divergence; no silent reinterpretation

---

# 7. DAG Export

## ENG-API-22 — export_area_dag

Returns deterministic Area DAG including:

- sessions  
- resolutions  
- receipts  
- annotations

Export suitable for rehydration.

---

# 8. Degraded Read-Only Mode

Mutating commands fail with:

- DEGRADED_MODE_ACTIVE

Allowed operations:

- evaluate_session  
- read-only queries  
- export_area_dag

No new receipts emitted.

---

# 9. Determinism Guarantees

- Identical inputs → identical ordered errors  
- Acceptance independent of prior evaluation  
- No timestamp or UUID-time precedence  
- No cross-area influence  
- participant_id and candidate_id never reused across rounds

---

# 10. Final Guarantees

- Single-Area runtime  
- Supersession only via acceptance  
- Receipts immutable and deterministic  
- Participant epochs enforced  
- Evaluation pure and idempotent  
- Acceptance atomic  
- Degraded mode read-only  
- DAG export deterministic

---

# 11. Mental Model

The Engine is a deterministic legitimacy compiler.

- Evaluation inspects  
- Acceptance commits  
- Supersession evolves governance  
- Participant epochs bind presence in time  
- Receipts freeze closure  
- Rehydration defines the universe  
- Incremental compilation batches legitimate historical DAGs  
- Spec verification ensures backward-compatible rule provenance  
- Under review resolutions allow administrative state without affecting legitimacy

Everything explicit.  
Everything deterministic.  
Nothing inferred.  
Nothing auto-repaired.