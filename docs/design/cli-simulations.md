# Charter CLI — Canonical Simulations (Frozen)

Status: **FROZEN**  
Applies to: **Charter CLI behavior layered over Charter Core**  
Does NOT define: UX guarantees, permissions, roles, or engine internals

These simulations validate that the **CLI preserves engine legitimacy while enforcing cognitive safety and operational discipline**.

They are:
- not tests
- not UX walkthroughs
- not promises of convenience

They answer one question only:

> **Does the CLI ever allow a human to bypass, dilute, or accidentally fabricate legitimacy?**

If any simulation fails, a **CLI invariant has been violated**.

---

## I. Bootstrap & Governance Formation (CLI-Mediated)

### Simulation CLI-1 — Area Initialization (Mandatory Bootstrap)

**Context**  
A new Area cannot function until governance exists.

**Flow**
1. `charter init`
2. Area `A-1` created
3. Area status: `UNINITIALIZED`
4. CLI blocks all sessions except Authority / Scope definition

**Authority Definition**
- Session started: “Define Authority”
- Participants explicitly declared
- Authority candidate proposed
- Session accepted

**Scope Definition**
- Session started: “Define Scope”
- Participants explicitly declared
- Scope candidate proposed
- Session accepted

**Outcome**
- Area becomes `INITIALIZED`
- Exactly one active Authority resolution
- Exactly one active Scope resolution

**Validated CLI Invariants**
- Areas require explicit governance
- Authority and Scope are first-class resolutions
- No implicit defaults
- Participants are never inferred by the CLI

---

## II. Normal Decision Making

### Simulation CLI-2 — Flat Authority Collaboration (Peers)

**Context**  
Two peers collaborate without hierarchy.

**Authority**  
`UNANIMOUS_PRESENT`

**Flow**
1. Session started with explicit participants (Alice, Bob)
2. Multiple candidates proposed
3. Both participants explicitly accept one candidate
4. Session accepted

**Outcome**
- Resolution accepted legitimately
- Participant set frozen at first stance

**Validated CLI Invariants**
- Explicit unanimity only
- Participant presence is mechanical, not assumed
- No silent acceptance

---

### Simulation CLI-3 — Single-User Governance (Solo Journal)

**Context**  
A solo founder uses Charter as a decision journal.

**Flow**
1. Session started
2. Candidate proposed
3. `session accept` used without explicit voting

**Engine Reality (Exposed by CLI Audit)**
- Participant list = `[founder]`
- Stance = `ACCEPT`
- Authority evaluated normally

**Outcome**
- Full audit trail preserved
- No shortcuts at the engine layer

**Validated CLI Invariants**
- Scale independence
- Explicit acceptance even in solo mode
- Ergonomic collapse does not weaken legitimacy

---

### Simulation CLI-4 — Normal Team Decision (Majority)

**Context**  
A team chooses an architecture.

**Authority**  
`MAJORITY_PRESENT`

**Flow**
1. Session started
2. Participants declared (Alice, Bob, Carol)
3. Votes recorded:
   - Alice: ACCEPT
   - Bob: ACCEPT
   - Carol: ABSTAIN

**Outcome**
- Majority satisfied
- Resolution accepted
- Abstention recorded explicitly

**Validated CLI Invariants**
- Abstention is first-class
- Majority is computed mechanically
- No semantic inference by CLI

---

## III. Blocking, Disagreement, and Reality

### Simulation CLI-5 — Deadlock Without Coercion

**Context**  
Unanimous authority with disagreement.

**Flow**
- Participants: Alice, Bob, Carol
- Votes:
  - Alice: ACCEPT
  - Bob: REJECT
  - Carol: ACCEPT
- Attempt to accept fails

**Outcome**
- Session enters `BLOCKED`
- No forced outcome
- No auto-closure

**Validated CLI Invariants**
- Deterministic blocking
- No coercion
- No implicit failure state

---

### Simulation CLI-6 — Participant Leaves Mid-Session

**Context**  
Deadlocked unanimous session reflects reality.

**Initial State**
- Alice: ACCEPT
- Bob: REJECT
- Carol: ACCEPT
- Session: `BLOCKED`

**Change**
- Bob removed explicitly from participant list

**CLI / Engine Response**
- Authority re-evaluated
- Remaining participants: Alice, Carol

**Outcome**
- Resolution accepted legitimately

**Validated CLI Invariants**
- Authority evaluates present participants only
- No vote reinterpretation
- Participant changes are explicit and auditable

---

## IV. Authority Evolution

### Simulation CLI-7 — Authority Change Requires Its Own Session

**Context**  
Team wants faster decisions.

**Flow**
1. New session started under existing Authority
2. New Authority candidate proposed
3. Accepted under old Authority

**Outcome**
- Old Authority superseded
- New Authority becomes active
- No retroactive impact

**Validated CLI Invariants**
- Authority mutable only via sessions
- Non-retroactivity
- Governance evolves explicitly

---

## V. CLI-Specific Cognitive Constraints

### Simulation CLI-8 — Concurrent Sessions (Solo Mode Constraint)

**Context**  
Solo mode limits cognitive load.

**Flow**
- Session A is active
- User attempts to start Session B

**Outcome**
- CLI rejects the action
- Engine state unchanged

**Validated CLI Invariants**
- CLI enforces ergonomic limits
- Engine remains neutral
- One-session rule respected

---

## VI. Baseline (Import / Consolidation)

### Simulation CLI-9 — Baseline Initialization

**Context**  
External history imported for consolidation.

**Flow**
1. `charter import consolidate legacy.json`
2. Baseline created
3. All imported resolutions marked `UNDER_REVIEW`
4. All active sessions paused

**Outcome**
- No local legitimacy created
- No authority inferred
- Baseline is sole active review process

**Validated CLI Invariants**
- Consolidation is non-destructive
- Import does not legitimize decisions
- One baseline at a time

---

### Simulation CLI-10 — Baseline Acceptance (Solo)

**Context**  
Solo user reviews imported history.

**Flow**
- Baseline opened
- User accepts resolutions one at a time
- Each acceptance creates a hidden session
- Authority evaluated locally

**Outcome**
- Accepted resolutions become `ACTIVE`
- Rejected or skipped ones remain non-active
- Baseline persists until explicitly closed

**Validated CLI Invariants**
- Acceptance always occurs via sessions
- Ergonomic batching does not weaken legitimacy
- Baseline is controlled and auditable

---

### Simulation CLI-11 — Baseline Acceptance (Multi-User, Long-Running)

**Context**  
Departments review external decisions over time.

**Flow**
- Baseline opened
- Resolution-by-resolution deliberation
- Baseline paused and resumed
- Sessions occur across days or weeks

**Outcome**
- Baseline persists across time
- Partial progress preserved
- Audit shows full deliberation timeline

**Validated CLI Invariants**
- Baselines are durable
- Legitimacy is never time-dependent
- Audit integrity across pauses

---

### Simulation CLI-12 — Baseline Close

**Context**  
Review process is complete.

**Flow**
- `baseline close`

**Outcome**
- Remaining `UNDER_REVIEW` → `ABANDONED`
- Baseline becomes immutable
- Normal sessions may resume

**Validated CLI Invariants**
- Explicit closure
- No silent drops
- Clear audit boundary

---

## VII. Export & Restore Safety

### Simulation CLI-13 — Import Restore (Hard Reset)

**Context**  
User restores a verified export.

**Flow**
- `charter import restore backup.json`

**CLI / Engine Behavior**
- Existing Area replaced entirely
- All sessions and baselines terminated
- Global audit records replacement

**Outcome**
- Restored history is authoritative
- No merging
- No reinterpretation

**Validated CLI Invariants**
- Restore is destructive and explicit
- Audit scope supremacy
- Deterministic replacement

---

### Simulation CLI-14 — Export with Active Sessions

**Context**  
User exports while sessions are active.

**Flow**
- Export requested

**Behavior**
- Closed sessions included
- Active / paused sessions ignored
- CLI emits explicit warning

**Outcome**
- No legitimacy fork
- Export safe to share

**Validated CLI Invariants**
- Sessions cannot be cloned
- Closed sessions are immutable history
- CLI warns but does not block

---

## VIII. Flat File Import Scenarios

### Simulation CLI-15 — First-Time Bootstrap from Flat File

**Context**  
Team adopts Charter with existing business rules.

**Flow**
1. Flat file import executed
2. Baseline review opens
3. All resolutions marked `UNDER_REVIEW`
4. User batch-accepts all
5. Sessions created behind the scenes
6. Baseline closed

**Outcome**
- Area initialized with legitimate resolutions
- No inferred authority
- Full audit trail exists

**Validated CLI Invariants**
- Explicit legitimacy
- Flat imports are non-authoritative
- Review-first workflow

---

### Simulation CLI-16 — Repeated Consultation with Unchanged Rules

**Context**  
External team sends identical rules again.

**Flow**
- Flat file imported
- CLI detects identical content
- Items labeled `UNCHANGED`
- User batch-accepts unchanged items
- Sessions recorded

**Outcome**
- Legitimacy reaffirmed explicitly
- No silent trust of foreign data
- Minimal cognitive load

**Validated CLI Invariants**
- Content matching is ergonomic only
- Explicit reaffirmation over inference

---

### Simulation CLI-17 — Mixed Changes and Stability

**Context**  
Some rules unchanged, some modified.

**Flow**
- Flat file imported
- CLI highlights unchanged vs modified
- User batch-accepts unchanged
- User deliberates modified items individually
- Baseline remains open across days
- Baseline closed explicitly

**Outcome**
- Only deliberate changes affect governance
- Stable rules remain stable
- Audit shows exactly what changed and when

**Validated CLI Invariants**
- Granular acceptance
- Review persistence
- Historical clarity

---

### Simulation CLI-18 — Misuse Attempt (Auto-Accept Expectation)

**Context**  
User expects unchanged imports to auto-activate.

**Flow**
- Flat file imported
- User skips review and checks status

**CLI Response**
- All imported resolutions remain `UNDER_REVIEW`
- CLI displays next actions:
  - `baseline show`
  - `baseline accept`
  - `baseline close`

**Outcome**
- No silent legitimacy
- User guided explicitly

**Validated CLI Invariants**
- CLI honesty
- No convenience over correctness

---

## Closing Observation

The Charter CLI does not optimize for speed, automation, or trust.

It optimizes for:
- **Explicit intent**
- **Cognitive safety**
- **Audit clarity**
- **Legitimacy preservation**

These simulations define what must **never break**, even as ergonomics evolve.