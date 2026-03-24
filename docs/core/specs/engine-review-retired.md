# ENG-REVIEW-RETIRED — Usability Suspension & Deprecation Semantics
 
Status: REFACTORED (v5 – Terminal Retirement Alignment)  
Applies to: Engine Core (V1/V2+)  
 
Authority: Foundational authority for UNDER_REVIEW and RETIRED usability semantics.
 
Subordinate references consumed from:
 
- ENG-DOMAIN
- ENG-SUPERSESSION
- ENG-SESSION
- ENG-DECISION
- ENG-INTEGRITY
- ENG-PERSISTENCE
- ENG-RECEIPT
- ENG-ERROR
 
If conflict exists, ENG-INTEGRITY governs runtime halt behavior.  
If conflict exists regarding structural ACTIVE, ENG-SUPERSESSION governs graph meaning.
  
# 1. Purpose
 
ENG-REVIEW-RETIRED defines the semantics of UNDER_REVIEW and RETIRED for governance usability.
 
It is the authoritative specification for:
 
- allowed administrative and governance state transitions involving UNDER_REVIEW and RETIRED
- forward usability semantics
- temporary suspension semantics
- deprecation semantics
- session-facing consequences of unusable referenced artifacts
- legitimacy neutrality of administrative suspension and deprecation
 
ENG-REVIEW-RETIRED does not define:
 
- structural ACTIVE derivation
- supersession graph semantics
- object schemas
- runtime halt conditions
- receipt structure
- session lifecycle structure
- atomic persistence boundaries
 
Those are defined respectively in:
 
- ENG-SUPERSESSION
- ENG-DOMAIN
- ENG-INTEGRITY
- ENG-RECEIPT
- ENG-SESSION
- ENG-PERSISTENCE
 
UNDER_REVIEW and RETIRED do not create legitimacy, revoke historical legitimacy, or alter supersession graph structure.
 
SUPERSEDED remains the only graph-altering lifecycle outcome.
  
# 2. Conceptual Boundary
 
## ENG-REVIEW-RETIRED-01 — Structural Graph Truth vs Usability
 
ENG-REVIEW-RETIRED governs usability, not graph structure.
 
Structural ACTIVE is defined only in ENG-SUPERSESSION.
 
UNDER_REVIEW and RETIRED:
 
- do not create supersession edges
- do not remove supersession edges
- do not alter structural ACTIVE derivation
- do not alter historical receipts
- do not reinterpret past legitimacy
 
They affect only whether a structurally valid artifact may be used for forward legitimacy evaluation.
  
# 3. Governance Control Model
 
## ENG-REVIEW-RETIRED-02 — Session Governance Rule
 
All lifecycle changes that affect forward governance usability must occur through valid governance session acceptance, except where explicitly defined as administrative transitions.
 
The only administrative transition governed here is:
 
- ACTIVE ↔ UNDER_REVIEW
 
All other lifecycle changes remain governed by ordinary governance and graph rules elsewhere.
 
ENG-REVIEW-RETIRED does not define supersession graph mutation.  
It defines only whether an artifact is administratively suspended or deprecated for forward use.
  
## ENG-REVIEW-RETIRED-03 — Authority Restrictions
 
Authority is never allowed to enter:
 
- UNDER_REVIEW
- RETIRED
 
Authority usability and structural participation are governed elsewhere:
 
- ENG-SUPERSESSION
- ENG-INTEGRITY
- ENG-DECISION
 
ENG-REVIEW-RETIRED only establishes that Authority is not a reviewable or retired governance artifact type.
  
## ENG-REVIEW-RETIRED-04 — Scope Restrictions
 
Scope may enter:
 
- UNDER_REVIEW
 
Scope may not enter:
 
- RETIRED
 
Scope supersession remains structural graph behavior governed by ENG-SUPERSESSION.
 
ENG-REVIEW-RETIRED defines only that UNDER_REVIEW suspends forward usability of Scope for legitimacy evaluation.
  
# 4. Reviewable Object Classes
 
## ENG-REVIEW-RETIRED-05 — Supported Usability States by Object Class
 
Resolution may support:
 
- ACTIVE
- UNDER_REVIEW
- RETIRED
- SUPERSEDED
 
Scope Resolution may support:
 
- ACTIVE
- UNDER_REVIEW
- SUPERSEDED
 
Authority Resolution may support:
 
- ACTIVE
- SUPERSEDED
 
Object schemas and enumerations are defined in ENG-DOMAIN.
 
ENG-REVIEW-RETIRED defines which of those states carry usability suspension meaning.
  
# 5. Usability Rules
 
## ENG-REVIEW-RETIRED-06 — Resolution Usability
 
A Resolution is usable for forward legitimacy evaluation only if:
 
- it is structurally ACTIVE according to ENG-SUPERSESSION
- it is not UNDER_REVIEW
- it is not RETIRED
 
ENG-REVIEW-RETIRED does not define structural ACTIVE. It consumes it.
  
## ENG-REVIEW-RETIRED-07 — Scope Usability
 
A Scope Resolution is usable for forward legitimacy evaluation only if:
 
- it is structurally ACTIVE according to ENG-SUPERSESSION
- it is not UNDER_REVIEW
 
Scope does not support RETIRED.
  
## ENG-REVIEW-RETIRED-08 — Authority Usability
 
Authority must be usable according to the governance and graph rules defined elsewhere.
 
ENG-REVIEW-RETIRED defines only that Authority cannot be placed UNDER_REVIEW or RETIRED.
 
Authority evaluation mechanics belong to:
 
- ENG-DECISION
- ENG-SUPERSESSION
- ENG-INTEGRITY
  
# 6. Allowed State Transitions
 
## ENG-REVIEW-RETIRED-09 — Resolution Transition Semantics
 
For Resolution usability semantics:
 
Allowed here:
 
- ACTIVE ↔ UNDER_REVIEW
- ACTIVE → RETIRED
 
Not defined here:
 
- graph mutation to SUPERSEDED
- structural edge creation
- supersession acceptance mechanics
 
SUPERSEDED remains structural and terminal under ENG-SUPERSESSION.
 
ENG-REVIEW-RETIRED does not authorize RETIRED → SUPERSEDED as a usability transition.  
Graph-level lifecycle semantics belong to ENG-SUPERSESSION and governance acceptance elsewhere.
  
## ENG-REVIEW-RETIRED-10 — Scope Transition Semantics
 
For Scope usability semantics:
 
Allowed here:
 
- ACTIVE ↔ UNDER_REVIEW
 
SUPERSEDED remains a structural graph outcome defined elsewhere.
 
Scope does not support RETIRED.
  
# 7. Legitimacy Neutrality
 
## ENG-REVIEW-RETIRED-11 — UNDER_REVIEW Neutrality
 
UNDER_REVIEW:
 
- creates no new legitimacy artifact
- modifies no supersession edge
- alters no structural ACTIVE derivation
- affects no historical receipt validity
- suspends forward usability only
  
## ENG-REVIEW-RETIRED-12 — RETIRED Neutrality
 
RETIRED:
 
- creates no new legitimacy artifact
- modifies no supersession edge
- alters no structural ACTIVE derivation
- affects no historical receipt validity
- suspends forward usability only
- marks the artifact permanently deprecated for future use
  
# 8. Session-Facing Consequences
 
## ENG-REVIEW-RETIRED-13 — Decision Layer Consumption
 
ENG-REVIEW-RETIRED does not define session lifecycle transitions.
 
It defines the usability truths consumed by:
 
- ENG-DECISION
- ENG-SESSION
 
If a session references an artifact that is unusable under this specification, acceptance must not proceed.
 
How the session transitions in response is defined in:
 
- ENG-DECISION
- ENG-SESSION
  
## ENG-REVIEW-RETIRED-14 — Temporary vs Permanent Consequence Boundary
 
ENG-REVIEW-RETIRED establishes only the usability cause, not the lifecycle transition implementation.
 
General intent:
 
- UNDER_REVIEW causes reversible forward unavailability
- RETIRED causes permanent forward deprecation and unavailability
- SUPERSEDED remains a structural graph outcome with irreversible graph consequences
 
Exact session blocking, vote clearing, resume, and closure behavior belong to:
 
- ENG-SESSION
- ENG-DECISION
  
# 9. Acceptance Guard Semantics
 
## ENG-REVIEW-RETIRED-15 — Unusable Referenced Objects Prevent Acceptance
 
Acceptance must fail deterministically if a referenced object is unusable under this specification.
 
This includes:
 
- Resolution under UNDER_REVIEW
- Resolution under RETIRED
- Scope under UNDER_REVIEW
 
ENG-REVIEW-RETIRED defines the usability truth only.
 
Acceptance orchestration belongs to ENG-DECISION.  
Atomic commit behavior belongs to ENG-PERSISTENCE.  
Session state behavior belongs to ENG-SESSION.
  
# 10. Concurrency Semantics
 
## ENG-REVIEW-RETIRED-16 — Administrative Usability Changes Must Be Seen Atomically
 
Transitions affecting usability must not be partially observable.
 
If an administrative or governance state change occurs concurrently with acceptance evaluation:
 
- acceptance must consume a deterministic usability result
- no partial usability state may be committed
- no partial session mutation may result from ambiguous usability state
 
Atomic persistence and mutation safety are defined in:
 
- ENG-PERSISTENCE
- ENG-INTEGRITY
 
ENG-REVIEW-RETIRED defines only that usability changes must behave as coherent truths.
  
# 11. Resume & Reconfirmation Boundary
 
## ENG-REVIEW-RETIRED-17 — Resume Never Overrides Usability
 
A resumed session must not regain acceptance eligibility merely by resuming if referenced artifacts remain unusable.
 
Resume semantics, round reset, participant re-specification, and vote clearing belong to ENG-SESSION.
 
ENG-REVIEW-RETIRED establishes that:
 
- resumption does not override UNDER_REVIEW
- resumption does not override RETIRED
- resumption does not bypass forward usability constraints
  
# 12. Receipt Interaction
 
## ENG-REVIEW-RETIRED-18 — Historical Receipts Remain Valid
 
Receipts freeze governance context at acceptance time.
 
Subsequent transitions of referenced artifacts into:
 
- UNDER_REVIEW
- RETIRED
- SUPERSEDED
 
do not invalidate historical receipts and do not retroactively revoke legitimacy.
 
Receipt schema and canonical integrity belong to:
 
- ENG-RECEIPT
- ENG-CANON
- ENG-SPECVERIFY
 
ENG-REVIEW-RETIRED defines only that forward usability changes do not reach backward into legitimacy history.
  
# 13. Runtime Relationship to Integrity
 
## ENG-REVIEW-RETIRED-19 — Runtime Halt Does Not Originate Here
 
ENG-REVIEW-RETIRED is not the authority for runtime halt or degraded mode decisions.
 
If usability semantics interact with structural inconsistencies, runtime safety outcomes are determined by ENG-INTEGRITY.
 
ENG-REVIEW-RETIRED defines:
 
- what is usable
- what is suspended
- what is deprecated
 
ENG-INTEGRITY defines whether the runtime may proceed safely with those truths.
  
# 14. Relationship to Supersession
 
## ENG-REVIEW-RETIRED-20 — Supersession Is External Graph Authority
 
ENG-REVIEW-RETIRED does not define:
 
- when a supersession edge is valid
- how a graph becomes SUPERSEDED
- how structural ACTIVE is recomputed
- whether a cycle exists
- whether a graph conflict exists
 
Those belong to ENG-SUPERSESSION.
 
ENG-REVIEW-RETIRED only defines usability consequences that are intentionally independent of graph structure.
  
# 15. Engine Invariants
 
- UNDER_REVIEW never alters supersession edges
- RETIRED never alters supersession edges
- UNDER_REVIEW never alters structural ACTIVE derivation
- RETIRED never alters structural ACTIVE derivation
- UNDER_REVIEW never creates legitimacy
- RETIRED never creates legitimacy
- UNDER_REVIEW never destroys historical legitimacy
- RETIRED never destroys historical legitimacy
- RETIRED is terminal and irreversible
- forward usability must reflect administrative suspension and deprecation truths defined here
- dependent specifications must consume these usability truths rather than redefine them
  
# 16. Mental Model
 
ENG-REVIEW-RETIRED defines forward usability truth.
 
It answers:
 
- whether a structurally valid artifact is temporarily suspended
- whether a structurally valid artifact is permanently deprecated for future legitimacy use
- whether a session may rely on a referenced artifact for forward legitimacy evaluation
 
It does not answer:
 
- whether the graph is structurally ACTIVE
- whether the Engine must halt
- how receipts are serialized
- how session rounds reset
- how acceptance is persisted atomically
 
Those belong elsewhere.
 
ENG-REVIEW-RETIRED is the usability layer.  
Other specifications must consume it rather than duplicate it.