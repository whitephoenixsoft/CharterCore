# Charter — <Substrate Name> Integration Contract

Status: STABLE (WORKING)  
Purpose: Define how other substrates and Runtime interact with <Substrate Name>  

---

# 1. Overview

This document defines the integration surface of <Substrate Name>.

It describes:

- what it consumes  
- what it produces  
- how it is queried  
- how it participates in Runtime orchestration  

---

# 2. Inputs

This substrate consumes:

- <input source 1>  
- <input source 2>  

Include:

- data types  
- source substrates  
- required vs optional  

---

# 3. Outputs

This substrate produces:

- <output 1>  
- <output 2>  

Outputs may include:

- read surfaces  
- derived data  
- commit artifacts  

---

# 4. Commit Emission

If applicable, define commit types:

- <commit type 1>  
- <commit type 2>  

For each:

- purpose  
- authority level (authoritative / derived / non-authoritative)  

---

# 5. Query Integration (CQL)

Define how this substrate is queried:

- domain name  
- supported subjects (views, raw)  
- supported targets  

---

# 6. Runtime Integration

Define how Runtime interacts:

- lifecycle interactions  
- orchestration responsibilities  
- registration model (adapters, handlers)  

---

# 7. Persistence Expectations

Define:

- required storage  
- optional storage  
- derived vs authoritative data  

---

# 8. Federation Behavior

Define:

- what is shared externally  
- how it is transported (CCS)  
- what remains local  

---

# 9. Error Surface

Define:

- possible failure types  
- blocking vs non-blocking errors  

---

# 10. Determinism Guarantees

Define:

- what must be reproducible  
- what depends on external systems  

---

# 11. Constraints

This substrate must not:

- <constraint 1>  
- <constraint 2>  

---

# 12. Integration Notes

Additional notes for:

- Runtime  
- CLI  
- VDS  
- VLS