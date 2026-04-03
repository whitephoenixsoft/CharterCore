# Charter Core — Acceptance Tests (Engine Core, Spec-Alitygned)

Status: REFACTORED (V6 – Structure/Usability Alignment, Solo-Mode Removal & Determinism Closure)  
Applies to: Charter Core Engine  
Test Type: Black-box library acceptance testing  
Purpose: Verify constitutional invariants, runtime behavior, structural safety, and interface correctness. CLI and UX concerns excluded.

---

## 1. Purpose

These acceptance tests validate the Engine as a deterministic legitimacy compiler.

They verify:

- legitimacy is created only through explicit session acceptance
- runtime behavior is deterministic
- structural integrity is enforced
- receipts and structural evolution remain consistent
- API commands respect mutation and reporting boundaries
- import, rehydration, and degraded mode behave correctly

These tests do not validate:

- CLI workflow
- user review workflow
- file envelope handling
- operator ergonomics
- external audit pipelines

---

## 2. Test Model

The tests are black-box from the library boundary.

They exercise:

- public Engine APIs
- runtime rehydration
- evaluation commands
- mutation commands
- export behavior
- degraded mode behavior

Tests must assert:

- deterministic outcomes
- deterministic EvaluationReports
- absence of hidden mutation
- structural and legitimacy consistency

Where a test expects failure, it must also assert:

- no unauthorized state mutation occurred
- no unauthorized artifact was created
- reporting remained deterministic

---

## A. Core Legitimacy & Determinism

### AT-01 — Explicit Decisions Only

Given:

- a successfully rehydrated Area runtime
- exactly one structurally valid active Authority
- exactly one structurally valid active Scope
- a session containing candidates and votes

When:

- no explicit acceptance command is executed

Then:

- no Resolution is created
- no LEGITIMACY receipt is emitted
- the session remains non-terminal unless separately closed
- legitimacy is not inferred from votes alone

Fail if:

- any legitimacy artifact appears without acceptance

Alignment:

- ENG-LEG-01
- ENG-LEG-02
- ENG-ACCEPT-01
- ENG-API

---

### AT-02 — Sessions Are the Sole Legitimacy Mechanism

Given:

- proposal-like content exists outside any session

When:

- legitimacy is attempted without a valid session acceptance path

Then:

- the operation is rejected
- no Resolution is created
- no receipt is emitted

Fail if:

- legitimacy is created outside a session

Alignment:

- ENG-LEG-02
- ENG-SESSION
- ENG-DECISION
- ENG-API

---

### AT-03 — Immutable Resolution History

Given:

- an accepted Resolution exists

When:

- direct mutation, deletion, or overwrite is attempted

Then:

- the operation is rejected
- the historical Resolution remains queryable
- accepted context remains unchanged

Fail if:

- accepted history is rewritten in place

Alignment:

- ENG-ID-02
- ENG-HIST-01
- ENG-DOMAIN
- ENG-API

---

### AT-04 — Deterministic Evaluation

Given:

- identical runtime state
- identical session state
- identical command input

When:

- evaluation is executed repeatedly

Then:

- EvaluationReports are identical in outcome
- errors appear in identical order
- no stochastic variation occurs

Alignment:

- ENG-LEG-04
- ENG-ERROR
- ENG-API
- ENG-CORE-PURITY

---

### AT-05 — No Semantic Interpretation

Given:

- candidates contain arbitrary wording, annotation, or content

When:

- the Engine evaluates or attempts acceptance

Then:

- only mechanical rules affect the result
- wording does not affect legitimacy outcome

Fail if:

- semantic interpretation changes evaluation outcome

Alignment:

- ENG-CORE-01
- ENG-LEG-01
- ENG-DECISION

---

## B. Runtime Entry, Area Boundaries & Governance

### AT-06 — Areas Are Hard Governance Boundaries

Given:

- two Area graphs with separate governance artifacts

When:

- one Area is rehydrated into runtime

Then:

- only that Area participates in legitimacy evaluation
- external Area objects have no governing effect

Fail if:

- legitimacy depends on a foreign Area

Alignment:

- ENG-AREA-01
- ENG-CORE-03
- ENG-INTEGRITY
- ENG-INITIALIZATION

---

### AT-07 — Rehydration Requires a Complete Single-Area Structural Graph

Given:

- a host submits a graph with missing structural references or mixed Area objects

When:

- rehydrate_engine is called

Then:

- initialization fails deterministically
- runtime is not entered
- no partial Area runtime is created

Alignment:

- ENG-INITIALIZATION
- ENG-INTEGRITY
- ENG-DOMAIN
- ENG-API

---

### AT-08 — Authority Is First-Class and Mechanical

Given:

- an Area runtime with an active Authority Resolution AUTH1

When:

- a valid Authority supersession is accepted producing AUTH2

Then:

- AUTH2 becomes structurally active
- AUTH1 becomes superseded
- historical legitimacy under AUTH1 remains unchanged

Alignment:

- ENG-AUTH-01
- ENG-STRUCTURE
- ENG-DECISION
- ENG-RECEIPT

---

### AT-09 — Scope Is First-Class

Given:

- an Area runtime with an active Scope Resolution SCOPE1

When:

- a valid Scope supersession is accepted producing SCOPE2

Then:

- SCOPE2 becomes structurally active
- SCOPE1 becomes superseded
- historical legitimacy under SCOPE1 remains unchanged

Alignment:

- ENG-SCOPE-01
- ENG-STRUCTURE
- ENG-DECISION
- ENG-RECEIPT

---

### AT-10 — Context Preservation

Given:

- Resolution R1 was accepted under Authority AUTH1 and Scope SCOPE1

When:

- governance later changes

Then:

- R1 permanently retains AUTH1 and SCOPE1 as historical context
- later governance changes do not rewrite R1

Alignment:

- ENG-CONTEXT-01
- ENG-ACCEPT-04
- ENG-RECEIPT
- ENG-DOMAIN

---

### AT-11 — Governed Runtime Is Required for Ordinary Legitimacy Evaluation

Given:

- a runtime graph lacking required governance completeness for ordinary evaluation

When:

- a governance-dependent session acceptance is attempted

Then:

- acceptance is rejected or runtime entry fails according to the governing specs
- no legitimacy is created

Alignment:

- ENG-AREA-02
- ENG-INTEGRITY
- ENG-INITIALIZATION
- ENG-DECISION

---

## C. Session Mechanics & Concurrency

### AT-12 — Governance Context Is Bound to the Session

Given:

- a session was created under a specific governance context

When:

- evaluation or acceptance is later attempted

Then:

- the session uses its recorded governance references
- later governance usability changes are handled explicitly by rule
- governance is not silently swapped mid-session

Alignment:

- ENG-CONTEXT-01
- ENG-SESSION
- ENG-DECISION
- ENG-USABILITY

---

### AT-13 — Concurrent Sessions Are Isolated Until Explicit Structural Conflict

Given:

- two concurrent sessions in the same Area
- no shared governance invalidation
- no structural conflict

When:

- both are evaluated independently

Then:

- they do not interfere with one another

Alignment:

- ENG-CONCUR-01
- ENG-STRUCTURE
- ENG-DECISION

---

### AT-14 — Structural Change Triggers Explicit Forward Consequence

Given:

- session S1 depends on Resolution R1
- another accepted outcome supersedes R1

When:

- S1 later attempts forward progress

Then:

- S1 does not proceed as though R1 were still fully forward-usable
- the consequence is explicit and deterministic

Alignment:

- ENG-SUP-01
- ENG-SUP-04
- ENG-STRUCTURE
- ENG-DECISION

---

### AT-15 — Pause and Resume Create a New Deterministic Round

Given:

- a session is paused or temporarily blocked

When:

- resume is executed

Then:

- a new round begins
- participant epochs are reset
- prior round votes do not carry forward
- prior round participants do not carry forward
- prior round candidates do not carry forward
- prior round constraints do not carry forward
- prior round informational reference sets do not carry forward

Alignment:

- ENG-SES-03
- ENG-SESSION
- ENG-DECISION

---

### AT-16 — Resume Does Not Override Forward Usability Rules

Given:

- a session is blocked because a referenced artifact is unusable

When:

- resume is attempted

Then:

- resume does not silently restore legitimacy eligibility
- forward progress still depends on explicit revalidation under current rules

Alignment:

- ENG-SUP-05
- ENG-USABILITY
- ENG-SESSION
- ENG-DECISION

---

## D. Candidates, Constraints & Voting

### AT-17 — Candidates Are Neutral Until Accepted Outcome Exists

Given:

- candidates exist inside a session

When:

- a candidate is rejected, abandoned, removed, or never accepted

Then:

- no legitimacy artifact is created from that candidate alone

Alignment:

- ENG-SES-01
- ENG-LEG-02
- ENG-DECISION

---

### AT-18 — Candidate Set Freezes at the Vote Boundary

Given:

- a session is in mutable pre-vote state

When:

- the session enters voting

Then:

- candidate structure becomes frozen for that round
- add/remove/edit candidate operations fail thereafter for that round

Alignment:

- ENG-SES-02
- ENG-SESSION
- ENG-API

---

### AT-19 — Constraints Are Engine-Enforced

Given:

- explicit constraints exist in a session

When:

- evaluation or acceptance is executed

Then:

- constraints are mechanically enforced
- unsatisfied constraints prevent acceptance
- constraints do not silently alter Authority or Scope

Alignment:

- ENG-CON-01
- ENG-DECISION
- ENG-SESSION

---

### AT-20 — Votes Are Evaluative Only

Given:

- votes have been recorded in a session

When:

- acceptance has not yet occurred

Then:

- no legitimacy exists yet
- no Resolution exists yet

Alignment:

- ENG-VOTE-01
- ENG-ACCEPT-01
- ENG-DECISION

---

### AT-21 — Explicit Dissent Has Mechanical Effect

Given:

- a decision rule where dissent prevents acceptance
- explicit rejecting vote(s) are recorded

When:

- acceptance is attempted

Then:

- acceptance fails deterministically
- no legitimacy artifact is created

Alignment:

- ENG-LEG-05
- ENG-DECISION
- ENG-ERROR

---

### AT-22 — Acceptance Uses Frozen Structural Inputs

Given:

- a session has crossed into voting
- participant and candidate sets are frozen for the round

When:

- acceptance is attempted

Then:

- outcome depends only on the frozen round structures and vote state
- late structural modifications do not participate

Alignment:

- ENG-SESSION
- ENG-DECISION
- ENG-ACCEPT-03

---

### AT-22A — Engine Does Not Infer Votes

Given:

- a valid session
- exactly one participant
- one or more candidates
- no votes recorded

When:

- evaluate_session or attempt_acceptance is executed

Then:

- the Engine does not create, infer, or synthesize votes
- no legitimacy artifact is created
- evaluation fails deterministically under authority evaluation rules

Alignment:

- ENG-DET-02
- ENG-DECISION
- ENG-SESSION
- ENG-API
- ENG-ERROR

---

### AT-22B — No Participants Present Fails Deterministically

Given:

- a session state that contains candidates
- no participants are present

When:

- evaluation or acceptance is attempted

Then:

- the operation fails deterministically
- NO_PARTICIPANTS_PRESENT is reported
- no mutation occurs
- no legitimacy artifact is created

Alignment:

- ENG-SESSION
- ENG-DECISION
- ENG-ERROR
- ENG-API

---

### AT-22C — Vote Absence Fails Through Authority Evaluation

Given:

- a valid session
- one or more participants
- one or more candidates
- no votes recorded

When:

- evaluation or acceptance is attempted

Then:

- the operation fails deterministically
- AUTHORITY_RULE_VIOLATION is reported
- no mutation occurs
- no legitimacy artifact is created

Alignment:

- ENG-DECISION
- ENG-ERROR
- ENG-API

---

### AT-22D — Split Win Fails Deterministically

Given:

- a valid session
- multiple candidates remain eligible
- the explicit final vote state leaves authority without a unique winning candidate

When:

- evaluation or acceptance is attempted

Then:

- the operation fails deterministically
- AUTHORITY_RULE_VIOLATION is reported
- no winner is inferred
- no mutation occurs

Alignment:

- ENG-DECISION
- ENG-ERROR
- ENG-API
- ENG-DET-02

---

### AT-22E — No Eligible Candidates Remain

Given:

- a session with one or more candidates
- every candidate is blocked or invalid under current rules

When:

- evaluation or acceptance is attempted

Then:

- the operation fails deterministically
- NO_ELIGIBLE_CANDIDATES is reported
- no mutation occurs
- no legitimacy artifact is created

Alignment:

- ENG-DECISION
- ENG-ERROR
- ENG-API

---

## E. Acceptance, Receipts & Non-Retroactivity

### AT-23 — Explicit Acceptance Creates Legitimacy

Given:

- a valid session in an acceptable runtime state
- all applicable acceptance conditions are satisfied

When:

- explicit acceptance is attempted

Then:

- a Resolution is created
- the session becomes ACCEPTED
- a LEGITIMACY receipt is emitted
- required structural effects are applied
- all occur atomically

Alignment:

- ENG-ACCEPT-01
- ENG-ACCEPT-02
- ENG-PERSISTENCE
- ENG-RECEIPT
- ENG-DECISION

---

### AT-24 — Acceptance Cannot Be Replayed

Given:

- a session has already been accepted

When:

- acceptance is attempted again

Then:

- no duplicate legitimacy artifact is created
- the operation deterministically fails under command semantics
- historical legitimacy remains singular

Alignment:

- ENG-ID-02
- ENG-SESSION
- ENG-API
- ENG-ERROR

---

### AT-25 — Acceptance Freezes Historical Truth

Given:

- a session has been accepted

When:

- post-acceptance mutation of vote history or accepted context is attempted

Then:

- mutation is rejected
- historical accepted truth remains unchanged

Alignment:

- ENG-ACCEPT-03
- ENG-ACCEPT-04
- ENG-RECEIPT
- ENG-DOMAIN

---

### AT-26 — Later Governance Changes Do Not Rewrite Historical Receipts

Given:

- a LEGITIMACY receipt exists for an accepted session

When:

- referenced artifacts later become ON_HOLD, RETIRED, or SUPERSEDED

Then:

- the receipt remains historically valid
- past legitimacy is not revoked or rewritten

Alignment:

- ENG-HIST-03
- ENG-USABILITY
- ENG-RECEIPT

---

### AT-27 — Closed Sessions Emit EXPLORATION Receipts Only

Given:

- a session closes without accepted legitimacy

When:

- close_session succeeds

Then:

- an EXPLORATION receipt is emitted
- no Resolution is created

Alignment:

- ENG-RECEIPT
- ENG-PERSISTENCE
- ENG-SESSION
- ENG-API

---

## F. ON_HOLD, Retirement & Structural Evolution

### AT-28 — ON_HOLD Suspends Forward Usability Only

Given:

- a Resolution later enters ON_HOLD

When:

- historical receipts or past accepted legitimacy are examined

Then:

- past legitimacy remains intact
- only forward usability is affected

Alignment:

- ENG-HIST-03
- ENG-USABILITY

---

### AT-29 — RETIRED Suspends Forward Usability Only

Given:

- a Resolution later enters RETIRED

When:

- historical receipts or past accepted legitimacy are examined

Then:

- past legitimacy remains intact
- only forward usability is affected

Alignment:

- ENG-HIST-03
- ENG-USABILITY

---

### AT-30 — Structural Supersession Is One-Way

Given:

- Resolution R1 has been superseded

When:

- an operation attempts to restore R1 as structurally active

Then:

- the operation is rejected
- structural ACTIVE does not revert

Alignment:

- ENG-HIST-04
- ENG-SUP-01
- ENG-STRUCTURE

---

### AT-31 — Administrative Usability State Does Not Alter Structural ACTIVE Derivation

Given:

- a structurally active Resolution enters ON_HOLD or RETIRED

When:

- structural graph properties are reconstructed

Then:

- graph-derived structural relationships remain unchanged
- only forward usability semantics differ

Alignment:

- ENG-HIST-03
- ENG-STRUCTURE
- ENG-USABILITY

---

## G. Import, Rehydration & Compilation

### AT-32 — Import Does Not Create Legitimacy

Given:

- historical artifacts are submitted through import or rehydration paths

When:

- no already-valid legitimacy artifacts exist for the claimed historical legitimacy

Then:

- import fails or remains non-activating according to the governing spec
- legitimacy is not manufactured by import

Alignment:

- ENG-IMP-01
- ENG-IMPORT
- ENG-INITIALIZATION

---

### AT-33 — Valid Export Rehydrates Cleanly

Given:

- an Engine export generated from a valid runtime

When:

- that export is rehydrated unchanged

Then:

- runtime entry succeeds
- structural state is preserved
- legitimacy history is preserved deterministically

Alignment:

- ENG-API
- ENG-INITIALIZATION
- ENG-INTEGRITY

---

### AT-34 — Tampering Is Detected

Given:

- an exported structural artifact set is modified

When:

- rehydration or validation is attempted

Then:

- receipt, structure, or provenance validation fails deterministically
- runtime does not silently accept tampered legitimacy

Alignment:

- ENG-INTEGRITY
- ENG-RECEIPT
- ENG-CANON
- ENG-SPECVERIFY

---

### AT-35 — Failed Rehydration Is Side-Effect Free

Given:

- an existing valid runtime or persisted exported state
- a new invalid graph is submitted for rehydration

When:

- initialization fails

Then:

- no partial runtime entry occurs
- no legitimacy artifacts are created
- no hidden repair occurs

Alignment:

- ENG-INITIALIZATION
- ENG-INTEGRITY
- ENG-ERROR

---

### AT-36 — Compilation Reconstructs, It Does Not Re-Decide

Given:

- a historical graph with valid receipts and Resolutions

When:

- compilation or historical reconstruction is executed

Then:

- the graph is reconstructed deterministically
- legitimacy is not re-evaluated from scratch
- receipts are treated as historical source artifacts

Alignment:

- ENG-COMP-01
- ENG-COMPILATION
- ENG-RECEIPT

---

### AT-37 — Compilation Does Not Use Timestamp Authority

Given:

- historical artifacts with timestamps

When:

- compilation reconstructs the graph

Then:

- legitimacy precedence is not derived from timestamps alone
- deterministic reconstruction uses canonical structural rules only

Alignment:

- ENG-COMPILATION
- ENG-CORE-PURITY
- ENG-INTEGRITY

---

## H. Identity, References & Locality

### AT-38 — Stable Object Identity

Given:

- Engine-created structural objects

When:

- the graph is exported, re-imported, or rehydrated

Then:

- structural identifiers remain stable
- identity is preserved across transport

Alignment:

- ENG-ID-01
- ENG-DOMAIN
- ENG-CORE-PURITY

---

### AT-39 — Cross-Area References Are Informational Only

Given:

- structural artifacts contain cross-area informational references

When:

- local legitimacy is evaluated

Then:

- those references do not affect legitimacy outcome
- those references are not treated as structural dependencies

Alignment:

- ENG-AREA-01
- ENG-CORE-03
- ENG-CORE-07
- ENG-DOMAIN

---

### AT-40 — Missing Structural References Fail Deterministically

Given:

- a graph with missing required structural references

When:

- initialization is attempted

Then:

- initialization fails deterministically
- runtime is not entered

Alignment:

- ENG-DOMAIN
- ENG-INTEGRITY
- ENG-INITIALIZATION

---

### AT-40A — Missing Cross-Area Informational References Do Not Invalidate Local Runtime

Given:

- a graph containing cross-area informational references
- those external references are not locally resolvable

When:

- initialization is attempted

Then:

- initialization is not invalidated solely for those missing external informational references
- local legitimacy semantics remain Area-local

Alignment:

- ENG-DOMAIN
- ENG-INTEGRITY
- ENG-INITIALIZATION
- ENG-CORE-PURITY

---

### AT-40B — Unresolved Intra-Area Informational Resolution References Fail Initialization

Given:

- a graph containing intra-Area informational Resolution references
- one or more such references do not resolve to existing local Resolution objects

When:

- initialization is attempted

Then:

- initialization fails deterministically
- runtime is not entered

Alignment:

- ENG-DOMAIN
- ENG-INTEGRITY
- ENG-INITIALIZATION

---

## I. Audit & Observability

### AT-41 — Audit Never Creates Legitimacy

Given:

- audit records exist describing accepted or attempted actions

When:

- corresponding structural legitimacy artifacts are absent

Then:

- legitimacy is not inferred from audit
- audit does not substitute for receipts or Resolutions

Alignment:

- ENG-AUD-01
- ENG-AUD
- ENG-RECEIPT

---

### AT-42 — Audit Is Observational Only

Given:

- audit events are present, absent, reordered in storage, or externally exported

When:

- Engine evaluation and rehydration occur

Then:

- legitimacy outcomes remain unchanged
- structural validity does not depend on audit

Alignment:

- ENG-AUD
- ENG-INITIALIZATION
- ENG-INTEGRITY

---

### AT-43 — Successful Mutations Produce Audit Events Without Altering Legitimacy Semantics

Given:

- a successful state-changing Engine command

When:

- the command completes

Then:

- corresponding audit events are emitted according to the audit specification
- audit emission remains observational only
- command legitimacy does not depend on audit semantics

Alignment:

- ENG-AUD
- ENG-API
- ENG-PERSISTENCE

---

## J. Degraded Mode & Runtime Safety

### AT-44 — Degraded Mode Rejects Mutation

Given:

- runtime has entered DEGRADED_READ_ONLY

When:

- a mutating command is executed

Then:

- the command fails deterministically with DEGRADED_MODE_ACTIVE
- no mutation occurs

Alignment:

- ENG-INTEGRITY
- ENG-INITIALIZATION
- ENG-API
- ENG-ERROR

---

### AT-45 — Degraded Mode Still Permits Safe Read-Only Operations

Given:

- runtime has entered DEGRADED_READ_ONLY

When:

- read-only queries, evaluation, or DAG export are requested

Then:

- those operations remain available if allowed by the governing runtime mode rules
- no new legitimacy is created

Alignment:

- ENG-INTEGRITY
- ENG-API
- ENG-ERROR

---

## K. Atomicity & Crash Safety

### AT-46 — Acceptance Is Atomic

Given:

- acceptance succeeds

When:

- the final accepted legitimacy artifacts are observed

Then:

- session terminal state
- Resolution creation
- structural mutation
- LEGITIMACY receipt creation

must either all exist together or not exist at all

Alignment:

- ENG-ACCEPT-02
- ENG-PERSISTENCE
- ENG-DECISION

---

### AT-47 — Closure Is Atomically Recorded

Given:

- close_session succeeds without acceptance

When:

- terminal closure artifacts are observed

Then:

- CLOSED state and EXPLORATION receipt exist together
- no Resolution exists for that closure

Alignment:

- ENG-PERSISTENCE
- ENG-RECEIPT
- ENG-SESSION

---

### AT-48 — Crash Before Commit Creates No Legitimacy

Given:

- acceptance is interrupted before atomic commit completes

When:

- state is later inspected

Then:

- no accepted legitimacy artifact set exists from that attempt

Alignment:

- ENG-PERSISTENCE
- ENG-INTEGRITY

---

## L. Reporting & Determinism

### AT-49 — EvaluationReport Is Deterministic

Given:

- identical runtime state
- identical command input

When:

- the same command is executed repeatedly

Then:

- EvaluationReport outcome is identical
- errors appear in identical order
- primary_error_code is identical

Alignment:

- ENG-ERROR
- ENG-API
- ENG-CORE-PURITY

---

### AT-50 — Failed Commands Do Not Mutate State

Given:

- a command violates one or more applicable rules

When:

- the command returns REJECTED or BLOCKED

Then:

- no unauthorized state mutation occurs
- no unauthorized receipt or Resolution is created

Alignment:

- ENG-ERROR
- ENG-API
- ENG-SESSION
- ENG-DECISION

---

## Final Note

These tests verify the Engine as a deterministic legitimacy compiler.

They are library-facing tests only.

They do not define workflow, operator procedure, or CLI ergonomics.

If these tests pass, the Engine satisfies the externally visible behavioral guarantees expected from the current constitutional and detailed specifications.