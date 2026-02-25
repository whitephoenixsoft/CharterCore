# ENG-SESSION — Session Lifecycle, Governance Enforcement & Receipt Emission Specification  
Status: FROZEN (v4 – Receipt Emission & Reconfirmation Integrated)  
Applies to: Engine Core (V1/V2+)  
Scope: Session lifecycle, transitions, governance hygiene, legitimacy boundaries, and receipt emission  

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

# 2. Identity & Purity

## ENG-SESSION-02 — Engine-Assigned Identity

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

# 3. State & Phase Model

Sessions have two independent lifecycle dimensions:

## 3.1 SessionState (Governance Lifecycle)

Exactly one of:

- ACTIVE  
- PAUSED  
- BLOCK_TEMPORARY  
- BLOCK_PERMANENT  
- ACCEPTED  
- CLOSED  

No additional states are permitted.

---

## 3.2 SessionPhase (Voting Lifecycle)

Exactly one of:

- PRE_STANCE  
- VOTING  
- TERMINAL  

Phase is distinct from state.

Rules:

- PRE_STANCE → no votes recorded  
- VOTING → at least one vote recorded OR implicit vote inserted during acceptance (Solo Mode)  
- TERMINAL → state is ACCEPTED or CLOSED  

Fail if:

- Phase and state conflict  
- TERMINAL exists without ACCEPTED or CLOSED  

---

# 4. State Semantics

## 4.1 ACTIVE

- Session is live  
- Participants and candidates may be modified only if phase = PRE_STANCE  
- Votes may be recorded  
- Acceptance may be evaluated  

---

## 4.2 PAUSED

- Explicit user pause  
- No votes may be recorded  
- No acceptance allowed  
- Session may resume if valid  
- Participants and candidates may be modified only if phase = PRE_STANCE  

---

## 4.3 BLOCK_TEMPORARY

- Reversible interruption  
- Votes are cleared upon entry  
- Session may resume explicitly  
- Does not area-block acceptance  

Participants and candidates remain subject to phase rules.

Resume from BLOCK_TEMPORARY requires participant reconfirmation.

---

## 4.4 BLOCK_PERMANENT

- Irreversible governance conflict  
- Votes are preserved  
- Session progression is frozen  
- Acceptance permanently disallowed  

Area Hygiene Rule:

If any session in an Area is BLOCK_PERMANENT:

- No session in that Area may transition to ACCEPTED  

Engine must reject acceptance attempts until all BLOCK_PERMANENT sessions are explicitly CLOSED.

Engine must not:

- Auto-close sessions  
- Auto-resume sessions  
- Auto-consolidate sessions  

Explicit operator action required.

---

## 4.5 ACCEPTED

- Deterministic acceptance succeeded  
- Resolution created  
- Phase becomes TERMINAL  
- Session becomes immutable  

Produces legitimacy.

Produces a LEGITIMACY receipt.

---

## 4.6 CLOSED

- Explicit termination without acceptance  
- No Resolution created  
- Phase becomes TERMINAL  
- Session becomes immutable  

Produces no legitimacy.

Produces an EXPLORATION receipt.

---

# 5. Valid State Transitions

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

No other transitions are permitted.

Fail if:

- Transition not listed occurs  
- Transition happens implicitly  
- BLOCK_PERMANENT transitions to ACTIVE  
- ACCEPTED transitions to any other state  
- CLOSED transitions to any other state  

Blocking transitions are engine-detected, never caller-declared.

---

# 6. Authority & Scope Snapshot

## ENG-SESSION-03 — Context Freeze

At session creation:

- Active Authority Resolution is snapshotted  
- Active Scope Resolution is snapshotted  

These references are immutable for the lifetime of the session.

Fail if:

- Authority changes mid-session  
- Scope changes mid-session  
- Resume renegotiates governance  

---

# 7. Participant & Candidate Freeze Boundary

## ENG-SESSION-04 — PRE_STANCE Is the Only Mutable Boundary

While phase = PRE_STANCE:

- Participants may be added or removed  
- Candidates may be added or removed  
- Constraints may be modified  

When phase transitions to VOTING:

- Participant set becomes frozen  
- Candidate set becomes frozen  
- Constraints become frozen  

Freeze Trigger:

- First recorded vote  
- OR implicit vote inserted during acceptance (Solo Mode)

Fail if:

- Participant modified after VOTING begins  
- Candidate modified after VOTING begins  
- Constraints modified after VOTING begins  

Freeze is phase-driven, not state-driven.

---

# 8. Candidate Rules

## ENG-SESSION-05 — Candidate Set Is Structurally Frozen at VOTING

- Candidates may be added or removed only in PRE_STANCE  
- No candidate edits permitted after VOTING begins  

Fail if:

- Candidate added after VOTING  
- Candidate removed after VOTING  
- Candidate content mutated at any time  

---

# 9. Vote Rules

## ENG-SESSION-06 — Votes Are Mechanical & Immutable

- One vote per participant per candidate  
- Votes immutable once recorded  
- Silence is not interpreted as consent  
- Acceptance is purely mechanical  

Votes are cleared when entering BLOCK_TEMPORARY.

Votes are preserved in BLOCK_PERMANENT.

Fail if:

- Engine infers intent  
- Engine interprets silence (except Solo Mode implicit vote rule)  
- Vote mutated after recording  

---

## ENG-SESSION-06A — Solo Mode Implicit Vote Rule

In Solo Governance Mode (V1):

- Exactly one participant exists in the session  

If acceptance is attempted and:

- No vote exists for the candidate  

Then:

- Engine inserts implicit ACCEPT vote  
- Phase transitions PRE_STANCE → VOTING  
- Governance freezes  
- Vote becomes immutable  

Acceptance cannot occur without a vote object existing.

---

# 10. Pause, Resume & Participant Reconfirmation

## ENG-SESSION-07 — Reconfirmation Requirement

Resume allowed only from:

- PAUSED  
- BLOCK_TEMPORARY  

On resume:

- Votes cleared  
- Participant set must be explicitly re-specified  
- Phase resets to PRE_STANCE  

The Engine must record a participant reconfirmation event.

Reconfirmation events:

- Do not alter legitimacy  
- Do not emit receipts immediately  
- Are included in the eventual session receipt snapshot  

BLOCK_PERMANENT cannot resume.  
ACCEPTED and CLOSED cannot resume.

Fail if:

- Resume occurs without participant reconfirmation  
- Votes persist after resume  

---

# 11. Deterministic Acceptance

## ENG-SESSION-08 — Acceptance Is Pure Evaluation

Acceptance evaluated solely based on:

- Authority rule  
- Constraints  
- Recorded votes (explicit or implicit)  

Given identical inputs, outcome must be identical.

Fail if:

- Acceptance depends on storage order  
- Acceptance depends on runtime timing  
- Acceptance depends on external context  

---

# 12. Receipt Emission Rules

## ENG-SESSION-09 — Mandatory Receipt Emission

The Engine must emit exactly one receipt per session upon transition to TERMINAL.

If state transitions to:

- ACCEPTED → Emit LEGITIMACY receipt  
- CLOSED → Emit EXPLORATION receipt  

Receipt emission must be:

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

- TERMINAL state reached without receipt  
- Receipt snapshot mismatches frozen session state  
- Receipt generated after mutation  

---

# 13. Governance Hygiene Enforcement

## ENG-SESSION-10 — Area Blocking Invariant

If any session in an Area has:

state = BLOCK_PERMANENT

Then:

- No session in that Area may transition to ACCEPTED  

Engine must reject acceptance attempts.

Manual closure required.

---

# 14. Immutability Guarantees

## ENG-SESSION-11 — Terminal States Are Immutable

If state ∈ {ACCEPTED, CLOSED}:

- Session immutable  
- No transitions permitted  
- No participant mutation  
- No candidate mutation  
- No vote mutation  
- No receipt mutation  

Fail if:

- Any mutation occurs after TERMINAL phase  

---

# 15. Audit Requirements

## ENG-SESSION-12 — Lifecycle Is Auditable

Engine must emit audit events for:

- Session creation  
- State transitions  
- Vote recording (explicit or implicit)  
- Resume and participant reconfirmation  
- Acceptance  
- Closure  
- Block entry  

Audit must not:

- Alter legitimacy  
- Replace receipts  
- Create legitimacy  

Audit is descriptive.  
Receipts formalize closure.

---

# 16. Failure Semantics

## ENG-SESSION-13 — Fail Explicitly

Violations must:

- Fail immediately  
- Leave session unchanged  
- Produce structured EvaluationReport  

Engine must not:

- Attempt silent correction  
- Attempt automatic recovery  
- Modify state implicitly  

---

# 17. Summary Guarantees

- Legitimacy is session-scoped  
- Authority and Scope frozen per session  
- PRE_STANCE is sole mutable boundary  
- First vote freezes structure  
- Solo Mode inserts vote, not bypass  
- Resume requires participant reconfirmation  
- Terminal states emit exactly one receipt  
- Receipts capture reconfirmation history  
- Acceptance deterministic  
- Governance hygiene enforced  
- No implicit transitions  

---

# Mental Model

Sessions freeze governance context.

PRE_STANCE is the only mutable boundary.

First vote freezes structure.

Resume resets voting and requires explicit reconfirmation.

Acceptance is deterministic.

Terminal transition emits a receipt.

Receipts preserve closure history.

Nothing changes unless explicitly commanded.