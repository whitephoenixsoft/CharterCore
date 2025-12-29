# Referencing Other Objects

## The Key Insight

> References are not authority.
> References are not enforcement.
> References are breadcrumbs.

Therefore:
- We do not need to choose one reference type.
- We need a tiered, explicit reference model.

---

## Engine Model

Session May Reference:
- Zero or more Areas
- Zero or more Resolutions

And nothing else.

### Why This Works

| Need                  | Area Ref | Resolution Ref |
| :-------------------- | :------- | -------------- |
| Human reminder        | ✅        | ⚠️             |
| Audit precision       | ⚠️       | ✅              |
| Multi-user signaling  | ✅        | ⚠️             |
| Engine simplicity     | ✅        | ✅              |
| No semantic inference | ✅        | ✅              |
Scopes stay implicit, via the resolution reference if needed.

---

## Engine API Shape (Conceptual)

```Text
start_session(
  area_id,
  session_id,
  problem_statement,
  participant_list,
  preceding_resolution_id?,
  referenced_area_ids?,
  referenced_resolution_ids?
)
```
Rules:
- References are immutable metadata
- No enforcement
- No automatic blocking
- No authority leakage
- Superseded referenced resolutions may trigger **informational warnings only**

---
## CLI Ergonomics (Solo-First, Future-Safe)

### Default Mental Model

- Area reference = “Heads up”
- Resolution reference = “This is related to that decision”

### CLI Examples

```Bash
# Broad dependency
charter session start \
  --ref-area finance

# Precise dependency
charter session start \
  --ref-resolution finance/R-12
```
Or both:
```Bash
charter session start \
  --ref-area finance \
  --ref-resolution finance/R-12
```

---
## Multi-User Future (Without Engine Changes)

Later, an integration can:
- Watch for referenced_area_ids
- Notify stakeholders in that Area

Build dashboards:
- “Decisions referencing your area”
- “External dependencies”

All outside the engine.

---
## Example Audit Output (Resolution with References)

### Resolution Audit — Example
```Text
RESOLUTION R-101
AREA A-NULLSEC
SESSION S-101
ACCEPTED_AT 2025-03-18T14:22:09Z

AUTHORITY R-AUTH-NULLSEC-1
SCOPE R-SCOPE-NULLSEC-1

REFERENCES:
  AREA B-LEADERSHIP
  AREA C-LOGISTICS

CANDIDATE C-DEPLOY-ALL-CAPS

PARTICIPANTS:
  pilot:alice ACCEPT
  pilot:bob   ACCEPT
  pilot:carol ACCEPT

SUPERSEDES: none
STATUS: ACTIVE
```
Notice what’s not present:
- No “approved by”
- No “violated scope”
- No interpretation of silence
- No enforcement hints

This makes references visible without moralizing.

---
## CLI Area Status 

For summary:
```Bash
charter area status

Area: platform
Authority: R-AUTH-3
Scope: R-SCOPE-2

Active Sessions: 1
Active Resolutions: 12

Referenced By:
  5 references (last: 2025-03-17)
```

For details:
```Bash
charter area references --since 30d
```

Output:
```Text
2025-03-17  AREA security  SESSION S-88
2025-03-12  AREA product   SESSION S-74
2025-03-09  AREA infra     SESSION S-61
```

### Explicit CLI Rule (Important)

- Never show references inline with authority
- Never use “needs attention” language
- Never block commands based on references

They are awareness signals, not tasks.