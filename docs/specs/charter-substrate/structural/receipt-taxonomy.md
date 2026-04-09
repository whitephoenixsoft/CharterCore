# Charter Receipt Taxonomy — Foundation Specification (Revised)

Status: FOUNDATIONAL  
Applies to: Runtime Layer, Review Workflows, CDS, Legitimacy Engine, CCS  
Does NOT Define: legitimacy semantics, alignment computation, transport (CRS), or guidance behavior  

---

# I. Purpose

This document defines the structural taxonomy for **receipts** within the Charter system.

It establishes:

- Receipt categories  
- Required receipt properties  
- Lifecycle triggers  
- Lineage rules  
- Audit guarantees  

The objective is to:

- prevent semantic ambiguity  
- prevent accidental legitimacy inference  
- ensure full audit reconstructability  
- make structural closure explicit and machine-verifiable  

Receipts formalize closure.  
They do not create authority unless explicitly defined as legitimacy receipts.

---

# II. Receipt Core Definition

A receipt is:

> An immutable, structured record emitted at the closure of a bounded lifecycle event.

Receipts:

- are append-only  
- have canonical engine IDs  
- are audit-visible  
- do not mutate legitimacy  
- may reference prior receipts (lineage)  
- are commit-level artifacts  

Receipts represent structural finalization, not agreement.

---

# III. Receipt Categories

Receipts are strictly categorized.  
Category must be explicit and machine-readable.

The canonical categories are:

- DELIBERATE  
- REVIEW  
- RECONCILIATION  
- LEGITIMACY  
- EXPLORATION (specialized, non-primary)

No additional categories are permitted without explicit version governance.

---

## A. Deliberate Receipts

### Trigger

Emitted upon closure of a **deliberate instance**.

Closure types include:

- SYNTHESIZED  
- ABANDONED  
- FORKED  
- ARCHIVED  

### Captures

- receipt_type: DELIBERATE  
- engine_id  
- deliberate_id  
- closure_type  
- item_ids  
- applied_item_ids  
- settled_item_ids  
- participant snapshot (if present)  
- referenced review_receipt_ids (if applicable)  
- referenced resolution_ids (optional)  
- timestamp  
- annotations (optional, non-authoritative)  

### Does NOT Capture

- authority  
- legitimacy outcomes  
- stance evaluation  

### Principle

> Deliberate receipts record the closure of structured thinking, not decisions.

---

## B. Review Receipts

### Trigger

Emitted upon closure of a **Review workflow**.

Applies to:

- Foreign Integration Review  
- import workflows  
- consolidation workflows  
- deliberate output evaluation  
- candidate relationship evaluation  

### Captures

- receipt_type: REVIEW  
- engine_id  
- review_id  
- source type (import, deliberate, federation, etc.)  
- review item IDs  
- accepted IDs  
- rejected IDs  
- abandoned IDs  
- accepted relationship definitions (if any)  
- resulting session_ids (if initiated)  
- referenced prior receipts (optional)  
- timestamp  

### Does NOT Capture

- authority evaluation results  
- legitimacy outcomes  
- stances  

### Principle

> Review receipts record selection and preparation, not legitimacy.

---

## C. Reconciliation Receipts (NEW)

### Trigger

Emitted upon closure of a **Reconciliation workflow**.

Applies to:

- legitimacy → deliberate synchronization  
- resolution → item projection workflows  
- deliberate state updates based on legitimacy outcomes  

### Captures

- receipt_type: RECONCILIATION  
- engine_id  
- reconciliation_id  
- source artifact IDs (e.g., resolution_ids)  
- resulting item_ids created or updated  
- affected deliberate_id(s)  
- linkage references:
  - resolution ↔ item mappings  
  - derived_from relationships (if used)  
- reconciliation actions performed:
  - created  
  - updated  
  - mapped  
- timestamp  

### Does NOT Capture

- authority  
- legitimacy evaluation  
- structural admission  

### Principle

> Reconciliation receipts record synchronization between thinking and decision without creating authority.

---

## D. Legitimacy Receipts

### Trigger

Emitted upon closure of a **Session**.

This includes:

- accepted outcomes  
- rejected outcomes  
- canceled sessions  

### Captures

- receipt_type: LEGITIMACY  
- engine_id  
- session_id  
- authority_id  
- scope_id  
- participant set (immutable snapshot)  
- candidate set (immutable snapshot)  
- stances (ACCEPT / REJECT / ABSTAIN)  
- topic  
- annotations  
- timestamp  
- final outcome (accepted / rejected / canceled)  

### Principle

> This is the only receipt category that records legitimacy.

---

## E. Exploration Receipts (Specialized)

### Trigger

Emitted upon explicit closure of **exploratory artifacts** that do not result in legitimacy.

Examples:

- synthesis artifacts frozen without review  
- abandoned exploratory outputs  
- non-legitimizing investigation endpoints  

### Captures

- receipt_type: EXPLORATION  
- engine_id  
- artifact_id  
- originating deliberate_id (if applicable)  
- artifact IDs  
- end-state (e.g., SYNTHESIZED, ABANDONED)  
- timestamp  
- optional annotations  

### Does NOT Capture

- authority  
- legitimacy  
- review outcomes  

### Principle

> Exploration receipts record non-legitimizing closure of investigation artifacts.

---

# IV. Receipt Invariants

The following must always hold:

1. Every session closure MUST produce a LEGITIMACY receipt.  
2. Every review closure MUST produce a REVIEW receipt.  
3. Every reconciliation closure MUST produce a RECONCILIATION receipt.  
4. Every deliberate closure MUST produce a DELIBERATE receipt.  
5. Receipts are immutable.  
6. Receipts are the canonical proof artifacts for structural closure.  
7. Audit logs must allow deterministic reconstruction of receipts.  
8. If a receipt and audit diverge, the system is invalid.  
9. Receipt presence never implies correctness.  
10. Receipt presence never implies consensus beyond recorded stances.  
11. Receipt absence means structural closure did not occur.  

Violation of these invariants indicates system correctness failure.

---

# V. Receipt Lineage Model

Receipts form a **directed, auditable lineage graph**.

Common patterns include:

- Deliberate → Review → Legitimacy  
- Legitimacy → Reconciliation → Deliberate  
- Deliberate ↔ Reconciliation ↔ Legitimacy  

### Lineage Requirements

- references must be machine-traceable  
- references must use canonical engine IDs  
- lineage must be explicit and directional  
- no implicit inference is allowed  

Receipts may reference multiple prior receipts.

### Reconstruction Goals

Lineage must allow reconstruction of:

- idea origin (CDS)  
- review path  
- legitimacy path  
- reconciliation cycles  

without interpreting semantic meaning.

---

# VI. Audit Requirements

Audit output must:

- render receipts as discrete events  
- include receipt type  
- include canonical engine IDs  
- preserve stable ordering  
- preserve lineage references  
- allow filtering by receipt type  
- avoid interpreting intent  

Receipts must be distinguishable from:

- sessions  
- deliberates  
- reviews  
- reconciliation processes  

Audit is the system of record.

If receipt lineage cannot be reconstructed from audit,  
the system has failed determinism.

---

# VII. Scope and Extensibility

Receipts apply to:

- Runtime workflows  
- CDS  
- Review systems  
- Legitimacy engine  

Future extensions MAY include:

- CSP processing receipts (aggregation, window closure)  
- CRS operational receipts (transport events)  

Such extensions must:

- not violate core invariants  
- not introduce implicit authority  
- remain explicitly categorized  

---

# Final Principle

Receipts formalize structural closure.

Only LEGITIMACY receipts record legitimacy.

All other receipt types preserve process integrity,  
traceability, and system determinism  
without ever implying authority.