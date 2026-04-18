# CAS — Foundation Specification
Status: FOUNDATIONAL (REVISED DRAFT)
Applies to: CAS Substrate, CAS Semantic State Model, CAS Structural Detection, CAS Views, CQL Integration
Depends On: Charter Legitimacy, CSG Structure Model, CIS Identity Model, CCare Signal Model
Does NOT define: signal emission mechanics, numeric formulas, storage backend implementation, UI rendering, or CQL language design

---

# 1. Purpose

This document defines the foundation of CAS as a full substrate.

CAS exists to make the relationship between:

- declared intent
- observed condition
- structural scope

mechanically legible without changing legitimacy, identity, or authority.

CAS is the substrate responsible for:

- consuming observational input
- deriving alignment-related condition
- propagating scoped influence
- detecting structural patterns
- computing alignment dynamics
- projecting human-readable semantic condition
- exposing queryable outputs

CAS is observational, derived, deterministic, and non-authoritative.

---

# 2. Core Principle

> CAS describes and analyzes condition. It does not create legitimacy, identity, authority, or obligation.

All CAS outputs must remain:

- descriptive
- non-authoritative
- deterministic given inputs
- rebuildable from upstream truth
- separate from enforcement and decision-making

---

# 3. Position in the Platform

CAS is not Charter, CSG, CIS, or CCare.

CAS depends on them.

## 3.1 Charter
Charter defines explicit legitimacy.

It provides:

- resolutions
- sessions
- areas
- supersession lineage
- legitimacy history

CAS does not create or alter legitimacy.

---

## 3.2 CSG
CSG defines graph structure.

It provides:

- node relationships
- DAG structure
- reference paths
- graph-level topology needed for propagation and structural analysis

CAS does not define graph truth.
CAS consumes graph truth.

---

## 3.3 CIS
CIS defines identity scope.

It provides:

- identities
- identity versions
- identity boundaries
- scoped identity overlays across graph regions

CAS does not define identity.
CAS consumes identity truth.

---

## 3.4 CCare
CCare provides observational input.

It emits check-ins and related observation records.

These inputs are:

- descriptive
- optional
- non-authoritative
- silence-valid

CAS does not emit CCare check-ins.
CAS consumes them.

---

## 3.5 CQL
CQL is the common query gateway above substrates.

CAS exposes queryable outputs through CQL, but CQL is not part of CAS computational semantics.

CAS defines what can be queried.
CQL defines how cross-substrate querying is expressed.

---

# 4. Primary Inputs

CAS consumes three primary input classes.

## 4.1 Observational Input (CCare)

Observational input includes check-ins and related observation records.

These may include:

- observational state labels
- confidence
- timestamp
- target reference
- optional context

Observational input is never legitimacy.

It is evidence of observed condition only.

---

## 4.2 Structural Input (CSG)

Structural input includes:

- graph topology
- lineage relationships
- supersession edges
- node references
- area membership
- graph partitions

Structural input defines the shape within which CAS operates.

---

## 4.3 Identity Input (CIS)

Identity input includes:

- identity boundaries
- identity versions
- identity-scoped regions
- transition and coexistence context

Identity input determines scope overlays used in analysis.

---

# 5. Core Output Classes

CAS produces four distinct classes of output.

## 5.1 Derived Condition
Derived condition includes normalized and aggregated condition produced from inputs.

Examples:

- signal density
- recency-aware counts
- local aggregates
- scoped aggregates
- confidence-derived measures

This is intermediate derived state.

---

## 5.2 Structural Findings
Structural findings describe graph- and identity-based conditions.

Examples:

- supersession-related regions
- lineage bends
- boundary crossings
- structurally isolated regions
- concentration or dependency patterns

These findings are not dynamic field outputs.

---

## 5.3 Dynamic Findings
Dynamic findings describe mathematical or field-like behavior over time and scope.

Examples:

- drift
- variance
- gravity
- tension
- cascade emergence
- shock propagation
- potential
- equilibrium tendency

These belong to the dynamics dimension of CAS.

---

## 5.4 Semantic Projections
Semantic projections are human-readable descriptions of condition.

They are produced for:

- node scope
- identity scope
- area scope
- graph scope

Semantic projections are descriptive outputs for human understanding.

They are not computational primitives.

---

# 6. Internal CAS Decomposition

CAS contains distinct internal parts.

These parts may be implemented together, but they must remain conceptually separate.

## 6.1 Intake and Derivation
Consumes observational, structural, and identity inputs and derives base condition.

This part is responsible for:

- intake of CCare observations
- normalization
- windowing
- local aggregation
- scoped derivation
- derived state materialization

This layer does not produce final semantic explanation by itself.

---

## 6.2 Propagation
Propagates influence and derived condition through structure and scope.

This part is responsible for:

- alignment cones
- alignment horizons
- ancestor and descendant influence
- node to identity rollup
- node to area rollup
- graph-level scoped aggregation

Propagation must operate on derived condition and scoped influence.

Propagation must not be understood as literal copying of human-readable labels through the graph.

---

## 6.3 Structural Detection
Detects graph-based and identity-based patterns independent of field dynamics.

This part is responsible for:

- lineage pattern detection
- supersession-related structure detection
- boundary-aware structural findings
- graph-shape interpretation relevant to alignment understanding

Structural detection may use graph and identity truth.

Structural detection must not require dynamic field formulas.

---

## 6.4 Alignment Dynamics
Computes mathematical and field-like behavior.

This part is responsible for:

- drift
- variance
- stability-related measures
- gravity
- tension
- cascades
- shock
- potential
- equilibrium
- predictive and temporal behavior

Dynamics may use:

- observational input
- derived condition
- propagation results
- structural scope

Dynamics must not depend on final human-readable semantic projection as upstream truth.

---

## 6.5 Semantic Projection
Projects derived condition into human-readable system condition.

This part is responsible for:

- semantic bundle generation
- semantic transition naming
- descriptive posture across scopes
- operator-facing summary language

Semantic projection is downstream of derivation.

Semantic projection is not the basis for dynamic computation.

Its purpose is explanation, not calculation.

---

## 6.6 Views
Views organize CAS outputs for reading and interpretation.

Views may combine:

- semantic projection
- structural findings
- dynamic findings
- scoped state

Views are read-only derived projections.

They do not mutate CAS state.

---

## 6.7 Query Exposure
CAS exposes queryable outputs to CQL and other read-only consumers.

Query exposure is not the same as computation.

It provides access to derived and projected CAS results.

---

# 7. The Semantic Rule

> Semantic output is a projection layer, not a computational primitive.

This rule is foundational.

It means:

- semantic output describes what appears true
- semantic output does not drive dynamic computation
- semantic output does not create authority
- semantic output does not prescribe action

CAS may compute semantic projection from derived condition.

CAS must not compute dynamics from semantic explanation text or semantic bundle outputs.

---

# 8. The Dynamics Rule

> Dynamics is the mathematical and field-analysis dimension of CAS.

Dynamics includes all formula-based or field-based analysis.

Examples include:

- drift
- variance
- trend measures
- gravity
- tension
- shock
- cascade
- potential
- equilibrium

Anything that depends on arithmetic, weighting, thresholding, or temporal field analysis belongs to dynamics.

Dynamics explains behavior more deeply than semantic projection.

Dynamics is not the same thing as semantic status.

---

# 9. The Structural Rule

> Structural detection is separate from dynamics.

Structural detection answers:

- how condition is organized in graph form
- where structure itself introduces significance
- how lineage, supersession, and identity boundaries shape interpretation

Structural detection is not merely a view.

It is a distinct analytical dimension inside CAS.

Structural detection may inform semantic projection and views.

Structural detection must remain conceptually separate from field math.

---

# 10. The Scope Rule

CAS must support multiple projection scopes without changing core semantics.

Required scopes:

- node
- identity
- area
- graph

The same foundational rules apply at every scope.

CAS may derive different scoped outputs from the same upstream truth, but scope must never change legitimacy semantics.

---

# 11. The Silence Rule

Silence is valid.

Absence of check-ins is not automatically equivalent to alignment, misalignment, or certainty.

CAS may derive silence-related or inconclusive condition where appropriate, but must not treat missing observations as proof of correctness.

---

# 12. Present vs Predictive Distinction

CAS must preserve a hard distinction between present description and predictive behavior.

## 12.1 Present
Present outputs answer:

What appears true now?

Examples:

- current semantic posture
- current scoped condition
- current structural findings
- current drift or variance

---

## 12.2 Predictive
Predictive outputs answer:

Where does this appear to be moving?

Examples:

- drift velocity
- drift acceleration
- cascade emergence
- shock risk
- transition risk
- equilibrium tendency

Predictive outputs are observational projections only.

They are not commitments.

---

# 13. Numeric vs Semantic Distinction

CAS uses both numeric and semantic forms, but they are not interchangeable.

## 13.1 Numeric
Used for:

- arithmetic
- weighting
- rolling windows
- aggregation
- propagation math
- thresholds
- field analysis

---

## 13.2 Semantic
Used for:

- human-readable condition
- semantic bundle outputs
- transition phrases
- descriptive explanation
- operator-facing summary

The numeric dimension computes.

The semantic dimension explains.

Neither replaces the other.

---

# 14. Alignment State Store

CAS maintains a derived state store for efficient query and projection.

This store is:

- derived
- rebuildable
- non-authoritative
- query-oriented
- append-aware

Loss of the store must not lose truth.

Truth remains upstream in legitimacy, structure, identity, and observation sources.

---

# 15. Boundary Guarantees

CAS may:

- consume observations
- normalize
- derive
- propagate
- detect structure
- compute dynamics
- project semantic condition
- materialize derived state
- expose read-only queries

CAS may never:

- create legitimacy
- modify resolutions
- change authority
- mutate identity truth
- enforce action
- infer obligation from visibility
- suppress disagreement
- convert description into command

---

# 16. Design Guarantees

CAS must remain:

- observational
- deterministic
- derived
- rebuildable
- backend-agnostic
- scope-aware
- non-authoritative

Given identical upstream truth and context, CAS outputs must be reproducible.

---

# 17. Mental Model

Charter defines what was explicitly decided.

CSG defines how relevant things are structurally related.

CIS defines who or what the scoped structure claims to be.

CCare defines what is being observed.

CAS derives, propagates, detects, analyzes, and explains how those realities relate.

CQL provides the common way to query those results.

---

# 18. Why This Matters

Without CAS:

- declared intent remains disconnected from observed condition
- graph structure remains underused for analysis
- identity remains disconnected from condition
- observations remain hard to interpret across scope
- deeper instability remains difficult to detect

With CAS:

- condition becomes mechanically legible
- semantic explanation becomes consistent
- structural context becomes visible
- dynamic behavior becomes analyzable
- cross-scope understanding becomes queryable without coercion

CAS exists to make condition understandable without taking agency away from humans.

---

# 19. Final Constraint

CAS exists to answer questions such as:

- What appears to be happening?
- Where is condition concentrated?
- Is instability local or systemic?
- What structural patterns matter here?
- How is this changing over time?
- What descriptive summary best explains current condition?

CAS must never answer:

- Which decision is correct
- What action is required
- Who is authoritative
- What humans must do next