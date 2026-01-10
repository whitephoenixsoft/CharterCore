# Charter Core — Product Brief 

Modern systems lose legitimacy over time.

Decisions are made in meetings, chats, or documents, then silently encoded as flags, configs, or policies. Authority is implied, scope is forgotten, and history is overwritten. Months later, no one can answer why something is allowed — only that it is.

**Charter Core exists to preserve decision legitimacy.**

Charter Core is a minimal governance engine that treats decisions as first-class, immutable records. It explicitly captures:

- What was decided
- Who had authority to decide
- Within what scope the decision applies
- How agreement was reached
- When the decision was accepted, superseded, or retired

Decisions in Charter Core are never implicit and never overwritten. They are accepted through structured sessions and evolve only by supersession, preserving a complete and auditable decision history.

---

## What Charter Core Is

- A decision ledger
- A legitimacy engine
- A governance foundation for other systems

Charter Core is designed to be embedded as a library or accessed through APIs by tools that need reliable decision memory.

--- 

## What Charter Core Is Not

- Not a workflow engine
- Not a policy language
- Not a state store
- Not a chat system
- Not an AI system

Charter Core does not execute actions or interpret meaning.
It records explicit human decisions and preserves their legitimacy over time.

---

## Where Charter Core Is Useful

Charter Core applies wherever authority matters and history should not be rewritten, including:

- Software architecture and technical decisions
- Organizational policies and standards
- Governance and compliance processes
- Configuration and rule changes
- Multiplayer systems and game canon
- Any domain requiring auditability, traceability, and integrity

---

## Charter Core and Responsibility Patterns

Many systems use procedural patterns such as *Chain of Responsibility* to determine who handles an action at runtime. These patterns are effective for execution, routing, and control flow.

Charter Core addresses a different problem.

Charter Core does not route requests or execute decisions. Instead, it records how responsibility was legitimately resolved when a decision became canonical.

Where Chain of Responsibility answers:

>“Who handled this?”

Charter Core answers:

>“Under what authority was this decision accepted?”

This distinction allows Charter Core to coexist with existing architectures without replacing them. Runtime systems may evolve, handlers may change, and roles may disappear — but Charter preserves the legitimacy, authority, and scope that were in force *at the moment of decision*.

Charter Core is therefore not a workflow engine or execution framework. It is a **governance ledger** that complements procedural systems by preserving institutional truth.

---

## The Core Idea

> State answers “what is true.”
> Charter answers “why it is allowed.”

Charter Core provides the missing layer between human agreement and system behavior.

---

## Why This Matters

By separating decision legitimacy from system execution, Charter Core enables systems that are:

- easier to reason about
- safer to evolve
- harder to abuse
- resilient to team and organizational change

Charter Core does not replace existing tools — it stabilizes them.