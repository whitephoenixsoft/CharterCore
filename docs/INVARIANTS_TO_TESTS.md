# Charter Core — Acceptance Tests (Invariant-Driven, Frozen)

These acceptance tests define the minimum behavioral guarantees of Charter Core.

If any test fails, the engine has violated a core invariant.
Acceptance tests assert what must or must not happen, not how it is implemented.

---

## Section A — Core Legitimacy Invariants

### AT-1 Explicit Decisions Only

#### Given

- An initialized Area with an active Authority and Scope
- A session with candidates and recorded stances

#### When

- No candidate satisfies the Authority rule

#### Then

- No resolution is created
- No implicit “winner” is inferred
- The session remains ACTIVE or enters BLOCKED

#### Fail if

- A resolution exists without explicit acceptance

Note: BLOCKED implies a deterministic unmet condition; ACTIVE implies evaluation is incomplete.

---

### AT-2 Sessions Are the Sole Unit of Legitimacy

#### Given

- A candidate exists outside of any session

#### When

- An attempt is made to accept it

#### Then

- Acceptance is rejected
- No resolution is created

---

### AT-3 Immutable Resolution History

#### Given

- A resolution R-1 has been accepted

#### When

- Any attempt is made to modify, overwrite, or delete R-1

#### Then

- The operation is rejected
- R-1 remains unchanged and queryable

#### And

- Only a new resolution may supersede or retire R-1

---

### AT-4 Deterministic Evaluation

#### Given

- Identical session state:
- same participants
- same stances
- same Authority rule

#### When

- Evaluation is run multiple times

#### Then

- The outcome is identical every time

#### Fail if

- Non-deterministic results occur

---

### AT-5 No Semantic Interpretation

#### Given

- Candidates with arbitrary content
- Optional rationale text

#### When

- The engine evaluates or accepts candidates

#### Then

- Content and rationale are never interpreted
- Only mechanical Authority and Scope rules are applied

#### Fail if

- Meaning, wording, or intent affects outcome

---

## Section B — Areas, Authority, and Scope

### AT-6 Areas Define Hard Governance Boundaries

#### Given

- Two Areas A and B
- Each has independent Authority and Scope

#### When

- A session is opened in Area A

#### Then

- Only Area A’s Authority governs the session
- Area B has no effect unless explicitly referenced

#### Fail if

- Authority or Scope from another Area is implicitly applied

---

### AT-7 Authority Is a First-Class Resolution

#### Given

- An Area with an active Authority resolution A-AUTH-1

#### When

- A new Authority candidate is accepted

#### Then

- A-AUTH-2 is created
- A-AUTH-1 is marked SUPERSEDED
- Exactly one active Authority exists

#### Fail if

-Multiple active Authorities exist

---

### AT-8 Scope Is a First-Class Resolution

#### Given

- An Area with an active Scope resolution S-1

#### When

- A new Scope candidate is accepted

#### Then

- S-2 is created
- S-1 is marked SUPERSEDED
- Exactly one active Scope exists

---

### AT-9 Context Preservation (Authority & Scope)

#### Given

- A session S accepts a resolution R under Authority A-1 and Scope S-1

#### When

- Authority A-2 and Scope S-2 later become active

#### Then

- R permanently references A-1 and S-1
- R is not re-evaluated, altered, or flagged

#### Fail if

- Historical resolutions are altered by later context

---

## AT-10 Area Initialization Is Required

#### Given

- An Area with no active Authority or Scope

#### When

- A non-Authority, non-Scope resolution is accepted

#### Then

- Acceptance is blocked
- Initialization is required

---

## Section C — Session Mechanics & Blocking

### AT-11 Authority Rule Is Fixed per Session

#### Given

- A session is started

#### Then

- Exactly one Authority resolution governs the session
- The rule is fixed for the session’s lifetime

#### Fail if

- Authority changes mid-session without revalidation

---

### AT-12 Standing Is Action-Based

#### Given

- Authority rule = UNANIMOUS_PRESENT
- A session with Alice, Bob, Charlie listed

#### When

- Alice and Bob cast stances (ACCEPT, REJECT, ABSTAIN)
- Charlie takes no action

#### Then

- Present set = {Alice, Bob}
- Charlie is not counted

Notes:
- A stance is any explicit engine-recorded expression of acceptance or rejection.
- Presence is derived from recorded actions within the session.

---

### AT-13 Explicit Disagreement Blocks Unanimity

#### Given

- Authority rule = UNANIMOUS_PRESENT
- Alice, Bob, Charlie all present

#### When

- Alice: ACCEPT
- Bob: ACCEPT
- Charlie: REJECT

#### Then

- Resolution cannot be accepted
- Session remains blocked
- Objection is recorded in audit trail

Note: A stance is any explicit engine-recorded expression of acceptance or rejection.

---

### AT-14 Session Blocking and Revalidation

#### Given

- A session is PAUSED or BLOCKED

#### When

- Authority, Scope, or Preceding Resolution changes

#### Then

- Session cannot resume without revalidation

#### Fail if

- Acceptance proceeds under changed context

---

### AT-15 Concurrent Sessions Are Isolated

#### Given

- Two sessions active in the same Area

#### When

- No Authority, Scope, or superseding resolution is accepted

#### Then

- Sessions do not interfere

---

### AT-16 Supersession Triggers Revalidation

#### Given

- Session S references Resolution R
- Another session supersedes R

#### Then

- S requires revalidation
- S may not accept candidates until handled

---

## Section D — Resolution Lifecycle

### AT-17 Explicit Resolution Lifecycle

#### Given

- A resolution R is ACTIVE

#### When

- It is superseded or retired via a session

#### Then

- Its lifecycle state changes explicitly
- R remains queryable forever

#### Fail if

- R disappears or is silently altered

---

### AT-18 Resolution State Changes Require Sessions

#### Given

- A resolution R is ACTIVE

#### When

- An API attempts to mark it SUPERSEDED or RETIRED without a session

#### Then

- Operation is rejected
- Engine reports: “Resolution legitimacy may only change via a decision session.”

---

### Section E — Import / Export & Integrity

### AT-19 Valid Export Imports Successfully

#### Given

- A Charter Core instance with Areas, Sessions, and Resolutions
- An export generated by the engine

#### When

- The export is imported without modification

#### Then

- Import succeeds
- All references are preserved
- No resolutions are marked UNDER_REVIEW

---

### AT-20 Tampered Export Is Detected

#### Given

- A valid export
- Manual modification of content or metadata

#### When

- The export is imported

#### Then

- Integrity verification fails
- Import is rejected or
- All affected resolutions are marked UNDER_REVIEW

---

### AT-21 Structural Integrity Is Enforced

#### Given

An export where:
- A resolution references a missing session, or
- A session references a missing Authority

#### When

- The export is imported

#### Then

- Import fails deterministically
- No partial state is created

---

### AT-22 Failed Import Does Not Mutate History

#### Given

- An existing Charter Core instance
- A failed import attempt

#### Then

- Existing history remains unchanged

---

### AT-23 Fresh Import from Flat Resolution List

#### Given

- An organization with no existing Charter history
- A flat list of resolutions with no sessions or candidates

#### When

- The list is imported in CONSOLIDATE mode

#### Then

- Each imported resolution is created
- Each resolution is marked UNDER_REVIEW
- No Authority or Scope is inferred
- No resolution becomes ACTIVE without a session

#### Fail if

- Any resolution is treated as accepted
- Authority or Scope is assumed implicitly

---
## Section F - Import Reviews

### AT-IR-01 — Chronological Review Order

#### Given

An imported history containing resolutions in this order:
1. Authority R-AUTH-2
2. Resolution R-A
3. Resolution R-B
4. All marked UNDER_REVIEW

#### When

- The user initiates review

#### Then

- The engine presents R-AUTH-2, then R-A, then R-B in order

#### Fail if

- Any resolution is reviewed out of order
- Any resolution type is prioritized

---

### AT-IR-02 — Imported Authority Does Not Govern Review

#### Given

- Local Area has Authority R-AUTH-LOCAL (SOLO)
- Imported Authority R-AUTH-IMPORTED defines MAJORITY
- R-AUTH-IMPORTED is UNDER_REVIEW

#### When

- The user attempts to accept R-AUTH-IMPORTED during review

#### Then

- Acceptance is evaluated using R-AUTH-LOCAL
- Imported Authority mechanics are not applied

#### Fail if

- Imported Authority affects vote evaluation

--- 

### AT-IR-03 — Authority Rejection Does Not Block Later Review

#### Given

Imported resolutions in order:
1. Authority R-AUTH-2
2. Resolution R-A
3. Both UNDER_REVIEW

#### When

- R-AUTH-2 is explicitly rejected

#### Then

- R-A remains reviewable
- No automatic rejection occurs

#### Fail if

- R-A is auto-rejected or blocked

--- 

### AT-IR-04 — Imported Context Is Preserved

#### Given

- Resolution R-A imported with acceptance context:
- Authority: R-AUTH-2
- Scope: R-SCOPE-2

#### When

- R-A is reviewed

#### Then

- The engine exposes:
- Imported Authority reference
- Imported Scope reference

#### And

- No reinterpretation occurs

#### Fail if

- Context is overwritten or recalculated

--- 

#### AT-IR-05 — No Retroactive Authority Application

#### Given

- Imported Authority R-AUTH-2 is accepted during review
- Imported Resolution R-A appears earlier in the timeline

#### When

- Review proceeds

#### Then

- R-AUTH-2 does not retroactively govern R-A

#### Fail if

- Earlier resolutions are re-evaluated

--- 

### AT-IR-06 — Review Acceptance Is Explicit

#### Given

- Multiple imported resolutions under review

#### When

- The user accepts only some of them

#### Then

- Only explicitly accepted resolutions become active
- Others remain rejected or under review

#### Fail if

- Implicit acceptance occurs

---

## Section G — CLI-Critical Acceptance Tests (Single-User Mode)

### AT-CLI-1 CLI Initialization Flow

#### Given

- A new local Charter instance

#### When

- A user initializes an Area
- Defines Authority and Scope via sessions

#### Then

- Area becomes initialized
- Decisions may proceed

---

### AT-CLI-2 Export → Import Round Trip

#### Given

- A local Charter history
- An export saved to source control

#### When

- The export is imported unchanged

#### Then

- No resolution enters UNDER_REVIEW
- Full auditability is preserved

---

### AT-CLI-3 Consolidation Detects Divergence

#### Given

- A local initialized Area
- An imported export with divergent resolutions

#### When

- Imported in CONSOLIDATE mode

#### Then

- Imported resolutions are marked UNDER_REVIEW
- No local resolutions are altered

---

## Section H - References

### AT-REF-01: Sessions May Reference External Areas Explicitly

#### Given

- Area A-Product exists
- Area A-Finance exists
- Both Areas are initialized with Authority and Scope

#### When

- A session S-1 is started in Area A-Product
- S-1 explicitly references Area A-Finance

#### Then

- Session S-1 records the reference to A-Finance immutably
- No validation error occurs
- No Authority or Scope from A-Finance is applied

#### Fail if

- Referenced Area influences decision mechanics
- Referenced Area Authority is evaluated

---

### AT-REF-02: Sessions May Reference External Resolutions Explicitly

#### Given

- Area A-Finance has an active resolution R-Budget-7
- Area A-Product is initialized

#### When

- A session S-2 is started in A-Product
- S-2 explicitly references resolution R-Budget-7

#### Then

- Reference to R-Budget-7 is stored immutably in S-2
- R-Budget-7’s Area, Authority, and Scope are not applied
- Acceptance mechanics remain governed solely by A-Product

#### Fail if

- Referenced resolution affects acceptance
- Referenced resolution is revalidated or enforced

---

### AT-REF-03: Referenced Resolutions May Be Superseded Without Invalidating Sessions

#### Given

- Session S-3 references resolution R-Infra-2
- R-Infra-2 is ACTIVE at session start

#### When

- Another session supersedes R-Infra-2 with R-Infra-3

#### Then

- S-3 remains valid but requires revalidation before acceptance
- No automatic invalidation occurs
- No retroactive change to S-3’s stored reference

#### Fail if

- Session silently proceeds without revalidation
- Session is auto-closed or invalidated

---

### AT-REF-04: Multiple References Are Allowed and Independent

#### Given

- Session S-4 references:
	- Area A-Legal
	- Area A-Security
	- Resolution R-Compliance-9

#### When

- Session progresses normally

#### Then

- All references are preserved
- No priority or ordering is inferred
- No conflict is detected by the engine

#### Fail if

- Engine attempts to reconcile or rank references

---

### AT-REF-05: References Are Informational Only

#### Given

- Session S-5 references Area A-HR
- A candidate clearly violates HR policy (semantically)

#### When

- Authority rule is satisfied in S-5

#### Then

- Resolution is accepted
- No engine-level blocking occurs

#### Fail if

- Engine blocks acceptance based on referenced Area content

---

### AT-REF-06: Referenced Areas Do Not Require Initialization Compatibility

#### Given

- Session S-6 in Area A-Engineering
- A-Future exists but is uninitialized

#### When

- S-6 references Area A-Future

#### Then

- Reference is accepted
- No validation error occurs

#### Fail if

- Engine enforces governance completeness on referenced Areas

---

### AT-REF-07: References Are Immutable After Session Start

#### Given

- Session S-7 is started with references to Area A-X

#### When

- An attempt is made to add or remove references

#### Then

- Operation is rejected
- Original references remain unchanged

#### Fail if

- References mutate mid-session

---

### AT-REF-08: References Are Preserved in Export and Import

#### Given

- A session with references is exported
- The export is imported via RESTORE or CONSOLIDATE

#### Then

- All references are preserved exactly
- No inference or reconciliation occurs

#### Fail if

- References are dropped, normalized, or reinterpreted

---

## Final Note (Non-Test)

These acceptance tests intentionally do not:
- Enforce UX
- Define CLI flags
- Infer semantics
- Require rationale
- Require AI

They define the legitimacy envelope of Charter Core.

NEW

AT-AUD-01 — Audit Scope Supremacy
Given
An Area A-1 exists
Global Audit exists
When
Any auditable action occurs (session start, resolution acceptance, import, deletion)
Then
At least one audit record exists in a scope whose lifecycle outlives the subject
No auditable action is recorded only inside a destructible scope
Fail if
An action’s audit trail is lost when its subject is deleted
AT-AUD-02 — Area Deletion Emits Global Audit Event
Given
Area A-1 with existing sessions and resolutions
When
A-1 is deleted or replaced (explicitly or via RESTORE import)
Then
A Global Audit event records:
Area ID
Deletion or replacement reason
Timestamp
Area-scoped audit may terminate
Fail if
Area deletion removes the only audit record of the deletion
AT-IMP-01 — Import Is Always Globally Audited
Given
A valid import blob
When
import_area is called in any mode
Then
Global Audit records:
Import attempt
Mode (RESTORE / CONSOLIDATE)
Source fingerprint
Result (success / failure)
Fail if
Import leaves no global audit record
AT-IMP-02 — Consolidation Import Does Not Rewrite History
Given
Existing Area A-1 with active resolutions
Import blob containing overlapping resolutions
When
Import is executed in CONSOLIDATE mode
Then
Imported resolutions enter UNDER_REVIEW
Existing resolutions are unchanged
Global Audit records import
Area Audit records new UNDER_REVIEW entries
Fail if
Existing resolutions are superseded or altered automatically
AT-IMP-03 — Restore Import Emits Replacement Audit
Given
Existing Area A-1
Import blob containing a full Area history
When
Import is executed in RESTORE mode
Then
Existing Area A-1 is replaced
Global Audit records:
Replacement
Old Area ID
New Area ID
Imported Area history is preserved
Fail if
Prior Area disappears without a global audit record
AT-IMP-04 — Failed Import Does Not Mutate State
Given
Existing legitimate Areas and history
A malformed or tampered import blob
When
Import is attempted
Then
Import fails deterministically
No Areas, Sessions, or Resolutions are mutated
Global Audit records the failed attempt
Fail if
Partial state is created
Existing history is modified
