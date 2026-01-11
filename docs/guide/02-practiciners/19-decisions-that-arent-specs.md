# Decisions That Aren’t Specs (But Still Matter)

Not every important decision is:
- a specification
- a piece of code
- a process document
- a ticket
- a checklist

But some decisions still **must persist**.

If you don’t give them a home, they don’t disappear.
They just come back — louder, messier, and more personal.

---

## The Problem We All Recognize

Teams keep having the same conversations:

- “Do we deploy on Fridays?”
- “Does security block this release or advise on it?”
- “Is this team allowed to touch that system?”
- “Who actually owns this boundary?”

Everyone *thinks* the answer exists.
No one knows where it lives.

So the decision:
- gets re-argued
- depends on who’s present
- changes with turnover
- quietly becomes a power struggle

This isn’t because people are careless.

It’s because the decision never had a place to stay.

---

## These Are Not Specs

A spec answers:
> “How should this work?”

These decisions answer:
> “How do we operate?”

Trying to force them into specs causes problems:
- specs become bloated
- technical decisions get mixed with governance
- readers don’t know what is mandatory vs contextual

These decisions are **orthogonal** to design.

They constrain it — but they are not it.

---

## These Are Not Code Decisions

Code expresses *what happens*.

These decisions express:
- what is allowed
- what is blocked
- what is owned
- what requires escalation

You cannot grep them.
You cannot infer them reliably.
You cannot expect newcomers to guess.

If it matters, it must be stated.

---

## These Are Not Process Documents Either

Processes describe flows.

These decisions describe **boundaries**.

Examples:
- “Security reviews block releases.”
- “This team owns authentication.”
- “Incidents override feature work.”
- “We do not deploy on Fridays.”

They don’t explain *how*.
They explain *what is true*.

---

## The Decision Memory Space

These decisions belong in a **decision memory space**.

A place that exists to answer:
- “Is this still true?”
- “Who decided this?”
- “What does it apply to?”
- “What replaced it, if anything?”

Not everything needs to be there.
But anything that keeps coming back does.

---

## The Rule of Recurrence

Here’s the simplest thinking model:

> **If this decision keeps coming back, it deserves a home.**

Not a ticket.
Not a meeting.
Not a Slack thread or Teams chat.
Not a tribal memory.

A place where it can:
- persist
- be referenced
- be replaced deliberately
- stop wasting energy

---

## Why This Reduces Friction

When these decisions are explicit:
- arguments shrink
- escalation feels less personal
- onboarding gets faster
- authority becomes visible, not assumed

People stop fighting *each other*  
and start working with **reality**.

---

## The Quiet Payoff

You don’t need more rules.

You need fewer *floating* ones.

When decisions that matter have a home:
- specs get cleaner
- code gets simpler
- people get calmer

That’s not control.

That’s relief.