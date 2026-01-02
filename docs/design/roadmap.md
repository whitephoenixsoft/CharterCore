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