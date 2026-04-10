# Charter Runtime — Incoming Reconciliation Review Process (Draft v2)

Status: FOUNDATIONAL (DRAFT)  
Applies to: Runtime Layer, Legitimacy Engine (invocation only), CDS, CCS, Commit Store  
Does NOT define: legitimacy semantics, engine decision rules, structural graph computation, or identity semantics  

---

# 1. Purpose

The Incoming Reconciliation Review process defines how external or investigative artifacts are:

- evaluated  
- aligned  
- structured  
- approved  

before entering legitimacy.

It exists to ensure:

- no artifact enters legitimacy without explicit evaluation  
- imported or proposed structures are reconciled against existing structure  
- authority rules are respected prior to session execution  
- lineage and provenance are preserved  
- structural ambiguity is resolved before legitimacy is attempted  

---

# 2. Core Principle

> Review prepares structure for legitimacy.  
> Only sessions create legitimacy.

The review process:

- evaluates proposals  
- applies governance constraints  
- determines review approval eligibility  
- prepares accepted proposals for atomic session batching  

It does not:

- create legitimacy  
- mutate existing resolutions  
- bypass the Legitimacy Engine  

---

# 3. Scope Boundary

This document defines the **incoming** reconciliation path only:

- imported graph → review → legitimacy  
- imported flat file → review → legitimacy  
- CDS LOCKED output → review → legitimacy  
- foreign artifact intake → review → legitimacy  

It does not define:

- reverse reconciliation into CDS  
- direct restore semantics  
- CRR workflows  
- federation transport behavior  

---

# 4. Relationship to Restore

Restore and incoming reconciliation review are distinct.

- **Restore** attempts direct runtime/legitimacy re-entry from imported structure  
- **Incoming Reconciliation Review** is the governed consolidation path when direct restore is not valid, not possible, or not desired  

If restore fails runtime or legitimacy validation, the user must reconcile through review.

---

# 5. Inputs

Incoming reconciliation may operate on:

- imported DAGs (CCE format)  
- flat resolution files  
- CDS LOCKED Items  
- foreign artifacts (federation/discovery)  

All inputs are transformed into:

→ **Proposals (non-legitimate resolution candidates)**

---

# 6. Review Workspace

## 6.1 Isolation

All review activity occurs in an isolated **Review Workspace**.

Properties:

- read-only with respect to source artifacts  
- non-legitimizing  
- fully auditable  
- discardable until committed  

---

## 6.2 Local Context

The workspace has access to:

- local active resolutions for the Area  
- imported proposals  
- structural comparison results  
- provenance metadata  
- Area governance context  
- Runtime rule identity  

---

## 6.3 Runtime Context

Runtime may support multiple active reconciliation reviews, but interfering reviews must be concurrency-blocked.

Interference includes any case where one review may affect another review’s:

- proposal targets  
- relationship assumptions  
- supersession assumptions  
- accepted batch viability  

---

# 7. Proposal Model

## 7.1 Definition

A Proposal represents a candidate resolution-like artifact prepared for review.

Properties:

- non-legitimate  
- mutable only during PRE_STANCE  
- derived from imported or CDS artifacts  
- may reference existing resolutions  
- structurally parallel to Engine Candidate shape  

---

## 7.2 Proposal Structure

A proposal should mirror candidate-style structure and may include:

- proposal_id  
- proposal_action_type  
- proposal_payload  
- proposal_provenance  
- proposal_relationships  
- proposal_round_index  
- source identity fields  

This structure exists to support:

- eventual session translation  
- federation intake  
- CDS-to-legitimacy flow  
- deterministic batching  

---

## 7.3 Proposal States

Workflow states:

- UNDER_REVIEW  
- ACCEPTED  
- REJECTED  
- ABANDONED  

Terminal states:

- ACCEPTED → eligible for session batching  
- REJECTED → explicitly denied  
- ABANDONED → not acted upon or canceled  

---

## 7.4 Structural Flags (Non-Workflow)

Proposals may carry structural classification flags:

- MATCHING (equivalent to existing resolution)  
- SIMILAR (semantically close, explicitly non-superseding)  
- DIVERGENT (conflicts or differs materially)  
- HISTORICAL (superseded within imported set)  
- INVALID_TARGET (attempts to supersede already superseded resolution)  

Flags:

- do not create legitimacy  
- do not replace review decisions  
- may influence guidance  
- may contribute to blocking when structurally invalid  

### Structural Flag Principle

`SIMILAR` exists to prevent accidental supersession when a diverged graph contains artifacts that are mechanically close but semantically different.

---

# 8. Review State Model

States:

- PRE_STANCE  
- VOTING  
- PAUSED  
- BLOCKED  
- COMPLETED  
- CANCELED  

---

## 8.1 PRE_STANCE

Mutable phase.

Allowed:

- add/remove proposals  
- define or revise relationships  
- assign participants  
- define constraints  
- modify proposal states  
- resolve structural ambiguity  
- request guidance assistance  

Transition:

→ first vote → VOTING (freeze)

---

## 8.2 VOTING

Frozen phase.

Rules:

- proposals immutable  
- relationships immutable  
- participants immutable  
- constraints immutable  
- votes mutable (vacillation allowed)  

Voting in this phase applies to the **review as a whole**.

What is being voted on is the overarching proposed change set, not individual proposals.

Outcome:

- voting continues until explicit approval attempt or pause  

---

## 8.3 PAUSED

Pause behavior:

- current voting round ends  
- voting state is cleared  
- review returns to PRE_STANCE on resume  
- round increments  

Paused review does not create legitimacy and does not preserve an active vote state.

---

## 8.4 BLOCKED

Blocked review cannot proceed to approval in its current round.

A blocked review must be:

- explicitly resumed  
- returned to PRE_STANCE  
- advanced to a new round  

Blocked reviews remain visible and auditable.

---

## 8.5 COMPLETED

Terminal state after:

- successful review approval  
- successful atomic session batching  
- receipt emission  

Completed reviews are immutable.

---

## 8.6 CANCELED

Terminal state:

- all unresolved proposals become ABANDONED  
- no sessions execute  
- review closes immutably  

---

# 9. Round Model

- review begins at round_index = 1  
- rounds increment on resume  
- each round is isolated  
- review history preserves all prior rounds  

Reset on new round:

- votes  
- participants  
- constraints  
- round voting state  

Relationships and proposal structure return to PRE_STANCE mutability for editing in the new round.

History:

- preserved for audit and review receipt purposes  
- non-legitimizing  
- must not affect future review approval semantics directly  

---

# 10. Governance Context

## 10.1 Review Governance Alignment

The review must use a decision rule aligned with the Area Authority and must operate within the Area Scope.

Incoming reconciliation review is governed, but not legitimizing.

---

## 10.2 Scope and Authority Blocking

The review becomes blocked if:

- Area Scope is ON_HOLD  
- required Authority is invalid or unusable  
- governance context changes materially during review  
- review constraints cannot be satisfied  

If Authority or Scope changes materially:

- the active voting round is invalidated  
- the review must resume into a new PRE_STANCE round  

---

# 11. Voting & Decision Model

## 11.1 Decision Rule

The review must define a decision rule aligned with Area Authority.

Examples:

- unanimous  
- majority  
- designated approver  

This rule governs review approval, not legitimacy itself.

---

## 11.2 Voting Scope

Voting occurs at:

→ review-level approval (not per proposal)

Meaning:

- proposals are prepared individually  
- the review is voted on as one governed package  
- approval authorizes batching into real engine sessions  

---

## 11.3 Voting Mechanics

Review voting should follow session-style mechanics:

- first stance freezes the round  
- votes are explicit  
- votes may vacillate during VOTING  
- only final vote state is authoritative at approval attempt  
- no votes are inferred  

---

## 11.4 Constraints

Constraints may include:

- required participant approval  
- structural completeness requirements  
- relationship validation completeness  
- designated approver conditions  

Failure:

- prevents review approval  
- may block the review until resumed  

---

# 12. Proposal Operations

Users may:

- accept proposal  
- reject proposal  
- leave proposal unresolved  
- define relationships:
  - supersedes  
  - similar_to  
  - references  
  - other explicitly supported structural relationships  

Unresolved proposals become ABANDONED on completion or cancellation.

---

# 13. Structural Reconciliation

Before and during review:

- imported active proposals are cross-referenced against local active resolutions  
- proposals are structurally compared for:
  - equivalence  
  - supersession  
  - conflict  
  - divergence  
  - invalid targets  

Results populate structural flags.

No automatic mutation occurs.

No implicit relationships are created.

---

# 14. Blocking Model

## 14.1 Proposal-Level Blocking

A proposal may be:

- temporarily blocked (missing relationship clarity, unresolved structural dependency)  
- permanently blocked (invalid target, impossible supersession, structurally invalid action)  

Blocked proposals:

- cannot be accepted  
- remain visible in review history  

---

## 14.2 Review-Level Blocking

The entire review may be blocked if:

- authority requirements are not met  
- scope is ON_HOLD  
- governance context changes materially  
- constraints are not satisfied  
- no structurally valid accepted proposal set remains  
- another active review creates an interfering concurrency condition  

Blocked review:

- cannot be approved  
- must resume into PRE_STANCE through a new round  

---

## 14.3 Structural Blocking Principle

Outside legitimacy, relationships are first-class.

Therefore:

- structural issues are hard blockers in review  
- unresolved structural ambiguity is not advisory  
- relationship correctness must be explicit before approval  

---

# 15. Acceptance & Session Batching

## 15.1 Review Approval Boundary

Successful review approval does not create legitimacy.

It authorizes Runtime to prepare and execute one atomic accepted batch through real engine sessions.

---

## 15.2 Batch Construction

Upon successful review approval:

- all ACCEPTED proposals are grouped into one reconciliation batch  
- batching must be atomic  
- no partial success is allowed  

---

## 15.3 Ordering Rules

Within the batch, ordering must be deterministic.

Ordering should prefer:

- proposals with dependency relationships first  
- relationship-establishing changes first  
- proposals that other proposals depend on first  

Because:

- relationship changes are session-significant  
- later relationship changes require new sessions  
- order affects validity  

---

## 15.4 Atomicity

Session batching must succeed completely or have no legitimacy effect.

There is:

- no partial legitimacy creation  
- no partial accepted batch commit  
- no partial review success  

If session batching fails:

- the review becomes BLOCKED  
- it must be resumed into PRE_STANCE through a new round  

---

# 16. Relationship to CRR

CRR does not occur inside incoming reconciliation review.

CRR should occur:

- before review in CDS or another workflow  
- or after review as a separate workflow  

If CRR produces changes that must enter legitimacy, those changes should return through reconciliation review again.

---

# 17. Receipt Model

## 17.1 Review Receipt

One review receipt per completed or canceled review.

Includes:

- review_id  
- area_id  
- runtime_rule_identity  
- participants  
- proposals and final states  
- structural flags  
- round history  
- decision rule  
- outcome  
- resulting session identifiers where applicable  

---

## 17.2 Proposal Outcome Records

Each proposal produces a review outcome record:

- ACCEPTED → included in session batching  
- REJECTED → explicit record  
- ABANDONED → explicit or derivable closure record  

These are review artifacts, not legitimacy receipts.

---

## 17.3 Session Receipts

Engine session receipts produced during accepted batching retain engine rule identity and remain the only legitimacy receipts.

---

## 17.4 Properties

Review receipts are:

- immutable  
- auditable  
- non-legitimizing  
- provenance-bearing  

---

# 18. Provenance

Review must preserve:

- source of proposal (import, CDS, federation)  
- original identifiers  
- source rule identity if available  
- lineage references  
- runtime rule identity on review artifacts  

No provenance is lost or rewritten.

Unknown source rule identities must not be silently reinterpreted.

---

# 19. Guidance Assistance

CGL may assist during PRE_STANCE and moments of discovery.

This may include:

- conflict detection  
- semantic distinction surfacing  
- ambiguity highlighting  
- structural suggestion prompts  

CGL may proactively surface conflicts and suggestions.

All deeper assistance remains explicitly user-invoked.

Guidance remains:

- non-authoritative  
- non-legitimizing  
- non-mutating  

---

# 20. Invariants

- no proposal becomes legitimate without session execution  
- review does not create legitimacy  
- PRE_STANCE is the only mutable phase  
- freeze occurs on first vote  
- voting is explicit and mutable until approval attempt  
- rounds are deterministic and isolated  
- review approval is required before session batching  
- proposals must be explicitly accepted to be processed  
- rejected and abandoned proposals are preserved  
- review artifacts are immutable after completion or cancellation  
- provenance must be preserved  
- structural flags do not create legitimacy  
- structural blockers must be explicit  
- no implicit relationships are created  
- blocked reviews must resume through a new PRE_STANCE round  
- accepted batch execution is atomic  
- runtime review receipts and engine session receipts remain distinct  
- concurrency interference must block affected reviews  

---

# 21. Mental Model

Incoming reconciliation review is:

- a staging ground for legitimacy  
- a governance-aligned approval process  
- a structured negotiation of imported or proposed truth  
- a batch authorization step for real sessions  

It is not:

- a legitimacy system  
- a graph mutation engine  
- a substitute for engine sessions  
- a hidden CRR workflow  

---

# 22. Final Principle

Nothing enters legitimacy silently.

Everything must be:

- examined  
- aligned  
- structurally clarified  
- explicitly approved  

before it is allowed to become part of the system.