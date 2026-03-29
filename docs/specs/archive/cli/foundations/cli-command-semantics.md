# Charter CLI — Command Semantics

Status: FOUNDATIONAL  
Applies to: Charter CLI Runtime and Future Interfaces  
Does NOT define: engine legitimacy rules, storage implementation, or transport protocols  

---

# 1. Purpose

This document defines the semantic categories of Charter CLI commands.

Its purpose is to ensure that command design remains consistent with Charter’s architectural boundaries.

It exists to prevent:

- workflow commands from becoming legitimacy shortcuts
- read commands from mutating state
- CLI ergonomics from redefining governance
- future command growth from collapsing layer boundaries

This document is an architectural anchor, not a complete command list.

---

# 2. Core Principle

Charter CLI commands fall into distinct categories.

A command may do one of the following:

- read state
- mutate workflow state
- trigger legitimacy evaluation through the engine
- manage isolation and storage
- perform verification
- move artifacts across boundaries

These categories must not be silently mixed.

---

# 3. Command Categories

## 3.1 Read Commands

Read commands inspect state without mutating it.

Examples:

- status
- show
- list
- audit timeline
- spec list
- spec show

Read commands must:

- be side-effect free
- never create legitimacy
- never mutate workflow state
- never alter storage

---

## 3.2 Workflow Mutation Commands

Workflow mutation commands modify pre-legitimacy or non-legitimacy state.

Examples:

- baseline accept
- baseline reject
- baseline close
- deliberate start
- deliberate abandon
- breakout complete
- session pause
- session resume
- participant add/remove before finalization

Workflow mutation commands may:

- change review state
- change exploration state
- change annotations
- prepare material for legitimacy

Workflow mutation commands must not:

- create legitimacy directly
- evaluate authority as legitimacy
- emit resolutions directly
- bypass sessions

---

## 3.3 Legitimacy Trigger Commands

Legitimacy trigger commands invoke the Legitimacy Engine.

Examples:

- session start
- session close
- session vote
- baseline close when configured to batch accepted proposals into sessions

These commands may:

- create sessions
- submit candidates
- record stances
- trigger acceptance evaluation
- emit legitimacy receipts
- create resolutions

These commands must always preserve the invariant:

Only the Legitimacy Engine creates legitimacy.

---

## 3.4 Storage and Isolation Commands

These commands manage Contexts, Areas, exports, restores, and local runtime boundaries.

Examples:

- context create
- context use
- area create
- export
- import restore

These commands may:

- select isolated storage universes
- initialize new Areas
- restore canonical ledger state
- write or load persistent data

They must not:

- infer authority
- reinterpret history
- silently merge legitimacy

---

## 3.5 Verification Commands

Verification commands inspect compatibility, integrity, or rule identity.

Examples:

- spec verify
- integrity checks
- import compatibility checks

Verification commands must:

- be deterministic
- be side-effect free
- never repair automatically
- never reinterpret unknown rule sets

They may block destructive or legitimacy-affecting operations.

---

## 3.6 Transport Commands

Transport commands move artifacts across explicit boundaries.

Examples:

- relay push
- relay fetch
- export bundle transfer
- deliberate export/import

Transport commands may:

- copy artifacts
- preserve provenance
- preserve timestamps and identities where applicable

Transport commands must not:

- create legitimacy
- infer compatibility
- merge histories implicitly
- resolve authority

---

# 4. Mixed Commands

Some commands may appear mixed at the UX layer, but their internal behavior must remain layered.

Example:

baseline close may:

1. finalize workflow review state
2. emit a review receipt
3. create one session per accepted proposal
4. trigger engine-side legitimacy evaluation for those sessions

This is allowed only if each step preserves its layer boundary.

The CLI may orchestrate multiple phases.
It must not collapse them into one semantic act.

---

# 5. Baseline Review Command Semantics

Baseline review commands are workflow commands.

## 5.1 Marking Commands

Commands such as:

- baseline accept
- baseline reject

are marking operations only.

They:

- mark proposal disposition within the review workspace
- do not create legitimacy
- do not emit resolutions directly
- do not bypass future session creation

## 5.2 Closure Command

baseline close is the structural closure point for the review workspace.

At closure, the CLI may:

- finalize accepted / rejected / abandoned sets
- emit a Review Receipt
- batch accepted proposals into one session each
- invoke engine legitimacy evaluation using the required mirrored review rule

This preserves the rule:

Baseline review prepares and batches.  
The Legitimacy Engine decides.

---

# 6. Session Command Semantics

Sessions are the only legitimacy boundary.

Session commands must preserve the following:

- participants are explicit
- votes / stances are explicit
- authority is mechanical
- candidate set freezes when voting begins
- any substantive candidate change after voting requires pause/resume or restart flow
- legitimacy is proven by resulting legitimacy receipts

Session commands must never infer:

- attendance
- silence as consent
- authority from labels or metadata

---

# 7. Deliberate and Breakout Command Semantics

Deliberate and breakout commands are exploration commands.

They may:

- create options
- revise drafts
- synthesize artifacts
- produce exploration receipts
- hand material to baseline review

They must not:

- create legitimacy
- evaluate authority
- imply consensus
- bypass review or sessions

Deliberate remains