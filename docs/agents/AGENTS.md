# DevStar MCP Server — Agent Reference

## Overview

The DevStar MCP server exposes project planning tools to AI agents via JSON-RPC over stdio. Agents can browse templates, create projects, track sprint progress, check off items, and log errors — all through the shared SQLite database.

## Connection

The server is a binary (`devstar-mcp`) that runs on stdio. Agents connect by spawning the process and communicating via JSON-RPC 2.0.

### MCP Config

```json
{
  "mcpServers": {
    "devstar": {
      "command": "devstar-mcp",
      "args": []
    }
  }
}
```

## Tools

### Read-Only Tools

#### `list_templates`
List all available templates with sprint counts.

**Input:** `{}`

**Output:**
```json
{
  "templates": [
    { "id": 1, "name": "Full-Stack Web App", "description": "...", "color": "#3b82f6", "sprint_count": 12 }
  ]
}
```

#### `get_template`
Get a template's full sprint and section hierarchy.

**Input:** `{ "template_id": 1 }`

**Output:**
```json
{
  "template": { "id": 1, "name": "...", "description": "...", "color": "..." },
  "sprints": [
    {
      "id": 1, "name": "Planning & Setup", "description": "...", "sort_order": 0,
      "section_count": 2,
      "sections": [
        { "id": 1, "name": "Project Planning & Scoping", "description": "...", "color": "#6366f1", "is_linked": true }
      ]
    }
  ]
}
```

#### `list_shared_sections`
List all reusable shared sections.

**Input:** `{}`

**Output:** `{ "sections": [{ "id": 1, "name": "...", "description": "...", "color": "...", "item_count": 10 }] }`

#### `list_shared_sprints`
List all reusable shared sprints.

**Input:** `{}`

**Output:** `{ "sprints": [{ "id": 1, "name": "...", "description": "...", "section_count": 2 }] }`

#### `get_project`
Get full project details with sprints, sections, and progress.

**Input:** `{ "project_id": 1 }`

**Output:**
```json
{
  "project": { "id": 1, "name": "...", "description": "...", "template_id": 1, "color": "..." },
  "sprints": [
    {
      "id": 1, "name": "...", "description": "...", "status": "active", "sort_order": 0,
      "section_count": 2,
      "sections": [
        { "id": 1, "name": "...", "description": "...", "is_custom": false, "linked_from_section_id": 1, "item_count": 10, "checked_count": 5 }
      ]
    }
  ],
  "progress": { "checked": 45, "total": 120 }
}
```

#### `get_active_sprint`
Get the current active sprint with all sections and items.

**Input:** `{ "project_id": 1 }`

**Output:**
```json
{
  "sprint": { "id": 2, "name": "...", "description": "...", "sort_order": 1 },
  "sections": [
    {
      "id": 5, "name": "...", "description": "...", "is_custom": false,
      "items": [
        { "id": 20, "title": "...", "description": "...", "checked": false, "notes": "" }
      ]
    }
  ]
}
```

#### `get_progress`
Get completion percentage for a project.

**Input:** `{ "project_id": 1 }`

**Output:** `{ "checked": 45, "total": 120, "percentage": 38 }`

### Write Tools

#### `create_project`
Create a new project from a template. Copies all sprints, sections, and items.
If `project_dir` is provided, writes a `.devstar.json` config file to that directory so any agent can auto-discover the project.

**Input:**
```json
{
  "name": "My API Project",
  "template_id": 6,
  "description": "Optional description",
  "color": "#06b6d4",
  "project_dir": "/home/user/work/my-api"
}
```

**Output:** `{ "project_id": 5, "name": "My API Project" }`

#### `get_project_context`
Read `.devstar.json` from the current or specified directory and return full project details including the active sprint. This is the primary entry point for agents — call this first when starting work in a project directory.

**Input:** `{ "project_dir": "." }` (or omit for current directory)

**Output:**
```json
{
  "project_dir": ".",
  "project_id": 5,
  "project": { "id": 5, "name": "My API", ... },
  "sprints": [...],
  "progress": { "checked": 45, "total": 120, "percentage": 38 },
  "active_sprint": { "id": 2, "name": "...", ... },
  "active_sprint_sections": [
    { "id": 5, "name": "...", "items": [{ "id": 20, "title": "...", "checked": false }] }
  ]
}
```

#### `update_item`
Check/uncheck an item or add notes.

**Input:**
```json
{
  "item_id": 42,
  "checked": true,
  "notes": "Completed on 2025-04-03"
}
```

**Output:** `{ "ok": true, "item_id": 42 }`

#### `add_item`
Add a custom checklist item to a section.

**Input:**
```json
{
  "section_id": 10,
  "title": "Custom task",
  "description": "Optional description"
}
```

**Output:** `{ "ok": true, "item_id": 150 }`

#### `set_sprint_status`
Set a sprint's status manually.

**Input:** `{ "sprint_id": 3, "status": "active" }`

**Output:** `{ "ok": true, "sprint_id": 3, "status": "active" }`

#### `complete_sprint`
Mark all items in a sprint as done, mark the sprint as done, and activate the next sprint.

**Input:** `{ "sprint_id": 2, "project_id": 1 }`

**Output:** `{ "ok": true, "next_sprint_id": 3 }`

#### `log_error`
Log an error as an unchecked todo item. Creates an "Agent Errors" section if it doesn't exist.

**Input:**
```json
{
  "project_id": 1,
  "sprint_id": 2,
  "error": "Failed to compile: missing dependency"
}
```

**Output:** `{ "ok": true, "item_id": 200, "section_id": 50 }`

## Project Discovery

When an agent starts working in a project directory, it should call `get_project_context` first. This reads the `.devstar.json` file in the directory and returns everything the agent needs: project details, all sprints, the active sprint with its sections and items, and current progress.

**Creating a project with auto-discovery:**
```
create_project({ name: "My API", template_id: 6, project_dir: "/home/user/work/my-api" })
```
This creates the project AND writes `.devstar.json` to `/home/user/work/my-api`.

**Any agent can then discover it:**
```
get_project_context({ project_dir: "/home/user/work/my-api" })
// or simply:
get_project_context({})  // uses current directory
```

## Sprint Lifecycle

```
pending ──(set_sprint_status or complete_sprint)──► active ──(complete_sprint or all items checked)──► done
```

- First sprint starts as `active`, rest are `pending`
- When all items in the active sprint are checked, it auto-advances
- Agents can manually `set_sprint_status` or `complete_sprint`

## Security Model

| Concern | Solution |
|---|---|
| Template corruption | Templates are read-only for MCP clients |
| DB corruption | SQLite WAL mode, transactions |
| Rate limiting | Built into the Rust backend |
| Project isolation | Agents only know projects they create or discover via `.devstar.json` |

## Agent Workflow Example

### Starting a New Project
1. **Browse templates**: Call `list_templates` to see available options
2. **Inspect a template**: Call `get_template` with a template_id to see its sprints and sections
3. **Create a project**: Call `create_project` with name, template_id, and project_dir
4. **Auto-discovery**: The `.devstar.json` file is written — any agent can now find this project

### Working on an Existing Project
1. **Discover project**: Call `get_project_context` — returns everything in one call
2. **See active sprint**: The response includes the active sprint with all sections and items
3. **Work through items**: Call `update_item` to check off completed tasks
4. **Log errors**: Call `log_error` when encountering issues
5. **Complete sprint**: Call `complete_sprint` when all items are done
6. **Continue**: Call `get_project_context` again to get the new active sprint
7. **Track progress**: Call `get_progress` to see overall completion
