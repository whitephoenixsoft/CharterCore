# ENG-COMPILATION  
Historical Replay & Incremental DAG Compilation  
Status: DRAFT (Pre-Freeze)  
Applies to: Engine Core (V1/V2+)  

---

# 1. Purpose

ENG-COMPILATION defines the deterministic mechanism for:

- Historical session replay  
- Incremental compilation  
- Conflict detection during replay  
- Resolution index construction  
- Canonical replay ordering  
- Replay rejection semantics  

Compilation reconstructs a valid legitimacy DAG from historical artifacts.

Compilation must produce identical results across compliant implementations when provided identical inputs.

Compilation must never create or alter legitimacy beyond what historical sessions legitimately produced.

---

# 2. Compiler Execution Model

The engine operates in two execution modes.

Runtime Mode performs:

- Session evaluation  
- Session acceptance  
- Resolution creation  
- Receipt emission  

Compilation Mode performs:

- Historical session import  
- Receipt verification  
- Deterministic replay  
- Resolution index construction  

Runtime Mode must never rewrite legitimacy history.

Compilation Mode must never invalidate legitimacy already accepted by the runtime.

---

# 3. Resolution Index

During compilation the engine constructs a deterministic resolution index.

The index contains **all accepted resolutions in the Area**, including historical resolutions.

The index exists only in memory and must be fully derivable from domain objects.

Each entry must include:

- resolution_id  
- accepted_at  
- session_id  
- state  
- supersedes_resolution_id (nullable)  
- receipt_hash  

The state field may contain:

- ACTIVE  
- UNDER_REVIEW  
- RETIRED  
- SUPERSEDED  

The index must contain all resolutions regardless of state.

The resolution index is used for:

- dependency validation  
- conflict detection  
- supersession validation  
- canonical replay ordering  

The index must be deterministic.

Identical DAG inputs must produce identical resolution indexes.

---

# 4. Canonical Replay Ordering

Historical sessions must be replayed in deterministic order.

Canonical ordering key:

1. accepted_at timestamp  
2. receipt_hash  

The receipt hash serves as a deterministic tie-breaker.

All compliant implementations must produce identical replay order.

---

# 5. Compilation Procedure

Compilation executes the following deterministic procedure:

1. Rehydrate the engine using the provided domain graph.  
2. Build the resolution index from accepted sessions.  
3. Collect historical sessions to be compiled.  
4. Verify receipts for each session.  
5. Sort sessions using canonical replay ordering.  
6. Replay sessions sequentially.

For each session:

- Attempt acceptance using the standard engine acceptance rules.  
- If acceptance succeeds, create the resolution and update the resolution index.  
- If acceptance fails, mark the session as REPLAY_REJECTED.

Replay must obey the same legitimacy validation rules as runtime acceptance.

Compilation must never bypass governance rules.

---

# 6. Runtime Precedence Rule

If the runtime DAG already contains an accepted session that conflicts with a replayed session:

The replayed session must be marked REPLAY_REJECTED.

Runtime legitimacy always takes precedence over compilation replay.

Compilation must never invalidate previously accepted sessions.

---

# 7. Dual Replay Conflict Rule

If two replayed sessions conflict during compilation:

The session appearing earlier in canonical replay order must be accepted.

The later session must be marked REPLAY_REJECTED.

This ensures deterministic DAG construction across implementations.

---

# 8. Supersession Validation During Replay

Supersession targets must be structurally valid.

A session attempting to supersede a resolution must reference a resolution whose state is:

- ACTIVE  
or  
- UNDER_REVIEW  

Supersession must not target a resolution whose state is:

- RETIRED  
- SUPERSEDED  

If a session attempts to supersede an invalid target resolution, the session must be marked REPLAY_REJECTED.

---

# 9. Replay Rejection Conditions

A session must be marked REPLAY_REJECTED when any of the following conditions occur:

- Supersession conflict with an already accepted resolution  
- Invalid governance context  
- Missing resolution dependency  
- Receipt verification failure  
- Runtime precedence violation  
- Invalid supersession target  
- Structural integrity violation  

Replay rejected sessions must not produce:

- new resolutions  
- supersession edges  
- legitimacy receipts  

Replay rejection exists only during compilation.

Runtime sessions must never enter REPLAY_REJECTED state.

---

# 10. Incremental Compilation

Incremental compilation allows historical sessions to be applied without reconstructing the entire DAG.

During incremental compilation the engine must:

- verify session receipts  
- validate resolution dependencies  
- update the resolution index  
- enforce deterministic replay rules  

If required dependencies are missing, compilation must fail with:

RESOLUTION_MISSING

Compilation must not proceed until required domain objects are supplied.

---

# 11. Administrative State Handling

Administrative review states must not affect canonical replay behavior.

During compilation:

- UNDER_REVIEW must not affect replay ordering  
- UNDER_REVIEW must not alter structural legitimacy  
- UNDER_REVIEW must not affect supersession graph reconstruction  

Administrative states affect runtime evaluation only.

They must not change legitimacy history during compilation.

---

# 12. Determinism Requirement

Compilation must be fully deterministic.

Given identical inputs:

- domain objects  
- sessions  
- receipts  
- timestamps  

All compliant engines must produce identical results for:

- resolution index  
- supersession graph  
- ACTIVE resolution set  
- replay rejection outcomes  

Heuristics, non-deterministic ordering, or environment-dependent behavior are prohibited.

---

# 13. Audit Requirements

Compilation must emit audit events documenting compilation activity.

Required events include:

- COMPILATION_STARTED  
- SESSION_IMPORTED  
- SESSION_REPLAY_ACCEPTED  
- SESSION_REPLAY_REJECTED  
- COMPILATION_COMPLETED  

Audit records must include:

- session_id  
- resolution_id  
- receipt_hash  
- rejection_reason  

Audit events must not influence legitimacy evaluation.

Audit is observational only.

---

# 14. Engine Invariants

Compilation must never:

- rewrite accepted sessions  
- fabricate supersession edges  
- modify receipts  
- invalidate historical legitimacy  

Compilation may only:

- replay legitimate sessions  
- reconstruct the resolution index  
- reject invalid sessions deterministically  

The engine remains a deterministic legitimacy compiler.