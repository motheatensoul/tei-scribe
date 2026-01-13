# Contributing to Saga-Scribe

Thank you for your interest in contributing to Saga-Scribe! This document provides guidelines for contributors.

## Development Environment

### Prerequisites
- **Bun**: Our primary package manager and runtime.
- **Rust**: Required for the Tauri backend (version 1.77.2+).
- **Node.js**: Underlying requirement for Vite/SvelteKit.

### Setup
1. Clone the repository.
2. Run `bun install` to install frontend dependencies.
3. Run `bun run dev` to start the development environment.

## Coding Standards

Refer to `docs/UI_STANDARDS.md` for UI and CSS guidelines, and `AGENTS.md` for detailed architecture patterns.

### Frontend (Svelte 5 + TypeScript)
- Use **Runes** (`$state`, `$derived`, `$props`) for state management.
- Store files must use the `.svelte.ts` extension.
- Components should be well-typed and follow the DaisyUI theme.
- All components must pass accessibility (a11y) checks.

### Backend (Rust)
- Adhere to standard Rust idioms (`rustfmt`).
- Use `Result<T, String>` (or more specific error types) for error handling.
- Avoid `unwrap()` or `expect()` in production code.
- New parser logic should be placed in `src-tauri/src/parser/`.

## Quality Assurance

Before submitting a Pull Request, please ensure the following commands pass:

```bash
bun run check:all   # Runs both frontend (svelte-check) and backend (clippy)
bun run test:rust    # Runs backend test suite
```

## Pull Request Process
1. Create a new branch for your feature or bugfix.
2. Ensure your changes adhere to the project's architecture.
3. Update the `CHANGELOG.md` with a summary of your changes.
4. Submit the PR for review.

## License
By contributing, you agree that your contributions will be licensed under the project's **GPL-3.0-or-later** license.
