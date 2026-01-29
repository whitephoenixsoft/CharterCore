# Session Voting & Acceptance
## Explicit Evaluation, Explicit Commitment

Status: FROZEN  
Applies to: Charter Core, CLI, and future interfaces  
Scope: All Sessions that evaluate Proposals

---

## Core Principle

> **Votes express evaluation.  
> Acceptance expresses commitment.  
> Charter never conflates the two.**

A proposal is never accepted *because* votes exist.
A proposal is accepted only when an explicit acceptance action is taken —
and only if the recorded votes satisfy Authority at that moment.

---

## Two-Phase Session Model

Every session evaluating proposals operates in two distinct phases:

### Phase 1 — Evaluation (Voting)

- Participants may record stances:
  - ACCEPT
  - REJECT
  - ABSTAIN
- Votes are:
  - mutable
  - auditable
  - non-legitimizing
- Votes may be changed any number of times
- No authority is evaluated continuously

This phase answers:
> “What do people currently think?”

---

### Phase 2 — Commitment (Acceptance)

Acceptance is a **separate, explicit action**.

- Acceptance:
  - is invoked deliberately (`session accept`)
  - evaluates authority **once**
  - freezes the outcome
- Acceptance is irreversible
- Acceptance closes the session

This phase answers:
> “Are we committing to this now?”

---

## Enforcement Invariants

### INV-SESSION-ACCEPT-01 — Authority Gate

A proposal **cannot be accepted** unless:

- Authority evaluation **passes**
- Based on **current recorded stances**
- At the moment acceptance is attempted

If authority does not pass:
- Acceptance fails
- Session remains open
- No legitimacy is created

---

### INV-SESSION-ACCEPT-02 — No Auto-Accept

Charter MUST NOT:
- auto-accept when votes cross a threshold
- infer acceptance from voting patterns
- close sessions implicitly

Votes never trigger acceptance.
Humans do.

---

### INV-SESSION-ACCEPT-03 — Vote Mutability Before Acceptance

Before acceptance:
- Any participant may change their vote
- Vote changes:
  - do not create audit ambiguity
  - do not imply instability
  - reflect human reality

After acceptance:
- Votes are frozen
- Authority context is sealed

---

### INV-SESSION-ACCEPT-04 — Single Commitment Point

- Each proposal may be accepted **at most once**
- Each session may produce:
  - acceptance
  - or non-acceptance
- Re-evaluation requires a new session

There is no partial acceptance.
There is no provisional legitimacy.

---

## CLI / Library Behavior

- CLI may display:
  - “Authority satisfied” (informational only)
  - “Authority not satisfied” (informational only)
- CLI MUST still require:
  - an explicit `accept` action
- CLI MUST block acceptance if authority fails

This preserves:
- cognitive clarity
- mechanical legitimacy
- political neutrality

---

## Why This Matters

This design ensures:

- Charter records **decisions**, not momentum
- Consensus is never inferred
- Politics stays outside the system
- Pressure to “just let it pass” is structurally blocked

Charter does not ask:
> “Did enough people agree?”

It asks:
> “Did someone explicitly commit — under rules everyone could see?”

---

## Mental Model

Votes are **notes on the table**.  
Acceptance is **signing the document**.

You can rearrange the notes as much as you like.  
Once signed, history is sealed.

---

## Final Lock

If acceptance were automatic,
Charter would become a political machine.

By separating voting from acceptance,
Charter remains a **memory of human intent**, not a referee of human behavior.