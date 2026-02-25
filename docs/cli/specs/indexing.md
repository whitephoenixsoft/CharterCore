# Charter Core — Index Specification

Document ID: ENG-IDX  
Status: FROZEN (v1)  
Audience: Charter Core engine implementers  
Scope: Engine-internal only (never exported)  

---

## 1. Purpose

### ENG-IDX-01 — Indexes Exist for Performance Only

- Indexes exist solely to **accelerate queries**.  
- Indexes do **not** define:

  - Legitimacy  
  - History  
  - Authority  
  - Truth  

Fail if:

- Engine behavior changes when an index is missing  
- Index data is treated as authoritative  

---

## 2. Core Principle

### ENG-IDX-02 — Indexes Are Derivable

- Every index **must** be fully derivable from:

  - Object Store  
  - Ref Store  
  - Audit Store (if needed)  

- Indexes must **not** be the sole source of truth.  

Fail if:

- Loss of an index causes loss of meaning  
- Index corruption alters engine behavior  

---

## 3. Index Lifecycle

### ENG-IDX-03 — Indexes Are Rebuilt at Boot

- On engine startup:

  - Load object store  
  - Load ref store  
  - Rebuild all indexes in memory  

- Indexes are:

  - In-memory by default  
  - Discardable  
  - Rebuildable  

Fail if:

- Indexes are required to persist across restarts  
- Startup assumes precomputed index correctness  

---

## 4. Required Indexes (v1)

### ENG-IDX-04 — Resolution State Index

- Maintain an index of resolutions by lifecycle state:
```
(state, area_id) → Set
```
- States:

- ACTIVE  
- SUPERSEDED  
- RETIRED  
- UNDER_REVIEW  

- Used for:

- Listing active resolutions  
- Import review  
- Supersession checks  

Fail if:

- Active resolutions are discovered via refs  
- Resolution lookup requires full scans  

### ENG-IDX-05 — Supersession Graph Index

- Maintain a directed graph:
```
ResolutionHash → Set ResolutionHash → Set
```
- Properties:

- Directional  
- Immutable edges  
- No cycles allowed (validated)  

- Used for:

- Validating supersession  
- Import divergence analysis  
- Audit reasoning  

Fail if:

- Supersession order inferred by timestamps alone  
- Cycles are permitted  

### ENG-IDX-06 — Area Membership Index

- Maintain:
```
AreaId → Set
```
- Used for:

- Area-scoped queries  
- Export validation  
- fsck validation  

Fail if:

- Area membership is inferred indirectly  
- Cross-area leakage occurs  

### ENG-IDX-07 — Session Outcome Index

- Maintain:
```
SessionId → Set
```
- Used for:

- Export validation  
- Legitimacy checks  
- Import filtering  

Fail if:

- Resolutions exist without a closed session  
- Session outcomes are inferred post hoc  

---

## 5. Index Construction Rules

### ENG-IDX-08 — Indexes Are Built from Objects, Not Refs

- Indexes **must** be constructed primarily from object content.  
- Refs may be used only to:

- Locate “current” context  
- Seed traversal  

- Refs **must not** be used to:

- Infer resolution state  
- Infer supersession  

Fail if:

- Index correctness depends on ref correctness  

### ENG-IDX-09 — Index Build Order Is Deterministic

- Index rebuild order **must** be deterministic:

1. Load objects  
2. Index object types  
3. Build supersession graph  
4. Index lifecycle states  
5. Apply ref-derived context  

Fail if:

- Index order affects outcomes  
- Partial indexes are exposed  

---

## 6. Mutation Rules

### ENG-IDX-10 — Indexes Update on Engine Events

- Indexes **must** update only in response to:

- Object insertion  
- Ref mutation  
- Explicit lifecycle transitions  

- Indexes **must not** update:

- Lazily  
- Implicitly  
- As side effects of reads  

Fail if:

- Reading data mutates indexes  

---

## 7. fsck Interaction

### ENG-IDX-11 — Indexes Are Verified, Not Trusted

- fsck **must**:

- Recompute indexes from scratch  
- Compare against live indexes  
- Report mismatches  

- fsck **must not**:

- Repair indexes automatically  
- Modify objects or refs  

Fail if:

- fsck mutates index state  

---

## 8. Import / Export Semantics

### ENG-IDX-12 — Indexes Are Never Exported

- Indexes **must not** appear in exports  
- Indexes **must not** be imported  
- Indexes **must not** affect import semantics  

- Import operations:

- Load objects  
- Rebuild indexes locally  

Fail if:

- Imported data alters index rules  
- Indexes are serialized  

---

## 9. Performance Guarantees

### ENG-IDX-13 — Common Queries Are O(1) or O(log n)

- The following **must not** require full scans:

- List active resolutions  
- Get area authority  
- Get area scope  
- Validate supersession targets  

Fail if:

- Engine performance degrades linearly with history size  

---

## 10. Design Guarantees

### ENG-IDX-14 — Index Loss Is Non-Fatal

- Deleting all indexes **must** be equivalent to:

- Restarting the engine  
- Rebuilding from storage  

Fail if:

- Index loss causes irrecoverable state  

---

## Mental Model

- Objects are facts  
- Refs are current pointers  
- Indexes are scratchpads for speed  
- Audit is the memory  
- Legitimacy never depends on caches  

---

## Why This Matters

- Ensures consistent behavior at any scale  
- Guarantees deterministic import review  
- Enables meaningful fsck and consistency checks