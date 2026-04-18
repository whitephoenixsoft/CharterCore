CAS — Semantic Vocabulary Specification

Status: FOUNDATIONAL (DRAFT)
Applies to: CAS Semantic State Model, Views, CQL Outputs
Depends On: CCare Signal Model, CAS Derivation + Propagation + Dynamics
Does NOT define: signal emission, numeric computation, enforcement logic

---

1. Purpose

This document defines the canonical semantic vocabulary used by CAS.

It exists to:

- provide clear, human-readable descriptions of system condition
- unify semantic language across all CAS outputs
- replace technical or mathematical terminology with natural language
- ensure consistency across nodes, identities, areas, and global views

This vocabulary is the human interface layer of CAS.

---

2. Core Principle

«Semantic vocabulary describes condition. It does not compute, infer authority, or prescribe action.»

All semantic expressions must remain:

- descriptive
- non-authoritative
- deterministic given inputs
- readable without domain expertise

---

3. Vocabulary Model

Semantic output is not a single label.

It is a composable bundle of dimensions.

Each dimension answers a different question about system condition.

---

3.1 Semantic Bundle Structure

A semantic description is composed of:

- alignment posture
- capacity posture (optional)
- stability posture (optional)
- trend posture (optional)
- confidence posture (optional)
- interpretation phrase (optional)

---

3.2 Output Format

Semantic outputs should follow:

Alignment + optional qualifiers

Examples:

On track, stable
Off track, declining
Under strain, unstable, low confidence
Unclear, fluctuating

---

4. Alignment Posture (Primary Dimension)

Describes whether behavior matches intent.

Required for all semantic outputs.

Allowed values:

- On track
- Off track
- Unclear

---

5. Capacity Posture

Describes ability to execute or sustain activity.

Optional.

Allowed values:

- Healthy
- Under strain
- Limited
- Overloaded
- Paused

Notes:

- "Paused" reflects intentional pause or inactive state
- Capacity posture should not imply correctness

---

6. Stability Posture

Describes consistency of behavior over time.

Optional.

Allowed values:

- Stable
- Unstable
- Fluctuating
- Volatile

Notes:

- Stable indicates consistent signals
- Volatile indicates rapid or high-frequency change

---

7. Trend Posture

Describes direction of change over time.

Optional.

Allowed values:

- Improving
- Declining
- Holding
- Shifting

Notes:

- Trend is derived from observed change, not prediction certainty
- Shifting indicates directional inconsistency

---

8. Confidence Posture

Describes reliability of observations.

Optional.

Allowed values:

- High confidence
- Moderate confidence
- Low confidence
- Inconclusive

Notes:

- Confidence reflects signal density, consistency, and recency
- Inconclusive indicates insufficient or conflicting data

---

9. Interpretation Phrases (Transition Semantics)

Describes recognizable patterns or transitions in behavior.

Optional.

Used to replace technical or mathematical descriptions.

Allowed phrases:

- Breaking down
- Losing capacity
- In conflict
- Going back and forth
- Stabilizing
- Recovering

Notes:

- These are derived from semantic transitions
- They must remain natural language and non-technical
- They must not introduce mathematical or physical terminology

---

10. Composition Rules

10.1 Ordering

Semantic output should follow:

Alignment → Capacity → Stability → Trend → Confidence → Interpretation

Example:

Off track, under strain, unstable, declining, high confidence

---

10.2 Conciseness

Only include dimensions that are meaningful.

Do not include redundant or neutral descriptors.

Example:

On track, stable
NOT
On track, stable, holding, high confidence (unless required)

---

10.3 Non-Contradiction

Dimensions must not contradict each other.

Example:

Invalid:

Stable, volatile

Valid:

Fluctuating (used instead of conflicting terms)

---

10.4 Human Readability

All outputs must:

- be understandable without technical knowledge
- avoid abbreviations
- avoid domain-specific jargon
- avoid mathematical language

---

11. Disallowed Terminology

The following categories must not appear in semantic outputs:

11.1 Mathematical Terms

- vector
- magnitude
- gradient
- scalar
- acceleration

11.2 Technical System Terms

- drift score
- variance
- signal density
- propagation index

11.3 Abstract Model Terms

- attractor
- potential field
- equilibrium state

These belong to the dynamics layer, not semantic output.

---

12. Relationship to CCare Signals

CCare emits observational inputs such as:

- alignment
- misalignment
- uncertainty
- reduced_capacity
- intentional_pause
- need_reassessment

These are input categories, not final semantic outputs.

CAS semantic vocabulary:

- transforms and extends these inputs
- combines them with derived condition
- produces human-readable descriptions

---

13. Scope Awareness

Semantic outputs must support multiple projection scopes:

- resolution (node)
- identity
- area
- global graph

The same vocabulary must be usable at all scopes.

---

14. Examples

Example 1

On track, stable

Example 2

Off track, declining

Example 3

Under strain, unstable, low confidence

Example 4

Unclear, fluctuating

Example 5

Off track, under strain, unstable, declining, high confidence

Example 6

Paused, stable, awaiting review

---

15. Final Constraint

Semantic vocabulary exists to answer:

What does the system appear to be doing?

It must never answer:

What should be done?

All decision-making remains outside CAS semantic output.