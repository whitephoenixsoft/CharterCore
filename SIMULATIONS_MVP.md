# Charter — MVP Simulations

This document defines the minimal simulations the MVP must support
to be considered correct.

---

## MVP Simulation 1 — Single User, One Area

- User creates Area
- User starts a session
- User proposes a resolution
- User explicitly accepts
- Resolution becomes immutable

**Must Hold**
- Explicit acceptance
- Immutability
- Decision memory

---

## MVP Simulation 2 — Scope Violation Blocks Acceptance

- User proposes out-of-scope resolution
- System blocks acceptance
- Block reason is visible

**Must Hold**
- No silent overreach
- Session blocking

---

## MVP Simulation 3 — Supersession

- Active resolution exists
- New resolution for same scope is accepted
- Old resolution becomes superseded

**Must Hold**
- Single active resolution per scope
- History preserved

---

## MVP Simulation 4 — AI Assistance Without Authority (If Enabled)

- AI suggests resolution text
- AI cannot accept
- Human must explicitly accept

**Must Hold**
- AI facilitator-only role

---

## MVP Simulation 5 — Block and Resume

- Session becomes blocked
- User resolves issue manually
- Session resumes with revalidation

**Must Hold**
- No continuation under invalid assumptions

---

## Exit Criteria

If the MVP fails any of these simulations,
it is considered incorrect regardless of feature completeness.

---

MVP Simulation A — Manual Session, No AI

User creates Area

User starts session

User manually inputs candidate

User explicitly accepts

Resolution is immutable


Must Hold

Explicit acceptance

No silent decisions

Decision memory



---

MVP Simulation B — Slack-Backed Decision

Discussion happens externally

Candidate is summarized manually

Resolution links to external context


Must Hold

Integrations are contextual only

Charter remains the source of legitimacy



---

MVP Simulation C — Supersession

Active resolution exists

New resolution is accepted

Old resolution is superseded, not edited


Must Hold

History preserved

Single active resolution per scope



---

MVP Simulation D — AI Disabled by Default

AI is fully off

System remains usable

No invariant depends on AI


Must Hold

AI optionality

Engine completeness
