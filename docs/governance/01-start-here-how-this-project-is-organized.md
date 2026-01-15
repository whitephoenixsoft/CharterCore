# Start Here — How This Project Is Organized (Areas × Layers)

If you’re feeling slightly disoriented right now, that’s normal.
You’ve arrived at a project that *cares about decisions* — and that’s not something most projects explain up front.

This document is not a rulebook.
It’s a **map**.

You don’t need to memorize it.
You don’t need to understand all of it.
You just need to know how to find your bearings.

---

## The One Idea That Makes Everything Else Make Sense

This project is organized along **two dimensions**:

- **Areas** — *What kind of thing are we deciding about?*
- **Layers** — *How permanent or authoritative is this decision?*

Most confusion happens when these two get mixed together.

This map exists to prevent that.

---

## Areas: What Kind of Thing Is This?

An **Area** is a domain of concern.
Think of it as a “room” where related decisions live.

Examples of Areas in this project include:

- **Engine**
  - Core logic
  - Determinism
  - Data integrity
- **CLI**
  - User interaction
  - Commands
  - Ergonomics
- **Specifications**
  - Formal definitions
  - Expected behavior
- **Design Decisions**
  - Tradeoffs
  - Rationale
  - Architecture choices
- **Governance**
  - How changes are proposed
  - How decisions become permanent
- **Guides**
  - Mental models
  - Thinking tools
  - Orientation documents (like this one)

Important:
Areas are **about scope**, not importance.

Working in one Area does not require touching others.

---

## Layers: How Serious Is This Decision?

Layers describe **how durable** a decision is.

Think of them as levels of commitment.

### Layer 1 — Informal Work
This is where most things start.

Examples:
- Issues
- Experiments
- Notes
- Draft docs
- Early sketches

Characteristics:
- Easy to change
- Easy to abandon
- No long-term promise

This is not “lesser” work.
It’s how thinking happens safely.

---

### Layer 2 — Design & Intent
This is where ideas start to stabilize.

Examples:
- Design notes
- Rationale documents
- Tradeoff discussions

Characteristics:
- Explains *why*
- Still changeable
- Not authoritative yet

This layer exists so people don’t have to re-argue the same ideas repeatedly.

---

### Layer 3 — Specifications & Rules
This is where things become precise.

Examples:
- Specs
- Acceptance criteria
- Defined behavior

Characteristics:
- Clear expectations
- Testable
- Still separate from enforcement

Specs describe what *should* happen.
They are not the same as decisions that *must never change*.

---

### Layer 4 — Invariants (Non-Negotiables)
This is the smallest layer — and the heaviest.

Examples:
- Safety guarantees
- Determinism requirements
- Integrity constraints

Characteristics:
- Rare
- Hard to change
- Breaking them stops everything

If something reaches this layer, it’s because the cost of being wrong is extremely high.

---

## How Areas and Layers Work Together

Every change lives at **one intersection** of Area × Layer.

Examples:
- An idea about CLI usability?
  - Area: CLI
  - Layer: Informal or Design
- A new core integrity guarantee?
  - Area: Engine
  - Layer: Invariant
- A guide explaining how to think better?
  - Area: Guides
  - Layer: Design or Reference

You do **not** need to touch all layers.
You do **not** need to promote things upward.
Most things should stay where they start.

Promotion is optional.
Stability is earned.

---

## Why This Exists (And Why It’s Kind)

Without this map:
- Everything feels urgent
- Every change feels heavy
- People argue past each other
- Fear replaces judgment

With this map:
- You know where you are
- You know what’s expected
- You know what you’re *not* responsible for

Confusion becomes navigable.
Disagreement becomes productive.
Change becomes possible without drama.

---

## The Most Important Rule (Unofficial, but Real)

If you’re unsure where something belongs:

Leave it informal.

Promotion is irreversible.
Informal work is safe.

This project would rather have:
- many half-finished ideas
- than one premature “final decision”

---

## What This Map Is Not

This map is not:
- a workflow
- a gatekeeping tool
- a maturity test
- a performance metric

It’s a shared mental model.

Once you have it, the rest of the governance documents stop feeling heavy —
they start feeling obvious.

---

## What Comes Next

If this document helped you orient yourself:
- The next step is understanding **what actually needs a decision**
- And what can safely remain work-in-progress

You don’t need to learn everything at once.

You just need to know:
You’re not lost.
You’re exactly where you should be.