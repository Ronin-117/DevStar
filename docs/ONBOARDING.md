# Developer Onboarding Guide

Welcome to DevStar! This guide will get you up and running quickly.

## Quick Start

```bash
# Install dependencies
npm install

# Run in development mode (web only, fastest)
npm run dev

# Run with Tauri desktop app
npm run tauri dev

# Type check
npx tsc --noEmit
```

## What is DevStar?

DevStar is a desktop app for managing project development checklists. Think of it as a structured, sprint-based planning tool for software projects. You:

1. **Create templates** from 12 pre-built types (Web, Mobile, Game, AI, etc.)
2. **Customize** them by adding/removing sprints and sections
3. **Create projects** from templates
4. **Track progress** through sprints (pending → active → done)
5. **Use Live Mode** — a floating window showing your current sprint for focused work

## Project Structure

```
ProjectTracker/
├── src/                          # Frontend (React + TypeScript)
│   ├── lib/
│   │   ├── api.ts                # Tauri invoke wrappers
│   │   ├── types.ts              # TypeScript type definitions
│   │   └── utils.ts              # Utility functions (cn)
│   ├── store/
│   │   └── index.ts              # Zustand state management
│   ├── components/
│   │   ├── active/               # Live Mode window
│   │   ├── projects/             # Project views
│   │   ├── templates/            # Template & shared library views
│   │   └── shared/               # Reusable UI components
│   ├── App.tsx                   # Main app component
│   └── main.tsx                  # Entry point
├── src-tauri/                    # Backend (Rust)
│   ├── src/
│   │   ├── lib.rs                # Tauri commands & setup
│   │   └── db/                   # Database layer
│   │       ├── schema.sql        # SQLite schema
│   │       └── seeds/            # Seed data (12 templates)
│   └── tauri.conf.json           # Tauri configuration
└── docs/                         # Documentation
```

## Key Concepts

### Data Hierarchy

```
Shared Sections (reusable checklists)
       ↓
Shared Sprints (reusable sprint templates)
       ↓
Templates (project blueprints)
       ↓
Projects (instances with tracked progress)
```

### Link vs Copy

When adding shared items to a template:
- **Link**: Changes to the shared source propagate to all linked instances
- **Copy**: Independent copy; no connection to the source

### Sprint Lifecycle

```
pending → active → done → (auto-advance) → next sprint active
```

Sprints auto-advance when all items are checked.

## Common Tasks

### Adding a New Template

1. Create a new file in `src-tauri/src/db/seeds/templates/my_template.rs`
2. Export it from `seeds/templates/mod.rs`
3. Call it from `seeds/mod.rs::seed_all()`
4. See existing templates for patterns

### Adding a New API Endpoint

1. Add the Rust function in the appropriate `src-tauri/src/db/*.rs` file
2. Register it as a Tauri command in `lib.rs`
3. Add the invoke wrapper in `src/lib/api.ts`
4. Use it in your component

### Adding a New View Component

1. Create the component in the appropriate `src/components/` folder
2. Import and render it in `App.tsx`
3. Add any needed store state in `src/store/index.ts`

## Testing

No test framework is configured yet. To add:

```bash
npm i -D vitest @testing-library/react @testing-library/jest-dom jsdom
```

Then add `"test": "vitest"` to `package.json` scripts.

## Troubleshooting

### "No space left on device"
Clean build artifacts:
```bash
rm -rf src-tauri/target dist
```

### Database not seeding
The DB is wiped and re-seeded on every run. Check `src-tauri/src/lib.rs` for the seed call.

### TypeScript errors
Run `npx tsc --noEmit` to see all errors. Common issues:
- Unused imports (strict mode enforces `noUnusedLocals`)
- Missing type annotations
- Incorrect import paths

## Further Reading

- [Architecture](./ARCHITECTURE.md) — System design and data model
- [ADR](./ADR.md) — Architecture decision records
- [Seed Data](./SEED_DATA.md) — Complete seed data documentation
