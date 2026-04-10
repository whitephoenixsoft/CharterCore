# Session Constraints

## 1. What Session Constraints Are (Clean Definition)

Session constraints are temporary, explicit acceptance prerequisites scoped to a single session round.

They answer this question:

> “What conditions must be satisfied for a candidate to be eligible for acceptance before authority rules are applied?”

Key properties:

- Declared during PRE_STANCE
- Mutable only during PRE_STANCE
- Frozen at the first recorded stance
- Apply only to the current session round
- Are purely mechanical and deterministic
- Prevent acceptance eligibility when unsatisfied
- Do not grant or compute acceptance power

This is important:  
**Constraints gate acceptance; they do not decide acceptance.**

---

## 2. Authority vs Session Constraints (The Crucial Distinction)

This distinction must remain strict.

### Authority

Authority:

- Decides how acceptance is computed
- Is an Area-level, first-class Resolution
- Is persistent and contextual
- Defines standing and decision rules

Examples:

- SOLE_ACTOR
- UNANIMOUS_PRESENT
- MAJORITY_PRESENT

---

### Session Constraints

Session constraints:

- Determine whether acceptance is allowed to proceed
- Are session-local and round-scoped
- Do not compute acceptance
- Do not override authority

Examples of valid constraints:

- “A designated participant must cast an ACCEPT vote”
- “A specific participant must be present in the session”
- “All participants must have recorded a stance”

Notice what’s intentionally absent:

- No voting weights  
- No veto semantics  
- No role inheritance  
- No delegation  
- No permanent power  

Constraints remain strictly mechanical gating conditions.

---

## 3. Determinism Requirement

Session constraints must be fully deterministic.

They must:

- be evaluable entirely from session state
- depend only on Engine-managed data
- produce identical results given identical inputs

They must not:

- depend on external systems
- depend on time or timing windows
- depend on human interpretation
- require off-chain confirmation

If a condition cannot be evaluated mechanically by the Engine, it is not a valid constraint.

---

## 4. Relationship to Decision Evaluation

Constraints are evaluated as part of decision evaluation before authority rules are applied.

- If any constraint fails:
  - the candidate is not eligible for acceptance
  - acceptance eligibility fails deterministically

Constraints do not:

- block the session at the state level
- mutate session state
- influence vote counting or authority rules

They strictly gate whether acceptance can proceed.

---

## 5. What Session Constraints Are Not

Session constraints are not:

- Roles  
- Veto powers  
- Delegation mechanisms  
- Approval workflows  
- Escalation paths  
- Governance inheritance  

They do not:

- grant authority  
- redefine authority  
- introduce new decision logic  
- persist beyond the session round  

They exist only to define explicit, mechanical prerequisites for acceptance within a bounded context.

---

## 6. Mental Model

Authority answers:

> “Given the votes, does this pass?”

Constraints answer:

> “Are we even allowed to attempt acceptance?”

They operate in sequence:

1. Constraints determine eligibility  
2. Authority determines outcome  

Neither replaces the other.