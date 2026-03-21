# Charter Core — Engine Invariants

Status: REFACTORED (V4 – Constitutional Alignment Pass)  
Applies to: Charter Core Engine  
Role: Constitutional invariants  
Violations indicate an engine correctness failure

---

## Purpose

These invariants define the non-negotiable mechanical behavior of the Charter Core engine.

They preserve:

- legitimacy
- determinism
- auditability
- non-retroactivity

These invariants are constitutional.

They do not replace the detailed engine specifications.  
They state the truths those specifications must uphold.

Behavior not guaranteed here must be implemented outside the engine.

---

## I. Core Boundary

### ENG-CORE-01 — Charter Core Is a Legitimacy Engine

Charter Core does not:

- reason about content
- facilitate workflow
- interpret semantics
- infer intent

Its responsibility is strictly to:

- enforce explicit legitimacy
- maintain deterministic governance rules
- preserve structural history
- ensure auditability
- prevent retroactive reinterpretation

---

## II. Identity & Immutability

### ENG-ID-01 — Canonical Engine Identity

Engine-owned structural entities must have stable canonical identifiers.

Identity must survive:

- restart
- export
- import
- serialization

Fail if identity changes due to relabeling, reorganization, transport, or representation.

---

### ENG-ID-02 — Accepted Resolutions Are Immutable

Once a Resolution is accepted:

- its accepted content is immutable
- its acceptance context is immutable
- its historical legitimacy is immutable

Subsequent lifecycle evolution may occur only through explicit mechanisms defined elsewhere, including supersession and permitted forward-usability state changes.

Fail if accepted content or acceptance context is edited, rewritten, or reinterpreted.

---

## III. Legitimacy & Decision Semantics

### ENG-LEG-01 — Explicit Decisions Only

Legitimacy arises only from explicit acceptance within a session.

---

### ENG-LEG-02 — Sessions Are the Sole Unit of Legitimacy

Sessions are the sole mechanism through which legitimacy is created.

Resolutions outside sessions do not confer legitimacy.

---

### ENG-LEG-03 — Legitimacy Evaluated at Acceptance

Legitimacy is determined only by the accepted session state, including:

- Authority governing that session
- Scope governing that session
- constraints active for that session
- recorded votes at acceptance

Past legitimacy must not be recomputed from later governance changes.

---

### ENG-LEG-04 — Deterministic Evaluation

Identical structural inputs must produce identical evaluation outcomes.

The engine must not depend on:

- timing
- environment
- storage order
- audit order
- inferred intent

---

### ENG-LEG-05 — Explicit Dissent Is First-Class

Silence is not consent.

Absence is not rejection.

Only explicit recorded votes have evaluative meaning.

---

## IV. Resolution Lifecycle & History

### ENG-HIST-01 — Append-Only History

Resolution history is append-only.

Accepted history is never rewritten.

Corrections require explicit forward artifacts, such as:

- supersession
- clarifying resolutions
- permitted administrative forward-usability transitions

---

### ENG-HIST-02 — Explicit Lifecycle States

Resolution lifecycle states are explicit and finite.

Where supported by the governing schema, valid states are:

- ACTIVE
- UNDER_REVIEW
- SUPERSEDED
- RETIRED

Lifecycle transitions must be explicit, auditable, and rule-bound.

---

### ENG-HIST-03 — Administrative States Do Not Alter Structural Legitimacy

UNDER_REVIEW and RETIRED affect forward usability only.

They do not:

- revoke past legitimacy
- change structural ACTIVE derivation
- rewrite history
- invalidate historical receipts

---

### ENG-HIST-04 — Supersession Is One-Way

Supersession is directional and irreversible.

A superseded Resolution cannot regain structural ACTIVE status.

---

## V. Areas, Authority, and Scope

### ENG-AREA-01 — Areas Are Governance Boundaries

Each structural Resolution belongs to exactly one Area.

Legitimacy is Area-local.

Cross-area references may exist only as explicit non-governing references unless a governing schema states otherwise.

---

### ENG-AREA-02 — Governed Runtime Requires Authority and Scope

An Area runtime intended for ordinary legitimacy evaluation requires exactly one ACTIVE Authority and one ACTIVE Scope.

Decisions must not proceed in an ungoverned runtime context.

Bootstrap exceptions, if supported by the detailed specifications, must be:

- explicit
- deterministic
- recorded structurally
- non-implicit

This invariant does not itself define bootstrap procedure details.

---

### ENG-AREA-03 — Governance Anchors Do Not Disappear Implicitly

Authority and Scope are first-class governance anchors.

They must evolve only through explicit rule-governed mechanisms.

They must never vanish, be inferred, or be silently replaced.

---

### ENG-AUTH-01 — Authority Is Mechanical

Authority defines vote evaluation mechanically.

Authority:

- is structural
- is singular within an Area runtime
- cannot be inferred
- cannot be interpreted semantically by the engine
- never rewrites historical legitimacy

---

### ENG-SCOPE-01 — Scope Is Structural Governance Context

Scope is a first-class governance artifact.

Scope may affect whether sessions may proceed.

Scope semantics must remain explicit and auditable.

---

### ENG-CONTEXT-01 — Authority and Scope Are Historically Bound

Every accepted legitimacy artifact permanently records the governing Authority and Scope context under which it was accepted.

Historical legitimacy must remain bound to that context permanently.

---

## VI. Sessions, Candidates, and Constraints

### ENG-SES-01 — Candidates Are Neutral Until Acceptance

Candidates are proposal artifacts, not legitimacy artifacts.

Rejection, abandonment, or removal of a candidate does not create legitimacy.

---

### ENG-SES-02 — Candidate Set Freezes at the Vote Boundary

Once the session crosses from mutable pre-vote state into voting, candidate structure is frozen for that round.

---

### ENG-CON-01 — Constraints Are Engine-Enforced

Constraints are explicit, session-scoped, and mechanically enforced.

They do not redefine Authority or Scope.

They tighten acceptance conditions only through explicit engine rules.

---

### ENG-SES-03 — Resume Creates a New Deterministic Participation Round

Resume never restores prior voting state.

Resume creates a new deterministic round and requires renewed forward participation under the then-valid session rules.

Historical rounds remain historical.

---

## VII. Voting & Acceptance

### ENG-VOTE-01 — Votes Are Evaluative Only

Votes contribute to evaluation.

Votes do not themselves create legitimacy.

---

### ENG-ACCEPT-01 — Explicit Acceptance Required

Legitimacy arises only through explicit session acceptance.

No amount of votes, proposals, or inferred agreement may substitute for acceptance.

---

### ENG-ACCEPT-02 — Acceptance Is Atomic

Accepted legitimacy must be created atomically with its required structural artifacts.

There must be no partial accepted legitimacy.

---

### ENG-ACCEPT-03 — Acceptance Freezes Historical Truth

Acceptance freezes the accepted session truth that produced legitimacy.

That frozen truth must remain reconstructable and verifiable.

---

### ENG-ACCEPT-04 — Non-Retroactivity

Changes after acceptance, including changes to:

- votes
- governance usability
- scope
- authority
- later supersession relationships

must not retroactively alter past legitimacy.

---

## VIII. Concurrency & Isolation

### ENG-CONCUR-01 — Concurrent Sessions Are Isolated Until Structural Conflict

Concurrent sessions remain isolated unless explicit structural conflict arises.

Conflicts may arise through changes such as:

- governance change
- supersession of referenced artifacts
- blocking conditions

Interference must be explicit, deterministic, and rule-governed.

---

## IX. Import, Export, and Compilation

### ENG-EXP-01 — Export Preserves Structural Legitimacy History

Exports must preserve the structural artifacts required to reconstruct legitimacy history.

Unfinished runtime state must not be exported as completed legitimacy.

---

### ENG-IMP-01 — Import Does Not Create Legitimacy

Import introduces historical structural artifacts.

Import does not create legitimacy.

Imported legitimacy must already be provable from the imported artifacts themselves.

---

### ENG-COMP-01 — Compilation Reconstructs, It Does Not Re-Decide

Historical compilation reconstructs the legitimacy DAG from historical artifacts.

Compilation must be deterministic.

Compilation must not:

- recreate legitimacy
- reinterpret legitimacy under new rules silently
- choose winners heuristically
- override historical receipts

---

## X. Supersession, Review, and Governance Stability

### ENG-SUP-01 — Supersession Is Structural and One-Way

Supersession changes structural governance history forward.

It is irreversible and directional.

---

### ENG-SUP-02 — Authority Evolves Only by Explicit Structural Means

Authority cannot drift, be inferred, or be administratively reinterpreted into a new governing truth.

Authority evolution must remain explicit and structural.

---

### ENG-SUP-03 — Review State Is Usability, Not History Rewrite

Review-like states suspend forward usability only.

They must not rewrite historical legitimacy.

---

### ENG-SUP-04 — Review Blocks Forward Dependence

If a session depends on an artifact that is not forward-usable, the session must not proceed as though nothing changed.

The exact lifecycle consequence is defined elsewhere, but the interruption must be explicit and deterministic.

---

### ENG-SUP-05 — Context Revalidation Must Be Explicit

When governing context changes in a way that matters to a pending session, forward progress must require explicit revalidation.

---

### ENG-SUP-06 — Terminal Forward-Usability States Remain Terminal

Terminal lifecycle states remain terminal according to the governing lifecycle model.

The engine must not silently re-open or reinterpret them.

---

## XI. Audit Requirements

### ENG-AUD-01 — Structural and Administrative Actions Are Auditable

Administrative and structural actions that affect forward governance or state evolution must be auditable.

Audit must remain observational only.

Audit may record legitimacy-related actions, but must never create legitimacy.

---

## XII. Determinism & Locality

### ENG-DET-01 — Legitimacy Is Structural and Local

Legitimacy must be computable from:

- local Area structural artifacts
- explicit rules
- explicit references
- explicit receipts where required

It must not depend on:

- external Areas
- workflow context
- hidden metadata
- human interpretation by the engine

---

### ENG-DET-02 — Nothing Is Inferred Implicitly

The engine must not infer:

- missing governance
- missing receipts
- missing votes
- missing intent
- missing legitimacy

If legitimacy cannot be proven mechanically, the engine must not pretend otherwise.

---

## XIII. Explicit Non-Goals

### ENG-NONGOAL-01 — No Workflow or Semantic Facilitation

Charter Core does not provide:

- reasoning
- workflow management
- facilitation
- semantic interpretation
- policy authorship
- social decision support

Those belong outside the engine.

---

## Lock Statement

These invariants are constitutional.

Detailed specifications may refine behavior, but they must not violate these invariants.

Adherence defines engine correctness.