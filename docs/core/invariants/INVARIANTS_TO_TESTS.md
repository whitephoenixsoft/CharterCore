# Charter Core — Acceptance Tests (Engine Only, Invariant-Driven)

Status: CANONICAL  
Applies to: Charter Core Engine  
Test Type: Black-box legitimacy validation  
Purpose: Verify core invariants only. CLI, UX, or ergonomics are excluded.  

---

## A. Core Legitimacy & Determinism

### AT-01 — Explicit Decisions Only
Given:
- An initialized Area with exactly one ACTIVE Authority and one ACTIVE Scope
- A session with candidates and recorded stances
When:
- No explicit acceptance occurs
Then:
- No resolution becomes ACTIVE
- Session remains ACTIVE or BLOCKED
- Audit captures stances only
Fail if:
- A resolution is inferred without acceptance

### AT-02 — Sessions Are the Sole Unit of Legitimacy
Given:
- Candidates exist outside any session
When:
- Attempted acceptance occurs
Then:
- Operation is rejected
- No resolution is created

### AT-03 — Immutable Resolution History
Given:
- Resolution R1 has been accepted
When:
- Attempted mutation, edit, or deletion occurs
Then:
- Operation is rejected
- R1 remains queryable
- Only supersession or retirement via new resolution is allowed

### AT-04 — Deterministic Evaluation
Given:
- Identical session state:
  - Same participants
  - Same stances
  - Same Authority
  - Same Constraints
When:
- Evaluation runs multiple times
Then:
- Outcome is identical
- No stochastic behavior or race conditions allowed

### AT-05 — No Semantic Interpretation
Given:
- Candidates with arbitrary content and optional rationale
When:
- Engine evaluates acceptance
Then:
- Only mechanical rules are applied
- Content, wording, or rationale do not affect outcome

---

## B. Areas, Authority, and Scope

### AT-06 — Areas Are Hard Governance Boundaries
Given:
- Two Areas A and B with independent Authority and Scope
When:
- A session runs in Area A
Then:
- Only Area A’s Authority/Scope govern the session
- Area B has no effect unless explicitly referenced

### AT-07 — Authority Is First-Class and Mechanical
Given:
- Area has an active Authority resolution AUTH1
When:
- New Authority resolution AUTH2 is accepted via session
Then:
- AUTH2 becomes ACTIVE
- AUTH1 becomes SUPERSEDED
- Exactly one active Authority exists at all times

### AT-08 — Scope Is First-Class
Given:
- Area has an active Scope resolution SCOPE1
When:
- New Scope resolution SCOPE2 is accepted
Then:
- SCOPE2 becomes ACTIVE
- SCOPE1 becomes SUPERSEDED
- Exactly one active Scope exists

### AT-09 — Context Preservation
Given:
- Resolution R1 accepted under AUTH1 and SCOPE1
When:
- Authority AUTH2 and Scope SCOPE2 later become active
Then:
- R1 permanently references AUTH1 and SCOPE1
- Historical legitimacy is never reinterpreted

### AT-10 — Area Initialization Is Required
Given:
- An Area with no ACTIVE Authority or Scope
When:
- A non-Authority/Scope resolution is attempted
Then:
- Acceptance is blocked until Area is initialized

---

## C. Session Mechanics & Concurrency

### AT-11 — Authority Fixed Per Session
Given:
- A session has started
Then:
- Exactly one Authority governs
- Authority cannot change mid-session

### AT-12 — Participant Standing Is Action-Based
Given:
- Authority = UNANIMOUS_PRESENT
- Participants: Alice, Bob, Charlie
When:
- Alice and Bob record stances
- Charlie records none
Then:
- Only Alice and Bob are counted for authority evaluation
- Charlie is ignored

### AT-13 — Explicit Dissent Blocks Acceptance
Given:
- UNANIMOUS_PRESENT Authority
- Alice ACCEPT, Bob ACCEPT, Charlie REJECT
Then:
- Resolution cannot be accepted
- Session becomes BLOCKED
- Dissent is auditable

### AT-14 — Blocking & Revalidation
Given:
- Session is PAUSED or BLOCKED
When:
- Authority, Scope, or referenced resolutions change
Then:
- Session cannot resume without explicit revalidation
- Acceptance under changed context is rejected

### AT-15 — Concurrent Sessions Are Isolated
Given:
- Two concurrent sessions in the same Area
- No changes to Authority, Scope, or referenced resolutions
Then:
- Sessions execute independently
- No implicit interference occurs

### AT-16 — Supersession Triggers Revalidation
Given:
- Session S1 references resolution R1
- Another session supersedes R1
Then:
- S1 cannot accept until revalidated

---

## D. Resolution Lifecycle

### AT-17 — Explicit Lifecycle Transitions
Given:
- Resolution R is ACTIVE
When:
- Superseded or retired via session
Then:
- State changes explicitly
- R remains queryable forever
- No silent mutation

### AT-18 — Lifecycle Changes Require Sessions
Given:
- Resolution R is ACTIVE
When:
- API attempts to mark R SUPERSEDED or RETIRED directly
Then:
- Operation is rejected

---

## E. Import / Export & Integrity

### AT-19 — Valid Export Imports Cleanly
Given:
- Engine export generated
When:
- Imported unchanged
Then:
- Import succeeds
- References preserved
- No resolution marked UNDER_REVIEW unnecessarily

### AT-20 — Tampering Is Detected
Given:
- Export is modified
When:
- Attempted import
Then:
- Import fails or affected resolutions enter UNDER_REVIEW

### AT-21 — Structural Integrity Enforced
Given:
- Export with missing references
When:
- Attempted import
Then:
- Import fails
- No partial or inconsistent state is created

### AT-22 — Failed Import Is Side-Effect Free
Given:
- Existing history
When:
- Import fails
Then:
- Existing state remains unchanged

### AT-23 — Flat Import Creates No Legitimacy
Given:
- Resolutions without sessions
When:
- Imported in CONSOLIDATE mode
Then:
- All resolutions are UNDER_REVIEW
- No Authority/Scope inferred
- No ACTIVE resolutions created

---

## F. References (Informational Only)

### AT-REF-01 — References Are Informational
Given:
- Resolutions with references
When:
- Engine evaluates acceptance
Then:
- References never confer authority, precedence, or obligation
- References survive export/import

---

## G. Constraints & Resume Invariants

### AT-24 — Constraints Are Authority-Equivalent
Given:
- Constraints declared at session start
Then:
- Constraints affect agreement evaluation
- Cannot change mid-session or on resume

### AT-25 — Candidate Set Freezes on First Stance
Given:
- Session has candidates
When:
- First stance recorded
Then:
- No add, remove, or edit of candidates allowed

### AT-26 — Resume Cannot Introduce New Legitimacy Rules
Given:
- Paused or blocked session
When:
- Resume attempted
Then:
- Authority and constraints remain unchanged
- Session cannot create legitimacy until revalidated

---

## H. Storage & Identity

### AT-28 — Storage Isolation
Given:
- Multiple storage roots
Then:
- No visibility or references across roots

### AT-29 — Engine Requires Explicit Storage Root
Fail if:
- Engine operates without explicit storage root

### AT-30 — Stable Object Identity
Given:
- Objects created
Then:
- IDs survive restart, export/import, or relocation
- Querying by engine ID returns same object

---

## I. Audit Supremacy

### AT-AUD-01 — Engine Provides Complete Audit Context
Given:
- Session, resolution, or candidate lifecycle events
Then:
- CLI or external system can reconstruct audit using engine-provided context
- Engine exposes timestamps, IDs, acceptance states, supersession

### AT-AUD-02 — Audit Facts Only
Fail if:
- Engine interprets meaning or infers correctness

### AT-AUD-03 — Export Is Deterministically Rehydratable
Given:
- Exported engine snapshot
Then:
- Reimported engine state is logically identical
- Deterministic resolution and session outcomes

---

## J. Voting & Acceptance Semantics

### AT-VA-01 — Votes Without Acceptance Create No Legitimacy
Given:
- Participants vote in session
When:
- No acceptance executed
Then:
- No resolution becomes ACTIVE
- Session remains open

### AT-VA-02 — Acceptance Freezes Votes
Given:
- Session acceptance executed
When:
- Participants attempt to change vote
Then:
- Engine rejects mutation
- Votes remain immutable

### AT-VA-03 — Acceptance Uses Frozen Participant Set
Given:
- Participants frozen at stance start
Then:
- Late joiners are ignored
- Outcome is deterministic

### AT-VA-04 — Acceptance Cannot Be Replayed
Given:
- Session has accepted resolution
When:
- Acceptance attempted again
Then:
- Engine rejects action
- No duplicate ACTIVE resolutions

### AT-VA-05 — Legitimacy Non-Retroactivity
Given:
- Changes to votes, authority, or scope after acceptance
Then:
- Resolution legitimacy remains unchanged

---

## Final Note

These tests cover all engine invariants, are independent of CLI, UX, AI, or filesystem concerns, and are suitable for Python API stress testing, property-based testing, and regression verification.

If all tests pass, the engine fully respects the frozen invariants.

