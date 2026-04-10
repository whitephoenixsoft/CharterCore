# Charter Core — State vs History Foundation
## Present State vs Immutable Record

Status: FOUNDATIONAL  
Layer: Conceptual / Engine Core  
Applies to: Engine runtime, receipts, audit, structural derivation, and all read/write operations  
Does NOT define: specific storage formats, indexing strategies, or canonical serialization  

---

## Purpose

This document defines the separation between **state** and **history** in Charter Core.

Its purpose is to ensure that:

- historical truth is never rewritten  
- current state is always explainable from prior events  
- legitimacy remains auditable across time  
- runtime views do not become sources of truth  
- reconstruction is always possible and deterministic  

This separation is fundamental to Charter’s identity as a **legitimacy system**, not a mutable database.

---

## Core Principle

> History is immutable. State is derived.

History answers:

> “What happened?”

State answers:

> “What is currently true as a result of what happened?”

State must always be explainable by history.  
History must never be altered to match state.

---

## Why This Separation Exists

Without a strict separation between state and history, the system would risk:

- silent rewriting of decisions  
- loss of auditability  
- ambiguity about legitimacy  
- inability to reconstruct past truth  
- drift between observed state and recorded events  

Charter prevents this by enforcing:

- append-only history  
- derived state  
- no reverse mutation  

---

## History

## Definition

History is the complete, append-only record of all events, artifacts, and commitments that have occurred within the system.

It is the authoritative source of truth.

---

## What History Contains

History includes:

- immutable domain objects (e.g., resolutions, sessions, participants, votes as recorded artifacts)  
- receipts (terminal session outcomes)  
- audit records (observational traces of actions)  
- structural relationships established at creation time (e.g., supersession links)  

History is not limited to “successful” outcomes.  
It includes all recorded, valid artifacts of the system.

---

## Properties of History

History must be:

- immutable  
- append-only  
- deterministic in structure  
- fully reconstructible  
- independent of runtime context  

Once an element enters history:

- it cannot be modified  
- it cannot be deleted  
- it cannot be reinterpreted by mutation  

If something is incorrect, the system records a **new fact**, not a correction of the old one.

---

## What History Must Not Do

History must not:

- be rewritten  
- be patched  
- be inferred from current state  
- be compressed in a way that loses meaning  
- depend on runtime-derived interpretations  
- change due to governance updates or usability transitions  

History records what *was*, not what *is now believed*.

---

## State

## Definition

State is the current, derived view of the system as computed from history under a specific rule set and runtime context.

State is not stored as primary truth.  
It is computed.

---

## What State Represents

State represents:

- current ACTIVE resolutions  
- current governance context (Authority, Scope)  
- current session lifecycle positions  
- current usability conditions (e.g., ON_HOLD, RETIRED effects)  
- current eligibility and blocking conditions  
- current runtime-visible structure  

State answers questions like:

- What is currently governing?  
- What is currently usable?  
- What is currently eligible?  

---

## Properties of State

State must be:

- derivable from history  
- deterministic given inputs  
- dependent on the rule set (spec_set_hash)  
- dependent on runtime mode  
- replaceable (it can be recomputed at any time)  

State is not authoritative in isolation.  
Its authority comes from its derivation.

---

## State Is Not Truth

State must not be treated as:

- an independent source of truth  
- a replacement for receipts  
- a substitute for audit  
- a mutable authority over history  

If state and history appear to conflict:

- history is correct  
- state must be recomputed or considered invalid  

---

## Separation Between State and History

The Engine must maintain a strict boundary:

### History Must Not:

- be modified to reflect current state  
- be recalculated based on new rules  
- be invalidated by new governance  
- be rewritten due to errors discovered later  

### State Must Not:

- override historical artifacts  
- reinterpret past legitimacy  
- discard historical structure  
- introduce facts not present in history  

---

## Reconstruction Requirement

A core invariant of Charter:

> State must be reconstructible from history.

Given:

- complete historical artifacts  
- a specification identity (spec_set_hash)  

The Engine must be able to:

- rebuild current state deterministically  
- explain all derived relationships  
- reproduce legitimacy outcomes  

If reconstruction is not possible:

- the system is invalid  

---

## Relationship to Receipts

Receipts are:

- part of history  
- the authoritative record of session outcomes  
- the binding artifact of legitimacy  

State must:

- respect receipt contents  
- derive current structure without altering them  

Receipts must not:

- be recomputed  
- be overridden by state  
- be invalidated by later changes  

---

## Relationship to Audit

Audit is:

- observational history  
- non-authoritative for legitimacy  
- append-only  

Audit helps explain:

- how state evolved  
- what actions were taken  

But:

- audit does not define state  
- audit does not override receipts  

---

## Relationship to Evaluation

Evaluation operates on **state derived from history**.

It must not:

- use incomplete history  
- rely on mutable state shortcuts  
- bypass reconstruction guarantees  

Evaluation answers:

- what is currently true  

It does not modify history.

---

## Relationship to Mutation

Mutation:

- appends new history  
- never alters existing history  

Every mutation:

- produces new artifacts  
- changes future derived state  
- leaves prior history untouched  

State changes because history grows, not because history changes.

---

## Time and Historical Stability

History is stable across time.

Later changes such as:

- supersession  
- retirement  
- ON_HOLD transitions  

affect:

- current state  

but do not affect:

- historical truth  

A past decision remains valid as a record of what happened, even if its forward usability changes.

---

## Invalid State Detection

If the Engine cannot derive a valid state from history:

- the system must not silently proceed  
- degraded mode or failure must occur  

Invalid state indicates:

- incomplete history  
- corrupted artifacts  
- violated invariants  

State must never be “guessed.”

---

## No Implicit Compression

The Engine must not collapse history into state in a way that loses meaning.

Examples of forbidden behavior:

- replacing full history with only current ACTIVE objects  
- discarding superseded artifacts  
- summarizing past decisions without preserving originals  

State may be optimized internally, but:

- full history must remain reconstructible  

---

## Conceptual Invariants

- History is immutable  
- History is append-only  
- History is authoritative  
- State is derived from history  
- State is deterministic  
- State is reconstructible  
- State must not override history  
- Mutation appends history, never rewrites it  
- Receipts preserve legitimacy truth permanently  
- Audit reflects events but does not define legitimacy  

---

## Mental Model

History is the ledger.  
It records everything that happened, exactly as it happened.

State is the current view.  
It reflects how those recorded facts combine under current rules.

You may recompute the view.  
You may extend the ledger.

You may never rewrite it.