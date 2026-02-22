# Charter Core — Engine Boot & Rehydration Specification

Document ID: ENG-BOOT  
Status: FROZEN (v1)  
Audience: Charter Core engine & CLI implementers  
Scope: Engine initialization, rehydration, and readiness guarantees  

---

## 1. Purpose

### ENG-BOOT-01 — Deterministic Engine Initialization

ENG-BOOT defines how a Charter Core engine instance:

- Starts from a provided storage root  
- Rehydrates its internal state  
- Verifies object and ref integrity  
- Becomes ready to evaluate legitimacy

Boot **must** be:

- Deterministic  
- Read-only with respect to history  
- Free of implicit migration or mutation  

Fail if:

- Engine behavior depends on boot timing  
- Engine mutates state during boot without explicit operator action  

---

## 2. Engine Boundary

### ENG-BOOT-02 — Boot Is Host-Driven

The engine:

- Does **not** discover storage roots  
- Does **not** infer context  
- Does **not** read filesystem implicitly  

The host (CLI, server, test harness) **must provide**:

- Explicit storage root  
- Boot mode flags (e.g., `READ-ONLY`, `VERIFY-ONLY`)  

Fail if:

- Engine assumes current working directory  
- Engine resolves paths internally  

---

## 3. Boot Phases (Ordered & Mandatory)

### ENG-BOOT-03 — Phase 0: Storage Attachment

Engine **must**:

- Attach to provided storage root  
- Verify accessibility  
- Fail fast on I/O errors  

No semantic interpretation occurs at this phase.

Fail if:

- Engine continues silently on partial access  

---

### ENG-BOOT-04 — Phase 1: Object Store Load

Engine **must**:

- Discover all stored object envelopes  
- Validate envelope structure  
- Index objects by `object_hash`  

Notes:

- No refs are resolved  
- No legitimacy is evaluated  

Fail if:

- Duplicate hashes map to different content  
- Envelope metadata does not match digest  

---

### ENG-BOOT-05 — Phase 2: Object Integrity Verification

Engine **must**:

- Recompute hash for each object  
- Verify:

  - Hash algorithm  
  - Hash version  
  - Object type  

- Detect:

  - Corruption  
  - Truncation  
  - Mismatched content  

Failure handling:

- Boot **must fail** or  
- Engine may enter **DEGRADED / READ-ONLY** mode  

Fail if:

- Corrupted objects are silently accepted  

---

### ENG-BOOT-06 — Phase 3: Ref Store Load

Engine **must**:

- Load all refs  
- Validate ref syntax  
- Ensure each ref points to an existing object hash  

No authority or scope evaluation occurs yet.

Fail if:

- A ref points to a missing object  
- Required refs are malformed  

---

### ENG-BOOT-07 — Phase 4: Ref Consistency Validation

Engine **must** validate per Area:

- Exactly one active Authority ref  
- Exactly one active Scope ref  
- Ref target type matches ref semantics  

Fail if:

- Multiple active Authority refs exist  
- Authority or Scope ref is missing  
- Ref points to incorrect object type  

---

### ENG-BOOT-08 — Phase 5: Liveness Resolution

Engine **must compute**:

- **Live object set:** reachable from refs  
- **Inert object set:** unreachable from refs  

Rules:

- Inert objects must **not** affect engine behavior  
- Inert objects remain addressable for audit  

Fail if:

- Engine behavior depends on inert objects  

---

### ENG-BOOT-09 — Phase 6: Session State Rehydration

Engine **must**:

- Identify all session state refs  
- Rehydrate session state machines  

Enforce invariants:

- Blocked sessions remain blocked  
- Paused sessions remain paused  
- Closed sessions remain closed  

Fail if:

- Session state changes during boot  
- Incomplete sessions are auto-closed  

---

### ENG-BOOT-10 — Phase 7: Deterministic Readiness Check

Engine **must verify**:

- All Areas are either:

  - Initialized (Authority + Scope present)  
  - Explicitly uninitialized and blocked  

- No session violates current Authority or Scope invariants  

If violations exist:

- Engine enters **BLOCKED state**  
- Explicit operator action is required  

Fail if:

- Engine proceeds with illegitimate state  

---

## 4. Boot Modes

### ENG-BOOT-11 — Supported Boot Modes

Engine **must support**:

- **NORMAL:** full verification, read/write refs  
- **READ-ONLY:** full verification, no mutation  
- **VERIFY-ONLY:** integrity checks only, no rehydration  
- **RECOVERY:** load as much as possible, report all failures, no legitimacy evaluation  

Fail if:

- Mode semantics are conflated  

---

## 5. Migration & Upgrade Constraints

### ENG-BOOT-12 — No Implicit Migration

During boot:

- Hash upgrades **must not** occur  
- Ref rewrites **must not** occur  
- Object rewrites **must not** occur  

All migration requires:

- Explicit operator command  
- Explicit audit record  
- Explicit ref rebind  

Fail if:

- Engine mutates state during boot  

---

## 6. Import Interaction

### ENG-BOOT-13 — Import Is Post-Boot

- Imports **must occur after successful boot**  
- Imports **must use engine APIs**  
- Imports **must emit audit events**  

Fail if:

- Import logic executes during boot  

---

## 7. Determinism Guarantee

### ENG-BOOT-14 — Boot Is Deterministic

Given identical:

- Object store  
- Ref store  
- Engine version  

Boot **must produce**:

- Identical live object sets  
- Identical active authorities  
- Identical session states  

Fail if:

- Non-deterministic ordering affects engine behavior  

---

## 8. Audit Requirements

### ENG-BOOT-15 — Boot Is Auditable but Non-Legitimizing

Boot **may emit**:

- Diagnostic audit events  
- Integrity warnings  

Boot **must not** emit:

- Legitimacy events  
- Resolution acceptances  
- Session transitions  

Fail if:

- Boot alters legitimacy state  

---

## 9. CLI Responsibility (Non-Engine)

### ENG-BOOT-16 — CLI Owns Context Resolution

- CLI resolves user context  
- CLI selects storage root  
- CLI passes storage root to engine  

Engine **treats storage root as opaque**:

- Does not persist context pointers  

Fail if:

- Engine manages contexts internally  

---

## 10. Failure Semantics

### ENG-BOOT-17 — Fail Loud, Fail Early

Boot failures **must be**:

- Explicit  
- Descriptive  
- Non-recovering without operator intent  

Fail if:

- Engine attempts silent recovery  
- Engine masks corruption  

---

## 11. Summary Guarantees

ENG-BOOT guarantees:

- No silent mutation  
- Deterministic startup  
- Clear failure modes  
- Clean CLI ↔ engine boundary  
- Safe foundation for legitimacy evaluation  

---

## Mental Model

- Boot loads facts  
- Refs define relevance  
- Sessions define legitimacy  
- Nothing changes unless explicitly commanded  

This completes the core engine spine:

- **ENG-OBJ / ENG-HASH:** What exists  
- **ENG-REF:** What matters now  
- **ENG-AUD:** What happened  
- **ENG-BOOT:** How the engine comes alive