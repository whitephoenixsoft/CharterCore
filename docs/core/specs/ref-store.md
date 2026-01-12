Excellent — here is the formal, engine-grade Ref Store specification, written to match the tone, rigor, and style of your OS-* and ENG-* rules.
This locks in the mental model and gives you something future-you can rely on when the implementation feels “too simple”.
Charter Core — Ref Store Specification
Status: FROZEN (v1)
Applies To: Engine internals only
Audience: Charter Core implementers
Change Policy: Any change requires a new engine version and invariant review
1. Purpose & Scope
ENG-REF-00 — Ref Store Responsibility
The Ref Store provides mutable pointers to selected objects in the Object Store.
It exists solely to:
Identify current governing objects
Support ergonomic lookup by stable names
Enable atomic state transitions without rewriting history
The Ref Store is not:
A history mechanism
A query engine
A lifecycle tracker
A legitimacy evaluator
Fail if:
Refs are used to infer legitimacy
Refs encode historical sequences
2. Core Concepts
ENG-REF-01 — Ref Is a Named Pointer
A ref is a mapping:
Copy code

RefName → ObjectHash
Where:
RefName is a stable, human-readable identifier
ObjectHash is an immutable object identity
Refs:
Are mutable
Are replaceable
Have no intrinsic meaning beyond pointer semantics
Fail if:
A ref points to multiple objects
A ref encodes metadata beyond a pointer
ENG-REF-02 — Refs Are Explicitly Scoped
Every ref MUST belong to exactly one scope:
Global scope
Area scope
Ref names are resolved only within their scope.
Fail if:
A ref is implicitly global
Cross-scope visibility exists without explicit reference
3. Required Refs (Minimum Set)
ENG-REF-03 — Mandatory Area Refs
For each Area, the engine MUST maintain exactly:
area/<area_id>/authority → active Authority resolution
area/<area_id>/scope → active Scope resolution
Rules:
These refs MUST exist once the Area is initialized
These refs MUST be updated only via accepted decisions
These refs MUST always point to ACTIVE resolutions
Fail if:
Authority or Scope is inferred without a ref
Multiple active authority refs exist
ENG-REF-04 — No Ref Per Resolution
The engine MUST NOT create refs for individual resolutions.
Resolution lifecycle is determined by:
Resolution object state
Supersession graph
Acceptance rules
Fail if:
Active resolutions are tracked via refs
Ref count grows with resolution count
4. Ref Semantics
ENG-REF-05 — Refs Define “Current”, Not “Valid”
Refs indicate current governing context, not legitimacy.
Legitimacy is determined exclusively by:
Authority active at acceptance time
Constraints active at acceptance time
Session mechanics
Fail if:
Ref mutation retroactively affects legitimacy
Ref lookup is used during acceptance validation
ENG-REF-06 — Ref Updates Are Atomic
Ref updates MUST be:
Atomic
Auditable
Ordered
Replacing a ref:
Never mutates objects
Never deletes history
Emits an audit event
Fail if:
Partial ref updates are observable
Ref updates are silent
5. Ergonomic Refs (Optional)
ENG-REF-07 — Ergonomic Refs Are Non-Legitimizing
The engine MAY support optional ergonomic refs, such as:
area/<area_id>/head
session/current
User-defined labels
Rules:
Ergonomic refs MUST NOT affect engine logic
Ergonomic refs MUST NOT influence acceptance
Ergonomic refs MAY be deleted freely
Fail if:
Ergonomic refs affect legitimacy
Engine logic depends on their existence
6. Ref Resolution Rules
ENG-REF-08 — Ref Resolution Is Deterministic
Resolving a ref name MUST:
Be deterministic
Fail explicitly if missing
Never fall back silently
Resolution order:
Explicit scope
Explicit name
Fail
Fail if:
Resolution guesses intent
Ambiguous names resolve implicitly
7. Ref Persistence & Rebuild
ENG-REF-09 — Refs Are Rebuildable
The ref store MUST be reconstructible from:
Object store
Audit log
The ref store MUST NOT be the sole record of:
Authority history
Scope history
Supersession history
Fail if:
Loss of refs destroys historical meaning
ENG-REF-10 — Ref Store Is Mutable State
Unlike the object store, the ref store:
Is mutable
Is not append-only
Represents current state only
This distinction MUST be preserved.
Fail if:
Ref updates are stored as object mutations
8. Import / Export Semantics
ENG-REF-11 — Refs Are Local State
Refs:
MUST NOT be exported as authoritative history
MAY be recreated during import
MUST NOT override imported legitimacy
Import behavior:
RESTORE may recreate refs
CONSOLIDATE MUST rebuild refs locally
Fail if:
Imported refs override local authority
Imported refs affect acceptance mechanics
9. fsck Interaction
ENG-REF-12 — fsck Validates, Never Repairs
fsck MAY:
Verify that refs point to existing objects
Verify object type compatibility
Report dangling refs
fsck MUST NOT:
Rewrite refs
Guess intended targets
Auto-repair mismatches
Fail if:
fsck mutates ref state
10. Scalability Guarantees
ENG-REF-13 — Ref Count Is Bounded
The number of refs MUST be bounded by:
Number of Areas
Number of optional ergonomic refs
Ref count MUST NOT scale with:
Number of resolutions
Length of history
Import size
Fail if:
Ref store growth is unbounded
11. Design Guarantees (Non-Negotiable)
ENG-REF-14 — History Is Immutable, State Is Mutable
Charter Core enforces:
Immutable object history (Object Store)
Mutable current state (Ref Store)
This separation MUST be preserved across:
Import
Export
Upgrade
Consolidation
Recovery
Fail if:
History is rewritten to simulate state
State is inferred by mutating history
Mental Model (Lock This In)
Objects are facts.
Refs are pointers.
Supersession is time.
Indexes are caches.
Legitimacy is evaluated once.
What This Unlocks Next
With this spec in place, you are now safe to:
Implement the ref store trivially (HashMap)
Add indexing without fear
Scale to large histories
Implement fsck correctly
Implement import/review without ref explosions
Natural next RFCs (in order):
ENG-IDX — Index Structures
ENG-AUD — Audit Event Model
ENG-BOOT — Engine Boot & Rehydration Sequence
ENG-SESSION — Session State Machine (Formal)
Say which one you want next — you’re building this the right way.