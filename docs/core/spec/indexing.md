Perfect. Here is #1 — ENG-IDX, written in the same engine-internal RFC style as the Ref Store and Object Store specs.
Take it, read it later, and don’t worry about implementation yet. This is about locking the mental model so future coding feels obvious instead of fragile.
Charter Core — Index Specification
Document ID: ENG-IDX
Status: FROZEN (v1)
Audience: Charter Core engine implementers
Scope: Engine-internal only (never exported)
1. Purpose
ENG-IDX-01 — Indexes Exist for Performance Only
Indexes exist solely to make common queries fast.
Indexes:
Do not define legitimacy
Do not define history
Do not define authority
Do not define truth
Fail if:
Engine behavior changes when an index is missing
Index data is treated as authoritative
2. Core Principle
ENG-IDX-02 — Indexes Are Derivable
Every index MUST be fully derivable from:
Object Store
Ref Store
Audit Store (if needed)
Indexes MUST NOT be the only place where information exists.
Fail if:
Loss of an index causes loss of meaning
Index corruption alters engine behavior
3. Index Lifecycle
ENG-IDX-03 — Indexes Are Rebuilt at Boot
On engine startup:
Load object store
Load ref store
Rebuild all indexes in memory
Indexes are:
In-memory by default
Discardable
Rebuildable
Fail if:
Indexes are required to persist across restarts
Startup assumes precomputed index correctness
4. Required Indexes (v1)
ENG-IDX-04 — Resolution State Index
The engine MUST maintain an index of resolutions by lifecycle state:
Copy code

(state, area_id) → Set<ResolutionHash>
States include:
ACTIVE
SUPERSEDED
RETIRED
UNDER_REVIEW
Used for:
“List active resolutions”
Import review
Supersession checks
Fail if:
Active resolutions are discovered via refs
Resolution lookup requires full scans
ENG-IDX-05 — Supersession Graph Index
The engine MUST maintain a directed graph:
Copy code

ResolutionHash → Set<ResolutionHash it supersedes>
ResolutionHash → Set<ResolutionHash that supersede it>
Properties:
Directional
Immutable edges
No cycles allowed (validated)
Used for:
Validating supersession
Import divergence analysis
Audit reasoning
Fail if:
Supersession order is inferred by timestamps alone
Cycles are permitted
ENG-IDX-06 — Area Membership Index
The engine MUST maintain:
Copy code

AreaId → Set<ResolutionHash>
Used for:
Area-scoped queries
Export
fsck validation
Fail if:
Area membership is inferred indirectly
Cross-area leakage occurs
ENG-IDX-07 — Session Outcome Index
The engine MUST maintain:
Copy code

SessionId → Set<Accepted ResolutionHash>
Used for:
Export validation
Legitimacy checks
Import filtering
Fail if:
Resolutions exist without a closed session
Session outcomes are inferred post hoc
5. Index Construction Rules
ENG-IDX-08 — Indexes Are Built from Objects, Not Refs
Indexes MUST be constructed primarily from object content.
Refs may be used:
To locate “current” context
To seed traversal
Refs MUST NOT be used:
To infer resolution state
To infer supersession
Fail if:
Index correctness depends on ref correctness
ENG-IDX-09 — Index Build Order Is Deterministic
Index rebuild order MUST be deterministic:
Load objects
Index object types
Build supersession graph
Index lifecycle states
Apply ref-derived context
Fail if:
Index order affects outcomes
Partial indexes are exposed
6. Mutation Rules
ENG-IDX-10 — Indexes Update on Engine Events
Indexes MUST update only in response to:
Object insertion
Ref mutation
Explicit lifecycle transitions
Indexes MUST NOT update:
Lazily
Implicitly
As side effects of reads
Fail if:
Reading data mutates indexes
7. fsck Interaction
ENG-IDX-11 — Indexes Are Verified, Not Trusted
fsck MUST:
Recompute indexes from scratch
Compare against live indexes
Report mismatches
fsck MUST NOT:
Repair indexes automatically
Modify objects or refs
Fail if:
fsck mutates index state
8. Import / Export Semantics
ENG-IDX-12 — Indexes Are Never Exported
Indexes:
MUST NOT appear in exports
MUST NOT be imported
MUST NOT affect import semantics
Import:
Loads objects
Rebuilds indexes locally
Fail if:
Imported data alters index rules
Indexes are serialized
9. Performance Guarantees
ENG-IDX-13 — Common Queries Are O(1) or O(log n)
The following MUST NOT require full scans:
List active resolutions
Get area authority
Get area scope
Validate supersession targets
Indexes exist to guarantee this.
Fail if:
Engine performance degrades linearly with history size
10. Design Guarantees
ENG-IDX-14 — Index Loss Is Non-Fatal
Deleting all indexes MUST be equivalent to:
Restarting the engine
Rebuilding from storage
Fail if:
Index loss causes irrecoverable state
Mental Model (Lock This In)
Objects are facts
Refs are current pointers
Indexes are scratchpads
Audit is the memory
Legitimacy never depends on caches
Why This Matters
This spec guarantees that:
10 resolutions or 10 million resolutions behave the same
Import review scales
fsck is meaningful
You never paint yourself into a performance corner
You can safely “start simple” without future rewrites
When you’re ready, the next logical document is:
ENG-AUD — Audit Event Model
That one will finally unify:
Area deletion
Imports
Ref changes
Hash upgrades
Just say the word when you want #2.