# Charter Core — Acceptance Tests (Invariant-Driven)

---

## AT-1 Explicit Decisions Only

### Given

- An initialized Area with valid Authority and Scope
- A session with candidates and recorded positions

### When

No candidate satisfies the Authority rule

### Then

- No resolution is created
- No implicit “winner” is inferred
- Session remains ACTIVE or BLOCKED

### Fail if

A resolution exists without explicit acceptance

---

## AT-2 Immutable History

### Given

An accepted resolution R-1

### When

Any attempt is made to modify, overwrite, or delete R-1

### Then

- The operation is rejected
- R-1 remains unchanged

### And

Only a new resolution may supersede or retire R-1

---

## AT-3 Areas Define Governance Boundaries

### Given

- Two Areas A and B
- Each has independent Authority and Scope

### When

A session is opened in Area A

Then

- Only Area A’s Authority governs the session
- Area B has no effect unless explicitly referenced

### Fail if

Authority or Scope from another Area is implicitly applied

---

## AT-4 Authority Is a First-Class Resolution

### Given

An Area with an active Authority resolution A-AUTH-1

### When

A new Authority candidate is accepted

### Then

- A new Authority resolution A-AUTH-2 is created
- A-AUTH-1 is marked Superseded or Retired
- No other Authority remains active

### Fail if

Multiple active Authorities exist in the same Area

---

## AT-5 Scope Is a First-Class Resolution

### Given

An Area with an active Scope resolution S-1

### When

A new Scope candidate is accepted

### Then

- A new Scope resolution S-2 is created
- S-1 is superseded or retired
- S-2 becomes the only active Scope

---

## AT-6 Context Preservation (Authority & Scope)

### Given

- A session S accepted under Authority A-1 and Scope S-1
- Later, Authority A-2 and Scope S-2 become active

### Then

- Resolution R created in S permanently references A-1 and S-1
- R is not re-evaluated or invalidated

### Fail if

Historical resolutions are altered by later context changes

---

## AT-7 Sessions Are the Sole Unit of Legitimacy

### Given

A candidate exists outside of any session

### When

An attempt is made to accept it

### Then

Acceptance is rejected

### And

No resolution is created

---

## AT-8 Candidates Are Neutral

### Given

Multiple candidates exist in a session

### When

Some candidates receive no positions or discussion

### Then

- No effect occurs
- Only accepted candidates produce resolutions

### Fail if

Mere existence or ordering affects outcome

---

## AT-9 Deterministic Evaluation

### Given

Identical session state:
- same participants
- same positions
- same Authority rule

### When

Evaluation is run multiple times

### Then

The outcome is identical every time

### Fail if

Non-deterministic results occur

---

## AT-10 Explicit Resolution Lifecycle

### Given

A resolution R-1 is Active

### When

It is superseded or retired

### Then

- Its lifecycle state changes explicitly
- R-1 remains queryable

### Fail if

R-1 disappears or is silently altered

---

## AT-11 No Generic Policy Streams

### Given

Resolutions of arbitrary types exist

### When

They are processed by the engine

### Then

- Only Authority and Scope receive special handling
- All others are treated uniformly

### Fail if

Engine behavior varies by resolution type beyond Authority/Scope

---

## AT-12 Transparency of Governing Context

### Given

A session is opened

### Then

The engine can return:
- active Authority
- active Scope
- any explicitly referenced Scopes

### Fail if

A resolution can be accepted without this information being available

---

## AT-13 Decision Rules Announced at Session Start

### Given

A session is opened

### Then

- Exactly one Authority resolution governs the session
- That rule remains fixed for the session’s lifetime

### Fail if

The decision rule changes mid-session without closing or revalidation

---

## AT-14 Session Blocking and Revalidation

### Given

A session is BLOCKED or PAUSED

### When

Authority or Scope changes externally

### Then

Session cannot resume without revalidation

### Fail if

Acceptance proceeds under changed context without detection

---

## AT-15 No Permissions or Identity Semantics

### Given

Participant identifiers are provided

### Then

- Engine treats them as opaque identifiers
- No permission checks occur internally

### Fail if

Engine enforces roles, ranks, or access rules

---

## AT-16 No Side Effects Beyond State

### Given

A resolution is accepted

### Then

Only internal state is updated

### Fail if

- External systems are invoked
- Tasks or workflows are triggered

---

## AT-17 Area Name Does Not Affect Legitimacy or Enforcement

### Given

- An Area A-1 is created with:
    - Name: "Backend Architecture"
- An initialization session creates:
    - Authority resolution R-AUTH-1
    - Scope resolution R-SCOPE-1: “Decisions related to backend architecture, databases, and APIs.”

### When

1. A session S-1 is started in Area A-1.
2. A candidate C-1 is proposed: “Adopt PostgreSQL as primary database.”
3. Candidate C-1 is accepted under R-AUTH-1.

### Then

- Resolution R-DB-1 is created with metadata:
    - Area: A-1
    - Authority: R-AUTH-1
    - Scope: R-SCOPE-1
- The Area name "Backend Architecture":
    - Is not evaluated
    - Is not parsed
    - Is not used in authority checks
    - Is not used in scope validation
    - 
### And When (Rename Case)

4. The Area name is changed to "Platform Engineering" without a session.

### Then

- No resolutions are created
- No authority or scope changes occur
- All existing resolutions remain valid
- Future sessions still reference:
    - A-1
    - The currently active Authority and Scope resolutions

### And When (Misleading Name Case)

5. The Area name is changed to "Company Legal Policies".

### Then

- The engine:
    - Does not block decisions
    - Does not raise conflicts
    - Does not infer scope or authority changes
- Only explicit Scope or Authority resolutions can change applicability

### Pass Condition

✔ Area name changes never require a session
✔ Area name changes never affect legitimacy
✔ Area name is never used for semantic inference