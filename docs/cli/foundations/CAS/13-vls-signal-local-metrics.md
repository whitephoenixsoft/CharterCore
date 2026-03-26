# Charter Alignment Engine — Signal & Local Metrics Specification (V1)

Status: DRAFT (FOUNDATIONAL)  
Applies to: Alignment Engine (V4+)  
Depends On: Charter DAG, VDS Signals, VLS DAG Reconstruction  
Does NOT define: Graph propagation, vector aggregation, tension fields, or index persistence  

---

# 1. Purpose

This specification defines the **lowest-level computational layer** of the Alignment Engine:

- Signal encoding (hybrid semantic + numeric)
- Local per-resolution metrics
- Rolling statistics
- Adaptive window inputs

This layer is:

- deterministic
- stateless (recomputable)
- backend-agnostic
- independent of higher-level field dynamics

It provides the foundation for:

- drift analysis
- variance detection
- adaptive windowing
- higher-order alignment dynamics

---

# 2. Core Principles

## AE-SLM-01 — Signals Are Observational Only
Signals:

- do not create legitimacy
- do not modify the DAG
- do not imply authority
- are append-only observations

---

## AE-SLM-02 — Hybrid Signal Model
Each signal MUST have:

- semantic representation (human meaning)
- numeric representation (computational value)

Neither replaces the other.

---

## AE-SLM-03 — Local Metrics Are Scalar
At the local (per-resolution) level:

- all primary metrics are scalar
- vector composition occurs in higher layers

---

## AE-SLM-04 — Metrics Are Windowed
All local metrics MUST be computed over a time window:

- fixed (initial implementation), or
- adaptive (preferred)

---

## AE-SLM-05 — Metrics Are Derived
Local metrics:

- are never persisted as authoritative truth
- MUST be recomputable from signals
- MAY be cached or indexed

---

# 3. Signal Model

## 3.1 Signal Structure

Each signal MUST include:

- resolution_id
- semantic_state
- confidence_level
- timestamp
- optional metadata

---

## 3.2 Semantic States (V1)

Allowed states:

- alignment
- misalignment
- uncertainty
- reduced_capacity
- pause

These represent **distinct semantic meanings**, even if numerically similar.

---

## 3.3 Numeric Mapping

Each semantic state maps to a scalar weight:

alignment        = +1.0  
uncertainty      = -0.5  
reduced_capacity = -0.7  
pause            = -0.4  
misalignment     = -1.0  

---

## 3.4 Confidence Multipliers

Signals include a confidence level:

low    = 0.5  
medium = 1.0  
high   = 1.5  

---

## 3.5 Effective Signal Value

Each signal is converted into a numeric value:

effective_signal = signal_weight × confidence_multiplier

Examples:

- alignment (high) → +1.5  
- misalignment (low) → -0.5  
- reduced_capacity (medium) → -0.7  

---

## 3.6 Signal Stream

Signals form an append-only stream:

SignalStream(resolution_id) = ordered list of signals by timestamp

---

# 4. Local Metric State

For each resolution, the engine MUST maintain (conceptually or via cache):

- count         (N)
- sum_x         (Σ x)
- sum_x2        (Σ x²)
- last_signal_time

Derived:

- mean
- variance
- signal_density

---

# 5. Rolling Statistics

## 5.1 Mean (Drift)

mean_drift = sum_x / N

Interpretation:

- > 0   → alignment tendency  
- ≈ 0   → mixed / unclear  
- < 0   → misalignment tendency  

---

## 5.2 Variance

variance = (sum_x2 / N) − (mean_drift)²

Interpretation:

- low variance → stable state  
- high variance → instability / disagreement  

Important:

- low variance + negative mean = stable misalignment  
- high variance + near-zero mean = oscillation / tension  

---

## 5.3 Signal Density

signal_density = N / window_duration

Used for:

- adaptive windowing
- pressure indicators
- activity measurement

---

## 5.4 Last Signal Time

Tracks recency:

last_signal_time = timestamp of most recent signal

Used for:

- silence detection
- adaptive expansion

---

# 6. Adaptive Window Inputs

Each resolution’s window size MAY be dynamically adjusted based on:

- variance
- signal_density
- time_since_last_signal
- confidence distribution

Conceptual function:

window_size = f(variance, density, silence)

Guidelines:

- higher variance → smaller window  
- higher density → smaller window  
- longer silence → larger window  

---

# 7. Interpretation Bands (Initial Calibration)

## 7.1 Mean Drift

mean > 0.2     → aligned tendency  
-0.2 to 0.2    → mixed / neutral  
mean < -0.2    → misaligned tendency  

---

## 7.2 Variance

variance < 0.30   → stable  
0.30–0.49         → mixed / active  
≥ 0.50            → unstable  

These thresholds are:

- empirical starting points
- subject to tuning
- not normative or authoritative

---

# 8. Separation of Concerns

This layer MUST NOT:

- perform graph traversal
- aggregate across resolutions
- compute intent gravity
- compute tension fields
- compute alignment cones

Those belong to higher layers.

---

# 9. Output Contract

For each resolution, this layer produces:

- mean_drift
- variance
- signal_density
- last_signal_time

These outputs are consumed by:

- propagation layer
- alignment dynamics layer
- indexing layer

---

# 10. Determinism

Given identical:

- signal stream
- window definition

The outputs MUST be identical.

No randomness or heuristic mutation is allowed.

---

# 11. Failure Modes

Fail if:

- signals are missing required fields  
- confidence is not applied  
- variance is computed inconsistently  
- window boundaries are ambiguous  
- metrics differ across identical inputs  

---

# 12. Mental Model

Signals are measurements  
Mean is direction  
Variance is stability  
Density is activity  
Windows define context  

This layer does not decide meaning.

It measures conditions.

---

# 13. Why This Matters

Without this layer:

- higher-level metrics become unstable  
- propagation becomes inconsistent  
- alignment analysis becomes subjective  

With it:

- all higher dynamics remain grounded in deterministic computation  
- the system scales without losing clarity  
- semantics remain separate from math  

---

# 14. Final Constraint

This layer is:

- simple
- mechanical
- verifiable

All complexity must emerge above it — never within it.