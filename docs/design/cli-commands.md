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

charter unit

Behavior:
- Sets up storage

### charter area create

charter area create A-PLATFORM --name "Platform Engineering"

### charter area use

charter area use A-PLATFORM
Behavior:
- Fails if an active session exists
- Prints Authority + Scope on success
### charter area list

charter area list
### charter area status

charter area status
## Sessions

### charter session start

charter session start "Choose database"

Option flags:
--preceding R-DB-1
--reference-scope R-SCOPE-3

Behavior:
- Fails if Area uninitialized
- Fails if another session is active
- Captures Authority, Scope, participants

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

## Persistence

### charter export

charter export area A-PLATFORM > platform.charter.json
### charter import

charter import area platform.charter.json --mode consolidate

Special case (first-time bootstrap):
- Flat resolution list → all marked UNDER_REVIEW
## End to end

```Bash
charter area create A-PLATFORM
charter area switch A-PLATFORM

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