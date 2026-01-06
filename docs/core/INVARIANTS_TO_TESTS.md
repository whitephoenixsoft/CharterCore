# Charter Core — Acceptance Tests (Engine Only, Invariant-Driven)

These acceptance tests define the **minimum behavioral guarantees** of Charter Core.

If any test fails, the engine has violated a **core invariant**.

Acceptance tests assert **what must or must not happen**, never how it is implemented.
They intentionally do **not** specify UX, CLI flags, or workflow ergonomics.

---

## Section A — Core Legitimacy Invariants

### AT-1 — Explicit Decisions Only
**Given**
- An initialized Area with an active Authority and Scope
- A session with candidates and recorded stances

**When**
- No candidate satisfies the Authority rule

**Then**
- No resolution is created
- No implicit “winner” is inferred
- The session remains `ACTIVE` or enters `BLOCKED`

**Fail if**
- A resolution exists without explicit acceptance

**Notes**
- `BLOCKED` implies a deterministic unmet condition
- `ACTIVE` implies evaluation is incomplete

---

### AT-2 — Sessions Are the Sole Unit of Legitimacy
**Given**
- A candidate exists outside of any session

**When**
- An attempt is made to accept it

**Then**
- Acceptance is rejected
- No resolution is created

---

### AT-3 — Immutable Resolution History
**Given**
- A resolution R-1 has been accepted

**When**
- Any attempt is made to modify, overwrite, or delete R-1

**Then**
- The operation is rejected
- R-1 remains unchanged and queryable

**And**
- Only a new resolution may supersede or retire R-1

---

### AT-4 — Deterministic Evaluation
**Given**
- Identical session state:
  - same participants
  - same stances
  - same Authority rule

**When**
- Evaluation is run multiple times

**Then**
- The outcome is identical every time

**Fail if**
- Non-deterministic results occur

---

### AT-5 — No Semantic Interpretation
**Given**
- Candidates with arbitrary content
- Optional rationale text

**When**
- The engine evaluates or accepts candidates

**Then**
- Content and rationale are never interpreted
- Only mechanical Authority and Scope rules are applied

**Fail if**
- Meaning, wording, or intent affects outcome

---

## Section B — Areas, Authority, and Scope

### AT-6 — Areas Define Hard Governance Boundaries
**Given**
- Two Areas A and B
- Each has independent Authority and Scope

**When**
- A session is opened in Area A

**Then**
- Only Area A’s Authority governs the session
- Area B has no effect unless explicitly referenced

**Fail if**
- Authority or Scope from another Area is implicitly applied

---

### AT-7 — Authority Is a First-Class Resolution
**Given**
- An Area with an active Authority resolution A-AUTH-1

**When**
- A new Authority candidate is accepted

**Then**
- A-AUTH-2 is created
- A-AUTH-1 is marked `SUPERSEDED`
- Exactly one active Authority exists

**Fail if**
- Multiple active Authorities exist

---

### AT-8 — Scope Is a First-Class Resolution
**Given**
- An Area with an active Scope resolution S-1

**When**
- A new Scope candidate is accepted

**Then**
- S-2 is created
- S-1 is marked `SUPERSEDED`
- Exactly one active Scope exists

---

### AT-9 — Context Preservation (Authority & Scope)
**Given**
- A session accepts resolution R under Authority A-1 and Scope S-1

**When**
- Authority A-2 and Scope S-2 later become active

**Then**
- R permanently references A-1 and S-1
- R is not re-evaluated, altered, or flagged

**Fail if**
- Historical resolutions are altered by later context

---

### AT-10 — Area Initialization Is Required
**Given**
- An Area with no active Authority or Scope

**When**
- A non-Authority, non-Scope resolution is accepted

**Then**
- Acceptance is blocked
- Initialization is required

---

## Section C — Session Mechanics & Blocking

### AT-11 — Authority Rule Is Fixed per Session
**Given**
- A session is started

**Then**
- Exactly one Authority resolution governs the session
- The rule is fixed for the session’s lifetime

**Fail if**
- Authority changes mid-session without a new session

---

### AT-12 — Standing Is Action-Based
**Given**
- Authority rule = `UNANIMOUS_PRESENT`
- Participants: Alice, Bob, Charlie

**When**
- Alice and Bob cast stances
- Charlie takes no action

**Then**
- Present set = {Alice, Bob}
- Charlie is not counted

**Notes**
- A stance is any explicit engine-recorded ACCEPT / REJECT / ABSTAIN

---

### AT-13 — Explicit Disagreement Blocks Unanimity
**Given**
- Authority rule = `UNANIMOUS_PRESENT`
- Alice, Bob, Charlie are present

**When**
- Alice: ACCEPT
- Bob: ACCEPT
- Charlie: REJECT

**Then**
- Resolution cannot be accepted
- Session remains `BLOCKED`
- Objection is recorded in audit

---

### AT-14 — Session Blocking and Revalidation
**Given**
- A session is `PAUSED` or `BLOCKED`

**When**
- Authority, Scope, or a referenced Resolution changes

**Then**
- Session cannot resume without revalidation

**Fail if**
- Acceptance proceeds under changed legitimacy context

---

### AT-15 — Concurrent Sessions Are Isolated
**Given**
- Two sessions active in the same Area

**When**
- No Authority, Scope, or superseding resolution is accepted

**Then**
- Sessions do not interfere

---

### AT-16 — Supersession Triggers Revalidation
**Given**
- Session S references Resolution R
- Another session supersedes R

**Then**
- S requires revalidation
- S may not accept candidates until handled

---

## Section D — Resolution Lifecycle

### AT-17 — Explicit Resolution Lifecycle
**Given**
- A resolution R is `ACTIVE`

**When**
- It is superseded or retired via a session

**Then**
- Its lifecycle state changes explicitly
- R remains queryable forever

**Fail if**
- R disappears or is silently altered

---

### AT-18 — Resolution State Changes Require Sessions
**Given**
- A resolution R is `ACTIVE`

**When**
- An API attempts to mark it `SUPERSEDED` or `RETIRED` without a session

**Then**
- Operation is rejected
- Engine reports:
  “Resolution legitimacy may only change via a decision session.”

---

## Section E — Import / Export & Integrity

### AT-19 — Valid Export Imports Successfully
**Given**
- A Charter Core instance with Areas, Sessions, Resolutions
- A valid export generated by the engine

**When**
- The export is imported unchanged

**Then**
- Import succeeds
- All references are preserved
- No resolution is marked `UNDER_REVIEW`

---

### AT-20 — Tampered Export Is Detected
**Given**
- A valid export
- Manual modification of content or metadata

**When**
- The export is imported

**Then**
- Integrity verification fails  
  **OR**
- All affected resolutions are marked `UNDER_REVIEW`

---

### AT-21 — Structural Integrity Is Enforced
**Given**
- An export where:
  - a resolution references a missing session, or
  - a session references a missing Authority

**When**
- The export is imported

**Then**
- Import fails deterministically
- No partial state is created

---

### AT-22 — Failed Import Does Not Mutate History
**Given**
- An existing Charter Core instance

**When**
- A failed import attempt occurs

**Then**
- Existing history remains unchanged

---

### AT-23 — Flat Import Does Not Create Legitimacy
**Given**
- A flat list of resolutions with no sessions

**When**
- Imported in `CONSOLIDATE` mode

**Then**
- All resolutions are created as `UNDER_REVIEW`
- No Authority or Scope is inferred
- No resolution becomes `ACTIVE`

---

## Section F — References (Informational Only)

### AT-REF-01 — Sessions May Reference External Areas
### AT-REF-02 — Sessions May Reference External Resolutions
### AT-REF-03 — Superseded References Require Revalidation
### AT-REF-04 — Multiple References Are Independent
### AT-REF-05 — References Never Enforce Semantics
### AT-REF-06 — Referenced Areas Need Not Be Initialized
### AT-REF-07 — References Are Immutable Per Session
### AT-REF-08 — References Survive Export and Import

*(All preserved exactly as written; engine must not infer, rank, or enforce references.)*

---

## Section G — Audit Supremacy

### AT-AUD-01 — Audit Scope Supremacy
### AT-AUD-02 — Area Deletion Emits Global Audit Event

---

## Section H — Import Auditing

### AT-IMP-01 — Import Is Always Globally Audited
### AT-IMP-02 — Consolidation Import Does Not Rewrite History
### AT-IMP-03 — Restore Import Emits Replacement Audit
### AT-IMP-04 — Failed Import Does Not Mutate State

---

## Section I — Constraints & Resume Invariants

### AT-24 — Constraints Are Authority-Equivalent
### AT-25 — Resume Cannot Introduce New Legitimacy Conditions
### AT-26 — Constraints Must Be Declared at Session Start
### AT-27 — Candidate Set Freezes on First Stance

---
## Section J — Session Export Legitimacy

### AT-EXP-01 — Active Sessions Are Not Exportable as Legitimate Artifacts
**Given**
- An Area with:
    - One CLOSED session S-1
    - One ACTIVE session S-2

**When**
- An export is generated

**Then**
- S-1 is included in the export
- S-2 is excluded from the export
- No placeholder or partial representation of S-2 exists

**Fail if**
- An active or paused session appears in the export
- Partial deliberation is serialized

---
### AT-EXP-02 — Exported Resolutions Must Reference Closed Sessions
Given
A resolution R-1 referencing session S-1
And S-1 is CLOSED

When
An export is generated

Then
R-1 is exported normally

And Given
A resolution R-2 referencing session S-2
And S-2 is ACTIVE or PAUSED

When
An export is generated

Then
Export fails deterministically

Fail if
A resolution references a non-closed session
Resolution legitimacy is preserved without a closed session

### AT-EXP-03 — Export Does Not Mutate Session State
Given
An Area with an ACTIVE or PAUSED session
When
An export is generated
Then
Session state remains unchanged
No session is auto-closed
No timestamps or metadata change
Fail if
Export causes any session state transition

---
## Section K — Import & Consolidation Legitimacy Boundaries

### AT-IMP-LEG-01 — Imported Deliberation Has No Mechanical Effect
Given
An import blob containing:
Sessions
Candidates
Votes
Accepted resolutions
When
Import is executed in CONSOLIDATE mode
Then
Imported resolutions are materialized as UNDER_REVIEW
Imported sessions, candidates, and votes are ignored by the engine
No imported vote contributes to acceptance
Fail if
Imported deliberation affects local legitimacy
Votes are replayed or evaluated

### AT-IMP-LEG-02 — Legitimacy Cannot Be Forked Mid-Session
Given
System A with an ACTIVE session S-1
System B imports an export from A
When
Export is generated while S-1 is active
Then
S-1 does not appear in the export
No continuation of S-1 is possible in B
Fail if
A decision can be completed outside its original context

### AT-IMP-LEG-03 — Imported Sessions Are Non-Authoritative
Given
An import blob containing closed sessions
When
Imported into a local system
Then
Imported sessions are not eligible for evaluation
Imported sessions do not govern acceptance
Only locally created sessions may create legitimacy
Fail if
Imported sessions can accept candidates
Imported sessions influence authority evaluation

---
## Section L — Determinism & History Preservation

### AT-HIST-01 — Consolidation Preserves Resolution History, Not Deliberation
Given
An import blob containing:
Multiple superseded resolutions
Complex authority evolution
Session history
When
Imported in CONSOLIDATE mode
Then
Resolution lineage is preserved
Supersession chains remain intact
Deliberation history is not reactivated
Fail if
Engine attempts to reconcile deliberation
Imported session history is treated as authoritative

### AT-HIST-02 — No Implicit Reconstruction of Governance
Given
Imported resolutions without locally accepted Authority or Scope
When
Review begins
Then
No Authority or Scope is inferred
No resolution becomes ACTIVE without a local session
Fail if
Governance is assumed
Legitimacy is fabricated for convenience

---
## Section M — Storage Isolation & Instance Integrity

### AT-28 — Storage Root Isolation
Invariant: ENG-INV-13 — Storage Isolation
Given
Two Charter Core instances:
Instance A with storage root /data/charter/A
Instance B with storage root /data/charter/B
Each instance initializes an Area with identical labels but different engine IDs
When
A session and resolution are created in Instance A
Instance B performs any query (list areas, sessions, resolutions)
Then
Instance B sees no objects from Instance A
No shared IDs, references, or audit entries exist
Fail if
Any object created in one storage root is visible in another
Engine assumes a global singleton store

### AT-29 — Cross-Root Reference Is Rejected
Invariant: ENG-INV-13 — Storage Isolation
Given
Resolution R-A exists in storage root A
Session S-B exists in storage root B
When
An attempt is made to reference R-A from S-B
Then
The engine rejects the reference
Error indicates: “Referenced object not found in storage root”
Fail if
Engine allows cross-root references
Engine treats external IDs as opaque but valid

### AT-30 — Storage Root Is Explicit
Invariant: ENG-INV-13 — Storage Isolation
Given
An engine API invocation without a resolved storage root
When
Any mutating or query operation is attempted
Then
The engine rejects the operation
Error indicates missing or undefined storage context
Fail if
Engine falls back to an implicit default store
Engine silently initializes a new store

---
## Section N — Audit Scope Supremacy (Extension)
*(These complement earlier audit tests but apply to the new storage boundary.)*

### AT-31 — Global Audit Survives Storage Object Loss
Invariant: ENG-INV-14 — Audit Scope Supremacy
Given
An Area A-1 exists in a storage root
Global audit scope exists
A session and resolution are created in A-1
When
Area A-1 is deleted or replaced
Then
Global audit still records:
Area deletion or replacement
IDs of affected objects
No audit record is lost
Fail if
Deleting an Area removes the only audit trail of its deletion

### AT-32 — Audit Cannot Be Scoped Only to a Destructible Object
Invariant: ENG-INV-14 — Audit Scope Supremacy
Given
A session S-1 exists
An audit event is emitted for S-1
When
S-1 is deleted or becomes unreachable
Then
At least one audit record for that event still exists
Audit is recoverable via a surviving scope
Fail if
Audit data is only stored inside the deleted session

---

Section J — Storage Agnosticism & Identity
AT-ENG-STOR-01 — Engine Is Storage-Location Agnostic
Given
The engine is initialized with two different storage roots
Both storage roots contain identical logical Charter data
When
Identical engine operations are executed against each storage root
Then
All object identities, resolution outcomes, and audit records are identical
Fail if
Filesystem paths affect object identity
Engine behavior depends on current working directory
AT-ENG-STOR-02 — Object Identity Survives Storage Relocation
Given
An Area with Sessions, Resolutions, and Audit records
The underlying storage is moved or re-mounted
When
The engine is restarted pointing to the new location
Then
All object IDs remain unchanged
All references resolve correctly
Fail if
Object IDs change due to storage relocation
References break due to path changes
Section K — Audit Durability
AT-ENG-AUD-01 — Audit Scope Outlives Subject Scope
Given
An Area A-1 with Sessions and Resolutions
When
Area A-1 is deleted or replaced
Then
At least one audit record exists outside A-1’s scope
The deletion or replacement is permanently recorded
Fail if
Deleting A-1 removes the only audit record of the deletion
AT-ENG-AUD-02 — Import Replacement Is Globally Audited
Given
An existing Area A-OLD
A valid RESTORE import replacing it with Area A-NEW
When
The import succeeds
Then
A global audit record exists containing:
Old Area ID
New Area ID
Timestamp
Operation type (RESTORE / REPLACEMENT)
Fail if
The prior Area disappears without a durable audit trail
Section L — History Preservation
AT-ENG-HIST-01 — No Implicit History Deletion
Given
An Area with historical Sessions and Resolutions
When
Any of the following occur:
Supersession
CONSOLIDATE import
RESTORE import
Then
Historical objects remain queryable
Objects may change lifecycle state but are never erased
Fail if
Any historical object is deleted implicitly
AT-ENG-HIST-02 — Supersession Does Not Remove Prior Resolutions
Given
Resolution R-1 is ACTIVE
When
Resolution R-2 supersedes R-1
Then
R-1 remains queryable
R-1 state becomes SUPERSEDED
R-2 references R-1 explicitly
Fail if
R-1 disappears or loses identity
Section M — Export Integrity
AT-ENG-EXP-01 — Export Is a Complete Logical Snapshot
Given
An Area with interlinked Sessions, Resolutions, and Authority history
When
An export is generated
Then
The export contains all referenced objects
No external engine state is required to interpret it
Fail if
Importing the export requires missing context
Referential integrity depends on external state
AT-ENG-EXP-02 — Export Is Deterministically Rehydratable
Given
An export generated from Engine Instance A
When
The export is restored into a clean Engine Instance B
Then
The logical state of Areas, Resolutions, Sessions, and lifecycle states is identical
Fail if
Rehydration produces different legitimacy outcomes
Object relationships differ
Section N — Determinism Across Imports
AT-ENG-IMP-01 — Restore Import Creates No New Legitimacy
Given
A valid export containing accepted resolutions
When
The export is imported in RESTORE mode
Then
No new sessions are created
No resolutions are re-evaluated
No audit records imply new acceptance
Fail if
Import creates legitimacy beyond what existed
AT-ENG-IMP-02 — Failed Import Is Side-Effect Free
Given
Existing Areas and legitimate history
A malformed or tampered export
When
Import is attempted
Then
Import fails deterministically
No Areas, Sessions, Resolutions, or Audits are mutated
Fail if
Partial state is created
Existing history is altered
Section O — Closed-Session Enforcement (Engine Perspective)
AT-ENG-SES-01 — Active Sessions Are Not Imported
Given
An export containing CLOSED, ACTIVE, and PAUSED sessions
When
The engine processes the import
Then
Only CLOSED sessions are materialized
ACTIVE and PAUSED sessions are ignored
Fail if
Active sessions are imported
Session legitimacy can be forked privately
AT-ENG-SES-02 — Ignored Sessions Do Not Affect Legitimacy
Given
An export where an ACTIVE session references candidates and votes
When
The export is imported
Then
No resolution outcome depends on the ignored session
No audit record implies its continuation
Fail if
Ignored sessions influence acceptance or supersession

---

## Final Note

These acceptance tests intentionally do **not**:

- enforce UX
- define CLI commands
- infer semantics
- require rationale
- require AI
- define filesystem layout
- define server or daemon behavior

They define the **legitimacy envelope** of Charter Core.

If these pass, the engine is trustworthy.
If any test fails, the engine is incorrect — regardless of usability gains.

Everything else is a client concern.