# Charter — Versioning & Identity Model

Status: FOUNDATIONAL  
Applies to: All Charter Substrates (Engine, Runtime, CCS, CSG, CIS, CAS, CCare, CDS, CGL, CRS, CQL)  
Depends On: Determinism, Identity Model, Non-Interpretation Principle  
Does NOT define: legitimacy semantics, workflow execution, alignment computation, or guidance behavior  

---

# 1. Purpose

This document defines the **Versioning & Identity Model** for Charter.

It exists to:

- establish clear identity boundaries across all layers of the system  
- prevent ambiguity between behavior, structure, and distribution  
- ensure that changes are explicit, detectable, and auditable  
- support forkability without impersonation  
- preserve historical meaning across system evolution  

Charter separates **what a system does**, **how data is represented**, and **how software is distributed**.

---

# 2. Core Principle

> Every layer of meaningful change must have its own explicit identity.

These identities must be:

- deterministic  
- immutable within scope  
- non-overlapping in meaning  
- independently versioned  

No identity domain may be used as a substitute for another.

---

# 3. Identity Domains

Charter defines multiple independent identity domains.

Each domain answers a different question.

---

## 3.1 Rule Identity (Behavior)

**Question answered:**  
“What rules does this system enforce?”

Rule Identity represents:

- behavioral logic  
- evaluation semantics  
- interpretation rules (where applicable)  

Examples:

- engine legitimacy rules  
- runtime orchestration rules  
- guidance heuristics (CGL)  
- alignment detection behavior (CAS)  

### Properties

- deterministic fingerprint (e.g., `spec_set_hash`)  
- immutable per build  
- independent of configuration  
- must change when behavior changes  

### Constraints

- identical Rule Identity implies identical behavior  
- different Rule Identity implies potential behavioral divergence  

---

## 3.2 Schema / Format Version (Representation)

**Question answered:**  
“How is this data structured and encoded?”

Schema / Format Version represents:

- file formats  
- serialization structures  
- encoding rules  
- field-level structure  

Examples:

- export file schemas  
- commit artifact formats  
- bundle structures  
- JSON or binary encodings  

### Properties

- versioned independently of behavior  
- governs parsing and validation  
- may evolve without changing semantics  

### Constraints

- same schema does not imply same meaning  
- different schema does not imply different meaning  

---

## 3.3 Protocol Version (Transport / Interaction)

**Question answered:**  
“How do systems communicate or exchange data?”

Protocol Version represents:

- communication rules  
- API contracts  
- relay interaction formats  
- streaming or request/response structures  

Examples:

- CRS relay protocol  
- federation exchange formats  
- API contracts between systems  

### Properties

- defines interaction compatibility  
- independent of rule identity and schema  
- may evolve separately  

---

## 3.4 Release Version (Distribution)

**Question answered:**  
“What build or package is this?”

Release Version represents:

- packaged software version  
- crate/library version  
- binary release identifier  

Examples:

- semantic version (e.g., `v1.4.2`)  
- git tag  
- build number  

### Properties

- human-readable  
- used for deployment and dependency management  
- may bundle multiple rule and schema changes  

### Constraints

- same release does not guarantee identical rule identity  
- different release does not guarantee behavioral difference  

---

## 3.5 Hash / Canonicalization Version

**Question answered:**  
“How are identities and hashes computed?”

Represents:

- hashing algorithms  
- normalization rules  
- canonicalization strategies  

Examples:

- object hashing rules  
- canonical JSON normalization  
- identity derivation methods  

### Properties

- affects identity stability  
- must be explicit when changed  

---

## 3.6 ABI / FFI Version (Forward-Looking)

**Question answered:**  
“How do binaries interoperate across language boundaries?”

Represents:

- function signatures  
- memory layout  
- calling conventions  

Examples:

- Rust → C ABI  
- shared library interfaces  

### Properties

- critical for multi-language integration  
- independent of rule identity  

---

# 4. Separation Rules

Identity domains must remain strictly separated.

---

## 4.1 No Cross-Domain Substitution

The system must not assume:

- same release ⇒ same rules  
- same schema ⇒ same semantics  
- same protocol ⇒ same behavior  

---

## 4.2 No Implicit Coupling

Changes in one domain must not implicitly:

- modify another domain  
- be assumed to imply changes elsewhere  

---

## 4.3 Explicit Declaration Required

All identity domains must be:

- explicitly declared  
- externally observable  
- queryable (where applicable)  

---

# 5. Artifact Binding

Artifacts may bind to one or more identity domains.

---

## 5.1 Required Binding (Behavior)

Artifacts that encode meaning must bind to:

- Rule Identity  

Examples:

- engine receipts  
- resolution artifacts  

---

## 5.2 Optional Binding

Artifacts may also include:

- schema version  
- protocol version  
- origin system identity  

---

## 5.3 Binding Constraints

- identity must not be inferred  
- identity must not be reconstructed implicitly  
- identity must be preserved across transport  

---

# 6. Verification Model

Verification operates only on **Rule Identity**.

---

## 6.1 Verification Scope

Verification compares:

- artifact Rule Identity  
- system Rule Identity  

---

## 6.2 Verification Outcomes

Examples:

- MATCH  
- LEGACY_MATCH  
- UNKNOWN  

These outcomes describe **compatibility**, not correctness.

---

## 6.3 Non-Behavioral Domains

Schema, protocol, and release versions:

- may be validated  
- must not be used to determine behavioral equivalence  

---

# 7. Evolution Rules

---

## 7.1 Rule Changes

If behavior changes:

- Rule Identity MUST change  

---

## 7.2 Schema Changes

If representation changes:

- schema version MUST change  

---

## 7.3 Protocol Changes

If communication rules change:

- protocol version MUST change  

---

## 7.4 Release Changes

Release version may change for:

- any reason  
- but must not be used as a proxy for other domains  

---

# 8. Forking and Divergence

Charter explicitly allows forks.

---

## 8.1 Fork Transparency

Forked systems must:

- produce a different Rule Identity if behavior differs  
- expose identity clearly  

---

## 8.2 No Impersonation

Systems must not:

- claim identical Rule Identity with different behavior  
- obscure divergence  

---

## 8.3 Compatibility Handling

Consumers must:

- rely on Rule Identity for compatibility decisions  
- not assume compatibility from naming or versioning  

---

# 9. Relationship to Non-Interpretation

This model enforces Non-Interpretation by ensuring:

- behavior is explicitly identified  
- meaning is not inferred from representation  
- compatibility is not assumed  

---

# 10. Invariants

- every identity domain must be explicit  
- identity domains must not be conflated  
- Rule Identity must change when behavior changes  
- schema version must change when structure changes  
- protocol version must change when interaction changes  
- release version must not imply behavior  
- verification must rely only on Rule Identity  
- identity must be immutable within scope  

Violation of these invariants introduces ambiguity and system inconsistency.

---

# 11. Mental Model

Different identities answer different questions:

- Rule Identity → “What does this system do?”  
- Schema Version → “How is this data structured?”  
- Protocol Version → “How do systems communicate?”  
- Release Version → “What build is this?”  

These must never be confused.

---

# 12. Final Principle

Charter systems must make all change visible.

They do this by ensuring that:

- behavior is identifiable  
- structure is versioned  
- communication is explicit  
- distribution is traceable  

So that:

- systems may evolve freely  
- forks may diverge safely  
- and history remains interpretable without ambiguity