# Charter — AI Assistance & Guidance Layer (V5)
Status: FROZEN (Design Foundation)  
Scope: Future Charter guidance layers (V5+)  
Does NOT define: engine semantics, authority rules, acceptance behavior

---

## Purpose

The V5 AI layer exists to **enhance human understanding**, not to make decisions.  

It is a **read-only, advisory layer** that:

- Surfaces potential gaps or inconsistencies
- Explains system state in human terms
- Summarizes history, context, and rationale
- Supports learning, reflection, and decision hygiene

AI guidance **must never** create legitimacy, enforce acceptance, or bypass authority.

> Mental Model:  
> “Explain what I’ve done.  
> Show me what I might be missing.  
> Never decide for me.”

---

## Core Principles

1. **Non-Blocking** — AI never prevents human action.  
2. **Read-Only** — AI observes immutable facts only.  
3. **Advisory** — Guidance may be incorrect; humans retain final authority.  
4. **Contextual** — AI explanations are always grounded in recorded data.  
5. **Transparent** — Any metadata, assumptions, or inferred observations are explicitly flagged.

---

## Guidance Surfaces

AI assistance operates over **immutable Charter facts** and is organized by functional layer.

### 1. Sessions
- Highlight resolution conflicts and scope mismatches
- Explain why acceptance may be blocked
- Summarize session outcomes (receipts)
- Provide rationale and contextual annotations for irreversible decisions

### 2. Queries & Discovery
- Natural-language querying over engine facts
- Locate relevant resolutions, sessions, or areas
- Rephrase CLI or API outputs into human-readable explanations
- Highlight missing context or potential exploration gaps

### 3. Baseline Review & Synthesis
- Detect duplicates or overlapping proposals
- Identify conflicts between imported and local history
- Summarize review deltas and accepted/rejected candidates
- Aid clarity for convergence decisions without enforcing them

### 4. Deliberate (Epic Guidance)
- Warn when discussion drifts from the epic
- Identify uncovered questions or assumptions
- Suggest readiness for synthesis (non-binding)
- Highlight missing perspectives or artifacts

### 5. Breakouts
- Check if produced artifacts meet declared criteria
- Flag incompleteness or ambiguity
- Never evaluate correctness or merit

### 6. Scope Awareness
- Surface scope violations or ambiguities
- Explain why a resolution or candidate does not fit the current area or scope
- Never auto-correct or modify scope

### 7. Auditing & Reporting
- Generate human-readable audit summaries
- Summarize candidate outcomes, session flows, and supersession chains
- Provide trend analysis for potential learning and psychological feedback
- Highlight potential cognitive risks (e.g., long-running unclosed sessions)

### 8. Scientific / Long-Horizon Practices
- Track hypothesis or assumption evolution over time
- Summarize rationale changes and decision hygiene
- Support low-budget scientific or research workflows without affecting legitimacy

---

## Human-Centric Philosophy

V5 AI guidance is designed to **support the user’s mind**:

- **Reflection:** Show the consequences of prior actions
- **Learning:** Surface previously unseen gaps or inconsistencies
- **Confidence:** Reduce fear or shame by clarifying decisions
- **Psychological Safety:** Provide advisory feedback without judgment
- **Mature Clarity:** Help users distinguish between considered and committed actions

---

## Metadata & Annotations

AI may leverage existing or new fields for richer guidance:

- **Rationale** — Explains why resolutions or sessions exist  
- **Irreversibility** — Highlights permanent or conditionally permanent decisions  
- **Guidance Annotations** — Suggestions or observations that do not affect legitimacy  
- **External Context Flags** — Identifies imported or foreign candidates, out-of-scope considerations

---

## Configuration & Deployment

- Local models, remote API models, or fully disabled operation  
- AI availability must **never** affect correctness, determinism, or legitimacy  
- Guidance outputs should remain **machine-readable**; CLI formatting may provide human-friendly presentation

---

## Hard Prohibitions

AI guidance **must never**:

- Accept, reject, or modify resolutions  
- Cast or change votes  
- Infer consent  
- Override authority or legitimacy rules  
- Mutate engine state  
- Block valid human action  

> If AI guidance feels authoritative, it has already failed.

---

## Relationship to Charter Canon

AI guidance in V5:

- Respects legitimacy principles and auditability guarantees  
- Observes engine immutability and authority rules  
- Provides advisory interpretations only; historical facts remain unchanged  
- May be wrong, but history must remain correct and reconstructible

---

## Future Considerations

Potential enhancements:

- Subtext advisory in CLI status outputs  
- “Consult” commands to interact with AI guidance  
- Mechanical questions to assess missing coverage without invoking chat-style responses  
- Integration with annotations for sessions, resolutions, and breakouts  

All future work must **adhere to boundaries** or explicitly revise them.