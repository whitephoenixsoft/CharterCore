# Charter Core — Resolution Foundation (V1)

Status: FROZEN (V1 Perspective)  
Scope: Conceptual foundation for resolutions, sessions, and legitimacy  
Audience: Charter contributors, future maintainers, and engineers

---

## 1. Introduction / Mental Model

A **Resolution** is the fundamental unit of commitment in Charter. It represents a fact, decision, or proposal that is auditable, traceable, and owned by an explicit actor.  

Resolutions are **immutable once created**, and their legitimacy is realized **only when accepted through sessions and baseline review**. They embody ownership and accountability and exist to maintain **coherent history**.

---

## 2. Connection to the Legitimacy Canon

Resolutions are central to Charter’s legitimacy principles:

- **Authorship:** Every resolution has a clear, explicit author.  
- **Auditability:** Resolutions are fully reconstructible from history.  
- **Truth Supremacy:** Resolutions reflect decisions, not interpretations of outcomes.  
- **Reversibility Awareness:** Each resolution declares whether it is reversible.  
- **Coherence:** Supersession preserves context and maintains a single narrative.

> Legitimacy is not about correctness; it is about traceability, accountability, and explicit ownership.

---

## 3. Sessions & Human Cognition

**Sessions** are the mechanism through which resolutions gain legitimacy:

- They contextualize decisions in time and participants.
- They prevent ambiguity about authority and scope.
- They allow humans to structure thinking before committing.
- They provide psychological safety: humans can reason, revise, or defer decisions while preserving history.

Resolutions without sessions remain **pre-legitimacy artifacts**, auditable but non-authoritative.

---

## 4. Core Resolution Fields (V1)

| Field | Description | Notes |
|-------|------------|-------|
| **Resolution ID** | Unique identifier for the resolution | Immutable |
| **Author** | Explicit creator of the resolution | Required |
| **Timestamp** | When the resolution was proposed | Required |
| **Candidate / Content** | Text or structured proposal | Immutable |
| **Authority** | Actor(s) whose acceptance is required | Evaluated via session |
| **Scope** | Defines the boundary of applicability | Evaluated via session |
| **Reversibility** | Declaration: REVERSIBLE / CONDITIONALLY_REVERSIBLE / IRREVERSIBLE | Explicit rationale required |
| **Annotations / Rationale** | Optional field for context or explanation | Recommended for human reasoning, can describe irreversibility decisions |
| **Parent / Superseded Resolutions** | References prior resolutions superseded | Immutable; preserves lineage |

---

### 4.1 Reversibility & Rationale

The **Reversibility** field is essential for human understanding:

- **REVERSIBLE:** Can be undone via standard session and supersession mechanisms.
- **CONDITIONALLY_REVERSIBLE:** Undo is possible but may require explicit approval, additional sessions, or contextual review.
- **IRREVERSIBLE:** Cannot be undone without violating historical legitimacy.  

**Annotation / Rationale Requirement:**  

Every non-reversible or conditionally reversible resolution should include a **rationale** explaining *why the decision cannot be fully reversed*. This helps:

- Future humans understand context
- Preserve historical reasoning
- Avoid confusion when revisiting long-term decisions

Example annotation for an irreversible resolution:
```
Annotation: This resolution establishes the official area naming convention. Reversal would invalidate audit history and break downstream references. Reversible only via formal supersession with explicit session approval.
```
---

## 5. Behavioral Philosophy

- Resolutions are **immutable objects**.
- Legitimacy flows **only through sessions and baseline review**.
- Supersession creates new objects; old objects remain for audit.
- Resolutions **do not infer correctness** — only ownership and traceability.
- Annotations allow humans to capture intent, context, or reasoning without affecting the engine’s legitimacy rules.

---

## 6. Examples / Mental Exercises

1. **Solo Developer:** Creates a resolution with a candidate and rationale, accepts it via session, marking it REVERSIBLE.
2. **Multi-User Session:** Participants vote; resolution becomes ACTIVE only after session closes, authority evaluated.
3. **Irreversible Resolution:** Declares a long-term governance rule. Annotation explains the rationale and implications for future contributors.

---

## 7. Relationship to Other Foundation Documents

- **Baseline Review:** Resolutions imported from external areas remain UNDER_REVIEW until formally accepted.
- **Deliberate / Breakouts:** Exploration spaces produce candidate resolutions without legitimacy.
- **Audit:** Reconstructs the history of resolutions, authority, scope, and annotations.

---

## 8. Long-Term Vision

- Future **V5 guidance/AI layers** can analyze resolutions for coherence, detect conflicts, and surface annotations to aid human understanding.  
- Annotations, rationale, and reversibility declarations create a rich dataset for guidance without ever altering legitimacy.  
- This foundation ensures that even decades later, all resolutions are auditable, traceable, and meaningful to humans and automated guidance alike.