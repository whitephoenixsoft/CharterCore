# CGL Ecosystem Integration Overview

## Status

Draft foundation overview.

This document defines how the Charter Guidance Layer (CGL) fits into the larger Charter ecosystem, how it may operate as a standalone library, and what boundaries it must preserve when assisting users, hosts, or external AI systems.

## Purpose

The Charter Guidance Layer exists to make Charter state understandable, explorable, and usable by a host without turning explanation into authority.

CGL provides exegesis, clarification, summarization, conflict detection, duplicate-meaning detection, next-step assistance, tone control, phase control, and AI model behavior constraint.

CGL does not create legitimacy.  
CGL does not decide for the host.  
CGL does not mutate canonical state.  
CGL does not normalize the host.  
CGL does not replace substrate authority.

Its purpose is to help a host understand what is already present, what may be unclear, what may conflict, what may be duplicated semantically, and what next actions may be available.

## Charter Ecosystem Context

Charter is the name of the larger ecosystem formed by multiple interoperating substrates.

Although Charter contains many substrates, the unified goal is simple:

Charter is a decision journal and legitimacy system that promotes exegesis, clarity, and shame-free reflection without removing agency.

Every feature in Charter should enhance the host’s ability to understand, reflect, decide, and preserve intent without coercing decisions or rewriting history.

## Substrate Independence

Each Charter substrate should be an independent library.

Each substrate is biased toward the Charter ecosystem, but should remain technically usable outside Charter.

Charter itself, when assembled from its substrates, can be viewed as a larger system-of-substrates.

CGL follows the same pattern. It is not merely a feature inside Charter. It is a standalone guidance substrate/library that can operate in two modes:

1. Charter-integrated mode.
2. Standalone adapter mode.

## CGL in Charter-Integrated Mode

In Charter-integrated mode, CGL sits near the top of the ecosystem.

It reads queryable state from Charter through CQL or CQL-available interfaces.

It helps explain runtime state, CDS deliberation state, CAS responses, CSG structure, CIS identity context, CCare signals, CSP feeds, CRS federation context, and other queryable substrate outputs.

CGL may be accessed through the CLI, runtime, CDS workspaces, CAS summaries, or other host-facing interfaces.

The basic relationship is:

Host  
→ Runtime / CDS / CAS / other host-facing workflows  
→ CQL  
→ Queryable substrate state  
→ CGL explanation and assistance  
→ Host understanding

CGL reads and explains. It does not become the source of truth.

## CGL in Standalone Adapter Mode

In standalone adapter mode, CGL can be used outside full Charter.

Another system may implement a CQL-compatible adapter and allow CGL to operate over that system’s data.

This makes CGL useful as a bounded AI guidance adapter for external AI models.

The pattern is:

External system  
→ CQL-compatible adapter  
→ CGL  
→ model adapter  
→ bounded AI assistance

In this mode, CGL still uses canon, phase contracts, tone contracts, markers, and guidance boundaries.

The purpose is to provide psychological safety, consistency, source-grounded explanation, and AI behavior constraint even when the full Charter ecosystem is not present.

## Core Architectural Position

CGL depends on CQL.

This is intentionally narrow.

By depending on CQL, CGL does not need direct knowledge of every substrate’s internal storage model. It needs a stable way to ask questions and receive structured context.

CGL should be able to understand anything queryable, but it must preserve the authority boundary of whatever it is explaining.

For example:

- A resolution is legitimate state.
- A CDS item is deliberative or simulation state.
- A CCare signal is descriptive feedback.
- A CAS snapshot is an authoritative historical derived commit.
- A CSP feed is a live view.
- A CGL output is ephemeral guidance unless explicitly stored as descriptive metadata.

CGL must not flatten these distinctions.

## Primary CGL Usage Locations

CGL is expected to be used most heavily in the runtime and CDS because those are the spaces where the host interacts most directly with Charter.

CGL is also expected to be heavily used for explaining CAS responses.

### Runtime Usage

In the runtime, CGL can assist with:

- explaining legitimacy errors,
- helping clarify user intent,
- detecting conflicts between proposed resolutions,
- detecting duplicate meaning expressed with different semantics,
- helping draft or refine resolutions,
- explaining area scope and decision rules,
- explaining why reconciliation or recontextualization may be needed,
- suggesting possible next steps,
- helping the host understand authority, scope, and legitimacy implications.

The runtime remains responsible for orchestration and legitimacy mechanics. CGL explains and assists.

### CDS Usage

CDS is a thinking system and workspace for non-legitimate deliberation.

CGL is especially useful in CDS because CDS is already designed for exploration, investigation, breakouts, observations, simulations, and proposal preparation.

In CDS, CGL can assist with:

- deliberate workspace support,
- breakout support,
- summarizing observations,
- conflict detection,
- duplicate semantic meaning detection,
- clarifying ambiguous ideas,
- preserving open questions,
- organizing investigation notes,
- comparing alternative item graph configurations,
- helping convert final items into proposal candidates,
- explaining simulation results,
- moderating multi-user deliberation.

CGL may be more exploratory in CDS than in legitimacy workflows, but it must still remain non-authoritative.

### CAS Usage

CAS produces alignment metrics, semantic projections, and derived state that may be difficult for a human to interpret directly.

CGL can summarize and explain CAS responses.

CGL can help explain:

- tension,
- collapse,
- variance,
- momentum,
- gravity,
- reduced capacity,
- relationship pressure,
- identity-bound alignment,
- area-level health,
- graph-level drift,
- simulation-only CAS outputs,
- authoritative CAS snapshots.

CGL should always explain whether a CAS result comes from legitimate resolution state, simulation state, local graph state, federated graph state, or snapshot state.

## Major Charter Substrates Relevant to CGL

### Runtime

The runtime orchestrates Charter behavior.

It embeds the legitimacy engine and coordinates host-facing workflows.

The runtime is persistence-agnostic and may run in memory as a long-living process.

CGL is useful in the runtime because the runtime is where users often need immediate clarification, error explanation, conflict detection, and next-step assistance.

### Legitimacy Engine

The legitimacy engine is a calculator with no persistence.

It validates and completes legitimacy actions.

A legitimate decision is called a resolution.

A resolution is the core legitimacy unit.

CGL may explain legitimacy mechanics, but it must not create legitimacy or decide whether something should be legitimate.

### Resolution

A resolution is the core legitimate decision unit.

Resolutions may relate to other resolutions.

Resolutions exist inside areas.

Resolutions may be derived from prior resolutions through recontextualization or from CDS items through reconciliation review.

CGL may explain resolution history, relationships, scope, conflicts, and possible implications.

### Area

An area is a bounded context for resolutions.

An area has scope and a decision rule.

Areas exist because not all decisions have the same authority, context, or focus.

CGL may explain area scope and decision-rule implications, but it must not invent missing authority.

### CCS — Charter Commit System

CCS is the protocol and wrapper structure for all commits.

A commit store saves commits and may be persisted or in memory.

All legitimate actions are copied to the commit store because once an action is legitimate, it affects all substrates.

CCS acts as a communication and correlation hub.

CGL may explain commit history and provenance, but it must not treat all commit types as equal in authority.

### CQL — Charter Query Language

CQL is the central query language between substrates.

CGL should depend on CQL.

CQL may use JSON IL to declare views, filters, and query intent.

A human-facing DSL may compile to JSON IL.

CGL may assist users in forming CQL queries or human DSL queries, but it should not silently decide what the user meant to query.

### CDS — Charter Deliberate System

CDS is the substrate for non-legitimate thinking.

It supports epics, deliberates, breakouts, items, observations, investigation, simulation, and proposal preparation.

CDS items are not resolutions.

Nothing is converted from CDS into legitimacy.

During Reconciliation Review, the host explicitly selects which CDS items should be used to derive new resolutions.

CGL is highly useful in CDS because CDS is a safe workspace for exploration, clarification, conflict detection, and synthesis.

### CSG — Charter Structural Graph

CSG derives graph structure from resolutions or items.

It can create graph views across areas or sparse graph views.

CSG may query the commit store or CDS through CQL.

CGL may explain graph structure, but CSG owns structural derivation.

### CIS — Charter Identity System

CIS consumes the CSG graph and adds identities bounded by graph nodes.

Identities may represent people, teams, departments, organizations, systems, or host-defined boundaries.

Identities may be nested or may share nodes.

CIS manages and versions identities.

Identity is a primary use case because it adds context to graph calculations and helps expose shared responsibility.

CGL may explain identity-bounded views and identity implications, but CIS owns identity state.

### CAS — Charter Alignment System

CAS calculates alignment metrics and semantic projections.

CAS builds on CSG and may optionally use CIS.

CAS can calculate tension, collapse, variance, momentum, gravity, and other dynamics across relationships and abstraction tiers.

CAS may operate over resolution graphs or item graphs.

CAS output over item graphs must be marked as simulation.

CAS may emit authoritative historical snapshot commits to communicate official state for part or all of a system.

CGL may summarize CAS output, explain CAS results, and help the host understand what to inspect next.

### CCare

CCare is a substrate for feedback signals.

Signals attach to resolutions and may be used with CDS items during simulation.

Signals are a main input to CAS calculations and semantic-state lattice projections.

Signal states may include:

- alignment,
- misalignment,
- uncertainty,
- reduced capacity,
- intentional pause,
- need for reassessment.

A CCare signal is a legitimate commit and includes a confidence level.

One signal commit may contain one signal or an aggregate of multiple signals.

CGL may explain signals, signal history, confidence, and relationships to resolutions or simulations.

### CSP — Charter Signal Processing

CSP is a non-authoritative signal-shaping substrate.

It processes high-frequency or noisy inputs safely.

It reduces noise through clustering and aggregation.

It provides configurable emission and cadence control.

It supports human-centric and automated systems.

It publishes named feeds for ongoing monitoring.

It supports identity-, area-, target-, and signal-scoped publication.

It may transform output into CCS-backed descriptive commits.

CSP feeds are live views.

CSP pipeline definitions are stored in a local store and may be backed up into the commit store.

CSP can protect the commit store from noise in high-output environments.

CGL may query CSP when it needs to explain feed behavior, aggregation, filtering, signal volume, or high-output signal history.

### CRS — Charter Relay System

CRS handles federation between systems.

CRS federates CCS commits.

Incoming external material must enter an untrusted store and go through Reconciliation Review before crossing into legitimacy.

CRS allows teams, organizations, systems, and automated hosts to share selected commits while preserving agency and cadence.

CGL may explain federation context, provenance, and trust boundaries, but it must not treat federated material as legitimate until it has passed the appropriate review.

### VDS — Value Driven System

VDS is a separate host.

VDS is an agent version of CCare for environments where humans are not directly emitting feedback.

VDS monitors an application or set of applications through the lens of explicit commitments.

Teams define how software meets mission goals.

Teams define metrics that map telemetry to signal types.

When thresholds are reached, VDS creates observations and emits signals.

VDS exists to:

- interpret telemetry through explicit commitments,
- surface alignment, drift, and pressure,
- provide non-coercive visibility into system behavior,
- support safe and observable system evolution,
- preserve autonomy while improving understanding.

VDS may use a federated deliberate containing copies of only the resolutions relevant to the software.

Teams may later copy snapshots of the VDS-altered deliberate for analysis.

CGL may explain VDS signals, observations, and drift summaries.

### VLS — Value Lineage System

VLS is a separate host.

VLS is an agent layer on top of CAS, CIS, and CSG.

VLS does not emit its own special commits directly. It uses other substrates and emits their commit types.

VLS exists to:

- make identity, scope, and purpose explicit,
- preserve continuity across change,
- record structural evolution through versioning,
- surface alignment and misalignment of declared intent,
- maintain narratable system history without rewriting the past.

VLS tracks identity and versions identity when scope changes.

VLS supports software and organizational change.

VLS may use deployment-window posture to tell affected VDS agents to reduce cadence temporarily because change is occurring.

VLS helps expose deprecation and sunsetting.

Deprecation and sunsetting preserve history. They do not rewrite or moralize the past.

CGL may summarize VLS lineage, deprecation, sunsetting, identity transitions, and mission-alignment changes.

## CGL Behavioral Boundaries

CGL may assist broadly, but it must remain bounded.

CGL must not:

- create legitimacy,
- decide for the host,
- mutate canonical state,
- infer authority,
- infer intent as fact,
- normalize the host,
- erase divergence,
- rewrite history,
- collapse simulation into legitimacy,
- treat descriptive state as authoritative,
- treat AI output as accepted decision,
- silently choose scope or query intent,
- override substrate semantics.

CGL may:

- explain,
- summarize,
- clarify,
- compare,
- detect possible conflict,
- detect duplicate meaning,
- ask clarifying questions,
- suggest possible next steps,
- help draft user-facing text,
- help construct queries,
- explain CAS outputs,
- assist CDS deliberation,
- assist runtime interaction,
- moderate multi-user deliberation,
- help users understand whether they are following their original intent.

## Output Categories

CGL outputs should be categorized.

Recommended categories:

- Exegesis
- Clarification
- Suggestion
- Description
- Summary
- Comparison
- Conflict notice
- Duplicate-meaning notice
- Query assistance
- CAS explanation
- CDS assistance

The category matters because different output types carry different authority risks.

### Exegesis

Exegesis explains recorded or queryable state.

This should be the default CGL mode.

### Clarification

Clarification helps the host understand ambiguity, missing context, possible intent, or competing interpretations.

Clarification must not pretend uncertainty is resolved.

### Suggestion

Suggestion is allowed for AI assistance.

Suggestions must remain optional, non-authoritative, and host-directed.

A suggestion should be framed as a possible next step, not as a decision.

### Description

Description is appropriate for mechanical errors, hard-to-understand runtime behavior, or technical system explanations.

### Summary

Summary condenses information without changing its authority.

Summaries must preserve source boundaries.

### Conflict Notice

A conflict notice identifies possible inconsistency, contradiction, or unresolved tension.

It must not claim moral failure.

### Duplicate-Meaning Notice

A duplicate-meaning notice identifies cases where different words or structures may express the same underlying meaning.

It must be framed as possible semantic overlap, not automatic equivalence.

## Contracts

CGL should use layered contracts.

The recommended stack is:

1. Legitimacy Canon
2. Phase Contract
3. Tone Contract
4. Markers
5. Model Adapter
6. Guidance Output

### Legitimacy Canon

The legitimacy canon is the first-layer prompt protocol.

It exists to empower the user, preserve truthfulness, and prevent AI from becoming hidden authority.

### Phase Contract

A phase contract controls thinking structure.

Phase contracts are first-class CGL artifacts.

They help constrain reasoning behavior, protect psychological safety, and preserve consistency across interactions.

Examples of phase behavior may include:

- expansion,
- structuring,
- divergence highlight,
- synthesis,
- temporal reflection,
- assumption tracking.

A phase contract controls how CGL thinks with the user.

### Tone Contract

A tone contract controls how information is presented.

Tone contracts are separate from phase contracts.

Tone is about digestibility, emotional posture, directness, warmth, concision, and style.

A user may need the same facts explained in different tones without changing the underlying phase or authority boundary.

### Markers

Markers help CGL track continuity, context-window changes, rehydration points, model changes, or phase transitions.

Markers support epistemic continuity.

They help CGL preserve reasoning posture even when the underlying model or context changes.

## Progressive Determinism

CGL should support progressive determinism.

If the user’s data does not change and the same question is asked under the same contracts, the answer should be substantially similar in meaning.

However, complete deterministic repetition is not the goal.

Exegesis may become clearer over time.

Repeated questions may surface better framing, clearer distinctions, or more helpful explanation.

Progressive determinism means:

Same facts and same question should preserve stable meaning, while allowing increased clarity.

This protects the host from arbitrary AI drift while preserving the usefulness of exegesis.

## Model Adapter Architecture

CGL should support multiple model adapters.

Possible early adapters include:

- local model support,
- Ollama-style local adapter,
- generic API-key adapter,
- later provider-specific adapters.

The adapter boundary should normalize:

- source facts,
- CQL query results,
- canon context,
- phase contract,
- tone contract,
- markers,
- model metadata,
- output category,
- guidance trace metadata.

The model adapter should not own CGL behavior. It should execute within CGL constraints.

## Guidance Traces

CGL should not produce legitimacy receipts.

The word receipt is too close to legitimacy.

The safer concept is a guidance trace.

A guidance trace records how a CGL output was produced without implying that the output is authoritative or accepted.

A guidance trace may include:

- user request,
- interpreted intent,
- phase contract,
- tone contract,
- model adapter,
- CQL queries or source interfaces used,
- source facts referenced,
- output category,
- continuity marker,
- optional host annotation.

Guidance traces should be ephemeral by default.

A host may choose to store a guidance trace as descriptive metadata or a descriptive commit, but it must not become a legitimacy artifact.

## Persistence Rules

CGL outputs are ephemeral by default.

CGL recommendations and summaries may be used as metadata during transmissions.

CGL guidance traces may be stored if the host explicitly chooses to preserve them.

Stored CGL traces or summaries remain descriptive.

They do not become legitimate unless the host separately creates legitimate resolutions through the proper legitimacy workflow.

## CGL and Reconciliation

CGL may assist with Reconciliation Review.

It may help explain candidate items, source context, conflict, duplication, missing context, or possible derived relationships.

CGL must not perform Reconciliation Review for the host.

CGL must not decide which items become resolutions.

The host explicitly selects what becomes legitimate.

## CGL and Reverse Reconciliation

CGL may explain reverse reconciliation.

Reverse reconciliation takes resolutions and ensures related CDS items are marked applied or closed so they do not accidentally derive duplicate resolutions during later Reconciliation Review.

Reverse reconciliation creates a relationship to the resolution before changing item status.

CGL may help identify candidate items that may need reverse reconciliation, but it must not close or apply them without host action.

## CGL and Simulation

CGL may assist with simulation inside CDS.

Simulation may copy an affected partial graph into item space.

The host may add observations to simulated nodes.

A CDS observation item may represent a state change when applied to a simulated item.

Historical signals or observation-derived signals may be replayed.

CAS may calculate cascade consequences.

The host may modify the item graph to explore alternatives.

CGL may explain the simulation, compare alternatives, or help summarize consequences.

CGL must always preserve the fact that simulation is not legitimacy.

## CGL and CSP

CGL may query CSP when useful.

CSP is the pipeline and feed layer.

CGL may explain:

- how many raw signals came through,
- how signals were filtered,
- how signals were clustered,
- what feed was used,
- what cadence was configured,
- what thresholds were reached,
- what descriptive commits were emitted.

Because CSP acts as a filter and live view, it may be more efficient in some cases for CGL to query through CSP rather than directly querying the commit store.

## CGL and CAS

CGL may summarize and explain CAS responses.

CAS remains the owner of alignment calculations and semantic projections.

CGL explains what CAS returned.

CGL should identify:

- whether the result is local or federated,
- whether the result is authoritative or simulation-only,
- what identity boundary was used,
- what area or graph was queried,
- what resolution or relationship was affected,
- what signals contributed,
- what historical snapshot was used,
- what next inspection may be useful.

CGL must not invent CAS results.

## CGL and Runtime/CDS Priority

The highest-value early CGL integration points are:

1. Runtime assistance.
2. CDS deliberation assistance.
3. CAS explanation.

Runtime and CDS should come first because they are direct host interaction points.

CAS explanation should follow because CAS outputs are likely to be powerful but hard to understand without a human-facing explanatory layer.

## Recommended Initial Implementation Slice

The first implementation slice should not attempt full Charter-wide CGL.

A narrow first slice should include:

- CQL dependency boundary,
- one model adapter,
- phase contract support,
- tone contract support,
- marker support,
- runtime or CDS text assistance,
- output categories,
- ephemeral guidance trace,
- exegesis,
- clarification,
- suggestion.

The second slice should add CAS explanation.

The third slice should add CSP/feed explanation and deeper CDS simulation support.

## Documentation Work Needed

### 1. Update CGL Foundation Spec

Add explicit language that CGL is a standalone guidance substrate/library in the Charter ecosystem.

Add the two operating modes:

- Charter-integrated mode,
- standalone adapter mode.

Add runtime, CDS, and CAS as primary usage locations.

### 2. Update Module Boundaries

Add CGL to module boundaries.

CGL owns:

- guidance behavior,
- exegesis,
- clarification,
- phase contracts,
- tone contracts,
- markers,
- model adapter boundaries,
- guidance traces,
- CAS explanation,
- runtime/CDS assistance.

CGL does not own:

- legitimacy,
- graph structure,
- identity truth,
- alignment metrics,
- signal truth,
- federation trust,
- reconciliation decisions,
- canonical state mutation.

### 3. Update Canonical Naming

Add CGL-specific terms:

- CGL,
- guidance,
- exegesis,
- phase contract,
- tone contract,
- marker,
- guidance trace,
- model adapter,
- progressive determinism,
- clarification,
- suggestion,
- duplicate meaning,
- conflict detection,
- CAS explanation.

### 4. Promote or Rename Archived AI Guidance Docs

Older AI guidance documents should be treated as archive/source ancestor material unless rewritten with CGL terminology.

Possible mapping:

- AI assistance overview becomes CGL overview ancestor.
- AI guidance defense becomes CGL design defense.
- AI guidance philosophy becomes CGL exegesis philosophy.
- AI guidance heuristics becomes CGL heuristic lens spec.
- AI guidance phases becomes CGL phase contract spec.
- AI guidance invariants becomes CGL invariants.

### 5. Define CGL Contract Specs

Create focused specs for:

- phase contracts,
- tone contracts,
- markers,
- model adapters,
- guidance traces,
- output categories.

### 6. Define CGL-CQL Interface

Specify how CGL queries CQL.

Define:

- query input format,
- JSON IL expectations,
- human DSL compilation support,
- source attribution,
- authority labels,
- simulation labels,
- federation/provenance labels,
- snapshot labels.

### 7. Define CGL Safety Invariants

CGL invariants should include:

- never create legitimacy,
- never mutate canonical state,
- never infer authority,
- never infer user intent as fact,
- never collapse simulation into legitimacy,
- never treat suggestion as decision,
- never treat summary as source truth,
- preserve identity boundaries,
- preserve source authority labels,
- preserve uncertainty,
- prefer clarification over assumption.

## Architecture Feedback

The architecture is coherent because it separates four major concerns:

1. Legitimacy.
2. Deliberation.
3. Derived analysis.
4. Explanation.

This is the key strength.

Many AI systems fail because explanation slowly becomes authority.

Charter avoids that by making the substrates own truth and letting CGL own explanation behavior.

The most important architectural separation is:

- Resolutions are legitimate.
- CDS items are deliberative or simulation units.
- CCare signals are feedback.
- CSP feeds are live signal views.
- CAS snapshots are authoritative derived historical commits.
- CGL outputs are ephemeral guidance unless explicitly stored as descriptive metadata.

That separation protects the host.

The main design risk is semantic overload.

Many concepts are close to each other:

- resolution vs item,
- area vs CDS sandbox,
- signal vs observation,
- CAS snapshot vs CGL summary,
- suggestion vs exegesis,
- reconciliation vs reverse reconciliation,
- federation vs legitimacy,
- identity boundary vs graph boundary,
- posture vs signal,
- descriptive commit vs legitimate commit.

The next major documentation goal should be boundary discipline.

Every substrate spec should clearly answer:

- What does this substrate own?
- What does it read?
- What does it emit?
- What is authoritative?
- What is descriptive?
- What is derived?
- What is simulation-only?
- What must pass through Reconciliation Review?
- What must never be inferred?

## Final Positioning Statement

CGL is the standalone guidance and exegesis substrate of the Charter ecosystem.

When used inside Charter, it reads CQL-available state from the runtime, CDS, CAS, and other substrates to help hosts understand decisions, signals, conflicts, simulations, lineage, and alignment.

When used outside Charter, it acts as a CQL-backed AI adapter that constrains model behavior through canon, phase contracts, tone contracts, markers, and progressive determinism.

CGL’s purpose is not to decide.

CGL’s purpose is to help the host understand, clarify, reflect, and act with agency.