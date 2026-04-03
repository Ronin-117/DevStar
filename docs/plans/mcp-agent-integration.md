# DevStar — MCP Server & Agent Integration Plan

> Created: 2026-04-01
> Status: Planned — waiting to start implementation

---

## Phase 1: Rename Everything to DevStar

**Files to update:**
- `src-tauri/tauri.conf.json` → `productName: "devstar"`, `identifier: "com.njne2.devstar"`
- `src-tauri/Cargo.toml` → `name = "devstar"`, binary name `devstar`
- `src/App.tsx` → "DevStar" in title/header
- `src/components/shared/TitleBar.tsx` → "DevStar"
- `src-tauri/src/lib.rs` → DB path `com.njne2.devstar`
- `index.html` → `<title>DevStar</title>`
- All window titles, comments, strings

---

## Phase 2: MCP Server (stdio transport)

### Architecture
- New binary `devstar-mcp` in `src-tauri/src/mcp_server.rs`
- Uses JSON-RPC over stdio (MCP protocol)
- Shares the same `Database` core — opens the same SQLite file
- **Read-only for templates**, **full access for projects**

### MCP Tools

| Tool | Description | Permissions |
|---|---|---|
| `list_templates` | List all available templates with section/item counts | Read-only |
| `get_template` | Get a template's sections and items | Read-only |
| `create_project` | Create a new project from a template | Full |
| `get_project` | Get project details, sections, items, and progress | Own projects only |
| `update_item` | Check/uncheck an item, add notes | Own projects only |
| `add_section` | Add a custom section to a project | Own projects only |
| `add_item` | Add a custom item to a section | Own projects only |
| `log_error` | Create "Agent Errors" section with error items as unchecked todos | Own projects only |
| `get_progress` | Get completion stats for a project | Read own projects |

---

## Phase 3: Agent Identity (auto-generated, no config)

### How it works
1. On first MCP connection, the server checks for `~/.devstar/agent_id`
2. If missing, generates a UUID and saves it
3. Every MCP call includes the agent ID automatically (no env var needed)
4. Agent ID is used for:
   - **Project ownership** — agents can only modify projects they created
   - **Error attribution** — error items show which agent logged them
   - **Audit trail** — DB tracks which agent made each change

### File
- `~/.devstar/agent_id` — contains a single UUID string
- `~/.devstar/settings.json` — user settings (MCP enabled/disabled)

---

## Phase 4: Auto-Start MCP Server

### How it works
- When the Tauri app opens, it spawns the MCP server as a **background child process**
- MCP server runs on stdio — agents connect by launching `devstar-mcp`
- **Settings toggle** in the UI: "Enable MCP Server" (on/off)
- When disabled: MCP server doesn't start, saves memory
- Setting persisted in `~/.devstar/settings.json`

### Settings file (`~/.devstar/settings.json`)
```json
{
  "mcp_enabled": true
}
```

---

## Phase 5: Error Logging as Todo Items

### How it works
- Agent calls `log_error` with project ID + error message
- Server checks if project has an "Agent Errors" section
- If not, creates it automatically
- Adds the error as an **unchecked todo item** in that section
- Agent can later check it off when resolved
- Error items include: timestamp, agent ID, error message

### DB changes
- Add `agent_id` column to `project_items` (nullable, for attribution)
- Error items are regular todos — can be checked off, not deleted by agents

---

## Phase 6: Agent Documentation

### Files to create
- `docs/agents/AGENTS.md` — Complete reference: tools, examples, security model
- `docs/agents/CLAUDE.md` — Claude Code integration guide
- `docs/agents/mcp-config.json` — Ready-to-use MCP config for any agent
- `docs/agents/quick-start.md` — 5-minute setup guide

---

## Security Model

| Concern | Solution |
|---|---|
| Agent corrupts DB | SQLite WAL mode, transactions, read-only template access |
| Agent modifies others' work | Project ownership via auto-generated agent_id |
| Agent modifies templates | Templates are read-only for MCP clients |
| Error log tampering | Error section is append-only for agents (can check off, not delete) |
| Rate limiting | Already implemented per-connection |
| User doesn't want MCP | Settings toggle disables auto-start |
| Agent floods requests | Rate limiting (already implemented) |

---

## Implementation Order

1. **Rename** → DevStar everywhere (quick, unblocks everything)
2. **MCP server binary** → stdio transport, all tools
3. **Agent identity** → auto-generated `~/.devstar/agent_id`
4. **Auto-start** → spawn MCP from Tauri, settings toggle
5. **Error logging** → "Agent Errors" section as todos
6. **Documentation** → agent-facing MD files
7. **Test** → verify with Claude Code / Antigravity

---

## Key Design Decisions

- **MCP transport:** stdio (local agents run alongside the app)
- **Agent identity:** auto-generated UUID in `~/.devstar/agent_id` (no config needed)
- **Error logging:** append-only "Agent Errors" section with todos (not a separate table)
- **Background mode:** MCP auto-starts with app, can be disabled in UI settings
- **Security:** project ownership, read-only templates, rate limiting, no destructive operations for agents
