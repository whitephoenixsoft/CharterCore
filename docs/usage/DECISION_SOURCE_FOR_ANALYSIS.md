# Usage Pattern — Charter Core as a Decision Source for Empirical Analytics

## Audience
Integrators building:
- analytics platforms
- research tools
- performance dashboards
- decision-quality tracking systems

that evaluate **outcomes**, not legitimacy.

---

## Intent

This pattern uses Charter Core as a **canonical source of legitimate decisions**, while an external system independently tracks how those decisions perform in the real world.

Key separation:

> **Charter Core records what was decided.  
> Analytics observe what happened next.**

These domains must never collapse into each other.

---

## Core Separation of Concerns

### Charter Core (Normative System)
Records:
- resolutions
- authority
- scope
- acceptance timestamps
- supersession relationships

Does not record:
- success or failure
- performance metrics
- outcomes
- effectiveness judgments

---

### Analytics System (Descriptive System)
Records:
- metrics (e.g. ROI, latency, error rates)
- time windows
- environmental context
- correlations across decisions

Does not:
- alter Charter Core state
- influence acceptance
- create or retire resolutions

---

## Required Integration Behavior

### 1. Referential Linking Only

Analytics may:
- reference resolutions by engine ID
- associate metrics with time ranges
- observe supersession boundaries

Analytics must not:
- modify resolutions
- annotate legitimacy state
- inject metadata back into Charter Core

The link is **one-way and read-only**.

---

### 2. Time-Bound Interpretation

Metrics should be evaluated relative to:
- resolution acceptance time
- resolution supersession time (if any)

This preserves:
- historical accuracy
- non-retroactivity
- analytical clarity

Do not reinterpret past legitimacy based on future outcomes.

---

### 3. No Feedback Loops Into Legitimacy

Strictly forbidden:
- auto-superseding “bad” decisions
- auto-retiring low-performing resolutions
- blocking sessions due to metrics
- accepting decisions based on analytics thresholds

Corrections must always occur via:
- new sessions
- new resolutions
- explicit human acceptance

---

## Allowed and Safe Uses

Integrators may safely:
- compare decision lifetimes
- analyze frequency of supersession
- correlate authority models with outcomes
- study scope clarity vs performance
- visualize decision timelines

All of these are **observational**, not authoritative.

---

## UI & Messaging Requirements

Any analytics UI must clearly communicate:

- Effectiveness ≠ legitimacy
- Performance data is advisory only
- Decisions remain legitimate regardless of outcome

Ambiguity here is a serious integration failure.

---

## What Charter Core Will Never Do for Analytics

Charter Core will not:
- score decisions
- rank resolutions
- infer correctness
- recommend changes
- explain outcomes

All interpretation belongs to the analytics layer.

---

## Summary for Integrators

This pattern enables:
- evidence-based decision reflection
- governance research
- institutional learning

Without corrupting:
- legitimacy
- audit integrity
- determinism

**Charter Core issues decisions.  
Analytics observe consequences.  
Humans decide what to do next.**