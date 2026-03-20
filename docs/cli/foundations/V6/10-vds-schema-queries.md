# Charter Alignment Framework
## Signal Schema & Guidance Queries

Status: Design Reference
Scope: Commit Store + Guidance Layer

--------------------------------

I. Commit Core Schema (Universal)

All commits share a common structural base.

Commit ID
Unique identifier for the commit.

Author(s)
Explicit owners of the commit.

Timestamp
When the commit was recorded.

Commit Type
Defines the nature of the commit.

Examples:
resolution
annotation
signal
receipt
import
baseline_review

Decision Statement
A clear description of the decision or observation.

Rationale
Why this decision or observation is recorded now.

Expected Observable Outcome
Evidence that would indicate success, failure, or change.

Reversibility Declaration
Reversible
Conditionally Reversible
Irreversible

Lineage (Optional)
References to prior commits this builds upon, modifies, or supersedes.

Target Reference (Optional)
Which object the commit concerns.

Examples:
resolution_id
session_id
area_id
system_component

Metadata (Extensible)
Key-value field for additional attributes.

Examples:
confidence
tags
severity
environment

--------------------------------

II. Signal Commit Schema

Signal commits represent alignment observations.

Commit Type
signal

Signal Type
alignment
misalignment
uncertainty
reduced_capacity
intentional_pause
need_reassessment

Target Reference
What the signal applies to.

Examples:
resolution
policy
system component
feature
service

Actor
Optional reporting actor.

Example:
team
service
individual
automation

Confidence Level (Metadata)
low
medium
high

Signal Weight (Derived)
Used for drift indicators.

Example default mapping:

alignment = +1
uncertainty = -0.5
misalignment = -1
reduced_capacity = -0.7
pause = -0.4

Horizon Level (Optional Metadata)
Maximum graph distance for signal influence.

Example:
implementation signals horizon = 2
policy signals horizon = 4
intent signals horizon = unlimited

Notes (Optional)
Additional human context.

--------------------------------

III. Signal Example

Example signal commit.

Commit Type
signal

Commit ID
SIG-92A7

Author
payment-service

Timestamp
2026-04-03T13:14

Signal Type
uncertainty

Target Reference
resolution: enable_async_checkout

Rationale
Payment timeout behavior unclear under high load.

Expected Observable Outcome
Load testing results clarify stability.

Reversibility
N/A

Metadata
confidence: medium
environment: staging

--------------------------------

IV. Drift Store Structure

The drift store aggregates signal statistics over time windows.

Stored aggregates:

signal_count
alignment_count
misalignment_count
uncertainty_count
capacity_signals
signal_variance
drift_score

Drift records are indexed by:

intent
area
resolution path
system component
time window

--------------------------------

V. Minimal Guidance Query Set

A small set of deterministic queries can power most dashboards.

--------------------------------

Query 1 — Intent Alignment Summary

Purpose
Measure alignment signals associated with an intent.

Inputs
intent_id
time_window

Outputs

alignment_count
uncertainty_count
misalignment_count
drift_score
variance

--------------------------------

Query 2 — Path Drift Indicator

Purpose
Measure drift along a resolution lineage.

Inputs

resolution_id
time_window

Algorithm

collect signals along lineage
apply signal weights
sum weighted values

Output

path_drift_score

--------------------------------

Query 3 — Signal Variance Detector

Purpose
Identify unstable alignment conditions.

Inputs

area_id
time_window

Algorithm

calculate variance of signal weights

Output

variance_score

Interpretation

low variance → stable
high variance → instability

--------------------------------

Query 4 — Intent Gravity Report

Purpose
Identify dominant intents influencing decisions.

Inputs

context_id
time_window

Algorithm
```
intent_gravity =
decision_references
+ alignment_signals
- misalignment_signals
+ recency_weight
```
Output

ranked intent gravity list

--------------------------------

Query 5 — Tension Zone Detector

Purpose
Identify conflicting intents.

Inputs

resolution_set
time_window

Algorithm

detect resolutions referencing multiple competing intents
collect associated signals

Output

tension_zone_report

--------------------------------

Query 6 — Capacity Pressure Monitor

Purpose
Detect execution constraints.

Inputs

area_id
time_window

Algorithm

pressure = capacity_signals / time_window

capacity_rate =
capacity_signals / total_signals

pressure_index =
(capacity_signals / window_duration)
× variance_factor

Output

capacity_rate
capacity_pressure_index

--------------------------------

Query 7 — Drift Trend Monitor

Purpose
Detect worsening or improving alignment.

Inputs

intent_id
time_series

Algorithm

compute slope of drift scores

Output

drift_trend

Possible results

improving
stable
degrading

--------------------------------

VI. Dashboard Views

Dashboards combine guidance queries.

Example panels:

Intent Alignment Dashboard
Displays alignment status for each intent.

Drift Trend Dashboard
Shows stability trends over time.

Capacity Pressure Map
Highlights areas reporting reduced capacity.

Intent Gravity Map
Shows dominant decision forces.

Tension Zone Map
Visualizes competing intents.

--------------------------------

VII. Design Principles

Signals are observations, not judgments.

Drift scores assist comparison but never decide legitimacy.

All signal history remains immutable.

Analysis always occurs through windows.

The engine remains unaware of signals and alignment logic.

--------------------------------

VIII. Boundary Reminder

Charter Engine Responsibilities

sessions
resolutions
authority
legitimacy
decision DAG

Guidance Layer Responsibilities

signals
drift analysis
alignment dashboards
intent gravity
field tension analysis

This separation keeps governance deterministic and interpretation flexible.

---

IX. Term Mapping for Humans

| Current Term      | Better Operator Term |
| ----------------- | -------------------- |
| Intent            | Goal                 |
| Intent lineage    | Goal chain           |
| Intent gravity    | Goal influence       |
| Intent field      | Goal landscape       |
| Intent energy     | Goal momentum        |
| Intent tension    | Goal friction        |
| Alignment cone    | Influence zone       |
| Alignment horizon | Influence limit      |
| Drift store       | Stability tracker    |
| Signal lattice    | Signal meaning model |

---

X. Example dashboards 

1. Goal Influence Map

Shows which goals dominate the system.

Example:

```
Goal Influence

Security         ██████████
Speed            █████████
Cost             █████
User Experience  ████
Compliance       ██
```

Operators instantly see:

```
what priorities dominate decisions
```

2. Stability Heatmap

Shows drift or volatility by area.

Example:
```
System Stability

Payments       🟢 stable
Search         🟡 volatile
Notifications  🔴 unstable
Analytics      🟢 stable
```

Color meaning:

```
green = stable
yellow = volatile
red = unstable
```

3. Goal Landscape Map

This visualizes tension zones.

Example:
```
          Security
               ▲
               │
Speed ◄──── Decisions ────► Cost
               │
               ▼
        User Experience
```

Clusters of volatility appear where goals compete.

This visualization is extremely powerful for humans.

