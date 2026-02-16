# Charter Interfaces — Version Freezes & Evolution Plan (V1–V7)

Status: FROZEN (through V3)  
Scope: Charter interaction layers (CLI → Library → Guidance → Federation → Relay)  
Does NOT define: engine semantics, authority rules, audit mechanics, or human behavior

This document records what each version is allowed to care about —  
and, more importantly, what it is never allowed to do.

Charter evolves by layering, not mutation.

---

## I. Version Freezes

### V1 — Deterministic Solo Governance
Status: FROZEN

V1 establishes the irreducible core: legitimacy without ambiguity, usable by a single human.

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
- Annotations and irreversibility metadata
- Spec verification 

**V1 explicitly does NOT know about:**
- Deliberate
- Breakouts
- Synthesis
- Draft candidates
- Participant groups
- Parallel work
- Human workflow modeling

**V1 mental model:**  
“I make decisions deliberately, one at a time.  
I can annotate and track irreversible choices to preserve clarity and confidence.”

---

### V2 — Multi-Participant Governance & Review
Status: FROZEN

V2 expands who participates and how agreement is computed, without changing where legitimacy lives.

**Characteristics:**
- Multiple participants
- Explicit voting semantics
- Authority-aware acceptance rules
- Still session-centric
- Still decision-first

**V2 adds:**
- Participant add/remove
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
“We decide together, and the rules of agreement are explicit and provable.”

---

### V3 — Deliberative Orchestration & Human Workflow Modeling
Status: FROZEN

V3 is not an extension of sessions. It is a layer above sessions.

**Characteristics:**
- Explicit separation of exploration, convergence, and legitimacy
- Long-running, human-shaped workflows
- Sessions become terminal acts, not workspaces
- Legitimacy pathways unchanged

**V3 introduces:**
- Deliberate (epic-scoped thinking spaces)
- Breakouts
- Synthesis artifacts
- Draft candidates (non-authoritative)
- Participant groups
- Deliberate import/export

**V3 guarantees:**
- No action inside Deliberate creates legitimacy
- All legitimacy still flows through sessions and baseline review

**V3 mental model:**  
“We explore freely, converge intentionally, and only then decide.”

---

## II. Why V3 Is a Structural Break

V1 and V2 assume:
- Decisions are the center of activity
- Thinking happens inside sessions

V3 asserts:
- Thinking happens before sessions
- Sessions exist to own outcomes, not discover them

This is a cognitive break, not a technical one.

---

### V4 — Charter as a Library
Status: PLANNED (LOCKED DIRECTION)

V4 is a delivery transformation, not a conceptual one.

**Characteristics:**
- Charter interaction logic extracted into a library
- CLI rebuilt as a thin client
- No new governance semantics
- No new legitimacy paths

**V4 enables:**
- Embedding Charter into other tools
- Simulations and validation harnesses
- Alternative interfaces (UI, batch, automation)

**V4 explicitly does NOT add:**
- Server mode
- Background daemons
- AI facilitation
- Decision automation

**V4 mental model:**  
“Charter is a system you integrate, not just a tool you run.”

---

### V5 — Guidance, Semantics, and Clarification
**Status:** PLANNED (LOCKED DIRECTION)

V5 introduces a **read-only AI guidance layer** that enhances human understanding without creating or altering legitimacy.

**Characteristics:**
- Observes Charter facts without modifying them
- Canon-based evaluation (e.g., Legitimacy Ledger)
- Phase-aware guidance for deliberate workflows
- Personality-aware explanations (tone and style only)
- Summaries of sessions, deliberates, breakouts, and baseline reviews
- Drift, coherence, and assumption surfacing
- Optional and skippable AI assistance

**V5 may include:**
- Legitimacy analysis (descriptive only)
- Coherence checks across sessions or epics
- Decision hygiene feedback
- "What changed?" explanations
- Summaries and highlights for assumptions, annotations, and rationale
- Risk or drift flags for long-running workflows
- Phase announcements and pros/cons on changes
- Voice or personality application for clarity and readability

**V5 explicitly does NOT:**
- Accept, reject, or modify resolutions
- Infer consent, intent, or motivation
- Override authority or scope
- Create legitimacy
- Mutate engine state
- Recommend actions or solutions
- Judge correctness or quality
- Persuade or manipulate

**V5 mental model:**
“Help me understand what I’ve done — not what I should do.  
Show patterns, gaps, and drift, but never decide for me.”

---

### V6 — Federation, Signals, and Reflective Roll-Up

**Features / Capabilities**  

- Support **federation of multiple Charter areas** across teams, individuals, or decision domains  
- Human-initiated, **voluntary check-ins** report relationship to intent, drift, or capacity  
- Check-ins may include **optional annotations and rationale** to provide context  
- Signals use **predefined status enums**:  
  - `Intent Unchanged`  
  - `Capacity Reduced`  
  - `Action Taken`  
  - `Observations Inconclusive`  
  - `Paused Intentionally`  
  - `Reassessment Requested`  
- Silence is **first-class** and auditable if human-entered  
- Aggregation (federation) **summarizes without flattening** context or authority  
- Roll-ups enable **reflection and discussion** at human-chosen cadence (sprints, retros, milestones)  
- Supports **multiple VDS instances** for the same identity for redundancy or resilience  
- Tracks **team rhythms and deployment windows** as descriptive **health signals**, not schedules  
- Distinguishes between:  
  - **Operational decisions** (directly tied to mission outcomes)  
  - **Capacity decisions** (availability, coverage, or resources)  
- All outputs (**check-ins, annotations, silence records**) are **auditable artifacts**: immutable, append-only, and identity-tracked  
- Federation allows **observing inter-team dependencies, overlapping scope, and identity evolution**  
- Escalation, interpretation, or action is **always human-driven**  
- Preserves **agency, autonomy, and non-authoritative observation**

**Explicit Non-Goals / Restrictions**  

- Does **not enforce cadence or schedules**  
- Does **not treat check-ins or signals as performance metrics**  
- Does **not interpret intent or progress**  
- Does **not assign blame, urgency, or correctness**  
- Does **not automate check-ins or silence entries** — all actions are human-initiated  

---

### V7 — Online Commit Relay (No Legitimacy)

**Features / Capabilities**  

- Provides an **online, append-only commit relay** for Charter CLI  
- Stores **all commit types** identically:  
  - Resolution commits  
  - Deliberate commits  
  - Baseline review commits  
  - Import commits  
  - Annotation commits  
- Segregates commits by **context workspace** (user, team, system)  
- Commits are **immutable, append-only, and opaque**; no interpretation occurs  
- Supports **manual push/fetch operations** via CLI (similar to git)  
- Preserves **complete history of local and imported commits**  
- Enables **backup and restore of entire commit history**, in one or multiple transactions; restored commits remain foreign  
- Supports **multi-consumer access**: VDS and VLS may fetch commits for observation  
- CLI-based relay allows **manual collaboration without automating decisions**  
- Author identity is **optional and can be anonymous or alias-based**  
- Supports **local reconstruction of full history** while keeping legitimacy local to the consuming system  

**Explicit Non-Goals / Restrictions**  

- Does **not merge histories**  
- Does **not assign legitimacy**  
- Does **not enforce or declare current state**  
- Does **not interpret, summarize, or analyze commits**  
- Does **not create urgency, pressure, or authority**

---

## V. Explicit Non-Goals (Frozen Across All Versions)

Charter will never:
- Infer authority
- Assume consensus
- Optimize away explicitness
- Erase disagreement
- Treat silence as intent
- Collapse history into outcomes
- Convert observation into obligation

No future version may:
- Bypass sessions
- Weaken audit guarantees
- Blur preparation with legitimacy
- Make authority implicit

---

## VI. Final Freeze Statement

V1, V2, and V3 semantics are fully frozen.

V4, V5, V6, and V7 may add:
- Delivery mechanisms
- Guidance layers
- Federation visibility
- Online relay capability
- Analytical perspectives

They may not change:
- What legitimacy means
- How it is created
- Where it lives
- How it is proven

Charter does not evolve by becoming faster or smarter.  
It evolves by becoming clearer, more humane, and harder to misuse.