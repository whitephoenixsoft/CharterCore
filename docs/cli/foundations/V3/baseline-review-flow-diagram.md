# Baseline Review Flow Diagram — Inputs, Outputs, and Authority

Legend:
- ⬜ = Input / artifact
- 🔹 = Baseline review process
- ✅ = Accepted / session-ready output
- ❌ = Rejected / deferred / discarded
- 🔒 = Authority evaluation / legitimacy applied
- ⚠ = Foreign / imported / non-legitimate content

---

## 1. Flat File Import

⬜ External Resolutions (JSON, Formatted)  
      │
      ▼
🔹 Baseline Review (Flat File) ⚠
      │
      ├─✅ Accepted → Session (ACTIVE) 🔒
      └─❌ Rejected → ABANDONED / UNDER_REVIEW

Notes:
- Only affects local Area
- No implicit legitimacy
- Preserves audit trail

---

## 2. Baseline Consolidation (Internal)

⬜ Prior Local Decisions / Imported Baselines  
      │
      ▼
🔹 Baseline Consolidation Review
      │
      ├─✅ Re-affirmed → ACTIVE 🔒
      └─❌ Superseded → HISTORICAL

Notes:
- Can batch accept/reject
- Maintains lineage of past decisions
- Preserves audit

---

## 3. Deliberate Output

⬜ Synthesis Options (from Breakouts)  
      │
      ▼
🔹 Baseline Review (Deliberate) ⚠
      │
      ├─✅ READY → Session (ACTIVE) 🔒
      ├─❌ DEFERRED → Remain for later / Open Issues
      └─❌ OPEN_ISSUE → Requires further deliberation

Notes:
- May include multiple epics
- Options can come from multiple breakouts
- Consolidation is explicit
- Lineage and audit preserved

---

## 4. Deliberate Import

⬜ Exported Deliberate File ⚠  
      │
      ▼
🔹 Baseline Review (Imported Deliberate) ⚠
      │
      ├─✅ READY → Session (ACTIVE) 🔒
      ├─❌ DEFERRED → Pending further discussion
      └─❌ OPEN_ISSUE → Requires additional breakouts or work

Notes:
- Always requires consolidation
- Foreign content must remain non-legitimate until accepted
- Each import handled one deliberate at a time
- Integrity checks optional (hashing)

---

## 5. Session Batch / Multi-resolution

⬜ Multiple Session Resolutions  
      │
      ▼
🔹 Baseline Review (Session Batch) 🔒
      │
      └─✅ Each resolution → Session (ACTIVE)

Notes:
- Authority already applied
- Usually local to Area
- Can optionally feed into higher-level baseline for cross-Area consolidation

---

## 6. Foreign Baseline Merge

⬜ External / Historical Baseline ⚠  
      │
      ▼
🔹 Baseline Review (Foreign) ⚠
      │
      ├─✅ Accepted → ACTIVE 🔒
      └─❌ Superseded → HISTORICAL

Notes:
- Treat all incoming resolutions as foreign
- Must explicitly reconcile conflicts
- Preserves audit trail

---

### Key Shared Invariants Across All Baseline Reviews
- Explicit accept/reject flow
- One active baseline per Area
- Full audit trail
- No automatic creation of legitimacy
- All accepted outputs generate sessions for enforcement