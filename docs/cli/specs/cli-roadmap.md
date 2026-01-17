# Charter CLI — Command Roadmap

## V1 — Minimal, Engine-Safe Commands
Commands that touch the engine in a legitimate, auditable way. Must exist in V1 to support solo and multi-user sessions, baseline review, and audit.

### Context & Storage
- `charter init`
- `charter context create [name]`
- `charter context use <name>`
- `charter context list`
- `charter context show [name]`
- `charter context detach`
- `charter context remove`
- `charter context purge`

### Area
- `charter area create`
- `charter area use`
- `charter area remove [id]`
- `charter area purge`
- `charter area list`
- `charter area show <id>`

### Authority & Scope (V1 Minimum)
- `charter authority set <rule>`
- `charter authority show`
- `charter scope set <scope-definition>`
- `charter scope show`

### Sessions
- `charter session start`
- `charter session pause`
- `charter session resume`
- `charter session close`
- `charter session restart-from <session_id>`
- `charter session list`
- `charter session show <id>`

### Status
- `charter status`
- `charter status session [ID]`
- `charter status area [id]`
- `charter status resolution [id]`
- `charter status baseline [id]`
- `charter status context`

### Import / Export / Baseline
- `charter import <filename>` (default: non-destructive baseline review, auto-detect consolidate vs. restore optional later)
- `charter import consolidate <filename>`
- `charter import restore <filename>`
- `charter export <filename>`

### Baseline Review
- `charter baseline list`
- `charter baseline preview <filename>`
- `charter baseline open <import-id>`
- `charter baseline status`
- `charter baseline show [resolution-label]`
- `charter baseline accept <resolution-label> | <import-id> [--force] [--all]`
- `charter baseline reject <resolution-label> | <import-id> [--all]`
- `charter baseline close`
- `charter baseline purge <import-id>`

### Audit
- `charter audit timeline`
- `charter audit participants`
- `charter audit by-participant <actor_id>`
- `charter audit resolution --participants`

### Spec Verification
- `charter spec list`
- `charter spec show <SPEC_ID>`
- `charter spec verify`

---

## V2 — Participant & Voting Management
Commands for multi-user scenarios, adding/removing participants, and basic voting.

### Session Management
- `charter session add-participant <actor_id>`
- `charter session remove-participant <actor_id>`
- `charter session list-participants`
- `charter session vote <candidate_id> <ACCEPT|REJECT|ABSTAIN>`
- `charter session show-votes <session_id>`

### Baseline Review Enhancements
- `charter baseline accept-batch <resolution-id> [--all]`
- `charter baseline reject-batch <resolution-id> [--all]`
- `charter baseline show-participant-votes <resolution-id>`

### Authority / Scope Adjustments
- `charter authority modify <rule>` (requires session)
- `charter scope modify <scope-definition>` (requires session)

---

## V3 — Multi-User Collaboration & Ergonomics
Commands for ergonomics, automation helpers, and multi-area references.

### Area / Cross-Area References
- `charter area reference <area>/<label>` (reference resolutions across areas)
- `charter label create [optional-name]` (CLI-generated label if not provided)

### Context & Workspace Ergonomics
- `charter context switch <name>` (shortcut for use)
- `charter context clone <name>` (copy context for experimentation)

### Session Enhancements
- `charter session default-participant <actor_id>` (default added automatically when session starts)
- `charter session branch <session_id>` (create a derived session with lineage)
- `charter session merge <session_id> [--force]` (merge branches, respecting authority)

### Baseline & Import / Export
- `charter baseline preview-unchanged <import-id>` (show resolutions unchanged from previous)
- `charter baseline highlight-changes <import-id>` (show only modified)
- `charter import consolidate-auto <filename>` (CLI decides consolidate vs. restore automatically)
- `charter import restore-auto <filename>` (CLI decides consolidate vs. restore automatically)

### Audit & Spec Verification
- `charter audit by-area <area>` (filter audit output)
- `charter audit by-resolution <resolution>` (filter audit output)
- `charter spec diff <SPEC_ID_1> <SPEC_ID_2>` (compare spec versions)