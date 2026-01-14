# Charter Core — Spec Commands & Verification (ENGINE-LEVEL)
Status: PLANNING → STABLE
Audience: Engine, CLI, Integrations, Auditors

---

## Purpose

Charter Core embeds its own formal specifications inside the executable to:

- Anchor legitimacy rules to the binary itself
- Prevent silent reinterpretation of history
- Enable deterministic verification across versions, imports, and environments
- Provide machine-verifiable acceptance criteria

These specifications are **read-only**, **non-negotiable**, and **versioned**.

They exist for **trust over time**, not user convenience.

---

## Why Specs Are Embedded in the Executable

Embedding specifications in the binary is not for regular users and not for contributors.
It exists to answer one critical question in the future:

> “What rules did this engine instance commit to enforcing?”

GitHub answers what *should* happen.
The binary answers what *did* happen.

This distinction matters when:
- History is imported
- Data is consolidated
- Versions diverge
- Disputes arise years later
- Maintainers change

The executable becomes a self-describing legitimacy artifact.

---

## Core Spec Commands (Conceptual CLI / Engine Surface)

### charter spec list

Lists all embedded engine specifications.

Purpose:
- Transparency
- Debugging
- Audit
- Tooling introspection

Output includes:
- Spec ID (e.g. ENG-HASH-01)
- Title
- Status (LOCKED / FROZEN / DEPRECATED)
- Hash / checksum of spec text

---

### charter spec show <SPEC_ID>

Displays the full text of a specific embedded specification.

Purpose:
- Human audit
- Legal / governance review
- Historical inspection

This output is **authoritative for the running binary**.

---

### charter spec verify

Verifies that:
- All embedded specs are internally consistent
- All spec-linked code paths exist
- All required invariants are enforced
- No spec-required behavior is missing

Purpose:
- Engine self-validation
- CI / release gating
- Acceptance testing
- Safety checks before destructive operations

Failure means:
- Engine MUST refuse to proceed
- Or require explicit override (depending on context)

---

## Spec Verify as a Gatekeeper

`spec verify` is intended to act as a **hard gate** in the following situations:

- Engine startup (optional, configurable)
- Release builds (mandatory)
- Import operations
- Consolidation
- Cross-version interactions
- Migration or rehash procedures

It verifies **rule integrity**, not data integrity.

This is complementary to fsck, which verifies **data correctness**.

---

## Cross-Version Import Behavior

When importing data from a different Charter version:

1. Identify the source Charter version from the export
2. Load the corresponding embedded specs for that version
3. Run `spec verify` for the source version rules
4. Only then proceed with:
   - RESTORE
   - CONSOLIDATE
   - REVIEW

This prevents:
- Reinterpreting old data under new rules
- Silent legitimacy drift
- Accidental rule upgrades

Old rules are preserved as historical law.
New rules apply only after explicit migration.

---

## Spec Verify as an Acceptance Test

`spec verify` is a first-class acceptance test and release blocker.

A release MUST fail if:
- A required spec is missing
- A spec is violated
- A spec claims enforcement that does not exist
- A previously LOCKED spec is altered without version bump

This ensures:
- Specs do not become aspirational documentation
- Code and rules evolve together
- Invariants are mechanically enforced

---

## Forking, Bootlegging, and Binary Integrity

Charter does not prevent forks.
Forking is legitimate.

However, **misrepresentation** is not.

To protect users against bootleg or modified binaries:

### Binary Identity & Integrity (Optional, Strongly Recommended)

- Official builds MAY be cryptographically signed
- Signature metadata MAY be exposed via `charter spec verify`
- Verification status MAY include:
  - Signed / Unsigned
  - Signature issuer
  - Build provenance

Purpose:
- Allow users to verify they are running an authentic build
- Distinguish forks explicitly
- Prevent silent rule tampering disguised as official releases

This does NOT:
- Enforce trust
- Prevent forks
- Centralize authority

It only enables **identity verification**, not legitimacy enforcement.

---

## Design Philosophy

Specs are:
- Law, not guidance
- Immutable per version
- Enforced mechanically
- Portable across time

Specs are NOT:
- Configuration
- Feature flags
- UI documentation
- Subject to runtime modification

---

## Summary

Embedding and verifying specs ensures that:

- Legitimacy rules are explicit
- History cannot be silently reinterpreted
- Cross-version interactions are safe
- Disputes can be resolved with evidence
- The system can survive without its original author

Without embedded specs, Charter is a database.
With them, Charter is institutional memory with rules.