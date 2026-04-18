# Charter Platform — Module Boundaries Specification
Status: FOUNDATIONAL (REVISED DRAFT)
Applies to: Charter Platform Modules, CAS, CSG, CIS, CCare, CQL, Views, Integrations
Depends On: Charter Legitimacy, CAS Foundation Specification, CSG Structure Model, CIS Identity Model, CCare Signal Model
Does NOT define: implementation sequencing, storage backends, UI rendering, numeric formulas, or CQL language syntax

---

# 1. Purpose

This document defines the canonical module boundaries for the Charter platform.

It exists to:

- separate platform responsibilities clearly
- prevent concept drift across modules
- define what each module owns
- define what each module may consume but not redefine
- prevent descriptive layers from becoming authoritative
- preserve clean interfaces between legitimacy, structure, observation, analysis, and querying

These boundaries are foundational.

If they are violated, the platform becomes conceptually unstable even if the software still runs.

---

# 2. Core Principle

> Each module owns one kind of truth or one kind of derived understanding.

Modules may depend on one another.

Modules must not absorb or redefine responsibilities that belong elsewhere.

The platform must preserve a hard distinction between:

- authoritative truth
- derived condition
- human-readable explanation
- read-only access

---

# 3. Platform Modules

The platform consists of the following primary modules:

- Charter
- CSG
- CIS
- CCare
- CAS
- CQL

These modules interact, but they do not collapse into each other.

---

# 4. Charter Boundary

Charter owns legitimacy.

Charter is the authoritative source for:

- resolutions
- sessions
- areas
- legitimacy history
- supersession lineage
- authority and scope validation

Charter answers questions such as:

- What was decided?
- Who had authority?
- What supersedes what?
- What is the legitimacy history?

Charter does not own:

- graph analysis
- identity overlays
- observational input
- semantic condition
- structural detection
- dynamic analysis
- query federation

Charter may be consumed by other modules.

Other modules must not change Charter truth.

---

# 5. CSG Boundary

CSG owns structure.

CSG is the authoritative source for:

- graph shape
- node relationships
- directed edges
- parent and child structure
- graph partitions
- graph-level topology
- structural paths relevant to analysis

CSG answers questions such as:

- How are nodes related?
- What is the graph shape?
- What paths and boundaries exist?
- What structural topology is present?

CSG does not own:

- legitimacy
- authority
- identity
- observations
- semantic condition
- dynamic field behavior
- read-only query federation

CSG provides the structural truth consumed by CAS and other modules.

Other modules must not redefine graph truth.

---

# 6. CIS Boundary

CIS owns identity.

CIS is the authoritative source for:

- identities
- identity versions
- identity boundaries
- scope overlays
- coexistence states
- deprecation and sunset-related identity truth

CIS answers questions such as:

- What identity does this region belong to?
- What identity boundaries apply here?
- What identity version is active?
- Where are coexistence or transition regions?

CIS does not own:

- legitimacy
- graph topology
- observational input
- semantic condition
- dynamic analysis
- query federation

CIS overlays identity truth onto structure.

Other modules may consume identity truth.

Other modules must not redefine identity truth.

---

# 7. CCare Boundary

CCare owns observational input.

CCare is the authoritative source for:

- check-ins
- observational labels
- confidence values
- timestamps
- optional observation context
- silence-valid observation behavior

CCare answers questions such as:

- What was observed?
- When was it observed?
- With what confidence was it reported?
- Was there silence or explicit input?

CCare observations are:

- descriptive
- optional
- non-authoritative
- not legitimacy

CCare does not own:

- legitimacy
- graph truth
- identity truth
- semantic projection
- structural detection
- dynamic analysis
- cross-substrate querying

CCare emits observational input.

CAS consumes it.

CCare must not compute final semantic state for CAS.

---

# 8. CAS Boundary

CAS owns condition derivation and explanation.

CAS is the derived substrate responsible for making the relationship between intent, observation, structure, and scope mechanically legible.

CAS owns:

- intake and derivation of condition
- scoped propagation
- structural detection
- alignment dynamics
- semantic projection
- CAS views
- CAS read-only query exposure

CAS answers questions such as:

- What appears to be happening?
- Is condition local or systemic?
- What structural patterns matter here?
- Is behavior stable or degrading?
- How should current condition be described in plain language?
- What read-only outputs should be queryable?

CAS does not own:

- legitimacy truth
- graph truth
- identity truth
- raw observation truth
- cross-substrate query language definition

CAS depends on:

- Charter for legitimacy
- CSG for structure
- CIS for identity
- CCare for observation

CAS must remain:

- observational
- derived
- deterministic
- rebuildable
- non-authoritative

CAS must never:

- create legitimacy
- change authority
- mutate graph truth
- mutate identity truth
- reinterpret CCare as authority
- convert description into obligation

---

# 9. CQL Boundary

CQL owns common query access across substrates.

CQL is the top-level query gateway.

CQL is responsible for:

- providing a unified query surface
- exposing cross-substrate read paths
- enabling structured access to outputs from multiple modules
- presenting substrate outputs through a common access model

CQL answers questions such as:

- How do I query CAS, Charter, CSG, CIS, or CCare outputs consistently?
- How are read-only results accessed across substrates?
- How are structured queries expressed across modules?

CQL does not own:

- legitimacy truth
- graph truth
- identity truth
- observational truth
- CAS computation
- structural detection logic
- semantic vocabulary
- dynamic formulas

CQL sits above substrates.

It provides access.

It does not replace substrate semantics.

---

# 10. CAS Internal Boundaries

CAS contains distinct internal parts.

These parts may be implemented together, but their conceptual responsibilities must remain separate.

## 10.1 Intake and Derivation

Owns:

- consumption of CCare observations
- normalization
- windowing
- local aggregation
- scoped derivation
- derived condition materialization

Does not own:

- final semantic wording
- structural pattern interpretation
- dynamic field explanation
- cross-substrate query language

---

## 10.2 Propagation

Owns:

- cones
- horizons
- scoped influence flow
- node-to-identity rollup
- node-to-area rollup
- graph-level scoped aggregation

Does not own:

- semantic wording
- structural truth itself
- dynamic formulas by itself

Propagation works on derived condition and scoped influence.

It must not be understood as literal copying of human-readable labels.

---

## 10.3 Structural Detection

Owns:

- graph-based significance detection
- lineage-related structural findings
- supersession-related structural findings
- identity-boundary structural findings
- structural concentration, isolation, fragmentation, and exposure findings

Does not own:

- dynamic field behavior
- final semantic bundle outputs
- view rendering

Structural Detection is an analytical dimension inside CAS.

It is not merely a view.

---

## 10.4 Alignment Dynamics

Owns:

- drift
- variance
- stability-related measures
- gravity
- tension
- shock
- cascades
- potential
- equilibrium tendency
- temporal and predictive condition analysis

Does not own:

- raw observation truth
- graph truth
- identity truth
- semantic wording
- display rendering

Dynamics may consume derived condition and structural scope.

Dynamics must not use final semantic projection as upstream truth.

---

## 10.5 Semantic Projection

Owns:

- semantic bundle generation
- scope-aware human-readable condition
- transition naming in plain language
- concise descriptive summaries for humans

Does not own:

- legitimacy
- structural truth
- dynamic computation
- observation truth
- commands or recommendations

Semantic Projection is a projection layer only.

It explains.

It does not compute underlying field behavior.

---

## 10.6 Views

Owns:

- read-only projections over CAS outputs
- focus views
- trend views
- structural views
- semantic views
- dynamic views
- explanation-oriented combinations of CAS outputs

Does not own:

- authoritative truth
- base derivation
- structural detection logic
- dynamic formulas

Views are derived projections.

They do not mutate state.

---

## 10.7 CAS Query Exposure

Owns:

- read-only substrate-specific access to CAS outputs
- structured result exposure to CQL and other consumers

Does not own:

- cross-substrate query language
- upstream truth
- computation logic itself

CAS query exposure defines what CAS makes available.

CQL defines how common cross-substrate access is expressed.

---

# 11. Boundary Rules Between Modules

## 11.1 Charter to CAS

Charter provides legitimacy truth.

CAS may consume legitimacy truth.

CAS must not modify or reinterpret legitimacy into authority.

---

## 11.2 CSG to CAS

CSG provides graph truth.

CAS may consume graph truth for propagation and structural analysis.

CAS must not redefine graph topology.

---

## 11.3 CIS to CAS

CIS provides identity truth.

CAS may consume identity truth for scoped analysis.

CAS must not redefine identity boundaries or versions.

---

## 11.4 CCare to CAS

CCare provides observational input.

CAS may derive condition from observations.

CAS must not treat observations as authority.

CCare must not be treated as the source of final semantic projection.

---

## 11.5 CAS to CQL

CAS provides read-only derived outputs.

CQL may expose them through common query access.

CQL must not change CAS meaning.

---

# 12. Truth Class Distinction

The platform must preserve distinct truth classes.

## 12.1 Authoritative Truth

Owned by:

- Charter
- CSG
- CIS
- CCare

These are source truths.

---

## 12.2 Derived Condition

Owned by:

- CAS

This is rebuildable and non-authoritative.

---

## 12.3 Human-Readable Explanation

Owned by:

- CAS semantic projection
- CAS views

This is descriptive and downstream of derivation.

---

## 12.4 Read-Only Access

Owned by:

- CQL
- substrate query exposure layers

This is access, not truth.

---

# 13. Prohibited Boundary Violations

The following violations are forbidden.

## 13.1 Charter Violation

Any module other than Charter creating or mutating legitimacy.

---

## 13.2 Structure Violation

Any module other than CSG redefining graph truth.

---

## 13.3 Identity Violation

Any module other than CIS redefining identity truth.

---

## 13.4 Observation Violation

Any module other than CCare redefining observational source truth.

---

## 13.5 Semantic Violation

Any module treating semantic wording as upstream truth for dynamic computation.

---

## 13.6 Query Violation

Any query layer redefining the meaning of substrate outputs.

---

## 13.7 Authority Violation

Any descriptive or analytic module converting visibility into command, obligation, or authority.

---

# 14. Mental Model

Charter defines what was legitimately decided.

CSG defines how relevant entities are structurally related.

CIS defines who or what structural regions claim to be.

CCare defines what has been observed.

CAS derives, propagates, detects, analyzes, and explains condition across those realities.

CQL provides the common way to query the results.

Each module has one job.

The platform remains stable when those jobs remain separate.

---

# 15. Why This Matters

Without strong module boundaries:

- descriptive layers become authoritative
- observations get confused with truth
- structure and identity get redefined in downstream systems
- query surfaces begin to distort semantics
- implementation convenience replaces conceptual correctness

With strong boundaries:

- each module remains understandable
- derived outputs remain trustworthy
- explanations stay non-coercive
- cross-substrate access stays clean
- revisions remain possible without conceptual collapse

---

# 16. Final Constraint

This document exists to answer:

- What does each module own?
- What may each module consume?
- What must each module never redefine?
- How do the platform modules remain conceptually stable together?

It must never be used to justify:

- collapsing module responsibilities
- treating descriptive outputs as authority
- replacing substrate truth with convenience abstractions