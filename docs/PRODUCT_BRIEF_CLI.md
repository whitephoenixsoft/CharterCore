# Charter CLI — Product Brief

## Purpose

Charter CLI is a developer-focused interface for making, recording, and evolving legitimate decisions using Charter Core.

It provides a fast, local, solo-first workflow for capturing decisions with full auditability — while remaining fully compatible with collaborative, multi-user, and future server-based workflows.

> Charter CLI does not invent governance rules.
> It operationalizes them.

## Problem

Developers and technical leaders make important decisions constantly:
- Architectural choices
- Trade-offs and constraints
- Policy interpretations
- “Temporary” decisions that quietly become permanent

Today, these decisions are:
- scattered across chats, docs, and tickets
- overwritten instead of evolved
- impossible to audit later
- disconnected from authority and scope

Even disciplined teams lose decision lineage.

## Solution

Charter CLI turns decisions into first-class artifacts — locally, incrementally, and without ceremony.

It enables developers to:
- Explicitly define where a decision applies (Area)
- Declare how agreement is measured (Authority)
- Capture what the decision governs (Scope)
- Decide through structured Sessions
- Preserve full historical context automatically

All without requiring:
- a server
- AI
- workflows
- roles
- permissions
- coordination overhead

## What Charter CLI Is

Charter CLI is:
- A local decision journal with institutional memory
- A solo-first interface to Charter Core
- A source-control–friendly governance tool
- A staging and consolidation layer for decisions
- A legitimacy-preserving workflow for humans

It treats decisions the way Git treats code:
- nothing is implicit
- history is immutable
- divergence is visible
- reconciliation is explicit

## What Charter CLI Is Not

Charter CLI is not:
- A policy engine
- A workflow system
- A task tracker
- A chat tool
- An AI moderator
- A permissions system

Those can be built around Charter — not into it.

## Key Capabilities

### 1. Solo-First Decision Making

- Designed for individual developers by default
- One active session or review at a time
- Ergonomic commands that guide “next actions”
- Authority defaults to solo, but is always explicit

This makes Charter useful immediately — without coordination.
### 2. Explicit Decision Sessions

Decisions are made through sessions that:
- define a single problem
- stage candidates
- record acceptance mechanically
- close cleanly when complete

No silent acceptance.
No accidental decisions.
### 3. Immutable Audit Trail (by Construction)

Every resolution captures:
- when it was accepted
- under which authority
- within which scope
- from which session
- what it superseded

The user never has to “remember to document.”
### 4. Source-Control–Friendly Collaboration

Charter CLI supports:
- exporting decisions as canonical JSON
- committing them to source control
- importing divergent timelines
- consolidating via explicit review

Disconnected collaboration becomes possible — safely.
### 5. Review as Reconciliation (Not Guessing)

Imports enter a review workspace:
- resolutions are UNDER_REVIEW
- acceptance is explicit
- rejection is explicit
- abandonment is explicit

No auto-merging. No semantic inference. No silent overrides.

## Productivity Gains

Charter CLI improves productivity by:
- Eliminating re-litigation of old decisions
- Making assumptions visible
- Reducing context loss during handoffs
- Allowing fast, low-friction decision capture
- Preventing accidental governance drift

It does not speed up decisions by cutting corners.
It speeds them up by removing confusion.

## Design Principles

Charter CLI is built on these principles:
- Legitimacy over convenience
- Explicit over implicit
- History over overwrite
- Human judgment over automation
- Minimal surface area
- Predictable ergonomics

Every command maps cleanly to a Charter Core primitive.

## Relationship to Charter Core

Charter CLI is a thin, opinionated client of Charter Core.

- All legitimacy is enforced by the engine
- The CLI adds ergonomics, not power
- No CLI shortcut can violate engine invariants
- Everything recorded by the CLI can be replayed elsewhere

Charter CLI proves that Charter Core can be:
- embedded
- scripted
- versioned
- scaled

## Who This Is For

- Individual developers
- Tech leads
- Architects
- Founders
- Open-source maintainers
- Anyone tired of losing why decisions were made

## Why This Is Rare

Most tools optimize for:
- execution
- communication
- coordination

Almost none optimize for:
- legitimacy
- decision memory
- non-retroactive truth

Charter CLI exists because Charter Core exists — and Charter Core exists because most systems silently erase decisions.

## In One Sentence

> Charter CLI is a developer-first tool for making decisions that survive time, turnover, and disagreement — without slowing you down.