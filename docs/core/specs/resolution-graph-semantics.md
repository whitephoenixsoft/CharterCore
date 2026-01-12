Perfect — this did surface new rules, even though they were implicit before. Below is a formal RFC-style specification you can drop next to your existing OS-* and ENG-* documents.
I’ll keep it engine-only, technical, and invariant-focused.
Charter Core — Resolution Graph, Supersession & Ref Semantics
Status: FROZEN (v1)
Scope: Engine internals only
Audience: Charter Core implementers
Change Policy: Changes require new engine version + invariants review
1. Resolution Graph Model
ENG-GRAPH-01 — Resolutions Form an Explicit Directed Graph
Resolutions form a directed graph via explicit supersession links.
Each Resolution MAY:
Supersede zero or more prior resolutions
Be superseded by at most one resolution per supersession chain
Rules:
Supersession links are explicit and immutable
No implicit ordering exists
No resolution is ever modified to “insert” history
Fail if:
Resolution order is inferred without explicit linkage
Supersession is derived from timestamps alone
ENG-GRAPH-02 — Supersession Is the Only Temporal Ordering Primitive
The engine MUST NOT infer “comes after” or “replaces” relationships from:
Timestamps
Import order
Storage order
Object hash ordering
Temporal or logical replacement is expressed only by:
Supersedes / superseded_by references
Fail if:
Engine derives ordering without graph traversal
2. Resolution Lifecycle Semantics
ENG-LIFE-01 — Lifecycle State Is Orthogonal to Graph Position
Resolution lifecycle state (ACTIVE, SUPERSEDED, RETIRED, UNDER_REVIEW) is:
Stored on the resolution object
Independent of supersession graph position
Changed only by explicit acceptance of a decision
Fail if:
Lifecycle state is inferred from graph topology
Graph traversal mutates lifecycle
ENG-LIFE-02 — Active Is a Derived Property, Not a Stored Pointer
A resolution is considered Active if and only if:
Its lifecycle state is ACTIVE
It has not been superseded by another accepted resolution
This status:
Is computed
Is not stored as a ref
Is not cached as authoritative state
Fail if:
Active resolutions are tracked via per-resolution refs
3. Ref Store Semantics (Critical)
ENG-REF-01 — Refs Represent Current Pointers, Not History
The ref store contains only mutable pointers to current governing objects.
Refs MUST:
Point to a single object hash
Be explicitly named
Be replaceable atomically
Refs MUST NOT:
Encode history
Scale with object count
Encode lifecycle state
Fail if:
Refs are created per resolution
Refs are used as a query mechanism
ENG-REF-02 — Refs Are Minimal and Bounded
For each Area, the engine MUST maintain at most:
One ref for active Authority
One ref for active Scope
Zero or more optional ergonomic refs (non-legitimizing)
The number of refs MUST NOT grow with:
Number of resolutions
Number of sessions
Length of history
Fail if:
Ref count grows unbounded over time
ENG-REF-03 — Refs Define “Current”, Not “Valid”
Refs define what is current, not what is legitimate.
Legitimacy is determined solely by:
Resolution acceptance rules
Authority at acceptance time
Constraints at acceptance time
Fail if:
Ref presence affects legitimacy evaluation
4. Indexing & Query Model
ENG-IDX-01 — Indexes Are Derived, Rebuildable Structures
Indexes MAY be built to support queries such as:
Active resolutions
Supersession chains
Resolution lookup by Area
Indexes:
Are derived from object store + refs
May be rebuilt at startup
Are not authoritative
Fail if:
Index corruption affects legitimacy
Index state is treated as primary truth
ENG-IDX-02 — Object Existence Does Not Imply Relevance
The mere existence of a resolution object MUST NOT:
Affect current authority
Affect active resolution sets
Affect session behavior
Only reachability and lifecycle rules apply.
Fail if:
Orphaned objects affect engine behavior
5. fsck & Validation Semantics
ENG-FSCK-01 — fsck Traverses Graphs, Not Refs Alone
fsck MUST validate:
Object integrity (hash correctness)
Supersession graph consistency
Referential integrity
fsck MAY report:
Orphaned resolutions
Dangling supersession links
Unreachable but intact objects
fsck MUST NOT:
Mutate refs
Delete objects
Repair history automatically
Fail if:
fsck changes engine state
6. Scalability Guarantees
ENG-SCALE-01 — Resolution Count Is Unbounded
The engine MUST support:
Thousands of resolutions
Arbitrarily deep supersession chains
Without:
Ref explosion
Degraded legitimacy evaluation
Refactoring history
Fail if:
Resolution count affects ref count
Resolution count affects correctness
7. Design Guarantees (Non-Negotiable)
ENG-GUAR-01 — History Is a Graph, State Is a Pointer
Charter Core enforces a strict separation between:
History: immutable resolution graph
State: minimal mutable refs
This separation MUST be preserved across:
Import
Export
Restore
Consolidation
Upgrade
Fail if:
State is reconstructed by mutating history
History is rewritten to simulate state
Summary (For Future You)
Supersession is your commit parent.
Refs are your branches.
Indexes are caches.
History never moves.
This RFC closes the loop between:
Resolution design
Ref store minimalism
Active resolution discovery
Long-term scalability
When you’re ready, the next natural RFC is:
“Charter Core — Ref Store Specification (ENG-REF-*)”
where we define:
Naming rules
Atomic updates
Audit hooks
Rebuild semantics
Just say the word.