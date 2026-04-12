# Value-Directed System (VDS) — Foundation Specification

Status: FOUNDATIONAL (DRAFT)  
Applies to: VDS Hosts, System Monitoring Integrations, Federation Contexts  
Depends On: Charter Runtime, CQL, CCare Model, Commit Store, Provenance Model, Versioning & Identity Model  
Does NOT define: telemetry ingestion specifics, threshold DSL, alerting systems, or enforcement mechanisms  

---

# 1. Purpose

The Value-Directed System (VDS) is a **system-facing caregiving host** that observes real-world behavior relative to declared decisions.

It exists to:

- interpret telemetry through the lens of explicit commitments  
- surface alignment, drift, and pressure  
- provide non-coercive visibility into system behavior  
- support safe, observable system evolution  
- preserve autonomy while improving understanding  

---

# 2. Core Principle

> VDS interprets behavior. It does not control it.

VDS:

- observes  
- evaluates  
- surfaces  

It does not:

- enforce  
- optimize  
- execute change  
- assign authority  

---

# 3. Architectural Role

VDS is a **product-layer host system** built on Charter substrates.

It is not:

- a Charter substrate  
- a legitimacy system  
- a workflow engine  

It is:

- a consumer of Charter commitments  
- an interpreter of telemetry  
- a producer of caregiving signals  

---

# 4. Inputs

## 4.1 Charter-Derived Inputs

VDS consumes:

- decisions / commitments (via CQL)
- scope and boundaries (via CIS / Area context)
- structural relationships (via CSG where relevant)
- alignment metrics (via CAS where available)
- provenance and rule identity where needed

### Principle

> Telemetry has meaning only in relation to declared decisions.

---

## 4.2 External Inputs

VDS consumes telemetry such as:

- metrics
- logs
- traces
- events
- external system signals

These inputs are:

- non-authoritative  
- transient  
- system-dependent  

---

# 5. Interpretation Model

## 5.1 Decision-Anchored Interpretation

All interpretation must be anchored to:

- declared expectations  
- defined bounds or tolerances  
- explicit commitments  

If no explicit decision exists:

- VDS must remain observational only  

---

## 5.2 Thresholds and Conditions

Thresholds represent:

- bounds of acceptable behavior  
- tolerance ranges  
- expectation limits  

They are:

- derived from or attached to decisions  
- evaluated mathematically  
- non-coercive  

### Constraint

Thresholds must not:

- imply obligation  
- trigger automatic system changes  
- create hidden authority  

---

## 5.3 Interpretation Outcomes

VDS produces **caregiving signals** (check-ins), not alerts.

These may include:

- alignment confirmation  
- drift detection  
- pressure indication  
- anomaly surfacing  
- silence (no signal when stable)

---

# 6. Signal Model (CCare Alignment)

## 6.1 Signal Types

Signals may include:

- check-ins (state summaries)
- requests (non-binding prompts)
- supportability indicators
- risk surfacing

---

## 6.2 Signal Properties

Signals are:

- non-authoritative  
- non-binding  
- descriptive  
- ephemeral unless persisted intentionally  

---

## 6.3 Signal Constraint

> A VDS must never require action.

All signals:

- preserve user/system autonomy  
- support decision-making without enforcing it  

---

# 7. Federation Model

## 7.1 Purpose

VDS may participate in federation across systems.

---

## 7.2 Behavior

Federation propagates:

- signals  
- summaries  
- visibility  

Not:

- control  
- authority  
- system state  

---

## 7.3 Principle

> Visibility does not imply obligation.

Receiving systems:

- may observe signals  
- must not be required to act  

---

# 8. Deployment & Experimentation Support

VDS supports:

- rollout observation  
- experiment monitoring  
- transition visibility  

This includes:

- pre-change observation  
- in-flight monitoring  
- post-change evaluation  

---

## 8.1 Constraint

VDS must not:

- manage deployment execution  
- enforce rollout decisions  

---

# 9. Persistence & Query Integration

## 9.1 Query Model

VDS consumes Charter data via:

- CQL interfaces  
- domain query surfaces  

---

## 9.2 Persistence

VDS may optionally persist:

- signals  
- summaries  
- derived observations  

Persistence must not:

- override Charter truth  
- create competing sources of legitimacy  

---

# 10. Relationship to Charter

## 10.1 Charter as Source of Meaning

Charter provides:

- decisions  
- legitimacy  
- structure  

VDS provides:

- interpretation of behavior relative to those decisions  

---

## 10.2 Separation

- Charter defines what was decided  
- VDS observes how reality relates to it  

---

# 11. Non-Interpretation Boundary

VDS must not:

- infer intent beyond declared decisions  
- create implicit commitments  
- reinterpret historical decisions  

All interpretation must remain:

- bounded  
- explicit  
- anchored  

---

# 12. Determinism & Transparency

Given:

- identical decisions  
- identical telemetry  
- identical thresholds  

VDS must produce:

- consistent interpretations  

Interpretation logic must be:

- inspectable  
- explainable  
- reproducible  

---

# 13. Invariants

- VDS never executes change  
- VDS never creates legitimacy  
- VDS never enforces behavior  
- signals are non-binding  
- telemetry is non-authoritative  
- decisions define meaning  
- thresholds must be explicit  
- federation does not transfer control  
- interpretation must be explainable  

---

# 14. Known Gaps (Initial)

The following areas are intentionally incomplete:

- threshold definition model (DSL / schema)  
- mapping of decisions to measurable conditions  
- signal persistence standards  
- federation transport mechanisms  
- integration patterns for telemetry sources  

These may be:

- implemented within VDS initially  
- later extracted into shared substrate capabilities  

---

# 15. Mental Model

VDS is:

- a caregiver, not a controller  
- an observer, not an enforcer  
- a translator between decisions and reality  

It answers:

> “Given what we said matters… what is happening?”

---

# 16. Final Principle

VDS ensures that:

- behavior is visible relative to intent  
- misalignment is surfaced without blame  
- systems remain autonomous  
- decisions remain the source of meaning  

It turns:

> telemetry → interpretation → understanding  

without ever becoming a source of authority.