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

## ADR-006: Database Wipe and Re-seed on Every Run

**Date**: 2025-04-01
**Status**: Accepted (Temporary)

### Context

During active development, the seed data changes frequently. A migration system would add complexity.

### Decision

Wipe all tables and re-seed on every app start. This ensures the latest seed data is always present.

### Consequences

- **Positive**: Always up-to-date seed data, simple implementation
- **Negative**: All user data is lost on restart (acceptable during development)
- **Future**: Should implement proper migration system before production release

---

## ADR-007: Compact Minimized Live Mode

**Date**: 2025-04-01
**Status**: Accepted

### Context

Users need a way to keep the Live Mode accessible without taking up screen space.

### Decision

Minimize to a 48×48px round indigo button positioned at the top-right corner of the screen with transparent background. Clicking restores to full panel (340×500px) positioned to the left of the button.

### Consequences

- **Positive**: Always accessible, minimal screen footprint
- **Positive**: Clean visual design matching the app theme
- **Negative**: Requires window repositioning logic for restore
