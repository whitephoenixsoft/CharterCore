# Baseline Review v2
Status: DRAFT → FROZEN (Target)
Applies to: Charter CLI and Charter Core interaction layer

## Purpose

Baseline Review is the mechanism by which **foreign proposals** are evaluated
and either integrated into Charter history or explicitly rejected.

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

---

## What a Baseline Review Is Not

Baseline Review does NOT:

- vote
- infer authority
- create consensus
- accept decisions directly
- modify existing resolutions

Any appearance of legitimacy is a CLI abstraction only.

---

## Lifecycle

### 1. Creation

A Baseline Review is created by:

- flat file import
- consolidate import
- restore preparation (pre-confirmation)
- Deliberate handoff (future)

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

Acceptance and rejection are *preparatory actions*.

---

### 3. Acceptance Semantics

When a proposal is accepted:

- A **session is created** (explicit or hidden)
- Authority is evaluated normally
- Participants are explicitly defined
- Acceptance is recorded immutably

Batch acceptance:
- creates multiple sessions
- does not collapse legitimacy into one event

Rejected proposals:
- remain auditable
- do not affect local state

---

### 4. Closure

When `baseline close` is executed:

- Remaining `UNDER_REVIEW` proposals → `ABANDONED`
- Baseline becomes immutable
- Paused sessions may resume

Closure is irreversible.

---

## Invariants

- Baseline Review never creates legitimacy
- Every accepted proposal corresponds to a session
- No proposal becomes active implicitly
- Review history must be reconstructible end-to-end
- Authority evaluation is never bypassed

---

## Relationship to Other Constructs

- **Import**: creates a Baseline Review
- **Deliberate**: may produce a Baseline Review
- **Session**: the only source of legitimacy
- **Audit**: must show full baseline lineage

---

## Mental Model

Baseline Review is equivalent to:

> “We are looking at claims made elsewhere.
> Nothing here is true until we decide it again.”

That friction is intentional.