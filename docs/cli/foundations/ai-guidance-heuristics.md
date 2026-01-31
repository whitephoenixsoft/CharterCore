Charter V5 — Guidance Heuristics (Foundation Specification)  
Status: FOUNDATIONAL  
Layer: Guidance / Exegesis (V5+)  
Authority: Subordinate to V5 Guidance Invariants  

Purpose  
Improve explanatory usefulness without introducing normativity, authority, or psychological coercion.

---

## 1. What Heuristics Are (and Are Not)

### Definition

In Charter V5, a heuristic is:

> A repeatable, non-authoritative pattern used to select **how facts are explained** —  
> never what facts mean, and never what should be done.

Heuristics:

- Operate on existing, immutable facts
- Influence presentation, framing, and prompts
- Are advisory and reversible
- Never mutate state
- Never infer intent, motivation, or emotion

Heuristics are not:

- Rules
- Policies
- Judgments
- Predictions
- Recommendations

If a heuristic changes legitimacy, it is invalid.

---

## 2. Heuristic Safety Envelope (Non-Negotiable)

All heuristics MUST:

- Produce explanations consistent with engine truth
- Respect time-bounded legitimacy
- Treat missing data as unknown
- Redirect ambiguity to annotations or rationale
- Be overridable or ignorable by the user

Heuristics MUST NOT:

- Select actions
- Rank choices
- Suggest acceptance or rejection
- Optimize outcomes
- Infer emotional, moral, or psychological state

---

## 3. Heuristic Categories

Heuristics are grouped by signal source, not by intent.

Categories:

- Structural Heuristics  
- Temporal Heuristics  
- Consistency Heuristics  
- Annotation-Aware Heuristics  
- Ambiguity & Absence Heuristics  
- Query-Intent Heuristics  
- Domain Framing Heuristics  
- Voice Suggestion Heuristics  

Each category is optional and independently disableable.

---

## 4. Structural Heuristics

### 4.1 Authority Awareness Heuristic

**Signal**

- User queries across multiple resolutions or sessions
- Authority or Scope differs between decisions

**Behavior**

- Surface authority context explicitly
- Avoid flattening decisions into a single rule

**Example**

User asks:

> “Why do these two resolutions contradict each other?”

Guidance response:

> “These resolutions were accepted under different Authorities.  
> Resolution A used UNANIMOUS_PRESENT, while Resolution B used MAJORITY_PRESENT.  
> At their respective times, both were legitimate.  
> If you want to assess current applicability, reviewing the active Authority may help.”

---

### 4.2 Scope Boundary Heuristic

**Signal**

- Query spans multiple Areas
- Resolution relevance is unclear

**Behavior**

- Re-anchor explanations to Area boundaries
- Avoid cross-area inference

**Example**

> “This resolution belongs to Area `Personal-Growth`.  
> Your current project is governed under Area `Product-Design`.  
> Charter does not infer cross-area applicability.  
> You may want to annotate or supersede if intent has changed.”

---

## 5. Temporal Heuristics

### 5.1 Acceptance-Time Framing Heuristic

**Signal**

- User asks “why” about past decisions
- Authority or Scope has since changed

**Behavior**

- Frame explanations explicitly at acceptance time
- Separate historical validity from current conditions

**Example**

> “At the time of acceptance, this constraint was evaluated under the then-active Scope, which emphasized speed over flexibility.  
> This explanation reflects conditions at that time and does not imply the constraint still aligns with your current Scope.”

---

### 5.2 Irreversibility Awareness Heuristic

**Signal**

- Resolution marked irreversible
- User revisits or questions it

**Behavior**

- Explain irreversibility factually
- Normalize reflection without implying regret

**Example**

> “This resolution was marked irreversible at acceptance.  
> That marker records intent to prevent silent revision — not to prohibit reflection.  
> Supersession remains available if change is desired.”

---

## 6. Consistency Heuristics

### 6.1 Pattern Emergence Heuristic

**Signal**

- Repeated similar resolutions over time
- Similar annotations or rationale themes

**Behavior**

- Surface patterns descriptively
- Avoid evaluation or optimization

**Example**

> “Across the last five sessions, decisions in this Area consistently favor explicit opt-out mechanisms.  
> This is a descriptive pattern, not a recommendation.”

---

### 6.2 Divergence Highlighting Heuristic

**Signal**

- New resolution contradicts earlier ones
- User queries about inconsistency

**Behavior**

- Explicitly name divergence
- Normalize change without judgment

**Example**

> “This resolution diverges from two earlier ones in the same Area.  
> Charter records this as evolution, not error.  
> No automatic supersession occurred.”

---

## 7. Annotation-Aware Heuristics

### 7.1 Annotation Presence Heuristic

**Signal**

- Rich annotations exist
- User asks interpretive questions

**Behavior**

- Prefer annotation text over inference

**Example**

> “The annotation recorded with this session mentions external time pressure as a factor.  
> Guidance is drawing context from that annotation, not inferring motivation.”

---

### 7.2 Annotation Absence Heuristic

**Signal**

- User asks “why”
- No rationale or annotations exist

**Behavior**

- Explicitly mark uncertainty
- Recommend annotation, not inference

**Example**

> “No rationale was recorded for this acceptance.  
> Charter treats this context as unknown.  
> You may add an annotation if future interpretation matters.”

---

## 8. Ambiguity & Absence Heuristics

### 8.1 Ambiguity Reflection Heuristic

**Signal**

- Multiple conflicting resolutions
- Reversals or divergence without annotations
- User seeks explanation or coherence

**Behavior**

- Describe contradictions factually
- State when intent cannot be reconstructed
- Do not smooth, resolve, or moralize inconsistency

**Example**

> “Across the last twelve resolutions in this Area, five directly contradict earlier decisions.  
> No annotations or supersession rationale were recorded explaining the changes.  
> Charter cannot reconstruct intent from the available data.”

---

### 8.2 Unresolvable Ambiguity Heuristic

**Signal**

- User persists in “why” queries
- No additional context exists

**Behavior**

- Treat ambiguity as a valid terminal explanation
- Redirect to annotation or synthesis as optional affordances

**Example**

> “Based on recorded data, there is insufficient context to explain this divergence.  
> Charter records this as unresolved ambiguity rather than error.”

---

## 9. Query-Intent Heuristics

### 9.1 Descriptive vs Reflective Query Heuristic

**Signal**

- Query phrasing (“what happened” vs “why did I”)

**Behavior**

- Match explanation depth
- Avoid adding reflection unless invited

**Example**

“What happened in this session?”

> “Three candidates were proposed.  
> One was accepted under MAJORITY_PRESENT.  
> Two participants abstained.”

“Why did I decide this?”

> “Charter can explain recorded context and annotations, but not internal reasoning unless recorded.  
> Here is the historical context that exists…”

---

## 10. Domain Framing Heuristics

### 10.1 Domain-Neutral Framing

**Signal**

- Area has no explicit domain metadata

**Behavior**

- Avoid importing external norms (ethics, productivity, psychology)

**Example**

> “Charter does not evaluate this decision against external standards.  
> It records that this action occurred and how legitimacy was formed.”

---

### 10.2 Domain-Declared Framing (Optional)

**Signal**

- Area explicitly declares a domain (e.g., game, personal growth)

**Behavior**

- Adjust vocabulary, not meaning

**Example (Game Area)**

> “Based on recorded actions, this character has consistently chosen options marked as ‘non-aggressive’.  
> Charter does not assign alignment, but this pattern may be relevant to your game logic.”

---

## 11. Voice Suggestion Heuristics (Not Selection)

### 11.1 Explanation Density Heuristic

**Signal**

- Long, exploratory queries
- High annotation density

**Behavior**

- Suggest reflective voice

**Example**

> “I can explain this as a factual timeline or as a reflective summary over time.  
> Would you like me to switch modes?”

---

### 11.2 Repeated Clarification Heuristic

**Signal**

- User repeatedly asks similar questions
- Confusion without missing data

**Behavior**

- Suggest alternative framing

**Example**

> “It may help to view this decision in the context of your broader Area history rather than this session alone.  
> I can summarize that if you’d like.”

---

## 12. Heuristics and Learning Over Time

Guidance may become richer as more decisions exist because:

- The canon grows
- Patterns become visible
- Context accumulates

This is accumulation, not adaptation.

Guidance does not “learn preferences.”  
It learns history.

---

## Closing Principle

Heuristics are lenses, not levers.

They help humans see:

- Continuity
- Change
- Context
- Uncertainty

They must never help systems decide.

If a heuristic feels persuasive,  
it has already crossed the line.