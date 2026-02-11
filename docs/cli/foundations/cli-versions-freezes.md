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
Status: PLANNED (LOCKED DIRECTION)

V5 introduces guidance, not authority.

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
- Assumption surfacing
- Summaries of sessions, deliberates, and baseline reviews
- Risk or drift flags for long-running workflows

**V5 explicitly does NOT:**
- Accept resolutions
- Infer consent
- Override authority
- Create legitimacy
- Mutate engine state

**V5 mental model:**  
“Help me understand what I’ve done — not what I should do.”

---

### V6 — Federation, Signals, and Reflective Roll-Up
Status: PLANNED (LOCKED DIRECTION)

V6 introduces federation without control.

**Characteristics / Additions:**
- Multi-area visibility
- Self-reported **check-ins** (signals)
- Status enums describing relationship to intent
- Roll-ups that preserve detail or meaningful summaries
- Git-like metaphors:
  - `push` for signal elevation
  - `fetch` for intent or lineage updates
- Explicit recognition of silence as a valid state
- Area references as connective tissue across layers

**V6 explicitly does NOT:**
- Enforce cadence
- Require updates
- Treat status as performance
- Create urgency
- Interpret intent
- Decide what progress means

**V6 mental model:**  
“Show me how my decisions are holding up — without turning my work into a status report.”

---

### V7 — Online Commit Relay (No Legitimacy)
Status: PLANNED (LOCKED DIRECTION)

V7 introduces a **networked commit relay** for Charter.

**Purpose:**
- Serves as an online append-only store
- Routes commits between users or systems
- Preserves history without interpreting or merging
- Enables VDS/VLS to consume commits externally

**Characteristics / Additions:**
- One commit store per context (workspace)
- Segregates commits by user/system if needed
- Consumes commit exports from V6 or VDS/VLS
- Stores all commits immutably
- Provides external history to consumers
- Baseline reviews and deliberates may consume relay commits
- Optional restoration restores all commits, including local history
- Treats all commits as opaque, append-only events

**V7 explicitly does NOT:**
- Interpret commits
- Declare current state
- Create legitimacy
- Merge histories
- Execute or enforce behavior

**V7 mental model:**  
“Remember everything said, route it faithfully, but never decide.”

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