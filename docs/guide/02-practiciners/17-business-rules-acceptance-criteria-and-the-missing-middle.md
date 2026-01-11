# Business Rules, Acceptance Criteria, and the Missing Middle

Many teams live in a strange middle ground.

They don’t write specs.
They don’t write decisions.
They write *rules*.

Things like:
- “This must be approved by finance.”
- “The user cannot do X unless Y.”
- “This workflow blocks release.”

These aren’t design details.
They aren’t code.
They aren’t processes.

So they drift.

---

## Why Analysts Feel Stuck

Analysts often own:
- acceptance criteria
- business rules
- constraints that don’t live in code

But those rules:
- affect architecture
- affect ownership
- affect authority
- outlive individual features

Without a decision memory:
- rules get rewritten per ticket
- enforcement becomes inconsistent
- analysts feel like translators instead of decision-makers

The frustration isn’t about skill.

It’s about placement.

---

## Why Developers Get Defensive

Developers experience these rules as:
- late-breaking requirements
- “non-technical” blockers
- invisible constraints that show up during review

Without clarity:
- rules feel arbitrary
- enforcement feels personal
- work feels unstable

So developers push back — not against the rule, but against the ambiguity.

---

## What These Rules Actually Are

Most business rules are **governance decisions**.

They answer:
- what must be true
- what cannot be violated
- what blocks progress
- who has authority

They deserve the same treatment as any other decision:
- explicit
- reviewable
- replaceable
- remembered

---

## The Relief Point

When business rules are treated as decisions:
- analysts stop repeating themselves
- developers stop guessing intent
- acceptance criteria get simpler
- disputes move from people to records

Nothing becomes heavier.

It becomes clearer.

---

## The Missing Middle, Found

You don’t need more documents.

You need the right ones to hold the right kind of truth.

When rules have a home:
- specs describe behavior
- code enforces reality
- people stop carrying the burden in their heads