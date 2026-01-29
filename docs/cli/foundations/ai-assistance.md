# Charter — AI Assistance & Guidance Layer (V5 Holding Notes)

Status: HOLDING (Non-Spec Guidance)  
Applies to: Future Charter guidance layers (V5+)  
Does NOT define: engine semantics, authority rules, acceptance behavior

---

## Purpose

This document captures the **intended role of AI assistance in Charter**
as a *guidance and clarification layer*, not a decision-maker.

It exists to:
- preserve design intent
- prevent accidental authority leakage
- scope future AI work safely

This is not a commitment to implementation.
It is a boundary-setting artifact.

---

## Core Principle

AI in Charter exists to **help humans understand what already exists**.

It must never:
- decide
- accept
- reject
- infer authority
- mutate state
- shortcut legitimacy

AI guidance is *read-only*, *advisory*, and *non-blocking*.

---

## Intended Guidance Surfaces

AI assistance may operate over **existing Charter facts**, including:

### 1. Sessions
- Highlight resolution conflicts
- Surface scope mismatches
- Explain why acceptance is blocked
- Summarize session outcomes (receipts)

### 2. Queries & Discovery
- Natural-language querying over immutable facts
- Help locate relevant resolutions, sessions, or areas
- Rephrase mechanical output into human-readable explanations

### 3. Baseline Review & Synthesis
- Detect duplicate or overlapping proposals
- Flag conflicts between imported and local history
- Summarize review deltas and acceptance results
- Assist with convergence clarity (not decisions)

### 4. Deliberate (Orchestration Guidance)
- Warn when discussion drifts from the epic
- Identify uncovered questions or assumptions
- Suggest synthesis readiness (non-binding)
- Highlight missing perspectives or artifacts

### 5. Breakouts
- Check whether produced artifacts meet declared criteria
- Flag incompleteness or ambiguity
- Never evaluate correctness or merit

### 6. Scope Awareness
- Surface scope violations or ambiguities
- Explain why a proposal does not fit current scope
- Never auto-correct scope

### 7. Auditing & Reporting
- Generate human-readable audit summaries
- Produce reports suitable for reviewers or auditors
- Explain lineage and supersession chains

### 8. Scientific / Long-Horizon Practices (Exploratory)
- Track hypothesis changes over time
- Summarize rationale evolution
- Preserve decision hygiene during revision

---

## Configuration & Deployment

Guidance layers may support:
- Local models
- Remote models (API keys)
- Fully disabled operation

AI availability must never affect:
- correctness
- determinism
- legitimacy
- engine safety

---

## Hard Prohibitions (Frozen)

AI guidance must NEVER:
- accept a resolution
- cast or modify votes
- infer consent
- override authority
- mutate engine state
- block a valid human action

---

## Mental Model

> “Explain what I’ve done.
> Show me what I might be missing.
> Never decide for me.”

If guidance feels authoritative,
it has already failed.

---

## Relationship to Charter Canon

AI guidance is evaluated against:
- Charter legitimacy principles
- Auditability guarantees
- Engine immutability
- Explicit authority rules

Guidance may be wrong.
History must remain right.

---

## Status

This document is a **holding place**.
Details are intentionally deferred.

Future work must align with these boundaries
or explicitly revise them.