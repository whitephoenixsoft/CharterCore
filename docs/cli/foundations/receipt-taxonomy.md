# Receipt Taxonomy  
Status: FOUNDATIONAL (Proposed)  
Scope: CLI Interaction Layer (V3+)

---

# I. Purpose

This document defines the structural taxonomy for receipts within the Charter CLI (V3+).

It establishes:

- Receipt categories  
- Required receipt fields  
- Lifecycle triggers  
- Lineage rules  
- Audit guarantees  

The objective is to:

- Prevent semantic ambiguity  
- Prevent accidental legitimacy inference  
- Ensure full audit reconstructability  
- Make structural closure explicit and machine-verifiable  

Receipts formalize closure.  
They do not create authority unless explicitly defined as legitimacy receipts.

---

# II. Receipt Core Definition

A receipt is:

> An immutable, structured record emitted at the closure of a bounded lifecycle event.

Receipts:

- Are append-only  
- Have canonical engine IDs  
- Are audit-visible  
- Do not mutate legitimacy  
- May reference prior receipts (lineage)  
- Are commit-level artifacts  

Receipts represent structural finalization, not agreement.

---

# III. Receipt Categories

Receipts are strictly categorized.  
Category must be explicit and machine-readable.

There are three types:

- EXPLORATION  
- REVIEW  
- LEGITIMACY  

No additional categories are permitted without explicit version governance.

---

## A. Exploration Receipts

### Trigger

Emitted upon closure of:

- Deliberate  
- Breakout  
- Synthesis artifact finalization  

### Captures

- Receipt type: EXPLORATION  
- Engine ID  
- Lifecycle object ID (deliberate / breakout / synthesis)  
- Participants (if recorded)  
- Artifact IDs produced  
- Lineage links (prior artifacts or receipts)  
- Explicit end-state (e.g., SYNTHESIZED, ABANDONED)  
- Timestamp  

### Does NOT Capture

- Authority  
- Scope  
- Stances  
- Acceptance  
- Legitimacy evaluation  

Exploration receipts record thinking closure, not decisions.

---

## B. Review Receipts

### Trigger

Emitted upon:

- Baseline Review closure  

Applies to:

- Flat file import  
- V1/V2 import  
- Deliberate import  
- Consolidation  
- Foreign baseline merge  

### Captures

- Receipt type: REVIEW  
- Engine ID  
- Baseline ID  
- Source type (flat import, consolidation, deliberate import, etc.)  
- Proposal IDs reviewed  
- Accepted IDs  
- Rejected IDs  
- Abandoned IDs  
- Session IDs created (if any)  
- Lineage links (e.g., exploration receipts referenced)  
- Timestamp  

### Does NOT Capture

- Authority evaluation results  
- Stances  
- Legitimacy conclusions  

Review receipts summarize structural review outcomes.  
They do not create legitimacy.

---

## C. Legitimacy Receipts

### Trigger

Emitted upon:

- Session closure with acceptance  

A session that closes without acceptance does not emit a legitimacy receipt.

### Captures

- Receipt type: LEGITIMACY  
- Engine ID  
- Session ID  
- Authority ID  
- Scope ID  
- Participant set (immutable snapshot)  
- Candidate set (immutable snapshot)  
- Stances (ACCEPT / REJECT / ABSTAIN)  
- Topic  
- Annotations  
- Timestamp  
- Acceptance result  

This is the only receipt type that records legitimacy.

Legitimacy receipts represent canonical, auditable authority evaluation.

---

# IV. Receipt Invariants

The following are non-negotiable:

1. Every session acceptance MUST produce a LEGITIMACY receipt.  
2. Every baseline closure MUST produce a REVIEW receipt.  
3. Every deliberate closure MUST produce an EXPLORATION receipt.  
4. Every breakout closure MUST produce an EXPLORATION receipt.  
5. Receipts are immutable.  
6. Receipts must be reconstructible deterministically from audit logs.  
7. Receipt presence never implies correctness.  
8. Receipt presence never implies consensus beyond what is recorded.  
9. Receipt absence means structural closure did not occur.  

Violation of these invariants indicates CLI correctness failure.

---

# V. Receipt Lineage Model

Receipts form a traceable lineage chain.

Standard progression:

Exploration Receipt  
→ referenced by Review Receipt  
→ referenced by Legitimacy Receipt  

Lineage requirements:

- References must be machine-traceable  
- References must use canonical engine IDs  
- Lineage must be directional and immutable  
- No implicit inference of lineage is allowed  

Receipts may reference multiple prior receipts.

Lineage must allow reconstruction of:

- Idea origin  
- Review path  
- Legitimacy path  

Without interpreting content meaning.

---

# VI. Audit Requirements

Audit output must:

- Render receipts one per line  
- Include receipt type  
- Include canonical engine IDs  
- Preserve stable ordering  
- Allow grep by receipt type  
- Preserve lineage references  
- Never interpret intent  

Receipt events must be distinguishable from:

- SESSION  
- BASELINE  
- DELIBERATE  
- BREAKOUT  

Audit is the system of record.

If receipt lineage cannot be reconstructed from audit,
the system has failed determinism.

---

# Final Constraint

Receipts formalize structural closure.

Only LEGITIMACY receipts record legitimacy.

Exploration and Review receipts preserve process integrity,
but never imply authority.

This taxonomy is foundational for V3+.