# Charter Care Substrate (CCare) — Foundation Specification

Status: FOUNDATIONAL  
Applies to: Charter Care Substrate (CCare), Check-ins, Requests, Supportability Signals  
Depends On: Charter Commit System (CCS), Commit Store  
Consumed By: Charter Alignment System (CAS), Charter Guidance Layer (CGL), external systems  
Does NOT define: alignment computation, identity, legitimacy, workflow orchestration, or enforcement  

---

# 1. Purpose

The Charter Care Substrate (CCare) provides a structured, non-authoritative way to record:

- observations about alignment relative to declared decisions  
- expressions of capacity, uncertainty, or strain  
- requests for attention or clarification  

CCare exists to make **care visible** without:

- enforcing behavior  
- evaluating performance  
- creating legitimacy  

It answers:

> “What is being observed relative to what was declared?”

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

- produces commit artifacts (signals, requests)  
- does not modify existing resolutions  
- does not participate in legitimacy  

CCare is consumed by:

- CAS (for computation)  
- CGL (for interpretation)  
- external systems (optional)

---

# 4. Core Constructs

CCare defines three primary constructs:

---

## 4.1 Check-ins (Signals)

Check-ins are lightweight observations about alignment.

They:

- reference a target (resolution, area, or identity if present)  
- describe perceived alignment condition  
- may include confidence  

Check-ins do not:

- trigger action  
- imply authority  
- modify decisions  

---

### 4.1.1 Semantic States

Check-ins may express states such as:

- alignment  
- misalignment  
- uncertainty  
- reduced_capacity  
- intentional_pause  
- need_reassessment  

These states are:

- descriptive  
- non-exhaustive  
- not prescriptive  

---

### 4.1.2 Silence

Silence is a valid state.

Silence may indicate:

- stability  
- lack of change  
- absence of observation  

Silence must not be interpreted as:

- agreement  
- alignment  
- compliance  

---

## 4.2 Requests

Requests are non-coercive signals indicating:

- attention may be needed  
- clarification may be beneficial  
- reassessment may be appropriate  

Requests:

- do not create obligations  
- do not assign responsibility  
- do not enforce outcomes  

They are invitations, not commands.

---

## 4.3 Supportability Signals

Supportability signals describe whether the system can:

> reliably demonstrate alignment relative to declared intent

Examples:

- supportable  
- degraded supportability  
- unclear supportability  

These signals:

- reflect clarity, not correctness  
- may exist even when no misalignment is present  

---

# 5. Data Model

All CCare artifacts are stored as CCS commits.

---

## 5.1 Check-in Commit Structure

A check-in commit includes:

- target reference (resolution, area, identity optional)  
- semantic state  
- optional confidence  
- timestamp  
- optional annotations  

---

## 5.2 Request Commit Structure

A request commit includes:

- target reference  
- request type (attention, clarification, reassessment)  
- timestamp  
- optional context  

---

## 5.3 Supportability Commit Structure

A supportability commit includes:

- target reference  
- supportability state  
- timestamp  
- optional annotations  

---

# 6. Targeting Model

CCare may target:

- resolution  
- area  
- identity (if CIS present)  
- global  

Targets are references only.

CCare does not interpret structural relationships.

---

# 7. Temporal Model

CCare signals are:

- append-only  
- time-stamped  
- non-retractable  

They represent:

- observations at a point in time  
- not persistent truth  

CAS is responsible for interpreting temporal dynamics.

---

# 8. Non-Authority Guarantees

CCare must never:

- create legitimacy  
- override decisions  
- imply obligation  
- assign blame  
- enforce compliance  
- trigger automatic action  

---

# 9. Relationship to CAS

CCare provides inputs to CAS.

CAS:

- aggregates signals  
- computes alignment dynamics  
- derives trends and propagation  

CCare does not:

- compute alignment  
- aggregate signals  
- derive trends  

---

# 10. Relationship to CSG

CCare is structurally unaware.

It:

- references targets  
- does not interpret graph relationships  

CSG provides structure for CAS, not for CCare.

---

# 11. Relationship to CIS (Optional)

If identity exists:

- CCare may target identities  
- CCare does not define identity  

Identity semantics are handled by CIS.

---

# 12. Observation Philosophy

CCare treats:

- divergence as information  
- capacity limits as valid  
- uncertainty as acceptable  
- pauses as legitimate  

It avoids:

- performance framing  
- success/failure framing  
- optimization pressure  

---

# 13. Signal Characteristics

CCare signals are:

- sparse (no requirement for constant reporting)  
- voluntary  
- contextual  
- non-normalized across systems  

Different targets may:

- use different interpretations  
- express signals differently  

CAS normalizes through computation, not CCare.

---

# 14. Failure Modes

CCare explicitly allows:

- incomplete signal coverage  
- inconsistent reporting  
- delayed observations  

CAS must operate under these conditions.

---

# 15. Design Guarantees

CCare is:

- append-only  
- non-authoritative  
- structurally minimal  
- consumer-agnostic  
- compatible with silence  

---

# 16. Mental Model

CCare does not answer:

- “Are we performing well?”  
- “Are we meeting targets?”  

It answers:

> “What are we observing about our ability to remain aligned with what we declared?”

---

# 17. Final Principle

CCare exists to make care explicit without turning care into control.

It allows systems to:

- express strain  
- surface uncertainty  
- acknowledge limits  
- request attention  

without:

- fear of enforcement  
- distortion of truth  
- pressure to perform  

Care is recorded.  
Interpretation is deferred.