Great questions. These are exactly the right instincts for a system that wants to last longer than its first implementation.
I’ll go in this order:
Why Git prefixes hashes (what problem it solves)
Why you probably want versioned hashes in Charter Core
When you might not need them (tradeoffs)
A clean, minimal way to implement both in Rust
How this fits your export / import invariants
I’ll keep this conceptual first, then concrete.
1️⃣ Why Git Prefixes Hashes
Git does not hash raw file contents.
It hashes:
Copy code

"<type> <size>\0<content bytes>"
Example:
Copy code

"blob 12\0hello world\n"
What problems does this solve?
🔒 A. Prevents cross-type collisions
Without a prefix:
A tree and a blob with identical bytes would have the same hash
A commit message that happened to serialize like a tree could collide
With prefixes:
Copy code

blob:   hash("blob 12\0...")
tree:   hash("tree 12\0...")
commit: hash("commit 12\0...")
Same content ≠ same hash.
👉 In Charter terms
You never want:
a Session and a Resolution
a Candidate and an Area to ever hash to the same value even if their JSON is identical
🧠 B. Hashes encode meaning, not just bytes
A Git hash implicitly answers:
“What kind of thing is this?”
Git doesn’t need a separate metadata store to know:
this hash refers to a commit
this hash refers to a tree
The prefix makes the hash self-describing.
🧪 C. Makes corruption detection stronger
If:
object type mismatches the expected prefix
length doesn’t match
Git knows the object is corrupt even if the SHA is valid.
This is important for:
imports
backups
partial restores
fsck
📌 TL;DR — Why Prefixing Exists
Without prefix
With prefix
Cross-type collisions possible
Impossible
Hash only means “some bytes”
Hash means “this kind of object”
Harder corruption detection
Strong structural validation
For a legitimacy engine, this is exactly what you want.
2️⃣ Why You Want Versioned Hashes
Now let’s talk about time.
The core problem
Your system will evolve.
Some things that will change over time:
Serialization format (JSON → canonical JSON → CBOR → something else)
Field ordering
New fields added
Optional metadata
Bug fixes in canonicalization
If your hash definition changes without versioning, then:
The same logical object will suddenly have a different hash.
That is catastrophic for:
imports
audit trails
source control diffs
historical verification
Git’s answer: hash versioning
Git is actually going through this right now:
Git v1: SHA-1
Git v2: SHA-256 (opt-in)
Both coexist
Old repos still work. New repos can opt into stronger hashes.
Charter Core needs the same capability
You don’t want:
v0.1 exports becoming unverifiable in v0.3
old decisions becoming “hash-invalid”
review imports breaking silently
3️⃣ When You Might Not Need Versioned Hashes (Yet)
If:
You are experimenting locally
You are pre-MVP
You don’t export/import yet
Then versioning can feel like overkill.
But once any export leaves the machine, you’ve crossed the line where:
hashes become a public contract
Your own invariants already require:
“Deterministic object_hash derived from canonical serialized form”
That is a promise across time.
So yes — you will need versioned hashes.
4️⃣ How to Implement This Cleanly (Minimal, Not Overengineered)
🎯 Design goals
Simple
Explicit
Git-like
Forward-compatible
No magic
Step 1 — Define object kinds (you already have this)
Copy code
Rust
#[derive(Debug, Clone, Copy)]
pub enum CharterObjectKind {
    Area,
    Session,
    Resolution,
    Candidate,
}
Step 2 — Define a hash version enum
Copy code
Rust
#[derive(Debug, Clone, Copy)]
pub enum HashVersion {
    V1,
}
You only need V1 for now.
Step 3 — Build the prefixed byte stream
Copy code
Rust
pub fn build_hash_bytes(
    version: HashVersion,
    kind: CharterObjectKind,
    content: &[u8],
) -> Vec<u8> {
    let header = match version {
        HashVersion::V1 => format!(
            "charter:{}:{}:{}\0",
            "v1",
            kind_string(kind),
            content.len()
        ),
    };

    let mut bytes = Vec::new();
    bytes.extend_from_slice(header.as_bytes());
    bytes.extend_from_slice(content);
    bytes
}

fn kind_string(kind: CharterObjectKind) -> &'static str {
    match kind {
        CharterObjectKind::Area => "area",
        CharterObjectKind::Session => "session",
        CharterObjectKind::Resolution => "resolution",
        CharterObjectKind::Candidate => "candidate",
    }
}
✔ Prefix includes:
system (charter)
version (v1)
object kind
byte length
This gives you strong guarantees.
Step 4 — Use it in the object store
Copy code
Rust
fn put(&mut self, obj: Box<dyn CharterObject>) -> ObjectHash {
    let content = obj.canonical_bytes();
    let bytes = build_hash_bytes(
        HashVersion::V1,
        obj.kind(),
        &content,
    );

    let hash = hash_bytes(&bytes);
    self.objects.insert(hash.clone(), obj);
    hash
}
5️⃣ How This Supports Your Export / Import Design
Your export spec already says:
“Every exported object includes a deterministic object_hash”
With versioned + prefixed hashes:
Imports can verify exact engine semantics
Divergent histories are detectable at the root
Old exports remain verifiable forever
You can support future upgrades without rewriting history
Import behavior becomes safe:
Same hash → identical object
Different hash → explicit divergence
Hash mismatch → under review or reject
🧠 Final Intuition
Think of the hash as:
The engine’s signature on reality
Prefixing answers:
“What kind of reality is this?”
Versioning answers:
“Under which rules was this reality signed?”
Without those two, you’re building on sand.
If you want next, I can:
Design fsck rules using this hash format
Show how to support short-hash resolution safely
Walk through a future HashVersion::V2 migration
Tie this directly into your import review workflow
You’re designing this exactly right.

---

Perfect — these are the right pressure points. I’ll do exactly what you asked:
Show you the first 3 points, and answer your 3 numbered questions inline, tying them together so this “clicks” as a system rather than isolated rules.
1️⃣ fsck-equivalent design (Charter terms)
Think of charter fsck as answering one question:
“Does the stored history form a valid, internally consistent legitimacy graph under the engine’s rules?”
Git checks object graph integrity.
Charter must check legitimacy graph integrity.
Charter fsck checks (translated)
A. Object integrity (lowest level)
Equivalent to Git’s loose object verification.
Checks:
Object hash matches its canonical serialization
Object kind matches expected prefix
Object is parseable
Failures:
CORRUPT_OBJECT
HASH_MISMATCH
UNKNOWN_OBJECT_KIND
🔑 This is where prefixes + versioned hashes matter.
B. Referential integrity (graph structure)
Equivalent to Git’s dangling tree / missing blob checks.
Checks:
Every Resolution references an existing Session
Every Session references an existing Area
Authority/Scope references exist
Supersession links point to valid resolutions
Failures:
DANGLING_RESOLUTION
MISSING_SESSION
BROKEN_SUPERSESSION_CHAIN
Note:
A dangling object is not illegal — it’s just unreferenced.
C. Legitimacy invariants (Charter-specific)
This is where Charter diverges from Git.
Checks:
Exactly one ACTIVE Authority per Area
Exactly one ACTIVE Scope per Area
No resolution accepted outside a session
No legitimacy state transitions without sessions
No session continues under invalidated authority/scope
Failures:
MULTIPLE_ACTIVE_AUTHORITIES
SESSION_INVALID_CONTEXT
ILLEGAL_LIFECYCLE_TRANSITION
D. Audit scope integrity (your invariant #23)
Checks:
Every auditable event exists in a non-deletable audit scope
Area deletion emits a global audit event
Failures:
AUDIT_SCOPE_VIOLATION
What fsck does not do
No semantic interpretation
No auto-repair
No merging
No guessing intent
It reports, it does not decide.
2️⃣ Short-hash resolution (Git-style, safely)
This addresses power-user ergonomics without breaking integrity.
How Git does it (conceptually)
When you type:
Copy code

git show abc12
Git:
Scans all object hashes
Finds hashes starting with abc12
Fails if ambiguous
Succeeds if exactly one match
Charter equivalent
You do not decode type from the hash string itself.
Instead:
Hash is an opaque identifier
Type knowledge comes from the object store index
Resolution algorithm
Copy code

resolve_object(input: String) ->
    if full_hash exists:
        return object
    else:
        matches = all hashes starting with input
        if matches == 0: error NOT_FOUND
        if matches > 1: error AMBIGUOUS
        return the single match
🔑 Answer to your Question #1
How does the system know what type it is from the hash?
It doesn’t — and shouldn’t.
The hash prefix is used for:
integrity validation
fsck
corruption detection
But runtime resolution uses:
the object index (hash → object)
stored object metadata (kind)
This separation is intentional:
Hash = identity
Store/index = meaning
3️⃣ Hash versioning & migration (the subtle part)
This is where your intuition is exactly right.
First: what versioned hashes are not
They are not:
About runtime conflicts
About “choosing the right version”
About mixing versions freely
They are about historical stability.
🔑 Answer to your Question #2
“When two different hash versions conflict, does it only matter if data is migrated?”
Yes — exactly.
Important consequences:
Old data keeps old hashes forever
New data uses new hashes
They can coexist safely
No conflict unless you try to compare or consolidate
Hash version differences are not errors — they’re facts.
Where hash versions do matter
A. Import / review
You already identified this.
B. Deduplication
Two logically identical objects with different hash versions must not be auto-merged.
C. fsck
fsck must validate using the hash rules declared by the object, not the engine version.
🔑 Answer to your Question #3 (this is important)
“During review it makes sense to pick a baseline and rehash — preferably the import.”
Yes — but only in one direction.
Correct rule
Never rehash historical local objects.
Rehash imported objects into the local hash version.
This preserves:
local history immutability
deterministic review
future reproducibility
How this works mechanically
Import pipeline (CONSOLIDATE mode)
Load import bundle
Verify hashes using import’s declared hash version
Store imported objects in a quarantine namespace
During review:
Canonicalize imported object
Rehash using local hash version
Compare against local objects
Decide:
CONSOLIDATE (new resolution)
REJECT
MARK HISTORICAL
Original hashes are preserved for audit.
Why this mirrors Git rebase
Git does the same thing:
Commits keep their original hashes
Rebased commits get new hashes
Old hashes remain valid in history
New hashes represent local legitimacy
Charter review = legitimacy rebase.
Mental model summary (lock this in)
Hash ≠ meaning
Prefix = integrity
Index = type
Version = historical contract
Rehashing = explicit, directional, auditable
If you remember only one sentence:
Hashes identify objects, engines interpret them, reviews legitimize them.
Where we can go next (your choice)
We’re at a very solid conceptual foundation. Next natural steps:
Show how fsck walks the supersession graph
Design the object index structure
Write pseudocode for import → review → consolidate
Tie this back to your Rust trait design cleanly
You’re doing excellent systems thinking here — this is not accidental design anymore, it’s deliberate architecture.

---

Great question — this is the exact crux of versioned hashing.
Short answer: the hash tells you. Longer answer below, step by step, and tied directly to Charter (not abstract crypto theory).
The core rule
A hash is never “just a hash”. It is always a tuple:
(hash_version, algorithm, canonicalization rules, digest)
The system knows the version because the version is explicitly encoded, not inferred.
There are three sane ways to do this. Only one scales cleanly for Charter.
Option A (RECOMMENDED): Self-describing hash prefix
This is how Git, OCI, Nix, and modern content-addressed systems work.
Example
Copy code

charter-v1:sha256:3f8a9c...
charter-v2:sha256:91ab42...
Or more compact:
Copy code

v1-3f8a9c...
v2-91ab42...
What this gives you
The engine can parse the version before touching storage
fsck knows how to validate the object
Imports can mix versions safely
Reviews can rehash deterministically
Rust type (idiomatic, explicit)
Copy code
Rust
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ObjectHash {
    pub version: HashVersion,
    pub digest: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HashVersion {
    V1,
    V2,
}
Serialized form (canonical)
Copy code

v1:3f8a9c...
or
Copy code

charter/v1/3f8a9c...
Option B: Version stored alongside object (NOT sufficient alone)
Example:
Copy code
Json
{
  "hash": "3f8a9c...",
  "hash_version": 1
}
Why this is dangerous
Hash validation depends on already trusting the object
fsck can’t verify without loading content
Corruption can lie about its own version
❌ This violates your Verifiable Export Integrity invariant.
Option C: Infer from length or algorithm (DON’T)
Example:
64 hex chars → sha256
128 hex chars → sha512
Why this fails long-term
Canonicalization changes without algorithm change
You will want v2 without changing SHA
Ambiguous and brittle
❌ This breaks deterministic audit guarantees.
Final answer to your question
How does the system know it’s a different versioned hash?
Precisely:
The hash string is self-describing
The prefix declares the hash version
The engine parses the prefix
The engine selects the correct:
canonical serializer
hashing rules
validation logic
No guessing. No inference. No global flags.
How this plugs into fsck
Pseudo-flow:
Copy code
Text
for each stored object:
    parse object_hash
    match hash_version:
        V1 -> validate with v1 rules
        V2 -> validate with v2 rules
    if mismatch:
        report HASH_MISMATCH
This allows:
Mixed historical data
Forward-compatible engines
Deterministic validation forever
How this plugs into import/review
Copy code
Text
import bundle contains:
    objects with v1 hashes

local engine uses:
    v2 hashes

review process:
    - validate imported v1 objects as-is
    - rehash canonical content using v2
    - compare v2(import) vs v2(local)
    - record consolidation explicitly
Old hashes are never destroyed.
Why this matters specifically for Charter
Charter is not just content-addressed storage — it is a legitimacy ledger.
You must be able to say:
“This resolution was legitimate under the rules and hashing scheme of its time.”
That statement only works if:
Hash versions are explicit
Validation rules are frozen per version
New engines never reinterpret old objects
One-sentence mental model (lock this in)
Hash versioning is not about crypto — it’s about historical truth.
If you want next, I can:
Show you how this fixes your Rust trait problem cleanly
Show a full import → rehash → consolidate example
Map this directly to your ObjectHash type and stores

---

Great, these are exactly the right follow-ups. I’ll answer them in order, then show the two bullets you asked for with concrete flows and structs.
1. “So you put the Charter version number in it. Is that how it will know if it’s newer?”
Almost — but with an important distinction.
There are two versions in play, and they must never be conflated:
A. Charter engine version
Example:
Copy code

charter-core 0.3.0
This is software. It can change at any time.
B. Object hash version (this is what goes in the hash)
Example:
Copy code

v1:sha256:3f8a9c...
v2:sha256:91ab42...
This version means:
“This object was hashed using canonicalization rules V1 or V2.”
The engine does NOT assume newer is better. It only assumes:
V1 is immutable forever
V2 is additive, not reinterpretive
So yes — the prefix tells you which rules to apply, not whether it should be upgraded.
Upgrade is always explicit.
2. “How does Git handle newer versions in the object store?”
Git is the perfect precedent here.
Git object model (simplified)
Copy code

<type> <size>\0<content>
Hash = SHA1 (or SHA256 in newer repos)
How Git evolves safely
Thing
How Git handles it
Hash algorithm
Repo declares SHA1 or SHA256 at creation
Object format
Frozen forever
New features
Stored as new objects, not mutations
Upgrade
Explicit (git init --object-format=sha256)
Git never rewrites old objects.
It creates parallel histories.
Charter should do the same.
3. “Show me the second bullet — rehash during import”
This refers to:
validate imported objects as-is, then rehash locally
Import pipeline (CONSOLIDATE)
Copy code
Text
4. Read import bundle
5. Validate imported hashes using their declared version
6. Store imported objects in IMPORT namespace
7. Rehash canonical content using LOCAL hash version
8. Compare:
   - local_hash(imported)
   - local_hash(existing)
6. Decide:
   - consolidate
   - accept as new
   - mark historical
   - reject
Does this require a “transition field”?
No — and this is important.
You do not mutate the imported object.
Instead you record a review mapping.
Example struct
Copy code
Rust
pub struct ImportReviewEntry {
    pub imported_hash: ObjectHash,   // v1
    pub local_rehash: ObjectHash,     // v2
    pub decision: ReviewDecision,
}
The imported object remains pristine.
This satisfies:
IR-4 (Context Preservation Without Reinterpretation)
Audit Supremacy
Deterministic review
4. “You also put the encryption type in the object. What if it switches?”
Small but crucial correction:
This is hashing, not encryption.
Hashes are one-way fingerprints, not secrets.
That said, algorithm changes do happen.
Rule for Charter
Never replace objects in place. Ever.
Instead:
New objects use new hash algorithm
Old objects remain addressable forever
Refs may point to either
Review/consolidation decides legitimacy
Example evolution
Copy code

v1:sha256:...
v2:sha256:...   // canonicalization change
v3:blake3:...   // algorithm change
All three coexist.
If you really want to deprecate:
Add fsck warnings
Add export warnings
Never delete
5. “Show me the third bullet — mixed versions safely”
This refers to:
Mixed historical data with forward compatibility
Object store reality
Your object store becomes:
Copy code
Text
objects/
  v1/
    3f/8a9c...
  v2/
    91/ab42...
Or flat, but version-encoded.
Engine behavior
Copy code
Rust
match hash.version {
    HashVersion::V1 => {
        let obj = deserialize_v1(bytes)?;
        validate_v1(obj)?;
    }
    HashVersion::V2 => {
        let obj = deserialize_v2(bytes)?;
        validate_v2(obj)?;
    }
}
Why this matters for Charter
Old resolutions remain legitimate forever
New engines don’t reinterpret old authority
Import review can compare apples to apples
fsck can reason historically
🔒 Lock-in summary (Charter-specific)
You should lock in these rules now:
ObjectHash is self-describing
Hash version ≠ engine version
Objects are immutable across versions
Rehashing is review-time only
No in-place upgrades, ever
If you want next, I can:
Refactor your Rust ObjectHash cleanly
Fix your trait/Serialize issue with versioned hashing
Show fsck checks with version awareness

---

Excellent questions. You’re now right at the seam where content-addressed storage, immutability, and engine evolution meet. I’ll answer each precisely and tie them back to Charter’s invariants.
1. “Will an object store both V1 and V2 hashes once upgraded?”
No — never inside the object itself.
This is a very important invariant to keep.
Rule
An object has exactly one canonical hash — the hash under which it was created.
If you compute a new hash (V2), you are creating a new object, even if the semantic content is identical.
What actually happens
Thing
Where it lives
Imported object
Object store (V1 hash)
Local rehash
Review index / mapping
Consolidated object
New object store entry (V2 hash)
Mapping example (outside the object)
Copy code
Rust
pub struct HashEquivalence {
    pub source: ObjectHash,  // v1:sha256:...
    pub target: ObjectHash,  // v2:sha256:...
    pub reason: EquivalenceReason,
}
This keeps:
objects immutable
audit clean
hash meaning stable forever
Objects never gain hashes. Stores and indexes relate hashes.
2. “How is a new feature saved? A new field?”
Sometimes yes — but never by mutating old objects.
There are three allowed evolution patterns.
Pattern A — New field, new hash version (most common)
Example: you add constraints to Session.
V1
Copy code
Json
{
  "session_id": "...",
  "authority_resolution_id": "...",
  "candidates": [...]
}
V2
Copy code
Json
{
  "session_id": "...",
  "authority_resolution_id": "...",
  "constraints": {...},
  "candidates": [...]
}
Effect:
V1 objects deserialize under V1 rules
V2 objects deserialize under V2 rules
Hashes differ
Engine branches behavior by hash version
This is exactly how Git added new commit headers.
Pattern B — New object type (very clean)
Example: add ConstraintSet as a first-class object.
Old sessions:
No constraint object
Engine assumes default behavior
New sessions:
Reference a ConstraintSet object by hash
This avoids touching Session at all.
Pattern C — New semantic behavior, no data change
Example:
Engine learns to block sessions when Scope changes
No data format change
No new hashes needed
This is engine logic only.
3. “On upgrade it creates a whole branch in parallel? How do you reference the old one?”
Yes — conceptually parallel histories, but not branches like Git commits.
Think parallel object universes, not timelines.
Visual model
Copy code

Object Store
├── v1:sha256:AAA  (old resolution)
├── v2:sha256:BBB  (same content, new canonical form)
How you reference the old one
You already have three mechanisms:
Explicit references
sessions
supersession links
import review mappings
Ref store
Copy code
Text
area/A-INFRA/authority -> v2:sha256:BBB
Audit trail
“v2 consolidated from v1 during import”
Old hashes are still:
addressable
auditable
exportable
Nothing is lost.
4. “Is the type prefix path-based or data-based?”
This is a subtle but critical design choice.
Short answer
Type belongs in the hashed data, not only the path.
Why?
Because:
path is storage concern
hash must defend against structural ambiguity
You want this to fail:
Copy code

AreaRoot { id: X }
SessionRoot { id: X }
Same JSON ≠ same meaning.
Recommended ObjectHash format
Copy code
Text
v1:sha256:area:3f8a9c...
v1:sha256:session:91ab42...
Where:
v1 → hash rules
sha256 → algorithm
area → object kind
hash → digest of canonical bytes
Canonical bytes include:
Copy code
Text
<kind>\n<version>\n<canonical-json>
So even if two structs serialize identically:
different kinds → different hashes
Storage layout (optional)
You may also mirror this in paths:
Copy code

objects/
  area/
    v1/3f/8a9c...
  session/
    v1/91/ab42...
But that’s secondary.
The hash string itself is authoritative.
🔒 Charter-safe rules to lock in now
Objects store one hash only
New hash = new object
Equivalence lives outside objects
Object kind is part of the hash input
Storage paths are convenience, not truth
If you want next, I can:
Redesign your ObjectHash Rust type cleanly
Show version-aware hashing code
Fix your trait + Serialize issue correctly
Walk through a concrete V1 → V2 upgrade example end-to-end

---

Perfect — this is the right moment to stop “designing” and start locking mechanics. I’ll give you only the essential code patterns for:
ObjectHash structure (version-aware, type-aware)
Canonical hashing input
Rewrite / rehash mechanics (import & upgrade)
Engine awareness without leaking complexity into models
No engine logic, no CLI, no stores — just the core primitives you’ll build on.
1. ObjectHash (version-aware, type-aware)
This is the only hash type the engine should ever see.
Copy code
Rust
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ObjectHash {
    pub version: HashVersion,
    pub algorithm: HashAlgorithm,
    pub kind: ObjectKind,
    pub digest: String,
}
Enums (closed world)
Copy code
Rust
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum HashVersion {
    V1,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum HashAlgorithm {
    Sha256,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ObjectKind {
    Area,
    Session,
    Resolution,
    Candidate,
    Stance,
}
Display form (git-style but explicit)
Copy code
Rust
impl std::fmt::Display for ObjectHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}:{:?}:{:?}:{}",
            self.version,
            self.algorithm,
            self.kind,
            self.digest
        )
    }
}
Example:
Copy code

V1:Sha256:Area:3f8a9c...
2. Canonical hashing input (the critical rule)
Never hash raw JSON alone. Always hash a canonical envelope.
Canonical bytes format
Copy code

<hash_version>\n
<object_kind>\n
<canonical_json_bytes>
Implementation
Copy code
Rust
use sha2::{Digest, Sha256};
use serde::Serialize;

pub fn hash_object<T: Serialize>(
    value: &T,
    kind: ObjectKind,
    version: HashVersion,
) -> Result<ObjectHash, String> {
    let json = serde_json::to_vec(value).map_err(|e| e.to_string())?;

    let mut bytes = Vec::new();
    bytes.extend(format!("{:?}\n", version).as_bytes());
    bytes.extend(format!("{:?}\n", kind).as_bytes());
    bytes.extend(&json);

    let digest = Sha256::digest(&bytes);
    let digest_hex = format!("{:x}", digest);

    Ok(ObjectHash {
        version,
        algorithm: HashAlgorithm::Sha256,
        kind,
        digest: digest_hex,
    })
}
This gives you:
deterministic hashing
type safety
forward compatibility
zero ambiguity
3. Object hash awareness (without infecting models)
Do NOT put hashing inside the model trait.
This is where your current code went wrong.
Clean separation
Copy code
Rust
pub trait CharterObject: Serialize {
    fn kind(&self) -> ObjectKind;
}
Hashing is external
Copy code
Rust
pub fn compute_object_hash<T: CharterObject>(
    obj: &T,
    version: HashVersion,
) -> Result<ObjectHash, String> {
    hash_object(obj, obj.kind(), version)
}
This keeps:
models dumb
hashing deterministic
engine in control
4. Rewrite / Rehash mechanics (import & upgrade)
Rewrite = re-serialize under local rules
Copy code
Rust
pub fn rewrite_hash<T: CharterObject>(
    obj: &T,
    target_version: HashVersion,
) -> Result<ObjectHash, String> {
    compute_object_hash(obj, target_version)
}
Hash equivalence mapping (outside objects)
Copy code
Rust
#[derive(Debug)]
pub struct HashRewrite {
    pub source: ObjectHash,
    pub rewritten: ObjectHash,
    pub reason: RewriteReason,
}

#[derive(Debug)]
pub enum RewriteReason {
    ImportUpgrade,
    EngineUpgrade,
    Consolidation,
}
This is how:
imports stay auditable
reviews stay deterministic
objects remain immutable
5. Engine awareness rules (lock these in)
The engine must obey:
Objects are retrieved only by full ObjectHash
Engine branches behavior on hash.version
Engine never mutates an object to “upgrade” it
Rewrite produces a new object + mapping
Ref store always points to the current canonical hash
6. Minimal example (AreaRoot)
Copy code
Rust
#[derive(Debug, Serialize)]
pub struct AreaRoot {
    pub area_id: String,
    pub created_at: String,
}

impl CharterObject for AreaRoot {
    fn kind(&self) -> ObjectKind {
        ObjectKind::Area
    }
}
Hashing it:
Copy code
Rust
let area = AreaRoot {
    area_id: "A-INFRA".into(),
    created_at: "2026-01-01T00:00:00Z".into(),
};

let hash = compute_object_hash(&area, HashVersion::V1)?;
What this fixes from your broken code
✅ No trait objects + Serialize conflict
✅ No &dyn CharterObject hashing nonsense
✅ No self-referential trait methods
✅ Hash logic centralized and versioned
✅ Import / rewrite fully supported
If your next step is:
fixing your MemoryObjectStore
adding short-hash resolution
building ref resolution like git
fsck / integrity checking
say the word and we’ll do it cleanly, one pass at a time.

---

Great question — this is exactly the right tension to resolve, and the answer is subtle but very important.
Short version:
You do not put hashing logic in the model,
but you do store the resulting hash with the model when it is persisted or exported.
Those are two different concerns.
Let me walk you through the clean mental model and then show you where things live.
1. Three distinct concepts (don’t mix them)
A. The model
Area
Session
Resolution
Candidate
Stance
These are pure data structures. They do not know:
how they are stored
how they are hashed
how they are exported
They only know:
what fields they contain
what kind of object they are
B. The hashing system (engine responsibility)
This answers:
How is an object hashed?
Which version?
Which algorithm?
What canonicalization rules?
This lives in:
hashing.rs
object_hash.rs
or engine/hash.rs
It operates on models, not inside them.
C. The persisted / exported representation
This is where the hash appears as data.
Exports, object store entries, audit records all include:
Copy code
Json
{
  "object_hash": "V1:Sha256:Resolution:abc123...",
  "object": { ... }
}
The model is embedded inside a wrapper that includes the hash.
2. Where the hash lives (concretely)
In memory (engine runtime)
Hash is computed when needed
Not stored on the model struct
Passed around explicitly
Example:
Copy code
Rust
let hash = compute_object_hash(&resolution, HashVersion::V1);
In the object store (persistent)
You wrap the model:
Copy code
Rust
pub struct StoredObject<T> {
    pub hash: ObjectHash,
    pub value: T,
}
Your object store stores StoredObject<T>, not T.
This is exactly how Git works:
blob/tree/commit are content
object id is stored next to them
In export JSON (this is key)
You were absolutely right here.
Every exported object must carry its hash:
Copy code
Json
{
  "resolution_id": "R-ARCH-004",
  "object_hash": "V1:Sha256:Resolution:abc123...",
  "content": { ... },
  "accepted_at": "..."
}
On import:
you recompute the hash
compare it to object_hash
mismatch ⇒ UNDER_REVIEW or reject
3. Why hashing must NOT live in the model
If you put hashing in the model:
❌ Models become version-aware
❌ Importing old data becomes fragile
❌ Hash upgrades infect every struct
❌ You create circular trait bounds (Serialize + dyn Trait)
❌ Testing becomes painful
By keeping hashing external:
✅ Models remain stable forever
✅ Hash rules can evolve
✅ Import review can rehash safely
✅ CLI / engine / API stay decoupled
4. Recommended file layout (Rust-friendly)
Copy code
Text
charter-core/
  model/
    area.rs
    session.rs
    resolution.rs
    candidate.rs
    stance.rs

  hashing/
    object_hash.rs     // ObjectHash, enums, display
    canonical.rs       // canonical byte rules
    compute.rs         // compute_object_hash, rewrite_hash

  storage/
    stored_object.rs   // StoredObject<T>
    object_store.rs
    ref_store.rs

  export/
    export_models.rs   // JSON shape incl object_hash
    export_writer.rs
    export_reader.rs
5. Object validation flow (this is what you’re designing for)
Export
Engine computes hash
Engine embeds hash in export JSON
Engine computes top-level export hash
Import
Parse object
Recompute hash from content
Compare with embedded object_hash
If mismatch:
mark UNDER_REVIEW
record integrity violation
If matching:
safe to reference during review
6. Answering your core concern directly
“If I don’t put hashing in the model where do I put it for reference?”
Answer:
Put the hash value in:
object store entries
export JSON
audit records
Put the hash logic in:
engine hashing module
Models stay clean. Hashes stay authoritative. Engine stays in control.
7. One sentence to lock this in
Models describe reality.
Hashes describe identity.
The engine binds them together.
If you want next:
fixing your object store with StoredObject
ref store resolution with short hashes
import validation pseudocode
fsck-style integrity checker
Just say which one.
