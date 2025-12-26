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

## Section F — CLI-Critical Acceptance Tests (Single-User Mode)

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

## Final Note (Non-Test)

These acceptance tests intentionally do not:
- Enforce UX
- Define CLI flags
- Infer semantics
- Require rationale
- Require AI

They define the legitimacy envelope of Charter Core.