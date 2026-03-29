# Audit, Breakouts, and Synthesis  
## How Charter Remembers Without Deciding

**Status:** FROZEN (Foundational)  
**Applies to:** Charter CLI / Interaction Layer  

This document explains **how Charter records human activity** without turning it into decisions.  
It exists to guide future specifications — especially auditing — by clarifying the *roles and boundaries* of:

- breakouts  
- options  
- synthesis  
- baselines  
- sessions  
- audit itself  

This is not a workflow.  
This is not a UI guide.  
This is a mental model.

---

## Core Principle

Charter distinguishes between three fundamentally different human activities:

- **Exploration** — thinking, learning, questioning, drafting  
- **Convergence** — structuring, grouping, refining, preparing  
- **Legitimacy** — deciding, accepting, committing  

Audit must span all three.

**Legitimacy exists only in the last.**  
Everything before legitimacy is memory.

---

## Moments in Time, Not Streams

Each moment in Charter (breakout, synthesis, session) is **bounded**:

- a start  
- a declared context  
- a set of participants (if any)  
- a set of inputs  
- a set of outputs  
- an explicit end  

Audit records **that the moment occurred** —  
not correctness, agreement, or success.

Charter captures moments so humans can revisit them later  
without rewriting history.

---

## Receipts as First-Class Audit Entities

Audit now treats **Receipts** as canonical, structured records for every bounded lifecycle:

- **Exploration Receipts** — emitted at Deliberate closure, Breakout closure, or Synthesis artifact finalization  
- **Review Receipts** — emitted at Baseline Review closure  
- **Legitimacy Receipts** — emitted at Session closure with accepted proposals  

Receipts must be:

- immutable  
- append-only  
- reconstructible deterministically  
- referenced in audit output for lineage and traceability  

---

## Breakouts: Recorded Exploration

Breakouts exist to explore without deciding. They:

- do not vote  
- do not accept or reject  
- do not evaluate authority  
- do not create legitimacy  

Artifacts produced:

- draft resolution text  
- option descriptions  
- problem statements  
- tradeoff notes  
- risks and concerns  
- unanswered questions  
- action items  
- recommendations  

Artifacts:

- have identity  
- are auditable  
- are referenceable  
- carry **zero authority**  

Breakouts answer:

> “What happened during this exploration?”  

They never answer:

> “What should be decided?”  

**Breakouts generate Exploration Receipts** at closure.

---

## Options: Exploratory Placeholders

Options represent **things worth thinking about**, not deciding.  

They may be:

- proposed solutions  
- questions  
- concerns  
- hypotheses  
- directions  
- problem reframings  

Options are mutable during exploration,  
never become candidates or proposals until formally promoted.

---

## Restarting Breakouts

Restarting a breakout:

- closes the prior breakout  
- preserves produced artifacts  
- records explicit lineage  
- begins a new exploratory moment  

Nothing is erased; earlier thinking remains visible.  
Audit reflects **how understanding evolved**, not just final outcomes.

---

## Synthesis: Convergence Without Authority

Synthesis structures exploration:

- groups related artifacts  
- refines language  
- separates alternatives  
- records discarded paths  
- identifies unresolved questions  
- clarifies readiness  

Synthesis **does not**:

- accept  
- reject  
- override  
- interpret authority  

**Synthesis emits Exploration Receipts** when artifacts are finalized.  

---

## Relationship to Baseline Review

Baseline review consolidates work for governance:

- evaluates foreign/consolidated material  
- pauses active sessions  
- prepares proposals for canonical sessions  

Neither synthesis nor baseline review creates legitimacy.  
Both **terminate explicitly** and emit **Receipts**:

- Baseline Review → Review Receipt  
- Synthesis → Exploration Receipt

---

## From Exploration to Legitimacy

Lifecycle progression:

1. Breakouts → Exploration Receipts  
2. Synthesis → Exploration Receipts  
3. Baseline Review → Review Receipts  
4. Sessions → Legitimacy Receipts  

At no point is intent assumed.  
Memory is never mistaken for agreement.

---

## Audit Requirements

Audit must:

- render receipts one per line  
- include **receipt type**  
- include canonical engine IDs  
- preserve **stable, deterministic ordering**  
- allow grep/search by receipt type  
- allow full lineage reconstruction:

  - Exploration → Review → Legitimacy  

Audit captures **everything that happened**, not just outcomes.

---

## Why This Matters

Without explicit audit and receipts:

- history collapses into outcomes  
- rejected or deferred paths disappear  
- trust erodes quietly  

Charter preserves:

- all moments of exploration  
- structured convergence artifacts  
- canonical legitimacy events  

---

## Final Note

Charter audit:

- remembers every bounded event  
- never assumes correctness or agreement  
- treats receipts as first-class, reconstructible artifacts  

Exploration is allowed to be incomplete.  
Convergence is allowed to be messy.  
Legitimacy is allowed to be rare.