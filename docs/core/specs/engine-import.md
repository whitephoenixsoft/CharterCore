# ENG-IMPORT — Import Submission, Restore Entry & Historical Graph Intake

Status: REFACTORED (v6 – Structure/Usability Alignment & Candidate Action Inclusion)  
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
- ENG-STRUCTURE
- ENG-USABILITY
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
- structural graph semantics  
- session acceptance rules  
- incremental compilation replay algorithm  

Those are defined respectively in:

- ENG-DOMAIN  
- ENG-INTEGRITY  
- ENG-RECEIPT  
- ENG-CANON  
- ENG-SPECVERIFY  
- ENG-STRUCTURE  
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

If imported legitimacy cannot already be proven from the submitted historical graph, import must fail or remain non-activating according to authoritative validation outcomes.

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

Host-side validation does not replace Engine validation.

---

## ENG-IMPORT-03 — Engine Responsibilities

The Engine is responsible for consuming imported graphs only through the APIs and initialization procedures defined elsewhere.

The Engine is responsible for:

- structural validation through ENG-INTEGRITY  
- runtime entry through ENG-INITIALIZATION  
- structural graph reconstruction through ENG-STRUCTURE  
- receipt validation through ENG-RECEIPT + ENG-CANON + ENG-SPECVERIFY  
- deterministic outcome reporting through ENG-ERROR and ENG-API  

ENG-IMPORT does not create a separate validation doctrine.  
It defines which graph shapes are submitted for authoritative evaluation.

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
- ENG-STRUCTURE  
- ENG-SPECVERIFY  

---

# 5. Area Restore

## ENG-IMPORT-05 — Area Restore Submits a Complete Runtime Graph

Area Restore submits a complete Area graph intended for runtime activation.

A restore graph must provide all structural artifacts necessary for runtime rehydration, including:

- Sessions  
- Resolutions  
- Receipts  
- supporting structural references  

Informational references may also be present according to ENG-DOMAIN:

- cross-area informational references  
- intra-Area informational Resolution references  

Receipts are mandatory wherever required under ENG-RECEIPT and ENG-INTEGRITY.

ENG-IMPORT defines intent, not completeness validation.

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
- ENG-STRUCTURE  
- ENG-INTEGRITY  

ENG-IMPORT defines only intent.

---

# 6. Historical Incremental Intake

## ENG-IMPORT-08 — Historical Incremental Intake Submits Historical Artifacts

Historical incremental intake submits a partial historical graph.

It may include:

- Resolutions  
- Sessions  
- Receipts  
- supporting structural references  
- informational references permitted by ENG-DOMAIN  

Its purpose is historical intake, not immediate legitimacy creation.

---

## ENG-IMPORT-09 — Historical Intake Must Not Fabricate Legitimacy

A historical intake graph may be incomplete for runtime activation.

However, it must not fabricate legitimacy.

For any submitted terminal session:

- required receipts must already exist  

The Engine must not invent missing legitimacy artifacts.

---

## ENG-IMPORT-10 — Historical Intake Does Not Bypass Compilation Rules

If historical intake is later used for replay or incremental compilation:

- replay rules belong to ENG-COMPILATION  
- structural graph rules belong to ENG-STRUCTURE  

ENG-IMPORT does not define:

- replay ordering  
- precedence rules  
- conflict resolution  

It defines only that imported graphs are inputs to those processes.

---

# 7. Imported Artifact Requirements

## ENG-IMPORT-11 — Imported Objects Must Be Domain-Conformant

Imported objects must conform to ENG-DOMAIN.

This includes:

- correct object structure  
- valid identifiers  
- required schema_version  
- valid enum values  
- structural reference validity  
- informational reference validity  
- candidate action payload structure and target reference form  
- rule provenance fields where required  
- receipt structure where required  

Failures are handled by authoritative validation specifications.

---

## ENG-IMPORT-12 — Imported Legitimacy Must Be Historically Complete

Where legitimacy is claimed, the imported artifact set must be sufficient to support that claim.

ENG-IMPORT does not restate artifact rules.  
They belong to:

- ENG-RECEIPT  
- ENG-PERSISTENCE  
- ENG-INTEGRITY  
- ENG-DOMAIN  

---

# 8. Receipt Relationship

## ENG-IMPORT-13 — Receipts Are Consumed, Not Defined

Import may include receipts as structural artifacts.

ENG-IMPORT does not define:

- receipt schema  
- hashing rules  
- serialization  

Receipts must be preserved exactly as submitted.

---

# 9. Specification Identity Relationship

## ENG-IMPORT-14 — Rule Provenance Must Be Preserved

Imported artifacts must preserve rule provenance fields exactly.

ENG-IMPORT does not interpret them.  
ENG-SPECVERIFY does.

---

# 10. Relationship to ON_HOLD / RETIRED

## ENG-IMPORT-15 — Import Preserves Lifecycle State

Imported artifacts may include lifecycle states:

- ACTIVE  
- ON_HOLD  
- RETIRED  
- SUPERSEDED  

ENG-IMPORT must not:

- promote ON_HOLD to ACTIVE  
- demote ACTIVE to ON_HOLD  
- reinterpret RETIRED  
- rewrite structural graph relationships  

Usability semantics belong to ENG-USABILITY.  
Structural graph semantics belong to ENG-STRUCTURE.

---

# 11. Relationship to Informational References

## ENG-IMPORT-16 — Informational References Are Preserved Without Reinterpretation

Imported graphs may include informational references:

- cross-area  
- intra-Area  

ENG-IMPORT must not:

- reinterpret informational references as structural graph edges  
- infer legitimacy from them  
- infer ordering or precedence  
- discard them due to non-semantic status  

Validation and consequences belong to:

- ENG-DOMAIN  
- ENG-INTEGRITY  
- ENG-STRUCTURE  
- ENG-CANON  

Informational references are preserved but remain subject to downstream validation outcomes.

---

# 12. Deterministic Import Outcome

## ENG-IMPORT-17 — Deterministic Behavior

Given identical input and mode:

- identical outcomes must result  

Import must not depend on:

- file order  
- timestamps  
- environment  

---

# 13. Failure Semantics

## ENG-IMPORT-18 — Import Failure Is Non-Mutating

If import fails:

- failure must be explicit  
- no partial runtime activation  
- no implicit repair  

---

# 14. Relationship to API

## ENG-IMPORT-19 — Import Uses Engine APIs

Import is executed through:

- rehydrate_engine  
- incremental compilation APIs  

It does not bypass validation layers.

---

# 15. Relationship to Initialization

## ENG-IMPORT-20 — Initialization Is the Activation Gate

A submitted graph becomes runtime only if:

- ENG-INITIALIZATION  
- ENG-INTEGRITY  

permit it.

---

# 16. Engine Invariants

- import never creates legitimacy  
- imported legitimacy must already exist  
- import never reconstructs governance  
- import preserves lifecycle state  
- import preserves informational references  
- import preserves rule provenance  
- import uses Engine APIs only  
- restore expects runtime-capable graphs  
- historical intake must not fabricate legitimacy  

---

# 17. Mental Model

ENG-IMPORT defines the intake boundary.

It answers:

- how graphs are submitted  
- what restore vs historical intake means  
- who is responsible for what  

It does not answer:

- whether the graph is valid  
- whether runtime may start  
- how graph truth is derived  
- how legitimacy is evaluated  

ENG-IMPORT is the submission layer.  
Other specifications determine truth.