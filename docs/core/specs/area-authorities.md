# Area Authority for the Engine 

## Enums

### 1. SOLE_ACTOR

#### Meaning

- Exactly one actor has standing
- Acceptance requires that actor’s explicit acceptance

#### Mechanical Rule

- If the sole actor accepts → resolution accepted
- No voting aggregation

#### Common Use

- Solo founder
- Personal decision journal
- CLI default mode

#### Why It Exists

- Prevents inventing fake consensus
- Makes solo usage explicit and auditable

---
### 2. UNANIMOUS_PRESENT

#### Meaning

- All currently present participants must agree

#### Mechanical Rule

- Every present participant must explicitly accept
- Abstention or objection blocks acceptance

#### Important Property

- Membership is evaluated at time of acceptance
- Leaving the session changes the present set

#### Common Use

- Small teams
- High-trust groups
- Early-stage projects

---
### 3. MAJORITY_PRESENT

#### Meaning

- More than half of present participants must accept

#### Mechanical Rule

- accepts > floor(present / 2)

#### Important Property

- Presence matters more than membership
- Late joiners don’t retroactively affect outcomes

#### Common Use

- Democratic teams
- Committees
- Fast-moving groups

---
## Defaults

### Default for the CLI

SOLE_ACTOR

Why:
- Zero surprise
- No phantom governance
- Matches solo developer mental model

---
### Default for UI or Tool 

UNANIMOUS_PRESENT

Why:
- Conservative
- Forces explicit agreement
- Easy to reason about