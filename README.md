<p align="center">
  <img src="logo-bar.png" alt="DevStar" width="280">
</p>

<p align="center">
  <strong>Sprint-based project planning for developers — powered by AI agents</strong>
</p>

<p align="center">
  <a href="#features">Features</a> •
  <a href="#installation">Installation</a> •
  <a href="#getting-started">Getting Started</a> •
  <a href="#ai-agent-integration">AI Agent Integration</a> •
  <a href="#mcp-server">MCP Server</a> •
  <a href="#architecture">Architecture</a> •
  <a href="#building-from-source">Build from Source</a>
</p>

---

## What is DevStar?

DevStar is a desktop application that helps developers plan and track projects using a **sprint-based checklist workflow**. It comes with **12 pre-built templates** covering every major type of software project — from full-stack web apps to embedded systems — each broken down into detailed, actionable sprints and sections.

What sets DevStar apart is its **built-in MCP server** — AI coding agents (Claude Code, Cursor, etc.) can read your project plan, check off completed tasks, add new work items, and even auto-advance sprints when all tasks are done. The agent works directly within the DevStar ecosystem, keeping the UI in sync in real-time.

## Features

- **12 Professional Templates** — Pre-built for Web, Mobile, Desktop, Game Dev, Embedded/IoT, API/Backend, Data Science/AI, Cloud/Infra, Systems Programming, Enterprise, Security, and Tools/Libraries
- **Sprint-Based Workflow** — Track progress through sprints (pending → active → done) with auto-advance on completion
- **Live Mode** — A compact floating window showing your current active sprint for focused task completion
- **AI Agent Integration** — Built-in MCP server with 20 tools for agents to manage projects, sprints, and tasks
- **Project Scoping** — Agents working in a project directory are scoped to that project only (via `.devstar.json`)
- **Live UI Sync** — Changes made by AI agents appear in the UI within 3 seconds via background polling
- **Auto-PATH** — Install directory is automatically added to user PATH on first launch
- **Shared Sections & Sprints** — Reusable checklist blocks that can be linked or copied
- **System Tray** — Runs in the background, accessible from the system tray
- **Cross-Platform** — Windows (NSIS/MSI) and Linux (.deb) builds

## Installation

### Windows

Download the latest `.exe` (NSIS) installer from the [Releases](https://github.com/Ronin-117/DevStar/releases) page. The installer will:

- Install DevStar to your system
- Create a desktop shortcut
- Add DevStar to startup (runs in system tray on login)
- Add the install directory to your PATH so `devstar-mcp` is available from any terminal

### Linux

Download the latest `.deb` package and install:

```bash
sudo dpkg -i devstar_*.deb
sudo apt-get install -f  # Fix any missing dependencies
```

## Getting Started

1. **Launch DevStar** — Click the desktop shortcut or find it in your Start Menu
2. **Browse Templates** — Go to the **Library** tab to see all 12 templates with their sprint breakdowns
3. **Create a Project** — Click **+ New Project**, pick a template, and give it a name
4. **Track Progress** — Open your project, check off items as you complete them, and watch sprints auto-advance
5. **Use Live Mode** — Click the Live Mode button for a compact floating window showing your active sprint

## AI Agent Integration

DevStar's MCP server enables AI coding agents to fully participate in your project workflow. An agent working in your project directory can:

### Typical Agent Workflow

1. **Discover** — Call `get_project_context` to read `.devstar.json` and see the current sprint
2. **Plan** — Call `get_project_sprints` for the full project plan, or `search_tasks` to find related work
3. **Work** — Complete tasks using `check_task` (by title) or `update_item` (by ID)
4. **Organize** — Add new work with `add_task`, create categories with `add_section`
5. **Advance** — Sprints auto-advance when all tasks are done, or call `complete_sprint` to fast-forward

### Agent Self-Sufficiency

Every MCP tool has detailed descriptions that explain what it does, what arguments it needs, and when to use it. An agent that has never seen DevStar before can figure it out from the tool descriptions alone — no prior knowledge required.

### Project Scoping

When you create a project, DevStar writes a `.devstar.json` file to your working directory. This file scopes the agent to that specific project — they can only see and edit tasks within it. Shared resources (templates, sections, sprints) remain accessible. Agents in different directories work on different projects.

### Live UI Sync

Changes made by the agent (checking tasks, adding items, advancing sprints) appear in the DevStar UI within ~3 seconds via background polling. No restart needed.

## Architecture

```
┌─────────────────────────────────────────────┐
│                 DevStar App                  │
├──────────────────┬──────────────────────────┤
│   Frontend       │   Backend (Rust)         │
│   React + TS     │   SQLite + Tauri         │
│   Zustand Store  │   MCP Server (stdio)     │
│   Tailwind CSS   │   System Tray            │
└──────────────────┴──────────────────────────┘
```

### Data Model

```
Template → TemplateSprints → TemplateSprintSections → SharedSections
SharedSection → SharedSectionItems
SharedSprint → SharedSprintSections
Project → ProjectSprints → ProjectSprintSections → ProjectItems
```

## Building from Source

### Prerequisites

- **Node.js 20+** and npm
- **Rust stable** (install via [rustup](https://rustup.rs/))
- **Tauri dependencies** (platform-specific, see [Tauri docs](https://v2.tauri.app/start/prerequisites/))

### Build

```bash
# Install frontend dependencies
npm install

# Development mode (web only)
npm run dev

# Development mode (full Tauri desktop app)
npm run tauri dev

# Production build
npm run tauri build
```

The built installer will be in `src-tauri/target/release/bundle/`.

## MCP Server

DevStar includes a built-in **MCP (Model Context Protocol) server** that allows AI coding agents to interact with your project plans programmatically. The server runs as a background process when DevStar launches.

### Available Tools (20 total)

| Tool | Description |
|------|-------------|
| `get_project_context` | Zero-config discovery via `.devstar.json`. Returns project overview + active sprint with all tasks |
| `dashboard` | Compact overview of ALL projects — name, progress %, active sprint |
| `create_project` | Create from template + write `.devstar.json` for agent scoping |
| `get_active_sprint_detail` | Current active sprint with all sections and tasks |
| `get_project_sprints` | ALL sprints with status, progress, and section counts |
| `get_sprint` | Get any sprint by number (1-based) or name with full task list |
| `get_tasks` | List tasks in active sprint, filtered by status (pending/done/all) |
| `add_task` | Add a task to active sprint by title only — no IDs needed |
| `check_task` | Mark a task done by title (partial, case-insensitive match) |
| `uncheck_task` | Undo a task check by title |
| `update_item` | Low-level: check/uncheck/add notes by item ID |
| `add_item` | Low-level: add task to specific section by section ID |
| `add_section` | Add a new section/category to the active sprint |
| `search_tasks` | Search ALL tasks across ALL sprints by keyword |
| `complete_sprint` | Mark all tasks done, mark sprint done, activate next sprint |
| `get_progress` | Project completion stats (checked/total/percentage) |
| `log_error` | Log an error as an unchecked task with agent attribution |
| `list_templates` | All available templates with sprint counts |
| `get_template` | Template structure (sprints, sections) |
| `list_shared_sections` | Reusable checklist blocks |
| `list_shared_sprints` | Reusable sprint templates |

### Configuration

Add to your AI agent's MCP config:

```json
{
  "mcpServers": {
    "devstar": {
      "command": "devstar-mcp"
    }
  }
}
```

The MCP server starts automatically when DevStar launches and runs as a background process. The `devstar-mcp` binary is available on PATH after installation.

## Project Structure

```
ProjectTracker/
├── src/                          # Frontend (React + TypeScript)
│   ├── components/
│   │   ├── active/               # Live Mode window
│   │   ├── projects/             # Project views
│   │   ├── templates/            # Template & library views
│   │   └── shared/               # Reusable UI components
│   ├── store/                    # Zustand state management
│   └── lib/                      # API layer, types, utilities
├── src-tauri/                    # Backend (Rust)
│   ├── src/
│   │   ├── db/                   # Database layer
│   │   │   ├── seeds/            # Seed data (12 templates)
│   │   │   │   └── templates/    # Per-template seed files
│   │   │   └── *.rs              # CRUD operations
│   │   ├── lib.rs                # Tauri commands & setup
│   │   └── mcp_server.rs         # MCP server binary
│   └── tauri.conf.json           # Tauri configuration
└── .github/workflows/            # CI/CD pipeline
```

## CI/CD

Every push to `main` triggers:

- **TypeScript type checking**
- **Rust clippy** (with `-D warnings`)
- **Rust tests**
- **Full app build** for Windows and Linux (Linux build is non-blocking)

Tagged releases (`v*`) produce downloadable installers:

- Windows: NSIS `.exe` installer + standalone `.exe`
- Linux: `.deb` package (best-effort, doesn't block release)

## License

MIT

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request
