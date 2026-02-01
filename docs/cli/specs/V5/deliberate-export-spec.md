# Deliberate Export File Spec

The export file should resemble a **self-contained snapshot** of a deliberate. It must allow:

- Reimport into the same or another environment
- Integrity verification
- Preservation of audit and lineage

---
## File Format

- JSON (primary), optionally YAML for readability.
- Top-level object represents the deliberate.
```json
{
  "deliberate_label": "D-2026-01",
  "epic": "Epic Name or Goal",
  "status": "IN_PROGRESS | COMPLETE | ABANDONED",
  "created_at": "ISO8601 timestamp",
  "updated_at": "ISO8601 timestamp",
  "local_spec_hash": "<hash of local spec>",
  "export_hash": "<hash of this export content>",
  "breakouts": [
    {
      "breakout_label": "B-01",
      "parent_deliberate": "D-2026-01",
      "status": "COMPLETE | ABANDONED",
      "participants": ["Alice", "Bob"],
      "created_at": "ISO8601 timestamp",
      "closed_at": "ISO8601 timestamp if complete",
      "inputs": ["option_id_1", "option_id_2"],
      "outputs": ["option_id_3", "option_id_4"],
      "notes": ["Freeform notes or observations"]
    }
  ],
  "options": [
    {
      "option_id": "option_id_1",
      "content": "Description of option",
      "state": "READY | IN_PROGRESS | OPEN_ISSUE | DEFERRED",
      "source_breakout": "B-01 | null",
      "created_at": "ISO8601 timestamp",
      "updated_at": "ISO8601 timestamp"
    }
  ],
  "synthesis": {
    "finalized_options": ["option_id_3", "option_id_4"],
    "discarded_options": ["option_id_5"],
    "open_issues": ["option_id_6"],
    "notes": ["Summary of reasoning"]
  }
}
```

---
## Integrity / Hashing

- local_spec_hash → checksum of the engine specs at the time of export.
- export_hash → checksum of the JSON content (excluding this field) for integrity verification.
- CLI can warn if imported deliberate’s local_spec_hash does not match the current engine.

## Rules

1. Each export contains exactly one deliberate.
2. All breakouts and options must reference the deliberate.
3. Breakouts may reference parent breakouts (nested), but only one active at a time.
4. Import does not create legitimacy; imported options must go through deliberate → baseline → session to become resolutions.
5. Exported timestamps are immutable and required.
6. Freeform notes are optional, but always auditable.
