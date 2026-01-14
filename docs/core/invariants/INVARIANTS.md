# Charter Core — Formal Engine Invariants & Boundaries

**Status: LOCKED (V1)**  
Changes to these invariants require explicit justification and new simulations demonstrating preserved legitimacy.

---

## I. Core Principle

Charter Core is a **legitimacy engine**, not a reasoning engine, workflow engine, or collaboration tool.

Its sole responsibility is to ensure that decisions are:
- Explicit
- Auditable
- Governed
- Deterministic
- Non-retroactive

---

## II. Decision & Legitimacy Invariants

### 1. Explicit Decisions Only
No decision is legitimate unless it is explicitly accepted within a session.
- Silence is not consent
- Metadata is not acceptance
- Automation is not authority
- Inactivity has no meaning

### 2. Sessions Are the Unit of Legitimacy
Resolutions may only be accepted within sessions.

Sessions:
- Define participants
- Enforce authority rules
- Enforce session constraints
- Produce zero or more resolutions

### 3. Legitimacy Is Evaluated at Acceptance Time
A resolution’s legitimacy is determined solely by:
- The Authority active at acceptance
- The Scope active at acceptance
- The decision rule satisfied at acceptance

No future change may retroactively affect legitimacy.

### 4. Deterministic Evaluation
Given identical:
- participants
- stances
- authority
- constraints

The outcome must be identical.

### 5. Explicit Dissent Invariant
Charter Core must support explicit expression of disagreement.
- Silence or absence must never be interpreted as consent or rejection

---

## III. History & Immutability Invariants

### 6. Immutable History
Once accepted, a resolution is immutable.
- Resolutions are never edited
- Corrections require superseding resolutions
- History is append-only

### 7. Explicit Resolution Lifecycle
Resolutions transition only through explicit states:
- ACTIVE
- UNDER_REVIEW
- SUPERSEDED
- RETIRED

Rules:
- No resolution is ever removed
- UNDER_REVIEW resolutions may not govern authority or scope
- All transitions are auditable
- Legitimacy states change only via session acceptance
- Workflow states must not affect legitimacy

### 8. Relevance Is Human, Not Mechanical
Charter Core does not determine relevance.
Relevance is expressed only through:
- Supersession
- Retirement
- Clarifying resolutions

The engine must never auto-retire or suppress decisions.

---

## IV. Areas, Authority, and Scope

### 9. Areas Are Hard Governance Boundaries
Every resolution belongs to exactly one Area.
- No implicit overlap
- No inheritance
- Cross-area relevance must be explicit

### 10. Area Initialization Requirement
An Area must have:
- Exactly one active Authority resolution
- Exactly one active Scope resolution

Until then:
- The Area is UNINITIALIZED
- Only Authority/Scope-defining sessions are permitted

### 11. Authority Is a First-Class Resolution
Authority defines the mechanical rule for agreement.

Authority:
- Defines who has standing
- Defines how acceptance is evaluated
- Is purely mechanical

Authority does **not**:
- Interpret content
- Assign roles
- Judge correctness
- Encode semantics

Rules:
- Exactly one active Authority per Area
- Changes require a session
- Changes never rewrite history

### 12. Scope Is a First-Class, Descriptive Resolution
Scope documents applicability and intent.

Scope:
- Is descriptive, not enforcing
- Exists for audit and human clarity
- Is immutable per resolution

Rules:
- Exactly one active Scope per Area
- Changes require a session
- Changes never invalidate prior resolutions

### 13. Context Preservation (Authority & Scope)
Every session and resolution must permanently record:
- Authority active at acceptance
- Scope active at acceptance
- Any explicitly referenced scopes

Later changes must never reinterpret past decisions.

---

## V. Sessions, Constraints, and Candidates

### 14. Candidates Are Neutral
Candidates:
- Imply no intent
- Imply no endorsement
- Have no legitimacy unless accepted

Rejection or abandonment has no semantic meaning.

### 15. Candidate Set Freezes on First Stance
After any stance is recorded:
- No candidate may be added, removed, or edited
- Violations must fail explicitly

### 16. Session Constraints Are Engine-Enforced
Constraints:
- Are declared at session start
- Apply only to that session
- Are enforced mechanically
- Do not modify Authority or Scope

### 17. Constraints Are Authority-Equivalent
Any rule that changes how agreement is evaluated is authority-equivalent.

Consequences:
- Constraints cannot change mid-session
- Constraints require a governing decision session
- Constraints are governed by active Authority

### 18. Constraints Must Be Declared at Session Start
Fail if:
- Constraints are added after first stance
- Constraints change after pause or resume
- Constraints are inferred implicitly

### 19. Session Blocking and Pausing
If authority or constraints cannot be satisfied:
- Session enters BLOCKED or PAUSED

On resume:
- Authority and Scope are revalidated
- If legitimacy conditions differ → BLOCKED

Resume restores context; it does not renegotiate it.

### 20. Resume Cannot Introduce New Legitimacy Conditions
On resume:
- Participants may change
- Votes may change
- Authority and constraints may not

---

## VI. Concurrency & Isolation

### 21. Concurrency Invariant
Multiple sessions may exist concurrently in an Area.

Interference occurs only if a resolution:
- Changes Authority
- Changes Scope
- Supersedes an active resolution

Affected sessions must be revalidated, paused, or blocked.

### 22. Legitimacy Cannot Be Forked Mid-Process
The engine must prevent:
- Forking active sessions
- Completing decisions outside original context

Fail if:
- In-progress sessions can be finalized elsewhere

---

## VII. Imports, Exports, and Review

### 23. Export Invariants

#### EXP-01 — Only Closed Sessions Are Legitimate Artifacts
- Only CLOSED sessions may participate in legitimacy-bearing exports
- Active or paused sessions must be ignored (with warning)

#### EXP-02 — Exported Resolutions Must Originate from Closed Sessions
Fail if:
- A resolution references an open or paused session

### 24. Import Invariants

#### IMP-01 — Consolidation Preserves Legitimacy, Not Deliberation
In CONSOLIDATE mode:
- Imported resolutions are historical artifacts
- Imported deliberation is non-authoritative

Fail if:
- Imported votes or sessions affect acceptance

#### IMP-02 — Imported Session History Is Non-Authoritative
If preserved:
- Must be read-only
- Must never govern legitimacy

### 25. Import Review Invariants

- **IR-1 — Chronological Review**
  Imported resolutions must be reviewed in original order.

- **IR-2 — Local Authority Governs Review**
  Imported Authority/Scope never govern review mechanics.

- **IR-3 — No Cascading Rejection**
  Rejecting one imported resolution does not affect others.

- **IR-4 — Context Preservation Without Reinterpretation**
  Imported context is preserved for audit only.

---

## VIII. References & Semantics

### 26. References Are Informational Only
References:
- Grant no authority
- Imply no approval
- Create no precedence
- Trigger no enforcement

All effects are external to the engine.

### 27. No Semantic Inference
Charter Core must never infer:
- Authority overlap
- Scope overlap
- Role equivalence
- Intent

---

## IX. Audit & Integrity

### 28. Audit Scope Supremacy
All auditable events must be recorded in a scope that outlives the subject.

Rules:
- At least one non-deletable Global Audit exists
- Deleting an entity must not erase its audit trail

### 29. Verifiable Export Integrity
Exports must allow detection of:
- Structural tampering
- Content modification

On failure:
- Reject import or mark UNDER_REVIEW

This is detection, not cryptographic trust.

---

## X. Storage & Persistence Invariants

### 30. Storage Isolation
Each engine instance operates over a single explicit storage root.
- No cross-root visibility
- No implicit global state

### 31. Engine Is Storage-Location Agnostic (ENG-STOR-01)
The engine:
- Receives a storage root from its host
- Treats storage as opaque

Fail if:
- Engine depends on filesystem layout or CWD

### 32. Stable Object Identity (ENG-STOR-02)
Object identities must be:
- Stable across restarts
- Stable across exports/imports
- Independent of filesystem paths

### 33. Audit Scope Outlives Subject (ENG-STOR-03)
Fail if:
- Deleting an entity deletes the only audit record

### 34. No Implicit History Deletion (ENG-STOR-04)
The engine must never delete history implicitly.
History may only be superseded or abandoned with audit.

### 35. Export Is a Complete Logical Snapshot (ENG-STOR-05)
Exports must be:
- Complete
- Referentially intact
- Deterministically rehydratable

---

## XI. AI Boundary

### 36. AI Is Outside the Engine Boundary
AI may:
- Suggest
- Annotate
- Warn

AI may never:
- Accept decisions
- Modify resolutions
- Override authority
- Bypass constraints

---

## XII. Frozen Non-Goals (Boundaries)

Charter Core explicitly does **not** provide:
- Chat systems
- Workflow orchestration
- Task execution
- Role management
- Identity systems
- Semantic reasoning
- Inferred conflict resolution
- UX patterns (rounds, moderation, facilitation)

These belong to higher layers.

---
NEW

# Charter Core — Engine Invariants
Status: FROZEN
Applies to: Charter Core Engine
Violations indicate a correctness failure.

## 1. Identity & Immutability

ENG-INV-01: Accepted Resolutions Are Immutable  
Once a resolution is accepted:
- Its content must never change
- Its acceptance context must never change
- It may only be superseded or retired explicitly

ENG-INV-02: Canonical Engine IDs  
All engine entities must have stable, canonical IDs.
IDs must not change due to:
- relabeling
- formatting changes
- storage reorganization

## 2. Authority & Legitimacy

ENG-INV-03: Legitimacy Is Explicit  
A resolution is legitimate only if:
- accepted via a session
- evaluated under a declared authority
- within an explicit scope

ENG-INV-04: Authority Is Evaluated Mechanically  
Authority evaluation must:
- be deterministic
- rely only on recorded session state
- never infer intent or silence

ENG-INV-05: Participant Snapshot Is Binding  
The participant set at first stance:
- is frozen for legitimacy evaluation
- must be auditable
- must not be inferred or reconstructed

## 3. Temporal Semantics

ENG-INV-06: Acceptance Has a Moment  
Every accepted resolution must have:
- a single, explicit acceptance moment
- no retroactive effect

ENG-INV-07: Non-Retroactivity  
New rules, authorities, or specs:
- must not reinterpret accepted history
- apply only to future sessions

ENG-INV-08: Supersession Is Explicit  
A resolution may only stop being active via:
- explicit supersession
- explicit retirement
Never by deletion or mutation.

## 4. Scope & Isolation

ENG-INV-09: Areas Are Isolated  
No resolution may:
- implicitly affect another Area
- be evaluated across Areas without explicit linkage

ENG-INV-10: Scope Is Declarative  
Scope determines applicability.
Scope must never be inferred from content or labels.

## 5. Baseline & Import Semantics

ENG-INV-11: Baseline Is Non-Legitimizing  
Baseline review:
- does not evaluate authority
- does not create legitimacy
- prepares resolutions for explicit acceptance only

ENG-INV-12: Acceptance Always Occurs via Sessions  
No resolution may become ACTIVE without a session.

ENG-INV-13: Restore Is Destructive and Explicit  
RESTORE:
- replaces all engine state
- terminates all sessions and baselines
- emits an auditable global event

## 6. Determinism & Audit

ENG-INV-14: Deterministic Evaluation  
Given the same inputs:
- the engine must produce the same outputs

ENG-INV-15: Audits Are Read-Only  
Audit operations must never mutate state.

## 7. Embedded Specs

ENG-INV-16: Specs Are Immutable per Version  
Embedded specs:
- must not change without an engine version bump
- are authoritative for that binary

ENG-INV-17: Spec Verification Is Mechanical  
Spec verification must:
- detect missing enforcement
- detect altered locked specs
- fail deterministically