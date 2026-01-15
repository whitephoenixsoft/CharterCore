# Invariants and Specifications Lifecycle
*A governance process for earning permanence, not declaring it prematurely*

## Why This Document Exists

Charter is built to preserve legitimacy over time.

That means **not everything deserves to be permanent** — especially not early.
This document defines how invariants and specifications are allowed to emerge, mature, and eventually become binding.

Its purpose is to:

- Prevent premature formalization
- Protect exploration and iteration
- Make permanence intentional
- Ensure invariants are earned, not assumed
- Align documentation, code, and future Charter imports

This is not a process for speed.
It is a process for long-term trust.

---

## Core Principle

**Invariants are discovered — not designed.**

Specifications describe behavior.
Invariants constrain possibility.

Confusing the two leads to brittle systems and false certainty.

---

## The Five Phases of Legitimacy

All invariants and specs pass through these phases.
Nothing skips steps.

### Phase 0 — Exploration (Pre-Legitimacy)

**State:** Informal, provisional, allowed to be wrong

Artifacts:
- Notes
- Draft specs
- Hypotheses
- Experimental code
- Partial documentation

Characteristics:
- No guarantees
- No permanence
- No import into Charter
- No “must never break” language

This phase exists to allow thinking.

If something is unclear here, that is success — not failure.

---

### Phase 1 — Provisional Specification

**State:** Behavior is described, not defended

Artifacts:
- Written specifications
- Early doc tests
- Engine semantics in flux

Purpose:
- Capture observed behavior
- Make assumptions visible
- Enable discussion and refactoring

Rules:
- Specs may change freely
- Specs may be rewritten or removed
- Specs describe what happens, not what must always happen

At this stage, specs are documentation — not law.

---

### Phase 2 — Testing and Abuse

**State:** Reality pushes back

Artifacts:
- Acceptance tests
- API-level tests
- Edge-case simulations
- Refactors that preserve behavior

Signals of maturity:
- Behavior survives refactoring
- Edge cases are discovered and handled
- Attempts to break semantics fail
- Tests represent external pressure, not just internal intent

This phase answers:
“Does this hold when stressed?”

If it doesn’t, it is not ready.

---

### Phase 3 — Invariant Extraction

**State:** Non-negotiables are named

Only after behavior has survived pressure may invariants be declared.

Invariants:
- Are derived from stable specs
- Are phrased narrowly
- Are defensive, not aspirational
- Represent trust boundaries

An invariant means:
“If this ever breaks, the system must stop or refuse to proceed.”

Rules:
- Invariants are rare by design
- Invariants are expensive to change
- Invariants outrank features, ergonomics, and convenience

At this stage, invariants become binding.

---

### Phase 4 — Import as Institutional Memory

**State:** Explicit decision to remember forever

Once invariants and mature specs are imported into Charter format:

- They become immutable history
- They may only be superseded, never edited
- Mistakes require new decisions, not corrections

Import is not a storage mechanism.
It is a legitimacy declaration.

Only artifacts that have earned permanence belong here.

---

## Relationship Between Specs and Invariants

Not every spec implies an invariant.

- Specs answer: “What does the system do?”
- Invariants answer: “What must never be violated?”

Most specs will evolve.
Few deserve invariant status.

A useful test:
“If changing this would invalidate past decisions or trust, it may be an invariant.”

---

## What This Process Protects Against

This lifecycle exists to prevent:

- Over-documentation
- False certainty
- Premature locking of ideas
- Governance theater
- Rewriting history silently
- Encoding taste as law

If something feels uncomfortable to lock in —
that discomfort is usually correct.

---

## Alignment With Charter

This lifecycle is intentionally aligned with Charter concepts:

- Drafts → Candidates
- Discussion → Sessions
- Acceptance → Legitimate decisions
- Imports → Immutable history
- Supersession → Explicit evolution

Until Charter is fully implemented, markdown and tests act as temporary memory.

The rules do not change when software arrives.
Only enforcement becomes mechanical.

---

## Final Note

This process is conservative on purpose.

It assumes:
- We are fallible
- Understanding emerges over time
- Trust is easier to preserve than repair

If you ever feel pressure to formalize before something has earned it —
pause.

Legitimacy cannot be rushed.
But once earned, it must be protected.

This document exists to ensure we do that deliberately.