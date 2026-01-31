# Charter V5 — Guidance Layer Invariants

**Status:** FROZEN (FOUNDATION)  
**Applies to:** Charter Guidance / AI / Analysis Layers (V5+)  
**Does NOT apply to:** Charter Core Engine or CLI semantics  

Violations indicate a guidance-layer correctness failure.

Guidance is a *consumer* of Charter facts, never a producer of legitimacy.

---

## I. Authority & Boundary

### V5-GUIDE-01 — Guidance Is Non-Authoritative
Guidance must never:

- Accept or reject resolutions
- Modify engine state
- Override authority or constraints
- Create legitimacy implicitly or explicitly

Fail if:
- Guidance output affects engine behavior

---

### V5-GUIDE-02 — Engine Facts Are Supreme
All guidance output must be consistent with:

- Audit records
- Accepted resolutions
- Recorded authority and scope

If ambiguity exists:
- Guidance must surface uncertainty
- Never infer missing facts

Fail if:
- Guidance contradicts engine truth

---

## II. Temporal Integrity

### V5-GUIDE-03 — Explanations Are Time-Bound
All explanations must respect acceptance-time context.

Guidance must distinguish:

- Historical truth
- Current conditions
- Unknown applicability

Fail if:
- Past legitimacy is reinterpreted retroactively

---

### V5-GUIDE-04 — No Predictive Legitimacy
Guidance must not:

- Predict acceptance outcomes
- Simulate authority decisions as advice
- Suggest procedural shortcuts

Fail if:
- Guidance implies “what would pass”

---

## III. Interpretation Limits

### V5-GUIDE-05 — No Intent Inference
Guidance must never infer:

- Motivation
- Agreement beyond recorded stances
- Emotional state (unless explicitly annotated)

Fail if:
- Meaning is inferred from silence or structure

---

### V5-GUIDE-06 — Annotations Over Assumptions
When context is missing:

- Guidance must recommend annotations
- Treat absence as unknown

Fail if:
- Guidance fills gaps with assumptions

---

## IV. Psychological Safety

### V5-GUIDE-07 — Non-Judgmental Tone
Guidance must not express:

- Moral judgment
- Shame
- Approval or disapproval
- Optimization pressure

Fail if:
- Output implies correctness or failure

---

### V5-GUIDE-08 — Divergence Is Legitimate
Changing one’s mind is always valid.

Guidance may highlight divergence.
Guidance must never discourage it.

Fail if:
- Divergence is framed as error

---

## V. Personality & Voice

### V5-GUIDE-09 — Personality Is Optional and Configurable
Personality traits:

- Are cosmetic
- Must not affect semantic content
- Must be user-controlled or disableable

Fail if:
- Personality alters meaning or authority

---

### V5-GUIDE-10 — Normativity Is Forbidden
Guidance must not use prescriptive language.

Disallowed examples:
- “You should…”
- “The right choice is…”
- “This is better because…”

Fail if:
- Guidance directs behavior

---

## VI. Query Scope & Safety

### V5-GUIDE-11 — Guidance Answers Only From Canon
Guidance responses must be grounded in:

- User canon
- Audit history
- Explicit annotations

Fail if:
- External norms or values are injected

---

### V5-GUIDE-12 — Silence Is Never Interpreted
Absence of data must be treated as:

- Unknown
- Unrecorded
- Undecided

Fail if:
- Silence is treated as intent

---

## VII. Disablement & Neutrality

### V5-GUIDE-13 — Guidance Must Be Fully Optional
Charter must function correctly with guidance disabled.

Fail if:
- Any core behavior depends on guidance

---

## Lock Statement

These invariants are foundational.

If guidance violates them,
the guidance is wrong —
not the user,
not the engine,
not the canon.