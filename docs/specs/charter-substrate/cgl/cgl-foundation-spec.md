# Charter Guidance Layer (CGL) — Foundation Specification

Status: FOUNDATIONAL  
Intent: Provide read-only exegesis over Charter substrates  
Scope: Explanation, reflection, synthesis support, and cross-substrate interpretation  
Depends On: CQL, CCS, CSG, CIS, CAS, CCare, CDS, Runtime  
Does NOT Define: legitimacy, alignment computation, identity semantics, workflow execution, or authority  

---

# 1. Purpose

The Charter Guidance Layer (CGL) provides a unified, read-only system for explaining Charter state.

It exists to:

- explain recorded artifacts across all substrates  
- surface gaps, inconsistencies, and ambiguity  
- support reflection, synthesis, and understanding  
- provide cross-substrate interpretation without coupling  

CGL is:

- non-authoritative  
- read-only  
- deterministic (given same inputs)  
- optional  

---

# 2. Core Principle

> CGL explains Charter state.  
> It does not decide, enforce, or create meaning beyond recorded facts.

CGL must:

- operate only on queryable state  
- remain subordinate to all substrates  
- treat absence as unknown  
- preserve ambiguity  

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
- runtime artifacts (sessions, reviews)

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

- session receipts
- review receipts
- deliberate closure artifacts
- supersession chains

---

# 5. Core Function: Exegesis

CGL performs **exegesis**, not reasoning.

Exegesis includes:

- explanation of structure
- summarization of history
- surfacing divergence
- highlighting absence
- contextual framing

CGL must not:

- infer intent  
- evaluate correctness  
- recommend actions  

---

# 6. Phases

CGL operates in structured phases.

---

## 6.1 Phase Model

- Expansion  
- Structuring  
- Divergence Highlight  
- Synthesis  
- Temporal Reflection  
- Assumption Tracking  

---

## 6.2 Phase Behavior

Phases:

- affect presentation, not truth  
- may be system-assigned or user-selected  
- must be explicitly announced  

---

## 6.3 Phase Constraints

- Expansion allows ambiguity  
- Divergence normalizes conflict  
- Synthesis prepares for review (without deciding)  
- Temporal preserves acceptance-time truth  

---

# 7. Heuristics

Heuristics define **how explanations are presented**, not what is true.

---

## 7.1 Heuristic Properties

Heuristics:

- operate on existing data  
- are optional and overridable  
- must not affect legitimacy  
- must not infer intent  

---

## 7.2 Heuristic Categories

- structural  
- temporal  
- consistency  
- annotation-aware  
- ambiguity-aware  
- query-intent  
- domain framing  
- voice  

---

## 7.3 Heuristic Principle

> Heuristics are lenses, not levers.

---

# 8. Cross-Substrate Interpretation

CGL may combine data across domains.

Examples:

- CDS + CAS → “alignment of thinking”  
- CCare + CSG → “signals over structure”  
- CIS + CAS → “alignment across identities”  
- Runtime + CDS → “decision evolution”  

CGL must not:

- merge semantics  
- create new relationships  
- infer missing structure  

---

# 9. Next-Step Guidance

CGL may surface:

- available actions  
- workflow affordances  

These must be:

- non-prescriptive  
- optional  
- non-authoritative  

---

# 10. Personality & Voice

Personality:

- affects tone only  
- must not affect meaning  
- must be optional  

---

# 11. Invariants

CGL must never:

- create legitimacy  
- modify state  
- infer intent  
- recommend decisions  
- override authority  
- interpret silence as meaning  

All explanations must:

- be time-bound  
- respect recorded facts  
- preserve ambiguity  

---

# 12. Mental Model

CGL is:

- a mirror over Charter  
- a narrator of recorded history  
- a cross-substrate explainer  

It is not:

- a decision system  
- an optimizer  
- an authority  
- a reasoning engine  

---

# 13. Final Principle

CGL exists so that:

- users can understand their system  
- systems can explain themselves  
- history remains visible without distortion  

CGL makes Charter legible  
without ever changing what Charter is.