# ENG-API — Engine Interface & Execution Boundary Specification

Status: REFACTORED (v18 – ENG-STRUCTURE / ENG-USABILITY Renaming, ON_HOLD Alignment & Reference Command Completion)  
Applies to: Engine Core (V1/V2+)  
Scope: Deterministic Engine interface, command surface, and runtime interaction boundary

Authority: Interface specification subordinate to ENG-INITIALIZATION and ENG-INTEGRITY.

Subordinate references consumed from:

- ENG-DOMAIN
- ENG-SESSION
- ENG-DECISION
- ENG-STRUCTURE
- ENG-RECEIPT
- ENG-CANON
- ENG-ERROR
- ENG-SPECVERIFY
- ENG-INITIALIZATION
- ENG-IMPORT
- ENG-COMPILATION
- ENG-PERSISTENCE
- ENG-USABILITY

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

The API provides access to Engine behavior.

It must not:

- redefine validation logic
- bypass governing specifications
- introduce alternative execution paths
- encode hidden legitimacy rules

All behavior is delegated to authoritative specifications.

---

## ENG-API-02 — Single-Area Runtime Model

The Engine operates on exactly one Area at a time.

- Runtime state exists only after successful initialization
- Area switching occurs only via rehydration
- No cross-Area evaluation is permitted

This invariant is enforced by:

- ENG-INITIALIZATION
- ENG-INTEGRITY

---

## ENG-API-03 — Deterministic Reporting

All mutating commands must produce exactly one EvaluationReport.

EvaluationReports:

- are defined in ENG-ERROR
- must be deterministic
- must contain ordered errors
- must not encode meaning through exceptions or side channels

The Engine must not:

- throw semantic exceptions
- return boolean legitimacy flags
- short-circuit validation

---

## ENG-API-04 — Identifier Ownership

All identifiers are Engine-generated UUIDv7 values.

This includes:

- session_id
- participant_id
- candidate_id
- resolution_id
- receipt_id
- vote_id

The API must not allow caller-supplied identifiers for Engine objects.

Identifier semantics are defined in ENG-DOMAIN and ENG-SESSION.

---

## ENG-API-05 — No Implicit Legitimacy

The API must not provide any operation that:

- creates legitimacy outside session acceptance
- directly mutates Resolution state except administrative transitions defined elsewhere
- bypasses governance validation
- reconstructs legitimacy artifacts

Legitimacy creation is governed exclusively by ENG-DECISION and committed via ENG-PERSISTENCE.

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
Deterministic result type such as EvaluationReport, query result, runtime outcome, or exported graph.

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
- ENG-STRUCTURE  
- ENG-RECEIPT  
- ENG-CANON  
- ENG-SPECVERIFY  

**Notes**  
- establishes single-Area runtime  
- replaces all prior runtime state  

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

**High-level Preconditions**  
- session exists in active Area

**Behavioral Authority**  
- ENG-DECISION  
- ENG-SESSION

**Notes**  
- reflects candidate-level viability  
- reflects session-level blocking  
- does not mutate votes or session state  
- does not persist solo-mode vote materialization  

---

# 6. Session Lifecycle

### create_session

**Purpose**  
Create a new session.

**Inputs**  
- session_type (AUTHORITY | SCOPE | REGULAR)  
- annotation (optional)

**Output**  
- session_id  
- EvaluationReport

**Mutation**  
Yes

**High-level Preconditions**  
- governance preconditions satisfied

**Behavioral Authority**  
- ENG-SESSION  
- ENG-DECISION  
- ENG-INTEGRITY  

**Notes**  
- governance snapshot captured at creation  
- annotation is informational only  

---

### resume_session

**Purpose**  
Advance a session to the next round.

**Inputs**  
- session_id

**Output**  
- EvaluationReport

**Mutation**  
Yes

**High-level Preconditions**  
- session is resumable

**Behavioral Authority**  
- ENG-SESSION  

**Notes**  
- clears round-scoped data  
- increments round_index  

---

### close_session

**Purpose**  
Close a session without acceptance.

**Inputs**  
- session_id

**Output**  
- receipt_id  
- EvaluationReport

**Mutation**  
Yes

**High-level Preconditions**  
- session is not already terminal

**Behavioral Authority**  
- ENG-SESSION  
- ENG-RECEIPT  
- ENG-PERSISTENCE  

**Notes**  
- emits EXPLORATION receipt  

---

### attempt_acceptance

**Purpose**  
Attempt to accept the currently winning eligible candidate.

**Inputs**  
- session_id

**Output**  
- resolution_id (if success and a Resolution is created)  
- receipt_id (if success)  
- EvaluationReport

**Mutation**  
Yes (only on success, except permitted solo-mode vote materialization in the mutating acceptance path)

**High-level Preconditions**  
- session exists  
- session is non-terminal  
- session is not barred from acceptance by runtime mode

**Behavioral Authority**  
- ENG-DECISION  
- ENG-SESSION  
- ENG-STRUCTURE  
- ENG-USABILITY  
- ENG-PERSISTENCE  
- ENG-RECEIPT  

**Notes**  
- winning candidate must satisfy authority rules  
- participants may change votes prior to invocation  
- includes candidate-level blocking checks  
- includes session-level blocking checks  
- may materialize a real solo-mode vote during the mutating acceptance path  

---

# 7. Session Mutation (PRE_STANCE Only)

### add_participant

**Purpose**  
Add a participant to the current session round.

**Inputs**  
- session_id  
- display_name  
- annotation (optional)

**Output**  
- participant_id  
- EvaluationReport

**Mutation**  
Yes

**High-level Preconditions**  
- session is in PRE_STANCE phase

**Behavioral Authority**  
- ENG-SESSION  
- ENG-DOMAIN  

**Notes**  
- participant identity is epoch-based  
- annotation is informational only  

---

### remove_participant

**Purpose**  
Remove a participant from the current session round.

**Inputs**  
- session_id  
- participant_id

**Output**  
- EvaluationReport

**Mutation**  
Yes

**High-level Preconditions**  
- session is in PRE_STANCE phase  
- participant exists in current round

**Behavioral Authority**  
- ENG-SESSION  
- ENG-ERROR  

**Notes**  
- cannot violate participant-count rules defined elsewhere  

---

### add_candidate

**Purpose**  
Add a candidate proposal to the current session round.

**Inputs**  
- session_id  
- candidate_action_type (ADOPT_RESOLUTION | SUPERSEDE_RESOLUTIONS | RETIRE_RESOLUTION)  
- candidate_payload  
- annotation (optional)

**Output**  
- candidate_id  
- EvaluationReport

**Mutation**  
Yes

**High-level Preconditions**  
- session is in PRE_STANCE phase

**Behavioral Authority**  
- ENG-SESSION  
- ENG-DOMAIN  

**Notes**  
- candidate represents an action proposal  
- payload structure enforced by ENG-DOMAIN  
- annotation is informational only  

---

### remove_candidate

**Purpose**  
Remove a candidate from the current session round.

**Inputs**  
- session_id  
- candidate_id

**Output**  
- EvaluationReport

**Mutation**  
Yes

**High-level Preconditions**  
- session is in PRE_STANCE phase  
- candidate exists in current round

**Behavioral Authority**  
- ENG-SESSION  
- ENG-ERROR  

**Notes**  
- candidate removal is not allowed after freeze  

---

### add_constraint

**Purpose**  
Add a constraint to the current session round.

**Inputs**  
- session_id  
- constraint_type  
- constraint_parameters  
- annotation (optional)

**Output**  
- constraint_id  
- EvaluationReport

**Mutation**  
Yes

**High-level Preconditions**  
- session is in PRE_STANCE phase

**Behavioral Authority**  
- ENG-SESSION  
- ENG-DECISION  
- ENG-DOMAIN  

**Notes**  
- annotation is informational only  

---

### remove_constraint

**Purpose**  
Remove a constraint from the current session round.

**Inputs**  
- session_id  
- constraint_id

**Output**  
- EvaluationReport

**Mutation**  
Yes

**High-level Preconditions**  
- session is in PRE_STANCE phase  
- constraint exists in current round

**Behavioral Authority**  
- ENG-SESSION  
- ENG-ERROR  

**Notes**  
- constraint removal is not allowed after freeze  

---

### cast_vote

**Purpose**  
Record or replace a vote.

**Inputs**  
- session_id  
- participant_id  
- candidate_id  
- stance  
- annotation (optional)

**Output**  
- vote_id  
- EvaluationReport

**Mutation**  
Yes

**High-level Preconditions**  
- participant and candidate exist in current round

**Behavioral Authority**  
- ENG-SESSION  
- ENG-DECISION  
- ENG-DOMAIN  

**Notes**  
- votes may change prior to acceptance  
- one current vote per participant per candidate per round  
- annotation is informational only  

---

### clear_vote

**Purpose**  
Remove a participant’s current vote for a candidate in the current round.

**Inputs**  
- session_id  
- participant_id  
- candidate_id

**Output**  
- EvaluationReport

**Mutation**  
Yes

**High-level Preconditions**  
- vote exists in current round  
- session is not terminal

**Behavioral Authority**  
- ENG-SESSION  
- ENG-DECISION  
- ENG-ERROR  

**Notes**  
- supported because vacillation is part of the design  
- subsequent voting may recreate the vote before acceptance  

---

### set_session_annotation

**Purpose**  
Set or replace the session annotation during mutable session operation.

**Inputs**  
- session_id  
- annotation

**Output**  
- EvaluationReport

**Mutation**  
Yes

**High-level Preconditions**  
- session exists  
- session is non-terminal

**Behavioral Authority**  
- ENG-DOMAIN  
- ENG-SESSION  

**Notes**  
- annotation is informational only  

---

### set_participant_annotation

**Purpose**  
Set or replace a participant annotation while the participant remains mutable.

**Inputs**  
- session_id  
- participant_id  
- annotation

**Output**  
- EvaluationReport

**Mutation**  
Yes

**High-level Preconditions**  
- session is in PRE_STANCE phase  
- participant exists in current round

**Behavioral Authority**  
- ENG-DOMAIN  
- ENG-SESSION  

**Notes**  
- annotation is informational only  

---

### set_candidate_annotation

**Purpose**  
Set or replace a candidate annotation while the candidate remains mutable.

**Inputs**  
- session_id  
- candidate_id  
- annotation

**Output**  
- EvaluationReport

**Mutation**  
Yes

**High-level Preconditions**  
- session is in PRE_STANCE phase  
- candidate exists in current round

**Behavioral Authority**  
- ENG-DOMAIN  
- ENG-SESSION  

**Notes**  
- annotation is informational only  

---

### set_constraint_annotation

**Purpose**  
Set or replace a constraint annotation while the constraint remains mutable.

**Inputs**  
- session_id  
- constraint_id  
- annotation

**Output**  
- EvaluationReport

**Mutation**  
Yes

**High-level Preconditions**  
- session is in PRE_STANCE phase  
- constraint exists in current round

**Behavioral Authority**  
- ENG-DOMAIN  
- ENG-SESSION  

**Notes**  
- annotation is informational only  

---

### set_vote_annotation

**Purpose**  
Set or replace a vote annotation while the vote remains mutable.

**Inputs**  
- session_id  
- vote_id  
- annotation

**Output**  
- EvaluationReport

**Mutation**  
Yes

**High-level Preconditions**  
- vote exists in current round  
- session is not terminal

**Behavioral Authority**  
- ENG-DOMAIN  
- ENG-SESSION  

**Notes**  
- annotation is informational only  

---

### add_internal_resolution_reference

**Purpose**  
Add an informational same-Area Resolution reference to the current session round.

**Inputs**  
- session_id  
- resolution_id

**Output**  
- EvaluationReport

**Mutation**  
Yes

**High-level Preconditions**  
- session is in PRE_STANCE phase  
- target Resolution exists in active Area

**Behavioral Authority**  
- ENG-DOMAIN  
- ENG-SESSION  
- ENG-INTEGRITY  

**Notes**  
- informational only  
- must preserve uniqueness and lexicographic ordering  

---

### remove_internal_resolution_reference

**Purpose**  
Remove an informational same-Area Resolution reference from the current session round.

**Inputs**  
- session_id  
- resolution_id

**Output**  
- EvaluationReport

**Mutation**  
Yes

**High-level Preconditions**  
- session is in PRE_STANCE phase  
- reference exists in current round

**Behavioral Authority**  
- ENG-DOMAIN  
- ENG-SESSION  

**Notes**  
- informational only  

---

### add_cross_area_reference

**Purpose**  
Add an informational cross-Area reference to the current session round.

**Inputs**  
- session_id  
- external_area_id  
- external_area_label  
- external_resolution_id (optional)  
- external_resolution_label (optional)

**Output**  
- EvaluationReport

**Mutation**  
Yes

**High-level Preconditions**  
- session is in PRE_STANCE phase

**Behavioral Authority**  
- ENG-DOMAIN  
- ENG-SESSION  

**Notes**  
- informational only  
- must not be interpreted as structural dependency  

---

### remove_cross_area_reference

**Purpose**  
Remove an informational cross-Area reference from the current session round.

**Inputs**  
- session_id  
- external_area_id  
- external_resolution_id (optional)

**Output**  
- EvaluationReport

**Mutation**  
Yes

**High-level Preconditions**  
- session is in PRE_STANCE phase  
- matching reference exists in current round

**Behavioral Authority**  
- ENG-DOMAIN  
- ENG-SESSION  

**Notes**  
- informational only  

---

# 8. Incremental Compilation

### begin_incremental_compilation

**Purpose**  
Begin incremental compilation mode.

**Inputs**  
- none

**Output**  
- EvaluationReport

**Mutation**  
Yes

**High-level Preconditions**  
- runtime initialized

**Behavioral Authority**  
- ENG-COMPILATION  
- ENG-INTEGRITY  

**Notes**  
- suspends normal mutating session workflows while active  

---

### add_incremental_batch

**Purpose**  
Add a batch to incremental compilation.

**Inputs**  
- domain_objects

**Output**  
- EvaluationReport

**Mutation**  
Yes

**High-level Preconditions**  
- compilation mode active

**Behavioral Authority**  
- ENG-COMPILATION  
- ENG-INTEGRITY  
- ENG-STRUCTURE  

**Notes**  
- historical replay ordering is defined externally  

---

### end_incremental_compilation

**Purpose**  
Finalize incremental compilation.

**Inputs**  
- none

**Output**  
- EvaluationReport

**Mutation**  
Yes

**High-level Preconditions**  
- compilation mode active

**Behavioral Authority**  
- ENG-COMPILATION  
- ENG-INTEGRITY  

**Notes**  
- restores normal runtime workflow if successful  

---

# 9. Administrative Operations

### set_resolution_on_hold

**Purpose**  
Place a Resolution in ON_HOLD state.

**Inputs**  
- resolution_id

**Output**  
- EvaluationReport

**Mutation**  
Yes

**High-level Preconditions**  
- Resolution exists  
- Resolution supports ON_HOLD transition

**Behavioral Authority**  
- ENG-USABILITY  
- ENG-INTEGRITY  

**Notes**  
- does not create legitimacy  
- does not alter historical legitimacy  

---

### restore_resolution_active

**Purpose**  
Restore a Resolution from ON_HOLD to ACTIVE.

**Inputs**  
- resolution_id

**Output**  
- EvaluationReport

**Mutation**  
Yes

**High-level Preconditions**  
- Resolution exists  
- Resolution is ON_HOLD

**Behavioral Authority**  
- ENG-USABILITY  
- ENG-INTEGRITY  

**Notes**  
- administrative usability transition only  

---

# 10. Queries

### list_sessions

**Purpose**  
List sessions in the current Area.

**Inputs**  
- none

**Output**  
- session summaries

**Mutation**  
No

**High-level Preconditions**  
- runtime initialized

**Behavioral Authority**  
- ENG-DOMAIN  

**Notes**  
- deterministic ordering required  

---

### list_resolutions

**Purpose**  
List Resolutions in the current Area.

**Inputs**  
- none

**Output**  
- Resolution summaries

**Mutation**  
No

**High-level Preconditions**  
- runtime initialized

**Behavioral Authority**  
- ENG-DOMAIN  

**Notes**  
- deterministic ordering required  

---

### get_session_state

**Purpose**  
Retrieve full session state.

**Inputs**  
- session_id

**Output**  
- session object

**Mutation**  
No

**High-level Preconditions**  
- session exists in active Area

**Behavioral Authority**  
- ENG-DOMAIN  

**Notes**  
- returns current runtime state only  

---

### get_resolution_state

**Purpose**  
Retrieve Resolution state.

**Inputs**  
- resolution_id

**Output**  
- Resolution object

**Mutation**  
No

**High-level Preconditions**  
- Resolution exists in active Area

**Behavioral Authority**  
- ENG-DOMAIN  

**Notes**  
- returns current runtime state only  

---

### get_session_receipt

**Purpose**  
Retrieve the terminal receipt for a session.

**Inputs**  
- session_id

**Output**  
- receipt

**Mutation**  
No

**High-level Preconditions**  
- session has terminal receipt

**Behavioral Authority**  
- ENG-RECEIPT  

**Notes**  
- terminal receipt is authoritative  

---

### list_session_receipts

**Purpose**  
List receipts associated with a session.

**Inputs**  
- session_id

**Output**  
- receipt list

**Mutation**  
No

**High-level Preconditions**  
- session exists in active Area

**Behavioral Authority**  
- ENG-RECEIPT  

**Notes**  
- deterministic ordering required  

---

### list_session_rounds

**Purpose**  
Return deterministic historical rounds.

**Inputs**  
- session_id

**Output**  
- ordered round snapshots

**Mutation**  
No

**High-level Preconditions**  
- session has receipt or historical data

**Behavioral Authority**  
- ENG-RECEIPT  
- ENG-DOMAIN  

**Notes**  
- supports reconstruction of prior configurations  
- informational only  

---

### list_session_candidates

**Purpose**  
List candidates with evaluated status.

**Inputs**  
- session_id

**Output**  
- candidate summaries with status

**Mutation**  
No

**High-level Preconditions**  
- session exists in active Area

**Behavioral Authority**  
- ENG-DECISION  
- ENG-API  

**Notes**  
- status is derived, not persisted  
- includes ELIGIBLE / BLOCKED_TEMPORARY / BLOCKED_PERMANENT / INVALID  

---

### get_candidate_status

**Purpose**  
Retrieve evaluated status for a candidate.

**Inputs**  
- session_id  
- candidate_id

**Output**  
- candidate status

**Mutation**  
No

**High-level Preconditions**  
- session and candidate exist in active Area

**Behavioral Authority**  
- ENG-DECISION  

**Notes**  
- reflects current runtime state  
- may change without candidate mutation  

---

# 11. Specification Identity

### get_spec_set_hash

**Purpose**  
Retrieve the current spec set hash.

**Inputs**  
- none

**Output**  
- spec_set_hash

**Mutation**  
No

**High-level Preconditions**  
- runtime initialized

**Behavioral Authority**  
- ENG-SPECVERIFY  

**Notes**  
- read-only identity query  

---

### verify_spec_hash

**Purpose**  
Verify a spec hash value.

**Inputs**  
- spec_set_hash

**Output**  
- verification result

**Mutation**  
No

**High-level Preconditions**  
- input hash provided

**Behavioral Authority**  
- ENG-SPECVERIFY  

**Notes**  
- does not reinterpret rule identity semantics  

---

# 12. Export

### export_area_dag

**Purpose**  
Export the current Area graph.

**Inputs**  
- none

**Output**  
- domain graph

**Mutation**  
No

**High-level Preconditions**  
- runtime initialized

**Behavioral Authority**  
- ENG-DOMAIN  
- ENG-CANON  
- ENG-RECEIPT  

**Notes**  
- export must be deterministic  
- export must preserve canonical structure required for downstream use  

---

# 13. Blocking Model (Interface-Level Clarification)

## ENG-API-07 — Dual-Level Blocking Model

Blocking exists at two levels.

### Session-Level Blocking

Caused by:

- Authority invalidation → BLOCK_PERMANENT  
- Scope supersession or invalidation → BLOCK_PERMANENT  
- Scope ON_HOLD → BLOCK_TEMPORARY  

### Candidate-Level Blocking

Caused by:

- superseded target  
- retired target  
- ON_HOLD target  
- unusable referenced artifacts  

ENG-API exposes both through EvaluationReport and query interfaces.  
It does not define their semantics.

---

# 14. Degraded Mode

## ENG-API-08 — Read-Only Enforcement

When runtime mode is DEGRADED_READ_ONLY:

- mutating commands must fail deterministically  
- read-only operations remain available  

Behavior is defined in:

- ENG-INTEGRITY  
- ENG-INITIALIZATION  

---

# 15. Determinism Guarantees

## ENG-API-09 — Deterministic Interface Behavior

Given identical inputs and runtime state:

- identical commands produce identical EvaluationReports  
- identical queries produce identical results  
- no behavior depends on timestamps, ordering, or environment  

Determinism is enforced by the authoritative specifications consumed by the API.

---

# 16. Engine Invariants

- API never creates legitimacy directly  
- API never bypasses validation  
- API never mutates state on failed validation except explicitly permitted solo-mode vote materialization during mutating acceptance flow  
- API exposes both session-level and candidate-level blocking  
- API reflects authoritative evaluation without reinterpretation  
- API never merges Areas  
- API never reuses identifiers  
- API never persists derived candidate status as candidate state  

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
- structural graph semantics  
- usability semantics  
- structural validity  

Those belong to their respective specifications.