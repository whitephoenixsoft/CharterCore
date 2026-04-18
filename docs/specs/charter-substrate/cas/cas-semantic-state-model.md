# CAS — Semantic State Model
Status: FOUNDATIONAL (REVISED DRAFT)
Applies to: CAS Semantic Projection, CAS Views, CAS Explanations, CQL Semantic Outputs
Depends On: CAS Foundation Specification, CAS Semantic Vocabulary Specification, CCare Signal Model, CSG Structure Model, CIS Identity Model
Does NOT define: signal emission, numeric formulas, dynamic field computation, structural detection formulas, UI rendering

---

# 1. Purpose

This document defines the Semantic State Model for CAS.

It exists to:

- provide human-readable descriptions of system condition
- standardize semantic output across all CAS scopes
- define how semantic projection is formed from derived condition
- separate semantic explanation from structural and dynamic computation

Semantic State is the descriptive projection layer of CAS.

It is how CAS explains condition to humans in clear language.

---

# 2. Core Principle

> Semantic State describes condition. It does not compute condition, determine authority, or prescribe action.

Semantic State must remain:

- descriptive
- non-authoritative
- deterministic given inputs
- scope-aware
- human-readable

Semantic State is not a computational primitive.

It is a projection over already-derived condition.

---

# 3. Definition

A Semantic State is:

> a derived, human-readable description of system condition for a defined projection scope.

It is produced from:

- observational input already consumed by CAS
- derived condition produced by CAS
- propagated scoped condition
- relevant structural findings where appropriate

It is not produced directly from legitimacy, and it does not create legitimacy.

---

# 4. Position in CAS

Semantic State is one distinct part of CAS.

It exists downstream of derivation and separate from structural detection and dynamics.

CAS includes:

- intake and derivation
- propagation
- structural detection
- alignment dynamics
- semantic projection
- views
- query exposure

Semantic State belongs to semantic projection.

It must not be used as upstream truth for dynamics.

---

# 5. Semantic State as Projection

Semantic State is a projection layer.

This means:

- it summarizes condition
- it explains condition in natural language
- it does not perform the underlying calculation
- it does not replace numeric or structural analysis

The semantic layer explains.

The derivation, structural, and dynamics layers compute.

---

# 6. Projection Scopes

Semantic State must support multiple scopes.

Required scopes:

- node
- identity
- area
- graph

A Semantic State always exists for a defined scope.

A scope changes what is being described.

A scope does not change the meaning of the semantic vocabulary itself.

---

# 7. Semantic Bundle Model

Semantic State is not a single label.

It is a semantic bundle composed of dimensions.

These dimensions together form the human-readable description of condition.

## 7.1 Required Dimension

Every Semantic State must include:

- alignment posture

## 7.2 Optional Dimensions

A Semantic State may also include:

- capacity posture
- stability posture
- trend posture
- confidence posture
- interpretation phrase

Only meaningful dimensions should be included.

Semantic output should remain concise.

---

# 8. Semantic Dimensions

## 8.1 Alignment Posture

Alignment posture describes whether observed condition appears to match intent.

Allowed values:

- On track
- Off track
- Unclear

This dimension is required.

---

## 8.2 Capacity Posture

Capacity posture describes whether execution or sustainment appears constrained.

Allowed values:

- Healthy
- Under strain
- Limited
- Overloaded
- Paused

This dimension is optional.

---

## 8.3 Stability Posture

Stability posture describes whether behavior appears consistent over time.

Allowed values:

- Stable
- Unstable
- Fluctuating
- Volatile

This dimension is optional.

---

## 8.4 Trend Posture

Trend posture describes direction of change.

Allowed values:

- Improving
- Declining
- Holding
- Shifting

This dimension is optional.

---

## 8.5 Confidence Posture

Confidence posture describes reliability of the currently available condition.

Allowed values:

- High confidence
- Moderate confidence
- Low confidence
- Inconclusive

This dimension is optional.

---

## 8.6 Interpretation Phrase

Interpretation phrases describe recognizable patterns in condition or transition.

Allowed values include:

- Breaking down
- Losing capacity
- In conflict
- Going back and forth
- Stabilizing
- Recovering

This dimension is optional.

Interpretation phrases must remain descriptive and natural-language.

---

# 9. Composition Rules

## 9.1 Ordering

Semantic State should be composed in this order:

- alignment posture
- capacity posture
- stability posture
- trend posture
- confidence posture
- interpretation phrase

Example:

Off track, under strain, unstable, declining, high confidence

---

## 9.2 Conciseness

Only include dimensions that are materially useful.

Do not include dimensions merely because they are available.

Examples:

Valid:
- On track, stable
- Unclear, fluctuating
- Off track, declining, low confidence

Overloaded:
- On track, healthy, stable, holding, high confidence

---

## 9.3 Non-Contradiction

A Semantic State must not contain contradictory descriptors.

Invalid examples:

- Stable, volatile
- Improving, declining
- High confidence, inconclusive

The semantic projection layer must resolve contradiction before producing output.

---

## 9.4 Human Readability

Semantic State must be:

- readable by non-experts
- short
- precise
- natural language
- free of abbreviations and technical shorthand

---

# 10. Relationship to CCare Observational Input

CCare provides observational input to CAS.

These observations may include labels such as:

- alignment
- misalignment
- uncertainty
- reduced_capacity
- intentional_pause
- need_reassessment

These are input categories.

They are not final Semantic State outputs.

CAS may consume these observations and derive Semantic State from them, but Semantic State must remain a separate descriptive projection layer.

This distinction is mandatory.

---

# 11. Relationship to Derived Condition

Semantic State is formed from derived condition.

Derived condition may include:

- local aggregates
- scoped aggregates
- confidence-related measures
- propagated condition
- transition-sensitive derived state

Semantic State may summarize these conditions.

Semantic State must not be treated as equivalent to these underlying measures.

---

# 12. Relationship to Propagation

Propagation enables Semantic State to exist at multiple scopes.

Propagation operates on derived condition and scoped influence.

Semantic State is then projected for the resulting scope.

Propagation must not be understood as literal copying of human-readable labels through the graph.

Instead:

- condition is propagated or aggregated
- semantic description is projected at the relevant scope

This rule is foundational.

---

# 13. Relationship to Structural Detection

Structural detection is distinct from Semantic State.

Structural detection may identify:

- lineage-related conditions
- supersession-related patterns
- identity boundary effects
- graph-shape findings

These findings may inform semantic explanation where appropriate.

However:

- structural detection is not itself Semantic State
- Semantic State must not collapse structural findings into technical language

Semantic State may summarize structural significance in human-readable form, but must remain descriptive.

---

# 14. Relationship to Dynamics

Dynamics and Semantic State are distinct.

Dynamics computes:

- drift
- variance
- gravity
- tension
- cascades
- shock
- potential
- equilibrium
- other mathematical or field-like behavior

Semantic State may summarize the apparent meaning of these dynamics.

Examples:

- volatile
- declining
- in conflict
- stabilizing

However:

- Semantic State is not the dynamics layer
- Semantic State must not expose raw dynamic terminology by default
- Semantic State must not be used as an input basis for dynamic computation

This separation is mandatory.

---

# 15. Transition Semantics

Semantic State may express recognizable change patterns through interpretation phrases.

This allows CAS to describe condition transitions in everyday language.

Examples:

alignment to uncertainty to misalignment
may be described as:
- Breaking down

alignment to reduced capacity to pause
may be described as:
- Losing capacity

uncertainty to misalignment
may be described as:
- In conflict

These phrases are descriptive names for recognizable semantic patterns.

They are not formulas.

They are not predictive claims by themselves.

---

# 16. Silence and Inconclusiveness

Silence is valid.

Absence of observational input must not automatically be interpreted as:

- On track
- Off track
- Stable
- certain

When observations are insufficient, stale, sparse, or conflicting, CAS may produce:

- Unclear
- Inconclusive
- Low confidence

Semantic State must not overstate certainty.

---

# 17. Present-State Priority

Semantic State is primarily a present-state description.

Its primary purpose is to answer:

What appears true now?

Examples:

- On track, stable
- Under strain, unstable
- Unclear, low confidence

Semantic State may include trend-oriented descriptors such as Improving or Declining, but it remains a descriptive layer rather than a prediction engine.

---

# 18. Disallowed Terminology

Semantic State must not use technical or mathematical terminology intended for deeper analytic layers.

Disallowed categories include:

## 18.1 Mathematical Terms

- vector
- scalar
- magnitude
- gradient
- acceleration

## 18.2 Raw Dynamic Terms

- drift score
- variance
- potential
- attractor
- equilibrium state

## 18.3 Internal System Terms

- cone influence
- propagation index
- signal density
- field model

These concepts may exist elsewhere in CAS, but they must not appear as default Semantic State language.

---

# 19. Example Semantic States

Examples of valid Semantic State outputs:

- On track, stable
- Off track, declining
- Unclear, fluctuating, low confidence
- Under strain, unstable
- Off track, under strain, volatile, declining
- Paused, stable
- On track, recovering
- Unclear, inconclusive
- Off track, in conflict

---

# 20. Semantic State at Different Scopes

## 20.1 Node Scope

Describes condition for one node.

Example:

Off track, under strain

---

## 20.2 Identity Scope

Describes condition across an identity-scoped region.

Example:

Unclear, fluctuating, low confidence

---

## 20.3 Area Scope

Describes condition across an area.

Example:

Under strain, unstable, declining

---

## 20.4 Graph Scope

Describes broad system condition across the graph currently in scope.

Example:

On track, but shifting

The same vocabulary applies at every scope.

Only the projection target changes.

---

# 21. Design Guarantees

Semantic State must remain:

- descriptive
- deterministic
- concise
- scope-aware
- non-authoritative
- readable by non-experts

Given identical derived condition and scope, semantic outputs must be reproducible.

---

# 22. Mental Model

CAS computes condition in deeper layers.

Semantic State is how CAS says, in clear language, what that condition appears to be.

It is the first explanation layer for humans.

It exists so a person can understand condition without first reading structural findings or dynamic analysis.

---

# 23. Final Constraint

Semantic State exists to answer:

- What appears to be happening here?
- How should this condition be described in plain language?
- What concise summary best explains current condition at this scope?

Semantic State must never answer:

- What action is required
- Which decision is correct
- Who is at fault
- What humans must do next