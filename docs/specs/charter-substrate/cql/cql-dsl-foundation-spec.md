# Charter Query Layer (CQL) — Foundation Specification (Revised vNext)

Status: FOUNDATIONAL  
Applies to: All Charter-readable substrates and stores (Runtime, Legitimacy, CCS, CSG, CIS, CAS, CCare, CDS, optional Audit)  
Depends On: Charter substrate specifications (Runtime Layer, Review Layer, Legitimacy Engine, CCS, Commit Store, CSG, CIS, CCare, CDS, CAS)  
Does NOT define: transport syntax, UI rendering, storage backends, computation semantics, or host language bindings  

---

# 1. Purpose

This document defines a minimal, stable query model for accessing **all Charter-readable state**.

CQL exists to:

- query raw substrate data  
- query built-in and derived views  
- uniformly access multiple substrates  
- scope queries across structure, identity, runtime, and alignment  
- expose traceability and provenance through audit queries  
- support host-defined extension views  

CQL is:

- read-only  
- deterministic  
- substrate-agnostic  
- composable  
- selector-oriented  

It is not:

- a programming language  
- a computation engine  
- an interpretation layer  
- a mutation interface  

---

# 2. Core Principle

> A CQL query selects a view or field set over a target, within a domain, under a scope, with optional filters and context.

CQL must remain:

- simple enough for CLI and JSON use  
- structured enough for library embedding  
- stable enough for cross-substrate use  
- extensible without breaking core semantics  

---

# 3. Query Model

A CQL query consists of six conceptual parts:

1. domain  
2. subject  
3. target  
4. scope  
5. filters  
6. output  

Not all parts are required in every query.

---

## 3.1 Domain

The domain defines which substrate or read surface is being queried.

Supported domains include:

- `runtime`
- `legitimacy`
- `ccs`
- `csg`
- `cis`
- `cas`
- `ccare`
- `cds`
- `audit` (optional)

Domains define:

- available targets  
- available views  
- available filters  

---

## 3.2 Subject

The subject defines what is being requested.

Two subject classes exist:

- `view`
- `raw`

Subjects are interpreted within the selected domain.

---

## 3.3 Target

Defines where the query applies.

May include:

- `resolution`
- `area`
- `identity`
- `global`
- `pair`
- `deliberate`
- `item`
- `session`
- `review`
- `signal`
- `receipt`
- `commit`

---

## 3.4 Scope

Constrains the slice of data.

Common dimensions:

- activity (`active`, `historical`)  
- time (`since`, `until`)  
- posture  
- structural mode (`local`, `global`)  
- lifecycle state  
- projection (`resolution`, `item`)  

### Projection Principle

> CQL operates over explicit projections.

Canonical projections:

- `projection=resolution`
- `projection=item`

Mixed live graphs are not the default model.

---

## 3.5 Filters

Domain-specific constraints such as:

- semantic state  
- lifecycle state  
- volatility  
- confidence  
- relationship type  
- provenance  

---

## 3.6 Output

Controls result form:

- `summary`
- `structured`
- `detailed`

---

# 4. Canonical Query Shape
```
domain subject on target [scope] [filters] [output]
```
Examples:

- `domain:cas view:posture on area:payments active projection=resolution`
- `domain:cds view:items on deliberate:delib_123 state=locked`
- `domain:audit view:provenance on resolution:res_123 detailed`

---

# 5. Domain Profiles

(Condensed for brevity — unchanged in structure, expanded views retained)

---

# 6. Audit Domain (Traceability)

Audit provides:

- provenance  
- receipt chains  
- federation origin  
- action history  
- review/session lineage  

### Principle

> Audit explains how artifacts came to exist.  
> It does not redefine structural or legitimacy truth.

---

# 7. Extension Model (Revised)

## 7.1 Purpose

CQL supports **host-defined extension views**.

These allow hosts to:

- expose custom metrics  
- combine Charter and host data  
- present composite read surfaces  
- provide domain-specific operational views  

---

## 7.2 Extension View Naming
```
view:x...
```
Examples:

- `view:x.cas.myhost.ops_surface`
- `view:x.cds.myhost.investigation_panel`
- `view:x.audit.myhost.provenance_bundle`

---

## 7.3 Capabilities

Extension views may:

- include canonical Charter fields  
- include host-defined fields  
- compute host-defined metrics  
- combine multiple internal data sources  
- expose composite surfaces (pseudo-joins)

They may behave like:

- derived views  
- raw-like field bundles  
- operational dashboards  

---

## 7.4 Composition Principle

> Extension views may combine Charter and host data, but must remain externally atomic.

CQL does not support:

- joins  
- multi-view composition  
- user-defined query algebra  

Composition occurs inside the extension view, not in the query language.

---

## 7.5 Execution Model (NEW)

> Extension views are resolved at query time through host-defined handlers.

### Flow

1. CQL parses query  
2. Identifies extension view  
3. Resolves registered handler  
4. Invokes handler with normalized query context  
5. Returns structured result  

---

## 7.6 Handler Contract

A host-defined extension view must provide a handler that receives:

- parsed target  
- parsed scope  
- filters  
- output mode  

It may access:

- Charter read interfaces (CSG, CAS, CDS, etc.)  
- host systems (databases, services, caches)  

It returns:

- a deterministic, read-only result  

---

## 7.7 Data Source Flexibility

Extension views may be backed by:

### A. Query-Time Computation (Default)
- compute results on demand  
- simplest implementation  

### B. Cached / Materialized Data
- precomputed surfaces  
- improved performance  

### C. Streaming / Feed-Based Systems
- real-time pipelines  
- optional, not required  

### Principle

> CQL does not require any specific data delivery model.

---

## 7.8 Filtering Behavior

Filtering for extension views:

- is defined by the host  
- must be explicitly supported per view  
- must be deterministic  

Unsupported filters must be rejected explicitly.

---

## 7.9 Restrictions

Extension views must:

- be read-only  
- be deterministic  
- be namespaced  
- not redefine canonical views  
- not redefine canonical raw fields  
- not modify query grammar  

---

## 7.10 Non-Goals

CQL does NOT support (V1):

- host-defined domains  
- host raw fields as first-class query subjects  
- joins  
- view composition syntax  
- computed expressions in query language  

---

# 8. Determinism Rules

Given identical:

- domain  
- subject  
- target  
- scope  
- filters  
- output  

Results must be identical.

---

# 9. Relationship to Hosts

CQL is host-agnostic.

Hosts:

- may expose extension views  
- must not alter core semantics  
- may control internal execution strategy  

---

# 10. Mental Model

CQL answers:

- what exists  
- what is connected  
- what is happening  
- how something came to exist (audit)  

Extension views answer:

- what the host cares about specifically  

---

# 11. Final Principle

CQL provides a stable, minimal query surface across Charter.

It ensures:

- separation of concerns  
- deterministic access  
- structural clarity  
- extensibility without grammar mutation  

Hosts extend **surfaces**, not **syntax**.

This preserves:

- simplicity  
- composability  
- long-term stability  

while allowing practical, real-world flexibility.