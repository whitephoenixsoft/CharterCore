Areas as First-Class Governance Domains (Even Before Charter)
You’re thinking in exactly the right direction with Areas. Yes — treat them as documents first, then later as engine entities.
Recommended Initial Areas
Start conservative. You can always split later.
1. Core Invariants Area
Purpose: What must never break
Engine invariants
Determinism guarantees
Audit guarantees
Legitimacy rules
Characteristics:
Very slow moving
Extremely high bar
Changes are rare and explicit
GitHub mapping:
Label: area:invariants
Requires explicit approval (CODEOWNERS later)
Separate doc folder: doc/core/invariants/
2. Core Engine Semantics Area
Purpose: How the engine behaves
Sessions
Authority evaluation
Resolution lifecycle
Supersession rules
GitHub mapping:
Label: area:engine
Docs: doc/core/specs/
Design notes: doc/core/design/
3. CLI Area
Purpose: Human interaction constraints
CLI invariants
Context rules
Ergonomic constraints
Safety rails
GitHub mapping:
Label: area:cli
Docs: doc/cli/invariants/, doc/cli/specs/
4. Design Decisions Area (Non-Spec, Persistent)
Purpose: Decisions that are real but not specs
Examples:
“CLI will always prefer explicitness over convenience”
“No auto-accept flows, even in solo mode”
“Server mode compatibility must not be broken”
GitHub mapping:
Label: area:design
Docs: doc/design/decisions/
This is hugely important and often missing in projects.
5. Operations / Release / Meta Area
Purpose: How the project itself is run
Contribution rules
Release discipline
Versioning philosophy
Compatibility guarantees
GitHub mapping:
Label: area:ops
Docs: doc/project/

---

