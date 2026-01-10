# Charter Core — FAQ (Invariant-Oriented)

This FAQ explains why Charter Core behaves the way it does.
Each section corresponds to one or more **engine invariants**.

Charter Core is intentionally minimal.
Anything not justified here belongs in higher layers (CLI, UI, workflow, AI).

---
## Invariant Group: Explicit Legitimacy

### Why does Charter Core require explicit acceptance?

Because legitimacy cannot be inferred.

Charter Core **never assumes agreement** from:
- silence
- inactivity
- majority presence
- automation
- UI shortcuts

If acceptance could be implied, the system would:
- lose auditability
- become vulnerable to abuse
- allow silent drift in governance

**Explicit acceptance is the minimum requirement for trust.**

---
### Why can’t decisions be auto-accepted or inferred?

Because inference destroys accountability.

Charter Core records decisions, not outcomes.
A decision exists only if someone explicitly accepts it under known conditions.

Anything else is state, not legitimacy.

---
## Invariant Group: Authority

### What is Authority in Charter Core?

**Authority is the decision rule.**

Authority defines:
- who has standing in a session
- how agreement is evaluated

Authority does not:
- interpret content
- judge correctness
- assign meaning
- prioritize candidates

This separation prevents political or social assumptions from becoming hidden logic.

---
### Why is Authority purely mechanical?

Because meaning belongs to humans, not the engine.

If Authority interpreted content or intent, Charter Core would:
- encode organizational politics
- require role semantics
- become non-deterministic

Mechanical authority ensures:
- determinism
- auditability
- portability across contexts

---
### Why must Authority changes be their own resolutions?

Because changing *how decisions are legitimized* is itself a decision.

Authority changes:
- cannot occur mid-session
- cannot occur on resume
- cannot be implied

They must be:
- explicitly proposed
- evaluated under the current authority
- accepted like any other resolution

This prevents silent shifts in power.

---
## Invariant Group: Scope

### What is Scope in Charter Core?

Scope defines **what kinds of decisions belong in an Area**.

Scope is:
- explicit
- informational
- recorded at acceptance time

Scope is **not**:
- a validator
- an enforcement mechanism
- a semantic interpreter

---
### Why doesn’t the engine enforce Scope?

Because relevance is a human judgment.

If Charter Core enforced Scope, it would need to:
- interpret meaning
- infer intent
- reject decisions based on semantics

That would make legitimacy dependent on guesswork.

Instead:
- Scope is recorded
- humans decide whether something belongs
- history preserves what was actually decided

---
## Invariant Group: Sessions

### Why are Sessions required?

Because legitimacy requires context.

A resolution must be traceable to:
- a specific problem
- a known participant set
- a known authority
- a known scope
- a specific moment in time

Sessions provide that boundary.

---
### Why can’t sessions silently adapt to changes?

Because that would rewrite history.

If Authority, Scope, or constraints change:
- sessions become blocked
- explicit action is required
- no retroactive reinterpretation occurs

Charter Core **reacts**, it does not auto-correct.

---
## Why are sessions immutable once closed?

Because closed sessions are historical facts.

They:
- cannot be resumed
- cannot be edited
- cannot be reused

They may inform future sessions, but never govern them.

---
## Invariant Group: Candidates

### Why are candidates neutral?

Because options are not decisions.

Candidates:
- carry no intent
- have no legitimacy
- have no effect until accepted

This allows ambiguity, exploration, and disagreement **without pressure to decide**.

---
### Why does the candidate set freeze after voting starts?

Because changing options after evaluation begins corrupts legitimacy.

Once any stance is recorded:
- no candidates may be added
- no candidates may be removed
- no candidates may be edited

If the problem changes, a **new session** is required.

---
## Invariant Group: Supersession & History

### Why are resolutions immutable?

Because history must not be rewritten.

Once accepted, a resolution:
- cannot be edited
- cannot be deleted
- cannot be reinterpreted

It may only be **superseded** by a new resolution .

---
### What happens when a resolution is superseded?

The old resolution remains:
- queryable
- auditable
- historically valid

Supersession explains evolution, not correction.

---
### Why doesn’t Charter Core auto-retire “irrelevant” decisions?

Because relevance is contextual and temporal.

Automatically retiring decisions would:
- erase accountability
- distort history
- encode assumptions about intent

Charter Core preserves decisions exactly as they were accepted.

---
## Invariant Group: Import & External History

### Why are imported resolutions marked UNDER_REVIEW?

Because imported legitimacy cannot be assumed.

Charter Core cannot know:
- whether authority was equivalent
- whether scope aligned
- whether participants overlapped

Marking imported resolutions UNDER_REVIEW forces explicit reconciliation.

---
### Why doesn’t the engine auto-merge timelines?

Because merging is semantic, not mechanical.

Charter Core records facts.
Humans decide meaning.

---
## Invariant Group: Auditability

### Why is the audit trail immutable?

Because legitimacy depends on traceability.

Audit records must:
- outlive the entities they describe
- never disappear due to cleanup
- never be rewritten

This ensures accountability even after structural change.

---
### Why doesn’t the engine provide opinions in audits?

Because audits report facts, not judgments.

Charter Core audits answer:
- *what happened*
- *when*
- *under what conditions*

They never answer:
- *was this correct*
- *was this a mistake*
- *should this have happened*

---
## Invariant Group: Minimalism

### Why doesn’t Charter Core support roles, permissions, or identities?

Because those introduce hidden semantics.

Roles require:
- identity systems
- lifecycle management
- interpretation of meaning

Charter Core avoids all of this.

Participants are identifiers.
Authority is mechanical.
Meaning remains external.

---
### What if these rules feel too strict?

That is intentional.

Charter Core optimizes for:
- long-term integrity
- explicit intent
- auditability under pressure

Convenience, ergonomics, and workflow belong in higher layers.

---
## Summary

Charter Core exists to answer one question reliably:

> “Why was this allowed?”

Everything in the engine exists to preserve that answer over time.

If a proposed change makes that answer weaker, ambiguous, or inferential,
it violates a core invariant — even if it feels convenient.

That boundary is the product.