# AGENTS.md - ProjectTracker

## Project Overview

Tauri v2 desktop app (React + TypeScript + Tailwind CSS v4) for managing projects and templates. Backend is Rust in `src-tauri/`. Frontend communicates with Rust via Tauri `invoke()` commands.

## Commands

| Task | Command |
|------|---------|
| Dev (web) | `npm run dev` |
| Dev (Tauri) | `npm run tauri dev` |
| Build (web) | `npm run build` |
| Build (Tauri) | `npm run tauri build` |
| Preview | `npm run preview` |
| Type check | `npx tsc --noEmit` |

**No test framework is configured.** There are no test scripts in `package.json` and no test files exist. To add tests, install Vitest: `npm i -D vitest @testing-library/react @testing-library/jest-dom jsdom`, then add `"test": "vitest"` to scripts.

**Running a single test (once Vitest is added):** `npx vitest run -t "test name"` or `npx vitest run src/path/to/file.test.tsx`

## Code Style

### Imports

- React hooks imported from `'react'` (e.g., `import { useEffect, useState } from 'react'`)
- Tauri APIs from `'@tauri-apps/api/core'`
- Use explicit named imports, no default imports except for React in `main.tsx`
- Import order: external libs → internal modules (relative paths) → CSS
- Type-only imports use `import type { ... }` syntax

### TypeScript

- **Strict mode enabled** (`"strict": true` in tsconfig)
- `noUnusedLocals` and `noUnusedParameters` are enforced — do not leave unused variables
- Use `unknown` for catch clause errors, cast to `Error` when accessing `.message`: `(e as Error).message`
- All Tauri `invoke()` calls are generic: `invoke<Template[]>('cmd_name', { args })`
- API functions return typed Promises, never `any`
- Interfaces for data types live in `src/lib/types.ts`

### Naming Conventions

- **Components**: PascalCase function components (`ProjectsView`, `TitleBar`, `ActiveMode`)
- **API functions**: camelCase with `api` prefix (`apiListTemplates`, `apiCreateProject`)
- **Store hooks**: camelCase (`useStore`)
- **Type interfaces**: PascalCase (`Template`, `ProjectSectionWithItems`)
- **Utility functions**: camelCase (`cn` for class merging)
- **CSS classes**: Tailwind utility classes only, no custom CSS modules

### Formatting

- 2-space indentation
- Single quotes for strings
- Semicolons required
- Trailing commas in multi-line objects/arrays

### Error Handling

- All async operations wrapped in `try/catch`
- Errors stored in Zustand store as `error: string | null`
- Use `set({ error: (e as Error).message })` pattern consistently
- Optional `silent` parameter on fetch actions to suppress loading state: `fetchProjectDetail(id, true)`
- API layer re-throws after logging; store layer catches and surfaces to UI

### State Management (Zustand)

- Single store at `src/store/index.ts`
- Selectors use inline arrow: `useStore((s) => s.view)`
- Direct store access via `useStore.getState()` for cross-callers (e.g., clearing state on view switch)
- Async actions call `get()` to access sibling actions (e.g., `await get().fetchProjects()`)
- Loading state pattern: `set({ loading: true, error: null })` → operation → `set({ loading: false })`

### Styling

- Tailwind CSS v4 via `@tailwindcss/vite` plugin
- CSS variables defined in `src/index.css` for theming (HSL format)
- Use `cn()` utility from `src/lib/utils.ts` for conditional class merging with `clsx` + `tailwind-merge`
- Tauri drag regions: `{ appRegion: 'drag' } as React.CSSProperties`

### Component Structure

- Feature folders under `src/components/{feature}/` (projects, templates, active, shared)
- Shared UI components in `src/components/shared/`
- Each view is a single exported function component
- No prop drilling — use Zustand store directly in components
- Inline event handlers are acceptable for simple callbacks

### Tauri Backend

- Rust source in `src-tauri/src/`
- Tauri commands are invoked by name (snake_case): `invoke('list_templates')`
- Input objects use snake_case keys to match Rust struct fields
- Window management commands: `toggle_mode`, `resize_active_window`, `close_window`, etc.
