# DevStar — MCP Server & Agent Integration Plan

> Created: 2026-04-01
> Last Updated: 2026-04-03
> Status: Planning — ready for implementation

---

## Current State (Completed)

### ✅ Sprint-Based Data Model
- `Template → TemplateSprints → TemplateSprintSections → SharedSections`
- `SharedSection → SharedSectionItems` (reusable checklist blocks)
- `SharedSprint → SharedSprintSections` (reusable sprint templates)
- `Project → ProjectSprints → ProjectSprintSections → ProjectItems`
- Sprint lifecycle: `pending → active → done` with auto-advance

### ✅ Templates & Seed Data
- 10 shared sections (10 items each)
- 8 shared sprints (composing shared sections)
- 12 templates with 8-12 sprints each (Web, Mobile, Desktop, Game, Embedded/IoT, API/Backend, Data Science/AI, Cloud/Infra, Systems, Enterprise, Security, Tools)
- Link vs Copy semantics for shared items

### ✅ UI Features
- Expandable hierarchy in all views (template → sprint → section)
- Custom or shared sprint/section addition with link/copy toggle
- Live Mode window with minimize (48px button) / restore (340×500px panel)
- Sprint auto-advance on completion, "Complete" button marks all items done
- Search on all Library tabs + MiniSearchInput on all shared item dropdowns
- Cross-window event sync (item toggles reflect instantly without refetch)
- Project cards show current sprint label and progress bar

### ✅ Documentation
- `AGENTS.md` — Complete project reference
- `docs/ARCHITECTURE.md` — System architecture and data model
- `docs/ADR.md` — 7 architecture decision records
- `docs/SEED_DATA.md` — Complete seed data documentation
- `docs/ONBOARDING.md` — New developer guide

---

## Remaining Work

### Phase 1: Internal Rename (Quick)

The UI already shows "DevStar" everywhere. Remaining internal renames:

| File | Current | Target |
|---|---|---|
| `src-tauri/Cargo.toml` | `name = "projecttracker"` | `name = "devstar"` |
| `src-tauri/Cargo.toml` | `name = "projecttracker_lib"` | `name = "devstar_lib"` |
| `src-tauri/tauri.conf.json` | `identifier: "com.njne2.projecttracker"` | `identifier: "com.njne2.devstar"` |
| `src-tauri/src/lib.rs` | DB path `com.njne2.projecttracker` | `com.njne2.devstar` |
| `src-tauri/src/lib.rs` | DB file `projecttracker.db` | `devstar.db` |
| `src-tauri/src/main.rs` | `projecttracker_lib::run()` | `devstar_lib::run()` |

**Note**: This will wipe the existing DB on next run (expected during development).

---

### Phase 2: MCP Server (stdio transport)

#### Architecture
- New crate `devstar-mcp` in `src-tauri/mcp/`
- Uses JSON-RPC over stdio (MCP protocol)
- Shares the same SQLite database — opens the same file
- **Read-only for templates/shared items**, **full access for projects**

#### MCP Tools (Updated for Sprint Hierarchy)

| Tool | Description | Permissions |
|---|---|---|
| `list_templates` | List all templates with sprint/section counts | Read-only |
| `get_template` | Get a template's sprints, sections, and items | Read-only |
| `list_shared_sections` | List all shared sections with item counts | Read-only |
| `list_shared_sprints` | List all shared sprints with section counts | Read-only |
| `create_project` | Create a new project from a template | Full |
| `get_project` | Get project details, sprints, sections, items, progress | Own projects only |
| `get_active_sprint` | Get the current active sprint for a project | Read own projects |
| `update_item` | Check/uncheck an item, add notes | Own projects only |
| `add_section` | Add a custom section to a project sprint | Own projects only |
| `add_item` | Add a custom item to a section | Own projects only |
| `set_sprint_status` | Set a sprint's status (pending/active/done) | Own projects only |
| `complete_sprint` | Mark all items done and advance to next sprint | Own projects only |
| `get_progress` | Get completion stats for a project | Read own projects |
| `log_error` | Create "Agent Errors" section with error items as unchecked todos | Own projects only |

#### MCP Resources
- `template://{id}` — Full template with sprints and sections
- `project://{id}` — Full project with sprints, sections, and items
- `progress://{id}` — Project progress summary

---

### Phase 3: Agent Identity (auto-generated, no config)

#### How it works
1. On first MCP connection, the server checks for `~/.devstar/agent_id`
2. If missing, generates a UUID and saves it
3. Every MCP call includes the agent ID automatically
4. Agent ID is used for:
   - **Project ownership** — agents can only modify projects they created
   - **Error attribution** — error items show which agent logged them
   - **Audit trail** — DB tracks which agent made each change

#### File
- `~/.devstar/agent_id` — contains a single UUID string
- `~/.devstar/settings.json` — user settings (MCP enabled/disabled)

---

### Phase 4: Auto-Start MCP Server

#### How it works
- When the Tauri app opens, it spawns the MCP server as a **background child process**
- MCP server runs on stdio — agents connect by launching `devstar-mcp`
- **Settings toggle** in the UI: "Enable MCP Server" (on/off)
- When disabled: MCP server doesn't start, saves memory
- Setting persisted in `~/.devstar/settings.json`

#### Settings file (`~/.devstar/settings.json`)
```json
{
  "mcp_enabled": true
}
```

---

### Phase 5: Error Logging as Todo Items

#### How it works
- Agent calls `log_error` with project ID + error message
- Server checks if project has an "Agent Errors" section
- If not, creates it automatically
- Adds the error as an **unchecked todo item** in that section
- Agent can later check it off when resolved
- Error items include: timestamp, agent ID, error message

#### DB changes
- Add `agent_id` column to `project_items` (nullable, for attribution)
- Error items are regular todos — can be checked off, not deleted by agents

---

### Phase 6: Agent Documentation

#### Files to create
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

1. **Internal rename** → DevStar crate name, identifier, DB path (quick, unblocks everything)
2. **MCP server crate** → stdio transport, all tools, resources
3. **Agent identity** → auto-generated `~/.devstar/agent_id`
4. **Auto-start** → spawn MCP from Tauri, settings toggle
5. **Error logging** → "Agent Errors" section as todos, DB migration
6. **Documentation** → agent-facing MD files
7. **Test** → verify with Claude Code / OpenCode / Antigravity

---

## Key Design Decisions

- **MCP transport:** stdio (local agents run alongside the app)
- **Agent identity:** auto-generated UUID in `~/.devstar/agent_id` (no config needed)
- **Error logging:** append-only "Agent Errors" section with todos (not a separate table)
- **Background mode:** MCP auto-starts with app, can be disabled in UI settings
- **Security:** project ownership, read-only templates, rate limiting, no destructive operations for agents
- **Sprint awareness:** MCP tools expose the full sprint hierarchy so agents can navigate sprints, manage sections within sprints, and track sprint-level progress
