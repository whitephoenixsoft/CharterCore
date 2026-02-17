# Charter Interfaces — Version Freezes & Evolution Plan (V1–V7)

Status: FROZEN (V1–V3 Semantics Locked)  
Scope: Charter interaction and transport layers (CLI → Library → Guidance → Federation → Relay)  
Does NOT define: engine internals beyond frozen semantics, human behavior, or external infrastructure policy  

This document records what each version is allowed to care about —  
and, more importantly, what it is never allowed to do.

Charter evolves by layering, not mutation.  
Resolution remains the smallest legitimacy unit across all versions.

---

# I. Version Freezes

## V1 — Deterministic Solo Governance  
Status: FROZEN  

V1 establishes the irreducible core: legitimacy without ambiguity, usable by a single human.

### Characteristics

- Single-user (solo mode)  
- One active session at a time  
- Explicit authority and scope  
- Deterministic acceptance  
- Linear, factual audit  
- No orchestration primitives  
- No exploratory workflows  

### V1 Knows About

- Areas  
- Contexts  
- Sessions  
- Candidates  
- Participants (single explicit user)  
- Authority sets  
- Scope sets  
- Baseline review (import + acceptance)  
- Resolutions (smallest legitimacy unit)  
- Annotations and irreversibility metadata  
- Spec verification  

### V1 Legitimacy Model (Frozen)

- Legitimacy is created only through accepted Resolutions  
- A Resolution is the smallest legitimacy unit  
- Sessions produce Resolutions  
- Audit records what occurred, not intent  

### V1 Explicitly Does NOT Know About

- Deliberate  
- Breakouts  
- Synthesis  
- Draft candidates  
- Participant groups  
- Parallel work  
- Workflow modeling  

**V1 mental model:**  
“I make decisions deliberately, one at a time. I can annotate and track irreversible choices to preserve clarity and confidence.”

---

## V2 — Multi-Participant Governance & Review  
Status: FROZEN  

V2 expands who participates and how agreement is computed, without changing where legitimacy lives.

### Characteristics

- Multiple participants  
- Explicit voting semantics  
- Authority-aware acceptance rules  
- Session-centric  
- Decision-first  

### V2 Adds

- Participant add/remove  
- Voting commands  
- Mechanical authority evaluation  
- Per-resolution baseline acceptance  
- Participant-centric audit queries  

### V2 Guarantees

- Resolution remains the smallest legitimacy unit  
- Legitimacy is computed mechanically and deterministically  
- No implicit authority inference  

### V2 Explicitly Does NOT Know About

- Deliberate  
- Breakouts  
- Synthesis  
- Workshops or facilitation  
- Session bundling or orchestration  

**V2 mental model:**  
“We decide together, and the rules of agreement are explicit and provable.”

---

## V3 — Deliberative Orchestration & Human Workflow Modeling  
Status: FROZEN  

V3 is layered above sessions. It does not alter legitimacy semantics.

### Characteristics

- Explicit separation of exploration, convergence, and legitimacy  
- Long-running, human-shaped workflows  
- Sessions become terminal acts, not workspaces  
- Legitimacy pathways unchanged  

### V3 Introduces

- Deliberate (bounded exploratory workspace)  
- Breakouts  
- Synthesis artifacts  
- Draft candidates (non-authoritative)  
- Participant groups  
- Deliberate import/export  

### Receipts in V3

V3 introduces lifecycle Receipts as immutable audit artifacts:

- Exploration Receipts (deliberate or breakout closure)  
- Review Receipts (baseline review closure)  
- Legitimacy Receipts (session closure producing resolutions)  

Receipts:

- Are first-class audit artifacts  
- Are immutable and append-only  
- Do not create legitimacy  
- Do not replace Resolutions  
- Document lifecycle boundaries only  

### V3 Guarantees

- No action inside Deliberate creates legitimacy  
- All legitimacy flows through sessions and baseline review  
- Resolution remains the smallest legitimacy unit  

**V3 mental model:**  
“We explore freely, converge intentionally, and only then decide.”

---

# II. Structural Boundary (V1–V3)

V1 and V2 assume:
- Decisions are the center of activity  
- Thinking happens inside sessions  

V3 asserts:
- Thinking happens before sessions  
- Sessions exist to own outcomes, not discover them  

This is a cognitive boundary, not a legitimacy mutation.

---

# III. Delivery & Interface Layers (Post-Freeze)

Versions V4–V7 may extend delivery, visibility, and transport.  
They may not alter legitimacy semantics.

---

## V4 — Charter as a Library  
Status: LOCKED DIRECTION  

V4 is a delivery transformation, not a governance change.

### Characteristics

- Charter logic extracted into a reusable library  
- CLI becomes a thin client  
- No new legitimacy paths  
- No semantic mutation  

### V4 Does NOT Add

- Server mode  
- Background daemons  
- Decision automation  
- AI facilitation  

**Mental model:**  
“Charter is an embeddable system, not just a CLI.”

---

## V5 — Guidance, Semantics, and Clarification  
Status: LOCKED DIRECTION  

V5 introduces read-only AI guidance.

### Characteristics

- Observes facts without mutating them  
- Canon-based descriptive analysis  
- Summaries and drift detection  
- Optional and skippable  

### V5 Explicitly Does NOT

- Create or modify Resolutions  
- Infer consent  
- Override authority  
- Mutate engine state  
- Produce legitimacy  

**Mental model:**  
“Help me understand what happened, not what I should do.”

---

## V6 — Federation, Signals, and Reflective Roll-Up  

V6 introduces cross-area visibility without authority.

### Capabilities

- Voluntary, human-initiated check-ins  
- Predefined signal enums  
- Silence as first-class artifact  
- Context-preserving aggregation  
- Reflective roll-ups  
- Identity-aware federation  

### Audit Model

All check-ins, annotations, silence entries, and signal emissions:

- Are immutable  
- Are append-only  
- May emit Exploration Receipts  
- Do not create legitimacy  

Federation roll-ups:

- Aggregate artifacts and receipts  
- Must not synthesize legitimacy  
- Must not convert signals into obligations  

### Explicit Non-Goals

- No enforcement  
- No performance metrics  
- No urgency assignment  
- No automated interpretation  

---

## V7 — CLI Commit Relay (Append-Only, No Legitimacy)

V7 introduces a transport and archival layer.

### Characteristics

- CLI-invoked push/fetch operations  
- Configurable relay endpoints  
- Append-only commit storage  
- Immutable archival model  
- No hosted service requirement  

### Commit Model

V7 stores all commit types identically, including:

- Resolution commits  
- Legitimacy Receipt commits  
- Exploration Receipt commits  
- Review Receipt commits  
- Deliberate commits  
- Baseline review commits  
- Import commits  
- Annotation commits  
- Signal and check-in commits  

Commits are:

- Opaque to the relay  
- UUID-identified  
- Immutable  
- Not interpreted  
- Not merged  

### V7 Guarantees

- Relay does not compute authority  
- Relay does not synthesize legitimacy  
- Relay does not reconstruct canonical state  
- Receipt lineage is preserved across push/fetch  
- Legitimacy remains local to consuming engines  

### Explicit Non-Goals

- No history merging  
- No authority evaluation  
- No state declaration  
- No decision automation  

**Mental model:**  
“The relay remembers what was recorded, not what it means.”

---

# IV. Explicit Non-Goals (Frozen Across All Versions)

Charter will never:

- Infer authority  
- Assume consensus  
- Optimize away explicitness  
- Erase disagreement  
- Treat silence as intent  
- Collapse history into outcomes  
- Convert observation into obligation  
- Allow receipts to replace Resolutions  

No future version may:

- Bypass sessions  
- Weaken audit guarantees  
- Blur preparation with legitimacy  
- Make authority implicit  
- Allow transport layers to derive legitimacy  

---

# V. Final Freeze Statement

V1–V3 legitimacy semantics are fully frozen.

Resolution remains the smallest legitimacy unit.  
Receipts document lifecycle boundaries but do not create authority.

V4–V7 may add:

- Delivery mechanisms  
- Guidance layers  
- Federation visibility  
- Transport and archival capability  

They may not change:

- What legitimacy means  
- How it is created  
- Where it lives  
- How it is proven  

Charter evolves by layering visibility and transport —  
not by mutating legitimacy.