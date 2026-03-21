# ENG-IMPORT — Import Submission, Restore Entry & Historical Graph Intake

Status: REFACTORED (v4 – Reference-Driven Model)  
Applies to: Engine Core (V1/V2+)  
Scope: Import boundary, restore submission, and historical graph intake

Authority: Import boundary specification subordinate to ENG-API, ENG-INITIALIZATION, and ENG-INTEGRITY.

Subordinate references consumed from:

- ENG-DOMAIN
- ENG-API
- ENG-INITIALIZATION
- ENG-INTEGRITY
- ENG-RECEIPT
- ENG-CANON
- ENG-SPECVERIFY
- ENG-SUPERSESSION
- ENG-REVIEW-RETIRED
- ENG-COMPILATION

---

# 1. Purpose

ENG-IMPORT defines how external domain graphs are presented to the Engine for:

1. Area Restore
2. Historical Incremental Intake

It is the authoritative specification for:

- the import boundary between host and Engine
- import mode distinctions
- host vs Engine responsibility separation for import
- requirements for imported historical legitimacy artifacts
- the rule that import itself does not create legitimacy
- the rule that runtime activation occurs only through rehydration / initialization

ENG-IMPORT does not redefine:

- object schemas
- structural validation doctrine
- runtime halt conditions
- receipt schema
- canonical serialization rules
- specification identity semantics
- supersession graph semantics
- session acceptance rules
- incremental compilation replay algorithm

Those are defined respectively in:

- ENG-DOMAIN
- ENG-INTEGRITY
- ENG-RECEIPT
- ENG-CANON
- ENG-SPECVERIFY
- ENG-SUPERSESSION
- ENG-DECISION
- ENG-COMPILATION

ENG-IMPORT defines how historical facts are submitted.  
Other specifications decide whether those facts are structurally admissible and operationally usable.

---

# 2. Core Principle

## ENG-IMPORT-01 — Import Does Not Create Legitimacy

Import operations must not create legitimacy.

Imported legitimacy artifacts must already exist as finalized historical artifacts.

Import may only introduce previously created historical objects into an Engine context.

The Engine must not during import:

- simulate acceptance
- generate new Resolutions
- generate new receipts
- infer missing governance
- reconstruct missing legitimacy artifacts

If imported legitimacy cannot already be proven from the submitted historical graph, import must fail or remain non-activating according to the authoritative validation outcomes consumed elsewhere.

---

# 3. Engine vs Host Responsibilities

## ENG-IMPORT-02 — Host Responsibilities

The host is responsible for:

- reading and writing import media
- selecting import mode
- validating transport envelopes, signatures, or file-level integrity if used
- assembling the submitted graph
- providing canonical domain objects to the Engine
- handling review workflows outside the Engine
- deciding when to request runtime activation
- deciding whether to submit a full restore graph or historical partial graph

The host may perform pre-validation, but such validation does not replace Engine validation.

---

## ENG-IMPORT-03 — Engine Responsibilities

The Engine is responsible for consuming imported graphs only through the APIs and initialization procedures defined elsewhere.

The Engine is responsible for:

- structural validation through ENG-INTEGRITY
- runtime entry through ENG-INITIALIZATION
- graph reconstruction through ENG-SUPERSESSION
- receipt artifact validation through ENG-RECEIPT + ENG-CANON + ENG-SPECVERIFY
- deterministic outcome reporting through ENG-ERROR and ENG-API

ENG-IMPORT does not create a separate validation doctrine.  
It defines which imported graph shapes are being submitted for those authorities to evaluate.

---

# 4. Import Modes

## ENG-IMPORT-04 — Two Import Modes

The Engine recognizes two import-intent modes:

1. Area Restore
2. Historical Incremental Intake

These modes describe submission intent only.

Structural validity, readiness, halt, and runtime mode are determined by:

- ENG-INITIALIZATION
- ENG-INTEGRITY
- ENG-SUPERSESSION
- ENG-SPECVERIFY

---

# 5. Area Restore

## ENG-IMPORT-05 — Area Restore Submits a Complete Runtime Graph

Area Restore submits a complete Area graph intended for runtime activation.

A restore graph must provide all structural artifacts necessary for runtime rehydration of that Area, including as applicable:

- Sessions
- Resolutions
- Receipts
- supporting structural references

Receipts are mandatory wherever terminal sessions require them under ENG-RECEIPT and ENG-INTEGRITY.

ENG-IMPORT does not independently define completeness validation.  
It defines the import intent that the submitted graph is complete enough to attempt runtime activation.

---

## ENG-IMPORT-06 — Restore Requires Runtime Rehydration

Area Restore does not activate runtime by itself.

Runtime activation occurs only through:

- rehydrate_engine
- initialization procedures defined in ENG-API and ENG-INITIALIZATION

Therefore:

- import submission is not runtime entry
- restore validity is not assumed until rehydration succeeds
- no partial activation is permitted

---

## ENG-IMPORT-07 — Governance Completeness for Restore

A restore submission is intended to activate a full Area runtime.

Therefore the submitted graph must be capable of satisfying governance structural requirements for runtime activation.

ENG-IMPORT does not define slot validity rules.  
Those belong to:

- ENG-DOMAIN
- ENG-SUPERSESSION
- ENG-INTEGRITY

ENG-IMPORT defines only that a restore submission is expected to provide a graph capable of full runtime entry, not merely historical storage.

---

# 6. Historical Incremental Intake

## ENG-IMPORT-08 — Historical Incremental Intake Submits Historical Legitimacy Artifacts

Historical incremental intake submits a partial historical graph.

It may include:

- Resolutions
- Sessions
- Receipts
- supporting structural references

Its purpose is historical intake, not immediate legitimacy creation.

Such a graph may be suitable for later replay, historical verification, storage, or incremental compilation workflows depending on the consuming system.

---

## ENG-IMPORT-09 — Historical Intake May Be Structurally Partial for Runtime, But Not Structurally Fabricated

A historical intake graph may be incomplete for live runtime governance activation.

However, it must not fabricate historical legitimacy.

For any submitted terminal session, required receipts must already exist according to the governing artifact rules.

The Engine must not invent missing legitimacy artifacts to complete the historical graph.

Runtime activation of a historically partial graph still requires the full runtime validations defined elsewhere.

---

## ENG-IMPORT-10 — Historical Intake Does Not Bypass Compilation Rules

If historical intake is later used for replay or incremental compilation, the authoritative replay and conflict rules belong to ENG-COMPILATION and ENG-SUPERSESSION.

ENG-IMPORT does not define:

- replay ordering
- resolution index rules
- historical precedence algorithm
- replay acceptance policy

It only defines that imported historical graphs are inputs to those processes, not substitutes for them.

---

# 7. Imported Artifact Requirements

## ENG-IMPORT-11 — Imported Objects Must Already Be Domain-Conformant

Imported objects must conform to ENG-DOMAIN.

ENG-IMPORT does not define a second schema.

All imported objects must therefore already satisfy, as applicable:

- correct object type structure
- valid identifier form
- required schema_version presence
- valid enum values
- structural reference form
- rule provenance field presence where required
- receipt artifact structure where required

Any failure of those requirements is handled by the consuming validation authorities.

---

## ENG-IMPORT-12 — Imported Legitimacy Artifacts Must Be Historically Complete

Where legitimacy is claimed historically, the imported artifact set must be historically complete enough to support that claim.

For accepted legitimacy this means, at minimum, the relevant structural legitimacy artifacts required by the authoritative specifications must already exist.

ENG-IMPORT intentionally does not restate those artifact rules.  
They belong to:

- ENG-RECEIPT
- ENG-PERSISTENCE
- ENG-INTEGRITY
- ENG-DOMAIN

ENG-IMPORT defines only that incomplete legitimacy claims must not be accepted as valid imported legitimacy.

---

# 8. Receipt Relationship

## ENG-IMPORT-13 — Import Consumes Receipt Validity, It Does Not Define It

Import may include receipts as first-class submitted structural artifacts.

ENG-IMPORT does not define:

- receipt schema
- receipt canonical hashing
- round snapshot structure
- provenance semantics

Those belong to:

- ENG-RECEIPT
- ENG-CANON
- ENG-SPECVERIFY

ENG-IMPORT defines only that submitted receipts are part of the imported graph and must be consumed as immutable historical artifacts rather than regenerated.

---

# 9. Specification Identity Relationship

## ENG-IMPORT-14 — Imported Rule Provenance Must Be Preserved

Imported artifacts that carry rule provenance fields must preserve them exactly.

ENG-IMPORT does not define whether a spec_set_hash is current, legacy-compatible, or unknown.  
That belongs to ENG-SPECVERIFY.

ENG-IMPORT requires only that import must not strip, rewrite, or reinterpret rule provenance during submission.

---

# 10. Relationship to UNDER_REVIEW / RETIRED

## ENG-IMPORT-15 — Import Preserves Lifecycle State, It Does Not Reclassify It

Imported artifacts may arrive with lifecycle states supported by ENG-DOMAIN.

ENG-IMPORT does not define the behavioral meaning of those states.

It must not:

- promote UNDER_REVIEW to ACTIVE
- demote ACTIVE to UNDER_REVIEW
- reinterpret RETIRED
- rewrite graph structure to fit usability expectations

Usability semantics belong to ENG-REVIEW-RETIRED.  
Structural graph semantics belong to ENG-SUPERSESSION.

ENG-IMPORT preserves the submitted state for authoritative validation elsewhere.

---

# 11. Deterministic Import Outcome

## ENG-IMPORT-16 — Identical Submission, Identical Import Result

Given identical imported graph content and identical declared import mode, import submission must yield identical Engine outcomes when processed through the same runtime validation authorities.

Import must not depend on:

- file order
- storage order
- external timestamps
- host environment differences
- audit presence

Deterministic outcome reporting is consumed from ENG-ERROR and ENG-API.

---

# 12. Failure Semantics

## ENG-IMPORT-17 — Import Failure Is Side-Effect Free for Engine Runtime State

If import submission fails to produce an admissible runtime or historical validation outcome:

- failure must be explicit
- failure must be deterministic
- no partial runtime activation may occur
- no implicit repair may occur

Any runtime state mutation semantics are governed by the called APIs and runtime authorities.

ENG-IMPORT requires that import itself never acts as a partial legitimacy mutation path.

---

# 13. Relationship to API

## ENG-IMPORT-18 — Import Uses Existing Engine APIs

ENG-IMPORT does not define a separate hidden execution path.

Import is realized through the existing Engine interfaces, including as applicable:

- rehydrate_engine
- read-only validation or export operations
- incremental compilation APIs defined in ENG-API and governed behaviorally by ENG-COMPILATION

Therefore:

- import does not bypass API validation
- import does not bypass initialization
- import does not bypass integrity checks

Runtime activation still occurs only through successful rehydration / initialization.

---

# 14. Relationship to Initialization

## ENG-IMPORT-19 — Initialization Is the Runtime Activation Gate

A submitted import graph becomes a runtime Area only if ENG-INITIALIZATION and ENG-INTEGRITY allow it.

ENG-IMPORT is therefore upstream of runtime activation.

It prepares facts for validation.  
It does not itself validate runtime readiness in an independent doctrinal sense.

---

# 15. Engine Invariants

- import never creates legitimacy
- imported legitimacy artifacts must already exist
- import never reconstructs missing governance
- import never reconstructs missing receipts or Resolutions
- import preserves submitted structural lifecycle state
- import preserves rule provenance fields
- import consumes existing Engine APIs rather than bypassing them
- restore intent expects a graph capable of runtime activation
- historical intake intent may be non-activating but must not fabricate legitimacy
- dependent specifications must consume ENG-IMPORT as an intake boundary, not as an alternate validation doctrine

---

# 16. Mental Model

ENG-IMPORT defines the intake boundary.

It answers:

- what it means to submit a graph for restore
- what it means to submit a graph for historical intake
- what the host is responsible for
- what the Engine is responsible for at the intake boundary
- why import itself never creates legitimacy

It does not answer:

- whether the graph is structurally valid
- whether runtime may start
- how receipts are verified canonically
- how supersession graph truth is derived
- how historical replay order is determined

Those belong elsewhere.

ENG-IMPORT is the submission layer.  
Other specifications must consume it rather than duplicate import-boundary rules.