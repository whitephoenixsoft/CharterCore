CLI Invariants — Canonical Set (LOCKED)
Scope: CLI layer only
Violations indicate CLI correctness failures, not engine bugs
A. Identity & Naming
CLI-INV-ID-01: Engine Identity Is Canonical
Every Area, Session, Resolution, Scope has a canonical engine ID
Labels NEVER replace engine IDs
All operations resolve to engine IDs
Fail if
Any command treats labels as authoritative identity
CLI-INV-ID-02: Labels Are Ergonomic Only
Labels encode no semantics
Labels affect no legitimacy, authority, or scope
Fail if
CLI logic interprets label structure or naming patterns
CLI-INV-ID-03: Labels Are Area-Scoped by Default
Labels must be unique within an Area
Cross-area operations require explicit qualification
Fail if
Label collisions create ambiguity or coupling
CLI-INV-ID-04: Area Context Is Required
Unqualified labels require an active Area
Otherwise the command must fail and prompt
CLI-INV-ID-05: Global Disambiguation Is Explicit
Global views must render <area>/<label>
Bare labels must never appear globally
CLI-INV-ID-06: Global Label Uniqueness Is Optional
Must be opt-in, reversible, documented
Never silently enforced
B. Context & Mode Safety
CLI-CTX-01: Context Switching Is Explicit
Area switches, session pauses, review entry are explicit commands
CLI-CTX-02: Decision vs Baseline Review Are Distinct Modes
CLI must visually and structurally distinguish:
Decision-making
Baseline review / consolidation
CLI-CTX-03: Human Output by Default
Human-readable output is default
Machine output requires flags (--json, --grep)
C. Sessions & Participants
CLI-INV-SES-01: Single Active Session (Solo Mode)
Only one active session allowed
Must pause or close before starting another
CLI-INV-SES-02: Participants Are Explicit State
CLI never infers participants
All changes are explicit commands
Acceptance snapshots participants immutably
CLI-INV-SES-03: Participant Revalidation on Resume
On session resume, CLI must:
Show previous vs current participants
Require explicit confirmation of:
Participants
Authority
Required approvers
Until confirmed
No votes
No acceptance
No candidate changes
CLI-INV-SES-04: Restart Is Terminal
restart-from:
Closes source session
Creates new session with new ID
Copies no votes or acceptance
Records lineage for audit only
CLI-INV-SES-05: Candidate Set Freezes on First Stance
After first vote:
No add/remove/edit candidates
CLI must guide user to restart-from
D. Authority & Constraints Awareness
CLI-INV-AUTH-01: Constraints Are Authority-Equivalent
Cannot change mid-session
Cannot change on resume
Require their own session
Governed by current authority
CLI-INV-AUTH-02: Resume Cannot Introduce New Legitimacy
Participants may change
Authority and constraints may not
CLI-INV-AUTH-03: CLI Never Creates Consensus
Commands never imply agreement
Absence of data ≠ approval
CLI-INV-AUTH-04: Non-Solo Authority Requires Participants
If Authority ≠ SOLO:
Sessions must specify participants
Acceptance must validate against them
CLI-INV-AUTH-05: CLI Is Honest About Its Limits
CLI must not claim to know:
Who attended
Who agreed
What was discussed
Annotations allowed. Inference forbidden.
E. Ergonomics (Authority-Aware)
CLI-ERG-01: Authority-Aware Collapse Allowed
In SOLO:
CLI may collapse vote + accept
Engine history must remain complete
CLI-ERG-02: Explicitness Beats Convenience
CLI may reduce keystrokes
Must never skip legitimacy checks
CLI-ERG-03: Next Actions Are Always Shown
After any command, CLI must list:
Valid next actions
Never illegal or blocked ones
F. Baseline Review (formerly “Review”)
CLI-INV-REV-01: Review Is Not a Session
No voting
No authority evaluation
No session blocking
CLI-INV-REV-02: Single Active Review (Solo Mode)
Only one active baseline review per Area
Must close explicitly before starting another
CLI-INV-REV-03: Review Is a Mutable Workspace
Accept/reject is reversible
Close is irreversible
CLI-INV-REV-04: Review Closure Semantics
On review close:
All UNDER_REVIEW → ABANDONED
ABANDONED is:
Immutable
Auditable
Re-importable
CLI-INV-REV-05: Review Acceptance Is Authority-Governed
In solo: direct accept/reject allowed
In non-solo: must be session-mediated or forbidden
CLI-INV-REV-06: Provenance Is Always Visible
For each imported resolution, CLI must show:
Import ID
Original Area (if any)
Original session problem statement
Superseded local resolution (if applicable)
CLI-INV-REV-07: Review Never Bypasses Sessions
Every accepted resolution corresponds to a session
Batch operations may hide sessions, not eliminate them
G. Audit Invariants
CLI-AUD-01: Audits Are Read-Only
CLI-AUD-02: Audits Do Not Infer Meaning
CLI-AUD-03: Audits Are Deterministic
CLI-AUD-04: Grep-Friendliness Is Mandatory
One event per line
Stable field order
Explicit keywords
Engine IDs always included
Never wrapped
CLI-AUD-05: Participants Are First-Class Audit Subjects
(locked above)