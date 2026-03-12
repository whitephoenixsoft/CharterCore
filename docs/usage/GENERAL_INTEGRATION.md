# Charter Integration Pattern
## Using the Charter Legitimacy Engine in External Systems

Status: CONCEPTUAL  
Audience: System architects, platform developers, AI infrastructure engineers  
Applies to: Applications embedding the Charter Core engine

---

# 1. Purpose

This document describes a conceptual integration pattern for embedding the Charter Core engine inside external systems.

The goal is to provide a **deterministic legitimacy layer** that records when a system moves from:

discussion → evaluation → commitment

Charter does not replace business logic, policy engines, or workflow systems.

Instead, it provides a **structured mechanism for recording legitimate decisions** that govern those systems.

---

# 2. The Core Role of Charter

Charter is not a reasoning engine.

Charter is not a workflow engine.

Charter is not a policy engine.

Charter is a **legitimacy engine**.

It answers a single question:

What decisions were explicitly accepted under visible authority?

External systems remain responsible for:

- application behavior
- business rules
- operational logic
- data processing
- AI reasoning
- workflow orchestration

Charter records **which outcomes become legitimate commitments**.

---

# 3. Conceptual Architecture

A typical integration architecture looks like this:

External System  
↓  
Proposals / Candidates  
↓  
Charter Sessions  
↓  
Accepted Resolutions  
↓  
System Policy / Behavior

The host system generates proposals.

Charter records the process that determines whether those proposals become legitimate decisions.

---

# 4. The Governance Boundary

The key architectural boundary is:

Exploration and reasoning occur outside Charter.  
Commitment occurs inside Charter.

Example lifecycle:

Exploration

Ideas  
Discussions  
AI planning  
Draft policies  

Evaluation

Candidates  
Sessions  
Participant stances  

Commitment

Accepted resolutions  
Immutable legitimacy records  

Charter begins at the moment a proposal is formally evaluated.

---

# 5. Host System Responsibilities

A host system embedding Charter is responsible for:

Generating proposals  
Starting sessions  
Recording participants  
Recording stances  
Triggering acceptance attempts  
Applying accepted resolutions to system behavior

Charter does not execute decisions.

It records that they were legitimately accepted.

---

# 6. Example Integration Pattern

Example: Dynamic system configuration.

A host application proposes a change:

Proposal: Increase API rate limit.

The host system creates a Charter session to evaluate the proposal.

Participants record stances.

When acceptance is attempted:

Charter evaluates the authority rule.

If authority passes:

A resolution is created.

The host system then applies the change to its configuration.

Charter records the legitimacy of the decision.

---

# 7. Using Charter With Autonomous Systems

Charter can serve as a governance layer for systems involving automated agents.

Agents may:

propose options  
analyze consequences  
generate candidate resolutions  

However, legitimacy still requires a session acceptance event.

This prevents systems from silently altering governance rules.

Example pattern:

AI agent proposes a policy change.

A session evaluates the proposal.

Authority rules determine whether acceptance is allowed.

Only accepted outcomes influence system behavior.

---

# 8. Decision Memory vs Information Memory

Most systems store:

logs  
notes  
embeddings  
conversation history  

These forms of memory describe what was discussed.

Charter records something different:

what was decided.

This distinction is critical for systems where behavior must remain consistent over time.

Charter ensures that:

discussion does not equal commitment.

---

# 9. Authority Integration

Authority rules define how decisions are evaluated.

External systems may define authority in many ways, including:

single operator approval  
multi-party approval  
automated safety checks  
organizational governance structures

Charter does not interpret authority semantics.

It only evaluates whether the required stances satisfy the authority rule.

---

# 10. Operational Use Cases

Common integration scenarios include:

Policy management

System rules are proposed and accepted through Charter sessions.

Configuration governance

Critical configuration changes require explicit acceptance.

Operational state changes

Systems entering degraded or emergency modes may record the decision as a resolution.

Multi-agent coordination

Multiple automated systems record consensus through Charter sessions.

Human oversight

Human operators participate in governance sessions when required.

---

# 11. Why Legitimacy Matters

Many systems record outcomes but not the process that produced them.

Without explicit legitimacy records:

decisions appear arbitrary  
history becomes unclear  
authority boundaries erode  
trust in system behavior declines

Charter preserves the legitimacy process.

This makes system behavior explainable.

---

# 12. Separation of Responsibilities

A well-designed integration maintains clear boundaries.

Application

Handles operational behavior and execution.

Policy logic

Defines how accepted resolutions influence system behavior.

Charter

Records whether a decision was legitimately accepted.

This separation ensures that governance does not become entangled with implementation.

---

# 13. Immutability and Auditability

Once accepted, a Charter resolution becomes immutable history.

Corrections occur through supersession.

This guarantees that:

past decisions cannot be rewritten  
changes remain visible  
authority shifts are traceable  

External systems benefit from a stable historical record.

---

# 14. Long-Term Governance

Over time, systems accumulate many decisions.

Charter allows those decisions to form a structured legitimacy history.

New decisions may supersede earlier ones.

However, the path of reasoning and commitment remains visible.

This enables systems to evolve without losing institutional memory.

---

# 15. Minimal Integration Model

At minimum, an integration requires:

Creating sessions  
Submitting candidates  
Recording stances  
Attempting acceptance  
Reading accepted resolutions

Everything else remains under host system control.

---

# 16. Conceptual Outcome

When embedded in external systems, Charter becomes a legitimacy layer that:

records commitments  
preserves authority boundaries  
separates discussion from decision  
ensures deterministic outcomes  

Over time, this allows systems to maintain consistent governance even as participants, policies, or technologies change.

---

# Final Principle

Charter does not decide what is correct.

Charter records when something became legitimate.

This distinction allows it to act as a neutral governance foundation for a wide range of systems.