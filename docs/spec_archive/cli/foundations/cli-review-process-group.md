# Baseline Review Lifecycle & States — Non-Solo Mode

This is the important clarification:

> Non-solo mode changes who must agree — not how reviews behave structurally.

## Review as a First-Class Evaluation Space

A Review is not:
- a session
- a vote
- a merge
- a legitimacy act

A Review contains potential legitimacy acts.

## Review Lifecycle (Non-Solo)

### 1. CREATED

- Triggered by charter baseline open \<import-id>
- Review is bound to:
    - a single import
    - a single Area
- No resolutions are modified yet.

CLI output emphasizes:
- source of import
- number of resolutions
- whether they conflict / supersede local ones

### 2. ACTIVE (default)

Resolutions inside the Baseline Review may be in one of the following review states:

| Review State | Meaning                                                    |
| :----------- | :--------------------------------------------------------- |
| UNDER_REVIEW | Awaiting evaluation                                        |
| HISTORICAL   | Imported but already superseded in the source timeline     |
| REJECTED     | Explicitly rejected by the reviewing group                 |
| CONSOLIDATED | Accepted into the local Area via sessions                  |
| ACTIVE       | Exists and active in the local Area (no action necessary ) |

Key rule:
- No resolution may transition to CONSOLIDATED without a valid session under the Area’s Authority.

### 3. CONSOLIDATION (implicit phase)

This is where non-solo mode matters.

For each resolution being accepted:
- A session must be opened (explicitly or CLI-assisted)
- Participants must be defined
- Authority rule must be satisfied
- Acceptance creates a new local resolution
- Imported resolution remains as historical reference

The Review itself does not:
- vote
- approve
- legitimize

It only organizes work.

### 4. CLOSED

Triggered by charter baseline close.

Rules:
- No further accept/reject actions allowed
- Baseline Review becomes immutable history
- Imported resolutions remain queryable
- Rejected resolutions may still be re-imported later via a new Baseline Review

## Why this works in non-solo mode

- Teams evaluate one proposal set at a time.
- Legitimacy is preserved via sessions
- No implicit approvals occur
- Baseline Reviews don’t replace meetings — they structure them
