# Charter Core — Session State Machine Specification

Document ID: ENG-SESSION  
Status: FROZEN (v1)  
Audience: Charter Core engine implementers  
Scope: Session lifecycle, transitions, enforcement, and legitimacy boundaries  

---

## 1. Purpose

### ENG-SESSION-01 — Sessions Are the Sole Legitimacy Mechanism

- Sessions are the **only structure** through which legitimacy can be created.  
- All acceptance, rejection, supersession, and authority changes **must** occur within a session.  

Fail if:

- A resolution is accepted outside a session  
- Legitimacy exists outside session boundaries  

---

## 2. Session Identity & Persistence

### ENG-SESSION-02 — Session Identity Is Stable

- Each session has a stable `session_id` independent of:

  - Storage location  
  - Object hash  
  - Export/import  
  - Context  

- Session identity **must** remain stable across restarts and exports.  

Fail if:

- Session identity changes due to persistence mechanics  

---

## 3. Session States (Closed Set)

### ENG-SESSION-03 — Valid Session States

A session must be in **exactly one** of:
```
CREATED 
ACTIVE
PAUSED
BLOCKED
ACCEPTED 
CLOSED
```
Fail if:

- Engine introduces implicit or transitional states  

---

## 4. State Semantics

### CREATED

- Metadata is defined  
- Authority & Scope captured  
- Constraints declared  
- No candidates or stances exist  

Permitted actions:

- Add candidates  
- Start session  

Forbidden:

- Record stances  
- Accept resolutions  

### ACTIVE

- Session is live  
- Authority and constraints enforced  
- Candidates may be added until first stance  
- Stances may be recorded  

Permitted actions:

- Record stances  
- Evaluate acceptance conditions  
- Close session if constraints satisfied  

Forbidden:

- Modify Authority  
- Modify Scope  
- Modify constraints after first stance  

### PAUSED

- Session paused explicitly by user  
- Authority & Scope frozen  
- No stances may be recorded  

Permitted actions:

- Resume session  
- Abandon session  

Forbidden:

- Accept resolutions  
- Modify constraints  

### BLOCKED

- Session cannot proceed due to invariant violation  

Causes:

- Authority change  
- Scope change  
- Constraint invalidation  
- Required participant removed  

Permitted actions:

- Explicit user resolution (restart, abandon)  

Forbidden:

- Resume without explicit handling  
- Accept resolutions  

### ACCEPTED

- Session concluded  
- Zero or more resolutions accepted  

Properties:

- Immutable  
- Export-eligible  
- Legitimacy-bearing  

Forbidden:

- Any mutation  

### CLOSED

- Session explicitly discarded  
- No legitimacy created  

Properties:

- Immutable  
- Non-legitimizing  
- Exportable only as audit context  

Forbidden:

- Reopening  
- Acceptance  

---

## 5. Valid State Transitions

### ENG-SESSION-04 — Allowed Transitions

| From       | To       | Trigger                       |
|------------|----------|-------------------------------|
| CREATED    | ACTIVE   | start_session                 |
| ACTIVE     | PAUSED   | pause                         |
| PAUSED     | ACTIVE   | resume (if valid)             |
| ACTIVE     | BLOCKED  | invariant violation           |
| PAUSED     | BLOCKED  | invariant violation           |
| BLOCKED    | PAUSED   | explicit user action          |
| CREATED    | ABANDONED| abandon                       |
| ACTIVE     | ABANDONED| abandon                       |
| PAUSED     | ABANDONED| abandon                       |
| ACTIVE     | CLOSED   | accept_outcome                |

Fail if:

- Any unlisted transition occurs  
- Transitions happen implicitly  

---

## 6. Authority & Scope Capture

### ENG-SESSION-05 — Authority Is Snapshotted

- At session creation:

  - Active Authority resolution captured  
  - Active Scope resolution captured  
  - Additional referenced scopes may be recorded  

- Values are **immutable** for session lifetime  

Fail if:

- Authority or Scope changes mid-session  
- Resume attempts renegotiation  

---

## 7. Constraint Enforcement

### ENG-SESSION-06 — Constraints Are Immutable

- Constraints declared at session creation  
- Must be visible before first stance  
- Must remain unchanged  

Fail if:

- Constraints added or modified after first stance  
- Constraints inferred implicitly  

---

## 8. Candidate Rules

### ENG-SESSION-07 — Candidate Set Freeze

- Before first stance: candidates may be added  
- After first stance: candidate set is frozen  

Fail if:

- Candidate is added, removed, or edited after first stance  

---

## 9. Stance Rules

### ENG-SESSION-08 — Stances Are Mechanical

- Immutable once recorded  
- Scoped to `(actor_id, candidate_id)`  
- Presence derived mechanically  

Fail if:

- Silence is interpreted as consent  
- Engine infers intent, role, or authority  

---

## 10. Acceptance Evaluation

### ENG-SESSION-09 — Deterministic Acceptance

- Acceptance evaluated mechanically based on:

  - Authority rule  
  - Constraints  
  - Recorded stances  

Fail if:

- Outcome is non-deterministic  

---

## 11. Closing a Session

### ENG-SESSION-10 — Closure Is Explicit

- Session may be CLOSED only when:

  - Acceptance conditions met  
  - User explicitly closes session  

- On closure:

  - Accepted candidates become resolutions  
  - Resolution context is captured  
  - Session becomes immutable  

Fail if:

- Session auto-closes  
- Partial outcomes are persisted  

---

## 12. Blocking Semantics

### ENG-SESSION-11 — Blocking Is Mechanical

- Session enters BLOCKED when:

  - Authority changes  
  - Scope changes  
  - Constraints invalidated  

Fail if:

- Blocked session proceeds silently  

---

## 13. Import Interaction

### ENG-SESSION-12 — Imported Sessions Are Non-Legitimizing

- In CONSOLIDATE mode:

  - Imported sessions must **not** enter ACTIVE  
  - Must **not** accept resolutions  
  - May exist as read-only context  

Fail if:

- Imported sessions affect legitimacy  

---

## 14. Export Eligibility

### ENG-SESSION-13 — Only CLOSED Sessions Are Legitimate

- Only CLOSED sessions:

  - May produce legitimacy-bearing resolutions  
  - May participate in authoritative export  

Fail if:

- ACTIVE, PAUSED, or BLOCKED sessions influence export  

---

## 15. Audit Requirements

### ENG-SESSION-14 — Session Lifecycle Is Audited

Engine **must emit audit events** for:

- Session creation  
- State transitions  
- Closure  
- Abandonment  
- Blocking  

Fail if:

- Session changes are not auditable  

---

## 16. Failure Semantics

### ENG-SESSION-15 — Fail Explicitly

Violations **must**:

- Fail immediately  
- Leave session state unchanged  
- Emit diagnostics  

Fail if:

- Engine attempts silent correction  

---

## 17. Summary Guarantees

- Legitimacy is session-scoped  
- Authority cannot drift  
- Constraints cannot mutate  
- Outcomes are deterministic  
- History is immutable  

---

## Mental Model

- Sessions freeze context  
- Votes are mechanical  
- Acceptance is deterministic  
- Legitimacy is created only at closure