# Charter Deliberate Substrate (CDS) — Foundation Specification (Revised v3)

Status: FOUNDATIONAL  
Intent: Define structured pre-legitimacy thinking, investigation, synthesis, and care continuity  
Scope: Deliberate workflows, item lifecycle, observations, collaboration, drafts, application, reconciliation, and storage  
Depends On: Runtime Layer, Charter Commit System (CCS) (for artifact packaging only)  
Does NOT Define: legitimacy, alignment (CAS), identity (CIS), graph structure (CSG), or guidance (CGL)  

---

# 1. Purpose

The Charter Deliberate Substrate (CDS) defines how thinking occurs **before, during, and alongside legitimacy**.

It exists to:

- provide a structured environment for exploration and investigation  
- support fragmented, multi-source problem discovery  
- enable collaboration without authority  
- support synthesis without enforcing outcomes  
- preserve the full pre-history and context of decisions  
- maintain ongoing care context even after decisions are made  

CDS is a **non-authoritative investigation and thinking substrate**.

---

# 2. Core Principle

> Deliberate enables investigation and thinking without creating authority.

CDS:

- allows unrestricted exploration within a bounded scope  
- preserves all intermediate states  
- supports convergence without enforcing outcomes  
- produces candidates for review, not decisions  
- may persist beyond decision-making as contextual memory  

---

# 3. Architectural Position

CDS is a **Runtime-hosted substrate**.

- Runtime orchestrates CDS workflows  
- CDS defines its own structure, lifecycle, and invariants  

CDS interacts with:

- Review workflows (via Runtime)  
- CCS (for artifact packaging of outputs)  
- CCare (as input signals)  

CDS does not:

- create legitimacy  
- enforce decisions  
- modify external substrates  

---

# 4. Core Constructs

## 4.1 Epic (Scope)

An Epic defines the **bounded scope of investigation or thinking**.

---

## 4.2 Deliberate Decision Rule (DDR)

Defines how internal coordination decisions are made within CDS.

Properties:

- coordination-only  
- non-authoritative  
- local to the deliberate instance  

Process:

- a deliberate a decision rule is must selected:
    - SOLE_ACTOR (default)
    - UNANIMOUS_PRESENT
    - MAJORITY_PRESENT
- participants are selected to be part of the voting for synthesis
- during synthesis the participants vote to allow the Item to become LOCKED

---

## 4.3 Item

An Item is the **universal unit of thought and investigation**.

An Item may represent:

- a question  
- a hypothesis  
- an observation  
- a proposal  
- a task  
- a constraint  
- a synthesized output  

---

### 4.3.1 Observations

Items may include an **observations block** representing evidence.

Observations may include:

- metric references (internal or external)  
- data snapshots  
- logs or telemetry  
- qualitative notes  
- external evidence  

Properties:

- observational only (non-authoritative)  
- may be incomplete or inconsistent  
- may evolve over time  

Principle:

> Observations represent what is seen, not what is decided.

---

### 4.3.2 External References (Expanded)

Items may reference multiple external artifacts:

- resolution_ids  
- signal_ids (CCare)  
- area_ids  
- identity_ids (optional)  

Properties:

- many-to-many relationships allowed  
- references do not imply causality  
- references may evolve during investigation  

Principle:

> Items aggregate fragmented context without enforcing structure.

---

### 4.3.3 Contributions

Items may record contributor interactions:

- creation  
- modification  
- annotation  
- synthesis actions  

Properties:

- descriptive only  
- non-authoritative  
- does not imply ownership  

Principle:

> Contributions record participation, not authority.

---

## 4.4 Participants

A deliberate instance may include participants.

Participants may be:

- individuals (by name)  
- participant groups  

Properties:

- descriptive only  
- mutable during runtime  
- not tied to authority  

Function:

- can contribute 
- can be selected for voting in DDR

Principle:

> Participation does not imply authority.

---

## 4.5 Board

The Board represents active thinking.

Contains Items in:

- IN_PROGRESS  
- READY  
- BLOCKED  
- DEFERRED  

---

## 4.6 Breakout

Breakouts are isolated by design. They are short-lived focused exploration.

They do not:

- compete for legitimacy  
- race for acceptance  
- alter engine state  

Their purpose is exploration, not convergence.

During the breakout the item state is IN_PROGRESS.
After a breakout and Item becomes READY, BLOCKED, or DEFERRED.

---

## 4.7 Synthesis

Synthesis is a **continuous convergence process**.

Synthesis may:

- refine, merge, split, or create Items  
- produce LOCKED Items  
- return Items to Board  
- prepare Items for Review  

> **Receipt Note:** Freezing a synthesis artifact emits an **Exploration Receipt** capturing artifact IDs, lineage, and end-state (e.g., SYNTHESIZED).

Principle:

> Synthesis shapes thinking without enforcing finality.

---

## 4.8 Drafts

Drafts are **reusable bundles of candidate material**.

A draft may include:

- Items  
- observations  
- annotations  
- candidate outputs  

Properties:

- reusable across workflows  
- non-authoritative  
- may be exported or shared  

Principle:

> Drafts package thinking without finalizing it.

---

# 5. Item Lifecycle

## 5.1 States

- IN_PROGRESS  
- READY  
- LOCKED  
- APPLIED  
- SETTLED  
- BLOCKED  
- DEFERRED  
- DISCARDED  

---

## 5.2 State Principles

- all transitions are explicit  
- no state implies legitimacy  
- lifecycle reflects investigation → stabilization → integration  

---

## 5.3 LOCKED

Stable output of deliberate.

---

## 5.4 APPLIED

Item has been integrated into legitimacy outcomes.

Application may result in:

- new resolution(s)  
- modified/superseded resolution(s)  
- parameter/annotation updates  
- no structural change (confirmed validity)  

---

## 5.5 SETTLED

Item has reached a stable conclusion within CDS.

Properties:

- no further active work required  
- may or may not be applied  
- remains part of history  

Principle:

> Settled means understood, not enforced.

---

## 5.6 Partial Application 

Items may be:

- fully applied  
- partially applied  
- not applied  

Properties:

- many-to-many mapping allowed  
- must preserve traceability  

Principle:

> Application is not one-to-one.

---

# 6. Synthesis Model

Application does not occur in CDS.

It occurs through:

→ Review  
→ Session  
→ Resolution  

---

# 7. Forking Model

Unchanged (new deliberate instance, no shared mutability).

---

# 8. Storage Model

## 8.1 Workspace Store
Mutable.

## 8.2 Artifact Store
Append-only.

Stores:

- LOCKED Items  
- APPLIED Items  
- SETTLED Items  
- Deliberate Receipts  

---

# 9. Application & Reconciliation

## 9.1 Application Boundary

Occurs only via Review + Session.

---

## 9.2 Reconciliation

Runtime maps:

- resolution(s) ↔ Items  

Transitions:

- LOCKED → APPLIED  

---

## 9.3 Properties

- explicit  
- auditable  
- does not require closure  

---

## 9.4 Principle

> Reconciliation connects thought to decision without collapsing either.

---

# 10. Closure

Closure records a snapshot.

---

## 10.1 Types

- SYNTHESIZED  
- ABANDONED  
- FORKED  
- ARCHIVED  

---

## 10.2 Deliberate Receipt

Includes:

- deliberate_id  
- closure_type  
- item_ids  
- applied_item_ids  
- settled_item_ids  
- resolution_ids  
- review_receipt_ids  
- timestamp  
- annotations  

---

## 10.3 Living Deliberates (NEW)

Deliberate instances may remain active across:

- multiple application cycles  
- ongoing monitoring  
- extended investigations  

Principle:

> Deliberate may persist as living context.

---

# 11. Annotation

Unchanged.

---

# 12. Relationships

## 12.1 Runtime
Orchestrates + reconciliation.

## 12.2 Review
Consumes LOCKED Items.

## 12.3 CCS
May include `derived_from`.

## 12.4 CCare

### CCare Intake (NEW)

- signals become Items or Item inputs  
- original context preserved  
- no automatic interpretation  

Principle:

> Signals are hosted, not redefined.

---

# 13. Invariants

- CDS does not create legitimacy  
- Items mutable until stabilized  
- APPLIED requires external legitimacy outcome  
- SETTLED requires explicit stabilization  
- reconciliation is explicit  
- drafts are non-authoritative  
- participants do not imply authority  
- deliberate may remain active indefinitely  

---

# 14. Mental Model

CDS is:

- an investigation space  
- a problem reconstruction system  
- a collaborative thinking environment  
- a bridge between observation and decision  

---

# 15. Final Principle

CDS exists so that:

- problems can be explored before being defined  
- evidence can be gathered without pressure  
- thinking can evolve without authority  
- decisions can emerge without rewriting history  

Deliberate preserves how we investigate —  
and records how understanding becomes action without ever becoming authority itself.
