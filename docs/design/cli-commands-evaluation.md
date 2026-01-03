Good, this is a real ergonomics question — not naming bikeshedding. I’ll stay disciplined and only analyze lines starting with charter, from the standpoint of emotional friction, first-use intuition, and “what breaks if they don’t read docs.”
I’ll structure this as:
Global first-impression ergonomics
Command-by-command friction analysis (grouped)
Where frustration will arise
Concrete ergonomic recommendations (without redesigning your model)
1. Global First-Impression Ergonomics
Overall Impression (Good News)
If someone has Git-level literacy, this CLI feels:
familiar
serious
intentionally constrained
not “cute”
That’s a feature, not a bug, for your target audience.
Emotionally, it signals:
“This tool respects you. It will not guess.”
That aligns perfectly with Charter’s philosophy.
Immediate Emotional Risk
Without docs, a new user will feel one of two things very quickly:
🟢 “This is precise and trustworthy”
🔴 “I don’t know what state I’m in”
The biggest ergonomic risk is state opacity, not command names.
Git survives this because:
git status is obsessively good
errors are very explicit
Charter will need the same discipline.
2. Command-by-Command Ergonomic Analysis
Area Commands
Copy code

charter init
charter area create <name>
charter area use <area>
charter area status
Intuition: ⭐⭐⭐⭐☆
init is perfect.
area create is obvious.
area use is emotionally good — signals context switching.
area status is redundant but comforting.
Emotional response:
“Okay, I’m inside something.”
Potential confusion:
What happens if they don’t area use?
Is there a default area?
👉 Recommendation:
On charter status, always show current area prominently.
On charter init, auto-create a default area and tell them.
Session Commands
Copy code

charter session start
charter session status
charter session pause
charter session resume
charter session close
charter session restart-from <session-id>
Intuition: ⭐⭐⭐☆☆
This is where cognitive load increases.
Good:
start, status, close are excellent.
pause / resume feel humane.
Risk:
Users may not understand why sessions exist yet.
restart-from is powerful but intimidating.
Emotional reaction (first time):
“Wait… am I in a session right now?”
That’s not fatal — but it must be answered immediately by tooling.
👉 Recommendations:
charter session start should explain itself briefly on first use.
charter status must loudly say:
Copy code

Active session: yes/no
restart-from should be hidden from help output unless --advanced or similar.
Participants
Copy code

charter session participants show
charter session participants add <id>
charter session participants remove <id>
Intuition: ⭐⭐⭐⭐☆
Clear, explicit, boring — in a good way.
Emotional response:
“Okay, this is governance, not vibes.”
Risk:
What is <id>? Username? Email? Arbitrary string?
👉 Recommendation:
Error messages must explain what an ID represents.
Consider participants list instead of show for git symmetry.
Candidates
Copy code

charter candidate add
charter candidate list
charter candidate remove
Intuition: ⭐⭐⭐☆☆
This is one of the hardest conceptual leaps.
The word “candidate” is correct — but unfamiliar emotionally.
First-time user reaction:
“Candidate for… what?”
Without docs, this will confuse people briefly.
👉 Recommendations:
charter candidate add should prompt interactively with:
“Describe the proposed decision”
candidate remove being blocked after first stance is good friction — but error messaging must explain why, not just that it’s blocked.
Voting / Acceptance
Copy code

charter vote <candidate> accept|reject|abstain
charter session accept <candidate>
Intuition: ⭐⭐⭐⭐☆
This is strong.
Emotionally:
vote feels collective
session accept feels decisive
The solo-mode collapse is smart and merciful.
Risk:
Users may not know when to use which.
👉 Recommendation:
If authority is solo, charter vote should gently suggest:
“You can also use: charter session accept ”
Authority & Scope
Copy code

charter authority set
charter scope set
Intuition: ⭐⭐⭐☆☆ (but unavoidable)
These are heavy concepts, but the commands themselves are clean.
Emotional response:
“Oh. This matters.”
That’s good.
Risk:
Surprise sessions being created “under the hood” may feel spooky.
👉 Recommendation:
Always print:
Copy code

This action created Session S-123
Transparency reduces anxiety.
Import / Baseline
Copy code

charter import consolidate <file>
charter import restore <file>
charter baseline status
charter baseline show
charter baseline accept [--all] [--force]
charter baseline reject [--all]
charter baseline close
Intuition: ⭐⭐☆☆☆
This is the most expert-only surface.
That’s okay — but only if:
it never appears accidentally
errors are extremely explicit
Emotional risk:
“I think I broke something.”
👉 Recommendation:
Guard these behind:
explicit subcommand discovery
or very loud status messages
baseline should always explain what baseline means in output.
Status & Audit
Copy code

charter status
charter status session
charter status area
charter audit session
charter audit resolution
charter audit area
Intuition: ⭐⭐⭐⭐⭐
This is excellent.
If charter status becomes as good as git status, you win.
Emotional effect:
“I know where I am. I know what exists.”
This is where user trust will be built or lost.
3. Where Frustration Will Actually Come From
Not from command names.
From:
Not knowing what state they’re in
Not knowing what the next valid action is
Being blocked without understanding why
If those three are handled, users will tolerate everything else.
4. High-Impact Ergonomic Improvements (Low Philosophy Risk)
These do not change your model.
5. Make charter status Obsessively Helpful
It should always answer:
Current area
Active session (yes/no)
Pending candidates
Authority mode (solo / group)
What actions are currently valid
This reduces 80% of frustration.
6. Prefer Gentle Guidance Over Errors
Instead of:
“No active session.”
Say:
“No active session. Start one with: charter session start”
This is Git’s secret weapon.
7. First-Time UX Matters More Than Power UX
First successful flow should be:
Copy code

charter init
charter session start
charter candidate add
charter session accept
If that feels good emotionally, people will explore the rest.
Bottom Line
From an emotional ergonomics standpoint:
The command set is coherent
The names are honest
The friction is intentional and defensible
The biggest risk is state opacity, not complexity
You are designing something that respects cognition — but you’ll need:
excellent status output
humane error messages
zero magic
If you want, next we can:
simulate a first-time user walkthrough
or rewrite charter status output together (that’s the keystone)
Just say which.