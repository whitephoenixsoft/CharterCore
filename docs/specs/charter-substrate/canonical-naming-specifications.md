# Charter Platform — Canonical Naming Specification
Status: FOUNDATIONAL (REVISED DRAFT)
Applies to: Charter, CAS, CSG, CIS, CCare, CQL, Views, Explanations, Integrations
Depends On: Charter Platform Module Boundaries Specification, CAS Foundation Specification, CAS Semantic Vocabulary Specification
Does NOT define: UI copy style, implementation-specific class names, storage schemas, or query syntax

---

# 1. Purpose

This document defines the canonical naming rules for the Charter platform.

It exists to:

- preserve conceptual precision across documents
- prevent term drift across modules
- distinguish authoritative truth from derived outputs
- ensure the same term is not reused for different layers
- ensure human-facing language and system-facing language remain compatible without collapsing into each other

Canonical naming is a platform stability requirement.

If naming drifts, boundaries drift with it.

---

# 2. Core Principle

> One canonical concept should have one canonical meaning within the platform.

A term may appear across modules only if:

- the meaning remains the same
- the ownership remains clear
- the term does not collapse distinct layers

If two things are not the same, they must not share the same canonical name.

---

# 3. Naming Priorities

Canonical naming must prioritize:

1. conceptual precision
2. boundary clarity
3. deterministic interpretation
4. human readability where possible
5. implementation convenience last

A convenient name that blurs layers is not acceptable.

---

# 4. Naming Classes

The platform uses four naming classes.

## 4.1 Foundational Terms

These define core platform concepts.

Examples:

- legitimacy
- structure
- identity
- observation
- condition
- semantic projection
- dynamics
- scope

These require the highest stability.

---

## 4.2 Module Terms

These define module-owned concepts.

Examples:

- resolution
- check-in
- identity version
- structural finding
- semantic state
- alignment dynamics
- CQL query

These must remain consistent with module boundaries.

---

## 4.3 Human-Facing Terms

These are terms intended for human-readable outputs.

Examples:

- On track
- Off track
- Under strain
- Unclear
- Stable
- Declining

These must remain descriptive and non-technical.

---

## 4.4 Implementation Terms

These are code-facing or schema-facing names.

Examples:

- field names
- API parameter names
- JSON property names
- internal type names

Implementation terms may vary, but they must map cleanly to canonical concepts.

Implementation convenience must not redefine canonical meaning.

---

# 5. Canonical Naming Rules

## 5.1 Single Meaning Rule

A canonical term must not carry multiple unrelated meanings.

Example:

The same term must not mean both a raw observation input and a projected semantic output.

---

## 5.2 Ownership Rule

Each canonical concept must have a primary owner.

Example:

- legitimacy is owned by Charter
- structure is owned by CSG
- identity is owned by CIS
- observational input is owned by CCare
- derived condition is owned by CAS
- cross-substrate query access is owned by CQL

---

## 5.3 Layer Separation Rule

A term must not blur:

- authoritative truth
- derived condition
- semantic explanation
- read-only access

If a term would collapse these layers, it must be split or renamed.

---

## 5.4 Human-Language Rule

Human-facing names should be:

- short
- precise
- natural language
- readable by non-experts

Human-facing names must avoid technical and mathematical language unless the document explicitly belongs to an analytic layer.

---

## 5.5 Stability Rule

Foundational terms should change rarely.

If a foundational term is revised, all dependent documents must be reviewed.

---

# 6. Canonical Module Names

The canonical platform module names are:

- Charter
- CAS
- CSG
- CIS
- CCare
- CQL

These names must be used consistently in platform architecture documents.

They must not be casually replaced with partial synonyms.

---

# 7. Canonical Ownership Table

## 7.1 Charter

Owns:

- legitimacy
- resolution
- session
- area
- supersession lineage
- authority

Canonical terms associated with Charter must refer to legitimacy and explicit decision history.

---

## 7.2 CSG

Owns:

- structure
- graph
- node relationship
- edge
- path
- topology
- partition

Canonical terms associated with CSG must refer to graph truth.

---

## 7.3 CIS

Owns:

- identity
- identity version
- identity boundary
- identity scope
- coexistence
- deprecation-related identity truth
- sunset-related identity truth

Canonical terms associated with CIS must refer to identity truth.

---

## 7.4 CCare

Owns:

- check-in
- observational input
- observational label
- confidence
- timestamp
- silence-valid observation

Canonical terms associated with CCare must refer to observation truth.

CCare does not own final semantic projection naming.

---

## 7.5 CAS

Owns:

- derived condition
- propagation
- structural detection
- structural finding
- alignment dynamics
- semantic projection
- semantic state
- semantic bundle
- view
- CAS query exposure
- alignment state store

Canonical terms associated with CAS must refer to derived condition and explanation.

---

## 7.6 CQL

Owns:

- common query access
- cross-substrate query gateway
- query expression
- read-only query access model

Canonical terms associated with CQL must refer to access, not substrate semantics.

---

# 8. Canonical Distinctions

This section defines required distinctions between commonly confused terms.

## 8.1 Check-In vs Semantic State

### Check-In
Owned by: CCare

Definition:

A check-in is an observational input record emitted by CCare.

A check-in may include:

- observational label
- confidence
- timestamp
- target
- optional context

A check-in is source observation truth.

It is not a CAS semantic output.

---

### Semantic State
Owned by: CAS

Definition:

A Semantic State is a derived, human-readable description of condition for a defined scope.

A Semantic State is a projection layer output.

It is not a raw check-in.

---

### Rule

The terms check-in and semantic state must never be treated as interchangeable.

This distinction is mandatory.

---

## 8.2 Observational Label vs Semantic Vocabulary

### Observational Label
Owned by: CCare

Examples:

- alignment
- misalignment
- uncertainty
- reduced_capacity
- intentional_pause
- need_reassessment

These are input categories.

---

### Semantic Vocabulary
Owned by: CAS

Examples:

- On track
- Off track
- Unclear
- Under strain
- Stable
- Declining
- Low confidence
- Recovering

These are human-readable projection terms.

---

### Rule

Observational labels are not final semantic vocabulary.

CAS may derive semantic vocabulary from observations, but the two naming systems must remain distinct.

---

## 8.3 Structural Detection vs Dynamics

### Structural Detection
Owned by: CAS

Definition:

Structural Detection identifies graph-based or identity-based significance.

Examples:

- structural bottleneck
- structural isolation
- structural fragmentation
- boundary-crossing region
- dense supersession branch

---

### Dynamics
Owned by: CAS

Definition:

Dynamics computes mathematical or field-like condition behavior.

Examples:

- drift
- variance
- gravity
- tension
- shock
- cascade
- potential
- equilibrium tendency

---

### Rule

Structural Detection and Dynamics must never be treated as the same analytical dimension.

---

## 8.4 Semantic State vs View

### Semantic State
Owned by: CAS semantic projection

Definition:

A semantic description of condition.

---

### View
Owned by: CAS views

Definition:

A read-only projection or arrangement of outputs for interpretation.

A view may include semantic state, structural findings, and dynamics together.

---

### Rule

A Semantic State is not a view.

A view may contain semantic state.

---

## 8.5 Query Exposure vs Query Language

### CAS Query Exposure
Owned by: CAS

Definition:

The read-only outputs CAS makes available.

---

### CQL
Owned by: CQL

Definition:

The common cross-substrate query gateway and access model.

---

### Rule

CAS defines what it exposes.

CQL defines how common query access is expressed.

The two must not be collapsed.

---

# 9. Canonical Semantic Naming

CAS human-facing semantic naming must use the canonical semantic vocabulary.

## 9.1 Required Alignment Posture Terms

- On track
- Off track
- Unclear

These are the canonical primary semantic posture terms.

---

## 9.2 Optional Capacity Terms

- Healthy
- Under strain
- Limited
- Overloaded
- Paused

---

## 9.3 Optional Stability Terms

- Stable
- Unstable
- Fluctuating
- Volatile

---

## 9.4 Optional Trend Terms

- Improving
- Declining
- Holding
- Shifting

---

## 9.5 Optional Confidence Terms

- High confidence
- Moderate confidence
- Low confidence
- Inconclusive

---

## 9.6 Optional Interpretation Phrases

- Breaking down
- Losing capacity
- In conflict
- Going back and forth
- Stabilizing
- Recovering

These phrases are semantic outputs.

They are not mathematical terms.

---

# 10. Disallowed Semantic Naming

The following terms are disallowed as default semantic state vocabulary.

## 10.1 Mathematical Terms

- vector
- scalar
- magnitude
- gradient
- acceleration

---

## 10.2 Raw Dynamic Terms

- drift score
- variance
- attractor
- potential field
- equilibrium state

---

## 10.3 Internal System Terms

- cone influence
- propagation index
- signal density
- tension field

These may exist in deeper analytic layers, but they must not be used as default human-facing semantic state names.

---

# 11. Canonical Structural Naming

Structural Detection should use names that describe structure directly.

Preferred structural naming patterns include:

- structurally isolated
- structurally exposed
- structurally concentrated
- structurally fragmented
- boundary-crossing
- transition-sensitive
- dense supersession branch
- dependency bottleneck
- contained branch

Structural names should remain:

- descriptive
- graph-oriented
- non-authoritative

They should not sound like commands, diagnoses of blame, or dynamic formulas.

---

# 12. Canonical Dynamic Naming

Dynamic terms may be technical when used in dynamic-specific documents and views.

Allowed dynamic terms include:

- drift
- variance
- gravity
- tension
- shock
- cascade
- potential
- equilibrium tendency

These terms belong to the dynamics layer.

They should not be reused as default semantic state vocabulary.

---

# 13. Canonical Scope Terms

The required canonical scope terms are:

- node
- identity
- area
- graph

These should be used consistently across CAS documents.

Avoid introducing unnecessary synonyms such as:

- unit
- region
- domain
- zone

unless a document explicitly defines a more specific concept.

---

# 14. Preferred vs Discouraged Naming

## 14.1 Preferred

- check-in
- observational label
- semantic state
- semantic bundle
- structural finding
- alignment dynamics
- view
- query exposure
- query gateway
- derived condition

These preserve distinctions cleanly.

---

## 14.2 Discouraged

- signal state
- semantic signal
- dynamic state
- graph health
- field state
- alignment status engine
- semantic vector
- influence score as semantic wording

These tend to blur layers or meanings.

---

# 15. Naming Migration Rules

When revising legacy documents:

## 15.1 Keep the concept, revise the term

If a legacy term is structurally correct but poorly named, preserve the concept and replace the label.

---

## 15.2 Split overloaded terms

If a term has been used for two layers, split it into two canonical terms.

Example:

- observational label
- semantic state

rather than one overloaded state term.

---

## 15.3 Preserve human readability

When choosing between two technically acceptable names, prefer the one a non-expert can understand unless technical precision would be lost.

---

## 15.4 Update dependent documents together

If a foundational term is changed, dependent documents must be revised in the same pass or queued immediately after.

---

# 16. Mental Model

Canonical naming keeps the platform stable by ensuring:

- authoritative truth has one name
- derived condition has another
- explanation has another
- access has another

Names are not decoration.

Names define the edges between concepts.

If names blur, the architecture blurs.

---

# 17. Final Constraint

This document exists to answer:

- What should each platform concept be called?
- Which terms are canonical?
- Which terms must remain distinct?
- Which names are allowed for human-facing semantic output?
- Which legacy naming patterns should be retired?

It must never be used to justify:

- reusing one term for multiple layers
- replacing conceptual precision with convenience
- collapsing module boundaries through naming shortcuts 