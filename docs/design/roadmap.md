# CLI Versioning Roadmap (Clean & Intentional)

What you’re intuitively doing already maps very well to a layered CLI evolution.

CLI V1 — Legitimacy Recording (MVP)
Goal: “I can record decisions correctly.”
Features:
Areas
Sessions
Candidates
Accept / reject
Solo authority
Status (not audit-heavy)
Export / import (basic)
No server
No participant ergonomics
This is about trusting the system.

CLI V2 — Accountability & Reflection (Current)
Goal: “I can understand what happened and who participated.”
Adds:
Audit views (participants, timelines)
Review flows (import consolidation)
Restart-from
Constraints invariants
Candidate freezing rules
Clear failure states
Grep-friendly outputs (flagged)
This is about confidence and clarity.
👉 This is where you finalize the engine APIs.

CLI V3 — Ergonomic Power (Still Local / Solo)
Goal: “I can move fast without breaking legitimacy.”
This is the phase you’re describing now.
Likely features:
Candidate bundles / templates
(pre-staged options)
Participant groups
(saved sets, not identities)
Session presets
(authority + constraints)
Shorthand commands
(still explicit underneath)
Rich annotations & notes
Better “next action” guidance
Important:
Still single-user
Still local
Still offline-capable
This is about speed and flow, not collaboration.

CLI V4 — Shared / Server Mode (GitHub Analogy)
Goal: “Multiple people share legitimacy.”
This is where:
Identity becomes real
Sessions reflect meetings
Reviews become shared
Authority maps to org structures
Permissions matter
At this point:
CLI is a client
Engine may run server-side
APIs must already be stable

The take away:
The engine defines legitimacy.
The CLI defines ergonomics.
The server defines coordination.

---
Section C — Frozen Decisions (V1 Lock-In)
These decisions are now fixed for V1.
Locked for V1
Active sessions are never exported as legitimate artifacts
Export ignores non-closed sessions with explicit warnings
Consolidation imports preserve:
Resolutions
Supersession chains
Acceptance context
Consolidation imports do not preserve:
Imported sessions
Imported candidates
Imported votes
Legitimacy is recreated locally and explicitly
No retroactive legitimacy
No mechanical replay
This is the minimal, correct, and honest system.
Section D — Deferred Feature (Explicitly Out of Scope)
CLI / Engine V2 or V3 — Imported Deliberation Context (Deferred)
This is not part of V1.
Possible future feature:
Preserve imported sessions as:
HISTORICAL
NON-LEGITIMIZING
READ-ONLY
Used only for:
Audits
Post-mortems
Cross-department transparency
Explicit constraints if implemented:
Never evaluated
Never enforced
Never blocks or enables decisions
Never confused with local legitimacy
Rationale:
Audit convenience, not core governance
Added complexity
Non-essential for solo and small-team use

---

Losing imported session history during consolidation — is that acceptable?
This is a subtler and more interesting edge case. You’re right to pause here.
Let’s restate the problem clearly
In CONSOLIDATE mode today:
Imported resolutions are preserved (as UNDER_REVIEW)
Imported sessions are effectively discarded
When a resolution is accepted locally:
A new local session is created
Imported decision-making history is not replayed
This means:
Legitimacy is clean
But decision process history may be lost
Given Charter’s emphasis on auditability, this feels wrong — even if it’s logically consistent.
Why this isn’t actually a violation (yet)
Important distinction:
Charter Core promises to preserve:
Legitimacy history
Acceptance context
Supersession chains
It does not promise to preserve:
All deliberation
All votes
All failed or alternate processes
In consolidation:
Imported sessions were legitimate in their original universe
They are not legitimate locally
Replaying them mechanically would be dishonest
So discarding them is defensible.
But your discomfort is valid — here’s the real issue
The problem is not legitimacy.
The problem is forensic audit value.
In some environments (regulated orgs, cross-department review, post-mortems), people may ask:
“How was this decided over there?”
“Was this contentious?”
“Who objected?”
“What alternatives were rejected?”
Right now, consolidation throws that away.
That doesn’t break Charter — but it limits its usefulness at scale.
The key insight: two kinds of history
What you’re circling is an important distinction:
1. Legitimacy history (must be preserved)
Accepted resolutions
Authority and scope context
Supersession chains
Audit events
2. Deliberation history (optional, non-legitimizing)
Sessions
Candidates
Votes
Deadlocks
Abandoned processes
Charter Core must preserve (1).
Charter Core may optionally preserve (2), but must never treat it as authoritative.
A clean V2 design that doesn’t contaminate V1
Here’s a way to support your concern without complicating the engine now.
V1 (what you should lock in now)
CONSOLIDATE imports:
Import resolutions only
Preserve original resolution context (authority/scope refs)
Do not import sessions
This is documented clearly
No audit guarantees about deliberation history across systems
This keeps V1 minimal and honest.
V2 (if and when it’s worth it)
Add a non-legitimizing imported history lane:
Imported sessions are stored as:
HISTORICAL_SESSION or IMPORTED_CONTEXT
They are:
Read-only
Clearly marked as non-local
Never used for acceptance
CLI and audits can show:
“This resolution was decided elsewhere under these conditions”
Crucially:
Engine never evaluates them
They never block or enable anything
They exist purely for audit and human understanding
This preserves Charter’s integrity while expanding usefulness.

Section C — Frozen Decisions (V1 Lock-In)
These decisions are now fixed for V1.
Locked for V1
Active sessions are never exported as legitimate artifacts
Export ignores non-closed sessions with explicit warnings
Consolidation imports preserve:
Resolutions
Supersession chains
Acceptance context
Consolidation imports do not preserve:
Imported sessions
Imported candidates
Imported votes
Legitimacy is recreated locally and explicitly
No retroactive legitimacy
No mechanical replay
This is the minimal, correct, and honest system.
Section D — Deferred Feature (Explicitly Out of Scope)
CLI / Engine V2 or V3 — Imported Deliberation Context (Deferred)
This is not part of V1.
Possible future feature:
Preserve imported sessions as:
HISTORICAL
NON-LEGITIMIZING
READ-ONLY
Used only for:
Audits
Post-mortems
Cross-department transparency
Explicit constraints if implemented:
Never evaluated
Never enforced
Never blocks or enables decisions
Never confused with local legitimacy
Rationale:
Audit convenience, not core governance
Added complexity
Non-essential for solo and small-team use
