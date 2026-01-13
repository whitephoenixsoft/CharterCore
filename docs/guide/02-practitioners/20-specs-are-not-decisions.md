# Specs Are Not Decisions
(And Treating Them Like They Are Is Why Specs Feel Useless)

Specs have a bad reputation.

People say:
- “No one reads them.”
- “They’re outdated immediately.”
- “They’re just code in English.”
- “They don’t reflect reality.”

Most of the time, the spec isn’t the problem.

The problem is that we ask specs to do a job they were never meant to do.

---

## What Specs Are Actually For

A spec exists to answer one question:

“How does this work?”

It describes:
- behavior
- structure
- interfaces
- constraints *as implemented*

It is about **execution**, not authority.

A spec is downstream of a decision — not the decision itself.

---

## What Specs Are Terrible At

Specs are bad at answering:
- Who decided this?
- Who had the authority?
- What alternatives were rejected?
- Is this still legitimate, or just historical?

When we force specs to carry those answers:
- they become bloated
- arguments get embedded as prose
- future readers can’t tell what is mandatory vs incidental

This is why specs feel political.

They’re holding decisions they were never meant to remember.

---

## The Hidden Failure Mode

When decisions aren’t recorded anywhere else, specs become the *de facto* source of truth.

That leads to:
- rewriting history by editing documents
- debates disguised as refactors
- “the spec changed” without any explicit decision

People think they’re disagreeing about design.

They’re actually disagreeing about legitimacy.

---

## The Clean Separation

Healthy systems separate:
- **Decisions** → what we commit to
- **Specs** → how that commitment is realized

When this separation exists:
- specs get shorter
- reviews get calmer
- changes become intentional instead of sneaky

You can change a spec many times.
You should change a decision deliberately.

---

## The Payoff

Specs stop being a battlefield.
They become what they were always meant to be:

A technical explanation of something that was already decided.

That’s when people start reading them again.