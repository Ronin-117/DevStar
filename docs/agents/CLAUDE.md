# DevStar — Claude Code Integration Guide

## Quick Setup

Add the MCP server to your Claude Code settings:

```json
{
  "mcpServers": {
    "devstar": {
      "command": "cargo",
      "args": ["run", "--bin", "devstar-mcp"],
      "cwd": "/path/to/ProjectTracker/src-tauri"
    }
  }
}
```

Or after building:

```json
{
  "mcpServers": {
    "devstar": {
      "command": "/path/to/ProjectTracker/src-tauri/target/debug/devstar-mcp"
    }
  }
}
```

## Prerequisites

1. The DevStar Tauri app must have been run at least once (to create the database)
2. Rust toolchain installed (`cargo` available)
3. Database located at `~/.local/share/com.njne2.devstar/devstar.db` (Linux) or `%APPDATA%\com.njne2.devstar\devstar.db` (Windows)

## Available Tools

See [AGENTS.md](./AGENTS.md) for the complete tool reference.

## Common Agent Workflows

### Planning a New Project

```
1. Call list_templates → browse available templates
2. Call get_template with template_id → review sprints and sections
3. Call create_project with name + template_id → instantiate project
4. Call get_project with project_id → confirm setup
```

### Working Through a Sprint

```
1. Call get_active_sprint with project_id → see current tasks
2. For each item, call update_item with checked: true → mark complete
3. When all done, call complete_sprint → auto-advances to next sprint
4. Repeat until project complete
```

### Logging Issues

```
1. Call get_active_sprint → find current sprint_id
2. Call log_error with project_id, sprint_id, and error message
3. Error appears as unchecked todo in "Agent Errors" section
4. Fix the issue, then call update_item to check it off
```

### Tracking Progress

```
1. Call get_progress with project_id → get checked/total/percentage
2. Call get_project for detailed breakdown by sprint and section
```

## Tips

- **Always check the active sprint first** before making changes
- **Use log_error** for any blockers — they become visible todos for the user
- **complete_sprint** is safer than set_sprint_status because it marks all items done first
- **Templates are read-only** — you can browse them but not modify them
- **Projects you create are yours** — agent identity is tracked automatically
