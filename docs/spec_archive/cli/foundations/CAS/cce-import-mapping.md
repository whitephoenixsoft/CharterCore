# Charter — CCE v1 Minimal + V6 Commit Mapping

Status: DRAFT (Architectural Mapping)  
Applies to: CCE v1, V6 Commit Model, V7 Relay  
Does NOT define: commit storage implementation, hashing mechanics, relay transport protocol, or legitimacy semantics beyond existing frozen rules  

---

# 1. Purpose

This document defines the architectural relationship between:

- CCE v1 (Charter Canonical Export)
- V6 Commit Model
- V7 Relay storage and transport

Its purpose is to make clear:

- what CCE is
- what a V6 commit is
- how CCE content maps into commits
- where the two models differ
- which representation is canonical for which purpose

This document prevents accidental collapse of:

- ledger snapshot into append-only stream
- workflow artifacts into legitimacy artifacts
- transport into authority

---

# 2. Core Distinction

## 2.1 CCE

CCE is a:

- canonical ledger snapshot
- restore-oriented export format
- deterministic structural serialization of legitimate history

CCE answers:

“What is the portable canonical state of this ledger at export time?”

---

## 2.2 V6 Commits

V6 commits are:

- append-only truth artifacts
- individually identified
- independently storable
- transportable without interpretation

V6 answers:

“What immutable artifacts have been recorded over time?”

---

## 2.3 Non-Equivalence

CCE and V6 commits are related, but not identical.

CCE is not:

- a relay feed
- a live commit stream
- a workflow queue

V6 commits are not:

- a canonical state snapshot
- a restore ledger by themselves
- legitimacy proof unless the commit type carries such proof

---

# 3. Minimal CCE v1 Scope

CCE v1 should remain minimal.

CCE v1 contains only what is required to preserve and restore canonical legitimacy history for Areas.

CCE v1 includes:

- Area identity
- Resolution objects
- Session objects
- Receipt objects
- active authority/scope pointers or equivalent reconstructable state
- structural references required for legitimacy reconstruction
- export provenance fields (format version, charter version, spec set hash, hash version)

CCE v1 excludes:

- baseline review workspaces
- deliberate workspaces
- live UNDER_REVIEW workflow state
- signal/check-in streams
- alignment state
- relay-only metadata
- ephemeral indexes

---

# 4. V6 Commit Model Scope

V6 commit model is broader than CCE.

V6 may include commit types such as:

- Resolution commits
- Session commits
- Legitimacy Receipt commits
- Review Receipt commits
- Exploration Receipt commits
- Deliberate commits
- Baseline Review commits
- Import commits
- Annotation commits
- Signal / check-in commits

Only a subset of V6 commits participate in CCE v1.

---

# 5. Mapping Principle

## 5.1 Canonical Ledger Subset

CCE v1 is derived from the subset of commits that preserve canonical legitimacy history.

That subset is:

- Area-defining state or equivalent area metadata
- Session commits or session-equivalent artifacts
- Resolution commits
- Legitimacy Receipt commits
- other receipt types only if included for structural closure traceability in export

CCE v1 must not require the full V6 commit universe.

---

## 5.2 Workflow and Observability Exclusion

The following V6 commit categories do not belong to minimal CCE v1:

- Deliberate commits
- Baseline Review commits
- Review workflow states
- Exploration-only artifacts
- Signal/check-in commits
- Alignment-derived artifacts

These remain valid V6 commits, but are not part of minimal canonical ledger export.

---

# 6. Canonical Mapping Table

| CCE v1 Object | V6 Commit Relationship | Notes |
|---|---|---|
| Area | May be represented by area metadata commit(s) or export-level area object | Area identity must survive export/restore |
| Session | Session commit or session-equivalent canonical artifact | Only closed sessions belong in CCE |
| Resolution | Resolution commit | Smallest legitimacy unit remains unchanged |
| Legitimacy Receipt | Legitimacy Receipt commit | Canonical proof of legitimacy closure |
| Review Receipt | Optional in minimal CCE; always valid in V6 | Include only if export requires workflow closure traceability |
| Exploration Receipt | Optional in minimal CCE; always valid in V6 | Same rule as above |
| Annotation | Annotation commit or embedded annotation field depending on object model | Never affects legitimacy |
| Signal / Check-in | V6 commit only | Excluded from minimal CCE v1 |

---

# 7. Receipts in the Mapping

## 7.1 Legitimacy Receipts

Legitimacy Receipts are first-class in both models.

They are:

- V6 commits
- part of canonical legitimacy proof
- part of CCE v1

---

## 7.2 Review and Exploration Receipts

Review and Exploration Receipts are always valid V6 commits.

Their inclusion in CCE v1 is optional and policy-driven.

Minimal CCE v1 may omit them if the goal is strictly:

- restore of canonical legitimacy history

Extended CCE variants may include them for:

- workflow traceability
- exegesis
- historical narrative continuity

---

# 8. Snapshot vs Stream

## 8.1 CCE as Snapshot

CCE is a snapshot representation.

It presents:

- a deterministic export-time view
- canonical objects grouped by Area
- reconstructable legitimacy history without replaying a full commit stream

---

## 8.2 V6 as Stream / Archive

V6 commit storage is an archive of independent artifacts.

It presents:

- individual immutable entries
- append-only growth
- transport suitability
- compatibility with relay

No single commit implies canonical state by itself.

---

# 9. Restore Semantics

Minimal CCE v1 is the preferred format for restore.

Restore from CCE means:

- reconstruct canonical ledger state
- preserve Area identity
- preserve legitimacy history
- preserve provenance
- avoid replay dependence on non-canonical workflow artifacts

Restore from raw V6 commit collections is a separate, broader capability and must not be assumed equivalent to CCE restore.

---

# 10. Relay Relationship

Relay stores V6 commits, not CCE snapshots by default.

A relay may transport:

- commits that later contribute to CCE export
- full export artifacts as opaque payloads if explicitly supported

But relay semantics remain commit-oriented, not snapshot-oriented.

CCE should be understood as an export artifact.
Relay should be understood as a commit archive / transport layer.

---

# 11. Anti-Collapse Invariants

The following must remain true:

- CCE is not the commit store
- commit store is not canonical state
- relay is not restore by itself
- workflow commits do not become legitimacy artifacts by inclusion
- signals do not become legitimacy by export
- annotations do not affect mapping semantics

---

# 12. Recommended Minimal Rule

For initial implementation, CCE v1 should map only:

- Areas
- closed Sessions
- Resolutions
- Legitimacy Receipts
- required provenance metadata

All other V6 commit types should remain outside minimal CCE v1 unless explicitly added in a future extended export profile.

---

# 13. Future Extension Direction

Future versions may define:

- CCE Extended profiles
- workflow-inclusive export bundles
- signal-inclusive observability bundles
- self-contained verification bundles
- commit-stream-to-snapshot reconstruction rules

Such extensions must preserve the distinction between:

- canonical legitimacy snapshot
- append-only artifact archive

---

# 14. Final Principle

CCE v1 preserves canonical legitimacy history.

V6 commits preserve the broader truth artifact universe.

CCE is the restore-oriented canonical subset of that universe.

They must remain related, but never collapsed into one another.