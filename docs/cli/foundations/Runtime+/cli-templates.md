# Charter CLI — Templates (Participant Groups & Draft Candidates)

Status: FOUNDATIONAL (Runtime Convenience Layer)  
Applies to: Charter CLI Runtime  
Does NOT apply to: Engine legitimacy semantics, CCE, receipts, or canonical history  

---

## 1. Purpose

This document defines **templates** in Charter CLI:

- Participant Groups  
- Draft Candidate Sets  

These constructs exist to:

- reduce repetitive input  
- improve workflow ergonomics  
- support repeated collaboration patterns  

They are explicitly **non-legitimizing**.

---

## 2. Core Principle

Templates are **reusable convenience structures**, not governance artifacts.

> Templates are not session state.  
> Templates only become relevant when explicitly applied within a bounded workflow or session.

---

## 3. Participant Groups

### 3.1 Definition

A Participant Group is a reusable collection of actor identifiers.

Example use cases:

- recurring project teams  
- committees  
- working groups  

---

### 3.2 Properties

Participant Groups are:

- mutable  
- reusable  
- CLI-managed  
- external to sessions  

They are not:

- evidence of participation  
- authority definitions  
- session-bound artifacts  

---

### 3.3 Application

When applied:

- the group is expanded into explicit participants in a session  
- participants become part of session state  
- authority is evaluated based on the explicit session participant set  

The group itself is not referenced by the engine.

---

### 3.4 Invariants

- Group membership does not imply presence  
- Group membership does not imply consent  
- Group membership does not imply authority  
- Only explicitly declared session participants are considered in legitimacy  

---

## 4. Draft Candidate Sets

### 4.1 Definition

A Draft Candidate Set is a reusable collection of candidate content.

Example use cases:

- approved software stacks  
- standard policy options  
- recurring decision templates  

---

### 4.2 Properties

Draft Candidate Sets are:

- mutable  
- reusable  
- pre-legitimacy artifacts  
- CLI-managed  

They are not:

- proposals  
- candidates within a session  
- legitimacy-bearing objects  

---

### 4.3 Application

When applied:

- draft items are copied into a session or workflow context  
- they become candidates only after explicit inclusion  
- they are evaluated under session authority rules  

The draft set itself is not referenced by the engine.

---

### 4.4 Invariants

- Draft sets do not imply endorsement  
- Draft sets do not imply readiness  
- Draft sets do not imply legitimacy  
- Only explicitly added candidates are evaluated in sessions  

---

## 5. Architectural Placement

Templates exist in the **Runtime Convenience Layer**.

They must not:

- be stored in the Engine object model  
- appear in CCE exports  
- appear in receipts  
- affect legitimacy computation  
- influence restore semantics  

They may:

- be stored locally  
- be versioned or exported as convenience data (future feature)  
- be modified without affecting historical legitimacy  

---

## 6. Relationship to Sessions

Sessions operate only on:

- explicit participant sets  
- explicit candidate sets  

Templates must be explicitly applied before use.

No implicit linkage between templates and sessions is permitted.

---

## 7. Design Constraints

If templates ever:

- imply authority  
- imply participation  
- imply endorsement  
- bypass explicit session construction  
- appear in legitimacy artifacts  

then the system has violated its separation of concerns.

---

## 8. Mental Model

Templates are:

- shortcuts for humans  
- reusable inputs  
- workflow accelerators  

They are not:

- decisions  
- commitments  
- evidence  
- governance  

---

## 9. Final Principle

Convenience must never become authority.

Templates reduce repetition.  
Sessions create legitimacy.