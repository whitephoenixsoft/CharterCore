# Annotations on objects 

Area Fields 
Copy code
Text
Area
- area_hash        // engine identity (git-style)
- short_label      // ergonomic CLI reference (A-INFRA)
- display_name     // human-readable name
- annotation?      // optional explanation
Interpretation
area_hash: never shown unless explicitly requested
short_label: what users type
display_name: what users read
annotation: why this Area exists, caveats, history
This avoids overloading the name field with explanation.
Invariant
No legitimacy or scope inference may use short_label, display_name, or annotation.
So yes — your instinct here is structural, not annotation creep.
2. Session: same pattern, but tighter
You’re exactly right: problem_statement is the “long name”.
Recommended structure:
Copy code
Text
Session
- session_hash
- short_label?        // optional, CLI convenience
- problem_statement  // canonical description
- annotation?        // optional context / background
Why this matters:
Problem statement = what is being decided
Annotation = why this session exists, constraints, circumstances
This lets you keep problem statements clean and auditable while still capturing nuance.
3. Candidate: rename rationale → annotation (but keep CLI term)
You’re correct to unify terminology internally.
Engine model
Copy code
Text
Candidate
- id
- content
- annotation?
CLI surface
Copy code
Bash
charter candidate add C-1 --rationale "..."
This is a UX affordance, not a semantic distinction.
Important invariant
Candidate annotations explain the option, not the decision.
This remains consistent with everything you’ve frozen.
4. Resolution: this is the subtle one (and you’re right to hesitate)
You’re asking the right question:
If a resolution annotation explains why it was accepted, does that weaken legitimacy?
Answer: no — if and only if we enforce one rule.
Key Rule (This resolves the tension)
Annotations may explain context, but never serve as a justification mechanism.
That sounds abstract, so here’s how to operationalize it.
Resolution annotations: allow them, but define their meaning
Recommended:
Copy code
Text
Resolution
- id
- content
- acceptance_context
- annotation?   // optional, immutable
Allowed uses:
Background
Circumstances
Constraints
Pointers to external discussion
Disallowed (conceptually, not mechanically):
“Accepted because Alice said so”
“Valid due to emergency override”
“This makes it legitimate”
But—and this matters—you do not enforce this mechanically.
Why? Because enforcement would require semantic interpretation, which Charter explicitly avoids.
Instead, you rely on this invariant:
Legitimacy comes exclusively from:
the session
authority
recorded stances not from text fields.
So even if a user writes “This was accepted because Bob is CEO” in an annotation, the engine still ignores it.
That keeps the system honest.

Accepting with an annotation — is it okay?
Yes — but the annotation must be attached to the resolution, not the acceptance act.
So:
❌ accept_candidate(..., annotation) ✅ accept_candidate(...) → creates Resolution → Resolution has annotation
CLI can do this ergonomically:
Copy code
Bash
charter accept C-2 \
  --annotation "Decision made during incident; limited review"
Under the hood:
Acceptance logic is unchanged
Annotation is stored on the Resolution object
This gives users what they want without contaminating legitimacy.
5. Default import annotation (recommendation)
Your instinct is correct: minimal, factual, non-judgmental.
I recommend auto-populating all of these when importing:
Copy code
Text
import_annotation:
- source_type: "file" | "api" | "migration"
- source_name: "policies_2021.json"
- imported_at: ISO-8601 timestamp
- import_mode: RESTORE | CONSOLIDATE | FLAT
- importer_version
Rendered to humans as something like:
“Imported from policies_2021.json on 2025-03-14 using CONSOLIDATE mode.”
Why this works:
Explains why it’s UNDER_REVIEW
Does not editorialize
Is deterministic and auditable
For flat-list initial imports, this annotation is extremely important.
6. Final unifying principle (this will keep you sane)
Here’s the principle that resolves all of this cleanly:
Annotations explain history; sessions create legitimacy.
If you hold to that, then:
Where annotations live becomes obvious
What they can’t do is clear
You never need special cases later

Final Recommendations (Concrete, Actionable)
Keep identity, labels, and explanation separate
Add optional annotation to:
Area
Session
Candidate
Resolution
Allow CLI to attach annotations at acceptance time
But store them on the Resolution
Auto-generate import annotations with:
file name
timestamp
import mode
Document explicitly:
Annotations are ignored by legitimacy logic