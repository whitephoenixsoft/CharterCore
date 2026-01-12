# How to Propose Changes
(Charter-Aligned Contribution Process)

This document defines how changes are proposed, evaluated, and accepted
*before* Charter is fully implemented.

It exists to preserve legitimacy, reduce ambiguity, and prevent silent decision-making.

Nothing here is automated yet.
That is intentional.

---

## Core Principle

**No change is valid unless its legitimacy is explicit.**

Code changes are easy.
Decision changes are not.

This process exists to make decisions visible *before* they become irreversible.

---

## Step 1: Identify the Kind of Change

Before proposing anything, identify **what type of decision this is**.

### Common Categories

- **Invariant-level change**
  - Affects what must never break
  - High bar, slow process
- **Engine semantics**
  - Affects how decisions are evaluated or recorded
- **CLI / ergonomics**
  - Affects usability without changing legitimacy
- **Documentation / clarity**
  - Explains existing behavior
- **Refactoring / implementation**
  - No semantic change

If you cannot clearly name the category, stop.
Ambiguity here means the proposal is premature.

---

## Step 2: State the Proposal Explicitly

Every proposal must answer, in plain language:

- What is changing?
- What is *not* changing?
- Why is this needed *now*?
- What problem exists today without this change?

Avoid solution-first framing.
Describe the pressure before the fix.

Bad:
> “We should add automatic acceptance here.”

Better:
> “Today, users may believe a decision is accepted when no explicit session occurred.”

---

## Step 3: Declare Scope and Authority

Every proposal must declare:

### Scope
What does this change affect?

- Core engine behavior
- CLI ergonomics
- Documentation only
- Future-facing design (no immediate effect)

### Authority
Who is allowed to accept this change?

Examples:
- Core maintainers
- Engine invariants owners
- Documentation-only approval
- Explicit consensus required

If authority is unclear, the proposal cannot proceed.

---

## Step 4: Check Against Invariants

Before discussion begins, the proposer must explicitly state:

- Which invariants this change touches
- Whether it weakens, strengthens, or clarifies them
- Why any perceived tension is acceptable (if applicable)

If a proposal violates an invariant:
- The invariant wins
- The proposal must be redesigned or abandoned

This is not negotiable.

---

## Step 5: Discussion (Deliberate, Not Reactive)

Discussion should focus on:

- Legitimacy impact
- Long-term consequences
- Edge cases and failure modes
- Audit and determinism implications

Discussion is **not** for:
- Personal preferences
- “It feels easier”
- Popularity or speed arguments

Silence is not consent.
Explicit positions are required.

---

## Step 6: Decision and Record

Every accepted change must result in:

- A clear statement of acceptance
- A reference to the authority under which it was accepted
- A summary of alternatives considered
- A note on what this supersedes (if anything)

Rejected proposals are also recorded.
Rejection is information, not failure.

---

## Step 7: Implementation Follows Legitimacy

Only after a decision is accepted may implementation proceed.

Implementation may:
- Lag behind acceptance
- Be split across commits
- Be revised technically

But the *decision itself* must remain stable.

---

## What This Process Prevents

This process exists to prevent:

- Silent authority drift
- “We already shipped it” decisions
- Retrofitted justification
- Accidental invariant erosion
- Emotional or political decision-making

If this feels slower than you expect —
that friction is intentional.

---

## Future Note

This process is designed to map directly onto Charter concepts:

- Proposals → Candidates
- Acceptance → Sessions
- Authority → Explicit governance
- Records → Immutable history

When Charter is integrated, this process will not change —
it will simply become enforceable by software.