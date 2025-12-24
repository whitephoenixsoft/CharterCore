# Charter Core — Minimal Engine API (Frozen Draft)

> This API defines the entire behavioral surface of Charter Core.
> Any UI, CLI, AI, or integration layer must operate strictly through these primitives.

## 1. Area APIs

Areas are hard governance boundaries.
All sessions and resolutions belong to exactly one Area.

`create_area`

```Text
create_area(
  area_id,
  metadata?
)
```

- Creates an Area in an uninitialized state
- No decisions allowed until Authority + Scope are established

---

`get_area`

```Text
get_area(area_id)
```

Returns:
- Area metadata
- Active Authority resolution (if any)
- Active Scope resolution (if any)
- Area status (initialized / uninitialized)

---

2. Session APIs
Sessions are the only place where legitimacy can occur.
start_session
Copy code
Text
start_session(
  area_id,
  session_id,
  problem_statement,
  referenced_scope_ids?   // optional, explicit
)
Rules:
Fails if Area is uninitialized
Captures:
Active Authority
Active Scope
Referenced scopes (explicit only)
Announces decision rule up front
get_session
Copy code
Text
get_session(session_id)
Returns:
Session state
Participants (engine-agnostic identifiers)
Candidates
Votes / acknowledgements (mechanical only)
Blocking / pause status
pause_session
Copy code
Text
pause_session(session_id, reason?)
Session becomes non-progressable
No candidates can be accepted
resume_session
Copy code
Text
resume_session(session_id)
On resume, engine must re-validate:
Active Authority
Active Scope
If changed:
Session enters blocked state
Explicit user action required
close_session
Copy code
Text
close_session(session_id, reason?)
No resolution accepted
Session becomes immutable historical record
3. Candidate APIs
Candidates are neutral options — no meaning, no intent.
add_candidate
Copy code
Text
add_candidate(
  session_id,
  candidate_id,
  content,
  rationale?
)
Notes:
Rationale is optional
Stored but never evaluated
list_candidates
Copy code
Text
list_candidates(session_id)
4. Decision / Acceptance APIs
record_vote
Copy code
Text
record_vote(
  session_id,
  actor_id,
  candidate_id
)
Engine records votes mechanically
No semantic interpretation
evaluate_session
Copy code
Text
evaluate_session(session_id)
Engine checks:
Authority rule
Scope validity
Session state
Returns:
ACCEPTABLE
BLOCKED (with reason)
INCOMPLETE
accept_candidate
Copy code
Text
accept_candidate(session_id, candidate_id)
Rules:
Exactly one candidate per resolution
Fails if:
Authority rule not satisfied
Candidate under review
Session blocked or paused
Creates:
A Resolution
Captures acceptance context immutably
5. Resolution APIs
Resolutions are immutable, first-class records.
get_resolution
Copy code
Text
get_resolution(resolution_id)
Returns:
Content
Lifecycle state
Acceptance context:
Authority at acceptance
Scope at acceptance
Referenced scopes
Session ID
list_resolutions
Copy code
Text
list_resolutions(
  area_id,
  state_filter?
)
transition_resolution_state
Copy code
Text
transition_resolution_state(
  resolution_id,
  new_state
)
Allowed states:
ACTIVE
UNDER_REVIEW
SUPERSEDED
RETIRED
Rules:
State transitions are explicit
UNDER_REVIEW resolutions cannot be accepted as authority/scope
6. Authority & Scope APIs (Specialized but Minimal)
Authority and Scope are normal resolutions with enforced uniqueness.
get_active_authority
Copy code
Text
get_active_authority(area_id)
get_active_scope
Copy code
Text
get_active_scope(area_id)
accept_authority_resolution
Copy code
Text
accept_authority_resolution(
  session_id,
  candidate_id
)
Rules:
Only one active Authority per Area
Previous Authority becomes RETIRED
Uses existing Authority to validate change
accept_scope_resolution
Copy code
Text
accept_scope_resolution(
  session_id,
  candidate_id
)
Rules:
Only one active Scope per Area
Previous Scope becomes RETIRED
7. Import / Export APIs (Engine-Level)
export_area
Copy code
Text
export_area(
  area_id,
  include_sessions = true
)
Exports:
Areas
Resolutions
Sessions
Candidates
Full audit trail
Single canonical export format (engine-owned).
import_area
Copy code
Text
import_area(
  export_blob,
  mode = CONSOLIDATE | RESTORE
)
Modes:
RESTORE: recreate full history
CONSOLIDATE: imported resolutions enter UNDER_REVIEW
Engine does not auto-merge semantics.
8. Query / Audit APIs
get_resolution_history
Copy code
Text
get_resolution_history(resolution_id)
get_area_audit_log
Copy code
Text
get_area_audit_log(area_id)
Returns:
Session timeline
Authority changes
Scope changes
Resolution lifecycle transitions
9. Explicit Non-APIs (By Design)
Charter Core does not provide:
User roles
Identity enforcement
Semantic analysis
Conflict inference
AI suggestions
Workflow orchestration
Notifications
Permissions beyond Authority mechanics
All of these belong outside the engine.
10. Design Guarantees (API-Level)
This API guarantees:
No implicit decisions
No retroactive changes
No semantic guessing
No required rationale
No AI dependency
No hidden state transitions
If an integration needs more power, it must build on top, not inside.