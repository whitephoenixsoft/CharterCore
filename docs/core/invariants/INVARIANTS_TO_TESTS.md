# Charter Core — Acceptance Tests (Engine Only, Invariant-Driven)

These acceptance tests define the **minimum behavioral guarantees** of Charter Core.

If any test fails, the engine has violated a **core invariant**.

Acceptance tests assert **what must or must not happen**, never how it is implemented.
They intentionally do **not** specify UX, CLI flags, workflows, or ergonomics.

---

## A. Core Legitimacy & Determinism

### AT-1 — Explicit Decisions Only
Given:
- An initialized Area with active Authority and Scope
- A session with candidates and recorded stances

When:
- No candidate satisfies the Authority rule

Then:
- No resolution is created
- No implicit “winner” is inferred
- Session remains ACTIVE or becomes BLOCKED

Fail if:
- A resolution exists without explicit acceptance

---

### AT-2 — Sessions Are the Sole Unit of Legitimacy
Given:
- A candidate exists outside of any session

When:
- An attempt is made to accept it

Then:
- Acceptance is rejected
- No resolution is created

---

### AT-3 — Immutable Resolution History
Given:
- A resolution R-1 has been accepted

When:
- Any attempt is made to modify, overwrite, or delete R-1

Then:
- Operation is rejected
- R-1 remains unchanged and queryable

And:
- Only a new resolution may supersede or retire R-1

---

### AT-4 — Deterministic Evaluation
Given:
- Identical session state:
  - same participants
  - same stances
  - same Authority
  - same constraints

When:
- Evaluation runs multiple times

Then:
- Outcome is identical every time

Fail if:
- Results are non-deterministic

---

### AT-5 — No Semantic Interpretation
Given:
- Candidates with arbitrary content
- Optional rationale text

When:
- The engine evaluates acceptance

Then:
- Content and rationale are never interpreted
- Only mechanical rules are applied

Fail if:
- Meaning, wording, or intent affects outcome

---

## B. Areas, Authority, and Scope

### AT-6 — Areas Are Hard Governance Boundaries
Given:
- Two Areas A and B
- Independent Authority and Scope in each

When:
- A session runs in Area A

Then:
- Only Area A’s Authority governs
- Area B has no effect unless explicitly referenced

Fail if:
- Cross-Area authority leaks implicitly

---

### AT-7 — Authority Is a First-Class Resolution
Given:
- Area with active Authority A-AUTH-1

When:
- A new Authority is accepted

Then:
- A-AUTH-2 becomes ACTIVE
- A-AUTH-1 becomes SUPERSEDED
- Exactly one active Authority exists

Fail if:
- Multiple active Authorities exist

---

### AT-8 — Scope Is a First-Class Resolution
Given:
- Area with active Scope S-1

When:
- A new Scope is accepted

Then:
- S-2 becomes ACTIVE
- S-1 becomes SUPERSEDED
- Exactly one active Scope exists

---

### AT-9 — Context Preservation
Given:
- Resolution R accepted under Authority A-1 and Scope S-1

When:
- Authority A-2 and Scope S-2 later become active

Then:
- R permanently references A-1 and S-1
- R is not altered or re-evaluated

Fail if:
- Historical legitimacy is reinterpreted

---

### AT-10 — Area Initialization Is Required
Given:
- An Area with no active Authority or Scope

When:
- A non-Authority, non-Scope resolution is attempted

Then:
- Acceptance is blocked
- Initialization is required

---

## C. Session Mechanics & Concurrency

### AT-11 — Authority Is Fixed Per Session
Given:
- A session has started

Then:
- Exactly one Authority governs the session
- Authority does not change mid-session

Fail if:
- Authority changes without a new session

---

### AT-12 — Standing Is Action-Based
Given:
- Authority = UNANIMOUS_PRESENT
- Participants: Alice, Bob, Charlie

When:
- Alice and Bob record stances
- Charlie records nothing

Then:
- Present set = {Alice, Bob}
- Charlie is not counted

---

### AT-13 — Explicit Disagreement Blocks Unanimity
Given:
- UNANIMOUS_PRESENT authority
- Alice ACCEPT, Bob ACCEPT, Charlie REJECT

Then:
- Resolution cannot be accepted
- Session becomes BLOCKED
- Dissent is auditable

---

### AT-14 — Blocking & Revalidation
Given:
- Session is PAUSED or BLOCKED

When:
- Authority, Scope, or referenced Resolution changes

Then:
- Session cannot resume without revalidation

Fail if:
- Acceptance proceeds under changed context

---

### AT-15 — Concurrent Sessions Are Isolated
Given:
- Two sessions active in the same Area

When:
- No superseding Authority, Scope, or Resolution is accepted

Then:
- Sessions do not interfere

---

### AT-16 — Supersession Triggers Revalidation
Given:
- Session S references Resolution R
- Another session supersedes R

Then:
- S requires revalidation
- S cannot accept until handled

---

## D. Resolution Lifecycle

### AT-17 — Explicit Lifecycle Transitions
Given:
- Resolution R is ACTIVE

When:
- It is superseded or retired via session

Then:
- State changes explicitly
- R remains queryable forever

Fail if:
- R disappears or mutates silently

---

### AT-18 — Lifecycle Changes Require Sessions
Given:
- Resolution R is ACTIVE

When:
- API attempts to mark it SUPERSEDED or RETIRED directly

Then:
- Operation is rejected
- Engine reports legitimacy violation

---

## E. Import, Export & Integrity

### AT-19 — Valid Export Imports Cleanly
Given:
- Engine-generated export

When:
- Imported unchanged

Then:
- Import succeeds
- References preserved
- No resolution marked UNDER_REVIEW

---

### AT-20 — Tampering Is Detected
Given:
- Modified export

When:
- Import attempted

Then:
- Import fails OR affected resolutions are UNDER_REVIEW

---

### AT-21 — Structural Integrity Enforced
Given:
- Export with missing references

When:
- Import attempted

Then:
- Import fails deterministically
- No partial state created

---

### AT-22 — Failed Import Is Side-Effect Free
Given:
- Existing history

When:
- Import fails

Then:
- Existing state remains unchanged

---

### AT-23 — Flat Import Creates No Legitimacy
Given:
- Flat resolutions without sessions

When:
- Imported in CONSOLIDATE mode

Then:
- All resolutions are UNDER_REVIEW
- No Authority or Scope inferred
- No ACTIVE resolutions created

---

## F. References (Informational Only)

### AT-REF-01 through AT-REF-08
The engine must ensure:
- References grant no authority
- References enforce no semantics
- References survive export/import
- References never imply approval, precedence, or obligation

Fail if:
- References affect legitimacy mechanically

---

## G. Audit Supremacy

### AT-AUD-01 — Audit Scope Supremacy
All auditable events must survive beyond subject deletion.

---

### AT-AUD-02 — Area Deletion Emits Global Audit
Deleting or replacing an Area must emit a global audit record.

---

## H. Constraints & Resume Invariants

### AT-24 — Constraints Are Authority-Equivalent
Constraints changing legitimacy mechanics require sessions.

---

### AT-25 — Resume Cannot Introduce New Legitimacy Conditions
Authority and constraints may not change on resume.

---

### AT-26 — Constraints Declared at Session Start
Fail if constraints are added after first stance.

---

### AT-27 — Candidate Set Freezes on First Stance
Fail if candidates change after voting begins.

---

## I. Export Legitimacy Boundaries

### AT-EXP-01 — Active Sessions Are Not Exported
Active or paused sessions must not appear in exports.

---

### AT-EXP-02 — Exported Resolutions Reference Closed Sessions Only
Fail if:
- A resolution references a non-closed session

---

### AT-EXP-03 — Export Does Not Mutate State
Export must not close, pause, or modify sessions.

---

## J. Import Legitimacy Boundaries

### AT-IMP-LEG-01 — Imported Deliberation Has No Effect
Votes, candidates, and sessions in imports must not affect acceptance.

---

### AT-IMP-LEG-02 — Legitimacy Cannot Be Forked
Active sessions must never be importable.

---

### AT-IMP-LEG-03 — Imported Sessions Are Non-Authoritative
Imported sessions may never accept candidates locally.

---

## K. Storage Isolation & Identity

### AT-28 — Storage Root Isolation
No visibility across storage roots.

---

### AT-29 — Cross-Root References Are Rejected
Fail if:
- Objects in different roots can reference each other

---

### AT-30 — Storage Root Is Explicit
Fail if:
- Engine operates without an explicit storage root

---

## L. History Preservation

### AT-HIST-01 — Consolidation Preserves History, Not Deliberation
Resolution lineage preserved; deliberation inert.

---

### AT-HIST-02 — No Implicit Governance Reconstruction
Fail if:
- Authority or Scope is inferred from imports

---

## M. Audit Durability & Export Completeness

### AT-ENG-EXP-01 — Export Is a Complete Snapshot
Export must be self-contained and referentially complete.

---

### AT-ENG-EXP-02 — Export Is Deterministically Rehydratable
Restored state must be logically identical.

---

## Final Note

These acceptance tests intentionally do **not**:
- Enforce UX
- Define CLI commands
- Infer semantics
- Require rationale
- Require AI
- Define filesystem layout
- Define server or daemon behavior

They define the **legitimacy envelope** of Charter Core.

If these pass, the engine is trustworthy.
If any fail, the engine is incorrect — regardless of usability gains.

---

# Charter Core — Acceptance Tests
## Voting & Acceptance Semantics

Status: CANONICAL  
Applies to: Charter Core Engine  
Test Type: Black-box legitimacy validation

---

## AT-01 — Votes Without Acceptance Create No Legitimacy

**Given**
- A session with valid authority
- Participants have all voted ACCEPT

**When**
- No acceptance action is executed

**Then**
- No resolution becomes ACTIVE
- Session remains open
- Audit shows votes only

---

## AT-02 — Acceptance Fails If Authority Not Satisfied

**Given**
- Authority: UNANIMOUS_PRESENT
- Participants: Alice, Bob
- Votes:
  - Alice: ACCEPT
  - Bob: REJECT

**When**
- Acceptance is attempted

**Then**
- Acceptance is rejected
- Session remains open
- No resolution is created
- Audit records failed acceptance attempt

---

## AT-03 — Vote Change Enables Later Acceptance

**Given**
- Same setup as AT-02

**When**
- Bob changes vote to ACCEPT
- Acceptance is attempted

**Then**
- Authority evaluates to satisfied
- Resolution is accepted
- Session closes
- Votes are frozen

---

## AT-04 — Acceptance Freezes Votes

**Given**
- A session where acceptance has occurred

**When**
- A participant attempts to change a vote

**Then**
- Engine rejects the mutation
- Audit records attempted mutation
- Resolution remains unchanged

---

## AT-05 — Authority Evaluated Only at Acceptance

**Given**
- Votes fluctuate over time
- Authority would sometimes pass and sometimes fail

**When**
- No acceptance is attempted

**Then**
- No legitimacy is created
- No blocking occurs
- Session remains evaluative only

---

## AT-06 — Acceptance Cannot Be Replayed

**Given**
- A session where acceptance has occurred

**When**
- Acceptance is attempted again

**Then**
- Engine rejects the action
- No additional resolutions are created
- Audit records invalid attempt

---

## AT-07 — Acceptance Uses Frozen Participant Set

**Given**
- Participants frozen at first stance
- Votes satisfy authority

**When**
- Acceptance is attempted

**Then**
- Authority evaluation uses frozen participants only
- Late joiners are ignored
- Outcome is deterministic
