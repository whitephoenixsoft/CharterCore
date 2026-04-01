# Charter Core — Spec Versioning & Legacy Ingestion Policy

**Status:** FROZEN (Foundation)  
**Applies to:** Engine, CLI, Libraries, Import/Export, Audit  
**Audience:** Maintainers, Integrators, Auditors, CLI implementers  

---

## Purpose

This document defines how Charter manages versions of:

- Embedded specifications (per library)  
- Engine versions  
- CLI versions  
- Exported files  

It also specifies how legacy data, old commits, and historical specifications are ingested and interpreted.  

Goals:

1. Preserve historical meaning and legitimacy.  
2. Prevent silent reinterpretation of past data.  
3. Enable deterministic verification across multiple versions.  
4. Support safe evolution of libraries and the CLI.  
5. Ensure auditability of old and new data.

---

## Core Principles

### 1. Immutable Specs

- Each engine/library version embeds a fixed, read-only set of specifications.  
- Spec sets are content-addressed (hashes) and immutable.  
- Specs do not change within a version; new versions produce new hashes.  
- Verification is deterministic: the binary itself proves which specs it enforces.

### 2. Version Domains

Charter distinguishes multiple independent version domains:

1. **Engine Version (Primary)** – Defines legitimacy semantics and authority evaluation.  
2. **Library Spec Versions (Derived)** – Each library may embed its own spec set; used for verification.  
3. **Export File Version (Structural)** – Defines schema and encoding; does not define legitimacy.  
4. **Hash Algorithm Version (Technical)** – Governs IDs and integrity; representation-only.  
5. **CLI Version (Ergonomic)** – Governs interface and defaults; never modifies legitimacy.

---

### 3. Legacy Ingestion Rules

When importing old commits, sessions, or baselines:

1. Data is interpreted using the **original version of the engine/specs** that created it.  
2. All legitimacy evaluation, authority checks, and constraints use the **original rules**.  
3. Once ingested, data may be optionally **upgraded** to current engine/library rules:  
   - Create a new resolution or commit reflecting the same intent.  
   - Preserve audit and provenance information.  
   - Ensure append-only integrity; never mutate the original object.  
4. Old commits/specs remain accessible for deterministic verification; they are never deleted.  
5. Verification of ingested legacy data requires comparing **embedded spec hashes** against the hashes recorded at the time of creation.

---

### 4. Per-Library Spec Verification

- Each library (Engine, CLI, V5 guidance, V6 baseline, V7 relay) may embed its own spec set.  
- Spec hashes allow independent verification of each library’s behavior.  
- When libraries interact:  
  - Engine can assert that a library commit conforms to its embedded specs.  
  - CLI can verify cross-library consistency.  
- Enables detection of behavioral divergence across forks or external systems.

---

### 5. Audit & Provenance

- All version information and spec hashes are stored in the audit trail.  
- Audit entries include:  
  - Engine/library version  
  - Embedded spec hash  
  - Commit, session, or resolution ID  
  - Timestamp of ingestion  
  - Upgrade actions (if applied)  
- This ensures:  
  - All data is traceable to the exact rules in effect at the time of creation.  
  - Verification of legacy data is reproducible independently.

---

### 6. Upgrade & Compatibility Policy

- Older commits **must not be reinterpreted silently** under newer engine rules.  
- Upgrades require explicit action and produce new history objects.  
- Engine, library, or CLI upgrades may add new specs or features, but **do not invalidate prior legitimacy**.  
- The system behaves as a versioned, append-only compiler: old inputs produce old outputs unless explicitly upgraded.  
- Backward compatibility is guaranteed only for read-only operations.  

---

### 7. Use Cases

- **Legacy Data Import:** Old commits evaluated under original rules, optionally upgraded.  
- **Fork Detection:** Different spec hashes identify forks or divergent binaries.  
- **Cross-Library Verification:** Embedded spec hashes ensure consistent behavior across Engine, CLI, V5, V6, V7.  
- **Audit Verification:** Every commit can be traced to its rules and version domain.

---

### 8. Non-Goals

- This policy does **not** enforce trust or prevent forks.  
- Does **not** automatically upgrade or reconcile old commits.  
- Does **not** replace manual governance or human review.  
- Only guarantees deterministic evaluation and traceable audit.

---

### 9. Summary

Embedding and versioning specifications per library allows:

- Deterministic verification of behavior.  
- Auditability and traceability across versions.  
- Safe ingestion of legacy data without reinterpretation.  
- Explicit handling of upgrades and forks.  
- Independent verification of all components (Engine, CLI, Guidance, Baseline, Relay).

Legacy and new data coexist safely, while compliance with embedded specs ensures integrity and mechanical legitimacy.