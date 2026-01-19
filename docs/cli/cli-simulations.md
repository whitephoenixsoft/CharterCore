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

**Does the CLI ever allow a human to bypass, dilute, or accidentally fabricate legitimacy?**

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
4. CLI blocks all sessions except Authority and Scope definition

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
- Governance is explicit
- No implicit defaults
- Authority and Scope are first-class
- Participants are never inferred

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
- Explicit unanimity
- No silent acceptance
- Mechanical authority evaluation

---

### Simulation CLI-3 — Single-User Governance (Solo Journal)

**Context**  
A solo founder uses Charter as a decision journal.

**Flow**
1. Session started
2. Candidate proposed
3. `session accept` used without explicit voting

**Engine Reality (via audit)**
- Participant list: `[founder]`
- Stance recorded: `ACCEPT`
- Authority evaluated normally

**Outcome**
- Full audit trail preserved
- No engine shortcuts

**Validated CLI Invariants**
- Scale independence
- Ergonomic collapse preserves legitimacy

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
- Majority computed mechanically
- No semantic inference

---

## III. Blocking, Disagreement, and Reality

### Simulation CLI-5 — Deadlock Without Coercion

**Context**  
Unanimous authority with disagreement.

**Flow**
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
- No implicit failure

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
- Bob removed explicitly from participants

**Outcome**
- Authority re-evaluated
- Remaining participants satisfy unanimity
- Resolution accepted legitimately

**Validated CLI Invariants**
- Participant changes are explicit
- No vote reinterpretation
- Authority evaluates present participants only

---

## IV. Authority Evolution

### Simulation CLI-7 — Authority Change Requires Its Own Session

**Context**  
Team wants faster decisions.

**Flow**
1. New session started under current Authority
2. New Authority candidate proposed
3. Accepted under old Authority

**Outcome**
- Old Authority superseded
- New Authority becomes active
- No retroactive effect

**Validated CLI Invariants**
- Authority mutable only via sessions
- Non-retroactivity
- Explicit governance evolution

---

## V. CLI Cognitive Constraints

### Simulation CLI-8 — Concurrent Sessions (Solo Mode Constraint)

**Context**  
Solo mode limits cognitive load.

**Flow**
- Session A is active
- User attempts to start Session B

**Outcome**
- CLI blocks the action
- Engine state unchanged

**Validated CLI Invariants**
- One active session in solo mode
- Ergonomic constraint without semantic impact

---

## VI. Baseline (Import / Consolidation)

### Simulation CLI-9 — Baseline Initialization

**Context**  
External history imported for consolidation.

**Flow**
1. `charter import consolidate legacy.json`
2. Baseline created
3. Imported resolutions marked `UNDER_REVIEW`
4. Active sessions paused

**Outcome**
- No local legitimacy created
- No authority inferred
- One baseline active

**Validated CLI Invariants**
- Import never creates legitimacy
- Baseline is review-only
- Session separation enforced

---

### Simulation CLI-10 — Baseline Acceptance (Solo)

**Context**  
Solo user reviews imported history.

**Flow**
- Baseline opened
- User accepts resolutions one at a time
- Each acceptance creates a hidden session

**Outcome**
- Accepted resolutions become `ACTIVE`
- Full audit trail exists
- Baseline remains open

**Validated CLI Invariants**
- Acceptance always occurs via sessions
- Ergonomic batching preserves legitimacy

---

### Simulation CLI-11 — Baseline Acceptance (Multi-User, Long-Running)

**Context**  
Departments review imported decisions over time.

**Flow**
- Baseline opened
- Deliberation occurs over days
- Baseline paused and resumed
- Sessions created as needed

**Outcome**
- Partial progress preserved
- Audit timeline intact

**Validated CLI Invariants**
- Baselines are durable
- Legitimacy is not time-dependent

---

### Simulation CLI-12 — Baseline Close

**Context**  
Review process completes.

**Flow**
- `baseline close`

**Outcome**
- Remaining `UNDER_REVIEW` → `ABANDONED`
- Baseline becomes immutable
- Sessions may resume

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

**Outcome**
- Area replaced entirely
- All sessions and baselines terminated
- Global audit emitted

**Validated CLI Invariants**
- Restore is destructive and explicit
- No partial mutation
- Deterministic replacement

---

### Simulation CLI-14 — Export with Active Sessions

**Context**  
User exports while sessions are active.

**Flow**
- Export requested

**Outcome**
- Closed sessions exported
- Active/paused sessions ignored
- CLI emits explicit warning

**Validated CLI Invariants**
- Sessions cannot be cloned
- No implicit closure

---

## VIII. Flat File Import Scenarios

### Simulation CLI-15 — First-Time Bootstrap from Flat File

**Context**  
Team adopts Charter with existing rules.

**Flow**
1. Flat file import
2. Baseline opened
3. All resolutions `UNDER_REVIEW`
4. User batch-accepts all
5. Sessions created
6. Baseline closed

**Outcome**
- Area initialized legitimately
- No inferred authority
- Full audit preserved

---

### Simulation CLI-16 — Repeated Consultation (Unchanged Rules)

**Context**  
External team sends identical rules again.

**Flow**
- Flat file imported
- CLI detects identical content
- Items marked `UNCHANGED`
- User batch-accepts

**Outcome**
- Explicit reaffirmation
- No silent trust

---

### Simulation CLI-17 — Mixed Changes and Stability

**Context**  
Some rules unchanged, some modified.

**Flow**
- Import executed
- CLI highlights differences
- User batch-accepts unchanged
- Modified items reviewed individually
- Baseline closed

**Outcome**
- Only deliberate changes affect governance
- Audit shows exact evolution

---

### Simulation CLI-18 — Misuse Attempt (Auto-Accept Expectation)

**Context**  
User expects unchanged imports to auto-activate.

**Flow**
- Import executed
- User skips review

**CLI Response**
- All items remain `UNDER_REVIEW`
- CLI displays valid next actions

**Outcome**
- No silent legitimacy
- User guided explicitly

---

## IX. Participant Defaults & Safety (NEW)

### Simulation CLI-19 — Default Participant Is Explicit, Not Inferred

**Context**  
User starts a session without specifying participants.

**Flow**
1. `session start`
2. CLI inserts current user as participant
3. CLI displays participant list

**Outcome**
- Participant set is explicit
- User may remove themselves before first stance

**Validated CLI Invariants**
- Participant state is explicit
- Defaults do not create legitimacy

---

### Simulation CLI-20 — Empty Participant Set Blocks Acceptance

**Context**  
All participants removed.

**Flow**
- User attempts to vote or accept

**Outcome**
- CLI blocks action
- No engine mutation

**Validated CLI Invariants**
- Authority requires participants
- CLI never fabricates legitimacy

---

## X. Labels & Identity (NEW)

### Simulation CLI-21 — Auto-Generated Labels Are Non-Semantic

**Context**  
CLI generates labels automatically.

**Flow**
- Labels assigned
- Labels renamed
- Cross-area references used

**Outcome**
- Engine behavior unchanged
- Engine IDs remain canonical

**Validated CLI Invariants**
- Labels are ergonomic only
- Identity is engine-owned

---

## XI. Spec Verification (NEW)

### Simulation CLI-22 — Spec Verification Is Read-Only

**Context**  
User verifies specs and invariants.

**Flow**
- `charter spec verify`

**Outcome**
- Pass/fail reported
- No state mutation
- No audits emitted

**Validated CLI Invariants**
- Verification is non-legitimizing
- Analysis does not equal decision

---
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

**Does the CLI ever allow a human to bypass, dilute, or accidentally fabricate legitimacy?**

If any simulation fails, a **CLI invariant has been violated**.

---

## XII. Extreme User Perspective Simulations (Scientists & Monks)

**Purpose**  
These simulations do not validate workflow or UX. They validate whether the CLI produces outputs that survive **scrutiny by the strictest possible users**: scientists (reproducible, auditable) and monks (canonically preserved, historically defensible).  

**Context**  
CLI artifacts (sessions, baselines, breakouts, syntheses, proposals) must be **legible, auditable, and defensible** regardless of whether such users ever interact with the system.

**Rationale**  
- These simulations **codify extreme scrutiny** as a contributor-facing guideline.  
- They help future developers understand why the CLI behaves conservatively, why authority is never inferred, and why audit and legitimacy separation is critical
---

### Simulation CLI-23 — Scientist Perspective

**Scenario**  
A researcher reviews decisions to ensure reproducibility.

**Flow**
1. Inspect all accepted resolutions in an Area.
2. Trace resolution provenance through:
   - Baseline reviews
   - Sessions
   - Synthesis artifacts
   - Breakouts
3. Confirm all authority, participant sets, and stances are recorded explicitly.

**Outcome**
- Any resolution can be reconstructed step-by-step.
- No inference, no assumption, no retroactive legitimacy.
- CLI audit provides full timeline.

**Validated CLI Invariants**
- ENG-INV-03, ENG-INV-06, CLI-AUD-01–07  
- Legitimacy is explicit and reconstructible.

---

### Simulation CLI-24 — Monastic / Canonical Perspective

**Scenario**  
A monastic archivist ensures decisions could survive centuries.

**Flow**
1. Examine all historical artifacts.
2. Confirm:
   - Supersession and retirement are explicit
   - No silent deletions
   - Baseline reviews and syntheses are fully auditable
3. Check whether any authority, acceptance, or candidate state was inferred or skipped.

**Outcome**
- Historical continuity preserved.
- Any future reader can see decision lineage and context.
- CLI enforces audit permanence.

**Validated CLI Invariants**
- ENG-INV-01, ENG-INV-08, CLI-STOR-02, CLI-BL-07  
- Immutable history is maintained.

---

### Simulation CLI-25 — Developer-as-Converted-Scientist Perspective

**Scenario**  
A software developer evaluates whether decisions can be explained and defended.

**Flow**
1. Examine all session votes and stances.
2. Trace proposals to synthesis outputs.
3. Ensure any external or imported material was reviewed explicitly.
4. Check that no “shortcuts” or inferred consensus affected legitimacy.

**Outcome**
- Every decision has clear reasoning.
- Every resolution could be audited by a third party.
- Dissent and abstention are preserved.

**Validated CLI Invariants**
- CLI-SES-01–05, CLI-AUTH-03, CLI-DEL-01–05, CLI-BL-01–08

---

## Closing Observation

The Charter CLI does not optimize for speed, automation, or trust.

It optimizes for:
- explicit intent
- cognitive safety
- audit clarity
- legitimacy preservation

These simulations define what must **never break**, even as ergonomics evolve.
