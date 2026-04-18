# CAS — Structural Detection Specification
Status: FOUNDATIONAL (REVISED DRAFT)
Applies to: CAS Structural Detection, CAS Views, CAS Explanations, CQL Structural Outputs
Depends On: CAS Foundation Specification, CSG Structure Model, CIS Identity Model, Charter Legitimacy
Does NOT define: signal emission, numeric field formulas, dynamic propagation formulas, UI rendering, or CQL language design

---

# 1. Purpose

This document defines Structural Detection within CAS.

Structural Detection exists to identify and describe graph-based and identity-based conditions that matter for understanding system condition.

It exists to:

- detect meaningful structural patterns in the graph
- reveal how lineage, supersession, and scope shape interpretation
- provide structural findings that support semantic explanation and deeper analysis
- remain distinct from both semantic projection and alignment dynamics

Structural Detection is a first-class analytical dimension inside CAS.

It is not merely a view.

It is not the dynamics layer.

---

# 2. Core Principle

> Structural Detection identifies condition that arises from structure itself.

Structural Detection must remain:

- descriptive
- deterministic given structural inputs
- non-authoritative
- scope-aware
- separate from dynamic field computation

Structural Detection does not decide what should be done.

It reveals what is structurally significant.

---

# 3. Definition

A structural finding is:

> a derived description of graph-based or identity-based significance detected from structure and scope.

Structural findings are produced from:

- graph truth from CSG
- legitimacy lineage from Charter
- identity truth from CIS
- scope-aware structural relationships

Structural findings are not legitimacy.

Structural findings do not create authority or obligation.

---

# 4. Position in CAS

Structural Detection is one distinct part of CAS.

CAS includes:

- intake and derivation
- propagation
- structural detection
- alignment dynamics
- semantic projection
- views
- query exposure

Structural Detection is separate from:

- semantic projection
- alignment dynamics
- read-only views

It may inform them, but it is not reducible to them.

---

# 5. Structural Detection Inputs

Structural Detection consumes structural and scope truth.

## 5.1 Charter Inputs

Charter provides legitimacy-related structure such as:

- resolutions
- lineage
- supersession history
- area membership
- legitimacy boundaries

These inputs provide the decision history within which structural significance is detected.

---

## 5.2 CSG Inputs

CSG provides graph truth such as:

- node relationships
- directed edges
- parent and child structure
- references
- graph topology
- partitions and subgraphs

Structural Detection operates over this graph truth.

---

## 5.3 CIS Inputs

CIS provides identity truth such as:

- identities
- identity versions
- scope boundaries
- coexistence regions
- identity transitions

These inputs allow structure to be analyzed relative to claimed identity boundaries.

---

# 6. Structural Detection Scope

Structural Detection must support multiple scopes.

Required scopes:

- node
- identity
- area
- graph

Structural findings must always be understood relative to a defined scope.

The same structure may yield different findings at different scopes.

This is expected.

---

# 7. Structural Detection Output Class

Structural Detection produces structural findings.

These findings describe significance such as:

- lineage shape
- supersession-related shape
- boundary crossing
- isolation
- concentration
- dependency exposure
- structural asymmetry
- transition-sensitive structure

Structural findings are descriptive.

They are not commands.

They are not dynamic field outputs.

---

# 8. What Structural Detection Is For

Structural Detection exists to answer questions such as:

- What in the graph shape matters here?
- Is this condition structurally local or structurally exposed?
- Is this region heavily dependent on a narrow path?
- Is this area isolated from the rest of the graph?
- Does supersession create a meaningful structural break?
- Do identity boundaries align with graph behavior or cut across it?
- Is this condition concentrated, distributed, or structurally constrained?

These are structural questions.

They are not semantic or dynamic questions.

---

# 9. What Structural Detection Is Not

Structural Detection is not:

- legitimacy
- semantic projection
- drift analysis
- variance analysis
- field dynamics
- a dashboard view
- an action recommendation layer

Structural Detection may support those layers, but it is distinct from all of them.

---

# 10. Structural Detection Categories

Structural Detection may identify several categories of findings.

## 10.1 Lineage Findings

Lineage findings describe significance arising from historical and hierarchical graph relationships.

Examples include:

- long lineage chains
- shallow lineage
- structurally narrow ancestry
- structurally broad descent
- concentrated path dependency

These findings help explain how condition is situated in decision history.

---

## 10.2 Supersession Findings

Supersession findings describe significance arising from replacement and revision structure.

Examples include:

- structurally recent supersession
- dense supersession region
- layered supersession history
- unresolved supersession concentration
- structurally significant break in lineage

These findings help explain how replacement history affects present interpretation.

---

## 10.3 Boundary Findings

Boundary findings describe significance arising from area or identity boundaries.

Examples include:

- identity boundary crossing
- area boundary crossing
- boundary-contained region
- scope fragmentation
- structural overlap between identity regions

These findings help explain how structural condition relates to organizational or system boundaries.

---

## 10.4 Dependency Findings

Dependency findings describe significance arising from concentration, exposure, and graph reliance.

Examples include:

- structurally central node
- dependency bottleneck
- narrow dependency region
- structurally exposed region
- concentrated upstream reliance
- concentrated downstream reliance

These findings help explain how graph shape creates structural sensitivity.

---

## 10.5 Isolation Findings

Isolation findings describe significance arising from weak connectivity or containment.

Examples include:

- isolated node region
- weakly connected region
- structurally detached branch
- contained subgraph
- structurally local condition

These findings help explain where condition is structurally limited or disconnected.

---

## 10.6 Transition Findings

Transition findings describe significance arising from identity or structure change across time.

Examples include:

- identity transition region
- coexistence boundary
- deprecation-affected region
- sunset-sensitive branch
- structurally transitional zone

These findings help explain where structure is shaped by ongoing change rather than stable arrangement.

---

# 11. Structural Detection and Propagation

Propagation and Structural Detection are related but distinct.

Propagation determines how derived condition and scoped influence move across structure.

Structural Detection determines what the structure itself means.

Structural Detection may use propagated scope-aware context where appropriate, but it must remain focused on structural significance rather than semantic labeling or dynamic field behavior.

Propagation answers:

- how condition reaches or affects scope

Structural Detection answers:

- what the shape of the structure means

---

# 12. Structural Detection and Semantic Projection

Semantic projection may summarize structural findings in human-readable form.

Examples:

- structurally isolated
- boundary-crossing
- structurally concentrated
- structurally exposed

However:

- structural findings are not themselves semantic bundles
- semantic projection must not replace structural detection
- structural detail should remain available as its own output class

Structural Detection provides structural truth for explanation.

Semantic projection provides plain-language summary.

---

# 13. Structural Detection and Dynamics

Structural Detection is distinct from dynamics.

Dynamics answers questions such as:

- how condition changes over time
- where drift is increasing
- whether tension is forming
- whether a cascade is emerging

Structural Detection answers questions such as:

- how condition is organized in graph form
- whether there is a bottleneck
- whether there is isolation
- whether supersession creates structural concentration
- whether identity boundaries shape significance

Structural Detection may inform dynamics.

Dynamics may use structure as an input.

But Structural Detection must not be collapsed into field math.

This distinction is mandatory.

---

# 14. Time and Structural Detection

Structural Detection is primarily shape-oriented rather than field-oriented.

It may consider structural history when the structure itself changes.

Examples include:

- supersession history
- identity version changes
- deprecation and sunset structure
- coexistence structure

However:

- Structural Detection must not become a substitute for dynamic time-series analysis
- structural time awareness is only used where the structure itself has changed

---

# 15. Structural Detection Rules

## 15.1 Structure-First Rule

Structural findings must arise from graph or identity truth.

They must not be invented from semantic wording or dynamic labels.

---

## 15.2 Scope Rule

Every structural finding must be interpretable relative to a scope.

A finding without scope is incomplete.

---

## 15.3 Non-Authority Rule

Structural findings must never imply:

- obligation
- fault
- correctness
- command

They describe significance only.

---

## 15.4 Rebuild Rule

Structural findings must be rebuildable from upstream truth.

Loss of cached findings must be non-fatal.

---

## 15.5 Distinction Rule

Structural findings must remain distinct from:

- semantic bundle outputs
- numeric field outputs
- display-only views

---

# 16. Structural Detection Patterns

The following patterns are representative categories of structural significance.

These are descriptive categories, not formulas.

## 16.1 Structural Bottleneck

A narrow point through which much relevant dependency passes.

Why it matters:

A bottleneck increases structural sensitivity and concentration of influence.

---

## 16.2 Structural Isolation

A node or region that is weakly connected, bounded, or detached relative to its scope.

Why it matters:

Isolation may contain condition locally or indicate fragmentation.

---

## 16.3 Structural Concentration

A region where dependency, lineage significance, or supersession intensity is unusually concentrated.

Why it matters:

Concentration may amplify the significance of local change.

---

## 16.4 Structural Fragmentation

A region where related structure is split across boundaries or weakly joined paths.

Why it matters:

Fragmentation may reduce coherence across scope.

---

## 16.5 Structural Break

A meaningful interruption or redirection in lineage or supersession shape.

Why it matters:

A break may explain why older context no longer cleanly governs the present region.

---

## 16.6 Structural Transition Zone

A region shaped by identity version change, coexistence, deprecation, or sunset conditions.

Why it matters:

Transition zones often need to be interpreted differently from stable regions.

---

# 17. Structural Detection and Identity

Identity is not the same thing as graph structure.

Identity overlays structure.

Structural Detection must therefore support findings such as:

- identity-aligned region
- identity-crossing region
- mixed-identity branch
- transition-sensitive identity region
- structurally contained identity scope

These findings help explain how structure and claimed ownership or responsibility relate.

---

# 18. Structural Detection and Areas

Areas create legitimacy and organizational partitions relevant to interpretation.

Structural Detection must support findings such as:

- area-contained branch
- cross-area dependency
- area bottleneck
- structurally concentrated area region
- area-isolated region

These findings help explain how structural condition is distributed across areas.

---

# 19. Output Expectations

Structural outputs should be:

- structured
- machine-readable
- scope-aware
- explainable in plain language
- available independently from semantic and dynamic outputs

Structural outputs should support:

- direct queries
- explanation layers
- views
- CQL exposure

---

# 20. Example Structural Findings

Examples of valid structural findings include:

- structurally isolated node region
- dense supersession branch
- boundary-crossing dependency region
- concentrated upstream bottleneck
- identity transition region
- structurally contained area branch
- structurally exposed downstream path
- structurally fragmented identity scope

These are descriptive examples only.

They do not prescribe action.

---

# 21. Design Guarantees

Structural Detection must remain:

- descriptive
- deterministic
- rebuildable
- scope-aware
- backend-agnostic
- non-authoritative
- distinct from dynamics
- distinct from semantic projection

Given identical graph, lineage, and identity truth, structural findings must be reproducible.

---

# 22. Mental Model

CSG provides structure.

CIS provides scope and identity overlays.

Charter provides legitimacy lineage.

Structural Detection is the CAS function that identifies what those shapes and boundaries mean for understanding condition.

It is the structural explanation layer of CAS.

It exists so graph form is not treated as passive storage, but as meaningful condition-bearing structure.

---

# 23. Final Constraint

Structural Detection exists to answer:

- What structural patterns matter here?
- Where is this condition structurally concentrated or isolated?
- How do lineage, supersession, and boundaries shape interpretation?
- Is this region structurally stable, constrained, exposed, or transitional?

Structural Detection must never answer:

- What action is required
- Which decision is correct
- Who is authoritative
- What humans must do next