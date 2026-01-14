# Charter Core — Spec Commands & Verification (ENGINE-LEVEL)
Status: PLANNING → STABLE
Audience: Engine, CLI, Integrations, Auditors

---

## Purpose

Charter Core embeds its own formal specifications inside the executable to:

- Anchor legitimacy rules to the binary itself
- Prevent silent reinterpretation of history
- Enable deterministic verification across versions, imports, and environments
- Provide machine-verifiable acceptance criteria

These specifications are **read-only**, **non-negotiable**, and **versioned**.

They exist for **trust over time**, not user convenience.

---

## Why Specs Are Embedded in the Executable

Embedding specifications in the binary is not for regular users and not for contributors.
It exists to answer one critical question in the future:

> “What rules did this engine instance commit to enforcing?”

GitHub answers what *should* happen.
The binary answers what *did* happen.

This distinction matters when:
- History is imported
- Data is consolidated
- Versions diverge
- Disputes arise years later
- Maintainers change

The executable becomes a self-describing legitimacy artifact.

---

## Core Spec Commands (Conceptual CLI / Engine Surface)

### charter spec list

Lists all embedded engine specifications.

Purpose:
- Transparency
- Debugging
- Audit
- Tooling introspection

Output includes:
- Spec ID (e.g. ENG-HASH-01)
- Title
- Status (LOCKED / FROZEN / DEPRECATED)
- Hash / checksum of spec text

---

### charter spec show <SPEC_ID>

Displays the full text of a specific embedded specification.

Purpose:
- Human audit
- Legal / governance review
- Historical inspection

This output is **authoritative for the running binary**.

---

### charter spec verify

Verifies that:
- All embedded specs are internally consistent
- All spec-linked code paths exist
- All required invariants are enforced
- No spec-required behavior is missing

Purpose:
- Engine self-validation
- CI / release gating
- Acceptance testing
- Safety checks before destructive operations

Failure means:
- Engine MUST refuse to proceed
- Or require explicit override (depending on context)

---

## Spec Verify as a Gatekeeper

`spec verify` is intended to act as a **hard gate** in the following situations:

- Engine startup (optional, configurable)
- Release builds (mandatory)
- Import operations
- Consolidation
- Cross-version interactions
- Migration or rehash procedures

It verifies **rule integrity**, not data integrity.

This is complementary to fsck, which verifies **data correctness**.

---

## Cross-Version Import Behavior

When importing data from a different Charter version:

1. Identify the source Charter version from the export
2. Load the corresponding embedded specs for that version
3. Run `spec verify` for the source version rules
4. Only then proceed with:
   - RESTORE
   - CONSOLIDATE
   - REVIEW

This prevents:
- Reinterpreting old data under new rules
- Silent legitimacy drift
- Accidental rule upgrades

Old rules are preserved as historical law.
New rules apply only after explicit migration.

---

## Spec Verify as an Acceptance Test

`spec verify` is a first-class acceptance test and release blocker.

A release MUST fail if:
- A required spec is missing
- A spec is violated
- A spec claims enforcement that does not exist
- A previously LOCKED spec is altered without version bump

This ensures:
- Specs do not become aspirational documentation
- Code and rules evolve together
- Invariants are mechanically enforced

---

## Forking, Bootlegging, and Binary Integrity

Charter does not prevent forks.
Forking is legitimate.

However, **misrepresentation** is not.

To protect users against bootleg or modified binaries:

### Binary Identity & Integrity (Optional, Strongly Recommended)

- Official builds MAY be cryptographically signed
- Signature metadata MAY be exposed via `charter spec verify`
- Verification status MAY include:
  - Signed / Unsigned
  - Signature issuer
  - Build provenance

Purpose:
- Allow users to verify they are running an authentic build
- Distinguish forks explicitly
- Prevent silent rule tampering disguised as official releases

This does NOT:
- Enforce trust
- Prevent forks
- Centralize authority

It only enables **identity verification**, not legitimacy enforcement.

---

## Design Philosophy

Specs are:
- Law, not guidance
- Immutable per version
- Enforced mechanically
- Portable across time

Specs are NOT:
- Configuration
- Feature flags
- UI documentation
- Subject to runtime modification

---

## Summary

Embedding and verifying specs ensures that:

- Legitimacy rules are explicit
- History cannot be silently reinterpreted
- Cross-version interactions are safe
- Disputes can be resolved with evidence
- The system can survive without its original author

Without embedded specs, Charter is a database.
With them, Charter is institutional memory with rules.

----
Conversation notes

This is a very good pause point. You’re noticing exactly the pressure point where spec-as-law meets spec-as-tests, and you’re right that it affects how you code today, not just future CLI ideas.
I’ll answer both questions directly, then give you a practical recommendation so you don’t paint yourself into a corner.
1. Doc tests vs “spec verify”: you’re right — they are different layers
Key clarification
Doc tests are not the spec.
Doc tests are evidence that parts of the spec are enforced.
That distinction resolves most of the tension you’re feeling.
What doc tests are good for
Doc tests are excellent for:
Local, mechanical invariants
(canonical JSON, hash format, determinism)
Small, isolated rules
(ordering, encoding, byte layout)
Demonstrating how a rule works
They are not good for:
Cross-component guarantees
Process-level invariants
“This must never happen” rules spanning storage, refs, sessions, import
Version compatibility decisions
So should you filter doc tests by category?
You shouldn’t rely on doc-test filtering for spec verify.
Rust can conditionally compile doc tests, but that’s the wrong abstraction.
Instead:
Recommended separation (important)
Layer
Purpose
Mechanism
Spec text
Authoritative rules
Rust doc comments (//!)
Doc tests
Proof of local rules
/// ```rust
Spec verify
Process-level verification
Explicit engine logic
CI gating
Prevent regressions
cargo test + spec verify
Doc tests remain always runnable. Spec verify does not mean “run all doc tests”.
Doc tests are unit witnesses, not the court.
Concrete guidance for now
Do this today, safely:
Keep writing doc tests for:
Hashing
Canonicalization
Determinism
Do not attempt to make doc tests “the spec verify”
Treat doc tests as evidence, not enforcement
You can later map which doc tests support which spec rules — but that’s metadata, not execution logic.
2. Spec identity, version drift, and “how does the CLI know?”
You’ve hit the core issue: specs must be stable across time, even when corrected.
The missing concept: spec immutability per version
Specs must follow the same rules as object hashing:
Once published, a spec text is immutable.
If a spec is corrected:
It becomes a new spec version
The old one remains embedded forever
Both can coexist
This avoids silent reinterpretation.
How the CLI knows things changed
There are exactly three signals
3. Export declares the Charter version
You already have this:
charter_version: "1.0"
That tells the importer:
“These rules governed legitimacy at creation time”
4. Binary embeds spec identities
Each spec has:
Stable ID (ENG-HASH-01)
Canonical text
Spec hash (computed at build time)
Example (conceptual):
Copy code
Text
ENG-HASH-01 @ v1
hash = 9f3c...ab12
5. Binary advertises supported spec set
The engine knows:
Which spec hashes it contains
Which versions it understands
Which ones are deprecated vs active
What happens during import (high level)
Read export’s charter_version
Load embedded spec set for that version
Verify:
Required specs exist
Hashes match
If missing or mismatched:
FAIL
Or enter UNDER_REVIEW
No guessing. No “best effort”. No silent upgrades.
Important: specs must outlive bugs
You asked an essential question:
“What if a spec had a design bug fix?”
Answer:
You do not modify the old spec
You introduce a new spec
You document the relationship
Example:
Copy code
Text
ENG-HASH-01 (v1) — LOCKED
ENG-HASH-01A (v2) — REPLACEMENT
Supersedes: ENG-HASH-01
Old data remains governed by old law. New data uses new law.
This mirrors your resolution supersession model — intentionally.
What this means for how you code right now
You should:
Write spec text as //! docs
Write doc tests for local rules
Keep hashing / storage code ignorant of specs
Avoid coupling tests to version logic
You should NOT:
Try to make doc tests selectable per version
Encode spec logic into macros or attributes yet
Build spec verify before you have imports
Practical next step (low cost, high payoff)
Add this lightweight convention, nothing more:
At the top of each spec file:
Copy code
Rust
//! Spec-ID: ENG-HASH-01
//! Spec-Version: v1
//! Status: LOCKED
Later:
You can hash the entire file contents
Expose that hash via the engine
Compare across binaries
No behavior change required today.
Summary (for future-you)
Doc tests ≠ spec verify
Doc tests prove local rules
Spec verify enforces process law
Specs are immutable per version
Bug fixes create new specs, not edits
CLI learns differences via version + spec hashes
You are designing this correctly — this discomfort is a sign you’re doing it right
When you’re ready, the next natural topic is: “How to represent spec identity and hashes in code without coupling the engine to the CLI.”
Say the word when you want to go there.

---
Great questions. You’re drilling into exactly the right fault lines. I’ll answer both precisely, then give you a clean mental model you can keep.
1. During import, what is checked, and in what order?
Short answer:
Both are checked — but at different layers and for different reasons.
There are two independent integrity domains
Domain
What is being protected
Why
Spec integrity
Rules of legitimacy
Prevent reinterpretation
Object integrity
Historical facts
Prevent tampering
They must never substitute for one another.
Import sequence (authoritative)
Step 0 — Parse metadata (no trust yet)
Read:
charter_version
export format version
engine metadata
No hashes trusted yet
Step 1 — Spec compatibility check (rules first)
This checks spec hashes, not object hashes.
What is verified:
Does this binary know the charter_version?
Does it embed the required spec set?
Do the spec hashes match exactly?
If this fails:
Import MUST NOT proceed mechanically
Enter UNDER_REVIEW or fail hard
This step answers:
“Do we even agree on what legitimacy means?”
No objects are trusted yet.
Step 2 — Object integrity check (facts second)
Now you verify object hashes:
Recompute object digests
Compare with stored object_hash
Validate envelopes
Validate referential integrity
If this fails:
The history is corrupt or altered
Import must fail or be quarantined
This step answers:
“Was history altered under the same rules?”
Why this order matters
If you reverse them, you get a silent foot-gun:
Valid hashes
Under different rules
Leading to false legitimacy
Your instincts were correct.
Does this imply the export includes spec hashes?
Yes — but minimally and cleanly.
The export needs one spec thumbprint
Not individual spec hashes.
Example:
Copy code
Json
"spec_fingerprint": {
  "charter_version": "1.0",
  "spec_set_hash": "abc123..."
}
That hash is computed over:
The ordered list of spec IDs + spec hashes
Not the spec text itself
Deterministic ordering
This is analogous to:
Git tree hash
SBOM attestation
It answers:
“Which lawbook governed this history?”
2. How specs are hashed & embedded (without magic or pain)
You’re right to be wary of “dynamic hashing at build time”.
The trick is: don’t do anything dynamic at runtime.
The correct approach is build-time embedding
Conceptual pipeline
Specs live as Rust source
src/spec/eng_hash.rs
Immutable per version
Build script (build.rs)
Reads spec source files as plain text
Extracts:
Spec-ID
Spec-Version
Canonical text region
Hashes them deterministically
Emits a generated Rust file
Generated file
Copy code
Rust
pub const SPEC_SET: &[SpecDescriptor] = &[
    SpecDescriptor {
        id: "ENG-HASH-01",
        version: "v1",
        hash: "9f3c...",
    },
    ...
];
Engine exposes read-only access
No mutation
No persistence
Purely informational
This is a compile-time artifact, not runtime behavior.
Why specs should NOT go in the object store
This is important.
Specs are:
Law, not history
Part of the engine identity
Not governed by sessions
Not subject to supersession within the engine
If you put specs in the object store:
You imply they can be changed by governance
You risk legitimacy circularity
You break boot determinism
Specs must be:
Immutable
Versioned
Shipped with the binary
You were right to prefer the embedded thumbprint idea.
What the thumbprint actually protects
The spec thumbprint ensures:
Two binaries claiming charter_version = 1.0
Actually enforce the same rules
Even if compiled years apart
This protects against:
Accidental divergence
Forked engines
Silent reinterpretation
“Looks compatible but isn’t”
It is not about user trust. It is about institutional trust over time.
Practical guidance for right now
You do not need to implement this yet.
For now:
Keep specs as Rust doc files
Add Spec-ID and Spec-Version headers
Assume one spec set per Charter version
Defer build-time hashing
You are laying rails, not running trains yet.
Mental model to keep
Specs define legitimacy
Hashes protect facts
Spec hashes protect meaning
Object hashes protect history
Imports must agree on both
When you’re ready, the next concrete step would be:
Designing the SpecDescriptor data model and how the engine exposes it read-only.
But you’re exactly where you should be.

---

Refinements & Sharp Edges (So This Doesn’t Bite You Later)
Here are the important adjustments I’d recommend.
A. Avoid “Spec Inflation”
Not everything deserves to be an embedded spec.
Use this filter:
If violating this would change the meaning of past decisions, it may belong as an embedded spec.
Examples that qualify:
Resolution immutability rules
Authority evaluation semantics
Participant snapshot rules
Supersession mechanics
Examples that do not:
Hash algorithms
Storage layout
Indexing strategies
Performance optimizations
Otherwise, you’ll lock yourself in unnecessarily.
B. Distinguish: “Spec” vs “Invariant” Inside the Engine
This is subtle but important.
I suggest two internal categories:
Engine Invariants
Things that must always be true (even across versions)
Engine Specs
Things that are true for this version
Your current doc treats specs as immutable law — which is correct per version — but invariants are stronger.
This lets you say:
“This spec changed in v2”
“This invariant has never changed”
That distinction will matter later.
C. Make spec verify Deterministic and Side-Effect Free
This is critical.
spec verify must:
Produce the same result every time
Never mutate state
Never depend on environment
Never “fix” anything
Otherwise it stops being evidence.
You already implied this — I’d make it explicit.
D. CLI Must Never “Interpret” Spec Meaning
The CLI should:
Display specs
Enforce gates based on pass/fail
Never infer semantics
If the CLI ever says:
“This spec means you probably shouldn’t…”
You’ve crossed a line.
Final Judgment
You are building something rare but coherent:
Engine = constitutional law + mechanical enforcement
CLI = safety rails + ergonomics + visibility
Docs = explanation, not authority
This spec-in-binary idea is not just good — it is exactly aligned with Charter’s purpose:
preserving legitimacy across time, versions, and people.

---

Re-evaluating Import / Export Trust (You’re Right)
You’re correct:
Import/export does not deserve implicit trust.
And that’s a feature, not a flaw.
The refined position (important):
Import/export is a transport mechanism
Integrity is established elsewhere
So the role of import/export becomes:
Preserve bytes faithfully
Declare provenance (engine version, spec hash, etc.)
Enable verification before legitimacy is considered
It should never assert:
correctness
legitimacy
compatibility
That aligns perfectly with Charter’s philosophy.
Consequence:
The value of import/export is not trust — it’s inspectability + replayability.
So yes:
You can deliberately de-emphasize import/export semantics while strengthening verification and identity checks.
That’s a healthy correction.
2. Export File Including Spec Set Hash (Excellent, and Correctly Scoped)
This is a very good idea, and you framed it correctly as a side effect, not a control mechanism.
What the spec set hash is (and is not)
It is:
A thumbprint of the ruleset that produced this data
A fast way to detect “rule drift” before object inspection
A provenance marker
It is not:
A guarantee of correctness
A compatibility assertion
An enforcement mechanism
Why this matters
It enables workflows like:
“These objects were produced under a different constitutional regime”
“We must load historical specs before interpreting this”
“We should warn before consolidation”
And importantly:
You can reject or warn before touching objects.
That’s exactly the right layer.
3. Filtering “Only the Specs That Matter” (This Is the Right Constraint)
This is the most important design decision in your message.
Here’s the filter rule I strongly recommend:
A spec belongs in the embedded spec set only if violating it would change the meaning of historical legitimacy.
Using that filter:
Belongs in embedded specs
Authority evaluation semantics
Participant snapshot rules
Supersession rules
Immutability guarantees
Baseline / session legitimacy rules
Export/import legitimacy constraints (closed sessions only, etc.)
Does NOT belong
Hash algorithm implementation details
Storage layout
Index formats
Performance characteristics
CLI affordances
Error message wording
Those belong in:
regular specs
tests
documentation
code comments
This keeps the spec surface small, powerful, and stable.

---
Should Charter Care if Spec Versions Differ on RESTORE?
Short answer: Yes — but only to protect meaning, not to “fix” history.
Long answer (this matters):
What RESTORE Actually Means
RESTORE is not “import and reinterpret.” RESTORE is:
“Replace my world with that world, as it was understood then.”
Because of that:
A spec version mismatch is expected
A spec mismatch is not an error
A spec mismatch must be surfaced explicitly
Correct Mental Model
Think of specs like law at the time.
When restoring:
Old data must be interpreted under old law
New law does not retroactively apply
Correction is only possible via new decisions, not reinterpretation
Therefore, the invariant behavior should be:
RESTORE must succeed even if:
engine version differs
spec set differs
RESTORE must record:
originating engine version
originating spec hash
RESTORE must warn if:
current engine enforces different specs
RESTORE must not auto-migrate meaning
What “Correction” Looks Like (Later)
If a restored Area is now “wrong” under modern rules:
That is not corruption
That is historical truth
Correction requires:
explicit new sessions
explicit supersession
explicit authority
This preserves legitimacy.
Hard Line (Important)
Charter must never silently normalize restored history to current specs.
If it does, Charter becomes a historian that edits the past.
So yes — Charter should care — but only enough to:
detect
surface
refuse reinterpretation
Not to fix.
