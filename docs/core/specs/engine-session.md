# ENG-SESSION — Session Lifecycle, Governance Enforcement & Receipt Emission Specification  
Status: FROZEN (v7 – Deterministic Multi-Error Acceptance Clarified)  
Applies to: Engine Core (V1/V2+)  
Scope: Session lifecycle, session classification, governance enforcement, legitimacy boundaries, deterministic acceptance, and receipt emission  

Authority: Subordinate to ENG-DOMAIN, ENG-DECISION, ENG-RECEIPT, ENG-INTEGRITY  

---

# 1. Purpose

## ENG-SESSION-01 — Sessions Are the Sole Legitimacy Mechanism

Sessions are the only structure through which legitimacy may be created.

All:

- Resolution acceptance  
- Supersession  
- Authority evolution  
- Scope evolution  

must occur through a valid session.

Receipts formalize session closure but do not create legitimacy.

Fail if:

- A Resolution is created outside a session  
- Legitimacy emerges without session acceptance  
- Supersession occurs without a session  

---

# 2. Session Classification & Bootstrap Constraints

## ENG-SESSION-02 — Session Type Is Explicit and Immutable

Each session is classified at creation as exactly one of:

- AUTHORITY  
- SCOPE  
- REGULAR  

Session type:

- Required at creation  
- Immutable for the lifetime of the session  
- Determines governance preconditions  

Fail if:

- Session type changes  
- Session type undefined  

---

## ENG-SESSION-03 — Creation Preconditions

### AUTHORITY Session

Allowed only if:

- No ACTIVE Authority exists  
  OR  
- A current ACTIVE Authority exists and supersession rules apply  

Fail if:

- Multiple ACTIVE Authority exist  
- Parallel Authority attempted without supersession  

---

### SCOPE Session

Allowed only if:

- Exactly one ACTIVE Authority exists  

Fail if:

- No ACTIVE Authority  
- Multiple ACTIVE Authority  

---

### REGULAR Session

Allowed only if:

- Exactly one ACTIVE Authority  
- Exactly one ACTIVE Scope  

Fail if:

- Authority missing  
- Scope missing  
- Governance structurally invalid  

---

# 3. Identity & Participation Epochs

## ENG-SESSION-04 — Engine-Assigned Identity

- Every session has a UUIDv7 `session_id`  
- Engine generates all session IDs  
- Identity immutable  

Fail if:

- Caller-generated identity  
- Identity mutation  

---

## ENG-SESSION-04A — Participant Identity as Participation Epoch

- Each `participant_id` represents a single participation epoch  
- Engine-generated UUIDv7  
- Never reused within a session lifecycle  
- Terminated explicitly (removal or resume reset)  

Fail if:

- Reuse occurs  
- Terminated epoch reactivated  
- Vote references non-active epoch  

---

# 4. State & Phase Model

## 4.1 SessionState

Exactly one of:

- ACTIVE  
- PAUSED  
- BLOCK_TEMPORARY  
- BLOCK_PERMANENT  
- ACCEPTED  
- CLOSED  

---

## 4.2 SessionPhase

Exactly one of:

- PRE_STANCE  
- VOTING  
- TERMINAL  

Rules:

- PRE_STANCE → no votes  
- VOTING → at least one vote or implicit Solo vote  
- TERMINAL → state ∈ {ACCEPTED, CLOSED}  

Fail if inconsistent.

---

# 5. State Semantics

(Sections 5.1–5.6 unchanged in behavior; deterministic enforcement implied.)

---

# 6. Valid State Transitions

Only listed transitions permitted (unchanged from v6).

Blocking transitions are engine-detected only.  
No implicit transitions permitted.

---

# 7. Governance Context Freeze

## ENG-SESSION-05 — Governance Snapshot

At session creation:

- ACTIVE Authority Resolution snapshotted  
- ACTIVE Scope Resolution snapshotted  

Immutable for session lifetime.

Fail if governance renegotiated mid-session.

---

# 8. Freeze Boundary

## ENG-SESSION-06 — PRE_STANCE Is Sole Mutable Boundary

While PRE_STANCE:

- Participants mutable  
- Candidates mutable  
- Constraints mutable  

On transition to VOTING:

- Participant set frozen  
- Candidate set frozen  
- Constraints frozen  

Triggered by:

- First recorded vote  
- OR implicit Solo vote  

Fail if mutation after freeze.

---

# 9. Vote Rules

## ENG-SESSION-08 — Votes Are Mechanical

- One vote per participant epoch per candidate  
- Immutable once recorded  
- Silence is not consent  
- Acceptance purely mechanical  

Votes cleared on BLOCK_TEMPORARY.  
Votes preserved on BLOCK_PERMANENT.

Fail if engine infers intent or mutates votes.

---

## ENG-SESSION-08A — Solo Mode Implicit Vote

If exactly one participant epoch exists:

- Acceptance attempt inserts implicit ACCEPT if absent  
- PRE_STANCE → VOTING  
- Governance freezes  

Acceptance still requires vote object.

---

# 10. Resume & Reconfirmation

## ENG-SESSION-09 — Epoch Reset on Resume

Resume allowed only from:

- PAUSED  
- BLOCK_TEMPORARY  

On resume:

- All prior participation epochs terminated  
- Participant set cleared  
- New participant IDs generated  
- Votes cleared  
- Phase resets to PRE_STANCE  

Fail if prior participant_id reused or votes persist.

---

# 11. Deterministic Acceptance

## ENG-SESSION-10 — Full Validation, No Short-Circuit

Acceptance evaluation must:

- Execute a full deterministic validation pass  
- Evaluate all applicable governance, constraint, vote, and block rules  
- Accumulate all detected violations  
- Never short-circuit on first violation  
- Derive outcome only after evaluation completes  

Acceptance depends solely on:

- Governance preconditions  
- Authority rule  
- Constraints  
- Recorded votes  
- Block state  

Identical inputs → identical:

- Violation set  
- Violation ordering  
- Outcome  

Fail if:

- Acceptance depends on timing  
- Acceptance depends on storage order  
- Acceptance depends on external context  
- Evaluation stops after first detected violation  

Outcome derivation must follow ENG-ERROR precedence rules.

---

# 12. Receipt Emission

## ENG-SESSION-11 — Mandatory Receipt Emission

Exactly one receipt per session upon TERMINAL transition.

If:

- ACCEPTED → LEGITIMACY receipt  
- CLOSED → EXPLORATION receipt  

Receipt emission:

- Atomic with terminal transition  
- Deterministic  
- Immutable  

Receipt snapshot must match frozen session state, including:

- Final participant epoch set  
- Candidate snapshot  
- Stance snapshot  
- Participant reconfirmation history  
- Authority snapshot reference  
- Scope snapshot reference  
- Annotations  
- Deterministic content_hash  

Fail if mismatch.

---

# 13. Governance Hygiene

## ENG-SESSION-12 — Area Blocking Invariant

If any session in Area is BLOCK_PERMANENT:

- No session may transition to ACCEPTED  

Engine must reject acceptance attempts after full validation pass.

---

# 14. Immutability

## ENG-SESSION-13 — Terminal States Immutable

If state ∈ {ACCEPTED, CLOSED}:

- No transitions  
- No participant mutation  
- No candidate mutation  
- No vote mutation  
- No receipt mutation  

---

# 15. Audit

## ENG-SESSION-14 — Lifecycle Auditable

Engine must emit audit events for lifecycle actions.

Audit:

- Descriptive only  
- Does not create legitimacy  
- Does not replace receipts  

---

# 16. Failure Semantics

## ENG-SESSION-15 — Explicit Failure, No Implicit Repair

Violations must:

- Fail deterministically  
- Leave session unchanged  
- Produce structured EvaluationReport  

Engine must not:

- Short-circuit evaluation  
- Attempt silent correction  
- Attempt automatic recovery  
- Mutate state implicitly  

---

# 17. Summary Guarantees

- Sessions sole legitimacy mechanism  
- Governance bootstrap enforced  
- Governance context frozen at creation  
- PRE_STANCE sole mutable boundary  
- First vote freezes structure  
- Resume resets participation epochs  
- No participant_id reuse  
- Acceptance uses full deterministic validation  
- No short-circuit evaluation  
- Terminal states emit exactly one receipt  
- Governance hygiene enforced  
- No implicit transitions  

---

# Mental Model

Sessions freeze governance context at creation.

PRE_STANCE is the only mutable boundary.

First vote freezes structure.

Resume terminates prior participation epochs.

Acceptance is a deterministic, full validation pass.

Terminal transition emits a receipt.

Nothing changes unless explicitly commanded.