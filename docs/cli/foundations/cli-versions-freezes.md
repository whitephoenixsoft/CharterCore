# Charter Interfaces — Version Freezes & Evolution Plan (V1–V5)
Status: FROZEN (through V3)  
Scope: Charter interaction layers (CLI → Library → Guidance)  
Does NOT define: engine semantics, authority rules, or audit mechanics

This document records what each version is allowed to care about, and—more importantly—what it is not allowed to do.

Charter evolves by layering, not by mutation.

---

## I. Version Freezes

### V1 — Deterministic Solo Governance
Status: FROZEN

V1 establishes the irreducible core:  
legitimacy without ambiguity, usable by a single human.

**Characteristics:**

- Single-user (solo mode)  
- One active session at a time  
- Explicit authority and scope  
- Deterministic acceptance  
- No orchestration primitives  
- No exploratory workflows  

**V1 knows about:**

- Areas  
- Contexts  
- Sessions  
- Candidates  
- Participants (single explicit user)  
- Authority sets  
- Scope sets  
- Baseline review (import + acceptance)  
- Linear, factual audit  
- **Annotations & Irreversibility metadata** for sessions and resolutions  

**V1 explicitly does NOT know about:**

- Deliberate  
- Breakouts  
- Synthesis  
- Draft candidates  
- Participant groups  
- Parallel work  
- Human workflow modeling  

**V1 mental model:**  
> “I make decisions deliberately, one at a time, and I can safely re-decide imported history.  
> I can annotate and track irreversible choices to preserve clarity and confidence.”

---

### V2 — Multi-Participant Governance & Review
Status: FROZEN

V2 expands who participates and how agreement is computed,  
without changing where legitimacy lives.

**Characteristics:**

- Multiple participants  
- Explicit voting semantics  
- Authority-aware acceptance rules  
- Still session-centric  
- Still decision-first  

**V2 adds:**

- Explicit participant add/remove  
- Voting commands  
- Mechanical authority evaluation  
- Per-resolution baseline acceptance  
- Participant-centric audit queries  

**V2 explicitly does NOT know about:**

- Deliberate  
- Breakouts  
- Synthesis  
- Workshops or facilitation  
- Session bundling or orchestration  

**V2 mental model:**  
> “We decide together, and the rules of agreement are explicit and provable.”

---

### V3 — Deliberative Orchestration & Human Workflow Modeling
Status: FROZEN

V3 is not an extension of sessions.  
It is a **layer above sessions** and treats thinking as first-class.

**Characteristics:**

- Explicit separation of:
  - exploration
  - convergence
  - legitimacy  
- Long-running, human-shaped workflows  
- Sessions become terminal acts, not workspaces  
- Legitimacy pathways unchanged  

**V3 introduces:**

- Deliberate (epic-scoped thinking spaces)  
- Breakouts (bounded exploratory moments)  
- Synthesis (explicit convergence artifacts)  
- Draft candidates (non-authoritative text)  
- Participant groups  
- Deliberate import/export (foreign thinking)  

**V3 guarantees:**

- No action inside Deliberate creates legitimacy  
- All legitimacy still flows through:
  - sessions  
  - baseline review  

**V3 mental model:**  
> “We explore freely, converge intentionally, and only then decide.”

---

## II. Why V3 Is a Structural Break

**V1 / V2 assume:**

- Decisions are the center of activity  
- Thinking happens inside sessions  

**V3 asserts:**

- Thinking happens before sessions  
- Sessions exist to finalize legitimacy, not discover it  

**Key shift:**

- Sessions stop being places where humans think  
- Sessions become places where humans own outcomes  

> This is a cognitive, not technical, transition.

---

### V4 — Charter as a Library (Planned)
Status: PLANNED (LOCKED DIRECTION)

V4 is a delivery transformation, not a conceptual one.  
The CLI becomes:

- a thin client  
- a reference interface  
- one consumer of a Charter interaction library  

**Characteristics:**

- Charter interaction logic extracted into a library  
- CLI rebuilt on top of that library  
- No new governance semantics  
- No new legitimacy paths  

**V4 exists to enable:**

- embedding Charter into other tools  
- simulations and validation harnesses  
- future server processes  
- alternative interfaces (UI, batch, automation)  

**V4 explicitly does NOT add:**

- server mode  
- background daemons  
- AI facilitation  
- automation of decisions  

**V4 mental model:**  
> “Charter is a system you integrate, not just a tool you run.”

---

### V5 — Guidance, Semantics, and Clarification (Planned)
Status: PLANNED (LOCKED DIRECTION)

V5 introduces **guidance, not authority**.

This layer exists to help humans:

- think clearly  
- notice ambiguity  
- detect incoherence  
- understand consequences  

**Characteristics:**

- Read-only analysis of Charter state  
- Canon-based evaluation (e.g., Legitimacy Ledger)  
- AI used for clarification, not control  
- Multiple semantic lenses over the same facts  

**V5 may include:**

- Legitimacy analysis  
- Coherence checks  
- Decision hygiene feedback  
- “What changed?” explanations  
- “What assumptions exist?” surfacing  
- Summaries of sessions, baseline reviews, and deliberates  
- Contextual advice for scope, breakouts, and epic alignment  
- Trend or risk flags for long-running workflows  

**V5 explicitly does NOT:**

- accept resolutions  
- infer consent  
- override authority  
- create legitimacy  
- mutate engine state  

**V5 mental model:**  
> “Help me understand what I’ve done — don’t decide for me.  
> Guide me, summarize, and explain without judgment.”

---

## V. Explicit Non-Goals (Frozen Across All Versions)

Charter will never:

- infer authority  
- assume consensus  
- optimize away explicitness  
- erase disagreement  
- treat silence as intent  
- collapse history into outcomes  

No future version may:

- bypass sessions  
- weaken audit guarantees  
- blur preparation with legitimacy  
- make authority implicit  

---

## VI. Final Freeze Statement

V1, V2, and V3 semantics are fully frozen.  

V4 and V5 may add:

- new delivery mechanisms  
- new guidance layers  
- new analytical perspectives  

They **may not change**:

- what legitimacy means  
- how it is created  
- where it lives  
- how it is proven  

> Charter does not evolve by becoming faster or smarter.  
> It evolves by becoming clearer.