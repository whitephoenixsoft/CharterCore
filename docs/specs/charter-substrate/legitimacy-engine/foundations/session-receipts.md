# Charter Core — Receipt Foundation  
## Immutable Record of Legitimacy

Status: FOUNDATIONAL  
Layer: Conceptual / Engine-Adjacent  
Applies to: Engine Core, CLI, audit consumers, and long-term verification  
Does NOT define: canonical encoding, hashing algorithms, or persistence mechanics  

---

## 1. Purpose

Receipts are the immutable artifacts that record the **final outcome of a session**.

They exist to:

- preserve legitimacy as it was created  
- bind outcomes to their governing rules  
- enable deterministic reconstruction of history  
- provide portable, verifiable evidence of decisions  
- separate historical truth from current state  

Receipts do not create legitimacy.  
They record that legitimacy was created.

---

## 2. Core Principle

> A receipt is the permanent, unchangeable record of what happened at the moment a session ended.

A receipt answers:

- What was decided?  
- Under which rules?  
- In what context?  
- With which participants and inputs?  

It must answer these questions **without requiring external interpretation**.

---

## 3. Mental Model

A receipt is a **sealed snapshot of a decision boundary**.

- Sessions are where decisions happen  
- Receipts are what remains after the decision is sealed  

If sessions are conversations,  
receipts are signed records.

---

## 4. What a Receipt Represents

A receipt represents:

- the terminal state of a session  
- the full context required to understand that outcome  
- the exact inputs that produced the result  
- the rule system under which the decision was made  

It is not a summary.  
It is a **complete and faithful record**.

---

## 5. Receipt Types (Conceptual)

Receipts reflect how a session ended.

### LEGITIMACY

- A decision was accepted  
- Legitimacy was created  
- A resolution (or equivalent outcome) was produced  

### EXPLORATION

- No decision was accepted  
- No legitimacy was created  
- The session history is preserved for audit and learning  

Both are equally important:

- LEGITIMACY records commitment  
- EXPLORATION records consideration  

---

## 6. Immutability

Receipts are **permanently immutable**.

Once created:

- no field may be changed  
- no context may be added  
- no history may be rewritten  

If a mistake is discovered:

- a new session must create a new outcome  
- the original receipt remains as historical truth  

Receipts preserve what happened — not what should have happened.

---

## 7. Deterministic Completeness

A receipt must contain enough information to:

- reconstruct the session outcome  
- verify rule compliance  
- validate structural correctness  
- confirm legitimacy independently  

Given a receipt and the corresponding specification identity:

- the outcome must be fully explainable  
- no hidden state may be required  

Receipts must not depend on:

- external databases  
- implicit context  
- runtime assumptions  

---

## 8. Relationship to Specification Identity

Every receipt must be bound to a **specific rule set**.

This ensures:

- decisions are interpreted under the correct rules  
- historical outcomes remain stable across versions  
- verification is deterministic  

A receipt is not just:

> “what happened”

It is:

> “what happened under these exact rules”

---

## 9. Relationship to Sessions

Receipts are produced **only when a session becomes terminal**.

- Sessions create legitimacy  
- Receipts record it  

A session without a receipt is incomplete.  
A receipt without a session is invalid.

Receipts:

- do not influence session behavior  
- do not participate in decision-making  
- exist only after the decision boundary is crossed  

---

## 10. Relationship to State

Receipts are **historical artifacts**, not active state.

They must not be used to:

- derive current authority  
- determine active scope  
- compute structural relationships  
- infer usability  

Those concerns belong to:

- structural logic  
- usability rules  
- current runtime state  

Receipts describe the past.  
They do not define the present.

---

## 11. Historical Stability

Once emitted, a receipt must remain valid regardless of future changes.

This includes changes such as:

- governance evolution  
- supersession of resolutions  
- retirement of artifacts  
- usability transitions  

These changes affect **future behavior**, not past truth.

A receipt must always answer:

> “What was true at the time?”

---

## 12. No Interpretation Layer

Receipts must not:

- interpret intent  
- summarize reasoning  
- infer meaning  
- apply semantic judgment  

They record:

- structure  
- inputs  
- outcomes  

Human interpretation belongs to:

- annotations  
- interfaces  
- guidance layers  

---

## 13. Portability

Receipts must be portable across systems.

They must:

- stand alone as verifiable artifacts  
- be transferable without loss of meaning  
- remain valid outside their original runtime  

A receipt should remain understandable and verifiable:

- across environments  
- across implementations  
- across time  

---

## 14. Failure and Integrity

Invalid receipts must be detectable.

The system must be able to determine:

- whether a receipt has been tampered with  
- whether it matches its declared rule set  
- whether it is structurally complete  

If a receipt cannot be verified:

- it must not be trusted  
- it must not be used for reconstruction  

Integrity is mandatory, not optional.

---

## 15. Conceptual Invariants

- Every terminal session produces exactly one receipt  
- Receipts are immutable  
- Receipts are deterministic  
- Receipts are complete  
- Receipts are bound to a rule set  
- Receipts do not create legitimacy  
- Receipts do not influence current state  
- Receipts preserve full historical truth  

---

## 16. Summary

Receipts are the **permanent memory of Charter**.

They:

- record decisions exactly as they occurred  
- bind outcomes to their governing rules  
- preserve history without reinterpretation  
- enable long-term verification and trust  

Without receipts, decisions are ephemeral.

With receipts, decisions become **institutional memory**.