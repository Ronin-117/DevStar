# Architecture

## System Overview

DevStar is a Tauri v2 desktop application with a React + TypeScript frontend and a Rust backend using SQLite for persistence. The app manages project development checklists using a sprint-based workflow.

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                    Tauri Desktop App                        │
├──────────────────────────┬──────────────────────────────────┤
│     Frontend (Web)       │      Backend (Rust)              │
│                          │                                  │
│  ┌────────────────────┐  │  ┌────────────────────────────┐ │
│  │   React + TSX      │  │  │  Tauri Commands            │ │
│  │   (Views/Comps)    │◄─┼─►│  (invoke/handler)          │ │
│  └────────┬───────────┘  │  └────────────┬───────────────┘ │
│           │              │               │                  │
│  ┌────────▼───────────┐  │  ┌────────────▼───────────────┐ │
│  │   Zustand Store    │  │  │  DB Layer (SQLite)         │ │
│  │   (State Mgmt)     │  │  │  ┌──────────────────────┐  │ │
│  └────────┬───────────┘  │  │  │  schema.sql          │  │ │
│           │              │  │  │  seeds/              │  │ │
│  ┌────────▼───────────┐  │  │  │  project_sprints.rs  │  │ │
│  │   API Layer        │  │  │  │  templates.rs        │  │ │
│  │   (invoke calls)   │  │  │  │  shared_sections.rs  │  │ │
│  └────────────────────┘  │  │  │  shared_sprints.rs   │  │ │
│                          │  │  └──────────────────────┘  │ │
└──────────────────────────┘  └────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
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
├── TitleBar
├── Header (nav tabs: Projects | Library)
│   └── Library sub-tabs (Templates | Shared Sections | Shared Sprints)
├── Main Content
│   ├── ProjectsView
│   ├── ProjectDetailView
│   ├── TemplatesView
│   ├── TemplateEditorView
│   ├── SharedSectionsView
│   └── SharedSprintsView
└── ActiveMode (separate window)
```

## Backend Architecture

### Rust Module Structure

```
src-tauri/src/
├── lib.rs              # Tauri command registration, app setup, window management
├── main.rs             # Entry point
└── db/
    ├── mod.rs          # Module exports
    ├── types.rs        # Rust types matching DB schema
    ├── schema.sql      # SQLite schema
    ├── seeds/          # Seed data (10 sections, 8 sprints, 12 templates)
    │   ├── mod.rs      # Seed orchestrator
    │   ├── shared_sections.rs
    │   ├── shared_sprints.rs
    │   └── templates/
    │       ├── mod.rs  # Helper functions
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

1. **Management** (`management`): Main window with full UI
2. **Active** (`active`): Compact floating window showing the active sprint

The Active window can be:
- **Full panel** (340×500px): Shows sprint name, sections, and checklist
- **Minimized** (48×48px): Single round indigo button positioned at top-right of screen
