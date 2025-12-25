# Charter Core — Formal Engine Invariants & Boundaries 

> Status: FINALIZING 
> Changes to these invariants require explicit justification and new simulations demonstrating preserved legitimacy.

---

## Core Principle

Charter Core is a **legitimacy engine**, not a reasoning engine, workflow engine, or collaboration tool.

Its sole responsibility is to ensure that decisions are:
- Explicit
- Auditable
- Governed
- Non-retroactive

---

## Engine Invariants (Frozen)

---

### 1. Explicit Decisions Only

No decision is legitimate unless it is explicitly accepted within a session.
- Silence is not consent
- Metadata is not acceptance
- Automation is not authority

---

### 2. Immutable History

Once accepted, a resolution is immutable.

- Resolutions are never edited
- Corrections require superseding resolutions
- History is append-only

---

### 3. Areas Are Hard Governance Boundaries

Every resolution belongs to exactly one Area.

- Areas do not implicitly overlap
- Areas do not inherit authority or scope
- Cross-area relevance must be explicitly referenced

---

### 4. Authority Is a First-Class Resolution

Each Area must maintain **exactly one Active Authority resolution** at any given time.

Authority:
- Defines who has standing in a session
- Defines the deterministic decision rule for the agreement
- Is purely mechanical

Authority does **not**:
- Interpret content
- Judge correctness
- Assign roles
- Apply semantic meaning

Authority changes:
- Require a decision session
- Never rewrite history

---

### 5. Scope Is a First-Class, Descriptive Resolution

Each Area must have exactly one active Scope resolution at any time.

Scope:
- Describes what kinds of decisions belong in the Area
- Is descriptive, not enforcing
- Exists to inform humans and block obvious misuse

Scope changes:
- Require a decision session
- Never invalidate prior resolutions

---

### 6. Context Preservation (Authority & Scope)

Every session and every accepted resolution must permanently record:
- The active Authority resolution at the time of acceptance
- The active Scope resolution at the time of acceptance
- Any additional Scopes explicitly referenced during the session

Later changes to Authority or Scope must never invalidate, reinterpret, suppress, or alter previously accepted resolutions.
Relevance, applicability, or correction may only be expressed through new resolutions.

---

### 7. Sessions Are the Unit of Legitimacy

Resolutions may only be accepted within sessions.

Sessions:
- Define participants
- Enforce authority rules
- Enforce session constraints
- Produce zero or more resolutions

---

### 8. Candidates Are Neutral

Candidates are options under consideration.

They:
- Imply no intent
- Imply no endorsement
- May be abandoned without consequence

Candidates have no lifecycle outside a session and no legitimacy unless accepted.

Rejection and removal are non-semantic and must not affect engine state.

Only accepted candidates become resolutions.

---

### 9. Explicit Resolution Lifecycle

Resolutions transition only through explicit states:
- Active
- Under Review
- Superseded
- Retired

Rules:
- No resolution is ever removed
- A resolution under review may not be accepted
- All transitions are auditable
- Resolution legitimacy states (Active, Superseded, Retired) may only change through the acceptance of a decision within a session.
- Non-legitimacy workflow states (e.g. Under Review) may be modified directly but must not affect authority, scope, or decision validity.

---

### 10. Session Constraints Are Engine-Enforced

Sessions may declare explicit constraints at creation time.

Constraints:
- Apply only to the current session
- Are enforced mechanically by the engine
- Must be satisfied before acceptance
- Do not modify Authority or Scope

Constraints prevent premature or illegitimate acceptance.

---

### 11. Session Blocking and Pausing Are Explicit

If a session cannot satisfy its Authority rule or constraints, it must:
- Enter a blocked or paused state

When resuming:
- Active Authority is revalidated
- Active Scope is revalidated

If context has materially changed, explicit handling is required.

---

### 12. Rationale Is Optional but Preservable

Charter Core must never require rationale to legitimize a decision.

However:
- Any rationale provided must be preserved
- The audit trail (sessions, candidates, supersession) explains why decisions evolved

Legitimacy comes from process, not prose.

---

### 13. No Semantic Inference

Charter Core must never infer:
- Authority overlap
- Scope overlap
- Role equivalence
- Intent

All meaning is explicit or human-interpreted.

---

### 14. AI Is Outside the Engine Boundary

Charter Core must be fully functional without AI.

If integrated:
- AI may suggest
- AI may annotate
- AI may warn

AI may never:
- Accept decisions
- Modify resolutions
- Override authority
- Bypass constraints

---

### 15. Legitimacy Is Evaluated at Acceptance Time

A resolution’s legitimacy is determined solely by:
- The Authority active at acceptance
- The Scope active at acceptance
- The decision rule satisfied at acceptance

No future state change may retroactively affect legitimacy.

---

### 16. Relevance Is Human, Not Mechanical

Charter Core does not determine whether a resolution is still relevant.

Relevance is expressed only through:
- Supersession
- Retirement
- Clarifying resolutions

Charter Core must not auto-retire or suppress resolutions based on context drift.

---

### 17. Area Initialization Requirement

An Area must have exactly one active Authority resolution and exactly one active Scope resolution before any other resolution may be accepted within that Area.

Until both are present:
- The Area is considered uninitialized
- Only sessions whose sole purpose is to establish Authority and/or Scope are permitted
- All other sessions must be blocked

---

### 18. Verifiable Export Integrity

Charter Core must provide a deterministic mechanism to verify the integrity of exported data.

- Exports must include sufficient information to detect:
    - Structural tampering
    - Content modification outside Charter Core
- On import, Charter Core must never silently accept altered data as legitimate history.
- If integrity verification fails, imported content must:
    - Be rejected or
    - Enter an explicit Under Review state requiring human confirmation

This invariant applies only at import/export boundaries.

It does **not** affect:
- Authority semantics
- Scope semantics
- Session mechanics
- Resolution legitimacy rules

Charter Core remains a legitimacy engine, not a cryptographic trust system.

#### Rationale (Why This Exists)

- Charter already guarantees logical integrity internally.
- Import/export introduces an external trust boundary.
- Silent tampering would undermine auditability and legitimacy.
- Detection is required; prevention is not.

This mirrors established systems (Git, package managers, SBOMs):
- History remains immutable inside
- External artifacts are verified on entry

---

## 19. Concurrency Invariant

Charter Core allows multiple concurrent sessions within the same Area.

Sessions are isolated while active and do not interfere with one another.

Interference may occur only after the acceptance of a resolution that:
- Changes Authority
- Changes Scope
- Supersedes an active Resolution

When such a change occurs, any affected active sessions must be:
- Re-validated, or
- Explicitly paused or blocked

No session may continue under invalidated governing context.

---

## 20. Explicit Dissent Invariant

Charter Core must support explicit expression of disagreement within a session.

Silence or absence must never be interpreted as consent or rejection.

---
## Frozen Boundary (Non-Goals)

Charter Core explicitly does not provide:
- Chat systems
- Workflow orchestration
- Task execution
- Role management
- Identity systems
- Semantic reasoning
- Conflict resolution by inference
- UX patterns (rounds, turns, moderation)

These belong to higher layers.

