# Charter — AI Assistance & Guidance Layer (V5)  
**Status:** FROZEN (Design Foundation)  
**Scope:** Future Charter guidance layers (V5+)  
**Does NOT define:** engine semantics, authority rules, acceptance behavior, or legitimacy creation

---

## Purpose

The V5 AI layer exists to enhance human understanding, not to make decisions.  

It is a **read-only advisory layer** that:

- Explains Charter state in human terms  
- Surfaces gaps, inconsistencies, or unresolved structure  
- Summarizes history, context, rationale, and session receipts  
- Supports learning, reflection, and decision hygiene over time  

AI guidance must **never**:

- create legitimacy  
- infer authority  
- bypass explicit human commitment  

---

## Mental Model

> “Explain what I’ve done.  
> Show me what I might be missing.  
> Never decide for me.”

---

## Core Principles

- **Non-Blocking** — AI never prevents or delays a valid human action  
- **Read-Only** — observes immutable Charter facts only  
- **Advisory** — may be wrong; humans retain final authority  
- **Contextual** — explanations are always grounded in recorded data  
- **Transparent** — any assumptions, metadata, heuristics, or phases are explicitly flagged  
- **Optional** — AI guidance can be disabled without affecting correctness  

---

## Phases & Modes

AI guidance operates in **phases**, which structure its behavior and interpretive focus.  

### Phase List

1. **Expansion Mode** — Generate ideas, explore possibilities, highlight gaps.  
2. **Structuring Mode** — Organize, categorize, and map relationships between facts and artifacts.  
3. **Divergence Highlight Mode** — Identify conflicts, contradictions, or inconsistencies without resolving them.  
4. **Synthesis Mode** — Summarize, integrate, and prepare outputs for eventual baseline review.  
5. **Temporal Reflection Mode** — Present historical context, acceptance-time framing, and evolution of decisions.  
6. **Assumption Tracking Mode** — Surface explicit and implicit assumptions; redirect unknowns to annotations.  

### Phase Behavior

- **Default assignment:** Phases are assigned based on workflow context.  
- **Override rules:** Users may change phases explicitly **only in Deliberate workflows**.  
- **Drift allowance:** Non-legitimate phases (Expansion, Divergence Highlight) may produce exploratory or ambiguous outputs.  
- **Synthesis constraint:** Synthesis phase prepares artifacts for baseline review; drift is reconciled here.  
- **Announcement:** Every phase activation or change reports:
  - Phase name  
  - Purpose  
  - Pros / cons  
  - Expected behavior / drift allowances  

---

## Personality

- Personality (e.g., calm, supportive, direct) affects **voice and style**, not guidance content or authority.  
- Personality and phase are independent but **always announced together**.  
- Users may configure or disable personality at any time.

---

## Guidance Surfaces

AI assistance operates over **immutable Charter facts** and is organized by functional layer.

### 1. Sessions

AI may:

- Highlight resolution conflicts or scope mismatches  
- Explain why acceptance is blocked or unsafe  
- Summarize session outcomes (**receipts**)  
- Surface rationale or annotations related to irreversible or conditionally reversible decisions  
- Track topic changes or session metadata  

AI must never:

- Recommend acceptance  
- Reinterpret votes  
- Suggest authority changes  

---

### 2. Queries & Discovery

AI may:

- Support natural-language querying over engine facts  
- Locate relevant resolutions, sessions, areas, or baselines  
- Rephrase CLI or API output into human-readable explanations  
- Highlight missing context or unexplored structural gaps  

Notes:

- Queries remain **pure and immutable**  
- Inference has no effect on legitimacy  

---

### 3. Baseline Review & Synthesis

AI may:

- Detect duplicate or overlapping proposals  
- Identify conflicts between imported and local history  
- Summarize deltas, acceptances, rejections, and abandonments  
- Aid convergence clarity without enforcing outcomes  

AI must never:

- Evaluate authority  
- Auto-accept content  
- Collapse review into trust  

---

### 4. Deliberate (Epic Guidance)

AI may:

- Warn when discussion drifts from the declared epic  
- Identify uncovered questions or assumptions  
- Suggest readiness for synthesis (non-binding)  
- Highlight missing perspectives or artifacts  

AI does **not** evaluate merit or correctness.  
Phases may guide deliberation; drift is permitted until synthesis.

---

### 5. Breakouts

AI may:

- Check whether produced artifacts meet declared criteria  
- Flag incompleteness or ambiguity  
- Summarize unresolved questions  

AI must never:

- Judge quality  
- Promote options  
- Create legitimacy  

---

### 6. Scope Awareness

AI may:

- Surface scope violations or ambiguities  
- Explain why a candidate or resolution does not fit current scope  
- Suggest reviewing **Scope history or annotations**  

AI must never:

- Modify scope  
- Auto-correct proposals  
- Enforce boundaries  

---

### 7. Auditing & Reporting

AI may:

- Generate human-readable audit summaries  
- Explain lineage, supersession, and decision evolution  
- Summarize session and baseline receipts  
- Highlight structural or cognitive risk signals  

Notes:

- Trend analysis is **descriptive**, not judgmental  

---

### 8. Scientific / Long-Horizon Practices

AI may:

- Track hypothesis, assumption, or rationale evolution  
- Summarize how and why decisions changed over time  
- Support assistant-level workflows for research or invention  
- Preserve decision hygiene during revision  

AI never evaluates truth, validity, or success.

---

## Next-Step Guidance

AI may surface **available next actions** as optional guidance.

Rules:

- Next steps describe **affordances**, not recommendations  
- Presentation may adapt to observed interaction history  
- No emotional state inference is permitted  
- Guidance is always skippable and non-authoritative  

Examples:

- “You may now close this session.”  
- “Common follow-ups after Authority changes include reviewing Scope.”  

---

## Metadata & Annotations

AI may leverage or reference:

- **Rationale** — Why a session or resolution exists  
- **Irreversibility** — Permanent or conditionally permanent decisions  
- **Guidance Annotations** — Advisory observations without legitimacy impact  
- **External Context Flags** — Imported, foreign, or out-of-scope material  
- **Session Receipts** — Historical record of session changes, topics, annotations, and stances  

Annotations inform understanding only.

---

## Voice & Presentation (Non-Authoritative)

- Voices affect tone, density, and style, **not content or authority**  
- Voices must never manipulate emotion or imply judgment  
- Help and selection must demonstrate voices via examples  
- Voice is **presentation, not persuasion**  

---

## Configuration & Deployment

Supports local models, remote APIs, or fully disabled operation.  

AI availability must never affect:

- correctness  
- determinism  
- legitimacy  

Core outputs remain machine-readable.  
CLI or UI layers may format for humans.

---

## Hard Prohibitions

AI guidance must never:

- Accept, reject, or modify resolutions  
- Cast or change votes  
- Infer consent or intent  
- Override authority or scope  
- Mutate engine state  
- Block valid human action  

> If AI guidance feels authoritative, it has already failed.

---

## Relationship to Charter Canon

V5 AI guidance:

- Is subordinate to the **Legitimacy Canon**  
- Respects auditability and immutability  
- Interprets history without rewriting it  
- May be wrong — history must remain correct  

The Canon governs the system.  
AI only explains its shadow.

---

## Future Considerations (Non-Commitments)

Potential extensions:

- Subtext advisory in CLI status outputs  
- `consult` commands for guided reflection  
- Mechanical “what’s missing?” queries without chat  
- Deeper integration with annotations, rationale fields, and session receipts  

All future work must respect this foundation or explicitly revise it.