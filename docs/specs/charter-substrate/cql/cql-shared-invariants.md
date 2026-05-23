# Shared CQL Invariants

Status: FOUNDATIONAL  
Applies to: CQL Foundation Specification, JSON IL Specification, DSL Specification, Adapter Contract Specification  
Depends On: Charter substrate ownership model, determinism principle, non-interpretation principle, managed read surface model  

---

# 1. Purpose

This section defines the shared invariants that apply to all CQL representations and execution paths.

These invariants apply whether a query originates from:

- JSON Intermediate Language
- CQL DSL
- SDK builder APIs
- CLI input
- host integrations
- substrate adapters
- future human-facing query tools

CQL must remain a deterministic, read-only, substrate-neutral query access layer.

CQL standardizes how queries are expressed, validated, routed, and shaped.

CQL does not own the truth, meaning, derivation, or authority of the substrates it queries.

---

# 2. Foundational Invariant

CQL queries are deterministic read-only access requests.

JSON Intermediate Language is the canonical representation.

The DSL is a human-authoring syntax that compiles into JSON Intermediate Language.

Neither layer may mutate state, infer authority, redefine substrate meaning, or bypass adapter-declared capabilities.

---

# 3. Shared Invariants

## CQL-INV-01 — Read-Only Access

A CQL query must never mutate substrate state.

CQL may read from substrate-owned or host-exposed read surfaces.

CQL must not:

- create legitimacy
- alter graph truth
- modify identity truth
- emit observations
- change CAS outputs
- repair data
- trigger workflows
- enqueue state-changing commands
- perform write-side reconciliation

CQL exists to expose read access, not to perform action.

---

## CQL-INV-02 — Access, Not Meaning

CQL may expose substrate-owned data, views, and derived outputs.

CQL must not redefine what those outputs mean.

Substrate ownership remains authoritative:

- Charter owns legitimacy, resolutions, sessions, authority, and supersession semantics.
- CSG owns graph structure and topology.
- CIS owns identity truth and identity boundaries.
- CCare owns observations, check-ins, confidence, and timestamps.
- CAS owns semantic condition, derived alignment state, and alignment dynamics.
- CSP owns signal processing and feed emission behavior.
- CRS owns relay/federation behavior.
- Host systems own host-specific extension surfaces.

CQL standardizes access.

It does not standardize substrate truth.

---

## CQL-INV-03 — Deterministic Query Meaning

Given the same query, same adapter capability declaration, same substrate state, and same execution context, a CQL query must have the same meaning.

The following query components must be deterministically interpreted:

- domain
- subject
- target
- scope
- filters
- output
- context
- metadata

CQL must not allow hidden interpretation, ambient guessing, or host-specific semantic drift outside declared capabilities.

---

## CQL-INV-04 — Explicit Projection

When a query crosses representational layers, the projection must be explicit.

Examples of projections may include:

- resolution
- item
- commit
- graph
- signal
- observation
- snapshot

CQL must not silently mix projections.

A query over resolutions must not accidentally return items unless that behavior is explicitly modeled.

A query over items must not accidentally reinterpret them as legitimacy resolutions.

Mixed live graphs are not the default model.

Projection is part of query meaning and must be visible to validation.

---

## CQL-INV-05 — Scope Before Filter

Scope defines the visible slice of data.

Filters constrain data within that visible slice.

A filter must never expand scope.

Examples of scope include:

- active
- historical
- since
- until
- projection
- window
- current_round
- include_superseded

Examples of filters include:

- state
- confidence
- volatility
- relationship_type
- provenance
- semantic_state
- participant
- identity

Scope answers:

“What data is visible to this query?”

Filters answer:

“Within that visible data, what constraints apply?”

---

## CQL-INV-06 — Capability-Validated Execution

A CQL query must be validated against the selected domain or adapter before execution.

Adapters must declare what they support.

Validation must reject unsupported:

- domains
- subjects
- views
- raw fields
- target kinds
- scope fields
- filters
- output modes
- extension views
- argument names
- argument values
- projection modes

Unsupported query features must produce explicit validation errors.

They must not be ignored silently.

---

## CQL-INV-07 — Extension Surfaces, Not Syntax

Hosts and substrates may extend CQL by exposing managed read surfaces.

They must not extend CQL by changing the core grammar or canonical query model.

Extension mechanisms may define:

- extension views
- extension raw surfaces
- extension filters
- extension scope dimensions
- extension output modes
- extension metadata

Extension mechanisms must not:

- redefine canonical fields
- override core grammar
- change query semantics
- introduce mutation
- add implicit joins
- bypass capability validation

Extensions add surfaces.

They do not add new language semantics.

---

## CQL-INV-08 — No Joins in V1 Query Language

CQL V1 must not expose general joins, user-defined algebra, or arbitrary multi-view composition syntax.

Managed read surfaces may internally compose data.

Externally, composed surfaces must appear as declared views or raw surfaces.

This means CQL users may query:

- a CAS posture view
- a CSG graph view
- a host-defined operational summary view
- a CDS deliberation read surface

But CQL V1 should not let users freely join those surfaces together inside the query language.

Composition belongs behind managed read surfaces.

It does not belong in the V1 query grammar.

---

# 4. Enforcement Requirements

All CQL implementations must enforce these invariants during:

- DSL parsing
- DSL compilation
- JSON Intermediate Language validation
- SDK builder validation
- adapter registration
- adapter dispatch
- result envelope construction
- extension registration

An implementation that accepts invalid, ambiguous, unsupported, or mutating query behavior is not compliant with CQL.

---

# 5. Design Consequence

CQL must remain thin but formal.

It should be strong enough to provide common read access across Charter substrates.

It should not become a hidden semantic authority.

The core contract is:

CQL owns query expression, validation, routing, and result access.

Substrates own meaning.