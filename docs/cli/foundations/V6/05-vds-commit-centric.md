# Charter V6 — Foundation Specification (Commit-Centric)
Version: 2.0  
Status: FOUNDATIONAL — V6 FEATURES LOCKED  
Scope: Local legitimacy creation, commit object store, federation boundary  
Does NOT Define: server mode, background sync, shared truth, observation, caregiving, enforcement, AI interpretation

Charter V6 defines what **is allowed** and, more importantly, what it is **not allowed** to become.  
Its design evolves by **clarifying boundaries**, not by accumulating features.

---

## 1. What Charter V6 Is

Charter V6 is a **local legitimacy engine** backed by an **append-only commit object store**.

It is:

- Offline-first  
- Human-paced  
- Explicitly authority-bound  
- Focused on legitimacy, not awareness  

Charter V6:

- Creates legitimacy through **explicit human decisions**  
- Preserves history as **immutable memory**  
- Produces durable **artifacts** for optional downstream consumption  

It does **not**:

- Observe behavior  
- Provide caregiving or guidance  
- Optimize, enforce, or direct action  

**Contrast:**

- Unlike VDS, Charter does **not emit or interpret check-ins**  
- Unlike V7, Charter has **no online relay or shared store**

> Charter V6 exists to decide deliberately — and remember why.

---

## 2. The Commit as the Core Primitive

The **commit** is the fundamental building block of Charter V6.

A commit is:

- The only durable artifact  
- The only unit of history  
- The only interface to federation  
- The only boundary between systems  

Commit properties:

- Immutable and append-only  
- Referentially transparent  
- Identity-addressed or content-addressed (implementation-agnostic)  

Charter V6 maintains **no mutable state**.  
No “current state” exists outside what can be derived from commit history.

> Meaning lives in commits, not runtime memory.

---

## 3. Commit Types in Charter V6

All semantics are expressed via **commit types**, not system modes.

### Resolution Commit
- Creates legitimacy  
- Requires authority, scope, and acceptance  
- Anchors meaning for downstream systems  

### Deliberate Commit
- Exploratory and non-authoritative  
- Used for drafting, thinking, and synthesis  
- May reference other commits  

### Baseline Review Commit
- Integrates foreign history into local legitimacy  
- Freezes imported meaning  
- Supports explicit evaluation  

### Import Commit
- Records foreign commits verbatim  
- Confers no legitimacy by itself  

### Annotation Commit
- Captures rationale, context, reversibility, or commentary  
- Does not alter legitimacy  

> Future semantics must compose from these commit types; no parallel mechanisms are allowed.

---

## 4. The Local Object Store

Charter V6 stores commits in a **local object store**.

Characteristics:

- Local by default  
- Append-only  
- Typed commits with referential links (parents, references, area links)  
- No garbage collection of meaning  
- Optional compaction, never rewriting history  

> The object store is **a memory of decisions**, not a database of outcomes.

---

## 5. Areas, Identity, and Scope as Commit References

Identity **emerges from commit lineage**, not as a separate object.

- Areas are referenced by commits  
- Identity is established through historical record  
- Scope changes are explicit commits  
- Deprecation is recorded as a commit, not deletion  

> This approach binds identity mechanics directly to the commit structure.

---

## 6. Exports, Imports, and Foreign History

All sharing happens via **commit bundles**:

- Exports are **containers**, not transformations  
- Imported commits are always **foreign**  
- Foreign commits confer **no local legitimacy** until explicitly accepted via a baseline review  
- No merge semantics exist  
- No automatic synchronization occurs  

Only three actions are defined:

1. Import foreign commits  
2. Review them  
3. Accept explicitly  

> V1-style exports remain **single-commit bundles** for portability and continuity.

---

## 7. Relationship to VDS and VLS

Charter V6 is **agnostic to consumers**:

- VDS consumes Resolution commits as decision anchors  
- VDS may emit check-in commits, stored locally as foreign  
- VLS consumes Resolution commits as intent lineage  

> The commit is the **interface between all systems**.

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

These are **permanent boundaries**.

---

## 9. Forward Compatibility (Without Promising V7)

Charter V6 guarantees:

- Commit format stability  
- Portable, local storage  
- Clear boundaries for federation  

Future systems may:

- Relay commits online  
- Index or mirror commits  

Future versions **cannot**:

- Retroactively legitimize history  
- Centralize authority  
- Convert awareness into legitimacy  

> Local legitimacy remains **deliberate, earned, and preserved honestly**.

---

## 10. Integration with Charter CLI

V6 commits form the **CLI’s interface** for all local operations:

- Voting on resolutions requires participant identity (alias or key)  
- Commits are tied to areas via unique IDs  
- Check-ins, annotations, and baseline reviews are **optional, human-initiated**, and stored locally  
- Federation boundaries and artifacts are respected

> The CLI is the **workhorse**; commits are the single source of truth locally.

---

## Final Statement

Charter V6 is **complete by design**:

- It does **not grow by adding features**  
- It grows by **clarifying boundaries and preserving legitimacy**  

Its power comes from **restraint**,  
and from treating legitimacy as something that must be:

**earned deliberately and preserved honestly over time**.