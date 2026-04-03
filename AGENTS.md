# AGENTS.md - DevStar

## Project Overview

DevStar — Tauri v2 desktop app (React + TypeScript + Tailwind CSS v4) for managing project development checklists using a sprint-based workflow. Users create templates composed of sprints, each containing sections of checklist items. Projects are instantiated from templates and tracked through sprints (pending → active → done). A "Live Mode" floating window shows the current sprint for focused task completion.

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

**No test framework is configured.** To add: `npm i -D vitest @testing-library/react @testing-library/jest-dom jsdom`, then add `"test": "vitest"` to scripts.

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
                 Modal.tsx, ProgressBar.tsx, TitleBar.tsx
```

### Cross-Window Sync

- `apiToggleProjectItem()` emits `project-item-toggled` event via Tauri
- Store listens and updates cached `projectSprints` Map in-place
- No full refetch, no scroll jump, no flicker

### Tauri Backend

- Rust source in `src-tauri/src/`
- DB: SQLite at `src-tauri/src/db/` with schema in `schema.sql`
- Seed data in `seed.rs` (6 shared sections, 5 shared sprints, 3 templates)
- Window management: `toggle_mode`, `set_active_window_compact`, `set_active_window_full`
- Sprint auto-advance: `check_and_advance_sprint`, `complete_sprint` (marks all items done + advances)
- Crate name: `projecttracker_lib` (internal identifier)

### Key Features

- **Templates**: Expandable hierarchy — template → sprints → sections. Add custom or shared sprints/sections (link or copy).
- **Shared Sections**: Reusable checklist blocks with inline item CRUD.
- **Shared Sprints**: Reusable sprint templates composed of shared sections.
- **Projects**: Created from templates, track sprint progress with status badges.
- **Live Mode**: Compact floating window showing active sprint with checkable items. Minimize to a round indigo button; restore to full panel. Auto-advances sprints on completion.
- **Navigation**: Top tabs `Projects | Library`. Library sub-tabs: `Templates | Shared Sections | Shared Sprints` (always visible).
