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

  ### V5 — Commit Store, VDS/VLS Substrate & Alignment Engine

Status: LOCKED DIRECTION  

V5 introduces the structural and observational substrate required to make alignment mechanically computable.

This version integrates:

- Commit storage (truth artifacts)
- VDS (observational signals)
- VLS (identity and lineage structure)
- Alignment Engine (derived computation)

V5 is the first version where **intent, identity, and observed reality become computationally related**.

#### Characteristics

- Append-only commit model  
- Signals as first-class, non-legitimate artifacts  
- Identity-aware lineage and scope modeling  
- Reconstructed DAG across Areas and identities  
- Deterministic alignment computation  
- Derived, queryable alignment state  
- Cross-area visibility without authority  

#### V5 Adds

- Local commit store (per Context)  
- Commit identity (UUID)  
- Commit lineage (append-only, supersedable)  
- Signal and check-in commits (VDS artifacts)  
- Identity-aware aggregation (VLS substrate)  
- Reconstructed distributed DAG (multi-area, multi-identity)  
- Alignment Engine (deterministic, observational computation layer)  
- Alignment State Store (derived, rebuildable, query-oriented)  
- Cross-area visibility without authority  

#### Alignment Engine Role

The Alignment Engine:

- Consumes:
  - reconstructed VLS DAG  
  - signal and check-in commits  
  - identity and area context  
  - deployment and experiment posture  

- Computes:
  - drift, variance, and signal density  
  - semantic lattice states  
  - propagation across lineage  
  - alignment cones and horizons  
  - tension, cascades, and shock behavior  
  - predictive indicators (velocity, acceleration, instability risk)  

- Produces:
  - derived alignment state (per resolution, area, identity, global)  

The Alignment Engine:

- Is deterministic  
- Is rebuildable  
- Is non-authoritative  
- Never creates or modifies legitimacy  

#### Alignment State Model

Alignment state includes:

- Present state (current observed condition)  
- Predictive state (directional trends and risks)  
- Structural state (graph influence and propagation effects)  

All alignment state is:

- Derived from commits and signals  
- Rebuildable from source artifacts  
- Never a source of truth  

#### V5 Guarantees

- Resolution remains the smallest legitimacy unit  
- Legitimacy remains local and session-bound  
- Signals do not create obligation  
- Alignment does not create authority  
- Identity remains governed by VLS invariants  
- All computed state is derived and reproducible  

#### V5 Explicitly Does NOT

- Enforce behavior  
- Execute changes  
- Infer authority  
- Convert signals into obligations  
- Create legitimacy outside sessions  
- Diagnose root cause  
- Replace human judgment  


**V5 mental model:**  
“Truth accumulates, structure becomes visible, and alignment becomes computable.”

---

### V6 — Guidance, Canon, and Exegesis (Read-Only Interpretation Layer)

Status: LOCKED DIRECTION  

V6 introduces a read-only interpretation layer over all stored and derived artifacts.

Guidance in V6 is **exegesis** — an honest, descriptive interpretation of:

- what was committed  
- how identity has evolved  
- what has been observed  
- how alignment is behaving  

V6 does not compute alignment.  
It interprets alignment and related artifacts.

#### Characteristics

- Read-only interpretation layer  
- Canon-based descriptive analysis  
- Alignment-aware explanation  
- Narrative and structural summaries  
- Stateless or ephemeral output  
- No mutation of any underlying system  

#### V6 Consumes

- Commit store (all commit types)  
- Receipts (exploration, review, legitimacy)  
- Signal and check-in artifacts  
- Identity lineage (VLS)  
- Alignment state (Alignment Engine output)  
- Deployment and experiment context  

#### V6 Produces

- Summaries of decisions and evolution  
- Alignment interpretations  
- Drift and instability explanations  
- Tension and conflict descriptions  
- Identity transition narratives  
- Inconsistency and ambiguity highlighting  
- Canon-based descriptive insights  

#### Nature of Exegesis

Guidance may express observations such as:

- persistent misalignment within stable systems  
- tension between competing goals  
- instability during identity transitions  
- alignment recovery after supersession  
- capacity pressure within areas  
- deprecated regions that remain structurally relevant  

All outputs are:

- descriptive  
- non-prescriptive  
- non-authoritative  
- explainable  

#### V6 Guarantees

- Guidance is read-only  
- Guidance does not mutate any state  
- Guidance does not create legitimacy  
- Guidance does not infer consent or authority  
- Guidance does not convert interpretation into obligation  

#### V6 Explicitly Does NOT

- Modify Resolutions  
- Override authority  
- Enforce decisions  
- Execute workflows  
- Create alignment state (delegated to V5)  
- Replace human judgment  

**V6 mental model:**  
“Help me understand what happened, what is happening, and what it appears to mean.”

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