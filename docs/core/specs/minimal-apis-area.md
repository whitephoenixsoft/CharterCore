# Minimal Area API (Foundational)

## 1. create_area
Definition: Create a new governance area.

Inputs:
- label (short identifier)
- long_name (human-readable)
- annotations (optional)

Outputs:
- area_id (engine ID)
- status = CREATED

Behavior:
- No Authority or Scope yet; area is non-operational until initialization.
- CLI may provide default solo-mode Authority/Scope for convenience.
- All state recorded for audit.

Fail if:
- label duplicates an existing area.
- invalid input formats.

---

## 2. initialize_area_session
Definition: Start a session to initialize Authority and Scope.

Inputs:
- area_id
- proposed_authority (optional, default: solo-mode)
- proposed_scope (optional)
- participant_ids (list)
- annotations (optional)

Outputs:
- session_id (engine ID)
- status = ACTIVE

Behavior:
- This session creates the first Authority and Scope.
- Must follow all session invariants (auditable, explicit acceptance).

Fail if:
- Area already initialized.
- Participants empty (unless overridden).

---

## 3. get_governance_state
Definition: Retrieve current snapshot of Area governance.

Inputs:
- area_id

Outputs:
- authority_id
- scope_id
- active_sessions
- annotations
- audit_snapshot

Behavior:
- Returns the canonical snapshot used to validate sessions.
- Used to block or permit session operations.

Fail if:
- Area is not created.

---

## 4. list_area_resolutions
Definition: Enumerate all resolutions in an Area.

Inputs:
- area_id
- optional filters: status, type

Outputs:
- List of resolution records:
  - ID, status, type, audit info

Behavior:
- Returns full history including SUPERSEDED, RETIRED, UNDER_REVIEW.
- Immutable; auditable.

Fail if:
- Area is not created.

---

## 5. archive_area
Definition: Retire an Area.

Inputs:
- area_id
- annotations (optional)

Outputs:
- status = ARCHIVED

Behavior:
- No new sessions can start.
- Existing audit/history preserved.

Fail if:
- Active sessions exist (must be paused or closed).