# Charter V7 — Foundation Specification  
**Architecture: CLI Commit Relay (Append-Only, No Legitimacy)**

**Status:** FOUNDATIONAL  
**Depends On:** Commit Canon, V6 Commit Semantics  
**Does Not Define:** legitimacy, decision authority, merging, interpretation, enforcement, or state reconstruction  

---

## Purpose

Charter V7 defines a **CLI-mediated commit relay mechanism** for transporting and archiving Charter commits.

It introduces:

- An append-only commit store  
- Manual push and fetch operations  
- Configurable relay endpoints (local or remote)  
- Immutable archival preservation of all commit types  

V7 is **not a hosted service** and does not require a persistent runtime process.  
All operations are explicitly invoked through the Charter CLI.

V7 exists to:

- Store commits immutably  
- Transport commits between users, teams, or systems  
- Preserve exported history intact  
- Provide a historical feed for consuming systems  

V7 does **not**:

- Interpret commits  
- Assign legitimacy  
- Merge histories  
- Reconstruct canonical state  
- Compute active resolutions  
- Infer supersession  
- Make decisions  

Relay endpoints function as archival libraries of typed commits.

---

## 1. Deployment and CLI Model

V7 is:

- CLI-invoked only  
- Explicitly configured (similar to Git remotes)  
- Manual: push, fetch, and import are user-triggered  
- Transport-agnostic (filesystem, SSH, HTTPS, or equivalent mechanisms)  

V7 does not define:

- Authentication mechanisms  
- Access control policies  
- Server-side enforcement  
- Background synchronization  

Relay endpoints may be local or remote but are not defined as Charter services.

---

## 2. Commit Store Model

Each relay endpoint contains:

- **One append-only commit store per workspace**

A workspace:

- Represents a user, team, or system context  
- Contains all commits for that context  
- Is logically isolated from other workspaces  

### Commit Properties

All commits are:

- Opaque (relay does not interpret contents)  
- Immutable and append-only  
- UUID-identified (idempotent across repeated pushes or restores)  
- Chronologically ordered for storage purposes only  

Commits may contain structural references (parents, linked areas, related commits), but:

- References are metadata only  
- Relay does not validate or enforce reference integrity  
- Ordering does not imply dependency  
- Commits do not form a mutable state chain  

No mutable state exists in the relay. History only grows.

---

## 3. Commit Types

V7 stores all upstream commit types, including Receipts.

### Canonical Types

- Resolution Commits — canonical resolution objects (smallest legitimacy unit)  
- Legitimacy Receipt Commits — immutable records of session closure that produced resolutions  
- Exploration Receipt Commits — records of deliberate or breakout closure  
- Review Receipt Commits — records of baseline review closure  
- Deliberate Commits — exploratory, non-authoritative artifacts  
- Baseline Review Commits — review workspace artifacts  
- Import Commits — verbatim foreign history  
- Annotation Commits — rationale or contextual metadata  

### Handling Rule

V7:

- Treats all commit types identically  
- Stores and routes them uniformly  
- Preserves metadata in full fidelity  
- Does not interpret receipt categories  
- Does not evaluate authority  
- Does not derive legitimacy from receipts  

Receipts are first-class commit objects, but the relay does not interpret their meaning.

---

## 4. Independence and Archival Model

Commits in V7 are:

- Independent archival entries  
- Self-contained historical objects  
- Not dependent on replay order for validity  

The relay behaves as an archival library, not a version-control state machine.

Unlike Git:

- There is no working tree  
- There is no shared mutable branch state  
- There is no automatic merge logic  
- There is no implied layering of changes  

Consumers reconstruct meaning locally if needed.

---

## 5. Relay Semantics

Relay operations are manual and CLI-triggered:

- Push commits to a relay endpoint  
- Fetch commits from a relay endpoint  
- Import fetched commits into a local workspace  

Relay constraints:

- No aggregation  
- No prioritization  
- No summarization  
- No interpretation  
- No legitimacy evaluation  
- No state reconstruction  

Receipts:

- Are preserved identically to other commit types  
- Retain canonical identifiers  
- Maintain lineage across push and fetch operations  
- Are never aggregated or synthesized into decisions  

Legitimacy is always determined by the consuming engine.

---

## 6. Handling of Imported Commits

Fetched commits are treated as:

- Foreign  
- Immutable  
- Historically preserved  

Repeated fetch or restore operations are safe and idempotent due to UUID identity.

The relay does not:

- Validate legitimacy  
- Enforce authority  
- Promote foreign resolutions to local canonical state  

Baseline reviews or sessions consuming foreign commits are local engine actions and outside V7 scope.

---

## 7. Deliberates, Baseline Review, and Receipts

V7 stores without interpretation:

- Deliberate artifacts  
- Baseline review artifacts  
- Exploration Receipts  
- Review Receipts  
- Legitimacy Receipts  

Receipts emitted at lifecycle boundaries are committed unchanged.

The relay:

- Does not aggregate receipts  
- Does not synthesize legitimacy  
- Does not compute roll-ups  
- Does not interpret lifecycle transitions  

---

## 8. Relationship to Charter V6

V6:

- Pushes commits to V7  
- Fetches commits from V7 as foreign  
- Evaluates authority locally  
- Reconstructs canonical state locally  

V7:

- Does not evaluate legitimacy  
- Does not compute active resolutions  
- Does not interpret supersession  

Resolution remains the smallest legitimacy unit.  
Receipts document lifecycle events only.

---

## 9. Relationship to VDS and VLS

VDS and VLS may:

- Push check-in commits  
- Fetch identity or intent commits  
- Consume receipt commits for traceability  

V7:

- Applies no special rules per commit type  
- Does not distinguish governance from communication  
- Stores all commits uniformly  

Interpretation remains local to consuming systems.

---

## 10. Backup and Restore Semantics

V7 may function as:

- A transport endpoint  
- A historical archive  
- A backup library  

Full commit exports:

- May be restored in a single transaction  
- Are idempotent  
- Remain foreign until locally evaluated  

Relay restore does not alter legitimacy or canonical state.

---

## 11. Design Constraints

Charter V7 must remain:

- CLI-driven  
- Manual and explicit  
- Append-only  
- Immutable  
- Stateless with respect to canonical state  
- Interpretation-free  
- Legitimacy-neutral  
- Metadata-preserving  
- Receipt-preserving  

It must never:

- Compute authority  
- Derive legitimacy  
- Aggregate receipts into decisions  
- Construct shared mutable state  

---

## 12. Closing Principle

Charter V7 remembers what was recorded, not what it means.

It is a CLI-mediated archival transport layer for typed commits, including:

- Resolutions (legitimacy units)  
- Exploration, Review, and Legitimacy Receipts  
- Deliberate and baseline artifacts  
- Communication and check-in commits  

All semantics, legitimacy, and decision-making remain local to consuming systems.

Multiple relays, repeated restores, and overlapping workspaces are safe, idempotent, and interpretation-free.