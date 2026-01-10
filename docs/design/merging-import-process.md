# Merging Import Process

> Charter Core never merges decisions.
> It only merges object graphs and asks humans to legitimize outcomes.

## Why This Is Hard (And Rare)

Most systems:
- diff content
- auto-resolve conflicts
- overwrite history

Charter:
- diff identity
- preserve timelines
- require legitimacy

That’s why this feels heavy — because it’s honest.

## What an Import Really Is (Engine View)

An import is **not** “merging histories”.

It is:

> “Introduce foreign immutable objects and decide whether to attach references to them.”

That’s it.

---
## Mental Model

*Charter Core storage ≈ Git’s content-addressed store*

There  effectively have two layers:

### 1. Object Store (immutable)

Like Git’s object database.

Stores:
- Areas
- Sessions
- Candidates
- Resolutions
- Votes / stances
- Authority & Scope resolutions (no special storage, just special rules)

Each object:
- Has a content hash (hash of canonical serialized content)
- Is immutable once written
- Can exist without being “active”

### 2. Reference Tables (mutable pointers)

Like Git refs (HEAD, branches, tags).

Stores:
- Active Authority per Area
- Active Scope per Area
- Active Resolutions
- Resolution lifecycle state
- Area initialization status

Critical distinction:
- Import compares objects, reconciliation updates references.

This keeps invariants intact.

---
## Restore Mode (Briefly)

RESTORE skips ref comparison:
- All refs recreated exactly
- Object store populated

Used for:
- new machines
- disaster recovery
- migration

---

## Step-by-Step: Import Compared to Memory

Assume:
- Engine already has an object store
- Engine has ref tables representing current state
- Import arrives as an export blob

---
### Step 0: Parse + Canonicalize

Engine:
- Parses export
- Canonicalizes each object (ordering, normalization)
- Computes hash for each object

If parsing fails → hard fail
If canonicalization fails → hard fail

*No state touched yet.*

---

### Step 1: Integrity Verification (Graph Validity)

Before any comparison to memory:

Engine verifies internal consistency of the import:
- Every Session references:
    - an Area
    - an Authority
    - a Scope
- Every Resolution references:
    - a Session
    - an Area
    - Authority + Scope at acceptance
- No dangling references
- No cycles where forbidden (e.g. Authority superseding itself)

If this fails:
- Import fails deterministically
- Or (if import_on_failure=true): continue but everything is UNDER_REVIEW

*Still no interaction with local state.*

This matches Git’s “object graph is valid” step.

---

### Step 2: Object Presence Comparison (Hash-Level)

Now the engine compares hashes, not semantics.

For each imported object:

Case | Meaning
:- | :-
Hash exists locally | Object already known
Hash does not exist | New object
Important:
- No “difference”
- No “conflict”
- Only **identity**

This is why hashing is powerful:
*You never diff content at the engine level.*

---

### Step 3: Store Missing Objects

For each object whose hash is missing:
- Insert into object store
- Mark as `origin = imported`
- Optionally record import_id, timestamp, source annotation

At this point:
- All objects exist locally
- No legitimacy has been granted
- No references updated

This is the exact Git equivalent of `git fetch`.

---

### Step 4: Reference Comparison

For each Area in the import:

Engine checks:
1. Does the Area exist locally?
2. Does the import introduce:
    - new resolutions?
    - a different active Authority?
    - a different active Scope?

Remember:
> Authority & Scope are just resolutions with enforced uniqueness.

---

#### Comparison Matrix (Per Area)

##### Case A — Area does not exist locally

- Create Area
- Attach imported references
- Area is initialized if Authority + Scope exist 

*No ambiguity*

##### Case B — Area exists, no overlapping references
*(e.g. import only contains historical resolutions)*

- Objects are stored
- No refs updated
- Done

*This is the historical enrichment case.*

##### Case C — Area exists, overlapping but identical references
*(hash of active Authority/Scope matches)*

- No action required
- Import is effectively a no-op at the ref level

*This is the “same baseline” case.*

##### Case D — Area exists, divergent references

Examples:
- Different active Scope
- Different active Authority
- New active Resolution for same domain

Engine behavior (CONSOLIDATE mode):
- Imported resolutions do not become active
- Imported resolutions are marked UNDER_REVIEW
- No local references are modified
- Audit records:
    - import occurred
    - divergence detected

*The engine refuses to guess which timeline is correct.*

---

## What Happens When User “Agrees” Later

This is important to your earlier confusion about audit.

When Bob later accepts an imported resolution:
- He starts a new session
- Candidate may literally reference the imported resolution content
- Acceptance creates a new local resolution
- Imported one remains as historical context

So:
- We do not lose the “why”
- We do not collapse histories
- We create a new legitimate act

This is exactly how Git creates a new commit when resolving conflicts.

---

## Why This Matches Invariants

1. Immutable History
    - Objects are never altered, only added.
2.  No Retroactive Changes
    - Refs are not updated automatically.
3. Authority Is Mechanical
    - Authority comparisons are structural, not semantic.
4. Sessions Are the Unit of Legitimacy
    - Imported resolutions are not legitimate locally until accepted via a session.
5. CLI Can Guide, Engine Does Not
    - The engine exposes:
        - “These resolutions are under review”
        - “These differ from your active ones”
        
What to do next is entirely outside the engine.