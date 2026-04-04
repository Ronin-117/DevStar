<p align="center">
  <img src="logo-bar.png" alt="DevStar" width="280">
</p>

<p align="center">
  <strong>Sprint-based project planning for developers</strong>
</p>

<p align="center">
  <a href="#features">Features</a> •
  <a href="#installation">Installation</a> •
  <a href="#getting-started">Getting Started</a> •
  <a href="#architecture">Architecture</a> •
  <a href="#building-from-source">Build from Source</a> •
  <a href="#mcp-server">MCP Server</a>
</p>

---

## What is DevStar?

DevStar is a desktop application that helps developers plan and track projects using a **sprint-based checklist workflow**. It comes with **12 pre-built templates** covering every major type of software project — from full-stack web apps to embedded systems — each broken down into detailed, actionable sprints and sections.

Whether you're a solo developer planning your next side project or a team lead structuring a production release, DevStar gives you a proven starting point and the flexibility to customize it to your needs.

## Features

- **12 Professional Templates** — Pre-built for Web, Mobile, Desktop, Game Dev, Embedded/IoT, API/Backend, Data Science/AI, Cloud/Infra, Systems Programming, Enterprise, Security, and Tools/Libraries
- **Sprint-Based Workflow** — Track progress through sprints (pending → active → done) with auto-advance on completion
- **Shared Sections & Sprints** — Reusable checklist blocks that can be linked (auto-updates) or copied (independent)
- **Live Mode** — A compact floating window showing your current active sprint for focused task completion
- **System Tray** — Runs in the background with MCP server, accessible from the system tray
- **AI Agent Integration** — Built-in MCP server so AI coding agents can read and update your project plans
- **Cross-Platform** — Windows (.msi, .exe) and Linux (.deb) builds

## Installation

### Windows

Download the latest `.msi` installer from the [Releases](https://github.com/Ronin-117/DevStar/releases) page. The installer will:

- Install DevStar to your system
- Create a desktop shortcut
- Add DevStar to startup (runs in system tray on login)

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

DevStar includes a built-in **MCP (Model Context Protocol) server** that allows AI coding agents to interact with your project plans programmatically.

### Available Tools

| Tool | Description |
|------|-------------|
| `list_templates` | List all available templates |
| `get_template` | Get a template's full sprint/section hierarchy |
| `create_project` | Create a new project from a template |
| `get_project_context` | Read `.devstar.json` and return full project state |
| `get_active_sprint` | Get the current active sprint with all items |
| `update_item` | Check/uncheck an item or add notes |
| `complete_sprint` | Mark all items done and advance to next sprint |
| `get_progress` | Get completion stats for a project |
| `log_error` | Log an error as a todo item |

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

The MCP server starts automatically when DevStar launches and runs as a background process.

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
- **Full app build** for Windows and Linux

Tagged releases (`v*`) produce downloadable installers:

- Windows: `.msi` installer + standalone `.exe`
- Linux: `.deb` package

## License

MIT

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request
