Perfect choice. A transcript is the truth serum for this design.
Below is a canonical solo-mode CLI transcript, end to end, that exercises:
initialization
area usage
authority & scope (explicit, but ergonomic)
sessions
candidates
acceptance
audit
a scope change
a superseding resolution
import/export (solo handoff)
I’ll annotate lightly, but the transcript itself should mostly stand on its own.
Charter CLI — Solo Mode Canonical Transcript
Assumption:
Single-user, offline
Engine IDs exist but CLI shows labels by default
Authority defaults to SOLO only via charter init
1. Initialize Charter
Copy code
Bash
$ charter init
Output:
Copy code

Initialized Charter repository.

Created Area: default

Authority initialized:
  auth-solo
  Rule: Sole present participant may accept decisions

Scope initialized:
  scope-unrestricted
  Rule: All decisions within this Area are in scope
State:
Storage initialized
Area default exists
Authority + Scope are explicit resolutions
System is usable immediately
2. Start a Session (First Real Decision)
Copy code
Bash
$ charter session start "Choose primary database"
Output:
Copy code

Session started: S-1
Area: default
Authority: auth-solo
Scope: scope-unrestricted
3. Add Candidates
Copy code
Bash
$ charter candidate add "PostgreSQL"
Copy code

Candidate added: C-1
Copy code
Bash
$ charter candidate add "MySQL"
Copy code

Candidate added: C-2
Copy code
Bash
$ charter candidate add "SQLite"
Copy code

Candidate added: C-3
4. List Candidates (Optional, Ergonomic)
Copy code
Bash
$ charter candidate list
Copy code

C-1  PostgreSQL
C-2  MySQL
C-3  SQLite
5. Accept a Candidate (Solo Shortcut)
Copy code
Bash
$ charter session accept C-1
Output:
Copy code

Candidate accepted: C-1
Resolution created: R-1
Session closed: S-1
What happened mechanically:
Implicit ACCEPT stance recorded
Authority rule validated (SOLO)
Resolution R-1 created
Session closed immutably
6. Audit the Area (High-Level)
Copy code
Bash
$ charter audit area
Output:
Copy code

Area: default

Active Authority:
  auth-solo

Active Scope:
  scope-unrestricted

Resolutions:
  R-1  PostgreSQL chosen as primary database
(No semantic interpretation — just facts.)
7. Inspect a Resolution (Detail)
Copy code
Bash
$ charter audit resolution R-1
Output:
Copy code

Resolution: R-1
Area: default
Content: PostgreSQL
Status: ACTIVE

Accepted in session: S-1
Authority at acceptance: auth-solo
Scope at acceptance: scope-unrestricted
8. Change Scope (Explicit Governance Evolution)
Later, the user realizes decisions should be constrained.
Copy code
Bash
$ charter session start "Define database decision scope"
Copy code

Session started: S-2
Copy code
Bash
$ charter candidate add "Only relational databases used for core services"
Copy code

Candidate added: C-4
Copy code
Bash
$ charter session accept C-4
Copy code

Scope resolution accepted: scope-2
Previous scope superseded: scope-unrestricted
Session closed: S-2
9. Supersede a Resolution
The user wants to refine the database decision.
Copy code
Bash
$ charter session start "Refine database decision" --supersede R-1
Copy code

Session started: S-3
Superseding: R-1
Copy code
Bash
$ charter candidate add "PostgreSQL 16 with logical replication"
Copy code
Bash
$ charter session accept C-5
Copy code

Resolution created: R-2
Superseded: R-1
Session closed: S-3
10. Audit Shows Lineage Clearly
Copy code
Bash
$ charter audit area
Copy code

Area: default

Active Authority:
  auth-solo

Active Scope:
  scope-2

Resolutions:
  R-2  PostgreSQL 16 with logical replication
  R-1  (superseded)
11. Export for Handoff or Backup
Copy code
Bash
$ charter export --area default --output decisions.charter.json
Copy code

Export complete.
Includes full history and audit trail.
12. Import on Another Machine (Consolidate)
Copy code
Bash
$ charter import decisions.charter.json
Copy code

Import complete.
2 resolutions imported under review.
(If this were a fresh system, they’d all be UNDER_REVIEW.)
13. Review Imported Resolutions
Copy code
Bash
$ charter review list
Copy code

R-1  PostgreSQL chosen as primary database
R-2  PostgreSQL 16 with logical replication
Copy code
Bash
$ charter review accept R-2
Copy code

Resolution accepted: R-2
Copy code
Bash
$ charter review reject R-1
Copy code

Resolution rejected (remains historical, not consolidated).
14. Final State Is Clean and Auditable
Copy code
Bash
$ charter audit area
Copy code

Area: default

Active Authority:
  auth-solo

Active Scope:
  scope-2

Resolutions:
  R-2  PostgreSQL 16 with logical replication

15. Blocked session
$ charter session start "Choose cache strategy"

error: another session is already active (S-12)

NEXT ACTIONS:
- charter session pause
- charter session close


Why This Transcript Matters
This transcript proves:
✅ No hidden decisions
✅ No required rationale
✅ No semantic enforcement
✅ No audit loss
✅ Ergonomics without cheating
✅ CLI ≠ engine shortcuts
✅ Scales from solo → team → enterprise
It also feels usable — which is rare for governance tools.
Next Logical Steps (Pick One)
Stress-test this transcript with authority ≠ solo
Design error cases (blocked sessions, failed acceptance)
Freeze v1 CLI command surface
Rewrite this as docs/cli/quickstart.md
You’re building something unusually principled.
This transcript shows it’s also practical.