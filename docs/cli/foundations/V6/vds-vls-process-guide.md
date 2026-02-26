# How VDS Check-Ins Roll Up to VLS  
## A Practical Guide for Software Teams

This document explains, in plain terms, how Value-Directed Systems (VDS) and Value Lineage Systems (VLS) work together.

It is not a technical protocol description.  
It is a mental model for how teams participate.

---

# 1. The Core Idea

VDS is local.  
VLS is federated.

VDS answers:
“Are we still capable of fulfilling our declared intent?”

VLS answers:
“Are we collectively aligned across teams and identities?”

VDS does not collect metrics automatically.  
VDS does not interpret dashboards.  
Humans check in.

VLS does not judge performance.  
VLS aggregates signals and surfaces alignment patterns.

---

# 2. The General Flow

Every participating team follows this pattern:

1. Declare system intent.
2. Define care metrics (health indicators).
3. Establish thresholds for capacity shifts.
4. Periodically reflect.
5. Submit a structured VDS signal.

That signal includes:
- Intent state
- Capacity state
- Optional annotation
- Optional action declaration

Common signal states:
- Intent Unchanged
- Capacity Reduced
- Action Taken
- Observations Inconclusive
- Paused Intentionally
- Reassessment Requested

This is descriptive, not evaluative.

It does not say:
“We performed well.”

It says:
“Our system’s capacity relative to intent is X.”

---

# 3. Example Walkthroughs

## Example 1: E-Commerce Ordering System

### Declared Intent
Provide reliable and accurate order processing.

### Care Metrics
- Order processing success rate
- Inventory synchronization lag
- Checkout error rate
- Payment timeout frequency

### Local VDS Check-In

Observed:
- Order failure rate above threshold
- Inventory sync lag exceeds defined limits

Team Signal:
- Intent Unchanged
- Capacity Reduced
- Action Taken (scaling sync infrastructure)

No narrative required.  
No blame assigned.  
Just structural truth.

### VLS Roll-Up

Marketing Team:
- Intent Unchanged
- Capacity Normal
- Promotion campaign active

Fulfillment Team:
- Intent Unchanged
- Capacity Normal

VLS surfaces:

Demand generation is increasing while ordering capacity is reduced.

No automatic intervention.
No escalation.
Just visibility of cross-team strain.

---

## Example 2: SaaS Platform

### Declared Intent
Deliver stable, dependable customer workflows.

### Care Metrics
- 95th percentile response time
- Rollback frequency
- SLA-breaching tickets
- Error rate by endpoint

### Local VDS Check-In

Observed:
- Response times elevated beyond threshold
- Increased rollback frequency

Signal:
- Intent Unchanged
- Capacity Reduced
- Reassessment Requested

### VLS Roll-Up

Product Team:
- Intent Unchanged
- Capacity Normal
- Accelerated feature roadmap

Support Team:
- Intent Unchanged
- Capacity Reduced

VLS surfaces:

Feature velocity may be contributing to stability degradation and support strain.

Again:
No judgment.
No scoring.
Only alignment visibility.

---

## Example 3: Internal IT Service Desk

### Declared Intent
Provide timely and effective technical support.

### Care Metrics
- Ticket aging beyond SLA
- Technician workload distribution
- Escalation frequency
- Repeat issue rate

### Local VDS Check-In

Observed:
- Sustained SLA breaches
- Workload imbalance

Signal:
- Intent Unchanged
- Capacity Reduced
- Action Taken (training + redistribution)

### VLS Roll-Up

HR:
- Intent Unchanged
- Hiring Freeze Active

IT:
- Capacity Reduced

VLS surfaces:

Organizational staffing constraints impacting service capacity.

This makes institutional strain visible without political escalation.

---

## Example 4: API Platform

### Declared Intent
Provide stable and predictable integration contracts.

### Care Metrics
- Breaking changes per release
- Documentation lag
- Contract test failure rate
- Deprecations without notice

### Local VDS Check-In

Observed:
- Increase in undocumented behavior
- Breaking changes above baseline

Signal:
- Intent Unchanged
- Capacity Reduced
- Observations Conclusive

### VLS Roll-Up

Developer Relations:
- Increased onboarding campaigns

Engineering:
- Capacity Reduced

VLS surfaces:

Adoption growth is occurring while contract stability is degrading.

Early warning before partner trust erodes.

---

# 4. What Makes This Different From Standard Reporting

Traditional Reporting:
- Metrics dashboard
- Performance interpretation
- Executive summary
- Action plan

VDS Reporting:
- Intent state
- Capacity state
- Optional annotation
- Optional declared action

No grading.
No ranking.
No competitive scoring.

Just structured clarity.

---

# 5. What VLS Actually Does

VLS does not:
- Average performance scores
- Compare teams competitively
- Enforce behavior
- Assign blame

VLS:
- Aggregates identity-scoped signals
- Preserves context
- Highlights cross-identity tension
- Surfaces alignment patterns over time

It provides organizational situational awareness.

---

# 6. The Emotional Shift

Instead of asking:
“Are we hitting targets?”

Teams ask:
“Are we still structurally capable of delivering what we promised?”

Instead of:
“Who failed?”

The organization sees:
“Where is structural strain emerging?”

This reduces defensiveness.
It increases clarity.
It separates care from performance.

---

# 7. Final Summary

VDS is local care reporting.  
VLS is federated alignment visibility.

VDS answers:
“Are we structurally sound?”

VLS answers:
“Are we collectively aligned?”

Together, they create an environment where:

- Capacity shifts are surfaced early.
- Intent remains explicit.
- Misalignment becomes visible before failure.
- Organizational memory is preserved.

This is not about optimization.

It is about responsible continuity.