# How to Document These Rules Properly (Best Practice)

## 1. Separate “Spec” from “Code”

Create a dedicated folder:
```
/docs/spec/
  object-store.md
  hashing.md
  export-format.md
  invariants.md
```
Each spec:
- Is normative (“MUST”, “MUST NOT”, “FAIL if”)
- Is versioned
- Is referenced by code comments, not duplicated

## 2. Treat Specs as Immutable APIs

Rules:
- Specs change slower than code
- Any change requires:
    - A version bump
    - A migration strategy
    - New tests
- If a rule changes silently, you’ve broken auditability.

## 3. Encode Rules as Failing Tests

Every major rule should have:
- At least one negative test
- At least one round-trip test

Examples:
- Changing whitespace changes hash → FAIL
- Reordered JSON keys → same hash
- Envelope mismatch → fsck error

This turns the spec into executable truth.

## 4. Reference Specs in Code, Not Explanations

Bad:
```Rust
// Hashes object
```
Good:
```Rust
// Implements Charter Hash Spec v1 — docs/spec/hashing.md
```

This prevents tribal knowledge.

## 5. Never Explain “Why” in Code

- Code enforces
- Specs explain
- Tests verify

This keeps your engine boring — which is exactly what you want.

## Final Verdict

You are now in a very strong architectural position:
- Object identity is stable
- Hash evolution is possible
- Imports are deterministic
- Audits are defensible

CLI and services won’t paint you into a corner