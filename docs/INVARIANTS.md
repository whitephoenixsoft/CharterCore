# Charter Core — Formal Engine Invariants (Revised)

## 1. Explicit Decisions Only

Charter Core recognizes a decision only when it is explicitly accepted within a session according to the active Authority rule.
No implicit, inferred, or out-of-band decisions are permitted.

---

## 2. Immutable History

All accepted resolutions are immutable.
Resolutions are never edited or deleted.

Any change in meaning, rule, or applicability must be expressed as a new resolution that supersedes or retires a previous one.

---

## 3. Areas Define Governance Boundaries

All sessions, candidates, and resolutions exist within an Area.
An Area is a bounded governance context and the highest-level container recognized by the engine.

Charter Core does not infer or enforce relationships between Areas.

---

## 4. Authority Is a First-Class Resolution

Each Area must maintain exactly one active Authority resolution at any given time.

Authority:
- Defines who has standing in a session
- Defines the deterministic decision rule used to evaluate agreement
- Is purely mechanical and non-interpretive

Authority does not:
- Evaluate candidate content
- Prioritize candidates
- Apply semantic judgment

Authority changes may occur only through a decision session.
All previous Authority resolutions remain immutable and historically accessible.

---

## 5. Scope Is a First-Class Resolution

Each Area must maintain exactly one active Scope resolution at any given time.

Scope:
- Is descriptive and informational to the engine
- Does not enforce or block decisions mechanically

Scope changes may occur only through a decision session.
All previous Scope resolutions remain immutable and historically accessible.

---

## 6. Context Preservation (Authority & Scope)

Every session and every accepted resolution must permanently record:

- The active Authority resolution of the primary Area at the time of acceptance
- The active Scope resolution of the primary Area at the time of acceptance
- Any additional Scope resolutions explicitly referenced during the session

Later changes to Authority or Scope must never retroactively affect prior sessions or resolutions.

---

## 7. Sessions Are the Sole Unit of Legitimacy

A resolution may be accepted only within a session.

Sessions define:
- Participants
- Candidates
- Positions
- Evaluation state
- Outcome

Charter Core does not support decisions made outside of sessions.

---

## 8. Candidates Are Neutral

Candidates represent proposed options under consideration.
They imply no endorsement, priority, or intent.

A candidate has no effect unless accepted through a session.

---

## 9. Deterministic Evaluation

Given the same:
- Participants
- Positions
- Authority rule
- Session state

Charter Core must always produce the same outcome.

No probabilistic, heuristic, or semantic evaluation is permitted within the engine.

---

## 10. Explicit Resolution Lifecycle

Resolutions may transition only through explicit states, including:
- Active
- Under Review
- Superseded
- Retired

State transitions must be explicit and auditable.
No resolution is ever removed from history.

---

## 11. No Generic Policy Streams

Charter Core does not support arbitrary rule streams, aliases, or version tracks.

Only Authority and Scope receive special semantic treatment by the engine.

All other resolutions are treated uniformly.

---

## 12. Transparency of Governing Context

Before a session may accept a resolution, Charter Core must expose:

The active Authority resolution

The active Scope resolution

This requirement exists to ensure that decisions are made with explicit knowledge of governing context.

---

## 13. Decision Rules Are Announced at Session Start

Every session must be associated with exactly one Authority resolution.

The Authority’s decision rule:
- Governs acceptance, rejection, blocking, and closure
- Is fixed for the duration of the session
- Must be known to all participants at session start

---

## 14. Session Blocking and Pausing Are Explicit

If a session cannot satisfy its Authority rule, it must enter a blocked or paused state.

When a paused or blocked session resumes, Charter Core must re-validate:
- Active Authority
- Active Scope

If the governing context has changed, the session may not proceed without explicit handling.

---

## 15. No Permissions or Identity Semantics

Charter Core does not implement:
- User authentication
- Permissions
- Roles
- Access control

All participant identity and permission enforcement occur outside the engine.

---

## 16. No Side Effects Beyond State

Charter Core produces only internal state changes and queryable records.

It does not:
- Trigger workflows
- Create tasks
- Call external systems
- Enforce execution

Downstream systems consume Charter Core state explicitly via APIs.

---

## 17. Area Name

Area names are human-readable identifiers only and have no semantic, authority, or scope-enforcement meaning.
