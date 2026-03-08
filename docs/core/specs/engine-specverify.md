# ENG-SPECVERIFY — Specification Identity & Verification
Status: FROZEN (v1 – Engine Rule Identity & Deterministic Specification Verification)
Applies to: Engine Core (V1/V2+)
Scope: Engine rule identity, specification manifest embedding, spec-set hashing, verification of exported artifacts, and long-term rule provenance

Authority: Foundational to all Engine behavior specifications

Subordinate to: None

Referenced by:
ENG-DOMAIN
ENG-SESSION
ENG-DECISION
ENG-RECEIPT
ENG-INTEGRITY
ENG-API

---

# 1. Purpose

ENG-SPECVERIFY defines how an Engine binary identifies and proves the exact rule set it enforces.

The purpose of specification verification is to:

- Bind an Engine binary to a deterministic set of specifications
- Prevent silent reinterpretation of historical legitimacy events
- Allow deterministic cross-version verification
- Support long-term institutional auditability
- Enable transparent fork differentiation
- Provide rule provenance for exported artifacts

Specification verification does not create legitimacy.

Legitimacy is created only through:

- Sessions
- Authority evaluation
- Deterministic acceptance
- Immutable receipts

Specification verification provides evidence of the rules under which legitimacy was evaluated.

---

# 2. Core Principle

An Engine instance must be able to answer the following deterministically:

“What exact rule set does this binary enforce?”

The answer must be:

- Machine-verifiable
- Immutable for the lifetime of the binary
- Independent of external repositories
- Deterministic across identical builds
- Explicitly represented in exported artifacts

The Engine must be self-describing with respect to its rule system.

---

# 3. Specification Manifest

Every Engine binary must embed a read-only Specification Manifest.

The manifest defines the full specification set enforced by the binary.

The manifest must include:

engine_version  
spec_manifest  
spec_set_hash

Where:

spec_manifest is a deterministic mapping of specification identifiers to their canonical hashes.

Example conceptual structure:

spec_manifest:
- ENG-DOMAIN
- ENG-SESSION
- ENG-DECISION
- ENG-RECEIPT
- ENG-INTEGRITY
- ENG-ERROR
- ENG-API
- ENG-SPECVERIFY

Each entry must include:

spec_name  
spec_hash

Specification hashes must be deterministic cryptographic hashes of the canonical specification content.

The manifest must be:

- Embedded in the binary
- Immutable at runtime
- Deterministic across identical builds
- Independent of network access
- Sufficient to uniquely identify the rule set

---

# 4. Specification Hash Requirements

Each specification hash must be computed using the Engine's declared hash_algorithm.

Requirements:

- Hash computed over canonical specification text
- Canonical UTF-8 encoding
- Deterministic whitespace normalization
- No environment-dependent transformation

If a specification changes in any way that affects rule semantics, the specification hash must change.

Specification hashes must never be mutated retroactively.

---

# 5. Specification Set Hash

The Engine must compute an aggregate hash representing the full rule set.

This value is called:

spec_set_hash

The spec_set_hash must be computed deterministically from the spec_manifest.

Algorithm:

1. Sort spec_manifest entries lexicographically by spec_name.
2. For each entry produce a canonical string:

   spec_name + ":" + spec_hash

3. Concatenate the ordered strings with newline separators.
4. Hash the resulting string using the declared hash_algorithm.

The resulting value is spec_set_hash.

Properties:

- Deterministic
- Stable across identical implementations
- Unique to the rule set
- Independent of build environment

spec_set_hash is the canonical identifier of the Engine's rule system.

---

# 6. Manifest Immutability

The specification manifest must be immutable.

The Engine must not allow:

- Runtime modification of the manifest
- Dynamic specification loading
- Specification replacement
- Hidden rule extensions

Any binary that claims to enforce specifications not present in its manifest is inconsistent.

Such behavior constitutes an Engine defect.

---

# 7. Export Artifact Requirements

All exported artifacts must explicitly record rule provenance.

Exported artifacts include:

- Receipts
- DAG exports
- Snapshots
- External verification packages

Each exported artifact must include:

engine_version  
spec_set_hash  
export_schema_version  
hash_algorithm

These fields must be explicit.

No consumer may infer rule identity implicitly.

This ensures that any artifact can later answer:

- Which Engine version produced it
- Which exact rule set enforced it

---

# 8. Receipt Integration

Receipts must include:

engine_version  
spec_set_hash

These fields must participate in canonical serialization and hashing.

Therefore:

content_hash = hash(canonical_receipt_content)

Where canonical_receipt_content includes spec_set_hash.

This guarantees that the receipt hash binds:

- session outcome
- rule set identity

Receipt integrity therefore proves both:

- what happened
- under which rules it happened

---

# 9. Resolution Provenance (Recommended Integration)

Resolutions should also record:

engine_version  
spec_set_hash

This ensures that governance artifacts remain traceable even if receipts are separated from exported graphs.

Resolutions remain immutable once created.

---

# 10. Verification Semantics

When an Engine loads a receipt or exported artifact it must perform specification verification.

Verification compares:

artifact.spec_set_hash  
engine.spec_set_hash

Possible outcomes:

MATCH

The Engine enforces the same specification set that created the artifact.

Full verification may proceed.

LEGACY_MATCH

The artifact references a spec_set_hash recognized by the Engine as a supported historical specification set.

Verification may proceed in legacy verification mode.

SPEC_SET_UNKNOWN

The artifact references a specification set unknown to the Engine.

The Engine may:

- store the artifact
- export the artifact
- report provenance

The Engine must not:

- reinterpret the artifact under different rules
- silently assume compatibility
- recompute legitimacy

---

# 11. Legacy Verification Support

An Engine may optionally support verification of historical rule sets.

If implemented:

supported_spec_sets must be defined as a deterministic mapping:

spec_set_hash → manifest definition

Legacy verification allows:

- historical receipt validation
- audit reconstruction
- cross-version legitimacy verification

Legacy support must never mutate historical artifacts.

---

# 12. Relationship to Legitimacy

Specification verification does not determine legitimacy.

Legitimacy is determined exclusively by:

- Session mechanics
- Authority evaluation
- Deterministic acceptance
- Receipt emission

Specification verification provides rule provenance.

In institutional analysis, legitimacy evaluation may consider:

- audit history
- receipt artifacts
- specification identity
- engine version

Together these reconstruct:

what occurred  
under which rules it occurred

---

# 13. Fork Transparency

Forking is legitimate and expected.

Specification verification does not prohibit:

- alternative implementations
- specification modification
- rule divergence

However:

A modified binary must not represent itself as enforcing a specification set that differs from its manifest.

If specifications change, spec_hash values must change.

This produces a different spec_set_hash.

This mechanism enables transparent fork detection.

---

# 14. Binary Authenticity Considerations

Specification identity verifies the rule set enforced by a binary.

It does not verify the authenticity or integrity of the binary itself.

Operators are expected to verify the integrity of distributed Engine binaries using checksums or cryptographic signatures provided through the software distribution channel.

Binary packaging, signing, and supply-chain verification are intentionally outside the scope of the Engine specifications.

This separation preserves the distinction between:

Rule Identity — defined by the embedded specification manifest  
Binary Authenticity — defined by external distribution and packaging processes

Users may combine binary checksum verification with specification identity to create a complete trust chain for operational environments.

---

# 15. Non-Goals

Specification verification does not:

- enforce trust
- prohibit forks
- guarantee binary authenticity
- replace governance
- modify legitimacy rules
- alter session mechanics

Optional layers such as signatures or build attestations may provide authenticity guarantees.

Specification verification concerns only rule identity.

---

# 16. Stability Guarantees

The specification verification mechanism must remain:

- deterministic
- backward-auditable
- immutable for each binary
- independent of runtime configuration

Future versions may:

- add specifications
- deprecate specifications
- expand manifest detail

Future versions must not:

- allow mutable rule identity
- allow runtime rule substitution
- permit undocumented rule enforcement

---

# 17. Long-Term Institutional Durability

Receipts and exported artifacts include spec_set_hash and engine_version.

These fields allow artifacts to remain interpretable independently of the original Engine implementation or repository.

If the specifications referenced by spec_set_hash remain available, future systems can reconstruct:

- which rules governed the decision
- how the session was evaluated
- which governance structures were applied

This property enables long-term institutional auditability and prevents silent reinterpretation of historical decisions.

---

# 18. Engine Invariants

Every Engine binary must:

Embed a deterministic specification manifest.

Compute spec_set_hash deterministically.

Expose rule identity through the Engine API.

Record spec_set_hash in exported artifacts.

Record spec_set_hash in receipts.

Reject silent rule reinterpretation.

Remain deterministic across implementations.

Violation of these invariants constitutes an Engine integrity defect.

---

# 19. Possible Future Extensions

Future versions of the Engine specifications may define mechanisms that allow receipts or exported artifacts to be evaluated independently of the Engine runtime.

Such mechanisms could allow artifacts to act as self-contained verification objects capable of re-evaluating legitimacy decisions using the referenced specification set.

This document does not define such mechanisms but preserves compatibility for future expansion.

---

# Mental Model

Audit answers:

“What happened?”

Receipts answer:

“What decision was finalized?”

Specification verification answers:

“Under which rules did it happen?”

Binary verification answers:

“Is this the binary I intended to run?”

Together these layers provide long-term institutional integrity.

Nothing in this mechanism grants authority.

It only reveals the rule system under which authority operated.