# Charter CAS Mathematical and Physics Foundations — Knowledge Skill

Version: 0.3.0
Status: Research baseline / knowledge skill; not a CAS specification
Scope: Mathematical research, model evaluation, validation, traceability, and learning for Charter CAS
Supersedes: 0.2.0

## Purpose

Use this skill to study, compare, derive, simulate, critique, or document mathematics and physics-inspired models that might help explain CAS-observable phenomena. Its purpose is **research integrity**, not automatic acceptance of formulas, causal stories, or CAS requirements.

A correct calculation, a coherent model, an empirically adequate model, and a supported causal claim are different achievements. Never substitute one for another.

## Governing architectural boundaries

- CAS is observational, derived, deterministic given its inputs, scope-aware, rebuildable, and non-authoritative.
- Charter owns legitimacy and accepted decisions; CSG owns graph structure; CIS owns identity and scope; CCare owns observational input. CAS consumes upstream truth and derives condition, scoped influence, structural findings, dynamics, and semantic projections. Follow the current module-boundaries specification for exact interfaces.
- Structural exposure is not observed misconduct. Behavioral prediction is not observed behavior. A semantic projection is not the primitive propagated through the graph.
- CAS cannot create authority, obligations, legitimacy, or enforcement actions; research cannot silently add these capabilities.
- Preserve observation, uncertainty, confidence, evidence provenance, and incomparable dimensions. Do not equate numerical scores with legitimacy or correctness.
- Current CAS specifications outrank historical VDS/VLS proposals; historical documents are research evidence, not current implementation contracts.
- No experimental formula, threshold, state transition, or metric becomes a CAS requirement without a separate explicit specification decision.

## Source precedence and execution audit

Before substantive CAS-specific analysis, consult the relevant sources in this order:

1. Current CAS foundation and module boundaries.
2. Relevant current structural detection, semantic state/vocabulary, temporal behavior, flow observation, artifact lifecycle, and canonical naming specifications.
3. `docs/specs/charter-substrate/cas/Notes/cas-study-notes.md` as a **study agenda**, not normative CAS requirements.
4. Relevant archived proposals in `docs/specs/archive/cli/foundations/CAS/`, especially `09-vds-vls-alignment.md`, `12-vls-alignment-dynamics.md`, `13-vls-signal-local-metrics.md`, and `14-vls-alignment-engine.md`.
5. Relevant earlier CAS, compiler, graph, and relationship-model discussions as conversation-derived research context, not canonical truth.
6. External mathematical references and empirical research when needed, labeled separately from CAS specifications.

Every report must include a **source execution audit**: sources reviewed (with version, URL, commit, or date when available); sources unavailable; sources intentionally omitted and why; conflicting definitions; applied precedence; and the effect of missing sources on confidence. Do not claim complete review after partial inspection. If required sources cannot be accessed, proceed only with an explicit scope limitation.

### Reproducible source snapshots (mandatory for regressions)

Record a **source snapshot manifest** for each regression run. For repository-backed CAS specifications, prefer the immutable repository commit SHA and exact path; record blob SHA or content hash when available. For external research record DOI/version, publication or retrieval date, and immutable archive or checksum when feasible. For conversations record an identifiable conversation reference/date and clearly mark mutable or unavailable content. Record the skill version and exact regression protocol revision alongside the source manifest.

A branch name, `main`, a moving URL, or an access date alone is **not** a reproducible snapshot. If the exact revision cannot be resolved, label the snapshot `UNPINNED`, explain the limitation, and do not claim controlled same-source regression comparability. Include the source snapshot ID (for example `SRCSET-001`) in each research run and link it to all findings that depend on that source set.

To compare skill versions, use identical pinned source snapshots, input datasets, parameter definitions, and regression scenarios; explicitly list any differences. If sources changed, classify the comparison as a **changed-source comparison**, not a controlled skill-only regression. A source snapshot documents what was consulted; it does not establish that every relevant source was reviewed.

## Mathematical research map

### A. Discrete mathematics and graph algorithms

Reachability, ancestor/descendant sets, DAGs, SCCs, topological order, dominators/postdominators where relation semantics justify them, cut sets, bottlenecks, centrality, cones, horizons, lineage, supersession, concentration, isolation, and connectivity. Separate CSG structural truth from CAS-derived interpretation. Graph topology alone does not prove motives or misconduct.

### B. Order theory, lattices, and abstract interpretation

Partial orders, joins/meets, incomparability, product domains, monotone transfer functions, fixed points, widening, and provenance-aware projection. Historical semantic lattices do not automatically define current CAS state ordering. Verify ordering and join laws before asserting a lattice.

### C. Linear algebra and graph signal processing

Vectors, norms, projections, angles, cosine similarity, incidence/adjacency operators, graph Laplacians, neighborhood variation, and lineage bends. Explicitly define dimensions, units, scaling, and edge meaning. An intent vector is a model, not a physical fact.

### D. Statistics, uncertainty, and time series

Rolling means, variance, density, recency, confidence, missingness, event windows, trends, change points, burstiness, and uncertainty propagation. Distinguish negative condition, volatility, disagreement, and absence of observations. Silence is not evidence of stability or misalignment. For temporal models, distinguish event time, reporting/ingestion time, observation schedule/frequency, and evidence reliability; freshness alone does not characterize the observation process.

### E. Dynamics, fields, and network propagation

Weighted relationships, attenuation, thresholds, hysteresis, feedback, cascades, shocks, diffusion, percolation, potential, gravity, attractors, equilibrium, stability, and Lyapunov-style candidates. Treat physics terminology as analogy until state variables, update laws, units, and falsifiable properties are defined. Do not infer health or legitimacy from convergence.

### F. Compiler analysis and incremental computation

Data-flow analogies, transfer functions, dependency graphs, fixed-point solvers, monotonicity, incremental invalidation, replay, and deterministic recomputation. A dominator requires a meaningful root and edge interpretation; a Charter relationship is not automatically control flow.

### G. Discrete/continuous boundary

Hybrid-system thinking, continuous indicators versus discrete semantic projection, threshold selection, hysteresis, event-triggered recomputation, and anti-oscillation. A computational threshold is not a governance decision.

### H. Game theory, mechanism design, and strategic interaction

Utility, preferences, dominant strategies, Nash equilibrium, repeated and coordination games, principal-agent problems, information asymmetry, incentive compatibility, allocation rules, reputation, risk preferences, and strategic network formation. Model agents as strategic only when assumptions justify it. Game-theoretic predictions are **conditional model implications**, not CAS observations of intent or behavior. Distinguish incentive exposure from actual manipulation; consider alternative payoff structures and institutional constraints.

### I. Causal inference and identifiability

Causal graphs, confounding, selection effects, reverse causation, observational equivalence, interventions, natural experiments, and predictive versus causal models. These are research tools, not CAS authority. If competing mechanisms explain the same observations, report non-identifiability and what new evidence could distinguish them.

## Historical concepts to preserve and re-evaluate

Semantic lattice; multidimensional semantic bundles; drift/path drift; variance/volatility; intent gravity, energy, potential, fields, and attractors; alignment cones and horizons; lineage bend and supersession shock; tension, collapse, cascades, equilibrium, recovery; confidence, recency, normalization, adaptive windows; scoped propagation and semantic compression.

Archived weighted means, variances, gravity sums, confidence multipliers, and thresholds are **historical candidates**, not current CAS defaults.

## Mandatory research gates

All substantive model evaluations must pass through the following gates. If a gate cannot be completed, mark it **NOT CHECKED**, **UNDETERMINED**, or **NOT APPLICABLE** with reasons; do not silently omit it. An incomplete gate limits the claim, not necessarily the usefulness of the research.

### Gate 1 — Phenomenon, claims, and evidence

1. State the research question, observable phenomenon, scope, and time horizon.
2. Separate directly observed data, structural facts, assumptions, interpretations, and predicted outcomes.
3. Classify **each material claim** using one or more of:
   - `MATHEMATICAL_IDENTITY_OR_THEOREM` — mathematical statement under explicit premises.
   - `CONDITIONAL_MODEL_IMPLICATION` — consequence within a specified model.
   - `DESCRIPTIVE_OBSERVATION` — observed or sourced condition.
   - `STATISTICAL_ASSOCIATION` — relationship in data, without automatic causation.
   - `CAUSAL_HYPOTHESIS` — proposed causal mechanism, not established.
   - `EMPIRICALLY_SUPPORTED_CAUSAL_FINDING` — causal claim supported by an identified empirical design and its assumptions.
4. If a statement mixes claim types, split it. Never upgrade conditional mathematics to an empirical or causal finding by rhetoric.

### Gate 2 — Assumptions, model, and identifiability

1. Define variables, domains, units, graph direction, state, timing, inputs, update laws, and aggregation semantics.
2. Identify invented, assumed, measured, and estimated parameters separately; include data sources and estimation methods when available.
3. State causal assumptions explicitly if causal language is used. Identify confounders, reverse causation, selection, and potential feedback where relevant.
4. Propose at least one plausible competing explanatory mechanism for behavioral or causal interpretations, or explain why none is relevant.
5. Specify which observables would distinguish models, which parameters cannot be identified from available evidence, and what additional observations or interventions would help.
6. Fitting an observation does not identify its generating mechanism. If models remain observationally equivalent, preserve ambiguity.
7. **For temporal models, define the observation process separately from latent/system dynamics.** Specify event time (when the underlying occurrence happened), reporting/ingestion time (when evidence became available), observation schedule and frequency (when measurement was possible or attempted), and evidence reliability (error, bias, coverage, and uncertainty). Identify late, missing, duplicated, backfilled, censored, or revised reports where relevant. State which timestamps are known, inferred, or absent. Do not substitute time since last received report for time since last underlying event.
8. Distinguish **system-state uncertainty** from **observation-process uncertainty**. If an apparent trend can be produced by changes in reporting cadence, delay, or reliability, report that alternative explanation and whether the trend is identifiable.

### Gate 3 — Verification, sensitivity, and counterexamples

For each claimed mathematical property, record its premises, verification method, actual result, and limitations. Use **method-specific statuses**, not a single maturity ladder:

- `NOT_CHECKED`
- `ANALYTICALLY_VERIFIED`
- `NUMERICALLY_TESTED`
- `PROPERTY_TESTED`
- `EMPIRICALLY_VALIDATED`
- `FAILED`

Multiple statuses may apply to different properties or evidence dimensions. `EMPIRICALLY_VALIDATED` requires identified data and a validation design; numerical simulation alone does not qualify. Never claim a test was run unless it was actually executed. Distinguish necessary, sufficient, and necessary-and-sufficient conditions.

**Property proof is not protocol completion.** Maintain two independent registers:

- **Mathematical property register:** property ID, premises, method, evidence/artifact, result, and limitations; e.g. `ANALYTICALLY_VERIFIED` for a fixed-point theorem under declared conditions.
- **Regression execution register:** check ID, expected evidence, execution state (`NOT_RUN`, `PARTIAL`, `COMPLETED`, `BLOCKED`, or `NOT_APPLICABLE`), actual evidence, failures/omissions, and scope of completion.

A central theorem may be proven while the regression remains `PARTIAL` because sensitivity, alternative cases, observation-process tests, or source audits were not executed. Never report `REGRESSION_COMPLETE` solely from a proven mathematical property. Distinguish **protocol execution completeness** from **acceptance outcome** (`PASS`, `PASS_WITH_LIMITATIONS`, `FAIL`, or `INDETERMINATE`); a completed protocol can fail, and an incomplete one cannot be presented as fully validated.

Check relevant properties: mathematical well-definedness, dimensions, determinism, invariance, boundedness, monotonicity (when claimed), existence/uniqueness, convergence, stability, and algorithmic reproducibility. Explain why non-applicable properties are omitted.

For models with invented or estimated parameters, **mandatory sensitivity analysis** must identify influential parameters, defensible or explicitly illustrative ranges, boundary conditions, thresholds, and conditions that reverse the conclusion. Perform analytic range analysis or an actual parameter sweep when feasible. If neither is feasible, mark robustness `NOT CHECKED` and all numeric conclusions illustrative. Report robustness as `ROBUST_WITHIN_TESTED_RANGE`, `FRAGILE`, or `UNDETERMINED`, never universally robust without proof.

Attempt meaningful counterexamples, including competing incentives, missing or stale observations, contradictory signals, disconnected scopes, high-degree hubs, cycles, changing topology, alternative payoff matrices, and confounding, as relevant. Record whether each counterexample defeats, limits, or leaves the claim intact.

### Gate 4 — Traceability and source audit

Use lightweight **research-local IDs** (not new CAS/CIS identity semantics):

`RQ-### -> H-### -> A-### -> M-### -> P-### -> D-### -> V-### -> F-### -> I-### -> DISP-###`

- `RQ`: research question; `H`: hypothesis; `A`: assumption; `M`: model; `P`: parameter; `D`: derivation or computation; `V`: verification; `F`: finding; `I`: interpretation; `DISP`: disposition.
- Every finding cites its model, assumptions, relevant parameters, derivation, verification, and source evidence.
- Each interpretation cites findings and its claim classification; never imply stronger evidence than its dependencies.
- Record version, source provenance, and changes. When an assumption, source, parameter, or model changes, identify downstream findings and interpretations needing reevaluation.
- Include the source execution audit described above. Link each research run to a pinned `SRCSET-###` snapshot manifest, or mark it `UNPINNED`. Maintain separate mathematical property and regression execution registers; preserve check IDs across skill-version comparisons.

### Gate 5 — Interpretation and disposition

Distinguish explicitly:

- Mathematical correctness vs model adequacy vs empirical support.
- Correlation vs causation; prediction vs observation.
- Structural concentration/exposure vs behavioral dysfunction.
- Equilibrium **existence**, **uniqueness**, **stability**, **efficiency**, **desirability**, and **legitimacy**.
- Stable condition vs healthy condition; stable degradation vs accelerating degradation.
- Confidence degradation from missing feedback vs observed flow degradation.

Do not infer ethical desirability, organizational health, alignment, or legitimacy from equilibrium or convergence. A stable adverse condition is possible. Do not infer misalignment from stale feedback. Keep descriptive CAS outputs separate from decisions and governance.

Assign each model an explicit disposition: `ESTABLISHED_MATHEMATICAL_TOOL`, `CURRENT_CAS_SPECIFIED_MECHANISM` (with exact current citation), `HISTORICAL_PROPOSAL`, `RESEARCH_CANDIDATE`, `REJECTED_HYPOTHESIS`, or `OPEN_QUESTION`. This is classification, **not automatic promotion**. Propose specification changes separately, requiring explicit review.

## Three independent evidence dimensions

Every substantive model summary must report:

1. **Mathematical verification:** What mathematical properties were proven or tested under which premises?
2. **Model adequacy:** What justifies using this model for the phenomenon, and what rival models remain plausible?
3. **Empirical support:** What observed data, predictive tests, or causal designs support its application?

For each dimension report evidence, limitations, and `SUPPORTED`, `PARTIAL`, `NOT TESTED`, `FAILED`, or `UNDETERMINED`. These are **separate axes**, not an aggregate score. A verified Nash equilibrium may have untested model adequacy and no empirical support.

## Standard research output contract

A report should include, in this order:

1. Research question, scope, and CAS boundary.
2. Source execution audit, immutable source snapshot manifest (`SRCSET-###`), and source-precedence discrepancies.
3. Hypothesis, alternative explanations, and claim classifications.
4. Traceable assumptions, variables, model, and parameters.
5. Worked derivation/example with references to assumptions and model IDs.
6. Mathematical property verification register (property, assumptions, method, status, result, limitation). Separately report regression protocol execution (check ID, run state, evidence, omissions, acceptance result).
7. Parameter sensitivity, boundary conditions, robustness, and counterexamples. Include temporal observation-process sensitivity where applicable.
8. Identifiability assessment: indistinguishable explanations and required additional evidence.
9. Separate mathematical-verification, model-adequacy, and empirical-support assessments.
10. Findings, semantic interpretation limits, and traceability references.
11. Model disposition, open questions, proposed experiments, and proposed documentation changes (never silently applied).

For lightweight exploratory conversations, concise formatting is acceptable, but no claim may silently bypass a required gate. Label incomplete analysis rather than inventing verification.

## Regression protocol for skill changes

Use a fixed hypothesis and an **identical immutable source snapshot manifest (`SRCSET-###`)** to compare skill versions. Pin the repository commit(s), relevant file paths/content hashes, data version, parameters, and protocol revision. If pinning is impossible, declare the comparison uncontrolled. Evaluate **research quality, not report length**. Compare unsupported claims caught, assumption coverage, rival explanations, verified properties, parameter robustness, traceability, and changes in evidence classification. Record what was actually computed, simulated, or empirically tested.

For **every** numbered regression check, record `NOT_RUN`, `PARTIAL`, `COMPLETED`, `BLOCKED`, or `NOT_APPLICABLE`, plus evidence and acceptance outcome. Publish a completion matrix and list remaining checks. Mathematical proofs and executed examples can support a check but never silently substitute for checks that were not run. Do not call a regression fully executed unless every applicable check has been completed; a completed check may still fail its acceptance criterion.

### Regression A — Organizational incentives and strategic behavior

Hypothesis: *Concentrated authority, scarce rewards, and opaque allocation rules can encourage political manipulation.*

Required checks:

1. Construct an explicit strategic interaction model with assumptions and at least two payoff structures.
2. Measure or illustrate structural concentration without inferring actual misconduct.
3. Derive equilibrium results and state their mathematical scope.
4. Sweep or analytically vary meaningful payoffs, success probabilities, transparency, and enforcement assumptions; identify reversals.
5. Distinguish strategic incentives from observed conduct and actual causal mechanisms.
6. Include alternative explanations such as legitimate confidentiality, fear, norms, and coordinated distributed authority.
7. Identify unidentifiable parameters, counterexamples, and evidence required for real-world claims.
8. Apply claim classifications, verification register, evidence dimensions, and traceability IDs.
9. Keep all experimental formulas out of CAS specifications absent separate promotion.

### Regression B — Temporal degradation and stability

Question: *Can a modeled system be mathematically stable while CAS has degraded observational confidence or while flow capacity is persistently impaired?*

Required checks:

1. Separate **feedback degradation** (staleness/recency and confidence) from **flow degradation** (persistent constraints and capacity effects), following the current temporal addendum.
2. Construct at least one explicit discrete-time candidate model with declared state, units, parameters, and observation process.
3. Check a stability property analytically or numerically; distinguish sufficient from necessary conditions and stability from desirability.
4. Compare stable adverse equilibrium, recovering disturbance, and escalating disturbance; report which cases are actually demonstrated.
5. Vary missing-observation patterns, feedback delay, thresholds, and parameters; record sensitivity and uncertainty.
6. **Model the observation process explicitly**, distinguishing event time, reporting/ingestion time, observation frequency/sampling schedule, and evidence reliability. Include at least two contrasting observation processes applied to the **same underlying system trajectory** (for example delayed reporting versus genuinely reduced event frequency), and determine whether they yield different apparent stability or degradation. If not executed, mark the check `NOT_RUN` or `PARTIAL`; freshness-only models do not satisfy this check.
7. Do not equate absent signals with evidence of stability or deterioration.
8. Preserve the current semantic state's projection role; do not make a research stability label authoritative.
9. Apply the full evidence and traceability contract.

### Acceptance criteria for both regressions

- **AC-01:** Invented parameters yield explicitly conditional claims.
- **AC-02:** Competing explanations and identifiability limits are recorded.
- **AC-03:** Parameter ranges, thresholds, and reversals are checked or explicitly untested.
- **AC-04:** Convergence never implies health, desirability, or legitimacy.
- **AC-05:** Each claimed mathematical property has an honest verification method/status.
- **AC-06:** CAS remains observational and non-authoritative.
- **AC-07:** Current-source precedence and omissions are auditable.
- **AC-08:** Findings link to assumptions, models, evidence, and derivations.
- **AC-09:** Counterexamples are attempted and their effect recorded.
- **AC-10:** Every model has a research disposition without silent specification promotion.
- **AC-11:** Mathematical verification, model adequacy, and empirical support are assessed separately.
- **AC-12:** The revision improves claim discipline or traceability, not just output volume.
- **AC-13:** A regression has an immutable source snapshot manifest, or is explicitly marked uncontrolled/`UNPINNED`.
- **AC-14:** Mathematical property verification and regression check execution are reported independently; unexecuted checks remain visible even when central theorems are proven.
- **AC-15:** Temporal regression distinguishes event time, report time, sampling cadence, and evidence reliability, and tests at least two observation processes against a shared underlying trajectory (or records the test as incomplete).

## Priority study sequence

1. Graph structure, scope, reachability, and relationship semantics.
2. Order theory, semantic product domains, and abstract interpretation.
3. Statistics, missingness, temporal windows, and confidence.
4. Linear algebra and graph-attached signals.
5. Propagation, thresholds, hysteresis, and strategic interactions where relevant.
6. Cascades, shocks, stability, equilibrium, and causal identifiability.
7. Deterministic incremental analysis and reproducible verification.

## Important pitfalls

- High variance is not necessarily tension; persistent low-variance negative condition may still be adverse.
- Centrality or reward concentration does not prove intent, favoritism, or illegitimacy.
- Graph propagation is not a sufficient model of strategic agents who adapt their incentives.
- A lineage bend is not an angle unless vector dimensions and scaling are justified.
- Gravity, energy, potential, and fields are not physical quantities without explicit models; do not invent conservation laws.
- A DAG for one relationship type does not guarantee acyclicity for mixed relationships.
- Smoothing and aggregation can hide anomalies; retain source-level provenance.
- Semantic states may be incomparable; avoid invented total orders.
- Predictive estimates must be separated from current observations.
- A contraction condition such as `||alpha W|| < 1` can be sufficient under specified norm and update assumptions; do not call it necessary without proof.
- A numeric experiment is not empirical validation of human behavior.
- A fresh report is not necessarily a recent event; a stale report is not necessarily evidence that no event occurred.
- Increased reporting frequency can imitate increased event frequency; delayed/backfilled reporting can imitate abrupt changes.
- A proven property is not evidence that every regression scenario was executed.
- A moving branch URL is not an immutable research snapshot.

## Canonical and historical links

- Current CAS directory: https://github.com/whitephoenixsoft/CharterCore/tree/main/docs/specs/charter-substrate/cas
- Study notes: https://github.com/whitephoenixsoft/CharterCore/blob/main/docs/specs/charter-substrate/cas/Notes/cas-study-notes.md
- Archived CAS directory: https://github.com/whitephoenixsoft/CharterCore/tree/main/docs/specs/archive/cli/foundations/CAS
- Archived observability: https://github.com/whitephoenixsoft/CharterCore/blob/main/docs/specs/archive/cli/foundations/CAS/09-vds-vls-alignment.md
- Archived dynamics: https://github.com/whitephoenixsoft/CharterCore/blob/main/docs/specs/archive/cli/foundations/CAS/12-vls-alignment-dynamics.md
- Archived local metrics: https://github.com/whitephoenixsoft/CharterCore/blob/main/docs/specs/archive/cli/foundations/CAS/13-vls-signal-local-metrics.md
- Archived engine: https://github.com/whitephoenixsoft/CharterCore/blob/main/docs/specs/archive/cli/foundations/CAS/14-vls-alignment-engine.md

## Maintenance and changelog

### v0.3.0 — 2026-11-06

- Required pinned source snapshot manifests for reproducible skill-version regression comparisons; documented uncontrolled comparisons.
- Separated mathematical property verification from regression protocol execution and acceptance outcomes, with per-check execution states.
- Required explicit temporal observation-process modeling: event time, report/ingestion time, observation schedule/frequency, and evidence reliability.
- Strengthened temporal regression with same-trajectory/different-observation-process comparisons and added AC-13 through AC-15.
- Preserved CAS boundaries: research observation models are not automatically CAS formulas or requirements.

### v0.2.0 — 2026-11-06

- Preserved the v0.1.0 mathematical research map and CAS boundaries.
- Added strategic interaction, mechanism design, causal inference, and identifiability.
- Replaced loosely suggested validation with five mandatory research gates.
- Added explicit claim classifications, separate evidence dimensions, property verification statuses, mandatory sensitivity and counterexamples, research-local traceability IDs, and source execution audit.
- Added two fixed regression protocols and twelve acceptance criteria.
- Clarified equilibrium interpretation, observational ambiguity, and no automatic CAS specification promotion.

### v0.1.0 — Initial research baseline

- Established mathematical disciplines, historical CAS research inventory, source precedence, architectural boundaries, and exploratory research workflow.

When new research is added, record date, sources, assumptions, evidence, confidence, relationship to current specifications, and whether it changes only this knowledge skill or warrants a separately reviewed specification proposal. Preserve historical claims and their dispositions.
