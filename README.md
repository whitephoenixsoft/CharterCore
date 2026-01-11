# Charter Core

Charter Core is a deterministic governance engine for recording, evolving, and auditing decisions made under explicit authority and scope.

It is designed to preserve institutional integrity by making decisions:
- explicit
- immutable
- reviewable over time

Charter Core does not provide chat, AI facilitation, or user experience features.
Those concerns are intentionally handled by higher layers.

The engine is fully deterministic and testable:
the same inputs always produce the same outcomes.

---

## Status

EARLY DEVELOPMENT

The engine is under active construction.
APIs, storage layout, and integrations are still evolving.

The conceptual model, however, is stable and documented.

---

## Intended Usage

Charter Core is a library and engine, not an end-user product.

It is intended to be embedded into:
- CLIs
- developer tools
- internal systems
- governance-aware applications
- simulations or games

All UX, collaboration, and workflow concerns live outside the engine by design.

---

## What Charter Core Does

- Defines governance Areas
- Records Authority and Scope as first-class decisions
- Facilitates explicit decision sessions
- Captures candidates and accepted resolutions
- Preserves immutable decision history
- Supports supersession and retirement without rewriting history

---

## What Charter Core Does NOT Do

- Monitor conversations
- Infer intent or consensus
- Provide user interfaces
- Require AI
- Enforce workflows beyond decision legitimacy

Charter Core is intentionally boring, explicit, and trustworthy —
by design, not convention.

---

## Why Charter Exists (For Developers)

### Flags Explain State. Charter Explains Legitimacy.

Most systems encode decisions as flags, fields, or enums:

```c#
feature_enabled = true  
is_admin = true  
policy_version = 3  
```
These explain what is currently true.

They do not explain:
- who decided this
- under what authority
- within what scope
- whether it was contested
- why it is legitimate
- what it replaced

As systems grow, this creates familiar questions:

- “Who changed this?”
- “Was this approved?”
- “Is this still valid?”
- “Why does prod differ from staging?”
- “Can we undo this safely?”

These are not bugs.
They are missing decision memory.

---

## Charter Is a Decision Ledger, Not a State Store

Charter does not replace your state.

It replaces implicit legitimacy.

Instead of this:

`is_leader = true`

Charter records:

```
Resolution R-17:  
“User X is the leader”  
Accepted under Authority R-AUTH-3  
Within Scope R-SCOPE-4  
In Area “Team Governance”
```
Your system may still derive flags from this —
but the source of truth is the decision, not the boolean.

---

## Why This Matters

Flags can be changed silently.  
Resolutions cannot.

Flags lose context.  
Resolutions preserve it.

Flags explain what.  
Resolutions explain why.

Once a decision is accepted:
- it is immutable
- it is auditable
- it may be superseded, but never rewritten

This makes systems:
- safer to evolve
- easier to debug
- harder to abuse
- more resilient to turnover

---

## Where Charter Fits

Charter is useful wherever:
- humans make decisions
- rules change over time
- authority matters
- history should not be rewritten

Examples include:
- feature gates
- policy enforcement
- configuration governance
- workflow approvals
- infrastructure changes
- multiplayer or simulation rules

If you’ve ever written:

`//TODO: document this decision`

Charter is what that TODO was pointing at.

---

## Conceptual Guide (Recommended Reading)

Charter Core can be used without reading anything else.

However, if you want to understand why it is shaped this way —  
or apply the same thinking without software — see the guides in:

/doc/guide

The suggested starting point are the Orientation Guides in:

  - /doc/guide/00-orientation
  - /doc/guide/01-thinking-models 
  - /doc/guide/02-practitioners
  - /doc/guide/03-charter 

And read them in order.

These documents introduce the mental models Charter enforces —
without requiring the engine to exist.

---

## What Charter Is Not

- not a workflow engine
- not a rules engine
- not a policy language
- not a state manager
- not AI-driven by default

Charter is intentionally narrow:

It records explicit decisions and preserves their legitimacy over time.

Everything else stays decoupled.

---

## The Design Pattern

Use flags for execution.  
Use Charter for legitimacy.

If you remember only one thing:

> State answers “what is true.”  
> Charter answers “why it is allowed.”

---

## Further Reading

See:
/doc/FAQ

or the conceptual guide under:
/doc/guide