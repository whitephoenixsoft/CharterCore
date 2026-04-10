# Charter Core — External References Foundation

Status: FROZEN (Conceptual)  
Applies to: Charter Core, CLI, and future interfaces  
Does NOT define: storage schema, wire formats, or command syntax  

---

## Purpose

This document defines the conceptual model for **external (cross-Area) references**.

Its purpose is to preserve:

- historical intelligibility  
- legitimacy boundaries  
- audit clarity  

while allowing humans to reference ideas that evolve over time.

---

## Core Principle

> References identify what was pointed to, not what it is called now.

A reference captures the **recorded identity and human context at the moment of creation**, not live meaning.

---

## What an External Reference Is

An external reference is:

- a **pointer to an external identity** (Area or Resolution)  
- accompanied by **human-readable context**  
- immutable once created  
- non-legitimate by design  

A reference exists to support:

- understanding  
- traceability  
- audit reconstruction  

It does **not** participate in authority, scope, or acceptance.

---

## What an External Reference Is Not

An external reference is NOT:

- a legitimacy claim  
- an authority transfer  
- a dependency declaration  
- a live alias  
- a semantic guarantee  

References never:

- confer meaning  
- update automatically  
- rewrite history  
- participate in governance  

---

## Identity vs Meaning

Charter strictly separates:

- **Identity** — immutable, authoritative  
- **Meaning** — human, contextual, time-bound  

For external references:

- Identity is expressed through:
  - external_area_id  
  - external_resolution_id (optional)  

- Meaning is conveyed through labels and descriptions  

References bind to **identity**, not meaning.

---

## Reference Contents (Conceptual)

A reference MUST conceptually contain:

- **External Area ID**  
  The immutable Area identity  

- **External Resolution ID (optional)**  
  The specific Resolution being referenced  

- **Label Snapshot(s)**  
  Human-readable names known at the time of reference  

Optional (non-authoritative):

- description  
- additional labels  

These fields are:

- informational  
- historical  
- never normative  

---

## Relationship Declaration

A reference may include a **relationship** describing how the author views the connection.

Examples:

- derived_from  
- affects  
- affected_by  
- related  

Relationship properties:

- informational only  
- local to the referencing artifact  
- not required to be reciprocated  
- not a dependency  
- not interpreted by legitimacy logic  

The Engine must not infer structure, causality, or requirements from relationship declarations.

---

## Renaming Semantics

Renaming an Area or Resolution:

- modifies metadata  
- may emit audit events  
- does NOT alter existing references  

Existing references remain historically correct:

> “This is what the author called it at the time.”

Current names are discoverable through:

- audit  
- metadata history  
- query layers  

No back-patching occurs.

---

## External Existence Independence

External references:

- do not require the external Area or Resolution to exist locally  
- do not require validation of external targets  
- must not block legitimacy if the external target is unavailable  

They are opaque identifiers from the perspective of the local Engine.

---

## Audit Philosophy

External references exist to support the question:

> “What did the author believe they were pointing to at that moment?”

Audit may show:

- referenced identity  
- label snapshot at time of reference  
- evolution of metadata over time  

Audit must not infer intent beyond what was recorded.

---

## Legitimacy Boundary

External references:

- do not create authority  
- do not create scope  
- do not activate Areas  
- do not imply agreement  

They are informational artifacts only.

Any legitimacy involving Areas or Resolutions must occur through:

- sessions  
- resolutions  
- reconciliation review  

---

## Engine Responsibility

The Engine:

- preserves reference content immutably when included in immutable artifacts  
- treats reference content as informational data  
- never interprets or resolves external meaning  

The Engine must not:

- validate label correctness  
- enforce naming consistency  
- infer relationships from references  
- require external resolution or Area presence  

---

## Interface Responsibility

Interfaces (CLI, UI, reporting layers) MAY:

- display both historical and current labels  
- annotate differences for clarity  
- assist navigation and discovery  

Interfaces MUST NOT:

- mutate references  
- infer authority  
- collapse identity and meaning  
- treat references as structural dependencies  

---

## Design Intent (Frozen)

This model exists to ensure that:

- history never rewrites itself  
- names remain human, not authoritative  
- references remain safe across time and systems  
- audit remains intelligible long-term  

If references ever become authoritative or structural,
then Charter has violated its core legitimacy discipline.