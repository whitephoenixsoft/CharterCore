# Charter Core — CLI Simulation (Single-User Mode)

> This simulation demonstrates Charter Core used as a **local decision ledger** by a single developer. All legitimacy rules still apply.

## Assumptions (Implicit, but Important)

- User is operating locally
- No remote sync
- No AI
- No permissions system exposed yet
- Single-user bootstrap authority is allowed **only for initialization**

## 1. Create an Area

```Bash
charter area create "Backend Architecture"
```
### Output 
```Text
Area created:
  ID: A-1
  Name: Backend Architecture

This Area has no Authority or Scope.
Initialization session required.
```

## 2. Initialize Area (Authority + Scope)

Charter **forces** an initialization session.
```Bash
charter session start --area A-1 --init
```
### Output 
```Text
Session S-1 started (Initialization)
Problem:
  Define Authority and Scope for Area A-1
```

