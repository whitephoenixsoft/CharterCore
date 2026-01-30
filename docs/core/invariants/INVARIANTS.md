# Charter Core — Engine Invariants
Status: FROZEN (V1)  
Applies to: Charter Core Engine  
Violations indicate an engine correctness failure

---

## Purpose

These invariants define the **non-negotiable behavior** of the Charter Core.

They exist to preserve:

- legitimacy
- determinism
- auditability
- non-retroactivity

Charter Core is intentionally narrow.

If behavior is not guaranteed here,
it must be implemented **outside** the engine.

If an implementation violates an invariant,
the implementation is wrong — not the invariant.

---

## I. Core Boundary

### ENG-CORE-01 — Charter Core Is a Legitimacy Engine

Charter Core is **not**:

- a reasoning engine
- a workflow engine
- a collaboration system
- a facilitation tool
- a semantic interpreter

Its sole responsibility is to ensure that decisions are:

- explicit
- governed
- deterministic
- auditable
- non-retroactive

Fail if:

- engine behavior implies reasoning, facilitation, or interpretation

---

## II. Identity & Immutability

### ENG-ID-01 — Canonical Engine Identity

All engine entities  
(Areas, Sessions, Resolutions, Candidates, Authority, Scope, Participants)
MUST have stable, canonical engine IDs.

Rules:

- IDs are stable across restarts
- IDs survive export/import
- IDs are independent of labels, formatting, or storage layout

Fail if:

- identity changes due to relabeling or reorganization

---

### ENG-ID-02 — Accepted Resolutions Are Immutable

Once a resolution is accepted:

- its content must never change
- its acceptance context must never change

A resolution may only change status via:

- explicit supersession
- explicit retirement

Fail if:

- accepted content is edited or reinterpreted

---

## III. Legitimacy & Decision Semantics

### ENG-LEG-01 — Explicit Decisions Only

No decision is legitimate unless it is **explicitly accepted** within a session.

Rules:

- silence is not consent
- metadata is not acceptance
- automation is not authority
- inactivity has no meaning

Fail if:

- legitimacy appears without explicit acceptance

---

### ENG-LEG-02 — Sessions Are the Unit of Legitimacy

Resolutions may only be accepted **within sessions**.

Sessions:

- define participants
- enforce authority rules
- enforce constraints
- produce zero or more accepted resolutions

Fail if:

- legitimacy exists outside a session

---

### ENG-LEG-03 — Legitimacy Is Evaluated at Acceptance Time

A resolution’s legitimacy is determined **only** by:

- Authority active at acceptance
- Scope active at acceptance
- Constraints active at acceptance
- Recorded stances at acceptance

Later changes MUST NOT reinterpret legitimacy.

Fail if:

- future state alters past legitimacy

---

### ENG-LEG-04 — Deterministic Evaluation

Given identical:

- participants
- stances
- authority
- constraints

The outcome MUST be identical.

Fail if:

- evaluation is non-deterministic

---

### ENG-LEG-05 — Explicit Dissent Is First-Class

The engine MUST support explicit disagreement.

Rules:

- abstention is explicit
- silence has no meaning
- absence is not rejection

Fail if:

- dissent is inferred or implied

---

## IV. Resolution Lifecycle & History

### ENG-HIST-01 — History Is Append-Only

Accepted resolutions:

- are never edited
- are never deleted
- remain auditable indefinitely

Corrections require:

- superseding resolutions
- clarifying resolutions

Fail if:

- history mutates implicitly

---

### ENG-HIST-02 — Explicit Resolution States

Resolutions transition only through explicit states:

- ACTIVE
- UNDER_REVIEW
- SUPERSEDED
- RETIRED

Rules:

- no resolution is ever removed
- UNDER_REVIEW has no governing power
- all transitions are auditable
- legitimacy states change only via sessions
- workflow states must not affect legitimacy

Fail if:

- resolution state changes implicitly

---

### ENG-HIST-03 — Relevance Is Human, Not Mechanical

The engine does not determine relevance.

Relevance is expressed only through:

- supersession
- retirement
- clarifying resolutions

Fail if:

- the engine auto-retires, suppresses, or prioritizes decisions

---

## V. Areas, Authority, and Scope

### ENG-AREA-01 — Areas Are Hard Governance Boundaries

Every resolution belongs to **exactly one Area**.

Rules:

- no implicit overlap
- no inheritance
- cross-area relevance must be explicit

Fail if:

- authority or scope leaks across Areas

---

### ENG-AREA-02 — Area Initialization Is Mandatory

An Area MUST have:

- exactly one active Authority resolution
- exactly one active Scope resolution

Until then:

- the Area is UNINITIALIZED
- only Authority- or Scope-defining sessions are permitted

Fail if:

- decisions occur in an uninitialized Area

---

### ENG-AUTH-01 — Authority Is First-Class and Mechanical

Authority defines **how agreement is evaluated**.

Authority:

- defines standing
- defines acceptance mechanics
- is purely mechanical

Authority does NOT:

- interpret content
- assign roles
- judge correctness
- encode semantics

Rules:

- exactly one active Authority per Area
- changes require a session
- changes never rewrite history

Fail if:

- authority implies meaning or intent

---

### ENG-SCOPE-01 — Scope Is First-Class and Descriptive

Scope documents applicability and intent.

Scope:

- is descriptive, not enforcing
- exists for audit and clarity
- is immutable per resolution

Rules:

- exactly one active Scope per Area
- changes require a session
- changes never invalidate prior resolutions

Fail if:

- scope is inferred or enforced mechanically

---

### ENG-CONTEXT-01 — Authority and Scope Are Permanently Recorded

Every session and accepted resolution MUST permanently record:

- Authority active at acceptance
- Scope active at acceptance
- any explicitly referenced scopes

Fail if:

- context can only be reconstructed implicitly

---

## VI. Sessions, Candidates, and Constraints

### ENG-SES-01 — Candidates Are Neutral

Candidates:

- imply no intent
- imply no endorsement
- have no legitimacy unless accepted

Rejection or abandonment has no semantic meaning.

Fail if:

- candidates are treated as decisions

---

### ENG-SES-02 — Candidate Set Freezes on First Stance

After any stance is recorded:

- candidates may not be added
- candidates may not be removed
- candidates may not be edited

Fail if:

- mutation occurs after stances begin

---

### ENG-CON-01 — Constraints Are Engine-Enforced

Constraints:

- are declared at session start
- apply only to that session
- are enforced mechanically
- do not modify Authority or Scope

Fail if:

- constraints are inferred or modified implicitly

---

### ENG-CON-02 — Constraints Are Authority-Equivalent

Any rule that alters agreement evaluation is authority-equivalent.

Consequences:

- constraints cannot change mid-session
- constraints cannot change on resume
- constraints require a governing session

Fail if:

- constraints bypass authority governance

---

### ENG-SES-03 — Pause, Block, and Resume Preserve Context

If legitimacy conditions cannot be satisfied:

- a session becomes PAUSED or BLOCKED

On resume:

- Authority and Scope are revalidated
- if legitimacy conditions differ → BLOCKED

Fail if:

- resume alters legitimacy conditions

---

### ENG-SES-04 — Resume Cannot Introduce New Legitimacy Rules

On resume:

- participants may change
- stances may change
- authority and constraints may NOT

Fail if:

- legitimacy rules change on resume

---

## VII. Voting & Acceptance (Legitimacy Creation)

### ENG-VOTE-01 — Votes Are Evaluative Only

Recorded stances:

- ACCEPT
- REJECT
- ABSTAIN

Votes:

- create no legitimacy
- may change prior to acceptance
- are fully auditable
- have no automatic effect

The engine MUST NOT:

- auto-accept based on votes
- infer consensus
- close sessions implicitly

---

### ENG-ACCEPT-01 — Explicit Acceptance Required

A resolution becomes legitimate only if:

- an explicit acceptance action occurs
- within a session
- under declared authority
- within declared scope

If no acceptance occurs:

- no decision exists

---

### ENG-ACCEPT-02 — Authority Gate at Acceptance Time

At acceptance:

- authority MUST be evaluated
- using current stances
- using the frozen participant set

If authority fails:

- acceptance MUST be rejected
- session remains open
- engine state MUST NOT mutate

---

### ENG-ACCEPT-03 — Single Commitment Moment

For a given proposal in a session:

- acceptance may occur at most once
- rejection does not create legitimacy
- re-evaluation requires a new session

There is:

- no provisional acceptance
- no conditional legitimacy
- no implicit closure

---

### ENG-ACCEPT-04 — Non-Retroactivity

Changes to:

- votes
- authority
- scope

after acceptance MUST NOT reinterpret legitimacy.

---

## VIII. Concurrency & Isolation

### ENG-CONCUR-01 — Concurrent Sessions Are Isolated

Sessions may run concurrently in an Area.

Interference occurs only if a resolution:

- changes Authority
- changes Scope
- supersedes an active resolution

Affected sessions MUST be:

- revalidated
- paused
- or blocked

Fail if:

- interference is silent or implicit

---

### ENG-CONCUR-02 — Legitimacy Cannot Be Forked

The engine MUST prevent:

- finalizing decisions outside original context
- forking in-progress legitimacy

Fail if:

- sessions can be completed elsewhere

---

## IX. Import, Export, and Review

### ENG-EXP-01 — Only Closed Sessions Export Legitimacy

Exports may include only:

- CLOSED sessions
- their resulting resolutions

Fail if:

- active or paused sessions affect legitimacy

---

### ENG-IMP-01 — Consolidation Preserves Legitimacy Only

In CONSOLIDATE mode:

- imported resolutions are historical artifacts
- imported deliberation is non-authoritative

Fail if:

- imported votes affect acceptance

---

### ENG-IMP-02 — Imported Session History Is Read-Only

Imported session history:

- is immutable
- never governs legitimacy

Fail if:

- imported history influences decisions

---

### ENG-REV-01 — Import Review Is Mechanical

Import review must satisfy:

- chronological review
- local authority governs review
- no cascading rejection
- context preserved for audit only

Fail if:

- review interprets intent

---

## X. References & Semantics

### ENG-REF-01 — References Are Informational Only

References:

- grant no authority
- imply no approval
- create no precedence
- trigger no enforcement

Fail if:

- references affect legitimacy

---

### ENG-REF-02 — No Semantic Inference

The engine MUST NOT infer:

- authority overlap
- scope overlap
- role equivalence
- intent

Fail if:

- meaning is inferred from structure or content

---

## XI. Audit & Integrity

### ENG-AUD-01 — Audit Is Authoritative

Audit is the system of record for:

- what happened
- when it happened
- under what authority
- with which participants

Fail if:

- outcomes cannot be reconstructed via audit

---

### ENG-AUD-02 — Audit Outlives Subjects

Audit records MUST outlive the entities they describe.

Fail if:

- audit disappears with deletion

---

### ENG-AUD-03 — Export Integrity Is Verifiable

Exports MUST allow detection of:

- structural tampering
- content modification

Fail if:

- tampering cannot be detected

---

## XII. Storage & Persistence

### ENG-STOR-01 — Storage Isolation

Each engine instance operates over a single explicit storage root.

Fail if:

- cross-root visibility exists

---

### ENG-STOR-02 — Engine Is Storage-Agnostic

The engine treats storage as opaque.

Fail if:

- filesystem layout affects behavior

---

### ENG-STOR-03 — Stable Object Identity

Object identity MUST survive:

- restarts
- export/import
- storage relocation

Fail if:

- identity depends on paths

---

### ENG-STOR-04 — No Implicit History Deletion

History may only be:

- superseded
- retired
- abandoned

Always with audit.

Fail if:

- history disappears silently

---

### ENG-STOR-05 — Export Is a Complete Snapshot

Exports MUST be:

- complete
- referentially intact
- deterministically rehydratable

Fail if:

- exports are partial or ambiguous

---

## XIII. AI Boundary

### ENG-AI-01 — AI Is Outside the Engine Boundary

AI systems may:

- suggest
- annotate
- warn

AI systems MUST NOT:

- accept decisions
- modify resolutions
- override authority
- bypass constraints

Fail if:

- AI affects legitimacy

---

## XIV. Explicit Non-Goals

Charter Core does NOT provide:

- chat systems
- workflow orchestration
- task execution
- role management
- identity systems
- semantic reasoning
- inferred conflict resolution
- UX patterns (rounds, moderation, facilitation)

These belong to higher layers.

---

## Lock Statement

These invariants are frozen.

Charter Core correctness is defined by adherence to them.