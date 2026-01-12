CLI reality: participants cannot be detected

You’re completely right here, and this is a key correction:
In the CLI, participants cannot be inferred.
So the CLI must treat participant management as explicit user input, not observation.
This actually strengthens the design.
Participant handling: explicit, mutable, auditable
Core principles
The CLI never assumes who is present
Participant changes are explicit events
Authority is always evaluated against the current declared participant set
History preserves who was considered present at acceptance time
Required CLI commands (v2-compatible, v1-safe)
You already hinted at this, and it’s unavoidable:
Copy code
Text
charter session participants list
charter session participants add <id>
charter session participants add <id> --required-approver
charter session participants remove <id>
charter session participants set <if>
     - this confirms participants 
These commands:
mutate session state
are auditable
trigger revalidation of authority constraints
Behavioral rules (important)
When participants change:
Session enters a “needs re-evaluation” state
CLI must announce:
authority rule
whether acceptance is currently possible
No silent continuation
Example output:
Participant Bob removed
Authority: MAJORITY_PRESENT
Required approvers: Alice
Current state: ACCEPTANCE BLOCKED (Alice has not voted)
This keeps the user honest without moralizing.
This solves meetings, recordings, and solo mode
Solo mode
Default participant: yourself
No confusion
Ergonomic shortcuts apply
Recorded meetings (v2)
Participants added/removed as people enter/leave
Votes recorded when they happen
Acceptance reflects reality at the moment of decision
Multi-user future
Server may suggest participants
Engine still requires explicit declaration
No trust leak from transport layer into legitimacy layer