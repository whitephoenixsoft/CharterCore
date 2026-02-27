# ENG-SESSION — Session Lifecycle, Governance Enforcement & Receipt Emission Specification  
Status: FROZEN (v5 – Governance Bootstrap Integrated)  
Applies to: Engine Core (V1/V2+)  
Scope: Session lifecycle, session classification, governance enforcement, legitimacy boundaries, and receipt emission  

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

- Is required at creation  
- Is immutable for the lifetime of the session  
- Determines governance preconditions  

Fail if:

- Session type changes after creation  
- Session type is undefined  

---

## ENG-SESSION-03 — Creation Preconditions

Session creation must enforce governance bootstrap rules.

### AUTHORITY Session Creation

Allowed only if:

- No ACTIVE Authority exists in the Area  
  OR  
- A current ACTIVE Authority exists and the session intends to supersede it under standard supersession rules.

Authority sessions do not bypass governance rules.  
They are subject to the current Authority if one exists.

Fail if:

- Multiple ACTIVE Authority already exist  
- Parallel Authority attempted without supersession  

---

### SCOPE Session Creation

Allowed only if:

- Exactly one ACTIVE Authority exists in the Area.

Fail if:

- No ACTIVE Authority exists  
- Multiple ACTIVE Authority exist  

---

### REGULAR Session Creation

Allowed only if:

- Exactly one ACTIVE Authority exists  
- Exactly one ACTIVE Scope exists  

Fail if:

- Authority not initialized  
- Scope not initialized  
- Governance structurally invalid  

No special-case evaluation logic exists.  
Only governance preconditions apply.

---

# 3. Identity & Purity

## ENG-SESSION-04 — Engine-Assigned Identity

- Every session has a UUIDv7 `session_id`  
- The Engine generates all session IDs  
- Identity is stable for the lifetime of the session  

Identity must not depend on:

- Storage  
- Hashing  
- File location  
- Import/export mechanics  

Fail if:

- Session identity is caller-generated  
- Identity changes after creation  

---

# 4. State & Phase Model

Sessions have two independent lifecycle dimensions:

## 4.1 SessionState (Governance Lifecycle)

Exactly one of:

- ACTIVE  
- PAUSED  
- BLOCK_TEMPORARY  
- BLOCK_PERMANENT  
- ACCEPTED  
- CLOSED  

No additional states permitted.

---

## 4.2 SessionPhase (Voting Lifecycle)

Exactly one of:

- PRE_STANCE  
- VOTING  
- TERMINAL  

Rules:

- PRE_STANCE → no votes recorded  
- VOTING → at least one vote recorded OR implicit vote inserted (Solo Mode)  
- TERMINAL → state is ACCEPTED or CLOSED  

Fail if:

- Phase and state conflict  
- TERMINAL exists without ACCEPTED or CLOSED  

---

# 5. State Semantics

## 5.1 ACTIVE

- Session is live  
- Participants and candidates modifiable only if phase = PRE_STANCE  
- Votes may be recorded  
- Acceptance may be evaluated  

---

## 5.2 PAUSED

- Explicit user pause  
- No votes may be recorded  
- No acceptance allowed  
- Session may resume if valid  
- Structural mutation only if phase = PRE_STANCE  

---

## 5.3 BLOCK_TEMPORARY

- Reversible interruption  
- Votes cleared upon entry  
- Session may resume explicitly  
- Does not area-block acceptance  

Resume requires participant reconfirmation.

---

## 5.4 BLOCK_PERMANENT

- Irreversible governance conflict  
- Votes preserved  
- Session progression frozen  
- Acceptance permanently disallowed  

Area Hygiene Rule:

If any session in an Area is BLOCK_PERMANENT:

- No session in that Area may transition to ACCEPTED  

Manual closure required.

Engine must not:

- Auto-close  
- Auto-resume  
- Auto-consolidate  

---

## 5.5 ACCEPTED

- Deterministic acceptance succeeded  
- Resolution created  
- Phase becomes TERMINAL  
- Session immutable  
- Emit LEGITIMACY receipt  

---

## 5.6 CLOSED

- Explicit termination without acceptance  
- No Resolution created  
- Phase becomes TERMINAL  
- Session immutable  
- Emit EXPLORATION receipt  

---

# 6. Valid State Transitions

Only the following transitions are permitted:

| From            | To                | Trigger                          |
|-----------------|-------------------|----------------------------------|
| ACTIVE          | PAUSED            | explicit pause                   |
| PAUSED          | ACTIVE            | explicit resume (valid)          |
| ACTIVE          | BLOCK_TEMPORARY   | invariant-triggered interruption |
| PAUSED          | BLOCK_TEMPORARY   | invariant-triggered interruption |
| BLOCK_TEMPORARY | ACTIVE            | explicit resume                  |
| ACTIVE          | BLOCK_PERMANENT   | invariant-triggered conflict     |
| PAUSED          | BLOCK_PERMANENT   | invariant-triggered conflict     |
| BLOCK_PERMANENT | CLOSED            | explicit closure                 |
| ACTIVE          | ACCEPTED          | deterministic acceptance         |
| ACTIVE          | CLOSED            | explicit closure                 |
| PAUSED          | CLOSED            | explicit closure                 |
| BLOCK_TEMPORARY | CLOSED            | explicit closure                 |

No other transitions permitted.

Fail if:

- Unlisted transition occurs  
- Transition happens implicitly  
- BLOCK_PERMANENT transitions to ACTIVE  
- ACCEPTED or CLOSED transitions further  

Blocking transitions are engine-detected only.

---

# 7. Authority & Scope Snapshot

## ENG-SESSION-05 — Governance Context Freeze

At session creation:

- ACTIVE Authority Resolution is snapshotted  
- ACTIVE Scope Resolution is snapshotted  

These references are immutable for the lifetime of the session.

Fail if:

- Authority changes mid-session  
- Scope changes mid-session  
- Resume renegotiates governance  

---

# 8. Participant & Candidate Freeze Boundary

## ENG-SESSION-06 — PRE_STANCE Is the Only Mutable Boundary

While phase = PRE_STANCE:

- Participants may be added/removed  
- Candidates may be added/removed  
- Constraints may be modified  

When phase transitions to VOTING:

- Participant set frozen  
- Candidate set frozen  
- Constraints frozen  

Freeze triggered by:

- First recorded vote  
- OR implicit vote insertion (Solo Mode)

Fail if:

- Mutation occurs after VOTING begins  

---

# 9. Candidate Rules

## ENG-SESSION-07 — Candidate Set Frozen at VOTING

- Candidates modifiable only in PRE_STANCE  
- No edits after VOTING  

Fail if:

- Candidate added after VOTING  
- Candidate removed after VOTING  
- Candidate content mutated at any time  

---

# 10. Vote Rules

## ENG-SESSION-08 — Votes Are Mechanical

- One vote per participant per candidate  
- Votes immutable once recorded  
- Silence is not consent  
- Acceptance purely mechanical  

Votes cleared when entering BLOCK_TEMPORARY.  
Votes preserved in BLOCK_PERMANENT.

Fail if:

- Engine infers intent  
- Engine mutates recorded vote  

---

## ENG-SESSION-08A — Solo Mode Implicit Vote Rule

In Solo Mode:

- Exactly one participant exists  

If acceptance attempted with no vote:

- Engine inserts implicit ACCEPT  
- Phase transitions PRE_STANCE → VOTING  
- Governance freezes  
- Vote immutable  

Acceptance requires a vote object.

---

# 11. Pause, Resume & Participant Reconfirmation

## ENG-SESSION-09 — Reconfirmation Requirement

Resume allowed only from:

- PAUSED  
- BLOCK_TEMPORARY  

On resume:

- Votes cleared  
- Participant set explicitly re-specified  
- Phase resets to PRE_STANCE  

Engine must record reconfirmation event.

Reconfirmation:

- Does not create legitimacy  
- Does not emit receipt immediately  
- Included in final receipt snapshot  

BLOCK_PERMANENT, ACCEPTED, CLOSED cannot resume.

Fail if:

- Resume without reconfirmation  
- Votes persist after resume  

---

# 12. Deterministic Acceptance

## ENG-SESSION-10 — Acceptance Is Pure Evaluation

Acceptance depends solely on:

- Governance preconditions (ENG-DECISION)  
- Authority rule  
- Constraints  
- Recorded votes  

Identical inputs → identical outcomes.

Fail if:

- Acceptance depends on timing  
- Acceptance depends on storage order  
- Acceptance depends on external context  

---

# 13. Receipt Emission Rules

## ENG-SESSION-11 — Mandatory Receipt Emission

Exactly one receipt per session upon TERMINAL transition.

If:

- ACCEPTED → LEGITIMACY receipt  
- CLOSED → EXPLORATION receipt  

Receipt emission:

- Atomic with terminal transition  
- Deterministic  
- Immutable  

Receipt must capture:

- session_id  
- final state  
- participant snapshot  
- candidate snapshot  
- stance snapshot  
- participant reconfirmation history  
- authority snapshot reference  
- scope snapshot reference  
- annotations  
- content_hash  

Fail if:

- TERMINAL reached without receipt  
- Receipt mismatches frozen state  

---

# 14. Governance Hygiene Enforcement

## ENG-SESSION-12 — Area Blocking Invariant

If any session in an Area has:

state = BLOCK_PERMANENT

Then:

- No session in that Area may transition to ACCEPTED  

Engine must reject acceptance attempts.

Manual closure required.

---

# 15. Immutability Guarantees

## ENG-SESSION-13 — Terminal States Immutable

If state ∈ {ACCEPTED, CLOSED}:

- Session immutable  
- No transitions permitted  
- No participant mutation  
- No candidate mutation  
- No vote mutation  
- No receipt mutation  

Fail if mutation occurs post-terminal.

---

# 16. Audit Requirements

## ENG-SESSION-14 — Lifecycle Auditable

Engine must emit audit events for:

- Session creation  
- State transitions  
- Vote recording (explicit or implicit)  
- Resume and reconfirmation  
- Acceptance  
- Closure  
- Block entry  

Audit:

- Does not alter legitimacy  
- Does not replace receipts  
- Does not create legitimacy  

Audit descriptive.  
Receipts formalize closure.

---

# 17. Failure Semantics

## ENG-SESSION-15 — Fail Explicitly

Violations must:

- Fail immediately  
- Leave session unchanged  
- Produce structured EvaluationReport  

Engine must not:

- Attempt silent correction  
- Attempt automatic recovery  
- Modify state implicitly  

---

# 18. Summary Guarantees

- Sessions classified at creation  
- Governance bootstrap enforced  
- Authority and Scope snapshotted  
- PRE_STANCE sole mutable boundary  
- First vote freezes structure  
- Resume requires reconfirmation  
- Terminal states emit exactly one receipt  
- Acceptance deterministic  
- Governance hygiene enforced  
- No implicit transitions  

---

# Mental Model

Sessions freeze governance context at creation.

Governance must exist before regular legitimacy can begin.

PRE_STANCE is the only mutable boundary.

First vote freezes structure.

Resume resets voting and requires reconfirmation.

Acceptance is deterministic.

Terminal transition emits a receipt.

Nothing changes unless explicitly commanded.