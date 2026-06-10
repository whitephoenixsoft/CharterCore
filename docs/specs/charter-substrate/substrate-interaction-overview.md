# Charter — Substrate Interaction Overview
Status: FOUNDATIONAL (DRAFT)
Applies to: Charter Runtime, Charter Substrates, Hosts, Integrations, CAS Context
Depends On: Charter Legitimacy, CCS, CSG, CIS, CDS, CCare, CSP, CAS, CRS, CQL, CGL
Does NOT define: full implementation details, formulas, storage backends, UI behavior, or host-specific workflows in detail

---

# 1. Purpose

This document provides a high-level overview of how Charter substrates work together.

It exists to:

- explain the shared purpose of the substrate ecosystem
- define the role of each major substrate
- clarify how truth, observation, deliberation, and interpretation interact
- show how commits, graphs, signals, and hosts fit together
- provide a stable architectural overview before deeper specification work

This document is intentionally broad.

It is meant to anchor understanding across the platform rather than replace individual substrate specifications.

---

# 2. Unifying Goal

Charter is the name of the substrate ecosystem as a whole.

Its unified goal is:

- to act as a decision journal with strong history
- to promote exegesis with clarity
- to support understanding without shame
- to preserve agency rather than normalize decisions
- to help humans and systems think with less drift

Every substrate contributes to that goal in a different way.

No single substrate is the whole system.

---

# 3. Core Architectural Principle

> Charter preserves distinct kinds of truth and understanding without collapsing them into one layer.

The platform keeps separate:

- legitimacy
- deliberation
- observation
- structure
- identity
- signal shaping
- derived condition
- querying
- exegesis

These layers may relate closely.

They must not be confused.

---

# 4. Runtime Role

The runtime orchestrates the substrates.

It is persistent-agnostic and may run in memory as a long-lived process.

The runtime connects the ecosystem and embeds the legitimacy engine.

In that sense, the runtime is one of the main ways Charter exists as a whole.

The runtime is responsible for coordination, not conceptual ownership of every substrate concern.

---

# 5. Major Truth Classes

The platform contains several distinct truth classes.

## 5.1 Legitimacy Truth

Legitimacy truth answers:

- what was decided
- where it was decided
- under what decision rule
- how decision history evolved

This is the domain of resolutions, areas, supersession, and legitimacy history.

---

## 5.2 Deliberative Truth

Deliberative truth answers:

- what was explored
- what was investigated
- what was observed in thinking space
- what possible conclusions or proposals were produced

This is the domain of CDS items, observations, breakouts, and deliberates.

---

## 5.3 Structural Truth

Structural truth answers:

- how nodes relate
- what graph shape exists
- what paths and partitions matter

This is the domain of CSG.

---

## 5.4 Identity Truth

Identity truth answers:

- who or what a region claims to be
- where boundaries exist
- how identities version and change

This is the domain of CIS.

---

## 5.5 Observational Truth

Observational truth answers:

- what was observed
- when it was observed
- with what confidence or context it was emitted

This is the domain of CCare and related upstream observation models.

---

## 5.6 Derived Condition

Derived condition answers:

- what appears to be happening
- how condition is distributed
- whether it is stable, strained, degrading, or unclear
- what patterns matter structurally and dynamically

This is the domain of CAS.

---

# 6. Legitimacy Layer

The legitimacy engine is responsible for legitimate decisions.

Its core legitimacy unit is the resolution.

Resolutions:

- are append-only in history
- evolve through supersession rather than destructive replacement
- may relate to other resolutions
- exist inside areas

Areas are bounded legitimacy contexts.

An area includes:

- scope
- decision rule

Areas exist because not all decisions share the same authority model or focus.

This is why a dedicated legitimacy engine or calculator is needed.

---

# 7. CCS — Commit Backbone

CCS is the commit substrate.

It acts as the central hub for durable substrate communication and archival trace.

A commit is the central durable wrapper used for many substrate artifacts.

CCS holds commits for all substrates, even though many substrates mainly consume from it rather than add to it directly.

CCS is important because it enables:

- durable communication
- archival history
- correlation
- indexing of related items
- federation through durable units

CCS is one of the main platform backbones.

---

# 8. CDS — Deliberation Substrate

CDS is the substrate for non-legitimate thinking.

Its core unit is the item.

An item is intentionally amorphous.

This allows CDS to support:

- exploration
- investigation
- breakout thinking
- long-running issues
- simulation
- copied or federated thinking spaces
- observation records
- proposal generation for later legitimacy

CDS is a workspace, not a legitimacy system.

A deliberate is a workspace for an epic or line of inquiry.

A deliberate may remain open indefinitely, may close when the issue is over, or may close when the lifetime of tracked resolutions is over.

CDS does not require final legitimacy.

It supports thinking whether legitimacy follows or not.

---

# 9. Observation in CDS

One important item subtype is observation.

Observation items may record facts such as:

- blockers
- exceeded thresholds
- concrete instances of problems
- milestone conditions used for replay

Observation items are investigatory and structured.

They are not the same as CCare signals.

They belong to deliberation space.

They may relate to copied items representing resolutions or other CDS items.

---

# 10. Reconciliation Review

Reconciliation review is the controlled bridge into trusted legitimacy.

It is used when non-legitimate or external material may become legitimate.

This matters for:

- CDS proposals becoming legitimacy
- external material crossing in through federation
- trusted adoption of imported or exploratory results

Reconciliation review is central to trust.

It prevents uncontrolled crossing into legitimacy.

---

# 11. Reverse Reconciliation

Reverse reconciliation is used after downstream work has already been applied through legitimacy or other trusted action.

It creates a relationship to the resulting resolution before changing the related CDS item status.

This relationship uses `derived_from`.

This matters because later reconciliation review must ignore already-applied material correctly.

Reverse reconciliation allows exploratory items to be put to bed without losing continuity.

---

# 12. Continuity Through `derived_from`

`derived_from` is one of the most important continuity relations in the ecosystem.

It is used for:

- copied resolution to item continuity
- reverse reconciliation continuity
- recontextualization continuity
- other cross-space lineage-preserving transformations

This relation preserves traceability across substrate boundaries and trust contexts.

It is not limited to one substrate.

---

# 13. CSG — Canonical Graph Substrate

CSG is the canonical graph substrate.

It is responsible for all graph structure related work.

CSG builds graph structure from commits only, at least for now.

It does not consume whole graph objects directly.

CSG may build:

- one graph representing all areas together
- sparse graphs
- separate item and resolution graphs

Both legitimacy and deliberation feed CSG.

However:

- item graphs
- resolution graphs

remain separate.

Items may point to resolutions, but those cross-class relationships are ignored when the graph is created.

This preserves separate graph classes.

CSG is a second major platform backbone.

---

# 14. CIS — Identity Substrate

CIS consumes the CSG graph and adds identity boundaries to nodes.

Identities may be:

- nested
- overlapping
- shared-node
- versioned

Because graphs may become highly connected, hosts must sometimes define where an identity ends even when multiple relationships exist.

CIS manages:

- identity boundaries
- identity versions
- identity scope overlays
- coexistence across versions
- identity-related transition context

An identity is a commit type.

CIS truth can be federated because it is represented through CCS.

---

# 15. Recontextualization

Recontextualization changes abstraction tier.

It may:

- promote
- demote
- split
- join
- move

a node across abstraction tiers, including within the same tier when needed.

Abstraction tier is part of the commit.

It is not a separate graph-only concept.

After recontextualization, `derived_from` preserves continuity.

This feature matters for:

- structural evolution
- interpretation across tiers
- historical explanation
- later analysis in CAS

---

# 16. CRS — Relay and Federation

CRS is the relay substrate for federation between systems.

It uses commits for sending and receiving.

Anything coming in through federation must enter an untrusted store until reconciliation review occurs where trust crossing matters.

This is especially important for legitimacy.

Federation allows:

- signals to be shared
- resolutions to be shared
- identities to be shared
- other commit types to be shared
- snapshots to be shared

Agency and cadence remain up to the host or team.

Federation is optional, not assumed.

---

# 17. Flow Observation Model

Flow Observations describe how work moves through the system.

They are not semantic and do not prescribe action.

They exist to represent:

- movement
- delay
- obstruction
- dependency pressure
- execution friction

They may apply to:

- a resolution
- a dependency relationship
- a path
- a bounded region

Flow Observations are represented in CDS as observation items.

They are grounded in structure.

They support later signal generation and investigation.

---

# 18. CCare — Care Signal Substrate

CCare is its own substrate.

It is responsible for defining the signals and allowing hosts to emit them.

CCare owns care/check-in style observational input.

These signals are descriptive and non-authoritative.

They may later be consumed by CAS.

CCare does not own:

- final semantic projection
- dynamic analysis
- legitimacy
- structural truth

It owns signal definition and emission.

---

# 19. CSP — Charter Signal Processing

CSP is the non-authoritative signal shaping substrate.

It exists to:

- process high-frequency or noisy inputs safely
- cluster and aggregate signals
- control cadence and throughput
- publish feeds
- support ingress and egress flow control
- bridge raw observation and structured care

CSP can create clustered aggregated signals separated by type while preserving details as metadata.

It may push outward through CRS or inward through feeds.

Its purpose is to prevent the commit store and the rest of the ecosystem from being overwhelmed by noise.

Feeds are transient monitoring channels.

Commits are durable records.

Pipelines may read commits in order to publish to feeds.

---

# 20. Relationship Between Flow, CSP, and CCare

A common path is:

- real-world or system behavior
- structured observation or metrics
- flow observation or other upstream observation
- CSP shaping, clustering, cadence control
- CCare signal emission
- durable commits and/or feeds
- downstream interpretation by CAS or further filtering by CDS

This path allows the system to remain usable in both sparse human contexts and high-output automated contexts.

---

# 21. CAS — Condition and Interpretation Substrate

CAS builds on CSG and may also use CIS.

It can analyze both:

- resolution graphs
- item graphs

The default is to apply the same analysis to both, though simulation-sensitive variations may be useful later.

CAS is responsible for:

- intake and derivation of condition
- propagation across scope
- structural detection
- alignment dynamics
- semantic projection
- snapshots of derived state

CAS does not create legitimacy.

CAS explains condition.

---

# 22. Temporal Behavior in CAS

CAS must account for time, not just structure and explicit conflict.

It must consider:

- persistence
- recency
- continuity
- silence
- stale reinforcement
- prolonged friction
- loss of feedback

This matters because a system can degrade through:

- sustained friction
- lack of progress
- absence of reinforcement

not only through overt conflict.

Temporal behavior affects:

- confidence
- trend
- stability
- interpretation

It must not automatically imply misalignment.

---

# 23. Structural Detection in CAS

CAS includes structural detection because graph shape matters.

Structural detection helps explain:

- bottlenecks
- concentration
- isolation
- fragmentation
- supersession-sensitive structure
- boundary-sensitive structure

This is one reason CSG is essential.

CAS does not merely attach semantics to nodes.

It interprets condition over structure.

---

# 24. CAS Snapshots

CAS snapshots may be emitted as CCS commit types.

Their original purpose is to preserve consistent views of derived state across federation and cadence boundaries.

They become even more useful for:

- distributed communication of derived condition
- avoiding unnecessary recalculation across partial systems
- simulation initialization
- replay setup
- milestone state transfer

CAS may analyze only part of the graph locally.

Federated snapshots help communicate derived condition without assuming every instance sees the whole graph.

---

# 25. Simulation Through CDS and CAS

Simulation operates in CDS using items only.

A host may:

- copy the affected partial graph into item form
- add observation items to nodes
- replay historical signals to items
- replay observation-derived signals in original sequence
- jump directly to milestone states using observation items
- inspect cascade consequences
- modify the item graph to test alternative configurations

This creates a replay and debugging space that preserves continuity with real system history while remaining sandboxed.

CAS can then analyze the simulated item graph.

---

# 26. Hosts

Charter is designed to support multiple hosts.

The main planned hosts include:

- CLI for human use
- VDS for software telemetry interpretation
- VLS for identity and continuity-aware software/system evolution

Other future hosts are possible.

Hosts use Charter as a library ecosystem.

They do not replace the substrate model.

---

# 27. VDS — Value Driven System Host

VDS is an agent or host layer tied to one or more applications.

It interprets telemetry through the lens of explicit commitments.

Teams define:

- how the application supports mission goals
- which metrics map to which signal types
- threshold conditions for signal emission

When thresholds are reached, VDS emits signals through Charter substrates.

VDS exists to:

- interpret telemetry through explicit commitments
- surface alignment, drift, and pressure
- provide non-coercive visibility into system behavior
- support safe observable evolution
- preserve autonomy while improving understanding

VDS uses Charter as a library.

It is not the same thing as CCare or CSP, though it uses both.

---

# 28. VLS — Value Lineage System Host

VLS is an agent or host layer focused on identity, scope, and continuity across change.

It exists to:

- make identity, scope, and purpose explicit
- preserve continuity across change
- record structural evolution through versioning
- surface declared intent relative to evolving identity
- maintain narratable history without rewriting the past

VLS uses identity actively through CSG and CIS.

It also supports deployment posture and transition awareness.

Examples include:

- deprecation of decisions that remain active but no longer support mission
- sunsetting of older identity versions
- transition windows during major system change

Posture changes are also a way for VLS to control noise.

VLS can instruct VDS agents to signal less and use greater tolerance during sensitive transition periods.

---

# 29. Human Team and Organization Use

A team may use Charter to track:

- governance decisions
- software intent
- mission-supporting resolutions

These usually live in different areas because areas differ in scope and decision rule.

Leaders, departments, and teams may define goals in different areas and relate them through resolution relationships.

Teams may define team and department identities through CIS.

People then use CCare through the CLI to send care signals about how they align to different governance decisions or goals.

These signals are usually sparse and human-centered.

Later, teams may choose to federate signals or resolutions through CRS.

An organization can then rebuild broader structure through CSG and use CAS over the federated version to track mission condition.

Teams can still use CAS locally.

This supports local and federated understanding without forcing a single whole-graph assumption.

---

# 30. Problem Resolution Through CDS

If a team sees an alignment problem, they may create a deliberate using selected signals or resolutions as the basis.

They can:

- copy the relevant resolution into item form
- preserve continuity to the original via `derived_from`
- create observation items about the problem
- research selected signal ranges
- document concrete instances, blocker conditions, or threshold violations
- share or federate the deliberate
- let other teams work on separate copies
- later reconcile resulting proposals into legitimacy
- reverse reconcile old items into applied status when appropriate

This allows complex and long-running issues to be handled collaboratively without forcing immediate legitimacy.

---

# 31. CQL — Charter Query Language

CQL is the central read-only query language.

It sits above substrates.

It provides a unified way to query:

- resolutions
- items
- graph structure
- identities
- signals
- CAS views and outputs
- other substrate data

CQL especially affects CAS because CAS needs rich querying of:

- views
- scopes
- filters
- derived outputs

The intended architecture includes:

- JSON IL as the canonical machine form
- a DSL for human users that compiles to JSON IL

CQL defines access, not substrate truth.

---

# 32. CGL — Charter Guidance Layer

CGL is the read-only AI exegesis layer.

It uses CQL to help the host understand the system objectively when requested.

CGL does not make decisions.

CGL does not create legitimacy.

CGL exists to:

- explain
- clarify
- summarize
- support exegesis with less drift

It remains read-only and non-authoritative.

---

# 33. Example Long-Term Vision

In a high-output scenario, Charter could receive signals from many components of a complex system.

A host could then:

- show broad system status through CAS
- create CDS deliberates for affected subsystems
- attach feeds for ongoing health observation
- let automated or human agents investigate and resolve issues
- use snapshots and replay to understand changes over time

This is one example of how the substrate ecosystem may support much broader coordination later.

The platform is intended as a primitive that may support more kinds of systems than originally expected.

---

# 34. Why This Architecture Matters

Charter matters because it preserves distinctions that many systems collapse.

It keeps separate:

- decision history
- exploration
- observation
- structure
- identity
- signal shaping
- derived condition
- explanation

This allows the system to support:

- people
- teams
- organizations
- software applications
- federated systems
- future AI-assisted workflows

without losing agency or rewriting history.

---

# 35. Mental Model

A useful mental model is:

- the runtime orchestrates the ecosystem
- CCS preserves durable communication
- legitimacy preserves explicit authoritative decisions
- CDS preserves exploratory and investigatory history
- CSG builds structural reality
- CIS defines identity scope over that structure
- CCare defines care/check-in signals
- CSP shapes noisy signal streams
- CRS federates durable artifacts
- CAS derives and explains condition
- CQL provides read-only access
- CGL provides read-only exegesis
- hosts make the ecosystem usable for humans and systems

---

# 36. Final Constraint

This document exists to answer:

- what the major Charter substrates are
- what each substrate contributes
- how they interact
- how truth moves through the ecosystem
- how observation becomes signal
- how signal becomes condition
- how deliberation, legitimacy, federation, and interpretation relate

It must not be used to erase substrate distinctions or collapse the ecosystem into one simplified layer.