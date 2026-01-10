# Usage Pattern: Legitimacy Gate for integration with Systems 

## Intent

Use Charter Core as a legitimacy oracle for actions performed by another system.

Charter does not execute actions, evaluate correctness, or manage workflows. It answers one question only:

> Is this action legitimate according to recorded authority, scope, and accepted resolutions?

This pattern allows systems to enforce governance without embedding governance logic.

---
## Problem

Many systems must enforce rules that are:
- human-defined
- socially agreed
- auditable over time

Examples:
- “Who approved this?”
- “Was this change authorized?”
- “Did we agree this was allowed?”


Hard-coding these rules leads to:
- policy drift
- undocumented exceptions
- silent bypasses

---

## Solution

Treat Charter Core as a legitimacy gate.

1. The host system defines an action requiring legitimacy.
2. The system queries Charter Core with:
    - Area
    - Action description
    - Relevant scope(s)
3. Charter Core evaluates:
    - Active Authority
    - Active Scope
    -  Accepted resolutions
4. Charter responds with:
    - Legitimate
    -  Not Legitimate
    -  Ambiguous (human clarification required)

The host system decides how to react.

---

## What Charter Core Does

- Records explicit decisions
- Preserves immutable history
- Evaluates authority mechanically
- Provides traceable legitimacy

---

## What Charter Core Does Not Do

- Enforce execution
- Trigger workflows
- Interpret meaning
- Evaluate correctness
- Resolve ambiguity automatically

All semantics remain human-owned.

---

## Example: Software Delivery Gate (Conceptual)

- Action: “Deploy service X to production”
- Charter query:
    - Area: Production Operations
    - Scope: Customer-facing services
- Charter result:
    - Legitimate (Resolution R-42)

Deployment proceeds.

If challenged later:

> “Why was this allowed?” Answer: “Resolution R-42, accepted under Authority R-AUTH-7.”

---

## Example: Policy Enforcement

- Action: “Publish updated HR policy”
- Charter verifies:
    - HR authority
    - Legal co-approval

- No approval → action blocked
- No debate → legitimacy is explicit

---

## Why This Works

- Governance becomes explicit
- Exceptions leave a trail
- Authority cannot be implied
- History cannot be rewritten

Legitimacy is preserved even when systems change.

---

## Failure Modes (Intentional)

- Ambiguity forces human resolution
- Missing authority blocks action
- Overreach is visible, not silent

These are features, not bugs.

---

## When Not to Use This Pattern

- Purely technical validation
- Performance-critical hot paths
- Fully automated systems with no human governance

Charter optimizes for legitimacy, not speed.