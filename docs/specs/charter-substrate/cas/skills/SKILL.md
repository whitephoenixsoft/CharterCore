# Charter CAS Mathematical and Physics Foundations — Knowledge Skill

Version: 0.1.0 (research baseline)
Status: Research / knowledge skill; not a CAS specification
Scope: Charter CAS mathematical foundations, historical models, evaluation, and learning

## Purpose

Use this skill when studying, evaluating, comparing, simulating, or documenting mathematics and physics-inspired models that might support Charter CAS. Preserve provenance, distinguish established mathematics from metaphors and unvalidated hypotheses, and do not silently promote research into CAS requirements.

## Governing boundaries

- CAS is observational, derived, deterministic, scope-aware, and non-authoritative.
- Charter legitimacy owns authority and accepted decisions; CSG owns structural graph facts; CIS owns identity and scope; CCare owns observational input. CAS consumes these and derives condition, scoped influence, structural findings, dynamics, and semantic projections.
- Semantic projection describes computed condition; semantic labels are not the primitive propagated through the graph.
- Distinguish observations, measured properties, derived indicators, interpretive labels, and decisions. Preserve uncertainty, confidence, provenance, and incomparability.
- The current foundation and module-boundary specifications outrank archived VDS/VLS proposals. Archive is historical evidence, not an implementation contract.
- Numerical scores must not silently erase different semantic meanings or determine correctness, legitimacy, or obligation.

## Source precedence and research method

1. Read current CAS foundation, module boundaries, structural detection, semantic state/vocabulary, temporal behavior, flow observation, and lifecycle documents.
2. Read `docs/specs/charter-substrate/cas/Notes/cas-study-notes.md` as a study agenda, not a specification.
3. Consult `docs/specs/archive/cli/foundations/CAS/` for historical hypotheses and original terminology; especially `09-vds-vls-alignment.md`, `12-vls-alignment-dynamics.md`, `13-vls-signal-local-metrics.md`, and `14-vls-alignment-engine.md`.
4. Consult relevant prior CAS/compiler/graph conversations for research questions, marking them as conversation-derived rather than canonical.
5. When sources disagree, explicitly record the discrepancy and defer to current specifications for architectural boundaries; do not invent a reconciliation.

## Mathematical research map

### A. Discrete mathematics and graph algorithms
Reachability, ancestor/descendant sets, DAGs, topological order, SCCs, dominators/postdominators where their edge semantics permit, cut sets, bottlenecks, centrality, cones, horizons, lineage, supersession, concentration, isolation, and connectivity. Distinguish graph facts computed by CSG from CAS interpretation of those facts.

### B. Order theory, lattices, and abstract interpretation
Partial orders, joins/meets, incomparable states, product domains, monotone transfer functions, fixed points, widening, and provenance-aware projections. Historical semantic lattice examples are research precedents, not automatically current CAS state ordering. Test whether a proposed lattice has valid ordering and join laws before calling it a lattice.

### C. Linear algebra and graph signal processing
Vectors, norms, projections, angles, cosine similarity, incidence/adjacency operators, graph Laplacians, neighborhood variation, and directional lineage bends. A vector representation requires explicit dimensions, comparable scales, and defensible semantics. Do not treat metaphorical intent directions as physical vectors without defining a measurable embedding.

### D. Statistics and time series
Rolling means, variance, density, recency, confidence weighting, event windows, trend, velocity/acceleration analogies, burstiness, and change-point detection. Preserve distinctions among disagreement, temporal volatility, uncertainty, and persistent negative condition. Test sensitivity to window selection and missing observations.

### E. Dynamics, fields, and network propagation
Influence kernels, weighted relationships, attenuation, thresholds, hysteresis, feedback, cascades, shocks, potential, gravity, attractors, equilibrium, stability, Lyapunov-style candidates, diffusion, and percolation. Treat physics words as analogies until units, state variables, update laws, and falsifiable properties are defined. CAS does not control or enforce the observed system.

### F. Compiler analysis and incremental computation
Control-flow/data-flow analogies, dependency graphs, transfer functions, fixed-point solvers, monotonicity, incremental recomputation, invalidation, replay, and deterministic convergence. Dominance is meaningful only with a defined root and edge interpretation; not every Charter relationship is control flow.

### G. Discrete/continuous boundary
Hybrid-system thinking, continuous indicators versus discrete semantic projections, promotion/release thresholds, anti-oscillation behavior, state persistence, and event-triggered updates. Distinguish threshold-driven computation from human authority or decision promotion.

## Historical concepts to preserve and re-evaluate

- Semantic lattice and multidimensional semantic bundles
- Drift and path drift; variance and volatility
- Intent gravity, energy, potential, attractors, and fields
- Alignment cones and horizons
- Lineage bend and supersession shock
- Tension, collapse, cascades, equilibrium, and recovery
- Signal normalization, confidence, recency, adaptive windows
- Scope-specific propagation and semantic compression

Historical example formulas, such as weighted signal means, population variance, gravity sums, and confidence multipliers, are **historical candidates**. Do not adopt numeric mappings or thresholds from archived documents as current defaults.

## Required analysis procedure

For each proposed phenomenon:

1. **Phenomenon:** What exactly is being observed, and at which scope and time?
2. **Evidence:** Which source artifacts, observations, and graph relations support it?
3. **Measurable properties:** What is directly measurable versus inferred?
4. **Mathematical basis:** Specify graph structure, state variables, domain, formula or algorithm, assumptions, units, and edge direction.
5. **Mechanism:** Define local computation, propagation, aggregation, windows, and update ordering.
6. **Properties:** Check determinism, monotonicity where expected, boundedness, invariance, convergence, and sensitivity.
7. **Semantic interpretation:** Define optional labels without collapsing distinct dimensions or claiming authority.
8. **Counterexamples:** Test missing data, contradictory signals, disconnected scopes, high-degree hubs, cycles, stale signals, and changing topology.
9. **Traceability:** Link each derived finding to its evidence, transformations, assumptions, and source versions.
10. **Disposition:** Classify as established mathematical tool, CAS-specified mechanism, historical proposal, research candidate, rejected hypothesis, or open question.

## Standard output for future research

- Question / phenomenon
- CAS architectural boundary
- Source and historical provenance
- Mathematical model and assumptions
- Worked miniature example
- Expected observations and counterexamples
- Computational complexity and Rust implementation candidates (when requested)
- Evidence gaps / risks / limitations
- Proposed experiment and pass/fail criteria
- Disposition and documentation updates proposed (never silently applied)

## Priority study sequence

1. Graph structure, scope, and reachability; relationship semantics.
2. Order theory, semantic product domains, and abstract interpretation.
3. Statistical measurements and temporal windows.
4. Linear algebra and graph-attached signals.
5. Weighted propagation, thresholds, and hysteresis.
6. Cascade and shock models; stability and equilibrium.
7. Incremental deterministic analysis, compiler-style fixed points, and validation.

## Important distinctions and pitfalls

- A high-variance signal stream does not necessarily imply structural tension; a low-variance negative stream can indicate persistent adverse condition.
- Graph centrality does not prove legitimate authority or intent priority.
- A lineage bend is not defined by angle unless vector dimensions are justified.
- An attractor, gravity, potential, or energy is not a physical quantity without a defined model; never imply conservation laws by metaphor.
- A DAG over one relation type does not guarantee acyclicity over all mixed relation types.
- Smoothing may erase local anomalies; aggregation must preserve source traceability.
- Semantic states need not form a total order; incomparability can be meaningful.
- Predictive estimates must remain separate from present-state measurements.

## Canonical and historical source links

- Current study notes: https://github.com/whitephoenixsoft/CharterCore/blob/main/docs/specs/charter-substrate/cas/Notes/cas-study-notes.md
- Archived CAS foundation directory: https://github.com/whitephoenixsoft/CharterCore/tree/main/docs/specs/archive/cli/foundations/CAS
- Archived alignment observability: https://github.com/whitephoenixsoft/CharterCore/blob/main/docs/specs/archive/cli/foundations/CAS/09-vds-vls-alignment.md
- Archived alignment dynamics: https://github.com/whitephoenixsoft/CharterCore/blob/main/docs/specs/archive/cli/foundations/CAS/12-vls-alignment-dynamics.md
- Archived local metrics: https://github.com/whitephoenixsoft/CharterCore/blob/main/docs/specs/archive/cli/foundations/CAS/13-vls-signal-local-metrics.md
- Archived alignment engine: https://github.com/whitephoenixsoft/CharterCore/blob/main/docs/specs/archive/cli/foundations/CAS/14-vls-alignment-engine.md

## Maintenance

When new research is added, record date, sources, assumptions, confidence, relationship to current CAS specifications, and whether the result changes only the knowledge skill or warrants a separate specification proposal. Maintain a changelog and never overwrite historical claims without noting their status.
