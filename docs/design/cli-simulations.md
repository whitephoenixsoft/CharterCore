Charter Core — Canonical Simulations (Re-run with Updated Commands)
Simulation 1 — Area Initialization (Bootstrap)
Context (unchanged)
A new Area cannot function until Authority and Scope exist.
Flow (updated CLI semantics)
Copy code
Text
charter init
→ creates Area A-1
→ Area status: UNINITIALIZED
→ Next action: define authority
Copy code
Text
charter session start "Define Authority"
charter session participants add alice
Authority candidate added and accepted:
Copy code
Text
charter candidate add "SOLO authority (Alice)"
charter session accept
Repeat for Scope:
Copy code
Text
charter session start "Define Scope"
charter session participants add alice
charter candidate add "Applies to product decisions"
charter session accept
Outcome
Area becomes ACTIVE
Authority and Scope are first-class resolutions
New validation added
✅ Participants explicitly declared
✅ Solo authority collapses vote+accept ergonomically
❌ No implicit defaults introduced
Simulation 2 — Flat Authority Collaboration (Students)
Context (unchanged)
Two peers collaborate without hierarchy.
Authority
Declared as UNANIMOUS_PRESENT
Flow (new mechanics explicit)
Copy code
Text
charter session start "Choose project topic"
charter session participants add alice
charter session participants add bob
Copy code
Text
charter candidate add "Build a compiler"
charter candidate add "Build a game engine"
Votes:
Copy code
Text
charter vote accept C-1 --by alice
charter vote accept C-1 --by bob
Copy code
Text
charter session accept C-1
Outcome
Resolution accepted legitimately
Participant set recorded immutably
New validation added
✅ No acceptance without explicit unanimity
✅ No ambiguity about “who was there”
Simulation 3 — Single-User Governance (Solo Journal)
Context
Solo founder records decisions over time.
Flow (ergonomic shortcut)
Copy code
Text
charter session start "Choose pricing model"
charter candidate add "Usage-based pricing"
charter session accept
What changed
CLI collapses vote+accept
Engine still records:
participant list = [founder]
stance = ACCEPT
Outcome
No loss of audit integrity.
Simulation 4 — Normal Decision Session (Team)
Context
Team chooses architecture.
Authority
MAJORITY_PRESENT
Flow (new participant awareness)
Copy code
Text
charter session start "Choose architecture"
charter session participants add alice
charter session participants add bob
charter session participants add carol
Votes:
Copy code
Text
alice → ACCEPT
bob   → ACCEPT
carol → ABSTAIN
Majority satisfied → accept.
Outcome
Resolution references:
Authority
Scope
Participant snapshot
New validation added
✅ Abstention is first-class
✅ Majority computed mechanically, not inferred
Simulation 5 — Deadlock Without Abuse
Context
Unanimous authority, disagreement.
Flow
Copy code
Text
participants: alice, bob, carol
votes: alice ACCEPT, bob REJECT, carol ACCEPT
Copy code
Text
charter session accept
→ FAIL: Authority rule not satisfied
→ Status: BLOCKED
→ Next actions:
   - change participants
   - change authority (new session)
   - close session
Outcome
No forced resolution.
Simulation 6 — Participant Leaves Mid-Session (Critical Change)
This is where the new model shines.
Context
Deadlocked unanimous session.
Original votes
Copy code
Text
alice ACCEPT
bob REJECT
carol ACCEPT
Blocked.
Participant leaves
Copy code
Text
charter session participants remove bob
CLI response:
Copy code
Text
Participants changed.
Authority: UNANIMOUS_PRESENT
Current participants: alice, carol
Re-evaluation required.
Now:
Copy code
Text
charter session accept
Outcome
Resolution accepted legitimately.
What’s new
✅ No vote reinterpretation
✅ Departure is explicit
✅ Acceptance reflects reality at the moment
Simulation 7 — Authority Change Requires New Session
Context
Team wants faster decisions.
Flow (unchanged, but clearer)
Copy code
Text
charter session start "Change authority"
participants: alice, bob, carol
Authority candidate: MAJORITY_PRESENT
Votes recorded under old authority.
Accept → old authority superseded.
Outcome
Authority evolution is explicit
No retroactive impact
Simulation 8 — Concurrent Sessions (Solo Constraint Enforced)
New CLI invariant applied
Solo mode: only one active session allowed
Attempt:
Copy code
Text
charter session start "Decision A"
charter session start "Decision B"
CLI response:
Copy code
Text
Error: active session exists
Next actions:
- pause current session
- close current session
Outcome
Cognitive load controlled
Engine unchanged
Simulation 9 — Import + Review (Preview)
Context
Flat import, all under review.
Copy code
Text
charter import legacy.json --mode consolidate
charter review open I-1
Review shows:
Imported resolutions
Their original authority/scope
Supersession conflicts
Acceptance uses sessions behind the scenes, but ergonomically hidden.