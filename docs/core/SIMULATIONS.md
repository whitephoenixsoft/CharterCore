# Charter Core — Canonical Simulations (Engine Only)

## Purpose

This document records **design-validation simulations** for Charter Core.

These are:
- not tests  
- not UI flows  
- not product promises  

They answer one question:

**Does Charter Core preserve legitimacy, determinism, and historical integrity under real pressure?**

If any simulation fails, the engine has violated a **core invariant**.

---

## I. Bootstrap & Governance Formation

### Simulation 1 — Mandatory Area Initialization
**Context**  
A new Area is created. No decisions are allowed until governance exists.

**Actors**  
- Alice  
- Bob  

**Flow**
1. Area is created
2. System requires initialization
3. Authority candidates proposed
4. Scope candidates proposed
5. Authority accepted
6. Scope accepted

**Outcome**
- Exactly one active Authority
- Exactly one active Scope
- Area becomes initialized

**Validated Invariants**
- Areas require explicit governance
- Authority and Scope are first-class resolutions
- No silent defaults

---

## II. Basic Decision Making

### Simulation 2 — Flat Authority Collaboration
**Context**  
Two peers collaborate without hierarchy.

**Flow**
1. Session opened in initialized Area
2. Multiple candidates proposed
3. Both participants accept one candidate
4. Resolution created
5. Later session supersedes it

**Outcome**
- Legitimate decision
- Clear supersession lineage

**Validated Invariants**
- Explicit acceptance
- Supersession preserves history
- Authority need not be hierarchical

---

### Simulation 3 — Single-User Governance
**Context**  
Solo founder uses Charter as a decision journal.

**Flow**
1. Self-authority and scope defined
2. Candidates proposed
3. One explicitly accepted
4. Later superseded

**Outcome**
- Full audit trail
- No shortcuts or implicit legitimacy

**Validated Invariants**
- Scale independence
- Explicit acceptance
- Immutable decision memory

---

### Simulation 4 — Standard Team Decision
**Context**  
Team chooses an architecture.

**Flow**
1. Session opened
2. Candidates proposed
3. Positions recorded
4. Authority rule satisfied

**Outcome**
- Resolution references Area, Authority, Scope

**Validated Invariants**
- Sessions are the unit of legitimacy
- Authority evaluated mechanically
- Scope is recorded, not enforced

---

### Simulation 5 — Partial Acceptance
**Context**  
Multiple goals proposed.

**Flow**
1. Candidates proposed
2. Some accepted
3. Others rejected or undecided

**Outcome**
- Only accepted candidates become resolutions

**Validated Invariants**
- Explicit acceptance only
- No implied decisions

---

## III. Blocking, Disagreement, and Reality

### Simulation 6 — Deadlock Without Abuse
**Context**  
Unanimous authority, disagreement.

**Flow**
1. Votes split
2. Authority not satisfied
3. Session becomes BLOCKED

**Outcome**
- No resolution created

**Validated Invariants**
- Deterministic evaluation
- No coercion
- Explicit blocking

---

### Simulation 7 — Participant Leaves Mid-Session
**Context**  
Deadlocked session changes composition.

**Flow**
1. Alice, Bob, Charlie present
2. Charlie leaves
3. Authority re-evaluated
4. Resolution accepted

**Outcome**
- Legitimate acceptance
- Departure recorded

**Validated Invariants**
- Standing is action-based
- No reinterpretation of past votes

---

## IV. Authority & Scope Evolution

### Simulation 8 — Authority Change Requires Session
**Context**  
Team wants faster decisions.

**Flow**
1. New session opened
2. New Authority proposed
3. Old Authority governs acceptance
4. New Authority accepted

**Outcome**
- Authority evolves explicitly

**Validated Invariants**
- Authority mutable only via resolutions
- Non-retroactivity

---

### Simulation 9 — Scope Awareness Without Enforcement
**Context**  
Candidate outside stated scope.

**Flow**
1. Candidate proposed
2. Participants reject manually

**Outcome**
- No mechanical enforcement

**Validated Invariants**
- Scope is informational only
- No semantic interpretation

---

## V. References & Cross-Area Awareness

### Simulation 10 — Multi-Area References
**Context**  
Decision touches multiple domains.

**Flow**
1. Session opened in primary Area
2. Other Areas referenced explicitly
3. Acceptance governed only by primary Authority

**Outcome**
- Awareness without authority leakage

**Validated Invariants**
- Referencing is informational
- Authority remains singular

---

### Simulation 11 — Late Discovery of Overlap
**Context**  
Impact discovered months later.

**Flow**
1. Original session paused or closed
2. New governance created elsewhere

**Outcome**
- No retroactive invalidation

**Validated Invariants**
- Context preservation
- Pause vs close distinction

---

### Simulation 12 — Closed Sessions Are Historical
**Context**  
Closed session referenced later.

**Outcome**
- History informs but does not govern

**Validated Invariants**
- Sessions are not reusable
- Resolutions carry legitimacy

---

## VI. Structural Separation

### Simulation 13 — Separate Areas, Separate Authorities
**Context**  
Engineering and Finance coexist.

**Outcome**
- No inferred overlap

**Validated Invariants**
- Areas are hard boundaries
- No semantic inference

---

### Simulation 14 — Blanket Authority Ignored Unless Referenced
**Context**  
High-level authority exists elsewhere.

**Outcome**
- No effect unless explicitly referenced

**Validated Invariants**
- Authority is opt-in
- Silence has no meaning

---

### Simulation 15 — Explicit Authority Conflict Resolution
**Context**  
Multiple Authorities referenced.

**Flow**
1. Engine detects conflict
2. User resolves via session

**Outcome**
- Explicit supersession

**Validated Invariants**
- No silent precedence
- Intent must be declared

---

## VII. Imports, Divergence, and History

### Simulation 16 — Flat Import (First Adoption)
**Context**  
Organization adopts Charter.

**Flow**
1. Import in CONSOLIDATE mode

**Outcome**
- Resolutions created as UNDER_REVIEW
- No Authority or Scope inferred

**Validated Invariants**
- Legitimacy is session-bound
- Import fabricates nothing

---

### Simulation 17 — Concurrent Sessions with Supersession
**Context**
- Active resolution exists

**Flow**
1. Session references active resolution
2. Another session supersedes it

**Outcome**
- Referencing session flagged for revalidation

**Validated Invariants**
- Concurrency isolation
- Supersession triggers revalidation

---

### Simulation 18 — RESTORE Import
**Context**  
Verified export restored.

**Outcome**
- Identical historical state
- No new legitimacy created

**Validated Invariants**
- Deterministic rehydration
- Import does not fabricate authority

---

### Simulation 19 — Divergent Authority Review
**Context**  
Imported Authority rejected locally.

**Outcome**
- No retroactive changes
- Clear audit trail

**Validated Invariants**
- Authority non-retroactivity
- Explicit legitimacy

---

### Simulation 20 — Conflicting Consolidation Timeline
**Context**  
Local and imported paths diverge.

**Outcome**
- Imported history preserved
- Local legitimacy explicit

**Validated Invariants**
- No silent supersession
- Local acceptance dominates

---

## VIII. Export & Forking Safety

### Simulation 21 — Export Excludes Active Sessions
**Context**  
Decision in progress.

**Outcome**
- Active session excluded

**Validated Invariants**
- Legitimacy cannot be forked

---

### Simulation 22 — Private Forking Attempt Prevented
**Context**  
Export copied mid-session.

**Outcome**
- Acceptance rejected

**Validated Invariants**
- Sessions are the unit of legitimacy
- No legitimacy laundering

---

## IX. Storage & Identity Isolation

### Simulation 23 — Separate Instances Never Collide
**Context**  
Two unrelated instances.

**Outcome**
- No shared identity or history

**Validated Invariants**
- Storage isolation
- No global singleton

---

### Simulation 24 — Accidental ID Copy Is Rejected
**Context**  
Resolution ID pasted across instances.

**Outcome**
- Reference fails explicitly

**Validated Invariants**
- Explicit import required

---

## X. Audit Supremacy

### Simulation 25 — Audit Outlives Its Subject
**Context**  
Area deleted.

**Outcome**
- Global audit records deletion

**Validated Invariants**
- Audit scope supremacy

---

## XI. Storage Determinism

### Simulation 26 — Storage Location Irrelevance
**Context**  
Same data, different paths.

**Outcome**
- Identical IDs and outcomes

**Validated Invariants**
- Storage agnosticism

---

### Simulation 27 — Storage Relocation Safety
**Context**  
Data moved physically.

**Outcome**
- Identity preserved

**Validated Invariants**
- Object identity stability

---

## XII. Export & Import Integrity

### Simulation 28 — Export Is Complete Snapshot
**Outcome**
- No dangling references

**Validated Invariants**
- Referential completeness

---

### Simulation 29 — Failed Import Is Side-Effect Free
**Outcome**
- No partial state
- Audit records failure

**Validated Invariants**
- Side-effect-free failure
- Audit durability

---

## XIII. Determinism Across Systems

### Simulation 30 — Independent Restores Match Exactly
**Outcome**
- Identical resolution graphs and audits

**Validated Invariants**
- Deterministic rehydration

---

## Final Observation

Charter Core does not optimize for:
- speed
- convenience
- agreement

It optimizes for:
- legitimacy
- determinism
- historical integrity

Nothing happens unless someone explicitly decides it should.

These simulations intentionally exclude:
- CLI UX
- prompts or warnings
- ergonomics
- context switching commands

They validate only the **engine’s invariants**.

If these simulations hold, Charter Core is trustworthy.
If any fail, the engine is incorrect.