# ENG-USABILITY — Forward Usability, Suspension & Deprecation Semantics

Status: REFACTORED (v6 – ON_HOLD Terminology, Structural Separation Alignment)  
Applies to: Engine Core (V1/V2+)  

Authority: Foundational authority for ON_HOLD and RETIRED usability semantics.

Subordinate references consumed from:

- ENG-DOMAIN
- ENG-STRUCTURE
- ENG-SESSION
- ENG-DECISION
- ENG-INTEGRITY
- ENG-PERSISTENCE
- ENG-RECEIPT
- ENG-ERROR

If conflict exists, ENG-INTEGRITY governs runtime halt behavior.  
If conflict exists regarding structural ACTIVE, ENG-STRUCTURE governs graph meaning.

---

# 1. Purpose

ENG-USABILITY defines the semantics of ON_HOLD (temporary suspension) and RETIRED (permanent deprecation) for governance usability.

It is the authoritative specification for:

- allowed administrative and governance state transitions affecting forward usability
- forward usability semantics for governance artifacts
- temporary suspension semantics (ON_HOLD)
- permanent deprecation semantics (RETIRED)
- session-facing consequences of unusable referenced artifacts
- legitimacy neutrality of usability state changes

ENG-USABILITY does not define:

- structural ACTIVE derivation
- structural graph semantics
- object schemas
- runtime halt conditions
- receipt structure
- session lifecycle structure
- atomic persistence boundaries

Those are defined respectively in:

- ENG-STRUCTURE
- ENG-DOMAIN
- ENG-INTEGRITY
- ENG-RECEIPT
- ENG-SESSION
- ENG-PERSISTENCE

ON_HOLD and RETIRED:

- do not create legitimacy
- do not revoke historical legitimacy
- do not alter structural graph relationships

SUPERSEDED remains the only graph-altering lifecycle outcome and is governed by ENG-STRUCTURE.

---

# 2. Conceptual Boundary

## ENG-USABILITY-01 — Structural Graph Truth vs Usability

ENG-USABILITY governs usability, not structure.

Structural ACTIVE is defined only in ENG-STRUCTURE.

ON_HOLD and RETIRED:

- do not create structural edges
- do not remove structural edges
- do not alter structural ACTIVE derivation
- do not alter historical receipts
- do not reinterpret past legitimacy

They affect only whether a structurally valid artifact may be used for forward legitimacy evaluation.

---

# 3. Governance Control Model

## ENG-USABILITY-02 — Session Governance Rule

All lifecycle changes that affect forward usability must occur through valid governance session acceptance, except where explicitly defined as administrative transitions.

The only administrative transition governed here is:

- ACTIVE ↔ ON_HOLD
- ACTIVE → RETIRED is not an administrative transition in this specification set and must occur only through valid governance acceptance

All other lifecycle changes remain governed by:

- governance sessions (ENG-SESSION)
- structural graph rules (ENG-STRUCTURE)

ENG-USABILITY does not define structural mutation.  
It defines only whether an artifact is suspended or deprecated for forward use.

---

## ENG-USABILITY-03 — Authority Restrictions

Authority must never enter:

- ON_HOLD
- RETIRED

Authority usability and participation are governed by:

- ENG-STRUCTURE
- ENG-INTEGRITY
- ENG-DECISION

ENG-USABILITY establishes that Authority is not a usability-managed artifact type.

---

## ENG-USABILITY-04 — Scope Restrictions

Scope may enter:

- ON_HOLD

Scope must not enter:

- RETIRED

Scope supersession remains structural graph behavior governed by ENG-STRUCTURE.

ON_HOLD suspends forward usability of Scope for legitimacy evaluation.

---

# 4. Supported Usability States

## ENG-USABILITY-05 — Object-Class State Support

Resolution supports:

- ACTIVE
- ON_HOLD
- RETIRED
- SUPERSEDED

Scope Resolution supports:

- ACTIVE
- ON_HOLD
- SUPERSEDED

Authority Resolution supports:

- ACTIVE
- SUPERSEDED

Object schemas and enums are defined in ENG-DOMAIN.  
ENG-USABILITY defines which states impose usability constraints.

---

# 5. Usability Rules

## ENG-USABILITY-06 — Resolution Usability

A Resolution is usable for forward legitimacy evaluation only if:

- it is structurally ACTIVE (ENG-STRUCTURE)
- it is not ON_HOLD
- it is not RETIRED

ENG-USABILITY consumes structural truth; it does not define it.

---

## ENG-USABILITY-07 — Scope Usability

A Scope Resolution is usable only if:

- it is structurally ACTIVE (ENG-STRUCTURE)
- it is not ON_HOLD

Scope does not support RETIRED.

---

## ENG-USABILITY-08 — Authority Usability

Authority must be usable according to:

- ENG-STRUCTURE
- ENG-DECISION
- ENG-INTEGRITY

ENG-USABILITY defines only that Authority cannot be suspended or retired.

---

# 6. Allowed State Transitions

## ENG-USABILITY-09 — Resolution Transitions

Allowed:

- ACTIVE ↔ ON_HOLD
- ACTIVE → RETIRED

Not defined here:

- SUPERSEDED transitions
- structural edge creation
- supersession mechanics

SUPERSEDED remains structural and terminal under ENG-STRUCTURE.

RETIRED → SUPERSEDED is not defined as a usability transition.

---

## ENG-USABILITY-10 — Scope Transitions

Allowed:

- ACTIVE ↔ ON_HOLD

SUPERSEDED remains structural and governed by ENG-STRUCTURE.

Scope does not support RETIRED.

---

# 7. Legitimacy Neutrality

## ENG-USABILITY-11 — ON_HOLD Neutrality

ON_HOLD:

- creates no legitimacy
- modifies no structural edges
- alters no structural ACTIVE derivation
- affects no historical receipt validity
- suspends forward usability only

---

## ENG-USABILITY-12 — RETIRED Neutrality

RETIRED:

- creates no legitimacy
- modifies no structural edges
- alters no structural ACTIVE derivation
- affects no historical receipt validity
- permanently removes forward usability
- is terminal and irreversible

---

# 8. Session-Facing Consequences

## ENG-USABILITY-13 — Decision Layer Consumption

ENG-USABILITY defines usability truths consumed by:

- ENG-DECISION
- ENG-SESSION

If a session references an unusable artifact:

- acceptance must fail

Session transitions in response are defined in:

- ENG-DECISION
- ENG-SESSION

---

## ENG-USABILITY-14 — Temporary vs Permanent Effects

General intent:

- ON_HOLD → reversible suspension
- RETIRED → permanent deprecation
- SUPERSEDED → structural graph consequence

Session behavior (blocking, resume, etc.) is defined externally.

---

# 9. Acceptance Guard

## ENG-USABILITY-15 — Unusable References Prevent Acceptance

Acceptance must fail if any referenced artifact is unusable:

- Resolution ON_HOLD
- Resolution RETIRED
- Scope ON_HOLD

ENG-USABILITY defines the condition only.

- evaluation → ENG-DECISION
- commit → ENG-PERSISTENCE
- lifecycle → ENG-SESSION

---

# 10. Concurrency Semantics

## ENG-USABILITY-16 — Atomic Visibility

Usability transitions must be atomically observed.

During concurrent evaluation:

- a single deterministic usability state must be used
- no partial state may be observed
- no partial mutation may occur

Atomicity is enforced by:

- ENG-PERSISTENCE
- ENG-INTEGRITY

---

# 11. Resume Boundary

## ENG-USABILITY-17 — Resume Does Not Override Usability

Resuming a session must not:

- bypass ON_HOLD
- bypass RETIRED

Usability constraints persist across rounds.

---

# 12. Receipt Interaction

## ENG-USABILITY-18 — Historical Stability

Receipts remain valid regardless of later:

- ON_HOLD
- RETIRED
- SUPERSEDED

Usability changes do not affect historical legitimacy.

---

# 13. Runtime Relationship

## ENG-USABILITY-19 — No Halt Authority

ENG-USABILITY does not determine:

- runtime halt
- degraded mode

These belong to ENG-INTEGRITY.

---

# 14. Relationship to Structure

## ENG-USABILITY-20 — Structure Is Independent

ENG-USABILITY does not define:

- graph validity
- ACTIVE derivation
- conflict resolution
- supersession correctness

Those belong to ENG-STRUCTURE.

ENG-USABILITY consumes structural outcomes.

---

### ENG-USABILITY-20A — Deterministic Usability Outcomes

Given identical:

- structural ACTIVE truth
- lifecycle state
- object class
- schema versions
- governing rule identity where applicable

ENG-USABILITY must produce identical usability outcomes.

---

# 15. Engine Invariants

- ON_HOLD never alters structural graph
- RETIRED never alters structural graph
- ON_HOLD never alters ACTIVE derivation
- RETIRED never alters ACTIVE derivation
- ON_HOLD never creates legitimacy
- RETIRED never creates legitimacy
- ON_HOLD never destroys historical legitimacy
- RETIRED never destroys historical legitimacy
- RETIRED is terminal
- usability affects forward use only
- dependent specs must consume usability truth

---

# 16. Mental Model

ENG-USABILITY defines forward usability.

It answers:

- can this artifact be used right now?
- is it temporarily suspended?
- is it permanently deprecated?

It does not answer:

- what is structurally ACTIVE
- how the graph behaves
- how acceptance works
- how persistence occurs

ENG-USABILITY is the usability layer.  
Other specifications must consume it.