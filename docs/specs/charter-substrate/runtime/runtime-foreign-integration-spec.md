# Charter Runtime — Baseline Review & Foreign Integration Specification (Revised)

Status: FOUNDATIONAL  
Applies to: Runtime Layer, CLI Orchestration, Charter Commit System (CCS), Commit Store, Discoverability & Federation  
Does NOT define: legitimacy semantics, alignment computation, transport protocols (CRS), or guidance behavior  

---

# 1. Purpose

This document defines **Baseline Review** as the primary mechanism for:

- integrating foreign or isolated artifacts into a local Area  
- evaluating external or pre-governance material safely  
- promoting deliberate outputs into legitimacy  
- preserving legitimacy boundaries while enabling reconciliation  

Baseline Review exists to ensure that:

- foreign or pre-legitimacy structure can be observed without being trusted  
- integration is explicit, auditable, and reversible (until committed)  
- legitimacy is never created outside the engine  

---

# 2. Core Principle

> Baseline Review is a legitimacy firewall.

It allows systems to:

- inspect foreign or pre-legitimacy material  
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

This includes:

- foreign artifacts (federation/import)  
- locally produced deliberate outputs (CDS)  

---

# 4. Inputs and Sources

Baseline Review may be created from:

- discovered and fetched foreign Areas  
- imported commit bundles or flat files  
- foreign baselines from federation  
- deliberate synthesis outputs (LOCKED Items)  
- prior local baselines (consolidation)  

All inputs are treated as:

- non-authoritative  
- potentially incomplete  
- subject to explicit evaluation  

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

- safe inspection of foreign or pre-legitimacy material  
- prevention of accidental integration  
- clear separation between candidate and local truth  

---

# 6. Review Items

## 6.1 Definition

Baseline Review operates on **review items**.

A review item may represent:

- a proposal  
- a foreign resolution  
- a deliberate LOCKED Item  
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
- differences between local and candidate structure may be explored  
- provisional relationships may be created  

No legitimacy is created.

---

## 7.2 Provisional References

Within review:

- references to foreign Areas, Resolutions, or Deliberate Items may be created  
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
- imported or deliberate-derived structure becomes locally anchored  

---

## 8.4 Derivation Recording (NEW)

When review items originate from deliberate outputs:

- resulting resolution commits SHOULD include a `derived_from` structural reference  
- this reference may cross Areas  
- it preserves lineage even if the original deliberate context is unavailable  

These references:

- are structural only (CCS-level)  
- do not imply authority or correctness  
- enable traceability across substrates  

---

## 8.5 Application Backlink (NEW)

When a deliberate-derived review item results in successful session output:

- the system MAY record backlinks to originating deliberate Items  
- these backlinks may be used by Runtime to perform **Deliberate Application Reconciliation**  

Baseline Review itself does not mutate deliberate state.

---

# 9. Relationship to Deliberate (CDS)

## 9.1 Deliberate Inputs

Baseline Review may consume:

- LOCKED Items from CDS  

These Items:

- remain non-legitimate  
- are treated identically to other review inputs  
- require full review and session processing  

---

## 9.2 Non-Closure Principle

Baseline Review must NOT require:

- closure of the originating deliberate instance  

Deliberate instances may:

- remain active  
- continue evolving  
- receive reconciliation updates independently  

---

## 9.3 Separation of Concerns

- Review performs legitimacy preparation  
- CDS maintains thinking context  
- Runtime performs reconciliation between them  

---

# 10. Relationship to Legitimacy

Baseline Review:

- does not evaluate authority  
- does not create legitimacy  
- does not reinterpret prior decisions  

Legitimacy is created only by:

- sessions  
- under canonical engine rules  

Baseline Review prepares inputs for legitimacy.

---

# 11. Relationship to Discoverability and Federation

## 11.1 Discoverability

Unchanged.

---

## 11.2 Federation

Unchanged, but includes:

- deliberate-derived artifacts crossing system boundaries  

Baseline Review remains the integration boundary.

---

# 12. Receipts and Audit

## 12.1 Review Receipt

Closure of a Baseline Review emits a **Review Receipt**.

The receipt:

- records all review items  
- records ACCEPTED, REJECTED, ABANDONED states  
- references resulting sessions  
- preserves provenance  
- may include references to originating deliberate items (if applicable)  

It does not:

- summarize meaning  
- reinterpret legitimacy  
- infer correctness  

---

## 12.2 Audit Guarantees

The system must preserve:

- full review lineage  
- mapping from review items to resulting sessions  
- traceability from deliberate Items → review → sessions → resolutions  

---

# 13. CLI Orchestration Policies (Non-Structural)

Unchanged.

---

# 14. Invariants

- Baseline Review must not create legitimacy  
- All inputs are non-authoritative until session completion  
- All integration must pass through sessions  
- Review must operate on isolated data  
- Provisional references must not become durable without completion  
- Review must be fully auditable  
- Closure must emit a Review Receipt  
- Deliberate instances must not be implicitly closed by review  
- Derivation links must be explicit if recorded  
- History must never be rewritten  

---

# 15. Mental Model

Baseline Review means:

> “We can see candidate structure,  
> but we must decide it again before it becomes ours.”

It allows systems to:

- safely observe external or pre-legitimacy truth  
- model relationships under uncertainty  
- reconcile differences explicitly  

without ever:

- inheriting authority  
- assuming correctness  
- or rewriting history  

---

# 16. Final Principle

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