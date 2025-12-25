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

## 2. Session APIs

Sessions are the only place where legitimacy can occur.

`start_session`

```Text
start_session(
  area_id,
  session_id,
  problem_statement,
  participant_list,
  preceding_resolution_id?, // optional, explicit
  referenced_scope_ids?   // optional, explicit
)
```

Rules:
- Fails if Area is uninitialized
- Fails if preceding resolution does not exist or status is not `ACTIVE`or if not in the same Area.
- Fails if participant list is empty 
- Captures:
    - Active Authority
    - Active Scope
    - The list of participants at the start of the session 
    - Referenced scopes (explicit only)
    - Resolution being superceded (explicit only)
- Announces decision rule up front

The engine does not proactively invalidate sessions; it reacts at interaction points.

---

`get_session`

```Text
get_session(session_id)
```

Returns:
- Session state
- Participants (engine-agnostic identifiers)
- Candidates
- Votes / acknowledgements (mechanical only)
- Blocking / pause status

---

`pause_session`

```Text
pause_session(session_id, reason?)
```

- Session becomes non-progressable
- No candidates can be accepted

--- 

`resume_session`

```Text
resume_session(session_id, participant_list)
```

On resume, engine must re-validate:
- Active Authority
- Active Scope
- Preceding Resolution

If changed:
- Session enters blocked state
- Explicit user action required

---

`close_session`

```Text
close_session(session_id, reason?)
```

- No resolution accepted
- Session becomes immutable historical record

---

## 3. Candidate APIs

Candidates are neutral options — no meaning, no intent.

`add_candidate`

```Text
add_candidate(
  session_id,
  candidate_id,
  content,
  rationale?
)
```

Notes:
- Rationale is optional
- Stored but never evaluated

---

`list_candidates`

```Text
list_candidates(session_id)
```
---

## 4. Decision / Acceptance APIs

`record_stance`

```Text
record_stance(
    session_id, 
    candidate_id, 
    actor_id, 
    stance
)
```

Rules:
- stance ∈ {ACCEPT, REJECT, ABSTAIN}
- Engine records votes mechanically
- No semantic interpretation

---

`evaluate_session`

```Text
evaluate_session(session_id, participant_list)
```

Engine checks:
- Authority rule
- Scope validity
- Session state
- Preceding Resolution state

Returns:
- `ACCEPTABLE`
- `BLOCKED` (with reason)
- `INCOMPLETE`

This function is informational only. 

---

`accept_candidate`

```Text
accept_candidate(
  session_id, 
  candidate_id,
  participant_list
)
```

Rules:
- Exactly one candidate per resolution
- Participant list is used to validate authority 

Fails if:
- Authority rule not satisfied for that Candidate 
- Session blocked or paused
-  Preceding Resolution is not `ACTIVE`

Creates:
- A Resolution
- Captures acceptance context immutably

This method is authoritive.

The engine does not proactively invalidate sessions; it reacts at interaction points.

---

## 5. Resolution APIs

Resolutions are immutable, first-class records.

`get_resolution`

```Text
get_resolution(resolution_id)
```

Returns:
- Content
- Lifecycle state

Acceptance context:
- Authority at acceptance
- Scope at acceptance
- Referenced scopes
- Session ID
- Superceded Resolution 

---

`list_resolutions`

```Text
list_resolutions(
  area_id,
  state_filter?
)
```

---

`mark_resolution_under_review`

```Text
mark_resolution_under_review(resolution_id)
```
Rules:
- Cannot be called on Scope or Authority resolutions
- Resolution must be `ACTIVE`

---

`clear_resolution_under_review`

```Text
clear_resolution_under_review(resolution_id)
```

Rules :
- Will set the status of the resolution to `ACTIVE`
- Only resolutions marked `UNDER_REVIEW` manually (not imported) qualify

---

## 6. Authority & Scope APIs (Specialized but Minimal)

Authority and Scope are normal resolutions with enforced uniqueness.

`get_active_authority`

```Text
get_active_authority(area_id)
```
---

`get_active_scope`

```Text
get_active_scope(area_id)
```
---

`accept_authority_resolution`

```Text
accept_authority_resolution(
  session_id,
  candidate_id
)
```

Rules:
- Only one active Authority per Area
- Previous Authority becomes `SUPERCEDED`
- Uses existing Authority to validate change

---

`accept_scope_resolution`

```Text
accept_scope_resolution(
  session_id,
  candidate_id
)
```

Rules:
- Only one active Scope per Area
- Previous Scope becomes `SUPERCEDED`

---

## 7. Import / Export APIs (Engine-Level)

`export_area`

```Text
export_area(
  area_id,
  include_full_history = true
)
```

Exports:
- Areas
- Resolutions
- Sessions
- Candidates
- Full audit trail
- Single canonical export format (engine-owned).
- setting `include_full_history` to `false` will only export resolutions

---

`import_area`

```Text
import_area(
  export_blob,
  mode = CONSOLIDATE | RESTORE,
  import_on_failure = false 
)
```

Modes:
- `RESTORE`: recreate full history
- `CONSOLIDATE`: imported resolutions enter UNDER_REVIEW

Returns:
- `true` if import is completed
- `false` if the import fails

Rules:
-  import_on_failure will only force a continue on a valid import file but when the integrity of the history is questionable. At that point, any resolution imported will be marked as `UNDER_REVIEW`

Engine does not auto-merge semantics.

---

## 8. Query / Audit APIs

`get_resolution_history`

```Text
get_resolution_history(resolution_id)
```
--- 

`get_area_audit_log`

```Text
get_area_audit_log(area_id)
```

Returns:
- Session timeline
- Authority changes
- Scope changes
- Resolution lifecycle transitions

---

## 9. Explicit Non-APIs (By Design)

Charter Core does not provide:
- User roles
- Identity enforcement
- Semantic analysis
- Conflict inference
- AI suggestions
- Workflow orchestration
- Notifications
- Permissions beyond Authority mechanics

All of these belong outside the engine.

---

## 10. Design Guarantees (API-Level)

This API guarantees:
- No implicit decisions
- No retroactive changes
- No semantic guessing
- No required rationale
- No AI dependency
- No hidden state transitions

If an integration needs more power, it must build on top, not ins