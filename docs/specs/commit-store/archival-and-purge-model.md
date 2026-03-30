# Charter — Runtime Archival & Purge Model

Status: FOUNDATIONAL (Canonical)  
Replaces: Archival Model (Deprecated), Archival Modes (Deprecated — concepts retained)  
Applies to: Runtime Layer, Charter Commit System (CCS), Commit Store, Relay Interaction  
Does NOT apply to: Legitimacy Engine semantics  

---

# 1. Purpose

This document defines how Charter performs **archival and purge** of local runtime data.

It exists to:

- safely reduce active storage in the Runtime Layer  
- preserve historical artifacts without loss of meaning  
- provide a structured, auditable path to removal of local data  
- ensure no data is destroyed before it is preserved  

Archival is a **preservation workflow**.  
Purge is a **controlled destruction workflow**.

They are related, but never the same operation.

---

# 2. Core Principle

> Nothing is removed until it is safely preserved.

Archival:
- preserves data in a durable artifact form  

Purge:
- removes local runtime data only after preservation is verified  

At no point may the system:
- destroy data without explicit user intent  
- remove data before preservation  
- rewrite or reinterpret historical meaning  

---

# 3. Storage Model

Charter operates across two distinct storage domains:

## 3.1 Runtime Store (Local Object Store)

Holds:

- sessions  
- deliberates  
- breakouts  
- synthesis artifacts  
- baseline review workspaces  
- candidates and workflow state  
- active operational data  

Characteristics:

- mutable (within workflow constraints)  
- optimized for active use  
- subject to archival and purge  

---

## 3.2 Commit Store

Holds:

- immutable commit artifacts  
- resolutions  
- receipts  
- archival packages  
- imported artifacts  

Characteristics:

- append-only  
- immutable  
- preservation layer  
- not directly purged in this model  

---

## 3.3 Key Distinction

- Runtime Store = **working state**  
- Commit Store = **preserved truth artifacts**  

Archival moves data from **working state → preserved artifacts**.  
Purge removes data only from **working state**.

---

# 4. Definition: Archival

Archival is:

> An explicit, auditable workflow that selects runtime artifacts, packages them, and preserves them as immutable artifacts in the Commit Store.

Archival does NOT:

- change legitimacy  
- modify artifacts  
- reinterpret history  
- delete data  

It only changes **where and how data is stored**.

---

# 5. Archival Pipeline

Archival follows four explicit stages.

---

## 5.1 Stage 1 — Archival Candidate Selection

Artifacts are selected from the Runtime Store using:

- explicit user selection  
- time-based filters  
- state-based filters  
- workflow classification  

Examples:

- completed deliberates  
- closed breakouts  
- abandoned proposals  
- historical workflow artifacts  

---

## 5.2 Stage 2 — Archival Review

A structured review process validates:

- inclusion of artifacts  
- dependency completeness  
- archival mode (see Section 6)  

This stage:

- is non-legitimizing  
- does not create authority  
- does not modify artifacts  

It exists to ensure safe and understandable archival.

---

## 5.3 Stage 3 — Archival Packaging

Selected artifacts are packaged into an **archival unit**.

This may be implemented as:

- a temporary archival Area  
- a structured archive bundle  
- an export package  

The archival unit:

- preserves all IDs  
- preserves hashes  
- preserves lineage  
- preserves provenance  

No transformation occurs.

---

## 5.4 Stage 4 — Commit Store Preservation

The archival unit is written into the Commit Store via CCS.

This produces:

- immutable archival artifacts  
- optional archival record metadata  

Once complete:

- preservation is considered successful  
- artifacts are now durable and retrievable  

---

## 5.5 Stage 5 — Purge Eligibility

After preservation is verified:

- original runtime artifacts become **purge-eligible**

They are not removed automatically.

---

# 6. Archival Modes

Archival modes determine **completeness**, not meaning.

---

## 6.1 MINIMAL

- includes only selected artifacts  
- does not include dependencies  
- may contain unresolved references  

Use when:

- quick offloading is needed  
- full reconstruction is not required  

---

## 6.2 CLOSURE

- includes all required dependencies  
- produces a self-contained archive  

Guarantees:

- independent interpretability  
- deterministic reconstruction  

---

## 6.3 NARRATIVE

- includes closure + contextual workflow artifacts  

Includes:

- deliberates  
- breakouts  
- synthesis  
- rejected paths  
- annotations  

Use when:

- preserving reasoning and evolution  
- institutional memory is important  

---

## 6.4 Mode Principle

> Modes affect completeness, not truth.

All modes:

- preserve identity  
- preserve hashes  
- preserve lineage  

---

# 7. Definition: Purge

Purge is:

> An explicit, irreversible operation that removes selected runtime artifacts after they have been safely archived.

---

## 7.1 Purge Requirements

Purge MUST require:

- explicit user confirmation  
- verified archival preservation  
- dependency safety validation  
- audit record of removal  

---

## 7.2 Purge Scope

Purge applies ONLY to:

- Runtime Store artifacts  

Purge does NOT affect:

- Commit Store  
- archived artifacts  
- relay-stored data  

---

## 7.3 Purge Guarantees

After purge:

- archived data remains accessible  
- lineage remains intact through preserved artifacts  
- no legitimacy is altered  

---

## 7.4 Purge Is Not Deletion of History

Purge removes **local working copies**, not historical truth.

---

# 8. Archival Outputs

Archival may produce:

- archival commit artifacts  
- archive bundles  
- export packages  
- relay-published archives  

All outputs must preserve:

- identity  
- provenance  
- integrity  

---

# 9. Relationship to CCS

The Charter Commit System (CCS):

- defines archival artifact structure  
- assigns identity (UUID)  
- preserves lineage  

CCS does not:

- interpret archival meaning  
- enforce archival policy  

---

# 10. Relationship to Commit Store

The Commit Store:

- is the preservation destination for archival artifacts  
- guarantees immutability  
- supports retrieval and export  

It does not:

- enforce archival workflows  
- perform purge  
- interpret completeness  

---

# 11. Relationship to Relay

The Charter Relay System (CRS):

- may store archival artifacts  
- may distribute archive bundles  

It does not:

- interpret archives  
- validate completeness  
- reconstruct state  

Relay treats all artifacts as opaque.

---

# 12. Retrieval and Reuse

Archived artifacts may be:

- inspected directly from Commit Store  
- exported  
- pushed to or fetched from relay  
- reintroduced via import workflows  

Reintroduction must follow:

- baseline review  
- explicit acceptance  

---

# 13. Invariants

- Archival must not change legitimacy  
- Archival must preserve identity and hashes  
- Archival must preserve lineage and provenance  
- Archival must be explicit and auditable  
- Purge must not occur before preservation  
- Purge must be explicit and irreversible  
- Commit Store must remain append-only  
- Relay must remain non-interpreting  

---

# 14. Mental Model

Archival is:

- moving working material into a permanent record  

Purge is:

- removing local copies after they are safely stored  

The system:

- never forgets  
- only changes where data lives  

---

# 15. Future Extension (Non-Normative)

A future model may introduce:

- Commit Store archival (cold storage / compaction)  
- long-term artifact retention strategies  

This is explicitly **out of scope** for this document.

---

# 16. Final Principle

Charter preserves before it removes.

Archival creates safety.  
Purge requires proof.