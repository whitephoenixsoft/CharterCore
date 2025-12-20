# Charter Core — Formal Invariants

## 1. Explicit Decisions Only
All decisions must be explicitly accepted within a session.
No implicit or inferred decisions are permitted.

## 2. Immutable History
Once accepted, resolutions must never be modified.
Changes occur only through new resolutions.

## 3. Areas Define Governance Boundaries
All decisions occur within an Area.
An Area defines a bounded governance context.

## 4. Authority as a First-Class Resolution
Each Area must maintain exactly one active Authority resolution.
Authority changes require a decision session.
Previous Authority resolutions remain immutable.

## 5. Scope as a First-Class Resolution
Each Area must maintain exactly one active Scope resolution.
Scope changes require a decision session.
Previous Scope resolutions remain immutable.

## 6. Authority and Scope Context Preservation
Every session and resolution must record the Authority and Scope
resolutions active at the time of acceptance.

Later changes to Authority or Scope must not retroactively
invalidate past decisions.

## 7. Sessions Are the Unit of Legitimacy
Decisions may only be accepted within a session.
Sessions define participants, objections, and outcomes.

## 8. Candidates Are Neutral
Candidates represent options under consideration.
They imply no intent or endorsement.

## 9. Resolution Lifecycle Is Explicit
Resolutions may be:
- Active
- Under Review
- Superseded
- Retired

No resolution is ever deleted.

## 10. No Generic Policy or Rule Streams
Charter Core does not support generic resolution streams.
Only Authority and Scope receive special semantic treatment.

## 11. AI Is Optional and Non-Authoritative
Charter Core must remain fully functional without AI.
AI assistance must not create, modify, or legitimize decisions.