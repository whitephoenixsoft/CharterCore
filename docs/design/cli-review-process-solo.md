# Review Process

> Review is not just about new resolutions — it is about resolving conflicts between timelines.

## Command Set

```bash
charter review list
charter review open <import-id>
charter review status
charter review show [resolution-label]
charter review accept <resolution-label> | <import-id> [--force]
charter review reject <resolution-label> | <import-id>
charter review close
charter review purge <import-id>
```

## Command Semantics & Messages

### 1. charter review list

Lists all known imports.

```text
$ charter review list

IMPORTS
I-1  alice-laptop.json      12 resolutions  UNDER_REVIEW
I-2  ci-backup.json         38 resolutions  UNDER_REVIEW
I-3  prod-restore.json      42 resolutions  APPLIED

Next actions:
- charter review use I-1
- charter review use I-2
```
Rules
- Sorted by import time (newest first)
- Status is summarized
- No mutation

---
### 2. charter review open \<import-id>

Enters a scoped review context.

```text
$ charter review open I-1

Entered review context:
Import: I-1
Source: alice-laptop.json
Resolutions under review: 12

Next actions:
- charter review show
- charter review accept <resolution>
- charter review reject <resolution>
- charter review close
```

Rules:
- Fails if import does not exist
- Fails if another review is already active
- Clears any prior review context

Behavior:
- Enters a review workspace
- Scopes all subsequent review commands
- No engine mutation

Error if fails due to another review 
```
error: review I-4 already active
next actions:
  - charter review show
  - charter review accept I-4
  - charter review reject I-4
  - charter review close I-4
```

---
### 3. charter review status

Shows progress for the active review context.

```text
$ charter review status

REVIEW CONTEXT
Import: I-1
Source: alice-laptop.json

Resolutions:
- Accepted: 4
- Rejected: 2
- Remaining: 6

Blocking notes:
- Authority change pending review

Next actions:
- charter review show
- charter review accept <resolution>
- charter review reject <resolution>
```
Rules:
- Fails if no review context
- Purely informational

Must clearly display:
- Import ID
- Resolution label
- Imported resolution state:
    - UNDER_REVIEW
    - CONSOLIDATED
    - REJECTED
    - HISTORICAL (superseded on import)
- Supersession info (if applicable):
    - “Supersedes local: R-3”
- Optional provenance:
    - Imported session problem statement (read-only)
    - Imported authority + scope labels

 No semantic diff. No judgment.

---
### 4. charter review show \[resolution-label]

#### Without argument

Shows all remaining items under review.

```text
$ charter review show

UNDER REVIEW
R-ARCH-2   "Adopt microservices"
R-SCOPE-5  "Expand deployment scope"
R-DB-3     "Switch to PostgreSQL"

Next actions:
- charter review accept <resolution>
- charter review reject <resolution>
```
With supersession aware output:
```
$ charter review show

UNDER REVIEW (Import: I-2)

R-ARCH-5  "Adopt microservices"
  ⚠ Supersedes local resolution: R-ARCH-3

R-SCOPE-2 "Expand deployment responsibility"

Next actions:
- charter review show R-ARCH-5
- charter review accept <resolution>
- charter review reject <resolution>
```

#### With argument

Shows full details.

```text
$ charter review show R-ARCH-2

RESOLUTION (IMPORTED)
Label: R-ARCH-2
Original Area: Platform
Accepted: 2024-11-02
Authority (imported): UNANIMOUS_PRESENT
Scope (imported): Platform Architecture

Status: UNDER_REVIEW
```
With supersession aware output:
```
$ charter review show R-ARCH-5

IMPORTED RESOLUTION
Label: R-ARCH-5
Area (imported): Platform
Status (imported): ACTIVE
Accepted: 2024-10-14

Problem Statement (imported session):
"Re-evaluate architectural direction for scaling concerns"

Authority (imported):
UNANIMOUS_PRESENT

Scope (imported):
Platform Architecture

Supersession Impact:
- This resolution supersedes local resolution: R-ARCH-3
- R-ARCH-3 is currently ACTIVE

Status: UNDER_REVIEW
```
Key points:
- Problem statement is shown (huge win for comprehension)
- Supersession is spelled out
- No semantic comparison
- No “preview diff”

This preserves Charter’s philosophy while making review humane.

---
### 5. charter review accept \<resolution-label>

Accepts one resolution from the active import.

```text
$ charter review accept R-ARCH-5

Accepting imported resolution R-ARCH-5
From import: I-2

This will:
- Create resolution R-ARCH-5 locally
- Mark local resolution R-ARCH-3 as SUPERCEDED
- Preserve both in audit history

Proceed? [y/N]
```
On confirmation: 
```
Resolution accepted.

R-ARCH-5 → ACTIVE
R-ARCH-3 → SUPERCEDED

Next actions:
- charter review show
- charter review close
```

Rules
Fails if:
no review context
resolution not in that import
resolution already handled

Why this is important:
Makes supersession a conscious act
Prevents accidental history rewrites
Aligns with your “no silent legitimacy shifts” rule

Creates a local resolution with full provenance

#### charter review accept \<import-id>
```
$ charter review accept I-4

3 resolutions eligible for batch acceptance:
- R-DOC-1
- R-OPS-2
- R-API-7

All are new (no local supersession).

Proceed? [y/N]
```
After:
```
Accepted 3 resolutions.
0 conflicts.
4 remaining UNDER_REVIEW.

Next actions:
- charter review show
- charter review accept --force I-4
- charter review close
```
##### Meaning
- Accept only imported resolutions that have no local equivalent
- i.e.:
    - no supersession relationship
    - no conflict
- These are “pure additions”

###### Mechanics
- For each resolution:
    - Create a single-purpose session internally
    - Accept exactly one candidate
    - Session annotation:
    > “Imported from I-4 (batch accept — new resolution)”
- Session auto-closes
- Resolution becomes ACTIVE locally

Rules:
- Accepts only new resolutions
- Skips any imported resolution that:
    - supersedes an active local resolution
- Each acceptance:
    - creates a hidden single-candidate session
    - auto-accepts
    - auto-closes
    - adds resolution annotation:
        “Consolidated from import \<import-id>”

Resulting state:
- Imported resolution → CONSOLIDATED
- Local resolution → ACTIVE

#### Forced Batch Accept (--force)
```
$ charter review accept I-4 --force

WARNING: This will accept all imported resolutions, including those that:
- Supersede existing local resolutions
- Change active governance history

This will:
- Create 7 new resolutions
- Supersede 3 currently ACTIVE resolutions

Proceed? [type ACCEPT to continue]
```
Meaning
- For every UNDER_REVIEW resolution, including ones that:
    - supersede a local active resolution
- The engine will:
    - create a new resolution
    - supersede the currently active one
- One at a time
- Fully auditable

Rules:
- Accepts all UNDER_REVIEW resolutions
- Including those that supersede local active ones
- Each accepted resolution:
    - supersedes the relevant local resolution
    - preserves full audit trail
- Strong confirmation required

---
### 6. charter review reject \<resolution-label>

Explicitly rejects an imported resolution.

```text
$ charter review reject R-SCOPE-5

Rejected resolution R-SCOPE-5
Imported from: I-1
Status: REJECTED (local)

Next actions:
- charter review show
- charter review accept <resolution>
```
Rules
Rejection is explicit and auditable
Rejected resolutions remain visible in audit

####  charter review reject \<import-id>
```
$ charter review reject I-4

This will:
- Reject 7 imported resolutions
- Leave local history unchanged

Rejected resolutions remain visible until review is closed.

Proceed? [y/N]
```
After:
```
Import I-4 status:
- UNDER_REVIEW: 0
- REJECTED: 7
- CONSOLIDATED: 0

Next actions:
- charter review close
- charter review show --rejected
```
This Means:
- For import I-4
- All resolutions with state UNDER_REVIEW
- Transition to REJECTED
- No engine sessions
- No local resolutions created
- No local history touched

Rules:
- Rejects all UNDER_REVIEW resolutions in the import
- No engine sessions
- No local state change
- Imported resolutions become REJECTED
- Reversible until review is closed

---
### 7. charter review close 

Leaves review mode safely.

> Review is mutable until closed.
> Close is final.

```bash
$ charter review close I-4

Closing review for import I-4.

Final status:
- CONSOLIDATED: 3
- REJECTED: 4
- UNDER_REVIEW: 0 (abandoned)

This review can no longer be modified.

Next actions:
- charter import purge I-4
- charter audit area
```
Rules
- No side effects
- Required before switching imports
#### Meaning

- Finalizes your stance toward import I-4
- Locks review states:
    - UNDER_REVIEW → ABANDONED 
    - REJECTED → final
    - CONSOLIDATED → final
- Review workspace is archived
- Import can now be purged
#### While Review Is Open

- REJECTED resolutions:
    - can be accepted later
- CONSOLIDATED resolutions:
    - remain legitimate local resolutions
    - rejecting later means:
        - you must supersede them explicitly
        - via a new acceptance (review or session)
Rules:
- Finalizes the review
- Locks all review decisions
- Remaining UNDER_REVIEW → ABANDONED 
- Import becomes purge-eligible
- No further accept/reject allowed

There is no undo, only supersession — which is exactly Charter’s philosophy.

---
### 8. charter review purge \<import-id>

Removes an import only if no resolutions remain under review.

```text
$ charter review purge I-1

Cannot purge import I-1
Reason: 6 resolutions still under review

Next actions:
- charter review use I-1
```

Successful case:
```text
$ charter review purge I-3

Purged import I-3
(All resolutions previously applied)

Next actions:
- charter review list
```

---
## Hard Guardrails (Non-Negotiable)
❌ No accepting without an active review context
❌ No guessing which import a resolution belongs to
✔ All actions show provenance

----
## End-to-end 

### Scenario

Solo user adopts Charter with existing decisions from another system.

Step 1 — Import Flat File (Consolidation Mode)
```
$ charter import decisions.json --mode consolidate

Imported file: decisions.json
Detected: 27 resolutions
No sessions found

All resolutions marked: UNDER_REVIEW

Import ID: I-4

Next actions:
- charter review use I-4
- charter review list
```
Step 2 — Enter Review Mode
```
$ charter review use I-4

Entered review context:
Import: I-4
Source: decisions.json
Resolutions under review: 27

Next actions:
- charter review show
```
Step 3 — Inspect
```
$ charter review show

UNDER REVIEW
R-1   "Adopt PostgreSQL"
R-2   "Use AWS ECS"
R-3   "Deprecate legacy API"

Next actions:
- charter review show R-1
- charter review accept <resolution>
- charter review reject <resolution>
```
Step 4 — Accept & Reject
```
$ charter review accept R-1

Accepted resolution R-1
Imported from: I-4
Status: ACTIVE
```

```
$ charter review reject R-2

Rejected resolution R-2
Imported from: I-4
Status: REJECTED
```
Step 5 — Progress Check
```
$ charter review status

Accepted: 1
Rejected: 1
Remaining: 25

Next actions:
- charter review show
```
Step 6 — Close Review
```
$ charter review close

Exited review context: I-4

Next actions:
- charter review list
- charter status
```
Step 7 — Purge (Later)
```
$ charter review purge I-4

Cannot purge import I-4
Reason: resolutions still under review
```
After completion:
```
$ charter review purge I-4

Purged import I-4
```

---
## Imported Resolution States

| State        | Meaning                                           |
| :----------- | :------------------------------------------------ |
| UNDER_REVIEW | Imported, unresolved locally                      |
| CONSOLIDATED | Explicitly accepted into local timeline           |
| REJECTED     | Explicitly rejected locally                       |
| HISTORICAL   | Was already superseded within the import timeline |
| ABANDONED    | Rejected by default due to a closed review        |
### How HISTORICAL Works

Example:
- Import contains R-1 → R-2 → R-3
- Only R-3 was active in that system
- R-1 and R-2 arrive as:
```
R-1  HISTORICAL (superseded in import)
R-2  HISTORICAL (superseded in import)
R-3  UNDER_REVIEW
```
CLI behavior:
- HISTORICAL resolutions:
    - are visible
    - are never actionable
    - exist for audit context only

This avoids cognitive noise while preserving provenance.