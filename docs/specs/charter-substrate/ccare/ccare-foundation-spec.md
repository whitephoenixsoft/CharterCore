# Charter Care Substrate (CCare) — Foundation Specification (Revised)

Status: FOUNDATIONAL  
Applies to: Charter Care Substrate (CCare), Check-ins, Requests, Supportability Signals, Care Suggestions  
Depends On: Charter Commit System (CCS), Commit Store  
Consumed By: Charter Alignment System (CAS), Charter Guidance Layer (CGL), Charter Deliberate Substrate (CDS), external systems  
Does NOT define: alignment computation, identity, legitimacy, workflow orchestration, or enforcement  

---

# 1. Purpose

The Charter Care Substrate (CCare) provides a structured, non-authoritative way to record:

- observations about alignment relative to declared decisions  
- expressions of capacity, uncertainty, or strain  
- requests for attention or clarification  
- optional suggestions for further thinking  

CCare exists to make **care visible** without:

- enforcing behavior  
- evaluating performance  
- creating legitimacy  

It answers:

> “What is being observed relative to what was declared, and what might be worth further consideration?”

---

# 2. Core Principle

> CCare records observation, not judgment.  
> It expresses condition, not obligation.

CCare must remain:

- descriptive  
- voluntary  
- non-coercive  
- non-authoritative  

---

# 3. Position in Architecture

CCare operates alongside CCS and the Commit Store.

It:

- produces commit artifacts (signals, requests, suggestions)  
- does not modify existing resolutions  
- does not participate in legitimacy  

CCare is consumed by:

- CAS (for computation)  
- CGL (for interpretation)  
- CDS (as optional input for thinking)  
- external systems (optional)

---

# 4. Core Constructs

CCare defines four primary constructs:

---

## 4.1 Check-ins (Signals)

Unchanged.

---

## 4.2 Requests

Unchanged.

---

## 4.3 Supportability Signals

Unchanged.

---

## 4.4 Care Suggestions (NEW)

Care Suggestions are non-authoritative expressions indicating:

> something may benefit from deliberate attention or exploration.

They may represent:

- potential gaps in understanding  
- emerging patterns  
- possible adjustments (e.g., thresholds, definitions)  
- invitations to revisit assumptions  
- prompts for deeper investigation  

---

### 4.4.1 Properties

Care Suggestions:

- are non-coercive  
- do not create obligations  
- do not assign responsibility  
- do not imply urgency or priority  
- may reference signals, resolutions, areas, or identities  

---

### 4.4.2 Relationship to Deliberate (CDS)

Care Suggestions:

- may be **adopted into CDS as Items**  
- must not automatically create CDS Items  
- must not trigger workflows  

Principle:

> CCare offers possibilities. CDS chooses whether to think about them.

---

### 4.4.3 Federation

Care Suggestions:

- may be shared across systems  
- remain non-authoritative when imported  
- must not be interpreted as commands or tasks  

---

# 5. Data Model

All CCare artifacts are stored as CCS commits.

---

## 5.4 Care Suggestion Commit Structure (NEW)

A care suggestion commit includes:

- target reference (optional)  
- optional related signal references  
- descriptive suggestion content  
- timestamp  
- optional annotations  

---

# 6. Targeting Model

Unchanged.

---

# 7. Temporal Model

Unchanged.

---

# 8. Non-Authority Guarantees

CCare must never:

- create legitimacy  
- override decisions  
- imply obligation  
- assign responsibility  
- enforce compliance  
- trigger automatic action  
- escalate suggestions into tasks  

---

# 9. Relationship to CAS

Unchanged.

---

# 10. Relationship to CDS (NEW)

CCare may act as a **source of thinking inputs**.

---

## 10.1 Intake Boundary

- CCare does not push into CDS  
- CDS may explicitly adopt CCare artifacts as Items  

---

## 10.2 Independence

- CCare artifacts remain unchanged after adoption  
- CDS Items derived from CCare are independent  

---

## 10.3 Principle

> Observation and thinking remain separate until explicitly connected.

---

# 11. Relationship to CSG

Unchanged.

---

# 12. Relationship to CIS (Optional)

Unchanged.

---

# 13. Observation Philosophy

Unchanged.

---

# 14. Signal Characteristics

Unchanged.

---

# 15. Failure Modes

Unchanged.

---

# 16. Design Guarantees

CCare is:

- append-only  
- non-authoritative  
- structurally minimal  
- consumer-agnostic  
- compatible with silence  
- safe for federation  

---

# 17. Mental Model

CCare does not answer:

- “What should we do?”  
- “Who should act?”  

It answers:

> “What are we observing, and what might be worth attention?”

---

# 18. Final Principle

CCare exists to make care explicit without turning care into control.

It allows systems to:

- express strain  
- surface uncertainty  
- acknowledge limits  
- suggest reflection  

without:

- fear of enforcement  
- distortion of truth  
- pressure to act  

Care is recorded.  
Thinking is optional.  
Action is separate.