# Charter V6 — Foundation Specification (Commit-Centric)

Status: FOUNDATIONAL — V6 FEATURES LOCKED  
Scope: Local legitimacy creation, commit object store, federation boundary  
Does NOT Define: server mode, background sync, shared truth, observation, caregiving, enforcement, AI interpretation

This document defines what **Charter V6 is allowed to be** and, more importantly, what it is not allowed to become.

Charter evolves by **clarifying boundaries**, not by accumulating features.

---

## 1. What Charter V6 Is

Charter V6 is a **local legitimacy engine** backed by an **append-only commit object store**.

It is:

- Offline-first  
- Human-paced  
- Explicitly authority-bound  
- Focused on legitimacy, not awareness  

Charter V6:

- Creates legitimacy through explicit human decisions  
- Preserves history as immutable memory  
- Produces durable artifacts that other systems may consume  

Charter V6 does **not** observe behavior.  
It does **not** provide caregiving.  
It does **not** optimize, enforce, or guide.

**Contrast:**

- Unlike VDS, Charter does not emit or interpret check-ins  
- Unlike V7, Charter has no online relay or shared store  

> Charter V6 exists to decide deliberately — and remember why.

---

## 2. The Commit as the Core Primitive

The **commit** is the center of gravity in Charter V6.

A commit is:

- The only durable artifact  
- The only unit of history  
- The only thing exported or imported  
- The only interface to federation  
- The only boundary between systems  

Commit properties:

- Immutable  
- Append-only  
- Referentially transparent  
- Identity-addressed or content-addressed (implementation-agnostic)  

Charter V6 has **no mutable state**.  
There is no “current state” outside what can be derived from commit history.

> Meaning lives in commits, not in runtime memory.

---

## 3. Commit Types in Charter V6

All Charter semantics are expressed as **commit types**, never as system modes.

Minimum commit types:

### Resolution Commit
- Creates legitimacy  
- Requires authority, scope, and acceptance  
- Anchors meaning for downstream systems  

### Deliberate Commit
- Exploratory and non-authoritative  
- Used for thinking, drafting, and synthesis  
- May reference other commits  

### Baseline Review Commit
- Accepts foreign history into local legitimacy  
- Freezes imported meaning  

### Import Commit
- Records foreign commits verbatim  
- Confers no legitimacy by itself  

### Annotation Commit
- Captures rationale, context, reversibility, or explanatory notes  
- Does not alter legitimacy  

All future semantics must compose from commit types, not introduce parallel mechanisms.

---

## 4. The Local Object Store

Charter V6 is backed by a **local commit object store**.

Characteristics:

- Local by default  
- Append-only  
- Typed commits  
- Referential links (parents, references, area links)  
- No garbage collection of meaning  
- Optional compaction that never rewrites history  

The object store is **not** a database of outcomes.

> It is a memory of decisions.

---

## 5. Areas, Identity, and Scope as Commit References

Identity is not stored separately.  
It emerges from commit lineage.

- Areas are referenced by commits  
- Identity is established through history  
- Scope changes are explicit commits  
- Deprecation is a commit, not a deletion  

This document does not redefine identity theory.  
It binds identity mechanics to commit structure.

---

## 6. Exports, Imports, and Foreign History

All sharing happens via **commit bundles**.

Rules:

- Exports are containers, not transformations  
- Imported commits are always foreign  
- Foreign commits have no legitimacy until baseline review  
- Baseline review is itself a commit  
- No merge semantics exist  
- There is no sync  

Only three actions exist:

1. Import foreign commits  
2. Review them  
3. Accept them explicitly  

V1-style exports are represented as single-commit bundles for portability and continuity.

---

## 7. Relationship to VDS and VLS (Commit Boundary)

Charter V6 is agnostic to its consumers.

- VDS consumes Resolution commits as decision anchors  
- VDS emits check-in commits that may be stored but are foreign  
- VLS consumes Resolution commits as intent lineage  

Charter does not interpret downstream usage.

> The commit is the interface between all systems.

---

## 8. What Charter V6 Explicitly Does NOT Do

Charter V6 will never:

- Run in server mode  
- Perform background synchronization  
- Maintain a shared or global truth  
- Observe systems or people  
- Emit caregiving signals  
- Enforce outcomes  
- Interpret intent automatically  
- Use AI to create or modify legitimacy  

These are permanent boundaries.

---

## 9. Forward Compatibility (Without Promising V7)

Charter V6 guarantees:

- Commit format stability  
- Portable storage  
- Clear federation boundaries  

Future systems may relay commits online.  
Future systems may index or mirror commits.

No future version may:

- Retroactively legitimize history  
- Centralize authority  
- Convert awareness into legitimacy  

> Legitimacy will always remain local.

---

## Final Statement

Charter V6 is complete by design.

It does not grow by doing more.  
It grows by remaining clear.

Its power comes from restraint,  
and from treating legitimacy as something that must be  
**earned deliberately and preserved honestly over time**.

