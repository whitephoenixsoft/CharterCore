# Charter + Git: Preserving Decision Legitimacy Alongside Code

## Why This Document Exists

Git excels at preserving *what changed*.

Charter exists to preserve *why it was allowed*.

This document explores a future-facing integration pattern where Charter and Git coexist — not by replacing one another, but by filling a gap that version control alone cannot address.

This is not a feature proposal.
It is a way of thinking about decisions as first-class artifacts.

---

## The Problem Git Solves (Extremely Well)

Git provides:
- immutable history
- distributed collaboration
- content-based versioning
- clear authorship of changes

When something breaks, Git can usually tell you:
- what commit introduced it
- who authored it
- when it landed

But Git does **not** answer:

- Who decided this was acceptable?
- Under what authority?
- Within what scope?
- Was it contested?
- What rule did it replace?
- Is this still legitimate, or just inherited?

These questions emerge naturally as systems scale.
They are not workflow failures.
They are missing decision memory.

---

## The Hidden Layer: Decisions Already Exist

In every repository, decisions already happen:

- merging a PR
- approving an exception
- introducing a policy
- bypassing a rule “just this once”
- changing defaults
- delegating authority implicitly

Git records the *effects* of these decisions.
It does not record their *legitimacy*.

As a result:
- authority becomes implicit
- silence becomes approval
- history becomes narrative instead of fact
- teams rely on memory and folklore

This works — until it doesn’t.

---

## What Charter Adds (Without Replacing Anything)

Charter does not replace:
- Git
- pull requests
- code review
- CI/CD
- permissions
- workflows

Charter adds a parallel ledger:

A **decision ledger** that records:
- explicit resolutions
- the authority under which they were accepted
- the scope they apply to
- their supersession history
- immutable acceptance context

Where Git answers:
> “What changed?”

Charter answers:
> “Why was this allowed?”

---

## Side-by-Side, Not On Top Of

In a future integration model:

| Git Concept | Charter Concept |
|------------|-----------------|
| Repository | Area |
| Pull Request | Decision Session |
| Proposed Change | Candidate |
| Review | Stance |
| Merge | Acceptance |
| Revert | Supersession |
| Branch History | Resolution Lineage |

Git remains the execution layer.
Charter becomes the legitimacy layer.

Nothing breaks.
Nothing is replaced.
Context is added.

---

## A New Category Emerges: Decision Infrastructure

If a platform like GitHub ever adopted this model, it would not be called governance.

It would quietly introduce something new:

**Decision Infrastructure**

Not:
- rules engines
- policy enforcement
- workflow automation

But:
- legitimacy graphs
- authority visibility
- decision memory
- institutional context

This would enable questions no system can answer today:

- Which rules exist because of decisions vs accidents?
- Which policies are legacy but still active?
- Which repositories intentionally diverge from standards?
- What authority was in force when this rule was accepted?
- Which decisions survived turnover — and which didn’t?

---

## Why This Matters (Even If It Never Ships)

Even without integration:

This model helps teams:
- separate state from legitimacy
- stop re-deciding the same things
- reduce blame-driven retrospectives
- understand authority without power games
- preserve context across turnover

Charter is useful even when it exists only as a mental model.

---

## A Quiet but Powerful Shift

Most tools focus on:
- speed
- automation
- enforcement

Charter focuses on:
- explicitness
- legitimacy
- memory

It does not tell teams *what* to decide.
It ensures decisions — once made — are not lost, rewritten, or silently overridden.

That alone changes how systems evolve.

---

## Final Thought

Git taught the world to respect history.

Charter applies that lesson to decisions.

If one day they meet, it will not be loud.
It will simply make long-lived systems calmer, safer, and easier to reason about.

That future does not require permission to imagine.