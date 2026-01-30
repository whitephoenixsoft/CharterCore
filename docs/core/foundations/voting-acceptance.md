# Charter Foundation — Voting & Acceptance
## Explicit Evaluation, Explicit Commitment

Status: FROZEN (V1 Foundation)  
Applies to: Charter Core, CLI, and all future interfaces  
Scope: Any Session that evaluates proposals

---

## Purpose

This document explains **why Charter separates voting from acceptance**  
and how that separation preserves legitimacy, clarity, and human dignity.

It is not a procedural spec.
It is the conceptual foundation behind the invariants.

Charter is not designed to measure agreement.
It is designed to record **owned commitment**.

---

## Core Principle

**Votes express evaluation.**  
**Acceptance expresses commitment.**

Charter never conflates the two.

A proposal is **never accepted because votes exist**.  
A proposal is accepted only when a human explicitly commits to it —
and only if authority rules are satisfied **at that moment**.

This separation is intentional and non-negotiable.

---

## Why Voting Exists

Voting exists to answer one question:

> *“What do people currently think?”*

Votes are:
- expressions of stance
- provisional
- changeable
- auditable
- non-legitimizing

Votes are not promises.  
Votes are not commitments.  
Votes do not create history.

They are a snapshot of human evaluation, nothing more.

---

## Why Acceptance Exists

Acceptance exists to answer a different question:

> *“Are we committing to this now?”*

Acceptance is:
- explicit
- deliberate
- irreversible
- legitimacy-creating

Acceptance is the moment when:
- authority is evaluated
- responsibility is owned
- history becomes sealed

Charter treats this moment with gravity on purpose.

---

## The Two-Phase Session Model

Every session that evaluates proposals operates in **two distinct phases**.

### Phase 1 — Evaluation (Voting)

During evaluation:

- Participants may record stances:
  - `ACCEPT`
  - `REJECT`
  - `ABSTAIN`
- Votes are:
  - mutable
  - auditable
  - non-authoritative
- Votes may be changed any number of times
- Authority is **not continuously evaluated**

This phase supports:
- exploration
- disagreement
- learning
- human uncertainty

Changing one’s mind is not instability.  
It is a sign that thinking is happening.

---

### Phase 2 — Commitment (Acceptance)

Acceptance is a **separate, explicit action**.

When acceptance is attempted:

- Authority is evaluated **once**
- Based on votes as they exist **at that moment**
- If authority passes:
  - acceptance succeeds
  - outcome is frozen
  - session closes
- If authority fails:
  - acceptance is blocked
  - session remains open
  - nothing becomes legitimate

Acceptance answers:
> *“Are we willing to own this now?”*

---

## Enforcement Philosophy (Conceptual)

Charter enforces separation not to be strict —
but to prevent accidental legitimacy.

### Authority Is a Gate, Not a Trigger

Authority does not act on its own.
It is only evaluated when a human asks to accept.

Votes never trigger acceptance.
Humans do.

---

### Mutability Before, Finality After

Before acceptance:
- Votes may change freely
- No implication of instability
- No legitimacy risk

After acceptance:
- Votes are frozen
- Authority context is sealed
- History is immutable

This mirrors real human decision-making:
thinking first, committing later.

---

### Single Commitment Point

Each proposal may be accepted **at most once**.  
Each session produces either:
- acceptance  
- or non-acceptance  

Re-evaluation requires a new session.

There is:
- no partial acceptance
- no provisional legitimacy
- no “almost decided” state

Charter records decisions, not momentum.

---

## Interface Responsibility (CLI / Library)

Interfaces may assist, but never decide.

Interfaces may:
- display whether authority would pass
- explain why acceptance is blocked
- summarize voting state

Interfaces must still:
- require an explicit accept action
- block acceptance when authority fails

This preserves:
- cognitive clarity
- mechanical legitimacy
- political neutrality

---

## Psychological Rationale

This model protects humans from:

- pressure to “just let it pass”
- legitimacy by exhaustion
- confusion between agreement and commitment
- shame for changing one’s mind

You are allowed to think.
You are allowed to hesitate.
You are allowed to revise.

But when you accept —
you are owning the outcome.

---

## Mental Model

> Votes are notes on the table.  
> Acceptance is signing the document.

You can rearrange the notes as much as you like.  
Once signed, history is sealed.

---

## Final Foundation Lock

If acceptance were automatic,
Charter would become a political machine.

By separating voting from acceptance,
Charter remains:

- a memory of human intent
- a record of owned decisions
- not a referee of human behavior

Charter does not ask:

> “Did enough people agree?”

It asks:

> “Did someone explicitly commit — under rules everyone could see?”

