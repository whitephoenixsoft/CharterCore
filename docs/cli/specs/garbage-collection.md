# ENG-SESSION-GC — Session Garbage Collection & Retention (Foundation)  
Status: FOUNDATIONAL (Conceptual, V3+)  
Applies to: Charter CLI & Engine Core (V1/V2+)  
Scope: Conceptual model for session retention, pruning, and archival  

---

# 1. Purpose

This document defines the **conceptual foundation** for session garbage collection (GC) and retention in the Charter CLI.  

Objectives:

- Ensure long-term storage hygiene without compromising structural integrity or legitimacy.  
- Distinguish **critical sessions** (required for audit or supersession) from **non-critical sessions** (exploratory, abandoned, or baseline-reviewed).  
- Provide a framework for future CLI commands such as pruning or archiving.  
- Preserve determinism, auditability, and reconstruction capabilities.  

This specification is **conceptual**. It does not mandate Engine behavior for GC; all deletion or archival is **CLI/host-driven**.

---

# 2. Engine vs CLI Responsibilities

## 2.1 Engine Responsibilities

The Engine must:

- Enforce **structural integrity**: no session may be removed if referenced by an ACTIVE or historical legitimacy object.  
- Maintain deterministic reconstruction: all ACCEPTED sessions and their receipts must remain intact.  
- Produce immutable **receipts** for every session closure (CLOSED or ACCEPTED) to support auditability.  
- Reject any operation that would compromise the **supersession graph** or legitimacy references.  

The Engine **must not**:

- Delete or archive sessions automatically.  
- Implicitly prune abandoned or exploratory sessions.  
- Make storage decisions based on age, phase, or participation.  

---

## 2.2 CLI / Host Responsibilities

The CLI is responsible for:

- Defining retention and pruning policies for **non-critical sessions**.  
- Executing **explicit commands** to prune or archive sessions:

  - `prune_sessions(older_than=<timestamp>)`  
  - `archive_sessions(target=<storage_location>)`  

- Determining which sessions are **critical** (must be preserved):

  - ACCEPTED sessions referenced by supersession graph  
  - Sessions required for reconstructing active governance history  
  - Sessions with receipts required for audit or legal compliance  

- Maintaining a **separate storage layer** for archived sessions.  
- Ensuring audit trails (receipts) remain immutable and accessible.  

---

# 3. Session Classification

Sessions are categorized conceptually for retention purposes:

| Category | Description | Retention Guidance |
|----------|-------------|------------------|
| Critical | ACCEPTED sessions contributing to legitimacy or supersession | Must be retained indefinitely |
| Semi-Critical | Sessions under review or with explicit baseline review | Retention optional; may be archived after consolidation |
| Non-Critical | Exploration, abandoned, or obsolete sessions | Safe to prune or archive at CLI discretion |

**Note:** Only the Engine can classify a session as critical based on legitimacy references; the CLI may provide additional user-driven classification.

---

# 4. Receipt Preservation

- Every session closure (ACCEPTED or CLOSED) emits a **canonical receipt**.  
- Receipts must be preserved even if the session itself is archived.  
- Receipt preservation ensures:

  - Reconstructable audit trail  
  - Chronological history of decision-making  
  - Legal admissibility (where required)  

- CLI pruning/archival operations must not delete or modify receipts associated with critical sessions.  

---

# 5. Conceptual Garbage Collection Policy

1. **Engine-Invariant Guarantee:**  
   The Engine guarantees that **critical session objects** remain intact; removal or pruning is forbidden.

2. **CLI-Driven Archival / Pruning:**  
   The CLI may archive or prune non-critical sessions explicitly, using deterministic, user-defined rules.  

3. **Determinism & Safety:**  
   - All pruning or archival must preserve referential integrity.  
   - Orphan detection must prevent deletion of any session referenced by ACTIVE or historical legitimacy.  

4. **Audit & Compliance:**  
   - Receipts are immutable and persist even if sessions are archived.  
   - Audit logs must remain reconstructable from preserved sessions and receipts.  

---

# 6. Conceptual Mental Model

- The Engine enforces **legitimacy integrity**, not storage hygiene.  
- The CLI manages **session lifecycle beyond terminal states**, including pruning and archival.  
- Only exploratory, abandoned, or baseline-reviewed sessions are eligible for pruning.  
- ACCEPTED sessions and receipts form the **immutable historical backbone**.  
- Garbage collection is explicit, deterministic, and user-controlled.  

---

# 7. Future Considerations

- Potential CLI commands:

  - `prune_sessions(criteria)` → remove non-critical sessions from active storage  
  - `archive_sessions(target)` → move sessions to secondary storage while preserving receipts  
  - `restore_archived_sessions()` → rehydrate archived sessions for review or audit  

- Integration with V3 features, such as **non-legitimate thinking sessions** and **enhanced exploratory tracking**.  
- Establish default retention policies for exploratory sessions while guaranteeing audit compliance.  

---

# 8. Scope Limitations

- This is a **conceptual foundation spec**; implementation details are CLI-dependent.  
- The Engine must remain **agnostic** to pruning and archival; all operations are **non-destructive with respect to legitimacy**.  
- The spec does not define retention duration, storage medium, or pruning heuristics.  

---

# 9. Summary Guarantees

- Engine preserves all critical sessions and receipts.  
- Non-critical sessions may be pruned or archived explicitly by the CLI.  
- Receipts remain immutable and reconstructable.  
- Supersession and legitimacy integrity are never compromised by pruning.  
- CLI-driven session GC is deterministic, explicit, and auditable.  

---

This specification establishes the **conceptual foundation for session garbage collection and retention** in Charter CLI, ensuring hygiene without compromising Engine integrity.