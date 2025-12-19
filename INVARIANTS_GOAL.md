# Charter Engine — Invariants (Goal State)

This document defines the **long-term invariants** of the Charter engine.

These invariants describe what must always be true for Charter to be trusted,
regardless of MVP shortcuts, architecture changes, or AI capabilities.

They represent the **goal state**, not the current implementation.

---

## Core Principle

> Charter governs **legitimacy of decisions**, not execution of work.

---

## Decision & History

1. **Immutability**  
   Accepted resolutions are immutable.

2. **Supersession, Not Editing**  
   Decisions evolve only through explicit supersession.

3. **Permanent Decision Memory**  
   All past resolutions remain queryable and explainable.

4. **Single Active Resolution per Scope**  
   At most one active resolution exists for any scope at a time.

---

## Process & Sessions

5. **Decisions Require Sessions**  
   All resolutions are created through structured sessions.

6. **Explicit Acceptance**  
   No decision is finalized without a clear human acceptance action.

7. **Block on Invalid Assumptions**  
   Sessions must block if scope or authority becomes invalid.

8. **Resume with Integrity**  
   Blocked sessions may resume only after conflicts are resolved.

---

## Scope

9. **Scope Defines Legitimacy**  
   Decisions outside scope are invalid.

10. **Scope Is Explicit and Visible**  
    Scope violations must block acceptance.

11. **Scope Is Non-Retroactive**  
    Changing scope does not invalidate past decisions.

---

## Authority

12. **Authority Is Explicit**  
    Authority is never implied by role, history, or AI confidence.

13. **Authority Checked at Acceptance**  
    Authority is evaluated only when a decision is accepted.

14. **Authority Changes Are Governed**  
    Authority may change only via dedicated sessions.

15. **Authority Is Non-Retroactive**  
    Past decisions remain valid after authority changes.

---

## AI

16. **AI Is a Facilitator Only**  
    AI may assist but never decide, accept, or enforce.

17. **AI Has No Authority**  
    AI confidence does not imply legitimacy.

18. **AI Is Optional**  
    Charter must function fully without AI.

---

## Structure & Templates

19. **Templates Enforce Structure Only**  
    Templates define shape, not meaning.

20. **Decisions Are Human-Readable**  
    Resolutions are designed for people first.

---

## Integration & Execution

21. **No Execution**  
    Charter does not execute, enforce, or configure systems.

22. **No Silent Side Effects**  
    Decisions do not cause external changes implicitly.

23. **Integrations Are Contextual**  
    External systems provide context, not authority.

---

## Federation & Scale

24. **Federated by Design**  
    Governance is scoped to Areas.

25. **No Implicit Cross-Area Authority**  
    Authority does not bleed across boundaries.

26. **Scale Independence**  
    All invariants apply equally to one user or many.

---

## Final Assertion

> Any implementation that violates these invariants is incorrect,
> even if it appears useful or feature-complete.