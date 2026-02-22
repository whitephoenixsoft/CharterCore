# Charter Interfaces — Version Freezes & Evolution Plan (V1–V7)

Status: FROZEN (V1–V3 Semantics Locked)  
Scope: Charter interaction, embedding, storage, and transport layers (Engine → CLI Runtime → Libraries → Guidance → Federation → Relay)  
Does NOT define: engine internals beyond frozen semantics, human behavior, or external infrastructure policy  

This document records what each version is allowed to care about —  
and, more importantly, what it is never allowed to do.

Charter evolves by layering, not mutation.  
Resolution remains the smallest legitimacy unit across all versions.

The CLI is an application runtime and library from V1 forward.  
The Engine remains minimal and legitimacy-primitive only.

---

## I. Version Freezes

### V1 — Deterministic Solo Governance (Library-First Runtime)  
Status: FROZEN  

V1 establishes the irreducible core: legitimacy without ambiguity, usable by a single human.

V1 is implemented as:

- A minimal Engine library (legitimacy primitives only)  
- A CLI Runtime library (storage, context, orchestration)  
- A thin executable wrapper over the CLI library  

#### Characteristics

- Single-user (solo mode)  
- One active session at a time (per Area)  
- Explicit authority and scope  
- Deterministic acceptance  
- Linear, factual audit  
- Hard context isolation  
- No orchestration primitives  
- No exploratory workflows  

#### V1 Structural Model

- Engine: legitimacy primitives only  
- CLI Runtime: Areas, Contexts, storage, baseline review orchestration  
- Context: hard storage isolation boundary  
- Area: legitimacy boundary  

#### V1 Knows About

- Areas (CLI concern, legitimacy boundary)  
- Contexts (CLI concern, storage isolation boundary)  
- Sessions (engine primitive)  
- Candidates  
- Participants (single explicit user)  
- Authority sets  
- Scope sets  
- Baseline review (import + acceptance gate)  
- Resolutions (smallest legitimacy unit)  
- Annotations and irreversibility metadata  
- Spec verification (embedded spec identity)  

#### V1 Legitimacy Model (Frozen)

- Legitimacy is created only through accepted Resolutions  
- A Resolution is the smallest legitimacy unit  
- Sessions produce Resolutions  
- Audit records what occurred, not intent  
- Engine is unaware of Context  

#### V1 Explicitly Does NOT Know About

- Deliberate  
- Breakouts  
- Synthesis  
- Draft candidates  
- Participant groups  
- Parallel work  
- Workflow modeling  
- Federation  
- Relay  

**V1 mental model:**  
“I make decisions deliberately, one at a time. I can annotate and track irreversible choices to preserve clarity and confidence.”

---

### V2 — Multi-Participant Governance & Mechanical Authority  
Status: FROZEN  

V2 expands who participates and how agreement is computed, without changing where legitimacy lives.

The Engine remains legitimacy-only.  
The CLI Runtime orchestrates multi-participant interaction.

#### Characteristics

- Multiple participants  
- Explicit voting semantics  
- Authority-aware acceptance rules  
- Session-centric  
- Decision-first  

#### V2 Adds

- Participant add/remove  
- Voting commands  
- Mechanical authority evaluation  
- Per-resolution baseline acceptance  
- Participant-centric audit queries  

#### V2 Guarantees

- Resolution remains the smallest legitimacy unit  
- Legitimacy is computed mechanically and deterministically  
- No implicit authority inference  
- No authority created outside sessions  

#### V2 Explicitly Does NOT Know About

- Deliberate  
- Breakouts  
- Synthesis  
- Workshops or facilitation  
- Session bundling or orchestration  
- Federation  
- Relay  

**V2 mental model:**  
“We decide together, and the rules of agreement are explicit and provable.”

---

### V3 — Deliberative Orchestration & Workflow Modeling  
Status: FROZEN  

V3 is layered above sessions. It does not alter legitimacy semantics.

V3 introduces non-legitimate workflow processing that culminates in sessions and baseline review.

#### Characteristics

- Explicit separation of exploration, convergence, and legitimacy  
- Long-running, human-shaped workflows  
- Sessions become terminal acts, not workspaces  
- Legitimacy pathways unchanged  

#### V3 Introduces

- Deliberate (bounded exploratory workspace)  
- Breakouts  
- Synthesis artifacts  
- Draft candidates (non-authoritative)  
- Participant groups  
- Generalized Baseline Review (gatekeeper across boundaries)  
- Deliberate import/export  

Baseline Review becomes:

- The structured boundary for importing artifacts  
- The gatekeeper for cross-boundary integration  
- The only path from non-legitimate workflow into legitimacy  

#### Receipts in V3

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

#### V3 Guarantees

- No action inside Deliberate creates legitimacy  
- All legitimacy flows through sessions and baseline review  
- Resolution remains the smallest legitimacy unit  

**V3 mental model:**  
“We explore freely, converge intentionally, and only then decide.”

---

## II. Structural Boundary (V1–V3)

V1–V2 assume:

- Decisions are the center of activity  
- Thinking happens inside sessions  

V3 asserts:

- Thinking happens before sessions  
- Sessions exist to own outcomes, not discover them  

This is a cognitive boundary, not a legitimacy mutation.

Engine legitimacy semantics remain frozen.

---

## III. Delivery, Embedding & Language Exposure

### V4 — Multi-Language Embedding & Stable Runtime Surface  
Status: LOCKED DIRECTION  

V4 formalizes Charter as an embeddable runtime platform.

The CLI Runtime is already a library.  
V4 stabilizes cross-language exposure.

#### Characteristics

- Stable Engine library boundary  
- Stable CLI Runtime library boundary  
- C ABI exposure (durable systems embedding)  
- Python binding (testing, AI integration, web orchestration)  
- .NET binding (enterprise and game integration)  
- Thin CLI executable remains optional wrapper  

#### V4 Does NOT Add

- New legitimacy paths  
- Server mode by default  
- Decision automation  
- AI facilitation  
- Background mutation services  

**Mental model:**  
“Charter is a legitimacy runtime platform, embeddable across ecosystems.”

---

### V5 — Guidance, Canon, and Clarification (Read-Only)  
Status: LOCKED DIRECTION  

V5 introduces read-only guidance layered above factual state.

Guidance may consume:

- Engine state  
- CLI runtime artifacts  
- Receipts  
- Commits  

But must never mutate legitimacy.

#### Characteristics

- Canon-based descriptive analysis  
- Drift detection  
- Summaries  
- Inconsistency highlighting  
- Ephemeral output  
- Stateless operation  

#### V5 Explicitly Does NOT

- Create or modify Resolutions  
- Infer consent  
- Override authority  
- Mutate engine state  
- Produce legitimacy  

**Mental model:**  
“Help me understand what happened, not what I should do.”

---

### V6 — Commit Store & Reflective Federation  

V6 introduces the commit abstraction and local append-only commit stores.

This version absorbs lessons from Value Driven Systems (VDS/VLS) into Charter infrastructure.

Commits become:

- Append-only truth artifacts  
- Independent units  
- Auditable  
- Supersedable, never deleted  

Resolution becomes a subset of Commit.  
Resolution remains the smallest legitimacy unit.

#### V6 Adds

- Local commit store (per Context)  
- Commit identity (UUID)  
- Commit lineage  
- Reflective roll-ups  
- Signals and check-ins (non-legitimate)  
- Identity-aware aggregation  
- Cross-area visibility without authority  

#### Audit Model

Commits:

- Are immutable  
- Are append-only  
- Do not synthesize legitimacy  
- May reference Areas  
- May be baseline reviewed into legitimacy  

Federation roll-ups:

- Aggregate artifacts  
- Must not derive authority  
- Must not convert signals into obligations  

#### Explicit Non-Goals

- No enforcement  
- No automated performance metrics  
- No authority synthesis  
- No legitimacy outside sessions  

**Mental model:**  
“Truth artifacts accumulate; legitimacy remains local.”

---

### V7 — Commit Relay & Archival Transport (Append-Only, No State Reconstruction)

V7 introduces a transport and archival layer for commits.

The relay:

- Stores commits immutably  
- Applies structural filtering and timestamping  
- Does not build canonical state  
- Does not merge histories  
- Does not compute legitimacy  

Every commit is independent.

#### Characteristics

- CLI-invoked push/fetch operations  
- Dedicated relay CLI installations permitted  
- Append-only commit storage  
- Immutable archival model  
- Structural filtering  
- Timestamp preservation  

#### Commit Model

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
- Not used to reconstruct authoritative state  

External commits:

- Must be ingested into a local commit store  
- Must pass baseline review before affecting legitimacy  

#### V7 Guarantees

- Relay does not compute authority  
- Relay does not synthesize legitimacy  
- Relay does not reconstruct canonical state  
- Legitimacy remains local to consuming runtimes  

**Mental model:**  
“The relay remembers what was recorded, not what it means.”

---

## IV. Context & Isolation Across Versions

Across all versions:

- Context is a hard storage isolation boundary  
- Area is a legitimacy boundary  
- Engine is unaware of Context  
- CLI Runtime enforces isolation  
- Commit stores are per Context  
- Cross-context movement is explicit and auditable  

No version may collapse:

Context isolation into legitimacy  
or  
Legitimacy into transport

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
- Allow receipts to replace Resolutions  
- Allow transport layers to derive legitimacy  

No future version may:

- Bypass sessions  
- Weaken audit guarantees  
- Blur preparation with legitimacy  
- Make authority implicit  
- Allow relay or federation to compute authority  
- Allow language bindings to alter semantics  

---

## VI. Final Freeze Statement

V1–V3 legitimacy semantics are fully frozen.

Resolution remains the smallest legitimacy unit.  
Receipts document lifecycle boundaries but do not create authority.  
Commits generalize truth storage but do not redefine legitimacy.

V4–V7 may add:

- Embedding and language exposure  
- Guidance layers  
- Commit storage  
- Federation visibility  
- Transport and archival capability  

They may not change:

- What legitimacy means  
- How it is created  
- Where it lives  
- How it is proven  

Charter evolves by layering isolation, embedding, visibility, and transport —  
never by mutating legitimacy.