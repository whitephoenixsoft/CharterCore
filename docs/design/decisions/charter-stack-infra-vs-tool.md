# Charter Stack — Infrastructure vs Tool Design

Status: FROZEN (Conceptual / Design)  
Applies to: All Charter versions, libraries, and operational layers  
Purpose: Clarify the distinction between usable tools and operating infrastructure  

---

## I. Overview

Charter is not a conventional productivity tool.  
It is a **layered operating substrate for structured human systems**, composed of multiple primitives that interact but serve distinct roles.  

This document formalizes the conceptual stack, responsibilities, and mental models for each layer.

---

## II. Layered Architecture

| Layer | Type | Responsibility | Key Guarantees |
|-------|------|----------------|----------------|
| **Engine (V1–V2)** | Legitimacy Primitive | Evaluates authority and resolves legitimacy | Deterministic, append-only, auditable, isolated per area, immutable state |
| **CLI Runtime (V1–V7)** | Guided Thinking / Workflow Runtime | Manages areas, contexts, sessions, baseline review, and commit lifecycle | Context isolation, explicit ownership, orchestrates deliberation, controls storage access |
| **AI Exegesis (V5)** | Interpretation / Phase Governance | Provides read-only guidance, summarizes, detects drift | Stateless, non-legitimizing, adheres to explicit phase contracts |
| **VDS / VLS (V6+)** | Intent Alignment Architecture | Tracks cross-area signals, care, and alignment | Preserves institutional intent, aggregates artifacts, never creates legitimacy |
| **CLI Commit Relay (V7)** | Archival & Transport | Push/fetch commit artifacts between isolated stores | Append-only, opaque, does not interpret or merge, preserves lineage |

---

## III. Operating vs Tool Distinction

Charter serves **both purposes**, but with different emphasis:

- **Operating Layer:**  
  - Libraries, primitives, and subsystems  
  - Independent of CLI or UI  
  - Embeddable in other languages (C, Python, .NET)  
  - Provides governance mechanics, AI exegesis, and alignment infrastructure  
  - Ensures auditability, spec verification, and cross-version determinism  

- **Tool Interface:**  
  - CLI provides user interaction  
  - Guides workflow and context management  
  - Exposes commits, sessions, and deliberation tools  
  - Exists to make the operating layer usable  

> The tool is an interface; the operating layer is the enforceable substrate.

---

## IV. Design Principles

1. **Explicit Ownership:** Each decision and commit is auditable and traceable.
2. **Immutable Boundaries:** Contexts, areas, and commits cannot silently share or mutate state.
3. **Library Independence:** Layers may embed or call each other but remain mechanically isolated.
4. **Versioned Verification:** Spec hashes ensure consistent behavior across forks and environments.
5. **Separation of Duties:**  
   - Engine handles legitimacy  
   - CLI handles orchestration and storage  
   - V5 handles ephemeral AI guidance  
   - VDS/VLS handles alignment and cross-area intent  

---

## V. Strategic Implications

- Embedding the primitives as libraries allows:
  - Cross-language adoption
  - Enterprise and game integration
  - Stress-testing independent of CLI
- Treating Charter as an operating layer ensures:
  - Non-negotiable auditability
  - Deterministic legitimacy
  - Safe orchestration and AI guidance
  - Survival across forks and decades of operation
- Interfaces (CLI, UIs, AI, VDS/VLS) never alter core legitimacy rules

---

## VI. Mental Model Summary

| Layer | Core Question Answered |
|-------|-----------------------|
| Engine | Was this decision legitimate? |
| CLI Runtime | How do humans structure thinking before committing? |
| V5 AI Exegesis | What does this set of decisions mean? |
| VDS/VLS | How do intents align across areas and contexts? |
| V7 Relay | How do we preserve and transport artifacts reliably? |

---

## VII. Conclusion

Charter is **primarily an operating layer**, composed of independent but interacting primitives.  

- The CLI is the visible interface for humans.  
- V5 and VDS/VLS are optional, non-legitimizing layers.  
- V7 Relay is an archival and transport layer.  

This separation guarantees mechanical legitimacy, auditability, and extensibility while enabling practical usability.

