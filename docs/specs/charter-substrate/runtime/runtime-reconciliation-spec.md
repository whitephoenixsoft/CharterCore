# Charter Runtime — Review Layer & Reconciliation Specification

Status: FOUNDATIONAL  
Applies to: Runtime Layer, CLI Orchestration, Charter Commit System (CCS), Commit Store, Discoverability & Federation  
Does NOT define: legitimacy semantics, alignment computation, transport protocols (CRS), or guidance behavior  

---

# 1. Purpose

This document defines the **Review Layer** as the set of workflows responsible for:

- integrating foreign or pre-legitimacy artifacts into legitimacy  
- promoting deliberate outputs into legitimacy  
- synchronizing deliberate state with legitimacy  
- admitting structural relationships into the graph  
- preserving legitimacy boundaries while enabling controlled evolution  

The Review Layer exists to ensure that:

- all structure entering legitimacy is explicitly evaluated  
- investigation and legitimacy remain separate but connected  
- synchronization between substrates is explicit and auditable  
- legitimacy is never created outside the engine  

---

# 2. Core Principle

> Review is a legitimacy boundary and synchronization mechanism.

The Review Layer allows systems to:

- inspect candidate material  
- model relationships provisionally  
- prepare integration into legitimacy  
- reconcile investigation with existing decisions  

But it does not:

- create legitimacy by itself  
- evaluate authority  
- mutate existing decisions directly  
- bypass the engine  

---

# 3. Review Types

The Review Layer defines two primary workflows:

---

## 3.1 Foreign Integration Review

> “Bring candidate structure into legitimacy.”

This is the evolution of Baseline Review.

It is responsible for:

- integrating foreign artifacts (federation/import)  
- promoting deliberate outputs (CDS)  
- admitting candidate structural relationships  
- consolidating prior baselines  

Direction:

→ **toward legitimacy**

---

## 3.2 Reconciliation Review

> “Align investigation with legitimacy.”

This is a **non-legitimizing synchronization workflow**.

It is responsible for:

- projecting legitimate structure into CDS  
- updating CDS Items based on legitimacy changes  
- maintaining alignment between investigation and decisions  

Direction:

↔ **bidirectional context synchronization**

Reconciliation Review does not create legitimacy.

---

# 4. Shared Review Properties

All review workflows share:

- isolation  
- explicit workflow states  
- auditability  
- non-legitimizing behavior until session execution  
- reversible operations until commitment  

---

# 5. Isolated Review Store

## 5.1 Isolation Requirement

All reviews operate on a **dedicated, isolated review store**.

This store is:

- read-only with respect to source artifacts  
- non-legitimizing  
- separate from active runtime state  

---

## 5.2 Purpose of Isolation

Isolation ensures:

- safe inspection of candidate material  
- prevention of accidental integration  
- clear separation between candidate and local truth  

---

# 6. Review Items

## 6.1 Definition

Review workflows operate on **review items**.

A review item may represent:

- a proposal  
- a foreign resolution  
- a deliberate LOCKED Item  
- a resolution-derived Item (from reconciliation)  
- an imported commit  
- a structural element of a graph  
- a candidate relationship  

---

## 6.2 Properties

All review items are:

- non-legitimate  
- mutable within the review workspace  
- auditable  
- reversible until committed through a session (if applicable)  

---

## 6.3 States

Review items may exist in:

- UNDER_REVIEW  
- ACCEPTED  
- REJECTED  
- ABANDONED  

These are **workflow states**, not legitimacy states.

---

# 7. Foreign Integration Review

## 7.1 Inputs and Sources

May be created from:

- foreign Areas (federation)  
- imported bundles  
- deliberate LOCKED Items  
- candidate relationships  
- prior baselines  

---

## 7.2 Evaluation Phase

During review:

- items may be inspected, grouped, annotated  
- candidate relationships may be explored and refined  
- provisional structure may be created  

---

## 7.3 Provisional References

- exist only within review  
- must not escape without successful session  

---

## 7.4 Acceptance and Integration

Accepted items must pass through:

→ Session creation  
→ Authority evaluation  
→ Legitimacy receipt emission  

Only then do they become:

- resolutions  
- structural relationships  

---

## 7.5 Relationship Admission

Review may accept:

- resolution ↔ resolution references  
- derivation relationships  
- cross-area relationships  
- investigation-derived structure  

---

## 7.6 Derivation Recording

Resulting resolution commits SHOULD include:

- `derived_from` references  

These:

- preserve lineage  
- may cross Areas  
- do not imply authority  

---

## 7.7 Reconciliation Linkage

Review must preserve:

- traceability from Items → sessions → resolutions  

Runtime uses this to perform reconciliation.

---

# 8. Reconciliation Review

## 8.1 Purpose

Reconciliation Review synchronizes CDS with legitimacy without creating legitimacy.

---

## 8.2 Reverse Reconciliation (Legitimacy → CDS)

Reconciliation Review may:

- select resolutions from legitimacy  
- project them into CDS as Items  

For each resolution:

- create a **resolution-derived Item**  
- assign `derived_from = resolution_id`  

---

## 8.3 Properties of Resolution-Derived Items

- remain Item nodes (not resolutions)  
- carry lineage to source resolution  
- may evolve independently within CDS  
- do not modify the source resolution  

---

## 8.4 Structural Preservation

Reconciliation may preserve:

- structural context (references, derivation, supersession)  

But:

- this context is investigative  
- it does not become legitimate structure automatically  

---

## 8.5 Investigative Evolution

Within CDS:

- resolution-derived Items may be:
  - split  
  - merged  
  - extended  
  - reorganized  

This produces:

- item → item structural relationships  

These remain:

- non-legitimate  
- confined to CDS  

---

## 8.6 Forward Reconciliation (CDS Context Update)

Reconciliation Review may also:

- evaluate existing Items against current legitimacy  
- update Item states  

Examples:

- APPLIED remains valid  
- APPLIED becomes STALE  
- unresolved Items become satisfied  
- Items become SETTLED or DISCARDED  

---

## 8.7 Non-Legitimizing Guarantee

Reconciliation Review must not:

- create resolutions  
- modify resolutions  
- create structural graph entries  

It only affects CDS state.

---

## 8.8 Reconciliation Receipts

Reconciliation Review SHOULD emit a **Reconciliation Receipt**.

This may include:

- source resolution_ids  
- created item_ids  
- updated item states  
- timestamp  
- annotations  

---

# 9. Relationship to Deliberate (CDS)

- CDS produces LOCKED Items for integration  
- CDS may consume resolution-derived Items via reconciliation  
- CDS remains active during all review processes  

Deliberate must not be:

- implicitly closed  
- implicitly modified by review  

---

# 10. Relationship to Legitimacy

- Only sessions create legitimacy  
- Review prepares inputs  
- Reconciliation aligns context  

---

# 11. Relationship to Federation

- foreign artifacts enter through Foreign Integration Review  
- reconciliation may operate after federation intake  
- no federation bypasses review  

---

# 12. Receipts and Audit

## 12.1 Review Receipt

Foreign Integration Review emits:

- Review Receipt  

---

## 12.2 Reconciliation Receipt

Reconciliation Review emits:

- Reconciliation Receipt  

---

## 12.3 Audit Guarantees

System must preserve:

- full review lineage  
- item → review → session → resolution traceability  
- resolution → reconciliation → item traceability  
- relationship admission lineage  

---

# 13. CLI Orchestration Policies

- one active review per Area (recommended)  
- reviews may pause sessions  
- reconciliation may run independently  

---

# 14. Invariants

- Review does not create legitimacy  
- Reconciliation does not create legitimacy  
- All integration passes through sessions  
- All review is isolated  
- Structural relationships must be explicitly accepted  
- Node-class boundaries must be preserved  
- Derivation links must be explicit  
- History must never be rewritten  

---

# 15. Mental Model

The Review Layer means:

> “We decide what becomes real,  
> and we keep our understanding aligned with what is real.”

It allows systems to:

- admit structure safely  
- explore structure safely  
- synchronize investigation with decisions  

without ever:

- inheriting authority  
- assuming correctness  
- collapsing boundaries  

---

# 16. Final Principle

The Review Layer creates **friction and symmetry by design**.

That ensures:

- no silent trust  
- no accidental legitimacy  
- no divergence between thinking and decision  

Everything must be:

- seen  
- evaluated  
- decided  
- and, when needed, reconciled  

before it becomes — or remains — part of the system.