# Baseline Review
**Status:** FROZEN (Target)  
**Applies to:** Charter CLI and Charter Core interaction layer

---

## Purpose

Baseline Review is the mechanism by which foreign, imported, or external proposals are evaluated and either integrated into canonical Charter history or explicitly rejected.

It exists to prevent:

- Silent trust of external data  
- Legitimacy drift across time or versions  
- Retroactive reinterpretation of prior decisions  

**Baseline Review is not a shortcut.**  
It is a legitimacy firewall: nothing reviewed here becomes canonical until explicitly accepted through a session.

---

## Core Idea

Baseline Review evaluates proposals, **not decisions**.  

- No proposal in a baseline creates legitimacy.  
- Legitimacy is created **only** via sessions in the main engine.  

---

## Baseline Review Properties

A Baseline Review is:

- A bounded review workspace  
- Mutable until explicitly closed  
- Fully auditable  
- Isolated from active decision-making  
- Non-legitimizing by design  

It **does not**:

- Vote  
- Evaluate or infer Authority  
- Create consensus or legitimacy  
- Modify existing resolutions  
- Reinterpret imported legitimacy  

Any appearance of legitimacy is a **CLI abstraction only**.  
All actual legitimacy comes from sessions.

---

## Lifecycle

### 1. Creation

A Baseline Review is created by:

- Flat-file import  
- Consolidate import  
- Deliberate handoff  

Restore operations **replace an Area entirely** and bypass review.  

Upon creation:

- All proposals enter state: `UNDER_REVIEW`  
- All active sessions are paused  
- Baseline becomes the sole active review context  

---

### 2. Evaluation Phase

During review:

- Proposals may be inspected, compared, grouped, or marked as unchanged/modified (ergonomic only)  
- Authority is **never evaluated**  
- No legitimacy is created  
- Acceptance or rejection is reversible preparatory work until the baseline is closed  

---

### 3. Acceptance Semantics

When a proposal is accepted from a baseline review:

- A **new session** is created in the main engine (explicit or hidden)  
- A **Legitimacy Receipt** is emitted for the session  
- The **topic** and **candidates** are copied from the baseline  
- **Participants** are explicitly defined for the session  
- **Authority and Scope** are evaluated according to the main engine rules (baseline Authority/Scope are **never imported**)  
- Acceptance is recorded immutably and auditably  
- Pre-existing annotations, rationale, and baseline session receipts are preserved for context  

**Batch acceptance**:

- Creates multiple sessions  
- Emits multiple **Legitimacy Receipts** (one per accepted proposal)  
- Does not collapse legitimacy into a single event  

**Rejected proposals**:

- Remain auditable  
- Do not affect local legitimacy  
- Carry no semantic meaning beyond “not accepted”  

> **Key Principle:** Baseline content informs preparation but does not bypass governance.  
> Legitimacy requires canonical participants, Authority, and Scope.

---

### 4. Closure

When a baseline review is closed:

- Remaining `UNDER_REVIEW` proposals → `ABANDONED`  
- Baseline becomes immutable  
- Paused sessions may resume  
- Closure is irreversible  
- Unaccepted proposals do not linger ambiguously  
- A **Review Receipt** is emitted capturing all reviewed, accepted, rejected, and abandoned proposals  

---

## Invariants

- **Baseline Review never creates legitimacy**  
- **Every accepted proposal corresponds to a session and emits a Legitimacy Receipt**  
- **No proposal becomes ACTIVE implicitly**  
- **Review history must be reconstructible end-to-end**  
- **Authority and Scope from baseline are advisory only**; canonical legitimacy uses main engine governance  
- **Topics, candidates, annotations, and session receipts are preserved** in acceptance  
- **Closure always emits a Review Receipt**  

---

## Relationship to Other Constructs

| Construct | Role in Baseline Review |
|-----------|------------------------|
| **Import (consolidate / flat)** | Creates a baseline review workspace |
| **Deliberate** | May produce a baseline review |
| **Session** | The only source of canonical legitimacy; generates Legitimacy Receipts |
| **Restore** | Replaces engine state, bypasses baseline review entirely |
| **Audit** | Must show full baseline lineage, including preserved topics and receipts |

---

## Mental Model

Think of Baseline Review as:

> “We are reviewing claims made elsewhere. Nothing here is official until we decide it again in the main engine.”

This friction is intentional: it prevents assumptions, preserves audit integrity, and separates exploratory review from canonical legitimacy.

---

## Notes on Session Changes

- **Topic**: always preserved when proposals are accepted from the baseline  
- **Candidates**: carried over into the main engine session  
- **Annotations / Rationale**: copied for context; cannot be edited in a way that would change meaning — new rationale requires a new resolution  
- **Participants**: defined fresh in the main engine session  
- **Authority / Scope**: evaluated in the main engine; baseline governance never counts for legitimacy  
- **Receipts**: preserved to link baseline evaluation to main engine acceptance; each accepted proposal produces a Legitimacy Receipt, and baseline closure produces a Review Receipt  

> Baseline Review informs preparation; it does not confer ownership, Authority, or Scope.  
> Receipts make all actions and lineage explicit and auditable.