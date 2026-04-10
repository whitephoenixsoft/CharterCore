# Charter Foundation — Voting & Acceptance  
## Explicit Evaluation, Explicit Commitment

Status: FROZEN  
Applies to: Charter Core, CLI, and all future interfaces  
Scope: Any session that evaluates proposals  

---

## Purpose

This document defines *why* Charter strictly separates **evaluation** from **commitment**,  
and how that separation preserves:

- legitimacy  
- clarity  
- auditability  
- human dignity  

This is **not** a procedural specification.  
It is the conceptual foundation underlying the engine invariants.

Charter is not designed to measure agreement.  
Charter is designed to record **owned commitment**.

---

## Core Principle

**Votes express evaluation.**  
**Acceptance expresses commitment.**

Charter never conflates the two.

A proposal is never accepted *because* votes exist.  
A proposal is accepted only when a human **explicitly commits** to it —  
and only if all required conditions are satisfied **at that moment**.

Before authority is evaluated, all explicit session constraints must be satisfied.

This separation is intentional and non-negotiable.

---

## Why Voting Exists

Voting exists to answer one question:

> **“What do people currently think?”**

Votes are:

- expressions of stance  
- provisional  
- changeable  
- auditable  
- non-legitimizing  

Votes are not promises.  
Votes are not commitments.  
Votes do not create history.

They are a snapshot of human evaluation — nothing more.

---

## Why Acceptance Exists

Acceptance exists to answer a different question:

> **“Are we committing to this now?”**

Acceptance is:

- explicit  
- deliberate  
- historically immutable  
- legitimacy-creating  

Once accepted, the outcome cannot be undone, only superseded through a new session.

Acceptance is the moment when:

- authority is evaluated  
- responsibility is owned  
- history becomes sealed  

Charter treats this moment with gravity on purpose.

---

## The Two-Phase Session Model

Every session that evaluates proposals operates in two distinct phases.

---

### Phase 1 — Evaluation (Voting)

During evaluation:

Participants may record stances:

- ACCEPT  
- REJECT  
- ABSTAIN  

Votes are:

- mutable  
- auditable  
- non-authoritative  

Rules:

- Votes may be changed any number of times  
- Authority is **not** continuously evaluated  
- No legitimacy is created  

This phase supports:

- exploration  
- disagreement  
- learning  
- human uncertainty  

Changing one’s mind is not instability.  
It is evidence that thinking is happening.

---

### Phase 2 — Commitment (Acceptance)

Acceptance is a **separate, explicit action**.

When acceptance is attempted:

1. All session constraints must be satisfied  
2. Authority is evaluated once  
3. Evaluation uses the vote state at that moment  

If all conditions are satisfied:

- acceptance succeeds  
- outcome is frozen  
- legitimacy is created  

If any condition fails:

- acceptance does not occur  
- session remains non-terminal  
- no legitimacy is created  

Acceptance answers one question only:

> **“Are we willing to own this now?”**

---

## Enforcement Philosophy (Conceptual)

Charter enforces this separation not to be strict —  
but to prevent **accidental legitimacy**.

---

### Authority Is a Gate, Not a Trigger

Authority never acts on its own.

- Votes do not trigger acceptance  
- Thresholds do not trigger acceptance  
- Time does not trigger acceptance  

Authority is evaluated **only** when a human explicitly asks to accept.

Humans create legitimacy.  
Mechanics verify it.

---

### Constraints Gate Eligibility

Constraints determine whether acceptance may be attempted.

- They must be satisfied before authority is evaluated  
- They do not decide outcomes  
- They do not replace authority  

Together:

- **Constraints determine eligibility**  
- **Authority determines outcome**

---

### Governance May Prevent Acceptance

Acceptance may be prevented by governance conditions independent of voting.

Examples include:

- Scope being under review (ON_HOLD)  
- Invalid or changing governance context  
- Structural or eligibility conditions not being satisfied  

These conditions:

- prevent acceptance  
- do not create legitimacy  
- do not rewrite history  

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

> Think first.  
> Commit second.

---

### Single Commitment Point

Each candidate within a session may be accepted **at most once**.

Each session ends as either:

- **ACCEPTED** — legitimacy created  
- **CLOSED** — no legitimacy created  

There is:

- no partial acceptance  
- no provisional legitimacy  
- no “almost decided” state  

Charter records decisions — not momentum.

---

## Interface Responsibility (CLI / Library)

Interfaces may assist, but never decide.

Interfaces **may**:

- display whether authority would pass  
- explain why acceptance is not eligible  
- summarize current voting state  

Interfaces **must**:

- require an explicit accept action  
- prevent acceptance when conditions are not satisfied  

This preserves:

- cognitive clarity  
- mechanical legitimacy  
- neutrality  

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

Votes are notes on the table.  
Acceptance is signing the document.

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

> **“Did someone explicitly commit — under rules everyone could see?”**