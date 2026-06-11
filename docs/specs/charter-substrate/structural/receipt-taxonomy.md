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
- EXPLORATION (deprecated / planned removal; non-primary)

No additional categories are permitted without explicit version governance.

---

## A. Deliberate Receipts

### Trigger

Emitted when a **Deliberate** reaches any terminal state: `CLOSED` or `ABANDONED`.

### Captures

- receipt_type: DELIBERATE  
- engine_id  
- deliberate_id  
- terminal_state  
- closure_type  
- closure_reason  
- declared_purpose / epic  
- scope  
- item_state_summary  
- locked_item_ids  
- abandoned_item_ids  
- deferred_item_ids  
- discarded_item_ids  
- settled_item_ids  
- applied_item_event_refs  
- transferred_item_event_refs  
- copied_item_event_refs  
- forked_item_event_refs  
- breakout_output_item_ids  
- ddr_record_refs  
- feed_selection_refs  
- feed_range_refs  
- evidence_refs  
- relationship_events  
- provenance_path  
- provenance_events  
- origin_deliberate_id  
- successor_deliberate_ids  
- unresolved_tensions  
- authority_assumptions (if present)  
- participant_snapshot (if present)  
- referenced review_receipt_ids (if applicable)  
- referenced resolution_ids (optional)  
- timestamp  
- annotations  

### Closure Type Normalization

Deliberate receipts distinguish terminal state from closure type and relationship/provenance events.

```yaml
terminal_state: CLOSED | ABANDONED
closure_type: STRUCTURAL_CLOSURE | WORK_TRANSFERRED | SUPERSEDED | PURPOSE_GIVEN_UP | LOSS_OF_CONTINUITY | OTHER
closure_reason: optional human-readable explanation
```

The following are prohibited as Deliberate `closure_type` values:

- FORKED  
- COPIED  
- FEDERATED  
- ARCHIVED  
- SYNTHESIZED  

`FORKED`, `COPIED`, `FEDERATED`, `SPLIT`, `MERGED`, `CONTINUED`, and `SUPERSEDED_BY` are relationship/provenance events.

`ARCHIVED` is an artifact lifecycle or storage posture.

`SYNTHESIZED` is a DDR outcome or closure reason, not a Deliberate state.

### Does NOT Capture

- authority  
- legitimacy outcomes  
- stance evaluation  

### Principle

> Deliberate receipts record terminal preservation and lineage for structured thinking, not decisions.

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

## E. Exploration Receipts (Deprecated / Planned Removal)

CDS does not require a separate Exploration Receipt category.

The entire Deliberate is exploratory/non-legitimate.

Exploratory work is preserved through:

- Deliberate state  
- Item history  
- Breakout outputs  
- DDR records  
- terminal Deliberate Receipts  

### Principle

> Exploratory work is preserved through Deliberate lineage and terminal receipts without requiring a separate receipt category.

---

# IV. Receipt Invariants

The following must always hold:

1. Every session closure MUST produce a LEGITIMACY receipt.  
2. Every review closure MUST produce a REVIEW receipt.  
3. Every reconciliation closure MUST produce a RECONCILIATION receipt.  
4. Every terminal Deliberate state MUST produce a DELIBERATE receipt.  
5. A Deliberate Receipt MUST include `terminal_state`.  
6. A Deliberate Receipt MUST distinguish terminal state from relationship/provenance events.  
7. A copied, forked, federated, split, merged, continued, or recovered Deliberate MUST preserve provenance.  
8. A receipt may preserve ancestry of non-legitimate thinking without implying authority.  
9. Receipts formalize terminal preservation and lineage. They do not elevate non-legitimate artifacts into authority.  
10. Receipts are immutable.  
11. Receipts are the canonical proof artifacts for structural closure.  
12. Audit logs must allow deterministic reconstruction of receipts.  
13. If a receipt and audit diverge, the system is invalid.  
14. Receipt presence never implies correctness.  
15. Receipt presence never implies consensus beyond recorded stances.  
16. Receipt absence means structural closure did not occur.  

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
- Receipt lineage must preserve Deliberate provenance across copied, forked, federated, split, merged, continued, recovered, and returned instances.  
- A returned receipt from a copied or federated Deliberate must preserve origin and provenance path sufficiently to reconstruct ancestry even if intermediate generations are absent.  

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