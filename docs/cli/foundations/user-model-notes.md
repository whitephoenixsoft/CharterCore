# Charter CLI — User Model Notes  

**Status:** Informational / Contributor Guidance  
**Purpose:** Provide mental-model framing for contributors, maintainers, and future self.

---

## 1. Core Principle

Charter must survive **the strictest possible scrutiny**, even if users never interact with it.  
Three archetypes guide this design:

1. **Scientists** — care about reproducibility, correctness, and defensibility.  
2. **Monks** — care about historical continuity, canonical preservation, and lineage.  
3. **Developers-as-converted-scientists** — care about explainability, auditability, and reasoning clarity.

---

## 2. Mental Models

### A. Lab Notebook (Scientist)

- Breakouts → Experiments  
- Synthesis → Analysis  
- Proposals → Hypotheses  
- Sessions → Peer review  
- Acceptance → Published result  
- Audit → Methodology / provenance  

**Goal:** Someone else can reproduce and validate the outcome.

---

### B. Canon Formation (Monk)

- Draft → Commentary  
- Breakouts → Study groups  
- Synthesis → Council prep  
- Proposals → Canon candidates  
- Sessions → Council deliberation  
- Acceptance → Canonization  
- Supersession → Later council update  

**Goal:** Historical continuity survives centuries; nothing is lost or overwritten.

---

### C. Developer as Converted Scientist

- Breakouts → Spike or RFC discussion  
- Synthesis → Design doc  
- Proposals → ADRs  
- Sessions → Review meetings  
- Acceptance → Merge / Decision  
- Audit → Git + ADR history  

**Goal:** Decisions are explainable, defensible, and auditable.

---

## 3. Acceptance-Test Lens (for future library)

**Three Non-Negotiable Questions**

1. **Reproducibility** — Can someone reconstruct the decision path?  
2. **Defensibility** — Can someone fairly challenge it?  
3. **Continuity** — Can it survive authors leaving or institutional change?  

Failing any of these indicates a **CLI invariant violation** or a **design flaw**.

---

## 4. Contributor Guidance

- Always assume **extreme scrutiny**.  
- CLI conservatism is intentional; never shortcut authority or legitimacy.  
- Audit is the system of record; everything else prepares, never decides.  
- Treat these mental models as **living guidelines**, not hard UX requirements.