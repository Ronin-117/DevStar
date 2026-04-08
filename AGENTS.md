# AGENTS.md - DevStar

## Project Overview

DevStar — Tauri v2 desktop app (React + TypeScript + Tailwind CSS v4) for managing project development checklists using a sprint-based workflow. Runs as a **background-first app**: starts as a system tray icon with an MCP server for AI agents, shows UI on demand. Users create templates composed of sprints, each containing sections of checklist items. Projects are instantiated from templates and tracked through sprints (pending → active → done). A "Live Mode" floating window shows the current sprint for focused task completion.

## Data Model

```
Template → TemplateSprints → TemplateSprintSections (linked to SharedSections)
SharedSection → SharedSectionItems (reusable checklist blocks)
SharedSprint → SharedSprintSections (reusable sprint templates)
Project → ProjectSprints → ProjectSprintSections → ProjectItems (checked/unchecked)
```

**Sprint lifecycle**: `pending` → `active` → `done`. Auto-advances when all items in the active sprint are checked.

## Commands

| Task | Command |
|------|---------|
| Dev (web) | `npm run dev` |
| Dev (Tauri) | `npm run tauri dev` |
| Build (web) | `npm run build` |
| Build (Tauri) | `npm run tauri build` |
| Preview | `npm run preview` |
| Type check | `npx tsc --noEmit` |
| Rust lint (CI) | `cd src-tauri && cargo clippy -- -D warnings` |
| Rust tests | `cd src-tauri && cargo test` |

**No frontend test framework is configured.** To add: `npm i -D vitest @testing-library/react @testing-library/jest-dom jsdom`, then add `"test": "vitest"` to scripts.

## Code Style

### Imports

- React hooks from `'react'`
- Tauri APIs from `'@tauri-apps/api/core'` and `'@tauri-apps/api/event'`
- Named imports only (no defaults except React in `main.tsx`)
- Order: external libs → internal modules → CSS
- Type-only imports: `import type { ... }`

### TypeScript

- **Strict mode** with `noUnusedLocals` and `noUnusedParameters` enforced
- Catch errors as `unknown`, cast with `(e as Error).message`
- All `invoke()` calls are generic: `invoke<Template[]>('cmd_name', { args })`
- API functions return typed Promises, never `any`
- All types in `src/lib/types.ts`

### Naming Conventions

- **Components**: PascalCase (`ProjectsView`, `TemplateEditorView`)
- **API functions**: camelCase with `api` prefix (`apiListProjectSprints`)
- **Store**: camelCase (`useStore`, `fetchTemplates`)
- **Types**: PascalCase (`ProjectSprintWithSections`)
- **Tauri commands**: snake_case (`list_project_sprints`)

### Formatting

- 2-space indent, single quotes, semicolons, trailing commas

### Error Handling

- All async wrapped in `try/catch`
- Errors stored in Zustand: `set({ error: (e as Error).message })`
- `silent` param on fetch actions suppresses loading state

### State Management (Zustand)

- Single store at `src/store/index.ts`
- Views: `'projects' | 'library' | 'template-editor'`
- `libraryTab`: `'templates' | 'shared-sections' | 'shared-sprints'`
- Detail caches use `Map<number, DetailType>` for keyed data
- Event listener syncs `project-item-toggled` events between windows (in-place update, no refetch)
- `useStore.getState()` for cross-callers; `get()` inside actions

### Styling

- Tailwind CSS v4 via `@tailwindcss/vite`
- `cn()` from `src/lib/utils.ts` for conditional classes
- Tauri drag regions: `style={{ ['appRegion' as string]: 'drag' }}`

### Component Structure

```
src/components/
  active/        ActiveMode.tsx (Live Mode window)
  projects/      ProjectsView.tsx, ProjectDetailView.tsx
  templates/     TemplatesView.tsx, TemplateEditorView.tsx,
                 SharedSectionsView.tsx, SharedSprintsView.tsx
  shared/        Checkbox.tsx, CollapsibleSection.tsx,
                 Modal.tsx, ProgressBar.tsx, TitleBar.tsx,
                 SearchInput.tsx, MiniSearchInput.tsx
```

### Cross-Window Sync

- `apiToggleProjectItem()` emits `project-item-toggled` event via Tauri
- Store listens and updates cached `projectSprints` Map in-place
- Progress map (`projectProgressMap`) also updated on toggle
- No full refetch, no scroll jump, no flicker

### Tauri Backend

- Rust source in `src-tauri/src/`
- DB: SQLite at `src-tauri/src/db/` with schema in `schema.sql`
- Seed data in `src/db/seeds/` — 10 shared sections, 8 shared sprints, 12 templates (see docs/SEED_DATA.md)
- **Seeds only on first run** (when `templates` table is empty)
- Window management: `toggle_mode`, `set_active_window_compact`, `set_active_window_full`
- Sprint auto-advance: `check_and_advance_sprint`, `complete_sprint` (marks all items done + advances)
- Crate name: `devstar_lib` (internal identifier)
- MCP server: `src/mcp_server.rs` — stdio JSON-RPC for AI agents

### App Lifecycle

```
App starts → Tray icon appears → MCP server spawns → Startup registry set
     ↓
User clicks tray → Management window opens
     ↓
User clicks "Live Mode" → Active window opens (management hides)
     ↓
User closes window → Window hides (app keeps running)
     ↓
User clicks "Stop DevStar" → MCP killed → App exits
```

### System Tray

- Left-click: Open management window
- Right-click menu: Open DevStar | Live Mode | Stop DevStar
- Icon: `app-icon.png`

### Key Features

- **Templates**: Expandable hierarchy — template → sprints → sections. Add custom or shared sprints/sections (link or copy).
- **Shared Sections**: Reusable checklist blocks with inline item CRUD.
- **Shared Sprints**: Reusable sprint templates composed of shared sections.
- **Projects**: Created from templates, track sprint progress with status badges.
- **Live Mode**: Compact floating window showing active sprint with checkable items. Minimize to a round button with app icon; restore to full panel. Auto-advances sprints on completion.
- **Navigation**: Top tabs `Projects | Library`. Library sub-tabs: `Templates | Shared Sections | Shared Sprints` (always visible).
- **Search**: Lightweight search on all Library tabs. Mini search on all shared item dropdowns (section/sprint selectors).
- **MCP Server**: 15 tools for AI agents to read/update project plans. Auto-starts on app launch.
- **Project Discovery**: `.devstar.json` file in project directory for zero-config agent discovery.

### Seed Data

On first run only, the DB is seeded with:
- **10 shared sections** (10 items each): Planning, Security, Testing, CI/CD, Docs, Code Quality, Performance, Monitoring, Database, Accessibility
- **8 shared sprints**: Planning & Setup, Security & Quality, Testing & QA, CI/CD & Deployment, Monitoring & Ops, Performance, Database, Accessibility & UX
- **12 templates** with 8-12 sprints each: Full-Stack Web, Mobile App, Desktop App, Game Dev, Embedded/IoT, API & Backend, Data Science/AI, Cloud & Infra, Systems Programming, Enterprise Systems, Security Software, Tools & Libraries

See `docs/SEED_DATA.md` for complete details.

### CI/CD

- `.github/workflows/ci.yml` runs on every push to `main`: TypeScript check, Rust clippy, Rust tests, full build (Windows MSI+EXE, Linux DEB)
- Tagged releases (`v*`) produce a GitHub Release with all installers attached
