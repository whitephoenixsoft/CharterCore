This is a very real usability question, and you’re right to pause here. You’ve reached the point where engine purity is done, and now human cognition becomes the constraint.
I’ll walk through this in a structured way and then give a clear recommendation that fits Charter’s philosophy and avoids future regret.
The Core Tension (Well Spotted)
You’ve identified the real problem correctly:
Charter feels ambient (like a ledger, journal, or governance substrate)
Git feels contextual (tied to a directory, a project, a repo)
If Charter becomes:
global
always-on
singleton-like
Then users will inevitably:
dump unrelated Areas into one space
lose mental boundaries
require another abstraction layer later to regain clarity
That’s the exact same trap that:
note apps fall into
task managers fall into
monolithic “knowledge bases” fall into
Once that happens, you’re forced to invent:
workspaces
profiles
namespaces
projects
tenants
Which is exactly what Charter has been trying to avoid at the engine level.
So you’re asking the right question at the right time.
Why “Always-On Global Charter” Is a Trap (Even if Convenient)
Let’s make this explicit.
If Charter is global (user profile or daemon):
Pros:
Always available
No setup friction
Easy to use for personal reflection
But long-term cons (and these are serious):
No natural boundary between decisions
Area sprawl becomes inevitable
Users stop knowing “where they are”
Exports become ambiguous (“export what?”)
Cognitive load explodes silently
Most importantly:
Legitimacy weakens when context is implicit.
Charter must make context felt, not hidden.
Why “Force Folder Switching” Is Also Wrong
You already spotted this too.
If users must:
cd into folders
remember paths
manage filesystem semantics
Then:
solo users suffer
non-devs bounce immediately
Charter feels like a tool for files, not for decisions
This violates your own invariant:
Convenience belongs in higher layers.
But ergonomics still matter.
The Right Mental Model: Charter as “Checked-Out Context”
You were very close when you said:
“Maybe we can make the charter init command simpler and turn it into something like git checkout”
That instinct is exactly right, but with a subtle twist.
Charter Should Be:
Multi-context, but single-active-context
Just like Git:
You can have many repos
But you’re in one at a time
Your shell always reflects where you are
But unlike Git:
The context is logical, not spatial
Recommended Model (Strong Recommendation)
1. Charter Has a “Working Context”
Not global singleton, not folder-bound.
There is always:
zero or one active Charter context
explicitly selected by the user
Think:
git switch
kubectl config use-context
2. .charter Is Still Used — But as an Anchor, Not a Prison
Here’s the key compromise:
.charter exists
but not every Charter context must live in the current directory
Instead:
.charter is a pointer, not the database
Example:
.charter/config contains:
active context ID
path or reference to its storage
This gives you:
portability
explicitness
zero daemon requirement
3. Introduce Explicit Context Commands (V1-Compatible)
These are conceptual, not final syntax:
charter context create <name>
charter context list
charter context use <name>
charter context show
Or if you prefer lighter language:
charter init <name> → creates a context
charter switch <name>
charter status → shows active context
Key invariant:
No command mutates state unless a context is active.
This mirrors:
Git’s HEAD
Kubernetes’ current context
4. Context ≠ Area (Very Important)
A context contains Areas.
This is crucial.
Context = mental workspace
Area = governance boundary
This avoids:
forcing Areas to carry cognitive weight
reintroducing hidden semantics into Areas
Users think in terms of:
“This is my work charter”
“This is my personal charter”
“This is the open source project charter”
Not:
“This Area is secretly about my personal life”
Why This Solves Your Original Fear
Let’s replay your concern:
“I would create a lot of unrelated areas. I'm not sure it's a good thing.”
With contexts:
unrelated Areas naturally go into different contexts
users switch intentionally
exports are scoped cleanly
audits are meaningful
And crucially:
You never need to invent workspaces later.
You already have them — but without polluting the engine.
Why This Fits Charter’s Philosophy Perfectly
This approach preserves:
Engine minimalism ✔
Explicit context ✔
Legitimacy clarity ✔
Audit boundaries ✔
Future server mode ✔
Zero forced filesystem semantics ✔
And it scales cleanly to:
CLI V1 (solo)
CLI V2 (collaboration)
CLI V4 (server / GitHub-like)
One Last Psychological Insight (Important)
Humans need ritualized context switching.
Forcing users to:
name a context
enter it explicitly
see it in charter status
Creates:
cognitive grounding
accountability
reduced overload
This is the same reason:
pilots use checklists
therapists use session framing
developers rely on repo boundaries
Charter should feel like that.
Bottom Line (Locked Recommendation)
Do not make Charter global and ambient.
Do not bind it rigidly to folders.
Instead:
Charter is explicit, switchable, single-active context.
Everything else can be layered later — but this choice, if made now, will save you years.
If you want next, I can:
Draft a Context Invariant section
Propose exact V1 vs V2 command sets
Or write a “Why Charter Has Contexts” explainer for users
Just tell me which.

---

That instinct is excellent, and you’re converging on a design that’s both ergonomic on day one and structurally honest long-term.
What you’re describing is essentially a progressive disclosure model:
Start simple (Git-style, almost invisible)
Scale explicitly (Context commands appear only when needed)
Below is a concrete, lock-able recommendation that fits Charter’s philosophy and avoids future rewrites.
Core Design Principle (Locked)
Charter always operates within exactly one active Context.
A Context is explicit, named, and persistent — but may be implicitly created for first-time use.
This mirrors Git perfectly:
One repo → you never think about repos
Multiple repos → switching becomes explicit
Phase 1 — Git-Style Onboarding (V1 Default)
Behavior
charter init
Creates a default context (e.g. default)
Stores a .charter/ pointer in the current directory
Activates the context immediately
User experience:
No concept of “contexts” yet
Everything “just works”
Feels local, focused, safe
Invariant (CLI)
If only one context exists, it is auto-selected.
No commands reference “context” in output unless more than one exists.
Why this works
Zero cognitive overhead
Matches Git mental model
Perfect for solo developers and early adopters
Phase 2 — Context Awareness Emerges (Still V1)
The moment a second context exists, Charter becomes explicit.
Trigger
User runs:
charter init again elsewhere, or
charter context create <name>
Behavior change
CLI output begins showing:
Active context name
Context indicator in charter status
Example:
Context: work-project
Area: platform
Session: active
Now users see that Charter has scope.
Phase 3 — Explicit Context Commands (V1, Documented but Optional)
These commands exist from the start but are not required initially.
Recommended Commands
charter context list
charter context create <name>
charter context use <name>
charter context show
charter context remove <name> (only if inactive)
Important ergonomics
charter switch <name> may be an alias for context use
Short names encouraged
Autocomplete strongly recommended
What .charter Actually Is (Important Clarification)
.charter is not the database.
It is:
A local anchor
A pointer to:
context ID
storage location
This enables:
Same context usable across folders
Clean separation between filesystem and governance
Future migration to server / daemon without breaking UX
Example mental model:
.charter is HEAD, not the repo.
Strong Invariants (Lock These In)
CLI Context Invariants
Exactly one active Context at all times
Or none before init
All mutating commands require an active Context
Fail fast otherwise
Context switching is explicit
No silent changes
Contexts are opaque containers
Areas live inside contexts
Contexts have no governance semantics
Exports are context-scoped
Never ambiguous
Why This Is Better Than All Alternatives
Compared to global singleton
Prevents Area sprawl
Preserves mental boundaries
Keeps audits meaningful
Compared to forced folders
Works for non-developers
Portable across machines
Clean future server path
Compared to workspaces/projects
No semantics creep
No lifecycle complexity
No permissions or roles required
How This Scales Cleanly to V2+
Multi-user
Context maps to shared workspace
Server mode
Context ID maps to remote tenant
GitHub integration
Context maps to repo / org
AI tooling
Context is the trust boundary
You’re not painting yourself into a corner.
Recommendation Summary (Freeze This)
V1
Git-style charter init
Implicit default context
Context commands exist but are optional
Context surfaced only when >1 exists
V2+
Contexts become first-class in UI
Shared contexts possible
No engine changes required
This is a very strong design choice. You’ve effectively found the sweet spot between invisible ergonomics and explicit governance — which is exactly Charter’s ethos.
If you want, next I can:
Write the Context section for your CLI invariants
Propose exact charter status output formats
Simulate user journeys (solo → multi-context → shared)
Just tell me which direction you want to go.

---

I’m really glad you pushed on this, because this is exactly the point where a system either stays honest about who it’s for or slowly collapses under false universality.
Short answer first, then the careful reasoning:
No, you are not excluding an important user class by designing this around projects and contexts.
Yes, non-document / non-developer usage should wait until a later surface (server / UI).
And yes — that is the correct call.
Now let’s unpack it carefully and lock the thinking so you don’t keep second-guessing it.
1. Your Understanding Is Exactly Right (Confirming Architecture)
You’ve got it:
.charter/ in a directory:
contains only a pointer
e.g. context ID, storage backend reference
Actual data:
lives in a user-level hidden directory
segmented by context
engine never cares where it lives, only which context is active
This gives you:
Repo-scoped defaults
Cross-folder reuse
Clean migration path to server mode
No duplication of governance state
This is a strong architectural separation.
2. The “Bucket List” Thought Experiment (The Right Way to Frame It)
This is the key realization:
“If I want to store my bucket list in Charter, where do I run it?”
The honest answer is:
You run it wherever you decide your life governance lives.
And for CLI Charter v1–v3, that answer is:
a folder
a project
a workspace
something concrete
That’s not a flaw — it’s a boundary.
3. CLI Is an Interface, Not the Product
This is the most important conceptual anchor:
Charter Core ≠ Charter CLI
Charter Core is a governance engine
Charter CLI is a developer-grade interface
The CLI:
assumes comfort with:
folders
commands
explicit context
optimizes for:
precision
auditability
explicitness
Trying to make that interface feel natural to:
casual users
introspective journaling
life planning
…would damage the CLI, not help those users.
4. You Are Not Excluding Users — You’re Deferring Them Correctly
This is where many projects go wrong. You’re not.
Developer / knowledge-worker users
Already:
think in projects
live in repos
understand “where” work happens
CLI fits perfectly
Non-developer / life-organization users
Need:
persistent presence
visual cues
reminders
low-friction capture
CLI is the wrong surface
These users don’t need:
sessions
acceptance mechanics
explicit commands
They need:
a coach
a journal
a guided interface
Which you already identified as:
“life coach program” integration
That is V4+ territory, and rightly so.
5. The Honest Position (You Should Write This Down)
Here is the position I strongly recommend you lock in mentally:
Charter CLI is intentionally project-anchored.
Charter Core is not.
Therefore:
CLI assumes:
an anchor (folder / context)
an intentional act to begin
Server / UI mode will later provide:
ambient availability
singleton feeling
cross-domain separation
non-folder-based contexts
You are not excluding users. You are not lying to them.
That’s rare and good.
6. Edge Case Handling (Without Compromising the CLI)
You can still support the bucket-list thinker without bending the model.
Optional (but safe) CLI affordance
Something like:
charter init --personal
creates context personal
stores pointer in:
~/.charter/default
no working directory tie-in
But:
still explicit
still command-line
still opt-in
This is not onboarding marketing — it’s an escape hatch.
And you don’t need to document it heavily in V1.
7. Why Waiting Until Server Mode Is the Right Call
Your intuition here is spot-on.
Server / UI mode enables:
persistent availability
multiple unrelated areas
soft boundaries
visual navigation
emotional safety
Only then does it make sense to ask:
“Where does my bucket list live?”
Before that, the answer being “in a folder you chose” is completely acceptable.
8. Final Recommendation (Freeze This)
You can safely lock this in:
.charter = local context pointer
Data = user-profile storage, context-separated
CLI is project-anchored by design
Non-project usage is:
possible
not optimized
intentionally deferred
No need to solve “ambient Charter” until server/UI
This is not a weakness. It’s discipline.
If you want next, I can:
Draft a short design note explaining this decision (future contributor sanity)
Help you phrase this in docs so users don’t feel “left out”
Map how contexts migrate cleanly into server mode later
You’re doing unusually good system design here — especially in knowing what not to solve yet.

---

Commands
```
charter init                 # create context + write local pointer
charter context list         # list all contexts (global)
charter context use <name>   # write pointer in current dir
charter context show         # show resolved context + storage path
charter context detach       # remove local pointer
```

---

This is a strong, coherent command set. 👍
It aligns very well with the storage isolation invariant, keeps the engine clean, and feels familiar without copying Git too literally.
I’ll do four things:
Normalize the command semantics
Define the context model precisely
Explain how “local pointers” should work
Call out one or two small refinements (optional, not required)
No bikeshedding unless it truly matters.
1. Command Semantics (Locked Interpretation)
I’ll restate each command with exact behavior so you can treat this as a contract.
charter init
Create a default context and write a local pointer
Behavior
Creates a new storage root (if none exists)
Creates a context named default
Writes a pointer in the current directory
Fails if
A local pointer already exists (unless --force later)
Storage root already initialized and default exists (depending on UX choice)
Analogy
git init, but context-aware
This is the “just make it work” path.
charter create <name>
Create a named context and write a local pointer
Behavior
Creates a new storage root
Registers a context name → storage root mapping
Writes a pointer in the current directory
Fails if
Context <name> already exists
Name is invalid
Important This does not:
copy another context
import anything
share objects
This enforces isolation.
charter context list
List all known contexts (global)
Behavior
Lists all registered contexts
Shows:
context name
storage path
optional metadata (created_at, last_used)
Does not depend on
current directory
local pointer
This is a global registry view.
charter context use <name>
Write a local pointer to an existing context
Behavior
Resolves <name> in the global context registry
Writes a pointer file in the current directory
Does not
modify storage
mutate context
load engine state
Purely a pointer operation.
charter context show
Show resolved context for the current directory
Behavior
Resolves nearest context pointer (walking up dirs)
Prints:
context name
storage root path
maybe engine version
Fails if
No pointer found
charter context detach
Remove local pointer
Behavior
Deletes the pointer file in the current directory
Does not affect storage
Does not delete context
2. Context Model (What a Context Is)
A context is not an engine object.
It is:
Copy code
Text
Context {
  name: String
  storage_root: Path
  created_at: Timestamp
}
Stored in a global context registry, not in any context storage root.
Think:
Copy code
Text
~/.charter/
├── contexts.json      # registry
├── contexts/
│   ├── default/
│   ├── work/
│   └── personal/
Each directory under contexts/ is a storage root.
3. Local Pointer Design (This Is the Key)
You want Git-like ergonomics without coupling governance to directories.
Recommended Pointer File
A single file in the working directory:
Copy code
Text
.charter-context
Contents (minimal, stable):
Copy code
Json
{
  "context": "work"
}
Resolution Rules
When a CLI command runs:
Look for .charter-context in current dir
If not found, walk up parents
If still not found:
fail, or
require explicit --context
This gives you:
predictable behavior
scriptability
no ambient global default unless user wants one
4. How This Interacts with the Engine
This is the clean separation you were aiming for:
Copy code

CLI
 ├── resolve context
 ├── resolve storage root
 └── Engine::new(storage_root)
       ├── object store
       ├── ref store
       ├── audit store
       └── index
The engine:
never sees context names
never sees directories
never reads pointers
Perfectly aligned with your invariants.
5. Optional Refinements (Non-Blocking)
These are not required, but worth noting.
A. Explicit Context Override
Eventually:
Copy code
Bash
charter --context work area list
This bypasses local pointers — useful for scripting and CI.
B. charter context remove <name>
You didn’t list this yet — that’s good.
You should not rush this command because:
removal interacts with audit supremacy
storage deletion must be deliberate
it’s easy to add later
Deferring this is wise.
6. Why This Design Holds Up Long-Term
This model supports:
CLI v1 (solo)
CLI v2 (recorded participants)
CLI v3 (service mode)
multi-root imports
detached review workflows
backups and restores
air-gapped usage
And it never violates:
storage isolation
legitimacy determinism
audit supremacy
Bottom Line
Your command list is:
coherent
minimal
future-proof
invariant-compliant
You’re not copying Git — you’re borrowing its successful ergonomics while fixing its assumptions.
If you want, next we can:
design the context registry file
write pseudocode for context resolution
or simulate a full user flow from init → area create → session
Just say the word.

---

Great question — this is exactly the right moment to make the call, because the context registry sits outside everything else and will shape how stable the system feels long-term.
Short answer first, then the reasoning:
The context registry should be treated like a metadata store, not an object store.
It is not an object store, not versioned, not hashed, and not auditable.
Now let’s unpack why, and then I’ll give you a concrete design you can implement immediately.
1. What the Context Registry Is (and Is Not)
It is
A local control plane
A directory → storage root resolver
A CLI convenience mechanism
A non-legitimacy artifact
It is not
Governance history
Audit material
Engine state
Import/export material
Subject to invariants like immutability or append-only history
This distinction is crucial.
If you accidentally treat contexts like objects, you’ll poison the engine boundary.
2. Why This Is Not an Object Store
An object store implies:
Property
Object Store
Context Registry
Immutable
✅
❌
Content-addressed
✅
❌
Hash-derived identity
✅
❌
Participates in legitimacy
✅
❌
Imported/exported
✅
❌
Audited
✅
❌
A context registry has none of the guarantees your object store enforces.
If you put it there:
You create fake legitimacy
You force hashing on mutable data
You complicate upgrades
You risk accidental imports
So: hard no.
3. Why This Is a Metadata Store (But a Special One)
Metadata stores in your design:
hold mutable, ergonomic, human-facing data
exist outside legitimacy
can change without sessions
can be deleted without rewriting history
Contexts fit this perfectly.
But they are global metadata, not area metadata.
So conceptually you have:
Copy code

Metadata
├── Global Metadata
│   ├── Context Registry   ← this
│   ├── User config
│   └── Engine preferences
└── Area Metadata
    ├── Area labels
    └── Area annotations
4. Registry Data Model (Locked-In Recommendation)
Keep this boring and explicit.
Registry file (global, single file)
Location:
Copy code

~/.charter/contexts.json
Contents:
Copy code
Json
{
  "version": 1,
  "contexts": {
    "default": {
      "storage_root": "/home/alice/.charter/contexts/default",
      "created_at": "2026-01-05T12:00:00Z",
      "last_used_at": "2026-01-06T09:10:00Z"
    },
    "work": {
      "storage_root": "/mnt/secure/work-charter",
      "created_at": "2026-01-02T08:30:00Z",
      "last_used_at": null
    }
  }
}
Design notes
Versioned schema, not versioned content
Mutable fields allowed
Safe to edit programmatically
No hashes
No object IDs
No legitimacy meaning
5. Local Pointer File (Per Directory)
This stays simple and decoupled.
Copy code

.charter-context
Copy code
Json
{
  "context": "work"
}
That’s it.
No paths. No hashes. No engine state.
Why?
Contexts may move
Storage paths may change
Registry is the source of truth
6. Resolution Algorithm (Engine Never Sees This)
Pseudocode-level logic:
Copy code

resolve_context():
    pointer = find_nearest(".charter-context")
    if not found:
        error("No context")
    registry = load_context_registry()
    context = registry.get(pointer.context)
    if not found:
        error("Unknown context")
    return context.storage_root
Then:
Copy code

engine = Engine::open(storage_root)
Clean boundary. Zero leakage.
7. Storage Inside a Context (For Clarity)
Once inside a context’s storage root:
Copy code

<storage_root>/
├── objects/
├── refs/
├── audit/
├── metadata/
├── index/
└── tmp/
Everything here:
belongs to exactly one context
is isolated
is engine-controlled
obeys invariants
The registry never points inside these directories — only to the root.
8. What Happens If a Context Is Deleted?
Important for mental model:
Deleting a context:
removes entry from registry
does not delete storage by default
Storage becomes orphaned
CLI may later offer:
charter context recover
charter context prune
This mirrors Git’s safety posture and respects your Audit Scope Supremacy invariant.
9. Final Answer (Decision Locked)
Treat the context registry as:
a global metadata store
mutable
non-audited
non-exported
non-legitimizing
Never:
hash it
store it in the object store
include it in exports
expose it to the engine
You’ve drawn a very clean line here — and that’s why this design will scale.
If you want, next we can:
design the context registry Rust struct + file IO
walk through charter init / create / use step-by-step
or simulate multiple contexts with imports
You’re building this the right way.

---

Excellent instinct — and yes, your NDJSON idea is fundamentally sound, but the context registry is the one place where I would not apply append-only event semantics.
Let me explain why, then show you how to reconcile both models cleanly without painting yourself into a corner.
Short Answer (Decision)
✅ Use append-only NDJSON for metadata that is
auditable
domain-meaningful
replayable
engine-adjacent
❌ Do NOT use append-only NDJSON for the context registry
keep it as a single mutable JSON document
This is not inconsistency — it’s boundary hygiene.
Why NDJSON Works Almost Everywhere Else
Append-only metadata logs are great when:
Property
NDJSON Metadata
Auditability
✅
Replay
✅
Debuggability
✅
Determinism
✅
Crash safety
✅
That makes them perfect for:
Area metadata
Area annotations
Configuration changes (user.name, defaults, etc.)
Engine feature flags
Import review state
Workspace state (sessions paused, blocked, etc.)
In other words: anything that lives inside Charter’s legitimacy universe.
Why Context Registry Is Different
The context registry:
Exists outside the engine
Is pure CLI plumbing
Has no historical meaning
Must be fast and trivial to resolve
Should survive schema upgrades easily
Should be user-editable in emergencies
If you make it append-only:
You introduce real problems
You must replay events on every CLI invocation
You need conflict resolution for renames
You need tombstones for deletions
You need compaction logic immediately
You need corruption handling for a file users may edit manually
All for something that does not benefit from history.
That’s the key point.
Does “One Entry for All Contexts” Cause Problems?
No — it’s exactly what you want.
Because:
Contexts are mutually exclusive
Only one context is active per working directory
The registry is a lookup table, not a log
Think of it like Git’s .git/config, not its object database.
Clean Hybrid Model (Recommended)
Here’s the pattern I recommend you lock in:
Copy code

~/.charter/
├── contexts.json          ← mutable registry (NOT NDJSON)
├── global-metadata.ndjson ← append-only metadata events
└── cache/
Rule of thumb
If deleting or editing it must not leave an audit trail → mutable JSON
If replaying it must reconstruct state → NDJSON
If You Really Want NDJSON for Contexts (Advanced Option)
You can do it safely only if you accept this constraint:
The context registry is always compacted on write.
That means:
NDJSON is internal
Only the last event per context name matters
You rewrite the file after every mutation
At that point… you’ve reinvented a JSON file with extra steps.
So I would not recommend it.
Suggested Rust Structures
Context Registry (simple, stable)
Copy code
Rust
struct ContextRegistry {
    version: u32,
    contexts: HashMap<String, ContextEntry>,
}

struct ContextEntry {
    storage_root: PathBuf,
    created_at: Timestamp,
    last_used_at: Option<Timestamp>,
}
Global Metadata (append-only)
Copy code
Json
{ "event": "config.set", "key": "user.name", "value": "alice", "ts": "..." }
{ "event": "area.annotate", "area": "A-ENG", "text": "..." }
{ "event": "import.opened", "import_id": "IMP-1", "ts": "..." }
Mental Model to Keep You Safe
Ask this question:
If I delete this file, do I expect governance history to be damaged?
If yes → object store / audit / NDJSON
If no → metadata JSON
The context registry is firmly in the second category.
Final Recommendation (Locked)
Keep contexts.json as a single mutable document
Use NDJSON append-only logs everywhere else
Never mix context resolution with engine state
Never export or import context data
This preserves:
isolation (Invariant 32)
determinism
debuggability
sanity
If you want next, we can:
design context registry read/write code in Rust
define failure modes (missing registry, dangling pointer)
or map context switching → engine boot sequence
You’re making consistently strong architectural calls — this one just needs a firm boundary, and now you’ve got it.

---

Yes — the engine boot sequence lives at the CLI (or host) boundary, not inside Charter Core itself.
Charter Core must remain pure, deterministic, storage-root–bound, and context-agnostic.
Below is a clean, layered boot sequence that matches your invariants, storage isolation rule, and future API plans.
High-Level Boot Responsibility Split
CLI / Host (owns)
Context resolution
Storage root selection
File existence & creation
Locking / concurrency guards
Migration checks
Engine construction
Charter Core Engine (owns)
Loading stores from a given root
Index reconstruction
Invariant validation
Read/write operations
Deterministic behavior
The engine never:
Picks a context
Touches global registries
Walks directories upward
Infers storage locations
Engine Boot Sequence (Step-by-Step)
Phase 0 — CLI Entry
Copy code
Text
$ charter status
CLI decides:
current working directory
command intent
whether context resolution is required
Phase 1 — Context Resolution (CLI Layer)
Input sources (in order):
Explicit flag: --context <name>
Local pointer file (e.g. .charter-context)
Default context (if exists)
Pseudocode
Copy code
Rust
fn resolve_context(cwd: Path) -> Context {
    if let Some(name) = cli_flag_context() {
        return registry.lookup(name)
    }

    if let Some(name) = read_local_pointer(cwd) {
        return registry.lookup(name)
    }

    registry.default_context()
}
❗ Failure here is CLI-level, not engine-level.
Phase 2 — Storage Root Validation (CLI Layer)
Copy code
Text
Context:
  name: "work"
  storage_root: ~/.charter/contexts/work/
Checks:
directory exists (or create on init)
permissions OK
optional lock file acquisition
Copy code
Rust
assert!(storage_root.is_dir());
acquire_lock(storage_root)?;
Phase 3 — Engine Boot (Boundary Crossing)
This is where Charter Core begins.
Copy code
Rust
let engine = Engine::open(storage_root)?;
From this point on:
No context names
No CLI concepts
Only paths + data
Phase 4 — Store Initialization (Engine)
Engine initializes all stores deterministically.
Copy code
Rust
Engine {
    object_store,
    ref_store,
    metadata_store,
    audit_store,
    index,
}
Concrete steps
Open / create store files
objects
refs
metadata
audit
Load raw data
deserialize objects
read refs
read metadata
read audit log
❗ No invariants enforced yet.
Phase 5 — Index Reconstruction (Engine)
Indexes are derived state and are always rebuilt.
Examples:
resolution supersession graph
active authority per area
active scope per area
area existence map
dangling references
orphaned objects
Copy code
Rust
index = Index::rebuild(
    object_store.iter(),
    ref_store.iter()
)?;
This is equivalent to git fsck --full + index rebuild.
Phase 6 — Structural Validation (Engine)
Hard failures:
missing referenced objects
broken hash integrity
impossible lifecycle transitions
invalid supersession graphs
Soft findings:
dangling objects
orphaned areas
unused resolutions
Copy code
Rust
index.validate_structure()?;
❗ Failure here means storage corruption, not user error.
Phase 7 — Legitimacy State Evaluation (Engine)
Now evaluate current active state:
Active Areas
Active Authority per Area
Active Scope per Area
Blocked / paused sessions
Initialization status
Copy code
Rust
engine.state = engine.evaluate_state();
This does not mutate history.
Phase 8 — Engine Ready
Engine is now usable.
Copy code
Text
Engine ready:
  Areas: 3
  Active Sessions: 1
  Blocked Sessions: 2
  Storage OK
CLI may now execute commands.
Full Boot Flow Diagram
Copy code

CLI
 ├─ resolve context
 ├─ validate storage root
 ├─ acquire lock
 └─ Engine::open(path)
      ├─ load stores
      ├─ rebuild indexes
      ├─ validate structure
      ├─ evaluate legitimacy state
      └─ ready
Why This Design Is Correct
Matches Invariants
Storage Isolation (32): single root, no leakage
Deterministic Evaluation (31): derived state rebuilt every time
Audit Scope Supremacy (23): audit store loaded before actions
Import Safety: imports use same boot path
Git-Equivalent Concepts
Git
Charter
.git/config
context registry
.git/HEAD
local context pointer
object db
object store
refs
ref store
index
rebuilt on startup
git fsck
index validation
Where This Lives in Code
Copy code
Text
charter-cli/
 └── main.rs
      └── boot.rs      ← context + storage resolution

charter-core/
 └── engine/
      ├── mod.rs       ← Engine::open
      ├── stores.rs
      ├── index.rs
      └── validate.rs
CLI depends on core. Core never depends on CLI.
Key Rule to Remember
The engine starts from a directory and nothing else.
All meaning above that line is the CLI’s responsibility.
If you want next, we can:
write Engine::open() pseudocode in Rust
design the index struct
define boot-time error types
simulate corrupted storage cases
map boot → import → review flows
You’ve now got a rock-solid spine for Charter’s runtime.
