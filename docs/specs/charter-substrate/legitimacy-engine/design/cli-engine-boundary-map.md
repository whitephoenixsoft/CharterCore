# Charter CLI ↔ Engine Boundary Map
**Purpose:** Make explicit what the CLI is allowed to do, what the engine guarantees, and where legitimacy is created.

This document exists to prevent:
- responsibility drift
- accidental authority creation
- UX features leaking into engine semantics

If behavior is unclear, this document wins.

---

## Core Principle

**The Engine is the source of legitimacy.  
The CLI is a guardian of explicit intent.**

The CLI may:
- block
- guide
- constrain
- structure

The CLI may never:
- infer intent
- create consensus
- invent authority
- reinterpret meaning

---

## Responsibility Split

### Engine Owns (Authoritative)
- Areas
- Sessions
- Candidates
- Resolutions
- Authority evaluation
- Scope evaluation
- Invariants
- Audit history
- Determinism guarantees

The engine:
- evaluates legitimacy mechanically
- does not know *how* a command was initiated
- trusts explicit inputs only

---

### CLI Owns (Ergonomic & Safety Layer)
- Context selection
- Label generation
- Participant declaration UX
- Session lifecycle gating
- Constraint declaration UX
- Baseline (import review) workflow
- Spec verification
- Audit presentation formatting

The CLI:
- prevents invalid engine calls
- makes legitimacy visible
- reduces accidental misuse
- preserves cognitive safety

---

## CLI → Engine Interaction Classes

### 1. Context Resolution
**CLI responsibility**
- Selects Area / Session / Baseline explicitly
- Enforces qualification rules

**Engine sees**
- IDs only

**Engine never**
- guesses context

---

### 2. Identity Resolution
**CLI responsibility**
- Resolves labels → engine IDs
- Generates labels when missing

**Engine sees**
- Canonical IDs only

**Engine never**
- interprets labels

---

### 3. Session Control
**CLI responsibility**
- Start / pause / resume / restart
- Block illegal transitions
- Enforce solo ergonomics

**Engine responsibility**
- Enforce invariant rules
- Record history

---

### 4. Participants & Authority
**CLI responsibility**
- Declare participants explicitly
- Display participant state
- Block acceptance when invalid

**Engine responsibility**
- Evaluate authority mechanically
- Snapshot participants at acceptance

---

### 5. Constraints
**CLI responsibility**
- Declare constraints before voting
- Treat constraints as authority-equivalent
- Block mid-session changes

**Engine responsibility**
- Evaluate constraints deterministically

---

### 6. Baseline (Import Review)
**CLI responsibility**
- Create review workspace
- Force explicit acceptance
- Prevent legitimacy leaks

**Engine responsibility**
- Record accepted outcomes as sessions
- Preserve provenance

---

### 7. Import / Export / Restore
**CLI responsibility**
- Gate destructive operations
- Warn loudly
- Require confirmation

**Engine responsibility**
- Replace or serialize state faithfully
- Emit global audit events

---

### 8. Spec Verification
**CLI responsibility**
- Load specs
- Query engine
- Report mismatches

**Engine responsibility**
- Provide state
- Remain immutable

---

## What the CLI Must Never Do

The CLI must never:
- infer agreement
- auto-accept unchanged data
- create sessions implicitly without audit
- mutate engine state during analysis
- hide authority evaluation
- pretend to know who “was there”

If a CLI feature requires breaking these rules, the feature is wrong.

---

## Design Rule of Thumb

If removing the CLI would change *what decisions exist* —
the CLI has crossed the boundary.

If removing the CLI would only change *how hard it is to act* —
the boundary is intact.

---

## Final Note

This boundary is intentional friction.

It exists to ensure that:
- legitimacy survives tooling changes
- authority survives UX improvements
- trust survives time

If the CLI ever feels “too helpful,”  
check this document first.