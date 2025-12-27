# CLI Invariants — Naming & Identity

These are CLI-layer invariants, not engine invariants.

Potential locations:
/docs/cli/INVARIANTS.md
or
/docs/patterns/cli-invariants.md

CLI-INV-01: Engine Identity Is Canonical
Every Area, Session, Resolution, and Scope has a canonical engine ID.
Engine IDs are:
content-addressed hashes when possible
UUIDs otherwise
CLI-visible labels never replace engine IDs.
Fail if
Any CLI operation relies on a label as the authoritative identity.
CLI-INV-02: Labels Are Area-Scoped by Default
User-assigned labels (e.g. R-1, S-Auth, Scope-Core) must be unique within an Area.
The CLI must never assume global uniqueness of labels.
Fail if
A label collision across Areas causes ambiguity or hidden coupling.
CLI-INV-03: Area Context Is Required for Unqualified Labels
If a command references a label without an Area qualifier:
The CLI must have an active Area context, or
The command must fail with a prompt to disambiguate.
Pass examples
Copy code

charter area use platform
charter resolution show R-2
Fail example
Copy code

charter resolution show R-2
error: label ambiguous without area context

CLI-INV-04: CLI Renders Global Disambiguation Explicitly
When listing or searching across Areas, the CLI must render labels as:
Copy code

<area>/<label>
or equivalent.
Example
Copy code

platform/R-3
finance/R-1
security/R-7
Fail if
The CLI prints bare labels in a global view.
CLI-INV-05: Global Labels Are Optional and Explicit
The CLI may support an optional configuration for global label uniqueness.
This must be:
opt-in
reversible
clearly documented as increasing merge/import friction
Fail if
Global uniqueness is silently enforced.

CLI-AUD-01: Audits Are Read-Only
No audit command may mutate state.
CLI-AUD-02: Audits Do Not Infer Meaning
No semantic judgments
No “this was wrong”
No “this violated scope”
CLI-AUD-03: Audits Are Deterministic
Same input → same output