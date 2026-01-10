# Restarting a session 

charter session restart-from S-12

```
Restarting from session S-12.
This creates a new session.
No votes or acceptance will carry forward.
```

## Changing the Problem Statement on Restart

A blocked or failed session often reveals:
the problem was framed poorly
the scope was too broad
the question needed decomposition

### Rule (CLI-level, explicit)
When using restart-from:
the problem statement may be edited
defaults to the original
user must confirm if changed

Example:
Copy code
Bash
charter session restart-from S-12 \
  --problem "Decide deployment strategy for v2 only"
  
CLI confirmation:
Copy code
Text
Problem statement changed from:
"Decide deployment strategy"
to:
"Decide deployment strategy for v2 only"

This creates a new decision problem.
Continue? [y/N]

### Why this is safe
Sessions are about framing a decision
Resolutions, not sessions, carry authority
Audit trail preserves lineage
This actually improves legitimacy, not weakens it.

Semantics
Closes the original session
Copies:
candidates
annotations
(optionally) participant list
Allows:
problem statement change
constraint/authority changes
Creates a new session ID

Why This Matters
Without this:
Users will try to hack resumes
Or worse, bypass sessions entirely
This is not a convenience feature — it’s legitimacy-preserving ergonomics.