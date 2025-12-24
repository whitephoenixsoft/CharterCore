# Charter Core — Frequently Asked Questions (FAQ)

This FAQ explains why Charter Core is intentionally minimal, and how certain design decisions preserve legitimacy, auditability, and long-term usefulness.

---
## What problem does Charter Core solve?

Charter Core preserves legitimate decisions over time.

It records:
- What was decided
- By whom
- Under what authority
- Within what scope
- And how decisions evolved

It does **not** manage conversation, tasks, workflows, or execution.

---
## Why does Charter Core require explicit acceptance?

Because legitimacy cannot be inferred.

If decisions could be implied (by silence, inactivity, or automation), the system would:
- Lose auditability
- Drift semantically
- Become vulnerable to abuse

Explicit acceptance is the foundation of trust.

---

## Why are Authority and Scope resolutions mandatory?

Because decisions without governance context are meaningless.

Authority answers:

> Who is allowed to legitimize decisions, and by what mechanical rule?

Scope answers:

> What kinds of decisions belong here?

Both must be explicit to prevent ambiguity and retroactive reinterpretation.

---
## Why doesn’t Charter Core support roles (e.g. “Legal”, “Admin”, “Moderator”)?

Because roles introduce hidden semantics.

Roles require:
- Identity systems
- Membership tracking
- Lifecycle rules
- Semantic interpretation

Charter Core intentionally avoids this.

Instead, it provides mechanical rules and explicit human decisions.
Meaning stays with people, not the engine.

---

## Why is Authority purely mechanical?

Authority defines:
- Who has standing in a session
- How agreement is evaluated

Authority does **not**:
- Interpret content
- Prioritize options
- Judge correctness

This separation prevents politics from becoming implicit logic.

---

## Why is there no veto power?

Because veto implies roles.

Instead of veto, Charter Core supports session constraints:
- Declared explicitly at session start
- Applied mechanically
- Scoped only to that session

This achieves the same effect without introducing identity semantics.

---

## What are Session Constraints?

Session constraints are explicit participation requirements declared when a session starts.

Examples:
- “All present participants must agree”
- “At least 3 participants must accept”
- “Acceptance requires confirmation from Alice”

Constraints:
- Apply only to the current session
- Do not persist
- Do not modify Authority
- Are enforced by the engine

---

## Why are Session Constraints enforced by the engine?

Because legitimacy cannot depend on UI behavior.

If constraints were client-only:
- A different client could bypass them
- Decisions could be accepted prematurely
- Auditability would be compromised

Engine enforcement ensures consistency across all integrations.

---

## Why doesn’t Charter Core infer conflicts between Areas?

Because inference guesses intent.

Charter Core only acts on:
- Explicit Area references
- Explicit Authority references
- Explicit Scope references

If overlap matters, humans must say so.

This avoids accidental coupling and preserves clarity.

---

## Why doesn’t Charter Core require rationale (“why”)?

Because requiring justification creates performative compliance.

Charter Core:
- Never requires rationale to legitimize a decision
- Preserves rationale if provided
- Relies on the audit trail (sessions, candidates, supersession) to explain evolution

Legitimacy comes from process, not prose.

---

## Can Charter Core be used by a single person?

Yes.

Charter Core works with:
- One person
- Flat groups
- Hierarchies
- Machines acting as participants

The rules do not change.

---

## What happens if these rules feel too strict?

That’s intentional.

Charter Core optimizes for:
- Long-term integrity
- Auditability
- Explicit intent

Convenience belongs in higher layers.

---

## What happens if Authority or Scope changes make an old decision “no longer make sense”?

Charter Core preserves decisions exactly as they were accepted.

Later changes to Authority or Scope do not invalidate, reinterpret, or erase prior resolutions.

If a decision is no longer applicable, users must explicitly:
- Supersede it
- Retire it
- Or clarify its context through a new resolution

This ensures:
- Auditability
- Historical integrity
- Protection against retroactive rewriting

Charter favors explicit correction over silent cleanup.

---

## Why doesn’t Charter automatically retire irrelevant decisions?

Because relevance is a human judgment, not a mechanical one.
Automatically retiring decisions would erase accountability and distort history.