# Charter — CLI Session Foundation
**Status:** FROZEN (Foundational)  
**Applies to:** Charter CLI / Engine Interaction Layer

## Purpose
This document explains how Charter turns human activity into **legitimate decisions** while preserving auditability, traceability, and psychological safety. It complements exploration (Breakouts, Synthesis), baseline review, and import workflows.

It covers:  

- Sessions  
- Authority  
- Proposals and candidates  
- Voting and stances  
- Acceptance and closure  
- Baseline review integration  
- Session receipts and audit  
- Preservation of topics and annotations  

## Core Distinction
Charter separates three things humans routinely collapse:  

1. **Exploration and work** — generating ideas, notes, or proposals  
2. **Agreement** — informal consensus or discussion  
3. **Legitimacy** — canonical, auditable decisions  

**Only the third creates authority.** Everything else exists to support it and **must never be mistaken for legitimacy**.  

## Sessions: The Only Place Legitimacy Is Created
A session is the smallest — and only — unit where legitimacy occurs.  

A session:  

- Declares an **Authority**  
- Declares **constraints**  
- Records explicit **participants**  
- Accepts **candidates** and proposals  
- Records **explicit stances** (ACCEPT, REJECT, ABSTAIN)  
- Tracks **topics** and annotations (mutable until acceptance)  
- Generates a **session receipt** capturing the full history  
- Terminates explicitly  

Nothing outside a session creates legitimacy:  

- Not Breakouts  
- Not Synthesis  
- Not Baseline Reviews  
- Not Imports  

**Sessions do not discover decisions; they record them.**

### Mutable and Immutable Fields
Within a session:  

- **Topic and annotations** may change freely until the first stance or acceptance  
- **Candidate content** is immutable after first stance  
- **Participants, Authority, Scope, and constraints** are frozen after first stance  
- **Session receipts** are generated incrementally, capturing all changes for audit  

## Proposals and Candidates: What Sessions Evaluate
A proposal is a concrete claim presented for legitimacy.  

Proposals may originate from:  

- Synthesis outputs  
- Baseline reviews  
- Deliberate authoring  
- Imported material (after review)  

A proposal:  

- Has **identity**  
- Has **content**  
- Has **provenance**  
- Carries **no authority** until accepted  

Within a session, proposals are evaluated as **candidates**:  

- Imply no intent  
- Imply no endorsement  
- Acquire meaning only if accepted  

## Authority: How Agreement Is Measured
Authority defines **how legitimacy is computed**, not what is correct.  

Authority specifies:  

- Who has **standing**  
- How **stances** are evaluated  
- What counts as **acceptance**  

Authority is:  

- Explicit  
- Immutable for the lifetime of the session  
- Evaluated mechanically  

Authority **never**:  

- Infers intent  
- Interprets silence  
- Assigns meaning to content  
- Resolves ambiguity  

## Stances: Explicit, Final, Auditable
Participants record stances explicitly: ACCEPT, REJECT, ABSTAIN  

- **First-class audit events**  
- Never expire or are reinterpreted  
- Freeze candidate sets once recorded  
- Silence, absence, or metadata do **not** count  

If it was not recorded, it did not happen.  

## Acceptance: Recording Legitimacy
Acceptance is **explicit, deliberate, and auditable**.  

- Occurs once per session per proposal  
- Authority is evaluated at the moment of acceptance  
- Participants, constraints, and scope must satisfy governing rules  
- Freezes **all session fields** permanently, except supersession in future  

Acceptance does **not**:  

- Erase disagreement  
- Imply correctness or permanence beyond supersession  
- Suppress alternatives  

## Closure: Ending the Legitimacy Moment
When a session closes:  

- Authority context is **sealed**  
- Participant set is **frozen**  
- Outcomes are **immutable**  
- **Session receipt** captures full history, including topic changes, annotations, pauses, resumes, and stance history  

Closed sessions:  

- Are exportable and auditable  
- Are referenceable for other workflows  
- Persist indefinitely  
- Never reopen  

**New legitimacy requires a new session.**

## Restart-From: Preserving Context Without Authority
Restarting:  

- Closes the prior session  
- Creates a new session  
- Resets **participants, stances, and acceptance**  
- Carries **topic, candidates, and annotations** as memory only  

Authority is **not inherited**. History remains visible.  

## Baseline Review: Legitimacy at Scale
Baseline Review is a **consolidation workspace**, not a decision engine.  

- Evaluates proposals, never authority  
- Proposals enter state **UNDER_REVIEW**  
- Active sessions are paused  
- Mutable until baseline closure  

### Acceptance From Baseline Review
When a proposal is accepted from a baseline:  

- A new session is created  
- Topic and candidates are **preserved**  
- Participants are **defined for the new session**  
- Authority and Scope are evaluated **fresh**  
- Acceptance creates **canonical legitimacy**  
- **Annotations and rationale** from baseline may be copied but do not override session evaluation  

**Key Principle:** importing Authority/Scope from baseline **is forbidden**, to prevent accidental legitimacy drift.  

## Importing and Foreign Material
Imported material is treated as:  

- Foreign and non-authoritative  
- Historically contextual  
- Auditable with preserved lineage  

Local authority always governs acceptance. Trust is **never assumed**, similarity never implies legitimacy.  

## Full Lifecycle (Top to Bottom)
**Exploration:** Breakouts → Notes → Options → Questions  
**Convergence:** Synthesis → Structured proposals → Discarded paths → Open issues  
**Evaluation:** Sessions → Authority → Explicit stances → Legitimacy  
**Commitment:** Acceptance → Closure → Immutable outcomes  
**Memory:** Audit → Export → Import → Baseline review → Historical continuity  

Charter never:  

- Guesses intent  
- Infers agreement  
- Reinterprets silence  
- Erases disagreement  
- Shortcuts legitimacy  

## Why This Matters
Most systems collapse:  

- Discussion into decision  
- Silence into consent  
- Outcomes into history  

Charter preserves:  

- Uncertainty and disagreement  
- Abandoned paths  
- Failed attempts  
- Context  
- **Legitimacy as demonstrable, not assumed**  

## Final Principle
Charter does **not** optimize for speed. It optimizes for **defensibility**.  

- Every accepted proposal can be traced to:  
  - When it was evaluated  
  - Under which Authority  
  - By whom  
  - Against what alternatives  
- If this cannot be demonstrated — Charter refuses to pretend it happened.