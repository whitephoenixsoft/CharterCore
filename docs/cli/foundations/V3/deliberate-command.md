# Deliberate
## Structured Thinking Without Authority

Status: CONCEPTUAL (Design Foundation)  
Applies to: Charter CLI and future interfaces  
Does NOT define: engine semantics, authority rules, acceptance behavior

---

## Purpose

Deliberate is a structured space for *thinking together* without creating decisions.

It exists to support:
- long or complex discussions
- parallel exploration
- synthesis of ideas
- deferred commitment

Deliberate is explicitly **pre-governance**.

It produces *inputs* to governance, not governance itself.

---

## What Deliberate Is

Deliberate is:
- a bounded workspace for discussion
- auditable but non-authoritative
- resumable across time
- compatible with solo and group use
- capable of producing structured outputs
- produces options with explicit states (e.g., READY, IN_PROGRESS, DEFERRED)

Deliberate models how humans actually think:
- workshops
- breakout rooms
- drafts
- synthesis sessions
- “we’re not ready to decide yet”

---

## What Deliberate Is Not

Deliberate is NOT:
- a decision mechanism
- a voting system
- a session substitute
- a soft-acceptance path
- a legitimacy shortcut

Participation in Deliberate implies **nothing** about authority, agreement, or consent.

---

## Core Boundary: Legitimacy Firewall

No action inside Deliberate may:
- accept a resolution
- activate a resolution
- supersede a resolution
- evaluate authority

Legitimacy begins **only** when outputs enter Baseline Review or Sessions.

This boundary is intentional and non-negotiable.

---

## Breakouts and Parallel Thinking

Deliberate may support:
- breakouts (parallel sub-discussions)
- draft creation and revision
- merging or discarding drafts

Breakouts are isolated by design.
They do not:
- compete for legitimacy
- race for acceptance
- alter engine state

Their purpose is exploration, not convergence.

---

## Synthesis

Synthesis is the explicit act of turning thinking into reviewable material.

Synthesis:
- is intentional
- is auditable
- produces a concrete artifact
- does not produce legitimacy

A synthesis may:
- create a new baseline review
- populate draft resolutions
- prepare candidate text

Synthesis may not:
- bypass review
- auto-accept outcomes
- imply consensus

Artifacts and options may be revised, deferred, or closed during Deliberate, as they represent thinking-in-progress rather than commitments.

---

## End States

Every Deliberate must end in exactly one of:
- **Synthesis** → Baseline Review
- **Abandonment** → Historical record only

Open-ended Deliberates are not allowed.
Closure preserves clarity and audit integrity.

---

## Audit Philosophy

Deliberate is auditable, but audits report:
- who participated
- what artifacts were produced
- how synthesis occurred

Audits do NOT report:
- agreement
- intent
- correctness
- implied consent

Charter records what happened — not what people meant.

---

## Relationship to the Engine

Deliberate is a CLI- and interface-level construct.

The engine:
- is unaware of Deliberate
- stores only resulting sessions and imports
- remains legitimacy-pure

This separation preserves engine minimalism and long-term correctness.

---

## Design Intent (Frozen)

Deliberate exists to make *not deciding yet* a first-class, safe action.

If Deliberate ever makes decisions easier than explaining them,
then Deliberate has failed.

If Deliberate ever feels slow, explicit, or slightly uncomfortable —
it is probably working as intended.