# Baseline Review Map — Sources, Inputs, Outputs, and Behavior

Status: DRAFT
Applies to: Charter CLI / Engine (V2 → V3)
Purpose: Compare different workflows that create or feed into baseline reviews

Legend:
- **Input Type** — what triggers the review
- **Accepted Output** — what becomes ACTIVE or session-ready
- **Rejected Output** — what is discarded or deferred
- **Statuses / Labels** — internal state indicators
- **Behavior Notes** — key differences from other review sources

---

| Source / Trigger | Input Type | Accepted Output | Rejected Output | Statuses / Labels | Behavior Notes |
|-----------------|------------|----------------|----------------|-----------------|----------------|
| **Flat File Import** | External resolutions (JSON, CSV, etc.) | Becomes session-ready once accepted | UNDER_REVIEW stays until explicit reject or abandoned | UNDER_REVIEW, ACTIVE (via session), ABANDONED | Works on single import; only affects local Area; no automatic authority evaluation |
| **Baseline Consolidation** | Prior local decisions or imported baselines | Already ACTIVE resolutions or session outputs are re-affirmed | Superseded or historical resolutions marked REJECTED or HISTORICAL | UNDER_REVIEW, ACTIVE, ABANDONED, HISTORICAL | Allows batch operations (accept all / reject all); preserves prior audit; may mark duplicates as HISTORICAL |
| **Deliberate Output** | Synthesis options, artifacts from breakouts | READY options become session-ready once accepted | Options marked DEFERRED or OPEN_ISSUE remain non-active | UNDER_REVIEW, READY, DEFERRED, OPEN_ISSUE | Highest-level consolidation; may include multiple epics; options can be split by epic; preserves lineage from all breakouts |
| **Deliberate Import** | Exported Deliberate file (foreign baseline) | READY options become session-ready after review | DEFERRED / DISCARDED artifacts remain non-active | UNDER_REVIEW, READY, DEFERRED, OPEN_ISSUE | Always requires consolidation; may include multiple breakouts or epics; integrity can be hashed but is non-legitimate until accepted; read-only until user approves |
| **Session Output (Batch / Multi-resolution)** | Multiple session decisions bundled | Each resolution becomes ACTIVE session object once approved | N/A — already resolved during session | ACTIVE, REJECTED (if session rollback) | Typically localized to the Area; can be a batch import into a baseline if desired; not foreign; authority evaluation already applied |
| **Foreign Baseline Merge** | Historical or external baseline already reviewed elsewhere | Approved resolutions become ACTIVE | Superseded or conflicting resolutions marked HISTORICAL | UNDER_REVIEW, ACTIVE, HISTORICAL | Treats incoming resolutions as “foreign”; no local authority assumptions; must reconcile conflicts explicitly |

---

## Key Observations / Behavioral Patterns

1. **All baseline reviews share:**
   - Explicit acceptance / rejection flow
   - Audit trail for every action
   - Mechanism to prevent implicit legitimacy creation

2. **Differences by source:**
   - **Deliberate outputs** are more complex: may span multiple epics and contain multiple breakouts
   - **Imports** always require consolidation, even if locally generated
   - **Session outputs** already have authority applied and are internally trusted
   - **Flat file imports** may need external provenance preserved

3. **Status mapping differences:**
   - Flat import: UNDER_REVIEW → ACTIVE / ABANDONED
   - Deliberate: UNDER_REVIEW → READY / DEFERRED / OPEN_ISSUE → session-ready
   - Foreign baseline: UNDER_REVIEW → ACTIVE / HISTORICAL
   - Consolidation allows multi-layer reconciliation, unlike single import reviews

4. **Shared invariants for all baseline reviews:**
   - CLI-BL-01 through CLI-BL-08 (single active baseline, full audit, no implicit carryover)
   - Flat file / deliberate import must never auto-activate resolutions
   - All approvals still create sessions under the hood

---