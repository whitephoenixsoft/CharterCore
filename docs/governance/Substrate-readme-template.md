# <Substrate Name>

Status: <DRAFT | ACTIVE | FROZEN | EVOLVING>  
Layer: <Runtime | Legitimacy | Artifact | Derived | Guidance | Transport>  
Depends On: <list of substrates>  
Used By: <list of substrates / products>  

---

# 1. Purpose

Describe what this substrate is responsible for.

Focus on:
- what it *does*
- what problem it solves
- why it exists in the system

---

# 2. Core Responsibilities

- 
- 
- 

---

# 3. Non-Responsibilities

Explicitly define what this substrate does **NOT** do.

- 
- 
- 

---

# 4. Core Concepts

Define key concepts introduced by this substrate.

## 4.1 <Concept Name>

- definition  
- properties  
- constraints  

---

# 5. Data / Artifact Model

Describe what this substrate produces or consumes.

## Produces
- 

## Consumes
- 

## Key Structures
- 

---

# 6. Operational Model

Describe how this substrate behaves at runtime.

- lifecycle
- interactions
- major flows

---

# 7. Boundaries & Interactions

## Upstream Dependencies
- what this substrate depends on

## Downstream Consumers
- what depends on this substrate

## Cross-Substrate Interactions
- notable integrations

---

# 8. Storage & Durability Posture

Describe how data is handled.

- Runtime-only (ephemeral)
- Locally persisted (mutable)
- Commit-backed (immutable)
- Relay/export capable

Explain current state vs future intent.

---

# 9. Commit Store Relationship (if applicable)

- Does this substrate produce commit artifacts?
- Does it require CCS?
- Does it depend on Commit Store?

Explain clearly.

---

# 10. API / Library Surface (Rust)

Describe intended shape:

- crate name (if known)
- major modules
- public vs internal APIs

---

# 11. FFI Considerations

- C ABI exposure: <planned / not planned>
- data structures crossing boundary
- stability concerns

---

# 12. Current Status

## Spec Status
- Draft / Revising / Frozen

## Implementation Status
- Not started / Prototype / Partial / Stable

## Stability
- Experimental / Evolving / Stable

---

# 13. Known Gaps / Risks

- 
- 
- 

---

# 14. Design Decisions

Capture important decisions and why they were made.

## DD-001 — <Title>

**Decision:**  
<what was decided>

**Context:**  
<what led to this>

**Implication:**  
<what this affects>

---

# 15. Open Questions

- 
- 
- 

---

# 16. Future Evolution

- expected changes
- expansion areas
- integrations not yet implemented

---

# 17. Mental Model

Explain how to think about this substrate in simple terms.

Example:

> "This substrate acts as..."

---

# 18. Related Specs

- link to detailed specs
- link to related substrates