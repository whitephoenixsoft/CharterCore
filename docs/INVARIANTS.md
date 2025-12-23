# Charter Core — Formal Engine Invariants & Boundaries (Frozen)

> Status: FROZEN
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

### 6. Context Preservation Is Mandatory

Every accepted resolution must permanently record:
- The active Authority resolution at acceptance
- The active Scope resolution at acceptance
- Any additional Areas or Scopes explicitly referenced during the session

Later changes must never retroactively alter legitimacy.

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

