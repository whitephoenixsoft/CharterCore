# Charter V7 — Foundation Specification  
**Architecture: Online Commit Relay (No Legitimacy)**

**Status:** FOUNDATIONAL  
**Depends On:** Commit Canon, V6 Commit Semantics  
**Does Not Define:** legitimacy, decision authority, merging, interpretation, enforcement

---

## Purpose

Charter V7 is an **online commit relay and historical object store**.

It exists to:

- Store commits immutably
- Route commits across contexts and participants
- Make historical commits available for consumption
- Preserve all imported and local commit history

It does **not**:

- Interpret commits
- Assign legitimacy
- Merge histories
- Maintain or enforce current state
- Make decisions

---

## 1. Commit Store Model

- **One commit store per context workspace**  
  - A workspace may represent a user, team, or system
  - Contains all commits for that context, including all areas
- Commits are:
  - **Opaque**: no internal semantics assigned by V7
  - **Append-only**
  - **Immutable**
  - **Referenceable**: parents, linked areas, or related commits
- No mutable state exists; history grows only

---

## 2. Commit Types (Opaque, Routed)

V7 stores all commit types defined upstream (V6 or deliberate):

- **Resolution Commits**: legitimacy anchors (foreign in V7)
- **Deliberate Commits**: exploratory, non-authoritative
- **Baseline Review Commits**: validate imported history
- **Import Commits**: verbatim foreign history
- **Annotation Commits**: rationale, context, reversibility notes

**Rule:** V7 treats all commit types identically — routing and storage only.

---

## 3. Context Workspaces and Segregation

- Each workspace is a **logical partition**
- Commits in a workspace are **isolated from other workspaces**
- Allows secure multi-user, multi-system, or multi-team operation
- Areas may overlap across workspaces but remain semantically scoped

---

## 4. Handling of Imported Commits

- All commits pushed or fetched from external sources are **foreign**
- Foreign commits:
  - Remain immutable
  - Do not create legitimacy locally
  - Can be stored, retrieved, and referenced
- Baseline reviews or deliberates that consume foreign commits are **local actions**, outside V7

---

## 5. Relay Semantics

V7 acts as a **commit router and historical feed**:

- Push commits to the relay for storage
- Fetch commits for local import
- Route commits to subscribed participants
- Supports read-only consumption by VDS, VLS, or local Charter V6
- No aggregation, interpretation, or prioritization occurs

**Constraint:** Relay commits are never authoritative — legitimacy is always determined by the consuming system.

---

## 6. Deliberates and Baseline Review Support

- Deliberate commits may reference historical or imported commits
- Baseline reviews can include relay commits to re-establish context locally
- V7 does **not** validate, summarize, or interpret these operations
- Supports complete history reconstruction in local stores

---

## 7. Relationship to Charter V6

- V6 may push commits to V7
- V6 may fetch commits from V7 as foreign
- V7 does not distinguish between commit types, but V6 can interpret locally
- V7 remains a **transport and historical object store only**

---

## 8. Relationship to VDS and VLS

- VDS may push or fetch **check-in commits**
- VLS may fetch **intent commits**
- V7 has no special rules per type; consumers decide how to process commits

---

## 9. Backup and Restore Semantics

- Full commit export can be restored into a local store
- Restores entire history, including all prior V6 and deliberate commits
- Restored commits remain foreign; legitimacy is local
- Relay can serve as a **backup-only feed** without affecting local live stores

---

## 10. Design Constraints

Charter V7 must remain:

- Optional
- Replaceable
- Boring: no semantic interpretation
- Immutable: history cannot be altered
- Stateless: no hidden authority, no decision-making

---

## 11. Closing Principle

> Charter V7 remembers **what was said**, not **what it means**.

It is a **historical commit relay**, a transport layer for all commit types, and a complete memory store. All meaning, legitimacy, and decision-making remain **local to the consuming systems**.