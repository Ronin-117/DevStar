# DevStar MCP — Agent Prompt Scenarios

This document contains ready-to-paste prompts for AI agents (Claude Code, Gemini CLI, or any
MCP-capable agent) covering every common workflow in DevStar. Copy the relevant scenario and
paste it as your opening message.

---

## Before You Start

### Gemini CLI config (`~/.gemini/settings.json`)

```json
{
  "mcpServers": {
    "devstar": {
      "command": "devstar-mcp",
      "args": [],
      "env": {}
    }
  }
}
```

### Claude Code config (`~/.claude/mcp_servers.json`)

```json
{
  "devstar": {
    "command": "devstar-mcp",
    "args": []
  }
}
```

---

## Scenario 1 — New Session: Plan a Project From Scratch

**When to use:** Starting a brand new project. No `.devstar.json` exists yet in the directory.

**Goal:** Understand the idea, browse templates, create the project, and populate a thorough
sprint plan that covers every sprint and section — not just the first one.

```
You have access to the DevStar MCP tool (server name: "devstar").

Your goal is to help me plan a new software project inside DevStar. Do NOT write any code yet.
This session is planning-only.

## Step 1 — Understand my project idea
Before touching any tools, ask me:
- What are we building? (one sentence)
- What is the rough scope? (tiny side project / medium app / large system)
- Any specific requirements or constraints I should know about?

Wait for my answers before proceeding.

## Step 2 — Browse and choose a template
After I answer, call `devstar:list_templates` to see what's available.
Call `devstar:get_template` on the 2-3 templates that seem most relevant.
Read the FULL output of each — every sprint, every section, every item.

Recommend one template to me with a short explanation of why it fits. Wait for my approval or
redirect before creating anything.

## Step 3 — Create the project
Once I approve, call `devstar:create_project` with:
- name: (agreed project name)
- template_id: (approved template id)
- description: (one sentence summary)
- project_dir: "." (current directory)

## Step 4 — Review the full generated plan
Call `devstar:get_full_project_plan` immediately after creation.
Read the ENTIRE output carefully — every sprint, every section, every item that was copied from
the template.

Tell me:
- How many sprints were created
- What each sprint is called and roughly what it covers
- Which sprint will be active (Sprint 1)

## Step 5 — Collaborative planning: fill ALL sprints
Now go through EVERY sprint one by one (not just Sprint 1). For each sprint:

1. Call `devstar:get_sprint` with that sprint's number to see its current sections and tasks.
2. Ask me: "Sprint N is called [name]. It currently has [X] sections covering [brief summary].
   What project-specific tasks should I add here that aren't already covered?"
3. Wait for my answer before adding anything.
4. For each thing I mention, add it using `devstar:add_task` with:
   - `title`: the task title
   - `section_name`: copied EXACTLY from the existing section name shown by get_sprint
     (do NOT create new sections unless I explicitly ask you to — reuse existing ones)

Important rules:
- Never add a task to the first section and then stop. Go through ALL sections in the sprint.
- Never create duplicate sections. Always check the section names from get_sprint first.
- If I say "nothing extra for this sprint", move on to the next sprint.
- Keep going until we've reviewed every sprint.

## Step 6 — Final confirmation
After all sprints are populated, call `devstar:get_full_project_plan` one more time.
Print a summary:
- Project name and UUID (from .devstar.json)
- Total sprints, total tasks
- Sprint-by-sprint breakdown: name, section count, task count
- Any sprints that still have 0 project-specific tasks (flag these for my attention)

Do not start implementing anything.
```

---

## Scenario 2 — Same Session: Implement and Check Off Tasks

**When to use:** Immediately after Scenario 1 (or starting fresh after planning is done).
The project exists and Sprint 1 is active. Time to build.

```
You have access to the DevStar MCP tool (server name: "devstar").
The project plan is already created. Now implement it.

## Before writing a single line of code
1. Call `devstar:get_full_project_plan` to see the complete current state of ALL sprints.
2. Call `devstar:get_tasks` with status "pending" to get the exact task list for the active sprint.
3. Tell me: which sprint is active, how many tasks are pending, and list them grouped by section.

Wait for my go-ahead before starting implementation.

## Implementation rules (follow these strictly)
- Work through tasks in section order, one at a time.
- After completing each task (writing the code, creating the file, etc.):
  → Call `devstar:check_task` with the task title (partial match is fine) BEFORE moving on.
- Only check off tasks you have actually completed. Do not check off things speculatively.
- If you hit a genuine blocker or unrecoverable error:
  → Call `devstar:log_error` with a clear description, then continue to the next task if possible.
  → Use log_error sparingly — only for real blockers, not minor issues you can solve yourself.
- If you realize a task is missing that should exist:
  → Call `devstar:add_task` to add it, then implement and check it off.

## After each section is finished
- Call `devstar:get_tasks` with status "pending" again to confirm nothing is missed in that section.

## After the sprint is done
- Call `devstar:get_progress` to show completion percentage.
- Call `devstar:get_full_project_plan` to show me what sprint is now active.
- Ask me: "Sprint 1 is complete. Sprint 2 is now active — shall I continue into it?"

## Final step
After all work in this session is done:
- Call `devstar:get_progress` for final stats.
- Show me a brief summary: what was built, what files were created, any errors logged.
```

---

## Scenario 3 — Same Session: Continue Work After a Break

**When to use:** You opened the same agent session (e.g. same Claude Code project or Gemini
`--resume` flag) but some time has passed and you need to re-orient before continuing.

```
You have access to the DevStar MCP tool (server name: "devstar").

I've been working in this session before. Let's pick up where we left off.

## Step 1 — Get full current state
Call `devstar:get_full_project_plan` (no arguments — use the scoped project from .devstar.json).

Read it carefully:
- What sprint is currently active?
- In the active sprint: which tasks are checked (done) and which are pending?
- Are there any sections in the active sprint with zero checked tasks?

## Step 2 — Report to me before doing anything
Tell me:
- Active sprint name and number
- Tasks completed so far in this sprint (list them)
- Tasks still pending in this sprint (list them, grouped by section)
- Overall project progress (X of Y tasks, percentage)

Wait for my confirmation before continuing.

## Step 3 — Resume work
Call `devstar:get_tasks` with status "pending" to get the exact pending list.
Continue from where we left off:
- Work through pending tasks in order
- Call `devstar:check_task` after each completed task
- Call `devstar:log_error` only for genuine blockers

## Step 4 — When the sprint completes
If all tasks in the active sprint are checked off, the sprint auto-advances.
Call `devstar:get_full_project_plan` to see the new state and report it to me.
Ask whether to continue into the next sprint.
```

---

## Scenario 4 — New Session: Resume an Existing Project

**When to use:** Starting a completely fresh agent session (new terminal window, new Claude Code
project) in a directory that already has a `.devstar.json` from a previous session.

```
You have access to the DevStar MCP tool (server name: "devstar").

I'm starting a new session to resume work on an existing project.

## Step 1 — Reconnect to the project
Read the file `.devstar.json` in the current directory.
Extract the `project_uuid` field from it.

Call `devstar:initialize` with that UUID:
{
  "project_uuid": "<uuid from .devstar.json>"
}

This scopes your session to this project — you won't need to pass project_id to any other tool.

## Step 2 — Full state assessment
Call `devstar:get_full_project_plan` immediately after initializing.

Report to me:
- Project name and UUID
- Overall progress: X of Y tasks done (percentage)
- Sprint breakdown: which are done, which is active, which are pending
- Active sprint: list every section with its checked/total count
- Any sections with 0 checked tasks

## Step 3 — Deep dive into active sprint
Call `devstar:get_tasks` with status "done" — list completed tasks.
Call `devstar:get_tasks` with status "pending" — list what's still left.

Group the pending tasks by section name and show them to me.

## Step 4 — Wait for my direction
Do NOT start implementing anything until I say so.
Ask me: "I can see [N] pending tasks in Sprint [X]. Shall I continue from here, or is there
something specific you want me to focus on?"

## Step 5 — Resume on my signal
Once I say go:
- Work through pending tasks in section order
- Call `devstar:check_task` after each completed task
- Call `devstar:log_error` for genuine blockers only
- Call `devstar:get_progress` after every section to show running totals
- If the sprint completes, report it and ask before moving to the next sprint

Always: read .devstar.json first — never guess the project_uuid.
```

---

## Scenario 5 — New Session: Add Tasks to a Specific Sprint

**When to use:** The project exists and you want to add more tasks to a specific sprint without
implementing anything yet. Useful when requirements change mid-project.

```
You have access to the DevStar MCP tool (server name: "devstar").

I want to add new tasks to an existing project's sprint plan. No implementation.

## Step 1 — Reconnect
Read `.devstar.json` and call `devstar:initialize` with the project_uuid.

## Step 2 — See the full plan
Call `devstar:get_full_project_plan` and show me the list of all sprints (number, name, status,
task count). Don't show me all the individual tasks yet — just the sprint overview.

## Step 3 — Which sprint?
Ask me: "Which sprint do you want to add tasks to? (give me a number or name)"
Wait for my answer.

## Step 4 — Show current state of that sprint
Call `devstar:get_sprint` with the sprint number or name I gave.
Show me every section and every existing task in that sprint.

## Step 5 — Discuss what to add
Ask me: "What new tasks or sections do you want to add here?"
Have a back-and-forth conversation if needed until we've agreed on exactly what to add.

## Step 6 — Add tasks precisely
For each agreed task:
- Call `devstar:add_task` with:
  - `title`: exact task title
  - `section_name`: copied EXACTLY from the existing section name (get it from the get_sprint output)
- If adding to a NEW section we agreed on, call `devstar:add_section` first, then add_task with that section name.

NEVER create a new section unless we explicitly agreed on it.
NEVER guess at section names — copy them verbatim from the get_sprint response.

## Step 7 — Verify
Call `devstar:get_sprint` one more time for that sprint to confirm everything looks correct.
Show me the updated section/task list.
```

---

## Scenario 6 — New Session: Project Health Check

**When to use:** You want a full overview of where the project stands without doing any work.
Good for team standups or when handing off to another agent.

```
You have access to the DevStar MCP tool (server name: "devstar").

Run a complete health check on this project and give me a status report.

## Step 1 — Connect
Read `.devstar.json` and call `devstar:initialize` with the project_uuid.

## Step 2 — Full plan
Call `devstar:get_full_project_plan`.

## Step 3 — Compile and present this report

### Project Overview
- Project name
- Total progress: X of Y tasks (Z%)

### Sprint Summary (table format)
| Sprint | Name | Status | Progress |
|--------|------|--------|----------|
| 1 | ... | done | 12/12 |
| 2 | ... | active | 5/14 |
| 3 | ... | pending | 0/18 |

### Active Sprint Detail
- Sprint name and number
- For each section: name, checked/total, list of pending tasks

### Logged Errors
- Call `devstar:search_tasks` with query "error" to find any logged errors in the "Agent Errors" section.
- List any unresolved errors with their titles.

### Recommendations
Based on the data, tell me:
- Are there sections with zero progress that might be stale or forgotten?
- Is the current sprint close to completion?
- Anything that looks off or worth my attention?

Do not make any changes to the project.
```

---

## Scenario 7 — New Session: Fix a Logged Error

**When to use:** You or a previous agent logged an error using `log_error` and you want to find
and resolve it.

```
You have access to the DevStar MCP tool (server name: "devstar").

I need to find and fix errors that were logged in a previous session.

## Step 1 — Connect
Read `.devstar.json` and call `devstar:initialize` with the project_uuid.

## Step 2 — Find logged errors
Call `devstar:search_tasks` with query "error" or "failed" or "blocker".
Also look for any section named "Agent Errors" in the active sprint by calling
`devstar:get_active_sprint_detail`.

List all unchecked items in "Agent Errors" sections across all sprints.

## Step 3 — Report to me
For each logged error:
- The exact error title/description
- Which sprint and section it's in
- Whether it's still unchecked (unresolved)

Ask me which ones to tackle.

## Step 4 — Fix and check off
For each error I ask you to fix:
- Investigate and fix it (look at the relevant code, run the relevant command, etc.)
- Call `devstar:check_task` with the error title once resolved.

Do not bulk-check errors without actually fixing them.
```

---

## Scenario 8 — New Session: Sprint Review and Advance

**When to use:** All tasks in the active sprint appear to be done and you want to formally close
it and see what's next.

```
You have access to the DevStar MCP tool (server name: "devstar").

I think the current sprint might be done. Let's review it and advance if appropriate.

## Step 1 — Connect
Read `.devstar.json` and call `devstar:initialize` with the project_uuid.

## Step 2 — Active sprint audit
Call `devstar:get_tasks` with status "pending".

If there are NO pending tasks:
- Tell me "Active sprint has 0 pending tasks — it's complete."
- Ask me: "Should I mark it as done and activate the next sprint?"
- Wait for my answer.

If there ARE pending tasks:
- List them grouped by section.
- Ask me: "There are still [N] pending tasks. Do you want to complete these before advancing, or
  skip them and advance anyway?"
- Wait for my answer.

## Step 3 — Advance (only with my approval)
If I say yes to advancing:
- Call `devstar:complete_sprint` to mark the sprint done and activate the next one.
- Call `devstar:get_full_project_plan` to show the new state.
- Tell me what the new active sprint is called and list its sections.

If I say to complete remaining tasks first, go back to Scenario 2 (implementation).
```

---

## Tips for All Scenarios

**Always use `get_full_project_plan` first in a new session** — it gives the complete picture in
one call. Don't call `get_sprint` for every sprint individually when the full plan is faster.

**Never guess section names** — always copy them verbatim from `get_sprint` or
`get_full_project_plan` output when passing `section_name` to `add_task`. Case-insensitive
matching is used, but exact names avoid any ambiguity.

**Don't stop at the first section** — when adding tasks during planning, go through EVERY
section in EVERY sprint. Ask the user for input per section. Only move on when they confirm
there's nothing to add to that section.

**Check off tasks as you go, not in batches** — call `check_task` immediately after completing
each individual task. This keeps the UI in sync and makes it easy to resume if interrupted.

**`log_error` is for real blockers only** — use it when you genuinely cannot proceed without
human intervention: compilation errors you can't fix, missing credentials, ambiguous requirements
that need a decision. Don't log minor issues you can resolve yourself.

**Session scoping** — if the session was initialized with a project_uuid, all tools work without
needing `project_id`. If not initialized, the tools fall back to reading `.devstar.json` from the
current directory automatically.