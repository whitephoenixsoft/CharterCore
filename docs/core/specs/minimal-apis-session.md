# Charter Core — Minimal Engine API Specification (V1+ with Annotations)
Status: FOUNDATIONAL
Layer: Engine / Minimal API
Authority: Subordinate to Charter Core Engine Invariants
Purpose: Define the minimal, deterministic, auditable operations required to create, evaluate, and record legitimacy in Charter.

This API is intentionally small.
Anything not guaranteed here must be implemented outside the engine.

All operations MUST:
- Respect all Engine Invariants (ENG-CORE through ENG-AI)
- Be deterministic
- Be fully auditable
- Never implicitly create legitimacy

Annotations are first-class and participate in legitimacy.
They are frozen at the same boundary as acceptance.

---

## 1. start_session

**Definition**  
Create a new session to evaluate one or more candidates for a specific problem or decision.

**Inputs**
- `area_id` (engine ID)
- `topic` (required, immutable problem statement or decision framing)
- `participant_ids` (explicit list of engine IDs)
- `preceding_resolution_id` (optional; must be ACTIVE if provided)
- `constraints` (optional; session-scoped, authority-equivalent rules)
- `annotations` (optional; context, intent, framing)

**Outputs**
- `session_id` (engine ID)
- `status = ACTIVE | BLOCKED`

**Behavior**
- Initializes a new session context.
- Records topic, participants, constraints, and annotations immutably.
- Records start time in session history.
- Governing Authority and Scope are bound to the session context.
- If Authority or Scope is UNDER_REVIEW, the session is created as `BLOCKED`.

**Fail if**
- Area is uninitialized.
- Participant set is empty (unless explicitly confirmed by the caller).
- An active session already exists in solo mode.
- `preceding_resolution_id` is not ACTIVE.

**Notes**
- Sessions may exist while BLOCKED.
- BLOCKED sessions may prepare candidates but may not record stances or acceptance.

---

## 2. add_candidate

**Definition**  
Add a candidate proposal to an active session.

**Inputs**
- `session_id`
- `candidate_content` (structured, deterministic payload)
- `annotations` (optional; rationale, framing, context)

**Outputs**
- `candidate_id` (engine ID)

**Behavior**
- Candidate has no legitimacy unless accepted.
- Candidate annotations are frozen at acceptance.
- Candidate content becomes immutable once the first stance is recorded.

**Fail if**
- Session is PAUSED or CLOSED.
- Candidate set is frozen (first stance already recorded).

---

## 3. record_stance

**Definition**  
Record a participant’s evaluative stance on a candidate.

**Inputs**
- `session_id`
- `participant_id`
- `candidate_id`
- `stance` (`ACCEPT | REJECT | ABSTAIN`)
- `annotations` (optional; reasoning behind the stance)

**Outputs**
- Updated session state

**Behavior**
- Freezes the candidate set on first stance.
- Records stance and annotations immutably in the session audit trail.
- Stances are mutable until acceptance.

**Fail if**
- Participant is not explicitly part of the session.
- Session is BLOCKED or CLOSED.

---

## 4. pause_session / resume_session / block_session

**Definition**  
Change the operational status of a session without creating legitimacy.

**Inputs**
- `session_id`
- `status_change` (`PAUSE | RESUME | BLOCK`)
- `annotations` (optional; rationale for status change)

**Outputs**
- Updated session status

**Behavior**
- **PAUSE**: Halts session activity; no legitimacy actions allowed.
- **BLOCK**: Prevents legitimacy due to unresolved dependency (e.g., Authority or Scope UNDER_REVIEW).
- **RESUME**: Requires explicit revalidation of governing context.
- All status transitions are recorded immutably in the session history.

**Fail if**
- Attempting to RESUME without explicit revalidation.
- Attempting to modify a CLOSED session.

---

## 5. accept_candidate

**Definition**  
Explicitly accept a candidate, creating a legitimate resolution.

**Inputs**
- `session_id`
- `candidate_id`
- `annotations` (optional; rationale for acceptance)

**Outputs**
- `resolution_id` (engine ID)
- `status = ACTIVE`

**Behavior**
- Evaluates Authority, Scope, constraints, participants, and stances at acceptance time.
- Freezes:
  - candidate content
  - all annotations
  - participant set
  - stances
  - governing Authority and Scope
- Records full acceptance context immutably.
- If a `preceding_resolution_id` was declared at session start, it becomes `SUPERSEDED`.

**Fail if**
- Session is PAUSED or BLOCKED.
- Authority or Scope is UNDER_REVIEW.
- Authority evaluation fails.
- Preceding resolution is not ACTIVE.

---

## 6. close_session

**Definition**  
Finalize a session, preventing further changes.

**Inputs**
- `session_id`
- `annotations` (optional; closure summary)

**Outputs**
- `status = CLOSED`

**Behavior**
- Freezes all remaining session state.
- Session history remains fully auditable and immutable.

**Fail if**
- Session is already CLOSED.
- Session has unresolved blocking dependencies (unless force-close is explicitly allowed by the caller).

---

## 7. import_restore

**Definition**  
Destructively replace an Area with a canonical export snapshot.

**Inputs**
- `area_id`
- `export_file`
- `annotations` (optional; rationale)

**Outputs**
- `status = AREA_RESTORED`
- Global audit entry

**Behavior**
- Replaces all sessions, resolutions, Authority, and Scope.
- Requires explicit confirmation.
- Emits a global audit event.

**Fail if**
- Attempting partial restore.
- Confirmation is bypassed.

---

## 8. import_consolidate

**Definition**  
Import external history into a baseline review workspace.

**Inputs**
- `area_id`
- `import_file`
- `annotations` (optional)

**Outputs**
- `baseline_id` (engine ID)
- Imported resolutions marked `UNDER_REVIEW` (baseline-only)

**Behavior**
- Creates a baseline review workspace.
- Pauses active sessions.
- Imported history has no legitimacy until reviewed via sessions.
- All imported annotations are preserved.

**Fail if**
- Imported resolutions gain ACTIVE status automatically.
- Sessions continue without explicit pause.

---

## 9. evaluate_session

**Definition**  
Evaluate whether a session is eligible to create legitimacy.

**Inputs**
- `session_id`
- `annotations` (optional)

**Outputs**
- `can_accept` (boolean)
- `reasoning` (deterministic explanation of blocking conditions)

**Behavior**
- Checks:
  - Authority and Scope state
  - participant set
  - constraints
  - stance completeness
  - session status
- Read-only and non-mutating.
- Intended for CLI status and guidance layers.

**Fail if**
- Any engine state is mutated.

---

## Final Guarantees

- Annotations participate in legitimacy and are frozen at acceptance.
- BLOCKED prevents legitimacy, not preparation.
- Sessions record a complete, immutable internal history (“session receipt”).
- No API call may implicitly create, alter, or infer legitimacy.

If an implementation violates this specification,
the implementation is wrong — not the specification.