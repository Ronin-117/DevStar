# DevStar MCP — 5-Minute Quick Start

> For full agent prompt scenarios and copy-paste prompts for every workflow, see
> **[docs/agents/PROMPTS.md](./PROMPTS.md)**

## What is this?

DevStar has an MCP server that lets AI agents (Claude Code, OpenCode, Antigravity, etc.) interact with your project plans. Agents can browse templates, create projects, track sprint progress, and log errors — all through your DevStar database.

## Setup (3 steps)

### 1. Build the MCP Server

```bash
cd src-tauri
cargo build --bin devstar-mcp
```

The binary will be at `src-tauri/target/debug/devstar-mcp` (or `release/devstar-mcp`).

### 2. Run DevStar Once

Open the DevStar desktop app at least once. This creates the SQLite database that the MCP server connects to.

### 3. Configure Your Agent

Add to your agent's MCP config:

```json
{
  "mcpServers": {
    "devstar": {
      "command": "/absolute/path/to/src-tauri/target/debug/devstar-mcp"
    }
  }
}
```

Replace the path with the actual path to the built binary.

## Test It

Ask your agent: "List the available templates in DevStar"

The agent should call `list_templates` and return the 12 available project templates.

## What Agents Can Do

| Action | Tool |
|---|---|
| Browse templates | `list_templates`, `get_template` |
| Create projects | `create_project` (writes `.devstar.json` for auto-discovery) |
| Discover project | `get_project_context` (reads `.devstar.json` from current dir) |
| View progress | `get_project`, `get_progress`, `get_active_sprint` |
| Check off items | `update_item` |
| Add custom tasks | `add_item` |
| Complete sprints | `complete_sprint` |
| Log errors | `log_error` |

## What Agents Cannot Do

- Modify templates (read-only)
- Delete projects or items
- Modify shared sections or sprints
- Access projects without a `.devstar.json` in the working directory

## How Project Discovery Works

1. **Create**: Agent calls `create_project` with `project_dir` → writes `.devstar.json` to that directory
2. **Discover**: Any agent in that directory calls `get_project_context` → reads `.devstar.json` → returns full project state
3. **Work**: Agent has everything it needs — project details, active sprint, sections, items, progress

No human needs to tell the agent the project ID. The `.devstar.json` file is the bridge.

## Troubleshooting

**"Database not found"** — Run the DevStar app at least first to create the DB.

**Agent can't connect** — Make sure the path to `devstar-mcp` is absolute and the binary exists.

**No templates showing** — The DB may have been wiped. Restart the DevStar app to re-seed.

**"No .devstar.json found"** — Create a project first with `create_project` including the `project_dir` parameter.
