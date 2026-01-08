Absolutely. This one is critical, and it sits cleanly at the CLI ↔ engine boundary without violating storage or legitimacy invariants.
Below is ENG-BOOT, written in the same locked, RFC-style form as the others.
Charter Core — Engine Boot & Rehydration Specification
Document ID: ENG-BOOT
Status: FROZEN (v1)
Audience: Charter Core engine & CLI implementers
Scope: Engine initialization, rehydration, and readiness guarantees
1. Purpose
ENG-BOOT-01 — Deterministic Engine Initialization
ENG-BOOT defines how a Charter Core engine instance:
Starts from a storage root
Rehydrates its internal state
Verifies integrity
Becomes ready to evaluate legitimacy
Boot MUST be:
Deterministic
Read-only with respect to history
Free of implicit migration or mutation
Fail if:
Engine behavior depends on boot timing
Engine modifies state during boot without explicit user intent
2. Engine Boundary
ENG-BOOT-02 — Boot Is a Host-Provided Action
The engine:
Does NOT discover storage roots
Does NOT infer context
Does NOT read the filesystem implicitly
The host (CLI, server, test harness) MUST provide:
An explicit storage root
Boot mode flags (e.g. read-only, verify-only)
Fail if:
Engine assumes current working directory
Engine performs path resolution internally
3. Boot Phases (Ordered & Mandatory)
ENG-BOOT-03 — Phase 0: Storage Attachment
The engine MUST:
Attach to the provided storage root
Verify storage accessibility
Fail fast on I/O errors
No semantic interpretation occurs at this phase.
Fail if:
Engine silently continues on partial access
ENG-BOOT-04 — Phase 1: Object Store Load
The engine MUST:
Discover all stored object envelopes
Validate envelope structure
Index objects by object_hash
At this stage:
No refs are resolved
No legitimacy is evaluated
Fail if:
Duplicate hashes map to different content
Envelope metadata mismatches digest header
ENG-BOOT-05 — Phase 2: Object Integrity Verification
The engine MUST:
Recompute the hash for every object
Verify:
hash algorithm
hash version
object type
Detect:
Corruption
Truncation
Mismatch
Failure handling:
Boot MUST fail or
Engine enters explicit DEGRADED / READ-ONLY mode
Fail if:
Engine silently accepts corrupted objects
ENG-BOOT-06 — Phase 3: Ref Store Load
The engine MUST:
Load all refs
Validate ref syntax
Ensure each ref points to an existing object hash
No authority or scope evaluation occurs yet.
Fail if:
A ref points to a missing object
Required refs are malformed
ENG-BOOT-07 — Phase 4: Ref Consistency Validation
The engine MUST validate:
Exactly one active Authority ref per Area
Exactly one active Scope ref per Area
Ref target types match ref semantics
Fail if:
Multiple authority refs exist for an Area
Authority or Scope ref is missing
Ref points to wrong object type
ENG-BOOT-08 — Phase 5: Liveness Resolution
The engine MUST compute:
Live object set (reachable from refs)
Inert object set (unreferenced)
Rules:
Inert objects MUST NOT affect engine behavior
Inert objects MUST remain addressable for audit
Fail if:
Engine behavior depends on inert objects
ENG-BOOT-09 — Phase 6: Session State Rehydration
The engine MUST:
Identify all session state refs
Rehydrate session state machines
Enforce invariants:
Blocked sessions remain blocked
Paused sessions remain paused
Closed sessions remain closed
Fail if:
Session state changes during boot
Incomplete sessions are auto-closed
ENG-BOOT-10 — Phase 7: Deterministic Readiness Check
The engine MUST verify:
All Areas are either:
Initialized (Authority + Scope present), or
Explicitly uninitialized and blocked
No session violates current Authority or Scope invariants
If violations exist:
Engine enters BLOCKED state
Explicit operator action is required
Fail if:
Engine proceeds with illegitimate state
4. Boot Modes
ENG-BOOT-11 — Supported Boot Modes
The engine MUST support:
NORMAL
Full verification
Read/write refs
READ-ONLY
Full verification
No ref or object mutation
VERIFY-ONLY
Integrity checks only
No rehydration
RECOVERY
Load as much as possible
Report all failures
No legitimacy evaluation
Fail if:
Mode semantics are conflated
5. Migration & Upgrade Constraints
ENG-BOOT-12 — No Implicit Migration
During boot:
Hash upgrades MUST NOT occur
Ref rewrites MUST NOT occur
Object rewrites MUST NOT occur
All migration requires:
Explicit operator command
Explicit audit records
Explicit rebind of refs
Fail if:
Engine mutates state during boot
6. Import Interaction
ENG-BOOT-13 — Import Is Not Boot
Import operations:
MUST occur after successful boot
MUST operate through engine APIs
MUST emit audit events
Fail if:
Import logic is executed during boot
7. Determinism Guarantee
ENG-BOOT-14 — Boot Is Deterministic
Given:
Identical object store
Identical ref store
Identical engine version
Boot MUST result in:
Identical live object sets
Identical active authorities
Identical session states
Fail if:
Non-deterministic ordering affects behavior
8. Audit Requirements
ENG-BOOT-15 — Boot Is Auditable but Non-Legitimizing
Boot MAY emit:
Diagnostic audit events
Integrity warnings
Boot MUST NOT emit:
Legitimacy events
Resolution acceptances
Session transitions
Fail if:
Boot alters legitimacy state
9. CLI Responsibility (Non-Engine)
ENG-BOOT-16 — CLI Owns Context Resolution
The CLI is responsible for:
Resolving user context
Selecting storage root
Passing storage root to engine
The engine:
Treats storage root as opaque
Does not persist context pointers
Fail if:
Engine manages contexts
10. Failure Semantics
ENG-BOOT-17 — Fail Loud, Fail Early
Boot failures MUST be:
Explicit
Descriptive
Non-recovering without operator intent
Fail if:
Engine attempts silent recovery
Engine masks corruption
11. Summary Guarantees
ENG-BOOT guarantees:
No silent mutation
Deterministic startup
Clear failure modes
Clean CLI ↔ engine boundary
Safe foundation for legitimacy evaluation
Mental Model (Lock This In)
Boot loads facts
Refs define relevance
Sessions define legitimacy
Nothing changes unless explicitly commanded
This completes the core engine spine:
ENG-OBJ / ENG-HASH — What exists
ENG-REF — What matters now
ENG-AUD — What happened
ENG-BOOT — How it comes alive
When you’re ready, the next natural document (if you want it) is:
ENG-IMPORT — Import, Consolidation & Review Pipeline
Or we can drop back down into Rust implementation patterns now that the foundation is locked.