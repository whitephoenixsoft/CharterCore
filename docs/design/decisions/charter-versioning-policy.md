# Charter Versioning Policy
Status: DESIGN DECISION
Applies to: Charter Core, CLI, Import/Export

## Purpose

This document defines how Charter versions its components to:
- preserve historical meaning
- prevent silent reinterpretation
- enable safe evolution
- reduce accidental coupling

Not all versions are equal.
Not all versions move together.

## Version Domains

Charter defines five version domains:

### 1. Engine Version (Primary)

The engine version defines:
- legitimacy semantics
- embedded specifications
- authority evaluation rules

This is the root of meaning.

Changing engine version may change:
- what is considered legitimate
- how authority is evaluated
- how history is interpreted

Engine versions must be explicit and auditable.

### 2. Embedded Spec Set (Derived)

Each engine version embeds a fixed set of specs.

Specs are:
- immutable per engine version
- identified by content hash
- verifiable mechanically

Spec sets are not independently versioned.
They are content-addressed and derived from the engine.

### 3. Export File Version (Structural)

The export version defines:
- file format
- encoding
- schema

It does NOT define legitimacy semantics.

Exports must declare:
- originating engine version
- originating spec hash

### 4. Hash Algorithm Version (Technical)

Hash versions define:
- ID generation
- integrity checks

Changing hash algorithms must not:
- alter legitimacy semantics
- reinterpret history

Hash changes affect representation only.

### 5. CLI Version (Ergonomic)

CLI version governs:
- command structure
- defaults
- user interaction

CLI versions must never redefine legitimacy.

## Compatibility Rules

- New engine versions must not reinterpret accepted history
- Old data must be evaluated under the rules it was created with
- Migration of meaning requires explicit new decisions
- RESTORE replaces state; it does not upgrade meaning

## Design Principle

History is law.
New law applies forward only.

Charter prefers:
- explicit breaks
- visible warnings
- mechanical verification

Over:
- silent upgrades
- inferred compatibility
- convenience-based reinterpretation