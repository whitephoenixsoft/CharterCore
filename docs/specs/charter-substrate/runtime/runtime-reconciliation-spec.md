# Charter Runtime — Review Layer & Reconciliation Specification (Revised)

Status: FOUNDATIONAL  
Applies to: Runtime Layer, CLI Orchestration, Charter Commit System (CCS), Commit Store, Discoverability & Federation  
Does NOT define: legitimacy semantics, alignment computation, transport protocols (CRS), or guidance behavior  

---

# 1. Purpose

This document defines the **Review Layer**, implemented as a unified **Reconciliation Review model**, responsible for:

- admitting candidate artifacts into legitimacy  
- promoting deliberate outputs into durable structure  
- synchronizing investigative state (CDS) with legitimacy  
- projecting legitimacy into investigation for simulation  
- admitting structural relationships into the graph  
- preserving legitimacy boundaries while enabling controlled evolution  

The Review Layer exists to ensure that:

- all structure entering legitimacy is explicitly evaluated  
- investigation and legitimacy remain separate but connected  
- synchronization between substrates is explicit and auditable  
- legitimacy is never created outside the engine  

---

# 2. Core Principle

> Reconciliation is the only bridge between investigation and legitimacy.

The Review Layer allows systems to:

- inspect candidate material  
- model relationships provisionally  
- prepare integration into legitimacy  
- project legitimacy into investigation  
- maintain alignment between thinking and decisions  

But it does not:

- create legitimacy by itself  
- evaluate authority independently  
- mutate existing decisions  
- bypass the engine  

---

# 3. Reconciliation Review Model

The Review Layer defines a single unified workflow:

→ **Reconciliation Review**

Reconciliation Review operates in multiple modes:

---

## 3.1 Forward Reconciliation (Toward Legitimacy)

> “Promote candidate structure into legitimacy.”

Inputs:

- CDS LOCKED Items  
- foreign artifacts (via discoverability/federation)  
- imported bundles  
- candidate structural relationships  

Process:

→ evaluation  
→ acceptance/rejection  
→ session execution  
→ legitimacy receipt  

Result:

- new or updated **resolution commits (CCS)**  
- admitted structural relationships (CSG)  

---

## 3.2 Reverse Reconciliation (Legitimacy → CDS)

> “Project legitimacy into investigation for simulation.”

Process:

- select existing resolutions  
- project them into CDS as Items  

For each resolution:

- create a **resolution-derived Item**  
- assign `derived_from = resolution_id`  

Properties:

- Items remain non-legitimate  
- source resolutions remain unchanged  
- lineage is preserved  

---

## 3.3 Context Synchronization

> “Keep investigation aligned with evolving legitimacy.”

Reconciliation may:

- evaluate CDS Items against current legitimacy  
- update Item states  

Examples:

- APPLIED remains valid  
- APPLIED becomes STALE  
- unresolved Items become satisfied  
- Items become SETTLED or DISCARDED  

---

## 3.4 Asymmetry Principle

Reconciliation modes are not symmetric:

- forward reconciliation produces **new durable artifacts (CCS)**  
- reverse reconciliation produces **non-durable investigative structure (CDS)**  

---

# 4. Runtime Local Access Model

CDS operates within the Runtime and has direct access to:

- the local Commit Store  
- structural projections (CSG)  
- local resolution artifacts  

Reconciliation Review operates directly on local state.

### Principle

> CCS is required for durability and interoperability, not for internal Runtime execution.

### Implications

- legitimacy → CDS projection does not require CCS transport  
- CDS → legitimacy requires CCS when producing new resolution commits  
- reconciliation reads from local structure and artifacts directly  

---

# 5. Shared Review Properties

All reconciliation operations share:

- isolation  
- explicit workflow states  
- auditability  
- non-legitimizing behavior until session execution (forward mode)  
- reversible operations until commitment  

---

# 6. Isolated Review Store

## 6.1 Isolation Requirement

Reconciliation operates on a **dedicated, isolated review workspace**.

This workspace is:

- read-only with respect to source artifacts  
- non-legitimizing  
- separate from active runtime state  

---

## 6.2 Purpose of Isolation

Isolation ensures:

- safe inspection of candidate material  
- prevention of accidental integration  
- clear separation between candidate and local truth  

---

# 7. Review Items

## 7.1 Definition

Reconciliation operates on **review items**.

A review item may represent:

- a proposal  
- a foreign resolution  
- a CDS LOCKED Item  
- a resolution-derived Item  
- a CRR recontextualization plan  
- an imported commit  
- a structural element of a graph  
- a candidate relationship  

---

## 7.2 Properties

All review items are:

- non-legitimate  
- mutable within the review workspace  
- auditable  
- reversible until session commitment (forward mode)  

---

## 7.3 States

- UNDER_REVIEW  
- ACCEPTED  
- REJECTED  
- ABANDONED  

These are workflow states, not legitimacy states.

---

# 7.4 CRR Integration (NEW)

CRR (Charter Resolution Recontextualization) produces:

- recontextualization plans  

These plans:

- are non-legitimate  
- are not commits  
- define candidate structural transformations  

Within the Review Layer:

- CRR plans MUST be treated as review items  
- CRR plans MUST undergo forward reconciliation  
- CRR plans MUST pass through session execution before producing resolution commits  

CRR must not:

- create resolution commits directly  
- bypass reconciliation workflows  
- introduce structure into legitimacy without review  

Principle:

> CRR defines how structure could change.  
> Reconciliation determines whether it becomes legitimate.

---

# 8. Structural Admission

## 8.1 Relationship Admission

Reconciliation may admit:

- resolution ↔ resolution references  
- derivation relationships  
- cross-area relationships  
- investigation-surfaced structure  

All relationships must be:

- explicit  
- reviewed  
- admitted through forward reconciliation  

---

## 8.2 Derivation Recording

Resulting resolution commits SHOULD include:

- `derived_from` references  

These:

- preserve lineage  
- may cross Areas  
- do not imply authority  

---

## 8.3 Node-Class Boundary

Reconciliation must preserve node-class distinctions:

- **resolution nodes** (legitimate)  
- **item nodes** (investigative)  

Mixed structures may exist in CDS or projections, but:

- legitimacy graph (CSG resolution projection) must remain clean  
- item-derived structure must not become legitimate without review  

---

# 9. CDS Interaction

CDS:

- produces LOCKED Items for forward reconciliation  
- consumes resolution-derived Items from reverse reconciliation  
- remains active during all reconciliation processes  

Reconciliation enables CDS to:

- simulate structural changes  
- explore alternative structures  
- surface candidate relationships  

---

# 10. CSP Interaction (Optional)

If CSP is present:

- signals may be shaped before becoming durable artifacts  
- reconciliation operates only on **durable CCS artifacts**  

CSP does not alter reconciliation semantics.

---

# 11. Relationship to Legitimacy

- Only sessions create legitimacy  
- Reconciliation prepares and aligns inputs  
- No legitimacy is created outside session execution  

---

# 12. Relationship to Federation

- foreign artifacts enter via discoverability  
- reconciliation evaluates and integrates them  
- no federation bypasses reconciliation  

---

# 13. Receipts and Audit

## 13.1 Review Receipt (Forward)

Forward reconciliation emits:

- Review Receipt  

---

## 13.2 Reconciliation Receipt (Reverse / Sync)

Reverse and synchronization modes may emit:

- Reconciliation Receipt  

---

## 13.3 Audit Guarantees

System must preserve:

- item → review → session → resolution traceability  
- resolution → reconciliation → item traceability  
- structural relationship admission lineage  

---

# 14. CLI Orchestration Policies

- one active reconciliation per Area (recommended)  
- reconciliation may pause sessions  
- reverse reconciliation may run independently  

---

# 15. Invariants

- Reconciliation does not create legitimacy by itself  
- Only sessions create legitimacy  
- All integration, including CRR-derived transformations, passes through forward reconciliation
- All review is isolated  
- Structural relationships must be explicitly admitted  
- Node-class boundaries must be preserved  
- Derivation links must be explicit  
- CCS is required only for durable artifact creation  
- History must never be rewritten  

---

# 16. Mental Model

Reconciliation means:

> “We decide what becomes real,  
> and we keep our understanding aligned with what is real.”

It allows systems to:

- admit structure safely  
- simulate structure safely  
- synchronize investigation with decisions  

without ever:

- inheriting authority  
- assuming correctness  
- collapsing boundaries  

---

# 17. Final Principle

The Review Layer creates **friction and asymmetry by design**.

That ensures:

- no silent trust  
- no accidental legitimacy  
- no confusion between thinking and decision  

Everything must be:

- observed  
- investigated  
- evaluated  
- decided  
- and reconciled  

before it becomes — or remains — part of the system.