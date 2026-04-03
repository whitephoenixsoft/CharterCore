# Charter Runtime — Baseline Review & Foreign Integration Specification

Status: FOUNDATIONAL  
Applies to: Runtime Layer, CLI Orchestration, Charter Commit System (CCS), Commit Store, Discoverability & Federation  
Does NOT define: legitimacy semantics, alignment computation, transport protocols (CRS), or guidance behavior  

---

# 1. Purpose

This document defines **Baseline Review** as the primary mechanism for:

- integrating foreign or isolated artifacts into a local Area  
- evaluating external or pre-governance material safely  
- preserving legitimacy boundaries while enabling reconciliation  

Baseline Review exists to ensure that:

- foreign structure can be observed without being trusted  
- integration is explicit, auditable, and reversible (until committed)  
- legitimacy is never created outside the engine  

---

# 2. Core Principle

> Baseline Review is a legitimacy firewall.

It allows systems to:

- inspect foreign or isolated material  
- model relationships provisionally  
- prepare integration  

But it does not:

- create legitimacy  
- evaluate authority  
- mutate existing decisions  
- bypass the engine  

---

# 3. Position in the System

Baseline Review is a **specialization of the general review construct**.

All reviews share:

- isolation  
- explicit workflow rules  
- auditability  
- non-legitimizing behavior  

Baseline Review is the variant that:

> integrates foreign or pre-governance material into legitimacy-relevant local structure.

---

# 4. Inputs and Sources

Baseline Review may be created from:

- discovered and fetched foreign Areas  
- imported commit bundles or flat files  
- foreign baselines from federation  
- deliberate synthesis outputs  
- prior local baselines (consolidation)  

All inputs are treated as:

- foreign  
- partial  
- potentially stale  
- non-authoritative  

---

# 5. Isolated Review Store

## 5.1 Isolation Requirement

Baseline Review operates on a **dedicated, isolated review store**.

This store is:

- read-only with respect to source artifacts  
- non-legitimizing  
- separate from active runtime state  

---

## 5.2 Purpose of Isolation

Isolation ensures:

- safe inspection of foreign graphs  
- prevention of accidental integration  
- clear separation between foreign and local truth  

---

# 6. Review Items

## 6.1 Definition

Baseline Review operates on **review items**.

A review item may represent:

- a proposal  
- a foreign resolution  
- a synthesis artifact  
- an imported commit  
- a structural element of a foreign graph  

---

## 6.2 Properties

All review items are:

- non-legitimate  
- mutable within the review workspace  
- auditable  
- reversible until committed through a session  

---

## 6.3 States

Review items may exist in:

- UNDER_REVIEW  
- ACCEPTED (prepared for integration)  
- REJECTED (explicitly declined)  
- ABANDONED (not evaluated before closure)  

These states are **workflow states**, not legitimacy states.

---

# 7. Review Workflow

## 7.1 Evaluation Phase

During review:

- items may be inspected, grouped, annotated  
- differences between local and foreign structure may be explored  
- provisional relationships may be created  

No legitimacy is created.

---

## 7.2 Provisional References

Within review:

- references to foreign Areas or Resolutions may be created  
- these are **provisional**  
- they exist only within the review context  

> Provisional references must not escape the review boundary.

---

## 7.3 Reversibility

All actions remain reversible until:

- a session is created  
- a legitimacy receipt is emitted  

After that point, outcomes are immutable.

---

# 8. Acceptance and Integration Semantics

## 8.1 Acceptance

Accepting a review item means:

- it is selected for integration  
- it is prepared for legitimacy evaluation  

Acceptance alone does NOT:

- create legitimacy  
- activate resolutions  
- modify local state  

---

## 8.2 Integration Path

All accepted items must pass through:

→ Session creation  
→ Authority and scope evaluation  
→ Legitimacy receipt emission  

Only then do they become:

- canonical resolutions  
- durable structural elements  

---

## 8.3 Structural Integration

After successful sessions:

- provisional references may become durable  
- relationships become part of the local graph  
- imported structure becomes locally anchored  

---

# 9. Relationship to Legitimacy

Baseline Review:

- does not evaluate authority  
- does not create legitimacy  
- does not reinterpret prior decisions  

Legitimacy is created only by:

- sessions  
- under canonical engine rules  

Baseline Review prepares inputs for legitimacy.

---

# 10. Relationship to Discoverability and Federation

## 10.1 Discoverability

Baseline Review typically follows:

- discovery  
- query  
- acquisition  

It operates on:

- isolated foreign graphs  

---

## 10.2 Federation

Federation is:

- repeated or structured acquisition of foreign graphs  

Baseline Review is:

- the local integration boundary for federated data  

No federation process bypasses review.

---

# 11. Receipts and Audit

## 11.1 Review Receipt

Closure of a Baseline Review emits a **Review Receipt**.

The receipt:

- records all review items  
- records ACCEPTED, REJECTED, ABANDONED states  
- references resulting sessions  
- preserves provenance  

It does not:

- summarize meaning  
- reinterpret legitimacy  
- infer correctness  

---

## 11.2 Audit Guarantees

The system must preserve:

- full review lineage  
- mapping from review items to resulting sessions  
- complete traceability of integration decisions  

---

# 12. CLI Orchestration Policies (Non-Structural)

The following are **CLI-level policies**, not runtime invariants:

- one active baseline review per Area  
- baseline review may pause active sessions in that Area  
- baseline review may act as the primary workspace during evaluation  

These policies:

- improve usability and safety  
- do not alter core system invariants  

---

# 13. Invariants

- Baseline Review must not create legitimacy  
- Foreign artifacts remain non-authoritative until accepted via session  
- All integration must pass through sessions  
- Review must operate on isolated data  
- Provisional references must not become durable without review completion  
- Review must be fully auditable  
- Closure must emit a Review Receipt  
- History must never be rewritten  

---

# 14. Mental Model

Baseline Review means:

> “We can see foreign or proposed structure,  
> but we must decide it again before it becomes ours.”

It allows systems to:

- safely observe external truth  
- model relationships under uncertainty  
- reconcile differences explicitly  

without ever:

- inheriting authority  
- assuming correctness  
- or rewriting history  

---

# 15. Final Principle

Baseline Review creates **friction by design**.

That friction ensures:

- no silent trust  
- no implicit authority transfer  
- no accidental legitimacy  

Everything must be:

- seen  
- understood  
- and decided again  

before it becomes part of the system.