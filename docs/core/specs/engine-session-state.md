# Charter Core — Session State Machine Specification

Document ID: ENG-SESSION
Status: FROZEN (v1)
Audience: Charter Core engine implementers
Scope: Session lifecycle, transitions, enforcement, and legitimacy boundaries

---
## 1. Purpose

### ENG-SESSION-01 — Sessions Are the Sole Legitimacy Mechanism

A Session is the only structure through which legitimacy can be created in Charter Core.
All acceptance, rejection, supersession, and authority changes MUST occur within a session.
Fail if:
A resolution is accepted without a session
Legitimacy is created outside a session boundary

---
## 2. Session Identity & Persistence

### ENG-SESSION-02 — Session Identity Is Stable

Each session has a stable session_id independent of:
Storage location
Object hash
Export/import
Context
Session identity MUST remain stable across restarts and exports.
Fail if:
Session identity changes due to persistence mechanics

---
## 3. Session States (Closed Set)

### ENG-SESSION-03 — Valid Session States

A session MUST be in exactly one of the following states:
Copy code

CREATED
ACTIVE
PAUSED
BLOCKED
CLOSED
ABANDONED
No other states are permitted.
Fail if:
Engine introduces implicit or transitional states

---
## 4. State Semantics (Mechanical)

### CREATED

Session metadata is defined
Authority and Scope are captured
Constraints are declared
No candidates or stances exist
Permitted actions:
Add candidates
Start session
Forbidden:
Record stances
Accept resolutions

### ACTIVE

Session is live
Authority and constraints are enforced
Candidates may be added until first stance
Stances may be recorded
Permitted actions:
Record stances
Evaluate acceptance conditions
Close session (if constraints satisfied)
Forbidden:
Modify Authority
Modify Scope
Modify constraints after first stance

### PAUSED

Session is explicitly paused by user action
No stances may be recorded
Authority and Scope remain frozen
Permitted actions:
Resume session
Abandon session
Forbidden:
Accept resolutions
Modify constraints

### BLOCKED

Session cannot proceed due to invariant violation
Block is mechanical, not discretionary
Common causes:
Authority changed
Scope changed
Constraint invalidated
Required participant removed
Permitted actions:
Explicit user resolution (e.g. restart, abandon)
Forbidden:
Resume without explicit handling
Accept resolutions

### CLOSED

Session has concluded
Outcome is finalized
Zero or more resolutions are accepted
Properties:
Immutable
Export-eligible
Legitimacy-bearing
Forbidden:
Any mutation

### ABANDONED

Session explicitly discarded
No legitimacy created
Properties:
Immutable
Non-legitimizing
Exportable only as audit context
Forbidden:
Reopening
Acceptance

---
## 5. Valid State Transitions

### ENG-SESSION-04 — Allowed Transitions Only

From
To
Trigger
CREATED
ACTIVE
start_session
ACTIVE
PAUSED
pause
PAUSED
ACTIVE
resume (if valid)
ACTIVE
BLOCKED
invariant violation
PAUSED
BLOCKED
invariant violation
BLOCKED
PAUSED
explicit user action
CREATED
ABANDONED
abandon
ACTIVE
ABANDONED
abandon
PAUSED
ABANDONED
abandon
ACTIVE
CLOSED
accept_outcome
Fail if:
Any transition not listed occurs
Transitions are implicit

---
## 6. Authority & Scope Capture

### ENG-SESSION-05 — Authority Is Snapshotted

At session creation:
Active Authority resolution ID is captured
Active Scope resolution ID is captured
Additional referenced scopes may be recorded
These values are immutable for the session lifetime.
Fail if:
Authority or Scope changes mid-session
Resume attempts to renegotiate Authority or Scope

---
## 7. Constraint Enforcement

### ENG-SESSION-06 — Constraints Are Immutable

Constraints:
Must be declared at session creation
Must be visible before any stance
Must remain unchanged for the session lifetime
Fail if:
Constraints are added or modified after first stance
Constraints are inferred implicitly

---
## 8. Candidate Rules

### ENG-SESSION-07 — Candidate Set Freeze

Before first stance:
Candidates may be added
After first stance:
Candidate set is frozen
Fail if:
Candidate is added, removed, or edited after first stance

---
## 9. Stance Rules

### ENG-SESSION-08 — Stances Are Mechanical

Stances:
Are immutable once recorded
Are scoped to (actor_id, candidate_id)
Derive presence implicitly
The engine MUST NOT infer:
Intent
Role
Authority
Fail if:
Silence is interpreted as consent

---
### 10. Acceptance Evaluation

### ENG-SESSION-09 — Deterministic Acceptance

Acceptance is evaluated mechanically based on:
Authority rule
Constraints
Recorded stances
Given identical inputs, outcome MUST be identical.
Fail if:
Non-deterministic evaluation occurs

---
## 11. Closing a Session

### ENG-SESSION-10 — Closure Is Explicit

A session may be CLOSED only when:
Acceptance conditions are met
User explicitly closes the session
On closure:
Accepted candidates become resolutions
Resolution context is captured
Session becomes immutable
Fail if:
Session auto-closes
Partial outcomes are persisted

---
## 12. Blocking Semantics

### ENG-SESSION-11 — Blocking Is Mechanical

A session MUST enter BLOCKED when:
Authority resolution changes
Scope resolution changes
Constraints are invalidated
Blocked sessions:
Cannot accept
Cannot resume without explicit handling
Fail if:
Blocked session proceeds silently

---
## 13. Import Interaction

### ENG-SESSION-12 — Imported Sessions Are Non-Legitimizing

In CONSOLIDATE mode:
Imported sessions MUST NOT enter ACTIVE
Imported sessions MUST NOT accept resolutions
Imported sessions MAY exist as read-only context
Fail if:
Imported sessions affect legitimacy

---
## 14. Export Eligibility

### ENG-SESSION-13 — Only CLOSED Sessions Are Legitimate

Only sessions in state CLOSED:
May produce legitimacy-bearing resolutions
May participate in authoritative export
Fail if:
ACTIVE, PAUSED, or BLOCKED sessions influence export

---
## 15. Audit Requirements

### ENG-SESSION-14 — Session Lifecycle Is Audited

The engine MUST emit audit events for:
Session creation
State transitions
Closure
Abandonment
Blocking
Fail if:
Session state changes are not auditable

---
## 16. Failure Semantics

### ENG-SESSION-15 — Fail Explicitly

Violations MUST:
Fail immediately
Leave session state unchanged
Emit diagnostics
Fail if:
Engine attempts silent correction

---
## 17. Summary Guarantees

### ENG-SESSION guarantees:

Legitimacy is session-scoped
Authority cannot drift
Constraints cannot mutate
Outcomes are deterministic
History is immutable

---
## Mental Model

- Sessions freeze context
- Votes are mechanical
- Acceptance is deterministic
- Legitimacy is born only at closure
