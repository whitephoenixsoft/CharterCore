# Usage Pattern: Game Legitimacy Engine 

## Intent

Use Charter Core as a decision tracking and legitimacy engine for games.

Charter records what was decided, by whom, and under which rules — and exposes this information to game systems without enforcing gameplay logic.

This pattern makes Charter a de facto engine for decision history, usable across genres and scales.

---

## Why Games

Games already operate on:
- explicit rules
- bounded domains
- authority structures
- irreversible choices

Charter does not introduce new mechanics — it stabilizes legitimacy.

If a game can answer:

> “Why is this allowed?”

with a resolution reference, the game world gains integrity.

---

## Core Concept

The game engine owns:
- state
- mechanics
- outcomes

Charter owns:
- legitimacy
- authority
- scope
- history of decisions

They are deliberately decoupled.

---

## Pattern Structure

### Areas

Areas represent decision domains, not game regions.

Examples:
- Kingdom Governance
- Guild Leadership
- World Events
- PvP Rules
- Quest Canon

---

### Authority

Authority defines who can make decisions, not what the decision means.

Examples:
- Guild leader
- Player vote
- Council of NPC factions
- System-defined roles

Authority rules are mechanical and auditable.

---

### Scope

Scope defines what kinds of decisions apply.

Examples:
- Trade laws
- War declarations
- Territory control
- Quest outcomes
- Faction alignment

Scope is descriptive, not enforced semantically.

---

### Resolutions

Resolutions represent canonical outcomes.

Examples:
- “The Northern Kingdom declared war”
- “Guild Alpha controls the eastern region”
- “The player allied with the Shadow Order”

Once accepted:
- immutable
- queryable 
- permanent

---

## Example: Multiplayer Guild Governance

### Context

- Guild members vote on leadership
- Disputes are common
- History matters

Flow

1. Area: Guild: Ironclad
2. Authority: Majority vote of active members
3. Scope: Guild leadership and rules
4. Session: “Elect Guild Leader”
5. Resolution accepted
6. Game engine queries Charter:

> “Who is the legitimate leader?”

Answer points to a resolution — not an opinion.

---

## Example: Open World Canon Tracking

### Context

- Player decisions affect the world
- Later systems need consistency

Flow

1. Area: World Canon
2. Resolution:

> “City of Asterfell was destroyed by the player”

3. NPC behavior, quests, and events query Charter

The world remembers why, not just that it changed.

---

## Example: Political Strategy Game

### Context

- Multiple factions
- Treaties, laws, coups

Flow

1. Areas represent councils or nations
2. Authority rules model governance
3. Resolutions represent laws and treaties
4. Systems check legitimacy before applying effects

Charter prevents:
- silent overrides
- rubber-stamping
- retroactive rule changes

---

## What Charter Core Guarantees

- No implicit decisions
- No rewritten history
- No assumed authority
- No semantic guessing

Every change points to:

> a session → a resolution → a rule

---

## What Games Gain

- Canon consistency
- Replayable legitimacy
- Mod-friendly governance
- Debuggable story logic
- Multiplayer dispute resolution
