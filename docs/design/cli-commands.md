# Charter Core — CLI Simulation (Single-User Mode)

> This simulation demonstrates Charter Core used as a **local decision ledger** by a single developer. All legitimacy rules still apply.

## Assumptions (Implicit, but Important)

- User is operating locally
- No remote sync
- No AI
- No permissions system exposed yet
- Single-user bootstrap authority is allowed **only for initialization**

## 1. Create an Area

```Bash
charter area create "Backend Architecture"
```
### Output 
```Text
Area created:
  ID: A-1
  Name: Backend Architecture

This Area has no Authority or Scope.
Initialization session required.
```

## 2. Initialize Area (Authority + Scope)

Charter **forces** an initialization session.
```Bash
charter session start --area A-1 --init
```
### Output 
```Text
Session S-1 started (Initialization)
Problem:
  Define Authority and Scope for Area A-1
```

## Context & Navigation

### charter init

```Bash
charter init
```

Behavior:
- initializes storage e.g. `.charter/`
- Recommends to create an area with an example or start importing from a file with examples.
- creates initial area and runs auto authority and scope creation like area create

### charter area create
```Bash
charter area create A-PLATFORM --name "Platform Engineering"
```
Behavior:
- creates the domain and objects and references
- If it is the first area then switch to that area context 
- Warns to change scope if working with limited scoped project
- creates initial authority of solo mode and display as auto created 
- creates initial scope of unrestricted (text) and display as auto created
- shows example how to create a resolution, quickly 
### charter area use
```Bash
charter area use A-PLATFORM
```
Behavior:
- switch to area context 
- Fails if an active session exists
- Prints Authority + Scope on success
### charter area list
```Bash
charter area list
```
Behavior:
- lists all the area short name and full names.
### charter area status
```Bash
charter area status
```
Behavior:
- displays sessions active, blocked, or paused
- displays the number of active resolutions
## Sessions

### charter session start
```Bash
charter session start "Choose database"
```
Option flags:
--preceding R-DB-1
--ref-area finance
--ref-resolution finance/R-12
--participants Alice,Bob,Charlie (maybe needs one per participant)

Behavior:
- Fails if Area uninitialized
- Fails if another session is active. Will warn the user to pause the session before trying again.
- Captures Authority, Scope, Participants, Preceding Resolution 

### charter session status

charter session status

Shows:
- Problem statement
- Authority rule
- Participants
- Candidates
- Vote summary
- Blocked / Paused state

### charter session pause

charter session pause "Need security input"

- required for context switching 

### charter session resume

charter session resume

Behavior:
- Revalidates Authority, Scope, Preceding Resolution
- Blocks if context changed

### charter session close

charter session close "Problem redefined"

### charter session evaluate (informational)

charter session evaluate

Returns:
- ACCEPTABLE
- INCOMPLETE
- BLOCKED (with reason)
### charter session accept 

charter session accept C-1

Behavior:
- Validates Authority using participant list
- Fails if blocked or paused
- Creates Resolution
- Supersedes preceding resolution if specified
- Session remains open unless user closes it

## Candidates & Decisions

### charter candidate add

charter candidate add "Use PostgreSQL"
charter candidate add "Use PostgreSQL" --why "Operational familiarity"

### charter candidate list

charter candidate list

### charter vote

charter vote accept C-1
charter vote reject C-2
charter vote abstain C-3

Notes:
- Votes are per participant (single-user CLI defaults to current actor)
- Re-voting overwrites previous stance
- All votes are auditable

### charter vote status

charter vote status

Displays:
- Participants
- Stances per candidate
- Authority evaluation preview

## Inspection & Legitimacy

### charter resolution list

charter resolution list

Filters:
--active
--under-review
--superseded
### charter resolution show

charter resolution show R-DB-1

Includes:
- Acceptance session
- Authority at acceptance
- Scope at acceptance
- Superseded resolution (if any)

### charter resolution (marking)

charter resolution mark-under-review R-DB-1
charter resolution clear-under-review R-DB-1

### charter session for authority change 

charter session start "Change decision rule"
charter candidate add "UNANIMOUS_PRESENT"
charter session accept-authority C-1

### charter session for scope change 

charter session start "Update scope definition"
charter candidate add "Applies to backend services only"
charter session accept-scope C-1
### charter audit area

Potential flags
--compact (grep-first)
--pretty (human) or --verbose
--json (tools)

Example
charter audit area platform

Output 
AREA: platform

Initialized:
  Authority: R-AUTH-1 (Unanimous Present)
  Scope: R-SCOPE-1 (Platform architecture decisions)

Authority Changes:
  - R-AUTH-2 (Majority Present) superseded R-AUTH-1

Scope Changes:
  - R-SCOPE-2 superseded R-SCOPE-1

Resolutions:
  - R-3 (Active): Adopt service mesh
  - R-1 (Superseded): Use monolith
  - R-2 (Retired): Vendor A CDN

Blocked Sessions:
  - S-7 (deadlock on API gateway)

Audit Guarantees:
  ✔ No retroactive changes
  ✔ All legitimacy paths intact

#### another view

charter audit area A-INFRA --verbose

Resolution R-DB-1
Accepted: 2025-03-15T09:30Z
Session: S-4 ("Choose primary database")
Authority: R-AUTH-1 (Unanimous Present)
Scope: R-SCOPE-1
Annotation:
  Decision made during outage recovery.

#### grep friendly 

2025-03-14T10:32Z AREA A-INFRA INIT
2025-03-14T10:35Z AUTH R-AUTH-1 ACCEPT session=S-1
2025-03-14T10:36Z SCOPE R-SCOPE-1 ACCEPT session=S-1
2025-03-15T09:12Z SESSION S-4 START "Choose primary database"
2025-03-15T09:20Z CANDIDATE C-1 ADD "PostgreSQL"
2025-03-15T09:30Z RESOLUTION R-DB-1 ACCEPT auth=R-AUTH-1 scope=R-SCOPE-1

Why this works:
grep postgres
grep AUTH
grep 2025-03-15
grep session=S-4
### charter audit resolution
Example 
charter audit resolution platform/R-3
Output 
RESOLUTION: platform/R-3

Status: ACTIVE
Supersedes: platform/R-1

Accepted In:
  Session: S-12
  Problem: "Adopt service mesh?"

Authority at Acceptance:
  R-AUTH-2 (Majority Present)

Scope at Acceptance:
  R-SCOPE-2

Participants:
  Alice (ACCEPT)
  Bob (ACCEPT)
  Charlie (ABSTAIN)

Candidates Considered:
  - Use service mesh (ACCEPTED)
  - Keep monolith (REJECTED)

Notes:
  - Rationale preserved (optional)
  - No retroactive evaluation occurred

#### another view

charter audit resolution R-DB-1

2025-03-15T09:12Z Session S-4 opened
2025-03-15T09:20Z Candidate C-1 added
2025-03-15T09:25Z Alice ACCEPT
2025-03-15T09:27Z Bob ACCEPT
2025-03-15T09:30Z Resolution accepted
2025-06-01T11:10Z Marked UNDER_REVIEW

## Persistence

### charter export

charter export area A-PLATFORM > platform.charter.json
### charter import

#### charter import area
charter import area platform.charter.json --mode consolidate

Special case (first-time bootstrap):
- Flat resolution list → all marked UNDER_REVIEW
##### charter import list 

charter import list
→ Shows only:
  - unresolved imports
  - imports with UNDER_REVIEW resolutions

charter import list --all

- To show all imports 
#### charter import show

charter import show <import-id>

- show the details of a specific import

#### charter import prune 

charter import prune

- Delete any resolved imports 

#### charter review 

charter review start <import-id>
charter review list
charter review accept --all
charter review close

Or

charter review start <import-id>
charter review list
charter review reject --all
charter review close

Audit output 

Session S-42 — Import Review
Area: Platform Architecture
Authority: Solo Authority v3
Scope: Platform Decisions
Source Import: import-2025-03-12.tar

Accepted:
- R-IMP-7  (“Adopt gRPC for internal APIs”)
Rejected:
- R-IMP-9  (“Migrate to NoSQL datastore”)
Consolidated:
- R-IMP-6 → R-LOCAL-12

Annotations:
- “Accepted with modifications — see R-LOCAL-12”

Grep output 

SESSION S-42 TYPE=REVIEW IMPORT=import-2025-03-12 AREA=A-1 AUTH=R-AUTH-3
REVIEW ACCEPT RES=R-IMP-7
REVIEW REJECT RES=R-IMP-9
REVIEW CONSOLIDATE RES=R-IMP-6 LOCAL=R-LOCAL-12
SESSION CLOSE S-42

Resolution level audit accepted 

Resolution: R-IMP-7
Status: ACCEPTED
Imported From: import-2025-03-12
Reviewed In Session: S-42

Resolution level audit rejected 

Resolution: R-IMP-9
Status: REJECTED
Imported From: import-2025-03-12
Rejected In Session: S-42

Notes:
- after import review all new resolutions
- can accept some or all of them 
- this will launch sessions (silently) where the new resolutions are be approved.
- Rejection requires an explicit session, even if batched.

Design change:
- add to resolution: 
    - derived_from_import: [imported_resolution_hash]
    - import_source: { file, timestamp, origin_area }
- text about the reference must show reference and not Supercession
        -  informed_by: Imported Resolution R'-5
- add more resolution states just for the import 

Status | Meaning
:- |  :-
UNDER_REVIEW | imported, awaiting local legitimacy
ACCEPTED | Accepted via a local session
REJECTED | Explicitly rejected via review
CONSOLIDATED | Accepted and linked to an existing local resolution
HISTORICAL | Imported but never eligible for review

Why this is worth it:
- REJECTED ≠ deleted
- CONSOLIDATED makes audits dramatically clearer
-  avoids semantic overloading of “superseded”

And importantly:
- These statuses never change imported history — only local interpretation.

How consolidation works (clean rule)

If the user accepts an imported resolution but an equivalent local resolution already exists:
- The imported one becomes CONSOLIDATED
- The local resolution gets:
    informed_by: imported_hash
- No supersession occurs unless explicitly decided

## End to end

```Bash
charter area create A-PLATFORM
charter area use A-PLATFORM

charter session start "Initialize governance"
charter candidate add "UNANIMOUS_PRESENT"
charter session accept-authority C-1

charter session start "Define scope"
charter candidate add "Platform architecture decisions"
charter session accept-scope C-1

charter session start "Choose database"
charter candidate add "PostgreSQL"
charter candidate add "MongoDB"

charter vote accept C-1
charter vote reject C-2

charter session evaluate
charter session accept C-1
```

Note:
CLI Ergonomic Invariant When the active Authority rule makes individual voting redundant (e.g., SOLO), the CLI may collapse vote + accept into a single user action, while preserving full mechanical history in the engine.

---

Extra

charter session participants list
charter session participants add <id>
charter session participants remove <id>

To confirm session resume:
charter session participants set <id>

With more than one participant:
charter vote accept C-1 --by alice
charter vote accept C-1 --by bob

---
CLI v2 audit commands 

Recommended V2 Audit Command Additions
1. charter audit participants
Purpose:
Summarize participation mechanically.
Example output:
Copy code
Text
Participant Summary (Area: platform)

Alice
  Sessions participated: 12
  Votes recorded: 31
  Resolutions accepted: 8
  Authority changes participated in: 1

Bob
  Sessions participated: 7
  Votes recorded: 14
  Resolutions accepted: 2
Notes:
No scoring
No ranking
No value judgments
2. charter audit by-participant <actor_id>
Purpose:
Answer: “What did this person participate in?”
Output:
Copy code
Text
Actor: Alice

Session S-14
  Problem: "Choose deployment strategy"
  Stance: ACCEPT on C-2
  Outcome: Resolution R-DEPLOY-3 accepted

Session S-18
  Problem: "Change on-call rotation"
  Stance: REJECT on C-1
  Outcome: No resolution
This is incredibly useful during:
retrospectives
trust repair
onboarding
compliance reviews
3. charter audit resolution --participants
Adds a section:
Copy code
Text
Participants at Acceptance
  Alice — ACCEPT
  Bob — ACCEPT
  Carol — ABSTAIN

Authority Rule:
  Unanimous of present participants
This already exists mechanically — V2 just surfaces it clearly.
4. charter audit timeline --actors
Timeline view with actors explicitly rendered:
Copy code
Text
[2025-03-02] SESSION START S-22
  Participants: Alice, Bob

[2025-03-02] STANCE
  Alice ACCEPT C-1

[2025-03-02] STANCE
  Bob ACCEPT C-1

[2025-03-02] RESOLUTION ACCEPTED R-SEC-4
This becomes grep-friendly and human-readable.
---
Participant Auditing — Locked for CLI V2 ✅
These are valid, necessary, and aligned with the transparency goal. They are CLI audit surfaces over engine data.
Locked Commands
Copy code
Bash
charter audit participants
Purpose
List all known participants across sessions
Show first-seen / last-seen timestamps
Show session counts
Copy code
Bash
charter audit by-participant <actor_id>
Purpose
Timeline view of everything a participant was involved in:
Sessions joined
Votes cast
Acceptances participated in
Resumes confirmed
Copy code
Bash
charter audit resolution <resolution-id> --participants
Purpose
Show participant snapshot(s) relevant to a resolution:
Session participants at acceptance
Any resumes before acceptance
Required approvers (if constraints existed)
Copy code
Bash
charter audit timeline --actors
Purpose
Chronological audit stream including:
Participant joins/leaves
Resume confirmations
Authority-relevant participant changes

----
Potential redo

CLI Command Set (Consolidated & Agreed)
Below is the current recommended CLI surface, reflecting everything we’ve agreed on.
Area
Copy code
Bash
charter init
charter area create <name>
charter area use <area>
charter area status
Session
Copy code
Bash
charter session start
charter session status
charter session pause
charter session resume
charter session close
charter session restart-from <session-id>
Participants
Copy code
Bash
charter session participants show
charter session participants add <id>
charter session participants remove <id>
Candidates
Copy code
Bash
charter candidate add
charter candidate list
charter candidate remove   # only before first stance
Voting / Acceptance
Copy code
Bash
charter vote <candidate> accept|reject|abstain
charter session accept <candidate>   # solo-mode ergonomic collapse
Authority & Scope
Copy code
Bash
charter authority set
charter scope set
(each creates a session under the hood, per your design)
Baseline Review (Imports)
Copy code
Bash
charter import consolidate <file>
charter import restore <file>

charter baseline status
charter baseline show
charter baseline accept [--all] [--force]
charter baseline reject [--all]
charter baseline close
Status & Audit
Copy code
Bash
charter status
charter status session
charter status area

# non-MVP but supported
charter audit session
charter audit resolution
charter audit area

----

