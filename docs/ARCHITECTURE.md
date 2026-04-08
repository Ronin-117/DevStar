# Architecture

## System Overview

DevStar is a Tauri v2 desktop application with a React + TypeScript frontend and a Rust backend using SQLite for persistence. It runs as a **background-first app** — starting as a system tray icon with an MCP server for AI agents, and only showing its UI on demand.

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                    Tauri Desktop App                        │
├──────────────────┬──────────────────────────────────────────┤
│   Frontend       │   Backend (Rust)                         │
│   (Web)          │                                          │
│                  │  ┌────────────┐  ┌────────────────────┐  │
│  React + TSX     │  │ MCP Server │  │ Tauri Commands     │  │
│  Zustand Store   │◄─┤ (stdio)    │◄─┤ (invoke/handler)   │  │
│  Tailwind CSS    │  └────────────┘  └────────┬───────────┘  │
│                  │                           │              │
│                  │  ┌────────────────────────▼───────────┐  │
│                  │  │  DB Layer (SQLite)                 │  │
│                  │  │  ├── schema.sql                    │  │
│                  │  │  ├── seeds/ (12 templates)         │  │
│                  │  │  └── *.rs (CRUD operations)        │  │
│                  │  └────────────────────────────────────┘  │
└──────────────────┴──────────────────────────────────────────┘
         ▲
         │ System Tray (left-click: open UI, right-click: menu)
         └── Open DevStar | Live Mode | Stop DevStar
```

## Data Model

### Core Entities

```
Template (id, name, description, color, created_at, updated_at)
  ├── TemplateSprint (id, template_id, name, description, sort_order, is_custom)
  │   └── TemplateSprintSection (id, sprint_id, section_id, sort_order, is_linked)
  │       └── references → SharedSection

SharedSection (id, name, description, color, created_at, updated_at)
  └── SharedSectionItem (id, section_id, title, description, sort_order)

SharedSprint (id, name, description, sort_order, created_at, updated_at)
  └── SharedSprintSection (id, sprint_id, section_id, sort_order, is_linked)
      └── references → SharedSection

Project (id, name, description, template_id, color, created_at, updated_at)
  ├── ProjectSprint (id, project_id, name, description, status, sort_order, is_custom)
  │   └── ProjectSprintSection (id, sprint_id, name, description, sort_order, is_custom, linked_from_section_id)
  │       └── ProjectItem (id, section_id, title, description, checked, notes, sort_order, is_custom)
```

### Key Relationships

- **Templates** are blueprints composed of sprints and sections
- **Shared Sections** are reusable checklist blocks used across templates
- **Shared Sprints** are reusable sprint templates composed of shared sections
- **Projects** are instantiated from templates, copying the sprint/section structure
- **Link vs Copy**: Linked items reference the shared source; copied items are independent

### Sprint Status Flow

```
pending ──(user action)──► active ──(all items checked)──► done ──(auto)──► next sprint active
```

## Frontend Architecture

### State Management

Single Zustand store (`src/store/index.ts`) with:

- **Data arrays**: `templates`, `projects`, `sharedSections`, `sharedSprints`
- **Detail caches**: `templateSprints`, `projectSprints`, `sharedSectionDetail`, `sharedSprintDetail` (all `Map<number, DetailType>`)
- **Progress tracking**: `projectProgressMap` for card-level progress display
- **Current sprint tracking**: `currentSprintMap` for project card labels
- **UI state**: `view`, `libraryTab`, `selectedProjectId`, `selectedTemplateId`, `editingProjectId`

### Cross-Window Communication

The Live Mode window and Management window communicate via Tauri events:

1. `apiToggleProjectItem()` in Live Mode calls the Rust backend
2. Backend toggles the item and the frontend emits `project-item-toggled` event
3. Management window's store listener catches the event
4. Store updates the cached `projectSprints` Map in-place
5. `projectProgressMap` is recalculated
6. React re-renders only the changed components

### Component Hierarchy

```
App
├── TitleBar (custom, with app icon)
├── Header (logo-bar, nav tabs: Projects | Library)
│   └── Library sub-tabs (Templates | Shared Sections | Shared Sprints)
├── Main Content
│   ├── ProjectsView
│   ├── ProjectDetailView
│   ├── TemplatesView
│   ├── TemplateEditorView
│   ├── SharedSectionsView
│   └── SharedSprintsView
└── ActiveMode (separate window, transparent background)
```

## Backend Architecture

### Rust Module Structure

```
src-tauri/src/
├── lib.rs              # Tauri commands, tray setup, MCP spawn, startup registry
├── main.rs             # Entry point
├── mcp_server.rs       # MCP server binary (stdio JSON-RPC)
├── rate_limit.rs       # Rate limiter for Tauri commands
└── db/
    ├── mod.rs          # Module exports
    ├── types.rs        # Rust types matching DB schema
    ├── schema.sql      # SQLite schema
    ├── tests.rs        # Unit tests
    ├── seeds/          # Seed data (10 sections, 8 sprints, 12 templates)
    │   ├── mod.rs      # Seed orchestrator + helper functions
    │   ├── shared_sections.rs
    │   ├── shared_sprints.rs
    │   └── templates/
    │       ├── mod.rs
    │       ├── web_dev.rs
    │       ├── mobile_app.rs
    │       ├── desktop_app.rs
    │       ├── game_dev.rs
    │       ├── embedded_iot.rs
    │       ├── api_backend.rs
    │       ├── data_science_ai.rs
    │       ├── cloud_infra.rs
    │       ├── systems_programming.rs
    │       ├── enterprise_systems.rs
    │       ├── security_software.rs
    │       └── tools_libraries.rs
    ├── project_sprints.rs  # Project sprint CRUD + auto-advance
    ├── projects.rs         # Project CRUD + create from template
    ├── shared_sections.rs  # Shared section CRUD
    ├── shared_sprints.rs   # Shared sprint CRUD
    ├── template_sprints.rs # Template sprint CRUD
    └── templates.rs        # Template CRUD
```

### Key Backend Operations

- **`create_project_from_template`**: Copies template sprints → project sprints, copying all sections and items
- **`check_and_advance_sprint`**: Checks if all items in active sprint are done; if so, marks it done and activates the next sprint
- **`complete_sprint`**: Marks all items in a sprint as checked, then marks sprint done and advances
- **`toggle_mode`**: Switches between Management and Active windows

## Window Management

DevStar uses two Tauri windows:

1. **Management** (`management`): Main window with full UI. Starts hidden (`visible: false`), shown on tray click.
2. **Active** (`active`): Compact floating window showing the active sprint. Transparent background.

The Active window can be:
- **Full panel** (340×500px): Shows sprint name, sections, and checklist
- **Minimized** (56×56px): Round button with app icon, positioned at top-right of screen

## System Tray

The app starts minimized to the system tray. The tray icon uses `app-icon.png` and provides:

- **Left-click**: Open management window
- **Right-click menu**:
  - **Open DevStar** — Show management window
  - **Live Mode** — Open active sprint window
  - **Stop DevStar** — Kill MCP server and exit app

## MCP Server

A separate binary (`devstar-mcp`) that runs as a background child process:

- Spawns on app startup (hidden, no console window on Windows)
- Communicates via JSON-RPC over stdio
- Shares the same SQLite database
- Exposes 14 tools for AI agents to read/update project plans
- Killed gracefully when the user selects "Stop DevStar" from the tray

## Startup Behavior

On first run, DevStar adds itself to the system startup:
- **Windows**: Registry key at `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`
- **Linux**: XDG autostart `.desktop` file at `~/.config/autostart/devstar.desktop`

The app starts hidden (tray only) with MCP server running.
