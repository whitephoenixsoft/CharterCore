# Baseline Review
Status: FROZEN (Target)  
Applies to: Charter CLI and Charter Core interaction layer  
Forward-Compatible With: Commit model (V6+)  

---

## Purpose

Baseline Review is the mechanism by which foreign, imported, or external proposals are evaluated and either integrated into canonical Charter history or explicitly rejected.

It exists to prevent:

- Silent trust of external data  
- Legitimacy drift across time or versions  
- Retroactive reinterpretation of prior decisions  

Baseline Review is not a shortcut.  
It is a legitimacy firewall: nothing reviewed here becomes canonical until explicitly accepted through a session in the main engine.

---

## Core Idea

Baseline Review evaluates proposals, not decisions.

- No proposal in a baseline creates legitimacy.  
- Legitimacy is created only via sessions in the main engine.  
- Baseline Review is a workflow boundary, not a governance boundary.  

In V6+ terminology:

- Proposals may later become commit payloads.  
- Baseline Review itself produces a non-legitimizing commit (Review Receipt).  
- Only session-generated Resolutions create legitimate commits.

---

## Scope & Isolation

Baseline Review operates per Area.

- It pauses active sessions within the same Area only.  
- It does not affect other Areas.  
- It does not cross Context boundaries.  

Contexts remain hard-isolated storage partitions.  
A baseline review in one context has no visibility into another.

---

## Baseline Review Properties

A Baseline Review is:

- A bounded review workspace  
- Mutable until explicitly closed  
- Fully auditable  
- Isolated from active decision-making  
- Non-legitimizing by design  

It does not:

- Vote  
- Evaluate or infer Authority  
- Create consensus or legitimacy  
- Modify existing resolutions  
- Reinterpret imported legitimacy  

Any appearance of legitimacy is a CLI abstraction only.  
All actual legitimacy comes from sessions.

---

## Lifecycle

### 1. Creation

A Baseline Review is created by:

- Flat-file import  
- Consolidate import  
- Deliberate handoff  
- Future federation intake  

Restore operations replace an Area entirely and bypass review.

Upon creation:

- All proposals enter state: UNDER_REVIEW  
- Active sessions in the same Area are paused  
- The baseline becomes the sole active review workspace for that Area  

Proposals are considered foreign artifacts.  
They do not possess canonical IDs until accepted through session.

---

### 2. Evaluation Phase

During review:

- Proposals may be inspected, compared, grouped, or annotated  
- CLI may mark items as unchanged or modified (ergonomic only)  
- Authority is never evaluated  
- No legitimacy is created  

Acceptance or rejection decisions remain reversible only until a session is created.

Reversible means:

- No engine session has yet been emitted  
- No Legitimacy Receipt has been created  
- No canonical Resolution exists  

Once a session is created, it is immutable.

---

### 3. Acceptance Semantics

When a proposal is accepted:

- A new session is created in the main engine (explicit or CLI-hidden)  
- Participants are explicitly defined for that session  
- Authority and Scope are evaluated under canonical engine rules  
- A Legitimacy Receipt is emitted  
- Resulting Resolutions become canonical  
- Acceptance is recorded immutably  

Baseline Authority or Scope metadata is advisory only and never imported as governance truth.

Batch acceptance:

- Creates multiple independent sessions  
- Emits multiple Legitimacy Receipts  
- Never collapses legitimacy into a single event  

Rejected proposals:

- Are not submitted to legitimacy evaluation  
- Are not failed sessions  
- Do not represent authority denial  
- Remain auditable as workflow outcomes only  

They are proposals not elevated to governance.

---

### 4. Closure

When a Baseline Review is closed:

- Remaining UNDER_REVIEW proposals → ABANDONED  
- The baseline becomes immutable  
- Paused sessions in the Area may resume  
- Closure is irreversible  

Closure emits a Review Receipt.

The Review Receipt:

- Is a commit artifact (V6+)  
- Aggregates all proposals reviewed  
- Records accepted, rejected, and abandoned proposals  
- References resulting session IDs  
- Does not summarize or reinterpret legitimacy  
- Does not infer semantic meaning  

It is structural, not interpretive.

---

## Proposal States

Proposals within a baseline may be:

- UNDER_REVIEW  
- ACCEPTED (once session emitted)  
- REJECTED (explicit workflow rejection)  
- ABANDONED (implicitly closed at baseline closure)  
- HISTORICAL (Superceded)
- ACTIVE (active in local area by Resolution id or decision text)

Rejected and Abandoned are distinct states:

- Rejected: explicitly declined during evaluation  
- Abandoned: not evaluated before closure  

Neither creates legitimacy.

---

## Invariants

- Baseline Review never creates legitimacy  
- Every accepted proposal corresponds to a session and emits a Legitimacy Receipt  
- No proposal becomes ACTIVE implicitly  
- Rejection is not a failed session  
- Review history must be reconstructible end-to-end  
- Authority and Scope from baseline are advisory only  
- Topics, candidates, annotations, and receipts are preserved as future commit payloads  
- Closure always emits a Review Receipt  
- Engine semantics are never bypassed  

---

## Relationship to Commits (V6+ Foreshadowing)

Under the commit model:

- Accepted proposals become commit payloads through Resolution commits  
- Session lifecycle emits Legitimacy Receipt commits  
- Baseline closure emits a Review Receipt commit  
- Rejected and Abandoned proposals are referenced in the Review Receipt  

Baseline Review does not generate Resolution commits directly.

Only sessions do.

This preserves the invariant:

Legitimacy flows through the engine, never through workflow.

---

## Relationship to Other Constructs

| Construct | Role in Baseline Review |
|-----------|------------------------|
| Import (flat / consolidate) | Creates a baseline review workspace |
| Deliberate | May produce proposals handed to baseline |
| Session | Sole source of canonical legitimacy |
| Resolution | Smallest legitimacy unit |
| Review Receipt | Workflow closure commit |
| Restore | Replaces Area state; bypasses baseline |
| Audit | Must show complete lineage |

---

## Mental Model

Baseline Review means:

“We are evaluating claims made elsewhere. Nothing becomes official until we decide it again.”

This friction is intentional.

It prevents:

- Imported authority leakage  
- Governance shortcuts  
- Silent reinterpretation of history  

Workflow informs preparation.  
Only sessions create legitimacy.  
Receipts preserve the boundary.

---

## Session Notes

Sessions created from baseline:

- May omit a problem statement (optional and non-semantic)  
- Preserve proposal topic and candidates  
- Copy annotations for context only  
- Require explicit participants  
- Evaluate Authority and Scope canonically  

Session descriptions never influence legitimacy.

---

## Final Boundary

If Baseline Review ever:

- Creates legitimacy  
- Evaluates authority  
- Collapses workflow into governance  
- Or synthesizes decisions  

Then Charter has violated its core separation discipline.

Baseline Review prepares.  
The engine decides.