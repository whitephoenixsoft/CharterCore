# Annotations on objects 

## 1. Area Fields 

```Text
Area
- area_hash        // engine identity (git-style)
- short_label      // ergonomic CLI reference (A-INFRA)
- display_name     // human-readable name
- annotation?      // optional explanation
```

### Interpretation
- area_hash: never shown unless explicitly requested
- short_label: what users type
- display_name: what users read
- annotation: why this Area exists, caveats, history

This avoids overloading the name field with explanation.
### Invariant
> No legitimacy or scope inference may use short_label, display_name, or annotation.

## 2. Session: same pattern, but tighter

*problem_statement is the “long name”.*

```
Text
Session
- session_hash
- short_label?        // optional, CLI convenience
- problem_statement  // canonical description
- annotation?        // optional context / background
```
### Why this matters
- Problem statement = what is being decided
- Annotation = why this session exists, constraints, circumstances
- This keeps problem statements clean and auditable while still capturing nuance.

## 3. Candidate: rename rationale → annotation (but keep CLI term)

Unify terminology internally.

### Engine model

```Text
Candidate
- id
- content
- annotation?
```

### CLI surface

```Bash
charter candidate add C-1 --rationale "..."
```

This is a UX affordance, not a semantic distinction.

### Important invariant
> Candidate annotations explain the option, not the decision.
This remains consistent with everything you’ve frozen.

## 4. Resolution: this is the subtle one 

Question:
> If a resolution annotation explains why it was accepted, does that weaken legitimacy?
> Answer: no — if and only if we enforce one rule.

### Key Rule (This resolves the tension)
> Annotations may explain context, but never serve as a justification mechanism.

Rule Meaning:
- Resolution annotations: allow them, but define their meaning

Solution:
```Text
Resolution
- id
- content
- acceptance_context
- annotation?   // optional, immutable
```
Allowed uses:
- Background
- Circumstances
- Constraints
- Pointers to external discussion

Disallowed (conceptually, not mechanically):
- “Accepted because Alice said so”
- “Valid due to emergency override”
- “This makes it legitimate”

But it is not enforce this mechanically because enforcement would require semantic interpretation, which Charter explicitly avoids.

### Invariant
Legitimacy comes exclusively from:
- the session
- authority
- recorded stances not from text fields.

So even if a user writes “This was accepted because Bob is CEO” in an annotation, the engine still ignores it.

### CLI Ergonomic Change 

```Bash
charter accept C-2 \
  --annotation "Decision made during incident; limited review"
```
Under the hood:
- Acceptance logic is unchanged
- Annotation is stored on the Resolution object

This gives users what they want without contaminating legitimacy.

## 5. Default import annotation idea

```Text
import_annotation:
- source_type: "file" | "api" | "migration"
- source_name: "policies_2021.json"
- imported_at: ISO-8601 timestamp
- import_mode: RESTORE | CONSOLIDATE | FLAT
- importer_version
```
Rendered to humans as something like:
> “Imported from policies_2021.json on 2025-03-14 using CONSOLIDATE mode.”

Why this works:
- Explains why it’s UNDER_REVIEW
- Does not editorialize
- Is deterministic and auditable

For flat-list initial imports, this annotation is extremely important.

## 6. Final unifying principle

Here’s the principle that resolves all of this cleanly:
> Annotations explain history; sessions create legitimacy.

If we hold to that, then:
- Where annotations live becomes obvious
- What they can’t do is clear
- You never need special cases later

## Summary
- Keep identity, labels, and explanation separate
- Add optional annotation to:
    - Area
    - Session
    - Candidate
    - Resolution
- Allow CLI to attach annotations at acceptance time
- But store them on the Resolution
- Auto-generate import annotations with:
    - file name
    - timestamp
    - import mode
- Document explicitly:
    - Annotations are ignored by legitimacy logic
