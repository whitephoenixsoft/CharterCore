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