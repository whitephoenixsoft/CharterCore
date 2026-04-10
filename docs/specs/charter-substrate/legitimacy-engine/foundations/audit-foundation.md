# Charter Core — Audit Foundation
Status: FROZEN  
Scope: Engine & CLI interaction layer  
Applies to: Sessions, Resolutions, Reconciliation Review, Candidate Observations  
Does NOT define: AI guidance, advanced workflows, Deliberate, Breakouts, or Synthesis

---

## Purpose

Audits exist to **reflect the state of the system faithfully** and support **human understanding and learning**.

They provide:

- Traceable history of session and resolution activity  
- Visibility into considered but unaccepted candidates  
- A safe feedback loop for users to review, reflect, and learn  
- Deterministic evidence of what occurred, separate from interpretation or opinion  

Audits are **mirrors, not arbiters**: they do not create legitimacy, authority, or enforce decisions.

---

## Scope

Audits cover:

- Sessions (start, pause, resume, close)  
- Resolution lifecycle (proposed, accepted, superseded, retired)  
- Reconciliation review activity  
- Candidate participation and outcomes as observed during sessions (including candidates that were not accepted)  
- Recorded outcomes related to authority and scope evaluation  
- Metadata for authorship, timestamps, and rationale  

Audits do **not**:

- Infer intent beyond recorded actions  
- Evaluate correctness of decisions  
- Predict outcomes or enforce recommendations  
- Act as a source of structural or legitimacy truth  

---

## Core Invariants

1. **Append-Only**  
   - No audit entry may be removed or altered after emission  

2. **Immutable**  
   - Historical audit data cannot be mutated, even if errors exist  

3. **Non-Authoritative**  
   - Audit data must not be used to determine structural validity or legitimacy  
   - Canonical truth is derived from domain objects and receipts  

4. **Non-Semantic**  
   - Audit data must not influence structural validation  
   - Audit data must not influence decision evaluation  
   - Audit data must not influence acceptance eligibility  

5. **Reconstructible (Observational)**  
   - Audit enables reconstruction of *observed history*  
   - It must not be used to reconstruct canonical engine state  

6. **Candidate Visibility**  
   - All observed candidates, accepted or not, are recorded as part of session history  

7. **Deterministic**  
   - Given identical inputs and ordering guarantees, audit output must be deterministic  

8. **Separation from Legitimacy**  
   - Audit reflects events only; it does not grant or remove authority, scope, or acceptance  

---

## Human-Centric Philosophy

Audits are designed to **support the human mind**:

- **Reflection** — Users can see what they did, what was proposed, and what was abandoned  
- **Learning** — Mistakes and abandoned candidates are visible, reinforcing safe experimentation  
- **Clarity** — Visibility into past actions builds trust and reduces cognitive load  
- **Confidence** — Users understand consequences without fear of hidden behavior  
- **Psychological Safety** — Uncommitted ideas are preserved without judgment  

> Mental Model: “Audit is my mirror, not my judge.”

---

## Candidate Perspective

- Accepted candidates appear as fully committed, legitimate resolutions  
- Unaccepted or abandoned candidates remain visible as part of session history  
- Users distinguish clearly between:  
  - **Considered but not committed**  
  - **Committed and legitimate**  

This mirrors real-world decision-making: exploration is safe; commitment is explicit.

---

## Lifecycle

1. **Event Emission**  
   - Engine operations emit audit events alongside execution  

2. **Aggregation**  
   - Events are collected, timestamped, and indexed  

3. **Visibility**  
   - CLI and API may query audits to inspect history and outcomes  

4. **Review**  
   - Users inspect sessions, decisions, and reconciliation activity  

5. **Persistence**  
   - Audit data is append-only and stored alongside other system artifacts  

---

## Implementation Notes

- Audit events are **observational records only**  
- No derivation or interpretation occurs within the audit layer  
- Audit entries may include:
  - Event type (session lifecycle, resolution lifecycle)  
  - Author(s)  
  - Timestamp  
  - Candidate / Resolution ID  
  - Scope / Area context  
  - Optional rationale / annotation  

- Audit read operations must never mutate state  

- Audit consumers may surface warnings when expected events are absent, but absence of audit data must not be treated as a structural or legitimacy error  

---

## Psychological Benefits Summary

Audits aim to:

- Make past actions visible  
- Support human learning and reflection  
- Preserve candidate exploration alongside committed outcomes  
- Reduce fear and shame in decision-making  
- Encourage confidence and clarity through transparency  

---

## Summary

Audit exists to **reflect what happened**, not to define what is true.

- Structural truth is defined by domain objects  
- Legitimacy truth is defined by receipts  
- Audit provides observational history for humans  

Reconciliation review helps humans think.  
Sessions help humans commit.  
Audit helps humans understand.

Nothing else.