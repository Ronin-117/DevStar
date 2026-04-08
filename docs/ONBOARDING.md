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

# Run Rust clippy (CI-grade linting)
cd src-tauri && cargo clippy -- -D warnings

# Run Rust tests
cd src-tauri && cargo test
```

## What is DevStar?

DevStar is a desktop app for managing project development checklists using a sprint-based workflow. It runs as a **background-first** application — starting as a system tray icon with an MCP server for AI agents, and only showing its UI on demand.

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
│   │   ├── api.ts                # Tauri invoke wrappers + event emit
│   │   ├── types.ts              # TypeScript type definitions
│   │   └── utils.ts              # Utility functions (cn)
│   ├── store/
│   │   └── index.ts              # Zustand state + event listener
│   ├── components/
│   │   ├── active/               # ActiveMode.tsx (Live Mode window)
│   │   ├── projects/             # ProjectsView, ProjectDetailView
│   │   ├── templates/            # TemplatesView, TemplateEditorView,
│   │   │                         # SharedSectionsView, SharedSprintsView
│   │   └── shared/               # Checkbox, CollapsibleSection,
│   │                             # Modal, ProgressBar, TitleBar,
│   │                             # SearchInput, MiniSearchInput
│   ├── assets/                   # app-icon.png, logo-bar.png
│   ├── App.tsx                   # Main app with nav routing
│   ├── main.tsx                  # Entry point + active-window detection
│   └── index.css                 # Global styles + scrollbar hiding
├── src-tauri/                    # Backend (Rust)
│   ├── src/
│   │   ├── lib.rs                # Tauri commands, tray, MCP spawn, startup
│   │   ├── main.rs               # Entry point
│   │   ├── mcp_server.rs         # MCP server binary (stdio JSON-RPC)
│   │   ├── rate_limit.rs         # Rate limiter
│   │   └── db/
│   │       ├── mod.rs            # Module exports
│   │       ├── types.rs          # Rust types
│   │       ├── schema.sql        # SQLite schema
│   │       ├── tests.rs          # Unit tests
│   │       ├── seeds/            # Seed data
│   │       │   ├── mod.rs        # Orchestrator + helpers
│   │       │   ├── shared_sections.rs
│   │       │   ├── shared_sprints.rs
│   │       │   └── templates/    # 12 template seed files
│   │       └── *.rs              # CRUD per entity
│   ├── icons/                    # App icons (PNG, ICO, ICNS)
│   ├── tauri.conf.json           # Tauri config
│   ├── Cargo.toml                # Rust dependencies
│   └── build.rs                  # Tauri build script
├── .github/workflows/
│   └── ci.yml                    # CI/CD pipeline
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

### Frontend
```bash
npx tsc --noEmit        # Type check
npm run build            # Build check
```

### Backend
```bash
cd src-tauri
cargo clippy -- -D warnings   # Lint (CI-grade)
cargo test                     # Unit tests
```

No frontend test framework is configured yet. To add:
```bash
npm i -D vitest @testing-library/react @testing-library/jest-dom jsdom
```
Then add `"test": "vitest"` to `package.json` scripts.

## CI/CD

The GitHub Actions workflow (`.github/workflows/ci.yml`) runs on every push to `main`:

1. **Lint** — `cargo clippy -- -D warnings` + `npx tsc --noEmit`
2. **Test** — `cargo test`
3. **Build** — Full Tauri build for Windows (MSI + EXE) and Linux (DEB)

Tagged releases (`v*`) produce a GitHub Release with all installers attached.

## Troubleshooting

### "Database not found"
Run the DevStar app at least once to create the DB.

### TypeScript errors
Run `npx tsc --noEmit`. Common issues:
- Unused imports (strict mode enforces `noUnusedLocals`)
- Missing type annotations
- Incorrect import paths

### Rust clippy errors
Run `cargo clippy -- -D warnings`. The CI treats all warnings as errors.
Common fixes: prefix unused variables with `_`, use `#[allow(...)]` for intentional patterns.

### Disk space issues
The Rust `target/` directory can grow to 7GB+. Clean it with:
```bash
cd src-tauri && cargo clean
```

## Further Reading

- [Architecture](./ARCHITECTURE.md) — System design and data model
- [ADR](./ADR.md) — Architecture decision records
- [Seed Data](./SEED_DATA.md) — Complete seed data documentation
- [AGENTS.md](./agents/AGENTS.md) — MCP server tool reference
