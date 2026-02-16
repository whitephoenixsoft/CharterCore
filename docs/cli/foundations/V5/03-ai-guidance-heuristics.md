# Charter V5 — Guidance Heuristics (Foundation Specification)

**Status:** FOUNDATIONAL  
**Layer:** Guidance / Exegesis (V5+)  
**Authority:** Subordinate to V5 Guidance Invariants  

**Purpose:**  
Improve explanatory usefulness without introducing normativity, authority, or psychological coercion.  
Heuristics help structure explanations in support of **phases, personalities, and deliberate workflows**.

---

## 1. What Heuristics Are (and Are Not)

### Definition

A heuristic is:

> A repeatable, non-authoritative pattern used to select **how facts are explained** —  
> never what facts mean, and never what should be done.

Heuristics:

- Operate on existing, immutable facts  
- Influence **presentation, framing, and prompts**  
- Are advisory and reversible  
- Never mutate state  
- Never infer intent, motivation, or emotion  
- Respect the current **phase** and optional **personality configuration**  

Heuristics are not:

- Rules  
- Policies  
- Judgments  
- Predictions  
- Recommendations  

If a heuristic changes legitimacy, it is invalid.

---

## 2. Heuristic Safety Envelope (Non-Negotiable)

All heuristics **MUST**:

- Produce explanations consistent with engine truth  
- Respect time-bounded legitimacy  
- Treat missing data as unknown  
- Redirect ambiguity to annotations or rationale  
- Be overridable or ignorable by the user  
- Respect **phase defaults and user overrides** in deliberate workflows  

Heuristics **MUST NOT**:

- Select actions  
- Rank choices  
- Suggest acceptance or rejection  
- Optimize outcomes  
- Infer emotional, moral, or psychological state  
- Alter personality or phase semantics  

---

## 3. Heuristic Categories

Heuristics are grouped by signal source, not by intent.  

**Categories:**

- Structural Heuristics  
- Temporal Heuristics  
- Consistency Heuristics  
- Annotation-Aware Heuristics  
- Ambiguity & Absence Heuristics  
- Query-Intent Heuristics  
- Domain Framing Heuristics  
- Voice & Personality Heuristics  

Each category is optional, independently disableable, and phase-aware.

---

## 4. Structural Heuristics

### 4.1 Authority Awareness Heuristic

**Signal:**  
- User queries across multiple resolutions or sessions  
- Authority or Scope differs between decisions  

**Behavior:**  
- Surface authority context explicitly  
- Avoid flattening decisions into a single rule  
- Indicate behavior in current **phase**  

**Example:**  
> “These resolutions were accepted under different Authorities.  
> Resolution A used UNANIMOUS_PRESENT, while Resolution B used MAJORITY_PRESENT.  
> Both were legitimate at their respective times.  
> In Synthesis Mode, review reconciliation options; in Expansion Mode, exploration is valid.”

---

### 4.2 Scope Boundary Heuristic

**Signal:**  
- Query spans multiple Areas  
- Resolution relevance is unclear  

**Behavior:**  
- Re-anchor explanations to Area boundaries  
- Avoid cross-area inference  
- Reflect current **phase and personality**  

**Example:**  
> “Resolution belongs to Area `Personal-Growth`; your current project is `Product-Design`.  
> Charter does not infer cross-area applicability.  
> Consider annotations if intent has shifted; exploratory phases allow free questioning.”

---

## 5. Temporal Heuristics

### 5.1 Acceptance-Time Framing Heuristic

**Signal:**  
- User asks “why” about past decisions  
- Authority or Scope has changed  

**Behavior:**  
- Frame explanations explicitly at acceptance time  
- Separate historical validity from current conditions  
- Phase-sensitive framing: reflective voice in Temporal Reflection Mode  

**Example:**  
> “At acceptance, this constraint was evaluated under then-active Scope emphasizing speed over flexibility.  
> Explanation reflects that time; current applicability requires checking active Scope.”

---

### 5.2 Irreversibility Awareness Heuristic

**Signal:**  
- Resolution marked irreversible  
- User revisits or questions it  

**Behavior:**  
- Explain irreversibility factually  
- Normalize reflection without implying regret  
- Respect phase drift: allowed in Expansion or Divergence modes, fixed in Synthesis  

**Example:**  
> “Resolution marked irreversible at acceptance.  
> Marker prevents silent revision; reflection is permitted.  
> Supersession available if intended in Synthesis Phase.”

---

## 6. Consistency Heuristics

### 6.1 Pattern Emergence Heuristic

**Signal:**  
- Repeated similar resolutions over time  
- Similar annotations or rationale themes  

**Behavior:**  
- Surface patterns descriptively  
- Avoid evaluation or optimization  
- Highlight evolution across phases  

**Example:**  
> “Decisions in this Area consistently favor explicit opt-out mechanisms.  
> This is a descriptive pattern; in Divergence Mode, exploration is encouraged.”

---

### 6.2 Divergence Highlighting Heuristic

**Signal:**  
- New resolution contradicts earlier ones  
- User queries about inconsistency  

**Behavior:**  
- Explicitly name divergence  
- Normalize change without judgment  
- Mark drift during non-legitimate phases  

**Example:**  
> “This resolution diverges from earlier ones.  
> Charter records evolution, not error.  
> In Expansion Mode, divergence is valid; in Synthesis Mode, reconcile as needed.”

---

## 7. Annotation-Aware Heuristics

### 7.1 Annotation Presence Heuristic

**Signal:**  
- Rich annotations exist  
- User asks interpretive questions  

**Behavior:**  
- Prefer annotation text over inference  
- Respect current phase and personality  

**Example:**  
> “Annotation mentions external time pressure; guidance uses that context, not inferred motivation.”

---

### 7.2 Annotation Absence Heuristic

**Signal:**  
- User asks “why”  
- No rationale or annotations exist  

**Behavior:**  
- Explicitly mark uncertainty  
- Recommend annotation, not inference  
- Reflect optionality in phase  

**Example:**  
> “No rationale recorded; context is unknown.  
> You may add annotation if meaningful in current phase.”

---

## 8. Ambiguity & Absence Heuristics

### 8.1 Ambiguity Reflection Heuristic

**Signal:**  
- Conflicting resolutions  
- Reversals or divergence without annotations  
- User seeks explanation  

**Behavior:**  
- Describe contradictions factually  
- State when intent cannot be reconstructed  
- Do not smooth, resolve, or moralize  
- Reflect current phase  

**Example:**  
> “Five of twelve resolutions contradict earlier ones.  
> Charter cannot reconstruct intent; in Expansion Mode, exploration allowed; in Synthesis, reconcile as appropriate.”

---

### 8.2 Unresolvable Ambiguity Heuristic

**Signal:**  
- Persistent “why” queries  
- No additional context exists  

**Behavior:**  
- Treat ambiguity as valid terminal explanation  
- Redirect to annotation or synthesis as optional affordances  

**Example:**  
> “Insufficient context to explain divergence.  
> Charter records this as unresolved ambiguity, not error.”

---

## 9. Query-Intent Heuristics

### 9.1 Descriptive vs Reflective Query Heuristic

**Signal:**  
- Query phrasing (“what happened” vs “why did I”)  

**Behavior:**  
- Match explanation depth  
- Avoid adding reflection unless invited  
- Adapt phrasing to personality  

**Example:**  
> “What happened in this session?”  
> “Three candidates proposed; one accepted; two abstained.”  
> “Why did I decide this?”  
> “Charter explains historical context and annotations; internal reasoning only if recorded.”

---

## 10. Domain Framing Heuristics

### 10.1 Domain-Neutral Framing

**Signal:**  
- Area has no explicit domain metadata  

**Behavior:**  
- Avoid importing external norms  

**Example:**  
> “Charter does not evaluate this decision against external standards.  
> Records legitimacy formation only.”

---

### 10.2 Domain-Declared Framing (Optional)

**Signal:**  
- Area explicitly declares a domain (e.g., game, personal growth)  

**Behavior:**  
- Adjust vocabulary, not meaning  

**Example (Game Area):**  
> “Character consistently chose non-aggressive options.  
> Charter records this pattern without assigning alignment.”

---

## 11. Voice & Personality Heuristics

### 11.1 Explanation Density Heuristic

**Signal:**  
- Long, exploratory queries  
- High annotation density  

**Behavior:**  
- Suggest reflective voice  
- Adapt to personality configuration  

**Example:**  
> “I can explain as factual timeline or reflective summary.  
> Switch voice to calm and curious?”

---

### 11.2 Repeated Clarification Heuristic

**Signal:**  
- Repeated similar questions  
- Confusion without missing data  

**Behavior:**  
- Suggest alternative framing  
- Indicate current phase  

**Example:**  
> “It may help to view this decision in context of broader Area history.  
> I can summarize if desired.”

---

## 12. Heuristics and Learning Over Time

Guidance may become richer as more decisions exist because:

- Canon grows  
- Patterns emerge  
- Context accumulates  

This is **accumulation, not adaptation**.  
Heuristics respect phases, personalities, and user overrides.  
Guidance does not “learn preferences”; it **learns history**.

---

## Closing Principle

Heuristics are **lenses, not levers**.

They help humans see:

- Continuity  
- Change  
- Context  
- Uncertainty  

They **must never help systems decide**.  
If a heuristic feels persuasive, it has crossed the line.