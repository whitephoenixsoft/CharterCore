# File System Check (fsck)
## Charter Core Object Graph (Git → Charter Translation)

Charter Core has a cleaner graph than Git.

### Charter Object Types (MVP)

| Charter Object | Rough Git Analogy     |
| :------------- | :-------------------- |
| Area           | top-level namespace   |
| Resolution     | commit                |
| Session        | staging / transaction |
| Candidate      | pre-commit diff       |
| Vote           | commit metadata       |
| Ref            | branch pointer        |
| Object hash    | object ID             |

### Charter Reachability

An object is reachable if:

It is referenced by:
- an Area ref
- a Session ref
- a Resolution ref
- a Global ref (config, audit)

---
## Charter fsck — Mechanical Checks (Engine View)

### Category A — Structural Errors (Hard Fail)

These violate invariants.

Check | Meaning
:- | :-
Missing object | Ref points to object that doesn’t exist
Broken reference | Resolution references missing Session
Invalid state | Resolution ACTIVE without Authority
Cycles | Resolution supersedes itself
**These must fail fsck.**

---
### Category B — Unreferenced Objects (Soft Findings)

These are Git dangling objects equivalents.

Charter Term | Git Term
:- | :-
Unreferenced Object | dangling object
Orphaned Session | dangling commit
Abandoned Candidate | dangling tree
Unattached Vote | dangling blob

Example:
> “Session S-123 exists but is not referenced by any Area.”

---
### Category C — Stale References

Check | Meaning
:- | :-
Paused session with invalid authority | needs revalidation
Session referencing superseded resolution | blocked
Area missing active scope | uninitialized

---
## Human-Friendly Terms (CLI / UX Layer)

Here’s where it improves on Git.

| Git term    | Charter term        | Why       |
| :---------- | :------------------ | --------- |
| dangling    | unattached          | non-scary |
| loose       | stored              | neutral   |
| corrupt     | broken              | clearer   |
| unreachable | not part of history | explicit  |

---

### Example CLI Output

```
Unattached object detected:
- Type: Session
- ID: S-ARCH-004
- Status: Not part of any Area history
- Impact: None
- Action: Safe to ignore or prune
```

---
## Charter fsck Sections

```
Charter Integrity Check
=======================

✔ Structural integrity
✔ Authority & Scope consistency
⚠ Unattached objects found (3)
⚠ Paused sessions requiring revalidation (1)

No legitimacy violations detected.
```

---
## Mapping Git Concepts → Charter Vocabulary (Cheat Sheet)

Git | Charter
:- | :-
fsck | Integrity check
dangling | unattached
unreachable | not part of history
reflog | audit log
commit | resolution
index | active session
stash | paused session

---
## Charter Core Reachability Graph

This answers: “When is an object considered legitimate, reachable, or merely stored?”

### Object Types (MVP)
We’ll assume these object types exist in the object store:
Area
Session
Resolution
Candidate
Vote
(later: ExportBundle, ImportBundle, ReviewRecord)

### Root Sets (Very Important)

Reachability always starts from roots.

Charter has two root sets, unlike Git which mostly has refs:

#### Root Set A — Active Engine State

These are current, live references:
- All Areas in the ref store
- All active Sessions in the session store
- Global config (later)
- Global audit log (later)

#### Root Set B — Immutable History

These are historical anchors:
- All ACTIVE, SUPERSEDED, RETIRED Resolutions
- All Sessions referenced by those resolutions

⚠️ Sessions that never produced a resolution are not part of immutable history.

---
### Reachability Rules (Edges)

Think of this as a directed graph.

#### Area → Session
An Area may reference many sessions
Only sessions started in that Area
#### Area → Resolution
An Area references active Authority + Scope
It does not reference every resolution explicitly (history is queried)
#### Session → Candidate
Candidates exist only inside sessions
If the session disappears, candidates lose meaning
#### Session → Vote
Votes exist only inside sessions
#### Session → Resolution (conditional)
Only if a candidate was accepted
Exactly one resolution per acceptance
#### Resolution → Session (mandatory)
Every resolution must reference its acceptance session
#### Resolution → Resolution
Supersedes / superseded_by links

---
### Reachability Classification

After traversal, every object is in exactly one bucket:

#### 1. Reachable (Healthy)
- Reachable from root sets
- Valid references
- No invariant violations
#### 2. Unattached (Safe, Non-Blocking)
Equivalent to Git dangling objects.

Examples:
- Session created but never closed
- Session closed without acceptance
- Candidates in abandoned sessions

These are allowed.
#### 3. Broken (Invariant Violation)
Examples:
- Resolution references missing session
- Area references missing authority
- Session references superseded resolution without revalidation

These are engine errors.