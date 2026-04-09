# Charter Deliberate Substrate (CDS) — Foundation Specification (Revised v4)

Status: FOUNDATIONAL  
Intent: Define structured pre-legitimacy thinking, investigation, synthesis, simulation, and care continuity  
Scope: Deliberate workflows, item lifecycle, observations, collaboration, subscriptions, drafts, application, reconciliation, and storage  
Depends On: Runtime Layer, Charter Commit System (CCS) (for artifact packaging only), Charter Signal Processing Substrate (CSP)  
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
- enable simulation of structure prior to legitimacy  

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
- may operate over both existing structure and newly created Items  

---

# 3. Architectural Position

CDS is a **Runtime-hosted substrate**.

- Runtime orchestrates CDS workflows  
- CDS defines its own structure, lifecycle, and invariants  

CDS interacts with:

- Review workflows (via Runtime)  
- CCS (for artifact packaging of outputs)  
- CCare (as signal input)  
- CSP (as feed input)  

CDS does not:

- create legitimacy  
- enforce decisions  
- modify external substrates  

CDS operates over both:

- Items (investigative structure)  
- referenced resolutions (legitimate structure)  

These must remain structurally distinct.

Principle:

> CDS may simulate over structure, but must not collapse investigative and legitimate nodes.

---

# 4. Core Constructs

## 4.1 Epic (Scope)

An Epic defines the **bounded scope of investigation or thinking**.

Properties:

- required before investigation begins  
- defines semantic framing of the deliberate  
- may be derived from signals, goals, or external context  

Principle:

> All deliberate activity must be scoped.

---

## 4.2 Deliberate Decision Rule (DDR)

Defines how internal coordination decisions are made within CDS.

Properties:

- coordination-only  
- non-authoritative  
- local to the deliberate instance  

### Timing

- a deliberate may begin **without a DDR**
- no Item may transition to LOCKED without an active DDR

### Supported Rules

- SOLE_ACTOR (default)  
- UNANIMOUS_PRESENT  
- MAJORITY_PRESENT  

### Process

- participants are selected for synthesis voting  
- selected participants evaluate Items  
- voting determines transition to LOCKED  

Principle:

> DDR governs convergence, not authority.

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

- metric references  
- data snapshots  
- logs or telemetry  
- qualitative notes  
- external evidence  

Properties:

- observational only  
- incomplete or inconsistent allowed  
- evolves over time  

Principle:

> Observations represent what is seen, not what is decided.

---

### 4.3.2 External References

Items may reference:

- resolution_ids  
- signal_ids (CCare)  
- area_ids  
- identity_ids (optional)  

Properties:

- many-to-many  
- no implied causality  
- may evolve  

Principle:

> Items aggregate fragmented context without enforcing structure.

---

### 4.3.3 Contributions

Items may record:

- creation  
- modification  
- annotation  
- synthesis participation  

Properties:

- descriptive only  
- includes contribution type  
- may include ordering  

Principle:

> Contributions record participation, not authority.

---

### 4.3.4 Relationship Surfacing (NEW)

Items may **implicitly or explicitly surface relationships** between referenced artifacts.

Examples:

- multiple resolutions referenced by one Item  
- grouped Items suggesting dependency or tension  
- derived hypotheses connecting artifacts  

Properties:

- non-authoritative  
- may be incomplete or conflicting  
- not part of CSG unless admitted through Review  

Principle:

> CDS may surface candidate structure without creating structure.

### 4.3.5 Resolution-Derived Items (NEW)

Items may be derived from existing resolutions via Reconciliation Review.

These Items:

- represent investigative projections of legitimate structure  
- carry `derived_from` lineage  
- may participate in simulation and synthesis  
- do not modify the source resolution  

Principle:

> A resolution-derived Item is a working counterpart, not a copy of legitimacy.

---

## 4.4 Participants

A deliberate instance may include participants.

Participants:

- may contribute  
- may be selected for DDR voting  

Properties:

- descriptive only  
- mutable  
- not authoritative  

---

## 4.5 Board

Contains Items in:

- IN_PROGRESS  
- READY  
- BLOCKED  
- DEFERRED  

---

## 4.6 Breakout

Short-lived, isolated exploration.

Properties:

- does not persist as structure  
- does not create authority  
- may produce Items  

---

## 4.7 Synthesis

Continuous convergence process.

Synthesis may:

- refine, merge, split, create Items  
- produce LOCKED Items  
- return Items to Board  

Receipt:

- Exploration Receipt emitted upon freeze  

---

## 4.8 Drafts

Reusable bundles of Items and context.

---

## 4.9 Subscriptions (NEW)

Deliberates may subscribe to **CSP feeds**.

### Properties

- references feed_id  
- optional local filters  
- non-authoritative  
- does not create Items automatically  

### Behavior

- provides ongoing observational context  
- enables monitoring of referenced resolutions or scopes  

Principle:

> Subscriptions provide visibility, not automation.

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

## 5.2 SETTLED Clarification

SETTLED includes:

- confirmed valid conclusions  
- confirmed invalid (false) conclusions  

DISCARDED is used for:

- abandoned or irrelevant Items  

---

## 5.3 LOCKED

Stable output, eligible for Review.

---

## 5.4 APPLIED

Item has been integrated into legitimacy outcomes.

---

## 5.5 Partial Application

- many-to-many mapping  
- may apply subset of Item content  

---

# 6. Item Seeding Model (NEW)

Items may originate from:

### A. Direct Creation
- questions  
- hypotheses  
- tasks  

### B. CCare Intake
- signals become observations  
- may seed Items or enrich existing Items  

### C. CSP Feeds
- feed data informs observation context  
- may inspire Item creation  

Principle:

> CDS does not automatically convert signals into decisions.

### D. Reconciliation Seeding (NEW)

Items may be created through **Reconciliation Review** from existing resolutions.

For each selected resolution:

- a new Item is created in CDS  
- the Item is assigned `derived_from = resolution_id`  

These Items are referred to as:

> **resolution-derived Items**

Properties:

- remain Item nodes (not resolutions)  
- preserve lineage to source resolution  
- may include observational or structural context  
- may evolve independently within CDS  

Principle:

> Reconciliation allows CDS to project legitimate structure into investigation without importing authority.

---

# 7. Synthesis Model

Application does not occur in CDS.

Occurs through:

→ Review  
→ Session  
→ Resolution  

---

# 8. Storage Model

## 8.1 Workspace Store
Mutable.

## 8.2 Artifact Store
Append-only.

Stores:

- LOCKED  
- APPLIED  
- SETTLED  
- Receipts  

---

# 9. Application & Reconciliation

## 9.1 Boundary

Application occurs via:

→ Review  
→ Session  
→ Resolution  

Reconciliation occurs via:

→ Reconciliation Review  

---

## 9.2 Forward Reconciliation (CDS → Legitimacy)

Runtime maps:

- Items → resulting resolution(s)  

May use:

- `derived_from` relationships in resulting resolutions  

Transitions:

- LOCKED → APPLIED  

---

## 9.3 Reverse Reconciliation (Legitimacy → CDS)

Through Reconciliation Review, CDS may:

- project existing resolutions into Items  
- create resolution-derived Items  

This enables:

- simulation over existing structure  
- investigation starting from legitimate context  

---

## 9.4 Reconciliation Properties

- explicit and auditable  
- bidirectional  
- supports partial mapping  
- does not require deliberate closure  
- does not mutate legitimacy  

---

## 9.5 Principle

> Reconciliation synchronizes investigation and decision without collapsing either.

---

# 10. Care Loop (NEW)

CDS participates in continuous cycles:

observation → investigation → synthesis → application → observation  

Sources:

- CCare  
- CSP  
- existing resolutions  

Principle:

> CDS maintains continuity between observation and decision.

---

# 11. Closure

Unchanged.

---

# 12. Relationships

## 12.1 Runtime
Orchestrates + reconciliation.

## 12.2 Review
Consumes LOCKED Items and surfaced relationships.

## 12.3 CCS
May include `derived_from`.

## 12.4 CCare
Signals seed observations.

## 12.5 CSP
Feeds provide ongoing context.

---

# 13. Invariants

- CDS does not create legitimacy  
- Items mutable until stabilized  
- DDR required for LOCKED  
- APPLIED requires external legitimacy  
- subscriptions do not create Items  
- reconciliation is explicit  
- relationships surfaced are non-authoritative  
- deliberate may remain active indefinitely  

---

# 14. Mental Model

CDS is:

- an investigation system  
- a simulation layer over structure  
- a collaborative reasoning environment  
- a bridge between observation and decision  

---

# 15. Final Principle

CDS exists so that:

- problems can be explored before being defined  
- evidence can be gathered without pressure  
- structure can be simulated before commitment  
- decisions can emerge without rewriting history  

Deliberate preserves how we investigate —  
and enables understanding to evolve into structure  
without ever becoming authority itself.