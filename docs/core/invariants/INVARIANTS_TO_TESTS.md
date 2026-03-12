# Charter Core — Updated Acceptance Tests (Engine Only, Invariant-Driven)

**Status:** FROZEN (V4 – Deterministic, Invariant-Complete, Extended)  
**Applies to:** Charter Core Engine  
**Test Type:** Black-box legitimacy validation  
**Purpose:** Verify core invariants, session mechanics, governance, and API correctness. CLI/UX excluded.

---

## A. Core Legitimacy & Determinism

### AT-01 — Explicit Decisions Only
**Given:** Initialized Area with one ACTIVE Authority and Scope, session with candidates and stances  
**When:** No explicit acceptance occurs  
**Then:**  
- No resolution becomes ACTIVE  
- Session remains ACTIVE or BLOCKED  
- Audit captures stances only  
**Fail if:** Any resolution is inferred  
**Invariant Alignment:** ENG-LEG-01, ENG-API-15

### AT-02 — Sessions Are the Sole Unit of Legitimacy
**Given:** Candidates outside any session  
**When:** Attempted acceptance occurs  
**Then:** Operation is rejected  
**Fail if:** Resolution created outside session  
**Invariant Alignment:** ENG-LEG-02, ENG-INITIALIZATION

### AT-03 — Immutable Resolution History
**Given:** Resolution R1 accepted  
**When:** Attempted mutation, edit, or deletion  
**Then:** Operation rejected, R1 queryable  
**Fail if:** Historical resolution altered without supersession  
**Invariant Alignment:** ENG-ID-02, ENG-HIST-01

### AT-04 — Deterministic Evaluation
**Given:** Identical session state (participants, stances, Authority, Constraints)  
**When:** Evaluation runs multiple times  
**Then:** Outcome identical, no stochastic behavior  
**Invariant Alignment:** ENG-LEG-04, ENG-INTEGRITY

### AT-05 — No Semantic Interpretation
**Given:** Candidates with arbitrary content/rationale  
**When:** Engine evaluates acceptance  
**Then:** Only mechanical rules applied  
**Invariant Alignment:** ENG-LEG-01, ENG-API-15

---

## B. Areas, Authority, and Scope

### AT-06 — Areas Are Hard Governance Boundaries
**Given:** Areas A and B with independent Authority/Scope  
**When:** Session runs in Area A  
**Then:** Only Area A Authority/Scope govern session  
**Invariant Alignment:** ENG-AREA-01, ENG-INTEGRITY

### AT-07 — Authority Is First-Class and Mechanical
**Given:** ACTIVE Authority AUTH1  
**When:** New Authority AUTH2 accepted via session  
**Then:** AUTH2 ACTIVE, AUTH1 SUPERSEDED  
**Invariant Alignment:** ENG-AUTH-01, ENG-SUP-02

### AT-08 — Scope Is First-Class
**Given:** ACTIVE Scope SCOPE1  
**When:** New Scope SCOPE2 accepted via session  
**Then:** SCOPE2 ACTIVE, SCOPE1 SUPERSEDED  
**Invariant Alignment:** ENG-SCOPE-01

### AT-09 — Context Preservation
**Given:** Resolution R1 accepted under AUTH1/SCOPE1  
**When:** Authority/SCOPE later changes  
**Then:** R1 permanently references AUTH1/SCOPE1  
**Invariant Alignment:** ENG-CONTEXT-01, ENG-INTEGRITY

### AT-10 — Area Initialization / SOLE_ACTOR Bootstrapping
**Given:** New Area with no Authority  
**When:** First session created  
**Then:** Session assumes temporary SOLE_ACTOR Authority  
**Then:** Authority recorded in DAG for audit  
**Fail if:** First session fails to establish temporary Authority  
**Invariant Alignment:** ENG-AREA-02, ENG-INTEGRITY

---

## C. Session Mechanics & Concurrency

### AT-11 — Authority Fixed Per Session
**Given:** Session started  
**Then:** Exactly one Authority governs session, cannot change mid-session  
**Invariant Alignment:** ENG-CONCUR-01, ENG-AUTH-01

### AT-12 — Participant Standing Is Action-Based
**Given:** Authority UNANIMOUS_PRESENT, Participants Alice/Bob/Charlie  
**When:** Alice/Bob record stances, Charlie none  
**Then:** Only Alice/Bob counted for evaluation  
**Invariant Alignment:** ENG-LEG-05

### AT-13 — Explicit Dissent Blocks Acceptance
**Given:** UNANIMOUS_PRESENT Authority, Alice/Bob ACCEPT, Charlie REJECT  
**Then:** Session blocked, resolution not accepted  
**Invariant Alignment:** ENG-ACCEPT-01, ENG-SUP-04

### AT-14 — Blocking & Revalidation
**Given:** Session PAUSED/BLOCKED  
**When:** Authority/Scope changes  
**Then:** Session cannot resume without revalidation  
**Invariant Alignment:** ENG-SUP-04, ENG-SUP-05

### AT-15 — Concurrent Sessions Are Isolated
**Given:** Two concurrent sessions in same Area, no context changes  
**Then:** Sessions execute independently  
**Invariant Alignment:** ENG-CONCUR-01

### AT-16 — Supersession Triggers Revalidation
**Given:** Session S1 references resolution R1, R1 superseded in S2  
**Then:** S1 cannot accept until revalidated  
**Invariant Alignment:** ENG-SUP-04, ENG-SUP-05

---

## D. Resolution Lifecycle

### AT-17 — Explicit Lifecycle Transitions
**Given:** Resolution R ACTIVE  
**When:** Superseded/Retired via session  
**Then:** State changes explicit, R remains queryable  
**Invariant Alignment:** ENG-HIST-02, ENG-SUP-01

### AT-18 — Lifecycle Changes Require Sessions
**Given:** Resolution R ACTIVE  
**When:** Attempted direct mutation to SUPERSEDED/RETIRED  
**Then:** Operation rejected  
**Invariant Alignment:** ENG-HIST-02, ENG-API rules

---

## E. Import / Export & Integrity

### AT-19 — Valid Export Imports Cleanly
**Given:** Engine export  
**When:** Imported unchanged  
**Then:** Import succeeds, references preserved  
**Invariant Alignment:** ENG-IMP-01, ENG-API-22

### AT-20 — Tampering Detection
**Given:** Modified export  
**When:** Attempted import  
**Then:** Import fails or affected resolutions enter UNDER_REVIEW  
**Invariant Alignment:** ENG-INTEGRITY degraded mode, ENG-API-26

### AT-21 — Structural Integrity Enforced
**Given:** Export with missing references  
**When:** Attempted import  
**Then:** Operation rejected, no partial state created  
**Invariant Alignment:** ENG-INITIALIZATION, ENG-INTEGRITY

### AT-22 — Failed Import Side-Effect Free
**Given:** Existing history  
**When:** Import fails  
**Then:** Existing state unchanged  
**Invariant Alignment:** ENG-INTEGRITY

### AT-23 — Flat Import Creates No Legitimacy
**Given:** Resolutions without sessions  
**When:** Imported in CONSOLIDATE mode  
**Then:** All resolutions UNDER_REVIEW  
**Invariant Alignment:** ENG-IMP-01, ENG-API rules

---

## F. References (Informational Only)

### AT-REF-01 — References Are Informational
**Given:** Resolutions with references  
**When:** Engine evaluates acceptance  
**Then:** References do not confer authority or precedence  
**Invariant Alignment:** ENG-INTEGRITY, ENG-INITIALIZATION

---

## G. Constraints & Resume Invariants

### AT-24 — Constraints Are Authority-Equivalent
**Given:** Session start  
**Then:** Constraints affect agreement evaluation, cannot change mid-session  
**Invariant Alignment:** ENG-CON-01, ENG-SES-03

### AT-25 — Candidate Set Freezes on First Stance
**Given:** Session with candidates  
**When:** First stance recorded  
**Then:** Candidates frozen, no add/remove/edit  
**Invariant Alignment:** ENG-SES-02, ENG-API PRE_STANCE

### AT-26 — Resume Cannot Introduce New Legitimacy Rules
**Given:** Paused or blocked session  
**When:** Resume attempted  
**Then:** Authority/constraints unchanged, session cannot create legitimacy until revalidated  
**Invariant Alignment:** ENG-SES-03, ENG-API-08

---

## H. Identity

### AT-27 — Stable Object Identity
**Given:** Created objects  
**Then:** IDs survive restart, export/import, relocation  
**Invariant Alignment:** ENG-ID-01, ENG-INTEGRITY

---

## I. Audit Supremacy

### AT-AUD-01 — Complete Audit Context
**Given:** Session/resolution/candidate lifecycle events  
**Then:** External system can reconstruct audit  
**Invariant Alignment:** ENG-API audit rules

### AT-AUD-02 — Audit Facts Only
**Fail if:** Engine interprets meaning  
**Invariant Alignment:** ENG-LEG-05, ENG-API-05

### AT-AUD-03 — Export Deterministically Rehydratable
**Given:** Export snapshot  
**Then:** Reimported engine state identical  
**Invariant Alignment:** ENG-INTEGRITY, ENG-API-22

---

## J. Voting & Acceptance Semantics

### AT-VA-01 — Votes Without Acceptance Create No Legitimacy
**Given:** Participants vote  
**When:** No acceptance executed  
**Then:** No ACTIVE resolution created  
**Invariant Alignment:** ENG-VOTE-01, ENG-API-15

### AT-VA-02 — Acceptance Freezes Votes
**Given:** Session accepted  
**Then:** Votes immutable, mutations rejected  
**Invariant Alignment:** ENG-INTEGRITY, ENG-API-15

### AT-VA-03 — Acceptance Uses Frozen Participant Set
**Given:** Participants frozen at stance start  
**Then:** Late joiners ignored, deterministic outcome  
**Invariant Alignment:** ENG-INTEGRITY

### AT-VA-04 — Acceptance Cannot Be Replayed
**Given:** Session accepted  
**Then:** Re-attempt rejected, no duplicate ACTIVE resolutions  
**Invariant Alignment:** ENG-ID-02, ENG-INTEGRITY

### AT-VA-05 — Legitimacy Non-Retroactivity
**Given:** Votes, Authority, Scope changed post-acceptance  
**Then:** Past resolution legitimacy unchanged  
**Invariant Alignment:** ENG-ACCEPT-04, ENG-INTEGRITY

---

## K. Degraded Mode

### AT-DM-01 — Degraded Read-Only Mode Enforcement
**Given:** Missing optional artifacts or audit unavailability  
**When:** Mutating command executed  
**Then:** Engine rejects with DEGRADED_MODE_ACTIVE  
**Then:** Read-only queries, evaluation, and DAG export still permitted  
**Invariant Alignment:** ENG-INTEGRITY 10, ENG-API-08
