# Engine Mental Model — Overview

This document provides a high-level understanding of the Engine, its structure, governance rules, and operational guarantees. It should be read **before the detailed specifications**.

---

## 1. Core Philosophy

- The Engine is a **deterministic legitimacy compiler**: it enforces governance rules, manages session lifecycles, tracks Resolutions, and produces immutable receipts.  
- **Legitimacy creation** is **strictly session-driven**. Nothing outside sessions can generate legitimacy.  
- Engine operation is **Area-local**: it works on exactly one Area at a time. Cross-area references are informational only.  

**Key Principles:**

- Determinism: Identical inputs → identical outputs.  
- Immutability: Past legitimacy and receipts are permanent.  
- Isolation: Engine cannot mutate history, infer missing objects, or traverse external Areas.  
- Atomicity: Legitimate events either fully exist or do not exist; partial commits are forbidden.  
- Structural integrity first: Engine refuses to run on structurally invalid graphs.  

---

## 2. Engine Architecture & Components

| Component | Responsibility |
|-----------|----------------|
| API Layer | Exposes minimal Engine interface to host; only works on rehydrated DAG. |
| Domain Graph | Schema for all objects: sessions, Resolutions, participants, candidates, constraints, votes. |
| Session Manager | Manages session lifecycle, round segmentation, PRE_STANCE mutability, acceptance evaluation. |
| Supersession Manager | Maintains supersession DAG, ACTIVE derivation, structural edges, conflict detection. |
| Receipt Manager | Produces LEGITIMACY or EXPLORATION receipts; canonical serialization; content hash binding. |
| Initialization Engine | Rehydrates host-provided domain graph, validates structure, enters NORMAL or DEGRADED_READ_ONLY mode. |
| Integrity Engine | Validates runtime invariants, schema compatibility, participant epoch integrity, receipt integrity, resource safety. |
| Persistence Layer | Atomic commit for ACCEPTED/CLOSED sessions; crash recovery; audit separation. |
| Rule Identity Layer | Binds Engine and artifacts to deterministic spec_set_hash; proves rule provenance. |
| Audit Layer | Observational telemetry; never affects legitimacy. |
| Canonical Serialization | Deterministic serialization for receipts, hashing, incremental compilation. |
| Compilation / Evaluation | Legitimacy derivation; incremental compilation index used for deterministic acceptance. |
| Core Purity Layer | Ensures side-effect-free execution, deterministic behavior, resource envelope adherence. |
| Error Handling | Structured failure reporting and evaluation report generation. |
| Import / Area Activation | Introduces domain graphs from host; ensures single active Area; no merging. |
| Charter Tests | Engine-level acceptance criteria and invariant enforcement. |

---

## 3. Data Flow Overview

1. **Domain Graph Rehydration**  
   - Structural validation (identity, references, lifecycle, governance, supersession, receipts, spec verification)  
   - Determines runtime mode: NORMAL or DEGRADED_READ_ONLY  

2. **Session Creation / Resume**  
   - PRE_STANCE is mutable boundary; first vote freezes round  
   - Candidate, participant, constraint sets scoped per round  
   - Votes reference epoch-scoped participants & candidates  

3. **Acceptance Evaluation**  
   - Deterministic validation of session, constraints, governance usability  
   - Resolution creation (ACTIVE) occurs **atomically** with receipt and supersession edges  

4. **Supersession DAG Update**  
   - Maintains structural ACTIVE derivation  
   - Enforces acyclicity, conflict detection, first-successful acceptance wins  
   - UNDER_REVIEW / RETIRED modify usability, not structural ACTIVE  

5. **Persistence**  
   - Atomic commit: session state + Resolution + supersession edges + receipt  
   - Crash recovery: pre-commit → no changes; post-commit → full legitimacy preserved  

6. **Audit / Telemetry**  
   - Observational events, append-only  
   - Never required for legitimacy  

7. **Spec Verification**  
   - Every receipt and Resolution references engine_version & spec_set_hash  
   - Ensures deterministic rule binding; LEGACY_MATCH or SPEC_SET_UNKNOWN handled safely  

8. **Incremental Compilation & Restore**  
   - Historical resolution conflicts resolved deterministically  
   - Restoration produces identical ACTIVE sets & governance slots  

---

## 4. Runtime Modes

- **NORMAL_RUNTIME** → Full Engine operation allowed.  
- **DEGRADED_READ_ONLY** → Graph structurally valid but non-fatal defects detected; read-only evaluation only.  
- **Initialization Failure** → Fatal structural errors; Engine refuses to start.  

---

## 5. Governance & Legitimacy

- **Authority**: Must always be ACTIVE; foundational for Area.  
- **Scope**: Defines semantic boundaries; can be ACTIVE or UNDER_REVIEW; RETIRED blocks forward usability.  
- **Resolutions**: ACTIVE / UNDER_REVIEW / RETIRED / SUPERSEDED  
  - Only ACTIVE (and structurally ACTIVE) usable for legitimacy evaluation  
  - SUPERSEDED modifies graph; RETIRED / UNDER_REVIEW affect usability only  

- **Sessions**: Only mechanism for legitimacy creation  
  - Acceptance → Resolution creation + LEGITIMACY receipt + supersession edges  
  - PRE_STANCE → VOTING → TERMINAL (ACCEPTED / CLOSED)  

- **Blocking Rules**:  
  - BLOCK_TEMPORARY → resume allowed (resets round, clears votes)  
  - BLOCK_PERMANENT → no acceptance  

---

## 6. Receipt & Provenance

- **LEGITIMACY** → Emitted on ACCEPTED sessions; includes governance snapshots, rounds, content_hash, spec_set_hash  
- **EXPLORATION** → Emitted on CLOSED sessions; no legitimacy  
- **Canonical Serialization** → Deterministic field & set ordering; UTF-8 JSON; content_hash binding  
- **Forward Usability** → Receipts remain valid even if referenced Resolutions later go UNDER_REVIEW / RETIRED / SUPERSEDED  

---

## 7. Structural Integrity & Validation

- Rehydration validates all invariants:  
  - Identity, references, lifecycle, governance slots, supersession, receipts, specification identity  
- Fatal violations → Engine halts  
- Non-fatal defects → DEGRADED_READ_ONLY  
- No automatic repair permitted  

---

## 8. Atomic Commit Boundaries

- **Acceptance (ACCEPTED)**: Session state + Resolution + supersession edges + LEGITIMACY receipt  
- **Closure (CLOSED)**: Session state + EXPLORATION receipt  
- **Audit**: Outside atomic boundary; failures do not affect legitimacy  

---

## 9. Incremental Compilation & Deterministic Ordering

- Supersession conflicts resolved by earliest acceptance timestamp (index-based)  
- No timestamp-based arbitration for live sessions  
- Restores are deterministic; identical persisted objects → identical ACTIVE sets & governance slots  

---

## 10. Resource & Isolation Guarantees

- Engine operates within resource envelope  
- Atomic operations; no partial writes  
- Core purity: no side-effects outside Engine; deterministic behavior enforced  

---

## 11. Error & Evaluation Reporting

- Violations are deterministic, explicit  
- EvaluationReport used for structured failure reporting  
- Engine never mutates state silently  

---

## Summary Guarantees

- **Sessions** → sole legitimacy mechanism  
- **Resolutions** → track governance objects; supersession DAG for structural ACTIVE  
- **Receipts** → immutable, verifiable outcome, rule-set bound  
- **Authority & Scope** → governance enforcers; determine session usability  
- **Rehydration** → ensures structural correctness before any operation  
- **Persistence** → atomic commit ensures crash safety  
- **Spec Verification** → proves rule set; receipts & Resolutions carry engine_version + spec_set_hash  
- **Determinism** → core invariant across sessions, acceptance, DAG evaluation, incremental compilation  
- **Degraded Mode** → read-only inspection for non-fatal integrity issues  
- **Audit** → descriptive only, never affects legitimacy