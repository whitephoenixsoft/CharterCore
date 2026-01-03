# Charter Core

Charter Core is a deterministic governance engine for recording, evolving, and auditing agreed decisions within explicit authority and scope.

Charter Core is designed to preserve institutional integrity by making decisions explicit, immutable, and reviewable over time.

It does not provide chat, AI facilitation, or user experience features.
Those are intentionally handled by separate layers.

Charter Core is designed to be fully deterministic and testable: the same inputs always produce the same outcomes.
## Status

**EARLY DEVELOPMENT**

## Intended Usage

Charter Core is a library and engine, not an end-user product.

It is intended to be embedded into:
- CLIs
- developer tools
- internal systems
- games or simulations
- governance-aware applications

All user experience, collaboration, and workflow concerns are intentionally handled outside the engine.
## What Charter Core Does

- Defines governance Areas
- Records Authority and Scope as first-class decisions
- Facilitates structured decision sessions
- Captures candidates and accepted resolutions
- Preserves immutable decision history
- Supports review, supersession, and retirement of decisions

## What Charter Core Does NOT Do

- Monitor conversations
- Infer decisions
- Provide user interfaces
- Require AI
- Enforce workflows beyond decision legitimacy

Charter Core is intentionally boring, explicit, and trustworthy — by design, not convention.

---

## Why Charter Exists (For Developers)

### Flags Explain State. Charter Explains Legitimacy.

Most systems encode decisions as flags, fields, or enums:


```Code
feature_enabled = true
is_admin = true
policy_version = 3
```

These explain what is currently true.

They do *not* explain:
- who decided this
- under what authority
- within what scope
- whether it was contested
- why it is legitimate
- what it replaced

As systems grow, this creates familiar problems:
- “Who changed this?”
- “Was this approved?”
- “Is this still valid?”
- “Why does prod differ from staging?”
- “Can we undo this safely?”

These questions are not bugs — they are missing decision memory.

---

###  Charter Is a Decision Ledger, Not a State Store

Charter does not replace your state.

It replaces implicit legitimacy.

Instead of this:
```Code
is_leader = true
```
Charter records:
```Text
Resolution R-17:
“User X is the leader”
Accepted under Authority R-AUTH-3
Within Scope R-SCOPE-4
In Area ‘Team Governance’
```

Your system may still derive flags from this —
but the source of truth is the decision, not the boolean.

---

### Why This Matters

Flags can be changed silently.
Resolutions cannot.

Flags lose context.
Resolutions preserve it.

Flags explain what.
Resolutions explain why.

Once a decision is accepted:
- it is immutable
- it is auditable
- it can be superseded but never rewritten

This makes systems:
- safer to evolve
- easier to debug
- harder to abuse
- more resilient to turnover

---

### Where Charter Fits

Charter is useful wherever:
- humans make decisions
- rules change over time
- authority matters
- history should not be rewritten

Examples:
- feature gates
- policy enforcement
- configuration governance
- multiplayer rules
- workflow approvals
- infrastructure changes

If you’ve ever added:

```Code
# TODO: document this decision
```

Charter is what that TODO was pointing at.

---

### What Charter Is Not

- not a workflow engine
- not a rules engine
- not a policy language
- not a state manager
- not AI-driven by default

Charter is intentionally narrow:

> It records explicit decisions and preserves their legitimacy over time.

Everything else stays decoupled.

---

### The Design Pattern

Use flags for **execution**.
Use Charter for **legitimacy**.

If you only remember one thing:

> State answers “what is true.”
> Charter answers “why it is allowed.”

---

## Further Questions 

If there are any questions, please read:

- `doc/FAQ`

