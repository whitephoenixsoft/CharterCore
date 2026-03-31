# Charter Core — Engine Invariants

Status: REFACTORED (V5 – Determinism Closure & Spec Alignment)  
Applies to: Charter Core Engine  
Role: Constitutional invariants  
Violations indicate an engine correctness failure

---

# 1. Purpose

These invariants define the non-negotiable mechanical behavior of the Engine.

They preserve:

- legitimacy  
- determinism  
- auditability  
- non-retroactivity  

These invariants are constitutional.

They do not replace detailed specifications.  
They define the truths those specifications must uphold.

---

# 2. Global Determinism (Foundational)

## ENG-DET-00 — Absolute Determinism

Given identical:

- domain objects  
- structural graph  
- receipts  
- command inputs  
- runtime mode  
- spec_set_hash  
- schema versions  

the Engine must produce identical:

- evaluation outcomes  
- EvaluationReports  
- structural mutations  
- receipts  
- canonical hashes  
- runtime mode decisions (normal / degraded / halt)  

---

## ENG-DET-01 — No Hidden Inputs

Engine behavior must not depend on:

- timestamps (non-semantic only)  
- storage order  
- environment  
- audit logs  
- execution timing  
- implicit defaults  
- inferred data  

All behavior must derive from explicit structural inputs.

---

## ENG-DET-02 — No Implementation Discretion

The Engine must not include:

- heuristic decisions  
- policy-based classification  
- implementation-defined outcome differences  

All outcomes must be rule-defined and reproducible.

---

# 3. Core Boundary

## ENG-CORE-01 — Engine Is a Legitimacy Compiler

The Engine:

- enforces legitimacy  
- evaluates explicit rules  
- preserves structural history  
- produces deterministic outcomes  

The Engine does not:

- interpret semantics  
- infer intent  
- provide workflow logic  
- reason about content  

---

# 4. Identity & Immutability

## ENG-ID-01 — Canonical Identity

Engine-owned structural entities must have stable identifiers.

Identity must survive:

- restart  
- export/import  
- serialization  

Identifiers must not carry semantic meaning.

---

## ENG-ID-02 — Accepted Artifacts Are Immutable

Once accepted:

- Resolution content is immutable  
- acceptance context is immutable  
- legitimacy is immutable  

Evolution occurs only through:

- supersession (structure)  
- usability transitions (forward-only)

---

# 5. Legitimacy Model

## ENG-LEG-01 — Explicit Legitimacy Only

Legitimacy arises only from explicit session acceptance.

---

## ENG-LEG-02 — Sessions Are the Sole Legitimacy Mechanism

Only sessions create legitimacy.

---

## ENG-LEG-03 — Acceptance-Time Truth

Legitimacy is determined solely by:

- accepted session state  
- authority and scope at acceptance  
- votes and constraints at acceptance  

Later changes must not affect past legitimacy.

---

## ENG-LEG-04 — Evaluation Is Deterministic

Evaluation must be:

- deterministic  
- complete  
- side-effect free  

---

## ENG-LEG-05 — Explicit Participation Only

Only explicit recorded votes have meaning.

No inference from absence.

---

# 6. Structural History

## ENG-HIST-01 — Append-Only History

History is append-only.

No in-place mutation of accepted artifacts.

---

## ENG-HIST-02 — Explicit Lifecycle States

- lifecycle states are explicit and finite
- in the current specification set, valid Resolution lifecycle states are:
  - ACTIVE
  - ON_HOLD
  - SUPERSEDED
  - RETIRED

---

## ENG-HIST-03 — Usability Does Not Alter Structure

ON_HOLD and RETIRED:

- do not change structural graph  
- do not change ACTIVE derivation  
- do not rewrite legitimacy  

---

## ENG-HIST-04 — Supersession Is One-Way

Supersession is:

- directional  
- irreversible  
- structural  

---

# 7. Structure vs Usability Separation

## ENG-SEP-01 — Structural Truth vs Usability

- STRUCTURE defines graph truth and ACTIVE  
- USABILITY defines forward availability  

They must remain strictly separate.

---

# 8. Areas and Governance

## ENG-AREA-01 — Area Locality

Legitimacy is strictly Area-local.

No cross-area dependency for legitimacy.

---

## ENG-AREA-02 — Governed Runtime Requirement

A valid runtime must have:

- exactly one ACTIVE Authority  
- exactly one ACTIVE Scope  

No implicit governance.

---

## ENG-AREA-03 — Governance Is Explicit

Authority and Scope:

- must be explicit  
- must not be inferred  
- must not disappear implicitly  

---

## ENG-CONTEXT-01 — Context Is Permanently Bound

Accepted legitimacy permanently binds:

- Authority  
- Scope  

---

# 9. Sessions & Candidates

## ENG-SES-01 — Candidates Are Non-Legitimate

Candidates are proposals only.

---

## ENG-SES-02 — Candidate State Is Derived

Candidate status is not stored.

It is derived deterministically during evaluation.

---

## ENG-SES-03 — Deterministic Rounds

Resume creates a new deterministic round.

No prior vote state is reused.

---

## ENG-CON-01 — Constraints Are Explicit

Constraints must be:

- explicit  
- deterministic  
- mechanically enforced  

---

# 10. Voting & Acceptance

## ENG-VOTE-01 — Votes Are Inputs Only

Votes contribute to evaluation only.

---

## ENG-ACCEPT-01 — Explicit Acceptance Required

Only acceptance creates legitimacy.

---

## ENG-ACCEPT-02 — Atomic Acceptance

Acceptance must be atomic with:

- Resolution  
- receipt  
- structural mutation  

---

## ENG-ACCEPT-03 — Acceptance Freezes Truth

Accepted state must remain:

- reconstructable  
- verifiable  
- immutable  

---

## ENG-ACCEPT-04 — Non-Retroactivity

No later change may alter past legitimacy.

---

# 11. Concurrency

## ENG-CONCUR-01 — Deterministic Conflict Resolution

Concurrent sessions must resolve conflicts deterministically.

No heuristic resolution permitted.

---

# 12. Persistence & Artifacts

## ENG-PERSIST-01 — Atomic Legitimacy

Legitimacy artifacts must persist atomically.

---

## ENG-EXP-01 — Export Preserves Truth

Export must preserve legitimacy artifacts completely.

---

## ENG-IMP-01 — Import Does Not Create Legitimacy

Import introduces artifacts only.

---

## ENG-COMP-01 — Compilation Is Reconstruction Only

Compilation:

- reconstructs history  
- does not re-evaluate legitimacy  
- does not reinterpret rules  

---

# 13. Supersession & Usability Effects

## ENG-SUP-01 — Supersession Is Structural

Supersession is a structural mechanism of forward evolution.

---

## ENG-SUP-02 — Usability Affects Forward Only

ON_HOLD / RETIRED:

- affect forward usage  
- do not affect history  

---

## ENG-SUP-03 — Blocking Must Be Explicit

If forward usage is invalid:

- blocking must be explicit  
- must be deterministic  

---

# 14. Audit

## ENG-AUD-01 — Audit Is Observational

Audit:

- records events  
- does not affect legitimacy  
- does not affect evaluation  

---

# 15. Determinism & Locality

## ENG-DET-03 — Local Structural Legitimacy

Legitimacy must be computable from:

- local structural artifacts  
- explicit rules  

---

## ENG-DET-04 — No Implicit Inference

The Engine must not infer:

- missing data  
- missing legitimacy  
- missing intent  

If legitimacy cannot be proven → it does not exist.

---

# 16. Non-Goals

## ENG-NONGOAL-01 — No Semantic or Workflow Logic

The Engine does not provide:

- reasoning  
- workflow  
- interpretation  
- policy  

---

# 17. Lock Statement

These invariants are constitutional.

All specifications must conform to them.

Any violation constitutes an Engine correctness failure.