# Charter Core — Audit Foundation (V1)
Status: FROZEN  
Scope: V1 — Engine & CLI interaction layer  
Applies to: Sessions, Resolutions, Baseline Review, Candidate Outcomes  
Does NOT define: AI guidance, V2/V3 workflows, Deliberate, Breakouts, or Synthesis

---

## Purpose

Audits exist to **reflect the state of the system faithfully** and support **human understanding and learning**.  

They provide:

- Traceable history of all resolutions and session activity
- Visibility into considered but unaccepted candidates
- A safe feedback loop for users to review, reflect, and learn
- Deterministic evidence of legitimacy, separate from interpretation or opinion

Audits are **mirrors, not arbiters**: they do not create legitimacy, authority, or enforce decisions.

---

## Scope

V1 audits cover:

- Sessions (start, pause, resume, close)
- Resolution lifecycle (proposed, accepted, superseded, abandoned)
- Baseline review acceptance and rejection
- Candidate outcomes, including those never accepted
- Authority and scope evaluation logs
- Metadata for authorship, timestamps, and rationale

Audits do **not**:

- Infer intent beyond recorded actions
- Evaluate correctness of decisions
- Predict outcomes or enforce recommendations

---

## Core Invariants

1. **Append-Only**
   - No audit entry may be removed or altered after emission.
2. **Immutable**
   - Historical audit data cannot be mutated, even if errors exist.
3. **Reconstructible**
   - Engine state can be fully rebuilt from audit + object + ref stores.
4. **Candidate Visibility**
   - All considered candidates, accepted or abandoned, are recorded.
5. **Deterministic**
   - Rebuilding the audit timeline yields the same sequence of events every time.
6. **Separation from Legitimacy**
   - Audit only reflects events; it does not grant or remove authority, scope, or acceptance.

---

## Human-Centric Philosophy

Audits are designed to **support the human mind**:

- **Reflection**: Users can see what they did, what was proposed, and what was abandoned.
- **Learning**: Mistakes and abandoned candidates are visible, reinforcing safe experimentation.
- **Clarity**: Visibility into past actions builds trust and reduces cognitive load.
- **Confidence**: Users understand the consequences of decisions without fear of hidden actions.
- **Psychological Safety**: Uncommitted ideas are preserved without judgment.

> Mental Model: “Audit is my mirror, not my judge.”

---

## Candidate Perspective

- Accepted candidates appear as fully committed, legitimate resolutions.
- Unaccepted or abandoned candidates are marked and visible.
- Users learn to distinguish **“considered but not committed”** from **“committed and legitimate.”**
- This mirrors real-world decision-making: exploring ideas is safe, committing requires ownership.

---

## Lifecycle

1. **Event Recording**
   - Each session, resolution, and baseline review emits audit events.
2. **Aggregation**
   - Audits are collected, timestamped, and indexed for retrieval.
3. **Visibility**
   - CLI and API can query audits to reconstruct history, candidate outcomes, or session activity.
4. **Review**
   - Users can audit their own actions, the area state, or imported baselines.
5. **Persistence**
   - Audit data is append-only and stored alongside objects, refs, and metadata.

---

## Implementation Notes (V1)

- **Audit events are primary sources for historical reconstruction.**
- **No derivation or interpretation occurs in V1.**
- Audit entries include:
  - Event type (session start/close, resolution propose/accept/reject)
  - Author(s)
  - Timestamp
  - Candidate / Resolution ID
  - Scope / Area context
  - Optional rationale / annotation
- **Audit read operations must never mutate state.**
- Errors should surface when expected events are missing; empty results are valid but flagged if context is incomplete.

---

## Psychological Benefits Summary

Audits in V1 aim to:

- Make past actions visible
- Support human learning and reflection
- Preserve the integrity of candidates, accepted or abandoned
- Reduce fear and shame in decision-making
- Encourage confidence and clarity through transparency