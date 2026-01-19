# Baseline Review v2
Status: DRAFT → FROZEN (Target)  
Applies to: Charter CLI and Charter Core interaction layer

## Purpose

Baseline Review is the mechanism by which **foreign or external proposals**
are evaluated and either integrated into Charter history
or explicitly rejected.

It exists to prevent:
- silent trust of external data
- legitimacy drift across time or versions
- retroactive reinterpretation of decisions

Baseline Review is not a shortcut.
It is a legitimacy firewall.

---

## Core Idea

> Baseline Review evaluates *proposals*, not decisions.

Nothing becomes legitimate by entering a Baseline.
Legitimacy is created only through sessions.

---

## What a Baseline Review Is

A Baseline Review is:

- a bounded review workspace
- containing one or more **foreign proposals**
- evaluated deliberately and explicitly
- producing zero or more acceptance sessions

A Baseline Review is:

- mutable until closed
- auditable in full
- isolated from active decision-making
- non-legitimizing by design

---

## What a Baseline Review Is Not

Baseline Review does NOT:

- vote
- infer authority
- create consensus
- accept decisions directly
- modify existing resolutions
- reinterpret imported legitimacy

Any appearance of legitimacy is a CLI abstraction only.
All legitimacy is created through sessions.

---

## Lifecycle

### 1. Creation

A Baseline Review is created by:

- flat file import
- consolidate import
- Deliberate handoff

Restore operations **do not** create baselines.
Restore replaces the Area entirely and bypasses review.

On creation:

- All proposals enter state: `UNDER_REVIEW`
- All active sessions are paused
- Baseline becomes the sole active review context

---

### 2. Evaluation Phase

During review:

- Proposals may be:
  - inspected
  - compared
  - grouped
  - marked unchanged / modified (ergonomic only)

No authority is evaluated at this stage.
No legitimacy is created.

Acceptance and rejection are **reversible preparatory actions**
until the baseline is closed.

---

### 3. Acceptance Semantics

When a proposal is accepted:

- A **session is created** (explicit or hidden)
- Authority is evaluated normally
- Participants are explicitly defined
- Acceptance is recorded immutably

Batch acceptance:
- creates multiple sessions
- does not collapse legitimacy into a single event

Rejected proposals:
- remain auditable
- do not affect local legitimacy
- carry no semantic meaning beyond “not accepted”

---

### 4. Closure

When `baseline close` is executed:

- Remaining `UNDER_REVIEW` proposals → `ABANDONED`
- Baseline becomes immutable
- Paused sessions may resume

Closure is irreversible.
Unaccepted proposals do not linger ambiguously.

---

## Invariants

- Baseline Review never creates legitimacy
- Every accepted proposal corresponds to a session
- No proposal becomes active implicitly
- Review history must be reconstructible end-to-end
- Authority evaluation is never bypassed
- Baselines never reinterpret imported authority or scope

---

## Relationship to Other Constructs

- **Import (consolidate / flat)**: creates a Baseline Review
- **Deliberate**: may produce a Baseline Review
- **Session**: the only source of legitimacy
- **Restore**: replaces state, bypasses review entirely
- **Audit**: must show full baseline lineage

---

## Mental Model

Baseline Review is equivalent to:

> “We are looking at claims made elsewhere.
> Nothing here is true until we decide it again.”

That friction is intentional.