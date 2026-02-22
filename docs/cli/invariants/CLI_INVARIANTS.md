# Charter CLI — Invariants  
Status: FROZEN  
Applies to: Charter CLI (all versions)  
Does NOT apply to: Charter Core engine semantics  

Violations indicate a CLI correctness failure, not an engine bug.

---

## Purpose

These invariants define the behavioral contract of the Charter CLI.

They ensure that human-facing ergonomics never compromise:

- legitimacy  
- determinism  
- auditability  
- structural finality  

If an implementation violates an invariant,  
the implementation is wrong — not the invariant.

---

# I. Identity & Naming

## CLI-ID-01 — Engine Identity Is Canonical

Every Area, Session, Resolution, Candidate, Scope, Authority, Participant,
Deliberate artifact, and Receipt has a canonical engine ID.

Rules:

- Engine IDs are content-addressed hashes when possible, UUIDs otherwise  
- CLI-visible labels never replace engine IDs  
- All CLI operations ultimately resolve to engine IDs  

Fail if:

- any CLI command treats a label as authoritative identity  

---

## CLI-ID-02 — Labels Are Ergonomic Only

Labels exist solely to aid human reference.

Rules:

- labels never encode meaning  
- labels do not affect authority, scope, legitimacy, or receipt behavior  
- labels do not influence engine evaluation  
- labels are Area-scoped by default  

Fail if:

- CLI logic interprets label structure or naming conventions  

---

## CLI-ID-03 — Area Context Is Required for Unqualified Labels

If a label is used without Area qualification:

- an active Area context must exist, or  
- the command must fail with a disambiguation error  

Fail if:

- CLI guesses the Area implicitly  

---

## CLI-ID-04 — Global Disambiguation Is Always Explicit

When listing across Areas:

- labels must render as `area/label`  

Fail if:

- bare labels appear in global output  

---

# II. Context & Mode Safety

## CLI-CTX-01 — Context Is Always Explicit

The CLI must never guess:

- Area  
- Session  
- Baseline  
- Deliberate  

Context must be explicitly selected or explicitly qualified.

Fail if:

- any command executes under inferred context  

---

## CLI-CTX-02 — Context Switching Is Always Visible

Changing:

- Area  
- active session  
- active baseline  
- active deliberate  

requires an explicit command.

Fail if:

- context changes silently  

---

## CLI-CTX-03 — One Active Context per Invocation

A CLI invocation operates against exactly one Charter context.

Fail if:

- commands act on multiple contexts implicitly  
- context ambiguity exists without explicit user intent  

---

## CLI-CTX-04 — Context Isolation Across CLI Invocations

CLI context is **local and isolated** per invocation.

Fail if:

- multiple CLI processes share context implicitly  
- session or baseline state leaks between CLI instances  

---

## CLI-CTX-05 — Context Switching Never Moves Data

Context switching must not:

- move Charter data  
- copy Charter data  
- reinitialize storage  

Fail if:

- history mutates due to context switching  

---

## CLI-CTX-06 — Concurrency Isolation

CLI must detect concurrent operations affecting the same Area or session across multiple processes.

Rules:
- overlapping CLI invocations must not silently interfere
- conflicting operations must be paused, blocked, or rejected explicitly

Fail if:
- session, baseline, or candidate state is mutated concurrently without explicit user coordination

---

# III. Governance Preconditions

## CLI-AREA-01 — Governance Precondition

No action that can create, affect, or validate legitimacy may occur unless the Area has:

- exactly one active Authority resolution  
- exactly one active Scope resolution  

Enforced at:

- session acceptance  
- baseline acceptance → session creation  
- import consolidation  
- deliberate → baseline handoff  

If violated:

- CLI/library must reject the action  
- emit an explicit error indicating missing governance  

This applies Area-wide.

---

# IV. Session Handling (CLI Layer)

## CLI-SES-01 — Single Active Session (Solo Mode)

In solo mode:

- only one active session may exist  
- new sessions require pause or close  

Fail if:

- multiple active sessions exist  

---

## CLI-SES-02 — Candidate Editing Is Pre-Stance Only

Candidates may be freely edited until the first stance is recorded.

After first stance:

- no add  
- no remove  
- no edit  

Fail if:

- candidate mutation occurs after stances begin  

---

## CLI-SES-03 — Restart-From Is Terminal

`restart-from`:

- closes the source session  
- creates a new session with no stances  
- records lineage for audit only  

Fail if:

- votes or acceptance carry forward  

---

## CLI-SES-04 — Participant State Is Explicit

Participants are explicit session state.

Rules:

- CLI never infers participants  
- participant changes require explicit commands  
- acceptance snapshots participants immutably  

Fail if:

- participants are implied or inferred  

---

## CLI-SES-05 — Resume Requires Governance & Participant Revalidation

On resume:

- display prior participants  
- display current participants  
- require explicit confirmation  
- validate Authority and Scope against session context

Until confirmed:

- no stances  
- no acceptance  
- no candidate changes  

Fail if:

- resume proceeds silently  
- legitimacy is created under outdated governance  

---
## CLI-SES-06 — Abandonment Lineage

All abandoned candidates, sessions, or baselines must remain auditable and preserve explicit parent references.

Fail if:
- lineage of abandoned or aborted work is lost
- historical connections to superseded or abandoned objects are broken

---

# V. Authority & Constraints (CLI Layer)

## CLI-AUTH-01 — Constraints Are Authority-Equivalent

Any rule that changes:

- who must agree  
- how agreement is evaluated  

is authority-equivalent.

Rules:

- cannot change mid-session  
- cannot change on resume  
- requires its own decision session  

Fail if:

- constraints mutate without authority governance  

---

## CLI-AUTH-02 — Constraints Must Be Declared at Session Start

All constraints must be:

- declared before any stance  
- visible in session metadata  
- immutable for the session lifetime  

Fail if:

- constraints are added after stances begin  

---

## CLI-AUTH-03 — CLI Never Creates Consensus

The CLI records outcomes; it never creates agreement.

Fail if:

- running a command implies consensus  
- silence is treated as approval  

---

## CLI-AUTH-04 — Ergonomic Collapse Preserves History

The CLI may collapse steps for ergonomics,
but votes, acceptance, authority context, and receipt emission
must still be recorded.

Fail if:

- mechanical history is skipped  

---

## CLI-AUTH-05 — CLI Is Honest About Its Limits

The CLI must not claim to know:

- who attended  
- who agreed  
- what was discussed  

Annotations may exist.  
Inferences may not.

---

# VI. Receipt Guarantees (FROZEN)

Receipts formalize structural closure.  
They are first-class, immutable commit objects.

---

## CLI-RCP-01 — Structural Closure Emits a Receipt

The following events MUST emit exactly one receipt:

- Deliberate closure → Exploration Receipt  
- Breakout closure → Exploration Receipt  
- Baseline closure → Review Receipt  
- Session acceptance / closure → Legitimacy Receipt  

Fail if:

- a bounded lifecycle ends without a receipt  

---

## CLI-RCP-02 — Receipts Are Categorized

Every receipt MUST declare its type:

- EXPLORATION  
- REVIEW  
- LEGITIMACY  

Fail if:

- receipt type is implicit or absent  

---

## CLI-RCP-03 — Receipts Are Immutable

Receipts:

- are append-only  
- cannot be edited  
- cannot be deleted  
- cannot be superseded  

Fail if:

- receipt content mutates  

---

## CLI-RCP-04 — Receipts Do Not Imply Legitimacy

Receipt presence:

- does not imply agreement  
- does not imply correctness  
- does not imply authority  

Only LEGITIMACY receipts record legitimacy.

Fail if:

- receipt existence is treated as endorsement  

---

## CLI-RCP-05 — Receipts Preserve Lineage & Audit Ownership

Receipts:

- must include explicit parent references  
- must survive export, restore, relay, and federation  
- must capture lineage across CLI operations and sessions  
- must be auditable independently of engine storage

Fail if:

- lineage cannot be traced end-to-end  
- audit is missing for historical events  

---
## CLI-RCP-06 — Failed or Rejected Actions Are Audited

Any rejected, blocked, or failed operation that affects Authority, Scope, session, or candidate state must:
- emit a receipt or audit event
- capture the attempted action and reason for failure

Fail if:
- structural attempts that fail leave no trace in audit or receipts

---

# VII. Baseline Review (Import / Consolidation)

## CLI-BL-01 — Single Active Baseline

At most one baseline may be active per Area.

Fail if:

- multiple baselines exist concurrently  

---

## CLI-BL-02 — Baseline Requires Session Pause

If a session is active:

- baseline start or import must pause it  

Fail if:

- sessions and baseline interleave  

---

## CLI-BL-03 — Baseline Is a Mutable Review Workspace

Until baseline close:

- accept / reject decisions are reversible  

On close:

- remaining UNDER_REVIEW → ABANDONED  
- baseline becomes immutable  
- exactly one REVIEW receipt is emitted  

Fail if:

- outcomes finalize implicitly  
- baseline closes without receipt  

---

## CLI-BL-04 — Baseline Review Never Creates Legitimacy

Baseline review:

- does not evaluate authority  
- does not vote  
- does not legitimize outcomes directly  

All acceptance still occurs via sessions.

Fail if:

- baseline behaves like a session  

---

## CLI-BL-05 — Baseline Acceptance Always Produces Sessions

Every accepted proposal:

- creates a session  
- produces a LEGITIMACY receipt upon session closure  
- is auditable  

Fail if:

- a resolution appears without a session and legitimacy receipt  

---

## CLI-BL-06 — Preview Is Read-Only

Preview commands must:

- perform validation only  
- create no objects  
- emit no audits  
- mutate no state  

Fail if:

- preview leaves artifacts  

---

## CLI-BL-07 — Baseline Lifecycle Is Fully Auditable

Audit must reconstruct:

- start  
- accept  
- reject  
- close  

Fail if:

- progress cannot be reconstructed  

---

## CLI-BL-08 — No Implicit Carryover Between Baselines

Each baseline is independent.

Fail if:

- prior baseline affects a new baseline implicitly  

---

# VIII. Audit Guarantees (CLI)

## CLI-AUD-01 — CLI Owns Audit Store

The CLI is responsible for recording, persisting, and managing all audit data.

Fail if:

- engine events exist without CLI-recorded audit  
- audit store is missing or incomplete  

---

## CLI-AUD-02 — Audit Is Read-Only

Audit commands:

- never mutate state  
- never infer correctness  

---

## CLI-AUD-03 — Audit Is Deterministic

Same input → same output.

Ordering and aggregation must be explicit.

---

## CLI-AUD-04 — Audit Is Grep-Friendly

Audit output must:

- one event per line  
- stable field order  
- explicit keywords:  
  `AREA, SESSION, RESOLUTION, AUTH, SCOPE, BASELINE, DELIBERATE, BREAKOUT, RECEIPT`  
- always include engine IDs  

---

## CLI-AUD-05 — Participant Audits Are First-Class

CLI must support:

- audit by participant  
- participant timelines  
- resolution participation views  

Fail if:

- participant involvement cannot be reconstructed  

---

## CLI-AUD-06 — Audit Is the System of Record

Audit is authoritative for:

- what happened  
- when it happened  
- under what authority  
- with which participants  

Fail if:

- outcomes are not reconstructible via audit  

---

## CLI-AUD-07 — Provenance Is Preserved End-to-End

Audit must preserve lineage between:

- breakouts → synthesis  
- synthesis → proposals  
- proposals → baseline  
- baseline → sessions  
- sessions → legitimacy receipts  

Fail if:

- origin cannot be traced through receipts  

---

## CLI-AUD-08 — Audit Never Interprets Intent

Audit records facts only.

Fail if:

- audit output implies agreement, consensus, or correctness  

---

## CLI-AUD-09 — Imported/Consolidated Objects Must Populate Audit

When importing or consolidating:

- all historical resolutions, stances, and sessions MUST be recorded in the CLI audit store  
- lineage must reflect original acceptance context  

Fail if:

- imported objects are missing audit entries  

---
## CLI-AUD-10 — Federated Operations Preserve Audit & Lineage

When operating in federated or multi-node setups:
- all CLI actions must maintain audit integrity
- receipts and lineage references must remain complete and traceable across nodes
- cross-node operations must not break deterministic audit ordering

Fail if:
- audit or lineage cannot be reconstructed across federated environments

---

# IX. Storage & Durability

## CLI-STOR-01 — No Authoritative Data in Working Directories

Working directories may contain only:

- context pointers  
- metadata  

Fail if:

- deleting a folder deletes Charter history  

---

## CLI-STOR-02 — Authoritative Data Lives in Durable User Scope

Authoritative data must:

- live in durable, user-scoped storage  
- outlive project directories  

Fail if:

- durability depends on project folders  

---

# X. Forward Compatibility

## CLI-FWD-01 — Server Compatibility Is Preserved

CLI behavior must remain compatible with:

- server mode  
- shared or remote storage  

Fail if:

- CLI assumptions block multi-user futures  

---

# XI. Deliberate, Breakout & Synthesis (Guided Recording)

## CLI-DEL-01 — Deliberate Records Thinking, Not Decisions

Deliberate exists to capture exploration.

It must never:

- accept  
- reject  
- activate  
- supersede  
- evaluate authority  

Fail if:

- legitimacy appears without sessions or baseline review  

---

## CLI-DEL-02 — Deliberate Artifacts Are Non-Authoritative

Drafts, options, notes, syntheses:

- have no authority  
- have no scope  
- have no acceptance state  

Fail if:

- artifacts behave like candidates or resolutions  

---

## CLI-DEL-03 — Deliberate Concludes Explicitly

Every deliberate must end in exactly one of:

- synthesis handed off to baseline review  
- explicit abandonment  

Closure MUST emit an EXPLORATION receipt.

Fail if:

- artifacts persist indefinitely  
- closure occurs without receipt  

---

## CLI-BRK-01 — Breakouts Capture Exploration Only

Breakouts may:

- explore  
- draft  
- reframe  
- question  

They must not:

- vote  
- accept  
- mutate authoritative engine state  

Fail if:

- breakout activity affects legitimacy  

---

## CLI-BRK-02 — Breakouts Are Time-Bound Records

Closed breakouts are immutable records.

Closure MUST emit an EXPLORATION receipt.

Fail if:

- closed breakout content is edited  
- breakout closes without receipt  

---

## CLI-SYN-01 — Synthesis Produces Framed Outcomes

Synthesis outputs:

- options  
- questions  
- proposals-in-waiting  

Fail if:

- synthesis implies decision or authority  

---

## CLI-SYN-02 — Synthesis State Is Explicit and Descriptive

Every option must be explicitly categorized:

- READY  
- IN_PROGRESS  
- OPEN_ISSUE  
- DEFERRED  

States describe attention, not priority or approval.

Fail if:

- state is inferred or omitted  

---

## CLI-DEL-05 — Deliberate Is Fully Auditable

Audit must reconstruct:

- deliberate start  
- breakouts  
- synthesis evolution  
- baseline handoff  

Fail if:

- deliberate history cannot be followed end-to-end  

---

## CLI-ERR-01 — Explicit Error Reporting

All CLI invariant violations:

- must produce explicit, human-readable errors  
- must never fail silently  
- must indicate cause and remediation where possible  

Fail if:

- invariant violation occurs without error feedback  

---

## Lock Statement

These invariants are frozen.

Future features may extend them explicitly,  
but must never weaken them implicitly.

Receipts are first-class structural commits.  
Legitimacy remains session-only.  
Audit store ownership, lineage, and concurrency guarantees are CLI responsibilities.
