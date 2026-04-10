# Charter Core — Evaluation & Mutation Foundation
## Pure Evaluation, Explicit Mutation

Status: FOUNDATIONAL  
Layer: Conceptual / Engine Core  
Applies to: Engine evaluation flows, mutating commands, runtime boundaries, and all future implementations  
Does NOT define: specific command APIs, persistence mechanics, or receipt schema details  

---

## Purpose

This document defines the foundational separation between **evaluation** and **mutation** in Charter Core.

Its purpose is to ensure that:

- legitimacy is never created accidentally  
- evaluation remains pure, repeatable, and safe  
- mutation is always explicit, bounded, and auditable  
- state transitions occur only through deliberate commitment  
- the Engine never blurs “what is true now” with “what should become true next”  

This separation is one of the core architectural protections in Charter.

Without it, the system would drift toward:

- implicit legitimacy  
- hidden side effects  
- non-deterministic outcomes  
- accidental commitment  

Charter therefore treats evaluation and mutation as distinct modes of operation with different responsibilities, guarantees, and risks.

---

## Core Principle

> Evaluation observes. Mutation commits.

Evaluation answers:

> “Given the current state and the current rules, what is true?”

Mutation answers:

> “Given an explicit command and a valid evaluation result, what new truth should now be committed?”

These are related, but they are not the same.

Evaluation does not create legitimacy.  
Mutation does not invent legitimacy independently of evaluation.

The Engine must preserve this boundary at all times.

---

## Why the Separation Exists

Charter is designed to record **owned commitment**, not merely to measure agreement or simulate behavior.

That requires a hard distinction between:

- inspecting the current world  
- deciding whether a change would be valid  
- actually changing the world  

This separation exists to prevent several classes of failure:

### 1. Accidental Legitimacy

If evaluation could mutate state, legitimacy could be created merely by checking whether something would pass.

That is forbidden.

A proposal must never become legitimate because:

- someone queried it  
- someone previewed it  
- a UI refreshed  
- a library checked viability  

Legitimacy requires explicit commitment.

---

### 2. Hidden Side Effects

If evaluation could alter runtime state, then:

- repeated queries would become unsafe  
- deterministic reasoning would break  
- interfaces could accidentally trigger history  

That would make the system unpredictable and politically unsafe.

---

### 3. Confusion Between Readiness and Commitment

A proposal may be:

- valid  
- eligible  
- currently winning  
- ready for acceptance  

and still **not yet legitimate**.

The fact that something *could* be accepted is not the same as accepting it.

This distinction is essential to:

- human agency  
- political clarity  
- deterministic auditability  

---

## Evaluation

## Definition

Evaluation is the process of applying Charter’s rules to existing state in order to determine:

- current validity  
- current eligibility  
- current blocking conditions  
- current authority outcome  
- current structural or governance consequences  

Evaluation operates entirely over already-existing facts.

It does not create new facts.

---

## Evaluation Characteristics

Evaluation must be:

- pure  
- deterministic  
- side-effect free  
- repeatable  
- non-creative  
- non-destructive  

Given identical:

- domain objects  
- runtime mode  
- command/query inputs  
- rule identity  

evaluation must produce identical results.

---

## What Evaluation May Do

Evaluation may:

- inspect sessions  
- inspect candidates  
- inspect votes  
- inspect constraints  
- inspect governance context  
- inspect receipts and structural state  
- determine whether a candidate is valid  
- determine whether a session is blocked  
- determine whether authority currently passes  
- determine whether acceptance would be eligible  
- surface invariant violations or structural problems  
- produce deterministic reports  

Evaluation may describe:

- success conditions  
- failure conditions  
- blocking conditions  
- candidate status  
- session status  
- governance invalidation  
- structural conflicts  

---

## What Evaluation Must Not Do

Evaluation must not:

- mutate sessions  
- create receipts  
- create resolutions  
- alter votes  
- alter participants  
- alter candidates  
- alter constraints  
- update references  
- emit audit as a constitutive system action  
- repair invalid state  
- infer missing state  
- transform provisional status into legitimacy  

Evaluation must never turn observation into commitment.

---

## Evaluation Is Safe To Repeat

A valid evaluation operation may be repeated indefinitely.

Repeated evaluation with identical inputs must:

- produce the same result  
- leave state unchanged  
- leave history unchanged  
- leave legitimacy unchanged  

This is essential for:

- CLI previews  
- UI refreshes  
- repeated checks before acceptance  
- audit inspection  
- degraded-mode read-only behavior  

---

## Mutation

## Definition

Mutation is the process by which the Engine changes canonical state in response to an explicit command.

Mutation is how the Engine:

- creates sessions  
- changes session lifecycle  
- records votes  
- accepts candidates  
- creates legitimacy artifacts  
- closes sessions  
- applies usability transitions  
- persists new structural history  

Mutation creates new truth.

---

## Mutation Characteristics

Mutation must be:

- explicit  
- deterministic  
- atomic  
- auditable by external observation  
- bounded by command semantics  
- governed by prior rule evaluation  

Mutation is never implied.

No state transition may occur merely because:

- conditions are satisfied  
- a threshold was reached  
- a query was executed  
- a user “looked at” eligibility  

Mutation always requires an explicit act.

---

## What Mutation May Do

Mutation may:

- create new domain artifacts  
- advance session lifecycle  
- record or replace mutable session-round data where allowed  
- commit legitimacy when acceptance succeeds  
- emit required terminal artifacts such as receipts  
- update current runtime state  
- append to history through new artifacts  

Mutation may only occur when the relevant governing rules permit it.

---

## What Mutation Must Not Do

Mutation must not:

- bypass governing evaluation  
- partially apply changes  
- create legitimacy implicitly  
- rewrite immutable history  
- reinterpret past receipts  
- repair corruption silently  
- merge Areas  
- derive authority from inference  
- transform informational metadata into structural force  

Mutation must remain mechanically bounded.

---

## Relationship Between Evaluation and Mutation

Evaluation and mutation are distinct, but mutation depends on evaluation.

Conceptually:

1. The Engine inspects current truth  
2. The Engine determines whether a command is allowed  
3. If allowed, the Engine commits new truth atomically  
4. If not allowed, no mutation occurs  

This means:

- evaluation may exist without mutation  
- mutation must not exist without rule-governed evaluation  

The Engine must never allow a mutating path that bypasses legitimacy rules.

---

## Explicit Commitment Boundary

The most important application of this separation is **acceptance**.

Before acceptance:

- candidates are only proposals  
- votes are only evaluative  
- legitimacy does not yet exist  

At acceptance:

- authority is evaluated  
- constraints are checked  
- governance conditions are checked  
- structural and usability conditions are checked  
- if everything passes, legitimacy is created atomically  

Therefore:

- evaluation may tell you a candidate is ready  
- only mutation may make it legitimate  

This is the difference between:

- readiness  
- and commitment  

Charter treats that boundary as sacred.

---

## Evaluation Failure vs Mutation Failure

The Engine must distinguish clearly between:

### Evaluation Failure

The current state does not satisfy the conditions required for the requested outcome.

Examples:

- authority does not pass  
- constraints fail  
- candidate is blocked  
- governance is unusable  
- session is not eligible  

In these cases:

- no mutation occurs  
- history does not change  
- legitimacy does not change  

---

### Mutation Failure

A mutation was explicitly attempted, but the commit could not be completed.

Examples:

- atomicity could not be guaranteed  
- persistence failed within the commit boundary  
- runtime mode forbids mutation  
- integrity conditions prevent safe commit  

In these cases:

- the mutation must not partially apply  
- state must remain unchanged  
- no partial legitimacy may exist  

Both cases are explicit, but neither may create hidden outcomes.

---

## Purity of Evaluation

Evaluation purity is not merely an implementation convenience.  
It is a constitutional rule.

Purity guarantees that:

- checking a thing is not changing a thing  
- explanation is not commitment  
- visibility is not power  
- diagnostics are not enforcement  

This is particularly important in Charter because the system is designed to be:

- politically neutral  
- mechanically explicit  
- resistant to accidental authority  

If evaluation were allowed to change state, that neutrality would be broken.

---

## Explicitness of Mutation

Mutation is not merely “write behavior.”  
It is the act by which Charter records new institutional truth.

Therefore mutation must always be:

- intentional  
- command-bounded  
- reviewable  
- deterministic  
- reconstructible  

No part of legitimacy may arise from:

- ambient system behavior  
- repeated polling  
- time passing  
- hidden automation inside core evaluation  

All legitimacy must be explicitly committed.

---

## No Hidden Transition Rule

The Engine must never contain hidden transitions between evaluation and mutation.

Forbidden examples include:

- automatic acceptance when authority passes  
- automatic closure after a failed check  
- automatic session repair after detecting an inconsistency  
- automatic promotion from “eligible” to “accepted”  
- implicit state changes during preview or query operations  

The difference between “can happen” and “did happen” must always remain explicit.

---

## Relationship to Runtime Modes

The evaluation/mutation boundary must remain valid across runtime modes.

### Normal Runtime

- evaluation is allowed  
- mutation is allowed where rules permit  

### Degraded Read-Only Runtime

- evaluation may still be allowed where safe  
- mutation is forbidden  

### Initialization Failure / No Runtime

- evaluation requiring runtime must not proceed  
- mutation must not proceed  

This ensures that even in damaged or restricted conditions, the Engine does not blur inspection with commitment.

---

## Relationship to Reports and Artifacts

Evaluation produces **reports**.  
Mutation produces **history**.

Evaluation may produce:

- deterministic result objects  
- EvaluationReports  
- derived candidate status  
- blocking explanations  

Mutation may produce:

- new sessions  
- new resolutions  
- receipts  
- updated lifecycle state  
- appended structural history  

A report is not an artifact of legitimacy.  
A receipt is.

This distinction must remain visible to implementers and users.

---

## Relationship to Human Agency

This separation protects human agency.

Humans are allowed to:

- inspect  
- reconsider  
- simulate  
- discuss  
- hesitate  

without creating legitimacy.

Only when a human explicitly crosses the commitment boundary does Charter record new institutional truth.

This protects against:

- coercive momentum  
- accidental commitments  
- legitimacy by interface behavior  
- confusion between consensus and ownership  

Charter therefore encodes a humane rule:

> Thinking must remain safe.  
> Commitment must remain explicit.

---

## Conceptual Invariants

- Evaluation is pure  
- Evaluation is deterministic  
- Evaluation is repeatable without side effects  
- Mutation is explicit  
- Mutation is atomic  
- Mutation creates new history  
- Mutation must not bypass evaluation  
- No legitimacy is created by inspection alone  
- No hidden transition may convert readiness into commitment  
- Reports describe truth; mutations append truth  

---

## Summary

Charter separates evaluation from mutation so that legitimacy is never accidental.

Evaluation:

- observes  
- checks  
- explains  
- reports  

Mutation:

- commits  
- records  
- seals  
- appends history  

The Engine must always preserve this distinction.

You may inspect the world as often as you like.  
Nothing changes.

You may commit to changing it.  
Then history begins.