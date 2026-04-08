# Architecture Decision Records

## ADR-001: Sprint-Based Data Model

**Date**: 2025-04-01
**Status**: Accepted

### Context

The original data model used a flat `Template → Sections → Items` hierarchy. This was insufficient for organizing complex project checklists that naturally group into phases/sprints.

### Decision

Adopt a sprint-based model: `Template → Sprints → Sections → Items`. Introduce `SharedSections` and `SharedSprints` as reusable building blocks that can be linked or copied into templates.

### Consequences

- **Positive**: Better organization, reusable components, realistic project planning
- **Positive**: Link vs Copy semantics allow both shared updates and project-specific customization
- **Negative**: More complex database schema and migration
- **Negative**: More complex frontend with nested expand/collapse UI

---

## ADR-002: SQLite with Rust Backend

**Date**: 2025-04-01
**Status**: Accepted

### Context

Needed a lightweight, self-contained persistence layer for a desktop app.

### Decision

Use SQLite via `rusqlite` crate, embedded directly in the Tauri app. No external database server required.

### Consequences

- **Positive**: Zero configuration, single-file database, portable
- **Positive**: rusqlite provides type-safe parameterized queries
- **Negative**: No built-in multi-user support (acceptable for desktop app)
- **Negative**: Manual schema migrations required

---

## ADR-003: Zustand for State Management

**Date**: 2025-04-01
**Status**: Accepted

### Context

Needed a lightweight, performant state management solution for React.

### Decision

Use Zustand with a single store pattern. Detail data cached in `Map<number, DetailType>` for O(1) keyed access.

### Consequences

- **Positive**: Minimal boilerplate, no providers needed, excellent TypeScript support
- **Positive**: Map-based caches prevent unnecessary re-renders
- **Negative**: Single store can become large (mitigated by detail caching)

---

## ADR-004: Cross-Window Event Sync

**Date**: 2025-04-01
**Status**: Accepted

### Context

The Live Mode window and Management window are separate Tauri windows with separate Zustand stores. Changes in one window need to reflect in the other without full refetches.

### Decision

Use Tauri's event system (`emit`/`listen`) to broadcast item toggle events. The management window's store listens and updates its cached data in-place, recalculating progress maps without refetching.

### Consequences

- **Positive**: No scroll jump, no flicker, instant updates
- **Positive**: Minimal network/backend load
- **Negative**: Event-based sync requires careful cache invalidation

---

## ADR-005: Link vs Copy Semantics for Shared Items

**Date**: 2025-04-01
**Status**: Accepted

### Context

Users need flexibility: sometimes they want shared sections to auto-update when the source changes; sometimes they want project-specific independence.

### Decision

Implement `is_linked` flag on sprint sections. Linked sections reference the shared source; copied sections are independent. For projects, `linked_from_section_id` stores the reference to the shared section.

### Consequences

- **Positive**: Maximum flexibility for users
- **Positive**: Clear semantic distinction
- **Negative**: More complex UI (toggle between link/copy modes)
- **Negative**: Need to handle linked section updates carefully

---

## ADR-006: Seed Database Only on First Run

**Date**: 2025-04-04
**Status**: Accepted
**Supersedes**: ADR-006 (original)

### Context

The original approach wiped and re-seeded the database on every app start. This meant all user projects were lost on every restart, making the app unusable for real work.

### Decision

Check if the `templates` table is empty on startup. Only seed if empty (first run). This preserves user data across restarts while still populating the database on first launch.

### Consequences

- **Positive**: User data persists across restarts
- **Positive**: Seed data still present on first run
- **Negative**: Seed data changes won't apply to existing databases (requires manual migration or DB deletion)

---

## ADR-007: Background-First App with System Tray

**Date**: 2025-04-04
**Status**: Accepted

### Context

DevStar needs to run as a background service (MCP server for AI agents) while optionally showing a UI. The app should start minimized to the system tray on login.

### Decision

- Management window starts with `visible: false`
- System tray icon created on startup with menu (Open, Live Mode, Stop)
- MCP server spawned as a hidden child process
- Window close hides the window instead of quitting
- "Stop DevStar" from tray kills MCP server and exits
- App adds itself to system startup on first run

### Consequences

- **Positive**: Always available for MCP agents, no need to manually launch
- **Positive**: Clean user experience — UI on demand, background always running
- **Negative**: More complex lifecycle management
- **Negative**: Requires platform-specific code for startup registration

---

## ADR-008: MCP Server for AI Agent Integration

**Date**: 2025-04-04
**Status**: Accepted

### Context

AI coding agents (Claude Code, OpenCode, Antigravity) need programmatic access to project plans so they can read sprint status, update items, and log errors without manual UI interaction.

### Decision

Build a separate binary (`devstar-mcp`) that implements the Model Context Protocol over stdio:
- Spawns as a background child process on app startup
- Shares the same SQLite database
- Exposes 14 tools for reading and modifying project data
- Hidden console window on Windows (`CREATE_NO_WINDOW`)
- Killed gracefully on app exit

### Consequences

- **Positive**: AI agents can autonomously track and update project progress
- **Positive**: Standard protocol (MCP) — works with any MCP-compatible agent
- **Negative**: Additional binary to maintain
- **Negative**: No authentication — any process that can connect to stdin/stdout can use it (acceptable for local-only use)

---

## ADR-009: .devstar.json for Project Discovery

**Date**: 2025-04-04
**Status**: Accepted

### Context

AI agents working in a project directory need to know which DevStar project they're working on without being told the project ID manually.

### Decision

When `create_project` is called with a `project_dir` parameter, write a `.devstar.json` file to that directory containing the `project_id`. Agents can call `get_project_context` to read this file and get full project state.

### Consequences

- **Positive**: Zero-configuration project discovery for agents
- **Positive**: Works across any AI agent that can read files
- **Negative**: File must be manually deleted if the project is deleted from DevStar

---

## ADR-010: Transparent Window for Live Mode

**Date**: 2025-04-04
**Status**: Accepted

### Context

The minimized Live Mode button should look like a floating icon on the desktop, not a rectangular window with a solid background.

### Decision

- Active window uses `transparent: true` in Tauri config
- Minimized state: transparent container with a white rounded button containing the app icon
- Full panel state: white content areas on transparent window background
- CSS forces `background: #ffffff` on content areas for readability

### Consequences

- **Positive**: Clean floating button aesthetic
- **Positive**: Content always readable with solid white backgrounds
- **Negative**: Requires careful CSS to avoid transparency bleeding through content
