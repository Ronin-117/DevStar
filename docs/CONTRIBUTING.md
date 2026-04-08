# Contributing to DevStar

Thank you for your interest in contributing! Here's a quick guide to get started.

## Development Setup

### Prerequisites

- **Node.js 20+** and npm
- **Rust stable** (install via [rustup](https://rustup.rs/))
- **Tauri dependencies** — see [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/)

### Getting Started

```bash
# Clone the repository
git clone https://github.com/Ronin-117/DevStar.git
cd DevStar

# Install dependencies
npm install

# Run in development mode
npm run tauri dev
```

## Code Style

### TypeScript/Frontend
- Strict mode with `noUnusedLocals` and `noUnusedParameters`
- 2-space indent, single quotes, semicolons, trailing commas
- Named imports only (no defaults except React)
- Type-only imports: `import type { ... }`
- Run `npx tsc --noEmit` before committing

### Rust/Backend
- Run `cargo clippy -- -D warnings` before committing (CI enforces this)
- Unused variables should be prefixed with `_` or use `#[allow(unused_variables)]` at crate level
- Error handling: return `Result<T, AppError>` from DB functions, `Result<T, String>` from Tauri commands
- All async operations in Tauri commands should use `check_rate_limit`

## Testing

```bash
# Frontend type check
npx tsc --noEmit

# Backend lint (CI-grade)
cd src-tauri && cargo clippy -- -D warnings

# Backend tests
cd src-tauri && cargo test
```

## Pull Request Process

1. Create a feature branch (`git checkout -b feature/your-feature`)
2. Make your changes
3. Ensure all checks pass:
   - `npx tsc --noEmit`
   - `cd src-tauri && cargo clippy -- -D warnings`
   - `cd src-tauri && cargo test`
4. Commit with a descriptive message
5. Push and open a Pull Request

## Adding a New Template

Templates are defined as Rust seed files. See [SEED_DATA.md](./SEED_DATA.md) for the structure.

1. Create `src-tauri/src/db/seeds/templates/your_template.rs`
2. Export from `src-tauri/src/db/seeds/templates/mod.rs`
3. Call from `src-tauri/src/db/seeds/mod.rs::seed_all()`

## Adding a New Tauri Command

1. Add the database function in the appropriate `src-tauri/src/db/*.rs` file
2. Add the `#[tauri::command]` function in `src-tauri/src/lib.rs`
3. Register it in the `invoke_handler` macro
4. Add the API wrapper in `src/lib/api.ts`
5. Use it in your component

## Reporting Issues

When reporting a bug, please include:
- OS and version
- DevStar version
- Steps to reproduce
- Expected vs actual behavior
- Screenshots if applicable

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
