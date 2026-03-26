# Charter — Physical Deletion Guidance

Status: ADVISORY (Non-Enforced)  
Applies to: Operators, administrators, and infrastructure owners  
Does NOT apply to: Engine semantics, CLI behavior, or legitimacy rules  

---

## 1. Purpose

This document defines the implications and recommended practices for **physical deletion of Charter data**.

Charter is designed as an append-only, auditable system.

Deletion is therefore:

- external to Charter semantics  
- irreversible  
- outside legitimacy guarantees  

This document exists to:

- clarify consequences  
- distinguish deletion from archival  
- prevent accidental loss of institutional memory  
- guide safe operational practices  

---

## 2. Core Principle

> Deletion destroys evidence.  
> Archival preserves it.

Charter does not provide a deletion primitive.

Any deletion is performed at the **storage layer**, not within Charter.

---

## 3. What Deletion Means

Physical deletion removes:

- object store data  
- audit history  
- receipts  
- session records  
- resolutions  
- provenance (engine version, spec_set_hash)  

After deletion:

- legitimacy may no longer be provable  
- audit reconstruction may be impossible  
- historical meaning may be permanently lost  

Deletion is not reversible.

---

## 4. What Deletion Is Not

Deletion is not:

- archiving  
- supersession  
- baseline rejection  
- workflow cleanup  
- context switching  

Deletion does not:

- preserve lineage  
- preserve references  
- preserve spec provenance  

Deletion is destruction, not transformation.

---

## 5. Recommended Alternative: Archival

Before deleting, consider:

- Archival into a new Area  
- Export via CCE or deliberate export  
- Storage in relay or external archive  
- Cold storage relocation  

Archival preserves:

- identity  
- lineage  
- legitimacy  
- auditability  

Deletion removes all of these.

---

## 6. Acceptable Use Cases for Deletion

Deletion may be appropriate only in cases such as:

- legal or regulatory requirements (e.g., data removal mandates)  
- test environments  
- corrupted or invalid data that cannot be repaired  
- explicit, informed institutional policy  

Even in these cases:

- archival should be considered first  
- deletion scope should be minimal and explicit  

---

## 7. Recommended Deletion Procedure

If deletion is required:

### 7.1 Create a Final Archive

- export all relevant Areas (CCE)
- verify integrity (hash + spec_set_hash)
- store archive in durable storage

---

### 7.2 Record Deletion Intent (Out-of-Band)

Because Charter cannot record its own deletion:

- create an external record (log, ticket, signed document)
- include:
  - timestamp
  - operator identity
  - scope of deletion
  - reason for deletion
  - archive location (if created)

---

### 7.3 Perform Physical Deletion

Delete at the storage layer:

- context directories
- object stores
- audit logs
- commit stores (if applicable)

Ensure:

- no partial deletion (avoid inconsistent state)
- no mixed retained/removed lineage

---

### 7.4 Verify Removal

- confirm data is no longer accessible
- confirm indexes and caches are cleared
- confirm no residual references remain

---

## 8. Partial Deletion Warning

Deleting subsets of data may:

- break referential integrity  
- invalidate receipts  
- prevent legitimacy verification  
- produce inconsistent historical state  

Partial deletion is strongly discouraged unless:

- the system is being fully retired  
- or the remaining dataset is independently valid  

---

## 9. Relationship to Relay and Backups

Deletion in one location does not imply deletion elsewhere.

Copies may exist in:

- relay endpoints  
- exported files  
- backups  
- other contexts  

Full removal requires:

- identifying all copies  
- deleting each explicitly  

---

## 10. Security Considerations

For sensitive data:

- deletion may require secure wipe procedures  
- consider filesystem-level or disk-level sanitization  
- consider encryption-based deletion (destroying keys)  

Charter does not enforce or verify secure deletion.

---

## 11. Design Position

Charter is intentionally designed to:

- preserve history  
- prevent silent loss  
- make deletion explicit and external  

If deletion becomes easy or implicit,  
the system’s guarantees are weakened.

---

## 12. Final Principle

> If legitimacy matters, deletion should be rare.  
> If deletion is required, it must be explicit, external, and irreversible.