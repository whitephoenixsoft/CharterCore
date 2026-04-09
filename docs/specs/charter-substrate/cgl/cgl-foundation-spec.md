# Charter Guidance Layer (CGL) — Foundation Specification (Revised)

Status: FOUNDATIONAL  
Intent: Provide read-only exegesis and optional assistance over Charter substrates  
Scope: Explanation, reflection, comparative analysis, synthesis support, and optional user-invoked guidance  
Depends On: CQL, CCS, CSG, CIS, CAS, CCare, CDS, Runtime  
Does NOT Define: legitimacy, alignment computation, identity semantics, workflow execution, or authority  

---

# 1. Purpose

The Charter Guidance Layer (CGL) provides a unified, read-only system for explaining and exploring Charter state.

It exists to:

- explain recorded artifacts across all substrates  
- surface gaps, inconsistencies, and ambiguity  
- help users understand differences, nuance, and conflict  
- support reflection and synthesis  
- optionally assist users in navigating complex systems  

CGL is:

- non-authoritative  
- read-only  
- deterministic (given identical inputs)  
- explicitly invoked when providing assistance  

CGL makes Charter legible, without changing what Charter is.

---

# 2. Core Principle

CGL explains and assists understanding of Charter state.  
It does not decide, enforce, or create authority.

CGL must:

- operate only on queryable state  
- remain subordinate to all substrates  
- treat absence as unknown  
- preserve ambiguity  
- never collapse multiple valid interpretations into a single truth  

---

# 3. Architectural Position

CGL operates above all substrates via CQL.

Flow:

Substrates → CQL → CGL → User

CGL:

- does not access stores directly  
- does not mutate state  
- does not introduce new truth  

---

# 4. Inputs

CGL consumes only:

## 4.1 CQL Queries

All data access must occur through CQL:

- structural queries (CSG)  
- identity queries (CIS)  
- alignment queries (CAS)  
- deliberate queries (CDS)  
- care signals (CCare)  
- runtime artifacts (sessions, reviews, receipts)  

---

## 4.2 Annotations

Annotations are primary meaning sources:

- rationale  
- assumptions  
- context  
- external references  

CGL must prefer annotations over inference.

---

## 4.3 Receipts & History

Includes:

- legitimacy receipts  
- review receipts  
- reconciliation artifacts  
- derivation lineage  
- commit history  

---

# 5. Core Function: Exegesis

CGL performs exegesis, not reasoning.

Exegesis includes:

- explanation of structure  
- summarization of history  
- surfacing divergence  
- highlighting absence  
- contextual framing  

CGL must not:

- infer intent  
- evaluate correctness  
- recommend decisions as authoritative outcomes  

---

# 6. Assistance Model

CGL supports optional assistance capabilities layered on top of exegesis.

These capabilities are:

- explicitly invoked  
- non-authoritative  
- non-binding  
- clearly distinguishable from system state  

---

## 6.1 Assistance Principle

Assistance helps users understand and navigate the system.  
It must never alter structure, legitimacy, or truth.

---

## 6.2 Assistance Modes

### A. Presentation Assistance

Transforms system outputs into more human-readable forms.

Examples:

- simplifying CLI output  
- summarizing complex data  
- rephrasing technical content  

Properties:

- preserves meaning  
- does not alter structure  
- does not introduce interpretation beyond formatting  

---

### B. Comparative & Analytical Assistance

Surfaces differences, conflicts, and nuance across artifacts, regardless of substrate.

Examples:

- distinguishing similar resolutions  
- identifying semantic conflicts across sessions  
- comparing CDS Items with subtle differences  
- highlighting divergence in assumptions or scope  
- identifying overlapping or duplicate structures  

Applies across:

- Legitimacy (resolutions, sessions)  
- CDS (items, investigations)  
- Review workflows (candidates, imports)  
- Cross-substrate relationships  

Capabilities:

- similarity detection  
- contrast explanation  
- semantic differentiation  
- conflict surfacing  
- ambiguity highlighting  

Key Property:

This mode helps users see distinctions and tensions, not resolve them.

Constraints:

- must not determine correctness  
- must not rank alternatives  
- must not collapse multiple artifacts into one interpretation  

---

### C. Suggestive Guidance

Provides optional, non-binding next-step ideas.

Examples:

- “These items may be duplicates worth reviewing together”  
- “This structure could be decomposed further”  
- “There may be missing relationships between these areas”  

Constraints:

- must be clearly labeled as suggestions  
- must not imply obligation  
- must not override user intent  
- must not create legitimacy  

---

### D. Interactive Consultation

User explicitly invokes assistance.

Examples:

- “Explain what’s happening here”  
- “What are the differences between these?”  
- “Where are the gaps?”  
- “What might I be missing?”  

Properties:

- opt-in only  
- scoped to user query  
- produces bounded responses  

---

## 6.3 Non-Authority Guarantee

All assistance must:

- not create legitimacy  
- not modify artifacts  
- not alter alignment state  
- not introduce structural relationships  

---

## 6.4 Visibility Requirement

Assistance output must be clearly distinguishable from:

- canonical data  
- structural truth  
- recorded artifacts  

---

## 6.5 Failure Mode

If assistance is removed:

- system correctness remains intact  
- only usability is reduced  

---

# 7. Phases

CGL operates in structured phases.

---

## 7.1 Phase Model

- Expansion  
- Structuring  
- Divergence Highlight  
- Synthesis  
- Temporal Reflection  
- Assumption Tracking  

---

## 7.2 Phase Behavior

Phases:

- affect presentation, not truth  
- may be system-assigned or user-selected  
- must be explicitly identifiable  

---

## 7.3 Phase Constraints

- Expansion allows ambiguity  
- Divergence normalizes conflict  
- Synthesis prepares understanding (not decisions)  
- Temporal preserves acceptance-time truth  

---

# 8. Heuristics

Heuristics define how explanations are presented.

---

## 8.1 Heuristic Properties

Heuristics:

- operate on existing data  
- are optional and overridable  
- must not affect legitimacy  
- must not infer intent  

---

## 8.2 Heuristic Categories

- structural  
- temporal  
- consistency  
- annotation-aware  
- ambiguity-aware  
- comparative  
- query-intent  
- domain framing  
- voice  

---

## 8.3 Heuristic Principle

Heuristics are lenses, not levers.

---

# 9. Cross-Substrate Interpretation

CGL may combine data across domains.

Examples:

- CDS + CAS → alignment of investigation  
- CCare + CSG → signals over structure  
- CIS + CAS → identity-level alignment  
- Runtime + CDS → evolution of decisions  

CGL must not:

- merge semantics  
- create new relationships  
- infer missing structure  

---

# 10. Next-Step Guidance

CGL may surface available actions and affordances.

These must be:

- optional  
- non-prescriptive  
- non-authoritative  

---

# 11. Personality & Voice

Personality:

- affects tone only  
- must not affect meaning  
- must be optional  

---

# 12. Invariants

CGL must never:

- create legitimacy  
- modify state  
- infer intent  
- enforce decisions  
- override authority  
- interpret silence as definitive meaning  

All explanations must:

- be time-bound  
- respect recorded facts  
- preserve ambiguity  
- allow multiple interpretations  

---

# 13. Mental Model

CGL is:

- a reflective layer over Charter  
- a narrator of recorded structure and history  
- an optional assistant for understanding complexity  

It is not:

- a decision system  
- an optimizer  
- an authority  
- a controller  

---

# 14. Final Principle

CGL exists so that:

- users can understand their system  
- complexity becomes navigable  
- differences and tensions become visible  
- reflection is supported without pressure  

It makes Charter:

understandable without judgment,  
explorable without risk,  
and usable without losing truth.