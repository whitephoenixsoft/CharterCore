# V3 — Multi-User Collaboration & Deliberate Workflows

Commands for ergonomics, multi-user collaboration, nested deliberations, and auditable workflows.

---

## Area / Cross-Area References
- `charter area reference <area>/<label>` — reference resolutions across areas
- `charter label create [optional-name]` — generate CLI label if not provided

---

## Context & Workspace Ergonomics
- `charter context switch <name>` — switch active context
- `charter context clone <name>` — copy context for experimentation
- `charter context list` — show all contexts
- `charter context show <name>` — display context details
- `charter context detach` — detach active context
- `charter context remove <name>` — delete context
- `charter context purge <name>` — delete with full history

---

## Session Enhancements
- `charter session start` — begin a new session
- `charter session pause` — pause session for nested Deliberate
- `charter session resume` — resume paused session
- `charter session close` — close session normally
- `charter session restart-from <session_id>` — restart session with lineage
- `charter session default-participant <actor_id>` — automatically added participant
- `charter session branch <session_id>` — create a derived session
- `charter session merge <session_id> [--force]` — merge branch respecting authority

---

## Deliberate / Breakouts
- `charter deliberate start <epic>` — start a new Deliberate with a goal (Epic)
- `charter deliberate pause` — pause active Deliberate
- `charter deliberate resume` — resume paused Deliberate
- `charter deliberate add-option <description>` — add new option to Deliberate
- `charter deliberate declare-complete` — finalize Deliberate, start baseline review
- `charter breakout start <deliberate-id> [--participants <group>]` — start nested breakout
- `charter breakout complete <breakout-id>` — finish breakout, pass artifacts to synthesis
- `charter breakout close <breakout-id>` — abandon breakout, discard outputs
- `charter breakout restart-from <breakout-id>` — restart breakout preserving lineage

---

## Baseline & Import / Export
- `charter baseline preview <import-id>` — preview imported baseline
- `charter baseline preview-unchanged <import-id>` — show unchanged resolutions
- `charter baseline highlight-changes <import-id>` — show only modified resolutions
- `charter baseline list` — list all baselines
- `charter baseline open <import-id>` — open baseline for review
- `charter baseline accept <resolution-label> | <import-id> [--force] [--all]` — accept resolutions
- `charter baseline reject <resolution-label> | <import-id> [--all]` — reject resolutions
- `charter baseline close` — finalize baseline review
- `charter baseline purge <import-id>` — remove baseline

- `charter import consolidate-auto <filename>` — CLI chooses consolidate/restore automatically
- `charter import restore-auto <filename>` — CLI chooses consolidate/restore automatically
- `charter export <filename>` — export data safely
- `charter import <filename>` — import resolutions or data
- `charter deliberate export <filename>` — export Deliberate artifacts for sharing
- `charter deliberate import <filename>` — import Deliberate artifacts (treated as foreign)

---

## Draft Candidates & Participant Groups
- `charter draft-candidate create <name>` — create reusable candidate set
- `charter draft-candidate list` — list available draft candidates
- `charter draft-candidate show <name>` — show details
- `charter draft-candidate delete <name>` — remove draft candidate
- `charter participant-group create <name>` — define reusable participant group
- `charter participant-group list` — list groups
- `charter participant-group show <name>` — show details
- `charter participant-group delete <name>` — remove group

---

## Option States (Internal to Deliberate / Synthesis)
- `charter option list <deliberate-id>` — list options with states (READY, IN_PROGRESS, OPEN_ISSUE, DEFERRED)
- `charter option set-state <option-id> <state>` — manually adjust option state

---

## Audit & Spec Verification
- `charter audit timeline` — global audit timeline
- `charter audit participants` — audit by participants
- `charter audit by-participant <actor_id>` — detailed participant audit
- `charter audit by-area <area>` — filter audit output
- `charter audit by-resolution <resolution>` — filter audit output
- `charter spec list` — list specs
- `charter spec show <SPEC_ID>` — display spec
- `charter spec verify` — verify implementation against spec
- `charter spec diff <SPEC_ID_1> <SPEC_ID_2>` — compare spec versions