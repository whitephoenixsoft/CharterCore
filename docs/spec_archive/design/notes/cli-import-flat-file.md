# Flat File Import — Baseline Review (CLI V1)

## Purpose

Flat file import exists to bootstrap or consult Charter Areas when formal governance history does not yet exist or is maintained externally.

This mechanism prioritizes:
- Speed of onboarding
- Preservation of foreign provenance
- Explicit legitimacy (never inferred)

Flat file imports never create legitimacy. They only materialize candidate resolutions for review.

## Flat File Import Process (Conceptual)

- User provides a plain text file containing one or more resolutions.
- CLI parses the file into discrete resolution blocks.
- Each block becomes an imported resolution object.
- All imported resolutions are marked UNDER_REVIEW.
- No Authority or Scope is inferred.
- User must explicitly review and accept or reject each resolution via baseline review.
- Acceptance creates local resolutions via sessions (even if hidden ergonomically).
- Flat imports always enter the baseline review workflow.

# Flat File Format (CLI V1)
Design Goals
Human-editable
Multi-line safe
Git-friendly
Zero semantic inference
Format Rules
--- denotes the start of a new resolution
Everything until the next --- belongs to that resolution
Content is preserved verbatim
Optional lightweight headers may exist (e.g. title:) but are non-semantic
No ordering, hierarchy, or authority is implied
Example (Illustrative)
title: Logging retention policy
Logs must be retained for a minimum of 180 days.
This includes:
Application logs
Audit logs
Access logs
title: Security approval required
All production deployments must be approved by Security.
Repeated Imports (Consultation Use Case)
Flat file imports are always treated as foreign
Even previously accepted content re-enters as UNDER_REVIEW
CLI may compare content hashes against active local resolutions
Identical content may be presented as:
“UNCHANGED (matches active resolution R-X)”
This is presentation only
Acceptance is still required to legitimize or reaffirm
No foreign content is ever auto-accepted.

Example Import Document:
```
---
title: Security approvals required for production deploys
---
All production deployments must be approved by Security.

This applies to:
- Backend services
- Infrastructure changes
- Emergency hotfixes

Exceptions must be documented.
---
title: Logging retention policy
---
Logs must be retained for a minimum of 180 days.

This includes:
- Application logs
- Audit logs
- Access logs
```