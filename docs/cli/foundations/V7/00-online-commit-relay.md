# Charter V7 — Foundation Specification
**Architecture: Online Commit Relay (Manual, No Legitimacy)**

**Status:** FOUNDATIONAL  
**Depends On:** Commit Canon, V6 Commit Semantics  
**Does Not Define:** legitimacy, decision authority, merging, interpretation, enforcement  

---

## Purpose

Charter V7 introduces a **manual, online commit relay feature** for the CLI while remaining fully agnostic to semantics and legitimacy.  

It exists to:

- Store commits immutably  
- Route commits between users, teams, or systems  
- Make historical commits available for consumption  
- Preserve imported and local commit history intact  

It does **not**:

- Interpret commits  
- Assign legitimacy  
- Merge histories  
- Maintain or enforce current state  
- Make decisions  

**Note:** All relay operations are manual. Users explicitly push, fetch, and import commits, similar to Git. No automation or background enforcement occurs.

---

## 1. Commit Store Model

- **One commit store per context workspace**  
  - A workspace may represent a user, team, or system  
  - Contains all commits for that context, including all areas  

- **Commit Properties**  
  - Opaque: V7 treats contents as uninterpreted  
  - Append-only and immutable  
  - Referenceable: parents, linked areas, or related commits  
  - UUID-identified: repeated pushes or restores are idempotent  
  - Metadata preserved: annotations, rationale, deliberates, baseline references  

- No mutable state exists; history only grows.

---

## 2. Commit Types

V7 stores all commit types defined upstream (V6 or deliberate):

- Resolution Commits — legitimate decisions anchored in canon  
- Deliberate Commits — exploratory, non-authoritative  
- Baseline Review Commits — validation of imported history  
- Import Commits — verbatim foreign history  
- Annotation Commits — rationale, context, or reversibility notes  

**Rule:** V7 treats all commit types identically; routing, storage, and referenceability are uniform. Metadata is preserved without interpretation.

---

## 3. Context Workspaces and Segregation

- Each workspace is a logical partition  
- Commits are isolated between workspaces  
- Allows multi-user, multi-team, or multi-system usage without accidental cross-over  
- Areas may overlap, but semantics remain scoped locally  
- Multiple V7 relays may exist; consumers resolve conflicts and authoritative views  

---

## 4. Handling of Imported Commits

- Imported commits are foreign: immutable and fully preserved  
- Legitimacy is never assumed locally  
- Restores may be repeated; commit UUIDs ensure idempotency  
- Baseline reviews or deliberates consuming foreign commits are local actions, outside V7  

---

## 5. Relay Semantics

V7 functions as a **manual commit router and historical feed**:

- Push commits to the relay for storage  
- Fetch commits for local import  
- Route commits to subscribed participants  
- Serve as read-only historical reference for VDS, VLS, or local Charter V6  

**Constraints:**  

- No aggregation, prioritization, or interpretation  
- Relay commits are never authoritative; legitimacy is determined by consumers  
- Operations are manual only, triggered by the CLI  

---

## 6. Deliberates and Baseline Review Support

- Deliberates may reference historical or imported commits  
- Baseline reviews can include relay commits to re-establish local context  
- V7 does not validate, summarize, or interpret these operations  
- Metadata, annotations, and rationale are fully preserved  

---

## 7. Relationship to Charter V6

- V6 can push commits to V7 or fetch commits from V7 as foreign  
- V7 treats all commit types identically; interpretation and legitimacy remain local to V6  
- V7 acts purely as a transport and historical object store  

---

## 8. Relationship to VDS and VLS

- VDS may push or fetch check-in commits  
- VLS may fetch intent or identity commits  
- V7 has no special rules per commit type; consumer modules determine usage and meaning  

---

## 9. Backup and Restore Semantics

- Full commit export can be restored into a local store in a single transaction  
- Repeated restores are safe and idempotent  
- Restored commits remain foreign; legitimacy is local  
- Relay can act purely as a backup feed without affecting live local stores  

---

## 10. Design Constraints

Charter V7 must remain:

- Optional and replaceable  
- Manual and CLI-driven  
- Stateless, boring, and transparent  
- Immutable: history cannot be altered  
- Metadata-preserving: rationale, annotations, deliberates, baseline references remain intact  

---

## 11. Closing Principle

> Charter V7 remembers **what was said**, not **what it means**.  

It is a **manual commit relay, historical store, and transport layer** for all commit types.  
All semantics, legitimacy, and decision-making remain **local to consuming systems**.  
Multiple relays, repeated restores, or overlapping workspaces are **safe and idempotent**.